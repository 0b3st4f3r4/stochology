//! Lab 2: Kennedy-O'Hagan Gaussian Process Structural Discrepancy and Info-Gap + Sniedovich Safeguard.

use stochology_core::{InfoGapOption, KennedyOHaganDiscrepancy};

/// Lab 2: Kennedy-O'Hagan Gaussian Process Structural Discrepancy and Info-Gap + Sniedovich Safeguard.
pub fn lab_uncertainty() {
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
