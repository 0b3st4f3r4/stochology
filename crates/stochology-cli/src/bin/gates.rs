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
use stochology_lenses::acceptance::{
    canonical_registry_lines, clopper_pearson_lower, evaluate_all_syntheses, is_deterministic,
    verify_registry_content, SynthesisAcceptance, ACCEPTANCE_REGISTRY_PATH, ACCEPTANCE_SEEDS,
    CONFIRMATION_SEED_RANGE, MIN_CLAIM_PASS_RATE,
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
use stochology_lenses::triad::{
    ComplexFractalToroidalTechniumSynthesis, EmpiricalTriadicTechniumAudit,
};
use stochology_siga::roadmap::OperatingPointSource;
use stochology_siga::{AlertFatigueAudit, EvidenceProcess, SaviConfidenceSequence, VarEsEBacktest};

/// Gate `G8` acceptance of the simulated syntheses: each gated claim must pass on the development seeds and on the sealed
/// confirmation seeds (`.det.` claims everywhere, others on at least `MIN_CLAIM_PASS_RATE`), and the registry that seals the claim
/// set, the claim-defining sources, and the confirmation pass rates must match the code and `data/SHA256SUMS`.
fn check_acceptance(root: &Path) -> Result<(), String> {
    let syntheses = evaluate_all_syntheses();
    for syn in &syntheses {
        report_synthesis(syn)?;
    }
    let path = root.join(ACCEPTANCE_REGISTRY_PATH);
    let bytes = std::fs::read(&path)
        .map_err(|e| format!("G8 failed: cannot read {ACCEPTANCE_REGISTRY_PATH}: {e}"))?;
    let actual = stochology_core::sha256_hex(&bytes);
    let manifest = stochology_cli::load_sha256sums_manifest(root)?;
    let sealed = manifest.get(ACCEPTANCE_REGISTRY_PATH).ok_or_else(|| {
        format!("G8 failed: {ACCEPTANCE_REGISTRY_PATH} is not sealed in data/SHA256SUMS")
    })?;
    if &actual != sealed {
        return Err(format!(
            "G8 failed: {ACCEPTANCE_REGISTRY_PATH} SHA-256 ({actual}) diverges from data/SHA256SUMS ({sealed})"
        ));
    }
    let content = String::from_utf8(bytes)
        .map_err(|e| format!("G8 failed: {ACCEPTANCE_REGISTRY_PATH} is not UTF-8: {e}"))?;
    let expected =
        canonical_registry_lines(root, &syntheses).map_err(|e| format!("G8 failed: {e}"))?;
    verify_registry_content(&content, &expected)
        .map_err(|e| format!("G8 failed: acceptance registry does not match the code: {e}"))?;
    println!(
        "  [G8] Acceptance registry sealed (SHA-256 {}...): claim set, claim-defining sources, and confirmation rates match the code",
        &actual[..12]
    );
    Ok(())
}

fn report_synthesis(syn: &SynthesisAcceptance) -> Result<(), String> {
    if !syn.development.accepted {
        return Err(format!(
            "G8 failed development acceptance for {} (identities on all seeds={}, deterministic constant={}): {:?}",
            syn.label,
            syn.development.identities_closed_on_all_seeds,
            syn.development.deterministic_claims_constant,
            syn.development.failing_claims()
        ));
    }
    if !syn.accepted() {
        return Err(format!(
            "G8 failed confirmation for {} on seeds {}..={} (identities on all seeds={}, deterministic constant={}): {:?}",
            syn.label,
            CONFIRMATION_SEED_RANGE.0,
            CONFIRMATION_SEED_RANGE.1,
            syn.confirmation.identities_closed_on_all_seeds,
            syn.confirmation.deterministic_claims_constant,
            syn.refuted_on_confirmation()
        ));
    }
    let deterministic = syn.gated.iter().filter(|n| is_deterministic(n)).count();
    let n_confirm = syn.confirmation.seeds.len() as u64;
    let (worst_claim, worst_lower) = syn
        .confirmation
        .claim_pass_rates
        .iter()
        .filter(|(name, _)| syn.gated.contains(name) && !is_deterministic(name))
        .map(|&(name, rate)| {
            let passes = (rate * n_confirm as f64).round() as u64;
            (name, clopper_pearson_lower(passes, n_confirm, 0.05))
        })
        .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap_or(("none", 1.0));
    println!(
        "  [G8] {}: {} gated claims ({} seed-dependent, {} deterministic) pass on >= {:.0}% of {} development and {} sealed confirmation seeds",
        syn.label,
        syn.gated.len(),
        syn.gated.len() - deterministic,
        deterministic,
        MIN_CLAIM_PASS_RATE * 100.0,
        syn.development.seeds.len(),
        syn.confirmation.seeds.len()
    );
    println!(
        "  [G8] {}: lowest one-sided 95% Clopper-Pearson bound on a confirmation pass rate is {:.3} ({worst_claim})",
        syn.label, worst_lower
    );
    for name in &syn.discarded {
        let rate = syn
            .confirmation
            .claim_pass_rates
            .iter()
            .find(|(n, _)| n == name)
            .map_or(f64::NAN, |&(_, r)| r);
        println!(
            "  [G8] {} discarded claim (recorded, not approved): {name} passes on {:.1}% of confirmation seeds",
            syn.label,
            rate * 100.0
        );
    }
    Ok(())
}

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
    // §II.C provenance: the Epic operating point is literature (Wong et al. 2021); the Stochological one is assumed.
    if !matches!(
        epic_sepsis.operating_point_source,
        OperatingPointSource::Literature { .. }
    ) || stoch_triage.operating_point_source != OperatingPointSource::Assumed
    {
        return Err(
            "G6 failed AlertFatigueAudit operating-point provenance (Literature vs Assumed) check"
                .into(),
        );
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
    for finding in &emp_stoch.discarded_findings {
        println!("  [G8] Stochary discarded finding (recorded, not approved): {finding}");
    }
    let triad_syn = ComplexFractalToroidalTechniumSynthesis::evaluate_canonical(DEFAULT_SEED);
    let emp_triad = EmpiricalTriadicTechniumAudit::evaluate_real_datasets(
        &vic_demand,
        &fire_losses,
        (&storm_probs, &storm_outcomes),
        &djia_returns,
        &bank_clusters,
    );
    if !triad_syn.all_six_bridges_verified || !emp_triad.empirical_triadic_verified {
        return Err(
            "G8 failed Triadic (C x Fractals x T^d) & Technium/Technosphere Synthesis verification"
                .into(),
        );
    }
    for finding in triad_syn
        .discarded_findings
        .iter()
        .chain(&emp_triad.discarded_findings)
    {
        println!("  [G8] Triadic discarded finding (recorded, not approved): {finding}");
    }
    check_acceptance(root)?;
    println!("  [G8] Almost-Perfect Machine M=(T,Phi,theta) (P1-P5), Authentic Form (delta(x) + S*), Nonlinear NILE/IRM, Coin Benchmark, Relational/Complex Systems, Analog/Wave Computing, Critical Stochastic Rationality, Stochary-Conjectural Cybernetics (PATE/VSM), and Triadic (C x Fractals x T^d) & Technium Synthesis -> APPROVED");
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
            // The positivist methods are actually run; identities must still close so the block comes from their outputs.
            let reified =
                CriticalStochasticRationalitySynthesis::evaluate_with_mode(DEFAULT_SEED, false);
            let fails_on_all_seeds = ACCEPTANCE_SEEDS.iter().all(|&seed| {
                let naive = CriticalStochasticRationalitySynthesis::evaluate_with_mode(seed, false);
                naive.all_identities_closed
                    && !naive.all_four_axes_verified
                    && naive
                        .claim_checks()
                        .iter()
                        .any(|&(name, ok)| !ok && !is_deterministic(name))
            });
            if fails_on_all_seeds
                && reified.all_identities_closed
                && !reified.all_four_axes_verified
                && !reified.adorno_non_identical_and_reification.axis1_verified
                && !reified.marcuse_rouvroy_stochastic_parrot.axis2_verified
                && !reified.dialectical_overcoming.axis4_verified
            {
                eprintln!(
                    "NEGATIVE PROOF [instrumental_reason] BLOCKED SUCCESSFULLY on all {} development seeds through seed-dependent claims (identities closed on every seed): at seed 2026 truncated-Gaussian VaR99 breached on {:.1}% of fresh draws, RRM excess performative risk={:.3}, Stochastic Parrot counterfactual MSE={:.2}, unconstrained recursion rule satisfaction={:.0}%, raw drifted readout accuracy={:.0}%, one-shot HP-INV error={:.1e}",
                    ACCEPTANCE_SEEDS.len(),
                    reified.adorno_non_identical_and_reification.deployed_var99_holdout_exceedance * 100.0,
                    reified.adorno_non_identical_and_reification.deployed_performative_excess_risk,
                    reified.marcuse_rouvroy_stochastic_parrot.deployed_counterfactual_mse,
                    reified.dialectical_overcoming.deployed_rule_satisfaction_rate * 100.0,
                    reified.dialectical_overcoming.a_ecc_drift_recovery_accuracy * 100.0,
                    reified.dialectical_overcoming.pku_40nm_rram_24bit_hp_inv_error
                );
                std::process::exit(1);
            }
        }
        "stochary_cybernetics" => {
            // The naive methods are actually run; identities must still close so the block comes from their outputs.
            let unshielded =
                StocharyConjecturalCyberneticSynthesis::evaluate_with_mode(DEFAULT_SEED, false);
            let fails_on_all_seeds = ACCEPTANCE_SEEDS.iter().all(|&seed| {
                let naive = StocharyConjecturalCyberneticSynthesis::evaluate_with_mode(seed, false);
                naive.all_identities_closed
                    && !naive.all_four_axes_verified
                    && naive
                        .claim_checks()
                        .iter()
                        .any(|&(name, ok)| !ok && !is_deterministic(name))
            });
            if fails_on_all_seeds
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
                    unshielded.topology_and_stock_axioms.low_pass_buffered_variance_ratio,
                    unshielded.topology_and_stock_axioms.ergodic_buffered_time_growth_rate,
                    unshielded.powell_friston_causal_engine.deployed_policy_choices_match_voi,
                    unshielded.powell_friston_causal_engine.pearl_front_door_identified_effect,
                    unshielded.powell_friston_causal_engine.pearl_transportability_error,
                    unshielded.strategic_stock_plant.supply_chain_physics.kingman_vut_buffered_vs_cliff_wait_hours.0,
                    unshielded.strategic_stock_plant.supply_chain_physics.ivanov_ripple_effect_viability_service_rate,
                    unshielded.strategic_stock_plant.simchi_levi_tts_to_ttr_ratio,
                    unshielded.strategic_stock_plant.wdro_robust_oos_disappointment,
                    unshielded.cybernetic_vsm_and_pate.conant_ashby_regulator_homomorphism_r2,
                    unshielded.cybernetic_vsm_and_pate.kahneman_judgment_noise_before_and_after.1,
                    unshielded.cybernetic_vsm_and_pate.plat_spearman_and_ks.0
                );
                std::process::exit(1);
            }
        }
        "triadic_technium" => {
            // The naive methods are actually run; identities must still close so the block comes from their outputs.
            let unshielded_triad =
                ComplexFractalToroidalTechniumSynthesis::evaluate_with_mode(DEFAULT_SEED, false);
            let fails_on_all_seeds = ACCEPTANCE_SEEDS.iter().all(|&seed| {
                let naive =
                    ComplexFractalToroidalTechniumSynthesis::evaluate_with_mode(seed, false);
                naive.all_identities_closed
                    && !naive.all_six_bridges_verified
                    && naive
                        .claim_checks()
                        .iter()
                        .any(|&(name, ok)| !ok && !is_deterministic(name))
            });
            if fails_on_all_seeds
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
                    unshielded_triad.complex_wave_gabor_helmholtz.deployed_helmholtz_residual_rel_mse,
                    unshielded_triad.complex_real_isomorphism_hmatrix.two_stage_block_amc_16x16_relative_error,
                    unshielded_triad.complex_real_isomorphism_hmatrix.hmatrix_deployed_relative_error,
                    unshielded_triad.fractional_ctrw_analog_ecc.deployed_pcm_drift_mse,
                    unshielded_triad.fractional_ctrw_analog_ecc.deployed_ctrw_spread_prediction_rel_error * 100.0,
                    unshielded_triad.fractional_ctrw_analog_ecc.analog_ecc_ume_recovery_accuracy,
                    unshielded_triad.multifractal_pdhte_rag_collapse.deployed_ate_rmse,
                    unshielded_triad.weyl_torus_hermite_mehler_dfm.deployed_weyl_discrepancy,
                    unshielded_triad.weyl_torus_hermite_mehler_dfm.deployed_dfm_l2_error,
                    unshielded_triad.weyl_torus_hermite_mehler_dfm.deployed_label_shift_kl,
                    unshielded_triad.sna_rlhf_fhrr_cohomology_technium.deployed_performative_gap_to_opt
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
        "Usage: gates [--help | --defect <defect_name> | --print-acceptance-registry]\n\
         Defects: cromwell, phi5, moulton, performativity, sniedovich, clock, vault, concept_drift, chouldechova, machine, analog_wall, instrumental_reason, stochary_cybernetics, peeking, brier_dominance, var_underreport, lee_point_estimate, replace_collapse, triadic_technium"
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
        [flag] if flag == "--print-acceptance-registry" => {
            match canonical_registry_lines(root, &evaluate_all_syntheses()) {
                Ok(lines) => lines.iter().for_each(|l| println!("{l}")),
                Err(e) => {
                    eprintln!("{e}");
                    std::process::exit(2);
                }
            }
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
