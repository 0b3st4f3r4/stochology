//! Progressive Chapter Laboratories (Chapters 1 to 9), coupling [`CommunityReserveEnv`]
//! with the sealed empirical datasets in `data/*.csv`.

use crate::loaders::{
    load_atlantic_storms_environments, load_atlantic_storms_forecasts, load_bank_call_clusters,
    load_cps_cohort_clusters, load_danish_fire, load_djia_returns, load_lalonde_nsw_cps,
    load_nyc_flight_delays, load_rtdsm_routput_vintages, load_toronto_arrests_prevalences,
    load_vic_elec_demand, verify_sha256sums_manifest,
};
use std::path::Path;
use stochology::{
    AlgedonicChannel, AnchoredDiscrepancy, CanguilhemHollingBasin, CaptureRecaptureAudit,
    ChouldechovaTrilemma, ClusterDesignEffect, CognitiveAlertFilter, CollapseProposition,
    CommunityReserveEnv, ContractAndLiabilityAudit, CorpPavDecomposition, CostLossFraming,
    DEFAULT_SEED, DiagnosticClue, DialecticalPenumbra, DiscursiveCommittee,
    EpistemicKellyAllocation, EvidenceProcess, GoodhartAudit, GoodhartVariant, HillTailEstimate,
    InfoGapMaximinSafeguard, KeynesEvidentialWeight, KishWeightAudit, LeeAttritionBounds,
    MacKenzieRegime, ManskiBounds, MengBigDataDefect, NaiveOptimizer, NonlinearSummary,
    OrdinalScaleAudit, PerformativeSystem, PetersonSurvivalBounds, ProcessTracingInference,
    ReflexiveBrakeAudit, SeverePowerAnalysis, SpecificationCurve, StochologicalAgent,
    TailMomentRegime, TelicEvaluation, TemporalVault, TianPearlCounterfactualBounds,
    TypeThreeFramingAudit, VintageRevisionAudit, VitalSlackBuffer, WaldSurvivorshipAudit,
    cromwell_clamp, e_bh_discoveries, evaluate_invariant_predictors, lakatos_shift_ratio, mean,
};

/// Chapter 1 — The Sequential Loop and the Collapse of the Naive Optimizer.
pub fn run_chapter_1() -> Result<(), String> {
    let env = CommunityReserveEnv::new(DEFAULT_SEED, 24, 45.0);
    let mut naive = NaiveOptimizer;
    let rep_naive = env.run_trajectory(&mut naive);
    let mut full = StochologicalAgent::full();
    let rep_full = env.run_trajectory(&mut full);

    println!(
        "=== Chapter 1: The Sequential Loop (CommunityReserveEnv, seed={DEFAULT_SEED}, T=24) ==="
    );
    println!(
        "  NaivePointOptimizer      -> coverage={:.1}%, mean_harm={:.2}, terminal_reserve={:.2}, ruin_step={:?}",
        rep_naive.epistemic_coverage_rate * 100.0,
        rep_naive.mean_social_harm,
        rep_naive.terminal_reserve,
        rep_naive.ruin_step
    );
    println!(
        "  StochologicalAgent(Full) -> coverage={:.1}%, mean_harm={:.2}, terminal_reserve={:.2}, ruin_step={:?}, triad={:?}",
        rep_full.epistemic_coverage_rate * 100.0,
        rep_full.mean_social_harm,
        rep_full.terminal_reserve,
        rep_full.ruin_step,
        rep_full.triadic_integrity.diagnose
    );

    if rep_naive.ruin_step.is_none() || rep_full.ruin_step.is_some() {
        return Err("Chapter 1 invariant failed: NaiveOptimizer must hit ruin and StochologicalAgent must survive".to_string());
    }
    if rep_full.epistemic_coverage_rate <= 0.85
        || rep_full.mean_social_harm >= rep_naive.mean_social_harm * 0.25
    {
        return Err(
            "Chapter 1 invariant failed: StochologicalAgent did not dominate NaiveOptimizer"
                .to_string(),
        );
    }
    Ok(())
}

/// Chapter 2 — Inquiry, Framing, and Antidogmatic Openness (Filter I · Tension 1).
pub fn run_chapter_2() -> Result<(), String> {
    let p_clamped = cromwell_clamp(0.0, 1e-4);
    let inst = CostLossFraming::evaluate(100.0, 1.0, 200.0, 0.20);
    let affected = CostLossFraming::evaluate(100.0, 1.0, 2000.0, 0.20);
    let underpowered = SeverePowerAnalysis::evaluate(0.30, 1.0, 18, 0.05);
    let severe = SeverePowerAnalysis::evaluate(0.30, 1.0, 140, 0.05);
    let type3 = TypeThreeFramingAudit::evaluate(12.0, 28.0);
    let robust_curve = SpecificationCurve::evaluate(
        &[0.28, 0.31, 0.26, 0.29, 0.33, 0.27],
        &[32.0, 41.0, 25.0, 36.0, 48.0, 28.0],
        0.05,
    );
    let cherry_e = [21.5, 0.6, 0.8, 0.5, 0.9, 0.7];
    let cherry_curve =
        SpecificationCurve::evaluate(&[0.42, -0.04, 0.01, -0.02, 0.03, -0.01], &cherry_e, 0.05);
    let cherry_disc = e_bh_discoveries(&cherry_e, 0.05);

    println!("=== Chapter 2: Inquiry, Framing & Antidogmatic Openness (Filter I · Tension 1) ===");
    println!(
        "  Cromwell floor={:.4} | Framing p*_inst={:.2} vs p*_affected={:.2} (gap={:.2}, EVPI_affected={:.1})",
        p_clamped,
        inst.threshold_p_star,
        affected.threshold_p_star,
        inst.framing_gap(&affected),
        affected.evpi
    );
    println!(
        "  Agnotology vs Severe Power (delta=0.30): n=18 power={:.3} vs n=140 power={:.3} | Type III net={:.1}",
        underpowered.power_at_min_harm, severe.power_at_min_harm, type3.global_net_utility
    );
    println!(
        "  Multiverse (K=6): robust consistency={:.0}% (avg_E={:.1}) vs cherry-picked consistency={:.1}% (e-BH discoveries={})",
        robust_curve.consistency_ratio * 100.0,
        robust_curve.average_e_value,
        cherry_curve.consistency_ratio * 100.0,
        cherry_disc.len()
    );

    if !(severe.power_at_min_harm > 0.90
        && underpowered.power_at_min_harm < 0.35
        && type3.is_type_three_error
        && robust_curve.consistency_ratio == 1.0
        && cherry_disc.is_empty())
    {
        return Err("Chapter 2 invariant failed".to_string());
    }
    Ok(())
}

/// Chapter 3 — Underdetermination, Partial Identification, and Severe Testing (Filter I · Tension 2).
pub fn run_chapter_3(root_dir: &Path) -> Result<(), String> {
    let (nsw_t, nsw_c, cps_c) = load_lalonde_nsw_cps(root_dir);
    let exp_diff = mean(&nsw_t) - mean(&nsw_c);
    let obs_diff = mean(&nsw_t) - mean(&cps_c);

    let manski = ManskiBounds::evaluate(&nsw_t, 35, 0.0, 35_000.0, 0.05);
    let lee = LeeAttritionBounds::evaluate(&nsw_t, nsw_t.len() + 18, &nsw_c, nsw_c.len() + 45);
    let peterson = PetersonSurvivalBounds::evaluate(20, 25, 55);
    let ep = EvidenceProcess::from_gaussian_increments(
        &[0.7, 0.85, 0.9, 0.8, 0.95, 0.88],
        0.0,
        1.0,
        0.8,
    );
    let lak_prog = lakatos_shift_ratio(ep.terminal_e_value, 0);
    let lak_deg = lakatos_shift_ratio(ep.terminal_e_value, 12);

    println!(
        "=== Chapter 3: Underdetermination & Partial Identification (Filter I · Tension 2) ==="
    );
    println!(
        "  LaLonde (nsw_experimental.csv vs cps_observational.csv): experimental_ATE=${:.0} vs naive_CPS_diff=${:.0}",
        exp_diff, obs_diff
    );
    println!(
        "  Manski bounds=[${:.0}, ${:.0}] (width=${:.0}) | Lee attrition bounds=[${:.0}, ${:.0}]",
        manski.lower_bound,
        manski.upper_bound,
        manski.identification_width,
        lee.lower_bound,
        lee.upper_bound
    );
    println!(
        "  Peterson survival bounds=[{:.2}, {:.2}] vs naive={:.2} | e-process E_t={:.2} (Lakatos prog={:.2} vs deg={:.2})",
        peterson.survival_lower,
        peterson.survival_upper,
        peterson.naive_independent_survival,
        ep.terminal_e_value,
        lak_prog,
        lak_deg
    );

    if !(exp_diff > 1_000.0
        && obs_diff < -5_000.0
        && manski.identification_width > 1_000.0
        && lak_prog > lak_deg)
    {
        return Err("Chapter 3 invariant failed".to_string());
    }
    Ok(())
}

/// Chapter 4 — The Non-Identical, Scale Invariance, Convexity, and Heavy Tails (Filter II · Tension 3).
pub fn run_chapter_4(root_dir: &Path) -> Result<(), String> {
    let fire_losses = load_danish_fire(root_dir);
    let (delays_13, delays_23) = load_nyc_flight_delays(root_dir, 3_000);

    let fire_summary = NonlinearSummary::evaluate(&fire_losses, |x| x * x);
    let fire_hill = HillTailEstimate::evaluate(&fire_losses, 120);
    let flight_summary = NonlinearSummary::evaluate(&delays_23, |d| d * d);
    let penumbra = DialecticalPenumbra::evaluate(&delays_13, 10.0, 20.0);
    let ordinal = OrdinalScaleAudit::evaluate(&[3.0, 3.0, 3.0], &[1.0, 3.0, 4.9], |x| x * x);

    println!(
        "=== Chapter 4: Asymmetry, Scale Invariance & Heavy Tails (Filter II · Tension 3) ==="
    );
    println!(
        "  Danish Fire (danish_fire.csv, n={}): mean={:.2}, median={:.2}, q95={:.2}, Jensen_gap={:.2}, Hill_alpha={:.3} ({:?})",
        fire_losses.len(),
        fire_summary.sample_mean,
        fire_summary.median,
        fire_summary.q95,
        fire_summary.jensen_gap,
        fire_hill.alpha_hat,
        fire_hill.regime
    );
    println!(
        "  NYC Flights (nyc_flights_2013/2023.csv): delay Jensen_gap={:.1} min^2, penumbra_mass(10..20m)={:.1}%, ordinal_err={:.1e}",
        flight_summary.jensen_gap,
        penumbra.penumbra_mass * 100.0,
        ordinal.ordinal_invariance_error
    );

    if !(fire_summary.jensen_gap > 50.0
        && flight_summary.jensen_gap > 1000.0
        && fire_hill.regime == TailMomentRegime::FiniteMeanInfiniteVariance
        && ordinal.mean_ranking_reversed
        && ordinal.ordinal_invariance_error < 1e-12)
    {
        return Err("Chapter 4 invariant failed".to_string());
    }
    Ok(())
}

/// Chapter 5 — From World to Archive: Evidential Weight, Big Data Paradox, and the Uncounted (Filter II · Tension 4).
pub fn run_chapter_5(root_dir: &Path) -> Result<(), String> {
    let cps_clusters = load_cps_cohort_clusters(root_dir);
    let bank_clusters = load_bank_call_clusters(root_dir);
    let vintages = load_rtdsm_routput_vintages(root_dir);

    let cps_deff = ClusterDesignEffect::evaluate(&cps_clusters, 0.75);
    let bank_deff = ClusterDesignEffect::evaluate(&bank_clusters, 0.80);
    let meng = MengBigDataDefect::evaluate(10_000_000.0, 2_300_000.0, -0.05, 0.50);
    let keynes = KeynesEvidentialWeight::evaluate(0.65, cps_deff.effective_n, 2_500.0);
    let kish = KishWeightAudit::from_strata(&[(990, 0.90), (10, 0.001)]);
    let chao = CaptureRecaptureAudit::evaluate(200, 300, 300);
    let rev_audit = VintageRevisionAudit::evaluate(&vintages);

    println!("=== Chapter 5: From World to Archive & Big Data Paradox (Filter II · Tension 4) ===");
    println!(
        "  Meng Big Data (n=2.3M / N=10M, rho=-0.05): n_eff={:.1} | CPS clusters (n={}): DEFF={:.2}, n_eff={:.1}, Keynes_V={:.3}",
        meng.effective_sample_size,
        cps_deff.total_observations_n,
        cps_deff.design_effect,
        cps_deff.effective_n,
        keynes.evidential_weight_v
    );
    println!(
        "  Bank Calls (bank_calls.csv): DEFF={:.2}, n_eff={:.1} | Kish floor n_eff={:.1} | Chao uncounted n00={:.0}",
        bank_deff.design_effect,
        bank_deff.effective_n,
        kish.kish_effective_n,
        chao.estimated_uncounted_n00
    );
    println!(
        "  Real-Time Vintages (rtdsm_routput_vintages.csv, Q={}): mean_revision={:.2} pp, max_revision={:.2} pp, sign_flips={:.1}%",
        rev_audit.num_quarters,
        rev_audit.mean_abs_growth_revision_pp,
        rev_audit.max_abs_growth_revision_pp,
        rev_audit.sign_flip_rate * 100.0
    );

    if !(cps_deff.design_effect > 1.2
        && bank_deff.design_effect > 1.5
        && meng.effective_sample_size < 500.0
        && rev_audit.num_quarters > 100
        && rev_audit.mean_abs_growth_revision_pp > 0.5)
    {
        return Err("Chapter 5 invariant failed".to_string());
    }
    Ok(())
}

/// Chapter 6 — Reactive Worlds: Causal Invariance, Performativity, and Goodhart's Law (Filter III · Tension 5).
pub fn run_chapter_6(root_dir: &Path) -> Result<(), String> {
    let (storm_p, storm_y) = load_atlantic_storms_forecasts(root_dir);
    let storm_envs = load_atlantic_storms_environments(root_dir);

    let corp = CorpPavDecomposition::evaluate(&storm_p, &storm_y);
    let icp = evaluate_invariant_predictors(&storm_envs);
    let gp = AnchoredDiscrepancy::fit(&[-1.0, 0.0, 1.0], &[0.08, -0.03, 0.05], 0.50, 0.65, 0.05);
    let (_, std_in) = gp.predict(0.0);
    let (_, std_out) = gp.predict(8.0);
    let tp = TianPearlCounterfactualBounds::evaluate(0.70, 0.20, 0.40, 0.10, 0.10, 0.40);
    let perf = PerformativeSystem::evaluate(1.0, 0.45, 0.20, 0.10);
    let goodhart = GoodhartAudit::evaluate(0.35, -0.12, GoodhartVariant::Adversarial);

    println!(
        "=== Chapter 6: Reactive Worlds, Performativity & Goodhart (Filter III · Tension 5) ==="
    );
    println!(
        "  Atlantic Storms CORP-PAV (atlantic_storms.csv, n={}): BS={:.4} = REL({:.4}) - RES({:.4}) + UNC({:.4}), closure_err={:.1e}",
        storm_p.len(),
        corp.brier_score,
        corp.reliability,
        corp.resolution,
        corp.uncertainty,
        corp.identity_closure_error
    );
    println!(
        "  Storms ICP across 3 seasons: physical_pressure_slope_range={:.3} vs proxy_latitude_slope_range={:.3}",
        icp[0].slope_range, icp[1].slope_range
    );
    println!(
        "  Perdomo/MacKenzie ({:?}): theta_PS={:.3} vs theta_PO={:.3} (risk_PO={:.3} < risk_PS={:.3}) | KOH std_in={:.3} -> std_out={:.3}",
        perf.mackenzie_regime,
        perf.theta_ps,
        perf.theta_po,
        perf.risk_at_po,
        perf.risk_at_ps,
        std_in,
        std_out
    );
    println!(
        "  Tian-Pearl PN=[{:.2}, {:.2}], PNS=[{:.2}, {:.2}] | Goodhart collapsed={}",
        tp.pn_bounds.0, tp.pn_bounds.1, tp.pns_bounds.0, tp.pns_bounds.1, goodhart.metric_collapsed
    );

    if !(corp.identity_closure_error < 1e-12
        && icp[0].slope_range < icp[1].slope_range
        && perf.mackenzie_regime == MacKenzieRegime::BarnesianSelfFulfilling
        && perf.risk_at_po < perf.risk_at_ps
        && std_out > 2.5 * std_in
        && goodhart.metric_collapsed)
    {
        return Err("Chapter 6 invariant failed".to_string());
    }
    Ok(())
}

/// Chapter 7 — Irreversible Time, Non-Ergodicity, and the Vital Margin (Filter III · Tension 6).
pub fn run_chapter_7(root_dir: &Path) -> Result<(), String> {
    let djia_returns = load_djia_returns(root_dir);
    let vic_demand = load_vic_elec_demand(root_dir);

    let djia_kelly = EpistemicKellyAllocation::from_return_series(&djia_returns);
    let vic_summary = NonlinearSummary::evaluate(&vic_demand, |d| d * d);
    let (basin_fresh, basin_worn) = CanguilhemHollingBasin::with_allostatic_wear(1.80, 10.0, 2.5);
    let basin_stressed = CanguilhemHollingBasin::evaluate(2.45, 10.0);
    let slack_safe = VitalSlackBuffer::evaluate(0.78, 1.0, 12.0, 2.0, 4.0);
    let slack_lean = VitalSlackBuffer::evaluate(0.97, 1.0, 2.0, 2.0, 4.0);
    let wald = WaldSurvivorshipAudit::evaluate(800, 232, 250, 40, 250);
    let sniedovich = InfoGapMaximinSafeguard::evaluate(100.0, 10.0, 60.0, -400.0, -50.0);

    println!(
        "=== Chapter 7: Irreversible Time, Non-Ergodicity & Vital Margin (Filter III · Tension 6) ==="
    );
    println!(
        "  DJIA (djia_1885_2023.csv, n={}): ensemble_mu={:.6}, time_growth_g={:.6}, Kelly_f*={:.2}",
        djia_returns.len(),
        djia_kelly.ensemble_drift_mu,
        djia_kelly.unleveraged_time_growth,
        djia_kelly.epistemic_kelly_fraction
    );
    println!(
        "  Victoria Grid (vic_elec.csv, n={}): mean={:.0} MW, q95={:.0} MW | Kingman wait: rho=0.78 -> {:.2}x vs rho=0.97 -> {:.2}x",
        vic_demand.len(),
        vic_summary.sample_mean,
        vic_summary.q95,
        slack_safe.kingman_wait_multiplier,
        slack_lean.kingman_wait_multiplier
    );
    println!(
        "  Canguilhem-Holling basin: fresh={:.2} -> worn(McEwen)={:.2} | Scheffer CSD rho1: {:.3} -> {:.3}",
        basin_fresh.vital_basin_radius,
        basin_worn.vital_basin_radius,
        basin_fresh.critical_slowing_down_rho1,
        basin_stressed.critical_slowing_down_rho1
    );
    println!(
        "  Wald 1943 lethality: wing={:.1}% vs engine={:.1}% | Sniedovich No-Man's-Land mirage={}",
        wald.lethality_given_hit_zone_a * 100.0,
        wald.lethality_given_hit_zone_b * 100.0,
        sniedovich.is_no_mans_land_mirage
    );

    if !(djia_kelly.unleveraged_time_growth < djia_kelly.ensemble_drift_mu
        && slack_safe.survives_disruption()
        && !slack_lean.survives_disruption()
        && basin_worn.vital_basin_radius < basin_fresh.vital_basin_radius
        && wald.survivorship_inversion_present
        && sniedovich.is_no_mans_land_mirage)
    {
        return Err("Chapter 7 invariant failed".to_string());
    }
    Ok(())
}

/// Chapter 8 — Moral Luck, Apt Agency, and Institutional Design (Ethical Mediation · Tension 7).
pub fn run_chapter_8(root_dir: &Path) -> Result<(), String> {
    let (p_a, p_b) = load_toronto_arrests_prevalences(root_dir);
    let chouldechova = ChouldechovaTrilemma::evaluate(p_a, p_b, 0.60, 0.75);
    let committee = DiscursiveCommittee::evaluate(&[
        [true, true, false],
        [true, false, true],
        [false, true, true],
    ]);
    let algedonic = AlgedonicChannel::evaluate(1.85, 20.0, 0.55, 8.0);
    let telic = TelicEvaluation::evaluate(0.92, true, false, 0.45);
    let contract =
        ContractAndLiabilityAudit::evaluate(0.70, 10.0, 40.0, 0.80, 8.0, 0.12, 20.0, 1.5, 2.5);
    let alert_noisy = CognitiveAlertFilter::evaluate(0.02, 0.80, 0.25, 1000);
    let alert_clean = CognitiveAlertFilter::evaluate(0.02, 0.82, 0.015, 1000);
    let mut vault = TemporalVault::seal(b"stochology-stage1-telic-commitment");
    vault
        .unseal(b"stochology-stage1-telic-commitment")
        .map_err(|e| e.to_string())?;

    println!("=== Chapter 8: Moral Luck, Apt Agency & Institutional Design (Tension 7) ===");
    println!(
        "  Toronto Arrests Chouldechova (toronto_arrests.csv): p_A={:.3}, p_B={:.3} -> FPR_A/FPR_B disparity={:.2}x",
        chouldechova.prevalence_a, chouldechova.prevalence_b, chouldechova.fpr_disparity_ratio
    );
    println!(
        "  List-Pettit Discursive Dilemma: premise_maj={} vs constitutional_veto={} | Fricker algedonic delay: {:.0} -> {:.0} shifts (TTS=8)",
        committee.premise_majority_approves,
        committee.constitutional_conjunctive_approves,
        algedonic.shifts_to_trip_undiscounted,
        algedonic.shifts_to_trip_discounted
    );
    println!(
        "  Sosa/Baron-Hershey grade={:?} (bias_removed={:.2}) | Holmstrom structural_liability={:.1}% vs frontline={:.1}%",
        telic.sosa_grade,
        telic.eliminated_outcome_bias,
        contract.structural_liability_share * 100.0,
        contract.operational_liability_share * 100.0
    );
    println!(
        "  Zheng-Meister & Gigerenzer Alerts (N=1000): noisy FDR={:.1}% vs filtered PPV={:.1}% ({} true alerts, {} false alarms)",
        alert_noisy.false_alarm_rate_fdr * 100.0,
        alert_clean.shift_ppv * 100.0,
        alert_clean.natural_freq_true_alerts,
        alert_clean.natural_freq_false_alarms
    );

    if !(chouldechova.fpr_disparity_ratio > 1.5
        && committee.discursive_dilemma_present
        && algedonic.fatal_testimonial_delay
        && contract.structural_liability_share > 0.80
        && vault.is_confirmatory)
    {
        return Err("Chapter 8 invariant failed".to_string());
    }
    Ok(())
}

/// Chapter 9 — Five Methodological Families, Process Tracing, and Reflexive Self-Application.
pub fn run_chapter_9() -> Result<(), String> {
    let clues = [
        DiagnosticClue {
            prob_given_hypothesis: 0.95,
            prob_given_rival: 0.45,
            clue_observed: true,
            weight_contribution: 15.0,
        },
        DiagnosticClue {
            prob_given_hypothesis: 0.35,
            prob_given_rival: 0.02,
            clue_observed: true,
            weight_contribution: 25.0,
        },
    ];
    let pt = ProcessTracingInference::evaluate(0.25, 10.0, 40.0, &clues);
    let brakes = ReflexiveBrakeAudit::evaluate(
        &[0.08, 0.15, 0.82, 0.91],
        &[0.0, 0.0, 1.0, 1.0],
        3.0,
        25.0,
        1.0,
        4.0,
        1.5,
        12.0,
        4.0,
    );

    let env = CommunityReserveEnv::new(DEFAULT_SEED, 24, 45.0);
    let rep_p1 = env.run_trajectory(&mut StochologicalAgent::without_epistemic_margin());
    let rep_p2 = env.run_trajectory(&mut StochologicalAgent::epistemic_only());
    let rep_p3 = env.run_trajectory(&mut StochologicalAgent::without_governance());
    let rep_full = env.run_trajectory(&mut StochologicalAgent::full());

    println!("=== Chapter 9: Methodological Families, Process Tracing & Reflexive Brakes ===");
    println!(
        "  Process Tracing (Family 5): prior={:.2} -> post={:.3} (+{:.1} dB), Keynes_V: {:.2} -> {:.2}, hoop_refutable={}",
        pt.prior_probability,
        pt.posterior_probability,
        pt.cumulative_decibans,
        pt.initial_keynes_weight_v,
        pt.posterior_keynes_weight_v,
        pt.has_refutable_hoop_condition
    );
    println!(
        "  Three Reflexive Goodhart Brakes: sharp_RES={:.3} vs lazy_RES={:.3} | optimal_welfare={:.2} > hoarding={:.2} | fast_path={:.1} <= TTS={:.1}",
        brakes.sharp_resolution,
        brakes.skeptical_inflation_resolution,
        brakes.optimal_buffer_social_welfare,
        brakes.regressive_hoarding_welfare,
        brakes.emergency_fast_path_time,
        brakes.emergency_tts
    );
    println!(
        "  Three Collapse Propositions Verified: P1={:?}, P2={:?}, P3={:?} -> Full={:?}",
        rep_p1.triadic_integrity.diagnose,
        rep_p2.triadic_integrity.diagnose,
        rep_p3.triadic_integrity.diagnose,
        rep_full.triadic_integrity.diagnose
    );

    if !(pt.posterior_probability > 0.90
        && pt.has_refutable_hoop_condition
        && brakes.all_brakes_active()
        && rep_p1.triadic_integrity.diagnose == CollapseProposition::Proposition1BlindUndersizing
        && rep_p2.triadic_integrity.diagnose == CollapseProposition::Proposition2ImpotentLucidity
        && rep_p3.triadic_integrity.diagnose == CollapseProposition::Proposition3CynicalCapture
        && rep_full.triadic_integrity.diagnose == CollapseProposition::IntactTriad)
    {
        return Err("Chapter 9 invariant failed".to_string());
    }
    Ok(())
}

/// Runs a single chapter (`1..=9`) or all chapters (`None`), verifying `data/SHA256SUMS` first.
pub fn run_all_or_chapter(root_dir: &Path, chapter: Option<usize>) -> Result<(), String> {
    let n_files = verify_sha256sums_manifest(root_dir)?;
    println!("[OK] Cryptographic manifest data/SHA256SUMS verified ({n_files} sealed datasets).");
    match chapter {
        Some(1) => run_chapter_1(),
        Some(2) => run_chapter_2(),
        Some(3) => run_chapter_3(root_dir),
        Some(4) => run_chapter_4(root_dir),
        Some(5) => run_chapter_5(root_dir),
        Some(6) => run_chapter_6(root_dir),
        Some(7) => run_chapter_7(root_dir),
        Some(8) => run_chapter_8(root_dir),
        Some(9) => run_chapter_9(),
        Some(other) => Err(format!("Unknown chapter {other}; valid chapters are 1..=9")),
        None => {
            run_chapter_1()?;
            run_chapter_2()?;
            run_chapter_3(root_dir)?;
            run_chapter_4(root_dir)?;
            run_chapter_5(root_dir)?;
            run_chapter_6(root_dir)?;
            run_chapter_7(root_dir)?;
            run_chapter_8(root_dir)?;
            run_chapter_9()?;
            println!(
                "=== ALL 9 STOCHOLOGICAL CHAPTER LABORATORIES & INVARIANTS VERIFIED (EXIT CODE 0) ==="
            );
            Ok(())
        }
    }
}
