//! Pure-Rust dense linear algebra and Ordinary Least Squares primitives (`stochology_core::math::linalg`).

use crate::rng::mean;
pub use crate::uncertainty::{
    cholesky_decompose, cholesky_log_det, cholesky_solve, solve_spd_system,
};

/// Euclidean $L_2$ norm $\|v\|_2 = \sqrt{\sum_i v_i^2}$.
#[inline]
pub fn l2_norm(v: &[f64]) -> f64 {
    v.iter().map(|&x| x * x).sum::<f64>().sqrt()
}

/// Inner product $\langle a, b \rangle = \sum_i a_i b_i$.
#[inline]
pub fn dot_product(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b.iter()).map(|(&x, &y)| x * y).sum()
}

/// Matrix-vector product $y = A x$.
pub fn mat_vec_mul(a: &[Vec<f64>], x: &[f64]) -> Vec<f64> {
    a.iter().map(|row| dot_product(row, x)).collect()
}

/// Dense matrix-matrix product $C = A B$.
pub fn mat_mul(a: &[Vec<f64>], b: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let m = a.len();
    if m == 0 || b.is_empty() {
        return Vec::new();
    }
    let k_dim = b.len();
    let n = b[0].len();
    let mut out = vec![vec![0.0; n]; m];
    for i in 0..m {
        for p in 0..k_dim {
            let a_ip = a[i][p];
            for j in 0..n {
                out[i][j] += a_ip * b[p][j];
            }
        }
    }
    out
}

/// Solves $A x = b$ via Gauss-Jordan elimination with partial pivoting.
/// Returns `None` if the system is singular within `1e-14`.
pub fn solve_linear_system(a: &[Vec<f64>], b: &[f64]) -> Option<Vec<f64>> {
    let n = b.len();
    if n == 0 || a.len() != n {
        return None;
    }
    let mut aug: Vec<Vec<f64>> = a
        .iter()
        .zip(b.iter())
        .map(|(row, &bi)| {
            let mut r = row.clone();
            r.push(bi);
            r
        })
        .collect();

    for col in 0..n {
        let mut pivot = col;
        let mut max_val = aug[col][col].abs();
        for (r, aug_row) in aug.iter().enumerate().skip(col + 1) {
            if aug_row[col].abs() > max_val {
                max_val = aug_row[col].abs();
                pivot = r;
            }
        }
        if max_val < 1e-14 {
            return None;
        }
        aug.swap(col, pivot);
        let diag = aug[col][col];
        for val in &mut aug[col][col..=n] {
            *val /= diag;
        }
        let pivot_slice = aug[col][col..=n].to_vec();
        for (r, row) in aug.iter_mut().enumerate() {
            if r != col {
                let factor = row[col];
                for (val, &piv_val) in row[col..=n].iter_mut().zip(pivot_slice.iter()) {
                    *val -= factor * piv_val;
                }
            }
        }
    }
    Some(aug.into_iter().map(|r| r[n]).collect())
}

/// Inverts an $n \times n$ matrix $A$ via Gauss-Jordan elimination with partial pivoting.
pub fn invert_matrix(a: &[Vec<f64>]) -> Option<Vec<Vec<f64>>> {
    let n = a.len();
    if n == 0 || a.iter().any(|row| row.len() != n) {
        return None;
    }
    let mut aug = vec![vec![0.0; 2 * n]; n];
    for i in 0..n {
        aug[i][..n].copy_from_slice(&a[i][..n]);
        aug[i][n + i] = 1.0;
    }
    for col in 0..n {
        let mut pivot = col;
        let mut max_v = aug[col][col].abs();
        for (r, row) in aug.iter().enumerate().skip(col + 1) {
            if row[col].abs() > max_v {
                max_v = row[col].abs();
                pivot = r;
            }
        }
        if max_v < 1e-13 {
            return None;
        }
        aug.swap(col, pivot);
        let d = aug[col][col];
        for val in &mut aug[col][col..(2 * n)] {
            *val /= d;
        }
        let pivot_slice = aug[col][col..(2 * n)].to_vec();
        for (r, row) in aug.iter_mut().enumerate() {
            if r != col {
                let f = row[col];
                for (val, &piv_val) in row[col..(2 * n)].iter_mut().zip(pivot_slice.iter()) {
                    *val -= f * piv_val;
                }
            }
        }
    }
    Some(aug.into_iter().map(|r| r[n..].to_vec()).collect())
}

/// Univariate Ordinary Least Squares slope $\hat{\beta}_1 = \frac{\sum (x_i - \bar{x})(y_i - \bar{y})}{\sum (x_i - \bar{x})^2}$.
pub fn ols_slope(x: &[f64], y: &[f64]) -> f64 {
    if x.len() != y.len() || x.len() < 2 {
        return 0.0;
    }
    let mx = mean(x);
    let my = mean(y);
    let mut num = 0.0;
    let mut den = 0.0;
    for (&xi, &yi) in x.iter().zip(y.iter()) {
        num += (xi - mx) * (yi - my);
        den += (xi - mx).powi(2);
    }
    num / den.max(1e-12)
}

/// Univariate Ordinary Least Squares fit returning `(intercept, slope, r_squared)`.
pub fn univariate_ols_fit(x: &[f64], y: &[f64]) -> (f64, f64, f64) {
    if x.len() != y.len() || x.len() < 2 {
        return (0.0, 0.0, 0.0);
    }
    let mx = mean(x);
    let my = mean(y);
    let slope = ols_slope(x, y);
    let intercept = my - slope * mx;
    let mut ss_res = 0.0;
    let mut ss_tot = 0.0;
    for (&xi, &yi) in x.iter().zip(y.iter()) {
        let pred = intercept + slope * xi;
        ss_res += (yi - pred).powi(2);
        ss_tot += (yi - my).powi(2);
    }
    let r2 = if ss_tot > 1e-15 {
        (1.0 - ss_res / ss_tot).clamp(0.0, 1.0)
    } else {
        0.0
    };
    (intercept, slope, r2)
}
