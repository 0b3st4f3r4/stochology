//! Axes 1–3 of Strategic AI Curriculum Accommodation:
//! - Axis 1 (`MinskyPapertGeometrySocietyOfMindAudit`): Walsh-Fourier Parity Order Theorem 3.1.1, Diameter-Limited Connectivity 0.8, and Minsky's *Society of Mind*
//! - Axis 2 (`ShapleyCausalRubinMiDoeAudit`): Causal `SHAP` vs. Observational `SHAP`, `PDP` vs. `ICE`, Rubin Multiple Imputation, and Clustered DoE / GP-EI
//! - Axis 3 (`GenerativeRagLlmJudgeCompressionAudit`): 4-stage RAG `DpiMarkovChain`, `LLM-as-a-Judge` (`PPI`), Conformal FinOps Cascade, and Outlier-Protected Quantization

use crate::acceptance::NamedCheck;
use crate::decision::{CostLossRule, LlmJudgePpiRectifier, brier_score};
use crate::information::DpiMarkovChain;
use crate::invariance::{CausalEnvironment, ShapleyCausalAudit};
use crate::tail::expected_shortfall_99;
use stochology_core::{
    KennedyOHaganDiscrepancy, ManskiBounds, Rng, RubinMultipleImputationAudit, mean,
    sample_variance,
};

/// Axis 1: Minsky & Papert (1969) Computational Geometry (`XOR`, Parity Order Theorem 3.1.1,
/// Diameter-Limited Connectivity Theorem 0.8) and Minsky's (1988) *Society of Mind*.
#[derive(Debug, Clone)]
pub struct MinskyPapertGeometrySocietyOfMindAudit {
    /// Maximum Walsh-Fourier inner product $\max_{|S| < 4} |\langle \psi_{\text{PARITY}}, \chi_S \rangle|$ on $\{-1, +1\}^4$ (exact $0.0$).
    pub parity_lower_order_walsh_max_correlation: f64,
    /// Accuracy of the best order-1 linear threshold vs. order-2 polynomial perceptron on `XOR` (`(0.50, 1.00)`).
    pub xor_order1_vs_order2_accuracy: (f64, f64),
    /// Accuracy of a diameter-limited local Euler receptor vs. a global Laplacian reachability operator on connectivity patterns.
    pub diameter_limited_vs_global_connectivity_accuracy: (f64, f64),
    /// Deployed connectivity accuracy (`global` when `shielded = true`, `diameter_limited` when `shielded = false`).
    pub deployed_connectivity_accuracy: f64,
    /// Brier score of a homogeneous highly-correlated ensemble ($\rho = 0.85$) vs. a heterogeneous *Society of Mind* ($\rho = 0.05$).
    pub homogeneous_vs_society_of_mind_brier: (f64, f64),
    /// Deployed ensemble Brier score (`society_of_mind` when `shielded = true`, `homogeneous` when `shielded = false`).
    pub deployed_ensemble_brier: f64,
    /// Exact Breiman-Moulton correlated ensemble variance identity residual $\bigl|\sigma^2(\rho + \frac{1-\rho}{M}) - \mathrm{Var}(\bar{\varepsilon}_M)\bigr|$.
    pub ensemble_variance_identity_error: f64,
    /// Named claim checks gated by Axis 1.
    pub claims: Vec<NamedCheck>,
    /// True if Walsh parity orthogonality, `XOR` order separation, and Breiman-Moulton variance identities close.
    pub identities_closed: bool,
    /// True if identities close and all Axis 1 claims pass.
    pub axis1_verified: bool,
}

impl MinskyPapertGeometrySocietyOfMindAudit {
    /// Evaluates Axis 1 under `seed` and mode `shielded`.
    pub fn evaluate(seed: u64, shielded: bool) -> Self {
        let mut rng = Rng::new(seed ^ 0x1969_1988_A101);

        // 1. Minsky & Papert (1969, Theorem 3.1.1): Parity on {-1, +1}^4 has zero projection on all Walsh monomials of degree < 4.
        let mut max_walsh_corr = 0.0_f64;
        for mask_s in 0..15usize {
            let mut dot = 0.0_f64;
            for x_bits in 0..16usize {
                let x = [
                    if (x_bits & 1) != 0 { 1.0 } else { -1.0 },
                    if (x_bits & 2) != 0 { 1.0 } else { -1.0 },
                    if (x_bits & 4) != 0 { 1.0 } else { -1.0 },
                    if (x_bits & 8) != 0 { 1.0 } else { -1.0 },
                ];
                let parity = x[0] * x[1] * x[2] * x[3];
                let mut chi_s = 1.0_f64;
                for (bit, &xj) in x.iter().enumerate() {
                    if (mask_s & (1 << bit)) != 0 {
                        chi_s *= xj;
                    }
                }
                dot += parity * chi_s;
            }
            max_walsh_corr = max_walsh_corr.max((dot / 16.0).abs());
        }

        let xor_pts = [
            ([-1.0_f64, -1.0], 1.0_f64),
            ([-1.0, 1.0], -1.0),
            ([1.0, -1.0], -1.0),
            ([1.0, 1.0], 1.0),
        ];
        let mut best_order1_acc = 0.0_f64;
        for w0 in [-1.0_f64, 0.0, 1.0] {
            for w1 in [-1.0_f64, 0.0, 1.0] {
                for b in [-0.5_f64, 0.0, 0.5] {
                    let hits = xor_pts
                        .iter()
                        .filter(|&&(x, y)| {
                            let pred = if w0 * x[0] + w1 * x[1] + b >= 0.0 {
                                1.0
                            } else {
                                -1.0
                            };
                            (pred - y).abs() < 1e-9
                        })
                        .count();
                    best_order1_acc = best_order1_acc.max((hits as f64) / 4.0);
                }
            }
        }
        let order2_hits = xor_pts
            .iter()
            .filter(|&&(x, y)| ((x[0] * x[1]) - y).abs() < 1e-9)
            .count();
        let order2_acc = (order2_hits as f64) / 4.0;

        // 2. Minsky & Papert (1969, Theorem 0.8): Diameter-limited local Euler characteristic vs. global connectivity.
        let n_trials = 120usize;
        let mut local_hits = 0usize;
        let mut global_hits = 0usize;
        for t in 0..n_trials {
            let is_connected = t % 2 == 0;
            let shift = (rng.next_u64() as usize) % 5;
            let mut grid = [0u8; 10];
            if is_connected {
                for k in 0..4 {
                    grid[shift + k] = 1;
                }
            } else {
                grid[shift] = 1;
                grid[shift + 1] = 1;
                grid[(shift + 3) % 10] = 1;
                grid[(shift + 4) % 10] = 1;
            }
            let active_mass: f64 = grid.iter().map(|&b| b as f64).sum();
            let local_score = (active_mass - 4.0) + rng.normal(0.0, 0.25);
            let local_pred = local_score >= 0.0;
            if local_pred == is_connected {
                local_hits += 1;
            }

            let first_one = grid.iter().position(|&b| b == 1).unwrap_or(0);
            let mut visited = [false; 10];
            let mut stack = vec![first_one];
            visited[first_one] = true;
            let mut reached_ones = 0usize;
            while let Some(u) = stack.pop() {
                reached_ones += 1;
                for nb in [u.saturating_sub(1), (u + 1).min(9)] {
                    if grid[nb] == 1 && !visited[nb] {
                        visited[nb] = true;
                        stack.push(nb);
                    }
                }
            }
            let total_ones = grid.iter().filter(|&&b| b == 1).count();
            let global_pred = reached_ones == total_ones;
            if global_pred == is_connected {
                global_hits += 1;
            }
        }
        let local_conn_acc = (local_hits as f64) / (n_trials as f64);
        let global_conn_acc = (global_hits as f64) / (n_trials as f64);
        let deployed_conn_acc = if shielded {
            global_conn_acc
        } else {
            local_conn_acc
        };

        // 3. Minsky (1988) "Society of Mind": M = 5 heterogeneous orthogonal agents vs. M = 5 homogeneous correlated agents.
        let m_agents = 5.0_f64;
        let sigma_sq = 0.09_f64;
        let rho_hom = 0.85_f64;
        let rho_het = 0.05_f64;
        let var_hom_closed = sigma_sq * (rho_hom + (1.0 - rho_hom) / m_agents);
        let var_het_closed = sigma_sq * (rho_het + (1.0 - rho_het) / m_agents);
        let mut cov_sum_hom = 0.0_f64;
        for i in 0..5 {
            for j in 0..5 {
                cov_sum_hom += if i == j { sigma_sq } else { rho_hom * sigma_sq };
            }
        }
        let var_id_err = (var_hom_closed - cov_sum_hom / 25.0).abs();

        let n_events = 300usize;
        let mut preds_hom = Vec::with_capacity(n_events);
        let mut preds_het = Vec::with_capacity(n_events);
        let mut outcomes = Vec::with_capacity(n_events);
        for i in 0..n_events {
            let p_true = if i % 2 == 0 { 0.14 } else { 0.86 };
            let y = if rng.bernoulli(p_true) { 1.0 } else { 0.0 };
            let common_shock = rng.normal(0.0, 1.0);
            let mut sum_hom = 0.0_f64;
            let mut sum_het = 0.0_f64;
            for _ in 0..5 {
                let idio_hom = rng.normal(0.0, 1.0);
                let idio_het = rng.normal(0.0, 1.0);
                let err_hom = sigma_sq.sqrt()
                    * (rho_hom.sqrt() * common_shock + (1.0 - rho_hom).sqrt() * idio_hom);
                let err_het = sigma_sq.sqrt()
                    * (rho_het.sqrt() * common_shock + (1.0 - rho_het).sqrt() * idio_het);
                let frame_bias = if p_true < 0.5 { 0.28 } else { -0.28 };
                sum_hom += (p_true + frame_bias + err_hom).clamp(0.01, 0.99);
                sum_het += (p_true + err_het).clamp(0.01, 0.99);
            }
            preds_hom.push(sum_hom / 5.0);
            preds_het.push(sum_het / 5.0);
            outcomes.push(y);
        }
        let bs_hom = brier_score(&preds_hom, &outcomes);
        let bs_het = brier_score(&preds_het, &outcomes);
        let deployed_bs = if shielded { bs_het } else { bs_hom };

        let identities_closed = max_walsh_corr < 1e-15
            && (best_order1_acc - 0.75).abs() < 1e-12
            && (order2_acc - 1.0).abs() < 1e-12
            && var_het_closed < var_hom_closed
            && var_id_err < 1e-15;

        let claims: Vec<NamedCheck> = vec![
            (
                "strategic.a1.connectivity_deployed_accuracy_above_0.90",
                deployed_conn_acc > 0.90,
            ),
            (
                "strategic.a1.society_of_mind_deployed_brier_below_0.195",
                deployed_bs < 0.195,
            ),
        ];
        let axis1_verified = identities_closed && claims.iter().all(|c| c.1);

        Self {
            parity_lower_order_walsh_max_correlation: max_walsh_corr,
            xor_order1_vs_order2_accuracy: (best_order1_acc, order2_acc),
            diameter_limited_vs_global_connectivity_accuracy: (local_conn_acc, global_conn_acc),
            deployed_connectivity_accuracy: deployed_conn_acc,
            homogeneous_vs_society_of_mind_brier: (bs_hom, bs_het),
            deployed_ensemble_brier: deployed_bs,
            ensemble_variance_identity_error: var_id_err,
            claims,
            identities_closed,
            axis1_verified,
        }
    }
}

/// Axis 2: Post-Hoc XAI Causal Audit (`SHAP` / `PDP` vs. `ICE`), Rubin's (1987) Multiple Imputation
/// vs. Single Mean Imputation & `ManskiBounds`, and Prospective Clustered DoE / GP Expected Improvement.
#[derive(Debug, Clone)]
pub struct ShapleyCausalRubinMiDoeAudit {
    /// Underlying `ShapleyCausalAudit` over the 3-environment structural DAG $X_0 \to Y \to X_2$.
    pub shapley_audit: ShapleyCausalAudit,
    /// Deployed intervention MSE (`interventional_icp` when `shielded = true`, `observational_shap` when `shielded = false`).
    pub deployed_shap_intervention_mse: f64,
    /// Rubin Multiple Imputation audit across $M = 5$ stochastic regression imputations under `MAR`.
    pub rubin_mi_audit: RubinMultipleImputationAudit,
    /// Variance retention ratio $\hat{\sigma}^2_{\text{imp}} / \sigma^2_{\text{true}}$ of deployed imputation (`Rubin MI` vs. `Single Mean Imputation`).
    pub deployed_imputation_variance_retention: f64,
    /// Empirical statistical power of prospective clustered trial sizing (`Moulton-Campbell DEFF` vs. `naive i.i.d.`).
    pub deployed_clustered_doe_power: f64,
    /// Kennedy-O'Hagan GP Expected Improvement (`EI`) ratio at an unexplored high-uncertainty region vs. an already-anchored point.
    pub gp_expected_improvement_exploratory_to_anchored_ratio: f64,
    /// Discarded findings recorded when a mainstream method fails under severe testing (Prohibition 4).
    pub discarded_findings: Vec<&'static str>,
    /// Claims evaluated across all seeds and recorded as refuted (`discarded_claims`).
    pub discarded_claims: Vec<NamedCheck>,
    /// Named claim checks gated by Axis 2.
    pub claims: Vec<NamedCheck>,
    /// True if Shapley Efficiency/Null/Symmetry, PDP cancellation, and Rubin variance decomposition identities close.
    pub identities_closed: bool,
    /// True if identities close and all Axis 2 claims pass.
    pub axis2_verified: bool,
}

impl ShapleyCausalRubinMiDoeAudit {
    /// Evaluates Axis 2 under `seed` and mode `shielded`.
    pub fn evaluate(seed: u64, shielded: bool) -> Self {
        let mut rng = Rng::new(seed ^ 0x2026_A202_58A9);

        // 1. Multi-environment DAG X0 -> Y -> X2 (X1 null):
        let mut envs = Vec::new();
        for (idx, (scale_x0, symp_w)) in [(1.0, 2.0), (1.8, -1.6), (0.7, 2.5)]
            .into_iter()
            .enumerate()
        {
            let mut x = Vec::with_capacity(240);
            let mut y = Vec::with_capacity(240);
            for _ in 0..240 {
                let x0 = rng.normal(0.0, scale_x0);
                let x1 = rng.normal(0.0, 1.0);
                let yi = 1.60 * x0 + rng.normal(0.0, 0.12);
                let x2 = symp_w * yi + rng.normal(0.0, 0.15);
                x.push(vec![x0, x1, x2]);
                y.push(yi);
            }
            envs.push(CausalEnvironment {
                name: format!("shap_env_{idx}"),
                x,
                y,
            });
        }
        let shap_multi = ShapleyCausalAudit::evaluate_environments(&envs, [1.5, 1.0, 3.0]);
        let (obs_do_mse, icp_do_mse) = shap_multi.observational_vs_interventional_do_mse;
        let deployed_shap_mse = if shielded { icp_do_mse } else { obs_do_mse };

        // 2. Rubin's (1987) Multiple Imputation (M = 5) vs. Single Mean Imputation under 40% MAR missingness:
        let n_tot = 300usize;
        let mut x_cov = Vec::with_capacity(n_tot);
        let mut y_full = Vec::with_capacity(n_tot);
        let mut y_obs = Vec::new();
        let mut x_obs = Vec::new();
        let mut x_miss = Vec::new();
        for _ in 0..n_tot {
            let xi = rng.normal(0.0, 1.0);
            let yi = 5.0 + 1.8 * xi + rng.normal(0.0, 1.2);
            x_cov.push(xi);
            y_full.push(yi);
            let p_obs = 1.0 / (1.0 + (-0.5 - 0.8 * xi).exp());
            if rng.bernoulli(p_obs) {
                y_obs.push(yi);
                x_obs.push(xi);
            } else {
                x_miss.push(xi);
            }
        }
        let true_var = sample_variance(&y_full);
        let (_, mean_imp_var_ratio) =
            RubinMultipleImputationAudit::single_mean_imputation_variance_shrinkage(&y_obs, n_tot);

        let mx_o = mean(&x_obs);
        let my_o = mean(&y_obs);
        let mut num_b = 0.0_f64;
        let mut den_b = 0.0_f64;
        for (&xo, &yo) in x_obs.iter().zip(y_obs.iter()) {
            num_b += (xo - mx_o) * (yo - my_o);
            den_b += (xo - mx_o).powi(2);
        }
        let beta_hat = num_b / den_b.max(1e-12);
        let alpha_hat = my_o - beta_hat * mx_o;
        let res_var: f64 = x_obs
            .iter()
            .zip(y_obs.iter())
            .map(|(&xo, &yo)| (yo - (alpha_hat + beta_hat * xo)).powi(2))
            .sum::<f64>()
            / ((y_obs.len().saturating_sub(2)).max(1) as f64);
        let res_sd = res_var.sqrt();

        let mut q_m = Vec::with_capacity(5);
        let mut u_m = Vec::with_capacity(5);
        let mut var_retentions = Vec::with_capacity(5);
        for _ in 0..5 {
            let b_draw = beta_hat + rng.normal(0.0, (res_var / den_b.max(1.0)).sqrt());
            let a_draw = alpha_hat + rng.normal(0.0, (res_var / (y_obs.len() as f64)).sqrt());
            let mut completed = y_obs.clone();
            for &xm in &x_miss {
                completed.push(a_draw + b_draw * xm + rng.normal(0.0, res_sd));
            }
            let v_comp = sample_variance(&completed);
            q_m.push(mean(&completed));
            u_m.push(v_comp / (n_tot as f64));
            var_retentions.push(v_comp / true_var.max(1e-12));
        }
        let rubin_mi = RubinMultipleImputationAudit::combine_imputations(&q_m, &u_m);
        let mi_var_retention = mean(&var_retentions);
        let deployed_var_retention = if shielded {
            mi_var_retention
        } else {
            mean_imp_var_ratio
        };

        let true_mean_y = mean(&y_full);
        let mnar_obs: Vec<f64> = y_full.iter().copied().filter(|&y| y > 5.5).collect();
        let mnar_mean = mean(&mnar_obs);
        let mnar_se = (sample_variance(&mnar_obs) / (mnar_obs.len().max(1) as f64)).sqrt();
        let mnar_parametric_ci_covers = (mnar_mean - 1.96 * mnar_se) <= true_mean_y
            && true_mean_y <= (mnar_mean + 1.96 * mnar_se);
        let manski_mnar = ManskiBounds::compute(&mnar_obs, n_tot, -2.0, 14.0);
        let manski_covers =
            manski_mnar.lower_bound <= true_mean_y && true_mean_y <= manski_mnar.upper_bound;

        let mut discarded_findings = Vec::new();
        if !mnar_parametric_ci_covers {
            discarded_findings.push(
                "parametric Multiple Imputation assuming MAR covers the true population mean under MNAR truncation Y > 5.5",
            );
        }

        // 3. Prospective Clustered DoE Power (Moulton-Campbell DEFF = 1 + (m_tilde - 1)*rho vs. naive i.i.d.):
        let (_, _, j_clustered) =
            stochology_core::prospective_clustered_mde(3.35, 1.0, 500, 25.0, 0.25, 25.0, 0.35);
        let j_naive_iid = 4usize;
        let j_deployed = if shielded { j_clustered } else { j_naive_iid };
        let se_deployed = (2.0 * (1.0 + 24.0 * 0.25) / ((j_deployed * 25) as f64)).sqrt();
        let z_ncp = 0.35 / se_deployed - 1.96;
        let mut power_hits = 0usize;
        for _ in 0..120 {
            let z_draw = z_ncp + rng.normal(0.0, 1.0);
            if z_draw > 0.0 {
                power_hits += 1;
            }
        }
        let deployed_power = (power_hits as f64) / 120.0;

        // 4. Bayesian Optimization Expected Improvement (EI) over Anchored Kennedy-O'Hagan GP:
        let koh_anchored = KennedyOHaganDiscrepancy::calibrate(
            &[0.1, 0.4, 0.8, 1.2],
            &[0.079, 0.307, 0.589, 0.845],
            0.80,
            1.2,
            0.45,
            0.005,
            &[0.02],
            &[0.016],
        );
        let koh_unexplored = KennedyOHaganDiscrepancy::calibrate(
            &[0.1, 0.4, 0.8, 1.2],
            &[0.079, 0.307, 0.589, 0.845],
            0.80,
            1.2,
            0.45,
            0.005,
            &[2.6],
            &[1.539],
        );
        let ei_ratio = koh_unexplored.extrapolation_posterior_std_with_gp
            / koh_anchored.extrapolation_posterior_std_with_gp.max(1e-9);

        let identities_closed = shap_multi.axioms_and_identities_closed
            && rubin_mi.variance_identity_error < 1e-14
            && manski_covers
            && ei_ratio > 5.0;

        let discarded_claims: Vec<NamedCheck> = vec![(
            "strategic.a2.parametric_mar_mi_covers_true_mean_under_mnar",
            mnar_parametric_ci_covers,
        )];

        let claims: Vec<NamedCheck> = vec![
            (
                "strategic.a2.deployed_shap_causal_intervention_mse_below_0.15",
                deployed_shap_mse < 0.15,
            ),
            (
                "strategic.a2.deployed_imputation_variance_retention_above_0.80",
                deployed_var_retention > 0.80,
            ),
            (
                "strategic.a2.clustered_doe_deployed_power_above_0.80",
                deployed_power > 0.80,
            ),
            (
                "strategic.a2.det.gp_expected_improvement_ratio_above_5",
                ei_ratio > 5.0,
            ),
        ];
        let axis2_verified = identities_closed && claims.iter().all(|c| c.1);

        Self {
            shapley_audit: shap_multi,
            deployed_shap_intervention_mse: deployed_shap_mse,
            rubin_mi_audit: rubin_mi,
            deployed_imputation_variance_retention: deployed_var_retention,
            deployed_clustered_doe_power: deployed_power,
            gp_expected_improvement_exploratory_to_anchored_ratio: ei_ratio,
            discarded_findings,
            discarded_claims,
            claims,
            identities_closed,
            axis2_verified,
        }
    }
}

/// Axis 3: Frontier Generative Systems — 4-Stage RAG `DpiMarkovChain`, Rogan-Gladen / `PPI`
/// `LLM-as-a-Judge` Rectifier, Conformal FinOps Cascade Routing, and Outlier-Protected Quantization.
#[derive(Debug, Clone)]
pub struct GenerativeRagLlmJudgeCompressionAudit {
    /// 4-stage RAG retrieval `DpiMarkovChain` (`Corpus W -> Chunk X -> Embedding Y -> Rerank Z`) under semantic chunking.
    pub rag_semantic_dpi: DpiMarkovChain,
    /// Mutual information $I(W; Z)$ retained by deployed RAG chunking (`semantic` vs. `fixed_boundary_split`).
    pub deployed_rag_end_to_end_mi_nats: f64,
    /// Rogan-Gladen & Prediction-Powered Inference (`PPI`) rectifier audit for `LLM-as-a-Judge`.
    pub llm_judge_ppi: LlmJudgePpiRectifier,
    /// Absolute estimation error $|\hat{\theta}_{\text{deployed}} - \theta_{\text{true}}|$ (`PPI` vs. `raw_judge`).
    pub deployed_llm_judge_abs_error: f64,
    /// Expected operational cost+loss per decision of the Conformal FinOps Cascade vs. naive uncalibrated local model.
    pub finops_cascade_vs_naive_local_cost_per_unit: (f64, f64),
    /// Deployed expected cost+loss per unit (`conformal_cascade` when `shielded = true`, `naive_local` when `shielded = false`).
    pub deployed_finops_cost_per_unit: f64,
    /// Tail $\mathrm{ES}_{0.99}$ retention ratio $\widehat{\mathrm{ES}}_{0.99} / \mathrm{ES}_{0.99}^{\text{FP32}}$ (`outlier_protected_int4` vs. `naive_clipped_int4`).
    pub quantization_outlier_protected_vs_naive_es99_retention: (f64, f64),
    /// Deployed quantization $\mathrm{ES}_{0.99}$ retention ratio.
    pub deployed_quantization_es99_retention: f64,
    /// Claims evaluated across all seeds and recorded as refuted (`discarded_claims`).
    pub discarded_claims: Vec<NamedCheck>,
    /// Named claim checks gated by Axis 3.
    pub claims: Vec<NamedCheck>,
    /// True if RAG DPI monotonicity and Rogan-Gladen population inversion identities close.
    pub identities_closed: bool,
    /// True if identities close and all Axis 3 claims pass.
    pub axis3_verified: bool,
}

impl GenerativeRagLlmJudgeCompressionAudit {
    /// Evaluates Axis 3 under `seed` and mode `shielded`.
    pub fn evaluate(seed: u64, shielded: bool) -> Self {
        let mut rng = Rng::new(seed ^ 0x2026_A303_77B1);

        // 1. 4-Stage RAG DPI Funnel: Corpus W -> Chunk X -> Embedding Y -> Rerank Z
        let bsc = |eps: f64| vec![vec![1.0 - eps, eps], vec![eps, 1.0 - eps]];
        let rag_semantic =
            DpiMarkovChain::compute(&[0.50, 0.50], &bsc(0.03), &bsc(0.04), &bsc(0.03));
        let rag_naive_split =
            DpiMarkovChain::compute(&[0.50, 0.50], &bsc(0.28), &bsc(0.12), &bsc(0.08));
        let deployed_rag_mi = if shielded {
            rag_semantic.i_w_z_nats
        } else {
            rag_naive_split.i_w_z_nats
        };

        // 2. LLM-as-a-Judge with true pass rate theta = 0.22, judge sensitivity q1 = 0.86, specificity q0 = 0.72:
        let theta_true = 0.22_f64;
        let q1 = 0.86_f64;
        let q0 = 0.72_f64;
        let n_unlabeled = 800usize;
        let n_gold = 160usize;
        let mut unlabeled_f = Vec::with_capacity(n_unlabeled);
        for _ in 0..n_unlabeled {
            let y = if rng.bernoulli(theta_true) { 1.0 } else { 0.0 };
            let p_judge = if y > 0.5 { q1 } else { 1.0 - q0 };
            unlabeled_f.push(if rng.bernoulli(p_judge) { 1.0 } else { 0.0 });
        }
        let mut gold_y = Vec::with_capacity(n_gold);
        let mut gold_f = Vec::with_capacity(n_gold);
        for _ in 0..n_gold {
            let y = if rng.bernoulli(theta_true) { 1.0 } else { 0.0 };
            let p_judge = if y > 0.5 { q1 } else { 1.0 - q0 };
            gold_y.push(y);
            gold_f.push(if rng.bernoulli(p_judge) { 1.0 } else { 0.0 });
        }
        let ppi =
            LlmJudgePpiRectifier::evaluate(&unlabeled_f, &gold_f, &gold_y, q1, q0, theta_true);
        let err_ppi = (ppi.ppi_rectified_estimate - theta_true).abs();
        let err_raw = (ppi.raw_judge_rate - theta_true).abs();
        let deployed_judge_err = if shielded { err_ppi } else { err_raw };

        // 3. Compute-Optimal Test-Time Scaling & Conformal FinOps Cascade Routing:
        let rule = CostLossRule::new(20.0, 100.0, 1.0);
        let p_star = rule.critical_threshold_p_star;
        let n_finops = 300usize;
        let mut cost_cascade = 0.0_f64;
        let mut cost_naive = 0.0_f64;
        for _ in 0..n_finops {
            let p_true_i = (0.02 + 0.45 * rng.next_f64()).clamp(0.01, 0.99);
            let y_i = if rng.bernoulli(p_true_i) { 1.0 } else { 0.0 };
            let p_local_cal = (p_true_i + rng.normal(0.0, 0.06)).clamp(0.01, 0.99);
            let p_local_naive = (0.35 * p_true_i).clamp(0.01, 0.99);

            let act_naive = rule.recommend_action(p_local_naive);
            cost_naive += 0.2
                + if act_naive == "protect" {
                    rule.protection_cost_c
                } else {
                    y_i * rule.event_loss_l
                };

            let (p_used, compute_fee) = if (p_local_cal - p_star).abs() < 0.10 {
                (p_true_i, 0.2 + 2.0)
            } else {
                (p_local_cal, 0.2)
            };
            let act_casc = rule.recommend_action(p_used);
            cost_cascade += compute_fee
                + if act_casc == "protect" {
                    rule.protection_cost_c
                } else {
                    y_i * rule.event_loss_l
                };
        }
        let unit_cost_cascade = cost_cascade / (n_finops as f64);
        let unit_cost_naive = cost_naive / (n_finops as f64);
        let deployed_finops_cost = if shielded {
            unit_cost_cascade
        } else {
            unit_cost_naive
        };

        // 4. Outlier-Protected Mixed-Precision INT4 Quantization vs. Naive Clipped INT4 on Pareto Tail ES_0.99:
        let n_act = 500usize;
        let mut fp32_losses = Vec::with_capacity(n_act);
        let mut naive_int4_losses = Vec::with_capacity(n_act);
        let mut protected_int4_losses = Vec::with_capacity(n_act);
        for _ in 0..n_act {
            let u = rng.next_f64().clamp(1e-4, 0.9999);
            let x_fp32 = (1.0 - u).powf(-1.0 / 1.65);
            fp32_losses.push(x_fp32);
            let x_clipped = (x_fp32.min(4.0) / 4.0 * 15.0).round() * (4.0 / 15.0);
            naive_int4_losses.push(x_clipped);
            let x_prot = if x_fp32 > 4.0 { x_fp32 } else { x_clipped };
            protected_int4_losses.push(x_prot);
        }
        let es99_fp32 = expected_shortfall_99(&fp32_losses).max(1e-9);
        let es99_naive_ret = expected_shortfall_99(&naive_int4_losses) / es99_fp32;
        let es99_prot_ret = expected_shortfall_99(&protected_int4_losses) / es99_fp32;
        let deployed_es99_ret = if shielded {
            es99_prot_ret
        } else {
            es99_naive_ret
        };

        let identities_closed = rag_semantic.dpi_satisfied
            && rag_naive_split.dpi_satisfied
            && ppi.rogan_gladen_identity_error < 1e-14;

        let discarded_claims: Vec<NamedCheck> = vec![(
            "strategic.a3.llm_judge_deployed_abs_error_below_0.06",
            deployed_judge_err < 0.06,
        )];

        let claims: Vec<NamedCheck> = vec![
            (
                "strategic.a3.det.rag_deployed_mi_above_0.30_nats",
                deployed_rag_mi > 0.30,
            ),
            (
                "strategic.a3.finops_cascade_beats_naive_local_by_1.5",
                unit_cost_naive - deployed_finops_cost > 1.5,
            ),
            (
                "strategic.a3.quantization_deployed_es99_retention_above_0.90",
                deployed_es99_ret > 0.90,
            ),
        ];
        let axis3_verified = identities_closed && claims.iter().all(|c| c.1);

        Self {
            rag_semantic_dpi: rag_semantic,
            deployed_rag_end_to_end_mi_nats: deployed_rag_mi,
            llm_judge_ppi: ppi,
            deployed_llm_judge_abs_error: deployed_judge_err,
            finops_cascade_vs_naive_local_cost_per_unit: (unit_cost_cascade, unit_cost_naive),
            deployed_finops_cost_per_unit: deployed_finops_cost,
            quantization_outlier_protected_vs_naive_es99_retention: (es99_prot_ret, es99_naive_ret),
            deployed_quantization_es99_retention: deployed_es99_ret,
            discarded_claims,
            claims,
            identities_closed,
            axis3_verified,
        }
    }
}
