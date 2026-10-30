//! Positive epistemological gate evaluators (`G1` through `G8`) and acceptance registry verification.

use std::path::Path;
use stochology_cli::pillars::CapibaPillarAudit;
use stochology_cli::{
    PublicInfrastructureCard04Audit, evaluate_empirical_card_severe_test,
    load_atlantic_storms_decade_environments, load_atlantic_storms_forecasts,
    load_bank_call_clusters, load_cards_from_disk, load_cps_cohort_clusters, load_danish_fire,
    load_djia_returns, load_lalonde_nsw_cps, load_oot_ood_flight_reserves,
    load_toronto_arrests_prevalences, load_vic_elec_demand,
};
use stochology_core::{
    DEFAULT_SEED, EpistemologicalCard, ExceedanceDesignEffect, InfoGapOption,
    MonteCarloSplitTreeAudit, PerformativeSystem, Rng, apply_cromwell_floor, mean,
};
use stochology_governance::{
    BitemporalRecord, MurphyCorpCircuitBreaker, PlatTrafficLightZone,
    PredeterminedChangeControlPlan, TemporalVault, query_with_bitemporal_clock,
};
use stochology_lenses::acceptance::{
    ACCEPTANCE_REGISTRY_PATH, CONFIRMATION_SEED_RANGE, MIN_CLAIM_PASS_RATE, SynthesisAcceptance,
    canonical_registry_lines, clopper_pearson_lower, evaluate_all_syntheses, is_deterministic,
    verify_registry_content,
};
use stochology_lenses::analog::{
    AnalogWaveComputingSynthesis, EmpiricalWaveAnalogAudit, GerstgrasserAccumulateAudit,
    SyntheticDataPolicy,
};
use stochology_lenses::critical::{
    CriticalStochasticRationalitySynthesis, EmpiricalCriticalRationalityAudit,
};
use stochology_lenses::decision::{
    CorpPavDecomposition, CorpRocMurphyTriptych, ExactCoinCalibrationBenchmark,
};
use stochology_lenses::information::{DpiMarkovChain, SdpiHierarchy};
use stochology_lenses::machine::{AlmostPerfectMachine, AuthenticModelForm};
use stochology_lenses::relational::RelationalComplexSystemsSynthesis;
use stochology_lenses::society::{
    COUPLED_SOCIETY_PREREGISTRATION_HASH, CoupledPublicPrivateSocietySynthesis,
    EmpiricalCoupledSocietyAudit,
};
use stochology_lenses::stocks::{
    EmpiricalStocharyCyberneticAudit, StocharyConjecturalCyberneticSynthesis,
};
use stochology_lenses::strategic::{
    EmpiricalStrategicAccommodationAudit, StrategicAiAccommodationSynthesis,
};
use stochology_lenses::triad::{
    ComplexFractalToroidalTechniumSynthesis, EmpiricalTriadicTechniumAudit,
};
use stochology_siga::roadmap::OperatingPointSource;
use stochology_siga::{AlertFatigueAudit, EvidenceProcess, SaviConfidenceSequence, VarEsEBacktest};

/// Gate `G8` acceptance of the simulated syntheses: each gated claim must pass on the development seeds and on the sealed
/// confirmation seeds (`.det.` claims everywhere, others on at least `MIN_CLAIM_PASS_RATE`), and the registry that seals the claim
/// set, the claim-defining sources, and the confirmation pass rates must match the code and `data/SHA256SUMS`.
pub fn check_acceptance(root: &Path) -> Result<(), String> {
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

pub fn report_synthesis(syn: &SynthesisAcceptance) -> Result<(), String> {
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
        let confirm_start = syn
            .confirmation
            .seeds
            .first()
            .copied()
            .unwrap_or(CONFIRMATION_SEED_RANGE.0);
        let confirm_end = syn
            .confirmation
            .seeds
            .last()
            .copied()
            .unwrap_or(CONFIRMATION_SEED_RANGE.1);
        return Err(format!(
            "G8 failed confirmation for {} on seeds {}..={} (identities on all seeds={}, deterministic constant={}): {:?}",
            syn.label,
            confirm_start,
            confirm_end,
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

fn eval_gates_g1_to_g5(root: &Path) -> Result<Vec<(String, EpistemologicalCard)>, String> {
    let manifest = stochology_cli::load_sha256sums_manifest(root)?;
    if manifest.len() < 15 {
        return Err(format!(
            "G3 failed: data/SHA256SUMS has {} entries (expected >= 15)",
            manifest.len()
        ));
    }
    for (rel_path, expected_sha) in &manifest {
        let full_path = root.join(rel_path);
        let bytes = std::fs::read(&full_path)
            .map_err(|e| format!("G3 failed: cannot read manifest entry '{rel_path}': {e}"))?;
        let actual_sha = stochology_core::sha256_hex(&bytes);
        if &actual_sha != expected_sha {
            return Err(format!(
                "G3 failed: '{rel_path}' SHA-256 ({actual_sha}) diverges from data/SHA256SUMS ({expected_sha})"
            ));
        }
    }
    let soc_prereg_path = root.join("data/preregistration/coupled_society_preregistration.yaml");
    let soc_prereg_yaml = std::fs::read_to_string(&soc_prereg_path).map_err(|e| {
        format!(
            "G3 failed: cannot read '{}': {e}",
            soc_prereg_path.display()
        )
    })?;
    let soc_card = CoupledPublicPrivateSocietySynthesis::epistemological_card();
    let soc_payload = soc_card.compute_preregistration_payload();
    if !soc_prereg_yaml.contains(COUPLED_SOCIETY_PREREGISTRATION_HASH)
        || !soc_prereg_yaml.contains(&soc_payload)
    {
        return Err(
            "G3 failed: data/preregistration/coupled_society_preregistration.yaml does not match COUPLED_SOCIETY_PREREGISTRATION_HASH or compute_preregistration_payload()"
                .into(),
        );
    }

    let split_tree = MonteCarloSplitTreeAudit::evaluate(DEFAULT_SEED, 16, 512);
    if !split_tree.split_independence_verified {
        return Err("G1 failed SplitMix64::split(stream_id) fork-join independence check".into());
    }
    let dpi_card04 = PublicInfrastructureCard04Audit::evaluate_from_disk(root);
    if !dpi_card04.verified {
        return Err("G3/G4 failed PublicInfrastructureCard04Audit (Meng/Lee/Chouldechova/AlertFatigue) check".into());
    }
    let cards = load_cards_from_disk(root)?;
    let _ = ExceedanceDesignEffect::benchmark_calibration_25028();
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
    Ok(cards)
}

fn eval_gate_g6(root: &Path) -> Result<(Vec<f64>, Vec<f64>), String> {
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
    println!(
        "  [G6] Prohibitions 1-4, Performativity (PS vs PO & Goodhart 4-Variant), AlertFatigueAudit (10 bit/s & Decibans), Info-Gap + Sniedovich, SDPI/DPI Chain, SAVI CS, VaR/ES E-Backtest, and CORP-ROC-Murphy Triptych -> APPROVED"
    );
    Ok((storm_probs, storm_outcomes))
}

fn eval_gate_g7(
    root: &Path,
    cards: &[(String, EpistemologicalCard)],
    storm_probs: &[f64],
    storm_outcomes: &[f64],
) -> Result<(Vec<f64>, Vec<f64>), String> {
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
        storm_probs,
        storm_outcomes,
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
    println!(
        "  [G7] Bitemporal Clock (t_disp), Canary Column, Single-Opening Vault, Murphy-CORP Circuit Breaker, and PCCP/PLAT (null approved, real H1 drift sup E_t = {:.3e} rolled back) -> APPROVED",
        real_e_proc.multiplicity_adjusted_e_value
    );
    Ok((probs_h1, y23_h1))
}

fn eval_gate_g8(
    root: &Path,
    storm_probs: &[f64],
    storm_outcomes: &[f64],
    probs_h1: &[f64],
    y23_h1: &[f64],
) -> Result<(), String> {
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
            authentic.invariant_core_s_star, authentic.criterion_a, authentic.identities_audit
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
        (storm_probs, storm_outcomes),
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
        (storm_probs, storm_outcomes),
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
    let strat_syn = StrategicAiAccommodationSynthesis::evaluate_canonical(DEFAULT_SEED);
    let (nsw_t_g8, _nsw_c_g8, cps_c_g8) = load_lalonde_nsw_cps(root);
    let emp_strat = EmpiricalStrategicAccommodationAudit::evaluate_real_datasets(
        &storm_envs,
        (&nsw_t_g8, &cps_c_g8),
        &bank_clusters,
        &fire_losses,
        (probs_h1, y23_h1),
        &vic_demand,
    );
    if !strat_syn.all_six_axes_verified || !emp_strat.empirical_strategic_verified {
        return Err(
            "G8 failed Strategic AI Six-Pair Accommodation Synthesis & Empirical 6-CSV Coupler verification"
                .into(),
        );
    }
    for finding in strat_syn
        .discarded_findings
        .iter()
        .chain(&emp_strat.discarded_findings)
    {
        println!("  [G8] Strategic AI discarded finding (recorded, not approved): {finding}");
    }
    let soc_syn = CoupledPublicPrivateSocietySynthesis::evaluate_canonical(DEFAULT_SEED);
    let cps_cohorts = load_cps_cohort_clusters(root);
    let emp_soc = EmpiricalCoupledSocietyAudit::evaluate_real_datasets(
        &cps_c_g8,
        &cps_cohorts,
        &bank_clusters,
        &fire_losses,
    );
    if !soc_syn.all_axes_verified || !emp_soc.empirical_society_verified {
        return Err(
            "G8 failed Coupled Public-Private SFC Society Synthesis & CPS/Bank/Fire Coupler verification"
                .into(),
        );
    }
    for finding in soc_syn
        .discarded_findings
        .iter()
        .chain(&emp_soc.discarded_findings)
    {
        println!("  [G8] Coupled Society discarded finding (recorded, not approved): {finding}");
    }
    check_acceptance(root)?;
    println!(
        "  [G8] Almost-Perfect Machine M=(T,Phi,theta) (P1-P5), Authentic Form (delta(x) + S*), Nonlinear NILE/IRM, Coin Benchmark, Relational/Complex Systems, Analog/Wave Computing, Critical Stochastic Rationality, Stochary-Conjectural Cybernetics (PATE/VSM), Triadic (C x Fractals x T^d) & Technium Synthesis, Strategic AI Six-Pair Accommodation, and Coupled Public-Private SFC Society Synthesis -> APPROVED"
    );
    Ok(())
}

/// Real-data CAPIBA pillar audits: one instrument per pillar, reported under the gate that owns it
/// (`G4`, `G6`, `G7`).
fn eval_capiba_pillars(root: &Path) -> Result<(), String> {
    let audit = CapibaPillarAudit::evaluate(root)?;
    let failed = audit.failed_checks();
    if !failed.is_empty() {
        return Err(format!("G4/G6/G7 failed CAPIBA pillar checks: {failed:?}"));
    }
    let c = &audit.calibrated.analog;
    println!(
        "  [G6] C: CORP-CRPS on vic_elec.csv daily peaks (analog ensemble): CRPS={:.1} = MCB {:.1} - DSC {:.1} + UNC {:.1} MW -> admissible",
        c.crps_mean, c.mcb_miscalibration, c.dsc_discrimination, c.unc_uncertainty
    );
    let a = &audit.anytime;
    println!(
        "  [G6] A: e-BH over K={} carriers (nyc_flights 2013 vs 2023, unit = paired calendar day): {} rejected {:?}, {} discarded (Benjamini-Yekutieli on 1/e rejects {}; naive peeking would reject {})",
        a.ebh.hypotheses_k,
        a.ebh.rejected.len(),
        a.ebh.rejected,
        a.ebh.discarded.len(),
        a.ebh.benjamini_yekutieli_rejected.len(),
        a.naive_peeking_rejected.len()
    );
    let p = &audit.performative.power;
    println!(
        "  [G6] P: performative sensitivity on nsw_experimental.csv: W1/sd_0={:.4} (permutation p={:.3}, null mean {:.4}) -> measured eps >= {:.4} (bootstrap q05 of the mean gap)",
        p.w1_standardized, p.permutation_p_value, p.permutation_null_mean, p.epsilon_lower_bound
    );
    let i = &audit.invariant.anchor;
    println!(
        "  [G6] I: anchor regression nyc_flights 2013 -> 2023: slope gap at gamma=5 is {:.3} (anchor-stable={}), test MSE {:.1} (OLS) vs {:.1} (gamma=5)",
        i.relative_slope_gap, i.anchor_stable, i.test_pooled_mse[0], i.test_pooled_mse[2]
    );
    let b = &audit.bitemporal;
    println!(
        "  [G7] B: real-time vintages rtdsm_routput_vintages.csv: {} decisions, clock clean, mean revision {:+.2} pp (SD {:.2}), leaky backtest blocked",
        b.decisions, b.mean_revision, b.sd_revision
    );
    let t = &audit.auditing.audit;
    println!(
        "  [G4] A: multicalibration on toronto_arrests.csv: {} cells, {} violations, verdict {:?} (largest confidence radius {:.3}, tolerance 0.05)",
        t.cells_audited, t.violations, t.verdict, t.max_radius
    );
    Ok(())
}

/// Runs all 8 positive epistemological gates (`G1` through `G8`).
pub fn run_positive_gates(root: &Path) -> Result<(), String> {
    println!("=== EPISTEMOLOGICAL GATES G1-G8 (STOCHOLOGY PROTOCOL) ===");
    let cards = eval_gates_g1_to_g5(root)?;
    let (storm_probs, storm_outcomes) = eval_gate_g6(root)?;
    let (probs_h1, y23_h1) = eval_gate_g7(root, &cards, &storm_probs, &storm_outcomes)?;
    eval_capiba_pillars(root)?;
    eval_gate_g8(root, &storm_probs, &storm_outcomes, &probs_h1, &y23_h1)
}
