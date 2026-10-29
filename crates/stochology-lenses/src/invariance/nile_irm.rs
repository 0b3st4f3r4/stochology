//! Nonlinear Invariant Causal Prediction and Nonlinear NILE/IRM Risk-Gradient Penalty
//! over the 3-atom orthonormal Hermite basis $\Phi(X)$.

use stochology_core::{mean, sample_std, sample_variance};

use super::icp::{eval_basis_model, fit_ols_multi, run_icp, CausalEnvironment};

/// Output of Nonlinear Invariant Causal Prediction and Nonlinear NILE/IRM over representation $\Phi(X)$.
#[derive(Debug, Clone, PartialEq)]
pub struct NonlinearIcpNileResult {
    /// Subsets $S \subseteq \{0, \dots, p-1\}$ whose nonlinear conditional expectations $\mathbb{E}_e[Y \mid \Phi(X_S)]$ and residual distributions are invariant across $e \in \mathcal{E}$.
    pub invariant_subsets: Vec<Vec<usize>>,
    /// Structural intersection accepted over $\Phi$: $\hat{S}^* = \bigcap_{S : \text{invariant}} S$.
    pub accepted_causal_predictors: Vec<usize>,
    /// Linear baseline (`env_ols_invariance_penalties`) preserved for direct comparison.
    pub linear_ols_baseline_penalties: Vec<f64>,
    /// True nonlinear NILE + IRM-v1 risk-gradient penalty over representation $\Phi(X_j)$ per candidate variable $j$.
    pub nonlinear_nile_irm_penalties: Vec<f64>,
    /// Maximum environment deviation of the accepted subset $\hat{S}^*$ on representation $\Phi$.
    pub max_accepted_env_deviation: f64,
}

pub(crate) fn expand_nonlinear_basis_1d(z: f64) -> [f64; 3] {
    let zc = z.clamp(-6.0, 6.0);
    [
        zc,
        (zc * zc - 1.0) * std::f64::consts::FRAC_1_SQRT_2,
        (zc * zc * zc - 3.0 * zc) / 6.0_f64.sqrt(),
    ]
}

fn expand_nonlinear_subset_row(row: &[f64], subset: &[usize], sigma_x: &[f64]) -> Vec<f64> {
    let mut phi = Vec::with_capacity(subset.len() * 3);
    for (&idx, &sx) in subset.iter().zip(sigma_x.iter()) {
        let z = row[idx] / sx;
        phi.extend_from_slice(&expand_nonlinear_basis_1d(z));
    }
    phi
}

/// Executes Nonlinear Invariant Causal Prediction and computes the Nonlinear NILE + IRM-v1
/// risk-gradient penalty over the 3-atom orthonormal Hermite basis
/// $\Phi(z) = \bigl[z,\ \frac{z^2-1}{\sqrt{2}},\ \frac{z^3-3z}{\sqrt{6}}\bigr]$
/// alongside the linear OLS baseline.
pub fn run_nonlinear_icp_nile(
    environments: &[CausalEnvironment],
    invariance_tolerance: f64,
) -> NonlinearIcpNileResult {
    let linear_base = run_icp(environments, invariance_tolerance);
    let p = environments[0].x[0].len();

    let mut nonlinear_penalties = vec![0.0; p];
    for (j, slot) in nonlinear_penalties.iter_mut().enumerate() {
        let mut raw_pooled_xj = Vec::new();
        let mut pooled_y = Vec::new();
        for env in environments {
            for (row, &yi) in env.x.iter().zip(env.y.iter()) {
                raw_pooled_xj.push(row[j]);
                pooled_y.push(yi);
            }
        }
        let mean_xj = mean(&raw_pooled_xj);
        let sd_xj = sample_std(&raw_pooled_xj).max(1e-12);
        let mean_y = mean(&pooled_y);
        let sigma_y = sample_std(&pooled_y).max(1e-12);
        let pooled_phi: Vec<Vec<f64>> = raw_pooled_xj
            .iter()
            .map(|&x| expand_nonlinear_basis_1d((x - mean_xj) / sd_xj).to_vec())
            .collect();
        let pooled_y_norm: Vec<f64> = pooled_y.iter().map(|&y| (y - mean_y) / sigma_y).collect();
        let theta_pool = fit_ols_multi(&pooled_phi, &pooled_y_norm);

        let mut nile_func_shift = 0.0;
        let mut irm_grad_penalty = 0.0;
        let mut env_pooled_risks = Vec::with_capacity(environments.len());

        for env in environments {
            let env_phi: Vec<Vec<f64>> = env
                .x
                .iter()
                .map(|r| expand_nonlinear_basis_1d((r[j] - mean_xj) / sd_xj).to_vec())
                .collect();
            let env_y_norm: Vec<f64> = env.y.iter().map(|&y| (y - mean_y) / sigma_y).collect();
            let theta_env = fit_ols_multi(&env_phi, &env_y_norm);

            let n_e = env_phi.len() as f64;
            let mut func_diff_sq = 0.0;
            let mut irm_grad_w1 = 0.0;
            let mut risk_pool_e = 0.0;

            for (phi_i, &yi) in env_phi.iter().zip(env_y_norm.iter()) {
                let f_pool = eval_basis_model(&theta_pool, phi_i);
                let f_env = eval_basis_model(&theta_env, phi_i);
                let diff = f_env - f_pool;
                func_diff_sq += diff * diff;
                let err_pool = yi - f_pool;
                irm_grad_w1 += -2.0 * err_pool * f_pool;
                risk_pool_e += err_pool * err_pool;
            }
            nile_func_shift += func_diff_sq / n_e;
            let g1 = irm_grad_w1 / n_e;
            irm_grad_penalty += g1 * g1;
            env_pooled_risks.push(risk_pool_e / n_e);
        }

        let k_env = environments.len() as f64;
        *slot = (nile_func_shift / k_env)
            + (irm_grad_penalty / k_env)
            + sample_variance(&env_pooled_risks);
    }

    let mut invariant_subsets = Vec::new();
    let total_masks = 1usize << p;
    for mask in 0..total_masks {
        let subset: Vec<usize> = (0..p).filter(|&bit| (mask & (1 << bit)) != 0).collect();
        let (inv, _) = is_nonlinear_subset_invariant(environments, &subset, invariance_tolerance);
        if inv {
            invariant_subsets.push(subset);
        }
    }

    let accepted = if invariant_subsets.is_empty() {
        Vec::new()
    } else {
        let mut inter = invariant_subsets[0].clone();
        for s in &invariant_subsets[1..] {
            inter.retain(|item| s.contains(item));
        }
        inter
    };

    let max_accepted_env_deviation = if accepted.is_empty() {
        1.0
    } else {
        let (_, dev) = is_nonlinear_subset_invariant(environments, &accepted, invariance_tolerance);
        dev
    };

    NonlinearIcpNileResult {
        invariant_subsets,
        accepted_causal_predictors: accepted,
        linear_ols_baseline_penalties: linear_base.env_ols_invariance_penalties,
        nonlinear_nile_irm_penalties: nonlinear_penalties,
        max_accepted_env_deviation,
    }
}

fn is_nonlinear_subset_invariant(
    environments: &[CausalEnvironment],
    subset: &[usize],
    tol: f64,
) -> (bool, f64) {
    let mut pooled_x_raw = Vec::new();
    let mut pooled_y = Vec::new();
    for env in environments {
        for (row, &yi) in env.x.iter().zip(env.y.iter()) {
            pooled_x_raw.push(row.clone());
            pooled_y.push(yi);
        }
    }

    let sigma_y = sample_std(&pooled_y).max(1e-12);
    let mut sigma_x = Vec::with_capacity(subset.len());
    for &idx in subset {
        let col_vals: Vec<f64> = pooled_x_raw.iter().map(|r| r[idx]).collect();
        sigma_x.push(sample_std(&col_vals).max(1e-12));
    }

    let pooled_phi: Vec<Vec<f64>> = pooled_x_raw
        .iter()
        .map(|r| expand_nonlinear_subset_row(r, subset, &sigma_x))
        .collect();
    let coeffs = fit_ols_multi(&pooled_phi, &pooled_y);

    let pooled_residuals: Vec<f64> = pooled_phi
        .iter()
        .zip(pooled_y.iter())
        .map(|(phi_r, &yi)| yi - eval_basis_model(&coeffs, phi_r))
        .collect();
    let sigma_res_pool = sample_std(&pooled_residuals).max(1e-12);

    let mut env_means = Vec::with_capacity(environments.len());
    let mut env_sds = Vec::with_capacity(environments.len());
    let mut max_func_dev = 0.0_f64;

    for env in environments {
        let env_phi: Vec<Vec<f64>> = env
            .x
            .iter()
            .map(|r| expand_nonlinear_subset_row(r, subset, &sigma_x))
            .collect();
        let env_coeffs = fit_ols_multi(&env_phi, &env.y);

        let f_rms = (env_phi
            .iter()
            .map(|phi_r| {
                let d = eval_basis_model(&coeffs, phi_r) - eval_basis_model(&env_coeffs, phi_r);
                d * d
            })
            .sum::<f64>()
            / (env_phi.len() as f64))
            .sqrt()
            / sigma_y;
        max_func_dev = max_func_dev.max(f_rms);

        let residuals: Vec<f64> = env_phi
            .iter()
            .zip(env.y.iter())
            .map(|(phi_r, &yi)| yi - eval_basis_model(&coeffs, phi_r))
            .collect();
        env_means.push(0.5 * mean(&residuals).abs() / sigma_res_pool);
        env_sds.push(0.5 * sample_std(&residuals) / sigma_res_pool);
    }

    let max_mean = env_means.into_iter().fold(0.0_f64, f64::max);
    let min_sd = env_sds.iter().copied().fold(f64::INFINITY, f64::min);
    let max_sd = env_sds.into_iter().fold(0.0_f64, f64::max);
    let sd_spread = max_sd - min_sd;
    let worst_dev = max_func_dev.max(max_mean).max(sd_spread);

    (
        max_func_dev <= tol && max_mean <= tol && sd_spread <= tol,
        worst_dev,
    )
}
