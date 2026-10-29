//! The Four Regimes of Production Drift and Continuous Production Monitor / Circuit Breaker
//! based on Isotonic CORP-Murphy Decomposition (`DriftRegime`, `MurphyCorpCircuitBreaker`, §4.2).

use stochology_lenses::decision::CorpPavDecomposition;

/// The Four Regimes of Production Drift (§4.2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriftRegime {
    Stationary,
    CovariateDrift,
    LabelShift,
    ConceptDriftCircuitBreaker,
    PerformativeFeedback,
}

/// Continuous Production Monitor and Circuit Breaker based on Isotonic CORP-Murphy Decomposition (§4.2).
#[derive(Debug, Clone, PartialEq)]
pub struct MurphyCorpCircuitBreaker {
    pub rel_miscalibration: f64,
    pub res_resolution: f64,
    pub unc0_uncertainty: f64,
    pub circuit_breaker_tripped: bool,
    pub diagnosed_regime: DriftRegime,
    pub recommended_corrective_action: String,
}

impl MurphyCorpCircuitBreaker {
    /// Evaluates rolling predictions and outcomes against `max_rel_threshold` and `min_res_threshold`.
    pub fn monitor(
        forecasts_p: &[f64],
        outcomes_y: &[f64],
        max_rel_threshold: f64,
        min_res_threshold: f64,
        covariate_shift_detected: bool,
        label_shift_detected: bool,
        performative_loop_active: bool,
    ) -> Self {
        let corp = CorpPavDecomposition::compute(forecasts_p, outcomes_y);
        let rel_degraded = corp.rel_miscalibration > max_rel_threshold;
        let res_collapsed = corp.res_resolution < min_res_threshold;

        let (tripped, regime, action) = if res_collapsed {
            (
                true,
                DriftRegime::ConceptDriftCircuitBreaker,
                "TRIP CIRCUIT BREAKER: RES_CORP collapsed below threshold (Concept Drift Q(Y|X) != P(Y|X)); suspend automated decisions and re-enter SIGA Funnel".into(),
            )
        } else if performative_loop_active {
            (
                rel_degraded,
                DriftRegime::PerformativeFeedback,
                "Performative feedback D(theta) active: audit RRM contraction (eps*beta/gamma < 1) and transition to Stackelberg optimum theta_PO".into(),
            )
        } else if label_shift_detected || rel_degraded {
            (
                false,
                DriftRegime::LabelShift,
                "RES_CORP preserved while REL_CORP shifted: apply Label Shift Bayes update via BBSE / MLLS / DFM / OTTER without full retraining".into(),
            )
        } else if covariate_shift_detected {
            (
                false,
                DriftRegime::CovariateDrift,
                "Covariate Drift Q(X) != P(X) with stable conditional physics: apply Exponential Tilt w_hat(x) and ExTRA-WCP-T conformal threshold".into(),
            )
        } else {
            (
                false,
                DriftRegime::Stationary,
                "Stationary regime: REL_CORP and RES_CORP within pre-registered operating bounds"
                    .into(),
            )
        };

        Self {
            rel_miscalibration: corp.rel_miscalibration,
            res_resolution: corp.res_resolution,
            unc0_uncertainty: corp.unc0_uncertainty,
            circuit_breaker_tripped: tripped,
            diagnosed_regime: regime,
            recommended_corrective_action: action,
        }
    }
}
