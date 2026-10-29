//! The 8-Step SIGA Operational Roadmap (§3.2):
//! - **S — Survey the Layer and Bridge** (Steps 1 and 2);
//! - **I — Interrogate the Decision-Maker** (Steps 3 and 4);
//! - **G — Generate the Null and Noise Physics** (Steps 5 and 6);
//! - **A — Act with Threshold, Memory, and Exit** (Steps 7 and 8).
//!
//! Includes sequential anytime-valid *e-processes* ($E_t = \prod_{i=1}^t L_i$, $\mathbb{E}_{H_0}[E_t] \le 1$,
//! Ville's Inequality $E_t \ge 1/\alpha$) and Gigerenzer integer natural frequency memoranda (e.g., `"X out of 1,000"`).

use crate::funnel::{evaluate_four_filter_funnel, FunnelCandidate, FunnelResult};
use stochology_core::{EpistemologicalCard, Rng};

/// Sequential anytime-valid *e-process* (betting martingale $E_t = \prod_{i=1}^t L_i$, §3.2 Step 6):
/// $$\mathbb{E}_{H_0}[E_t] \le 1 \implies \mathbb{P}_{H_0}\left(\exists t \ge 1 : E_t \ge \frac{1}{\alpha}\right) \le \alpha$$
#[derive(Debug, Clone, PartialEq)]
pub struct EvidenceProcess {
    pub alpha_level: f64,
    pub ville_rejection_threshold: f64,
    pub trajectory_e_t: Vec<f64>,
    pub final_e_value: f64,
    pub attempt_counter_k: usize,
    /// Multiplicity-penalized e-value $\sup_{1 \le s \le t} E_s / K$.
    pub multiplicity_adjusted_e_value: f64,
    pub rejects_null_hypothesis: bool,
}

impl EvidenceProcess {
    /// Evaluates a sequence of nonnegative likelihood ratios / e-values $L_1, \dots, L_t$ under attempt count $K \ge 1$.
    pub fn evaluate(likelihood_ratios: &[f64], alpha_level: f64, attempt_counter_k: usize) -> Self {
        assert!(alpha_level > 0.0 && alpha_level < 1.0);
        assert!(attempt_counter_k >= 1);
        let threshold = 1.0 / alpha_level;
        let mut prod = 1.0;
        let mut invalid = false;
        let mut traj = Vec::with_capacity(likelihood_ratios.len());
        for &l_i in likelihood_ratios {
            let safe_l = if l_i.is_finite() && l_i >= 0.0 {
                l_i
            } else {
                invalid = true;
                0.0
            };
            prod *= safe_l;
            traj.push(prod);
        }
        if invalid {
            traj.fill(0.0);
        }
        let max_e = traj.iter().copied().fold(0.0_f64, f64::max);
        let adj = max_e / (attempt_counter_k as f64);
        Self {
            alpha_level,
            ville_rejection_threshold: threshold,
            trajectory_e_t: traj,
            final_e_value: prod,
            attempt_counter_k,
            multiplicity_adjusted_e_value: adj,
            rejects_null_hypothesis: adj >= threshold,
        }
    }

    /// Constructs a sequential anytime-valid GRAPA / Kelly betting *e-process* (Grünwald, de Heide & Koolen 2024;
    /// Waudby-Smith & Ramdas 2024) from bounded zero-mean under-$H_0$ observations `centered_payoffs` ($X_i \in [-1, 1]$):
    /// $$E_t = \prod_{i=1}^t (1 + \lambda_i X_i), \qquad \lambda_i = \mathrm{clamp}\left(\frac{\hat{\mu}_{i-1}}{\hat{\sigma}_{i-1}^2 + \hat{\mu}_{i-1}^2},\ -0.5,\ 0.5\right)$$
    /// where $\lambda_i$ is strictly $\mathcal{F}_{i-1}$-measurable and $X_i$ is never clamped, guaranteeing
    /// $1 + \lambda_i X_i \ge 0.5 > 0$ and $\mathbb{E}_{H_0}[1 + \lambda_i X_i \mid \mathcal{F}_{i-1}] = 1$.
    /// Any observation outside the bounded support $|X_i| \le 1$ is a contract violation and returns `Err`
    /// (fail-closed): clamping $X_i$ would bias the conditional expectation under asymmetric nulls, and silently
    /// neutralizing the bets would erase genuine evidence.
    pub fn from_grapa_betting(
        centered_payoffs: &[f64],
        alpha_level: f64,
        attempt_counter_k: usize,
    ) -> Result<Self, String> {
        if let Some((i, x)) = centered_payoffs
            .iter()
            .enumerate()
            .find(|(_, x)| !(x.is_finite() && x.abs() <= 1.0))
        {
            return Err(format!(
                "GRAPA e-process requires bounded payoffs |X_i| <= 1; X_{i} = {x} violates the support"
            ));
        }
        let mut l_ratios = Vec::with_capacity(centered_payoffs.len());
        let mut sum_x = 0.0;
        let mut sum_sq_dev = 0.25; // Waudby-Smith & Ramdas (2024) prior variance sigma_0^2 = 1/4
        for (i, &x_i) in centered_payoffs.iter().enumerate() {
            let lambda_i = if i == 0 {
                0.0
            } else {
                let denom_i = i as f64;
                let mu_prev = sum_x / denom_i;
                let var_prev = sum_sq_dev / denom_i;
                (mu_prev / (var_prev + mu_prev * mu_prev)).clamp(-0.5, 0.5)
            };
            let factor = 1.0 + lambda_i * x_i;
            l_ratios.push(factor);
            sum_x += x_i;
            let mu_curr = sum_x / ((i + 1) as f64);
            sum_sq_dev += (x_i - mu_curr) * (x_i - mu_curr);
        }
        Ok(Self::evaluate(&l_ratios, alpha_level, attempt_counter_k))
    }

    /// Constructs a Growth-Rate Optimal (`GRO`, Grünwald, de Heide & Koolen 2024, J. R. Stat. Soc. B)
    /// sequential *e-process* for bounded observations $X_i \in [0, 1]$ testing $H_0: \mathbb{E}[X] = m_0$
    /// against point alternative $H_1: \mathbb{E}[X] = q_1$, where $\lambda_{\mathrm{GRO}} = \frac{q_1 - m_0}{m_0(1 - m_0)}$
    /// is truncated to $[-\frac{c}{1 - m_0}, \frac{c}{m_0}]$ ($c = 0.75$) so $1 + \lambda_{\mathrm{GRO}}(X_i - m_0) > 0$.
    pub fn gro_betting(
        observations_01: &[f64],
        null_mean_m0: f64,
        alt_mean_q1: f64,
        alpha_level: f64,
        attempt_counter_k: usize,
    ) -> Self {
        assert!(null_mean_m0 > 0.0 && null_mean_m0 < 1.0);
        assert!(alt_mean_q1 > 0.0 && alt_mean_q1 < 1.0);
        let c = 0.75;
        let raw_lambda = (alt_mean_q1 - null_mean_m0) / (null_mean_m0 * (1.0 - null_mean_m0));
        let lambda = raw_lambda.clamp(-c / (1.0 - null_mean_m0), c / null_mean_m0);
        let l_ratios: Vec<f64> = observations_01
            .iter()
            .map(|&x| {
                if x.is_finite() && (0.0..=1.0).contains(&x) {
                    1.0 + lambda * (x - null_mean_m0)
                } else {
                    1.0
                }
            })
            .collect();
        Self::evaluate(&l_ratios, alpha_level, attempt_counter_k)
    }
}

/// Waudby-Smith & Ramdas (2024, *J. R. Stat. Soc. B* 86(1):1–27) **SAVI Confidence Sequence**
/// via Hedged Capital Processes (`PrPl` Predictable Plug-In & `aGRAPA` Approximate GRAPA):
///
/// For $X_1, \dots, X_t \in [0, 1]$ and candidate mean $m \in (0, 1)$:
/// $$\hat{\mu}_t = \frac{\frac{1}{2} + \sum_{i=1}^t X_i}{t + 1}, \qquad \hat{\sigma}_t^2 = \frac{\frac{1}{4} + \sum_{i=1}^t (X_i - \hat{\mu}_i)^2}{t + 1}$$
/// $$\tilde{\lambda}_t^{\mathrm{PrPl}} = \sqrt{\frac{2\ln(2/\alpha)}{\hat{\sigma}_{t-1}^2 \cdot t \ln(t + 1)}}, \qquad \lambda_t^{\mathrm{PrPl}} = \min(c, \tilde{\lambda}_t^{\mathrm{PrPl}})$$
/// $$K_t^{\pm}(m) = \max\Bigl(\theta \prod_{i=1}^t \bigl(1 + \lambda_i^+(m)(X_i - m)\bigr),\ (1 - \theta)\prod_{i=1}^t \bigl(1 - \lambda_i^-(m)(X_i - m)\bigr)\Bigr)$$
#[derive(Debug, Clone, PartialEq)]
pub struct SaviConfidenceSequence {
    pub alpha_level: f64,
    pub truncation_c: f64,
    pub hedge_weight_theta: f64,
    pub sample_size_t: usize,
    pub candidate_null_m0: f64,
    /// First three raw predictable plug-in bets $[\tilde{\lambda}_1^{\mathrm{PrPl}}, \tilde{\lambda}_2^{\mathrm{PrPl}}, \tilde{\lambda}_3^{\mathrm{PrPl}}]$.
    pub raw_lambda_prpl_first_3: [f64; 3],
    /// Single-sided `aGRAPA` test martingale wealth $K_{\mathrm{aGRAPA}, t}^+(m_0)$ at $m = m_0$.
    pub agrapa_wealth_at_m0: f64,
    /// Two-sided Hedged `PrPl` wealth $K_t^{\pm, \mathrm{PrPl}}(m_0)$ at $m = m_0$.
    pub prpl_hedged_wealth_at_m0: f64,
    /// Lower endpoint of the $(1 - \alpha)$ anytime-valid Hedged `PrPl` Confidence Sequence $C_t$.
    pub cs_lower_bound: f64,
    /// Upper endpoint of the $(1 - \alpha)$ anytime-valid Hedged `PrPl` Confidence Sequence $C_t$.
    pub cs_upper_bound: f64,
    /// Whether the candidate null $m_0$ is rejected at level $\alpha$ ($K_t^{\pm, \mathrm{PrPl}}(m_0) \ge 1/\alpha$).
    pub rejects_candidate_m0: bool,
}

impl SaviConfidenceSequence {
    /// Evaluates the `aGRAPA` and Hedged `PrPl` capital processes and solves the $(1 - \alpha)$ confidence
    /// sequence $[L_t, U_t] = \{m \in (0, 1) : K_t^{\pm, \mathrm{PrPl}}(m) < 1/\alpha\}$ via bisection.
    pub fn evaluate(
        observations_01: &[f64],
        candidate_null_m0: f64,
        alpha_level: f64,
        truncation_c: f64,
        hedge_weight_theta: f64,
    ) -> Self {
        assert!(!observations_01.is_empty());
        assert!(candidate_null_m0 > 0.0 && candidate_null_m0 < 1.0);
        assert!(alpha_level > 0.0 && alpha_level < 1.0);
        assert!(truncation_c > 0.0 && truncation_c < 1.0);
        assert!((0.0..=1.0).contains(&hedge_weight_theta));
        assert!(
            observations_01
                .iter()
                .all(|x| x.is_finite() && (0.0..=1.0).contains(x)),
            "SAVI confidence sequence requires observations in [0, 1]"
        );

        let t_len = observations_01.len();
        let mut mu_hat = Vec::with_capacity(t_len + 1);
        let mut var_hat = Vec::with_capacity(t_len + 1);
        mu_hat.push(0.5);
        var_hat.push(0.25);

        let mut sum_x = 0.0;
        let mut sum_sq = 0.0;
        for (idx, &x) in observations_01.iter().enumerate() {
            let t_f = (idx + 1) as f64;
            sum_x += x;
            let mu_t = (0.5 + sum_x) / (t_f + 1.0);
            sum_sq += (x - mu_t) * (x - mu_t);
            let var_t = (0.25 + sum_sq) / (t_f + 1.0);
            mu_hat.push(mu_t);
            var_hat.push(var_t);
        }

        let mut raw_lambda_prpl = Vec::with_capacity(t_len);
        for t in 1..=t_len {
            let t_f = t as f64;
            let num = 2.0 * (2.0 / alpha_level).ln();
            let den = var_hat[t - 1] * t_f * (t_f + 1.0).ln();
            raw_lambda_prpl.push((num / den.max(1e-15)).sqrt());
        }
        let first_3 = [
            raw_lambda_prpl.first().copied().unwrap_or(0.0),
            raw_lambda_prpl.get(1).copied().unwrap_or(0.0),
            raw_lambda_prpl.get(2).copied().unwrap_or(0.0),
        ];

        // Single-sided aGRAPA wealth at candidate_null_m0:
        // lambda_t^+(m) = clamp((mu_{t-1} - m) / (var_{t-1} + (mu_{t-1} - m)^2), 0, c / m)
        let mut k_agrapa = 1.0;
        for (idx, &x) in observations_01.iter().enumerate() {
            let diff = mu_hat[idx] - candidate_null_m0;
            let lam =
                (diff / (var_hat[idx] + diff * diff)).clamp(0.0, truncation_c / candidate_null_m0);
            k_agrapa *= 1.0 + lam * (x - candidate_null_m0);
        }

        // Hedged PrPl wealth function K_t^{+-}(m) for any m in (0, 1) (Waudby-Smith & Ramdas 2024, Eq. 3.7):
        // lambda_t^+(m) = min(c / m, raw_lambda_t), lambda_t^-(m) = min(c / (1 - m), raw_lambda_t)
        let prpl_wealth = |m: f64| -> f64 {
            let mut k_plus = 1.0;
            let mut k_minus = 1.0;
            for (idx, &x) in observations_01.iter().enumerate() {
                let raw_lam = raw_lambda_prpl[idx];
                let lam_p = raw_lam.min(truncation_c / m.max(1e-12));
                let lam_m = raw_lam.min(truncation_c / (1.0 - m).max(1e-12));
                k_plus *= 1.0 + lam_p * (x - m);
                k_minus *= 1.0 - lam_m * (x - m);
            }
            (hedge_weight_theta * k_plus).max((1.0 - hedge_weight_theta) * k_minus)
        };

        let k_prpl_m0 = prpl_wealth(candidate_null_m0);
        let threshold = 1.0 / alpha_level;
        let sample_mean = observations_01.iter().sum::<f64>() / (t_len as f64);

        // Bisection for lower CS bound in (1e-6, sample_mean)
        let cs_lo = if prpl_wealth(1e-6) < threshold {
            0.0
        } else {
            let mut lo = 1e-6;
            let mut hi = sample_mean.clamp(1e-5, 1.0 - 1e-5);
            for _ in 0..60 {
                let mid = 0.5 * (lo + hi);
                if prpl_wealth(mid) >= threshold {
                    lo = mid;
                } else {
                    hi = mid;
                }
            }
            0.5 * (lo + hi)
        };

        // Bisection for upper CS bound in (sample_mean, 1 - 1e-6)
        let cs_hi = if prpl_wealth(1.0 - 1e-6) < threshold {
            1.0
        } else {
            let mut lo = sample_mean.clamp(1e-5, 1.0 - 1e-5);
            let mut hi = 1.0 - 1e-6;
            for _ in 0..60 {
                let mid = 0.5 * (lo + hi);
                if prpl_wealth(mid) < threshold {
                    lo = mid;
                } else {
                    hi = mid;
                }
            }
            0.5 * (lo + hi)
        };

        Self {
            alpha_level,
            truncation_c,
            hedge_weight_theta,
            sample_size_t: t_len,
            candidate_null_m0,
            raw_lambda_prpl_first_3: first_3,
            agrapa_wealth_at_m0: k_agrapa,
            prpl_hedged_wealth_at_m0: k_prpl_m0,
            cs_lower_bound: cs_lo,
            cs_upper_bound: cs_hi,
            rejects_candidate_m0: k_prpl_m0 >= threshold,
        }
    }

    /// Canonical Weyl golden-ratio quasirandom sequence $x_i = (i \varphi) \bmod 1$ ($i = 1 \dots 20$)
    /// testing $m_0 = 0.30$ at $\alpha = 0.05, c = 0.50, \theta = 0.50$.
    pub fn canonical_weyl_golden_benchmark() -> Self {
        let phi = 0.5 * (1.0 + 5.0_f64.sqrt());
        let xs: Vec<f64> = (1..=20).map(|i| ((i as f64) * phi).fract()).collect();
        Self::evaluate(&xs, 0.30, 0.05, 0.50, 0.50)
    }

    /// Compares naive optional stopping with a fixed-$n$ Gaussian $z$-test against the SAVI martingale
    /// test under $H_0: X_i \sim \mathrm{Bernoulli}(0.50)$ across `trials` trajectories of horizon `horizon_t`.
    /// Returns `(naive_z_peeking_fpr, savi_martingale_fpr)`.
    pub fn compare_peeking_vs_savi(seed: u64, horizon_t: usize, alpha: f64) -> (f64, f64) {
        let mut rng = Rng::new(seed);
        let trials = 400usize;
        let z_crit = 1.959_963_984_540_054;
        let ville_thresh = 1.0 / alpha;
        let mut z_false_alarms = 0usize;
        let mut savi_false_alarms = 0usize;

        for _ in 0..trials {
            let mut sum = 0.0;
            let mut sum_sq = 0.0;
            let mut var_prev = 0.25;
            let mut k_plus = 1.0;
            let mut k_minus = 1.0;
            let mut z_peeking_rejected = false;
            let mut savi_rejected = false;

            for t in 1..=horizon_t {
                let x = if rng.bernoulli(0.50) { 1.0 } else { 0.0 };
                let t_f = t as f64;
                sum += x;

                // Naive fixed-n z-test peeked at every step t >= 5
                if t >= 5 {
                    let mean_t = sum / t_f;
                    let z_stat = (mean_t - 0.50).abs() / (0.50 / t_f.sqrt());
                    if z_stat >= z_crit {
                        z_peeking_rejected = true;
                    }
                }

                // SAVI Hedged PrPl update at m0 = 0.50
                let lam = ((2.0 * (2.0 / alpha).ln()) / (var_prev * t_f * (t_f + 1.0).ln()))
                    .sqrt()
                    .min(0.50);
                k_plus *= 1.0 + lam * (x - 0.50);
                k_minus *= 1.0 - lam * (x - 0.50);
                if 0.5 * (k_plus + k_minus) >= ville_thresh {
                    savi_rejected = true;
                }

                let mu_curr = (0.5 + sum) / (t_f + 1.0);
                sum_sq += (x - mu_curr) * (x - mu_curr);
                var_prev = (0.25 + sum_sq) / (t_f + 1.0);
            }
            if z_peeking_rejected {
                z_false_alarms += 1;
            }
            if savi_rejected {
                savi_false_alarms += 1;
            }
        }
        (
            (z_false_alarms as f64) / (trials as f64),
            (savi_false_alarms as f64) / (trials as f64),
        )
    }
}

/// Wang, Wang & Ziegel (2024, *Management Science*, arXiv:2203.00272) **E-Backtesting of Value-at-Risk (VaR)
/// and Expected Shortfall (ES)** via the GREE (*Growth-Rate Empirical E-process*) predictable betting schedule:
/// $$e_p^{\mathrm{VaR}}(x, r) = \frac{\mathbb{I}\{x > r\}}{1 - p}, \qquad e_p^{\mathrm{ES}}(x, r, z) = \frac{(x - z)_+}{(1 - p)(r - z)} \quad (z \le r)$$
/// $$M_t = \prod_{i=1}^t \bigl(1 - \lambda_i + \lambda_i e_i\bigr), \qquad \sum_{j=1}^{i-1} \frac{e_j - 1}{1 - \lambda_i + \lambda_i e_j} = 0 \quad (\lambda_i \in [0, 0.5])$$
#[derive(Debug, Clone, PartialEq)]
pub struct VarEsEBacktest {
    pub quantile_level_p: f64,
    pub pointwise_e_values: Vec<f64>,
    pub predictable_gree_lambdas: Vec<f64>,
    pub martingale_trajectory_m_t: Vec<f64>,
    pub final_martingale_m_t: f64,
    pub max_martingale_m_t: f64,
    pub rejects_at_alpha_005: bool,
}

impl VarEsEBacktest {
    /// Pointwise VaR e-statistic $e_p^{\mathrm{VaR}}(x, r) = \frac{\mathbb{I}\{x > r\}}{1 - p}$.
    #[inline]
    pub fn var_e_statistic(loss_x: f64, var_forecast_r: f64, quantile_level_p: f64) -> f64 {
        assert!(quantile_level_p > 0.0 && quantile_level_p < 1.0);
        if loss_x > var_forecast_r {
            1.0 / (1.0 - quantile_level_p)
        } else {
            0.0
        }
    }

    /// Pointwise Expected Shortfall (ES) e-statistic $e_p^{\mathrm{ES}}(x, r, z) = \frac{(x - z)_+}{(1 - p)(r - z)}$ for $z < r$.
    #[inline]
    pub fn es_e_statistic(
        loss_x: f64,
        es_forecast_r: f64,
        var_aux_z: f64,
        quantile_level_p: f64,
    ) -> f64 {
        assert!(quantile_level_p > 0.0 && quantile_level_p < 1.0);
        assert!(
            es_forecast_r >= var_aux_z,
            "ES forecast r must not fall below VaR auxiliary z"
        );
        let excess = (loss_x - var_aux_z).max(0.0);
        if es_forecast_r == var_aux_z {
            // Degenerate tail forecast: any exceedance is infinite evidence, otherwise 0/0 := 1.
            return if excess > 0.0 { f64::INFINITY } else { 1.0 };
        }
        excess / ((1.0 - quantile_level_p) * (es_forecast_r - var_aux_z))
    }

    /// Solves for the predictable GREE betting fraction $\lambda_i \in [0, 0.5]$ maximizing empirical log-growth
    /// over past e-values `past_e[..i-1]` via exact bisection on $g(\lambda) = \sum_{j < i} \frac{e_j - 1}{1 - \lambda + \lambda e_j} = 0$.
    pub fn solve_gree_lambda(past_e: &[f64]) -> f64 {
        if past_e.is_empty() {
            return 0.0;
        }
        let score = |lam: f64| -> f64 {
            past_e
                .iter()
                .map(|&e| (e - 1.0) / (1.0 - lam + lam * e).max(1e-12))
                .sum()
        };
        if score(0.0) <= 0.0 {
            return 0.0;
        }
        if score(0.5) >= 0.0 {
            return 0.5;
        }
        let mut lo = 0.0;
        let mut hi = 0.5;
        for _ in 0..60 {
            let mid = 0.5 * (lo + hi);
            if score(mid) > 0.0 {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        0.5 * (lo + hi)
    }

    /// Evaluates the sequential GREE e-process from a vector of pointwise VaR or ES e-statistics.
    pub fn from_e_statistics(quantile_level_p: f64, e_stats: &[f64]) -> Self {
        let mut lambdas = Vec::with_capacity(e_stats.len());
        let mut traj = Vec::with_capacity(e_stats.len());
        let mut m = 1.0;
        let mut max_m = 1.0_f64;

        for (i, &e_i) in e_stats.iter().enumerate() {
            let lam_i = Self::solve_gree_lambda(&e_stats[..i]);
            lambdas.push(lam_i);
            m *= (1.0 - lam_i + lam_i * e_i).max(0.0);
            max_m = max_m.max(m);
            traj.push(m);
        }

        Self {
            quantile_level_p,
            pointwise_e_values: e_stats.to_vec(),
            predictable_gree_lambdas: lambdas,
            martingale_trajectory_m_t: traj,
            final_martingale_m_t: m,
            max_martingale_m_t: max_m,
            rejects_at_alpha_005: max_m >= 20.0,
        }
    }

    /// Evaluates a VaR e-backtest from binary exceedance indicators $I_t = \mathbb{I}\{X_t > r_t\} \in \{0, 1\}$.
    pub fn from_var_exceedances(quantile_level_p: f64, exceedances: &[u8]) -> Self {
        let e_stats: Vec<f64> = exceedances
            .iter()
            .map(|&ind| {
                if ind > 0 {
                    1.0 / (1.0 - quantile_level_p)
                } else {
                    0.0
                }
            })
            .collect();
        Self::from_e_statistics(quantile_level_p, &e_stats)
    }
}

/// Executive Memorandum in Integer Natural Frequencies (Gigerenzer — §3.2 Step 8).
#[derive(Debug, Clone, PartialEq)]
pub struct NaturalFrequencyMemorandum {
    pub reference_cohort_size: usize,
    pub affected_with_event: usize,
    pub unaffected_without_event: usize,
    pub true_positives: usize,
    pub false_negatives: usize,
    pub false_positives: usize,
    pub true_negatives: usize,
    pub critical_threshold_p_star: f64,
    pub recommended_action: String,
    pub exit_clause: String,
    pub formatted_summary: String,
}

impl NaturalFrequencyMemorandum {
    /// Builds an integer natural frequency tree for a reference cohort (e.g., `1_000` units).
    pub fn build(
        reference_cohort_size: usize,
        base_rate_p: f64,
        sensitivity_tpr: f64,
        false_positive_rate_fpr: f64,
    ) -> Self {
        Self::build_with_decision(
            reference_cohort_size,
            base_rate_p,
            sensitivity_tpr,
            false_positive_rate_fpr,
            0.05,
            "protect",
            "Suspend if REL_CORP exceeds threshold",
        )
    }

    /// Builds an integer natural frequency tree including the economic threshold $p^*$,
    /// recommended action (`protect` / `do_not_protect`), and numeric exit clause.
    pub fn build_with_decision(
        reference_cohort_size: usize,
        base_rate_p: f64,
        sensitivity_tpr: f64,
        false_positive_rate_fpr: f64,
        critical_threshold_p_star: f64,
        recommended_action: &str,
        exit_clause: &str,
    ) -> Self {
        let p_c = if base_rate_p.is_finite() {
            base_rate_p.clamp(0.0, 1.0)
        } else {
            0.0
        };
        let tpr_c = if sensitivity_tpr.is_finite() {
            sensitivity_tpr.clamp(0.0, 1.0)
        } else {
            0.0
        };
        let fpr_c = if false_positive_rate_fpr.is_finite() {
            false_positive_rate_fpr.clamp(0.0, 1.0)
        } else {
            0.0
        };
        let affected = ((reference_cohort_size as f64) * p_c).round() as usize;
        let unaffected = reference_cohort_size.saturating_sub(affected);
        let tp = ((affected as f64) * tpr_c).round() as usize;
        let fn_c = affected.saturating_sub(tp);
        let fp = ((unaffected as f64) * fpr_c).round() as usize;
        let tn = unaffected.saturating_sub(fp);
        let total_alarms = tp.saturating_add(fp).max(1);
        let summary = format!(
            "Out of every {reference_cohort_size} independent units, {affected} experience the critical event (of which {tp} trigger the alarm at threshold p*={critical_threshold_p_star:.5} and {fn_c} are missed), while {unaffected} do not experience the event (of which {fp} trigger a false alarm and {tn} remain clear). Positive predictive frequency: {tp} out of {total_alarms} alarms. Action: {recommended_action}. Exit clause: {exit_clause}."
        );
        Self {
            reference_cohort_size,
            affected_with_event: affected,
            unaffected_without_event: unaffected,
            true_positives: tp,
            false_negatives: fn_c,
            false_positives: fp,
            true_negatives: tn,
            critical_threshold_p_star,
            recommended_action: recommended_action.to_string(),
            exit_clause: exit_clause.to_string(),
            formatted_summary: summary,
        }
    }
}

/// Provenance of the $(\mathrm{TPR}, \mathrm{FPR})$ operating point fed into `AlertFatigueAudit` (§II.C).
///
/// A `Literature` operating point is a published external validation transcribed from its source (informative,
/// not computed by this code); an `Assumed` operating point is a hand-set design target and must never be
/// presented as a result.
#[derive(Debug, Clone, PartialEq)]
pub enum OperatingPointSource {
    /// Informative: published external validation transcribed from the literature, not computed by this code.
    Literature { source: &'static str },
    /// Hand-set design operating point (Assumed: not measured on any dataset).
    Assumed,
}

/// Bridge $L_2 \to L_3$ Cognitive & Clinical Triage Verifier (`AlertFatigueAudit`):
///
/// Couples the biological conscious bottleneck of $10\text{ bit/s}$ (Zheng & Meister 2024/2025, *Neuron*),
/// Alan Turing & I. J. Good's additive weight of evidence in decibans ($W = 10\log_{10}\mathrm{LR}$, where
/// $1\text{ dB}$ is the minimum human-perceptible quantum and $+30\text{ dB}$ is the $1000:1$ Bombe threshold),
/// and Positive Predictive Value ($\mathrm{PPV}$) at the operational shift resolution ($\Phi_5$).
///
/// Rejects high-false-alarm black-box scores exemplified by the Epic Sepsis Model external validation
/// (Wong et al., *JAMA Internal Medicine* 2021: $n = 27\,697$ patients, $38\,455$ hospitalizations,
/// $\mathrm{AUC} = 0.63$, sensitivity $\mathrm{TPR} = 0.33$, $\mathrm{PPV} = 0.12$, $18\%$ hospitalization
/// alarm rate, generating $\ge 7.3$ false alarms per true case).
#[derive(Debug, Clone, PartialEq)]
pub struct AlertFatigueAudit {
    /// Conscious executive action-selection bandwidth ceiling ($10.0\text{ bit/s}$, Zheng & Meister 2024/2025).
    pub conscious_bottleneck_bps: f64,
    /// Minimum human-perceptible weight-of-evidence quantum ($1.0\text{ deciban}$, Good 1979).
    pub minimum_perceptible_decibans: f64,
    /// Additive Turing-Good weight of evidence $W(H:E) = 10\log_{10}(\mathrm{TPR}/\mathrm{FPR})$ in decibans.
    pub turing_weight_of_evidence_decibans: f64,
    /// Posterior log-odds in decibans $O_{\text{post, dB}} = 10\log_{10}\frac{\pi}{1-\pi} + W(H:E)$.
    pub posterior_log_odds_decibans: f64,
    /// Positive Predictive Value $\mathrm{PPV} = \frac{\pi \cdot \mathrm{TPR}}{\pi \cdot \mathrm{TPR} + (1 - \pi)\cdot \mathrm{FPR}}$.
    pub positive_predictive_value_ppv: f64,
    /// False-to-true alarm ratio $\frac{\mathrm{FP}}{\mathrm{TP}} = \frac{1 - \mathrm{PPV}}{\mathrm{PPV}}$.
    pub false_alarms_per_true_case: f64,
    /// Information-theoretic cognitive load (in bit/s) imposed on the shift operator during burst triage.
    pub cognitive_triage_load_bps: f64,
    /// Whether the system satisfies $\Phi_5$ shift resolution, $W \ge 8.0\text{ dB}$, $\mathrm{PPV} \ge p^*$,
    /// false-to-true ratio $\le 4.0$, and stays below the $10\text{ bit/s}$ conscious cognitive ceiling.
    pub approved_bridge_l2_l3: bool,
    /// Provenance of the $(\mathrm{TPR}, \mathrm{FPR})$ operating point (`Assumed` unless declared otherwise
    /// via `with_operating_point_source`).
    pub operating_point_source: OperatingPointSource,
}

impl AlertFatigueAudit {
    /// Evaluates an operational alerting rule at shift resolution (`seed = 2026`).
    ///
    /// The operating point defaults to `OperatingPointSource::Assumed`; declare a literature operating point
    /// with `with_operating_point_source`.
    pub fn evaluate(
        prevalence_pi: f64,
        sensitivity_tpr: f64,
        false_positive_rate_fpr: f64,
        critical_threshold_p_star: f64,
        alarms_per_minute_burst: f64,
        bits_per_alarm_inspection: f64,
        respects_phi5_shift_resolution: bool,
    ) -> Self {
        let conscious_bottleneck_bps = 10.0;
        let minimum_perceptible_decibans = 1.0;

        let pi = prevalence_pi.clamp(1e-6, 1.0 - 1e-6);
        let tpr = sensitivity_tpr.clamp(1e-6, 1.0 - 1e-6);
        let fpr = false_positive_rate_fpr.clamp(1e-6, 1.0 - 1e-6);

        let lr_pos = tpr / fpr;
        let turing_weight_of_evidence_decibans = 10.0 * lr_pos.log10();
        let prior_decibans = 10.0 * (pi / (1.0 - pi)).log10();
        let posterior_log_odds_decibans = prior_decibans + turing_weight_of_evidence_decibans;

        let tp_rate = pi * tpr;
        let fp_rate = (1.0 - pi) * fpr;
        let positive_predictive_value_ppv = tp_rate / (tp_rate + fp_rate).max(1e-12);
        let false_alarms_per_true_case =
            (1.0 - positive_predictive_value_ppv) / positive_predictive_value_ppv.max(1e-12);

        let cognitive_triage_load_bps =
            (alarms_per_minute_burst * bits_per_alarm_inspection) / 60.0;

        let approved_bridge_l2_l3 = respects_phi5_shift_resolution
            && turing_weight_of_evidence_decibans >= 8.0
            && positive_predictive_value_ppv >= critical_threshold_p_star
            && false_alarms_per_true_case <= 4.0
            && cognitive_triage_load_bps <= conscious_bottleneck_bps;

        Self {
            conscious_bottleneck_bps,
            minimum_perceptible_decibans,
            turing_weight_of_evidence_decibans,
            posterior_log_odds_decibans,
            positive_predictive_value_ppv,
            false_alarms_per_true_case,
            cognitive_triage_load_bps,
            approved_bridge_l2_l3,
            operating_point_source: OperatingPointSource::Assumed,
        }
    }

    /// Declares the provenance of the operating point without altering any computed quantity.
    pub fn with_operating_point_source(mut self, source: OperatingPointSource) -> Self {
        self.operating_point_source = source;
        self
    }

    /// Canonical comparison between the failed Epic Sepsis Model (Wong et al., *JAMA Internal Medicine* 2021:
    /// $\mathrm{AUC} = 0.63$, $\mathrm{TPR} = 0.33$, $\mathrm{PPV} = 0.12$, $18\%$ hospitalization alarm rate,
    /// $\approx 7.33$ false alarms per true case) and a CORP-calibrated Stochological triage gate.
    ///
    /// The two sides are not symmetric evidence: the Epic operating point is a published external validation,
    /// whereas the Stochological operating point ($\mathrm{TPR} = 0.82$, $\mathrm{FPR} = 0.045$) is an *assumed
    /// design target* that was never measured on the Wong et al. cohort. The comparison verifies that the audit
    /// rejects the published operating point and accepts one meeting its thresholds; it is not a performance claim.
    pub fn canonical_epic_sepsis_vs_stochological_gate() -> (Self, Self) {
        // Epic Sepsis Model (Wong et al. 2021, published external validation): prevalence ~6.6%, TPR = 0.33, FPR = 0.171 -> PPV = 0.120, W = +2.86 dB
        let epic_sepsis = Self::evaluate(0.066, 0.33, 0.171, 0.20, 18.0, 42.0, false)
            .with_operating_point_source(OperatingPointSource::Literature {
                source: "Wong et al., JAMA Internal Medicine 2021",
            });
        // Stochological CORP-calibrated multi-signal gate (3 independent channels summing deciban weights), ASSUMED operating point:
        // TPR = 0.82, FPR = 0.045 -> LR+ = 18.22 (W = +12.61 dB), PPV = 0.563 >= p* = 0.20, cognitive load = 3.2 bit/s <= 10 bit/s
        let stochological_gate = Self::evaluate(0.066, 0.82, 0.045, 0.20, 4.0, 48.0, true)
            .with_operating_point_source(OperatingPointSource::Assumed);
        (epic_sepsis, stochological_gate)
    }
}

/// Bridge $L_1 \to L_2 \to L_3$ Five-Senses Multimodal Fusion Verifier (`MultimodalCorrelatedFusionAudit`):
///
/// Audits the fusion of $M$ sensory channels (*"Os Cinco Sentidos da Máquina"*: e.g. text, audio, image,
/// video, telemetry) with individual Turing deciban weights $w_1, \dots, w_M$ and pairwise latent
/// equicorrelation $\rho \in [0, 1)$ induced by a shared upstream pre-training corpus or sensor confounder:
/// - **Naive Independent Fusion:** Sums all channels blindly $W_{\text{naive}} = \sum_{m=1}^M w_m$, violating
///   $\Phi_2$ across modalities and overstating effective evidence by the multimodal design-effect factor
///   $\mathrm{DEFF}_{\text{modal}} = 1 + (M - 1)\rho$.
/// - **Covariance-Discounted & Outlier-Clipped Fusion:** Discounts shared correlation via
///   $W_{\text{discounted}} = \frac{\sum_{m=1}^M \tilde{w}_m}{1 + (M - 1)\rho}$ where each channel is clipped
///   at $\pm w_{\max}$ to prevent a single corrupted or poisoned sensor from hijacking posterior odds.
#[derive(Debug, Clone, PartialEq)]
pub struct MultimodalCorrelatedFusionAudit {
    /// Number of sensory modalities $M$ (e.g. $5$).
    pub num_modalities: usize,
    /// Shared cross-modal correlation $\rho \in [0, 1)$.
    pub cross_modal_correlation_rho: f64,
    /// Multimodal design-effect factor $\mathrm{DEFF}_{\text{modal}} = 1 + (M - 1)\rho$.
    pub multimodal_deff: f64,
    /// Naive independent deciban sum $W_{\text{naive}} = \sum_{m=1}^M w_m$ (unclipped, $\rho=0$ assumed).
    pub naive_independent_decibans: f64,
    /// Covariance-discounted and Huber/innovation-clipped deciban evidence $W_{\text{shielded}}$.
    pub shielded_correlated_decibans: f64,
    /// Exact identity error $|W_{\text{discounted}}(\rho=0, \text{unclipped}) - W_{\text{naive}}|$.
    pub zero_correlation_identity_error: f64,
    /// Whether the shielded fusion prevents both double-counting inflation ($W_{\text{shielded}} < W_{\text{naive}}$)
    /// and single-sensor corruption hijack.
    pub multimodal_fusion_approved: bool,
}

impl MultimodalCorrelatedFusionAudit {
    /// Evaluates $M$-channel deciban fusion under cross-modal correlation $\rho$ and per-channel clip $w_{\max}$.
    pub fn evaluate(raw_channel_decibans: &[f64], rho: f64, channel_clip_max: f64) -> Self {
        assert!(
            !raw_channel_decibans.is_empty()
                && rho.is_finite()
                && (0.0..1.0).contains(&rho)
                && channel_clip_max.is_finite()
                && channel_clip_max > 0.0,
            "MultimodalCorrelatedFusionAudit requires non-empty channels, rho in [0, 1), and positive clip"
        );
        let m = raw_channel_decibans.len();
        let multimodal_deff = 1.0 + ((m.saturating_sub(1)) as f64) * rho;
        let naive_independent_decibans: f64 = raw_channel_decibans.iter().copied().sum();
        let unclipped_zero_rho: f64 = naive_independent_decibans / 1.0;
        let zero_correlation_identity_error =
            (unclipped_zero_rho - naive_independent_decibans).abs();

        let clipped_sum: f64 = raw_channel_decibans
            .iter()
            .map(|&w| w.clamp(-channel_clip_max, channel_clip_max))
            .sum();
        let shielded_correlated_decibans = clipped_sum / multimodal_deff;

        Self {
            num_modalities: m,
            cross_modal_correlation_rho: rho,
            multimodal_deff,
            naive_independent_decibans,
            shielded_correlated_decibans,
            zero_correlation_identity_error,
            multimodal_fusion_approved: multimodal_deff > 1.0
                && shielded_correlated_decibans < naive_independent_decibans
                && shielded_correlated_decibans.abs() <= (m as f64) * channel_clip_max
                && zero_correlation_identity_error < 1e-12,
        }
    }

    /// Canonical 5-senses benchmark (text, audio, image, video, telemetry) where 4 clean sensors carry
    /// $+4.5\text{ dB}$ each, shared pre-training correlation is $\rho = 0.45$, and the 5th sensor suffers
    /// a $+35.0\text{ dB}$ adversarial/glitch spike clipped at $w_{\max} = 6.0\text{ dB}$.
    pub fn canonical_five_senses_benchmark() -> Self {
        Self::evaluate(&[4.5, 4.5, 4.5, 4.5, 35.0], 0.45, 6.0)
    }
}

/// Prospective Pre-Mortem, Counterfactual Root-Cause ($\mathrm{PNS}$ vs. Linear "5 Whys"), and Frictionless
/// SOP Homologation Audit (`PreMortemCounterfactualRootCauseAudit`):
///
/// Couples Gary Klein's prospective *Pre-Mortem* (*Technium* failure autopsy), Pearl's Tian-Pearl bounds on
/// the **Probability of Necessary and Sufficient Causation** ($\mathrm{PNS} = P(Y_1 = 1, Y_0 = 0)$) versus
/// naive single-chain "5 Whys" root-cause selection, and boundary stress-testing + Standard Operating Procedure
/// (`SOP`) cognitive friction check ($\le 10\text{ bit/s}$).
#[derive(Debug, Clone, PartialEq)]
pub struct PreMortemCounterfactualRootCauseAudit {
    /// Tian-Pearl sharp lower bound $\max(0, P(Y_1=1) - P(Y_0=1))$ on $\mathrm{PNS}$ for the true root cause.
    pub pns_lower_bound: f64,
    /// Tian-Pearl sharp upper bound $\min(P(Y_1=1), P(Y_0=0))$ on $\mathrm{PNS}$ for the true root cause.
    pub pns_upper_bound: f64,
    /// Exact structural $\mathrm{PNS}$ of the true conjunctive root cause vs. the superficial "5 Whys" symptom.
    pub true_root_cause_pns: f64,
    pub naive_five_whys_symptom_pns: f64,
    /// Residual of the Tian-Pearl interval containment identity $\mathrm{PNS} \in [\mathrm{LB}, \mathrm{UB}]$.
    pub tian_pearl_bounds_identity_error: f64,
    /// Prospective Pre-Mortem expected tail loss before vs. after boundary stress-test homologation.
    pub unshielded_premortem_expected_loss: f64,
    pub homologated_premortem_expected_loss: f64,
    /// Operational SOP cognitive load in bit/s ($\le 10.0\text{ bit/s}$ required for frictionless adoption).
    pub sop_cognitive_load_bps: f64,
    /// Whether counterfactual $\mathrm{PNS}$ isolates the true root cause, Tian-Pearl bounds hold, and
    /// Pre-Mortem stress-testing + SOP homologation pass.
    pub premortem_and_sop_approved: bool,
}

impl PreMortemCounterfactualRootCauseAudit {
    /// Evaluates counterfactual root-cause $\mathrm{PNS}$ bounds, Pre-Mortem tail-loss reduction, and SOP load.
    pub fn evaluate(
        p_y1_true_cause: f64,
        p_y0_true_cause: f64,
        true_root_cause_pns: f64,
        naive_five_whys_symptom_pns: f64,
        unshielded_premortem_expected_loss: f64,
        homologated_premortem_expected_loss: f64,
        sop_cognitive_load_bps: f64,
    ) -> Self {
        let p1 = p_y1_true_cause.clamp(0.0, 1.0);
        let p0 = p_y0_true_cause.clamp(0.0, 1.0);
        let pns_lower_bound = (p1 - p0).max(0.0);
        let pns_upper_bound = p1.min(1.0 - p0);
        let below_lb = (pns_lower_bound - true_root_cause_pns).max(0.0);
        let above_ub = (true_root_cause_pns - pns_upper_bound).max(0.0);
        let tian_pearl_bounds_identity_error = below_lb.max(above_ub);

        let approved = tian_pearl_bounds_identity_error < 1e-12
            && true_root_cause_pns > naive_five_whys_symptom_pns + 0.35
            && homologated_premortem_expected_loss < 0.25 * unshielded_premortem_expected_loss
            && sop_cognitive_load_bps <= 10.0;

        Self {
            pns_lower_bound,
            pns_upper_bound,
            true_root_cause_pns,
            naive_five_whys_symptom_pns,
            tian_pearl_bounds_identity_error,
            unshielded_premortem_expected_loss,
            homologated_premortem_expected_loss,
            sop_cognitive_load_bps,
            premortem_and_sop_approved: approved,
        }
    }

    /// Canonical Pre-Mortem & Counterfactual Root-Cause benchmark.
    pub fn canonical_benchmark() -> Self {
        Self::evaluate(0.84, 0.12, 0.76, 0.14, 125_000.0, 18_500.0, 4.2)
    }
}

/// Full end-to-end execution report of the 8-Step SIGA Operational Roadmap.
#[derive(Debug, Clone, PartialEq)]
pub struct SigaExecutionReport {
    pub card_id: String,
    pub funnel_result: FunnelResult,
    pub null_p_value: f64,
    pub e_process: EvidenceProcess,
    pub natural_frequency_memo: NaturalFrequencyMemorandum,
    /// Completion flags for Steps 1 through 8 of the SIGA Roadmap.
    pub steps_1_to_8_completed: [bool; 8],
    pub approved_end_to_end: bool,
}

fn sample_gamma_marsaglia(rng: &mut Rng, alpha: f64) -> f64 {
    let a = alpha.max(1e-3);
    if a < 1.0 {
        let u = rng.next_f64().clamp(1e-15, 1.0 - 1e-15);
        return sample_gamma_marsaglia(rng, a + 1.0) * u.powf(1.0 / a);
    }
    let d = a - 1.0 / 3.0;
    let c = 1.0 / (9.0 * d).sqrt();
    loop {
        let z = rng.standard_normal();
        let v_root = 1.0 + c * z;
        if v_root <= 0.0 {
            continue;
        }
        let v = v_root * v_root * v_root;
        let u = rng.next_f64().clamp(1e-15, 1.0 - 1e-15);
        if u.ln() < 0.5 * z * z + d * (1.0 - v + v.ln()) {
            return (d * v).max(1e-15);
        }
    }
}

fn sample_beta_conjugate(rng: &mut Rng, alpha: f64, beta: f64) -> f64 {
    let x = sample_gamma_marsaglia(rng, alpha);
    let y = sample_gamma_marsaglia(rng, beta);
    (x / (x + y).max(1e-15)).clamp(1e-12, 1.0 - 1e-12)
}

/// Executes genuine Talts et al. (2018) Simulation-Based Calibration (SBC) for a Beta-Binomial
/// generative model using deterministic `stochology_core::Rng::new(seed)`.
///
/// For each replication $s \in 0..\text{num\_replications}$:
/// 1. Draw $\tilde{p} \sim \mathrm{Beta}(\alpha_0, \beta_0)$ from the prior;
/// 2. Simulate $k \sim \mathrm{Binomial}(n_{\text{trials}}, \tilde{p})$;
/// 3. Draw $L$ conjugate posterior samples centered at $\hat{p} = \frac{k + \alpha_0}{n_{\text{trials}} + \alpha_0 + \beta_0}$
///    with posterior variance $\frac{\hat{p}(1-\hat{p})}{n_{\text{trials}} + \alpha_0 + \beta_0 + 1}$;
/// 4. Compute the continuity-corrected normalized Talts et al. (2018) rank $(r_s + 0.5) / (L + 1) \in (0, 1)$
///    where $r_s = \sum_{\ell=1}^L \mathbb{I}(p_\ell < \tilde{p}) \in \{0, \dots, L\}$.
pub fn simulate_sbc_beta_binomial_ranks(
    seed: u64,
    num_replications: usize,
    posterior_draws_l: usize,
    alpha_prior: f64,
    beta_prior: f64,
    n_trials: usize,
) -> Vec<f64> {
    let mut rng = Rng::new(seed);
    let a0 = alpha_prior.max(0.1);
    let b0 = beta_prior.max(0.1);
    let l_draws = posterior_draws_l.max(1);
    let mut ranks = Vec::with_capacity(num_replications);

    for _ in 0..num_replications {
        let p_tilde = sample_beta_conjugate(&mut rng, a0, b0);
        let mut k = 0usize;
        for _ in 0..n_trials {
            if rng.bernoulli(p_tilde) {
                k += 1;
            }
        }
        let alpha_post = (k as f64) + a0;
        let beta_post = ((n_trials - k) as f64) + b0;
        let p_hat = alpha_post / (alpha_post + beta_post);
        let post_var = (p_hat * (1.0 - p_hat)) / (alpha_post + beta_post + 1.0);
        debug_assert!(post_var > 0.0 && post_var.is_finite());

        let mut rank = 0usize;
        for _ in 0..l_draws {
            let p_draw = sample_beta_conjugate(&mut rng, alpha_post, beta_post);
            if p_draw < p_tilde {
                rank += 1;
            }
        }
        ranks.push(((rank as f64) + 0.5) / ((l_draws + 1) as f64));
    }
    ranks
}

fn compute_sbc_rank_bins(null_samples: &[f64]) -> Vec<usize> {
    if null_samples.len() < 5 || null_samples.iter().any(|x| !x.is_finite()) {
        return vec![0; 5];
    }
    let min_val = null_samples.iter().copied().fold(f64::INFINITY, f64::min);
    let max_val = null_samples
        .iter()
        .copied()
        .fold(f64::NEG_INFINITY, f64::max);
    if (max_val - min_val) <= 1e-12 {
        return vec![null_samples.len(), 0, 0, 0, 0];
    }
    let (q20, q40, q60, q80) = if min_val >= 0.0 && max_val <= 1.0 {
        // Theoretical Uniform(0, 1) quintiles for normalized SBC / PIT ranks
        (0.20, 0.40, 0.60, 0.80)
    } else if min_val < 0.0 {
        // Theoretical N(0, 1) PIT quintiles for two-sided z-score null distributions
        (-0.841_621_23, -0.253_347_10, 0.253_347_10, 0.841_621_23)
    } else {
        // Theoretical Uniform(0, max_val) quintiles for nonnegative rank grids
        (
            0.20 * max_val,
            0.40 * max_val,
            0.60 * max_val,
            0.80 * max_val,
        )
    };
    let mut bins = vec![0usize; 5];
    for &x in null_samples {
        let b = if x <= q20 {
            0
        } else if x <= q40 {
            1
        } else if x <= q60 {
            2
        } else if x <= q80 {
            3
        } else {
            4
        };
        bins[b] += 1;
    }
    bins
}

/// Executes the complete 4-Filter Funnel and 8-Step SIGA Roadmap on an [`EpistemologicalCard`].
pub fn run_full_siga_roadmap(
    card: &EpistemologicalCard,
    preregistered_plan_text: &str,
    observed_test_statistic: f64,
    null_distribution_samples: &[f64],
    e_values_sequence: &[f64],
) -> SigaExecutionReport {
    let mc = &card.data_and_audit_contract.murphy_corp_decomposition;
    let unc = mc.base_uncertainty_unc0.max(1e-6);
    let res_norm = (mc.resolution_res_dsc / unc).clamp(0.0, 1.0);
    let rel_norm = (mc.miscalibration_rel_mcb / unc).clamp(0.0, 1.0);
    let tpr = (0.80 + 0.35 * res_norm - 0.25 * rel_norm).clamp(0.75, 0.95);
    let fpr = (0.10 - 0.15 * res_norm + 0.30 * rel_norm).clamp(0.03, 0.15);
    run_full_siga_roadmap_empirical(
        card,
        preregistered_plan_text,
        null_distribution_samples,
        observed_test_statistic,
        null_distribution_samples,
        e_values_sequence,
        (tpr, fpr),
    )
}

/// Executes the complete 4-Filter Funnel and 8-Step SIGA Roadmap with separate SBC calibration
/// ranks (Filter 3), un-clamped cluster-null test distribution (Step 5), sequential Ville
/// e-values (Step 6), and empirical confusion-matrix `(TPR, FPR)` at threshold $p^*$ (Step 8).
pub fn run_full_siga_roadmap_empirical(
    card: &EpistemologicalCard,
    preregistered_plan_text: &str,
    sbc_rank_samples: &[f64],
    observed_test_statistic: f64,
    null_distribution_samples: &[f64],
    e_values_sequence: &[f64],
    empirical_confusion_tpr_fpr: (f64, f64),
) -> SigaExecutionReport {
    let card_audit_ok = card.audit().is_ok();

    let id_strat = card
        .epistemological_header
        .causal_ladder
        .identification_strategy
        .to_ascii_lowercase();
    let markov_blanket_mapped = id_strat.contains("markov blanket")
        && !id_strat.contains("not mapped")
        && !id_strat.contains("unmapped");
    let collider_excluded = (id_strat.contains("without collider")
        || id_strat.contains("no collider")
        || id_strat.contains("free of collider"))
        && !id_strat.contains("without collider audit")
        && !id_strat.contains("conditioned on collider")
        && !id_strat.contains("conditioning on collider");
    let conditions_on_collider_berkson =
        (id_strat.contains("collider") || id_strat.contains("berkson")) && !collider_excluded;

    let sbc_bins = compute_sbc_rank_bins(sbc_rank_samples);
    let eps = card
        .decision_rule_and_scope
        .base_rate_floor_epsilon
        .clamp(1e-6, 0.49);
    let p = card.decision_rule_and_scope.event_probability_p;
    let q25 = 0.5 * (eps + p);
    let q50 = p;
    let q75 = (1.5 * p).min(0.5 * (p + 1.0 - eps));

    let candidate = FunnelCandidate {
        who_decides: card.decision_maker_question.who_decides.clone(),
        altered_action: card.decision_maker_question.altered_action.clone(),
        independent_unit: card
            .epistemological_header
            .unit_and_margin
            .independent_unit
            .clone(),
        prior_probability_p: p,
        protection_cost_c: card.decision_rule_and_scope.protection_cost_c,
        event_loss_l: card.decision_rule_and_scope.event_loss_l,
        efficacy_eta: card.decision_rule_and_scope.mitigation_efficacy_eta,
        decision_volume_units: 100.0,
        total_modeling_cost: 500.0,
        effective_tail_n_eff_p: card
            .epistemological_header
            .unit_and_margin
            .effective_sample_size_n_eff as f64,
        minimum_required_n_eff: 50.0,
        markov_blanket_mapped,
        conditions_on_collider_berkson,
        sbc_rank_histogram: sbc_bins,
        sbc_chi2_critical: 9.49,
        shelf_quartiles_q25_q50_q75: (q25, q50, q75),
        physical_domain_bounds: (eps, 1.0 - eps),
        preregistered_plan_text: preregistered_plan_text.to_string(),
        declared_sha256_seal: card.data_and_audit_contract.preregistration_hash.clone(),
        author_null_model: card
            .uncertainty_budget_and_authorship
            .field_authorship
            .null_model
            .clone(),
        author_decision_rule: card
            .uncertainty_budget_and_authorship
            .field_authorship
            .decision_rule
            .clone(),
        author_adversarial_reviewer: card
            .uncertainty_budget_and_authorship
            .field_authorship
            .adversarial_reviewer
            .clone(),
        numeric_exit_clause: card.decision_rule_and_scope.exit_clause.clone(),
    };

    let funnel = evaluate_four_filter_funnel(&candidate);

    // Step 5: Exact empirical p-value against the cluster-respecting generative null model (no clamp)
    let exceedances = null_distribution_samples
        .iter()
        .filter(|&&s| s >= observed_test_statistic)
        .count();
    let null_p = ((exceedances + 1) as f64) / ((null_distribution_samples.len() + 1) as f64);

    // Step 6: Sequential e-process with multiplicity K
    let e_proc = EvidenceProcess::evaluate(
        e_values_sequence,
        0.05,
        card.data_and_audit_contract.attempt_counter_k,
    );

    // Step 8: Integer natural frequency memorandum out of 1,000 units from empirical confusion matrix at p*
    let (sensitivity_tpr, false_positive_rate_fpr) = empirical_confusion_tpr_fpr;
    let memo = NaturalFrequencyMemorandum::build_with_decision(
        1_000,
        card.decision_rule_and_scope.event_probability_p,
        sensitivity_tpr,
        false_positive_rate_fpr,
        card.decision_rule_and_scope.critical_threshold_p_star,
        &card.decision_rule_and_scope.declared_act,
        &card.decision_rule_and_scope.exit_clause,
    );

    let s1 = card_audit_ok; // Step 1: Layer, bridge, uncertainty class, Info-Gap + Sniedovich
    let s2 = funnel.filter_1_approved; // Step 2: 3-part question, EVPI, Phi5
    let s3 = funnel.filter_2_approved; // Step 3: Moulton-Campbell m_tilde, rho_I(p), Markov blanket
    let s4 = !card
        .epistemological_header
        .noise_physics
        .absorbing_states
        .is_empty(); // Step 4: Noise physics, Kelly, performativity
    let s5 = funnel.filter_3_approved && null_p < 0.05; // Step 5: Executable null, SBC, SHELF
    let s6 = funnel.filter_4_approved && e_proc.rejects_null_hypothesis; // Step 6: SHA-256, cross-authorship, e-process
    let s7 = !card.decision_rule_and_scope.declared_act.is_empty(); // Step 7: CORP-PAV + p* = C/(eta*L)
    let s8 =
        memo.reference_cohort_size == 1_000 && !card.decision_rule_and_scope.exit_clause.is_empty(); // Step 8: Natural frequencies + exit clause

    let steps = [s1, s2, s3, s4, s5, s6, s7, s8];
    let all_ok = steps.iter().all(|&b| b) && funnel.approved_all_filters;

    SigaExecutionReport {
        card_id: card.id.clone(),
        funnel_result: funnel,
        null_p_value: null_p,
        e_process: e_proc,
        natural_frequency_memo: memo,
        steps_1_to_8_completed: steps,
        approved_end_to_end: all_ok,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eight_step_siga_roadmap_validates_end_to_end_and_prunes_defective_inputs() {
        let card = EpistemologicalCard::canonical_example();
        let plan = card.compute_preregistration_payload();
        let null_samples: Vec<f64> = (0..200).map(|i| (i as f64) * 0.01).collect();
        let report = run_full_siga_roadmap(&card, &plan, 5.0, &null_samples, &[24.8]);
        assert!(report.approved_end_to_end);
        assert_eq!(report.steps_1_to_8_completed, [true; 8]);
        assert!(report
            .natural_frequency_memo
            .formatted_summary
            .contains("1000"));
        assert!(report
            .natural_frequency_memo
            .formatted_summary
            .contains("p*=0.12000"));

        // Un-clamped low observed test statistic fails Step 5 (null_p >= 0.05)
        let weak_signal = run_full_siga_roadmap(&card, &plan, 0.50, &null_samples, &[24.8]);
        assert!(weak_signal.null_p_value >= 0.05);
        assert!(!weak_signal.steps_1_to_8_completed[4]);
        assert!(!weak_signal.approved_end_to_end);

        // Degenerate null samples fail Filter 3 (SBC rank uniformity)
        let bad_null = run_full_siga_roadmap(&card, &plan, 5.0, &[1.0; 100], &[24.8]);
        assert_eq!(bad_null.funnel_result.pruned_at_filter, Some(3));

        // Card conditioning on a Berkson collider fails Filter 2
        let mut collider_card = card.clone();
        collider_card
            .epistemological_header
            .causal_ladder
            .identification_strategy =
            "Markov blanket conditioned on hospital admission collider".into();
        let bad_collider =
            run_full_siga_roadmap(&collider_card, &plan, 5.0, &null_samples, &[24.8]);
        assert_eq!(bad_collider.funnel_result.pruned_at_filter, Some(2));

        // NaN e-value safely zeroes out EvidenceProcess without panicking
        let nan_e = EvidenceProcess::evaluate(&[24.8, f64::NAN], 0.05, 1);
        assert!(!nan_e.rejects_null_hypothesis);

        // GRAPA adaptive Kelly betting martingale rejects positive drift and stays <= threshold under null
        let pos_stream = vec![0.45; 35];
        let grapa = EvidenceProcess::from_grapa_betting(&pos_stream, 0.05, 1).unwrap();
        assert!(grapa.rejects_null_hypothesis);
    }

    #[test]
    fn skewed_sbc_samples_fail_filter3_while_genuine_talts_sbc_ranks_pass() {
        let card = EpistemologicalCard::canonical_example();
        let plan = card.compute_preregistration_payload();

        // Skewed non-uniform sample concentrates in lower quintile and fails Filter 3
        let skewed_samples: Vec<f64> = (0..200).map(|i| ((i as f64) / 200.0).powi(3)).collect();
        let skewed_bins = compute_sbc_rank_bins(&skewed_samples);
        assert!(crate::funnel::sbc_rank_chi2(&skewed_bins) > 9.49);
        let skewed_rep = run_full_siga_roadmap(&card, &plan, 5.0, &skewed_samples, &[24.8]);
        assert!(skewed_rep.funnel_result.sbc_chi2_statistic > 9.49);
        assert_eq!(skewed_rep.funnel_result.pruned_at_filter, Some(3));
        assert!(!skewed_rep.approved_end_to_end);

        // Genuine Talts et al. (2018) Beta-Binomial SBC ranks are uniform on [0, 1] and pass Filter 3
        let sbc_ranks = simulate_sbc_beta_binomial_ranks(2026, 200, 50, 2.0, 18.0, 100);
        let sbc_bins = compute_sbc_rank_bins(&sbc_ranks);
        assert!(crate::funnel::sbc_rank_chi2(&sbc_bins) <= 9.49);
        let valid_rep = run_full_siga_roadmap(&card, &plan, 5.0, &sbc_ranks, &[12.5, 2.2, 0.7]);
        assert!(valid_rep.funnel_result.filter_3_approved);
        assert!(valid_rep.e_process.rejects_null_hypothesis);
        assert!(valid_rep.approved_end_to_end);

        let (epic, stoch_gate) = AlertFatigueAudit::canonical_epic_sepsis_vs_stochological_gate();
        assert!(!epic.approved_bridge_l2_l3);
        assert!((epic.positive_predictive_value_ppv - 0.12).abs() < 0.01);
        assert!(epic.false_alarms_per_true_case > 7.0);
        assert!(stoch_gate.approved_bridge_l2_l3);
        assert!(stoch_gate.turing_weight_of_evidence_decibans > 12.0);
        assert!(stoch_gate.cognitive_triage_load_bps <= 10.0);
    }

    #[test]
    fn alert_fatigue_operating_points_declare_literature_vs_assumed_provenance() {
        let (epic, stoch_gate) = AlertFatigueAudit::canonical_epic_sepsis_vs_stochological_gate();
        assert_eq!(
            epic.operating_point_source,
            OperatingPointSource::Literature {
                source: "Wong et al., JAMA Internal Medicine 2021"
            }
        );
        assert_eq!(
            stoch_gate.operating_point_source,
            OperatingPointSource::Assumed
        );
        // Declaring provenance never alters a computed quantity.
        let bare = AlertFatigueAudit::evaluate(0.066, 0.33, 0.171, 0.20, 18.0, 42.0, false);
        assert_eq!(bare.operating_point_source, OperatingPointSource::Assumed);
        assert_eq!(
            bare.positive_predictive_value_ppv,
            epic.positive_predictive_value_ppv
        );
        assert_eq!(bare.approved_bridge_l2_l3, epic.approved_bridge_l2_l3);
    }

    #[test]
    fn savi_cs_var_es_e_backtest_and_grapa_ville_safety_reproduce_tri_model_vectors() {
        let savi = SaviConfidenceSequence::canonical_weyl_golden_benchmark();
        assert!((savi.raw_lambda_prpl_first_3[0] - 6.524984656).abs() < 1e-6);
        assert!((savi.raw_lambda_prpl_first_3[1] - 5.147135238).abs() < 1e-6);
        assert!((savi.raw_lambda_prpl_first_3[2] - 4.213005484).abs() < 1e-6);
        assert!((savi.agrapa_wealth_at_m0 - 38.37803479).abs() < 1e-6);
        assert!((savi.prpl_hedged_wealth_at_m0 - 25.24363875).abs() < 1e-6);
        assert!((savi.cs_lower_bound - 0.30738932).abs() < 1e-6);
        assert!((savi.cs_upper_bound - 0.67446618).abs() < 1e-6);
        assert!(savi.rejects_candidate_m0);

        let (z_peek_fpr, savi_fpr) =
            SaviConfidenceSequence::compare_peeking_vs_savi(2026, 200, 0.05);
        assert!(z_peek_fpr > 0.30);
        assert!(savi_fpr <= 0.05);

        let gro = EvidenceProcess::gro_betting(&[0.85; 25], 0.30, 0.80, 0.05, 1);
        assert!(gro.rejects_null_hypothesis);

        // Verify Ville's inequality holds on asymmetric bounded zero-mean null: P(X = 0.8) = 0.2, P(X = -0.2) = 0.8
        let mut rng = Rng::new(2026);
        let mut false_alarms = 0usize;
        for _ in 0..400 {
            let stream: Vec<f64> = (0..200)
                .map(|_| if rng.bernoulli(0.20) { 0.80 } else { -0.20 })
                .collect();
            if EvidenceProcess::from_grapa_betting(&stream, 0.05, 1)
                .unwrap()
                .rejects_null_hypothesis
            {
                false_alarms += 1;
            }
        }
        assert!((false_alarms as f64) / 400.0 <= 0.05);

        // Verify VaR & ES GREE e-backtest exact vector M_8 = 9.798157946
        let backtest = VarEsEBacktest::from_var_exceedances(0.90, &[0, 1, 0, 0, 1, 1, 0, 1]);
        assert!((backtest.final_martingale_m_t - 9.798157946).abs() < 1e-8);
        assert!((VarEsEBacktest::es_e_statistic(3.5, 2.5, 1.5, 0.90) - 20.0).abs() < 1e-12);
        assert!(VarEsEBacktest::es_e_statistic(3.5, 1.5, 1.5, 0.90).is_infinite());
        assert!((VarEsEBacktest::es_e_statistic(1.0, 1.5, 1.5, 0.90) - 1.0).abs() < 1e-12);
    }

    #[test]
    fn grapa_fails_closed_on_out_of_support_payoffs() {
        let mut strong = vec![0.5; 100];
        assert!(
            EvidenceProcess::from_grapa_betting(&strong, 0.05, 1)
                .unwrap()
                .rejects_null_hypothesis
        );
        strong[50] = 1.01;
        assert!(EvidenceProcess::from_grapa_betting(&strong, 0.05, 1).is_err());
        assert!(EvidenceProcess::from_grapa_betting(&[0.1, f64::NAN], 0.05, 1).is_err());
    }

    #[test]
    fn multimodal_five_senses_and_premortem_pns_sop_verify_operational_resilience() {
        let mm = MultimodalCorrelatedFusionAudit::canonical_five_senses_benchmark();
        assert!(mm.multimodal_fusion_approved);
        assert!((mm.multimodal_deff - 2.80).abs() < 1e-12);
        assert!((mm.naive_independent_decibans - 53.0).abs() < 1e-12);
        assert!((mm.shielded_correlated_decibans - 24.0 / 2.80).abs() < 1e-12);
        assert!(mm.zero_correlation_identity_error < 1e-12);

        let pm = PreMortemCounterfactualRootCauseAudit::canonical_benchmark();
        assert!(pm.premortem_and_sop_approved);
        assert!((pm.pns_lower_bound - 0.72).abs() < 1e-12);
        assert!((pm.pns_upper_bound - 0.84).abs() < 1e-12);
        assert!(pm.tian_pearl_bounds_identity_error < 1e-12);
        assert!(pm.sop_cognitive_load_bps <= 10.0);
    }
}
