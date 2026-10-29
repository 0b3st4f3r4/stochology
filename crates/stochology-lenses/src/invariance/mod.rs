//! Lens 4 — Structure and Invariance: Pearl's Causal Ladder, Invariant Causal Prediction (ICP),
//! Environment-Projected OLS Invariance Penalty, Nonlinear NILE/IRM, and Hidden Confounding Threshold $\Lambda^*$ (§1.3.4).

pub mod icp;
pub mod nile_irm;
pub mod shapley_causal;

pub use icp::{
    chow_invariance_f_stat, cinelli_hazlett_robustness_value,
    hidden_confounding_threshold_lambda_star, run_icp, second_order_worst_case_loss,
    second_order_worst_case_loss_full, CausalEnvironment, IcpResult, PearlRung,
};
pub(crate) use nile_irm::expand_nonlinear_basis_1d;
pub use nile_irm::{run_nonlinear_icp_nile, NonlinearIcpNileResult};
pub use shapley_causal::ShapleyCausalAudit;

#[cfg(test)]
mod tests {
    use super::*;
    use stochology_core::Rng;

    #[test]
    fn icp_isolates_true_structural_cause_and_rejects_spurious_child() {
        let mut rng = Rng::new(2026);
        let mut envs = Vec::new();
        for (idx, (scale_x0, spurious_weight)) in [(1.0, 1.2), (2.2, -1.4), (0.7, 2.8)]
            .into_iter()
            .enumerate()
        {
            let mut x = Vec::with_capacity(300);
            let mut y = Vec::with_capacity(300);
            for _ in 0..300 {
                let x0 = rng.normal(0.0, scale_x0);
                let yi = 1.75 * x0 + rng.normal(0.0, 0.12);
                let x1 = spurious_weight * yi + rng.normal(0.0, 0.35);
                x.push(vec![x0, x1]);
                y.push(yi);
            }
            envs.push(CausalEnvironment {
                name: format!("env_{idx}"),
                x,
                y,
            });
        }

        let res = run_icp(&envs, 0.12);
        assert_eq!(res.accepted_causal_predictors, vec![0]);
        assert!(res.env_ols_invariance_penalties[0] < res.env_ols_invariance_penalties[1] * 0.05);

        let f_true = chow_invariance_f_stat(&envs, &[0]);
        let f_spurious = chow_invariance_f_stat(&envs, &[1]);
        assert!(f_true < 3.0 && f_spurious > 50.0);
        let rv = cinelli_hazlett_robustness_value(12.0, 890, 1.0);
        assert!(rv > 0.25 && rv < 1.0);
    }

    #[test]
    fn icp_is_scale_invariant_and_returns_empty_set_under_null_environment_shift() {
        let mut rng = Rng::new(2026);
        let mut envs = Vec::new();
        let mut envs_scaled = Vec::new();
        let mut envs_y_scaled = Vec::new();
        for (idx, (scale_x0, spurious_weight)) in [(1.0, 1.2), (2.2, -1.4), (0.7, 2.8)]
            .into_iter()
            .enumerate()
        {
            let mut x = Vec::with_capacity(300);
            let mut x_scaled = Vec::with_capacity(300);
            let mut y = Vec::with_capacity(300);
            let mut y_scaled = Vec::with_capacity(300);
            for _ in 0..300 {
                let x0 = rng.normal(0.0, scale_x0);
                let yi = 1.75 * x0 + rng.normal(0.0, 0.12);
                let x1 = spurious_weight * yi + rng.normal(0.0, 0.35);
                x.push(vec![x0, x1]);
                x_scaled.push(vec![x0 * 100.0, x1]);
                y.push(yi);
                y_scaled.push(yi * 1000.0);
            }
            envs.push(CausalEnvironment {
                name: format!("env_{idx}"),
                x: x.clone(),
                y: y.clone(),
            });
            envs_scaled.push(CausalEnvironment {
                name: format!("env_scaled_{idx}"),
                x: x_scaled,
                y,
            });
            envs_y_scaled.push(CausalEnvironment {
                name: format!("env_y_scaled_{idx}"),
                x,
                y: y_scaled,
            });
        }

        let res_orig = run_icp(&envs, 0.12);
        let res_scaled = run_icp(&envs_scaled, 0.12);
        let res_y_scaled = run_icp(&envs_y_scaled, 0.12);
        assert_eq!(res_orig.accepted_causal_predictors, vec![0]);
        assert_eq!(
            res_orig.accepted_causal_predictors,
            res_scaled.accepted_causal_predictors
        );
        assert_eq!(
            res_orig.accepted_causal_predictors,
            res_y_scaled.accepted_causal_predictors
        );
        assert_eq!(res_orig.invariant_subsets, res_scaled.invariant_subsets);
        assert_eq!(res_orig.invariant_subsets, res_y_scaled.invariant_subsets);
        assert!(
            (res_orig.env_ols_invariance_penalties[0] - res_scaled.env_ols_invariance_penalties[0])
                .abs()
                < 1e-9
        );

        let mut null_envs = Vec::new();
        for idx in 0..3 {
            let mut x = Vec::with_capacity(400);
            let mut y = Vec::with_capacity(400);
            for _ in 0..400 {
                x.push(vec![
                    rng.normal(0.0, 1.0 + idx as f64),
                    rng.normal(0.0, 1.0),
                ]);
                y.push(rng.normal(0.0, 1.0));
            }
            null_envs.push(CausalEnvironment {
                name: format!("null_{idx}"),
                x,
                y,
            });
        }
        let res_null = run_icp(&null_envs, 0.12);
        assert!(
            res_null.invariant_subsets.contains(&Vec::new()),
            "intercept-only empty set must be invariant when Y does not shift"
        );
        assert!(
            res_null.accepted_causal_predictors.is_empty(),
            "intersection must be empty when empty set is invariant"
        );
    }

    #[test]
    fn lambda_star_remains_finite_and_sensible_at_zero_null_threshold() {
        let lam_zero = hidden_confounding_threshold_lambda_star(1.75, 0.08, 0.0);
        assert!(
            lam_zero.is_finite() && lam_zero > 2.0 && lam_zero < 1_000.0,
            "expected finite sensible E-value at null_threshold=0.0, got {lam_zero}"
        );
        let lam_nonzero = hidden_confounding_threshold_lambda_star(1.75, 0.08, 0.60);
        assert!(lam_nonzero > 4.0 && lam_nonzero < 6.0);
    }

    #[test]
    fn nonlinear_nile_irm_isolates_nonlinear_structural_cause_over_phi() {
        let mut rng = Rng::new(2026);
        let mut envs = Vec::new();
        for (idx, (scale_x0, spurious_w)) in [(0.8, 1.3), (1.5, -1.5), (1.1, 2.4)]
            .into_iter()
            .enumerate()
        {
            let mut x = Vec::with_capacity(350);
            let mut y = Vec::with_capacity(350);
            for _ in 0..350 {
                let x0 = rng.normal(0.0, scale_x0);
                let yi =
                    1.10 * x0 + 0.75 * (x0 * x0 - 1.0) - 0.15 * x0.powi(3) + rng.normal(0.0, 0.10);
                let x1 = spurious_w * yi + rng.normal(0.0, 0.30);
                x.push(vec![x0, x1]);
                y.push(yi);
            }
            envs.push(CausalEnvironment {
                name: format!("nonlin_env_{idx}"),
                x,
                y,
            });
        }

        let nl = run_nonlinear_icp_nile(&envs, 0.15);
        assert_eq!(nl.accepted_causal_predictors, vec![0]);
        assert!(
            nl.nonlinear_nile_irm_penalties[0] < nl.nonlinear_nile_irm_penalties[1] * 0.05,
            "nonlinear NILE/IRM penalty on true cause X0 ({:.5}) must be < 5% of spurious child X1 ({:.5})",
            nl.nonlinear_nile_irm_penalties[0],
            nl.nonlinear_nile_irm_penalties[1]
        );
        assert!(nl.nonlinear_nile_irm_penalties[0] < nl.linear_ols_baseline_penalties[0]);
    }
}
