//! Dynamics of the Three Ontological Layers ($L_1 \to L_2 \to L_3 \to L_1$),
//! Bridge Mechanics, and Formal Performative Prediction (§1.1).

pub mod jensen;
pub mod missingness;
pub mod ontology;
pub mod performative_power;
pub mod performativity;
pub mod provenance;

pub use jensen::{NonlinearSummary, empirical_crps};
pub use missingness::{ManskiBounds, MengAudit, RubinMultipleImputationAudit};
pub use ontology::{
    DpiOperationalStage, Layer, UncertaintyClass, apply_cromwell_floor, laplace_succession,
    respects_cromwell,
};
pub use performative_power::{
    EpsilonDeclarationAudit, PerformativePowerAudit, exact_wasserstein_1,
};
pub use performativity::{
    GoodhartTaxonomyAudit, GoodhartVariant, PerformativeDiagnostic, PerformativeSystem,
    empirical_wasserstein_1,
};
pub use provenance::{ClaimProvenanceClass, EpistemicPkmProvenanceAudit};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rng::mean;

    #[test]
    fn meng_identity_closes_exactly_in_population() {
        let recorded = [true, true, true, false, false, false, false, false];
        let y = [10.0, 12.0, 14.0, 2.0, 3.0, 4.0, 1.0, 2.0];
        let audit = MengAudit::from_population(&recorded, &y);
        let y_obs_mean = (10.0 + 12.0 + 14.0) / 3.0;
        let y_pop_mean = mean(&y);
        assert!((audit.estimation_error - (y_obs_mean - y_pop_mean)).abs() < 1e-12);
        assert!(audit.rho_r_y > 0.90);
        let expected_n_eff = 3.0 / (1.0 + 5.0 * audit.rho_r_y * audit.rho_r_y);
        assert!((audit.n_eff_meng - expected_n_eff).abs() < 1e-12);
        assert!(audit.n_eff_meng < 1.0);
    }

    #[test]
    fn nonlinear_summary_measures_convex_jensen_gap_and_oos_crps() {
        let xs = [1.0, 2.0, 3.0, 4.0, 10.0];
        let summary = NonlinearSummary::evaluate(&xs, |x| x * x, 2.0);
        assert!(summary.jensen_gap > 0.0);
        assert_eq!(summary.f_at_mean, 16.0);
        assert_eq!(summary.expected_f, 26.0);
        assert!((summary.jensen_gap - 10.0).abs() < 1e-12);
        assert!(summary.empirical_crps > 0.0);

        let oos = NonlinearSummary::evaluate_out_of_sample(&xs[..4], &xs[4..], |x| x * x, 2.0);
        assert!(oos.empirical_crps > summary.empirical_crps);
    }

    #[test]
    fn performative_stability_vs_optimality_and_rrm_contraction() {
        let sys_zero_alpha = PerformativeSystem::new(1.0, 0.2, 0.0, 1.0, 1.2).diagnose();
        assert!((sys_zero_alpha.theta_ps - 1.25).abs() < 1e-12);
        assert!((sys_zero_alpha.theta_po - 1.25).abs() < 1e-12);
        assert!(sys_zero_alpha.risk_at_ps.abs() < 1e-12);
        assert!(sys_zero_alpha.risk_at_po.abs() < 1e-12);

        let sys = PerformativeSystem::new(1.0, 0.2, 0.5, 1.0, 1.2);
        let diag = sys.diagnose();
        assert!((diag.theta_sl - 1.5).abs() < 1e-12);
        assert!((diag.theta_ps - 1.875).abs() < 1e-12);
        assert!((diag.theta_po - 2.03125).abs() < 1e-12);
        assert!(diag.rrm_converges);
        assert!(diag.risk_at_po < diag.risk_at_ps);
        assert!((diag.rrm_trajectory.last().copied().unwrap() - diag.theta_ps).abs() < 1e-2);

        let unstable = PerformativeSystem::new(1.0, 0.8, 0.5, 1.0, 1.5).diagnose();
        assert!(!unstable.rrm_converges);
    }

    #[test]
    fn cromwell_floor_clamps_dogmatic_extremes() {
        assert!((apply_cromwell_floor(0.0, 0.001) - 0.001).abs() < 1e-15);
        assert!((apply_cromwell_floor(1.0, 0.001) - 0.999).abs() < 1e-15);
        assert!(respects_cromwell(apply_cromwell_floor(0.0, 0.001), 0.001));
    }

    #[test]
    fn lee_bounds_wasserstein_and_perfgd_converge_to_stackelberg_po() {
        let sys = PerformativeSystem::new(1.0, 0.2, 0.5, 1.0, 1.2);
        let diag = sys.diagnose();
        let perfgd = sys.perfgd_trajectory(0.0, 0.5, 60);
        let theta_final = *perfgd.last().unwrap();
        assert!((theta_final - diag.theta_po).abs() < 1e-6);

        let w1 = empirical_wasserstein_1(&[1.0, 2.0, 3.0], &[2.5, 3.5, 4.5]);
        assert!((w1 - 1.5).abs() < 1e-12);

        let (lo, hi, q) =
            ManskiBounds::lee_trimming_bounds(&[10.0, 12.0, 14.0, 16.0], 4, &[8.0, 9.0, 10.0], 4);
        assert!(lo <= hi && q > 0.0);

        let gh = GoodhartTaxonomyAudit::evaluate(2026, 45.0, 6.2);
        assert!(gh.all_four_variants_audited);
        assert!(gh.adversarial_grad2_dpr_at_ps.abs() < 1e-10);
        assert!(gh.adversarial_grad1_dpr_at_ps.abs() > 0.05);

        let perfgd_fd = sys.perfgd_finite_difference_trajectory(0.0, 0.5, 0.01, 60);
        assert!((perfgd_fd.last().copied().unwrap() - diag.theta_po).abs() < 1e-6);

        let (t_stab, t_opt, ratio) = PerformativeSystem::izzo_linear_2x_benchmark(-1.0, 2.0);
        assert!((t_stab - 2.0).abs() < 1e-12);
        assert!((t_opt - 1.0).abs() < 1e-12);
        assert!((ratio - 2.0).abs() < 1e-12);

        let meng_10m = MengAudit::dpi_benchmark_10m_80pct();
        assert!((meng_10m.data_defect_correlation_ddc() - 0.005).abs() < 1e-12);
        assert!((meng_10m.data_defect_index_ddi() - 2.5e-5).abs() < 1e-15);
        assert!((meng_10m.n_eff_srs_asymptotic() - 160_000.0).abs() < 1e-6);
        assert!((meng_10m.law_of_large_populations_z() - 15.811_388).abs() < 1e-5);

        let delta_star = ManskiBounds::lee_tipping_point_delta(
            &[10.0, 12.0, 14.0, 16.0],
            5,
            &[8.0, 9.0, 10.0],
            5,
        );
        assert!(delta_star < 0.0);

        assert_eq!(
            DpiOperationalStage::from_ontological(Layer::WorldL1, None),
            DpiOperationalStage::D1DataCollection
        );
        assert_eq!(
            DpiOperationalStage::from_ontological(Layer::ModelL2, Some("L1->L2")),
            DpiOperationalStage::D2FeatureEngineering
        );
        assert_eq!(
            DpiOperationalStage::from_ontological(Layer::ModelL2, None),
            DpiOperationalStage::D3ModelRepresentation
        );
        assert_eq!(
            DpiOperationalStage::from_ontological(Layer::AgentL3, Some("L3->L1")),
            DpiOperationalStage::D4HumanDecisionActuation
        );

        let rubin = RubinMultipleImputationAudit::combine_imputations(
            &[10.2, 10.8, 9.9, 10.5, 10.6],
            &[0.16, 0.18, 0.15, 0.17, 0.19],
        );
        assert!(rubin.variance_identity_error < 1e-15);
        assert!(rubin.total_variance_t_m > rubin.within_variance_u_bar);
        assert!(rubin.fraction_missing_information_fmi > 0.30);
        let (_, ratio_shrink) =
            RubinMultipleImputationAudit::single_mean_imputation_variance_shrinkage(
                &[2.0, 4.0, 6.0, 8.0, 10.0, 12.0],
                10,
            );
        assert!((ratio_shrink - 0.60).abs() < 1e-12);

        let pkm_ok = EpistemicPkmProvenanceAudit::audit_ingestion(
            &[
                (
                    ClaimProvenanceClass::Measured,
                    ClaimProvenanceClass::Measured,
                    true,
                ),
                (
                    ClaimProvenanceClass::Simulated,
                    ClaimProvenanceClass::Simulated,
                    false,
                ),
            ],
            0,
        );
        assert!(pkm_ok.pkm_provenance_clean);
        let pkm_bad = EpistemicPkmProvenanceAudit::audit_ingestion(
            &[(
                ClaimProvenanceClass::UnanchoredLlmSynthesis,
                ClaimProvenanceClass::Measured,
                false,
            )],
            1,
        );
        assert!(!pkm_bad.pkm_provenance_clean && pkm_bad.provenance_laundering_violations >= 1);
    }

    #[test]
    fn exact_wasserstein_matches_order_statistics_and_handles_unequal_sizes() {
        let p = [0.0, 1.0, 3.0];
        let q = [1.0, 2.0, 6.0];
        // Equal sizes: W1 = mean |p_(i) - q_(i)| = (1 + 1 + 3) / 3.
        assert!((exact_wasserstein_1(&p, &q) - 5.0 / 3.0).abs() < 1e-12);
        assert!((exact_wasserstein_1(&p, &q) - empirical_wasserstein_1(&p, &q)).abs() < 1e-12);
        // Unequal sizes: {0, 2} vs {1}: integral of |F_P - F_Q| = 0.5 * 1 + 0.5 * 1 = 1.
        assert!((exact_wasserstein_1(&[0.0, 2.0], &[1.0]) - 1.0).abs() < 1e-12);
        assert_eq!(exact_wasserstein_1(&[4.0, 4.0], &[4.0]), 0.0);
    }

    #[test]
    fn performative_power_detects_a_shifted_arm_and_flags_an_understated_epsilon() {
        let mut rng = crate::rng::Rng::new(2026);
        let control: Vec<f64> = (0..300).map(|_| rng.normal(0.0, 1.0)).collect();
        let treated: Vec<f64> = (0..300).map(|_| rng.normal(0.5, 1.0)).collect();
        let audit = PerformativePowerAudit::measure(&treated, &control, 2026);
        assert!(audit.significant && audit.permutation_p_value <= 0.01);
        assert!(
            audit.epsilon_lower_bound > 0.2 && audit.epsilon_lower_bound < audit.w1_standardized
        );

        let passive = audit.audit_declaration(0.0, 1.5, 1.0);
        assert!(passive.understated && passive.rrm_radius_declared == 0.0);
        let honest = audit.audit_declaration(audit.epsilon_lower_bound, 1.5, 1.0);
        assert!(!honest.understated);
        assert!(honest.rrm_contracts_at_bound == (audit.epsilon_lower_bound * 1.5 < 1.0));

        let null_arm: Vec<f64> = (0..300).map(|_| rng.normal(0.0, 1.0)).collect();
        let null = PerformativePowerAudit::measure(&null_arm, &control, 2026);
        assert!(!null.significant && null.epsilon_lower_bound == 0.0);
    }
}
