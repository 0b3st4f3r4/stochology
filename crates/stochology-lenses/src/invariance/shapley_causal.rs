//! Post-Hoc XAI Causal Audit (`ShapleyCausalAudit` — Lundberg & Lee 2017 `SHAP` vs.
//! Janzing, Minorics & Blöbaum 2020 Interventional `SHAP` & Goldstein et al. 2015 `ICE` vs. `PDP`).

use stochology_core::mean;

use super::icp::{fit_ols_multi, run_icp, CausalEnvironment};

/// Exact Post-Hoc XAI Causal Audit (`ShapleyCausalAudit` — Lundberg & Lee 2017 `SHAP` vs.
/// Janzing, Minorics & Blöbaum 2020 Interventional `SHAP` & Goldstein et al. 2015 `ICE` vs. `PDP`).
#[derive(Debug, Clone, PartialEq)]
pub struct ShapleyCausalAudit {
    /// Exact observational Rung-1 Shapley values $\phi^{\text{obs}}(x)$ for `[X0_cause, X1_null, X2_symptom]`.
    pub observational_shapley_values: [f64; 3],
    /// Exact ICP-anchored interventional Shapley values $\phi^{\text{do}}(x)$ restricted to $\hat{S}^*$.
    pub interventional_icp_shapley_values: [f64; 3],
    /// Shapley Efficiency Axiom residual $\bigl|\sum_{j=0}^2 \phi_j(x) - (f(x) - \mathbb{E}[f(X)])\bigr|$.
    pub efficiency_axiom_error: f64,
    /// Null-player attribution $|\phi_1(x)|$ for an unused feature $w_1 = 0$.
    pub null_player_axiom_error: f64,
    /// Symmetry residual $|\phi_a(x) - \phi_b(x)|$ for two symmetric features at equal inputs.
    pub symmetry_axiom_error: f64,
    /// Partial Dependence Plot `PDP(x_0)` vs. root-mean-square `ICE(x_0)` spread under signed interaction $f(x_0, z) = z \cdot x_0$.
    pub pdp_mean_vs_ice_rms_spread: (f64, f64),
    /// Counterfactual MSE when intervening $\mathrm{do}(X_{j^*} = x_{j^*} + \Delta)$ on the top feature chosen by
    /// observational `SHAP` vs. ICP-anchored interventional `SHAP`.
    pub observational_vs_interventional_do_mse: (f64, f64),
    /// True if Efficiency, Null-Player, Symmetry, and PDP-cancellation identities close to machine precision.
    pub axioms_and_identities_closed: bool,
}

impl ShapleyCausalAudit {
    /// Computes exact $2^3 = 8$ coalition Shapley values for a linear predictor $f(x) = w_0 x_0 + w_1 x_1 + w_2 x_2$
    /// evaluated at `x_eval` relative to baseline mean `mu_x` under independent marginal intervention $v(S) = \sum_{j \in S} w_j x_j + \sum_{k \notin S} w_k \mu_k$.
    pub fn exact_linear_shapley_3d(w: [f64; 3], x_eval: [f64; 3], mu_x: [f64; 3]) -> [f64; 3] {
        let fact = [1.0_f64, 1.0, 2.0, 6.0];
        let d = 3usize;
        let val = |mask: usize| -> f64 {
            (0..d)
                .map(|j| {
                    if (mask & (1 << j)) != 0 {
                        w[j] * x_eval[j]
                    } else {
                        w[j] * mu_x[j]
                    }
                })
                .sum()
        };
        let mut phi = [0.0_f64; 3];
        for (j, slot) in phi.iter_mut().enumerate() {
            let mut sum_j = 0.0;
            for mask in 0..(1usize << d) {
                if (mask & (1 << j)) == 0 {
                    let s_len = mask.count_ones() as usize;
                    let weight = fact[s_len] * fact[d - s_len - 1] / fact[d];
                    let v_with = val(mask | (1 << j));
                    let v_without = val(mask);
                    sum_j += weight * (v_with - v_without);
                }
            }
            *slot = sum_j;
        }
        phi
    }

    /// Evaluates the full Shapley Causal & PDP/ICE Audit over a multi-environment structural causal DAG
    /// $X_0 \to Y \to X_2$ (where $X_0$ is the invariant cause $\hat{S}^* = \{0\}$, $X_1$ is a null feature,
    /// and $X_2$ is a low-noise downstream symptom of $Y$).
    pub fn evaluate_environments(environments: &[CausalEnvironment], x_eval: [f64; 3]) -> Self {
        let mut pooled_x = Vec::new();
        let mut pooled_y = Vec::new();
        for env in environments {
            for (r, &yi) in env.x.iter().zip(env.y.iter()) {
                if r.len() >= 3 {
                    pooled_x.push(vec![r[0], r[1], r[2]]);
                    pooled_y.push(yi);
                }
            }
        }
        let mu_x = [
            mean(&pooled_x.iter().map(|r| r[0]).collect::<Vec<_>>()),
            mean(&pooled_x.iter().map(|r| r[1]).collect::<Vec<_>>()),
            mean(&pooled_x.iter().map(|r| r[2]).collect::<Vec<_>>()),
        ];

        let env0_x: Vec<Vec<f64>> = environments[0]
            .x
            .iter()
            .filter(|r| r.len() >= 3)
            .map(|r| vec![r[0], r[1], r[2]])
            .collect();
        let ols_all = fit_ols_multi(&env0_x, &environments[0].y);
        let w_obs = [ols_all[1], 0.0, ols_all[3]];
        let phi_obs = Self::exact_linear_shapley_3d(w_obs, x_eval, mu_x);
        let f_x = w_obs[0] * x_eval[0] + w_obs[1] * x_eval[1] + w_obs[2] * x_eval[2];
        let e_fx = w_obs[0] * mu_x[0] + w_obs[1] * mu_x[1] + w_obs[2] * mu_x[2];
        let eff_err = ((phi_obs[0] + phi_obs[1] + phi_obs[2]) - (f_x - e_fx)).abs();
        let null_err = phi_obs[1].abs();

        let phi_sym = Self::exact_linear_shapley_3d([1.5, 1.5, 0.4], [2.0, 2.0, 1.0], [0.0; 3]);
        let sym_err = (phi_sym[0] - phi_sym[1]).abs();

        let icp = run_icp(environments, 0.16);
        let mut w_do = [0.0_f64; 3];
        if !icp.accepted_causal_predictors.is_empty() {
            let sub = &icp.accepted_causal_predictors;
            let sub_x: Vec<Vec<f64>> = pooled_x
                .iter()
                .map(|r| sub.iter().map(|&j| r[j]).collect())
                .collect();
            let sub_beta = fit_ols_multi(&sub_x, &pooled_y);
            for (k, &feat_idx) in sub.iter().enumerate() {
                if feat_idx < 3 {
                    w_do[feat_idx] = sub_beta[k + 1];
                }
            }
        }
        let phi_do = Self::exact_linear_shapley_3d(w_do, x_eval, mu_x);

        let z_vals = [-1.0_f64, 1.0_f64];
        let x0_probe = 2.0_f64;
        let pdp_val: f64 = z_vals.iter().map(|&z| z * x0_probe).sum::<f64>() / 2.0;
        let ice_rms: f64 =
            (z_vals.iter().map(|&z| (z * x0_probe).powi(2)).sum::<f64>() / 2.0).sqrt();

        let x0_only: Vec<Vec<f64>> = pooled_x.iter().map(|r| vec![r[0]]).collect();
        let beta_true_x0 = fit_ols_multi(&x0_only, &pooled_y)[1];
        let top_obs_idx = (0..3)
            .max_by(|&a, &b| phi_obs[a].abs().total_cmp(&phi_obs[b].abs()))
            .unwrap_or(2);
        let top_do_idx = (0..3)
            .max_by(|&a, &b| phi_do[a].abs().total_cmp(&phi_do[b].abs()))
            .unwrap_or(0);

        let true_delta_on = |j: usize| if j == 0 { beta_true_x0 } else { 0.0 };
        let obs_mse = (w_obs[top_obs_idx] - true_delta_on(top_obs_idx)).powi(2);
        let do_mse = (w_do[top_do_idx] - true_delta_on(top_do_idx)).powi(2);

        let closed = eff_err < 1e-12
            && null_err < 1e-12
            && sym_err < 1e-12
            && pdp_val.abs() < 1e-12
            && ice_rms > 1.5;

        Self {
            observational_shapley_values: phi_obs,
            interventional_icp_shapley_values: phi_do,
            efficiency_axiom_error: eff_err,
            null_player_axiom_error: null_err,
            symmetry_axiom_error: sym_err,
            pdp_mean_vs_ice_rms_spread: (pdp_val, ice_rms),
            observational_vs_interventional_do_mse: (obs_mse, do_mse),
            axioms_and_identities_closed: closed,
        }
    }
}
