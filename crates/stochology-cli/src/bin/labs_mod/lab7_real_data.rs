//! Lab 7: Transversal Real-Data Synthesis Across All 13 Empirical Datasets (`data/*.csv`) and 4 Canonical Cards (`cards/*.yaml`).

use std::path::Path;
use stochology_cli::{
    PublicInfrastructureCard04Audit, load_atlantic_storms_decade_environments,
    load_atlantic_storms_forecasts, load_bank_call_clusters, load_cards_from_disk,
    load_danish_fire, load_djia_returns, load_lalonde_nsw_cps, load_nyc_flights_2013_2023,
    load_oot_ood_flight_reserves, load_toronto_arrests_prevalences, load_vic_elec_demand,
    verify_card_against_empirical_data,
};
use stochology_core::{
    ChouldechovaAudit, ExceedanceDesignEffect, NonlinearSummary, PerformativeSystem,
    apply_cromwell_floor, mean, sample_std,
};
use stochology_governance::{TemporalVault, fit_covariate_drift_extra_wcp};
use stochology_lenses::{
    CorpPavDecomposition, EmpiricalCriticalRationalityAudit, EmpiricalStocharyCyberneticAudit,
    EmpiricalTriadicTechniumAudit, EmpiricalWaveAnalogAudit, HillTailDiagnostic,
    TimeKellyDiagnostic,
};

/// Lab 7: Transversal Real-Data Synthesis Across All 13 Empirical Datasets (`data/*.csv`) and 4 Canonical Cards (`cards/*.yaml`).
pub fn lab_real_data(root: &Path) {
    println!(
        "=== LAB 7: TRANSVERSAL REAL-DATA SYNTHESIS (13 EMPIRICAL DATASETS & 4 CANONICAL CARDS) ==="
    );

    // 1. Public Policy, Digital Public Infrastructure & Observational Selection
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

    // 2. Banking Operations & Queue Physics
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

    // 3. Long-Horizon Financial Non-Ergodicity & Performativity
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

    // 4. Heavy Tails & Infinite Variance in Reinsurance and Energy
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

    // 5. Isotonic Calibration & Algebraic Fairness
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

    // 6. Multi-Horizon & Out-of-Domain Aviation Drift
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

    // 7. End-to-End Epistemological Card Ledger
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
