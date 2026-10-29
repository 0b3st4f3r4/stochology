//! Lab 6: Production Governance: Bitemporal Clock, Single-Opening Vault, and 4 Regimes of Drift.

use std::path::Path;
use stochology_cli::{load_nyc_flights_2013_2023, load_nyc_flights_cascade_2013_2023};
use stochology_core::{hmac_sha256_hex, mean};
use stochology_governance::{
    estimate_label_shift_four_methods, fit_calibrated_logistic_1d, fit_covariate_drift_extra_wcp,
    query_with_bitemporal_clock, AdaptiveConformalInference, BitemporalRecord,
    MurphyCorpCircuitBreaker, PredeterminedChangeControlPlan, TemporalVault,
};

/// Lab 6: Production Governance: Bitemporal Clock, Single-Opening Vault, and 4 Regimes of Drift.
pub fn lab_governance(root: &Path) {
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
