//! The 4 Eliminatory Filters (§3.1): Sequential Pruning Before Inference.
//!
//! - **Filter 1 (Decision & Resolution):** $p^* = \frac{C}{\eta L} \in (0, 1)$, $\mathrm{EVPI} > C_{\text{total}}$, and Invariant $\Phi_5$.
//! - **Filter 2 (Effective Sample Size & Causal Specification):** Moulton-Campbell $n_{\mathrm{eff}}(p) \ge n_{\min}$,
//!   declarative Markov Blanket specification check, and ban on conditioning on colliders (Berkson's paradox).
//! - **Filter 3 (Generative Physics & Prior Predictive Check):** Simulation-Based Calibration (SBC $\chi^2$
//!   rank uniformity) and declarative SHELF quartile domain coherence check.
//! - **Filter 4 (Cryptographic Pre-Registration & Cross-Authorship):** SHA-256 seal, independent
//!   cross-authorship (`null_model != decision_rule != adversarial_reviewer`), and numeric exit clause.

pub mod evaluator;
pub mod paradoxes;
pub mod ppi;
pub mod priors;

pub use evaluator::{evaluate_four_filter_funnel, FunnelCandidate, FunnelResult};
pub use paradoxes::{LongContextDpiDilutionAudit, SimpsonCausalParadoxAudit};
pub use ppi::{
    ppi_effective_sample_size_multiplier, prospective_clustered_mde,
    prospective_ppi_gold_sample_size,
};
pub use priors::{
    fit_shelf_beta_prior, ln_gamma_lanczos, regularized_incomplete_beta, sbc_rank_chi2,
};

#[cfg(test)]
mod tests {
    use super::*;
    use stochology_core::sha256_hex;

    fn valid_candidate() -> FunnelCandidate {
        let plan = "siga_plan_coastal_evacuation_shift_seed_2026";
        FunnelCandidate {
            who_decides: "Coastal Emergency Operations Director".into(),
            altered_action: "Trigger mandatory coastal evacuation order within 6h synoptic shift"
                .into(),
            independent_unit: "6h synoptic storm shift".into(),
            prior_probability_p: 0.05,
            protection_cost_c: 1_800.0,
            event_loss_l: 34_000.0,
            efficacy_eta: 1.0,
            decision_volume_units: 100.0,
            total_modeling_cost: 15_000.0,
            effective_tail_n_eff_p: 1_300.0,
            minimum_required_n_eff: 100.0,
            markov_blanket_mapped: true,
            conditions_on_collider_berkson: false,
            sbc_rank_histogram: vec![20, 20, 20, 20, 20],
            sbc_chi2_critical: 9.49,
            shelf_quartiles_q25_q50_q75: (0.02, 0.05, 0.09),
            physical_domain_bounds: (0.001, 0.999),
            preregistered_plan_text: plan.into(),
            declared_sha256_seal: sha256_hex(plan.as_bytes()),
            author_null_model: "inference".into(),
            author_decision_rule: "decision".into(),
            author_adversarial_reviewer: "independent_auditor".into(),
            numeric_exit_clause: "Suspend if REL_CORP > 0.015 over 14 days".into(),
        }
    }

    #[test]
    fn sound_candidate_passes_all_four_filters() {
        let c = valid_candidate();
        let res = evaluate_four_filter_funnel(&c);
        assert!(res.approved_all_filters);
        assert_eq!(res.pruned_at_filter, None);

        let (a_hat, b_hat, rmse) = fit_shelf_beta_prior(0.02, 0.05, 0.09);
        assert!(a_hat > 0.5 && b_hat > 5.0 && rmse < 0.03);
    }

    #[test]
    fn funnel_prunes_berkson_collider_at_filter2_and_same_author_at_filter4() {
        let mut c2 = valid_candidate();
        c2.conditions_on_collider_berkson = true;
        let r2 = evaluate_four_filter_funnel(&c2);
        assert_eq!(r2.pruned_at_filter, Some(2));

        let mut c4 = valid_candidate();
        c4.author_adversarial_reviewer = "inference".into();
        let r4 = evaluate_four_filter_funnel(&c4);
        assert_eq!(r4.pruned_at_filter, Some(4));

        let mut c4_no_digit = valid_candidate();
        c4_no_digit.numeric_exit_clause = "Suspend model when calibration degrades".into();
        let r4_no_digit = evaluate_four_filter_funnel(&c4_no_digit);
        assert_eq!(r4_no_digit.pruned_at_filter, Some(4));

        let mut c1_bad_pstar = valid_candidate();
        c1_bad_pstar.protection_cost_c = 40_000.0; // C >= eta * L -> p* >= 1.0
        let r1_bad_pstar = evaluate_four_filter_funnel(&c1_bad_pstar);
        assert_eq!(r1_bad_pstar.pruned_at_filter, Some(1));

        // Out-of-domain cost/loss/eta/NaN values short-circuit cleanly at Filter 1 without panicking
        let mut c1_bad_eta = valid_candidate();
        c1_bad_eta.efficacy_eta = 1.5;
        assert_eq!(
            evaluate_four_filter_funnel(&c1_bad_eta).pruned_at_filter,
            Some(1)
        );

        let mut c1_nonpos_cost = valid_candidate();
        c1_nonpos_cost.protection_cost_c = 0.0;
        assert_eq!(
            evaluate_four_filter_funnel(&c1_nonpos_cost).pruned_at_filter,
            Some(1)
        );

        let mut c1_nan_loss = valid_candidate();
        c1_nan_loss.event_loss_l = f64::NAN;
        assert_eq!(
            evaluate_four_filter_funnel(&c1_nan_loss).pruned_at_filter,
            Some(1)
        );
    }

    #[test]
    fn prospective_ppi_gold_sizing_and_ess_multiplier_resolve_finite_sample_refutation() {
        // At K = 5, n_gold = 160 with var_rectifier_diff ~ 0.22 has 95% half-width ~ 0.073 (> 0.06).
        // Prospective sizing computes the exact gold sample size needed to guarantee half-width <= 0.06:
        let n_star = prospective_ppi_gold_sample_size(0.06, 1.959_964, 2_000, 0.21, 0.22);
        assert!(n_star > 160 && n_star < 400);
        let ess_mult = ppi_effective_sample_size_multiplier(0.24, 0.21, 0.11, 2_000, n_star);
        assert!(ess_mult > 1.5);
    }

    #[test]
    fn simpson_paradox_and_long_context_dpi_dilution_verify_filter2_causal_pruning() {
        let simpson = SimpsonCausalParadoxAudit::canonical_pearl_benchmark();
        assert!(simpson.simpson_reversal_and_dag_dichotomy_verified);
        assert!(simpson.stratum_z0_diff > 0.05 && simpson.stratum_z1_diff > 0.04);
        assert!(simpson.pooled_diff < -0.04);
        assert!(simpson.confounder_do_ate > 0.04);
        assert!(simpson.mediator_do_ate < -0.04);
        assert!(simpson.total_probability_identity_error < 1e-12);

        let ctx = LongContextDpiDilutionAudit::evaluate(100, 3, 1, 2.2, 0.9);
        assert!(ctx.markov_blanket_pruning_approved);
        assert!(ctx.raw_blanket_attention_mass < 0.12);
        assert!(ctx.pruned_blanket_attention_mass > 0.95);
        assert!(ctx.snr_gain_decibans > 20.0);
        assert!(ctx.softmax_partition_identity_error < 1e-12);
    }
}
