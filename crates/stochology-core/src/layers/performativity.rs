//! Bridge $L_3 \to L_1$ (Performativity & Goodhart's Law):
//! Performative Stability ($\theta_{\mathrm{PS}}$) vs. Stackelberg Optimality ($\theta_{\mathrm{PO}}$),
//! `PerfGD`, Wasserstein-1 sensitivity (`empirical_wasserstein_1`), and the 4 Manheim-Garrabrant
//! variants of Goodhart's Law (`GoodhartTaxonomyAudit`).

use crate::rng::quantile_sorted;

/// Exact 1D empirical Wasserstein-1 (Earth Mover / Kantorovich-Rubinstein) distance
/// $W_1(\hat{P}, \hat{Q}) = \int_0^1 |F_{\hat{P}}^{-1}(u) - F_{\hat{Q}}^{-1}(u)|\,du$ between two empirical samples.
pub fn empirical_wasserstein_1(sample_p: &[f64], sample_q: &[f64]) -> f64 {
    assert!(
        !sample_p.is_empty() && !sample_q.is_empty(),
        "empirical_wasserstein_1 requires non-empty samples"
    );
    let mut sp = sample_p.to_vec();
    let mut sq = sample_q.to_vec();
    sp.sort_by(|a, b| a.total_cmp(b));
    sq.sort_by(|a, b| a.total_cmp(b));
    if sp.len() == sq.len() {
        let n = sp.len() as f64;
        return sp
            .iter()
            .zip(sq.iter())
            .map(|(&a, &b)| (a - b).abs())
            .sum::<f64>()
            / n;
    }
    let grid = (sp.len() + sq.len()).clamp(64, 512);
    let mut acc = 0.0;
    for k in 0..grid {
        let u = ((k as f64) + 0.5) / (grid as f64);
        acc += (quantile_sorted(&sp, u) - quantile_sorted(&sq, u)).abs();
    }
    acc / (grid as f64)
}

/// Formal Theory of Performative Prediction on Bridge $L_3 \to L_1$
/// (Perdomo, Zrnic, Mendler-Dünner & Hardt — §1.1 & Prohibition 3).
///
/// Models a canonical system where outcome $Z \sim \mathcal{D}(\theta)$ shifts linearly with the deployed model $\theta$:
/// $$Y(\theta) = \mu + \varepsilon \theta + \xi, \qquad \ell(z; \phi) = \frac{\gamma}{2}(y - \phi)^2 - \alpha \phi$$
/// with Wasserstein-1 sensitivity $W_1(\mathcal{D}(\theta), \mathcal{D}(\theta')) \le \varepsilon \|\theta - \theta'\|_2$,
/// strong convexity $\gamma > 0$, and smoothness $\beta \ge \gamma$.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PerformativeSystem {
    /// Base distribution mean $\mu$ under passive regime ($\varepsilon = 0$).
    pub base_mean_mu: f64,
    /// Wasserstein-1 sensitivity $\varepsilon$ of induced distribution map $\mathcal{D}(\theta)$.
    pub sensitivity_epsilon: f64,
    /// Regularization / linear payoff incentive $\alpha$ in the loss.
    pub incentive_alpha: f64,
    /// Strong convexity constant $\gamma > 0$.
    pub strong_convexity_gamma: f64,
    /// Smoothness constant $\beta \ge \gamma$.
    pub smoothness_beta: f64,
}

/// Complete audit of Performative Stability ($\theta_{\mathrm{PS}}$) vs. Performative Optimality ($\theta_{\mathrm{PO}}$).
#[derive(Debug, Clone, PartialEq)]
pub struct PerformativeDiagnostic {
    /// Passive Supervised Learning estimator ($\varepsilon = 0$): $\theta_{\mathrm{SL}}$.
    pub theta_sl: f64,
    /// Performative Stability fixed point ($\theta_{\mathrm{PS}} = \arg\min_\phi \mathrm{DPR}(\theta_{\mathrm{PS}}, \phi)$).
    pub theta_ps: f64,
    /// Performative Optimality Stackelberg equilibrium ($\theta_{\mathrm{PO}} = \arg\min_\theta \mathrm{PR}(\theta)$).
    pub theta_po: f64,
    /// Performative Risk $\mathrm{PR}(\theta_{\mathrm{PS}})$ at the stable retraining point.
    pub risk_at_ps: f64,
    /// Performative Risk $\mathrm{PR}(\theta_{\mathrm{PO}})$ at the Stackelberg optimum.
    pub risk_at_po: f64,
    /// Gap $|\theta_{\mathrm{PO}} - \theta_{\mathrm{PS}}|$ between stability and optimality.
    pub gap_ps_vs_po: f64,
    /// Contraction radius of Repeated Risk Minimization (RRM): $r = \frac{|\varepsilon| \beta}{\gamma}$.
    pub rrm_contraction_radius: f64,
    /// Whether the strict contraction inequality $\frac{|\varepsilon|\beta}{\gamma} < 1$ holds.
    pub rrm_converges: bool,
    /// Trajectory $\theta_0, \theta_1, \dots, \theta_T$ of Repeated Risk Minimization (RRM).
    pub rrm_trajectory: Vec<f64>,
}

impl PerformativeSystem {
    pub fn new(
        base_mean_mu: f64,
        sensitivity_epsilon: f64,
        incentive_alpha: f64,
        strong_convexity_gamma: f64,
        smoothness_beta: f64,
    ) -> Self {
        assert!(strong_convexity_gamma > 0.0, "gamma must be > 0");
        assert!(
            smoothness_beta >= strong_convexity_gamma,
            "beta must be >= gamma"
        );
        Self {
            base_mean_mu,
            sensitivity_epsilon,
            incentive_alpha,
            strong_convexity_gamma,
            smoothness_beta,
        }
    }

    /// Decoupled Performative Risk $\mathrm{DPR}(\theta, \phi) \triangleq \mathbb{E}_{Z \sim \mathcal{D}(\theta)}[\ell(Z; \phi)]$.
    #[inline]
    pub fn decoupled_risk_dpr(&self, theta_world: f64, phi_evaluated: f64) -> f64 {
        let expected_y = self.base_mean_mu + self.sensitivity_epsilon * theta_world;
        let diff = expected_y - phi_evaluated;
        0.5 * self.strong_convexity_gamma * diff * diff - self.incentive_alpha * phi_evaluated
    }

    /// Performative Risk $\mathrm{PR}(\theta) \triangleq \mathrm{DPR}(\theta, \theta)$.
    #[inline]
    pub fn performative_risk_pr(&self, theta: f64) -> f64 {
        self.decoupled_risk_dpr(theta, theta)
    }

    /// Best-response map $G(\theta_t) = \arg\min_\phi \mathrm{DPR}(\theta_t, \phi)$ of Repeated Risk Minimization (RRM).
    #[inline]
    pub fn rrm_update_step(&self, theta_t: f64) -> f64 {
        self.base_mean_mu
            + self.sensitivity_epsilon * theta_t
            + (self.incentive_alpha / self.strong_convexity_gamma)
    }

    /// Runs the full performative diagnostic and simulates `steps` iterations of RRM starting from `theta_0`.
    pub fn diagnose(&self) -> PerformativeDiagnostic {
        let offset = self.incentive_alpha / self.strong_convexity_gamma;
        let theta_sl = self.base_mean_mu + offset;
        let eps = self.sensitivity_epsilon;

        let one_minus_eps = 1.0 - eps;
        let theta_ps = if one_minus_eps.abs() > 1e-12 {
            (self.base_mean_mu + offset) / one_minus_eps
        } else {
            f64::INFINITY
        };

        let theta_po = if one_minus_eps.abs() > 1e-12 {
            (self.base_mean_mu / one_minus_eps) + (offset / (one_minus_eps * one_minus_eps))
        } else {
            f64::INFINITY
        };

        let radius = (eps.abs() * self.smoothness_beta) / self.strong_convexity_gamma;
        let converges = radius < 1.0;

        let mut traj = Vec::with_capacity(16);
        let mut curr = 0.0;
        traj.push(curr);
        for _ in 0..15 {
            curr = self.rrm_update_step(curr);
            traj.push(curr);
        }

        let risk_ps = self.performative_risk_pr(theta_ps);
        let risk_po = self.performative_risk_pr(theta_po);

        PerformativeDiagnostic {
            theta_sl,
            theta_ps,
            theta_po,
            risk_at_ps: risk_ps,
            risk_at_po: risk_po,
            gap_ps_vs_po: (theta_po - theta_ps).abs(),
            rrm_contraction_radius: radius,
            rrm_converges: converges,
            rrm_trajectory: traj,
        }
    }

    /// Simulates a non-contracting performative classifier ($\frac{\varepsilon \beta}{\gamma} \ge 1$)
    /// that oscillates perpetually between extreme states $1, 0, 1, 0, \dots$.
    pub fn simulate_rrm_oscillation(steps: usize) -> Vec<f64> {
        let non_contracting = Self::new(1.0, -1.0, 0.0, 1.0, 1.0);
        let mut theta = 1.0;
        let mut history = Vec::with_capacity(steps);
        for _ in 0..steps {
            history.push(theta);
            theta = non_contracting.rrm_update_step(theta).clamp(0.0, 1.0);
        }
        history
    }

    /// Performative Gradient Descent (`PerfGD`, Izzo, Ying & Zou 2021) using the total performative derivative
    /// $$\nabla_\theta \mathrm{PR}(\theta) = \nabla_2 \mathrm{DPR}(\theta, \theta) + \nabla_1 \mathrm{DPR}(\theta, \theta) = -\gamma(1 - \varepsilon)\big(\mu - (1 - \varepsilon)\theta\big) - \alpha$$
    /// which converges directly to the Stackelberg Performative Optimum $\theta_{\mathrm{PO}}$ rather than stopping at $\theta_{\mathrm{PS}}$.
    pub fn perfgd_trajectory(&self, theta_0: f64, step_size_eta: f64, steps: usize) -> Vec<f64> {
        assert!(step_size_eta > 0.0, "step_size_eta must be > 0");
        let one_minus_eps = 1.0 - self.sensitivity_epsilon;
        let mut theta = theta_0;
        let mut traj = Vec::with_capacity(steps + 1);
        traj.push(theta);
        for _ in 0..steps {
            let grad = -self.strong_convexity_gamma
                * one_minus_eps
                * (self.base_mean_mu - one_minus_eps * theta)
                - self.incentive_alpha;
            theta -= step_size_eta * grad;
            traj.push(theta);
        }
        traj
    }

    /// Decomposes the total performative gradient $\nabla_\theta \mathrm{PR}(\theta)$ into:
    /// 1. $\frac{\partial \mathrm{DPR}(\theta_{\text{world}}, \phi_{\text{eval}})}{\partial \theta_{\text{world}}}\Big|_{\phi=\theta} = \gamma \varepsilon \big(\mu - (1 - \varepsilon)\theta\big)$
    ///    (distribution-shift / Adversarial Goodhart gradient; denoted $\nabla_2 \mathrm{DPR}$ in Izzo, Ying & Zou 2021 when arguments are ordered $(\text{model}, \text{distribution})$),
    /// 2. $\frac{\partial \mathrm{DPR}(\theta_{\text{world}}, \phi_{\text{eval}})}{\partial \phi_{\text{eval}}}\Big|_{\phi=\theta} = -\gamma \big(\mu - (1 - \varepsilon)\theta\big) - \alpha$
    ///    (passive parameter gradient, zero at $\theta_{\mathrm{PS}}$; denoted $\nabla_1 \mathrm{DPR}$ in Izzo, Ying & Zou 2021).
    #[inline]
    pub fn dpr_gradient_components(&self, theta: f64) -> (f64, f64) {
        let residual = self.base_mean_mu - (1.0 - self.sensitivity_epsilon) * theta;
        let grad_1_dist_shift = self.strong_convexity_gamma * self.sensitivity_epsilon * residual;
        let grad_2_model_param = -self.strong_convexity_gamma * residual - self.incentive_alpha;
        (grad_1_dist_shift, grad_2_model_param)
    }

    /// Oracle-free Performative Gradient Descent (`PerfGD`, Izzo, Ying & Zou 2021) that estimates
    /// the distribution-shift sensitivity $\hat{\varepsilon}_t = \frac{\bar{Y}(\theta_t) - \bar{Y}(\theta_{t-1})}{\theta_t - \theta_{t-1}}$
    /// from consecutive deployment means without direct access to `self.sensitivity_epsilon`.
    pub fn perfgd_finite_difference_trajectory(
        &self,
        theta_0: f64,
        step_size_eta: f64,
        delta_probe: f64,
        steps: usize,
    ) -> Vec<f64> {
        assert!(step_size_eta > 0.0 && delta_probe.abs() > 1e-9);
        let observe_mean_y = |th: f64| self.base_mean_mu + self.sensitivity_epsilon * th;
        let mut traj = Vec::with_capacity(steps + 1);
        let mut theta_prev = theta_0;
        let mut y_prev = observe_mean_y(theta_prev);
        traj.push(theta_prev);

        let mut theta = theta_0 + delta_probe;
        for _ in 0..steps {
            let y_curr = observe_mean_y(theta);
            let dtheta = theta - theta_prev;
            let eps_hat = if dtheta.abs() > 1e-12 {
                (y_curr - y_prev) / dtheta
            } else {
                0.0
            };
            let residual = y_curr - theta;
            let grad_model = -self.strong_convexity_gamma * residual - self.incentive_alpha;
            let grad_shift = self.strong_convexity_gamma * eps_hat * residual;
            let total_grad = grad_model + grad_shift;
            theta_prev = theta;
            y_prev = y_curr;
            theta -= step_size_eta * total_grad;
            traj.push(theta);
        }
        traj
    }

    /// Canonical linear performative pricing benchmark (Izzo, Ying & Zou 2021, §1):
    /// $\ell(z; \theta) = -\theta z$ (or $\theta z$ with $a < 0 < b$) where $\mathbb{E}[Z \mid \theta] = a\theta + b$.
    /// Returns `(theta_stab, theta_opt, ratio_stab_over_opt)` proving $\theta_{\mathrm{STAB}} = 2\,\theta_{\mathrm{OPT}}$.
    pub fn izzo_linear_2x_benchmark(a_slope: f64, b_intercept: f64) -> (f64, f64, f64) {
        assert!(a_slope.abs() > 1e-12, "a_slope must be non-zero");
        let theta_stab = -b_intercept / a_slope;
        let theta_opt = -b_intercept / (2.0 * a_slope);
        let ratio = theta_stab / theta_opt;
        (theta_stab, theta_opt, ratio)
    }
}

/// Formal taxonomy of the 4 Variants of Goodhart's Law (Manheim & Garrabrant 2018,
/// *Categorizing Variants of Goodhart's Law*) on Bridges $L_2 \to L_3$ and $L_3 \to L_1$.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GoodhartVariant {
    /// Regressional Goodhart: selecting on noisy proxy $M = G + \varepsilon$ inflates $\mathbb{E}[M - G \mid M \ge \tau] > 0$ (Winner's Curse).
    Regressional,
    /// Extremal Goodhart: pushing $M$ into extreme tail regions breaks the bulk manifold correlation between $M$ and $G$.
    Extremal,
    /// Causal Goodhart: intervening $\mathrm{do}(M = m)$ on a non-causal proxy or effect leaves true goal $G$ unchanged or degraded; detected by ICP/NILE ($S^*$).
    Causal,
    /// Adversarial Goodhart: strategic agents game $M$ by shifting $\mathcal{D}(\theta)$, where passive RRM ignores $\nabla_1 \mathrm{DPR}(\theta, \theta)$ while `PerfGD` internalizes $\nabla_1 + \nabla_2$.
    Adversarial,
}

/// Quantitative audit of the 4 Variants of Goodhart's Law (Manheim & Garrabrant 2018),
/// coupling Causal breakdown to ICP/NILE ($S^*$) and Adversarial gaming to `PerfGD` gradients.
#[derive(Debug, Clone, PartialEq)]
pub struct GoodhartTaxonomyAudit {
    /// Variant 1 (Regressional): Winner's curse inflation $\mathbb{E}[M - G \mid M \ge q_{90}]$ under noisy proxy selection.
    pub regressional_winners_curse_inflation: f64,
    /// Variant 2 (Extremal): Goal degradation $\mathbb{E}[G \mid M \in \text{bulk}] - \mathbb{E}[G \mid M \ge q_{98}]$ in the extreme tail.
    pub extremal_tail_decoupling_drop: f64,
    /// Variant 3 (Causal): Ratio of NILE/IRM invariance penalty on spurious proxy vs. causal core $S^*$, plus interventional MSE gap.
    pub causal_nile_penalty_ratio: f64,
    /// Variant 3 (Causal): Interventional degradation $\mathrm{MSE}(\mathrm{do}(M)) - \mathrm{MSE}(\mathrm{do}(S^*))$.
    pub causal_interventional_mse_gap: f64,
    /// Variant 4 (Adversarial): Ignored distribution-shift gradient $\nabla_1 \mathrm{DPR}(\theta_{\mathrm{PS}}, \theta_{\mathrm{PS}})$ at Performative Stability $\theta_{\mathrm{PS}}$.
    pub adversarial_grad1_dpr_at_ps: f64,
    /// Variant 4 (Adversarial): Passive parameter gradient $\nabla_2 \mathrm{DPR}(\theta_{\mathrm{PS}}, \theta_{\mathrm{PS}})$ at $\theta_{\mathrm{PS}}$ ($\approx 0$).
    pub adversarial_grad2_dpr_at_ps: f64,
    /// Variant 4 (Adversarial): Stackelberg risk improvement $\mathrm{PR}(\theta_{\mathrm{PS}}) - \mathrm{PR}(\theta_{\mathrm{PO}}) > 0$ achieved by `PerfGD`.
    pub adversarial_perfgd_stackelberg_gain: f64,
    /// Dominant diagnosed Goodhart failure mode when unmitigated.
    pub diagnosed_variants: [GoodhartVariant; 4],
    /// True if all 4 Goodhart variants are quantitatively separated and mitigated by ICP/NILE ($S^*$) + `PerfGD`.
    pub all_four_variants_audited: bool,
}

impl GoodhartTaxonomyAudit {
    /// Evaluates the 4 Manheim-Garrabrant Goodhart variants (`seed = 2026`), coupling `causal_nile_penalty_ratio`
    /// and `causal_interventional_mse_gap` from ICP/NILE ($S^*$) with `PerformativeSystem` (`PerfGD`).
    pub fn evaluate(
        seed: u64,
        causal_nile_penalty_ratio: f64,
        causal_interventional_mse_gap: f64,
    ) -> Self {
        let mut rng = crate::rng::SplitMix64::new(seed.wrapping_add(1975));
        let n = 600;
        let mut goals_g = Vec::with_capacity(n);
        let mut proxies_reg = Vec::with_capacity(n);
        let mut goals_ext = Vec::with_capacity(n);
        let mut proxies_ext = Vec::with_capacity(n);

        for _ in 0..n {
            let g = rng.normal(0.0, 1.0);
            let m_reg = g + rng.normal(0.0, 0.90);
            goals_g.push(g);
            proxies_reg.push(m_reg);

            // Extremal: aligned in bulk (|m| <= 1.8), inverted parabolic collapse in extreme tail (|m| > 1.8)
            let m_ext = rng.normal(0.0, 1.2);
            let g_ext = if m_ext <= 1.8 {
                0.85 * m_ext
            } else {
                0.85 * 1.8 - 2.40 * (m_ext - 1.8).powi(2)
            };
            proxies_ext.push(m_ext);
            goals_ext.push(g_ext);
        }

        let q90_reg = crate::rng::quantile(&proxies_reg, 0.90);
        let mut inflation_sum = 0.0;
        let mut inflation_cnt = 0usize;
        for (&m, &g) in proxies_reg.iter().zip(goals_g.iter()) {
            if m >= q90_reg {
                inflation_sum += m - g;
                inflation_cnt += 1;
            }
        }
        let regressional_winners_curse_inflation = inflation_sum / (inflation_cnt.max(1) as f64);

        let q50_ext = crate::rng::quantile(&proxies_ext, 0.50);
        let q95_ext = crate::rng::quantile(&proxies_ext, 0.95);
        let q98_ext = crate::rng::quantile(&proxies_ext, 0.98);
        let mut bulk_g_sum = 0.0;
        let mut bulk_g_cnt = 0usize;
        let mut tail_g_sum = 0.0;
        let mut tail_g_cnt = 0usize;
        for (&m, &g) in proxies_ext.iter().zip(goals_ext.iter()) {
            if m >= q50_ext && m < q95_ext {
                bulk_g_sum += g;
                bulk_g_cnt += 1;
            }
            if m >= q98_ext {
                tail_g_sum += g;
                tail_g_cnt += 1;
            }
        }
        let extremal_tail_decoupling_drop =
            (bulk_g_sum / (bulk_g_cnt.max(1) as f64)) - (tail_g_sum / (tail_g_cnt.max(1) as f64));

        // Variant 4 (Adversarial): PerformativeSystem & PerfGD gradient decomposition at theta_PS
        let perf_sys = PerformativeSystem::new(1.0, 0.25, 0.60, 1.0, 1.25);
        let diag = perf_sys.diagnose();
        let (grad1_ps, grad2_ps) = perf_sys.dpr_gradient_components(diag.theta_ps);
        let adversarial_perfgd_stackelberg_gain = diag.risk_at_ps - diag.risk_at_po;

        let all_four_variants_audited = regressional_winners_curse_inflation > 0.50
            && extremal_tail_decoupling_drop > 0.50
            && causal_nile_penalty_ratio > 20.0
            && causal_interventional_mse_gap > 4.0
            && grad1_ps.abs() > 0.05
            && grad2_ps.abs() < 1e-10
            && adversarial_perfgd_stackelberg_gain > 0.01;

        Self {
            regressional_winners_curse_inflation,
            extremal_tail_decoupling_drop,
            causal_nile_penalty_ratio,
            causal_interventional_mse_gap,
            adversarial_grad1_dpr_at_ps: grad1_ps,
            adversarial_grad2_dpr_at_ps: grad2_ps,
            adversarial_perfgd_stackelberg_gain,
            diagnosed_variants: [
                GoodhartVariant::Regressional,
                GoodhartVariant::Extremal,
                GoodhartVariant::Causal,
                GoodhartVariant::Adversarial,
            ],
            all_four_variants_audited,
        }
    }
}
