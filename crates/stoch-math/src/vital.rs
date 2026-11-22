//! Operator II — the vital margin (`46`–`47`).
//!
//! Fixed-point volatility drag, non-ergodic gap, tail draw impact, monotone
//! reserve recovery-time bisection, and survival latency classification.

use crate::{narrow_op, OperatorError, UNIT};

/// The reserve dynamics of one trajectory: periodic inflow, periodic outflow,
/// and heavy-tail scale in operator units.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ReserveDynamics {
    inflow: i64,
    outflow: i64,
    tail_scale: i64,
}

impl ReserveDynamics {
    /// Declares non-negative reserve dynamics.
    pub fn new(inflow: i64, outflow: i64, tail_scale: i64) -> Result<Self, OperatorError> {
        if inflow < 0 || outflow < 0 || tail_scale < 0 {
            return Err(OperatorError::OutOfRange("negative reserve dynamics"));
        }
        Ok(ReserveDynamics {
            inflow,
            outflow,
            tail_scale,
        })
    }

    /// Periodic inflow to the reserve.
    pub const fn inflow(self) -> i64 {
        self.inflow
    }

    /// Periodic outflow from the reserve.
    pub const fn outflow(self) -> i64 {
        self.outflow
    }

    /// Tail dispersion scale.
    pub const fn tail_scale(self) -> i64 {
        self.tail_scale
    }

    /// Net periodic drift `inflow - outflow`.
    pub fn net(self) -> Result<i64, OperatorError> {
        self.inflow
            .checked_sub(self.outflow)
            .ok_or(OperatorError::Overflow)
    }
}

/// Second-order volatility drag of a multiplicative reserve: `sigma^2 / (2 * UNIT)`.
pub fn volatility_drag(tail_scale: i64) -> Result<i64, OperatorError> {
    if tail_scale < 0 {
        return Err(OperatorError::OutOfRange("negative tail scale"));
    }
    let ts = i128::from(tail_scale);
    let prod = ts.checked_mul(ts).ok_or(OperatorError::Overflow)?;
    narrow_op(prod / (2 * i128::from(UNIT)))
}

/// Time-average growth rate of a single trajectory: `drift - volatility_drag`.
pub fn time_growth(drift: i64, tail_scale: i64) -> Result<i64, OperatorError> {
    drift
        .checked_sub(volatility_drag(tail_scale)?)
        .ok_or(OperatorError::Overflow)
}

/// Ensemble expectation growth rate (`drift`).
pub const fn ensemble_growth(drift: i64) -> i64 {
    drift
}

/// Non-ergodic gap `ensemble_growth - time_growth`.
pub fn non_ergodic_gap(drift: i64, tail_scale: i64) -> Result<i64, OperatorError> {
    narrow_op(i128::from(ensemble_growth(drift)) - i128::from(time_growth(drift, tail_scale)?))
}

/// Linear reserve projection after `periods` steps of net drift.
pub fn reserve_at(
    dynamics: ReserveDynamics,
    initial: i64,
    periods: u32,
) -> Result<i64, OperatorError> {
    let delta = i128::from(i64::from(periods)) * i128::from(dynamics.net()?);
    narrow_op(i128::from(initial) + delta)
}

/// Reserve level after a tail draw of `sigmas` tail scales.
pub fn reserve_after_tail(
    dynamics: ReserveDynamics,
    reserve: i64,
    sigmas: i64,
) -> Result<i64, OperatorError> {
    if sigmas < 0 {
        return Err(OperatorError::OutOfRange("negative tail draw"));
    }
    let shock = i128::from(sigmas)
        .checked_mul(i128::from(dynamics.tail_scale))
        .ok_or(OperatorError::Overflow)?;
    narrow_op(i128::from(reserve) - shock)
}

/// Returns `true` iff a tail draw of `sigmas` drives the reserve below zero.
pub fn ruined_by_tail(
    dynamics: ReserveDynamics,
    reserve: i64,
    sigmas: i64,
) -> Result<bool, OperatorError> {
    Ok(reserve_after_tail(dynamics, reserve, sigmas)? < 0)
}

/// Least period `t <= horizon` with `reserve_at(dynamics, initial, t) >= target`,
/// found by monotone bisection, or `None` if no crossing occurs within `horizon`.
pub fn recovery_time(
    dynamics: ReserveDynamics,
    initial: i64,
    target: i64,
    horizon: u32,
) -> Result<Option<u32>, OperatorError> {
    if initial >= target {
        return Ok(Some(0));
    }
    if dynamics.net()? <= 0 {
        return Ok(None);
    }
    if reserve_at(dynamics, initial, horizon)? < target {
        return Ok(None);
    }
    let mut low: u32 = 1;
    let mut high: u32 = horizon;
    while low < high {
        let middle = low + (high - low) / 2;
        if reserve_at(dynamics, initial, middle)? >= target {
            high = middle;
        } else {
            low = middle + 1;
        }
    }
    Ok(Some(low))
}

/// Survival comparison `tts_survival >= ttr_recovery`.
pub const fn survival_condition(tts_survival: i64, ttr_recovery: i64) -> bool {
    tts_survival >= ttr_recovery
}

/// Three mutually exclusive survival outcomes.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SurvivalOutcome {
    /// Crossing exists within horizon and `TTS_survival >= TTR_recovery`.
    RecoversInTime,
    /// Crossing exists within horizon, yet arrives after survival budget expires.
    CrossesAfterRuin,
    /// No recovery crossing occurs inside the declared horizon.
    NoCrossingWithinHorizon,
}

impl SurvivalOutcome {
    /// Wire tag (`0..=2`).
    pub const fn tag(self) -> u8 {
        match self {
            SurvivalOutcome::RecoversInTime => 0,
            SurvivalOutcome::CrossesAfterRuin => 1,
            SurvivalOutcome::NoCrossingWithinHorizon => 2,
        }
    }
}

/// Classifies survival outcome given `tts_survival` and optional crossing `ttr_recovery`.
pub fn survival_outcome(tts_survival: i64, ttr_recovery: Option<i64>) -> SurvivalOutcome {
    match ttr_recovery {
        None => SurvivalOutcome::NoCrossingWithinHorizon,
        Some(ttr) if survival_condition(tts_survival, ttr) => SurvivalOutcome::RecoversInTime,
        Some(_) => SurvivalOutcome::CrossesAfterRuin,
    }
}
