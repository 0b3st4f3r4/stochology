//! Formal Drift Adaptation and Murphy-CORP Circuit Breaker across the 4 Regimes of Drift (§4.2):
//! 1. **Covariate Drift** ($Q(X) \neq P(X)$, $Q(Y \mid X) = P(Y \mid X)$): Exponential Tilt $\hat{w}(x)$
//!    and Weighted Conformal Prediction (*ExTRA-WCP-T*) with Lei & Candès coverage bound $1 - \alpha - d_{\mathrm{TV}}(Q, \hat{Q})$;
//! 2. **Label / Prior Probability Shift** ($Q(Y) \neq P(Y)$, $Q(X \mid Y) = P(X \mid Y)$): Four canonical
//!    estimators (**BBSE**, **MLLS/SLD**, **DFM**, and **OTTER**);
//! 3. **Concept / Structural Drift** ($Q(Y \mid X) \neq P(Y \mid X)$): Murphy-CORP circuit breaker;
//! 4. **Performative Feedback** ($\mathcal{D}(\theta)$): Stackelberg equilibrium / RRM contraction audit.

pub mod circuit_breaker;
pub mod conformal;
pub mod covariate;
pub mod ethics_of_aim;
pub mod label_shift;
pub mod pccp_plat;
pub mod statutory;

pub use circuit_breaker::{DriftRegime, MurphyCorpCircuitBreaker};
pub use conformal::AdaptiveConformalInference;
pub use covariate::{CovariateDriftResult, fit_covariate_drift_extra_wcp};
pub use ethics_of_aim::{
    CollapseProposition, ReflexiveStochologyGoodhartAudit, SosaJudgmentGrade,
    SosaVirtueAndInnesHolmstromContractAudit, TensionSevenInstitutionalAgencyAudit,
    TriadicSafeguardComplementarityAudit,
};
pub use label_shift::{
    LabelShiftDiagnostic, adjust_posterior_for_label_shift, estimate_label_shift_four_methods,
    fit_calibrated_logistic_1d,
};
pub use pccp_plat::{
    EvaluatedCompetenceCriterion, PlatTrafficLightZone, PredeterminedChangeControlPlan,
    ProfitLossAttributionTest,
};
pub use statutory::{CloudLocalLoraArchitectureAudit, RegulatoryComplianceAudit};

#[cfg(test)]
mod tests {
    use super::*;
    use stochology_core::Rng;

    #[test]
    fn covariate_drift_extra_wcp_recovers_tilt_and_lei_candes_bound() {
        let mut rng = Rng::new(2026);
        let x_p: Vec<f64> = (0..500).map(|_| rng.normal(0.0, 1.0)).collect();
        let x_q: Vec<f64> = (0..500).map(|_| rng.normal(0.45, 1.0)).collect();
        let scores: Vec<f64> = x_p.iter().map(|&x| 0.2 + 0.1 * x.abs()).collect();
        let res = fit_covariate_drift_extra_wcp(&x_p, &x_q, &scores, None, 0.01, 0.10);
        assert!(res.estimated_beta > 0.25 && res.estimated_beta < 0.65);
        assert!(
            res.total_variation_distance_d_tv > 0.0 && res.total_variation_distance_d_tv < 0.15
        );
        assert!(res.lei_candes_coverage_lower_bound < res.nominal_level_one_minus_alpha);
        assert!(res.lei_candes_coverage_lower_bound > 0.75);
    }

    #[test]
    fn four_label_shift_estimators_converge_toward_new_target_prior() {
        // Source prior p1 = 0.10, TPR = 0.85, FPR = 0.10 -> target prior q1 = 0.35
        // Target predicted positive rate = 0.35 * 0.85 + 0.65 * 0.10 = 0.3625 (72 / 200)
        // Source-calibrated posteriors: P_s(y=1 | y_hat=1) = 0.4857, P_s(y=1 | y_hat=0) = 0.0182
        let mut probs = Vec::with_capacity(200);
        for i in 0..200 {
            if i < 72 {
                probs.push(0.4857);
            } else {
                probs.push(0.0182);
            }
        }
        let diag =
            estimate_label_shift_four_methods(0.10, 0.85, 0.10, &probs, 10.0, 30.0, 17.0, 0.0);
        assert!((diag.target_prior_bbse - 0.3467).abs() < 0.02);
        assert!(diag.target_prior_mlls_sld > 0.25 && diag.target_prior_mlls_sld < 0.45);
        assert!((diag.target_prior_dfm - 0.35).abs() < 1e-9);
        assert!((diag.target_prior_otter - 0.345).abs() < 0.05);
        assert!(diag.confusion_matrix_sigma_min > 0.01);
        assert!(diag.hoffman_constant_kappa > 1.0);
    }

    #[test]
    fn murphy_corp_circuit_breaker_separates_concept_drift_from_label_shift() {
        // Inverted predictions -> RES_CORP = 0 -> trips Concept Drift circuit breaker!
        let p_bad = [0.9, 0.85, 0.15, 0.1];
        let y = [0.0, 0.0, 1.0, 1.0];
        let m_bad = MurphyCorpCircuitBreaker::monitor(&p_bad, &y, 0.05, 0.05, false, false, false);
        assert!(m_bad.circuit_breaker_tripped);
        assert_eq!(
            m_bad.diagnosed_regime,
            DriftRegime::ConceptDriftCircuitBreaker
        );
    }

    #[test]
    fn decile_tv_detects_bimodal_shift_with_matched_moments_and_dfm_is_translation_invariant() {
        let mut rng = Rng::new(2026);
        let x_p: Vec<f64> = (0..400).map(|_| rng.normal(0.0, 1.0)).collect();
        // Symmetric bimodal target (+1 / -1) has mean ~ 0 and variance ~ 1, matching N(0, 1) first two moments
        let x_q_bimodal: Vec<f64> = (0..400)
            .map(|i| if i % 2 == 0 { -1.0 } else { 1.0 })
            .collect();
        let scores: Vec<f64> = x_p.iter().map(|&x| 0.2 + 0.1 * x.abs()).collect();
        let res = fit_covariate_drift_extra_wcp(&x_p, &x_q_bimodal, &scores, None, 0.01, 0.10);
        assert!(res.total_variation_distance_d_tv > 0.15);

        // DFM translation invariance under alpha0 > 0: shifting all feature means by +500 leaves q1_dfm unchanged
        let probs = [0.4857; 72];
        let d1 =
            estimate_label_shift_four_methods(0.10, 0.85, 0.10, &probs, 10.0, 30.0, 16.3, 0.10);
        let d2 =
            estimate_label_shift_four_methods(0.10, 0.85, 0.10, &probs, 510.0, 530.0, 516.3, 0.10);
        assert!((d1.target_prior_dfm - d2.target_prior_dfm).abs() < 1e-12);
        assert!((d1.target_prior_dfm - 0.35).abs() < 1e-9);

        // Online ACI adapts to residual inflation where static conformal coverage drops to 75%
        let cal_scores: Vec<f64> = (1..=200).map(|i| (i as f64) * 0.01).collect();
        let shifted_stream: Vec<f64> = (0..200).map(|i| 1.40 + ((i % 20) as f64) * 0.028).collect();
        let aci =
            AdaptiveConformalInference::evaluate_stream(&cal_scores, &shifted_stream, 0.10, 0.02);
        assert!(aci.empirical_coverage_aci > aci.empirical_coverage_static + 0.10);
        assert!((aci.empirical_coverage_aci - 0.90).abs() < 0.04);

        // PCCP & PLAT traffic-light verification
        let rtpl: Vec<f64> = (0..100).map(|i| (i as f64) * 0.5).collect();
        let hpl_green: Vec<f64> = (0..100).map(|i| (i as f64) * 0.5 + 0.2).collect();
        let hpl_red: Vec<f64> = (0..100).map(|i| ((99 - i) as f64) * 0.5 + 15.0).collect();

        let pccp_ok = PredeterminedChangeControlPlan::evaluate_update(
            "PCCP-2026-GREEN",
            0.25,
            0.08,
            1.4,
            0.05,
            &rtpl,
            &hpl_green,
            0.004,
            0.035,
            0.05,
            true,
        );
        assert_eq!(
            pccp_ok.plat_result.traffic_light_zone,
            PlatTrafficLightZone::Green
        );
        assert!(pccp_ok.update_approved && !pccp_ok.automatic_rollback_triggered);

        let pccp_red = PredeterminedChangeControlPlan::evaluate_update(
            "PCCP-2026-RED",
            0.25,
            0.08,
            25.0,
            0.05,
            &rtpl,
            &hpl_red,
            0.004,
            0.035,
            0.05,
            true,
        );
        assert_eq!(
            pccp_red.plat_result.traffic_light_zone,
            PlatTrafficLightZone::Red
        );
        assert!(pccp_red.automatic_rollback_triggered && !pccp_red.update_approved);

        let arch = CloudLocalLoraArchitectureAudit::canonical_benchmark();
        assert!(arch.architecture_and_lora_approved);
        assert!(arch.tbs_hybrid_total_cost < arch.pure_api_total_cost);
        assert!(arch.tbs_hybrid_total_cost < arch.overloaded_pure_local_total_cost);
        assert!(arch.lora_retained_energy_ratio > 0.98);
        assert!(arch.eckart_young_energy_identity_error < 1e-12);
    }
}
