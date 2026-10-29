//! Lab 3: The Five Transversal Lenses and Five Syntheses on Real Datasets.

use std::path::Path;
use stochology_cli::{
    load_atlantic_storms_decade_environments, load_atlantic_storms_forecasts,
    load_bank_call_clusters, load_cps_cohort_clusters, load_danish_fire, load_djia_returns,
    load_lalonde_nsw_cps, load_oot_ood_flight_reserves, load_toronto_arrests_prevalences,
    load_vic_elec_demand,
};
use stochology_core::{DEFAULT_SEED, OnlineMoments, Rng, apply_cromwell_floor, mean, sample_std};
use stochology_lenses::{
    AlmostPerfectMachine, AnalogWaveComputingSynthesis, AuthenticModelForm, CausalEnvironment,
    ComplexFractalToroidalTechniumSynthesis, CorpPavDecomposition, CostLossRule,
    CoupledPublicPrivateSocietySynthesis, CriticalStochasticRationalitySynthesis, DpiMarkovChain,
    EmpiricalCoupledSocietyAudit, EmpiricalCriticalRationalityAudit,
    EmpiricalStocharyCyberneticAudit, EmpiricalStrategicAccommodationAudit,
    EmpiricalTriadicTechniumAudit, EmpiricalWaveAnalogAudit, ExactCoinCalibrationBenchmark,
    HillTailDiagnostic, MixingTimeAudit, RelationalComplexSystemsSynthesis, SdpiHierarchy,
    StocharyConjecturalCyberneticSynthesis, StrategicAiAccommodationSynthesis, TimeKellyDiagnostic,
    amplitude_constrained_mi_bound, blahut_arimoto_capacity, chow_invariance_f_stat,
    cinelli_hazlett_robustness_value, dedh_moment_tail_index, empirical_kelly_optimal_fraction,
    expected_shortfall_99, fano_error_floor, gpd_pot_fit, hidden_confounding_threshold_lambda_star,
    logarithmic_score, run_icp, run_nonlinear_icp_nile,
};

/// Lab 3: The Five Transversal Lenses on Real Datasets.
pub fn lab_lenses(root: &Path) {
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

    // 7. Relational Ontology & Complex Systems
    let rel_syn = RelationalComplexSystemsSynthesis::evaluate_canonical(DEFAULT_SEED);
    println!(
        "  [Relational Ontology & Complex Systems] MI_rel={:.4} vs MI_iso={:.4} nats | Prigogine dS={:+.2} (turnover={:.1}%, closure={:.1}%) | Schelling(tau=0.125->{:.1}% vs tau=0.375->{:.1}%) | Daisyworld(MAE={:.2}C vs dead={:.2}C, dT/dL={:+.2}) | verified={}",
        rel_syn
            .relational_figuration
            .relational_mutual_information_nats,
        rel_syn
            .relational_figuration
            .isolated_svabhava_mutual_information_nats,
        rel_syn.dissipative_autopoiesis.net_open_entropy_change_ds,
        rel_syn.dissipative_autopoiesis.material_turnover_fraction * 100.0,
        rel_syn.dissipative_autopoiesis.autopoietic_pattern_fidelity * 100.0,
        rel_syn.schelling_emergence.integrated_similarity_at_low_tau * 100.0,
        rel_syn.schelling_emergence.emergent_segregation_at_mild_tau * 100.0,
        rel_syn.daisyworld_homeostasis.homeostatic_mean_abs_error_c,
        rel_syn.daisyworld_homeostasis.lifeless_mean_abs_error_c,
        rel_syn
            .daisyworld_homeostasis
            .coexistence_feedback_slope_dt_dl,
        rel_syn.all_six_axes_verified
    );

    // 8. Physical, Analog & Wave-Interference Computing
    let analog_syn = AnalogWaveComputingSynthesis::evaluate_canonical(DEFAULT_SEED);
    let emp_wave = EmpiricalWaveAnalogAudit::evaluate_series(&vic_demand);
    println!(
        "  [Analog & Wave Computing] DRAM/FP32={:.0}x, collapse_var={:.2}->anchored={:.2} | RLC_iso_err={:.1e} | AIMC_sigma0.6(std={:.1}% vs NAT={:.1}%), PKU_HP_INV_err={:.2e} ({} iters, BlockAMC_err={:.2e}/{}ops, HaLoRA_drift_red={:.1}%) | SIREN_compr={:.0}x (Lap_MSE={:.1e} vs ReLU={:.1}), WaveRNN_sel={:.1}% | BrainDPI_compr={:.1e}x (ret={:.1}%) | VicElec_wave(R2={:.2}, HP_INV={:.2e}) | verified={}",
        analog_syn.von_neumann_and_collapse.dram_to_fp32_add_ratio,
        analog_syn
            .von_neumann_and_collapse
            .unanchored_synthetic_variance_retention,
        analog_syn
            .von_neumann_and_collapse
            .neurosymbolic_anchored_variance_retention,
        analog_syn.ode_isomorphism.max_isomorphism_discrepancy,
        analog_syn
            .aimc_analog_wall_hp_inv
            .standard_accuracy_at_sigma_060
            * 100.0,
        analog_syn.aimc_analog_wall_hp_inv.nat_accuracy_at_sigma_060 * 100.0,
        analog_syn
            .aimc_analog_wall_hp_inv
            .pku_hp_inv_final_relative_error,
        analog_syn
            .aimc_analog_wall_hp_inv
            .pku_hp_inv_iterations_to_fp32,
        analog_syn
            .aimc_analog_wall_hp_inv
            .pku_block_amc_relative_error,
        analog_syn
            .aimc_analog_wall_hp_inv
            .pku_block_amc_operations_count,
        analog_syn
            .aimc_analog_wall_hp_inv
            .abnat_halora_attention_drift_reduction
            * 100.0,
        analog_syn
            .wave_interference
            .phase_amplitude_compression_ratio,
        analog_syn.wave_interference.siren_helmholtz_laplacian_mse,
        analog_syn.wave_interference.relu_helmholtz_laplacian_mse,
        analog_syn
            .wave_interference
            .wave_rnn_target_probe_selectivity
            * 100.0,
        analog_syn
            .sensory_dpi_and_ecosystem
            .total_sensory_compression_ratio,
        analog_syn
            .sensory_dpi_and_ecosystem
            .actionable_information_retention_ratio
            * 100.0,
        emp_wave.wave_explained_variance_r2,
        emp_wave.empirical_hp_inv_relative_error,
        analog_syn.all_five_axes_verified && emp_wave.empirical_wave_verified
    );

    // 9. Critical Theory of Stochastic Rationality
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
        crit_syn
            .adorno_non_identical_and_reification
            .non_identical_liquidation_gap,
        crit_syn
            .adorno_non_identical_and_reification
            .es99_erasure_fraction
            * 100.0,
        crit_syn
            .adorno_non_identical_and_reification
            .var99_holdout_exceedance_empirical_vs_gaussian
            .0
            * 100.0,
        crit_syn
            .adorno_non_identical_and_reification
            .var99_holdout_exceedance_empirical_vs_gaussian
            .1
            * 100.0,
        crit_syn
            .adorno_non_identical_and_reification
            .deployed_koh_extrapolation_error,
        crit_syn
            .adorno_non_identical_and_reification
            .status_quo_reification_excess_risk,
        crit_syn
            .marcuse_rouvroy_stochastic_parrot
            .parrot_counterfactual_intervention_mse,
        crit_syn
            .marcuse_rouvroy_stochastic_parrot
            .causal_grounded_counterfactual_mse,
        crit_syn
            .marcuse_rouvroy_stochastic_parrot
            .spurious_proxy_invariance_penalty_ratio,
        crit_syn
            .marcuse_rouvroy_stochastic_parrot
            .preemptive_profiling_fpr_disparity_ratio,
        crit_syn
            .material_collapse_triad
            .synthetic_autophagy_variance_retention,
        crit_syn
            .material_collapse_triad
            .synthetic_autophagy_kl_divergence_nats,
        crit_syn
            .material_collapse_triad
            .human_token_exhaustion_window_years
            .0,
        crit_syn
            .material_collapse_triad
            .human_token_exhaustion_window_years
            .1,
        crit_syn
            .material_collapse_triad
            .ferran_alia_threshold_cliff_curve[3]
            * 100.0,
        crit_syn
            .dialectical_overcoming
            .neurosymbolic_rule_satisfaction_rate
            * 100.0,
        crit_syn
            .dialectical_overcoming
            .unanchored_statistical_rule_satisfaction_rate
            * 100.0,
        crit_syn
            .dialectical_overcoming
            .a_ecc_compensated_vs_raw_accuracy
            .0
            * 100.0,
        crit_syn
            .dialectical_overcoming
            .a_ecc_compensated_vs_raw_accuracy
            .1
            * 100.0,
        crit_syn
            .dialectical_overcoming
            .pku_40nm_rram_24bit_hp_inv_error,
        crit_syn
            .dialectical_overcoming
            .siren_vs_relu_helmholtz_laplacian_mse
            .0,
        crit_syn
            .dialectical_overcoming
            .siren_vs_relu_helmholtz_laplacian_mse
            .1,
        crit_syn
            .dialectical_overcoming
            .nat_vs_standard_accuracy_at_sigma_060
            .0
            * 100.0,
        crit_syn
            .dialectical_overcoming
            .tsinghua_accel_photonic_tops_per_watt,
        emp_crit.danish_fire_non_identical_erasure_fraction * 100.0,
        emp_crit.toronto_arrests_preemptive_fpr_ratio,
        emp_crit.atlantic_storms_spurious_to_invariant_penalty_ratio,
        crit_syn.all_identities_closed,
        crit_syn.all_four_axes_verified && emp_crit.empirical_critical_verified
    );

    // 10. Stochary-Conjectural & Cybernetic Synthesis
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
        "  [Stochary-Conjectural & Cybernetics] Tree150(H1={:.4}, J1={:.4}, H2={:.4}, J2={:.4}, orphans={}) | Axioms(LowPass={:.2} vs Bullwhip={:.2}x, Peters_g={:+.4} vs Kelly_sim={:+.4}, TailVega={:+.1} vs JIT={:+.1}, rho1={:.2}) | Powell(theta*={:.2}, CFA_gain={:+.1}), VOI(ambiguous probe={:.3}<act={:.3}, resolved probe={:.3}>act={:.3}, match={}), Pearl(FD={:+.2}, transport_err={:.3}) | Scarf(s*={:.0}, S*={:.0}), TTS/TTR={:.2} (REI_red={:.0}%), CVaR95={:.1}, WDRO_disapp={:+.2} vs naive={:+.2} | SupplyChain(GG1_Wq={:.2}h vs Kingman={:.2}h, cliff_sim={:.1}h, Sterman_SL_damp={:.2}, GravesWillems_saving={:.0}%, Eppen_tail_surge={:.2}x, TBS_saving={:.0}%, Ivanov_fill={:.0}%) | VSM(ConantAshby_R2={:.3}, Noise={:.3}->{:.3}, PLAT r_s={:.2}/D_KS={:.3}, Algedonic={}, PATE_Q3_alarm={}) | Emp5CSVs(BankLP={:.2}, FireWDRO={:+.1}, VicReserveCov={:.3}/{:.3}, discarded={:?}) | identities={} | verified={}",
        stoch_syn.topology_and_stock_axioms.shannon_entropy_l1_bits,
        stoch_syn.topology_and_stock_axioms.balance_index_j_l1,
        stoch_syn.topology_and_stock_axioms.shannon_entropy_l2_bits,
        stoch_syn.topology_and_stock_axioms.balance_index_j_l2,
        stoch_syn
            .topology_and_stock_axioms
            .orphan_single_leaf_subbranches,
        stoch_syn
            .topology_and_stock_axioms
            .low_pass_buffered_variance_ratio,
        stoch_syn
            .topology_and_stock_axioms
            .unbuffered_bullwhip_amplification_ratio,
        stoch_syn.topology_and_stock_axioms.peters_time_growth_rate,
        stoch_syn
            .topology_and_stock_axioms
            .ergodic_buffered_time_growth_rate,
        stoch_syn
            .topology_and_stock_axioms
            .buffered_convex_tail_vega,
        stoch_syn
            .topology_and_stock_axioms
            .unbuffered_jit_concave_tail_vega,
        stoch_syn
            .topology_and_stock_axioms
            .stock_statefulness_autocorrelation,
        stoch_syn
            .powell_friston_causal_engine
            .powell_cfa_tuned_theta,
        stoch_syn
            .powell_friston_causal_engine
            .powell_cfa_tuned_vs_myopic_gain,
        stoch_syn
            .powell_friston_causal_engine
            .voi_probe_vs_act_now_loss_ambiguous
            .0,
        stoch_syn
            .powell_friston_causal_engine
            .voi_probe_vs_act_now_loss_ambiguous
            .1,
        stoch_syn
            .powell_friston_causal_engine
            .voi_probe_vs_act_now_loss_resolved
            .0,
        stoch_syn
            .powell_friston_causal_engine
            .voi_probe_vs_act_now_loss_resolved
            .1,
        stoch_syn
            .powell_friston_causal_engine
            .deployed_policy_choices_match_voi,
        stoch_syn
            .powell_friston_causal_engine
            .pearl_front_door_identified_effect,
        stoch_syn
            .powell_friston_causal_engine
            .pearl_transportability_error,
        stoch_syn.strategic_stock_plant.scarf_optimal_s_and_big_s.0,
        stoch_syn.strategic_stock_plant.scarf_optimal_s_and_big_s.1,
        stoch_syn.strategic_stock_plant.simchi_levi_tts_to_ttr_ratio,
        stoch_syn
            .strategic_stock_plant
            .simchi_levi_rei_reduction_fraction
            * 100.0,
        stoch_syn
            .strategic_stock_plant
            .rockafellar_uryasev_var95_and_cvar95
            .1,
        stoch_syn
            .strategic_stock_plant
            .wdro_robust_oos_disappointment,
        stoch_syn
            .strategic_stock_plant
            .wdro_naive_oos_disappointment,
        stoch_syn
            .strategic_stock_plant
            .supply_chain_physics
            .kingman_vut_buffered_vs_cliff_wait_hours
            .0,
        stoch_syn
            .strategic_stock_plant
            .supply_chain_physics
            .kingman_formula_wait_hours
            .0,
        stoch_syn
            .strategic_stock_plant
            .supply_chain_physics
            .kingman_vut_buffered_vs_cliff_wait_hours
            .1,
        stoch_syn
            .strategic_stock_plant
            .supply_chain_physics
            .sterman_supply_line_oscillation_damping_ratio,
        stoch_syn
            .strategic_stock_plant
            .supply_chain_physics
            .clark_scarf_graves_willems_meio_saving_fraction
            * 100.0,
        stoch_syn
            .strategic_stock_plant
            .supply_chain_physics
            .eppen_risk_pooling_normal_vs_tail_multiplier
            .1,
        stoch_syn
            .strategic_stock_plant
            .supply_chain_physics
            .allon_van_mieghem_tbs_cost_saving_fraction
            * 100.0,
        stoch_syn
            .strategic_stock_plant
            .supply_chain_physics
            .ivanov_ripple_effect_viability_service_rate
            * 100.0,
        stoch_syn
            .cybernetic_vsm_and_pate
            .conant_ashby_regulator_homomorphism_r2,
        stoch_syn
            .cybernetic_vsm_and_pate
            .kahneman_judgment_noise_before_and_after
            .0,
        stoch_syn
            .cybernetic_vsm_and_pate
            .kahneman_judgment_noise_before_and_after
            .1,
        stoch_syn.cybernetic_vsm_and_pate.plat_spearman_and_ks.0,
        stoch_syn.cybernetic_vsm_and_pate.plat_spearman_and_ks.1,
        stoch_syn
            .cybernetic_vsm_and_pate
            .algedonic_not_measured_not_executed_enforced,
        stoch_syn
            .cybernetic_vsm_and_pate
            .pate_q3_jackpot_algedonic_alarm_triggered,
        emp_stoch.bank_calls_low_pass_variance_ratio,
        emp_stoch.danish_fire_naive_vs_wdro_disappointment.1,
        emp_stoch.vic_elec_reserve_excursion_coverage.0,
        emp_stoch.vic_elec_reserve_excursion_coverage.1,
        emp_stoch.discarded_findings,
        stoch_syn.all_identities_closed,
        stoch_syn.all_four_axes_verified && emp_stoch.empirical_stochary_verified
    );

    // 11. Triadic Mathematical & Physical Synthesis
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
        triad_syn
            .complex_wave_gabor_helmholtz
            .wire_complex_gabor_helmholtz_residual_mse,
        triad_syn
            .complex_wave_gabor_helmholtz
            .tanh_mlp_helmholtz_residual_mse,
        triad_syn
            .complex_wave_gabor_helmholtz
            .lssa_learned_frequency_relative_error,
        triad_syn
            .complex_wave_gabor_helmholtz
            .hughes_adjoint_in_situ_gradient_max_error,
        triad_syn
            .complex_real_isomorphism_hmatrix
            .complex_4x4_to_real_8x8_inversion_discrepancy,
        triad_syn
            .complex_real_isomorphism_hmatrix
            .two_stage_block_amc_16x16_relative_error,
        triad_syn
            .complex_real_isomorphism_hmatrix
            .hmatrix_admissible_vs_naive_relative_error
            .0,
        triad_syn
            .complex_real_isomorphism_hmatrix
            .hmatrix_admissible_vs_naive_relative_error
            .1,
        triad_syn
            .fractional_ctrw_analog_ecc
            .atomic_conductance_quantum_g0_microsiemens,
        triad_syn
            .fractional_ctrw_analog_ecc
            .pcm_uncompensated_vs_compensated_drift_mse
            .1,
        triad_syn
            .fractional_ctrw_analog_ecc
            .pcm_uncompensated_vs_compensated_drift_mse
            .0,
        triad_syn
            .fractional_ctrw_analog_ecc
            .ctrw_simulated_vs_anomalous_vs_fickian_spread_ratio
            .0,
        triad_syn
            .fractional_ctrw_analog_ecc
            .ctrw_simulated_vs_anomalous_vs_fickian_spread_ratio
            .1,
        triad_syn
            .fractional_ctrw_analog_ecc
            .ctrw_simulated_vs_anomalous_vs_fickian_spread_ratio
            .2,
        triad_syn
            .fractional_ctrw_analog_ecc
            .analog_ecc_trimmed_vs_plain_ls_recovery
            .0
            * 100.0,
        triad_syn
            .fractional_ctrw_analog_ecc
            .analog_ecc_trimmed_vs_plain_ls_recovery
            .1
            * 100.0,
        triad_syn
            .multifractal_pdhte_rag_collapse
            .mmar_multifractal_concavity_curvature,
        triad_syn
            .multifractal_pdhte_rag_collapse
            .mmar_theoretical_curvature,
        triad_syn
            .multifractal_pdhte_rag_collapse
            .pdhte_jk_bias_reduced_xi_vs_naive
            .0,
        triad_syn
            .multifractal_pdhte_rag_collapse
            .pdhte_jk_bias_reduced_xi_vs_naive
            .1,
        triad_syn
            .multifractal_pdhte_rag_collapse
            .tail_welsch_vs_ols_ate_rmse
            .0,
        triad_syn
            .multifractal_pdhte_rag_collapse
            .tail_welsch_vs_ols_ate_rmse
            .1,
        triad_syn
            .multifractal_pdhte_rag_collapse
            .bipartite_scaling_accumulate_vs_replace_g8
            .0,
        triad_syn
            .weyl_torus_hermite_mehler_dfm
            .golden_weyl_vs_rational_discrepancy
            .0,
        triad_syn
            .weyl_torus_hermite_mehler_dfm
            .golden_weyl_vs_rational_discrepancy
            .1,
        triad_syn
            .weyl_torus_hermite_mehler_dfm
            .mixgamma_min_transition_bits,
        triad_syn
            .weyl_torus_hermite_mehler_dfm
            .dfm_hermite_vs_rff_l2_error
            .0,
        triad_syn
            .weyl_torus_hermite_mehler_dfm
            .dfm_hermite_vs_rff_l2_error
            .1,
        triad_syn
            .weyl_torus_hermite_mehler_dfm
            .mlls_bcts_vs_bbse_kl_divergence
            .0,
        triad_syn
            .weyl_torus_hermite_mehler_dfm
            .mlls_bcts_vs_bbse_kl_divergence
            .1,
        triad_syn
            .weyl_torus_hermite_mehler_dfm
            .classify_and_count_kl_divergence,
        triad_syn
            .sna_rlhf_fhrr_cohomology_technium
            .schnakenberg_cycle_entropy_production_pi,
        triad_syn
            .sna_rlhf_fhrr_cohomology_technium
            .sna_fractal_exponent_and_negative_lyapunov
            .1,
        triad_syn
            .sna_rlhf_fhrr_cohomology_technium
            .sna_fractal_exponent_and_negative_lyapunov
            .0,
        triad_syn
            .sna_rlhf_fhrr_cohomology_technium
            .sna_orbital_synchronization_error,
        triad_syn
            .sna_rlhf_fhrr_cohomology_technium
            .rlhf_delay0_damped_sign_flips,
        triad_syn
            .sna_rlhf_fhrr_cohomology_technium
            .rlhf_delay0_vs_delayed_spectral_radius
            .1,
        triad_syn
            .sna_rlhf_fhrr_cohomology_technium
            .izzo_rrm_vs_perfgd_ratio_to_opt
            .1,
        triad_syn
            .sna_rlhf_fhrr_cohomology_technium
            .izzo_rrm_vs_perfgd_ratio_to_opt
            .0,
        triad_syn
            .sna_rlhf_fhrr_cohomology_technium
            .fhrr_phasor_exact_and_bundled_unbinding_cosine
            .1,
        triad_syn
            .sna_rlhf_fhrr_cohomology_technium
            .ltc_fused_solver_max_state_vs_euler
            .0,
        triad_syn
            .sna_rlhf_fhrr_cohomology_technium
            .ltc_fused_solver_max_state_vs_euler
            .1,
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

    // 12. Strategic AI Six-Pair Accommodation Synthesis & Empirical 6-CSV Coupler
    let strat_syn = StrategicAiAccommodationSynthesis::evaluate_canonical(DEFAULT_SEED);
    let (nsw_t_lens, _nsw_c_lens, cps_c_lens) = load_lalonde_nsw_cps(root);
    let (_x13_h1_l, _y13_h1_l, x23_h1_l, y23_h1_l, _, _) = load_oot_ood_flight_reserves(root);
    let probs_h1_l: Vec<f64> = x23_h1_l
        .iter()
        .map(|&x| apply_cromwell_floor(1.0 / (1.0 + (-(-1.35 + 0.15 * x)).exp()), 0.001))
        .collect();
    let emp_strat = EmpiricalStrategicAccommodationAudit::evaluate_real_datasets(
        &storm_envs,
        (&nsw_t_lens, &cps_c_lens),
        &bank_clusters_lens,
        &fire_losses,
        (&probs_h1_l, &y23_h1_l),
        &vic_demand,
    );
    println!(
        "  [Strategic AI Six-Pair Accommodation Synthesis] Axis1_Minsky(Walsh_corr={:.1e}, Conn_acc={:.2} vs {:.2}, SoM_Brier={:.3} vs {:.3}) | Axis2_CausalXAI_MI_DoE(SHAP_do_MSE={:.3} vs {:.2}, Rubin_MI_var_ret={:.2}, Clustered_Power={:.2}, GP_EI_ratio={:.1}) | Axis3_GenAI_RAG_Judge(RAG_MI={:.3} nats, PPI_err={:.3} vs bias={:.3}, FinOps_cost={:.2} vs {:.2}, INT4_ES99_ret={:.2} vs {:.2}) | Axis4_Networks(Fiedler_l2={:.3}, Hub_survival={:.2} vs {:.2}, Centola_adopt={:.2} vs {:.2}) | Axis5_MCDM_AIAct_PKM(AHP_val={:.2} vs {:.2}, ICE_bias={:+.1}, AIAct_LGPD={}, PKM_audit={}) | Axis6_Kalman_MPC(Huber_RMSE={:.3} vs L2={:.2}, Tube_MPC_surv={:.2} vs CE={:.2}) | Emp6CSVs(Storms_SHAP_p0={:.2}, Bank_MDE_ratio={:.2}x, Fire_ES99_prot={:.2} vs clip={:.2}, Vic_Huber_RMSE={:.1} vs L2={:.1}) | identities={} | discarded={:?} | verified={}",
        strat_syn
            .minsky_papert_society_of_mind
            .parity_lower_order_walsh_max_correlation,
        strat_syn
            .minsky_papert_society_of_mind
            .diameter_limited_vs_global_connectivity_accuracy
            .1,
        strat_syn
            .minsky_papert_society_of_mind
            .diameter_limited_vs_global_connectivity_accuracy
            .0,
        strat_syn
            .minsky_papert_society_of_mind
            .homogeneous_vs_society_of_mind_brier
            .1,
        strat_syn
            .minsky_papert_society_of_mind
            .homogeneous_vs_society_of_mind_brier
            .0,
        strat_syn
            .shapley_rubin_mi_doe
            .shapley_audit
            .observational_vs_interventional_do_mse
            .1,
        strat_syn
            .shapley_rubin_mi_doe
            .shapley_audit
            .observational_vs_interventional_do_mse
            .0,
        strat_syn
            .shapley_rubin_mi_doe
            .deployed_imputation_variance_retention,
        strat_syn.shapley_rubin_mi_doe.deployed_clustered_doe_power,
        strat_syn
            .shapley_rubin_mi_doe
            .gp_expected_improvement_exploratory_to_anchored_ratio,
        strat_syn
            .generative_rag_llm_judge_compression
            .deployed_rag_end_to_end_mi_nats,
        strat_syn
            .generative_rag_llm_judge_compression
            .deployed_llm_judge_abs_error,
        strat_syn
            .generative_rag_llm_judge_compression
            .llm_judge_ppi
            .estimated_judge_bias,
        strat_syn
            .generative_rag_llm_judge_compression
            .finops_cascade_vs_naive_local_cost_per_unit
            .0,
        strat_syn
            .generative_rag_llm_judge_compression
            .finops_cascade_vs_naive_local_cost_per_unit
            .1,
        strat_syn
            .generative_rag_llm_judge_compression
            .quantization_outlier_protected_vs_naive_es99_retention
            .0,
        strat_syn
            .generative_rag_llm_judge_compression
            .quantization_outlier_protected_vs_naive_es99_retention
            .1,
        strat_syn
            .complex_network_hub_contagion
            .fiedler_algebraic_connectivity_lambda2,
        strat_syn
            .complex_network_hub_contagion
            .buffered_vs_unbuffered_airspace_survival
            .0,
        strat_syn
            .complex_network_hub_contagion
            .buffered_vs_unbuffered_airspace_survival
            .1,
        strat_syn
            .complex_network_hub_contagion
            .centola_clustered_vs_random_adoption
            .0,
        strat_syn
            .complex_network_hub_contagion
            .centola_clustered_vs_random_adoption
            .1,
        strat_syn
            .multi_criteria_ahp_regulatory_pkm
            .ideal_mode_vs_naive_ahp_realized_value
            .0,
        strat_syn
            .multi_criteria_ahp_regulatory_pkm
            .ideal_mode_vs_naive_ahp_realized_value
            .1,
        strat_syn
            .multi_criteria_ahp_regulatory_pkm
            .ahp_ice_audit
            .ice_product_of_means_overestimation_bias,
        strat_syn
            .multi_criteria_ahp_regulatory_pkm
            .deployed_statutory_compliance_approved,
        strat_syn
            .multi_criteria_ahp_regulatory_pkm
            .deployed_pkm_provenance_clean,
        strat_syn
            .robust_kalman_closed_loop_control
            .kalman_control
            .l2_kalman_vs_huber_kalman_rmse
            .1,
        strat_syn
            .robust_kalman_closed_loop_control
            .kalman_control
            .l2_kalman_vs_huber_kalman_rmse
            .0,
        strat_syn
            .robust_kalman_closed_loop_control
            .kalman_control
            .certainty_equivalent_vs_tube_mpc_survival
            .1,
        strat_syn
            .robust_kalman_closed_loop_control
            .kalman_control
            .certainty_equivalent_vs_tube_mpc_survival
            .0,
        emp_strat.atlantic_storms_icp_shap_pressure_share,
        emp_strat.bank_calls_clustered_mde_inflation_ratio,
        emp_strat
            .danish_fire_outlier_protected_vs_clipped_es99_retention
            .0,
        emp_strat
            .danish_fire_outlier_protected_vs_clipped_es99_retention
            .1,
        emp_strat.vic_elec_huber_vs_l2_kalman_rmse.0,
        emp_strat.vic_elec_huber_vs_l2_kalman_rmse.1,
        strat_syn.all_identities_closed && emp_strat.empirical_identities_closed,
        strat_syn
            .discarded_findings
            .iter()
            .chain(&emp_strat.discarded_findings)
            .collect::<Vec<_>>(),
        strat_syn.all_six_axes_verified && emp_strat.empirical_strategic_verified
    );

    // 13. Coupled Public-Private Stock-Flow Consistent (SFC) Society Synthesis
    let soc_syn = CoupledPublicPrivateSocietySynthesis::evaluate_canonical(DEFAULT_SEED);
    let cps_cohorts = load_cps_cohort_clusters(root);
    let emp_soc = EmpiricalCoupledSocietyAudit::evaluate_real_datasets(
        &cps_c_lens,
        &cps_cohorts,
        &bank_clusters_lens,
        &fire_losses,
    );
    println!(
        "  [Coupled Public-Private SFC Society Synthesis] H1a_SFC_Regulator(Closed_Surv={:.3} vs Naive={:.3}, Open_Surv={:.3} vs {:.3}, DiD_Surv={:+.3}, DiD_CRRA_Welfare={:+.4}, SFC_err={:.1e}, KG_err={:.1e}, Perm_Linf={:.1e}) | H1b_Bester_Credit(lambda*={:.3}, Gain_0.50={:+.3}, Gain_0.88={:+.3}) | H2_VCG_Commons(eps_same M4={:.4}->M32={:.4}, eps_disj={:.1e}, Commons_Stock={:.3} vs Rebate={:.3}, E_60={:.1}) | EmpCPS(m_tilde={:.2}, n_eff={:.1}, Meng_n_eff={:.1}, Manski_W={:.1}, CRRA_DJ={:+.4}, Card_OK={}) | identities={} | discarded={:?} | verified={}",
        soc_syn
            .h1a_regulator_coupling
            .shielded_closed_loop
            .terminal_firm_survival_rate,
        soc_syn
            .h1a_regulator_coupling
            .naive_closed_loop
            .terminal_firm_survival_rate,
        soc_syn
            .h1a_regulator_coupling
            .shielded_open_loop
            .terminal_firm_survival_rate,
        soc_syn
            .h1a_regulator_coupling
            .naive_open_loop
            .terminal_firm_survival_rate,
        soc_syn
            .h1a_regulator_coupling
            .deployed_coupling_did_firm_survival,
        soc_syn
            .h1a_regulator_coupling
            .deployed_coupling_did_crra_welfare,
        soc_syn.h1a_regulator_coupling.max_sfc_bilinear_error,
        soc_syn.h1a_regulator_coupling.max_kalecki_godley_error,
        soc_syn
            .h1a_regulator_coupling
            .agent_order_permutation_linf_diff,
        soc_syn.h1b_credit_screening.analytical_lambda_star_bester,
        soc_syn
            .h1b_credit_screening
            .deployed_survival_gain_lambda_050,
        soc_syn
            .h1b_credit_screening
            .deployed_survival_gain_lambda_088,
        soc_syn.h2_vcg_commons.cohort_grid_points[0].eps_same_cohort_buffer,
        soc_syn.h2_vcg_commons.cohort_grid_points[3].eps_same_cohort_buffer,
        soc_syn.h2_vcg_commons.cohort_grid_points[3].eps_disjoint_cohort_buffer,
        soc_syn.h2_vcg_commons.disjoint_cohort_vcg_stock_ratio,
        soc_syn.h2_vcg_commons.proportional_rebate_vcg_stock_ratio,
        soc_syn.h2_vcg_commons.agrapa_monitoring_terminal_e_value,
        emp_soc.cps_moulton_m_tilde_and_n_eff.0,
        emp_soc.cps_moulton_m_tilde_and_n_eff.1,
        emp_soc.cps_meng_n_eff_and_manski_width.0,
        emp_soc.cps_meng_n_eff_and_manski_width.1,
        emp_soc.cps_crra_jensen_gap_and_crps.0,
        emp_soc.preregistered_card_verified,
        soc_syn.all_identities_closed && emp_soc.empirical_identities_closed,
        soc_syn
            .discarded_findings
            .iter()
            .chain(&emp_soc.discarded_findings)
            .collect::<Vec<_>>(),
        soc_syn.all_axes_verified && emp_soc.empirical_society_verified
    );
}
