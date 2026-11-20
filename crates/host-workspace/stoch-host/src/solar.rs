//! Energetic vital margin engine (`SolarWatchdogEngine`) strictly decoupled from Byzantine fraud.
//!
//! Uses purely fixed-point integer arithmetic (`UNIT = 1_000_000`), maps unpowered/LVD bus readings
//! cleanly to [`CoddValue::ApplicableUnknown`] instead of coercing `0.0 V` to a valid zero, enforces
//! non-compensatory `min(s_voltage, s_soc)`, applies integer EMA + Schmitt-trigger hysteresis, and
//! commits pre-LVD checkpoints under 2PC durability without ever emitting fraud certificates.

use core::sync::atomic::Ordering;

use stoch_custody::CustodyLedger;
use stoch_math::{CoddValue, UNIT};

use crate::metrics::LockFreeMetrics;
use crate::HostError;

/// Fixed-point physical telemetry reading from a local solar/battery bus.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct SolarReading {
    /// Scaled bus voltage headroom in millionths (`0..=UNIT`).
    pub voltage_scaled: i64,
    /// Scaled state-of-charge headroom in millionths (`0..=UNIT`).
    pub soc_scaled: i64,
    /// `false` when low-voltage disconnection / unpowered bus makes voltage unobservable.
    pub bus_powered: bool,
}

/// Schmitt-trigger conservation state protecting physical battery survival ahead of LVD.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SolarState {
    /// Full nominal operating headroom (`kappa_scale = 1_000_000`).
    Nominal,
    /// Cautionary headroom tightening (`kappa_scale = 750_000`).
    Caution,
    /// Precautionary load conservation (`kappa_scale = 350_000`).
    Conserve,
    /// Imminent low-voltage disconnect requiring 2PC checkpoint and `kappa_scale = 0`.
    PreLvdShutdown,
}

impl SolarState {
    /// Returns the outgoing conductance multiplier in millionths (`0..=UNIT`).
    #[must_use]
    pub const fn energy_kappa_scale_millionths(self) -> i64 {
        match self {
            Self::Nominal => UNIT,
            Self::Caution => 750_000,
            Self::Conserve => 350_000,
            Self::PreLvdShutdown => 0,
        }
    }
}

/// Threshold configuration governing fixed-point EMA smoothing and Schmitt hysteresis.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct SolarWatchdogConfig {
    /// Numerator of EMA weight (`alpha = alpha_num / alpha_den`, `1 <= alpha_num <= alpha_den`).
    pub alpha_num: i64,
    /// Denominator of EMA weight (`alpha_den > 0`).
    pub alpha_den: i64,
    /// Floor entering `PreLvdShutdown` (`< conserve_enter`).
    pub lvd_enter: i64,
    /// Recovery threshold leaving `PreLvdShutdown` (`> lvd_enter`).
    pub lvd_recover: i64,
    /// Floor entering `Conserve` (`< caution_enter`).
    pub conserve_enter: i64,
    /// Recovery threshold leaving `Conserve` (`> conserve_enter`).
    pub conserve_recover: i64,
    /// Floor entering `Caution`.
    pub caution_enter: i64,
    /// Recovery threshold leaving `Caution` (`> caution_enter`).
    pub caution_recover: i64,
}

impl SolarWatchdogConfig {
    /// Validated default physical watchdog thresholds scaled in millionths.
    #[must_use]
    pub const fn default_calibrated() -> Self {
        Self {
            alpha_num: 1,
            alpha_den: 4,
            lvd_enter: 150_000,
            lvd_recover: 220_000,
            conserve_enter: 350_000,
            conserve_recover: 420_000,
            caution_enter: 600_000,
            caution_recover: 680_000,
        }
    }

    fn validate(self) -> Result<(), HostError> {
        if self.alpha_den <= 0
            || self.alpha_num <= 0
            || self.alpha_num > self.alpha_den
            || !(0..self.lvd_recover).contains(&self.lvd_enter)
            || self.lvd_recover > self.conserve_enter
            || self.conserve_enter >= self.conserve_recover
            || self.conserve_recover > self.caution_enter
            || self.caution_enter >= self.caution_recover
            || self.caution_recover > UNIT
        {
            return Err(HostError::InvalidConfiguration(
                "invalid SolarWatchdogConfig EMA or hysteresis ordering",
            ));
        }
        Ok(())
    }
}

/// Output emitted by [`SolarWatchdogEngine::step_reading`], carrying strictly vital headroom
/// (structurally incapable of representing or emitting `SignedFraudProof`).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct WatchdogStepOutcome {
    /// Codd relational classification of bus voltage (`ApplicableUnknown` when unpowered).
    pub voltage_codd: CoddValue,
    /// Instantaneous non-compensatory vital floor (`min(s_v, s_soc)`).
    pub raw_vital_floor: i64,
    /// Smoothed fixed-point EMA floor driving hysteresis transitions.
    pub ema_vital_floor: i64,
    /// Current hysteresis operating state.
    pub state: SolarState,
    /// Conductance scale (`0..=UNIT`) passed to local coupling throttling.
    pub energy_kappa_scale_millionths: i64,
    /// `true` if entering `PreLvdShutdown` triggered an immediate 2PC custody checkpoint.
    pub checkpoint_written: bool,
}

/// Stateful per-node energy-survival monitor decoupled from adversarial fraud certification.
pub struct SolarWatchdogEngine {
    config: SolarWatchdogConfig,
    state: SolarState,
    ema_floor: Option<i64>,
}

impl SolarWatchdogEngine {
    /// Constructs a validated watchdog engine starting in [`SolarState::Nominal`].
    pub fn new(config: SolarWatchdogConfig) -> Result<Self, HostError> {
        config.validate()?;
        Ok(Self {
            config,
            state: SolarState::Nominal,
            ema_floor: None,
        })
    }

    /// Current Schmitt-trigger conservation state.
    #[must_use]
    pub const fn state(&self) -> SolarState {
        self.state
    }

    /// Processes a physical telemetry reading, maps unpowered bus readings to `ApplicableUnknown`,
    /// updates EMA + hysteresis, updates lock-free metrics, and writes a durable 2PC pre-LVD
    /// custody checkpoint when transitioning into [`SolarState::PreLvdShutdown`].
    pub fn step_reading(
        &mut self,
        reading: SolarReading,
        valid_ts: u64,
        tx_ts: u64,
        ledger: &CustodyLedger,
        metrics: &LockFreeMetrics,
    ) -> Result<WatchdogStepOutcome, HostError> {
        let soc = reading.soc_scaled.clamp(0, UNIT);
        let voltage_codd = if reading.bus_powered {
            CoddValue::Known(reading.voltage_scaled.clamp(0, UNIT))
        } else {
            CoddValue::ApplicableUnknown
        };

        let raw_vital_floor = match voltage_codd {
            CoddValue::Known(v) => v.min(soc),
            // Unpowered / unobservable voltage collapses local vital floor conservatively.
            CoddValue::ApplicableUnknown => 0,
            CoddValue::Inapplicable | CoddValue::NotAskable => 0,
        };

        let ema = match self.ema_floor {
            None => raw_vital_floor,
            Some(prev) => {
                let a_num = i128::from(self.config.alpha_num);
                let a_den = i128::from(self.config.alpha_den);
                let blended =
                    (a_num * i128::from(raw_vital_floor) + (a_den - a_num) * i128::from(prev)) / a_den;
                i64::try_from(blended).map_err(|_| {
                    HostError::InvalidConfiguration("EMA intermediate exceeded i64")
                })?
            }
        };
        self.ema_floor = Some(ema);

        let prev_state = self.state;
        self.state = next_schmitt_state(prev_state, ema, &self.config);
        let kappa = self.state.energy_kappa_scale_millionths();
        metrics.solar_kappa_scale_millionths.store(
            u64::try_from(kappa.max(0)).unwrap_or(0),
            Ordering::Relaxed,
        );

        let mut checkpoint_written = false;
        if self.state == SolarState::PreLvdShutdown && prev_state != SolarState::PreLvdShutdown {
            let mut payload = [0_u8; 16];
            payload[0..8].copy_from_slice(&ema.to_le_bytes());
            payload[8..16].copy_from_slice(&soc.to_le_bytes());
            ledger.append_contingency(valid_ts, tx_ts, &payload)?;
            metrics.redb_checkpoints_total.fetch_add(1, Ordering::Relaxed);
            checkpoint_written = true;
        }

        Ok(WatchdogStepOutcome {
            voltage_codd,
            raw_vital_floor,
            ema_vital_floor: ema,
            state: self.state,
            energy_kappa_scale_millionths: kappa,
            checkpoint_written,
        })
    }
}

const fn next_schmitt_state(
    current: SolarState,
    ema: i64,
    cfg: &SolarWatchdogConfig,
) -> SolarState {
    if ema < cfg.lvd_enter {
        return SolarState::PreLvdShutdown;
    }
    match current {
        SolarState::Nominal => {
            if ema < cfg.conserve_enter {
                SolarState::Conserve
            } else if ema < cfg.caution_enter {
                SolarState::Caution
            } else {
                SolarState::Nominal
            }
        }
        SolarState::Caution => {
            if ema < cfg.conserve_enter {
                SolarState::Conserve
            } else if ema >= cfg.caution_recover {
                SolarState::Nominal
            } else {
                SolarState::Caution
            }
        }
        SolarState::Conserve => {
            if ema >= cfg.conserve_recover {
                if ema >= cfg.caution_recover {
                    SolarState::Nominal
                } else {
                    SolarState::Caution
                }
            } else {
                SolarState::Conserve
            }
        }
        SolarState::PreLvdShutdown => {
            if ema >= cfg.lvd_recover {
                if ema >= cfg.conserve_recover {
                    SolarState::Caution
                } else {
                    SolarState::Conserve
                }
            } else {
                SolarState::PreLvdShutdown
            }
        }
    }
}
