//! Pearl's Causal Ladder, Linear Invariant Causal Prediction (`run_icp`),
//! Environment-Projected OLS Invariance Penalty, Chow F-statistic, and
//! Confounding Sensitivity ($\Lambda^*$, Cinelli-Hazlett $RV_q$).

use stochology_core::{mean, sample_std, sample_variance};

/// Rung on Judea Pearl's Causal Ladder (§1.3.4).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PearlRung {
    /// 1st Rung — Passive association $P(y \mid x)$ (*See*).
    SeeAssociation,
    /// 2nd Rung — Active intervention $P(y \mid \mathrm{do}(x))$ (*Act*).
    ActIntervention,
    /// 3rd Rung — Counterfactual imagination $P(y_x \mid x', y')$ (*Imagine*).
    ImagineCounterfactual,
}

impl PearlRung {
    pub const fn code(self) -> &'static str {
        match self {
            Self::SeeAssociation => "1-See",
            Self::ActIntervention => "2-Act",
            Self::ImagineCounterfactual => "3-Imagine",
        }
    }
}

/// Data observed within a historical regime or interventional environment $e \in \mathcal{E}$.
#[derive(Debug, Clone)]
pub struct CausalEnvironment {
    pub name: String,
    /// Candidate predictor matrix: each row is an observation with $p$ variables.
    pub x: Vec<Vec<f64>>,
    /// Outcome vector $Y$.
    pub y: Vec<f64>,
}

/// Output of Invariant Causal Prediction (ICP) and environment-projected OLS invariance penalty across environments $e \in \mathcal{E}$.
#[derive(Debug, Clone, PartialEq)]
pub struct IcpResult {
    /// Subsets $S \subseteq \{0, \dots, p-1\}$ whose slopes and residual distributions remain invariant across all environments.
    pub invariant_subsets: Vec<Vec<usize>>,
    /// Structural intersection accepted by ICP: $\hat{S}^* = \bigcap_{S : \text{invariant}} S$.
    pub accepted_causal_predictors: Vec<usize>,
    /// Environment-projected linear OLS slope and residual-SD invariance penalty per candidate variable.
    pub env_ols_invariance_penalties: Vec<f64>,
}

/// Executes Invariant Causal Prediction (ICP, Peters, Bühlmann & Meinshausen)
/// coupled with the environment-projected linear OLS invariance penalty across the provided environments.
pub fn run_icp(environments: &[CausalEnvironment], invariance_tolerance: f64) -> IcpResult {
    assert!(
        environments.len() >= 2,
        "ICP requires at least 2 distinct environments"
    );
    assert!(
        !environments[0].x.is_empty(),
        "environments must contain observations"
    );
    let p = environments[0].x[0].len();
    assert!(
        (1..=12).contains(&p),
        "dimension p for subset search must be in 1..=12"
    );
    for env in environments {
        assert!(
            env.x.len() >= 2 && env.x.len() == env.y.len(),
            "each environment must have at least 2 observations and matching x and y lengths"
        );
        assert!(
            env.x.iter().all(|row| row.len() == p),
            "all rows across all environments must have uniform feature dimension p"
        );
    }

    let mut env_penalties = vec![0.0; p];
    for (j, score_slot) in env_penalties.iter_mut().enumerate() {
        let mut raw_pooled_xj = Vec::new();
        let mut pooled_y = Vec::new();
        for env in environments {
            for (row, &yi) in env.x.iter().zip(env.y.iter()) {
                raw_pooled_xj.push(row[j]);
                pooled_y.push(yi);
            }
        }
        let sd_xj = sample_std(&raw_pooled_xj).max(1e-12);
        let pooled_xj: Vec<f64> = raw_pooled_xj.iter().map(|&x| x / sd_xj).collect();
        let (beta_pool, alpha_pool, _) = univariate_ols_fit(&pooled_xj, &pooled_y);
        let mut betas = Vec::with_capacity(environments.len());
        let mut res_sds = Vec::with_capacity(environments.len());
        let mut env_proj_penalty = 0.0;
        for env in environments {
            let col_j: Vec<f64> = env.x.iter().map(|row| row[j] / sd_xj).collect();
            let (b, _, sd) = univariate_ols_fit(&col_j, &env.y);
            let mean_res_e: f64 = col_j
                .iter()
                .zip(env.y.iter())
                .map(|(&xi, &yi)| yi - (alpha_pool + beta_pool * xi))
                .sum::<f64>()
                / (col_j.len() as f64);
            let var_x_e = sample_variance(&col_j);
            env_proj_penalty +=
                mean_res_e * mean_res_e + (b - beta_pool) * (b - beta_pool) * var_x_e;
            betas.push(b);
            res_sds.push(sd);
        }
        *score_slot = (env_proj_penalty / (environments.len() as f64))
            + sample_variance(&betas)
            + sample_variance(&res_sds);
    }

    let mut invariant_subsets = Vec::new();
    let total_masks = 1usize << p;

    for mask in 0..total_masks {
        let subset: Vec<usize> = (0..p).filter(|&bit| (mask & (1 << bit)) != 0).collect();
        if is_subset_invariant(environments, &subset, invariance_tolerance) {
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

    IcpResult {
        invariant_subsets,
        accepted_causal_predictors: accepted,
        env_ols_invariance_penalties: env_penalties,
    }
}

fn univariate_ols_fit(x: &[f64], y: &[f64]) -> (f64, f64, f64) {
    let mx = mean(x);
    let my = mean(y);
    let mut num = 0.0;
    let mut den = 0.0;
    for (&xi, &yi) in x.iter().zip(y.iter()) {
        num += (xi - mx) * (yi - my);
        den += (xi - mx) * (xi - mx);
    }
    let beta = if den > 1e-12 { num / den } else { 0.0 };
    let alpha = my - beta * mx;
    let res: Vec<f64> = x
        .iter()
        .zip(y.iter())
        .map(|(&xi, &yi)| yi - (alpha + beta * xi))
        .collect();
    let sd = sample_variance(&res).sqrt();
    (beta, alpha, sd)
}

fn is_subset_invariant(environments: &[CausalEnvironment], subset: &[usize], tol: f64) -> bool {
    let mut pooled_x_raw = Vec::new();
    let mut pooled_y = Vec::new();
    for env in environments {
        for (row, &yi) in env.x.iter().zip(env.y.iter()) {
            let sub_row: Vec<f64> = subset.iter().map(|&idx| row[idx]).collect();
            pooled_x_raw.push(sub_row);
            pooled_y.push(yi);
        }
    }

    let sigma_y = sample_std(&pooled_y).max(1e-12);
    let mut sigma_x = Vec::with_capacity(subset.len());
    for col in 0..subset.len() {
        let col_vals: Vec<f64> = pooled_x_raw.iter().map(|r| r[col]).collect();
        sigma_x.push(sample_std(&col_vals).max(1e-12));
    }

    let pooled_x: Vec<Vec<f64>> = pooled_x_raw
        .iter()
        .map(|r| {
            r.iter()
                .zip(sigma_x.iter())
                .map(|(&v, &sx)| v / sx)
                .collect()
        })
        .collect();
    let coeffs = fit_ols_multi(&pooled_x, &pooled_y);

    let pooled_residuals: Vec<f64> = pooled_x
        .iter()
        .zip(pooled_y.iter())
        .map(|(r, &yi)| {
            let pred: f64 = coeffs[0]
                + r.iter()
                    .zip(coeffs[1..].iter())
                    .map(|(&xj, &bj)| xj * bj)
                    .sum::<f64>();
            yi - pred
        })
        .collect();
    let sigma_res_pool = sample_std(&pooled_residuals).max(1e-12);

    let mut env_means = Vec::with_capacity(environments.len());
    let mut env_sds = Vec::with_capacity(environments.len());
    let mut max_coeff_dev = 0.0_f64;

    for env in environments {
        let env_x: Vec<Vec<f64>> = env
            .x
            .iter()
            .map(|r| {
                subset
                    .iter()
                    .zip(sigma_x.iter())
                    .map(|(&idx, &sx)| r[idx] / sx)
                    .collect()
            })
            .collect();
        let env_coeffs = fit_ols_multi(&env_x, &env.y);
        for (&c_pool, &c_env) in coeffs[1..].iter().zip(env_coeffs[1..].iter()) {
            max_coeff_dev = max_coeff_dev.max((c_pool - c_env).abs() / sigma_y);
        }

        let residuals: Vec<f64> = env_x
            .iter()
            .zip(env.y.iter())
            .map(|(r, &yi)| {
                let pred: f64 = coeffs[0]
                    + r.iter()
                        .zip(coeffs[1..].iter())
                        .map(|(&xj, &bj)| xj * bj)
                        .sum::<f64>();
                yi - pred
            })
            .collect();
        env_means.push(0.5 * mean(&residuals).abs() / sigma_res_pool);
        env_sds.push(0.5 * sample_std(&residuals) / sigma_res_pool);
    }

    let max_mean = env_means.into_iter().fold(0.0_f64, f64::max);
    let min_sd = env_sds.iter().copied().fold(f64::INFINITY, f64::min);
    let max_sd = env_sds.into_iter().fold(0.0_f64, f64::max);

    max_coeff_dev <= tol && max_mean <= tol && (max_sd - min_sd) <= tol
}

pub(crate) fn fit_ols_multi(x: &[Vec<f64>], y: &[f64]) -> Vec<f64> {
    let n = x.len();
    let d = x[0].len() + 1;
    let mut xtx = vec![vec![0.0; d]; d];
    let mut xty = vec![0.0; d];
    for (row, &yi) in x.iter().zip(y.iter()) {
        for i in 0..d {
            let xi = if i == 0 { 1.0 } else { row[i - 1] };
            xty[i] += xi * yi;
            for j in 0..d {
                let xj = if j == 0 { 1.0 } else { row[j - 1] };
                xtx[i][j] += xi * xj;
            }
        }
    }
    for (i, row) in xtx.iter_mut().enumerate() {
        row[i] += 1e-8 * (n as f64);
    }
    let mut aug: Vec<Vec<f64>> = xtx
        .into_iter()
        .enumerate()
        .map(|(i, mut r)| {
            r.push(xty[i]);
            r
        })
        .collect();
    for col in 0..d {
        let mut piv = col;
        for row in (col + 1)..d {
            if aug[row][col].abs() > aug[piv][col].abs() {
                piv = row;
            }
        }
        aug.swap(col, piv);
        let diag = aug[col][col];
        if diag.abs() < 1e-15 {
            continue;
        }
        #[allow(clippy::needless_range_loop)]
        for j in col..=d {
            aug[col][j] /= diag;
        }
        for row in 0..d {
            if row != col {
                let f = aug[row][col];
                #[allow(clippy::needless_range_loop)]
                for j in col..=d {
                    aug[row][j] -= f * aug[col][j];
                }
            }
        }
    }
    aug.into_iter().map(|r| r[d]).collect()
}

pub(crate) fn eval_basis_model(coeffs: &[f64], phi_row: &[f64]) -> f64 {
    coeffs[0]
        + phi_row
            .iter()
            .zip(coeffs[1..].iter())
            .map(|(&p, &c)| p * c)
            .sum::<f64>()
}

/// Second-order Taylor expansion of worst-case interventional loss with diagonal Hessian (§1.3.4):
/// $$L_{\mathrm{wc}}(\theta) \approx L_0 + \nabla_\theta L^\top \theta + \frac{1}{2}\sum_{i=1}^d H_{ii} \theta_i^2$$
pub fn second_order_worst_case_loss(
    base_loss_l0: f64,
    gradient: &[f64],
    hessian_diag: &[f64],
    theta: &[f64],
) -> f64 {
    assert_eq!(gradient.len(), theta.len());
    assert_eq!(hessian_diag.len(), theta.len());
    let lin: f64 = gradient
        .iter()
        .zip(theta.iter())
        .map(|(&g, &t)| g * t)
        .sum();
    let quad: f64 = hessian_diag
        .iter()
        .zip(theta.iter())
        .map(|(&h, &t)| 0.5 * h * t * t)
        .sum();
    base_loss_l0 + lin + quad
}

/// Second-order Taylor expansion of worst-case interventional loss with a full $d \times d$ Hessian matrix (§1.3.4):
/// $$L_{\mathrm{wc}}(\theta) \approx L_0 + \nabla_\theta L^\top \theta + \frac{1}{2}\theta^\top H_\theta \theta$$
pub fn second_order_worst_case_loss_full(
    base_loss_l0: f64,
    gradient: &[f64],
    hessian_matrix: &[Vec<f64>],
    theta: &[f64],
) -> f64 {
    let d = theta.len();
    assert_eq!(gradient.len(), d);
    assert_eq!(hessian_matrix.len(), d);
    assert!(hessian_matrix.iter().all(|row| row.len() == d));
    let lin: f64 = gradient
        .iter()
        .zip(theta.iter())
        .map(|(&g, &t)| g * t)
        .sum();
    let mut quad = 0.0;
    for i in 0..d {
        for j in 0..d {
            quad += 0.5 * theta[i] * hessian_matrix[i][j] * theta[j];
        }
    }
    base_loss_l0 + lin + quad
}

/// Critical Hidden Confounding Sensitivity Threshold $\Lambda^* \ge 1$ (§1.3.4):
/// Computes the minimum odds-ratio / confounding strength $\Lambda^*$ required to nullify
/// an observed causal effect `estimated_effect` relative to critical decision bound `null_threshold`.
pub fn hidden_confounding_threshold_lambda_star(
    estimated_effect: f64,
    standard_error: f64,
    null_threshold: f64,
) -> f64 {
    let conservative_lower = (estimated_effect.abs() - 1.96 * standard_error).max(0.0);
    if conservative_lower <= null_threshold.abs() {
        1.0
    } else {
        let ratio = if null_threshold.abs() > 1e-4 {
            (conservative_lower / null_threshold.abs()).max(1.0)
        } else {
            let d_std = conservative_lower / (4.0 * standard_error).max(1e-6);
            (0.91 * d_std).exp().max(1.0)
        };
        ratio + (ratio * (ratio - 1.0)).max(0.0).sqrt()
    }
}

/// Formal Chow (1960) structural stability $F$-statistic across environments $e \in \mathcal{E}$ for predictor subset `subset`.
pub fn chow_invariance_f_stat(environments: &[CausalEnvironment], subset: &[usize]) -> f64 {
    assert!(environments.len() >= 2);
    let mut pooled_x = Vec::new();
    let mut pooled_y = Vec::new();
    for env in environments {
        for (row, &yi) in env.x.iter().zip(env.y.iter()) {
            pooled_x.push(subset.iter().map(|&idx| row[idx]).collect::<Vec<f64>>());
            pooled_y.push(yi);
        }
    }
    let coeffs_pool = fit_ols_multi(&pooled_x, &pooled_y);
    let rss_pool: f64 = pooled_x
        .iter()
        .zip(pooled_y.iter())
        .map(|(r, &y)| (y - eval_basis_model(&coeffs_pool, r)).powi(2))
        .sum();

    let mut rss_env_sum = 0.0;
    for env in environments {
        let env_x: Vec<Vec<f64>> = env
            .x
            .iter()
            .map(|r| subset.iter().map(|&idx| r[idx]).collect())
            .collect();
        let coeffs_e = fit_ols_multi(&env_x, &env.y);
        rss_env_sum += env_x
            .iter()
            .zip(env.y.iter())
            .map(|(r, &y)| (y - eval_basis_model(&coeffs_e, r)).powi(2))
            .sum::<f64>();
    }
    let k_env = environments.len() as f64;
    let d = (subset.len() + 1) as f64;
    let n_tot = pooled_y.len() as f64;
    let df1 = ((k_env - 1.0) * d).max(1.0);
    let df2 = (n_tot - k_env * d).max(1.0);
    (((rss_pool - rss_env_sum).max(0.0) / df1) / (rss_env_sum.max(1e-12) / df2)).max(0.0)
}

/// Cinelli & Hazlett (2020, *JRSS-B*) Omitted Variable Bias Robustness Value $RV_q \in [0, 1)$.
pub fn cinelli_hazlett_robustness_value(
    t_statistic: f64,
    degrees_of_freedom: usize,
    q_fraction: f64,
) -> f64 {
    assert!(degrees_of_freedom >= 1 && q_fraction > 0.0 && q_fraction <= 1.0);
    let f_q = q_fraction * t_statistic.abs() / (degrees_of_freedom as f64).sqrt();
    let f2 = f_q * f_q;
    0.5 * ((f2 * f2 + 4.0 * f2).sqrt() - f2)
}
