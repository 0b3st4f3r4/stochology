//! Negative defect proof evaluators (`gates --defect <name>`, all 21 defects across the 6 CAPIBA pillars).

use std::path::Path;
use stochology_cli::{
    evaluate_empirical_card_severe_test, load_atlantic_storms_decade_environments,
    load_atlantic_storms_forecasts, load_bank_call_clusters, load_cards_from_disk,
    load_lalonde_nsw_cps, load_oot_ood_flight_reserves, load_toronto_arrests_prevalences,
};
use stochology_core::{
    ChouldechovaAudit, DEFAULT_SEED, ExceedanceDesignEffect, InfoGapOption, ManskiBounds,
    PerformativeSystem, Rng, verify_resolution_phi5,
};
use stochology_governance::{
    BitemporalRecord, MurphyCorpCircuitBreaker, TemporalVault, query_with_bitemporal_clock,
};
use stochology_lenses::acceptance::{ACCEPTANCE_SEEDS, SimulatedSynthesis};
use stochology_lenses::analog::{
    AnalogWaveComputingSynthesis, GerstgrasserAccumulateAudit, SyntheticDataPolicy,
};
use stochology_lenses::critical::CriticalStochasticRationalitySynthesis;
use stochology_lenses::decision::{MurphyDiagram, brier_score};
use stochology_lenses::machine::{AlmostPerfectMachine, AuthenticModelForm};
use stochology_lenses::society::CoupledPublicPrivateSocietySynthesis;
use stochology_lenses::stocks::StocharyConjecturalCyberneticSynthesis;
use stochology_lenses::strategic::StrategicAiAccommodationSynthesis;
use stochology_lenses::triad::ComplexFractalToroidalTechniumSynthesis;
use stochology_siga::{SaviConfidenceSequence, VarEsEBacktest};

/// Executes a negative defect proof (`--defect <name>`) and exits with code `1` when blocked.
pub fn run_negative_proof(root: &Path, defect: &str) -> ! {
    let cards = load_cards_from_disk(root).expect("load canonical YAML cards from disk");
    match defect {
        "cromwell" => {
            let mut c = cards[0].1.clone();
            c.decision_rule_and_scope.event_probability_p = 0.0;
            if let Err(v) = c.audit() {
                if evaluate_empirical_card_severe_test(root, &c).is_err() {
                    eprintln!("NEGATIVE PROOF [cromwell] BLOCKED SUCCESSFULLY: {v:?}");
                    std::process::exit(1);
                }
            }
        }
        "phi5" => {
            let mut c = cards[1].1.clone();
            c.decision_maker_question.altered_action =
                "Set a single annual aggregate average for the entire year".into();
            if !verify_resolution_phi5(
                &c.epistemological_header.unit_and_margin.independent_unit,
                &c.decision_maker_question.altered_action,
            ) && c.audit().is_err()
                && evaluate_empirical_card_severe_test(root, &c).is_err()
            {
                eprintln!(
                    "NEGATIVE PROOF [phi5] BLOCKED SUCCESSFULLY: coarse annual aggregate action against daily unit"
                );
                std::process::exit(1);
            }
        }
        "moulton" => {
            let mut c = cards[1].1.clone();
            c.epistemological_header
                .unit_and_margin
                .effective_sample_size_n_eff = c
                .epistemological_header
                .unit_and_margin
                .nominal_sample_size_n;
            let bank_clusters = load_bank_call_clusters(root);
            let b = ExceedanceDesignEffect::from_clustered_observations(&bank_clusters, 0.90);
            if b.n_effective_p < (b.n_nominal as f64) * 0.10
                && c.audit().is_err()
                && evaluate_empirical_card_severe_test(root, &c).is_err()
            {
                eprintln!(
                    "NEGATIVE PROOF [moulton] BLOCKED SUCCESSFULLY: nominal pseudoreplication n={} reduced to n_eff={:.0}",
                    b.n_nominal, b.n_effective_p
                );
                std::process::exit(1);
            }
        }
        "performativity" => {
            let mut c = cards[2].1.clone();
            c.epistemological_header
                .noise_physics
                .bridge_l3_l1_performativity
                .rrm_convergence_condition = "eps * beta / gamma = 0.20 * 6.0 / 1.0 = 1.20".into();
            let unstable = PerformativeSystem::new(1.0, -1.2, 0.5, 1.0, 1.0).diagnose();
            if !unstable.rrm_converges && c.audit().is_err() {
                eprintln!(
                    "NEGATIVE PROOF [performativity] BLOCKED SUCCESSFULLY: RRM radius {:.2} >= 1 diverges/oscillates",
                    unstable.rrm_contraction_radius
                );
                std::process::exit(1);
            }
        }
        "sniedovich" => {
            let mut c = cards[0].1.clone();
            c.uncertainty_budget_and_authorship
                .info_gap_robustness
                .global_maximin_safeguard =
                "Wald global Maximin bound = -999 in No-Man's Land with irreversible collapse"
                    .into();
            let turkey = InfoGapOption {
                id: "christmas_turkey",
                nominal_performance: 100.0,
                adverse_sensitivity: 25.0,
                favorable_propensity: 20.0,
                wald_global_maximin_bound: -500.0,
                reversible: false,
            }
            .evaluate(60.0, 120.0, 20.0);
            if !turkey.approved_sniedovich && c.audit().is_err() {
                eprintln!(
                    "NEGATIVE PROOF [sniedovich] BLOCKED SUCCESSFULLY: collapse in No-Man's Land (Wald={})",
                    turkey.wald_global_maximin_bound
                );
                std::process::exit(1);
            }
        }
        "clock" => {
            let rec = [BitemporalRecord {
                row_id: 1,
                t_event: 10,
                t_availability: 15,
                feature_x: 1,
                outcome_y: 1,
                canary_column: 1,
                t_availability_canary: 999,
            }];
            let mutated = query_with_bitemporal_clock(&rec, 50, false);
            if mutated.temporal_intrusion_alarm {
                eprintln!(
                    "NEGATIVE PROOF [clock] BLOCKED SUCCESSFULLY: canary triggered temporal intrusion alarm"
                );
                std::process::exit(1);
            }
        }
        "vault" => {
            let (_, _, _, y23_h1, _, _) = load_oot_ood_flight_reserves(root);
            let preds = vec![0.10; y23_h1.len()];
            let mut vault = TemporalVault::seal("violated_h1_vault", y23_h1);
            let _ = vault.open_once(&preds, 0.20);
            if let Err(e) = vault.open_once(&preds, 0.20) {
                eprintln!("NEGATIVE PROOF [vault] BLOCKED SUCCESSFULLY: reopening refused ({e:?})");
                std::process::exit(1);
            }
        }
        "concept_drift" => {
            let (storm_probs, storm_outcomes) = load_atlantic_storms_forecasts(root);
            let inverted_probs: Vec<f64> = storm_probs.iter().map(|&p| 1.0 - p).collect();
            let m = MurphyCorpCircuitBreaker::monitor(
                &inverted_probs,
                &storm_outcomes,
                0.05,
                0.05,
                false,
                false,
                false,
            );
            if m.circuit_breaker_tripped {
                eprintln!(
                    "NEGATIVE PROOF [concept_drift] BLOCKED SUCCESSFULLY: {}",
                    m.recommended_corrective_action
                );
                std::process::exit(1);
            }
        }
        "chouldechova" => {
            let mut c = cards[0].1.clone();
            c.decision_rule_and_scope.who_pays_for_error =
                "Chouldechova trilemma audit: equalizing PPV and FPR with FPR_A/FPR_B = 1.000 equally fair".into();
            let (p_a, p_b) = load_toronto_arrests_prevalences(root);
            let (_, _, ratio) =
                ChouldechovaAudit::fpr_ratio_under_equal_ppv_and_fnr(p_a, p_b, 0.65, 0.25);
            if (ratio - 1.0).abs() > 1e-6 && c.audit().is_err() {
                eprintln!(
                    "NEGATIVE PROOF [chouldechova] BLOCKED SUCCESSFULLY: algebraic impossibility FPR_A/FPR_B = {ratio:.4} != 1"
                );
                std::process::exit(1);
            }
        }
        "machine" => {
            let lossy_machine =
                AlmostPerfectMachine::evaluate_with_config(DEFAULT_SEED, 0.35, 2, 6, 0.12, 0.05);
            let storm_envs = load_atlantic_storms_decade_environments(root);
            let silent_drop_form = AuthenticModelForm::audit_environments(
                DEFAULT_SEED,
                &storm_envs,
                &[
                    "atom_0_barometric_pressure_drop".to_string(),
                    "atom_1_regime_shifted_lat_proxy".to_string(),
                ],
                0.15,
                false,
            );
            if !lossy_machine.all_p1_to_p5_approved && !silent_drop_form.approved_authentic_form {
                eprintln!(
                    "NEGATIVE PROOF [machine] BLOCKED SUCCESSFULLY: DPI gap {:.4} > delta=0.05 (P2={}) and silently dropped spurious atoms rejected (findings_closed={})",
                    lossy_machine.dpi_gap_delta_nats,
                    lossy_machine.p2_informational_sufficiency,
                    silent_drop_form.identities_audit.findings_accounting_closed
                );
                std::process::exit(1);
            }
        }
        "analog_wall" => {
            let unshielded_analog =
                AnalogWaveComputingSynthesis::evaluate_with_mode(DEFAULT_SEED, false);
            let fails_all_dev_seeds = ACCEPTANCE_SEEDS.iter().all(|&s| {
                let u = AnalogWaveComputingSynthesis::evaluate_with_mode(s, false);
                !u.all_five_axes_verified
                    && !u.von_neumann_and_collapse.axis1_verified
                    && !u.aimc_analog_wall_hp_inv.axis3_verified
            });
            if fails_all_dev_seeds
                && !unshielded_analog.all_five_axes_verified
                && !unshielded_analog.von_neumann_and_collapse.axis1_verified
                && !unshielded_analog.aimc_analog_wall_hp_inv.axis3_verified
            {
                eprintln!(
                    "NEGATIVE PROOF [analog_wall] BLOCKED SUCCESSFULLY: unshielded analog accuracy at sigma=0.60 collapsed to {:.1}%, 1-step LP-INV relative error {:.4} > 1e-7 without HP-INV refinement, and unanchored synthetic variance retention collapsed to {:.2}",
                    unshielded_analog
                        .aimc_analog_wall_hp_inv
                        .nat_accuracy_at_sigma_060
                        * 100.0,
                    unshielded_analog
                        .aimc_analog_wall_hp_inv
                        .pku_hp_inv_final_relative_error,
                    unshielded_analog
                        .von_neumann_and_collapse
                        .neurosymbolic_anchored_variance_retention
                );
                std::process::exit(1);
            }
        }
        "instrumental_reason" => {
            let reified =
                CriticalStochasticRationalitySynthesis::evaluate_with_mode(DEFAULT_SEED, false);
            if CriticalStochasticRationalitySynthesis::verify_negative_defect_all_dev_seeds()
                && reified.all_identities_closed
                && !reified.all_four_axes_verified
                && !reified.adorno_non_identical_and_reification.axis1_verified
                && !reified.marcuse_rouvroy_stochastic_parrot.axis2_verified
                && !reified.dialectical_overcoming.axis4_verified
            {
                eprintln!(
                    "NEGATIVE PROOF [instrumental_reason] BLOCKED SUCCESSFULLY on all {} development seeds through seed-dependent claims (identities closed on every seed): at seed 2026 truncated-Gaussian VaR99 breached on {:.1}% of fresh draws, RRM excess performative risk={:.3}, Stochastic Parrot counterfactual MSE={:.2}, unconstrained recursion rule satisfaction={:.0}%, raw drifted readout accuracy={:.0}%, one-shot HP-INV error={:.1e}",
                    ACCEPTANCE_SEEDS.len(),
                    reified
                        .adorno_non_identical_and_reification
                        .deployed_var99_holdout_exceedance
                        * 100.0,
                    reified
                        .adorno_non_identical_and_reification
                        .deployed_performative_excess_risk,
                    reified
                        .marcuse_rouvroy_stochastic_parrot
                        .deployed_counterfactual_mse,
                    reified
                        .dialectical_overcoming
                        .deployed_rule_satisfaction_rate
                        * 100.0,
                    reified.dialectical_overcoming.a_ecc_drift_recovery_accuracy * 100.0,
                    reified
                        .dialectical_overcoming
                        .pku_40nm_rram_24bit_hp_inv_error
                );
                std::process::exit(1);
            }
        }
        "stochary_cybernetics" => {
            let unshielded =
                StocharyConjecturalCyberneticSynthesis::evaluate_with_mode(DEFAULT_SEED, false);
            if StocharyConjecturalCyberneticSynthesis::verify_negative_defect_all_dev_seeds()
                && unshielded.all_identities_closed
                && !unshielded.all_four_axes_verified
                && !unshielded.topology_and_stock_axioms.axis1_verified
                && !unshielded.powell_friston_causal_engine.axis2_verified
                && !unshielded.strategic_stock_plant.axis3_verified
                && !unshielded.cybernetic_vsm_and_pate.axis4_verified
            {
                eprintln!(
                    "NEGATIVE PROOF [stochary_cybernetics] BLOCKED SUCCESSFULLY on all {} development seeds through seed-dependent claims (identities closed on every seed): at seed 2026 panic-order Bullwhip={:.2}x, full-exposure Peters growth={:+.4}, always-act VOI match={}, Rung-1 effect={:.2} (true 2.80), untransported effect error={:.2}, simulated queue wait at u=0.985={:.1} h, Ivanov fill rate={:.2}, TTS/TTR={:.2}, uncertified WDRO disappointment={:+.2}, persistence-model R2={:.2}, raw judge noise={:.3}, factor-omitting PLAT r_s={:.2}",
                    ACCEPTANCE_SEEDS.len(),
                    unshielded
                        .topology_and_stock_axioms
                        .low_pass_buffered_variance_ratio,
                    unshielded
                        .topology_and_stock_axioms
                        .ergodic_buffered_time_growth_rate,
                    unshielded
                        .powell_friston_causal_engine
                        .deployed_policy_choices_match_voi,
                    unshielded
                        .powell_friston_causal_engine
                        .pearl_front_door_identified_effect,
                    unshielded
                        .powell_friston_causal_engine
                        .pearl_transportability_error,
                    unshielded
                        .strategic_stock_plant
                        .supply_chain_physics
                        .kingman_vut_buffered_vs_cliff_wait_hours
                        .0,
                    unshielded
                        .strategic_stock_plant
                        .supply_chain_physics
                        .ivanov_ripple_effect_viability_service_rate,
                    unshielded
                        .strategic_stock_plant
                        .simchi_levi_tts_to_ttr_ratio,
                    unshielded
                        .strategic_stock_plant
                        .wdro_robust_oos_disappointment,
                    unshielded
                        .cybernetic_vsm_and_pate
                        .conant_ashby_regulator_homomorphism_r2,
                    unshielded
                        .cybernetic_vsm_and_pate
                        .kahneman_judgment_noise_before_and_after
                        .1,
                    unshielded.cybernetic_vsm_and_pate.plat_spearman_and_ks.0
                );
                std::process::exit(1);
            }
        }
        "triadic_technium" => {
            let unshielded_triad =
                ComplexFractalToroidalTechniumSynthesis::evaluate_with_mode(DEFAULT_SEED, false);
            if ComplexFractalToroidalTechniumSynthesis::verify_negative_defect_all_dev_seeds()
                && unshielded_triad.all_identities_closed
                && !unshielded_triad.all_six_bridges_verified
                && !unshielded_triad.complex_wave_gabor_helmholtz.axis1_verified
                && !unshielded_triad
                    .complex_real_isomorphism_hmatrix
                    .axis2_verified
                && !unshielded_triad.fractional_ctrw_analog_ecc.axis3_verified
                && !unshielded_triad
                    .multifractal_pdhte_rag_collapse
                    .axis4_verified
                && !unshielded_triad
                    .weyl_torus_hermite_mehler_dfm
                    .axis5_verified
                && !unshielded_triad
                    .sna_rlhf_fhrr_cohomology_technium
                    .axis6_verified
            {
                eprintln!(
                    "NEGATIVE PROOF [triadic_technium] BLOCKED SUCCESSFULLY on all {} development seeds through seed-dependent claims (identities closed on every seed): at seed 2026 tanh Helmholtz residual={:.3}, one-shot BlockAMC residual={:.1e}, naive H-matrix err={:.2e}, uncompensated PCM drift MSE={:.3}, Fickian CTRW err={:.1}%, plain-LS ECC acc={:.2}, OLS ATE RMSE={:.3}, rational Weyl D*={:.3}, random-Fourier DFM err={:.3}, classify-and-count KL={:.4}, RRM gap to theta_PO={:.3}",
                    ACCEPTANCE_SEEDS.len(),
                    unshielded_triad
                        .complex_wave_gabor_helmholtz
                        .deployed_helmholtz_residual_rel_mse,
                    unshielded_triad
                        .complex_real_isomorphism_hmatrix
                        .two_stage_block_amc_16x16_relative_error,
                    unshielded_triad
                        .complex_real_isomorphism_hmatrix
                        .hmatrix_deployed_relative_error,
                    unshielded_triad
                        .fractional_ctrw_analog_ecc
                        .deployed_pcm_drift_mse,
                    unshielded_triad
                        .fractional_ctrw_analog_ecc
                        .deployed_ctrw_spread_prediction_rel_error
                        * 100.0,
                    unshielded_triad
                        .fractional_ctrw_analog_ecc
                        .analog_ecc_ume_recovery_accuracy,
                    unshielded_triad
                        .multifractal_pdhte_rag_collapse
                        .deployed_ate_rmse,
                    unshielded_triad
                        .weyl_torus_hermite_mehler_dfm
                        .deployed_weyl_discrepancy,
                    unshielded_triad
                        .weyl_torus_hermite_mehler_dfm
                        .deployed_dfm_l2_error,
                    unshielded_triad
                        .weyl_torus_hermite_mehler_dfm
                        .deployed_label_shift_kl,
                    unshielded_triad
                        .sna_rlhf_fhrr_cohomology_technium
                        .deployed_performative_gap_to_opt
                );
                std::process::exit(1);
            }
        }
        "peeking" => {
            let mut c = cards[0].1.clone();
            c.data_and_audit_contract.e_process_multiplicity =
                "Optional stopping p-value peeking with z-test E_t = 0.85 < 20.0".into();
            let (naive_peek_fpr, savi_fpr) =
                SaviConfidenceSequence::compare_peeking_vs_savi(DEFAULT_SEED, 200, 0.05);
            if naive_peek_fpr > 0.30 && savi_fpr <= 0.05 && c.audit().is_err() {
                eprintln!(
                    "NEGATIVE PROOF [peeking] BLOCKED SUCCESSFULLY: optional stopping z-peeking inflated Type-I error to {naive_peek_fpr:.3} >> 0.05 (SAVI martingale FPR={savi_fpr:.3})"
                );
                std::process::exit(1);
            }
        }
        "brier_dominance" => {
            let mut c = cards[0].1.clone();
            c.data_and_audit_contract
                .murphy_corp_decomposition
                .miscalibration_rel_mcb += 0.15;
            let f_a = [0.35, 0.35, 0.90, 0.90];
            let f_b = [0.10, 0.10, 0.55, 0.55];
            let outs = [0.0, 0.0, 1.0, 1.0];
            let bs_a = brier_score(&f_a, &outs);
            let bs_b = brier_score(&f_b, &outs);
            let (dom, max_excess, worst_theta) =
                MurphyDiagram::check_uniform_dominance(&f_a, &f_b, &outs);
            if bs_a < bs_b && !dom && max_excess > 0.05 && c.audit().is_err() {
                eprintln!(
                    "NEGATIVE PROOF [brier_dominance] BLOCKED SUCCESSFULLY: forecaster A (BS={bs_a:.4} < BS_B={bs_b:.4}) fails Murphy uniform dominance at theta={worst_theta:.3} (excess={max_excess:.4})"
                );
                std::process::exit(1);
            }
        }
        "var_underreport" => {
            let mut rng = Rng::new(DEFAULT_SEED);
            let var_99_true = 2.326_347_874_040_840_8;
            let var_99_under = 0.80 * var_99_true;
            let mut exc_true = Vec::with_capacity(500);
            let mut exc_under = Vec::with_capacity(500);
            for _ in 0..500 {
                let loss = rng.standard_normal();
                exc_true.push(u8::from(loss > var_99_true));
                exc_under.push(u8::from(loss > var_99_under));
            }
            let bt_true = VarEsEBacktest::from_var_exceedances(0.99, &exc_true);
            let bt_under = VarEsEBacktest::from_var_exceedances(0.99, &exc_under);
            if !bt_true.rejects_at_alpha_005 && bt_under.rejects_at_alpha_005 {
                eprintln!(
                    "NEGATIVE PROOF [var_underreport] BLOCKED SUCCESSFULLY: 20%-underreported VaR_0.99 triggered GREE e-backtest alarm M_max={:.2} >= 20.0 (honest VaR M_max={:.2})",
                    bt_under.max_martingale_m_t, bt_true.max_martingale_m_t
                );
                std::process::exit(1);
            }
        }
        "lee_point_estimate" => {
            let mut c = cards[3].1.clone();
            c.data_and_audit_contract.missingness_mechanism =
                "MNAR naive point estimate +1340.80 only".into();
            let (nsw_t, nsw_c, _) = load_lalonde_nsw_cps(root);
            let t_pos: Vec<f64> = nsw_t.iter().copied().filter(|&y| y > 0.0).collect();
            let c_pos: Vec<f64> = nsw_c.iter().copied().filter(|&y| y > 0.0).collect();
            let (lee_lo, lee_hi, q_trim) =
                ManskiBounds::lee_trimming_bounds(&t_pos, nsw_t.len(), &c_pos, nsw_c.len());
            let delta_star =
                ManskiBounds::lee_tipping_point_delta(&t_pos, nsw_t.len(), &c_pos, nsw_c.len());
            if lee_lo < 0.0 && lee_hi > 0.0 && delta_star < 0.0 && c.audit().is_err() {
                eprintln!(
                    "NEGATIVE PROOF [lee_point_estimate] BLOCKED SUCCESSFULLY: naive positive point estimate spans zero under fractional Lee bounds [{lee_lo:.2}, {lee_hi:.2}] (q={q_trim:.4}, tipping delta*={delta_star:.2})"
                );
                std::process::exit(1);
            }
        }
        "replace_collapse" => {
            let replace_only = GerstgrasserAccumulateAudit::canonical_benchmark(
                DEFAULT_SEED,
                SyntheticDataPolicy::Replace,
            );
            if replace_only.simulation_closure_error <= 0.10
                && !replace_only.declared_policy_bounded
            {
                eprintln!(
                    "NEGATIVE PROOF [replace_collapse] BLOCKED SUCCESSFULLY: simulated replacement risk E_replace={:.4} exceeds the Basel ceiling {:.4} ({:.2}x the simulated accumulate risk {:.4}; closure error {:.3})",
                    replace_only.simulated_replace_risk_n,
                    replace_only.asymptotic_basel_ceiling,
                    replace_only.replace_to_accumulate_ratio,
                    replace_only.simulated_accumulate_risk_n,
                    replace_only.simulation_closure_error
                );
                std::process::exit(1);
            }
        }
        "strategic_accommodation" => {
            let unshielded_strat =
                StrategicAiAccommodationSynthesis::evaluate_with_mode(DEFAULT_SEED, false);
            if StrategicAiAccommodationSynthesis::verify_negative_defect_all_dev_seeds()
                && unshielded_strat.all_identities_closed
                && !unshielded_strat.all_six_axes_verified
                && !unshielded_strat
                    .minsky_papert_society_of_mind
                    .axis1_verified
                && !unshielded_strat.shapley_rubin_mi_doe.axis2_verified
                && !unshielded_strat
                    .generative_rag_llm_judge_compression
                    .axis3_verified
                && !unshielded_strat
                    .complex_network_hub_contagion
                    .axis4_verified
                && !unshielded_strat
                    .multi_criteria_ahp_regulatory_pkm
                    .axis5_verified
                && !unshielded_strat
                    .robust_kalman_closed_loop_control
                    .axis6_verified
            {
                eprintln!(
                    "NEGATIVE PROOF [strategic_accommodation] BLOCKED SUCCESSFULLY on all {} development seeds through seed-dependent claims (identities closed on every seed): at seed 2026 diameter-limited connectivity acc={:.2}, homogeneous voter Brier={:.3}, Rung-1 symptom SHAP do-MSE={:.2}, single-mean imputation var retention={:.2}, naive i.i.d. clustered power={:.2}, uncalibrated LLM-Judge err={:.3}, clipped INT4 ES99 retention={:.2}, unbuffered hub survival={:.2}, rank-reversed AHP portfolio val={:.2}, L2 Kalman glitch RMSE={:.2}, certainty-equivalent MPC survival={:.2}",
                    ACCEPTANCE_SEEDS.len(),
                    unshielded_strat
                        .minsky_papert_society_of_mind
                        .deployed_connectivity_accuracy,
                    unshielded_strat
                        .minsky_papert_society_of_mind
                        .deployed_ensemble_brier,
                    unshielded_strat
                        .shapley_rubin_mi_doe
                        .deployed_shap_intervention_mse,
                    unshielded_strat
                        .shapley_rubin_mi_doe
                        .deployed_imputation_variance_retention,
                    unshielded_strat
                        .shapley_rubin_mi_doe
                        .deployed_clustered_doe_power,
                    unshielded_strat
                        .generative_rag_llm_judge_compression
                        .deployed_llm_judge_abs_error,
                    unshielded_strat
                        .generative_rag_llm_judge_compression
                        .deployed_quantization_es99_retention,
                    unshielded_strat
                        .complex_network_hub_contagion
                        .deployed_airspace_survival,
                    unshielded_strat
                        .multi_criteria_ahp_regulatory_pkm
                        .deployed_portfolio_realized_value,
                    unshielded_strat
                        .robust_kalman_closed_loop_control
                        .deployed_kalman_state_rmse,
                    unshielded_strat
                        .robust_kalman_closed_loop_control
                        .deployed_lap_survival_rate
                );
                std::process::exit(1);
            }
        }
        "coupled_society" => {
            let unshielded_soc =
                CoupledPublicPrivateSocietySynthesis::evaluate_with_mode(DEFAULT_SEED, false);
            if CoupledPublicPrivateSocietySynthesis::verify_negative_defect_all_dev_seeds()
                && unshielded_soc.all_identities_closed
                && !unshielded_soc.all_axes_verified
                && !unshielded_soc.h1a_regulator_coupling.h1a_verified
                && !unshielded_soc.h1b_credit_screening.h1b_verified
                && !unshielded_soc.h2_vcg_commons.h2_verified
            {
                eprintln!(
                    "NEGATIVE PROOF [coupled_society] BLOCKED SUCCESSFULLY on all {} development seeds through seed-dependent claims (identities closed on every seed): at seed 2026 event-time MNAR RRM closed-loop firm survival={:.3} (Coupling DiD gain={:+.3}, CRRA welfare DiD={:+.4}), Stiglitz-Weiss uncollateralized pooling survival={:.3} (Bester gain={:+.3}), proportional-rebate VCG Ostrom commons stock S_T/K={:.3} (disjoint-cohort gain={:+.3})",
                    ACCEPTANCE_SEEDS.len(),
                    unshielded_soc
                        .h1a_regulator_coupling
                        .naive_closed_loop
                        .terminal_firm_survival_rate,
                    unshielded_soc
                        .h1a_regulator_coupling
                        .deployed_coupling_did_firm_survival,
                    unshielded_soc
                        .h1a_regulator_coupling
                        .deployed_coupling_did_crra_welfare,
                    unshielded_soc
                        .h1b_credit_screening
                        .point_lambda_050
                        .stiglitz_weiss_pooling_survival,
                    unshielded_soc
                        .h1b_credit_screening
                        .deployed_survival_gain_lambda_050,
                    unshielded_soc
                        .h2_vcg_commons
                        .proportional_rebate_vcg_stock_ratio,
                    unshielded_soc
                        .h2_vcg_commons
                        .deployed_commons_stock_gain_over_rebate
                );
                std::process::exit(1);
            }
        }
        other => {
            eprintln!("Unknown defect: '{other}'");
            std::process::exit(2);
        }
    }
    eprintln!("CRITICAL ERROR: negative proof '{defect}' did not block!");
    std::process::exit(0);
}
