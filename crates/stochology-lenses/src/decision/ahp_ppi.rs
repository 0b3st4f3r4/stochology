//! Thomas L. Saaty's (1980) Analytic Hierarchy Process (`AHP`) Spectral Consistency & Belton-Gear Rank-Reversal Audit,
//! and Rogan-Gladen (1978) / Prediction-Powered Inference (`PPI`) Rectifier for `LLM-as-a-Judge`.

/// Thomas L. Saaty's (1980) Analytic Hierarchy Process (`AHP`) Spectral Consistency,
/// Belton & Gear's (1983) Exact Rank-Reversal Audit, and `ICE`/`RICE` Covariance Fallacy on Bridge $L_2 \to L_3$.
#[derive(Debug, Clone, PartialEq)]
pub struct SaatyAhpRankReversalAudit {
    /// Principal eigenvalue $\lambda_{\max}$ of a $3\times 3$ consistent reciprocal matrix $A_{ij} = w_i / w_j$.
    pub consistent_lambda_max: f64,
    /// Consistency Index $\mathrm{CI} = \frac{\lambda_{\max} - n}{n - 1}$ of the consistent matrix ($\approx 0.0$).
    pub consistent_ci_error: f64,
    /// Standard column-normalized AHP scores $(S_A^{(3)}, S_B^{(3)})$ before adding a decoy copy of $B$ ($S_B^{(3)} > S_A^{(3)}$).
    pub ahp_scores_before_copy_a_vs_b: (f64, f64),
    /// Standard column-normalized AHP scores $(S_A^{(4)}, S_B^{(4)})$ after adding an exact copy $D = B$ ($S_A^{(4)} > S_B^{(4)}$ — Rank Reversal!).
    pub ahp_scores_after_copy_a_vs_b: (f64, f64),
    /// Ideal-Mode / Anchored Cost-Loss scores $(V_A, V_B)$ which remain strictly invariant when adding $D = B$ ($V_B > V_A$).
    pub anchored_ideal_mode_scores_a_vs_b: (f64, f64),
    /// Exact Jensen covariance bias $\mathbb{E}[I]\mathbb{E}[E] - \mathbb{E}[I \cdot E] = -\mathrm{Cov}(I, E) > 0$ when Impact $I$ and Ease $E$ are negatively correlated.
    pub ice_product_of_means_overestimation_bias: f64,
    /// True if $\lambda_{\max} = 3$, Belton-Gear rank reversal occurs under relative AHP, Ideal-Mode preserves order, and $\mathrm{Cov}(I,E)$ identity closes.
    pub identities_closed: bool,
}

impl SaatyAhpRankReversalAudit {
    /// Evaluates the spectral consistency of a $3\times 3$ reciprocal matrix, the Belton-Gear (1983, *Omega*)
    /// canonical rank-reversal test vector, and the `ICE` multiplicative score covariance bias.
    pub fn evaluate_canonical() -> Self {
        let w = [0.6_f64, 0.3, 0.1];
        let mut a_mat = [[0.0_f64; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                a_mat[i][j] = w[i] / w[j];
            }
        }
        let mut v = [1.0_f64 / 3.0; 3];
        let mut lambda_max = 3.0_f64;
        for _ in 0..30 {
            let mut av = [0.0_f64; 3];
            for i in 0..3 {
                for j in 0..3 {
                    av[i] += a_mat[i][j] * v[j];
                }
            }
            let sum_av: f64 = av.iter().sum();
            lambda_max = sum_av;
            for i in 0..3 {
                v[i] = av[i] / sum_av;
            }
        }
        let ci_err = ((lambda_max - 3.0) / 2.0).abs();

        let pool3 = [[1.0_f64, 9.0, 8.0], [9.0, 1.0, 9.0], [1.0, 1.0, 1.0]];
        let col_sums3: [f64; 3] = [
            pool3.iter().map(|r| r[0]).sum(),
            pool3.iter().map(|r| r[1]).sum(),
            pool3.iter().map(|r| r[2]).sum(),
        ];
        let score_rel = |row: &[f64; 3], sums: &[f64; 3]| -> f64 {
            (row[0] / sums[0] + row[1] / sums[1] + row[2] / sums[2]) / 3.0
        };
        let s_a3 = score_rel(&pool3[0], &col_sums3);
        let s_b3 = score_rel(&pool3[1], &col_sums3);

        let pool4 = [
            [1.0_f64, 9.0, 8.0],
            [9.0, 1.0, 9.0],
            [1.0, 1.0, 1.0],
            [9.0, 1.0, 9.0],
        ];
        let col_sums4: [f64; 3] = [
            pool4.iter().map(|r| r[0]).sum(),
            pool4.iter().map(|r| r[1]).sum(),
            pool4.iter().map(|r| r[2]).sum(),
        ];
        let s_a4 = score_rel(&pool4[0], &col_sums4);
        let s_b4 = score_rel(&pool4[1], &col_sums4);

        let v_a = (pool3[0][0] / 9.0 + pool3[0][1] / 9.0 + pool3[0][2] / 9.0) / 3.0;
        let v_b = (pool3[1][0] / 9.0 + pool3[1][1] / 9.0 + pool3[1][2] / 9.0) / 3.0;

        let impact = [9.0_f64, 7.0, 3.0, 1.0];
        let ease = [1.0_f64, 3.0, 7.0, 9.0];
        let mean_i = stochology_core::mean(&impact);
        let mean_e = stochology_core::mean(&ease);
        let prod_means = mean_i * mean_e;
        let mean_prod: f64 = impact
            .iter()
            .zip(ease.iter())
            .map(|(&i, &e)| i * e)
            .sum::<f64>()
            / 4.0;
        let cov_ie: f64 = impact
            .iter()
            .zip(ease.iter())
            .map(|(&i, &e)| (i - mean_i) * (e - mean_e))
            .sum::<f64>()
            / 4.0;
        let ice_bias = prod_means - mean_prod;
        let cov_identity_ok = (ice_bias - (-cov_ie)).abs() < 1e-12 && ice_bias > 5.0;

        let identities_closed =
            ci_err < 1e-12 && s_b3 > s_a3 && s_a4 > s_b4 && v_b > v_a && cov_identity_ok;

        Self {
            consistent_lambda_max: lambda_max,
            consistent_ci_error: ci_err,
            ahp_scores_before_copy_a_vs_b: (s_a3, s_b3),
            ahp_scores_after_copy_a_vs_b: (s_a4, s_b4),
            anchored_ideal_mode_scores_a_vs_b: (v_a, v_b),
            ice_product_of_means_overestimation_bias: ice_bias,
            identities_closed,
        }
    }
}

/// Rogan-Gladen (1978) & Prediction-Powered Inference (`PPI`, Angelopoulos et al. 2023, *Science*)
/// Rectifier for `LLM-as-a-Judge` automated evaluation pipelines on Bridge $L_1 \to L_2$.
#[derive(Debug, Clone, PartialEq)]
pub struct LlmJudgePpiRectifier {
    /// Raw uncalibrated `LLM-as-a-Judge` positive rate $\bar{f}_N = \frac{1}{N}\sum_{i=1}^N \hat{Y}_i$.
    pub raw_judge_rate: f64,
    /// Rogan-Gladen analytical prevalence estimate $\hat{\theta}_{\mathrm{RG}} = \frac{\bar{f}_N + q_0 - 1}{q_1 + q_0 - 1}$.
    pub rogan_gladen_estimate: f64,
    /// Prediction-Powered Inference rectified estimate $\hat{\theta}_{\mathrm{PPI}} = \bar{f}_N + \frac{1}{n}\sum_{j=1}^n (Y_j^{\text{gold}} - f_j^{\text{gold}})$.
    pub ppi_rectified_estimate: f64,
    /// Empirical rectifier bias correction $\hat{\Delta}_{\text{bias}} = \frac{1}{n}\sum_{j=1}^n (f_j^{\text{gold}} - Y_j^{\text{gold}})$.
    pub estimated_judge_bias: f64,
    /// Standard error of the PPI estimator $\sqrt{\frac{\hat{\sigma}_f^2}{N} + \frac{\hat{\sigma}_{Y-f}^2}{n}}$.
    pub ppi_standard_error: f64,
    /// Exact Rogan-Gladen population inversion error at the theoretical expectation $\mathbb{E}[\bar{f}] = \theta q_1 + (1-\theta)(1-q_0)$.
    pub rogan_gladen_identity_error: f64,
}

impl LlmJudgePpiRectifier {
    /// Evaluates the Rogan-Gladen and Prediction-Powered Inference (`PPI`) rectifiers.
    pub fn evaluate(
        unlabeled_judge_preds: &[f64],
        gold_judge_preds: &[f64],
        gold_human_labels: &[f64],
        judge_tpr_q1: f64,
        judge_tnr_q0: f64,
        true_theta_for_identity: f64,
    ) -> Self {
        assert!(
            !unlabeled_judge_preds.is_empty()
                && !gold_judge_preds.is_empty()
                && gold_judge_preds.len() == gold_human_labels.len(),
            "PPI requires non-empty unlabeled and paired gold slices"
        );
        let youden_j = (judge_tpr_q1 + judge_tnr_q0 - 1.0).max(1e-9);
        let raw_rate = stochology_core::mean(unlabeled_judge_preds);
        let rg_est = ((raw_rate + judge_tnr_q0 - 1.0) / youden_j).clamp(0.0, 1.0);

        let expected_raw = true_theta_for_identity * judge_tpr_q1
            + (1.0 - true_theta_for_identity) * (1.0 - judge_tnr_q0);
        let inverted_theta = (expected_raw + judge_tnr_q0 - 1.0) / youden_j;
        let rg_id_err = (inverted_theta - true_theta_for_identity).abs();

        let diffs: Vec<f64> = gold_human_labels
            .iter()
            .zip(gold_judge_preds.iter())
            .map(|(&y, &f)| y - f)
            .collect();
        let rect_delta = stochology_core::mean(&diffs);
        let ppi_est = (raw_rate + rect_delta).clamp(0.0, 1.0);
        let var_f = stochology_core::sample_variance(unlabeled_judge_preds);
        let var_diff = stochology_core::sample_variance(&diffs);
        let se = (var_f / (unlabeled_judge_preds.len() as f64)
            + var_diff / (gold_human_labels.len() as f64))
            .sqrt();

        Self {
            raw_judge_rate: raw_rate,
            rogan_gladen_estimate: rg_est,
            ppi_rectified_estimate: ppi_est,
            estimated_judge_bias: -rect_delta,
            ppi_standard_error: se,
            rogan_gladen_identity_error: rg_id_err,
        }
    }
}
