//! The 8-Step SIGA Operational Roadmap (§3.2):
//! - **S — Survey the Layer and Bridge** (Steps 1 and 2);
//! - **I — Interrogate the Decision-Maker** (Steps 3 and 4);
//! - **G — Generate the Null and Noise Physics** (Steps 5 and 6);
//! - **A — Act with Threshold, Memory, and Exit** (Steps 7 and 8).
//!
//! Includes sequential anytime-valid *e-processes* ($E_t = \prod_{i=1}^t L_i$, $\mathbb{E}_{H_0}[E_t] \le 1$,
//! Ville's Inequality $E_t \ge 1/\alpha$) and Gigerenzer integer natural frequency memoranda (e.g., `"X out of 1,000"`).

pub mod alert_fatigue;
pub mod blueprints;
pub mod martingales;
pub mod runner;

pub use alert_fatigue::{AlertFatigueAudit, NaturalFrequencyMemorandum, OperatingPointSource};
pub use blueprints::{MultimodalCorrelatedFusionAudit, PreMortemCounterfactualRootCauseAudit};
pub use martingales::{EvidenceProcess, SaviConfidenceSequence, VarEsEBacktest};
pub use runner::{
    SigaExecutionReport, run_full_siga_roadmap, run_full_siga_roadmap_empirical,
    simulate_sbc_beta_binomial_ranks,
};

#[cfg(test)]
mod tests {
    use super::runner::compute_sbc_rank_bins;
    use super::*;
    use stochology_core::{EpistemologicalCard, Rng};

    #[test]
    fn eight_step_siga_roadmap_validates_end_to_end_and_prunes_defective_inputs() {
        let card = EpistemologicalCard::canonical_example();
        let plan = card.compute_preregistration_payload();
        let null_samples: Vec<f64> = (0..200).map(|i| (i as f64) * 0.01).collect();
        let report = run_full_siga_roadmap(&card, &plan, 5.0, &null_samples, &[24.8]);
        assert!(report.approved_end_to_end);
        assert_eq!(report.steps_1_to_8_completed, [true; 8]);
        assert!(
            report
                .natural_frequency_memo
                .formatted_summary
                .contains("1000")
        );
        assert!(
            report
                .natural_frequency_memo
                .formatted_summary
                .contains("p*=0.12000")
        );

        // Un-clamped low observed test statistic fails Step 5 (null_p >= 0.05)
        let weak_signal = run_full_siga_roadmap(&card, &plan, 0.50, &null_samples, &[24.8]);
        assert!(weak_signal.null_p_value >= 0.05);
        assert!(!weak_signal.steps_1_to_8_completed[4]);
        assert!(!weak_signal.approved_end_to_end);

        // Degenerate null samples fail Filter 3 (SBC rank uniformity)
        let bad_null = run_full_siga_roadmap(&card, &plan, 5.0, &[1.0; 100], &[24.8]);
        assert_eq!(bad_null.funnel_result.pruned_at_filter, Some(3));

        // Card conditioning on a Berkson collider fails Filter 2
        let mut collider_card = card.clone();
        collider_card
            .epistemological_header
            .causal_ladder
            .identification_strategy =
            "Markov blanket conditioned on hospital admission collider".into();
        let bad_collider =
            run_full_siga_roadmap(&collider_card, &plan, 5.0, &null_samples, &[24.8]);
        assert_eq!(bad_collider.funnel_result.pruned_at_filter, Some(2));

        // NaN e-value safely zeroes out EvidenceProcess without panicking
        let nan_e = EvidenceProcess::evaluate(&[24.8, f64::NAN], 0.05, 1);
        assert!(!nan_e.rejects_null_hypothesis);

        // GRAPA adaptive Kelly betting martingale rejects positive drift and stays <= threshold under null
        let pos_stream = vec![0.45; 35];
        let grapa = EvidenceProcess::from_grapa_betting(&pos_stream, 0.05, 1).unwrap();
        assert!(grapa.rejects_null_hypothesis);
    }

    #[test]
    fn skewed_sbc_samples_fail_filter3_while_genuine_talts_sbc_ranks_pass() {
        let card = EpistemologicalCard::canonical_example();
        let plan = card.compute_preregistration_payload();

        // Skewed non-uniform sample concentrates in lower quintile and fails Filter 3
        let skewed_samples: Vec<f64> = (0..200).map(|i| ((i as f64) / 200.0).powi(3)).collect();
        let skewed_bins = compute_sbc_rank_bins(&skewed_samples);
        assert!(crate::funnel::sbc_rank_chi2(&skewed_bins) > 9.49);
        let skewed_rep = run_full_siga_roadmap(&card, &plan, 5.0, &skewed_samples, &[24.8]);
        assert!(skewed_rep.funnel_result.sbc_chi2_statistic > 9.49);
        assert_eq!(skewed_rep.funnel_result.pruned_at_filter, Some(3));
        assert!(!skewed_rep.approved_end_to_end);

        // Genuine Talts et al. (2018) Beta-Binomial SBC ranks are uniform on [0, 1] and pass Filter 3
        let sbc_ranks = simulate_sbc_beta_binomial_ranks(2026, 200, 50, 2.0, 18.0, 100);
        let sbc_bins = compute_sbc_rank_bins(&sbc_ranks);
        assert!(crate::funnel::sbc_rank_chi2(&sbc_bins) <= 9.49);
        let valid_rep = run_full_siga_roadmap(&card, &plan, 5.0, &sbc_ranks, &[12.5, 2.2, 0.7]);
        assert!(valid_rep.funnel_result.filter_3_approved);
        assert!(valid_rep.e_process.rejects_null_hypothesis);
        assert!(valid_rep.approved_end_to_end);

        let (epic, stoch_gate) = AlertFatigueAudit::canonical_epic_sepsis_vs_stochological_gate();
        assert!(!epic.approved_bridge_l2_l3);
        assert!((epic.positive_predictive_value_ppv - 0.12).abs() < 0.01);
        assert!(epic.false_alarms_per_true_case > 7.0);
        assert!(stoch_gate.approved_bridge_l2_l3);
        assert!(stoch_gate.turing_weight_of_evidence_decibans > 12.0);
        assert!(stoch_gate.cognitive_triage_load_bps <= 10.0);
    }

    #[test]
    fn alert_fatigue_operating_points_declare_literature_vs_assumed_provenance() {
        let (epic, stoch_gate) = AlertFatigueAudit::canonical_epic_sepsis_vs_stochological_gate();
        assert_eq!(
            epic.operating_point_source,
            OperatingPointSource::Literature {
                source: "Wong et al., JAMA Internal Medicine 2021"
            }
        );
        assert_eq!(
            stoch_gate.operating_point_source,
            OperatingPointSource::Assumed
        );
        // Declaring provenance never alters a computed quantity.
        let bare = AlertFatigueAudit::evaluate(0.066, 0.33, 0.171, 0.20, 18.0, 42.0, false);
        assert_eq!(bare.operating_point_source, OperatingPointSource::Assumed);
        assert_eq!(
            bare.positive_predictive_value_ppv,
            epic.positive_predictive_value_ppv
        );
        assert_eq!(bare.approved_bridge_l2_l3, epic.approved_bridge_l2_l3);
    }

    #[test]
    fn savi_cs_var_es_e_backtest_and_grapa_ville_safety_reproduce_tri_model_vectors() {
        let savi = SaviConfidenceSequence::canonical_weyl_golden_benchmark();
        assert!((savi.raw_lambda_prpl_first_3[0] - 6.524984656).abs() < 1e-6);
        assert!((savi.raw_lambda_prpl_first_3[1] - 5.147135238).abs() < 1e-6);
        assert!((savi.raw_lambda_prpl_first_3[2] - 4.213005484).abs() < 1e-6);
        assert!((savi.agrapa_wealth_at_m0 - 38.37803479).abs() < 1e-6);
        assert!((savi.prpl_hedged_wealth_at_m0 - 25.24363875).abs() < 1e-6);
        assert!((savi.cs_lower_bound - 0.30738932).abs() < 1e-6);
        assert!((savi.cs_upper_bound - 0.67446618).abs() < 1e-6);
        assert!(savi.rejects_candidate_m0);

        let (z_peek_fpr, savi_fpr) =
            SaviConfidenceSequence::compare_peeking_vs_savi(2026, 200, 0.05);
        assert!(z_peek_fpr > 0.30);
        assert!(savi_fpr <= 0.05);

        let gro = EvidenceProcess::gro_betting(&[0.85; 25], 0.30, 0.80, 0.05, 1);
        assert!(gro.rejects_null_hypothesis);

        // Verify Ville's inequality holds on asymmetric bounded zero-mean null: P(X = 0.8) = 0.2, P(X = -0.2) = 0.8
        let mut rng = Rng::new(2026);
        let mut false_alarms = 0usize;
        for _ in 0..400 {
            let stream: Vec<f64> = (0..200)
                .map(|_| if rng.bernoulli(0.20) { 0.80 } else { -0.20 })
                .collect();
            if EvidenceProcess::from_grapa_betting(&stream, 0.05, 1)
                .unwrap()
                .rejects_null_hypothesis
            {
                false_alarms += 1;
            }
        }
        assert!((false_alarms as f64) / 400.0 <= 0.05);

        // Verify VaR & ES GREE e-backtest exact vector M_8 = 9.798157946
        let backtest = VarEsEBacktest::from_var_exceedances(0.90, &[0, 1, 0, 0, 1, 1, 0, 1]);
        assert!((backtest.final_martingale_m_t - 9.798157946).abs() < 1e-8);
        assert!((VarEsEBacktest::es_e_statistic(3.5, 2.5, 1.5, 0.90) - 20.0).abs() < 1e-12);
        assert!(VarEsEBacktest::es_e_statistic(3.5, 1.5, 1.5, 0.90).is_infinite());
        assert!((VarEsEBacktest::es_e_statistic(1.0, 1.5, 1.5, 0.90) - 1.0).abs() < 1e-12);
    }

    #[test]
    fn grapa_fails_closed_on_out_of_support_payoffs() {
        let mut strong = vec![0.5; 100];
        assert!(
            EvidenceProcess::from_grapa_betting(&strong, 0.05, 1)
                .unwrap()
                .rejects_null_hypothesis
        );
        strong[50] = 1.01;
        assert!(EvidenceProcess::from_grapa_betting(&strong, 0.05, 1).is_err());
        assert!(EvidenceProcess::from_grapa_betting(&[0.1, f64::NAN], 0.05, 1).is_err());
    }

    #[test]
    fn multimodal_five_senses_and_premortem_pns_sop_verify_operational_resilience() {
        let mm = MultimodalCorrelatedFusionAudit::canonical_five_senses_benchmark();
        assert!(mm.multimodal_fusion_approved);
        assert!((mm.multimodal_deff - 2.80).abs() < 1e-12);
        assert!((mm.naive_independent_decibans - 53.0).abs() < 1e-12);
        assert!((mm.shielded_correlated_decibans - 24.0 / 2.80).abs() < 1e-12);
        assert!(mm.zero_correlation_identity_error < 1e-12);

        let pm = PreMortemCounterfactualRootCauseAudit::canonical_benchmark();
        assert!(pm.premortem_and_sop_approved);
        assert!((pm.pns_lower_bound - 0.72).abs() < 1e-12);
        assert!((pm.pns_upper_bound - 0.84).abs() < 1e-12);
        assert!(pm.tian_pearl_bounds_identity_error < 1e-12);
        assert!(pm.sop_cognitive_load_bps <= 10.0);
    }
}
