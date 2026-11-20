//! Autonomous local survival halt evaluator (`Regime III-B`).
//!
//! Operates strictly outside multi-party consensus ceremonies and mutual locking so correlated
//! wide-area brownouts (>1/3 peers unavailable) can never freeze an urgent safety stop.

use core::sync::atomic::Ordering;
use stoch_core::{step, Error as CoreError, Regime, State};

use crate::metrics::LockFreeMetrics;

/// Autonomous per-step survival halt evaluator decoupled from distributed agreement paths.
pub struct LocalEmergencyController;

impl LocalEmergencyController {
    /// Evaluates sealed online state locally through [`stoch_core::step`] regardless of how many
    /// remote peers are disconnected in brownout (`fleet_brownout_ratio_millionths`).
    ///
    /// Updates lock-free observability counters immediately when [`Regime::AlgedonicHalt`] fires.
    pub fn evaluate_local_survival(
        state: &State,
        fleet_brownout_ratio_millionths: u64,
        metrics: &LockFreeMetrics,
    ) -> Result<Regime, CoreError> {
        metrics
            .fleet_brownout_ratio_millionths
            .store(fleet_brownout_ratio_millionths, Ordering::Relaxed);
        let regime = step(state)?;
        if matches!(regime, Regime::AlgedonicHalt { .. }) {
            metrics
                .local_halt_triggers_total
                .fetch_add(1, Ordering::Relaxed);
        }
        Ok(regime)
    }
}
