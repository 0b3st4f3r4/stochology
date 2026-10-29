//! Online Adaptive Conformal Inference (`AdaptiveConformalInference`, Gibbs & Candès 2021, §4.2).

/// Online Adaptive Conformal Inference (ACI, Gibbs & Candès 2021, *NeurIPS*) under arbitrary
/// non-exchangeable distribution shift:
/// $$\alpha_{t+1} = \mathrm{clamp}\bigl(\alpha_t + \gamma (\alpha_{\text{target}} - \mathrm{err}_t),\ 0.001,\ 0.999\bigr)$$
/// where $\mathrm{err}_t = \mathbb{I}\{Y_t \notin \hat{C}_t(X_t; 1 - \alpha_t)\}$. Guarantees long-run coverage
/// $\lim_{T \to \infty} \frac{1}{T}\sum_{t=1}^T (1 - \mathrm{err}_t) = 1 - \alpha_{\text{target}}$ without exchangeability!
#[derive(Debug, Clone, PartialEq)]
pub struct AdaptiveConformalInference {
    pub target_miscoverage_alpha: f64,
    pub step_size_gamma: f64,
    pub empirical_coverage_aci: f64,
    pub empirical_coverage_static: f64,
    pub final_alpha_t: f64,
    pub alpha_trajectory: Vec<f64>,
}

impl AdaptiveConformalInference {
    /// Evaluates online ACI against a static conformal quantile on calibration scores `calibration_scores`
    /// and a shifting out-of-sample test stream `test_nonconformity_scores`.
    pub fn evaluate_stream(
        calibration_scores: &[f64],
        test_nonconformity_scores: &[f64],
        target_miscoverage_alpha: f64,
        step_size_gamma: f64,
    ) -> Self {
        assert!(!calibration_scores.is_empty() && !test_nonconformity_scores.is_empty());
        assert!(target_miscoverage_alpha > 0.0 && target_miscoverage_alpha < 1.0);
        assert!(step_size_gamma > 0.0);

        let mut sorted_cal = calibration_scores.to_vec();
        sorted_cal.sort_by(|a, b| a.total_cmp(b));
        let q_static =
            stochology_core::quantile_sorted(&sorted_cal, 1.0 - target_miscoverage_alpha);

        let mut alpha_t = target_miscoverage_alpha;
        let mut traj = Vec::with_capacity(test_nonconformity_scores.len() + 1);
        traj.push(alpha_t);

        let mut covered_aci = 0usize;
        let mut covered_static = 0usize;

        for &s_t in test_nonconformity_scores {
            if s_t <= q_static {
                covered_static += 1;
            }
            let q_t =
                stochology_core::quantile_sorted(&sorted_cal, (1.0 - alpha_t).clamp(0.001, 0.999));
            let err_t = if s_t > q_t { 1.0 } else { 0.0 };
            if err_t == 0.0 {
                covered_aci += 1;
            }
            alpha_t = (alpha_t + step_size_gamma * (target_miscoverage_alpha - err_t))
                .clamp(0.001, 0.999);
            traj.push(alpha_t);
        }

        let t_len = test_nonconformity_scores.len() as f64;
        Self {
            target_miscoverage_alpha,
            step_size_gamma,
            empirical_coverage_aci: (covered_aci as f64) / t_len,
            empirical_coverage_static: (covered_static as f64) / t_len,
            final_alpha_t: alpha_t,
            alpha_trajectory: traj,
        }
    }
}
