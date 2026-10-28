//! Binary `gates` — Enforces the 8 Epistemological Gates (G1 to G8) over the YAML Cards (`cards/*.yaml`)
//! and their anchored CSV datasets (`data/*.csv`), and executes negative defect proofs (`--defect`)
//! that must exit with code `1`.

use std::env;
use std::path::Path;
use stochology_cli::{
    evaluate_empirical_card_severe_test, load_atlantic_storms_decade_environments,
    load_atlantic_storms_forecasts, load_bank_call_clusters, load_cards_from_disk,
    load_danish_fire, load_djia_returns, load_lalonde_nsw_cps, load_oot_ood_flight_reserves,
    load_toronto_arrests_prevalences, load_vic_elec_demand, PublicInfrastructureCard04Audit,
};
use stochology_core::{
    apply_cromwell_floor, mean, verify_resolution_phi5, ChouldechovaAudit, ExceedanceDesignEffect,
    InfoGapOption, ManskiBounds, MonteCarloSplitTreeAudit, PerformativeSystem, Rng, DEFAULT_SEED,
};
use stochology_governance::{
    query_with_bitemporal_clock, BitemporalRecord, MurphyCorpCircuitBreaker, PlatTrafficLightZone,
    PredeterminedChangeControlPlan, TemporalVault,
};
use stochology_lenses::analog::{
    AnalogWaveComputingSynthesis, EmpiricalWaveAnalogAudit, GerstgrasserAccumulateAudit,
    SyntheticDataPolicy,
};
use stochology_lenses::critical::{
    CriticalStochasticRationalitySynthesis, EmpiricalCriticalRationalityAudit,
};
use stochology_lenses::decision::{
    brier_score, CorpPavDecomposition, CorpRocMurphyTriptych, ExactCoinCalibrationBenchmark,
    MurphyDiagram,
};
use stochology_lenses::information::{DpiMarkovChain, SdpiHierarchy};
use stochology_lenses::machine::{AlmostPerfectMachine, AuthenticModelForm};
use stochology_lenses::relational::RelationalComplexSystemsSynthesis;
use stochology_lenses::stocks::{
    EmpiricalStocharyCyberneticAudit, StocharyConjecturalCyberneticSynthesis,
};
use stochology_siga::{AlertFatigueAudit, EvidenceProcess, SaviConfidenceSequence, VarEsEBacktest};

fn run_positive_gates(root: &Path) -> Result<(), String> {
    println!("=== EPISTEMOLOGICAL GATES G1-G8 (STOCHOLOGY PROTOCOL) ===");

    // G1-G5: Load the 4 YAML card files directly from disk (`cards/*.yaml`), verify SHA-256 pre-registration
    // and dataset hashes, recompute all empirical quantities from `data/*.csv`, and run the 8-Step SIGA Roadmap.
    let split_tree = MonteCarloSplitTreeAudit::evaluate(DEFAULT_SEED, 16, 512);
    if !split_tree.split_independence_verified {
        return Err("G1 failed SplitMix64::split(stream_id) fork-join independence check".into());
    }
    let dpi_card04 = PublicInfrastructureCard04Audit::evaluate_from_disk(root);
    if !dpi_card04.verified {
        return Err("G3/G4 failed PublicInfrastructureCard04Audit (Meng/Lee/Chouldechova/AlertFatigue) check".into());
    }
    let cards = load_cards_from_disk(root)?;
    for (rel_path, card) in &cards {
        let (_audit, siga_rep) = evaluate_empirical_card_severe_test(root, card)?;
        if !siga_rep.approved_end_to_end {
            return Err(format!(
                "Card '{rel_path}' failed 8-Step SIGA Roadmap: {:?}",
                siga_rep.steps_1_to_8_completed
            ));
        }
    }
    println!(
        "  [G1-G5] YAML Cards ({}/{} from disk, verified against CSVs): Phi1 Simulability (.split fork-join), Phi2 Moulton-Campbell/Exceedance, Phi3 Null/Missingness (Meng/Lee), Phi4 Chouldechova/Exit, Phi5 Resolution -> APPROVED",
        cards.len(),
        cards.len()
    );

    // G6: Prohibitions 1-4, Performative Prediction, Info-Gap + Sniedovich, SDPI + 3-Step DPI Chain, and CORP-PAV on real data
    let perf_model = PerformativeSystem::new(1.0, 0.2, 0.5, 1.0, 1.5);
    let sys = perf_model.diagnose();
    let perfgd = perf_model.perfgd_trajectory(0.0, 0.5, 60);
    let theta_perfgd = *perfgd.last().unwrap_or(&0.0);
    if !sys.rrm_converges
        || sys.risk_at_po >= sys.risk_at_ps
        || (theta_perfgd - sys.theta_po).abs() > 1e-5
    {
        return Err("G6 failed performative / PerfGD Stackelberg verification".into());
    }
    let ig = InfoGapOption {
        id: "robust",
        nominal_performance: 88.0,
        adverse_sensitivity: 8.0,
        favorable_propensity: 15.0,
        wald_global_maximin_bound: 40.0,
        reversible: true,
    }
    .evaluate(60.0, 120.0, 20.0);
    if !ig.approved_sniedovich || ig.zero_ing_at_nominal != 0.0 {
        return Err("G6 failed Info-Gap / Sniedovich check".into());
    }
    let sdpi = SdpiHierarchy::compute(&[0.5, 0.5], &[vec![0.85, 0.15], vec![0.15, 0.85]]);
    let dpi_chain = DpiMarkovChain::compute(
        &[0.5, 0.5],
        &[vec![0.90, 0.10], vec![0.10, 0.90]],
        &[vec![0.85, 0.15], vec![0.15, 0.85]],
        &[vec![0.80, 0.20], vec![0.20, 0.80]],
    );
    if !sdpi.hierarchy_valid || !dpi_chain.dpi_satisfied {
        return Err("G6 failed SDPI hierarchy / 3-step DPI Markov chain check".into());
    }
    let (storm_probs, storm_outcomes) = load_atlantic_storms_forecasts(root);
    let corp = CorpPavDecomposition::compute(&storm_probs, &storm_outcomes);
    let triptych = CorpRocMurphyTriptych::evaluate(&storm_probs, &storm_outcomes);
    if corp.identity_error > 1e-12 || !triptych.all_triptych_identities_closed {
        return Err(
            "G6 failed CORP-PAV / Murphy Diagram triptych identity check on atlantic_storms.csv"
                .into(),
        );
    }
    let savi_weyl = SaviConfidenceSequence::canonical_weyl_golden_benchmark();
    let var_es_bt = VarEsEBacktest::from_var_exceedances(0.90, &[0, 1, 0, 0, 1, 1, 0, 1]);
    if !savi_weyl.rejects_candidate_m0
        || (savi_weyl.prpl_hedged_wealth_at_m0 - 25.24363875).abs() > 1e-5
        || (var_es_bt.final_martingale_m_t - 9.798157946).abs() > 1e-6
    {
        return Err(
            "G6 failed SAVI Confidence Sequence / VaR-ES GREE e-backtest verification".into(),
        );
    }
    let (epic_sepsis, stoch_triage) =
        AlertFatigueAudit::canonical_epic_sepsis_vs_stochological_gate();
    if epic_sepsis.approved_bridge_l2_l3 || !stoch_triage.approved_bridge_l2_l3 {
        return Err("G6 failed AlertFatigueAudit (10 bit/s & Turing decibans) check".into());
    }
    println!("  [G6] Prohibitions 1-4, Performativity (PS vs PO & Goodhart 4-Variant), AlertFatigueAudit (10 bit/s & Decibans), Info-Gap + Sniedovich, SDPI/DPI Chain, SAVI CS, VaR/ES E-Backtest, and CORP-ROC-Murphy Triptych -> APPROVED");

    // G7: Bitemporal Clock, Canary Column, Single-Opening Vault on real H1 reserve, and Drift Circuit Breaker
    let rec = [BitemporalRecord {
        row_id: 1,
        t_event: 10,
        t_availability: 15,
        feature_x: 1,
        outcome_y: 1,
        canary_column: 1,
        t_availability_canary: 999,
    }];
    let audit_on = query_with_bitemporal_clock(&rec, 50, true);
    let audit_off = query_with_bitemporal_clock(&rec, 50, false);
    if audit_on.temporal_intrusion_alarm || !audit_off.temporal_intrusion_alarm {
        return Err("G7 failed Bitemporal Clock / Canary check".into());
    }
    let (_x13_h1, y13_h1, x23_h1, y23_h1, _, _) = load_oot_ood_flight_reserves(root);
    let p_base_13_h1 = mean(&y13_h1).clamp(0.01, 0.99);
    let logit_base_13_h1 = (p_base_13_h1 / (1.0 - p_base_13_h1)).ln();
    let probs_h1: Vec<f64> = x23_h1
        .iter()
        .map(|&x| {
            let p_model = 1.0 / (1.0 + (-(logit_base_13_h1 + 0.15 * x)).exp());
            apply_cromwell_floor(p_model, 0.001)
        })
        .collect();
    let mut vault = TemporalVault::seal("g7_nyc_flights_2023_h1_vault", y23_h1.clone());
    let receipt = vault
        .open_once(&probs_h1, 0.20)
        .map_err(|e| format!("G7 failed initial Temporal Vault opening: {e:?}"))?;
    if vault.open_once(&probs_h1, 0.20).is_ok() {
        return Err("G7 permitted illegal reopening of Temporal Vault".into());
    }
    let cb = MurphyCorpCircuitBreaker::monitor(
        &storm_probs,
        &storm_outcomes,
        0.02,
        0.05,
        false,
        false,
        false,
    );
    if cb.circuit_breaker_tripped || receipt.evaluated_brier_score <= 0.0 {
        return Err("G7 failed Murphy-CORP circuit breaker check on calibrated storms".into());
    }
    let rtpl_g7: Vec<f64> = (0..80).map(|i| (i as f64) * 0.4).collect();
    let hpl_g7: Vec<f64> = (0..80).map(|i| (i as f64) * 0.4 + 0.15).collect();
    // PCCP drift monitor fed by a real e-process on the H1 vault model's calibration residuals Y_i - p_i
    // (|Y_i - p_i| <= 1, conditional mean 0 under "still calibrated"). Paired severe test on the same forecasts:
    // outcomes drawn from the executable null Y_i ~ Bernoulli(p_i) (seed = 2026) must be approved, while the real
    // 2023 H1 outcomes must trigger the algedonic rollback.
    let mut null_rng = Rng::new(DEFAULT_SEED);
    let null_residuals: Vec<f64> = probs_h1
        .iter()
        .map(|&p| f64::from(u8::from(null_rng.bernoulli(p))) - p)
        .collect();
    let real_residuals: Vec<f64> = probs_h1
        .iter()
        .zip(y23_h1.iter())
        .map(|(&p, &y)| y - p)
        .collect();
    let null_e_proc = EvidenceProcess::from_grapa_betting(&null_residuals, 0.05, 1)?;
    let real_e_proc = EvidenceProcess::from_grapa_betting(&real_residuals, 0.05, 1)?;
    let run_pccp = |e_proc: &EvidenceProcess| {
        PredeterminedChangeControlPlan::evaluate_update(
            "PCCP-G7-H1-VAULT-MODEL",
            0.25,
            0.07,
            e_proc.multiplicity_adjusted_e_value,
            0.05,
            &rtpl_g7,
            &hpl_g7,
            cards[0]
                .1
                .data_and_audit_contract
                .murphy_corp_decomposition
                .miscalibration_rel_mcb,
            cards[0]
                .1
                .data_and_audit_contract
                .murphy_corp_decomposition
                .resolution_res_dsc,
            0.04,
            true,
        )
    };
    let pccp_g7_null = run_pccp(&null_e_proc);
    let pccp_g7_real = run_pccp(&real_e_proc);
    if !pccp_g7_null.update_approved
        || pccp_g7_null.plat_result.traffic_light_zone != PlatTrafficLightZone::Green
    {
        return Err(
            "G7 failed PCCP / PLAT Green traffic-light & Evaluated Competence check under the executable null"
                .into(),
        );
    }
    if !pccp_g7_real.automatic_rollback_triggered {
        return Err(format!(
            "G7 failed: PCCP approved the H1 vault model despite real calibration drift (sup E_t = {:.3e})",
            real_e_proc.multiplicity_adjusted_e_value
        ));
    }
    println!("  [G7] Bitemporal Clock (t_disp), Canary Column, Single-Opening Vault, Murphy-CORP Circuit Breaker, and PCCP/PLAT (null approved, real H1 drift sup E_t = {:.3e} rolled back) -> APPROVED", real_e_proc.multiplicity_adjusted_e_value);

    // G8: Almost-Perfect Machine M=(T,Phi,theta) (P1-P5), Authentic Form (delta(x) + S* + Named Discarded Atoms),
    // Nonlinear NILE/IRM over Phi, and Exact Coin Calibration vs. Decision Benchmark
    let machine = AlmostPerfectMachine::evaluate_canonical(DEFAULT_SEED);
    if !machine.all_p1_to_p5_approved {
        return Err(format!(
            "G8 failed Almost-Perfect Machine P1-P5 check: {:?}",
            (
                machine.p1_universal_density,
                machine.p2_informational_sufficiency,
                machine.p3_fano_minimax_optimality,
                machine.p4_compositional_efficiency,
                machine.p5_atomic_completeness
            )
        ));
    }
    let storm_envs = load_atlantic_storms_decade_environments(root);
    let authentic = AuthenticModelForm::audit_environments(
        DEFAULT_SEED,
        &storm_envs,
        &[
            "atom_0_barometric_pressure_drop".to_string(),
            "atom_1_regime_shifted_lat_proxy".to_string(),
        ],
        0.15,
        true,
    );
    if !authentic.approved_authentic_form
        || authentic.invariant_core_s_star != vec![0]
        || authentic.discarded_atoms_named.is_empty()
    {
        return Err(format!(
            "G8 failed Authentic Model Form check on atlantic_storms.csv: S*={:?}, crit={:?}, id={:?}",
            authentic.invariant_core_s_star,
            authentic.criterion_a,
            authentic.identities_audit
        ));
    }
    let coin_bench = ExactCoinCalibrationBenchmark::evaluate(DEFAULT_SEED, 1_000);
    if !coin_bench.separates_decision_from_probability_calibration {
        return Err("G8 failed Exact Coin Calibration vs. Decision benchmark".into());
    }
    let rel_syn = RelationalComplexSystemsSynthesis::evaluate_canonical(DEFAULT_SEED);
    if !rel_syn.all_six_axes_verified {
        return Err("G8 failed Relational Ontology & Complex Systems 6-axis verification".into());
    }
    let analog_syn = AnalogWaveComputingSynthesis::evaluate_canonical(DEFAULT_SEED);
    let gerst_accum = GerstgrasserAccumulateAudit::canonical_benchmark(
        DEFAULT_SEED,
        SyntheticDataPolicy::Accumulate,
    );
    let vic_demand = load_vic_elec_demand(root);
    let emp_wave = EmpiricalWaveAnalogAudit::evaluate_series(&vic_demand);
    if !analog_syn.all_five_axes_verified
        || !gerst_accum.declared_policy_bounded
        || !emp_wave.empirical_wave_verified
    {
        return Err(
            "G8 failed Physical, Analog & Wave-Interference Computing / Gerstgrasser Accumulate verification".into(),
        );
    }
    let crit_syn = CriticalStochasticRationalitySynthesis::evaluate_canonical(DEFAULT_SEED);
    let fire_losses = load_danish_fire(root);
    let toronto_prev = load_toronto_arrests_prevalences(root);
    let emp_crit = EmpiricalCriticalRationalityAudit::evaluate_real_datasets(
        &fire_losses,
        toronto_prev,
        &storm_envs,
        &vic_demand,
    );
    if !crit_syn.all_four_axes_verified || !emp_crit.empirical_critical_verified {
        return Err(
            "G8 failed Critical Theory of Stochastic Rationality & Material Collapse verification"
                .into(),
        );
    }
    let stoch_syn = StocharyConjecturalCyberneticSynthesis::evaluate_canonical(DEFAULT_SEED);
    let djia_returns = load_djia_returns(root);
    let bank_clusters = load_bank_call_clusters(root);
    let emp_stoch = EmpiricalStocharyCyberneticAudit::evaluate_real_datasets(
        &djia_returns,
        &bank_clusters,
        &fire_losses,
        &vic_demand,
        (&storm_probs, &storm_outcomes),
    );
    if !stoch_syn.all_four_axes_verified || !emp_stoch.empirical_stochary_verified {
        return Err(
            "G8 failed Stochary-Conjectural & Cybernetic Synthesis (4 Stock Axioms, Powell/Friston/Pearl, Scarf/WDRO/CVaR, VSM/PATE) verification"
                .into(),
        );
    }
    println!("  [G8] Almost-Perfect Machine M=(T,Phi,theta) (P1-P5), Authentic Form (delta(x) + S*), Nonlinear NILE/IRM, Coin Benchmark, Relational/Complex Systems, Analog/Wave Computing, Critical Stochastic Rationality, and Stochary-Conjectural Cybernetics (PATE/VSM) -> APPROVED");
    Ok(())
}

fn run_negative_proof(root: &Path, defect: &str) -> ! {
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
            if !unshielded_analog.all_five_axes_verified
                && !unshielded_analog.von_neumann_and_collapse.axis1_verified
                && !unshielded_analog.aimc_analog_wall_hp_inv.axis3_verified
            {
                eprintln!(
                    "NEGATIVE PROOF [analog_wall] BLOCKED SUCCESSFULLY: unshielded analog accuracy at sigma=0.60 collapsed to {:.1}%, 1-step LP-INV relative error {:.4} > 1e-7 without HP-INV refinement, and unanchored synthetic variance retention collapsed to {:.2}",
                    unshielded_analog.aimc_analog_wall_hp_inv.nat_accuracy_at_sigma_060 * 100.0,
                    unshielded_analog.aimc_analog_wall_hp_inv.pku_hp_inv_final_relative_error,
                    unshielded_analog.von_neumann_and_collapse.neurosymbolic_anchored_variance_retention
                );
                std::process::exit(1);
            }
        }
        "instrumental_reason" => {
            let reified =
                CriticalStochasticRationalitySynthesis::evaluate_with_mode(DEFAULT_SEED, false);
            if !reified.all_four_axes_verified
                && !reified.adorno_non_identical_and_reification.axis1_verified
                && !reified.marcuse_rouvroy_stochastic_parrot.axis2_verified
                && !reified.dialectical_overcoming.axis4_verified
            {
                eprintln!(
                    "NEGATIVE PROOF [instrumental_reason] BLOCKED SUCCESSFULLY: Adorno Non-Identical liquidated (axis1={}), Marcuse/Rouvroy Stochastic Parrot counterfactual MSE={:.2} (authentic_blocked={}), and unanchored synthetic/analog collapse (rule_sat={:.0}%)",
                    reified.adorno_non_identical_and_reification.axis1_verified,
                    reified.marcuse_rouvroy_stochastic_parrot.parrot_counterfactual_intervention_mse,
                    reified.marcuse_rouvroy_stochastic_parrot.authentic_form_blocks_spurious_preemption,
                    reified.dialectical_overcoming.neurosymbolic_rule_satisfaction_rate * 100.0
                );
                std::process::exit(1);
            }
        }
        "stochary_cybernetics" => {
            let unshielded =
                StocharyConjecturalCyberneticSynthesis::evaluate_with_mode(DEFAULT_SEED, false);
            if !unshielded.all_four_axes_verified
                && !unshielded.topology_and_stock_axioms.axis1_verified
                && !unshielded.powell_friston_causal_engine.axis2_verified
                && !unshielded.strategic_stock_plant.axis3_verified
                && !unshielded.cybernetic_vsm_and_pate.axis4_verified
            {
                eprintln!(
                    "NEGATIVE PROOF [stochary_cybernetics] BLOCKED SUCCESSFULLY: acyclic tree (loops={}, orphans={}), Peters non-ergodic ruin g_time={:+.4} (100-step={:.2}), Bullwhip={:.2}x, Optimizer's Curse disapp={:+.2} (TTS/TTR={:.2} < 1), and PATE Q3 jackpot / Algedonic violation (algedonic={}, q3_alarm={})",
                    unshielded.topology_and_stock_axioms.active_cybernetic_feedback_loops,
                    unshielded.topology_and_stock_axioms.orphan_single_leaf_subbranches,
                    unshielded.topology_and_stock_axioms.ergodic_buffered_time_growth_rate,
                    unshielded.topology_and_stock_axioms.peters_100_step_unbuffered_terminal_stock,
                    unshielded.topology_and_stock_axioms.unbuffered_bullwhip_amplification_ratio,
                    unshielded.strategic_stock_plant.wdro_robust_oos_disappointment,
                    unshielded.strategic_stock_plant.simchi_levi_tts_to_ttr_ratio,
                    unshielded.cybernetic_vsm_and_pate.algedonic_not_measured_not_executed_enforced,
                    unshielded.cybernetic_vsm_and_pate.pate_q3_jackpot_algedonic_alarm_triggered
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
            // The verdict comes from recursive OLS refitting, not from the declared policy: the simulation must
            // close on both Gerstgrasser closed forms before its Replace risk is allowed to block.
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
        other => {
            eprintln!("Unknown defect: '{other}'");
            std::process::exit(2);
        }
    }
    eprintln!("CRITICAL ERROR: negative proof '{defect}' did not block!");
    std::process::exit(0);
}

fn print_usage() {
    println!(
        "Usage: gates [--help | --defect <defect_name>]\n\
         Defects: cromwell, phi5, moulton, performativity, sniedovich, clock, vault, concept_drift, chouldechova, machine, analog_wall, instrumental_reason, stochary_cybernetics, peeking, brier_dominance, var_underreport, lee_point_estimate, replace_collapse"
    );
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .expect("workspace root");

    match args.as_slice() {
        [] => {}
        [flag] if flag == "--help" || flag == "-h" => {
            print_usage();
            return;
        }
        [flag] if flag == "--defect" => {
            eprintln!("Missing defect name after '--defect'.");
            print_usage();
            std::process::exit(2);
        }
        [flag, defect] if flag == "--defect" => {
            run_negative_proof(root, defect);
        }
        other => {
            eprintln!("Unknown arguments: {other:?}");
            print_usage();
            std::process::exit(2);
        }
    }

    match run_positive_gates(root) {
        Ok(()) => {
            println!("=== ALL 8 EPISTEMOLOGICAL GATES APPROVED (EXIT CODE 0) ===");
        }
        Err(msg) => {
            eprintln!("EPISTEMOLOGICAL GATES FAILED: {msg}");
            std::process::exit(1);
        }
    }
}
