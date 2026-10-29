//! Shared non-parametric and parametric statistical primitives (`stochology_core::math::stats`).

pub use crate::layers::empirical_wasserstein_1;
pub use crate::rng::{mean, quantile, sample_std, sample_variance, OnlineMoments};

/// Sample Pearson linear correlation coefficient $r \in [-1, 1]$.
pub fn pearson_correlation(x: &[f64], y: &[f64]) -> f64 {
    if x.len() != y.len() || x.len() < 2 {
        return 0.0;
    }
    let mx = mean(x);
    let my = mean(y);
    let mut num = 0.0;
    let mut dx2 = 0.0;
    let mut dy2 = 0.0;
    for (&xi, &yi) in x.iter().zip(y.iter()) {
        let dx = xi - mx;
        let dy = yi - my;
        num += dx * dy;
        dx2 += dx * dx;
        dy2 += dy * dy;
    }
    let denom = (dx2 * dy2).sqrt();
    if denom > 1e-15 {
        (num / denom).clamp(-1.0, 1.0)
    } else {
        0.0
    }
}

/// Computes mid-rank (tie-averaged) 1-based ranks of slice `x`.
pub fn average_ranks(x: &[f64]) -> Vec<f64> {
    let n = x.len();
    let mut indexed: Vec<(usize, f64)> = x.iter().copied().enumerate().collect();
    indexed.sort_by(|a, b| a.1.total_cmp(&b.1));
    let mut ranks = vec![0.0; n];
    let mut i = 0;
    while i < n {
        let mut j = i + 1;
        while j < n && (indexed[j].1 - indexed[i].1).abs() <= 1e-12 {
            j += 1;
        }
        let avg_rank = (i + 1 + j) as f64 * 0.5;
        for k in i..j {
            ranks[indexed[k].0] = avg_rank;
        }
        i = j;
    }
    ranks
}

/// Spearman rank correlation $r_s \in [-1, 1]$ with exact mid-rank tie averaging.
pub fn spearman_rank_correlation(x: &[f64], y: &[f64]) -> f64 {
    if x.len() != y.len() || x.len() < 2 {
        return 0.0;
    }
    let rx = average_ranks(x);
    let ry = average_ranks(y);
    pearson_correlation(&rx, &ry)
}

/// Two-sample Kolmogorov-Smirnov supremum distance $D_{\mathrm{KS}} = \sup_t |\hat{F}_X(t) - \hat{F}_Y(t)|$.
pub fn two_sample_ks_distance(x: &[f64], y: &[f64]) -> f64 {
    if x.is_empty() || y.is_empty() {
        return 1.0;
    }
    let mut xs = x.to_vec();
    let mut ys = y.to_vec();
    xs.sort_by(|a, b| a.total_cmp(b));
    ys.sort_by(|a, b| a.total_cmp(b));
    let nx = xs.len() as f64;
    let ny = ys.len() as f64;
    let mut ix = 0usize;
    let mut iy = 0usize;
    let mut max_d = 0.0_f64;
    while ix < xs.len() && iy < ys.len() {
        let v = xs[ix].min(ys[iy]);
        while ix < xs.len() && xs[ix] <= v {
            ix += 1;
        }
        while iy < ys.len() && ys[iy] <= v {
            iy += 1;
        }
        let d = ((ix as f64 / nx) - (iy as f64 / ny)).abs();
        if d > max_d {
            max_d = d;
        }
    }
    max_d
}

/// 1D Wasserstein-1 distance $W_1(\hat{P}, \hat{Q}) = \int_0^1 |F_{\hat{P}}^{-1}(u) - F_{\hat{Q}}^{-1}(u)|\,du$
/// evaluated via `quantile_sorted` on a uniform midpoint grid of `grid_steps` points.
pub fn empirical_wasserstein_1_grid(a: &[f64], b: &[f64], grid_steps: usize) -> f64 {
    if a.is_empty() || b.is_empty() || grid_steps == 0 {
        return 0.0;
    }
    let mut sa = a.to_vec();
    let mut sb = b.to_vec();
    sa.sort_by(|x, y| x.total_cmp(y));
    sb.sort_by(|x, y| x.total_cmp(y));
    (0..grid_steps)
        .map(|k| {
            let u = (k as f64 + 0.5) / (grid_steps as f64);
            (crate::rng::quantile_sorted(&sa, u) - crate::rng::quantile_sorted(&sb, u)).abs()
        })
        .sum::<f64>()
        / (grid_steps as f64)
}
