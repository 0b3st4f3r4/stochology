//! Bridge $L_2 \to L_3$ (Misreading) and Prohibition 2 (Ban on the Lone Mean):
//! Jensen's Gap $\Delta_J = \mathbb{E}[f(X)] - f(\mathbb{E}[X])$, tail quantiles ($q_{05}, q_{95}$),
//! and Continuous Ranked Probability Score ($\mathrm{CRPS}$).

use crate::rng::{mean, median, quantile, sample_std};

/// Audit of Bridge $L_2 \to L_3$ (Misreading) and Prohibition 2 (Ban on the Lone Mean):
/// Quantifies Jensen's Gap $\Delta_J = \mathbb{E}[f(X)] - f(\mathbb{E}[X])$, median, tail quantiles
/// ($q_{05}, q_{95}$), and empirical CRPS.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NonlinearSummary {
    pub mean_x: f64,
    pub std_x: f64,
    /// Evaluation at the mean $f(\mathbb{E}[X])$.
    pub f_at_mean: f64,
    /// Simulated expectation $\mathbb{E}[f(X)]$.
    pub expected_f: f64,
    /// Exact Jensen Gap $\Delta_J = \mathbb{E}[f(X)] - f(\mathbb{E}[X])$.
    pub jensen_gap: f64,
    /// Second-order Taylor approximation $\frac{1}{2} f''(\mathbb{E}[X]) \sigma_X^2$.
    pub second_order_taylor_approx: f64,
    pub median_f: f64,
    pub q05_f: f64,
    pub q95_f: f64,
    /// Out-of-sample (or leave-one-out) Continuous Ranked Probability Score ($\mathrm{CRPS}$) of $f(X)$.
    pub empirical_crps: f64,
}

impl NonlinearSummary {
    /// Evaluates a nonlinear loss/payoff function $f(X)$ and its local curvature $f''(\mathbb{E}[X])$ over sample `xs`,
    /// computing exact leave-one-out out-of-sample $\mathrm{CRPS}$ in $O(n \log n)$ via order-statistic prefix sums.
    pub fn evaluate<F>(xs: &[f64], f: F, f_second_derivative_at_mean: f64) -> Self
    where
        F: Fn(f64) -> f64,
    {
        assert!(xs.len() >= 2, "NonlinearSummary requires at least 2 points");
        let fx: Vec<f64> = xs.iter().map(|&x| f(x)).collect();
        let crps = exact_loo_empirical_crps(&fx);
        Self::from_transformed(xs, &fx, f(mean(xs)), f_second_derivative_at_mean, crps)
    }

    /// Evaluates $f(X)$ on `forecast_xs` and scores out-of-sample $\mathrm{CRPS}$ against held-out `observed_xs`.
    pub fn evaluate_out_of_sample<F>(
        forecast_xs: &[f64],
        observed_xs: &[f64],
        f: F,
        f_second_derivative_at_mean: f64,
    ) -> Self
    where
        F: Fn(f64) -> f64,
    {
        assert!(
            forecast_xs.len() >= 2 && !observed_xs.is_empty(),
            "requires at least 2 forecast points and 1 observed out-of-sample point"
        );
        let fx: Vec<f64> = forecast_xs.iter().map(|&x| f(x)).collect();
        let oos_crps: f64 = observed_xs
            .iter()
            .map(|&y| empirical_crps(&fx, f(y)))
            .sum::<f64>()
            / (observed_xs.len() as f64);
        Self::from_transformed(
            forecast_xs,
            &fx,
            f(mean(forecast_xs)),
            f_second_derivative_at_mean,
            oos_crps,
        )
    }

    fn from_transformed(
        xs: &[f64],
        fx: &[f64],
        f_mx: f64,
        f_second_derivative_at_mean: f64,
        crps: f64,
    ) -> Self {
        let mx = mean(xs);
        let sx = sample_std(xs);
        let ef = mean(fx);
        let gap = ef - f_mx;
        let taylor = 0.5 * f_second_derivative_at_mean * sx * sx;
        let med = median(fx);
        let q05 = quantile(fx, 0.05);
        let q95 = quantile(fx, 0.95);
        Self {
            mean_x: mx,
            std_x: sx,
            f_at_mean: f_mx,
            expected_f: ef,
            jensen_gap: gap,
            second_order_taylor_approx: taylor,
            median_f: med,
            q05_f: q05,
            q95_f: q95,
            empirical_crps: crps,
        }
    }
}

/// Empirical Continuous Ranked Probability Score ($\mathrm{CRPS}$):
/// $$\mathrm{CRPS}(F, y) = \mathbb{E}_F|X - y| - \frac{1}{2}\mathbb{E}_F|X - X'|$$
pub fn empirical_crps(forecast_samples: &[f64], observation_y: f64) -> f64 {
    assert!(!forecast_samples.is_empty());
    let mut sorted = forecast_samples.to_vec();
    sorted.sort_by(|a, b| a.total_cmp(b));
    let n = sorted.len() as f64;
    let term1: f64 = sorted
        .iter()
        .map(|&x| (x - observation_y).abs())
        .sum::<f64>()
        / n;
    // O(n log n) evaluation of E|X - X'| = (2 / n^2) * sum_{i=0}^{n-1} (2*i - n + 1) * sorted[i]
    let mut spread = 0.0;
    for (i, &x) in sorted.iter().enumerate() {
        let coeff = (2.0 * (i as f64)) - n + 1.0;
        spread += coeff * x;
    }
    let e_xx = (2.0 * spread) / (n * n);
    term1 - 0.5 * e_xx
}

/// Exact $O(n \log n)$ leave-one-out average empirical $\mathrm{CRPS}$ across a sample `fx` of size $n \ge 2$:
/// $$\overline{\mathrm{CRPS}}_{\mathrm{LOO}} = \frac{1}{n}\sum_{i=1}^n \mathrm{CRPS}(F_{-i}, x_i) = \frac{n + 1}{2(n - 1)^2} \cdot \mathbb{E}_{\hat{F}_n}|X - X'|$$
fn exact_loo_empirical_crps(fx: &[f64]) -> f64 {
    assert!(fx.len() >= 2);
    let mut sorted = fx.to_vec();
    sorted.sort_by(|a, b| a.total_cmp(b));
    let n = sorted.len() as f64;
    let m = n - 1.0;
    let mut pair_diff_sum = 0.0;
    for (k, &x) in sorted.iter().enumerate() {
        pair_diff_sum += ((2.0 * (k as f64)) - n + 1.0) * x;
    }
    let avg_term1 = (2.0 * pair_diff_sum) / (n * m);
    let avg_half_spread = ((n - 2.0) * pair_diff_sum) / (n * m * m);
    avg_term1 - avg_half_spread
}
