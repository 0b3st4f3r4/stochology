//! `stochology-governance` — Bitemporal Clock, Single-Opening Temporal Vault,
//! Real-Time Data Vintages, and Formal Adaptation to the 4 Regimes of Production Drift (§4).

pub mod clock;
pub mod drift;
pub mod vault;
pub mod vintage;

pub use clock::{BitemporalClockAudit, BitemporalRecord, query_with_bitemporal_clock};
pub use drift::{
    AdaptiveConformalInference, CloudLocalLoraArchitectureAudit, CollapseProposition,
    CovariateDriftResult, DriftRegime, EvaluatedCompetenceCriterion, LabelShiftDiagnostic,
    MurphyCorpCircuitBreaker, PlatTrafficLightZone, PredeterminedChangeControlPlan,
    ProfitLossAttributionTest, ReflexiveStochologyGoodhartAudit, RegulatoryComplianceAudit,
    SosaJudgmentGrade, SosaVirtueAndInnesHolmstromContractAudit,
    TensionSevenInstitutionalAgencyAudit, TriadicSafeguardComplementarityAudit,
    adjust_posterior_for_label_shift, estimate_label_shift_four_methods,
    fit_calibrated_logistic_1d, fit_covariate_drift_extra_wcp,
};
pub use vault::{TemporalVault, VaultError, VaultOpeningReport, VaultState};
pub use vintage::{RealTimeVintageAudit, parse_quarter, quarter_label};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quarter_labels_round_trip() {
        let q = parse_quarter("1985Q1").unwrap();
        assert_eq!(q, 4 * 1985);
        assert_eq!(quarter_label(q + 7), "1986Q4");
        assert!(parse_quarter("1985Q5").is_none() && parse_quarter("x").is_none());
    }

    #[test]
    fn real_time_backtest_passes_the_clock_and_the_leaky_one_does_not() {
        // Vintage v holds levels through v - 1; every later vintage revises all levels up by 1%.
        let (start, end) = (
            parse_quarter("2000Q1").unwrap(),
            parse_quarter("2003Q4").unwrap(),
        );
        let mut rows = Vec::new();
        for v in start..=end {
            let revision = 1.0 + 0.01 * f64::from(v - start);
            for o in start - 8..v {
                let level = 100.0 * 1.01_f64.powi((o + 8 - start) as i32);
                rows.push((v, o, revision * level));
            }
        }
        let audit = RealTimeVintageAudit::evaluate(&rows, start + 2, end - 2);
        assert!(audit.decisions > 5);
        assert!(!audit.real_time_clock_alarm && audit.leaky_clock_alarm);
        assert_eq!(audit.leaky_inputs_after_decision, 4 * audit.decisions);
        // A uniform level revision leaves within-vintage growth unchanged.
        assert!(audit.mean_revision.abs() < 1e-9 && audit.sd_revision < 1e-9);
    }
}
