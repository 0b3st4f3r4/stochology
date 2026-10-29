//! Sequential anytime-valid *e-processes* ($E_t = \prod_{i=1}^t L_i$), `SAVI` confidence sequences,
//! and `GREE` VaR/ES e-backtesting (§3.2 Step 6).

use stochology_core::Rng;

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
