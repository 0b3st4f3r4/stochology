//! Binary `labs` — Executes the 7 methodological and empirical laboratories of the Stochology Protocol
//! over all 13 real datasets (`data/*.csv`) and the 4 canonical Epistemological Cards (`cards/*.yaml`).

use std::env;
use std::path::Path;
use stochology_cli::{
    load_atlantic_storms_decade_environments, load_atlantic_storms_forecasts,
    load_bank_call_clusters, load_cards_from_disk, load_danish_fire, load_djia_returns,
    load_lalonde_nsw_cps, load_nyc_flights_2013_2023, load_nyc_flights_cascade_2013_2023,
    load_oot_ood_flight_reserves, load_toronto_arrests_prevalences, load_vic_elec_demand,
    verify_card_against_empirical_data, PublicInfrastructureCard04Audit,
};
use stochology_core::{
    apply_cromwell_floor, empirical_wasserstein_1, hmac_sha256_hex, mean, sample_std,
    ChouldechovaAudit, ExceedanceDesignEffect, GoodhartTaxonomyAudit, InfoGapOption,
    KennedyOHaganDiscrepancy, ManskiBounds, MengAudit, MonteCarloSplitTreeAudit, NonlinearSummary,
    OnlineMoments, OperationalWeightingGap, PerformativeSystem, Rng, DEFAULT_SEED,
};
use stochology_governance::{
    estimate_label_shift_four_methods, fit_calibrated_logistic_1d, fit_covariate_drift_extra_wcp,
    query_with_bitemporal_clock, AdaptiveConformalInference, BitemporalRecord,
    MurphyCorpCircuitBreaker, PredeterminedChangeControlPlan, TemporalVault,
};
use stochology_lenses::{
    amplitude_constrained_mi_bound, blahut_arimoto_capacity, chow_invariance_f_stat,
    cinelli_hazlett_robustness_value, dedh_moment_tail_index, empirical_kelly_optimal_fraction,
    expected_shortfall_99, fano_error_floor, gpd_pot_fit, hidden_confounding_threshold_lambda_star,
    logarithmic_score, run_icp, run_nonlinear_icp_nile, AlmostPerfectMachine,
    AnalogWaveComputingSynthesis, AuthenticModelForm, CausalEnvironment,
    ComplexFractalToroidalTechniumSynthesis, CorpPavDecomposition, CostLossRule,
    CriticalStochasticRationalitySynthesis, DpiMarkovChain, EmpiricalCriticalRationalityAudit,
    EmpiricalStocharyCyberneticAudit, EmpiricalTriadicTechniumAudit, EmpiricalWaveAnalogAudit,
    ExactCoinCalibrationBenchmark, HillTailDiagnostic, MixingTimeAudit,
    RelationalComplexSystemsSynthesis, SdpiHierarchy, StocharyConjecturalCyberneticSynthesis,
    TimeKellyDiagnostic,
};
use stochology_siga::{fit_shelf_beta_prior, AlertFatigueAudit, EvidenceProcess};

/// Lab 1: Dynamics of the Three Ontological Layers ($L_1 \to L_2 \to L_3 \to L_1$), Meng, Manski, Jensen, and Performativity.
fn lab_layers(root: &Path) {
    println!("=== LAB 1: THREE ONTOLOGICAL LAYERS AND BRIDGE MECHANICS (L1 -> L2 -> L3 -> L1) ===");
    let (nsw_t, nsw_c, cps_c) = load_lalonde_nsw_cps(root);
    let mut pop_y = Vec::with_capacity(nsw_c.len() + cps_c.len());
    let mut rec = Vec::with_capacity(nsw_c.len() + cps_c.len());
    for &y in &cps_c {
        pop_y.push(y);
        rec.push(true);
    }
    for &y in &nsw_c {
        pop_y.push(y);
        rec.push(false);
    }
    let meng = MengAudit::from_population(&rec, &pop_y);
    let manski = ManskiBounds::compute(&nsw_t, nsw_t.len() + 30, 0.0, 60_000.0);
    let (lee_lo, lee_hi, lee_q) =
        ManskiBounds::lee_trimming_bounds(&nsw_t, nsw_t.len() + 15, &nsw_c, nsw_c.len() + 30);
    let w1_nsw = empirical_wasserstein_1(&nsw_t, &nsw_c);
    println!(
        "  [Bridge L1->L2] Meng (CPS n={} vs NSW n={}): rho_R_Y={:+.4}, selection_err={:+.2}, n_eff_Meng={:.2} | Manski MNAR=[{:.1}, {:.1}] | LeeBounds(q={:.3})=[{:.1}, {:.1}], W1={:.1}",
        cps_c.len(),
        nsw_c.len(),
        meng.rho_r_y,
        meng.estimation_error,
        meng.n_eff_meng,
        manski.lower_bound,
        manski.upper_bound,
        lee_q,
        lee_lo,
        lee_hi,
        w1_nsw
    );

    let clusters = load_bank_call_clusters(root);
    let daily_totals: Vec<f64> = clusters.iter().map(|c| c.iter().sum::<f64>()).collect();
    let split_idx = daily_totals.len() * 4 / 5;
    // Convex congestion cost f(x) = x^2 evaluated with out-of-sample CRPS on held-out days
    let jensen = NonlinearSummary::evaluate_out_of_sample(
        &daily_totals[..split_idx],
        &daily_totals[split_idx..],
        |x| x * x,
        2.0,
    );
    println!(
        "  [Bridge L2->L3] Jensen Gap (bank_calls, {} days): f(E[X])={:.2}, E[f(X)]={:.2}, Delta_J={:+.2}, median={:.2}, q05={:.2}, q95={:.2}, OOS_CRPS={:.2}",
        daily_totals.len(),
        jensen.f_at_mean,
        jensen.expected_f,
        jensen.jensen_gap,
        jensen.median_f,
        jensen.q05_f,
        jensen.q95_f,
        jensen.empirical_crps
    );

    let perf_sys = PerformativeSystem::new(1.0, 0.2, 0.5, 1.0, 1.5);
    let perf = perf_sys.diagnose();
    let perfgd = perf_sys.perfgd_trajectory(0.0, 0.5, 50);
    let theta_perfgd = *perfgd.last().unwrap_or(&0.0);
    let osc = PerformativeSystem::simulate_rrm_oscillation(5);
    let goodhart = GoodhartTaxonomyAudit::evaluate(DEFAULT_SEED, 45.0, 6.2);
    println!(
        "  [Bridge L3->L1] Performative Prediction: theta_SL={:.4}, theta_PS={:.4}, theta_PO={:.4} (PerfGD={:.4}), PR(PS)={:.4} vs PR(PO)={:.4}, r_RRM={:.3} (converges={}), oscillation_violated={:?} | Goodhart4(Reg={:+.2}, Ext={:+.2}, Causal_NILE={:.1}x, Adv_grad1={:+.3}, verified={})",
        perf.theta_sl,
        perf.theta_ps,
        perf.theta_po,
        theta_perfgd,
        perf.risk_at_ps,
        perf.risk_at_po,
        perf.rrm_contraction_radius,
        perf.rrm_converges,
        osc,
        goodhart.regressional_winners_curse_inflation,
        goodhart.extremal_tail_decoupling_drop,
        goodhart.causal_nile_penalty_ratio,
        goodhart.adversarial_grad1_dpr_at_ps,
        goodhart.all_four_variants_audited
    );
}

/// Lab 2: Kennedy-O'Hagan Gaussian Process Structural Discrepancy and Info-Gap + Sniedovich Safeguard.
fn lab_uncertainty() {
    println!("=== LAB 2: KENNEDY-O'HAGAN GP DISCREPANCY AND INFO-GAP + SNIEDOVICH ===");
    let koh = KennedyOHaganDiscrepancy::canonical_benchmark();
    let x_obs = [0.1, 0.2, 0.4, 0.8, 1.2, 1.6, 2.0, 2.4];
    let y_obs: Vec<f64> = x_obs.iter().map(|&x| 0.80 * x - 0.08 * x * x).collect();
    let x_ext = [2.6, 2.8, 3.0];
    let y_ext: Vec<f64> = x_ext.iter().map(|&x| 0.80 * x - 0.08 * x * x).collect();
    let (_eb_cal, ell_opt, sig_opt, lml_opt) = KennedyOHaganDiscrepancy::calibrate_empirical_bayes(
        &x_obs, &y_obs, 0.80, 0.005, &x_ext, &y_ext,
    );
    println!(
        "  [Kennedy-O'Hagan GP] theta_true={:.3} | without_GP={:.4} (bias={:.4}, extrap_err={:.4}) | with_GP={:.4} (bias={:.4}, extrap_err={:.4}) | EB_Cholesky(ell*={:.2}, sigma*={:.2}, LML={:+.2})",
        koh.theta_true,
        koh.theta_naive_without_gp,
        koh.pseudo_convergence_bias_without_gp,
        koh.extrapolation_error_without_gp,
        koh.theta_anchored_with_gp,
        koh.parameter_error_with_gp,
        koh.extrapolation_error_with_gp,
        ell_opt,
        sig_opt,
        lml_opt
    );

    let fragile = InfoGapOption {
        id: "fragile_nominal_turkey",
        nominal_performance: 100.0,
        adverse_sensitivity: 25.0,
        favorable_propensity: 20.0,
        wald_global_maximin_bound: -450.0,
        reversible: false,
    }
    .evaluate(60.0, 120.0, 20.0);

    let robust = InfoGapOption {
        id: "robust_satisficing_wald",
        nominal_performance: 88.0,
        adverse_sensitivity: 8.0,
        favorable_propensity: 15.0,
        wald_global_maximin_bound: 45.0,
        reversible: true,
    }
    .evaluate(60.0, 120.0, 20.0);

    println!(
        "  [Info-Gap + Sniedovich] {}: alpha_hat={:.2}, zero_ing={:.1}, Wald={:.1}, approved={} | {}: alpha_hat={:.2}, beta_hat={:.2}, Wald={:.1}, approved={}",
        fragile.id,
        fragile.robustness_alpha_hat,
        fragile.zero_ing_at_nominal,
        fragile.wald_global_maximin_bound,
        fragile.approved_sniedovich,
        robust.id,
        robust.robustness_alpha_hat,
        robust.opportuneness_beta_hat,
        robust.wald_global_maximin_bound,
        robust.approved_sniedovich
    );
}

/// Lab 3: The Five Transversal Lenses on Real Datasets.
fn lab_lenses(root: &Path) {
    println!(
        "=== LAB 3: THE FIVE TRANSVERSAL LENSES (SDPI/DPI, TIME/KELLY, HILL, ICP, CORP-PAV) ==="
    );
    // 1. Information Lens
    let bsc_w = vec![vec![0.85, 0.15], vec![0.15, 0.85]];
    let sdpi = SdpiHierarchy::compute(&[0.5, 0.5], &bsc_w);
    let (cap_ba, _) = blahut_arimoto_capacity(&bsc_w, 100, 1e-12);
    let dpi = DpiMarkovChain::compute(
        &[0.5, 0.5],
        &[vec![0.90, 0.10], vec![0.10, 0.90]],
        &[vec![0.85, 0.15], vec![0.15, 0.85]],
        &[vec![0.80, 0.20], vec![0.20, 0.80]],
    );
    let i_amp = amplitude_constrained_mi_bound(1.5, 1.0, 0.05);
    let fano = fano_error_floor(4, (4.0_f64).ln(), 0.65 * (4.0_f64).ln());
    println!(
        "  [Lens 1: Information] SDPI: eta_chi2={:.4} <= eta_KL={:.4} <= eta_TV={:.4} (valid={}, C_BA={:.4} nats) | DPI W->X->Y->Z: {:.4} >= {:.4} >= {:.4} (monotonic={}) | I_amp={:.4} | Fano P_e >= {:.4}",
        sdpi.eta_chi2,
        sdpi.eta_kl,
        sdpi.eta_tv,
        sdpi.hierarchy_valid,
        cap_ba,
        dpi.i_w_x_nats,
        dpi.i_w_y_nats,
        dpi.i_w_z_nats,
        dpi.dpi_satisfied,
        i_amp,
        fano
    );

    // 2. Time Lens (DJIA 1885-2023 + Kelly + Mixing Time)
    let djia_returns = load_djia_returns(root);
    let djia_moments = OnlineMoments::from_slice(&djia_returns);
    let (f_emp_djia, _g_emp_djia, f_merton_djia) = empirical_kelly_optimal_fraction(&djia_returns);
    let mu_annual = mean(&djia_returns) * 252.0;
    let sigma_annual = sample_std(&djia_returns) * (252.0_f64).sqrt();
    let kelly = TimeKellyDiagnostic::compute(mu_annual, sigma_annual, 0.54, 0.25, 0.20);
    let mix = MixingTimeAudit::compute(sdpi.eta_kl, 0.002, 4.0, 0.02);
    println!(
        "  [Lens 2: Time] DJIA ({} trading days, kurtosis={:.1}, JB={:.0}): g_ensemble={:.4} vs g_time_Ito={:.4} (drain={:.4}) | Kelly f*={:.4} (emp_DJIA={:.2} vs Merton={:.2}) | Mixing steps={}",
        djia_returns.len(),
        djia_moments.excess_kurtosis(),
        djia_moments.jarque_bera_stat(),
        kelly.ensemble_rate_mu,
        kelly.time_rate_ito,
        kelly.variance_drain,
        kelly.asymmetric_kelly_fraction,
        f_emp_djia,
        f_merton_djia,
        mix.mixing_steps
    );

    // 3. Scale & Tail Lens (Danish Fire + Victoria Electricity Demand)
    let fire_losses = load_danish_fire(root);
    let hill = HillTailDiagnostic::audit(&fire_losses, Some(100));
    let xi_dedh = dedh_moment_tail_index(&fire_losses, 100);
    let (xi_gpd, _beta_gpd, es99_pot) = gpd_pot_fit(&fire_losses, hill.threshold_x_nk);
    let es99_emp = expected_shortfall_99(&fire_losses);
    let vic_demand = load_vic_elec_demand(root);
    let hill_vic = HillTailDiagnostic::audit(&vic_demand, Some(100));
    println!(
        "  [Lens 3: Tail] Danish Fire (n={}): Hill alpha_k={:.4} (+/- {:.4}, DEdH_xi={:.3}, GPD_xi={:.3}), regime={:?}, median={:.3}, MAD={:.3}, q99={:.2}, ES99={:.2} (POT={:.2}), tail_n_eff={:.1} | VicElec (n={}): q99={:.1} MW",
        hill.sample_size_n,
        hill.alpha_hill,
        hill.asymptotic_se_alpha,
        xi_dedh,
        xi_gpd,
        hill.regime,
        hill.median,
        hill.mad,
        hill.quantile_99,
        es99_emp,
        es99_pot,
        hill.penalized_tail_n_eff,
        hill_vic.sample_size_n,
        hill_vic.quantile_99
    );

    // 4. Invariance Lens (ICP + Linear OLS Penalty + Nonlinear NILE/IRM over Phi + Authentic Form)
    let mut rng = Rng::new(DEFAULT_SEED);
    let mut envs = Vec::new();
    for (idx, (sc, sp_w)) in [(1.0, 1.2), (2.2, -1.4), (0.7, 2.8)]
        .into_iter()
        .enumerate()
    {
        let mut x = Vec::with_capacity(300);
        let mut y = Vec::with_capacity(300);
        for _ in 0..300 {
            let x0 = rng.normal(0.0, sc);
            let yi = 1.75 * x0 + rng.normal(0.0, 0.12);
            let x1 = sp_w * yi + rng.normal(0.0, 0.35);
            x.push(vec![x0, x1]);
            y.push(yi);
        }
        envs.push(CausalEnvironment {
            name: format!("regime_{idx}"),
            x,
            y,
        });
    }
    let icp = run_icp(&envs, 0.12);
    let storm_envs = load_atlantic_storms_decade_environments(root);
    let nl_icp = run_nonlinear_icp_nile(&storm_envs, 0.15);
    let auth_form = AuthenticModelForm::audit_environments(
        DEFAULT_SEED,
        &storm_envs,
        &[
            "atom_0_barometric_pressure_drop".to_string(),
            "atom_1_regime_shifted_lat_proxy".to_string(),
        ],
        0.15,
        true,
    );
    let lambda_star = hidden_confounding_threshold_lambda_star(1.75, 0.08, 0.60);
    let chow_f0 = chow_invariance_f_stat(&envs, &[0]);
    let chow_f1 = chow_invariance_f_stat(&envs, &[1]);
    let rv_ch = cinelli_hazlett_robustness_value(1.75 / 0.08, 898, 1.0);
    let pen_fmt: Vec<String> = icp
        .env_ols_invariance_penalties
        .iter()
        .map(|v| format!("{v:.4}"))
        .collect();
    let nl_fmt: Vec<String> = nl_icp
        .nonlinear_nile_irm_penalties
        .iter()
        .map(|v| format!("{v:.4}"))
        .collect();
    println!(
        "  [Lens 4: Invariance] ICP S*={:?}, env_OLS_penalty={:?} (ChowF=[{:.2}, {:.1}]), Storms_nonlinear_NILE_IRM={:?}, Lambda*={:.3} (RV={:.3}) | AuthenticForm(S*={:?}, discarded='{}', A={})",
        icp.accepted_causal_predictors,
        pen_fmt,
        chow_f0,
        chow_f1,
        nl_fmt,
        lambda_star,
        rv_ch,
        auth_form.invariant_core_s_star,
        auth_form.discarded_atoms_named[0].atom_name,
        auth_form.approved_authentic_form
    );

    // 5. Decision Lens (CORP-PAV on Atlantic Storms + Exact Fair-Coin Benchmark)
    let (probs, outcomes) = load_atlantic_storms_forecasts(root);
    let corp = CorpPavDecomposition::compute(&probs, &outcomes);
    let storm_rule = CostLossRule::new(1_800.0, 34_000.0, 1.0);
    let rev_storms_pav =
        storm_rule.relative_economic_value(&corp.recalibrated_probabilities_q, &outcomes);
    let logs_storms = logarithmic_score(&corp.recalibrated_probabilities_q, &outcomes, 0.001);
    let coin = ExactCoinCalibrationBenchmark::evaluate(DEFAULT_SEED, 1_000);
    println!(
        "  [Lens 5: Decision] CORP-PAV (Atlantic Storms, n={}): BS={:.5} = REL({:.5}) - RES({:.5}) + UNC0({:.5}) [id_err={:.2e}, LogS_PAV={:.4}, REV_PAV={:.3}] | FairCoin(p0=0.50): agree@0.30={:.0}%, REL_over={:.4} vs REL_cal={:.4}, regret@0.70=+{:.2}",
        probs.len(),
        corp.brier_total,
        corp.rel_miscalibration,
        corp.res_resolution,
        corp.unc0_uncertainty,
        corp.identity_error,
        logs_storms,
        rev_storms_pav,
        coin.decision_agreement_rate_at_p_star_a * 100.0,
        coin.overconfident_corp.rel_miscalibration,
        coin.calibrated_corp.rel_miscalibration,
        coin.shifted_threshold_excess_regret_per_unit
    );

    // 6. Almost-Perfect Machine M = (T, Phi, theta) (P1-P5)
    let machine = AlmostPerfectMachine::evaluate_canonical(DEFAULT_SEED);
    println!(
        "  [Almost-Perfect Machine M=(T,Phi,theta)] sup_err={:.4} (Barron C_f={:.2}), DPI_gap={:.4} nats, regions={} (ratio={:.1}x), d_eff={:.2}/{}, P1..P5={}",
        machine.sup_error_epsilon,
        machine.barron_norm_c_f,
        machine.dpi_gap_delta_nats,
        machine.composed_linear_regions,
        machine.region_to_parameter_ratio,
        machine.atomic_effective_dimension,
        machine.width_w,
        machine.all_p1_to_p5_approved
    );

    // 7. Relational Ontology & Complex Systems (Sunyata/RQM, Prigogine/Autopoiesis, Schelling 1971, Daisyworld 1983, Ontological Stack)
    let rel_syn = RelationalComplexSystemsSynthesis::evaluate_canonical(DEFAULT_SEED);
    println!(
        "  [Relational Ontology & Complex Systems] MI_rel={:.4} vs MI_iso={:.4} nats | Prigogine dS={:+.2} (turnover={:.1}%, closure={:.1}%) | Schelling(tau=0.125->{:.1}% vs tau=0.375->{:.1}%) | Daisyworld(MAE={:.2}C vs dead={:.2}C, dT/dL={:+.2}) | verified={}",
        rel_syn.relational_figuration.relational_mutual_information_nats,
        rel_syn.relational_figuration.isolated_svabhava_mutual_information_nats,
        rel_syn.dissipative_autopoiesis.net_open_entropy_change_ds,
        rel_syn.dissipative_autopoiesis.material_turnover_fraction * 100.0,
        rel_syn.dissipative_autopoiesis.autopoietic_pattern_fidelity * 100.0,
        rel_syn.schelling_emergence.integrated_similarity_at_low_tau * 100.0,
        rel_syn.schelling_emergence.emergent_segregation_at_mild_tau * 100.0,
        rel_syn.daisyworld_homeostasis.homeostatic_mean_abs_error_c,
        rel_syn.daisyworld_homeostasis.lifeless_mean_abs_error_c,
        rel_syn.daisyworld_homeostasis.coexistence_feedback_slope_dt_dl,
        rel_syn.all_six_axes_verified
    );

    // 8. Physical, Analog & Wave-Interference Computing (Horowitz/Collapse, RLC ODE, AIMC/NAT/PKU HP-INV/BlockAMC/HaLoRA, SIREN/Wave-RNN, 20W Brain DPI)
    let analog_syn = AnalogWaveComputingSynthesis::evaluate_canonical(DEFAULT_SEED);
    let emp_wave = EmpiricalWaveAnalogAudit::evaluate_series(&vic_demand);
    println!(
        "  [Analog & Wave Computing] DRAM/FP32={:.0}x, collapse_var={:.2}->anchored={:.2} | RLC_iso_err={:.1e} | AIMC_sigma0.6(std={:.1}% vs NAT={:.1}%), PKU_HP_INV_err={:.2e} ({} iters, BlockAMC_err={:.2e}/{}ops, HaLoRA_drift_red={:.1}%) | SIREN_compr={:.0}x (Lap_MSE={:.1e} vs ReLU={:.1}), WaveRNN_sel={:.1}% | BrainDPI_compr={:.1e}x (ret={:.1}%) | VicElec_wave(R2={:.2}, HP_INV={:.2e}) | verified={}",
        analog_syn.von_neumann_and_collapse.dram_to_fp32_add_ratio,
        analog_syn.von_neumann_and_collapse.unanchored_synthetic_variance_retention,
        analog_syn.von_neumann_and_collapse.neurosymbolic_anchored_variance_retention,
        analog_syn.ode_isomorphism.max_isomorphism_discrepancy,
        analog_syn.aimc_analog_wall_hp_inv.standard_accuracy_at_sigma_060 * 100.0,
        analog_syn.aimc_analog_wall_hp_inv.nat_accuracy_at_sigma_060 * 100.0,
        analog_syn.aimc_analog_wall_hp_inv.pku_hp_inv_final_relative_error,
        analog_syn.aimc_analog_wall_hp_inv.pku_hp_inv_iterations_to_fp32,
        analog_syn.aimc_analog_wall_hp_inv.pku_block_amc_relative_error,
        analog_syn.aimc_analog_wall_hp_inv.pku_block_amc_operations_count,
        analog_syn.aimc_analog_wall_hp_inv.abnat_halora_attention_drift_reduction * 100.0,
        analog_syn.wave_interference.phase_amplitude_compression_ratio,
        analog_syn.wave_interference.siren_helmholtz_laplacian_mse,
        analog_syn.wave_interference.relu_helmholtz_laplacian_mse,
        analog_syn.wave_interference.wave_rnn_target_probe_selectivity * 100.0,
        analog_syn.sensory_dpi_and_ecosystem.total_sensory_compression_ratio,
        analog_syn.sensory_dpi_and_ecosystem.actionable_information_retention_ratio * 100.0,
        emp_wave.wave_explained_variance_r2,
        emp_wave.empirical_hp_inv_relative_error,
        analog_syn.all_five_axes_verified && emp_wave.empirical_wave_verified
    );

    // 9. Critical Theory of Stochastic Rationality, Algorithmic Governmentality, and Material Collapse/Overcoming
    let crit_syn = CriticalStochasticRationalitySynthesis::evaluate_canonical(DEFAULT_SEED);
    let toronto_prev = load_toronto_arrests_prevalences(root);
    let emp_crit = EmpiricalCriticalRationalityAudit::evaluate_real_datasets(
        &fire_losses,
        toronto_prev,
        &storm_envs,
        &vic_demand,
    );
    println!(
        "  [Critical Stochastic Rationality] Adorno(Delta_NonId={:+.1}, ES99_erased={:.1}%, VaR99_breach={:.1}% vs Gauss={:.1}%, KOH_err={:.3}, Reif_excess={:+.3}) | Marcuse/Rouvroy(Parrot_CF_MSE={:.2} vs Grounded={:.3}, NILE_ratio={:.1}x, FPR_A/B={:.2}x) | Collapse(var_ret={:.3}, KL={:.2} nats; informative 300T@{}-{}, Alia_0.6={:.0}%) | Overcoming(rule_sat={:.0}% vs {:.0}%, A-ECC={:.0}% vs raw {:.0}%, HP-INV={:.1e}, SIREN/ReLU={:.1e}/{:.0}; informative NAT={:.1}%, ACCEL={:.0} TOPS/W) | EmpRealData(Fire_erased={:.1}%, Toronto_FPR={:.2}x, Storms_ratio={:.1}x) | identities={} | verified={}",
        crit_syn.adorno_non_identical_and_reification.non_identical_liquidation_gap,
        crit_syn.adorno_non_identical_and_reification.es99_erasure_fraction * 100.0,
        crit_syn.adorno_non_identical_and_reification.var99_holdout_exceedance_empirical_vs_gaussian.0 * 100.0,
        crit_syn.adorno_non_identical_and_reification.var99_holdout_exceedance_empirical_vs_gaussian.1 * 100.0,
        crit_syn.adorno_non_identical_and_reification.deployed_koh_extrapolation_error,
        crit_syn.adorno_non_identical_and_reification.status_quo_reification_excess_risk,
        crit_syn.marcuse_rouvroy_stochastic_parrot.parrot_counterfactual_intervention_mse,
        crit_syn.marcuse_rouvroy_stochastic_parrot.causal_grounded_counterfactual_mse,
        crit_syn.marcuse_rouvroy_stochastic_parrot.spurious_proxy_invariance_penalty_ratio,
        crit_syn.marcuse_rouvroy_stochastic_parrot.preemptive_profiling_fpr_disparity_ratio,
        crit_syn.material_collapse_triad.synthetic_autophagy_variance_retention,
        crit_syn.material_collapse_triad.synthetic_autophagy_kl_divergence_nats,
        crit_syn.material_collapse_triad.human_token_exhaustion_window_years.0,
        crit_syn.material_collapse_triad.human_token_exhaustion_window_years.1,
        crit_syn.material_collapse_triad.ferran_alia_threshold_cliff_curve[3] * 100.0,
        crit_syn.dialectical_overcoming.neurosymbolic_rule_satisfaction_rate * 100.0,
        crit_syn.dialectical_overcoming.unanchored_statistical_rule_satisfaction_rate * 100.0,
        crit_syn.dialectical_overcoming.a_ecc_compensated_vs_raw_accuracy.0 * 100.0,
        crit_syn.dialectical_overcoming.a_ecc_compensated_vs_raw_accuracy.1 * 100.0,
        crit_syn.dialectical_overcoming.pku_40nm_rram_24bit_hp_inv_error,
        crit_syn.dialectical_overcoming.siren_vs_relu_helmholtz_laplacian_mse.0,
        crit_syn.dialectical_overcoming.siren_vs_relu_helmholtz_laplacian_mse.1,
        crit_syn.dialectical_overcoming.nat_vs_standard_accuracy_at_sigma_060.0 * 100.0,
        crit_syn.dialectical_overcoming.tsinghua_accel_photonic_tops_per_watt,
        emp_crit.danish_fire_non_identical_erasure_fraction * 100.0,
        emp_crit.toronto_arrests_preemptive_fpr_ratio,
        emp_crit.atlantic_storms_spurious_to_invariant_penalty_ratio,
        crit_syn.all_identities_closed,
        crit_syn.all_four_axes_verified && emp_crit.empirical_critical_verified
    );

    // 10. Stochary-Conjectural & Cybernetic Synthesis (4 Stock Axioms, 150-Node Cybernetic Graph, Powell/Friston/Pearl, Scarf/WDRO/CVaR, VSM/PATE)
    let stoch_syn = StocharyConjecturalCyberneticSynthesis::evaluate_canonical(DEFAULT_SEED);
    let bank_clusters_lens = load_bank_call_clusters(root);
    let emp_stoch = EmpiricalStocharyCyberneticAudit::evaluate_real_datasets(
        &djia_returns,
        &bank_clusters_lens,
        &fire_losses,
        &vic_demand,
        (&probs, &outcomes),
    );
    println!(
        "  [Stochary-Conjectural & Cybernetics] Tree150(H1={:.4}, J1={:.4}, H2={:.4}, J2={:.4}, orphans={}) | Axioms(LowPass={:.2} vs Bullwhip={:.2}x, Peters_g={:+.4} vs Kelly_sim={:+.4}, TailVega={:+.1} vs JIT={:+.1}, rho1={:.2}) | Powell(theta*={:.2}, CFA_gain=+{:.1}), VOI(ambiguous probe={:.3}<act={:.3}, resolved probe={:.3}>act={:.3}, match={}), Pearl(FD={:+.2}, transport_err={:.3}) | Scarf(s*={:.0}, S*={:.0}), TTS/TTR={:.2} (REI_red={:.0}%), CVaR95={:.1}, WDRO_disapp={:+.2} vs naive={:+.2} | SupplyChain(GG1_Wq={:.2}h vs Kingman={:.2}h, cliff_sim={:.1}h, Sterman_SL_damp={:.2}, GravesWillems_MEIO=-{:.0}%, Eppen_tail_surge={:.2}x, TBS_save=-{:.0}%, Ivanov_fill={:.0}%) | VSM(ConantAshby_R2={:.3}, Noise={:.3}->{:.3}, PLAT r_s={:.2}/D_KS={:.3}, Algedonic={}, PATE_Q3_alarm={}) | Emp5CSVs(BankLP={:.2}, FireWDRO={:+.1}, VicReserveCov={:.2}, discarded={:?}) | identities={} | verified={}",
        stoch_syn.topology_and_stock_axioms.shannon_entropy_l1_bits,
        stoch_syn.topology_and_stock_axioms.balance_index_j_l1,
        stoch_syn.topology_and_stock_axioms.shannon_entropy_l2_bits,
        stoch_syn.topology_and_stock_axioms.balance_index_j_l2,
        stoch_syn.topology_and_stock_axioms.orphan_single_leaf_subbranches,
        stoch_syn.topology_and_stock_axioms.low_pass_buffered_variance_ratio,
        stoch_syn.topology_and_stock_axioms.unbuffered_bullwhip_amplification_ratio,
        stoch_syn.topology_and_stock_axioms.peters_time_growth_rate,
        stoch_syn.topology_and_stock_axioms.ergodic_buffered_time_growth_rate,
        stoch_syn.topology_and_stock_axioms.buffered_convex_tail_vega,
        stoch_syn.topology_and_stock_axioms.unbuffered_jit_concave_tail_vega,
        stoch_syn.topology_and_stock_axioms.stock_statefulness_autocorrelation,
        stoch_syn.powell_friston_causal_engine.powell_cfa_tuned_theta,
        stoch_syn.powell_friston_causal_engine.powell_cfa_tuned_vs_myopic_gain,
        stoch_syn.powell_friston_causal_engine.voi_probe_vs_act_now_loss_ambiguous.0,
        stoch_syn.powell_friston_causal_engine.voi_probe_vs_act_now_loss_ambiguous.1,
        stoch_syn.powell_friston_causal_engine.voi_probe_vs_act_now_loss_resolved.0,
        stoch_syn.powell_friston_causal_engine.voi_probe_vs_act_now_loss_resolved.1,
        stoch_syn.powell_friston_causal_engine.deployed_policy_choices_match_voi,
        stoch_syn.powell_friston_causal_engine.pearl_front_door_identified_effect,
        stoch_syn.powell_friston_causal_engine.pearl_transportability_error,
        stoch_syn.strategic_stock_plant.scarf_optimal_s_and_big_s.0,
        stoch_syn.strategic_stock_plant.scarf_optimal_s_and_big_s.1,
        stoch_syn.strategic_stock_plant.simchi_levi_tts_to_ttr_ratio,
        stoch_syn.strategic_stock_plant.simchi_levi_rei_reduction_fraction * 100.0,
        stoch_syn.strategic_stock_plant.rockafellar_uryasev_var95_and_cvar95.1,
        stoch_syn.strategic_stock_plant.wdro_robust_oos_disappointment,
        stoch_syn.strategic_stock_plant.wdro_naive_oos_disappointment,
        stoch_syn.strategic_stock_plant.supply_chain_physics.kingman_vut_buffered_vs_cliff_wait_hours.0,
        stoch_syn.strategic_stock_plant.supply_chain_physics.kingman_formula_wait_hours.0,
        stoch_syn.strategic_stock_plant.supply_chain_physics.kingman_vut_buffered_vs_cliff_wait_hours.1,
        stoch_syn.strategic_stock_plant.supply_chain_physics.sterman_supply_line_oscillation_damping_ratio,
        stoch_syn.strategic_stock_plant.supply_chain_physics.clark_scarf_graves_willems_meio_saving_fraction * 100.0,
        stoch_syn.strategic_stock_plant.supply_chain_physics.eppen_risk_pooling_normal_vs_tail_multiplier.1,
        stoch_syn.strategic_stock_plant.supply_chain_physics.allon_van_mieghem_tbs_cost_saving_fraction * 100.0,
        stoch_syn.strategic_stock_plant.supply_chain_physics.ivanov_ripple_effect_viability_service_rate * 100.0,
        stoch_syn.cybernetic_vsm_and_pate.conant_ashby_regulator_homomorphism_r2,
        stoch_syn.cybernetic_vsm_and_pate.kahneman_judgment_noise_before_and_after.0,
        stoch_syn.cybernetic_vsm_and_pate.kahneman_judgment_noise_before_and_after.1,
        stoch_syn.cybernetic_vsm_and_pate.plat_spearman_and_ks.0,
        stoch_syn.cybernetic_vsm_and_pate.plat_spearman_and_ks.1,
        stoch_syn.cybernetic_vsm_and_pate.algedonic_not_measured_not_executed_enforced,
        stoch_syn.cybernetic_vsm_and_pate.pate_q3_jackpot_algedonic_alarm_triggered,
        emp_stoch.bank_calls_low_pass_variance_ratio,
        emp_stoch.danish_fire_naive_vs_wdro_disappointment.1,
        emp_stoch.vic_elec_reserve_excursion_coverage,
        emp_stoch.discarded_findings,
        stoch_syn.all_identities_closed,
        stoch_syn.all_four_axes_verified && emp_stoch.empirical_stochary_verified
    );

    // 11. Triadic Mathematical & Physical Synthesis (C x Fractals x T^d) & Technium/Technosphere Governance
    let triad_syn = ComplexFractalToroidalTechniumSynthesis::evaluate_canonical(DEFAULT_SEED);
    let emp_triad = EmpiricalTriadicTechniumAudit::evaluate_real_datasets(
        &vic_demand,
        &fire_losses,
        (&probs, &outcomes),
        &djia_returns,
        &bank_clusters_lens,
    );
    println!(
        "  [Triadic (C x Fractals x T^d) & Technium Synthesis] Axis1_C(Gabor_res={:.1e} vs tanh={:.3}, LSSA_freq_err={:.1e}, Adjoint_err={:.2e}) | Axis2_Iso(C4x4->R8x8={:.2e}, BlockAMC_16x16={:.2e}, HMatrix_admissible={:.1e} vs naive={:.1}) | Axis3_Frac(G0={:.2}uS, PCM_drift_MSE={:.4} vs {:.3}, CTRW_ratio_sim={:.2} vs 16^(b/a)={:.2}/Fick={:.1}, ECC_trimmed={:.0}% vs plain={:.0}%) | Axis4_Tail(cascade_d2tau={:.3} vs theory={:.3}, PDHTE_xi={:.3}/DEdH={:.3}, Welsch_RMSE={:.3} vs OLS={:.3}, Basel_G8={:.4}<pi2/6) | Axis5_Torus(Weyl_D*={:.4} vs rational={:.3}, mixGamma_tr>={}, HM_DFM_err={:.4} vs RFF={:.4}, KL MLLS={:.1e}/BBSE={:.1e}/CC={:.1e}) | Axis6_SNA_Technium(Pi={:.2}, SNA_lyap={:.3}/mu={:.2}/sync={:.1e}, RLHF_flips={}/rho_tau1={:.3}, PerfGD/RRM->theta_PO={:.4}/{:.4}, FHRR_cos={:.2}, LTC_max={:.2} vs Euler={:.1e}) | Emp5CSVs(Fire_xi_JK={:.3}, Storms_q_DFM={:.3} vs realized={:.3}, DJIA_d2tau={:+.3}) | identities={} | discarded={:?} | verified={}",
        triad_syn.complex_wave_gabor_helmholtz.wire_complex_gabor_helmholtz_residual_mse,
        triad_syn.complex_wave_gabor_helmholtz.tanh_mlp_helmholtz_residual_mse,
        triad_syn.complex_wave_gabor_helmholtz.lssa_learned_frequency_relative_error,
        triad_syn.complex_wave_gabor_helmholtz.hughes_adjoint_in_situ_gradient_max_error,
        triad_syn.complex_real_isomorphism_hmatrix.complex_4x4_to_real_8x8_inversion_discrepancy,
        triad_syn.complex_real_isomorphism_hmatrix.two_stage_block_amc_16x16_relative_error,
        triad_syn.complex_real_isomorphism_hmatrix.hmatrix_admissible_vs_naive_relative_error.0,
        triad_syn.complex_real_isomorphism_hmatrix.hmatrix_admissible_vs_naive_relative_error.1,
        triad_syn.fractional_ctrw_analog_ecc.atomic_conductance_quantum_g0_microsiemens,
        triad_syn.fractional_ctrw_analog_ecc.pcm_uncompensated_vs_compensated_drift_mse.1,
        triad_syn.fractional_ctrw_analog_ecc.pcm_uncompensated_vs_compensated_drift_mse.0,
        triad_syn.fractional_ctrw_analog_ecc.ctrw_simulated_vs_anomalous_vs_fickian_spread_ratio.0,
        triad_syn.fractional_ctrw_analog_ecc.ctrw_simulated_vs_anomalous_vs_fickian_spread_ratio.1,
        triad_syn.fractional_ctrw_analog_ecc.ctrw_simulated_vs_anomalous_vs_fickian_spread_ratio.2,
        triad_syn.fractional_ctrw_analog_ecc.analog_ecc_trimmed_vs_plain_ls_recovery.0 * 100.0,
        triad_syn.fractional_ctrw_analog_ecc.analog_ecc_trimmed_vs_plain_ls_recovery.1 * 100.0,
        triad_syn.multifractal_pdhte_rag_collapse.mmar_multifractal_concavity_curvature,
        triad_syn.multifractal_pdhte_rag_collapse.mmar_theoretical_curvature,
        triad_syn.multifractal_pdhte_rag_collapse.pdhte_jk_bias_reduced_xi_vs_naive.0,
        triad_syn.multifractal_pdhte_rag_collapse.pdhte_jk_bias_reduced_xi_vs_naive.1,
        triad_syn.multifractal_pdhte_rag_collapse.tail_welsch_vs_ols_ate_rmse.0,
        triad_syn.multifractal_pdhte_rag_collapse.tail_welsch_vs_ols_ate_rmse.1,
        triad_syn.multifractal_pdhte_rag_collapse.bipartite_scaling_accumulate_vs_replace_g8.0,
        triad_syn.weyl_torus_hermite_mehler_dfm.golden_weyl_vs_rational_discrepancy.0,
        triad_syn.weyl_torus_hermite_mehler_dfm.golden_weyl_vs_rational_discrepancy.1,
        triad_syn.weyl_torus_hermite_mehler_dfm.mixgamma_min_transition_bits,
        triad_syn.weyl_torus_hermite_mehler_dfm.dfm_hermite_vs_rff_l2_error.0,
        triad_syn.weyl_torus_hermite_mehler_dfm.dfm_hermite_vs_rff_l2_error.1,
        triad_syn.weyl_torus_hermite_mehler_dfm.mlls_bcts_vs_bbse_kl_divergence.0,
        triad_syn.weyl_torus_hermite_mehler_dfm.mlls_bcts_vs_bbse_kl_divergence.1,
        triad_syn.weyl_torus_hermite_mehler_dfm.classify_and_count_kl_divergence,
        triad_syn.sna_rlhf_fhrr_cohomology_technium.schnakenberg_cycle_entropy_production_pi,
        triad_syn.sna_rlhf_fhrr_cohomology_technium.sna_fractal_exponent_and_negative_lyapunov.1,
        triad_syn.sna_rlhf_fhrr_cohomology_technium.sna_fractal_exponent_and_negative_lyapunov.0,
        triad_syn.sna_rlhf_fhrr_cohomology_technium.sna_orbital_synchronization_error,
        triad_syn.sna_rlhf_fhrr_cohomology_technium.rlhf_delay0_damped_sign_flips,
        triad_syn.sna_rlhf_fhrr_cohomology_technium.rlhf_delay0_vs_delayed_spectral_radius.1,
        triad_syn.sna_rlhf_fhrr_cohomology_technium.izzo_rrm_vs_perfgd_ratio_to_opt.1,
        triad_syn.sna_rlhf_fhrr_cohomology_technium.izzo_rrm_vs_perfgd_ratio_to_opt.0,
        triad_syn.sna_rlhf_fhrr_cohomology_technium.fhrr_phasor_exact_and_bundled_unbinding_cosine.1,
        triad_syn.sna_rlhf_fhrr_cohomology_technium.ltc_fused_solver_max_state_vs_euler.0,
        triad_syn.sna_rlhf_fhrr_cohomology_technium.ltc_fused_solver_max_state_vs_euler.1,
        emp_triad.danish_fire_pdhte_jk_xi_and_plateau_cv.0,
        emp_triad.atlantic_storms_out_of_time_dfm_prior.1,
        emp_triad.atlantic_storms_out_of_time_dfm_prior.2,
        emp_triad.djia_mmar_concavity,
        triad_syn.all_identities_closed && emp_triad.empirical_identities_closed,
        triad_syn
            .discarded_findings
            .iter()
            .chain(&emp_triad.discarded_findings)
            .collect::<Vec<_>>(),
        triad_syn.all_six_bridges_verified && emp_triad.empirical_triadic_verified
    );
}

/// Lab 4: Invariants $\Phi_1$–$\Phi_5$, Moulton-Campbell, Exceedance ICC, and Chouldechova.
fn lab_invariants(root: &Path) {
    println!("=== LAB 4: INVARIANTS PHI1-PHI5, MOULTON-CAMPBELL, EXCEEDANCE, AND CHOULDECHOVA ===");
    let split_tree = MonteCarloSplitTreeAudit::evaluate(DEFAULT_SEED, 16, 1024);
    let b1 = ExceedanceDesignEffect::benchmark_calibration_25028();
    let b2 = ExceedanceDesignEffect::benchmark_pseudoreplication_500_to_86();
    let amp = OperationalWeightingGap::variance_amplification_benchmark_69x();
    println!(
        "  [Phi1 SplitMix64::split(stream_id)] streams={}, max_|rho|={:.4}, Hamming_avalanche={:.1} bits (verified={}) | [Benchmarks Phi2] n={} -> m_bar={:.2}, m_tilde={:.2}, rho_I(0.90)={:.3}, n_eff={:.1} (naive Kish={:.1}) | n={} -> n_eff={:.1} | var_amp={:.1}x",
        split_tree.num_streams,
        split_tree.max_abs_pairwise_correlation,
        split_tree.mean_hamming_avalanche_bits,
        split_tree.split_independence_verified,
        b1.n_nominal,
        b1.m_bar,
        b1.m_tilde_moulton,
        b1.rho_exceedance_p,
        b1.n_effective_p,
        b1.n_effective_naive_kish,
        b2.n_nominal,
        b2.n_effective_p,
        amp
    );

    let clusters = load_bank_call_clusters(root);
    let real_deff = ExceedanceDesignEffect::from_clustered_observations(&clusters, 0.90);
    let profile_q =
        ExceedanceDesignEffect::profile_across_quantiles(&clusters, &[0.50, 0.90, 0.99]);
    println!(
        "  [Real Data: bank_calls.csv] n={}, clusters={}, m_bar={:.2}, m_tilde={:.2}, rho_I(0.90)={:.4}, DEFF={:.2}, n_eff(0.90)={:.1} (profile q50/q90/q99 n_eff=[{:.0}, {:.0}, {:.0}])",
        real_deff.n_nominal,
        real_deff.num_clusters_c,
        real_deff.m_bar,
        real_deff.m_tilde_moulton,
        real_deff.rho_exceedance_p,
        real_deff.deff_exceedance,
        real_deff.n_effective_p,
        profile_q[0].n_effective_p,
        profile_q[1].n_effective_p,
        profile_q[2].n_effective_p
    );

    let (p_a, p_b) = load_toronto_arrests_prevalences(root);
    let (fpr_a, fpr_b, ratio) =
        ChouldechovaAudit::fpr_ratio_under_equal_ppv_and_fnr(p_a, p_b, 0.65, 0.25);
    let kmr_audit =
        ChouldechovaAudit::from_confusion_matrices((180, 90, 630, 100), (70, 55, 825, 50));
    let (_, _, kmr_var_gap) = kmr_audit.kmr_continuous_impossibility_gap();
    println!(
        "  [Phi4: Chouldechova on toronto_arrests.csv] p_A={:.4} vs p_B={:.4} -> equalizing PPV=0.65 & FNR=0.25 forces FPR_A={:.4} vs FPR_B={:.4} (ratio={:.2}x, KMR_var_gap={:.4})",
        p_a, p_b, fpr_a, fpr_b, ratio, kmr_var_gap
    );
}

/// Lab 5: 4-Filter Eliminatory Funnel, 8-Step SIGA Roadmap, and Natural-Frequency Executive Memoranda on the 4 Canonical YAML Cards.
fn lab_siga(root: &Path) {
    println!("=== LAB 5: 4-FILTER ELIMINATORY FUNNEL AND 8-STEP SIGA ROADMAP ===");
    let (shelf_a, shelf_b, shelf_rmse) = fit_shelf_beta_prior(0.02, 0.05, 0.09);
    let grapa_demo = EvidenceProcess::from_grapa_betting(&vec![0.45; 35], 0.05, 1)
        .expect("bounded GRAPA demo payoffs");
    let (epic_sepsis, stoch_triage) =
        AlertFatigueAudit::canonical_epic_sepsis_vs_stochological_gate();
    println!(
        "  [Filter 3 SHELF Beta Fit & Step 6 GRAPA] Beta(alpha={:.2}, beta={:.2}, CDF_RMSE={:.4}) | GRAPA E_35={:.2} (rejects_H0={}) | [Bridge L2->L3 AlertFatigueAudit (10 bit/s & Turing Decibans)] EpicSepsis(PPV={:.1}%, FP/TP={:.1}, approved={}) vs Stochological(PPV={:.1}%, W={:+.2} dB, load={:.1} bit/s <= 10, approved={})",
        shelf_a,
        shelf_b,
        shelf_rmse,
        grapa_demo.final_e_value,
        grapa_demo.rejects_null_hypothesis,
        epic_sepsis.positive_predictive_value_ppv * 100.0,
        epic_sepsis.false_alarms_per_true_case,
        epic_sepsis.approved_bridge_l2_l3,
        stoch_triage.positive_predictive_value_ppv * 100.0,
        stoch_triage.turing_weight_of_evidence_decibans,
        stoch_triage.cognitive_triage_load_bps,
        stoch_triage.approved_bridge_l2_l3
    );
    let cards = load_cards_from_disk(root).expect("load canonical YAML cards from disk");
    for (rel_path, card) in cards {
        let (_audit, rep) = verify_card_against_empirical_data(root, &card)
            .expect("verify card against empirical dataset");
        println!(
            "  [SIGA {}] dataset={}, EVPI={:.1}, p_null={:.4} (unclamped), chi2_SBC={:.2}, E_30={:.1}, steps_1_to_8={:?}, approved={}",
            rel_path,
            card.data_and_audit_contract.dataset_path,
            rep.funnel_result.calculated_evpi,
            rep.null_p_value,
            rep.funnel_result.sbc_chi2_statistic,
            rep.e_process.final_e_value,
            rep.steps_1_to_8_completed,
            rep.approved_end_to_end
        );
        for line in rep.natural_frequency_memo.formatted_summary.lines() {
            println!("    {line}");
        }
    }
}

/// Lab 6: Production Governance: Bitemporal Clock, Single-Opening Vault, and 4 Regimes of Drift.
fn lab_governance(root: &Path) {
    println!("=== LAB 6: PRODUCTION GOVERNANCE: BITEMPORAL CLOCK, VAULT, AND 4 DRIFT REGIMES ===");
    let records = [
        BitemporalRecord {
            row_id: 1,
            t_event: 10,
            t_availability: 12,
            feature_x: 0,
            outcome_y: 0,
            canary_column: 0,
            t_availability_canary: 999,
        },
        BitemporalRecord {
            row_id: 2,
            t_event: 11,
            t_availability: 14,
            feature_x: 1,
            outcome_y: 1,
            canary_column: 1,
            t_availability_canary: 999,
        },
    ];
    let on = query_with_bitemporal_clock(&records, 50, true);
    let off = query_with_bitemporal_clock(&records, 50, false);
    println!(
        "  [Bitemporal Clock + Canary] enabled: MI_canary={:.4}, alarm={} | disabled (mutation): MI_canary={:.4}, alarm={}",
        on.canary_mutual_information_nats,
        on.temporal_intrusion_alarm,
        off.canary_mutual_information_nats,
        off.temporal_intrusion_alarm
    );

    let mut vault = TemporalVault::seal("vault_2026_oot", vec![0.0, 1.0, 1.0, 0.0]);
    let v_rep = vault
        .open_once(&[0.05, 0.95, 0.90, 0.05], 0.02)
        .expect("single opening");
    let hmac_tag = hmac_sha256_hex(b"stochology_auditor_key_2026", v_rep.sha256_seal.as_bytes());
    println!(
        "  [Single-Opening Temporal Vault] sha256_seal={}..{}, HMAC_tag={}..{}, Brier={:.4}, state={:?}",
        &v_rep.sha256_seal[..8],
        &v_rep.sha256_seal[56..],
        &hmac_tag[..8],
        &hmac_tag[56..],
        v_rep.evaluated_brier_score,
        v_rep.final_state
    );

    let (x13, y13, x23, _y23) = load_nyc_flights_2013_2023(root);
    let p_base_13 = mean(&y13).clamp(0.01, 0.99);
    let logit_base_13 = (p_base_13 / (1.0 - p_base_13)).ln();
    let scores_13: Vec<f64> = x13
        .iter()
        .zip(y13.iter())
        .map(|(&x, &y)| {
            let p_hat = 1.0 / (1.0 + (-(logit_base_13 + 0.15 * x)).exp());
            (y - p_hat).abs()
        })
        .collect();
    let cov = fit_covariate_drift_extra_wcp(&x13, &x23, &scores_13, None, 0.02, 0.10);
    let cal_aci: Vec<f64> = x13.iter().map(|&x| 0.10 + 0.35 * x.abs()).collect();
    let test_aci: Vec<f64> = x23
        .iter()
        .enumerate()
        .map(|(i, &x)| 0.18 + 0.35 * x.abs() + if i % 5 == 0 { 0.12 } else { 0.0 })
        .collect();
    let aci = AdaptiveConformalInference::evaluate_stream(&cal_aci, &test_aci, 0.10, 0.02);
    println!(
        "  [Covariate Drift: NYC Flights 2013->2023] beta_tilt={:+.4}, Z_P={:.4}, ExTRA_WCP_T_threshold={:.4}, Lei_Candes_bound >= {:.3} | Online_ACI_cov={:.3} (vs static={:.3})",
        cov.estimated_beta,
        cov.normalizing_constant_z_p,
        cov.weighted_conformal_threshold,
        cov.lei_candes_coverage_lower_bound,
        aci.empirical_coverage_aci,
        aci.empirical_coverage_static
    );

    // Fit a real 1D logistic classifier on 2013 source cascade data and evaluate on 2023 target data
    let (cx13, cy13, cx23, _cy23) = load_nyc_flights_cascade_2013_2023(root);
    let (p_source_13, tpr_s, fpr_s, mu_0_s, mu_1_s, post_target) =
        fit_calibrated_logistic_1d(&cx13, &cy13, &cx23);
    let ls = estimate_label_shift_four_methods(
        p_source_13,
        tpr_s,
        fpr_s,
        &post_target,
        mu_0_s,
        mu_1_s,
        mean(&cx23),
        0.02,
    );
    let cb = MurphyCorpCircuitBreaker::monitor(
        &[0.9, 0.85, 0.15, 0.1],
        &[0.0, 0.0, 1.0, 1.0],
        0.05,
        0.05,
        false,
        false,
        false,
    );
    println!(
        "  [Label Shift 4 Estimators (fit on 2013, TPR={:.3}, FPR={:.3})] BBSE={:.4} (sigma_min={:.3}), MLLS/SLD={:.4}, DFM={:.4} (Delta_min={:.1}), OTTER={:.4} (kappa={:.2}) | Concept Drift Breaker={:?} (tripped={})",
        tpr_s,
        fpr_s,
        ls.target_prior_bbse,
        ls.confusion_matrix_sigma_min,
        ls.target_prior_mlls_sld,
        ls.target_prior_dfm,
        ls.gram_eigenvalue_delta_min,
        ls.target_prior_otter,
        ls.hoffman_constant_kappa,
        cb.diagnosed_regime,
        cb.circuit_breaker_tripped
    );
    let rtpl_gov: Vec<f64> = (0..100).map(|i| (i as f64) * 0.5).collect();
    let hpl_gov: Vec<f64> = (0..100).map(|i| (i as f64) * 0.5 + 0.2).collect();
    let pccp = PredeterminedChangeControlPlan::evaluate_update(
        "PCCP-LAB6-2026",
        0.25,
        0.08,
        1.4,
        0.05,
        &rtpl_gov,
        &hpl_gov,
        0.004,
        0.035,
        0.05,
        true,
    );
    println!(
        "  [PCCP & PLAT Attribution (Basel III MAR33 / VSM Algedonic)] Spearman_rs={:.4}, D_KS={:.4}, Zone={:?}, Competence={}, Rollback={}, Approved={}",
        pccp.plat_result.spearman_rank_correlation,
        pccp.plat_result.kolmogorov_smirnov_distance,
        pccp.plat_result.traffic_light_zone,
        pccp.competence_criterion.competence_verified,
        pccp.automatic_rollback_triggered,
        pccp.update_approved
    );
}

/// Lab 7: Transversal Real-Data Synthesis Across All 13 Empirical Datasets (`data/*.csv`) and 4 Canonical Cards (`cards/*.yaml`).
fn lab_real_data(root: &Path) {
    println!("=== LAB 7: TRANSVERSAL REAL-DATA SYNTHESIS (13 EMPIRICAL DATASETS & 4 CANONICAL CARDS) ===");

    // 1. Public Policy, Digital Public Infrastructure & Observational Selection (nsw_experimental.csv + cps_observational.csv -> card_04)
    let dpi_card04 = PublicInfrastructureCard04Audit::evaluate_from_disk(root);
    let (_nsw_t, _nsw_c, cps_c) = load_lalonde_nsw_cps(root);
    println!(
        "  [1/6 Policy & DPI: nsw_experimental.csv + cps_observational.csv -> card_04] Bridge L1->L2 & Phi3/Phi4: rho_R_Y={:+.4}, n={} -> n_eff_Meng={:.2}, Manski=[{:.1}, {:.1}], LeeBounds(q={:.3})=[{:.1}, {:.1}], p*={:.2}, Chouldechova_FPR={:.2}x (verified={})",
        dpi_card04.meng_audit.rho_r_y,
        cps_c.len(),
        dpi_card04.meng_audit.n_eff_meng,
        dpi_card04.manski_bounds.lower_bound,
        dpi_card04.manski_bounds.upper_bound,
        dpi_card04.lee_trimming_bounds.2,
        dpi_card04.lee_trimming_bounds.0,
        dpi_card04.lee_trimming_bounds.1,
        dpi_card04.cost_loss_p_star,
        dpi_card04.chouldechova_fpr_ratio,
        dpi_card04.verified
    );

    // 2. Banking Operations & Queue Physics (bank_calls.csv + card_02_bank_queue_staffing.yaml)
    let clusters = load_bank_call_clusters(root);
    let daily_totals: Vec<f64> = clusters.iter().map(|c| c.iter().sum::<f64>()).collect();
    let split_idx = daily_totals.len() * 4 / 5;
    let jensen = NonlinearSummary::evaluate_out_of_sample(
        &daily_totals[..split_idx],
        &daily_totals[split_idx..],
        |x| x * x,
        2.0,
    );
    let deff = ExceedanceDesignEffect::from_clustered_observations(&clusters, 0.90);
    println!(
        "  [2/6 Banking: bank_calls.csv -> card_02] Bridge L2->L3 & Phi2/Phi5: n={} (days={}), Delta_J={:+.2e}, OOS_CRPS={:.2e}, DEFF(0.90)={:.2} -> n_eff={:.1}",
        deff.n_nominal,
        deff.num_clusters_c,
        jensen.jensen_gap,
        jensen.empirical_crps,
        deff.deff_exceedance,
        deff.n_effective_p
    );

    // 3. Long-Horizon Financial Non-Ergodicity & Performativity (djia_1885_2023.csv)
    let djia = load_djia_returns(root);
    let mu_ann = mean(&djia) * 252.0;
    let sig_ann = sample_std(&djia) * (252.0_f64).sqrt();
    let kelly = TimeKellyDiagnostic::compute(mu_ann, sig_ann, 0.54, 0.25, 0.20);
    let perf = PerformativeSystem::new(1.0, 0.2, 0.5, 1.0, 1.2).diagnose();
    println!(
        "  [3/6 Finance: djia_1885_2023.csv] Lens 2 & Bridge L3->L1: days={}, g_ens={:.4} -> g_time={:.4} (drain={:.4}), Kelly f*={:.4}, theta_PS={:.4} vs theta_PO={:.4} (r={:.2})",
        djia.len(),
        kelly.ensemble_rate_mu,
        kelly.time_rate_ito,
        kelly.variance_drain,
        kelly.asymmetric_kelly_fraction,
        perf.theta_ps,
        perf.theta_po,
        perf.rrm_contraction_radius
    );

    // 4. Heavy Tails & Infinite Variance in Reinsurance and Energy (danish_fire.csv + vic_elec.csv)
    let fire = load_danish_fire(root);
    let hill_fire = HillTailDiagnostic::audit(&fire, Some(100));
    let vic = load_vic_elec_demand(root);
    let hill_vic = HillTailDiagnostic::audit(&vic, Some(100));
    let vic_wave = EmpiricalWaveAnalogAudit::evaluate_series(&vic);
    println!(
        "  [4/6 Tails & Wave-Analog: danish_fire.csv + vic_elec.csv] Lens 3 & Analog: DanishFire(n={}, alpha={:.4}, {:?}, q99={:.2}M, tail_n_eff={:.1}) | VicElec(n={}, alpha={:.4}, q99={:.1} MW, wave_compr={:.0}x, R2={:.2}, PKU_HP_INV={:.2e})",
        hill_fire.sample_size_n,
        hill_fire.alpha_hill,
        hill_fire.regime,
        hill_fire.quantile_99,
        hill_fire.penalized_tail_n_eff,
        hill_vic.sample_size_n,
        hill_vic.alpha_hill,
        hill_vic.quantile_99,
        vic_wave.empirical_compression_ratio,
        vic_wave.wave_explained_variance_r2,
        vic_wave.empirical_hp_inv_relative_error
    );

    // 5. Isotonic Calibration & Algebraic Fairness (atlantic_storms.csv + toronto_arrests.csv + card_01_atlantic_storms_evacuation.yaml)
    let (storm_p, storm_y) = load_atlantic_storms_forecasts(root);
    let corp = CorpPavDecomposition::compute(&storm_p, &storm_y);
    let (p_a, p_b) = load_toronto_arrests_prevalences(root);
    let (_, _, fpr_ratio) =
        ChouldechovaAudit::fpr_ratio_under_equal_ppv_and_fnr(p_a, p_b, 0.65, 0.25);
    let storm_envs_real = load_atlantic_storms_decade_environments(root);
    let emp_crit_real = EmpiricalCriticalRationalityAudit::evaluate_real_datasets(
        &fire,
        (p_a, p_b),
        &storm_envs_real,
        &vic,
    );
    let emp_stoch_real = EmpiricalStocharyCyberneticAudit::evaluate_real_datasets(
        &djia,
        &clusters,
        &fire,
        &vic,
        (&storm_p, &storm_y),
    );
    let emp_triad_real = EmpiricalTriadicTechniumAudit::evaluate_real_datasets(
        &vic,
        &fire,
        (&storm_p, &storm_y),
        &djia,
        &clusters,
    );
    println!(
        "  [5/6 Calibration, Fairness, Critical Theory, Stochary & Triadic Technium: atlantic_storms.csv + toronto_arrests.csv -> card_01] Lens 5, Phi4, Critical Theory, Stochary & Triadic: Storms BS={:.5} (REL={:.5}, RES={:.5}, err={:.2e}, NILE_ratio={:.1}x, Telos={:+.4}, HM_DFM={:.4}) | Toronto FPR_A/FPR_B={:.2}x | DanishFire Non-Identical Erased={:.1}%, CVaR95={:.2}M (chronological CVaR gap={:+.2}, WDRO={:+.2}, PDHTE_xi={:.3}) | verified={}",
        corp.brier_total,
        corp.rel_miscalibration,
        corp.res_resolution,
        corp.identity_error,
        emp_crit_real.atlantic_storms_spurious_to_invariant_penalty_ratio,
        emp_stoch_real.atlantic_storms_telos_skill_vs_skopos_unc0.0,
        emp_triad_real.atlantic_storms_out_of_time_dfm_prior.1,
        fpr_ratio,
        emp_crit_real.danish_fire_non_identical_erasure_fraction * 100.0,
        emp_stoch_real.danish_fire_var95_and_cvar95.1,
        emp_stoch_real.danish_fire_naive_vs_wdro_disappointment.0,
        emp_stoch_real.danish_fire_naive_vs_wdro_disappointment.1,
        emp_triad_real.danish_fire_pdhte_jk_xi_and_plateau_cv.0,
        emp_crit_real.empirical_critical_verified
            && emp_stoch_real.empirical_stochary_verified
            && emp_triad_real.empirical_triadic_verified
    );

    // 6. Multi-Horizon & Out-of-Domain Aviation Drift (nyc_flights_2013/2023 + H1 reserves + houston_flights_2011.csv -> card_03)
    let (x13, y13, x23, _y23) = load_nyc_flights_2013_2023(root);
    let (x13_h1, y13_h1, x23_h1, y23_h1, x_hou, _y_hou) = load_oot_ood_flight_reserves(root);
    let p_base_13 = mean(&y13).clamp(0.01, 0.99);
    let logit_base_13 = (p_base_13 / (1.0 - p_base_13)).ln();
    let scores_13: Vec<f64> = x13
        .iter()
        .zip(y13.iter())
        .map(|(&x, &y)| {
            let p_hat = 1.0 / (1.0 + (-(logit_base_13 + 0.15 * x)).exp());
            (y - p_hat).abs()
        })
        .collect();
    let p_base_13_h1 = mean(&y13_h1).clamp(0.01, 0.99);
    let logit_base_13_h1 = (p_base_13_h1 / (1.0 - p_base_13_h1)).ln();
    let scores_13_h1: Vec<f64> = x13_h1
        .iter()
        .zip(y13_h1.iter())
        .map(|(&x, &y)| {
            let p_hat = 1.0 / (1.0 + (-(logit_base_13_h1 + 0.15 * x)).exp());
            (y - p_hat).abs()
        })
        .collect();
    let cov_nyc = fit_covariate_drift_extra_wcp(&x13, &x23, &scores_13, None, 0.02, 0.10);
    let cov_h1 = fit_covariate_drift_extra_wcp(&x13_h1, &x23_h1, &scores_13_h1, None, 0.02, 0.10);
    let cov_hou = fit_covariate_drift_extra_wcp(&x13, &x_hou, &scores_13, None, 0.02, 0.10);

    // Strictly out-of-time forecast calibrated on 2013 H1 source parameters without touching y23_h1
    let probs_h1: Vec<f64> = x23_h1
        .iter()
        .map(|&x| {
            let p_model = 1.0 / (1.0 + (-(logit_base_13_h1 + 0.15 * x)).exp());
            apply_cromwell_floor(p_model, 0.001)
        })
        .collect();
    let mut oot_vault = TemporalVault::seal("nyc_flights_2023_h1_oot", y23_h1);
    let vault_rep = oot_vault
        .open_once(&probs_h1, 0.20)
        .expect("single opening of H1 reserve vault");
    println!(
        "  [6/6 Aviation Drift (5 CSVs): nyc_flights_2013/2023 + H1 reserves + houston_flights_2011.csv -> card_03] Lei-Candes(NYC 13->23 >= {:.3}, H1 13->23 >= {:.3}, NYC->Houston OOD >= {:.3}) | H1 Vault={:?} (Brier={:.4})",
        cov_nyc.lei_candes_coverage_lower_bound,
        cov_h1.lei_candes_coverage_lower_bound,
        cov_hou.lei_candes_coverage_lower_bound,
        vault_rep.final_state,
        vault_rep.evaluated_brier_score
    );

    // 7. End-to-End Epistemological Card Ledger (cards/card_01..03.yaml)
    let cards = load_cards_from_disk(root).expect("load canonical YAML cards from disk");
    let mut cards_approved = 0usize;
    for (_, card) in &cards {
        if let Ok((_, rep)) = verify_card_against_empirical_data(root, card) {
            if rep.approved_end_to_end {
                cards_approved += 1;
            }
        }
    }
    println!(
        "  [Synthesis Ledger] 13/13 real CSV datasets audited | {}/{} canonical YAML cards approved (G1-G8 & SIGA)",
        cards_approved,
        cards.len()
    );
}

fn print_usage() {
    println!(
        "Usage: labs [--help | --all | --layers | --uncertainty | --lenses | --invariants | --siga | --governance | --real-data]"
    );
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() > 1 {
        eprintln!("Too many arguments: {args:?}");
        print_usage();
        std::process::exit(2);
    }
    let mode = args.first().map(|s| s.as_str()).unwrap_or("--all");
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .expect("workspace root");

    match mode {
        "--help" | "-h" => print_usage(),
        "--layers" => lab_layers(root),
        "--uncertainty" => lab_uncertainty(),
        "--lenses" => lab_lenses(root),
        "--invariants" => lab_invariants(root),
        "--siga" => lab_siga(root),
        "--governance" => lab_governance(root),
        "--real-data" => lab_real_data(root),
        "--all" => {
            lab_layers(root);
            lab_uncertainty();
            lab_lenses(root);
            lab_invariants(root);
            lab_siga(root);
            lab_governance(root);
            lab_real_data(root);
        }
        other => {
            eprintln!("Unknown option: '{other}'.");
            print_usage();
            std::process::exit(2);
        }
    }
}
