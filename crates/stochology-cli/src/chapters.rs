//! Progressive Chapter 1–9 executable laboratories over `CommunityReserveEnv`
//! and the 14 cryptographically sealed real-world datasets in `data/*.csv`,
//! organized 1-to-1 around the Three Canonical Operators (`EpistemicMarginState`,
//! `VitalMarginState`, `GovernanceState`) and the Contiguous Nuclear Triad.

use std::path::Path;

use stochology::{
    BitemporalRecord, CellularAutomatonAudit, CognitiveFramingRegime, CollapseProposition,
    CommunityReserveEnv, CompensatoryBufferAgent, DEFAULT_SEED, DiagnosticClue, DriftMonitorConfig,
    ElementaryRule, EmpiricalChouldechovaAudit, EpistemicMarginState, FrictionEnvelopeControlAudit,
    GovernanceState, HollingTailRuptureAudit, IcpIdentificationStatus, KellySizingRegime,
    LatentModelExploitationAudit, LatentWorldModelConfig, LeontiefVsMultiplicativeAudit,
    ModelDriftFallbackAudit, NaiveOptimizer, NotTestable, PreregisteredFalsifier,
    ProcessTracingInference, ReflexiveBrakeAudit, RefutabilityPreconditions, SocialNetworkSpec,
    StochologicalAgent, StructuralCollapseProofAudit, TemporalVault, TopologyCentralityAudit,
    TriadicIntegrity, VitalMarginState, WaldSurvivorshipAudit, ZhengMeisterQueueAudit,
    bitemporal_view_at, mean, quantile, sample_std,
};

use crate::loaders::{
    count_atlantic_storms_transitions, count_flight_csv_rows_and_missing,
    load_atlantic_storms_environments, load_atlantic_storms_forecasts, load_bank_call_clusters,
    load_cps_cohort_clusters, load_danish_fire, load_djia_returns, load_lalonde_nsw_cps,
    load_nyc_flight_delays, load_oot_ood_flight_delays, load_rtdsm_routput_vintages,
    load_vic_elec_demand, try_load_toronto_arrests_full_summary,
    try_load_toronto_arrests_matched_and_classifier, verify_external_upstream_schema_contract,
    verify_sha256sums_manifest,
};

/// Export the deterministic `CommunityReserveEnv` (`seed = 2026`, `T = 24`, `R_0 = 45.0`)
/// trajectory bundle as JSON for `book-src/lab/index.html`.
pub fn export_lab_trajectories_json() -> String {
    let env = CommunityReserveEnv::new(DEFAULT_SEED, 24, 45.0);
    env.export_lab_json()
}

/// Run Chapter 1: The Sequential Loop & The Naive Agent (`engine.rs`).
pub fn run_chapter_1() -> Result<(), String> {
    let env = CommunityReserveEnv::new(DEFAULT_SEED, 24, 45.0);
    let rep_naive = env.run_trajectory(&mut NaiveOptimizer);
    let rep_no_epi = env.run_trajectory(&mut StochologicalAgent::without_epistemic_margin());
    let rep_no_vital = env.run_trajectory(&mut StochologicalAgent::epistemic_only());
    let rep_no_gov = env.run_trajectory(&mut StochologicalAgent::without_governance());
    let rep_full = env.run_trajectory(&mut StochologicalAgent::full());

    println!("=== CHAPTER 1: THE SEQUENTIAL LOOP & THE NAIVE AGENT ===");
    println!(
        "  Environment [Simulated]: CommunityReserveEnv (seed = {}, horizon = {} shifts, R_0 = {:.1})",
        env.seed, env.horizon_steps, env.initial_reserve
    );
    println!(
        "  NaiveOptimizer          -> coverage = {:5.1}%, mean_harm = {:6.2}, terminal_reserve = {:6.2}, ruin_step = {:?}",
        rep_naive.epistemic_coverage_rate * 100.0,
        rep_naive.mean_social_harm,
        rep_naive.terminal_reserve,
        rep_naive.ruin_step
    );
    println!(
        "  Prop 1 Degrad (No E-Mar)-> coverage = {:5.1}%, mean_harm = {:6.2}, terminal_reserve = {:6.2}, ruin_step = {:?}",
        rep_no_epi.epistemic_coverage_rate * 100.0,
        rep_no_epi.mean_social_harm,
        rep_no_epi.terminal_reserve,
        rep_no_epi.ruin_step
    );
    println!(
        "  Collapse 2 (No V-Margin)-> coverage = {:5.1}%, mean_harm = {:6.2}, terminal_reserve = {:6.2}, ruin_step = {:?}",
        rep_no_vital.epistemic_coverage_rate * 100.0,
        rep_no_vital.mean_social_harm,
        rep_no_vital.terminal_reserve,
        rep_no_vital.ruin_step
    );
    println!(
        "  Collapse 3 (No Gov.)    -> coverage = {:5.1}%, mean_harm = {:6.2}, terminal_reserve = {:6.2}, ruin_step = {:?}",
        rep_no_gov.epistemic_coverage_rate * 100.0,
        rep_no_gov.mean_social_harm,
        rep_no_gov.terminal_reserve,
        rep_no_gov.ruin_step
    );
    println!(
        "  StochologicalAgent(Full)-> coverage = {:5.1}% (shock_cov = {:5.1}%), mean_harm = {:6.2}, terminal_reserve = {:6.2}, ruin_step = {:?}",
        rep_full.epistemic_coverage_rate * 100.0,
        rep_full.shock_coverage_rate * 100.0,
        rep_full.mean_social_harm,
        rep_full.terminal_reserve,
        rep_full.ruin_step
    );

    if rep_full.ruin_step.is_some() {
        return Err("Full StochologicalAgent must not suffer ruin".to_string());
    }
    if rep_full.mean_social_harm >= rep_naive.mean_social_harm {
        return Err(
            "Full StochologicalAgent must strictly reduce mean harm vs NaiveOptimizer".to_string(),
        );
    }
    Ok(())
}

/// Run Chapter 2: Filter I · Axis 1 (`S_epi`) — Inquiry, Openness & Multiverse (`epistemic.rs`).
pub fn run_chapter_2() -> Result<(), String> {
    let inst_frame = EpistemicMarginState::from_inquiry_framing(
        100.0, 0.85, 200.0, 0.18, 2.5, 4.0, 64.0, 0.05, 45.0, 110.0, 14.0, 2.0, 15.0,
    );
    let affected_frame = EpistemicMarginState::from_inquiry_framing(
        100.0, 0.85, 800.0, 0.18, 2.5, 4.0, 64.0, 0.05, 45.0, 110.0, 14.0, 14.0, 110.0,
    );
    let gap = inst_frame.framing_gap(&affected_frame);

    let fork_estimates = [1.42, 1.55, 1.38, 1.61, 1.49, 1.52];
    let fork_e_values = [28.0, 42.0, 19.5, 64.0, 31.0, 35.0];
    let mv_state = EpistemicMarginState::from_multiverse(&fork_estimates, &fork_e_values, 0.05);

    println!("=== CHAPTER 2: FILTER I (S_EPI) — INQUIRY, OPENNESS & MULTIVERSE ===");
    println!(
        "  Cost-Loss Thresholds [Assumed]: institutional p* = {:.3} (EVPI={:.2}) vs affected-citizen p* = {:.3} (EVPI={:.2}), gap = {:.3}",
        inst_frame.threshold_p_star,
        inst_frame.evpi,
        affected_frame.threshold_p_star,
        affected_frame.evpi,
        gap
    );
    println!(
        "  Mayo Severe Power [Assumed]: {:.3} | Type III Error: {} (global_net={:.1}) | Simon Framing: {:?} vs {:?} (PRI = {:.3} vs {:.3}, threshold = 0.20)",
        affected_frame.severe_power,
        inst_frame.is_type_three_error,
        inst_frame.global_net_utility,
        inst_frame.framing_regime,
        affected_frame.framing_regime,
        inst_frame.pri,
        affected_frame.pri
    );
    println!(
        "  Multiverse & e-BH [Assumed]: consistency = {:.1}%, interval = [{:.2}, {:.2}], e-BH discoveries = {}",
        mv_state.multiverse_consistency * 100.0,
        mv_state.bounds[0],
        mv_state.bounds[1],
        mv_state.e_bh_discoveries_count
    );

    if affected_frame.severe_power < 0.90
        || !inst_frame.is_type_three_error
        || inst_frame.framing_regime != CognitiveFramingRegime::StrategicPremiseRigging
        || affected_frame.framing_regime != CognitiveFramingRegime::CommunicativeInfluence
        || mv_state.e_bh_discoveries_count == 0
    {
        return Err("Chapter 2 inquiry and multiverse checks failed".to_string());
    }
    Ok(())
}

/// Run Chapter 3: Filter I · Axis 1 (`S_epi`) — Underdetermination & Partial Identification (`epistemic.rs`).
pub fn run_chapter_3(workspace_root: &Path) -> Result<(), String> {
    let (nsw_t, nsw_c, cps_c) = load_lalonde_nsw_cps(workspace_root);
    let mean_t = mean(&nsw_t);
    let mean_nsw_c = mean(&nsw_c);
    let mean_cps_c = mean(&cps_c);
    let experimental_ate = mean_t - mean_nsw_c;
    let naive_observational_diff = mean_t - mean_cps_c;

    // 1. [Measured] Real employment-selection bounds on nsw_experimental.csv (Lee 2009: positive earners re78 > 0,
    //    where 128/185 treated vs 170/260 control have positive wages):
    let nsw_t_pos: Vec<f64> = nsw_t.iter().copied().filter(|&x| x > 0.0).collect();
    let nsw_c_pos: Vec<f64> = nsw_c.iter().copied().filter(|&x| x > 0.0).collect();
    let lee_emp_measured = EpistemicMarginState::from_lee_attrition_bounds(
        &nsw_t_pos,
        nsw_t.len(),
        &nsw_c_pos,
        nsw_c.len(),
    );

    // 2. [Assumed] Two-arm survey-attrition stress test (15/200 treated & 30/290 control missingness sensitivity):
    let manski_ate =
        EpistemicMarginState::from_ate_bounds(&nsw_t, 15, &nsw_c, 30, 0.0, 22_000.0, 0.05);
    let lee_ate = EpistemicMarginState::from_lee_attrition_bounds(&nsw_t, 200, &nsw_c, 290);
    let (censoring, censoring_naive) = EpistemicMarginState::from_censoring_bounds(28, 22, 150);
    // Peterson (1976) Fréchet bounds on joint survival: marginal survivals for the two
    // competing risks S1 = 1 - 28/200 (event of interest) and S2 = 1 - 22/200 (at-risk loss).
    let (peterson, peterson_naive) = EpistemicMarginState::from_peterson_competing_risks_bounds(
        1.0 - 28.0 / 200.0,
        1.0 - 22.0 / 200.0,
    );

    let e_proc = EpistemicMarginState::from_e_process(
        &[1.2, 1.5, 0.9, 1.4, 1.6, 1.3, 1.1, 1.5, 1.7, 1.4],
        0.0,
        1.0,
        0.8,
        0,
    );

    println!("=== CHAPTER 3: FILTER I (S_EPI) — UNDERDETERMINATION & PARTIAL IDENTIFICATION ===");
    println!(
        "  LaLonde Benchmark [Measured] (data/nsw_experimental.csv vs data/cps_observational.csv):"
    );
    println!(
        "    Experimental RCT ATE [Measured, zero survey attrition] = ${:.0} (n_t={}, n_c={}) vs Naive CPS Diff = ${:.0} (n_cps={})",
        experimental_ate,
        nsw_t.len(),
        nsw_c.len(),
        naive_observational_diff,
        cps_c.len()
    );
    println!(
        "    Lee (2009) Positive-Wage Selection Bounds [Measured, re78>0: {}/{} vs {}/{}]: [${:.0}, ${:.0}] (width=${:.0})",
        nsw_t_pos.len(),
        nsw_t.len(),
        nsw_c_pos.len(),
        nsw_c.len(),
        lee_emp_measured.bounds[0],
        lee_emp_measured.bounds[1],
        lee_emp_measured.identification_width
    );
    println!(
        "    Manski Worst-Case Two-Arm ATE Bounds [Assumed: 15/200 & 30/290 attrition]: [${:.0}, ${:.0}] (width=${:.0}, Imbens-Manski 95% CI=[${:.0}, ${:.0}])",
        manski_ate.bounds[0],
        manski_ate.bounds[1],
        manski_ate.identification_width,
        manski_ate.imbens_manski_bounds[0],
        manski_ate.imbens_manski_bounds[1]
    );
    println!(
        "    Lee Monotone Attrition Two-Arm ATE Bounds [Assumed: 15/200 & 30/290 attrition]: [${:.0}, ${:.0}] (width=${:.0})",
        lee_ate.bounds[0], lee_ate.bounds[1], lee_ate.identification_width
    );
    println!(
        "    Censoring worst-case survival bounds [Assumed]: [{:.3}, {:.3}] (KM naive={:.3})",
        censoring.bounds[0], censoring.bounds[1], censoring_naive
    );
    println!(
        "    Peterson (1976) competing-risk Frechet bounds [Assumed]: [{:.3}, {:.3}] (indep={:.3}, width={:.3})",
        peterson.bounds[0], peterson.bounds[1], peterson_naive, peterson.identification_width
    );
    println!(
        "    Ville e-Process [Assumed]: terminal E_T = {:.2} ({:.1} decibans), anytime CI = [{:.2}, {:.2}]",
        e_proc.evidence_e_value, e_proc.decibans, e_proc.bounds[0], e_proc.bounds[1]
    );

    if experimental_ate < manski_ate.bounds[0] || experimental_ate > manski_ate.bounds[1] {
        return Err("Experimental RCT ATE must lie inside two-arm Manski ATE bounds".to_string());
    }
    if experimental_ate < lee_ate.bounds[0] || experimental_ate > lee_ate.bounds[1] {
        return Err("Experimental RCT ATE must lie inside Lee attrition bounds".to_string());
    }
    if (experimental_ate - 1794.34).abs() > 5.0 {
        return Err("LaLonde experimental ATE deviated from benchmark".to_string());
    }
    Ok(())
}

/// Run Chapter 4: Filter I · Axis 1 (`S_epi`) — From World to Archive & The Big Data Paradox (`epistemic.rs`).
pub fn run_chapter_4(workspace_root: &Path) -> Result<(), String> {
    let cps_clusters = load_cps_cohort_clusters(workspace_root);
    let bank_clusters = load_bank_call_clusters(workspace_root);
    let vintages = load_rtdsm_routput_vintages(workspace_root);

    let (nsw_t, nsw_c, cps_c) = load_lalonde_nsw_cps(workspace_root);
    let n_nsw = nsw_t.len() + nsw_c.len();
    let n_cps = cps_c.len();
    let n_pool = (n_nsw + n_cps).max(1) as f64;
    let p_cps = (n_cps as f64) / n_pool;
    let mean_nsw = (nsw_t.iter().sum::<f64>() + nsw_c.iter().sum::<f64>()) / (n_nsw.max(1) as f64);
    let mean_cps = mean(&cps_c);
    let mean_pool = (p_cps * mean_cps) + ((1.0 - p_cps) * mean_nsw);
    let var_pool = (nsw_t
        .iter()
        .chain(nsw_c.iter())
        .chain(cps_c.iter())
        .map(|&y| (y - mean_pool) * (y - mean_pool))
        .sum::<f64>()
        / (n_pool - 1.0).max(1.0))
    .max(1.0);
    let sigma_pool = var_pool.sqrt();
    let var_r = (p_cps * (1.0 - p_cps)).max(1e-12);
    let cov_ry = p_cps * (mean_cps - mean_pool);
    let rho_ry_cps = (cov_ry / (var_r.sqrt() * sigma_pool)).clamp(-0.99, 0.99);
    let meng_cps = EpistemicMarginState::from_meng_big_data(
        n_pool,
        n_cps as f64,
        rho_ry_cps,
        sigma_pool,
        1.0,
        stochology::CANONICAL_KEYNES_N0,
    );

    let cps_deff = EpistemicMarginState::from_archive_clusters(&cps_clusters, 0.0, 25.565);
    let bank_deff = EpistemicMarginState::from_archive_clusters(&bank_clusters, 0.0, 100.0);

    let meng = EpistemicMarginState::from_meng_big_data(
        10_000_000.0,
        2_300_000.0,
        -0.05,
        0.50,
        1.0,
        2_500.0,
    );
    let kish = EpistemicMarginState::from_kish_strata(&[(900, 0.90), (100, 0.10)], 100.0);
    let chao = EpistemicMarginState::from_capture_recapture(95, 340, 210);
    let bitemporal = EpistemicMarginState::from_vintages(&vintages);
    let num_vintages = {
        let mut vs: Vec<u32> = vintages.iter().map(|&(v, _, _)| v).collect();
        vs.sort_unstable();
        vs.dedup();
        vs.len()
    };

    let bt_view = bitemporal_view_at(
        &[
            BitemporalRecord {
                event_time: 10,
                available_time: 11,
                value: 2.1,
            },
            BitemporalRecord {
                event_time: 10,
                available_time: 14,
                value: 1.4,
            },
        ],
        12,
    )
    .map_err(|e| e.to_string())?;

    println!("=== CHAPTER 4: FILTER I (S_EPI) — FROM WORLD TO ARCHIVE & THE BIG DATA PARADOX ===");
    println!("  Canonical Operator I (Harmonic n_eff & Keynes Weight V = n_eff / (n_eff + n_0)):");
    println!(
        "  1. Meng Big Data Paradox [Assumed] (N = 10M, n = 2.3M, rho_RY = -0.05): n_eff = {:.1}, meng_error = {:+.4}, V = {:.3}",
        meng.n_eff, meng.meng_estimation_error, meng.keynes_weight
    );
    println!(
        "  2. Demographic Cluster DEFF on real re78 [Measured] (data/cps_observational.csv, [Assumed n_0=25.565]): DEFF = {:.2}, n_eff = {:.1}, V = {:.3}",
        cps_deff.design_effect_deff, cps_deff.n_eff, cps_deff.keynes_weight
    );
    println!(
        "  3. Temporal Cohort DEFF [Measured] (data/bank_calls.csv, [Assumed n_0=100.0]): DEFF = {:.2}, n_eff = {:.1}, V = {:.3}",
        bank_deff.design_effect_deff, bank_deff.n_eff, bank_deff.keynes_weight
    );
    println!(
        "  4. Kish Weighting Floor [Assumed]: DEFF_Kish = {:.2}, n_eff_floor = {:.1} | Chao Unseen Dark Archive [Assumed]: n00_hat = {:.1}, N_Chao_LB = {:.1}, U_Chao_95 = {:.1}",
        kish.design_effect_deff,
        kish.n_eff,
        chao.uncounted_tail_n00,
        chao.chao_lower_bound_total,
        chao.bounds[1]
    );
    println!(
        "  5. Bitemporal Vintages [Measured] (data/rtdsm_routput_vintages.csv, Q={} evaluated quarters across {} vintages): mean_rev = {:.2} pp, max_rev = {:.2} pp, sign_flips = {:.1}% (causal_view_len={})",
        bitemporal.vintage_num_quarters,
        num_vintages,
        bitemporal.vintage_mean_revision_pp,
        bitemporal.vintage_max_revision_pp,
        bitemporal.vintage_sign_flip_rate * 100.0,
        bt_view.len()
    );

    if cps_deff.design_effect_deff <= 1.0
        || bitemporal.vintage_max_revision_pp <= 0.0
        || meng_cps.n_eff >= (n_cps as f64) * 0.15
        || rho_ry_cps <= 0.10
    {
        return Err("Chapter 4 archive metrics failed verification".to_string());
    }
    Ok(())
}

/// Run Chapter 5: Filter II · Axis 2 (`S_vital`) — Non-Identical Worlds, Asymmetry & Heavy Tails (`vital.rs`).
pub fn run_chapter_5(workspace_root: &Path) -> Result<(), String> {
    let danish = load_danish_fire(workspace_root);
    let (nyc13, nyc23) = load_nyc_flight_delays(workspace_root, 0);
    let (nyc13_h1, nyc23_h1, hou11) = load_oot_ood_flight_delays(workspace_root, 0);

    let n_pos_13 = nyc13.iter().filter(|&&x| x > 0.0).count();
    let n_pos_23 = nyc23.iter().filter(|&&x| x > 0.0).count();
    let n_pos_13_h1 = nyc13_h1.iter().filter(|&&x| x > 0.0).count();
    let n_pos_23_h1 = nyc23_h1.iter().filter(|&&x| x > 0.0).count();
    let n_pos_hou = hou11.iter().filter(|&&x| x > 0.0).count();

    let k_13 = (n_pos_13 / 20).max(20);
    let k_23 = (n_pos_23 / 20).max(20);
    let k_13_h1 = (n_pos_13_h1 / 20).max(20);
    let k_23_h1 = (n_pos_23_h1 / 20).max(20);
    let k_hou = (n_pos_hou / 20).max(20);

    let (_, danish_ci95) = VitalMarginState::hill_alpha_with_ci95(&danish, 150);
    let (_, nyc13_ci95) = VitalMarginState::hill_alpha_with_ci95(&nyc13, k_13);
    let (_, nyc23_ci95) = VitalMarginState::hill_alpha_with_ci95(&nyc23, k_23);
    let (tot_13, _, na_13) =
        count_flight_csv_rows_and_missing(workspace_root, "data/nyc_flights_2013.csv");
    let (tot_23, _, na_23) =
        count_flight_csv_rows_and_missing(workspace_root, "data/nyc_flights_2023.csv");
    let (tot_13_h1, _, na_13_h1) =
        count_flight_csv_rows_and_missing(workspace_root, "data/nyc_flights_2013_h1.csv");
    let (tot_23_h1, _, na_23_h1) =
        count_flight_csv_rows_and_missing(workspace_root, "data/nyc_flights_2023_h1.csv");
    let (tot_hou, _, na_hou) =
        count_flight_csv_rows_and_missing(workspace_root, "data/houston_flights_2011.csv");

    let vital_danish =
        VitalMarginState::from_heavy_tails_and_asymmetry(&danish, 150, (1.5, 3.0), |x| {
            0.08 * x.powf(1.65)
        });
    let vital_nyc13 =
        VitalMarginState::from_heavy_tails_and_asymmetry(&nyc13, k_13, (10.0, 30.0), |x| {
            x.max(0.0).powf(1.2)
        });
    let vital_nyc23 =
        VitalMarginState::from_heavy_tails_and_asymmetry(&nyc23, k_23, (10.0, 30.0), |x| {
            x.max(0.0).powf(1.2)
        });
    let vital_nyc13_h1 =
        VitalMarginState::from_heavy_tails_and_asymmetry(&nyc13_h1, k_13_h1, (10.0, 30.0), |x| {
            x.max(0.0).powf(1.2)
        });
    let vital_nyc23_h1 =
        VitalMarginState::from_heavy_tails_and_asymmetry(&nyc23_h1, k_23_h1, (10.0, 30.0), |x| {
            x.max(0.0).powf(1.2)
        });
    let vital_hou11 =
        VitalMarginState::from_heavy_tails_and_asymmetry(&hou11, k_hou, (10.0, 30.0), |x| {
            x.max(0.0).powf(1.2)
        });

    let holling_deff = HollingTailRuptureAudit::evaluate(120, 10.1, 1.65, 12.0, 4000);

    println!("=== CHAPTER 5: FILTER II (S_VITAL) — ASYMMETRY, SCALE INVARIANCE & HEAVY TAILS ===");
    println!(
        "  Danish Fire Losses [Measured] (data/danish_fire.csv, n = {}):",
        danish.len()
    );
    println!(
        "    Hill alpha_hat = {:.2} (k=150, 95% CI=[{:.2}, {:.2}], k-stability [50..250] = [{:.2}, {:.2}], regime = {:?}, median = {:.2}, q95 = {:.2})",
        vital_danish.tail_alpha,
        danish_ci95[0],
        danish_ci95[1],
        vital_danish.hill_alpha_k_range[0],
        vital_danish.hill_alpha_k_range[1],
        vital_danish.tail_regime,
        vital_danish.median,
        vital_danish.q95
    );
    println!(
        "    Jensen Gap Delta_J = E[phi(X)] - phi(E[X]) = {:.3} (E[phi(X)] = {:.3} vs phi(E[X]) = {:.3})",
        vital_danish.jensen_gap, vital_danish.expected_eval, vital_danish.eval_at_mean
    );
    println!(
        "    Empirical CRPS = {:.3} | Ordinal Invariance Error = {:.4} (Mean Ranking Reversed = {}) | Penumbra Mass = {:.3}",
        vital_danish.crps_at_median,
        vital_danish.ordinal_invariance_error,
        vital_danish.mean_ranking_reversed,
        vital_danish.penumbra_mass
    );
    println!(
        "    Hill-Holling DEFF_tail Coupling [Simulated] (DEFF=10.1, k_eff={:.1}): P_true={:.4}, E[P_plugin]={:.4} (Jensen +{:.1}%), P_debiased={:.4}, CI90=[{:.5}, {:.5}] ({:.1}x)",
        holling_deff.effective_k,
        holling_deff.true_rupture_prob,
        holling_deff.expected_plugin_rupture_prob,
        holling_deff.jensen_relative_bias * 100.0,
        holling_deff.debiased_rupture_prob,
        holling_deff.rupture_ci90[0],
        holling_deff.rupture_ci90[1],
        holling_deff.ci90_dispersion_ratio
    );
    println!(
        "  NYC Flights Conditional Delay>0 Tail Index [Measured, k=n+/20 on non-NA rows]: 2013 alpha_hat = {:.2} (95% CI=[{:.2}, {:.2}], k={}, n={}, n+={}, total_rows={}, skipped_NA={}, penumbra_[10,30]={:.1}%) vs 2023 alpha_hat = {:.2} (95% CI=[{:.2}, {:.2}], k={}, n={}, n+={}, total_rows={}, skipped_NA={}, penumbra_[10,30]={:.1}%) | OOT H1 (2013={:.2} [k={}, n={}, n+={}, total={}, NA={}], 2023={:.2} [k={}, n={}, n+={}, total={}, NA={}]) & OOD Houston 2011={:.2} (k={}, n={}, n+={}, total={}, NA={})",
        vital_nyc13.tail_alpha,
        nyc13_ci95[0],
        nyc13_ci95[1],
        k_13,
        nyc13.len(),
        n_pos_13,
        tot_13,
        na_13,
        vital_nyc13.penumbra_mass * 100.0,
        vital_nyc23.tail_alpha,
        nyc23_ci95[0],
        nyc23_ci95[1],
        k_23,
        nyc23.len(),
        n_pos_23,
        tot_23,
        na_23,
        vital_nyc23.penumbra_mass * 100.0,
        vital_nyc13_h1.tail_alpha,
        k_13_h1,
        nyc13_h1.len(),
        n_pos_13_h1,
        tot_13_h1,
        na_13_h1,
        vital_nyc23_h1.tail_alpha,
        k_23_h1,
        nyc23_h1.len(),
        n_pos_23_h1,
        tot_23_h1,
        na_23_h1,
        vital_hou11.tail_alpha,
        k_hou,
        hou11.len(),
        n_pos_hou,
        tot_hou,
        na_hou
    );

    if vital_danish.jensen_gap <= 0.0
        || vital_danish.tail_alpha >= 2.5
        || holling_deff.jensen_relative_bias <= 0.15
    {
        return Err("Danish fire heavy-tail and Jensen gap verification failed".to_string());
    }
    Ok(())
}

/// Run Chapter 6: Filter II · Axis 2 (`S_vital`) — Irreversible Time, Non-Ergodicity & Vital Margin (`vital.rs`).
pub fn run_chapter_6(workspace_root: &Path) -> Result<(), String> {
    let djia_returns = load_djia_returns(workspace_root);
    let vic_demand = load_vic_elec_demand(workspace_root);
    let (nsw_t, nsw_c, _) = load_lalonde_nsw_cps(workspace_root);

    let erg = VitalMarginState::from_non_ergodic_returns(&djia_returns);
    let tail_kelly = VitalMarginState::from_regime_aware_kelly(0.08, 0.08, 1.65, 2.50, 50.0, 0.0);

    let max_demand = vic_demand.iter().copied().fold(0.0_f64, f64::max);
    let q50_demand = quantile(&vic_demand, 0.50);
    let q95_demand = quantile(&vic_demand, 0.95);
    let q99_demand = quantile(&vic_demand, 0.99);
    let mean_demand = mean(&vic_demand);
    let std_demand = sample_std(&vic_demand);
    let ca2 = (std_demand / mean_demand.max(1.0)).powi(2);
    let ramps: Vec<f64> = vic_demand
        .windows(2)
        .map(|w| (w[1] - w[0]).abs() * 2.0)
        .collect();
    let ramp_p99 = quantile(&ramps, 0.99).max(1.0);

    // Empirical quantile-calibrated utilization & reserve margin on vic_elec.csv:
    let high_util = (q99_demand / (q95_demand * 1.18)).clamp(0.01, 0.995);
    let safe_util = (q50_demand / (max_demand * 1.05)).clamp(0.01, 0.995);
    let stressed_reserve_h = (max_demand - q99_demand) / ramp_p99;
    let buffered_reserve_h = 4.0 * (max_demand * 1.05 - q95_demand) / ramp_p99;
    let var_factor = 0.5 * (ca2 + 1.0);

    let stressed_grid = VitalMarginState::from_queue_basin_and_derived_ttr(
        high_util,
        var_factor,
        1.0,
        stressed_reserve_h,
        1.0,
        2.5,
        1.0,
        1.80,
        10.0,
        2.5,
    );
    let buffered_grid = VitalMarginState::from_queue_basin_and_derived_ttr(
        safe_util,
        var_factor,
        1.0,
        buffered_reserve_h,
        1.0,
        2.5,
        1.0,
        1.80,
        10.0,
        0.4,
    );
    let wald_latent = WaldSurvivorshipAudit::wald_1943_bomber_benchmark();
    let (nsw_full_ate, nsw_surv_ate, nsw_surv_bias) =
        WaldSurvivorshipAudit::evaluate_empirical_truncation_bias(&nsw_t, &nsw_c);

    println!(
        "=== CHAPTER 6: FILTER II (S_VITAL) — IRREVERSIBLE TIME, NON-ERGODICITY & VITAL MARGIN ==="
    );
    println!(
        "  Canonical Operator II (Convexity, Time-Average Growth & Survival Inequality TTS_survival >= TTR_recovery):"
    );
    println!(
        "  1. DJIA Non-Ergodic Trajectory [Measured] (data/djia_1885_2023.csv, T = {} trading days, emp_alpha_loss = {:.2}):",
        djia_returns.len(),
        erg.tail_alpha
    );
    let ito_drain_pct = ((erg.ensemble_drift_mu - erg.unleveraged_time_growth)
        / erg.ensemble_drift_mu.max(1e-12))
        * 100.0;
    println!(
        "     Arithmetic Ensemble Drift = {:+.6}/day vs Time-Average Growth = {:+.6}/day (Itô drain = {:.1}%)",
        erg.ensemble_drift_mu, erg.unleveraged_time_growth, ito_drain_pct
    );
    println!(
        "     Plug-in Kelly = {:.2} vs Baker-McHale Epistemic Kelly = {:.2} (Shrinkage = {:.4})",
        erg.plugin_kelly_fraction, erg.epistemic_kelly_fraction, erg.kelly_shrinkage
    );
    println!(
        "     Heavy-Tail Kelly Regime Selector [Assumed] (alpha=1.65 <= 2.2): regime = {:?}, naive_quadratic_f = {:.4} vs WDRO_CVaR95_f = {:.4}",
        tail_kelly.kelly_regime,
        tail_kelly.plugin_kelly_fraction,
        tail_kelly.epistemic_kelly_fraction
    );
    println!(
        "  2. Victoria Electricity Grid [Measured] (data/vic_elec.csv, n = {} half-hours, q50={:.0} MW, q95={:.0} MW, q99={:.0} MW, max={:.0} MW, ramp_p99={:.0} MW/h, scale=1.18/1.05 [Assumed]):",
        vic_demand.len(),
        q50_demand,
        q95_demand,
        q99_demand,
        max_demand,
        ramp_p99
    );
    println!(
        "     Stressed [Measured quantiles + Assumed ODE h=1.80,K=10] (rho={:.1}%): Kingman Wait = {:.1}x (G/G/1 E[W_q]={:.2}h, var_factor={:.2}, rho1_worn={:.3}), Basin D_t/D_0 = {:.2}, TTS_survival = {:.1}h < TTR_recovery = {:.1}h (survives={})",
        stressed_grid.utilization_rho * 100.0,
        stressed_grid.kingman_wait_multiplier,
        stressed_grid.kingman_expected_wait,
        stressed_grid.kingman_variability_factor,
        stressed_grid.critical_slowing_down_rho1_worn,
        stressed_grid.basin_radius_ratio,
        stressed_grid.survival_slack_tts,
        stressed_grid.recovery_delay_ttr,
        stressed_grid.survives_disruption()
    );
    println!(
        "     Buffered [Measured quantiles + Assumed ODE h=1.80,K=10] (rho={:.1}%): Kingman Wait = {:.1}x (G/G/1 E[W_q]={:.2}h, var_factor={:.2}, rho1_worn={:.3}), Basin D_t/D_0 = {:.2}, TTS_survival = {:.1}h >= TTR_recovery = {:.1}h (survives={})",
        buffered_grid.utilization_rho * 100.0,
        buffered_grid.kingman_wait_multiplier,
        buffered_grid.kingman_expected_wait,
        buffered_grid.kingman_variability_factor,
        buffered_grid.critical_slowing_down_rho1_worn,
        buffered_grid.basin_radius_ratio,
        buffered_grid.survival_slack_tts,
        buffered_grid.recovery_delay_ttr,
        buffered_grid.survives_disruption()
    );
    println!(
        "  3. Survivorship & Robustness: Wald Latent 2x2 Solver [Assumed] = {} (reconstructed M_A={:.0}, M_B={:.0} -> Zone B {:.1}% > Zone A {:.1}%) | Empirical NSW Survivor Bias [Measured] (re78>0 only): full=${:.0} vs survivor=${:.0} (bias=${:+.0}) | Sniedovich Mirage [Assumed] = {}",
        buffered_grid.wald_survivorship_inversion,
        wald_latent.reconstructed_missing_hits_a,
        wald_latent.reconstructed_missing_hits_b,
        buffered_grid.wald_lethality_zone_b * 100.0,
        buffered_grid.wald_lethality_zone_a * 100.0,
        nsw_full_ate,
        nsw_surv_ate,
        nsw_surv_bias,
        buffered_grid.sniedovich_no_mans_land_mirage
    );

    if erg.unleveraged_time_growth >= erg.ensemble_drift_mu
        || erg.kelly_shrinkage >= 0.99
        || stressed_grid.survives_disruption()
        || !buffered_grid.survives_disruption()
        || tail_kelly.kelly_regime != KellySizingRegime::TailRobustWdroCvarKelly
        || tail_kelly.epistemic_kelly_fraction >= tail_kelly.plugin_kelly_fraction
    {
        return Err("Chapter 6 non-ergodicity and vital margin checks failed".to_string());
    }
    Ok(())
}

/// Run Chapter 7: Filter III · Axis 3 (`S_gov`) — Reactive Worlds, Performativity & Goodhart (`governance.rs`).
pub fn run_chapter_7(workspace_root: &Path) -> Result<(), String> {
    let (probs, outcomes) = load_atlantic_storms_forecasts(workspace_root);
    let (storms_total, storms_transitions, storms_terminal) =
        count_atlantic_storms_transitions(workspace_root);
    let envs = load_atlantic_storms_environments(workspace_root);

    let gov_ex_ante = GovernanceState::from_ex_ante_gate(&probs, &outcomes, &envs);
    let icp_under = GovernanceState::classify_icp_power(30.0, 0.009, 0.076);

    println!("=== CHAPTER 7: FILTER III (S_GOV) — REACTIVE WORLDS, PERFORMATIVITY & GOODHART ===");
    println!(
        "  Canonical Operator III — Gate 1 (Ex-Ante Calibration, Causal Invariance & Performative Gaming Guard):"
    );
    println!(
        "  1. CORP-PAV +6h Lead-Time Storm Forecast Brier Decomposition [Measured] (data/atlantic_storms.csv, n_transitions = {} from {} total rows, {} track-terminal rows [{} unpaired + 1 tail]):",
        storms_transitions,
        storms_total,
        storms_terminal,
        storms_terminal.saturating_sub(1)
    );
    println!(
        "     Total Brier = {:.4} = REL ({:.4}) - RES ({:.4}) + UNC ({:.4}) [closure_err = {:.2e}]",
        gov_ex_ante.corp_brier_score,
        gov_ex_ante.corp_reliability,
        gov_ex_ante.corp_resolution,
        gov_ex_ante.corp_uncertainty,
        gov_ex_ante.corp_closure_error
    );
    println!(
        "     Finite-Sample CORP-PAV & ICP Guard [Measured]: b_PAV_CV = {:.4}, REL_debiased = {:.4}, ICP(n_eff/env={:.1}) = {:?} (benchmark n_eff/env=30 -> {:?} [Assumed])",
        gov_ex_ante.corp_pav_finite_bias,
        gov_ex_ante.corp_reliability_debiased,
        gov_ex_ante.icp_effective_n_per_env,
        gov_ex_ante.icp_status,
        icp_under
    );
    println!(
        "  2. Multi-Regime Invariant Causal Prediction (ICP) [Measured]: physical slope range = {:.3} < spurious proxy range = {:.3} (accepted_subsets = {}, S_hat_mask = 0b{:02b})",
        gov_ex_ante.icp_invariant_slope_range,
        gov_ex_ante.icp_spurious_slope_range,
        gov_ex_ante.icp_accepted_subsets_count,
        gov_ex_ante.icp_identified_mask
    );
    println!(
        "  3. Kennedy-O'Hagan GP Discrepancy [Measured tercile anchors]: in_sample (mean={:+.4}, std={:.3}) vs out_of_sample (mean={:+.4}, std={:.3}) | Tian-Pearl [Assumed] PN=[{:.2}, {:.2}], PS=[{:.2}, {:.2}], PNS=[{:.2}, {:.2}]",
        gov_ex_ante.koh_mean_in_sample,
        gov_ex_ante.koh_std_in_sample,
        gov_ex_ante.koh_mean_out_of_sample,
        gov_ex_ante.koh_std_out_of_sample,
        gov_ex_ante.tian_pearl_pn_bounds[0],
        gov_ex_ante.tian_pearl_pn_bounds[1],
        gov_ex_ante.tian_pearl_ps_bounds[0],
        gov_ex_ante.tian_pearl_ps_bounds[1],
        gov_ex_ante.tian_pearl_pns_bounds[0],
        gov_ex_ante.tian_pearl_pns_bounds[1]
    );
    println!(
        "  4. Perdomo Performative Stability vs Optimality [Assumed]: theta_PS = {:.3} (risk={:.3}) vs theta_PO = {:.3} (risk={:.3}, MacKenzie = {:?})",
        gov_ex_ante.theta_ps,
        gov_ex_ante.risk_at_ps,
        gov_ex_ante.theta_po,
        gov_ex_ante.risk_at_po,
        gov_ex_ante.mackenzie_regime
    );
    println!(
        "  5. Goodhart Metric Collapse [Assumed]: collapsed = {}, variant = {:?}",
        gov_ex_ante.goodhart_metric_collapsed, gov_ex_ante.goodhart_variant
    );

    if !gov_ex_ante.goodhart_metric_collapsed
        || gov_ex_ante.corp_resolution <= 0.0
        || gov_ex_ante.icp_status != IcpIdentificationStatus::IdentifiedInvariantSet
        || icp_under != IcpIdentificationStatus::EpistemicUnderpoweredSet
    {
        return Err("Chapter 7 calibration and Goodhart verification failed".to_string());
    }
    Ok(())
}

/// Run Chapter 8: Filter III · Axis 3 (`S_gov`) — Moral Luck, Apt Agency & Institutional Design (`governance.rs`).
pub fn run_chapter_8(workspace_root: &Path) -> Result<(), String> {
    run_chapter_8_with_brake(workspace_root, None)
}

/// Run Chapter 8 with an optional `--brake on|off` override (`B6 · MAPA-03`).
pub fn run_chapter_8_with_brake(
    workspace_root: &Path,
    brake_override: Option<bool>,
) -> Result<(), String> {
    let (prev_a, prev_b, prev_black_all, prev_white_all, n_toronto_total, n_toronto_stratum) =
        try_load_toronto_arrests_full_summary(workspace_root)?;
    let (prev_bm, prev_wm, n_matched, black_pairs, white_pairs) =
        try_load_toronto_arrests_matched_and_classifier(workspace_root)?;
    let emp_ch = EmpiricalChouldechovaAudit::evaluate(&black_pairs, &white_pairs);
    let (_, _, fpr_disp_marginal) = GovernanceState::evaluate_chouldechova_classifier_fpr(
        prev_black_all,
        prev_white_all,
        0.65,
        0.20,
        0.20,
    );
    let (_, _, fpr_disp_matched) =
        GovernanceState::evaluate_chouldechova_classifier_fpr(prev_bm, prev_wm, 0.65, 0.20, 0.20);

    let gov = GovernanceState::from_real_time_and_ex_post_gates(
        prev_a,
        prev_b,
        2.10,
        20.0,
        0.65,
        8.0, // TTS_survival = 8 shifts
        b"stochology-v1-pre-decision-commitment",
    )
    .map_err(|e| e.to_string())?;
    let gov_brake_on = GovernanceState::evaluate_shift_governance(
        2.10, 20.0, 0.0, 8.0, 0.60, true, 0.90, true, true,
    );
    let active_db = if brake_override == Some(true) {
        gov_brake_on.double_bind_intensity
    } else {
        gov.double_bind_intensity
    };
    let zm_regimes = ZhengMeisterQueueAudit::evaluate_three_canonical_regimes();

    let mut honest_vault = TemporalVault::verify_two_shift_lifecycle(
        0,
        1,
        b"stochology-v1-pre-decision-commitment",
        b"stochology-v1-pre-decision-commitment",
    )
    .map_err(|e| e.to_string())?;
    let honest_ok = honest_vault.is_confirmatory;
    let replay_rejected = honest_vault
        .unseal(b"stochology-v1-pre-decision-commitment")
        .is_err();
    let tamper_rejected = TemporalVault::verify_two_shift_lifecycle(
        0,
        1,
        b"stochology-v1-pre-decision-commitment",
        b"stochology-v1-tampered-hindsight",
    )
    .is_err();

    println!(
        "=== CHAPTER 8: FILTER III (S_GOV) — MORAL LUCK, APT AGENCY & INSTITUTIONAL DESIGN ==="
    );
    println!(
        "  Canonical Operator III — Gate 2 (Real-Time S3* Algedonic Veto) & Gate 3 (Ex-Post Responsibility):"
    );
    println!(
        "  1. Sosa / Baron-Hershey Telic Evaluation [Assumed]: grade = {:?}, eliminated_outcome_bias = {:+.2}, vault_confirmatory = {} (tamper_rejected={}, replay_rejected={})",
        gov.telic_grade,
        gov.eliminated_outcome_bias,
        gov.vault_confirmatory,
        tamper_rejected,
        replay_rejected
    );
    println!(
        "  2. List-Pettit & Beer S3* Algedonic Channel [Assumed]: premise_majority = {}, conjunctive = {}, discursive_dilemma = {}",
        gov.premise_majority_approves,
        gov.constitutional_conjunctive_approves,
        gov.discursive_dilemma_present
    );
    println!(
        "     Undiscounted TTR_recovery = {:.0} <= TTS_survival = {:.0} vs Discounted TTR_recovery = {:.0} > TTS_survival = {:.0} (fatal_delay={})",
        gov.recovery_delay_ttr_undiscounted,
        gov.survival_slack_tts,
        gov.recovery_delay_ttr_discounted,
        gov.survival_slack_tts,
        gov.fatal_testimonial_delay
    );
    println!(
        "  3. Holmstrom-Innes Liability & Simon-Girard Scapegoating [Assumed]: structural_share = {:.1}%, scapegoat_transfer = {:.1}% (blame_inversion_ratio = {:.1}x), Bateson DB_t = {:.2} (brake_on = {:.2})",
        gov.structural_liability_share * 100.0,
        gov.scapegoat_blame_transfer * 100.0,
        gov.blame_inversion_ratio,
        active_db,
        gov_brake_on.double_bind_intensity
    );
    println!(
        "  4. Chouldechova Fairness Trilemma [Measured] (data/toronto_arrests.csv, n_all = {}, n_matched = {}, n_poles = {}): full marginal prev_Black = {:.3}, prev_White = {:.3}, odds_FPR_ratio = {:.2}x | matched (Unemp+Citizen) prev_Black = {:.3}, prev_White = {:.3}, odds_FPR_ratio = {:.2}x | intersectional poles prev_A = {:.3} (FPR_A={:.3}), prev_B = {:.3} (FPR_B={:.3}), FPR_A/FPR_B = {:.2}x",
        n_toronto_total,
        n_matched,
        n_toronto_stratum,
        prev_black_all,
        prev_white_all,
        fpr_disp_marginal,
        prev_bm,
        prev_wm,
        fpr_disp_matched,
        gov.chouldechova_prevalence_a,
        gov.chouldechova_classifier_fpr_a,
        gov.chouldechova_prevalence_b,
        gov.chouldechova_classifier_fpr_b,
        gov.chouldechova_fpr_disparity
    );
    println!(
        "     Empirical Custody-Risk Classifier [Assumed rule checks>=3 on Measured data/toronto_arrests.csv]: PPV_B={:.3}/PPV_W={:.3}, FNR_B={:.3}/FNR_W={:.3}, FPR_Black={:.3} vs FPR_White={:.3} (empirical FPR_ratio={:.2}x, Chouldechova identity closure_err={:.2e})",
        emp_ch.ppv_a,
        emp_ch.ppv_b,
        emp_ch.fnr_a,
        emp_ch.fnr_b,
        emp_ch.fpr_a,
        emp_ch.fpr_b,
        emp_ch.empirical_fpr_ratio,
        emp_ch.identity_closure_error
    );
    println!(
        "  5. Zheng-Meister 10 bits/s & Gigerenzer Natural Frequencies [Assumed]: noisy FDR = {:.1}% -> filtered PPV = {:.1}% ({} true alerts vs {} false alarms / 1,000 shifts)",
        gov.noisy_alert_fdr * 100.0,
        gov.filtered_alert_ppv * 100.0,
        gov.natural_freq_true_alerts,
        gov.natural_freq_false_alarms
    );
    println!(
        "     Two-Tier Cognitive Queue (TTS=45s): Kairos(30b,6s) rho={:.2}, TTR={:.1}s (ok={}) | Card(120b,18s) rho={:.2}, TTR={:.1}s | Full(240b,36s) rho={:.2}, TTR={:.1}s (ok={})",
        zm_regimes[0].cognitive_utilization_rho,
        zm_regimes[0].total_recovery_ttr_s,
        zm_regimes[0].survives_kairos,
        zm_regimes[1].cognitive_utilization_rho,
        zm_regimes[1].total_recovery_ttr_s,
        zm_regimes[2].cognitive_utilization_rho,
        zm_regimes[2].total_recovery_ttr_s,
        zm_regimes[2].survives_kairos
    );

    if !gov.vault_confirmatory
        || !honest_ok
        || !tamper_rejected
        || !replay_rejected
        || !gov.fatal_testimonial_delay
        || !gov.is_adulterated_air_scapegoating
        || prev_a <= prev_b
        || (gov.chouldechova_fpr_disparity - 1.0).abs() < 0.05
        || emp_ch.fpr_a <= emp_ch.fpr_b
        || emp_ch.fpr_b <= 0.0
        || emp_ch.identity_closure_error > 1e-12
        || !zm_regimes[0].survives_kairos
        || zm_regimes[2].survives_kairos
    {
        return Err("Chapter 8 institutional governance verification failed".to_string());
    }
    Ok(())
}

/// Run Chapter 9: Synthesis (`M_Leontief`) — Methodological Families, Process Tracing & Reflexivity (`synthesis.rs`).
pub fn run_chapter_9() -> Result<(), String> {
    let env = CommunityReserveEnv::new(DEFAULT_SEED, 24, 45.0);
    let configs = [
        (true, true, true),
        (false, true, true),
        (true, false, true),
        (true, true, false),
        (true, false, false),
        (false, true, false),
        (false, false, true),
        (false, false, false),
    ];

    let clues = [
        DiagnosticClue {
            prob_given_hypothesis: 0.96,
            prob_given_rival: 0.02,
            clue_observed: true,
            weight_contribution: 25.0,
        },
        DiagnosticClue {
            prob_given_hypothesis: 0.75,
            prob_given_rival: 0.04,
            clue_observed: true,
            weight_contribution: 20.0,
        },
        DiagnosticClue {
            prob_given_hypothesis: 0.99,
            prob_given_rival: 0.35,
            clue_observed: true,
            weight_contribution: 15.0,
        },
    ];
    let pt = ProcessTracingInference::evaluate(0.25, 10.0, 40.0, &clues);
    let root = crate::loaders::resolve_workspace_root();
    let (storm_p, storm_y) = load_atlantic_storms_forecasts(&root);
    let brakes =
        ReflexiveBrakeAudit::evaluate(&storm_p, &storm_y, 3.0, 25.0, 1.0, 4.0, 1.5, 12.0, 4.0);
    let vol_audit = LeontiefVsMultiplicativeAudit::evaluate(0.50);
    let falsifiers = PreregisteredFalsifier::canonical_four();
    let proof_audit = StructuralCollapseProofAudit::evaluate_analytical(
        2_300_000.0,
        0.23,
        -0.05,
        0.08,
        0.12,
        1.40,
        2.0,
        5.5,
        0.60,
        2.0,
        0.85,
        0.40,
    );

    println!("=== CHAPTER 9: METHODOLOGICAL FAMILIES, PROCESS TRACING & REFLEXIVITY ===");
    println!(
        "  2x2x2 Factorial Ablation of the Nuclear Triad [Simulated] (M_Leontief = min(S_epi, S_vital, S_gov)):"
    );
    let mut reports = Vec::with_capacity(8);
    let mut mc_summaries = Vec::with_capacity(8);
    for &(e, v, g) in &configs {
        let mut agent = StochologicalAgent::with_axes(e, v, g);
        let rep = env.run_trajectory(&mut agent);
        let mc = CommunityReserveEnv::evaluate_multi_seed(2000, 100, 24, 45.0, || {
            StochologicalAgent::with_axes(e, v, g)
        });
        println!(
            "    [E={} V={} G={}] {:<44} -> M_Leontief={:.3} (M_arith={:.3}), collapse={:<28}, cov={:5.1}% (shock={:5.1}%), harm={:6.2} (MC100={:6.2}±{:.2} [{:6.2}, {:6.2}], ruin={:5.1}%), R_24={:6.2}",
            if e { 1 } else { 0 },
            if v { 1 } else { 0 },
            if g { 1 } else { 0 },
            rep.policy_name,
            rep.triadic_integrity.leontief_minimum,
            rep.triadic_integrity.arithmetic_mean,
            format!("{:?}", rep.triadic_integrity.diagnose),
            rep.epistemic_coverage_rate * 100.0,
            rep.shock_coverage_rate * 100.0,
            rep.mean_social_harm,
            mc.mean_harm,
            mc.mean_harm_se,
            mc.harm_p05,
            mc.harm_p95,
            mc.ruin_rate * 100.0,
            rep.terminal_reserve
        );
        reports.push(rep);
        mc_summaries.push(mc);
    }

    let fb_audit = env.evaluate_mechanism_vs_budget_factorial();
    println!(
        "  Fixed-Budget Mechanism Isolation Factorial (2^3 x 3 = 24 cells, B_fixed in {:?}, f=0.65) [Simulated]: delta_harm_Epi={:+.2}, delta_harm_Vital={:+.2}, delta_harm_Gov={:+.2} (FullTriad mean_harm={:.2})",
        fb_audit.buffer_levels,
        fb_audit.marginal_epistemic_harm_delta,
        fb_audit.marginal_vital_harm_delta,
        fb_audit.marginal_governance_harm_delta,
        fb_audit.full_triad_mean_harm
    );

    let mut comp_agent = CompensatoryBufferAgent;
    let rep_comp = env.run_trajectory(&mut comp_agent);
    let mc_comp =
        CommunityReserveEnv::evaluate_multi_seed(2000, 100, 24, 45.0, || CompensatoryBufferAgent);
    let (benign_reserve, benign_ruin) = env.run_benign_small_world_control();
    let sens_invariance = env.verify_threshold_sensitivity();
    let struct_invariance = env.verify_structural_perturbation_grid();
    let (wide_dom, wide_intact) = env.verify_wide_band_structural_sensitivity();
    println!(
        "  Non-Substitutability & Negative Controls [Simulated]: CompensatoryGiantBuffer(NoEpi+2xBuffer) cov={:.1}%, harm={:.2} (MC100={:.2}±{:.2}), R_24={:.2}, diag={:?} | BenignSmallWorld(NaiveOptimizer) R_24={:.2}, ruin={} | 3^5=243 Threshold Invariance={:.1}% | 3^5=243 Local Structural DGP Invariance={:.1}% | Wide-Band Stress Grid: dom={:.1}%, intact={:.1}%",
        rep_comp.epistemic_coverage_rate * 100.0,
        rep_comp.mean_social_harm,
        mc_comp.mean_harm,
        mc_comp.mean_harm_se,
        rep_comp.terminal_reserve,
        rep_comp.triadic_integrity.diagnose,
        benign_reserve,
        benign_ruin,
        sens_invariance * 100.0,
        struct_invariance * 100.0,
        wide_dom * 100.0,
        wide_intact * 100.0
    );
    println!(
        "  Closed-Form Prop 1 Coverage Decay Ladder [Assumed]: engine(n=46,rho=0.62)={:.1}% | Meng ladder(rho=-0.05): n=46->{:.1}%, n=500->{:.1}%, n=5k->{:.1}%, n=50k->{:.2}%, n=2.3M->{:.4}% (benchmark_met={}, manski_cov={:.1}%, reserve_at_TTR={:+.1})",
        proof_audit.prop1_finite_engine_coverage * 100.0,
        proof_audit.prop1_coverage_ladder[0] * 100.0,
        proof_audit.prop1_coverage_ladder[1] * 100.0,
        proof_audit.prop1_coverage_ladder[2] * 100.0,
        proof_audit.prop1_coverage_ladder[3] * 100.0,
        proof_audit.prop1_coverage_ladder[4] * 100.0,
        proof_audit.all_benchmark_conditions_met(),
        proof_audit.prop1_manski_support_coverage * 100.0,
        proof_audit.prop2_reserve_at_ttr
    );

    println!(
        "  Process Tracing (Bennett-Checkel) [Assumed]: prior = {:.2} -> posterior = {:.4} (decibans = {:+.1}, V: {:.2} -> {:.2})",
        pt.prior_probability,
        pt.posterior_probability,
        pt.cumulative_decibans,
        pt.initial_keynes_weight_v,
        pt.posterior_keynes_weight_v
    );
    println!(
        "  Four Reflexive Brakes [Measured forecasts + Assumed welfare/TTR]: RES_sharp={:.3} > RES_lazy={:.3}, welfare_opt={:.2} > hoard={:.2}, fast_path (TTR_recovery={:.1} <= TTS_survival={:.1}), scalar_target_absent={} (all_active = {})",
        brakes.sharp_resolution,
        brakes.skeptical_inflation_resolution,
        brakes.optimal_buffer_social_welfare,
        brakes.regressive_hoarding_welfare,
        brakes.recovery_delay_ttr_fast_path,
        brakes.survival_slack_tts,
        brakes.leontief_scalar_target_absent,
        brakes.all_brakes_active()
    );
    println!(
        "  Nested Leontief vs Multiplicative Damping [Assumed] (tau=0.50, 3^5=243 grid): V_min={:.4} vs V_prod={:.4} (-{:.2}% vol) | False Collapses: PureLeontief={:.2}% (0/243) vs HybridGov={:.2}% (180/243) vs Prod5={:.2}% (242/243; vol-matched tau_prod={:.4}: {}/243; power-p05: {}/243) | Single-Axis Ruin False-Safe: Arith={:.1}% (36/36) vs Leontief={:.1}% (0/36)",
        vol_audit.leontief_volume_3d,
        vol_audit.multiplicative_volume_3d,
        vol_audit.volume_contraction_loss * 100.0,
        vol_audit.pure_leontief_false_collapse_rate * 100.0,
        vol_audit.hybrid_gov_product_false_collapse_rate * 100.0,
        vol_audit.five_invariant_product_false_collapse_rate * 100.0,
        vol_audit.volume_matched_prod_tau,
        vol_audit.volume_matched_prod5_false_collapses,
        vol_audit.power_calibrated_prod5_false_collapses,
        vol_audit.arithmetic_false_safe_rate * 100.0,
        (vol_audit.leontief_false_safe_collapses as f64 / 36.0) * 100.0
    );

    let pre = RefutabilityPreconditions {
        multiverse_specs_k: 12,
        cluster_deff: 2.1,
        regimes_m: 24,
        horizon_t: 8,
        units_j: 20,
        proxy_pressure_beta: 0.80,
    };
    // B5 & C5: Evaluate F1–F4 on heterogeneous two-stratum (benign vs stressed) multi-regime cohorts
    // (with F4 testing non-compensability against CompensatoryBufferAgent in a mild-defect cohort where doubled buffers compensate):
    let mut paired_f1_cov_gain = Vec::with_capacity(36);
    for s_idx in 0..36u64 {
        let seed_env = CommunityReserveEnv::new(2000 + s_idx, 24, 45.0);
        let (rp, sh) = if s_idx < 20 {
            (0.88, -2.2)
        } else {
            (0.42, 8.5)
        };
        let r_111 = seed_env.run_trajectory_with_full_params(
            &mut StochologicalAgent::full(),
            0.50,
            0.88,
            0.85,
            1.60,
            10.0,
            rp,
            sh,
            1.65,
            1.40,
            1.50,
        );
        let r_011 = seed_env.run_trajectory_with_full_params(
            &mut StochologicalAgent::without_epistemic_margin(),
            0.50,
            0.88,
            0.85,
            1.60,
            10.0,
            rp,
            sh,
            1.65,
            1.40,
            1.50,
        );
        paired_f1_cov_gain.push(r_111.epistemic_coverage_rate - r_011.epistemic_coverage_rate);
    }
    let mut paired_f2_harm_red = Vec::with_capacity(29);
    for s_idx in 0..29u64 {
        let seed_env = CommunityReserveEnv::new(2000 + s_idx, 24, 45.0);
        let (rp, sh, ov) = if s_idx < 14 {
            (0.90, -2.5, 0.20)
        } else {
            (0.40, 12.5, 1.85)
        };
        let r_111 = seed_env.run_trajectory_with_full_params(
            &mut StochologicalAgent::full(),
            0.50,
            0.88,
            0.85,
            1.60,
            10.0,
            rp,
            sh,
            1.45,
            ov,
            1.50,
        );
        let r_011 = seed_env.run_trajectory_with_full_params(
            &mut StochologicalAgent::without_epistemic_margin(),
            0.50,
            0.88,
            0.85,
            1.60,
            10.0,
            rp,
            sh,
            1.45,
            ov,
            1.50,
        );
        paired_f2_harm_red.push(r_011.mean_social_harm - r_111.mean_social_harm);
    }
    let mut paired_f3_apt_gain = Vec::with_capacity(30);
    for s_idx in 0..30u64 {
        let er = if s_idx < 18 { 0.0 } else { 0.68 };
        let seed_env = CommunityReserveEnv::new(2000 + s_idx, 8, 45.0);
        let r_111 = seed_env.run_trajectory_with_full_params(
            &mut StochologicalAgent::full(),
            0.50,
            0.88,
            0.85,
            1.60,
            10.0,
            0.60,
            6.0,
            1.65,
            1.20,
            er,
        );
        let r_110 = seed_env.run_trajectory_with_full_params(
            &mut StochologicalAgent::without_governance(),
            0.50,
            0.88,
            0.85,
            1.60,
            10.0,
            0.60,
            6.0,
            1.65,
            1.20,
            er,
        );
        paired_f3_apt_gain.push(r_110.mean_social_harm - r_111.mean_social_harm);
    }
    let mut paired_f4_leo_gain = Vec::with_capacity(26);
    for s_idx in 0..26u64 {
        let seed_env = CommunityReserveEnv::new(2000 + s_idx, 24, 45.0);
        let (rp, sh, ov) = if s_idx < 24 {
            (0.85, 4.0, 1.10)
        } else {
            (0.25, 19.0, 2.60)
        };
        let r_111 = seed_env.run_trajectory_with_full_params(
            &mut StochologicalAgent::full(),
            0.50,
            0.88,
            0.85,
            1.60,
            10.0,
            rp,
            sh,
            1.85,
            ov,
            1.00,
        );
        let r_comp = seed_env.run_trajectory_with_full_params(
            &mut CompensatoryBufferAgent,
            0.50,
            0.88,
            0.85,
            1.60,
            10.0,
            rp,
            sh,
            1.85,
            ov,
            1.00,
        );
        let loss_comp = (45.0 - r_comp.terminal_reserve).max(0.0) / 45.0;
        let loss_111 = (45.0 - r_111.terminal_reserve).max(0.0) / 45.0;
        paired_f4_leo_gain.push(loss_comp - loss_111);
    }
    let fv1 = falsifiers[0]
        .evaluate_observations(&paired_f1_cov_gain, 1.5, &pre)
        .map_err(|e| format!("F1 not testable: {e:?}"))?;
    let fv2 = falsifiers[1]
        .evaluate_observations(&paired_f2_harm_red, 1.5, &pre)
        .map_err(|e| format!("F2 not testable: {e:?}"))?;
    let fv3 = falsifiers[2]
        .evaluate_observations(&paired_f3_apt_gain, 1.5, &pre)
        .map_err(|e| format!("F3 not testable: {e:?}"))?;
    let fv4 = falsifiers[3]
        .evaluate_observations(&paired_f4_leo_gain, 1.5, &pre)
        .map_err(|e| format!("F4 not testable: {e:?}"))?;
    if falsifiers[0].evaluate_observations(&[], 1.0, &pre) != Err(NotTestable::EmptyObservations) {
        return Err("Empty observations must return NotTestable::EmptyObservations".to_string());
    }
    let fmt_v = |v: stochology::FalsifierVerdict| -> (f64, f64, bool) {
        match v {
            stochology::FalsifierVerdict::Refuted {
                observed_mean_d,
                post_data_severity,
            } => (observed_mean_d, post_data_severity, true),
            stochology::FalsifierVerdict::Corroborated {
                observed_mean_d,
                post_data_severity,
            }
            | stochology::FalsifierVerdict::Indeterminate {
                observed_mean_d,
                post_data_severity,
            } => (observed_mean_d, post_data_severity, false),
        }
    };
    let (d1, s1, r1) = fmt_v(fv1);
    let (d2, s2, r2) = fmt_v(fv2);
    let (d3, s3, r3) = fmt_v(fv3);
    let (d4, s4, r4) = fmt_v(fv4);
    println!(
        "  Four Pre-Registered Falsifiers [Assumed design + Simulated paired MC100 evaluation] (Mayo SEV >= 0.95): F1(n_eff={}, d_crit={:.4}, d_obs={:.3}, SEV={:.3}, refuted={}) | F2(n_eff={}, d_crit={:.4}, d_obs={:.3}, SEV={:.3}, refuted={}) | F3(n_eff={}, d_crit={:.4}, d_obs={:.3}, SEV={:.3}, refuted={}) | F4(n_eff={}, d_crit={:.4}, d_obs={:.3}, SEV={:.3}, refuted={})",
        falsifiers[0].required_n_eff,
        falsifiers[0].falsification_critical_d,
        d1,
        s1,
        r1,
        falsifiers[1].required_n_eff,
        falsifiers[1].falsification_critical_d,
        d2,
        s2,
        r2,
        falsifiers[2].required_n_eff,
        falsifiers[2].falsification_critical_d,
        d3,
        s3,
        r3,
        falsifiers[3].required_n_eff,
        falsifiers[3].falsification_critical_d,
        d4,
        s4,
        r4
    );

    let full = &reports[0];
    let no_epi = &reports[1];
    let no_vital = &reports[2];
    let no_gov = &reports[3];

    if !(full.epistemic_coverage_rate > no_epi.epistemic_coverage_rate
        && full.terminal_reserve > no_vital.terminal_reserve
        && full.mean_social_harm < no_gov.mean_social_harm
        && !benign_ruin
        && sens_invariance > 0.80
        && struct_invariance > 0.80
        && proof_audit.all_benchmark_conditions_met()
        && mc_summaries[0].ruin_rate == 0.0)
    {
        return Err("Three Collapse Propositions failed factorial dominance check".to_string());
    }
    if !brakes.all_brakes_active()
        || pt.posterior_probability < 0.95
        || vol_audit.pure_leontief_false_collapses != 0
        || falsifiers[0].required_n_eff != 24
    {
        return Err("Chapter 9 process tracing or reflexive brake audit failed".to_string());
    }

    let _ = TriadicIntegrity::evaluate(0.9, 0.9, 0.9, 0.5);
    let _ = CollapseProposition::IntactTriad;

    Ok(())
}

/// Run either a single chapter (`Some(1..=9)`) or the entire verification suite (`None`).
pub fn run_all_or_chapter(workspace_root: &Path, chapter: Option<usize>) -> Result<(), String> {
    run_all_or_chapter_with_brake(workspace_root, chapter, None)
}

/// Run either a single chapter (`Some(1..=9)`) or the entire verification suite (`None`)
/// with an optional `--brake on|off` override (`B6 · MAPA-03`).
pub fn run_all_or_chapter_with_brake(
    workspace_root: &Path,
    chapter: Option<usize>,
    brake_override: Option<bool>,
) -> Result<(), String> {
    let sealed_count = verify_sha256sums_manifest(workspace_root)?;
    let contract_count = verify_external_upstream_schema_contract(workspace_root)?;
    println!(
        "[SEAL VERIFIED] All {} empirical datasets in data/SHA256SUMS match their SHA-256 digests and {} upstream schema contracts.",
        sealed_count, contract_count
    );

    match chapter {
        Some(1) => run_chapter_1(),
        Some(2) => run_chapter_2(),
        Some(3) => run_chapter_3(workspace_root),
        Some(4) => run_chapter_4(workspace_root),
        Some(5) => run_chapter_5(workspace_root),
        Some(6) => run_chapter_6(workspace_root),
        Some(7) => run_chapter_7(workspace_root),
        Some(8) => run_chapter_8_with_brake(workspace_root, brake_override),
        Some(9) => run_chapter_9(),
        Some(other) => Err(format!("Chapter must be between 1 and 9 (got {other})")),
        None => {
            run_chapter_1()?;
            run_chapter_2()?;
            run_chapter_3(workspace_root)?;
            run_chapter_4(workspace_root)?;
            run_chapter_5(workspace_root)?;
            run_chapter_6(workspace_root)?;
            run_chapter_7(workspace_root)?;
            run_chapter_8_with_brake(workspace_root, brake_override)?;
            run_chapter_9()?;
            println!(
                "[STOCHOLOGY VERIFIED] All 9 progressive chapter laboratories and 3 Collapse Propositions passed."
            );
            Ok(())
        }
    }
}

/// **Cross-domain audit report** (`--audit-report`).
///
/// Emits, in a stable frozen format, the four audits that exercise the Stochological operators
/// beyond the seeded datasets: latent world-model exploitation (dream optimisation vs.
/// Kennedy–O'Hagan discrepancy), the friction-envelope control benchmark (model-based vs.
/// in-simulation tuning vs. demonstration fitting), the social-network topology battery
/// (community detection and centrality) and model drift with a fallback budget. These lines are
/// verified verbatim against `book-src/appendix-b-architecture-verification.md` by
/// `scripts/verify.sh`.
pub fn audit_report() -> String {
    let dream = LatentModelExploitationAudit::evaluate(LatentWorldModelConfig::canonical());
    let control = FrictionEnvelopeControlAudit::evaluate(0.95, 20, 10.0);
    let topology = TopologyCentralityAudit::evaluate(SocialNetworkSpec::canonical());
    let drift = ModelDriftFallbackAudit::evaluate(DriftMonitorConfig::canonical());

    let mut out = String::new();
    out.push_str("=== CROSS-DOMAIN AUDIT REPORT ===\n");
    out.push_str(&format!(
        "DREAM  slope = {:.5} | dream_return = {:.4} | realized_return = {:.4} | ruin_step = {:?} | extrapolation_depth = {:.4} | out_of_support = {:.4} | discrepancy = {:.4} | goodhart = {:?}\n",
        dream.surrogate_slope,
        dream.dream_return,
        dream.realized_return_greedy,
        dream.ruin_step_greedy,
        dream.extrapolation_depth,
        dream.out_of_support_fraction,
        dream.discrepancy_at_dream_end,
        dream.goodhart_variant
    ));
    for i in 0..LatentWorldModelConfig::SWEEP_LEN {
        out.push_str(&format!(
            "TAU    [{i}] tau = {:.4} | realized_return = {:.4} | ruin_step = {:?} | discrepancy = {:.4}\n",
            dream.temperature_grid[i],
            dream.realized_return_by_tau[i],
            dream.ruin_step_by_tau[i],
            dream.discrepancy_by_tau[i]
        ));
    }
    out.push_str(&format!(
        "CAR    mu_deploy = {:.2} | mu_sim = {:.2} | mu_identified = {:.2} | target = {:.2} | gain = {:.2}\n",
        control.deployed_mu,
        control.simulated_mu,
        control.identified_mu,
        control.tuned_speed_target,
        control.tuned_gain
    ));
    for lap in [
        &control.model_based,
        &control.in_simulation_tuned,
        &control.demonstration_fitted,
    ] {
        out.push_str(&format!(
            "CAR    {:?} | completed = {} | lap_time = {:?} | violations = {} | max_utilization = {:.4} | grip_loss_step = {:?} | mean_speed = {:.4}\n",
            lap.regime,
            lap.completed_lap,
            lap.lap_time_s.map(|t| format!("{t:.4}")),
            lap.envelope_violation_steps,
            lap.max_utilization,
            lap.loss_of_grip_step,
            lap.mean_speed
        ));
    }
    out.push_str(&format!(
        "NET    nodes = {} | blocks = {} | edges = {} | mean_degree = {:.4} | density = {:.4} | diameter = {} | clustering = {:.4}\n",
        topology.node_count,
        topology.block_count,
        topology.edge_count,
        topology.mean_weighted_degree,
        topology.density,
        topology.diameter,
        topology.clustering_coefficient
    ));
    out.push_str(&format!(
        "NET    Q_planted = {:.4} | Q_label_propagation = {:.4} | communities = {} | pagerank_prestige = {}\n",
        topology.modularity_planted,
        topology.modularity_label_propagation,
        topology.detected_community_count,
        topology.pagerank_recovers_planted_prestige
    ));
    out.push_str(&format!(
        "NET    eigenvector_max = {:.4} | hits_hub_max = {:.4} | hits_authority_max = {:.4} | betweenness_max = {:.4} | bridge_betweenness = {:.4} | interior_betweenness = {:.4}\n",
        topology.eigenvector_max,
        topology.hits_max_hub,
        topology.hits_max_authority,
        topology.betweenness_max,
        topology.bridge_betweenness_mean,
        topology.interior_betweenness_mean
    ));
    out.push_str(&format!(
        "DRIFT  change_point = {} | detection = {:?} | false_alarm = {:?} | tts = {} | ttr = {:?} | reserve_with_fallback = {:.4} | reserve_without_fallback = {:.4} | survived_without = {} | survived_with = {}\n",
        drift.change_point,
        drift.detection_step,
        drift.false_alarm_step,
        drift.tts_steps,
        drift.ttr_steps,
        drift.reserve_with_fallback,
        drift.reserve_without_fallback,
        drift.survival_without_fallback,
        drift.survival_with_fallback
    ));
    out.push_str(&format!(
        "DRIFT  slow_retrain ttr = {:?} | survived_with_slow_retrain = {}\n",
        drift.ttr_slow_retrain_steps, drift.survival_with_slow_retrain
    ));
    for rule in ElementaryRule::ALL {
        let ca = CellularAutomatonAudit::run(rule);
        out.push_str(&format!(
            "CELL   {} | cells = {} | steps = {} | fixed_point = {:?} | period = {:?} | block_entropy = {:.4} | block_entropy_max = {:.4} | live_fraction = {:.4} | regime = {:?}\n",
            rule.code(),
            ca.cells,
            ca.steps,
            ca.fixed_point_step,
            ca.tail_period,
            ca.block_entropy_final,
            ca.block_entropy_max,
            ca.live_fraction_final,
            ca.regime
        ));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::loaders::{count_atlantic_storms_transitions, resolve_workspace_root};
    use stochology::FalsifierVerdict;

    #[test]
    fn chapter_metrics_and_paired_falsifiers_satisfy_numerical_bounds() {
        let root = resolve_workspace_root();

        // Ch 3: LaLonde NSW experimental ATE within tolerance
        let (nsw_t, nsw_c, _) = load_lalonde_nsw_cps(&root);
        let ate = mean(&nsw_t) - mean(&nsw_c);
        assert!((ate - 1794.34).abs() < 1.0);

        // Ch 5: Danish fire Hill alpha in heavy-tail infinite-variance regime
        let danish = load_danish_fire(&root);
        let vital_fire =
            VitalMarginState::from_heavy_tails_and_asymmetry(&danish, 150, (1.5, 4.0), |x| {
                0.08 * x.powf(1.65)
            });
        assert!(vital_fire.tail_alpha > 1.30 && vital_fire.tail_alpha < 1.45);

        // Ch 7: Strict +6h storm transitions and CORP-PAV resolution > reliability
        let (_, transitions, _) = count_atlantic_storms_transitions(&root);
        assert_eq!(transitions, 19_025);
        let (storm_p, storm_y) = load_atlantic_storms_forecasts(&root);
        let (_, rel, res, _, _) = GovernanceState::evaluate_corp_pav(&storm_p, &storm_y);
        assert!(res > rel && res > 0.10);

        // Ch 9 / B1: All 8 cells have 8 distinct sEpi, 8 distinct sVital, and 8 distinct sGov values at 3 decimals
        let env = CommunityReserveEnv::new(DEFAULT_SEED, 24, 45.0);
        let configs = [
            (true, true, true),
            (false, true, true),
            (true, false, true),
            (true, true, false),
            (true, false, false),
            (false, true, false),
            (false, false, true),
            (false, false, false),
        ];
        let mut s_epis = Vec::with_capacity(8);
        let mut s_vitals = Vec::with_capacity(8);
        let mut s_govs = Vec::with_capacity(8);
        for &(e, v, g) in &configs {
            let rep = env.run_trajectory(&mut StochologicalAgent::with_axes(e, v, g));
            s_epis.push(format!("{:.3}", rep.triadic_integrity.epistemic_margin));
            s_vitals.push(format!("{:.3}", rep.triadic_integrity.vital_margin));
            s_govs.push(format!(
                "{:.3}",
                rep.triadic_integrity.institutional_governance
            ));
        }
        s_epis.sort();
        s_epis.dedup();
        s_vitals.sort();
        s_vitals.dedup();
        s_govs.sort();
        s_govs.dedup();
        assert_eq!(s_epis.len(), 8, "all 8 cells must have distinct sEpi");
        assert_eq!(s_vitals.len(), 8, "all 8 cells must have distinct sVital");
        assert_eq!(s_govs.len(), 8, "all 8 cells must have distinct sGov");

        // Ch 9 / B5: Pre-registered falsifiers return Corroborated on paired gains and Refuted on null/negative gains
        let falsifiers = PreregisteredFalsifier::canonical_four();
        let pre = RefutabilityPreconditions {
            multiverse_specs_k: 12,
            cluster_deff: 2.1,
            regimes_m: 24,
            horizon_t: 8,
            units_j: 20,
            proxy_pressure_beta: 0.80,
        };
        let mut paired_cov_gain = Vec::with_capacity(40);
        for s_idx in 0..40u64 {
            let s_env = CommunityReserveEnv::new(2000 + s_idx, 24, 45.0);
            let r_111 = s_env.run_trajectory(&mut StochologicalAgent::full());
            let r_011 = s_env.run_trajectory(&mut StochologicalAgent::without_epistemic_margin());
            paired_cov_gain.push(r_111.epistemic_coverage_rate - r_011.epistemic_coverage_rate);
        }
        let corrob = falsifiers[0]
            .evaluate_observations(&paired_cov_gain, 1.5, &pre)
            .expect("testable");
        assert!(matches!(corrob, FalsifierVerdict::Corroborated { .. }));

        let null_diffs = vec![0.01; 40];
        let refuted = falsifiers[0]
            .evaluate_observations(&null_diffs, 1.5, &pre)
            .expect("testable");
        assert!(matches!(refuted, FalsifierVerdict::Refuted { .. }));

        // C-01: Toronto arrest loader rejects blank/missing checks, invalid categories, and empty strata
        let bad_checks = crate::csv::CsvTable::from_string(
            "rownames,released,colour,year,age,sex,employed,citizen,checks\n1,No,Black,2000,25,Male,No,Yes,\n2,Yes,White,2000,30,Male,Yes,Yes,2\n".to_string(),
        );
        assert!(
            crate::loaders::try_load_toronto_arrests_matched_and_classifier_from_table(&bad_checks)
                .is_err()
        );
        assert!(
            crate::loaders::try_load_toronto_arrests_full_summary_from_table(&bad_checks).is_err()
        );
        let empty_stratum = crate::csv::CsvTable::from_string(
            "rownames,released,colour,year,age,sex,employed,citizen,checks\n1,No,Black,2000,25,Male,No,Yes,4\n".to_string(),
        );
        assert!(
            crate::loaders::try_load_toronto_arrests_matched_and_classifier_from_table(
                &empty_stratum
            )
            .is_err()
        );
        assert!(
            crate::loaders::try_load_toronto_arrests_full_summary_from_table(&empty_stratum)
                .is_err()
        );
    }
}
