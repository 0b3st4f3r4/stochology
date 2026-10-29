//! Filter 2 Prospective Gold-Set Sample Sizing for Prediction-Powered Inference (`PPI`, §3.1).

pub use stochology_core::prospective_clustered_mde;

/// Prospective gold-set sample size $n_{\text{gold}}^*$ for Prediction-Powered Inference (PPI;
/// Angelopoulos et al., 2023) at Filter 2 before deploying an `LLM-as-a-Judge` evaluator:
/// $$n_{\text{gold}}^* = \left\lceil \frac{\sigma_{\Delta}^2}{\bigl(h_{\text{target}} / z_{1-\alpha/2}\bigr)^2 - \sigma_f^2 / N} \right\rceil,$$
/// where $\sigma_{\Delta}^2 = \mathrm{Var}(Y - f(X))$ is the rectifier residual variance, $\sigma_f^2 = \mathrm{Var}(f(\tilde{X}))$
/// is the judge variance on $N$ unlabeled records, and $h_{\text{target}}$ is the maximum tolerated half-width.
/// Returns `usize::MAX` if the unlabeled variance floor $\sigma_f^2 / N$ already exceeds $(h_{\text{target}} / z_{1-\alpha/2})^2$.
pub fn prospective_ppi_gold_sample_size(
    target_half_width: f64,
    z_alpha_half: f64,
    unlabeled_n: usize,
    var_f: f64,
    var_rectifier_diff: f64,
) -> usize {
    assert!(
        target_half_width.is_finite()
            && target_half_width > 0.0
            && z_alpha_half.is_finite()
            && z_alpha_half > 0.0
            && unlabeled_n > 0
            && var_f.is_finite()
            && var_f >= 0.0
            && var_rectifier_diff.is_finite()
            && var_rectifier_diff > 0.0,
        "prospective_ppi_gold_sample_size requires positive finite arguments"
    );
    let target_var = (target_half_width / z_alpha_half).powi(2);
    let unlabeled_var = var_f / (unlabeled_n as f64);
    let residual_budget = target_var - unlabeled_var;
    if residual_budget <= 1e-15 {
        return usize::MAX;
    }
    ((var_rectifier_diff / residual_budget).ceil() as usize).max(2)
}

/// Effective sample-size multiplier of Prediction-Powered Inference relative to classical gold-only
/// estimation on $n_{\text{gold}}$ human-labeled records:
/// $$\mathrm{ESS}_{\text{mult}} = \frac{\sigma_Y^2 / n_{\text{gold}}}{\sigma_f^2 / N + \sigma_{\Delta}^2 / n_{\text{gold}}}.$$
pub fn ppi_effective_sample_size_multiplier(
    var_y: f64,
    var_f: f64,
    var_rectifier_diff: f64,
    unlabeled_n: usize,
    gold_n: usize,
) -> f64 {
    assert!(
        var_y.is_finite()
            && var_y > 0.0
            && var_f.is_finite()
            && var_f >= 0.0
            && var_rectifier_diff.is_finite()
            && var_rectifier_diff > 0.0
            && unlabeled_n > 0
            && gold_n > 0,
        "ppi_effective_sample_size_multiplier requires positive finite variances and sample sizes"
    );
    let classical_var = var_y / (gold_n as f64);
    let ppi_var = var_f / (unlabeled_n as f64) + var_rectifier_diff / (gold_n as f64);
    classical_var / ppi_var.max(1e-15)
}
