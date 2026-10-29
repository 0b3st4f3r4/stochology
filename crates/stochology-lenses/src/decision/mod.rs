//! Lens 5 — Decision: Cost-Loss Ratio ($C / (\eta L)$), EVPI, and Isotonic
//! CORP-Murphy PAV Decomposition (§1.3.5).

pub mod ahp_ppi;
pub mod corp_murphy;
pub mod cost_loss;

pub use ahp_ppi::{LlmJudgePpiRectifier, SaatyAhpRankReversalAudit};
pub use corp_murphy::{
    CorpPavDecomposition, CorpRocMurphyTriptych, ExactCoinCalibrationBenchmark, MurphyDiagram,
};
pub use cost_loss::{
    CostLossRule, brier_score, critical_cost_loss_threshold, logarithmic_score, quantile_score,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn corp_pav_decomposition_closes_identity_and_proves_resolution_invariance() {
        let p = [0.05, 0.15, 0.25, 0.35, 0.65, 0.75, 0.85, 0.95];
        let y = [0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 1.0];
        let c1 = CorpPavDecomposition::compute(&p, &y);
        assert!(c1.identity_error < 1e-14);
        assert!(c1.rel_miscalibration >= 0.0 && c1.res_resolution > 0.0);

        let p_distorted: Vec<f64> = p.iter().map(|&x| x * x).collect();
        let c2 = CorpPavDecomposition::compute(&p_distorted, &y);
        assert!(c2.identity_error < 1e-14);
        assert!((c1.res_resolution - c2.res_resolution).abs() < 1e-14);
        assert!(c2.rel_miscalibration > c1.rel_miscalibration);
    }

    #[test]
    fn corp_pav_preserves_isotonicity_for_tied_forecasts() {
        let forecasts = [0.75, 0.0, 0.25, 0.75, 0.0];
        let outcomes = [0.0, 1.0, 1.0, 0.0, 0.0];
        let corp = CorpPavDecomposition::compute(&forecasts, &outcomes);
        assert!(corp.identity_error < 1e-14);
        for &q in &corp.recalibrated_probabilities_q {
            assert!((q - 0.4).abs() < 1e-12, "expected constant 0.4, got {q}");
        }
        assert!(corp.res_resolution < 1e-14);
    }

    #[test]
    fn exact_coin_benchmark_proves_calibrated_decisions_are_not_calibrated_probabilities() {
        let bench = ExactCoinCalibrationBenchmark::evaluate(2026, 1_000);
        assert!(bench.separates_decision_from_probability_calibration);
        assert_eq!(bench.decision_agreement_rate_at_p_star_a, 1.0);
        assert!(bench.overconfident_corp.rel_miscalibration > 0.10);
        assert!(bench.calibrated_corp.rel_miscalibration < 0.005);
        assert!(
            (bench.calibrated_corp.res_resolution - bench.overconfident_corp.res_resolution).abs()
                < 1e-12
        );
        assert!(bench.shifted_threshold_excess_regret_per_unit > 15.0);

        let rule_b = CostLossRule::new(70.0, 100.0, 1.0);
        let rev_cal = rule_b.relative_economic_value(&[0.2, 0.8, 0.1, 0.9], &[0.0, 1.0, 0.0, 1.0]);
        assert!((rev_cal - 1.0).abs() < 1e-12);
        let logs = logarithmic_score(&[0.1, 0.9], &[0.0, 1.0], 0.001);
        assert!(logs > 0.0 && logs < 0.2);
    }

    #[test]
    fn corp_log_and_murphy_diagram_reproduce_tri_model_test_vector() {
        let x = [0.1, 0.3, 0.3, 0.5, 0.7, 0.9, 0.2, 0.6];
        let y = [0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 1.0];
        let trip = CorpRocMurphyTriptych::evaluate(&x, &y);
        assert!(trip.all_triptych_identities_closed);
        assert!((trip.brier_corp.brier_total - 0.1925000).abs() < 1e-6);
        assert!((trip.brier_corp.rel_miscalibration - 0.0466667).abs() < 1e-6);
        assert!((trip.brier_corp.res_resolution - 0.1041667).abs() < 1e-6);
        assert!((trip.brier_corp.unc0_uncertainty - 0.2500000).abs() < 1e-6);

        assert!((trip.log_corp.brier_total - 0.5503072).abs() < 1e-6);
        assert!((trip.log_corp.rel_miscalibration - 0.1383276).abs() < 1e-6);
        assert!((trip.log_corp.res_resolution - 0.2811676).abs() < 1e-6);
        assert!((trip.log_corp.unc0_uncertainty - std::f64::consts::LN_2).abs() < 1e-6);

        assert!((trip.murphy.area_under_murphy_curve - 0.0962500).abs() < 1e-12);
        assert!((trip.murphy.half_brier_score - 0.0962500).abs() < 1e-12);
        assert!(trip.murphy.area_identity_error < 1e-14);

        let f_a = [0.35, 0.35, 0.90, 0.90];
        let f_b = [0.10, 0.10, 0.55, 0.55];
        let outs = [0.0, 0.0, 1.0, 1.0];
        assert!(brier_score(&f_a, &outs) < brier_score(&f_b, &outs));
        let (dom, max_excess, worst_theta) =
            MurphyDiagram::check_uniform_dominance(&f_a, &f_b, &outs);
        assert!(!dom && max_excess > 0.05 && (0.10..=0.35).contains(&worst_theta));
    }
}
