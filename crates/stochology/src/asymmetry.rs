//! **Chapter 4 (Filter II · Tension 3) — The Non-Identical, Scale Invariance, Convexity, and Heavy Tails.**
//!
//! Mediates the tension between the simplifying arithmetic concept and the asymmetric, qualitative,
//! heavy-tailed reality (Adorno, 1966; Georgescu-Roegen, 1971; Stevens, 1946; Jensen, 1906; Hill, 1975):
//! - Georgescu-Roegen's dialectical penumbra boundary sensitivity (`DialecticalPenumbra`);
//! - Stevens–Krantz ordinal scale invariance via Mann–Whitney dominance vs. arithmetic mean reversal (`OrdinalScaleAudit`);
//! - Jensen's Gap $\Delta_J = \mathbb{E}[f(X)] - f(\mathbb{E}[X])$, tail quantiles, and empirical CRPS (`NonlinearSummary`);
//! - Hill's extreme-value tail index $\hat{\alpha}_{k,n}$ and moment existence regimes (`HillTailEstimate`).

use crate::rng::{mean, quantile_sorted};

/// Georgescu-Roegen (1971) dialectical penumbra audit: measures how much a binary classification
/// rate $\hat{p}(c) = \frac{1}{n}\sum_{i=1}^n \mathbb{I}(x_i \ge c)$ shifts across a fuzzy
/// boundary zone $[c_L, c_U]$ where two qualitative states overlap.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DialecticalPenumbra {
    /// Prevalence when the whole penumbra is included ($\hat{p}(c_L)$).
    pub rate_at_lower_cut: f64,
    /// Prevalence at the midpoint cut ($\hat{p}((c_L + c_U)/2)$).
    pub rate_at_midpoint_cut: f64,
    /// Prevalence when the whole penumbra is excluded ($\hat{p}(c_U)$).
    pub rate_at_upper_cut: f64,
    /// Fraction of observations falling inside the penumbra $[c_L, c_U)$.
    pub penumbra_mass: f64,
}

impl DialecticalPenumbra {
    /// Evaluates the boundary instability of a continuous slice `xs` across the penumbra `[c_low, c_high]`.
    pub fn evaluate(xs: &[f64], c_low: f64, c_high: f64) -> Self {
        assert!(
            !xs.is_empty() && c_low <= c_high,
            "DialecticalPenumbra requires non-empty slice and c_low <= c_high"
        );
        let n = xs.len() as f64;
        let c_mid = 0.5 * (c_low + c_high);
        let r_low = (xs.iter().filter(|&&x| x >= c_low).count() as f64) / n;
        let r_mid = (xs.iter().filter(|&&x| x >= c_mid).count() as f64) / n;
        let r_high = (xs.iter().filter(|&&x| x >= c_high).count() as f64) / n;
        Self {
            rate_at_lower_cut: r_low,
            rate_at_midpoint_cut: r_mid,
            rate_at_upper_cut: r_high,
            penumbra_mass: (r_low - r_high).max(0.0),
        }
    }
}

/// Computes the Mann–Whitney ordinal dominance probability
/// $\theta_{\mathrm{MW}}(A, B) = \mathbb{P}(A > B) + \frac{1}{2}\mathbb{P}(A = B) \in [0, 1]$,
/// which is strictly invariant under any strictly increasing transformation $\phi$.
pub fn mann_whitney_dominance(group_a: &[f64], group_b: &[f64]) -> f64 {
    assert!(
        !group_a.is_empty() && !group_b.is_empty(),
        "mann_whitney_dominance requires non-empty slices"
    );
    let mut wins = 0.0_f64;
    for &a in group_a {
        for &b in group_b {
            if (a - b).abs() <= 1e-12 {
                wins += 0.5;
            } else if a > b {
                wins += 1.0;
            }
        }
    }
    wins / ((group_a.len() * group_b.len()) as f64)
}

/// Stevens (1946) & Krantz et al. (1971) ordinal scale invariance audit:
/// compares arithmetic mean differences against Mann–Whitney ordinal dominance before and after
/// a strictly increasing monotone map $\phi(x)$.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OrdinalScaleAudit {
    /// Raw arithmetic mean difference $\bar{A} - \bar{B}$.
    pub raw_mean_diff: f64,
    /// Transformed arithmetic mean difference $\overline{\phi(A)} - \overline{\phi(B)}$.
    pub transformed_mean_diff: f64,
    /// True if the arithmetic mean reverses its ranking under the monotone map $\phi$.
    pub mean_ranking_reversed: bool,
    /// Mann–Whitney dominance $\mathbb{P}(A > B) + \frac{1}{2}\mathbb{P}(A = B)$ on raw scores.
    pub raw_ordinal_dominance: f64,
    /// Mann–Whitney dominance on transformed scores $\phi(A), \phi(B)$.
    pub transformed_ordinal_dominance: f64,
    /// Absolute discrepancy between raw and transformed ordinal dominance (identically $0$ for strictly increasing $\phi$).
    pub ordinal_invariance_error: f64,
}

impl OrdinalScaleAudit {
    /// Evaluates scale invariance of `group_a` vs. `group_b` under a strictly increasing map `monotone_map`.
    pub fn evaluate<F>(group_a: &[f64], group_b: &[f64], monotone_map: F) -> Self
    where
        F: Fn(f64) -> f64,
    {
        let phi_a: Vec<f64> = group_a.iter().map(|&x| monotone_map(x)).collect();
        let phi_b: Vec<f64> = group_b.iter().map(|&x| monotone_map(x)).collect();
        let raw_diff = mean(group_a) - mean(group_b);
        let trans_diff = mean(&phi_a) - mean(&phi_b);
        let mw_raw = mann_whitney_dominance(group_a, group_b);
        let mw_trans = mann_whitney_dominance(&phi_a, &phi_b);
        Self {
            raw_mean_diff: raw_diff,
            transformed_mean_diff: trans_diff,
            mean_ranking_reversed: raw_diff * trans_diff < 0.0,
            raw_ordinal_dominance: mw_raw,
            transformed_ordinal_dominance: mw_trans,
            ordinal_invariance_error: (mw_raw - mw_trans).abs(),
        }
    }
}

/// Empirical Continuous Ranked Probability Score (CRPS) of an ensemble forecast `forecast_samples`
/// evaluated against a realized scalar outcome `observation_y`:
/// $$\mathrm{CRPS}(F, y) = \frac{1}{m}\sum_{i=1}^m |X_i - y| - \frac{1}{2 m^2}\sum_{i=1}^m\sum_{j=1}^m |X_i - X_j|$$
/// Computed in $O(m \log m)$ via sorted order statistics.
pub fn crps_empirical(forecast_samples: &[f64], observation_y: f64) -> f64 {
    assert!(
        !forecast_samples.is_empty(),
        "crps_empirical requires non-empty forecast slice"
    );
    let mut sorted = forecast_samples.to_vec();
    sorted.sort_by(|a, b| a.total_cmp(b));
    let m = sorted.len();
    let mf = m as f64;

    let term1: f64 = sorted
        .iter()
        .map(|&x| (x - observation_y).abs())
        .sum::<f64>()
        / mf;
    // Note: \sum_{i,j} |x_i - x_j| = 2 \sum_{i=0}^{m-1} (2i - m + 1) x_{(i)}
    let mut weighted_sum = 0.0;
    for (i, &x) in sorted.iter().enumerate() {
        let coeff = (2 * i) as f64 - (m as f64) + 1.0;
        weighted_sum += coeff * x;
    }
    let half_spread = weighted_sum / (mf * mf);
    (term1 - half_spread).max(0.0)
}

/// Nonlinear distributional summary exposing the failure of planning by the lone mean
/// under convex harm $f(X)$ (Jensen, 1906):
/// $$\Delta_J = \mathbb{E}[f(X)] - f(\mathbb{E}[X]) \ge 0$$
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NonlinearSummary {
    /// Sample arithmetic mean $\mathbb{E}[X]$.
    pub sample_mean: f64,
    /// Naive evaluation of the loss/cost function at the mean: $f(\mathbb{E}[X])$.
    pub eval_at_mean: f64,
    /// True expected loss across the distribution: $\mathbb{E}[f(X)]$.
    pub expected_eval: f64,
    /// Jensen's Gap $\Delta_J = \mathbb{E}[f(X)] - f(\mathbb{E}[X])$.
    pub jensen_gap: f64,
    /// Sample median ($q_{50}$).
    pub median: f64,
    /// Lower $5\%$ quantile ($q_{05}$).
    pub q05: f64,
    /// Upper $95\%$ quantile ($q_{95}$).
    pub q95: f64,
    /// Empirical CRPS evaluated at the sample median.
    pub crps_at_median: f64,
}

impl NonlinearSummary {
    /// Computes the full nonlinear summary and Jensen's Gap for `xs` under loss function `loss_fn`.
    pub fn evaluate<F>(xs: &[f64], loss_fn: F) -> Self
    where
        F: Fn(f64) -> f64,
    {
        assert!(
            !xs.is_empty(),
            "NonlinearSummary requires a non-empty slice"
        );
        let mut sorted = xs.to_vec();
        sorted.sort_by(|a, b| a.total_cmp(b));
        let m = mean(&sorted);
        let f_of_mean = loss_fn(m);
        let mean_of_f = sorted.iter().map(|&x| loss_fn(x)).sum::<f64>() / (sorted.len() as f64);
        let med = quantile_sorted(&sorted, 0.50);
        let q05 = quantile_sorted(&sorted, 0.05);
        let q95 = quantile_sorted(&sorted, 0.95);
        let crps = crps_empirical(&sorted, med);

        Self {
            sample_mean: m,
            eval_at_mean: f_of_mean,
            expected_eval: mean_of_f,
            jensen_gap: mean_of_f - f_of_mean,
            median: med,
            q05,
            q95,
            crps_at_median: crps,
        }
    }
}

/// Moment-existence regime classified by the extreme-value tail index $\alpha$ (Hill, 1975).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TailMomentRegime {
    /// $\alpha \le 1$: Neither the population mean nor the variance is finite.
    InfiniteMeanAndVariance,
    /// $1 < \alpha \le 2$: Finite population mean, infinite population variance (classical CLT fails).
    FiniteMeanInfiniteVariance,
    /// $\alpha > 2$: Both population mean and variance are finite.
    FiniteVariance,
}

/// Bruce M. Hill (1975) extreme-value tail index estimator $\hat{\alpha}_{k,n}$ over the top $k$ order statistics.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HillTailEstimate {
    /// Number of upper order statistics $k$ used in the tail fit.
    pub tail_k: usize,
    /// Threshold order statistic $X_{(n-k)}$.
    pub threshold_x: f64,
    /// Estimated Pareto tail index $\hat{\alpha}_{k,n}$.
    pub alpha_hat: f64,
    /// Asymptotic standard error $\hat{\alpha}_{k,n} / \sqrt{k}$.
    pub standard_error: f64,
    /// Moment-existence regime implied by $\hat{\alpha}_{k,n}$.
    pub regime: TailMomentRegime,
}

impl HillTailEstimate {
    /// Estimates the upper tail index $\hat{\alpha}_{k,n}$ using the top `tail_k` positive observations of `xs`.
    pub fn evaluate(xs: &[f64], tail_k: usize) -> Self {
        let mut pos: Vec<f64> = xs.iter().copied().filter(|&x| x > 0.0).collect();
        pos.sort_by(|a, b| a.total_cmp(b));
        let n = pos.len();
        assert!(
            n >= 4 && tail_k >= 2 && tail_k < n,
            "HillTailEstimate requires at least 4 positive observations and 2 <= tail_k < n"
        );
        let x_thresh = pos[n - 1 - tail_k];
        let mut log_sum = 0.0;
        for &x in &pos[n - tail_k..] {
            log_sum += (x / x_thresh).max(1.0).ln();
        }
        let mean_log_exceedance = (log_sum / (tail_k as f64)).max(1e-12);
        let alpha = 1.0 / mean_log_exceedance;
        let se = alpha / (tail_k as f64).sqrt();
        let regime = if alpha <= 1.0 {
            TailMomentRegime::InfiniteMeanAndVariance
        } else if alpha <= 2.0 {
            TailMomentRegime::FiniteMeanInfiniteVariance
        } else {
            TailMomentRegime::FiniteVariance
        };
        Self {
            tail_k,
            threshold_x: x_thresh,
            alpha_hat: alpha,
            standard_error: se,
            regime,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordinal_scale_invariance_and_jensen_gap_hold() {
        let a = [3.0, 3.0, 3.0];
        let b = [1.0, 3.0, 4.9];
        let ord = OrdinalScaleAudit::evaluate(&a, &b, |x| x * x);
        assert!(ord.mean_ranking_reversed);
        assert!(ord.ordinal_invariance_error < 1e-12);

        let summary = NonlinearSummary::evaluate(&[1.0, 2.0, 3.0, 10.0], |x| x * x);
        assert!(summary.jensen_gap > 10.0);
        assert!(
            (summary.jensen_gap - (summary.expected_eval - summary.eval_at_mean)).abs() < 1e-12
        );
    }

    #[test]
    fn hill_estimator_classifies_heavy_tail_regimes() {
        let mut rng = crate::rng::SplitMix64::with_default_seed();
        let samples: Vec<f64> = (0..2000).map(|_| rng.pareto(1.0, 1.45)).collect();
        let hill = HillTailEstimate::evaluate(&samples, 150);
        assert_eq!(hill.regime, TailMomentRegime::FiniteMeanInfiniteVariance);
        assert!((hill.alpha_hat - 1.45).abs() < 0.25);
    }
}
