//! Adaptation to Covariate Drift via Exponential Tilt and Weighted Conformal Prediction
//! (*ExTRA-WCP-T*) with Lei & Candès Total Variation Bound (`CovariateDriftResult`, `fit_covariate_drift_extra_wcp`, §4.2).

use stochology_core::quantile;

/// Adaptation to Covariate Drift via Exponential Tilt and Weighted Conformal Prediction
/// (*ExTRA-WCP-T*) with Lei & Candès Total Variation Bound (§4.2).
///
/// Estimates $\beta$ by maximizing $\hat{L}(\beta) = \frac{1}{m}\sum_{j=1}^m \ln M_\beta(X_j^Q) - \ln Z_P(\beta) - \frac{\lambda}{2}\|\beta\|_2^2$
/// and computes the exact lower coverage bound:
/// $$\mathbb{P}_{P^n \times Q}\{Y_{n+1} \in C_{S, \hat{w}}(X_{n+1}) \mid \mathcal{F}\} \ge 1 - \alpha - d_{\mathrm{TV}}(Q, \hat{Q}), \qquad d_{\mathrm{TV}}(Q, \hat{Q}) = \frac{1}{2}\mathbb{E}_P|w^\star - \hat{w}|$$
#[derive(Debug, Clone, PartialEq)]
pub struct CovariateDriftResult {
    pub estimated_beta: f64,
    pub normalizing_constant_z_p: f64,
    pub regularized_objective_l_hat: f64,
    pub total_variation_distance_d_tv: f64,
    pub nominal_level_one_minus_alpha: f64,
    pub lei_candes_coverage_lower_bound: f64,
    pub weighted_conformal_threshold: f64,
}

pub fn fit_covariate_drift_extra_wcp(
    x_source_p: &[f64],
    x_target_q: &[f64],
    nonconformity_scores_source: &[f64],
    true_weights_optional: Option<&[f64]>,
    lambda_reg: f64,
    alpha: f64,
) -> CovariateDriftResult {
    assert!(!x_source_p.is_empty() && !x_target_q.is_empty());
    assert_eq!(x_source_p.len(), nonconformity_scores_source.len());
    let mean_q = stochology_core::mean(x_target_q);

    // Concave 1D Newton optimization for L_hat(beta) = beta * mean_Q(x) - ln E_P[exp(beta * x)] - 0.5 * lambda * beta^2
    let mut beta = 0.0_f64;
    for _ in 0..40 {
        let max_bx = x_source_p
            .iter()
            .map(|&x| beta * x)
            .fold(f64::NEG_INFINITY, f64::max);
        let exp_weights: Vec<f64> = x_source_p
            .iter()
            .map(|&x| (beta * x - max_bx).exp())
            .collect();
        let sum_w: f64 = exp_weights.iter().sum();
        let e_x: f64 = x_source_p
            .iter()
            .zip(exp_weights.iter())
            .map(|(&x, &w)| x * w)
            .sum::<f64>()
            / sum_w;
        let e_x2: f64 = x_source_p
            .iter()
            .zip(exp_weights.iter())
            .map(|(&x, &w)| x * x * w)
            .sum::<f64>()
            / sum_w;
        let grad = mean_q - e_x - lambda_reg * beta;
        let hess = (e_x2 - e_x * e_x + lambda_reg).max(1e-6);
        beta += grad / hess;
    }

    let max_bx = x_source_p
        .iter()
        .map(|&x| beta * x)
        .fold(f64::NEG_INFINITY, f64::max);
    let exp_shifted: Vec<f64> = x_source_p
        .iter()
        .map(|&x| (beta * x - max_bx).exp())
        .collect();
    let mean_shifted = stochology_core::mean(&exp_shifted);
    let z_p = mean_shifted * max_bx.exp();
    let w_hat: Vec<f64> = exp_shifted.iter().map(|&e| e / mean_shifted).collect();
    let l_hat = beta * mean_q - (max_bx + mean_shifted.ln()) - 0.5 * lambda_reg * beta * beta;

    // Total Variation distance d_TV(Q, Q_hat):
    // Uses exact 0.5 * E_P[|w* - w_hat|] when oracle weights w* are provided; otherwise uses the
    // empirical 10-decile-bin L1 Total Variation distance (and Gaussian Pinsker bound) between
    // target sample Q and tilted source distribution (x_source_p, w_hat).
    let d_tv = if let Some(w_star) = true_weights_optional {
        0.5 * w_star
            .iter()
            .zip(w_hat.iter())
            .map(|(&ws, &wh)| (ws - wh).abs())
            .sum::<f64>()
            / (w_hat.len() as f64)
    } else {
        let sum_wh: f64 = w_hat.iter().sum::<f64>().max(1e-12);
        let mu_q_hat: f64 = x_source_p
            .iter()
            .zip(w_hat.iter())
            .map(|(&x, &w)| x * w)
            .sum::<f64>()
            / sum_wh;
        let var_q_hat: f64 = (x_source_p
            .iter()
            .zip(w_hat.iter())
            .map(|(&x, &w)| (x - mu_q_hat).powi(2) * w)
            .sum::<f64>()
            / sum_wh)
            .max(1e-6);
        let var_q: f64 = if x_target_q.len() >= 2 {
            stochology_core::sample_variance(x_target_q).max(1e-6)
        } else {
            var_q_hat
        };
        let kl_gauss = 0.5
            * ((var_q_hat / var_q).ln() + (var_q + (mean_q - mu_q_hat).powi(2)) / var_q_hat - 1.0)
                .max(0.0);
        let tv_gauss = (0.5 * kl_gauss).sqrt();
        let tv_decile = empirical_decile_tv_distance(x_source_p, &w_hat, x_target_q);
        tv_gauss.max(tv_decile).clamp(0.0, 1.0)
    };

    // Weighted quantile (ExTRA-WCP-T) of nonconformity scores with Lei-Candès TV-adjusted level (1 - alpha + d_TV)
    let mut pairs: Vec<(f64, f64)> = nonconformity_scores_source
        .iter()
        .copied()
        .zip(w_hat.iter().copied())
        .collect();
    pairs.sort_by(|a, b| a.0.total_cmp(&b.0));
    let sum_weights: f64 = pairs.iter().map(|(_, w)| *w).sum::<f64>() + 1.0;
    let effective_quantile_level = (1.0 - alpha + d_tv).clamp(0.0, 1.0);
    let target_cum = effective_quantile_level * sum_weights;
    let mut cum = 0.0;
    let mut conformal_threshold = pairs
        .last()
        .map(|p| p.0)
        .unwrap_or(quantile(nonconformity_scores_source, 1.0 - alpha));
    for (s, w) in pairs {
        cum += w;
        if cum >= target_cum {
            conformal_threshold = s;
            break;
        }
    }

    let lei_candes_bound = (1.0 - alpha - d_tv).max(0.0);

    CovariateDriftResult {
        estimated_beta: beta,
        normalizing_constant_z_p: z_p,
        regularized_objective_l_hat: l_hat,
        total_variation_distance_d_tv: d_tv,
        nominal_level_one_minus_alpha: 1.0 - alpha,
        lei_candes_coverage_lower_bound: lei_candes_bound,
        weighted_conformal_threshold: conformal_threshold,
    }
}

/// Non-parametric empirical quantile-bin $L_1$ Total Variation distance between the target sample
/// `x_target_q` and the tilted source distribution `(x_source_p, w_hat)` across 10 pooled decile bins.
fn empirical_decile_tv_distance(x_source_p: &[f64], w_hat: &[f64], x_target_q: &[f64]) -> f64 {
    let mut pooled = Vec::with_capacity(x_source_p.len() + x_target_q.len());
    pooled.extend_from_slice(x_source_p);
    pooled.extend_from_slice(x_target_q);
    pooled.sort_by(|a, b| a.total_cmp(b));
    if pooled.is_empty() {
        return 0.0;
    }
    let mut cuts = [0.0_f64; 9];
    for (k, cut) in cuts.iter_mut().enumerate() {
        *cut = stochology_core::quantile_sorted(&pooled, ((k + 1) as f64) * 0.10);
    }
    let bin_index = |x: f64| -> usize {
        for (k, &c) in cuts.iter().enumerate() {
            if x <= c {
                return k;
            }
        }
        9
    };

    let mut p_bins = [0.0_f64; 10];
    let sum_w = w_hat.iter().sum::<f64>().max(1e-12);
    for (&x, &w) in x_source_p.iter().zip(w_hat.iter()) {
        p_bins[bin_index(x)] += w / sum_w;
    }

    let mut q_bins = [0.0_f64; 10];
    let inv_m = 1.0 / (x_target_q.len().max(1) as f64);
    for &x in x_target_q {
        q_bins[bin_index(x)] += inv_m;
    }

    0.5 * (0..10).map(|b| (p_bins[b] - q_bins[b]).abs()).sum::<f64>()
}
