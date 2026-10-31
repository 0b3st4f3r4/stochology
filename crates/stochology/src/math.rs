//! Essential mathematical primitives powering the sequential loop:
//! Pool-Adjacent-Violators (PAV) isotonic regression, ordinary/ridge least-squares,
//! Cholesky symmetric positive-definite solver, rank correlation, and Gaussian CDF.

/// Standard normal cumulative distribution function $\Phi(z)$ via Abramowitz & Stegun 7.1.26
/// rational approximation (maximum absolute error $< 1.5 \times 10^{-7}$).
pub fn standard_normal_cdf(z: f64) -> f64 {
    let sign = if z < 0.0 { -1.0 } else { 1.0 };
    let x = z.abs() / std::f64::consts::SQRT_2;
    let t = 1.0 / (1.0 + 0.3275911 * x);
    let poly = t
        * (0.254829592
            + t * (-0.284496736 + t * (1.421413741 + t * (-1.453152027 + t * 1.061405429))));
    let erf = 1.0 - poly * (-x * x).exp();
    0.5 * (1.0 + sign * erf)
}

/// Pool-Adjacent-Violators (PAV) algorithm for monotonically non-decreasing isotonic regression.
///
/// Given pairs $(x_i, y_i)$ already sorted by non-decreasing $x_i$, computes the unique
/// non-decreasing sequence $\hat{y}_1 \le \hat{y}_2 \le \dots \le \hat{y}_n$ minimizing
/// $\sum_{i=1}^n (y_i - \hat{y}_i)^2$.
pub fn pav_isotonic_non_decreasing(y: &[f64]) -> Vec<f64> {
    if y.is_empty() {
        return Vec::new();
    }
    // Each block stores (sum_y, count)
    let mut blocks: Vec<(f64, usize)> = Vec::with_capacity(y.len());
    for &val in y {
        blocks.push((val, 1));
        while blocks.len() >= 2 {
            let n = blocks.len();
            let (s_prev, c_prev) = blocks[n - 2];
            let (s_curr, c_curr) = blocks[n - 1];
            if s_prev / (c_prev as f64) > s_curr / (c_curr as f64) {
                blocks.pop();
                blocks[n - 2] = (s_prev + s_curr, c_prev + c_curr);
            } else {
                break;
            }
        }
    }
    let mut out = Vec::with_capacity(y.len());
    for (sum_y, count) in blocks {
        let avg = sum_y / (count as f64);
        for _ in 0..count {
            out.push(avg);
        }
    }
    out
}

/// Solves a symmetric positive-definite linear system $A x = b$ ($d \times d$) via Cholesky decomposition
/// $A = L L^\top$. Returns `None` if $A$ is not numerically positive-definite.
#[allow(clippy::needless_range_loop)]
pub fn solve_cholesky_spd(a: &[Vec<f64>], b: &[f64]) -> Option<Vec<f64>> {
    let d = b.len();
    if a.len() != d || a.iter().any(|row| row.len() != d) {
        return None;
    }
    let mut l = vec![vec![0.0; d]; d];
    for i in 0..d {
        for j in 0..=i {
            let mut sum = a[i][j];
            for k in 0..j {
                sum -= l[i][k] * l[j][k];
            }
            if i == j {
                if sum <= 1e-14 {
                    return None;
                }
                l[i][j] = sum.sqrt();
            } else {
                l[i][j] = sum / l[j][j];
            }
        }
    }
    // Forward substitution: L z = b
    let mut z = vec![0.0; d];
    for i in 0..d {
        let mut sum = b[i];
        for k in 0..i {
            sum -= l[i][k] * z[k];
        }
        z[i] = sum / l[i][i];
    }
    // Backward substitution: L^T x = z
    let mut x = vec![0.0; d];
    for i in (0..d).rev() {
        let mut sum = z[i];
        for k in (i + 1)..d {
            sum -= l[k][i] * x[k];
        }
        x[i] = sum / l[i][i];
    }
    Some(x)
}

/// Ordinary / ridge least-squares estimator $\hat{\beta} = (X^\top X + \lambda I)^{-1} X^\top y$.
#[allow(clippy::needless_range_loop)]
pub fn ols_ridge_fit(x: &[Vec<f64>], y: &[f64], ridge_lambda: f64) -> Option<Vec<f64>> {
    let n = x.len();
    if n == 0 || n != y.len() {
        return None;
    }
    let d = x[0].len();
    if d == 0 {
        return None;
    }
    let mut xtx = vec![vec![0.0; d]; d];
    let mut xty = vec![0.0; d];
    for (row, &yi) in x.iter().zip(y.iter()) {
        if row.len() != d {
            return None;
        }
        for j in 0..d {
            xty[j] += row[j] * yi;
            for k in 0..=j {
                xtx[j][k] += row[j] * row[k];
            }
        }
    }
    let reg = ridge_lambda.max(1e-10);
    for j in 0..d {
        xtx[j][j] += reg;
        for k in (j + 1)..d {
            xtx[j][k] = xtx[k][j];
        }
    }
    solve_cholesky_spd(&xtx, &xty)
}

/// Computes average-tie ranks of `xs` (1-based) and returns Spearman's rank correlation $\rho_s \in [-1, 1]$.
pub fn spearman_rank_correlation(x: &[f64], y: &[f64]) -> f64 {
    assert_eq!(x.len(), y.len(), "spearman requires equal-length slices");
    let n = x.len();
    if n < 2 {
        return 0.0;
    }
    let ranks = |vals: &[f64]| -> Vec<f64> {
        let mut idx: Vec<usize> = (0..n).collect();
        idx.sort_by(|&i, &j| vals[i].total_cmp(&vals[j]));
        let mut r = vec![0.0; n];
        let mut i = 0;
        while i < n {
            let mut j = i + 1;
            while j < n && (vals[idx[j]] - vals[idx[i]]).abs() <= 1e-12 {
                j += 1;
            }
            let avg_rank = 0.5 * ((i + 1 + j) as f64);
            for k in i..j {
                r[idx[k]] = avg_rank;
            }
            i = j;
        }
        r
    };
    let rx = ranks(x);
    let ry = ranks(y);
    let mean_r = 0.5 * ((n + 1) as f64);
    let mut num = 0.0;
    let mut dx2 = 0.0;
    let mut dy2 = 0.0;
    for (&a, &b) in rx.iter().zip(ry.iter()) {
        let da = a - mean_r;
        let db = b - mean_r;
        num += da * db;
        dx2 += da * da;
        dy2 += db * db;
    }
    let denom = (dx2 * dy2).sqrt();
    if denom <= 1e-15 { 0.0 } else { num / denom }
}

/// Two-sample Kolmogorov-Smirnov supremum distance $D_{\mathrm{KS}} = \sup_t |\hat{F}_1(t) - \hat{F}_2(t)|$.
pub fn kolmogorov_smirnov_distance(a: &[f64], b: &[f64]) -> f64 {
    if a.is_empty() || b.is_empty() {
        return 0.0;
    }
    let mut sa = a.to_vec();
    let mut sb = b.to_vec();
    sa.sort_by(|x, y| x.total_cmp(y));
    sb.sort_by(|x, y| x.total_cmp(y));
    let na = sa.len() as f64;
    let nb = sb.len() as f64;
    let mut ia = 0usize;
    let mut ib = 0usize;
    let mut max_d = 0.0_f64;
    while ia < sa.len() && ib < sb.len() {
        let v = sa[ia].min(sb[ib]);
        while ia < sa.len() && sa[ia] <= v {
            ia += 1;
        }
        while ib < sb.len() && sb[ib] <= v {
            ib += 1;
        }
        let d = ((ia as f64) / na - (ib as f64) / nb).abs();
        if d > max_d {
            max_d = d;
        }
    }
    max_d
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pav_isotonic_enforces_monotonicity_and_preserves_mean() {
        let y = [0.8, 0.2, 0.5, 0.4, 0.9];
        let iso = pav_isotonic_non_decreasing(&y);
        for w in iso.windows(2) {
            assert!(w[0] <= w[1] + 1e-12);
        }
        let sum_raw: f64 = y.iter().sum();
        let sum_iso: f64 = iso.iter().sum();
        assert!((sum_raw - sum_iso).abs() < 1e-12);
    }

    #[test]
    fn ols_ridge_recovers_exact_linear_relation() {
        let x = vec![
            vec![1.0, 1.0],
            vec![1.0, 2.0],
            vec![1.0, 3.0],
            vec![1.0, 4.0],
        ];
        let y = vec![5.0, 7.0, 9.0, 11.0]; // y = 3 + 2x
        let beta = ols_ridge_fit(&x, &y, 1e-12).expect("OLS fit");
        assert!((beta[0] - 3.0).abs() < 1e-7);
        assert!((beta[1] - 2.0).abs() < 1e-7);
    }

    #[test]
    fn normal_cdf_and_spearman_behave_as_expected() {
        assert!((standard_normal_cdf(0.0) - 0.5).abs() < 1e-7);
        assert!((standard_normal_cdf(1.95996) - 0.975).abs() < 1e-4);
        let a = [1.0, 2.0, 3.0, 4.0];
        let b = [10.0, 20.0, 30.0, 40.0];
        assert!((spearman_rank_correlation(&a, &b) - 1.0).abs() < 1e-12);
    }
}
