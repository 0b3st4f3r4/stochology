//! Bridge $L_3 \to L_1$: a **measured** lower bound on the Wasserstein-1 sensitivity $\varepsilon$
//! (Perdomo, Zrnic, Mendler-Dünner & Hardt 2020) from randomized action arms, in the spirit of measuring
//! performative power by experiment (Hardt, Jagadeesan & Mendler-Dünner 2022).

use crate::rng::{Rng, sample_std};

/// Exact Wasserstein-1 distance between two empirical samples of any sizes,
/// $W_1 = \int |F_P(x) - F_Q(x)|\,dx$, integrated over the merged order statistics.
pub fn exact_wasserstein_1(sample_p: &[f64], sample_q: &[f64]) -> f64 {
    assert!(!sample_p.is_empty() && !sample_q.is_empty());
    let mut sp = sample_p.to_vec();
    let mut sq = sample_q.to_vec();
    sp.sort_by(f64::total_cmp);
    sq.sort_by(f64::total_cmp);
    let (np, nq) = (sp.len() as f64, sq.len() as f64);
    let (mut i, mut j) = (0usize, 0usize);
    let mut x_prev = sp[0].min(sq[0]);
    let mut acc = 0.0;
    while i < sp.len() || j < sq.len() {
        let x_next = match (sp.get(i), sq.get(j)) {
            (Some(&a), Some(&b)) => a.min(b),
            (Some(&a), None) => a,
            (None, Some(&b)) => b,
            (None, None) => break,
        };
        acc += (i as f64 / np - j as f64 / nq).abs() * (x_next - x_prev);
        while i < sp.len() && sp[i] <= x_next {
            i += 1;
        }
        while j < sq.len() && sq[j] <= x_next {
            j += 1;
        }
        x_prev = x_next;
    }
    acc
}

/// Measured lower bound on the Wasserstein-1 sensitivity of a randomized binary action
/// $\theta \in \{0, 1\}$. With $\varepsilon$-sensitivity $W_1(\mathcal{D}(\theta), \mathcal{D}(\theta')) \le
/// \varepsilon\,|\theta - \theta'|$ (Perdomo, Zrnic, Mendler-Dünner & Hardt 2020, *ICML*, informative), and
/// $W_1 \ge |\mu_1 - \mu_0|$, randomized arms give, in control-SD units per unit of action,
/// $$\varepsilon \ge \frac{W_1(\mathcal{D}(1), \mathcal{D}(0))}{\mathrm{sd}_0} \ge \frac{|\mu_1 - \mu_0|}{\mathrm{sd}_0}.$$
/// Measuring power by randomized experiment follows Hardt, Jagadeesan & Mendler-Dünner (2022, *NeurIPS 35*,
/// informative). A permutation test on the plug-in $W_1$ (exact for equality of the arm distributions)
/// decides whether there is any effect. The plug-in $W_1$ is biased upward (the permutation null mean is
/// reported as its noise floor), so the bound is taken on the unbiased mean difference instead: the 5%
/// within-arm bootstrap quantile of $|\bar{y}_1 - \bar{y}_0| / \mathrm{sd}_0$ when the test rejects, else 0.
#[derive(Debug, Clone, PartialEq)]
pub struct PerformativePowerAudit {
    pub seed: u64,
    pub treated_n: usize,
    pub control_n: usize,
    pub control_sd: f64,
    /// Observed $W_1 / \mathrm{sd}_0$.
    pub w1_standardized: f64,
    /// $(1 + \#\{W_1^{\pi} \ge W_1\}) / (1 + B)$ over `PERMUTATIONS` label permutations.
    pub permutation_p_value: f64,
    /// Mean of the standardized $W_1$ over the label permutations: the plug-in noise floor.
    pub permutation_null_mean: f64,
    /// Observed $|\bar{y}_1 - \bar{y}_0| / \mathrm{sd}_0$.
    pub mean_difference_standardized: f64,
    /// 5% quantile of $|\bar{y}_1 - \bar{y}_0| / \mathrm{sd}_0$ over `BOOTSTRAPS` within-arm resamples.
    pub bootstrap_q05: f64,
    pub significant: bool,
    /// Measured lower bound on $\varepsilon$ (`bootstrap_q05` if significant, else 0).
    pub epsilon_lower_bound: f64,
}

/// Audit of a declared $\varepsilon$ against the measured lower bound, with the RRM contraction
/// condition $\varepsilon\beta/\gamma < 1$ re-evaluated at the bound.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EpsilonDeclarationAudit {
    pub declared_epsilon: f64,
    pub measured_lower_bound: f64,
    pub understated: bool,
    pub rrm_radius_declared: f64,
    pub rrm_radius_at_bound: f64,
    pub rrm_contracts_at_bound: bool,
}

impl PerformativePowerAudit {
    /// Assumed: label permutations of the noise-floor test.
    pub const PERMUTATIONS: usize = 999;
    /// Assumed: within-arm bootstrap resamples of the lower bound.
    pub const BOOTSTRAPS: usize = 999;
    /// Assumed: level of the permutation test.
    pub const ALPHA: f64 = 0.05;

    pub fn measure(treated: &[f64], control: &[f64], seed: u64) -> Self {
        assert!(treated.len() >= 2 && control.len() >= 2);
        let standardized = |t: &[f64], c: &[f64]| {
            let sd = sample_std(c);
            if sd > 0.0 {
                exact_wasserstein_1(t, c) / sd
            } else {
                0.0
            }
        };
        let mean_gap = |t: &[f64], c: &[f64]| {
            let sd = sample_std(c);
            if sd > 0.0 {
                (crate::rng::mean(t) - crate::rng::mean(c)).abs() / sd
            } else {
                0.0
            }
        };
        let observed = standardized(treated, control);
        let mut rng = Rng::new(seed);

        let mut pooled: Vec<f64> = treated.iter().chain(control).copied().collect();
        let n_t = treated.len();
        let mut exceed = 0usize;
        let mut null_sum = 0.0;
        for _ in 0..Self::PERMUTATIONS {
            rng.shuffle(&mut pooled);
            let w = standardized(&pooled[..n_t], &pooled[n_t..]);
            null_sum += w;
            if w >= observed {
                exceed += 1;
            }
        }
        let p_value = (1 + exceed) as f64 / (1 + Self::PERMUTATIONS) as f64;

        let resample = |rng: &mut Rng, xs: &[f64]| -> Vec<f64> {
            (0..xs.len())
                .map(|_| xs[(rng.next_u64() % xs.len() as u64) as usize])
                .collect()
        };
        let mut boot: Vec<f64> = (0..Self::BOOTSTRAPS)
            .map(|_| {
                let t = resample(&mut rng, treated);
                let c = resample(&mut rng, control);
                mean_gap(&t, &c)
            })
            .collect();
        boot.sort_by(f64::total_cmp);
        let q05 = crate::rng::quantile_sorted(&boot, 0.05);
        let significant = p_value <= Self::ALPHA;
        Self {
            seed,
            treated_n: treated.len(),
            control_n: control.len(),
            control_sd: sample_std(control),
            w1_standardized: observed,
            permutation_p_value: p_value,
            permutation_null_mean: null_sum / Self::PERMUTATIONS as f64,
            mean_difference_standardized: mean_gap(treated, control),
            bootstrap_q05: q05,
            significant,
            epsilon_lower_bound: if significant { q05 } else { 0.0 },
        }
    }

    /// Audits a card's declared $\varepsilon$ (same units) for a system with smoothness $\beta$ and
    /// strong convexity $\gamma$.
    pub fn audit_declaration(
        &self,
        declared_epsilon: f64,
        smoothness_beta: f64,
        strong_convexity_gamma: f64,
    ) -> EpsilonDeclarationAudit {
        assert!(strong_convexity_gamma > 0.0);
        let ratio = smoothness_beta / strong_convexity_gamma;
        let at_bound = self.epsilon_lower_bound * ratio;
        EpsilonDeclarationAudit {
            declared_epsilon,
            measured_lower_bound: self.epsilon_lower_bound,
            understated: declared_epsilon < self.epsilon_lower_bound,
            rrm_radius_declared: declared_epsilon.abs() * ratio,
            rrm_radius_at_bound: at_bound,
            rrm_contracts_at_bound: at_bound < 1.0,
        }
    }
}
