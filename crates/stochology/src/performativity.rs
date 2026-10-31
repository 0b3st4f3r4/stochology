//! **Chapter 6 (Filter III · Tension 5) — Reactive Worlds: Causal Invariance, Performativity, Calibration, and Goodhart's Law.**
//!
//! Mediates the tension between passive curve-fitting and intervention in reactive social systems
//! (Goodhart, 1975; Lucas, 1976; Strathern, 1997; Tian & Pearl, 2000; Kennedy & O'Hagan, 2001;
//! MacKenzie, 2006; Hudgens & Halloran, 2008; Peters, Bühlmann & Meinshausen, 2016;
//! Manheim & Garrabrant, 2018; Perdomo et al., 2020; Kleinberg & Raghavan, 2020;
//! Dimitriadis, Gneiting & Jordan, 2021):
//! - Isotonic CORP-Murphy Brier Score decomposition via PAV (`CorpPavDecomposition`);
//! - Invariant Causal Prediction across heterogeneous environments (`CausalEnvironment`, `InvariantPredictorAudit`);
//! - Kennedy–O'Hagan anchored Gaussian Process structural discrepancy (`AnchoredDiscrepancy`);
//! - Tian–Pearl sharp bounds on counterfactual necessity and sufficiency (`TianPearlCounterfactualBounds`);
//! - Perdomo performative stability ($\theta_{\mathrm{PS}}$) vs. Stackelberg optimality ($\theta_{\mathrm{PO}}$),
//!   MacKenzie's 4 performativity regimes, causal effort vs. cosmetic gaming, and congestion spillover (`PerformativeSystem`);
//! - Goodhart's 4 variants of metric collapse (`GoodhartVariant`, `GoodhartAudit`).

use crate::math::{ols_ridge_fit, pav_isotonic_non_decreasing, solve_cholesky_spd};
use crate::rng::mean;

/// Dimitriadis, Gneiting & Jordan (2021) Consistent, Optimally Binned, Reproducible (CORP)
/// isotonic Brier Score decomposition via the Pool-Adjacent-Violators (PAV) algorithm:
/// $$\mathrm{BS} = \mathrm{REL}_{\mathrm{CORP}} - \mathrm{RES}_{\mathrm{CORP}} + \mathrm{UNC}_0$$
#[derive(Debug, Clone, PartialEq)]
pub struct CorpPavDecomposition {
    /// Mean Brier Score $\mathrm{BS} = \frac{1}{n}\sum_{i=1}^n (p_i - y_i)^2$.
    pub brier_score: f64,
    /// CORP Reliability (miscalibration loss) $\mathrm{REL}_{\mathrm{CORP}} = \frac{1}{n}\sum_{i=1}^n (p_i - \hat{\pi}_i)^2 \ge 0$.
    pub reliability: f64,
    /// CORP Resolution (discriminative power) $\mathrm{RES}_{\mathrm{CORP}} = \frac{1}{n}\sum_{i=1}^n (\hat{\pi}_i - \bar{y})^2 \ge 0$.
    pub resolution: f64,
    /// Marginal base-rate uncertainty $\mathrm{UNC}_0 = \bar{y}(1 - \bar{y})$.
    pub uncertainty: f64,
    /// Isotonically recalibrated probabilities $\hat{\pi}_{(1)} \le \dots \le \hat{\pi}_{(n)}$ in the original order.
    pub recalibrated_probs: Vec<f64>,
    /// Exact algebraic closure error $|\mathrm{BS} - (\mathrm{REL} - \mathrm{RES} + \mathrm{UNC}_0)|$.
    pub identity_closure_error: f64,
}

impl CorpPavDecomposition {
    /// Computes the exact PAV isotonic CORP decomposition of `forecasts` against binary `outcomes` ($\in \{0.0, 1.0\}$).
    pub fn evaluate(forecasts: &[f64], outcomes: &[f64]) -> Self {
        assert!(
            !forecasts.is_empty() && forecasts.len() == outcomes.len(),
            "CorpPavDecomposition requires non-empty equal-length slices"
        );
        let n = forecasts.len();
        let nf = n as f64;
        let mut order: Vec<usize> = (0..n).collect();
        // Break forecast ties in descending order of outcome so PAV pools identical forecasts into one block
        order.sort_by(|&i, &j| {
            forecasts[i]
                .total_cmp(&forecasts[j])
                .then_with(|| outcomes[j].total_cmp(&outcomes[i]))
        });

        let y_sorted: Vec<f64> = order.iter().map(|&i| outcomes[i]).collect();
        let p_sorted: Vec<f64> = order.iter().map(|&i| forecasts[i]).collect();
        let pi_sorted = pav_isotonic_non_decreasing(&y_sorted);

        let mut recalibrated = vec![0.0; n];
        for (sorted_pos, &orig_idx) in order.iter().enumerate() {
            recalibrated[orig_idx] = pi_sorted[sorted_pos];
        }

        let y_bar = mean(outcomes);
        let unc = y_bar * (1.0 - y_bar);
        let bs = p_sorted
            .iter()
            .zip(y_sorted.iter())
            .map(|(&p, &y)| (p - y) * (p - y))
            .sum::<f64>()
            / nf;
        let bs_cal = pi_sorted
            .iter()
            .zip(y_sorted.iter())
            .map(|(&pi, &y)| (pi - y) * (pi - y))
            .sum::<f64>()
            / nf;
        let rel = (bs - bs_cal).max(0.0);
        let res = pi_sorted
            .iter()
            .map(|&pi| (pi - y_bar) * (pi - y_bar))
            .sum::<f64>()
            / nf;
        let err = (bs - (rel - res + unc)).abs();

        Self {
            brier_score: bs,
            reliability: rel,
            resolution: res,
            uncertainty: unc,
            recalibrated_probs: recalibrated,
            identity_closure_error: err,
        }
    }
}

/// One observational or interventional environment $e \in \mathcal{E}$ for Invariant Causal Prediction
/// (Peters, Bühlmann & Meinshausen, 2016).
#[derive(Debug, Clone, PartialEq)]
pub struct CausalEnvironment {
    /// Human-readable regime name.
    pub name: String,
    /// Design matrix rows $X_{i,\cdot}$ ($n_e \times d$).
    pub x: Vec<Vec<f64>>,
    /// Outcome vector $Y_i$ ($n_e$).
    pub y: Vec<f64>,
}

/// Audits whether each candidate predictor $j \in \{0, \dots, d-1\}$ maintains an invariant slope
/// and residual distribution across environments $e \in \mathcal{E}$ (Peters, Bühlmann & Meinshausen, 2016).
#[derive(Debug, Clone, PartialEq)]
pub struct InvariantPredictorAudit {
    /// Candidate predictor index $j$.
    pub predictor_index: usize,
    /// Environment-specific OLS slopes $\hat{\beta}_j^{(e)}$.
    pub env_slopes: Vec<f64>,
    /// Slope dispersion across environments $\max_e \hat{\beta}_j^{(e)} - \min_e \hat{\beta}_j^{(e)}$.
    pub slope_range: f64,
    /// Worst-case out-of-environment mean squared prediction error.
    pub worst_case_mse: f64,
}

/// Evaluates single-predictor causal invariance across heterogeneous `environments` for each feature column.
pub fn evaluate_invariant_predictors(
    environments: &[CausalEnvironment],
) -> Vec<InvariantPredictorAudit> {
    assert!(
        environments.len() >= 2 && !environments[0].x.is_empty(),
        "requires at least 2 non-empty environments"
    );
    let d = environments[0].x[0].len();
    let mut results = Vec::with_capacity(d);

    for j in 0..d {
        let mut slopes = Vec::with_capacity(environments.len());
        let mut intercepts = Vec::with_capacity(environments.len());
        for env in environments {
            let design: Vec<Vec<f64>> = env.x.iter().map(|row| vec![1.0, row[j]]).collect();
            let beta = ols_ridge_fit(&design, &env.y, 1e-8).unwrap_or_else(|| vec![0.0, 0.0]);
            intercepts.push(beta[0]);
            slopes.push(beta[1]);
        }
        let min_s = slopes.iter().copied().fold(f64::INFINITY, f64::min);
        let max_s = slopes.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let mean_b0 = mean(&intercepts);
        let mean_b1 = mean(&slopes);

        let mut worst_mse = 0.0_f64;
        for env in environments {
            let mse = env
                .x
                .iter()
                .zip(env.y.iter())
                .map(|(row, &yi)| {
                    let pred = mean_b0 + mean_b1 * row[j];
                    (yi - pred) * (yi - pred)
                })
                .sum::<f64>()
                / (env.y.len().max(1) as f64);
            if mse > worst_mse {
                worst_mse = mse;
            }
        }
        results.push(InvariantPredictorAudit {
            predictor_index: j,
            env_slopes: slopes,
            slope_range: (max_s - min_s).max(0.0),
            worst_case_mse: worst_mse,
        });
    }
    results
}

/// Kennedy & O'Hagan (2001) anchored Gaussian Process structural discrepancy $\delta(x) \sim \mathcal{GP}(0, k(x, x'))$.
///
/// Prevents parameter pseudo-convergence inside the training support and widens predictive variance
/// when extrapolating outside the observed anchor domain.
#[derive(Debug, Clone, PartialEq)]
pub struct AnchoredDiscrepancy {
    /// Anchor inputs $x_1, \dots, x_n$.
    pub anchor_x: Vec<f64>,
    /// Observed structural residuals $r_i = y_i - f_{\mathrm{model}}(x_i, \theta_0)$.
    pub anchor_residuals: Vec<f64>,
    /// Prior discrepancy amplitude $\sigma_\delta > 0$.
    pub amplitude_sigma: f64,
    /// RBF correlation lengthscale $\ell > 0$.
    pub lengthscale_l: f64,
    /// Observation noise standard deviation $\sigma_{\mathrm{noise}} > 0$.
    pub noise_sigma: f64,
    /// Dual weights $\alpha = (K + \sigma_{\mathrm{noise}}^2 I)^{-1} r$.
    dual_weights: Vec<f64>,
    /// Gram matrix $K + \sigma_{\mathrm{noise}}^2 I$.
    gram_matrix: Vec<Vec<f64>>,
}

impl AnchoredDiscrepancy {
    /// Fits a Gaussian Process discrepancy model over `anchor_x` and `anchor_residuals`.
    pub fn fit(
        anchor_x: &[f64],
        anchor_residuals: &[f64],
        amplitude_sigma: f64,
        lengthscale_l: f64,
        noise_sigma: f64,
    ) -> Self {
        assert!(
            !anchor_x.is_empty()
                && anchor_x.len() == anchor_residuals.len()
                && amplitude_sigma > 0.0
                && lengthscale_l > 0.0
                && noise_sigma > 0.0,
            "AnchoredDiscrepancy::fit requires valid non-empty inputs"
        );
        let n = anchor_x.len();
        let s2 = amplitude_sigma * amplitude_sigma;
        let l2 = lengthscale_l * lengthscale_l;
        let n2 = noise_sigma * noise_sigma;
        let mut gram = vec![vec![0.0; n]; n];
        for i in 0..n {
            for j in 0..n {
                let d = anchor_x[i] - anchor_x[j];
                gram[i][j] = s2 * (-0.5 * d * d / l2).exp() + if i == j { n2 } else { 0.0 };
            }
        }
        let weights = solve_cholesky_spd(&gram, anchor_residuals).unwrap_or_else(|| vec![0.0; n]);
        Self {
            anchor_x: anchor_x.to_vec(),
            anchor_residuals: anchor_residuals.to_vec(),
            amplitude_sigma,
            lengthscale_l,
            noise_sigma,
            dual_weights: weights,
            gram_matrix: gram,
        }
    }

    /// Predicts `(posterior_mean, posterior_std)` of the structural discrepancy $\delta(x_*)$ at query point `x_star`.
    /// Notice that as $|x_* - x_i| \to \infty$, `posterior_std` automatically expands back to `amplitude_sigma`.
    pub fn predict(&self, x_star: f64) -> (f64, f64) {
        let s2 = self.amplitude_sigma * self.amplitude_sigma;
        let l2 = self.lengthscale_l * self.lengthscale_l;
        let k_star: Vec<f64> = self
            .anchor_x
            .iter()
            .map(|&xi| {
                let d = x_star - xi;
                s2 * (-0.5 * d * d / l2).exp()
            })
            .collect();
        let post_mean: f64 = k_star
            .iter()
            .zip(self.dual_weights.iter())
            .map(|(&k, &w)| k * w)
            .sum();
        let v = solve_cholesky_spd(&self.gram_matrix, &k_star)
            .unwrap_or_else(|| vec![0.0; k_star.len()]);
        let reduction: f64 = k_star.iter().zip(v.iter()).map(|(&k, &vi)| k * vi).sum();
        let post_var = (s2 - reduction).max(1e-12);
        (post_mean, post_var.sqrt())
    }
}

/// Jin Tian & Judea Pearl (2000) sharp non-parametric bounds on the probabilities of causation:
/// - Probability of Necessity $\mathrm{PN} = \mathbb{P}(Y_{x_0} = 0 \mid X = x_1, Y = 1)$;
/// - Probability of Sufficiency $\mathrm{PS} = \mathbb{P}(Y_{x_1} = 1 \mid X = x_0, Y = 0)$;
/// - Probability of Necessity and Sufficiency $\mathrm{PNS} = \mathbb{P}(Y_{x_1} = 1, Y_{x_0} = 0)$.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TianPearlCounterfactualBounds {
    /// Lower and upper bounds on $\mathrm{PN}$.
    pub pn_bounds: (f64, f64),
    /// Lower and upper bounds on $\mathrm{PS}$.
    pub ps_bounds: (f64, f64),
    /// Lower and upper bounds on $\mathrm{PNS}$.
    pub pns_bounds: (f64, f64),
}

impl TianPearlCounterfactualBounds {
    /// Evaluates Tian & Pearl (2000) counterfactual bounds from experimental rates
    /// $p_{y_1} = \mathbb{P}(Y_{x_1}=1)$, $p_{y_0} = \mathbb{P}(Y_{x_0}=1)$ and observational joint probabilities
    /// $p_{11} = \mathbb{P}(X=1, Y=1)$, $p_{10} = \mathbb{P}(X=1, Y=0)$,
    /// $p_{01} = \mathbb{P}(X=0, Y=1)$, $p_{00} = \mathbb{P}(X=0, Y=0)$.
    pub fn evaluate(
        exp_p_y_do_x1: f64,
        exp_p_y_do_x0: f64,
        obs_p_x1_y1: f64,
        obs_p_x1_y0: f64,
        obs_p_x0_y1: f64,
        obs_p_x0_y0: f64,
    ) -> Self {
        let p_y = obs_p_x1_y1 + obs_p_x0_y1;
        let p_not_y = obs_p_x1_y0 + obs_p_x0_y0;

        let pns_lo = (exp_p_y_do_x1 - exp_p_y_do_x0)
            .max(p_y - exp_p_y_do_x0)
            .max(exp_p_y_do_x1 - p_y)
            .max(0.0);
        let pns_hi = exp_p_y_do_x1
            .min(1.0 - exp_p_y_do_x0)
            .min(obs_p_x1_y1 + obs_p_x0_y0)
            .min(exp_p_y_do_x1 - exp_p_y_do_x0 + obs_p_x1_y0 + obs_p_x0_y1)
            .clamp(pns_lo, 1.0);

        let pn_lo = ((p_y - exp_p_y_do_x0).max(0.0) / obs_p_x1_y1.max(1e-12)).clamp(0.0, 1.0);
        let pn_hi = (((1.0 - exp_p_y_do_x0 - obs_p_x0_y0).min(obs_p_x1_y1)).max(0.0)
            / obs_p_x1_y1.max(1e-12))
        .clamp(pn_lo, 1.0);

        let ps_lo = ((exp_p_y_do_x1 - p_y).max(0.0) / obs_p_x0_y0.max(1e-12)).clamp(0.0, 1.0);
        let ps_hi = (((exp_p_y_do_x1 - obs_p_x1_y1).min(obs_p_x0_y0)).max(0.0)
            / obs_p_x0_y0.max(1e-12))
        .clamp(ps_lo, 1.0);
        let _ = p_not_y;

        Self {
            pn_bounds: (pn_lo, pn_hi),
            ps_bounds: (ps_lo, ps_hi),
            pns_bounds: (pns_lo, pns_hi),
        }
    }
}

/// Donald MacKenzie (2006) classification of performativity regimes when a model or rule $\theta$
/// acts upon the socio-economic world (*An Engine, Not a Camera*).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MacKenzieRegime {
    /// `CameraPassive`: Wasserstein shift $\varepsilon \approx 0$; deploying the model does not alter the world.
    CameraPassive,
    /// `EffectivePerformative`: Deploying the rule alters the world and improves substantive social welfare.
    EffectivePerformative,
    /// `BarnesianSelfFulfilling`: Repeating the rule drives predictive residuals toward zero ($\theta \to \theta_{\mathrm{PS}}$)
    /// by molding agents to fit the formula, without reaching the substantive optimum ($\theta_{\mathrm{PO}}$).
    BarnesianSelfFulfilling,
    /// `CounterperformativeDestructive`: Deploying the rule triggers feedback that destabilizes the world
    /// ($\frac{\varepsilon\beta}{\gamma} \ge 1$) and degrades substantive outcomes.
    CounterperformativeDestructive,
}

/// Juan Perdomo et al. (2020) Performative Prediction system coupled with MacKenzie's (2006)
/// regime classification and Kleinberg & Raghavan's (2020) causal effort vs. cosmetic gaming audit.
///
/// Under distribution map $Y(\theta) = \mu_0 + \varepsilon \theta + \xi$ and loss
/// $\ell(y, \theta) = \frac{1}{2}(y - \theta)^2 + \frac{\lambda}{2}\theta^2$:
/// - **Performative Stability** (fixed point of Repeated Risk Minimization):
///   $$\theta_{\mathrm{PS}} = \frac{\mu_0}{1 + \lambda - \varepsilon}$$
/// - **Performative Optimality** (Stackelberg equilibrium accounting for $\frac{\partial \mathcal{D}(\theta)}{\partial \theta} = \varepsilon$):
///   $$\theta_{\mathrm{PO}} = \frac{(1 - \varepsilon)\mu_0}{(1 - \varepsilon)^2 + \lambda}$$
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PerformativeSystem {
    /// Baseline drift $\mu_0$.
    pub base_mean_mu0: f64,
    /// Wasserstein-1 performative sensitivity $\varepsilon$.
    pub sensitivity_epsilon: f64,
    /// Smoothness of the loss gradient $\beta > 0$.
    pub smoothness_beta: f64,
    /// Strong convexity parameter $\gamma = 1 + \lambda > 0$.
    pub convexity_gamma: f64,
    /// Contraction ratio $r = \frac{|\varepsilon|\beta}{\gamma}$ (RRM converges iff $r < 1$).
    pub rrm_contraction_ratio: f64,
    /// Performative stability fixed point $\theta_{\mathrm{PS}}$.
    pub theta_ps: f64,
    /// Performative optimality Stackelberg point $\theta_{\mathrm{PO}}$.
    pub theta_po: f64,
    /// Performative risk evaluated at $\theta_{\mathrm{PS}}$.
    pub risk_at_ps: f64,
    /// Performative risk evaluated at $\theta_{\mathrm{PO}}$ (always $\le \text{risk at }\theta_{\mathrm{PS}}$).
    pub risk_at_po: f64,
    /// Substantive gap $|\theta_{\mathrm{PS}} - \theta_{\mathrm{PO}}|$.
    pub stability_optimality_gap: f64,
    /// Classified MacKenzie (2006) performativity regime.
    pub mackenzie_regime: MacKenzieRegime,
}

impl PerformativeSystem {
    /// Evaluates the closed-form equilibria and contraction dynamics of a performative prediction system.
    pub fn evaluate(
        base_mean_mu0: f64,
        sensitivity_epsilon: f64,
        regularization_lambda: f64,
        noise_variance_sigma2: f64,
    ) -> Self {
        let lam = regularization_lambda.max(1e-6);
        let gamma = 1.0 + lam;
        let beta = 1.0_f64;
        let ratio = (sensitivity_epsilon.abs() * beta) / gamma;

        let denom_ps = (1.0 + lam - sensitivity_epsilon).max(1e-6);
        let theta_ps = base_mean_mu0 / denom_ps;
        let one_minus_eps = 1.0 - sensitivity_epsilon;
        let theta_po = (one_minus_eps * base_mean_mu0) / (one_minus_eps * one_minus_eps + lam);

        let eval_risk = |theta: f64| -> f64 {
            let mean_y = base_mean_mu0 + sensitivity_epsilon * theta;
            let bias = mean_y - theta;
            0.5 * (bias * bias + noise_variance_sigma2.max(0.0)) + 0.5 * lam * theta * theta
        };
        let risk_ps = eval_risk(theta_ps);
        let risk_po = eval_risk(theta_po);
        let gap = (theta_ps - theta_po).abs();

        let regime = if sensitivity_epsilon.abs() < 1e-4 {
            MacKenzieRegime::CameraPassive
        } else if ratio >= 1.0 {
            MacKenzieRegime::CounterperformativeDestructive
        } else if gap > 0.05 {
            MacKenzieRegime::BarnesianSelfFulfilling
        } else {
            MacKenzieRegime::EffectivePerformative
        };

        Self {
            base_mean_mu0,
            sensitivity_epsilon,
            smoothness_beta: beta,
            convexity_gamma: gamma,
            rrm_contraction_ratio: ratio,
            theta_ps,
            theta_po,
            risk_at_ps: risk_ps,
            risk_at_po: risk_po,
            stability_optimality_gap: gap,
            mackenzie_regime: regime,
        }
    }

    /// Evaluates Kleinberg & Raghavan (2020) / Miller, Milli & Hardt (2020) strategic effort response:
    /// given weight `w_causal` on a genuine causal investment (cost `c_causal`, true qualification effect `alpha_causal > 0`)
    /// and weight `w_proxy` on a cosmetic gaming feature (cost `c_proxy`, zero effect on true qualification),
    /// returns `(delta_true_qualification, cosmetic_gaming_waste, social_burden_ratio)` where `social_burden_ratio`
    /// is Milli et al. (2019)'s ratio of effort cost required from a disadvantaged group vs. an advantaged group.
    pub fn strategic_effort_audit(
        w_causal: f64,
        c_causal: f64,
        alpha_causal: f64,
        w_proxy: f64,
        c_proxy: f64,
        disadvantaged_gap: f64,
        advantaged_gap: f64,
    ) -> (f64, f64, f64) {
        let eff_causal = (w_causal / c_causal.max(1e-9)).max(0.0);
        let eff_proxy = (w_proxy / c_proxy.max(1e-9)).max(0.0);
        let delta_qualification = alpha_causal * eff_causal - 0.05 * eff_proxy;
        let burden_ratio = disadvantaged_gap.max(0.0) / advantaged_gap.max(1e-9);
        (delta_qualification, eff_proxy, burden_ratio)
    }

    /// Hudgens & Halloran (2008) interference / scale-up congestion spillover:
    /// given pilot effect `pilot_effect` at coverage $c_0 \approx 0$, congestion penalty `congestion_gamma`,
    /// and target scale-up coverage $c \in [0, 1]$, returns the attenuated equilibrium treatment effect
    /// $\tau(c) = \tau_{\mathrm{pilot}} - \gamma_{\mathrm{congestion}} \cdot c$.
    #[inline]
    pub fn scale_up_spillover_effect(
        pilot_effect: f64,
        congestion_gamma: f64,
        coverage_c: f64,
    ) -> f64 {
        pilot_effect - congestion_gamma * coverage_c.clamp(0.0, 1.0)
    }
}

/// Manheim & Garrabrant (2018) taxonomy of the four mechanisms by which a proxy metric $M$
/// decouples from the true substantive goal $G$ when optimized (Goodhart, 1975; Strathern, 1997).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GoodhartVariant {
    /// **Regressional Goodhart:** Selecting the tail of a noisy proxy $M = G + \varepsilon$ selects for extreme noise $\varepsilon$.
    Regressional,
    /// **Extremal Goodhart:** Pushing $M$ into an extreme tail leaves the domain where $M$ and $G$ were historically correlated.
    Extremal,
    /// **Causal Goodhart:** Intervening on a downstream symptom $M$ ($G \to M$) does not alter the upstream cause $G$.
    Causal,
    /// **Adversarial Goodhart:** Strategic agents actively game $M$ to capture rewards while hollowing out $G$.
    Adversarial,
}

/// Quantitative audit of proxy-goal decoupling across a before/after intervention or tail selection.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GoodhartAudit {
    /// Mean change in the proxy metric $\Delta M > 0$.
    pub proxy_gain: f64,
    /// Mean change in the true substantive goal $\Delta G$.
    pub substantive_goal_change: f64,
    /// Decoupling divergence $\Delta M - \Delta G$.
    pub decoupling_gap: f64,
    /// Dominant Goodhart mechanism identified.
    pub dominant_variant: GoodhartVariant,
    /// True if optimizing the proxy degraded or failed to improve the substantive goal ($\Delta M > 0$ yet $\Delta G \le 0$).
    pub metric_collapsed: bool,
}

impl GoodhartAudit {
    /// Evaluates proxy-goal decoupling and classifies the dominant Goodhart mechanism.
    pub fn evaluate(
        proxy_gain: f64,
        substantive_goal_change: f64,
        variant: GoodhartVariant,
    ) -> Self {
        Self {
            proxy_gain,
            substantive_goal_change,
            decoupling_gap: proxy_gain - substantive_goal_change,
            dominant_variant: variant,
            metric_collapsed: proxy_gain > 0.0 && substantive_goal_change <= 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn corp_pav_decomposition_closes_brier_identity_exactly() {
        let p = [0.1, 0.4, 0.35, 0.8, 0.9, 0.2, 0.75, 0.6];
        let y = [0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0];
        let corp = CorpPavDecomposition::evaluate(&p, &y);
        assert!(corp.identity_closure_error < 1e-14);
        assert!(corp.reliability >= 0.0 && corp.resolution >= 0.0);
    }

    #[test]
    fn performative_system_and_anchored_discrepancy_hold() {
        let sys = PerformativeSystem::evaluate(1.0, 0.45, 0.20, 0.10);
        assert!(sys.rrm_contraction_ratio < 1.0);
        assert!(sys.risk_at_po < sys.risk_at_ps);
        assert_eq!(
            sys.mackenzie_regime,
            MacKenzieRegime::BarnesianSelfFulfilling
        );

        let gp = AnchoredDiscrepancy::fit(&[-1.0, 0.0, 1.0], &[0.1, -0.05, 0.08], 0.50, 0.60, 0.05);
        let (_, std_in) = gp.predict(0.0);
        let (_, std_out) = gp.predict(10.0);
        assert!(std_out > 3.0 * std_in);
    }

    #[test]
    fn tian_pearl_bounds_and_icp_distinguish_causes_from_proxies() {
        let tp = TianPearlCounterfactualBounds::evaluate(0.70, 0.20, 0.40, 0.10, 0.0, 0.50);
        assert!((tp.pn_bounds.0 - 0.50).abs() < 1e-12);
        assert!((tp.pn_bounds.1 - 0.75).abs() < 1e-12);

        let e1 = CausalEnvironment {
            name: "e1".to_string(),
            x: vec![vec![1.0, 1.0], vec![2.0, 2.0], vec![3.0, 3.0]],
            y: vec![2.0, 4.0, 6.0],
        };
        let e2 = CausalEnvironment {
            name: "e2".to_string(),
            x: vec![vec![1.0, -1.0], vec![2.0, -2.0], vec![3.0, -3.0]],
            y: vec![2.0, 4.0, 6.0],
        };
        let icp = evaluate_invariant_predictors(&[e1, e2]);
        assert!(icp[0].slope_range < 1e-5);
        assert!(icp[1].slope_range > 3.0);
    }
}
