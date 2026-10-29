//! Shared mathematical, statistical, linear-algebraic, and complex-number kernel (`stochology_core::math`).

pub mod complex;
pub mod linalg;
pub mod stats;

pub use complex::Complex64;
pub use linalg::{
    cholesky_decompose, cholesky_log_det, cholesky_solve, dot_product, invert_matrix, l2_norm,
    mat_mul, mat_vec_mul, ols_slope, solve_linear_system, solve_spd_system, univariate_ols_fit,
};
pub use stats::{
    average_ranks, empirical_wasserstein_1, empirical_wasserstein_1_grid, mean,
    pearson_correlation, quantile, sample_std, sample_variance, spearman_rank_correlation,
    two_sample_ks_distance, OnlineMoments,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complex64_algebra_and_euler_identities() {
        let z1 = Complex64::new(3.0, 4.0);
        let z2 = Complex64::new(1.0, -2.0);
        assert!((z1.abs() - 5.0).abs() < 1e-12);
        assert!(((z1 * z1.conj()).re - z1.norm_sq()).abs() < 1e-12);
        assert!(((z1 * z2) / z2 - z1).abs() < 1e-12);

        let u = Complex64::exp_i(0.73);
        assert!((u.norm_sq() - 1.0).abs() < 1e-14);
        assert!((u.conj_mul(u) - Complex64::ONE).abs() < 1e-14);
    }

    #[test]
    fn stats_and_linalg_helpers_close_identities() {
        let x = [1.0, 2.0, 2.0, 4.0, 5.0];
        let y = [2.0, 4.0, 4.0, 8.0, 10.0];
        assert!((spearman_rank_correlation(&x, &y) - 1.0).abs() < 1e-12);
        assert!(two_sample_ks_distance(&x, &x).abs() < 1e-12);
        assert!(empirical_wasserstein_1_grid(&x, &x, 200).abs() < 1e-12);

        let a = vec![vec![4.0, 1.0], vec![1.0, 3.0]];
        let b = vec![1.0, 2.0];
        let sol = solve_linear_system(&a, &b).expect("invertible");
        let ax = mat_vec_mul(&a, &sol);
        assert!((ax[0] - b[0]).abs() < 1e-12 && (ax[1] - b[1]).abs() < 1e-12);

        let inv = invert_matrix(&a).expect("invertible");
        let prod = mat_mul(&a, &inv);
        assert!((prod[0][0] - 1.0).abs() < 1e-12 && prod[0][1].abs() < 1e-12);
    }
}
