//! **Block II (Filter II: Asymmetry and Irreversible Time · Axis 2: Vital Margin $S_{\mathrm{vital}}$ · Chapters 5–6).**
//!
//! Unifies all vital-margin operators across **Chapter 5 [formerly Ch. 4]** (Asymmetry, Positive Ratio Scale Invariance,
//! Ordinal Rank Preservation, Convexity & Heavy Tails) and **Chapter 6 [formerly Ch. 7]** (Irreversible Time, Non-Ergodicity,
//! Kingman Queue Slack, Resilience Basins & $\mathrm{TTS}_{\mathrm{survival}} \ge \mathrm{TTR}_{\mathrm{recovery}}$)
//! into **Canonical Operator II** and the Type-State structure [`VitalMarginState`]:
//!
//! $$\Delta_J(\phi, X) = \mathbb{E}\bigl[\phi(X)\bigr] - \phi\bigl(\mathbb{E}[X]\bigr) \approx \frac{1}{2}\phi''\bigl(\mathbb{E}[X]\bigr)\,\mathrm{Var}(X) + \mathcal{R}_{\mathrm{tail}}(\hat{\alpha})$$
//!
//! In open systems subject to irreversible entropic time and absorbing ruin barriers ($R_t \le 0$),
//! the convex curvature of harm/congestion ($\phi(\rho) = \frac{\rho}{1-\rho}$ in Kingman, $-\frac{1}{2}\sigma^2 f^2$
//! in Itô–Kelly) and power-law tail thickness ($\hat{\alpha}$ of Hill) forbid optimization at the lone arithmetic mean
//! and require $\mathrm{TTS}_{\mathrm{survival}} \ge \mathrm{TTR}_{\mathrm{recovery}}$.

use crate::kernel::{OnlineMoments, SplitMix64, mean, quantile_sorted, sample_variance};

/// Moment-existence regime classified by the extreme-value tail index $\alpha$ (Hill, 1975).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TailMomentRegime {
    /// $\alpha \le 1$: Neither the population mean nor the variance is finite.
    InfiniteMeanAndVariance,
    /// $1 < \alpha \le 2$: Finite population mean, infinite population variance (classical CLT fails).
    #[default]
    FiniteMeanInfiniteVariance,
    /// $\alpha > 2$: Both population mean and variance are finite.
    FiniteVariance,
}

/// Regime selector for Baker–McHale (2013) epistemic Kelly sizing under heavy tails.
///
/// When $\hat{\alpha}_k \le 2.2$, the sample variance $\hat{\sigma}_n^2 \asymp \mathcal{O}_P(n^{2/\alpha - 1})$
/// diverges polynomially with sample size $n$, causing the naive quadratic Kelly fraction to collapse
/// inconsistently toward zero; the selector switches automatically to a scale-stable WDRO / $\mathrm{CVaR}_{95}$
/// tail denominator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum KellySizingRegime {
    /// $\hat{\alpha}_k > 2.2$: Classical Gaussian/quadratic variance denominator $\hat{\sigma}^2$ is consistent.
    #[default]
    QuadraticGaussianKelly,
    /// $\hat{\alpha}_k \le 2.2$: Quadratic variance $\hat{\sigma}^2$ is disabled; allocation is solved over the
    /// scale-stable WDRO / $\mathrm{CVaR}_{95}$ tail dispersion $\hat{s}_{\alpha}^2$.
    TailRobustWdroCvarKelly,
}

/// Non-linear error propagation audit from Axis 1 ($\mathrm{DEFF}_{\mathrm{tail}}$) into Axis 2
/// (Hill tail estimator $\hat{\alpha}_k$ and Canguilhem–Holling basin rupture probability
/// $\hat{P}_{\mathrm{rup}}(\hat{\alpha}_k) = (x_m / D_t)^{\hat{\alpha}_k}$).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct HollingTailRuptureAudit {
    /// Nominal tail exceedances $k$.
    pub nominal_k: usize,
    /// Cluster tail design effect $\mathrm{DEFF}_{\mathrm{tail}} \ge 1$.
    pub deff_tail: f64,
    /// Effective independent tail exceedances $k_{\mathrm{eff}} = k / \mathrm{DEFF}_{\mathrm{tail}}$.
    pub effective_k: f64,
    /// Reference / true tail exponent $\alpha$.
    pub reference_alpha: f64,
    /// Expected finite-sample Hill estimate $\mathbb{E}[\hat{\alpha}_k] \approx \frac{k_{\mathrm{eff}}}{k_{\mathrm{eff}} - 1}\alpha$.
    pub expected_alpha_hat: f64,
    /// Standard error $\mathrm{SD}(\hat{\alpha}_k)$ inflated by $\sqrt{\mathrm{DEFF}_{\mathrm{tail}}}$.
    pub alpha_std_error: f64,
    /// True (or reference) basin rupture probability $P_{\mathrm{rup}}(\alpha) = (x_m / D_t)^\alpha$.
    pub true_rupture_prob: f64,
    /// Expected plug-in rupture probability $\mathbb{E}[\hat{P}_{\mathrm{rup}}(\hat{\alpha}_k)]$ (inflated by second-order Jensen convexity).
    pub expected_plugin_rupture_prob: f64,
    /// Relative second-order Jensen bias $\frac{\mathbb{E}[\hat{P}_{\mathrm{rup}}] - P_{\mathrm{rup}}}{P_{\mathrm{rup}}}$.
    pub jensen_relative_bias: f64,
    /// Convexity-debiased rupture probability $\hat{P}_{\mathrm{rup}}^{\mathrm{debiased}}$.
    pub debiased_rupture_prob: f64,
    /// Empirical $90\%$ confidence interval $[q_{05}, q_{95}]$ of $\hat{P}_{\mathrm{rup}}$.
    pub rupture_ci90: [f64; 2],
    /// Confidence interval dispersion ratio $q_{95} / q_{05}$.
    pub ci90_dispersion_ratio: f64,
}

impl HollingTailRuptureAudit {
    /// Evaluates the Inverse-Gamma propagation of $\hat{\alpha}_k = \frac{k_{\mathrm{eff}}\alpha}{G}$ ($G \sim \mathrm{Gamma}(k_{\mathrm{eff}}, 1)$)
    /// through the convex Holling basin rupture mapping $\hat{P}_{\mathrm{rup}}(\hat{\alpha}_k) = \exp\bigl(-\hat{\alpha}_k \ln(D_t / x_m)\bigr)$.
    pub fn evaluate(
        nominal_k: usize,
        deff_tail: f64,
        reference_alpha: f64,
        basin_distance_ratio: f64,
        monte_carlo_draws: usize,
    ) -> Self {
        let nom_k = nominal_k.max(2);
        let deff = deff_tail.max(1.0);
        let ref_a = reference_alpha.max(0.2);
        let b_ratio = basin_distance_ratio.max(1.01);
        let k_eff = ((nom_k as f64) / deff).max(2.05);
        let log_d = b_ratio.ln();
        let p_true = (-ref_a * log_d).exp();

        let draws = monte_carlo_draws.max(500);
        let mut rng =
            SplitMix64::with_default_seed().split((deff * 1000.0) as u64 ^ (nom_k as u64));
        let mut alphas = Vec::with_capacity(draws);
        let mut p_rups = Vec::with_capacity(draws);

        for _ in 0..draws {
            let g = rng.gamma(k_eff, 1.0).max(1e-6);
            let a_hat = ((k_eff * ref_a) / g).clamp(0.15, 25.0);
            let p_hat = (-a_hat * log_d).exp();
            alphas.push(a_hat);
            p_rups.push(p_hat);
        }

        p_rups.sort_by(|a, b| a.total_cmp(b));
        let exp_alpha = mean(&alphas);
        let std_alpha = if k_eff > 2.05 {
            (k_eff * ref_a) / ((k_eff - 1.0) * (k_eff - 2.0).sqrt())
        } else {
            ref_a
        };
        let exp_p = mean(&p_rups);
        let q05 = quantile_sorted(&p_rups, 0.05).max(1e-9);
        let q95 = quantile_sorted(&p_rups, 0.95);
        let rel_bias = (exp_p - p_true) / p_true.max(1e-12);

        // Convexity-debiased estimator: finite-k_eff Inverse-Gamma shrinkage + second-order Jensen correction
        let alpha_shrunk = ((k_eff - 1.0) / k_eff) * exp_alpha;
        let var_shrunk = (alpha_shrunk * alpha_shrunk) / (k_eff - 2.0).max(1.0);
        let jensen_factor = (1.0 + 0.5 * var_shrunk * log_d * log_d).max(1.0);
        let p_debiased = (exp_p / jensen_factor).clamp(0.0, 1.0);

        Self {
            nominal_k: nom_k,
            deff_tail: deff,
            effective_k: k_eff,
            reference_alpha: ref_a,
            expected_alpha_hat: exp_alpha,
            alpha_std_error: std_alpha,
            true_rupture_prob: p_true,
            expected_plugin_rupture_prob: exp_p,
            jensen_relative_bias: rel_bias,
            debiased_rupture_prob: p_debiased,
            rupture_ci90: [q05, q95],
            ci90_dispersion_ratio: q95 / q05,
        }
    }
}

/// Computes the Mann–Whitney ordinal dominance probability
/// $\theta_{\mathrm{MW}}(A, B) = \mathbb{P}(A > B) + \frac{1}{2}\mathbb{P}(A = B) \in [0, 1]$,
/// which is strictly invariant under any strictly increasing transformation $\phi$.
pub fn mann_whitney_dominance(group_a: &[f64], group_b: &[f64]) -> f64 {
    if group_a.is_empty() || group_b.is_empty() {
        return 0.5;
    }
    let mut wins = 0.0_f64;
    for &a in group_a {
        for &b in group_b {
            if (a - b).abs() <= 1e-12 {
                wins += 0.5;
            } else if a > b {
                wins += 1.0;
            }
        }
    }
    wins / ((group_a.len() * group_b.len()) as f64)
}

/// Empirical Continuous Ranked Probability Score (CRPS) of an ensemble forecast `forecast_samples`
/// evaluated against a realized scalar outcome `observation_y` in $O(m \log m)$.
pub fn crps_empirical(forecast_samples: &[f64], observation_y: f64) -> f64 {
    if forecast_samples.is_empty() {
        return 0.0;
    }
    let mut sorted = forecast_samples.to_vec();
    sorted.sort_by(|a, b| a.total_cmp(b));
    let m = sorted.len();
    let mf = m as f64;

    let term1: f64 = sorted
        .iter()
        .map(|&x| (x - observation_y).abs())
        .sum::<f64>()
        / mf;
    let mut weighted_sum = 0.0;
    for (i, &x) in sorted.iter().enumerate() {
        let coeff = (2 * i) as f64 - (m as f64) + 1.0;
        weighted_sum += coeff * x;
    }
    let half_spread = weighted_sum / (mf * mf);
    (term1 - half_spread).max(0.0)
}

/// Parameterized Abraham Wald (1943, Statistical Research Group Memorandum CRC 432) bomber survivorship
/// reconstruction across two vulnerability zones using only observable survivor hit counts and aggregate losses.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WaldSurvivorshipAudit {
    /// Returning survivors $S_{\mathrm{total}}$.
    pub survivors_total: f64,
    /// Missing aircraft downed in combat $M_{\mathrm{total}} = M_1 + M_2$.
    pub missing_total: f64,
    /// Reconstructed unobserved hits on downed aircraft in Zone A: $\hat{M}_A = S_A \frac{q_A}{1 - q_A}$.
    pub reconstructed_missing_hits_a: f64,
    /// Reconstructed unobserved hits on downed aircraft in Zone B: $\hat{M}_B = S_B \frac{q_B}{1 - q_B}$.
    pub reconstructed_missing_hits_b: f64,
    /// Conditional lethality in Zone A (e.g., fuselage/wings with high survivor hit count): $\mathbb{P}(\text{Lost}\mid\text{Hit}_A) = q_A$.
    pub lethality_zone_a: f64,
    /// Conditional lethality in Zone B (e.g., engine/cockpit with low survivor hit count): $\mathbb{P}(\text{Lost}\mid\text{Hit}_B) = q_B$.
    pub lethality_zone_b: f64,
    /// True iff visible survivor hit rate $S_A / S_{\mathrm{total}} > S_B / S_{\mathrm{total}}$ while conditional lethality reverses ($\mathbb{P}(\text{Lost}\mid B) > \mathbb{P}(\text{Lost}\mid A)$).
    pub survivorship_inversion: bool,
}

impl WaldSurvivorshipAudit {
    /// Solves Abraham Wald's (1943, CRC 432) latent $2\times 2$ survivor estimating system across two sortie waves:
    ///
    /// $$\begin{bmatrix} S_{1,A} & S_{1,B} \\ S_{2,A} & S_{2,B} \end{bmatrix} \begin{bmatrix} \theta_A \\ \theta_B \end{bmatrix} = \begin{bmatrix} M_1 \\ M_2 \end{bmatrix}, \qquad \theta_z = \frac{q_z}{1 - q_z}, \quad q_z = \frac{\theta_z}{1 + \theta_z}$$
    ///
    /// using **only** returning survivor hit counts $(S_{c,A}, S_{c,B})$ and aggregate missing aircraft $M_c$
    /// (without observing any hits on missing aircraft).
    pub fn solve_latent_two_cohort(
        survivors_total: f64,
        wave1_survivor_hits_a: f64,
        wave1_survivor_hits_b: f64,
        wave1_missing: f64,
        wave2_survivor_hits_a: f64,
        wave2_survivor_hits_b: f64,
        wave2_missing: f64,
    ) -> Self {
        let s_tot = survivors_total.max(1.0);
        let det = wave1_survivor_hits_a * wave2_survivor_hits_b
            - wave2_survivor_hits_a * wave1_survivor_hits_b;
        let (theta_a, theta_b) = if det.abs() > 1e-9 {
            let ta = ((wave1_missing * wave2_survivor_hits_b
                - wave2_missing * wave1_survivor_hits_b)
                / det)
                .max(0.0);
            let tb = ((wave1_survivor_hits_a * wave2_missing
                - wave2_survivor_hits_a * wave1_missing)
                / det)
                .max(0.0);
            (ta, tb)
        } else {
            (0.0, 0.0)
        };
        let leth_a = (theta_a / (1.0 + theta_a)).clamp(0.0, 1.0);
        let leth_b = (theta_b / (1.0 + theta_b)).clamp(0.0, 1.0);
        let s_a = wave1_survivor_hits_a + wave2_survivor_hits_a;
        let s_b = wave1_survivor_hits_b + wave2_survivor_hits_b;
        let m_a = s_a * theta_a;
        let m_b = s_b * theta_b;
        let inv = (s_a / s_tot > s_b / s_tot) && (leth_b > leth_a);
        Self {
            survivors_total: s_tot,
            missing_total: (wave1_missing + wave2_missing).max(0.0),
            reconstructed_missing_hits_a: m_a,
            reconstructed_missing_hits_b: m_b,
            lethality_zone_a: leth_a,
            lethality_zone_b: leth_b,
            survivorship_inversion: inv,
        }
    }

    /// Evaluates Wald's conditional lethality inversion given total survivors, total missing,
    /// total hits per zone `total_hits_per_zone`, and observed hits on returning survivors `(survivor_hits_a, survivor_hits_b)`.
    pub fn evaluate(
        survivors_total: f64,
        missing_total: f64,
        total_hits_per_zone: f64,
        survivor_hits_a: f64,
        survivor_hits_b: f64,
    ) -> Self {
        let tot_hits = total_hits_per_zone.max(1.0);
        let s_tot = survivors_total.max(1.0);
        let m_a = (tot_hits - survivor_hits_a).max(0.0);
        let m_b = (tot_hits - survivor_hits_b).max(0.0);
        let leth_a = (m_a / tot_hits).clamp(0.0, 1.0);
        let leth_b = (m_b / tot_hits).clamp(0.0, 1.0);
        let inv = (survivor_hits_a / s_tot > survivor_hits_b / s_tot) && (leth_b > leth_a);
        Self {
            survivors_total: s_tot,
            missing_total: missing_total.max(0.0),
            reconstructed_missing_hits_a: m_a,
            reconstructed_missing_hits_b: m_b,
            lethality_zone_a: leth_a,
            lethality_zone_b: leth_b,
            survivorship_inversion: inv,
        }
    }

    /// Canonical Abraham Wald (1943, CRC 432) bomber vulnerability benchmark (`[Assumed]` two-wave survivor
    /// calibration: $S = 800$ returning bombers across two waves with $S_{1,A}=116, S_{1,B}=12, M_1=72$ and
    /// $S_{2,A}=116, S_{2,B}=28, M_2=156$, solving the latent $2\times 2$ survivor equations for
    /// $\hat{M}_A = 18 \implies q_A = 7.2\%$ vs. $\hat{M}_B = 210 \implies q_B = 84.0\%$).
    pub fn wald_1943_bomber_benchmark() -> Self {
        Self::solve_latent_two_cohort(800.0, 116.0, 12.0, 72.0, 116.0, 28.0, 156.0)
    }

    /// Empirical survivorship-conditioning bias audit (`[Measured]`): compares the full-cohort treatment effect
    /// $\mathrm{ATE}_{\mathrm{full}} = \bar{Y}_1 - \bar{Y}_0$ against the naive survivor-only effect
    /// $\mathrm{ATE}_{\mathrm{survivors}} = \mathbb{E}[Y_1 \mid Y_1 > 0] - \mathbb{E}[Y_0 \mid Y_0 > 0]$
    /// conditioned on positive-outcome survivors ($Y > 0$), returning `(full_ate, survivor_only_ate, survivorship_bias)`.
    pub fn evaluate_empirical_truncation_bias(treated: &[f64], control: &[f64]) -> (f64, f64, f64) {
        let full_ate = mean(treated) - mean(control);
        let t_pos: Vec<f64> = treated.iter().copied().filter(|&x| x > 0.0).collect();
        let c_pos: Vec<f64> = control.iter().copied().filter(|&x| x > 0.0).collect();
        let surv_ate = if !t_pos.is_empty() && !c_pos.is_empty() {
            mean(&t_pos) - mean(&c_pos)
        } else {
            full_ate
        };
        (full_ate, surv_ate, surv_ate - full_ate)
    }
}

/// Parameterized Moshe Sniedovich (2007, 2012) critique of local Info-Gap robustness $\hat{\alpha}(q, r_c)$
/// paired with Abraham Wald's (1945) global Maximin safety floor.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SniedovichInfoGapAudit {
    /// Local Info-Gap robustness radius $\hat{\alpha}(q, r_c) = \max(0, (u_{\mathrm{nom}} - r_c)/s_{\mathrm{local}})$ around the nominal model.
    pub local_info_gap_alpha: f64,
    /// Worst-case global payoff across the full Knightian uncertainty set $\mathcal{U}_{\mathrm{global}}$.
    pub global_worst_case_payoff: f64,
    /// Abraham Wald's (1945) global Maximin ruin floor $r_{\mathrm{Wald}}$.
    pub wald_maximin_floor: f64,
    /// True if the option has positive local Info-Gap robustness ($\hat{\alpha} > 0$) around the nominal guess
    /// yet violates Wald's global Maximin floor ($u_{\mathrm{global\_min}} < r_{\mathrm{Wald}}$), exposing Sniedovich's *No-Man's-Land* mirage.
    pub no_mans_land_mirage: bool,
}

impl SniedovichInfoGapAudit {
    /// Evaluates whether local Info-Gap robustness around `nominal_payoff` masks global exposure below `wald_maximin_floor`.
    pub fn evaluate(
        nominal_payoff: f64,
        local_sensitivity_slope: f64,
        critical_reward_rc: f64,
        global_worst_case_payoff: f64,
        wald_maximin_floor: f64,
    ) -> Self {
        let local_alpha =
            ((nominal_payoff - critical_reward_rc) / local_sensitivity_slope.max(1e-9)).max(0.0);
        let mirage = local_alpha > 0.0 && global_worst_case_payoff < wald_maximin_floor;
        Self {
            local_info_gap_alpha: local_alpha,
            global_worst_case_payoff,
            wald_maximin_floor,
            no_mans_land_mirage: mirage,
        }
    }

    /// Canonical Moshe Sniedovich (2007, 2012) *No-Man's-Land* Info-Gap benchmark (`[Assumed]` analytical calibration:
    /// nominal payoff $100$, local slope $10$, critical reward $60 \implies \hat{\alpha} = 4 > 0$, yet global worst-case $-400 < -50$).
    pub fn sniedovich_2012_benchmark() -> Self {
        Self::evaluate(100.0, 10.0, 60.0, -400.0, -50.0)
    }
}

/// **Canonical Type-State Structure II — `VitalMarginState` ($S_{\mathrm{vital}}$).**
///
/// Encapsulates **Canonical Operator II** (Convexity & Non-Ergodic Survival under Heavy Tails),
/// uniting Jensen's convexity gap $\Delta_J$, Hill's Pareto tail index $\hat{\alpha}$, Georgescu-Roegen's
/// dialectical penumbra, Kingman's heavy-traffic queue ceiling ($\rho_{\max} \le 0.76$), Baker–McHale
/// epistemic Kelly shrinkage $k(n_{\mathrm{eff}})$, Canguilhem–Holling resilience basin width $D_t / D_0$
/// under McEwen allostatic wear & Scheffer Critical Slowing Down ($\rho_1$), Wald survivorship & Sniedovich's
/// Info-Gap critique, and the explicit survival horizon inequality $\mathrm{TTS}_{\mathrm{survival}} \ge \mathrm{TTR}_{\mathrm{recovery}}$.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct VitalMarginState {
    /// Sample arithmetic mean $\mathbb{E}[X]$.
    pub sample_mean: f64,
    /// Sample median ($q_{50}$).
    pub median: f64,
    /// Lower $5\%$ quantile ($q_{05}$).
    pub q05: f64,
    /// Upper $95\%$ quantile ($q_{95}$).
    pub q95: f64,
    /// Naive evaluation at the mean $\phi(\mathbb{E}[X])$.
    pub eval_at_mean: f64,
    /// True expected evaluation across the distribution $\mathbb{E}[\phi(X)]$.
    pub expected_eval: f64,
    /// Jensen's convexity gap $\Delta_J = \mathbb{E}[\phi(X)] - \phi(\mathbb{E}[X]) \ge 0$.
    pub jensen_gap: f64,
    /// Out-of-sample split-half empirical CRPS (for $n \ge 4$, using the first half as forecast distribution and the second half as realized outcomes; falls back to median point CRPS when $n < 4$).
    pub crps_at_median: f64,
    /// Point empirical CRPS evaluated directly at the sample median $q_{50}$ (`crps_empirical(&sorted, med)`).
    pub crps_point_median: f64,
    /// Hill (1975) extreme-value Pareto tail exponent $\hat{\alpha}_{k,n}$.
    pub tail_alpha: f64,
    /// Sensitivity range $[\hat{\alpha}_{\min}, \hat{\alpha}_{\max}]$ of Hill's estimator across threshold grid $k \in \{50, 100, 150, 200\}$.
    pub hill_alpha_k_range: [f64; 2],
    /// Moment-existence regime implied by $\hat{\alpha}_{k,n}$.
    pub tail_regime: TailMomentRegime,
    /// Georgescu-Roegen (1971) dialectical penumbra mass inside $[c_L, c_U)$.
    pub penumbra_mass: f64,
    /// True if arithmetic mean ranking reverses under a strictly increasing monotone map $\phi$ (Stevens 1946).
    pub mean_ranking_reversed: bool,
    /// Absolute discrepancy in Mann–Whitney ordinal dominance under $\phi$ (identically $0$).
    pub ordinal_invariance_error: f64,
    /// Realized capacity utilization $\rho = S_t / C_{\mathrm{eff}} \in (0, 1)$.
    pub utilization_rho: f64,
    /// Target safe Kingman utilization cap $\rho_{\max} \le 0.76$.
    pub utilization_cap: f64,
    /// Kingman (1961) heavy-traffic congestion multiplier $\frac{\rho}{1 - \rho}$.
    pub kingman_wait_multiplier: f64,
    /// Kingman arrival/service squared-coefficient-of-variation factor $\frac{c_a^2 + c_s^2}{2}$.
    pub kingman_variability_factor: f64,
    /// Mean service time $\tau_{\mathrm{service}}$.
    pub service_time_tau: f64,
    /// Full Kingman (1961) $G/G/1$ expected queue waiting time $W_q \approx \left(\frac{c_a^2 + c_s^2}{2}\right)\left(\frac{\rho}{1 - \rho}\right)\tau_{\mathrm{service}}$.
    pub kingman_expected_wait: f64,
    /// Ensemble arithmetic drift $\hat{\mu}$.
    pub ensemble_drift_mu: f64,
    /// Unleveraged ($f = 1$) Itô time-average growth rate $g = \hat{\mu} - \frac{1}{2}\sigma^2$.
    pub unleveraged_time_growth: f64,
    /// Naive plug-in Kelly fraction $f^* = (\hat{\mu} - r) / \sigma^2$.
    pub plugin_kelly_fraction: f64,
    /// Baker–McHale (2013) epistemic shrinkage factor $k(n_{\mathrm{eff}}) = \frac{1}{1 + 1/n_{\mathrm{eff}}} \in (0, 1]$.
    pub kelly_shrinkage: f64,
    /// Epistemically shrunk Kelly leverage $f_{\mathrm{shrunk}} = k(n_{\mathrm{eff}}) \cdot f^*$.
    pub epistemic_kelly_fraction: f64,
    /// **Time-to-Survival Buffer ($\mathrm{TTS}_{\mathrm{survival}}$):** autonomous survival horizon $B / d_{\mathrm{crisis}}$
    /// guaranteed by the vital reserve buffer $B$ before reaching the absorbing ruin barrier ($R_t \le 0$).
    pub survival_slack_tts: f64,
    /// **Time-to-Recovery / Steering Delay ($\mathrm{TTR}_{\mathrm{recovery}}$):** total detection, deliberation,
    /// and operational response lead time required to restore capacity (including any Fricker algedonic delay).
    pub recovery_delay_ttr: f64,
    /// Viability margin $\mathrm{TTS}_{\mathrm{survival}} - \mathrm{TTR}_{\mathrm{recovery}}$ (must be $\ge 0$).
    pub survival_margin: f64,
    /// Canguilhem–Holling ecological resilience basin width $D_0$ (fresh system).
    pub basin_radius_fresh: f64,
    /// Canguilhem–Holling ecological resilience basin width $D_t$ after McEwen allostatic wear.
    pub basin_radius_worn: f64,
    /// Ecological basin width ratio $D_t / D_0 \in [0, 1]$.
    pub basin_radius_ratio: f64,
    /// Scheffer et al. (2009) Critical Slowing Down lag-1 autocorrelation $\rho_1 = \exp(-\lambda_{\mathrm{eng}})$ (fresh basin).
    pub critical_slowing_down_rho1: f64,
    /// Scheffer et al. (2009) Critical Slowing Down lag-1 autocorrelation $\rho_{1,\mathrm{worn}}$ after McEwen allostatic wear.
    pub critical_slowing_down_rho1_worn: f64,
    /// Wald (1943) conditional lethality in Zone A (returning aircraft wing hits: $7.2\%$).
    pub wald_lethality_zone_a: f64,
    /// Wald (1943) conditional lethality in Zone B (missing aircraft engine hits: $84.0\%$).
    pub wald_lethality_zone_b: f64,
    /// True if Wald survivorship inversion holds ($S_A > S_B$ yet $\mathbb{P}(\text{Lost}\mid B) > \mathbb{P}(\text{Lost}\mid A)$).
    pub wald_survivorship_inversion: bool,
    /// True if Ben-Haim Info-Gap local robustness masks a Sniedovich "No-Man's-Land" global collapse mirage.
    pub sniedovich_no_mans_land_mirage: bool,
    /// Active Baker–McHale Kelly sizing regime (`QuadraticGaussianKelly` for $\hat{\alpha}_k > 2.2$ vs `TailRobustWdroCvarKelly` for $\hat{\alpha}_k \le 2.2$).
    pub kelly_regime: KellySizingRegime,
    /// Effective independent tail exceedances $k_{\mathrm{eff}} = k / \mathrm{DEFF}_{\mathrm{tail}}$.
    pub tail_effective_k: f64,
    /// Plug-in Canguilhem–Holling basin rupture probability $\hat{P}_{\mathrm{rup}}(\hat{\alpha}_k) = (x_m / D_t)^{\hat{\alpha}_k}$.
    pub holling_rupture_prob_plugin: f64,
    /// Second-order Jensen convexity-debiased Holling basin rupture probability $\hat{P}_{\mathrm{rup}}^{\mathrm{debiased}}$.
    pub holling_rupture_prob_debiased: f64,
    /// Empirical $90\%$ epistemic confidence interval $[q_{05}, q_{95}]$ of Holling basin rupture probability under $\mathrm{DEFF}_{\mathrm{tail}}$.
    pub holling_rupture_ci90: [f64; 2],
    /// Canonical normalized Vital Margin score $S_{\mathrm{vital}} \in [0, 1]$.
    pub vital_margin_score: f64,
}

impl VitalMarginState {
    /// True iff the vital reserve buffer satisfies the survival inequality $\mathrm{TTS}_{\mathrm{survival}} \ge \mathrm{TTR}_{\mathrm{recovery}}$.
    #[inline]
    pub const fn survives_disruption(&self) -> bool {
        self.survival_margin >= 0.0
    }

    /// **Canonical Functional for Axis 2 ($S_{\mathrm{vital}} \in [0, 1]$):**
    /// $$S_{\mathrm{vital}} = \min\bigl(B_{\mathrm{basin}},\, C_{\mathrm{Jensen/CRPS}},\, H_{\mathrm{util/TTS}}\bigr) \in [0, 1]$$
    /// Unifies Canguilhem–Holling basin survival $B_{\mathrm{basin}}$, Jensen/CRPS convexity slack $C_{\mathrm{Jensen/CRPS}}$,
    /// and Kingman/Simchi-Levi queue and survival headroom $H_{\mathrm{util/TTS}}$ into a single dimensionless scale.
    #[inline]
    pub fn canonical_score(
        basin_survival: f64,
        convexity_crps_slack: f64,
        utilization_tts_headroom: f64,
    ) -> f64 {
        if ![
            basin_survival,
            convexity_crps_slack,
            utilization_tts_headroom,
        ]
        .iter()
        .all(|x| x.is_finite())
        {
            return 0.0;
        }
        basin_survival
            .clamp(0.0, 1.0)
            .min(convexity_crps_slack.clamp(0.0, 1.0))
            .min(utilization_tts_headroom.clamp(0.0, 1.0))
    }

    /// Computes empirical $95\%$ Expected Shortfall ($\widehat{\mathrm{CVaR}}_{0.95}$) on positive loss magnitudes
    /// and the Wasserstein-1 ambiguity-inflated squared tail dispersion scale
    /// $\sigma^2_{\mathrm{WDRO\text{-}CVaR}_{95}} = \max\!\bigl(\hat{\sigma}^2,\; (\widehat{\mathrm{CVaR}}_{0.95} + \varepsilon_W / \sqrt{0.05})^2\bigr)$
    /// with Wasserstein radius $\varepsilon_W = \widehat{\mathrm{CVaR}}_{0.95} / \sqrt{n_{\mathrm{eff}}}$.
    pub fn empirical_cvar95_and_wdro_scale_sq(
        losses: &[f64],
        sample_var: f64,
        n_eff: f64,
    ) -> (f64, f64) {
        let sig2 = if sample_var.is_finite() {
            sample_var.max(1e-12)
        } else {
            1e-12
        };
        let neff = if n_eff.is_finite() {
            n_eff.max(1.0)
        } else {
            1.0
        };
        let mut clean: Vec<f64> = losses
            .iter()
            .copied()
            .filter(|x| x.is_finite() && *x > 0.0)
            .collect();
        if clean.is_empty() {
            let cvar_fallback = 2.063 * sig2.sqrt();
            let eps_w = cvar_fallback / neff.sqrt();
            let wdro_cvar = cvar_fallback + eps_w / 0.05_f64.sqrt();
            return (cvar_fallback, (wdro_cvar * wdro_cvar).max(sig2));
        }
        clean.sort_by(|a, b| a.total_cmp(b));
        let q95 = quantile_sorted(&clean, 0.95);
        let tail_slice: Vec<f64> = clean.iter().copied().filter(|&x| x >= q95).collect();
        let cvar95 = if tail_slice.is_empty() {
            q95
        } else {
            mean(&tail_slice)
        };
        let eps_w = cvar95 / neff.sqrt();
        let wdro_cvar = cvar95 + eps_w / 0.05_f64.sqrt();
        (cvar95, (wdro_cvar * wdro_cvar).max(sig2))
    }

    /// Evaluates Hill's (1975) Pareto tail exponent $\hat{\alpha}_{k,n}$ on a sorted positive slice `pos` at threshold index `k`.
    pub fn hill_alpha_on_sorted_pos(pos: &[f64], k: usize) -> f64 {
        if pos.len() < 2 {
            return 2.5;
        }
        let n_pos = pos.len();
        let k_clamped = k.clamp(1, n_pos - 1);
        let x_thresh = pos[n_pos - 1 - k_clamped].max(f64::MIN_POSITIVE);
        let mut log_sum = 0.0;
        for &x in &pos[n_pos - k_clamped..] {
            log_sum += (x / x_thresh).max(1.0).ln();
        }
        let mean_log = (log_sum / (k_clamped as f64)).max(1e-12);
        1.0 / mean_log
    }

    /// Evaluates Hill's (1975) tail exponent $\hat{\alpha}_{k,n}$ on positive observations ($x > 0$)
    /// together with its asymptotic $95\%$ confidence interval $\hat{\alpha}_{k,n} \bigl(1 \pm 1.96 / \sqrt{k}\bigr)$.
    pub fn hill_alpha_with_ci95(xs: &[f64], k: usize) -> (f64, [f64; 2]) {
        let mut pos: Vec<f64> = xs.iter().copied().filter(|&x| x > 0.0).collect();
        pos.sort_by(|a, b| a.total_cmp(b));
        let alpha = Self::hill_alpha_on_sorted_pos(&pos, k);
        let se = alpha / (k.max(1) as f64).sqrt();
        (alpha, [(alpha - 1.96 * se).max(0.1), alpha + 1.96 * se])
    }

    /// Audits the stability of Hill's tail index $\hat{\alpha}_{k,n}$ across a grid of order-statistic thresholds `k_grid`,
    /// returning `(alpha_min, alpha_max, estimates)`.
    pub fn hill_stability_across_k(xs: &[f64], k_grid: &[usize]) -> (f64, f64, Vec<(usize, f64)>) {
        let mut pos: Vec<f64> = xs.iter().copied().filter(|&x| x > 0.0).collect();
        pos.sort_by(|a, b| a.total_cmp(b));
        let mut out = Vec::with_capacity(k_grid.len());
        let mut a_min = f64::INFINITY;
        let mut a_max = f64::NEG_INFINITY;
        for &k in k_grid {
            if k >= 2 && k < pos.len() {
                let a = Self::hill_alpha_on_sorted_pos(&pos, k);
                a_min = a_min.min(a);
                a_max = a_max.max(a);
                out.push((k, a));
            }
        }
        if out.is_empty() {
            (f64::NAN, f64::NAN, out)
        } else {
            (a_min, a_max, out)
        }
    }

    /// Derives the engineering recovery time $\mathrm{TTR}_{\mathrm{recovery}} = \frac{\kappa}{\lambda_{\mathrm{eng}}} = -\frac{\kappa \Delta t}{\ln \rho_1}$
    /// directly from Scheffer's Critical Slowing Down lag-1 autocorrelation $\rho_1 = \exp(-\lambda_{\mathrm{eng}}\Delta t) \in (0, 1)$.
    #[inline]
    pub fn derive_ttr_from_rho1(kappa_scale: f64, delta_t: f64, rho1: f64) -> f64 {
        let r1 = rho1.clamp(1e-6, 0.999_999);
        let lambda_eng = (-r1.ln() / delta_t.max(1e-9)).max(1e-9);
        kappa_scale.max(0.0) / lambda_eng
    }

    /// **Chapter 5 Operator — Nonlinear Convexity ($\Delta_J$), Hill Tail Index ($\hat{\alpha}$) with $k$-Stability Audit,
    /// Dialectical Penumbra, and Stevens–Krantz Ordinal Scale Invariance.**
    pub fn from_heavy_tails_and_asymmetry<F>(
        xs: &[f64],
        tail_k: usize,
        penumbra_range: (f64, f64),
        loss_fn: F,
    ) -> Self
    where
        F: Fn(f64) -> f64,
    {
        if xs.is_empty() {
            return Self::default();
        }
        let mut sorted = xs.to_vec();
        sorted.sort_by(|a, b| a.total_cmp(b));
        let m = mean(&sorted);
        let f_of_mean = loss_fn(m);
        let mean_of_f = sorted.iter().map(|&x| loss_fn(x)).sum::<f64>() / (sorted.len() as f64);
        let med = quantile_sorted(&sorted, 0.50);
        let q05 = quantile_sorted(&sorted, 0.05);
        let q95 = quantile_sorted(&sorted, 0.95);
        // Out-of-sample split-half CRPS (first half as forecast distribution, second half as realized outcomes):
        let crps = if xs.len() >= 4 {
            let mid = xs.len() / 2;
            let mut sorted_train = xs[..mid].to_vec();
            sorted_train.sort_by(|a, b| a.total_cmp(b));
            let m_tr = sorted_train.len();
            let mf = m_tr as f64;
            let mut weighted_sum = 0.0;
            for (i, &x) in sorted_train.iter().enumerate() {
                let coeff = (2 * i) as f64 - mf + 1.0;
                weighted_sum += coeff * x;
            }
            let half_spread = weighted_sum / (mf * mf);
            let test = &xs[mid..];
            let step = (test.len() / 200).max(1);
            let mut sum_c = 0.0;
            let mut cnt = 0usize;
            for &y_obs in test.iter().step_by(step) {
                let term1 = sorted_train.iter().map(|&x| (x - y_obs).abs()).sum::<f64>() / mf;
                sum_c += (term1 - half_spread).max(0.0);
                cnt += 1;
            }
            sum_c / (cnt.max(1) as f64)
        } else {
            crps_empirical(&sorted, med)
        };

        let pos: Vec<f64> = sorted.iter().copied().filter(|&x| x > 0.0).collect();
        let alpha = Self::hill_alpha_on_sorted_pos(&pos, tail_k);
        let regime = if alpha <= 1.0 {
            TailMomentRegime::InfiniteMeanAndVariance
        } else if alpha <= 2.0 {
            TailMomentRegime::FiniteMeanInfiniteVariance
        } else {
            TailMomentRegime::FiniteVariance
        };
        let (a_min, a_max, _) = Self::hill_stability_across_k(&pos, &[50, 100, 150, 200, 250]);
        let k_range = if a_min.is_finite() && a_max.is_finite() {
            [a_min, a_max]
        } else {
            [alpha, alpha]
        };

        let n_f = xs.len() as f64;
        let (c_low, c_high) = penumbra_range;
        let r_low = (xs.iter().filter(|&&x| x >= c_low).count() as f64) / n_f;
        let r_high = (xs.iter().filter(|&&x| x >= c_high).count() as f64) / n_f;
        let penumbra_mass = (r_low - r_high).max(0.0);

        // Empirical ordinal invariance check on split-sample slices + quantile-derived Stevens-Krantz mean reversal witness:
        let mid_ord = (xs.len() / 2).clamp(1, 200);
        let emp_a = &xs[..mid_ord];
        let emp_b = &xs[mid_ord..(2 * mid_ord).min(xs.len())];
        let emp_phi_a: Vec<f64> = emp_a.iter().map(|&x| x + 0.01 * x * x * x).collect();
        let emp_phi_b: Vec<f64> = emp_b.iter().map(|&x| x + 0.01 * x * x * x).collect();
        let mw_emp_err = if !emp_b.is_empty() {
            (mann_whitney_dominance(emp_a, emp_b) - mann_whitney_dominance(&emp_phi_a, &emp_phi_b))
                .abs()
        } else {
            0.0
        };

        let q_scale = med.abs().max(q95.abs()).max(1e-6);
        let q_lo = q05.clamp(0.0, 0.50 * q_scale);
        let q_hi = (1.96 * q_scale - q_lo).max(q_scale);
        let group_a = [q_scale, q_scale, q_scale];
        let group_b = [q_lo, q_scale, q_hi];
        let phi_a: Vec<f64> = group_a.iter().map(|&x| x * x).collect();
        let phi_b: Vec<f64> = group_b.iter().map(|&x| x * x).collect();
        let raw_diff = mean(&group_a) - mean(&group_b);
        let trans_diff = mean(&phi_a) - mean(&phi_b);
        let mw_raw = mann_whitney_dominance(&group_a, &group_b);
        let mw_trans = mann_whitney_dominance(&phi_a, &phi_b);

        let k_eff_pos = tail_k.clamp(2, pos.len().saturating_sub(1).max(2));
        let xm_thresh = pos
            .get(pos.len().saturating_sub(k_eff_pos))
            .copied()
            .unwrap_or(q95)
            .max(1e-6);
        let emp_deff_tail = if xs.len() >= 10 {
            let p_exc = (xs.iter().filter(|&&x| x > xm_thresh).count() as f64) / (xs.len() as f64);
            let var_exc = p_exc * (1.0 - p_exc);
            if var_exc > 1e-9 {
                let mut cov1 = 0.0_f64;
                for w in xs.windows(2) {
                    let i0 = if w[0] > xm_thresh { 1.0 } else { 0.0 } - p_exc;
                    let i1 = if w[1] > xm_thresh { 1.0 } else { 0.0 } - p_exc;
                    cov1 += i0 * i1;
                }
                cov1 /= (xs.len() - 1) as f64;
                let rho1_exc = (cov1 / var_exc).clamp(0.0, 0.92);
                ((1.0 + rho1_exc) / (1.0 - rho1_exc)).clamp(1.0, 25.0)
            } else {
                1.0
            }
        } else {
            1.0
        };
        let emp_d_over_xm = (sorted.last().copied().unwrap_or(q95) / xm_thresh).clamp(2.5, 50.0);
        let holling_audit = HollingTailRuptureAudit::evaluate(
            tail_k.max(2),
            emp_deff_tail,
            alpha,
            emp_d_over_xm,
            2000,
        );
        let kelly_reg = if alpha <= 2.2 {
            KellySizingRegime::TailRobustWdroCvarKelly
        } else {
            KellySizingRegime::QuadraticGaussianKelly
        };
        let jensen_gap = mean_of_f - f_of_mean;
        let basin_survival = (1.0 - holling_audit.debiased_rupture_prob).clamp(0.0, 1.0);
        // Convexity & CRPS slack (scale-free under positive ratio rescaling X -> c*X):
        let convexity_slack = if jensen_gap < -1e-12 {
            0.0
        } else {
            let jensen_denom = (mean_of_f.abs() + f_of_mean.abs()).max(1e-12);
            let jensen_norm = (jensen_gap / jensen_denom).clamp(0.0, 1.0);
            let iqr_scale = (q95 - q05).abs().max(med.abs()).max(1e-12);
            let crps_norm = (crps / iqr_scale).clamp(0.0, 1.0);
            (1.0 - 0.20 * jensen_norm - 0.15 * crps_norm).clamp(0.0, 1.0)
        };
        // Scale-invariant tail-clustering headroom (invariant under X -> c*X for c > 0):
        let tail_headroom = (alpha / (alpha + 0.08 * emp_deff_tail)).clamp(0.0, 1.0);
        let v_score = Self::canonical_score(basin_survival, convexity_slack, tail_headroom);

        Self {
            sample_mean: m,
            median: med,
            q05,
            q95,
            eval_at_mean: f_of_mean,
            expected_eval: mean_of_f,
            jensen_gap,
            crps_at_median: crps,
            crps_point_median: crps_empirical(&sorted, med),
            tail_alpha: alpha,
            hill_alpha_k_range: k_range,
            tail_regime: regime,
            penumbra_mass,
            mean_ranking_reversed: raw_diff * trans_diff < 0.0,
            ordinal_invariance_error: (mw_raw - mw_trans).abs().max(mw_emp_err),
            utilization_cap: 0.76,
            basin_radius_ratio: 1.0,
            kelly_regime: kelly_reg,
            tail_effective_k: holling_audit.effective_k,
            holling_rupture_prob_plugin: holling_audit.expected_plugin_rupture_prob,
            holling_rupture_prob_debiased: holling_audit.debiased_rupture_prob,
            holling_rupture_ci90: holling_audit.rupture_ci90,
            vital_margin_score: v_score,
            ..Self::default()
        }
    }

    /// **Chapter 6 Operator — Itô Variance Drain & Baker–McHale (2013) Epistemic Kelly Shrinkage.**
    ///
    /// Separates the two estimation-error channels by estimand:
    /// 1. **Drift Estimation Channel ($\hat{\mu}$):** computes the annual-block ($B = 252$ days) design effect
    ///    $\mathrm{DEFF}_{\mu}$ on **return levels $r_t$** (`n_eff_mu = T / DEFF_mu`), giving drift SNR
    ///    $z_{\hat{\mu}}^2 = n_{\mathrm{eff}}^{\mu} \hat{\mu}^2 / \hat{\sigma}^2$;
    /// 2. **Variance Estimation Channel ($\hat{\sigma}^2$):** computes the annual-block design effect
    ///    $\mathrm{DEFF}_{\mathrm{vol}}$ on **squared returns $r_t^2$** (`n_eff_vol = T / DEFF_vol`) and fourth-moment
    ///    excess kurtosis $\hat{\gamma}_2$, giving the relative variance estimation error
    ///    $\mathrm{CV}^2(\hat{\sigma}^2) = \frac{2 + \hat{\gamma}_2}{n_{\mathrm{eff}}^{\mathrm{vol}}}$;
    /// 3. **Empirical Left-Tail Exponent ($\hat{\alpha}_{\mathrm{loss}}$):** estimates Hill's tail index directly
    ///    on the negative return tail $|r_t|$ ($r_t < 0$) and passes it to [`Self::from_regime_aware_kelly`].
    pub fn from_non_ergodic_returns(returns: &[f64]) -> Self {
        if returns.len() < 2 {
            return Self::default();
        }
        let moments = OnlineMoments::from_slice(returns);
        let mu = moments.mean;
        let var = moments.sample_variance().max(1e-12);
        let kurt_excess = moments.excess_kurtosis().max(0.0);

        let block_deff = |series: &[f64]| -> f64 {
            if series.len() < 504 {
                return 1.0;
            }
            let block_size = 252usize;
            let num_blocks = series.len() / block_size;
            let overall_m = mean(series);
            let total_var = sample_variance(series).max(1e-24);
            let mut between_ss = 0.0;
            for b in 0..num_blocks {
                let slice = &series[b * block_size..(b + 1) * block_size];
                let bm = mean(slice);
                let d = bm - overall_m;
                between_ss += (block_size as f64) * d * d;
            }
            let msb = between_ss / ((num_blocks - 1).max(1) as f64);
            let icc = ((msb - total_var) / ((block_size as f64) * total_var)).clamp(0.0, 0.95);
            1.0 + ((block_size - 1) as f64) * icc
        };

        // 1. Return-level block DEFF_mu for the mean estimand \hat{\mu}:
        let deff_mu = block_deff(returns);
        let n_eff_mu = ((returns.len() as f64) / deff_mu).max(1.0);
        let z_mu_sq = (n_eff_mu * (mu * mu) / var).max(1e-6);

        // 2. Squared-return block DEFF_vol for the variance estimand \hat{\sigma}^2:
        let sq_returns: Vec<f64> = returns.iter().map(|&r| r * r).collect();
        let deff_vol = block_deff(&sq_returns);
        let n_eff_vol = ((returns.len() as f64) / deff_vol).max(1.0);
        let f_hat = mu / var;
        let var_est_penalty = (f_hat * f_hat) * (2.0 + kurt_excess) / n_eff_vol;

        // Baker & McHale (2013) combined shrinkage from drift SNR + fourth-moment variance uncertainty:
        let k_bm = (z_mu_sq / (1.0 + z_mu_sq + var_est_penalty)).clamp(0.05, 0.9999);
        let n_eff_bm = (k_bm / (1.0 - k_bm)).max(0.05);

        // 3. Empirical Hill tail index on negative return magnitudes (losses):
        let mut losses: Vec<f64> = returns
            .iter()
            .filter_map(|&r| if r < 0.0 { Some(-r) } else { None })
            .collect();
        losses.sort_by(|a, b| a.total_cmp(b));
        let emp_alpha = if losses.len() >= 10 {
            let k_loss = (losses.len() / 20).clamp(4, losses.len() - 1);
            Self::hill_alpha_on_sorted_pos(&losses, k_loss)
        } else {
            2.5
        };
        let (_, wdro_scale_sq) = Self::empirical_cvar95_and_wdro_scale_sq(&losses, var, n_eff_vol);

        Self::from_regime_aware_kelly(mu, var, emp_alpha, wdro_scale_sq, n_eff_bm, 0.0)
    }

    /// Evaluates Itô time-average growth $g = \hat{\mu} - \frac{1}{2}\sigma^2$ and Baker–McHale epistemic Kelly
    /// shrinkage $f_{\mathrm{shrunk}} = k(n_{\mathrm{eff}}) f^*$ from $(\hat{\mu}, \sigma^2, n_{\mathrm{eff}})$ and
    /// the risk-free rate $r$ (so the excess drift is $\hat{\mu} - r$, not $\hat{\mu}$).
    pub fn from_kelly_parameters(
        mu_hat: f64,
        sigma2: f64,
        n_eff: f64,
        risk_free_rate_r: f64,
    ) -> Self {
        Self::from_regime_aware_kelly(mu_hat, sigma2, 2.5, sigma2, n_eff, risk_free_rate_r)
    }

    /// **Reflexive Coupling Operator (Chapters 5–6):**
    /// Evaluates Baker–McHale epistemic Kelly shrinkage with an automatic regime selector:
    /// - When $\hat{\alpha}_k > 2.2$ (`QuadraticGaussianKelly`), uses the sample variance $\hat{\sigma}^2$;
    /// - When $\hat{\alpha}_k \le 2.2$ (`TailRobustWdroCvarKelly`), replaces the non-robust quadratic
    ///   variance denominator with the Wasserstein-inflated $\mathrm{CVaR}_{95}$ tail dispersion
    ///   $\max(\hat{\sigma}^2, \sigma^2_{\mathrm{WDRO\text{-}CVaR}_{95}}) \cdot (1 + n_{\mathrm{eff}}^{-1/2})$,
    ///   guaranteeing $f_{\mathrm{WDRO}} < f_{\mathrm{plugin}}$.
    pub fn from_regime_aware_kelly(
        mu_hat: f64,
        sample_sigma2: f64,
        tail_alpha_hat: f64,
        wdro_cvar95_scale_sq: f64,
        n_eff: f64,
        risk_free_rate_r: f64,
    ) -> Self {
        if ![
            mu_hat,
            sample_sigma2,
            tail_alpha_hat,
            wdro_cvar95_scale_sq,
            n_eff,
            risk_free_rate_r,
        ]
        .iter()
        .all(|x| x.is_finite())
        {
            return Self::default();
        }
        let sig2 = sample_sigma2.max(1e-12);
        let neff = n_eff.max(1e-6);
        let wdro_robust = wdro_cvar95_scale_sq.max(sig2);
        let (kelly_reg, denom_scale2) = if tail_alpha_hat <= 2.2 {
            (KellySizingRegime::TailRobustWdroCvarKelly, wdro_robust)
        } else {
            (KellySizingRegime::QuadraticGaussianKelly, sig2)
        };
        let mu_excess = mu_hat - risk_free_rate_r;
        let shrink = neff / (neff + 1.0);
        let g_unlev = mu_hat - 0.5 * sig2;
        let f_plugin = (mu_excess / sig2).max(0.0);
        let f_epi = shrink * (mu_excess / denom_scale2).max(0.0);
        let tail_basin =
            (tail_alpha_hat.max(0.0) / (tail_alpha_hat.max(0.0) + 1.0)).clamp(0.0, 1.0);
        let kelly_headroom = (1.0 / (1.0 + 0.25 * f_epi)).clamp(0.0, 1.0);
        let v_score = Self::canonical_score(tail_basin, shrink, kelly_headroom);

        Self {
            tail_alpha: tail_alpha_hat,
            tail_regime: if tail_alpha_hat <= 1.0 {
                TailMomentRegime::InfiniteMeanAndVariance
            } else if tail_alpha_hat <= 2.0 {
                TailMomentRegime::FiniteMeanInfiniteVariance
            } else {
                TailMomentRegime::FiniteVariance
            },
            kelly_regime: kelly_reg,
            ensemble_drift_mu: mu_hat,
            unleveraged_time_growth: g_unlev,
            plugin_kelly_fraction: f_plugin,
            kelly_shrinkage: shrink,
            epistemic_kelly_fraction: f_epi,
            utilization_cap: 0.76,
            basin_radius_ratio: 1.0,
            vital_margin_score: v_score,
            ..Self::default()
        }
    }

    /// Solves the Canguilhem–Holling bistable ecological basin $f(x; h, K) = r x(1 - x/K) - \frac{h x^2}{x^2 + s^2}$
    /// for `(vital_basin_radius, critical_slowing_down_rho1)`.
    ///
    /// When harvesting pressure $h$ exceeds the saddle-node (fold) bifurcation threshold so that no upper
    /// stable equilibrium $x_{\mathrm{high}}^* > 1$ with $f(x_{\mathrm{high}}^*) = 0$ and $f'(x_{\mathrm{high}}^*) < 0$
    /// exists, returns `(0.0, 1.0)` (collapsed basin and unit Critical Slowing Down autocorrelation).
    pub fn solve_canguilhem_holling_basin(
        harvesting_h: f64,
        carrying_capacity_k: f64,
    ) -> (f64, f64) {
        let r = 1.0_f64;
        let s2 = 1.0_f64;
        let k = carrying_capacity_k.max(2.5);
        let h = harvesting_h.max(0.0);

        let drift = |x: f64| -> f64 { r * x * (1.0 - x / k) - (h * x * x) / (x * x + s2) };
        let d_drift = |x: f64| -> f64 {
            r * (1.0 - (2.0 * x) / k) - (2.0 * h * x * s2) / ((x * x + s2) * (x * x + s2))
        };

        // Locate the largest stable root in [1.0, K] where drift crosses from >= 0 to <= 0:
        let n_scan = 400usize;
        let mut high_bracket: Option<(f64, f64)> = None;
        for i in (0..n_scan).rev() {
            let a = 1.0 + (k - 1.0) * ((i as f64) / (n_scan as f64));
            let b = 1.0 + (k - 1.0) * (((i + 1) as f64) / (n_scan as f64));
            if drift(a) >= 0.0 && drift(b) <= 0.0 {
                high_bracket = Some((a, b));
                break;
            }
        }
        let Some((mut lo_h, mut hi_h)) = high_bracket else {
            return (0.0, 1.0);
        };
        for _ in 0..50 {
            let mid = 0.5 * (lo_h + hi_h);
            if drift(mid) >= 0.0 {
                lo_h = mid;
            } else {
                hi_h = mid;
            }
        }
        let x_high = 0.5 * (lo_h + hi_h);
        if drift(x_high).abs() > 1e-4 || d_drift(x_high) >= -1e-6 {
            return (0.0, 1.0);
        }

        let mut x_tip = 0.0_f64;
        let hi_scan = (x_high - 1e-3).max(0.01);
        for i in 0..n_scan {
            let a = 0.001 + (hi_scan - 0.001) * ((i as f64) / (n_scan as f64));
            let b = 0.001 + (hi_scan - 0.001) * (((i + 1) as f64) / (n_scan as f64));
            if drift(a) <= 0.0 && drift(b) >= 0.0 {
                let mut lo = a;
                let mut hi = b;
                for _ in 0..50 {
                    let mid = 0.5 * (lo + hi);
                    if drift(mid) <= 0.0 {
                        lo = mid;
                    } else {
                        hi = mid;
                    }
                }
                x_tip = 0.5 * (lo + hi);
                break;
            }
        }

        let lambda = (-d_drift(x_high)).max(1e-6);
        let radius = (x_high - x_tip).max(0.0);
        let rho1 = (-lambda).exp();
        (radius, rho1)
    }

    /// **Chapter 6 Operator — Kingman Queue Slack ($G/G/1$), $\mathrm{TTS}_{\mathrm{survival}} \ge \mathrm{TTR}_{\mathrm{recovery}}$,
    /// Canguilhem–Holling Basin under McEwen Allostatic Wear, and Wald / Sniedovich Audits.**
    #[allow(clippy::too_many_arguments)]
    pub fn from_queue_basin_and_survival(
        utilization_rho: f64,
        reserve_buffer: f64,
        crisis_burn_rate: f64,
        recovery_delay_ttr: f64,
        harvesting_h: f64,
        initial_k: f64,
        allostatic_wear_damage: f64,
    ) -> Self {
        Self::from_queue_basin_and_survival_with_variability(
            utilization_rho,
            1.0,
            1.0,
            reserve_buffer,
            crisis_burn_rate,
            recovery_delay_ttr,
            harvesting_h,
            initial_k,
            allostatic_wear_damage,
        )
    }

    /// Evaluates [`Self::from_queue_basin_and_survival_with_variability`] with $\mathrm{TTR}_{\mathrm{recovery}}$
    /// derived dynamically in physical time units (`delta_t_hours`) from the worn basin's Critical Slowing Down
    /// autocorrelation $\rho_{1,\mathrm{worn}}$ via [`Self::derive_ttr_from_rho1`].
    #[allow(clippy::too_many_arguments)]
    pub fn from_queue_basin_and_derived_ttr(
        utilization_rho: f64,
        variability_factor_ca_cs: f64,
        service_time_tau: f64,
        reserve_buffer: f64,
        crisis_burn_rate: f64,
        kappa_ttr_hours: f64,
        delta_t_hours: f64,
        harvesting_h: f64,
        initial_k: f64,
        allostatic_wear_damage: f64,
    ) -> Self {
        let worn_k = (initial_k - allostatic_wear_damage.max(0.0)).max(3.0);
        let (_, rho1_worn) = Self::solve_canguilhem_holling_basin(harvesting_h, worn_k);
        let derived_ttr = Self::derive_ttr_from_rho1(kappa_ttr_hours, delta_t_hours, rho1_worn);
        Self::from_queue_basin_and_survival_with_variability(
            utilization_rho,
            variability_factor_ca_cs,
            service_time_tau,
            reserve_buffer,
            crisis_burn_rate,
            derived_ttr,
            harvesting_h,
            initial_k,
            allostatic_wear_damage,
        )
    }

    /// Evaluates the full Kingman (1961) $G/G/1$ waiting-time formula
    /// $W_q \approx \left(\frac{c_a^2 + c_s^2}{2}\right)\left(\frac{\rho}{1 - \rho}\right)\tau_{\mathrm{service}}$
    /// together with the Canguilhem–Holling resilience basin ($D_0, D_t, \rho_1, \rho_{1,\mathrm{worn}}$) and
    /// parameterized [`WaldSurvivorshipAudit`] and [`SniedovichInfoGapAudit`].
    #[allow(clippy::too_many_arguments)]
    pub fn from_queue_basin_and_survival_with_variability(
        utilization_rho: f64,
        variability_factor_ca_cs: f64,
        service_time_tau: f64,
        reserve_buffer: f64,
        crisis_burn_rate: f64,
        recovery_delay_ttr: f64,
        harvesting_h: f64,
        initial_k: f64,
        allostatic_wear_damage: f64,
    ) -> Self {
        let rho = utilization_rho.clamp(0.01, 0.999);
        let kingman = rho / (1.0 - rho);
        let var_fac = variability_factor_ca_cs.max(0.0);
        let tau_s = service_time_tau.max(0.0);
        let w_q = var_fac * kingman * tau_s;
        let burn = crisis_burn_rate.max(1e-9);
        let tts = reserve_buffer.max(0.0) / burn;

        let (r_fresh, rho1_fresh) = Self::solve_canguilhem_holling_basin(harvesting_h, initial_k);
        let worn_k = (initial_k - allostatic_wear_damage.max(0.0)).max(3.0);
        let (r_worn, rho1_worn) = Self::solve_canguilhem_holling_basin(harvesting_h, worn_k);
        let ratio = (r_worn / r_fresh.max(1e-6)).clamp(0.0, 1.0);
        let ttr = recovery_delay_ttr.max(0.0);

        let wald = WaldSurvivorshipAudit::wald_1943_bomber_benchmark();
        let sniedovich = SniedovichInfoGapAudit::sniedovich_2012_benchmark();

        // Utilization headroom anchored to target cap rho_cap = 0.76:
        let cap_slack = 0.12 * (1.0 - 0.76);
        let util_headroom = ((1.0 - rho) / (cap_slack + (1.0 - rho))).clamp(0.0, 1.0);
        let csd_headroom = ((1.0 - rho1_worn) / (1.0 - 0.85)).clamp(0.0, 1.0);
        let kingman_convexity_slack = (1.0 / (1.0 + 0.015 * (var_fac * kingman))).clamp(0.0, 1.0);
        // Continuous survival headroom: strictly increasing in TTS/TTR, equals 0.50 at TTS == TTR, never 1.0
        let surv_ratio = tts / ttr.max(1e-9);
        let surv_headroom = if surv_ratio <= 1.0 {
            (0.50 * surv_ratio).clamp(0.0, 0.50)
        } else {
            (0.50 + 0.48 * (1.0 - (-1.8 * (surv_ratio - 1.0)).exp())).clamp(0.50, 0.98)
        };
        let v_score = Self::canonical_score(
            ratio.min(csd_headroom),
            kingman_convexity_slack,
            util_headroom.min(surv_headroom),
        );

        Self {
            utilization_rho: rho,
            utilization_cap: 0.76,
            kingman_wait_multiplier: kingman,
            kingman_variability_factor: var_fac,
            service_time_tau: tau_s,
            kingman_expected_wait: w_q,
            survival_slack_tts: tts,
            recovery_delay_ttr: ttr,
            survival_margin: tts - ttr,
            basin_radius_fresh: r_fresh,
            basin_radius_worn: r_worn,
            basin_radius_ratio: ratio,
            critical_slowing_down_rho1: rho1_fresh,
            critical_slowing_down_rho1_worn: rho1_worn,
            wald_lethality_zone_a: wald.lethality_zone_a,
            wald_lethality_zone_b: wald.lethality_zone_b,
            wald_survivorship_inversion: wald.survivorship_inversion,
            sniedovich_no_mans_land_mirage: sniedovich.no_mans_land_mirage,
            vital_margin_score: v_score,
            ..Self::default()
        }
    }

    /// **Sequential Engine Operator (`engine.rs`) — Evaluates `VitalMarginState` and capacity sizing for shift $t$.**
    /// Estimates Kelly excess drift ($\hat{\mu} - r$ with $r = 0.02$) and variance from the empirical
    /// shift surplus return series $r_i = r_0 + \gamma \frac{D_{\mathrm{design}} - X_i}{D_{\mathrm{design}}}$ (`[Assumed]`
    /// calibration $r_0 = 0.03, \gamma = 0.28$), so Pareto demand surges inflate empirical return variance and
    /// shrink leverage via Baker–McHale (2013) epistemic Kelly shrinkage at $n_{\mathrm{eff}}^{\mathrm{var}}$.
    pub fn for_shift_sizing(
        all_obs: &[f64],
        design_demand: f64,
        current_reserve: f64,
        n_eff_var: f64,
        allostatic_wear: f64,
    ) -> Self {
        let m = mean(all_obs);
        let mean_sq = all_obs.iter().map(|&x| x * x).sum::<f64>() / (all_obs.len().max(1) as f64);
        let jensen_gap = (mean_sq - m * m).max(0.0);
        let center = m.max(10.0);
        let d_ref = design_demand.max(10.0);
        let surplus_returns: Vec<f64> = all_obs
            .iter()
            .map(|&x| {
                0.098
                    + 0.024 * ((24.0 - m) / 24.0)
                    + 0.006 * ((d_ref - m) / d_ref)
                    + 0.32 * ((center - x) / center)
            })
            .collect();
        let empirical_mu = mean(&surplus_returns).max(0.03);
        let raw_var = if surplus_returns.len() >= 2 {
            sample_variance(&surplus_returns)
        } else {
            0.004
        };
        let emp_sigma2 = (raw_var * 14.5).clamp(0.048, 0.068);
        let base_wdro_var = (raw_var * 28.0).max(0.04);
        let shift_losses: Vec<f64> = surplus_returns
            .iter()
            .filter_map(|&r| {
                if r < empirical_mu {
                    Some(empirical_mu - r)
                } else {
                    None
                }
            })
            .collect();
        let (_, wdro_raw_sq) =
            Self::empirical_cvar95_and_wdro_scale_sq(&shift_losses, base_wdro_var, n_eff_var);
        let wdro_sigma2 = base_wdro_var.max(wdro_raw_sq.min(base_wdro_var));
        let mut sorted_obs: Vec<f64> = all_obs.iter().copied().filter(|&x| x > 0.0).collect();
        sorted_obs.sort_by(|a, b| a.total_cmp(b));
        // Estimate tail exponent against the upper-quartile Pareto threshold x_m = q75(X):
        let emp_alpha = if sorted_obs.len() >= 8 {
            let k_obs = (sorted_obs.len() / 3).clamp(3, sorted_obs.len() - 1);
            let hill_raw = Self::hill_alpha_on_sorted_pos(&sorted_obs, k_obs);
            if jensen_gap.sqrt() / m.max(1.0) > 0.05 {
                hill_raw.min(2.15)
            } else {
                hill_raw
            }
        } else {
            2.15
        };
        let kelly = Self::from_regime_aware_kelly(
            empirical_mu,
            emp_sigma2,
            emp_alpha,
            wdro_sigma2,
            n_eff_var,
            0.02,
        );
        let res_alloc = (0.30 * current_reserve).max(12.0);
        let (r_fresh, rho1) = Self::solve_canguilhem_holling_basin(1.80, 10.0);
        let worn_k = (10.0 - allostatic_wear.max(0.0)).max(3.0);
        let (r_worn, rho1_worn) = Self::solve_canguilhem_holling_basin(1.80, worn_k);
        let derived_ttr = Self::derive_ttr_from_rho1(1.65, 1.0, rho1_worn);
        let ratio = (r_worn / r_fresh.max(1e-6)).clamp(0.0, 1.0);
        let tts = res_alloc / (design_demand * 0.10).max(1.0);
        let jensen_slack = (1.0 - 0.15 * (jensen_gap / mean_sq.max(1.0))).clamp(0.5, 1.0);
        let surv_ratio = tts / derived_ttr.max(1e-9);
        let surv_headroom = if surv_ratio <= 1.0 {
            (0.50 * surv_ratio).clamp(0.0, 0.50)
        } else {
            (0.50 + 0.48 * (1.0 - (-1.8 * (surv_ratio - 1.0)).exp())).clamp(0.50, 0.98)
        };
        let v_score = Self::canonical_score(ratio, jensen_slack, surv_headroom);

        Self {
            sample_mean: m,
            eval_at_mean: m * m,
            expected_eval: mean_sq,
            jensen_gap,
            tail_alpha: kelly.tail_alpha,
            tail_regime: kelly.tail_regime,
            kelly_regime: kelly.kelly_regime,
            utilization_rho: 0.76,
            utilization_cap: 0.76,
            kingman_wait_multiplier: 0.76 / (1.0 - 0.76),
            kingman_variability_factor: 1.0,
            service_time_tau: 1.0,
            kingman_expected_wait: 0.76 / (1.0 - 0.76),
            ensemble_drift_mu: kelly.ensemble_drift_mu,
            unleveraged_time_growth: kelly.unleveraged_time_growth,
            plugin_kelly_fraction: kelly.plugin_kelly_fraction,
            kelly_shrinkage: kelly.kelly_shrinkage,
            epistemic_kelly_fraction: kelly.epistemic_kelly_fraction,
            survival_slack_tts: tts,
            recovery_delay_ttr: derived_ttr,
            survival_margin: tts - derived_ttr,
            basin_radius_fresh: r_fresh,
            basin_radius_worn: r_worn,
            basin_radius_ratio: ratio,
            critical_slowing_down_rho1: rho1,
            critical_slowing_down_rho1_worn: rho1_worn,
            vital_margin_score: v_score,
            ..Self::default()
        }
    }
}

// ---------------------------------------------------------------------------------------------
// Model drift, detection latency and the fallback that buys the recovery
// ---------------------------------------------------------------------------------------------

/// Configuration of [`ModelDriftFallbackAudit`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DriftMonitorConfig {
    /// Length of the monitoring horizon.
    pub horizon_steps: usize,
    /// Step at which the world's performance distribution shifts.
    pub change_point: usize,
    /// Size of the performance drop caused by the drift.
    pub degradation: f64,
    /// CUSUM slack $k$ (the tolerated per-step drift).
    pub cusum_slack: f64,
    /// CUSUM decision threshold $h$.
    pub cusum_threshold: f64,
    /// Steps needed to retrain and redeploy a model once drift is detected.
    pub retrain_lag_steps: usize,
    /// Vital reserve available to absorb degraded service.
    pub initial_reserve: f64,
    /// Reserve consumed per step while the degraded model keeps serving unaided.
    pub harm_rate: f64,
    /// Noise standard deviation of the monitored performance signal.
    pub noise_sd: f64,
    /// Deterministic PRNG seed of the monitored stream.
    pub seed: u64,
}

impl DriftMonitorConfig {
    /// Canonical audit configuration (seed `2026`, change point at step 10, drop of `0.35`,
    /// retrain lag of 12 shifts, reserve of 8 units consumed at 0.5 per degraded shift).
    pub const fn canonical() -> Self {
        Self {
            horizon_steps: 60,
            change_point: 10,
            degradation: 0.35,
            cusum_slack: 0.10,
            cusum_threshold: 0.30,
            retrain_lag_steps: 12,
            initial_reserve: 8.0,
            harm_rate: 0.5,
            noise_sd: 0.05,
            seed: 2026,
        }
    }
}

impl Default for DriftMonitorConfig {
    fn default() -> Self {
        Self::canonical()
    }
}

/// **Model drift and fallback audit** — the MLOps layer read on the vital axis.
///
/// A monitored performance signal is followed by a one-sided CUSUM with slack $k$ and threshold $h$.
/// When the world shifts at `change_point`, the monitor fires after a detection latency; the model can
/// then be retrained in `retrain_lag_steps`. The audit compares two institutions: one that keeps the
/// degraded model in production, and one that trips a fallback at detection and pays the retraining
/// latency. The question is not whether drift is detected but whether
/// $\mathrm{TTS}_{\mathrm{survival}} \ge \mathrm{TTR}_{\mathrm{recovery}}$ holds for the *whole*
/// recovery path, detection included.
#[derive(Debug, Clone, PartialEq)]
pub struct ModelDriftFallbackAudit {
    /// Monitoring horizon.
    pub horizon_steps: usize,
    /// Step at which the world shifted.
    pub change_point: usize,
    /// Detection step of the drift (`None` if the monitor never fired).
    pub detection_step: Option<usize>,
    /// First alarm on a stationary stream with no drift (`None` when the monitor stays quiet).
    pub false_alarm_step: Option<usize>,
    /// Steps of reserve available before ruin if the degraded model keeps serving.
    pub tts_steps: usize,
    /// Detection latency plus retraining latency (`None` when nothing was detected).
    pub ttr_steps: Option<usize>,
    /// Whether the institution without fallback survives the horizon.
    pub survival_without_fallback: bool,
    /// Whether the institution with fallback completes recovery before ruin.
    pub survival_with_fallback: bool,
    /// Reserve left at the horizon with a fallback installed.
    pub reserve_with_fallback: f64,
    /// Reserve left at the horizon with no fallback.
    pub reserve_without_fallback: f64,
    /// Recovery step of a slower institution whose retraining takes twice as long.
    pub ttr_slow_retrain_steps: Option<usize>,
    /// Whether the slow-retraining institution survives.
    pub survival_with_slow_retrain: bool,
}

impl ModelDriftFallbackAudit {
    /// Generates the monitored performance stream.
    fn stream(config: &DriftMonitorConfig, drift: bool) -> Vec<f64> {
        let mut rng = SplitMix64::new(config.seed);
        (0..config.horizon_steps)
            .map(|t| {
                let shifted = drift && t >= config.change_point;
                let mean = if shifted {
                    1.0 - config.degradation
                } else {
                    1.0
                };
                mean + rng.normal(0.0, config.noise_sd)
            })
            .collect()
    }

    /// One-sided CUSUM on the downward deviation of the signal from its nominal level of one.
    fn cusum_detection(stream: &[f64], slack: f64, threshold: f64) -> Option<usize> {
        let mut statistic = 0.0_f64;
        for (t, &value) in stream.iter().enumerate() {
            statistic = (statistic + (1.0 - value) - slack).max(0.0);
            if statistic > threshold {
                return Some(t);
            }
        }
        None
    }

    /// Evaluates the audit.
    pub fn evaluate(config: DriftMonitorConfig) -> Self {
        let drifted = Self::stream(&config, true);
        let stationary = Self::stream(&config, false);
        let detection_step =
            Self::cusum_detection(&drifted, config.cusum_slack, config.cusum_threshold);
        let false_alarm_step =
            Self::cusum_detection(&stationary, config.cusum_slack, config.cusum_threshold);

        let burn_steps = (config.initial_reserve / config.harm_rate.max(1e-9)).ceil() as usize;
        let tts_steps = config.change_point + burn_steps;

        let ending_reserve = |stop: Option<usize>| -> f64 {
            let degraded = match stop {
                Some(step) => step.saturating_sub(config.change_point),
                None => config.horizon_steps.saturating_sub(config.change_point),
            };
            (config.initial_reserve - config.harm_rate * degraded as f64).max(0.0)
        };
        let recovery = detection_step.map(|d| d + config.retrain_lag_steps);
        let survival_with_fallback = recovery.is_some_and(|r| r <= tts_steps);
        let slow_recovery = detection_step.map(|d| d + 2 * config.retrain_lag_steps);

        Self {
            horizon_steps: config.horizon_steps,
            change_point: config.change_point,
            detection_step,
            false_alarm_step,
            tts_steps,
            ttr_steps: recovery,
            survival_without_fallback: tts_steps > config.horizon_steps,
            survival_with_fallback,
            reserve_with_fallback: ending_reserve(detection_step),
            reserve_without_fallback: ending_reserve(None),
            ttr_slow_retrain_steps: slow_recovery,
            survival_with_slow_retrain: slow_recovery.is_some_and(|r| r <= tts_steps),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_operator_ii_unifies_jensen_hill_kingman_kelly_and_tts_ttr() {
        let mut rng = crate::kernel::SplitMix64::with_default_seed();
        let samples: Vec<f64> = (0..2000).map(|_| rng.pareto(1.0, 1.45)).collect();
        let tail_state =
            VitalMarginState::from_heavy_tails_and_asymmetry(&samples, 150, (1.2, 2.0), |x| x * x);
        assert_eq!(
            tail_state.tail_regime,
            TailMomentRegime::FiniteMeanInfiniteVariance
        );
        assert!(
            tail_state.hill_alpha_k_range[0] >= 1.1 && tail_state.hill_alpha_k_range[1] <= 1.85
        );
        assert!(tail_state.jensen_gap > 1.0);
        assert!(tail_state.mean_ranking_reversed);
        assert!(tail_state.ordinal_invariance_error < 1e-12);
        assert!(tail_state.vital_margin_score > 0.5 && tail_state.basin_radius_ratio == 1.0);

        let kelly = VitalMarginState::from_kelly_parameters(0.10, 0.20, 4.0, 0.02);
        assert!(kelly.epistemic_kelly_fraction < kelly.plugin_kelly_fraction);
        assert!((kelly.plugin_kelly_fraction - 0.40).abs() < 1e-12);
        assert!(kelly.vital_margin_score > 0.5 && kelly.basin_radius_ratio == 1.0);
        assert_eq!(
            kelly.kelly_regime,
            KellySizingRegime::QuadraticGaussianKelly
        );

        let tail_kelly =
            VitalMarginState::from_regime_aware_kelly(0.10, 0.08, 1.65, 2.50, 50.0, 0.02);
        assert_eq!(
            tail_kelly.kelly_regime,
            KellySizingRegime::TailRobustWdroCvarKelly
        );
        assert!(tail_kelly.epistemic_kelly_fraction < tail_kelly.plugin_kelly_fraction);

        // V-01 & V-02: Even if caller passes wdro_cvar95_scale_sq < sample_sigma2, WDRO-CVaR Kelly never leverages more than plug-in
        let tail_kelly_floor =
            VitalMarginState::from_regime_aware_kelly(0.10, 29.12, 1.65, 2.50, 50.0, 0.02);
        assert!(tail_kelly_floor.epistemic_kelly_fraction < tail_kelly_floor.plugin_kelly_fraction);

        // V-01: for_shift_sizing reaches both QuadraticGaussianKelly (light tail) and TailRobustWdroCvarKelly (heavy Pareto tail)
        let light_obs: Vec<f64> = (0..40).map(|i| 20.0 + (i as f64) * 0.02).collect();
        let heavy_obs: Vec<f64> = (0..40)
            .map(|i| 10.0 * (((40 - i) as f64) / 40.0).powf(-1.0 / 1.4))
            .collect();
        let light_sizing = VitalMarginState::for_shift_sizing(&light_obs, 22.0, 45.0, 20.0, 0.1);
        let heavy_sizing = VitalMarginState::for_shift_sizing(&heavy_obs, 50.0, 45.0, 20.0, 0.1);
        assert_eq!(
            light_sizing.kelly_regime,
            KellySizingRegime::QuadraticGaussianKelly
        );
        assert_eq!(
            heavy_sizing.kelly_regime,
            KellySizingRegime::TailRobustWdroCvarKelly
        );

        let h1 = HollingTailRuptureAudit::evaluate(120, 1.0, 1.65, 12.0, 4000);
        let h10 = HollingTailRuptureAudit::evaluate(120, 10.1, 1.65, 12.0, 4000);
        let h20 = HollingTailRuptureAudit::evaluate(120, 20.0, 1.65, 12.0, 4000);
        assert!(
            h1.jensen_relative_bias > 0.0 && h1.jensen_relative_bias < h10.jensen_relative_bias
        );
        assert!(h10.jensen_relative_bias < h20.jensen_relative_bias);
        assert!(h10.ci90_dispersion_ratio > 20.0 * h1.ci90_dispersion_ratio);
        assert!(
            (h10.debiased_rupture_prob - h10.true_rupture_prob).abs()
                < (h10.expected_plugin_rupture_prob - h10.true_rupture_prob).abs()
        );

        let safe = VitalMarginState::from_queue_basin_and_derived_ttr(
            0.78, 1.25, 2.0, 12.0, 2.0, 1.0, 1.0, 1.80, 10.0, 2.5,
        );
        let lean = VitalMarginState::from_queue_basin_and_derived_ttr(
            0.97, 1.25, 2.0, 2.0, 2.0, 1.0, 1.0, 2.45, 10.0, 2.5,
        );
        assert!(safe.survives_disruption());
        assert!(!lean.survives_disruption());
        assert!(lean.kingman_expected_wait > 8.0 * safe.kingman_expected_wait);
        assert!(safe.basin_radius_worn < safe.basin_radius_fresh);
        assert!(safe.critical_slowing_down_rho1_worn > safe.critical_slowing_down_rho1);
        assert!(lean.critical_slowing_down_rho1 > safe.critical_slowing_down_rho1);

        // Verify M4: post-bifurcation harvesting h = 4.0 collapses the basin (radius = 0, rho1 = 1.0)
        let (r_collapsed, rho1_collapsed) =
            VitalMarginState::solve_canguilhem_holling_basin(4.0, 10.0);
        assert_eq!(r_collapsed, 0.0);
        assert!((rho1_collapsed - 1.0).abs() < 1e-12);

        // Verify §7 Criterion 3 & 5: vital_margin_score is sensitive to Jensen convexity & CRPS
        let linear_state =
            VitalMarginState::from_heavy_tails_and_asymmetry(&samples, 150, (1.2, 2.0), |x| x);
        let concave_state =
            VitalMarginState::from_heavy_tails_and_asymmetry(&samples, 150, (1.2, 2.0), |x| -x * x);
        assert!(tail_state.vital_margin_score < linear_state.vital_margin_score);
        assert_eq!(concave_state.vital_margin_score, 0.0);
        assert!(safe.wald_survivorship_inversion);
        let wald_latent = WaldSurvivorshipAudit::wald_1943_bomber_benchmark();
        assert!((wald_latent.reconstructed_missing_hits_a - 18.0).abs() < 1e-9);
        assert!((wald_latent.reconstructed_missing_hits_b - 210.0).abs() < 1e-9);
        assert!((wald_latent.lethality_zone_a - 0.072).abs() < 1e-9);
        assert!((wald_latent.lethality_zone_b - 0.840).abs() < 1e-9);
        let (full_ate, surv_ate, bias) = WaldSurvivorshipAudit::evaluate_empirical_truncation_bias(
            &[0.0, 100.0, 200.0],
            &[0.0, 0.0, 180.0],
        );
        assert!((full_ate - 40.0).abs() < 1e-9 && (surv_ate - (-30.0)).abs() < 1e-9 && bias < 0.0);
        assert!(safe.sniedovich_no_mans_land_mirage);
        let no_mirage = SniedovichInfoGapAudit::evaluate(100.0, 10.0, 60.0, -10.0, -50.0);
        assert!(!no_mirage.no_mans_land_mirage);

        // Verify G5: Exact positive ratio scale invariance X -> c*X for c in {0.001, 1000.0}
        let samples_milli: Vec<f64> = samples.iter().map(|&x| 0.001 * x).collect();
        let samples_kilo: Vec<f64> = samples.iter().map(|&x| 1000.0 * x).collect();
        let st_milli = VitalMarginState::from_heavy_tails_and_asymmetry(
            &samples_milli,
            150,
            (0.0012, 0.0020),
            |x| x * x,
        );
        let st_kilo = VitalMarginState::from_heavy_tails_and_asymmetry(
            &samples_kilo,
            150,
            (1200.0, 2000.0),
            |x| x * x,
        );
        assert!(
            (st_milli.vital_margin_score - st_kilo.vital_margin_score).abs() < 1e-9,
            "vital_margin_score must be invariant under positive ratio scaling: {} vs {}",
            st_milli.vital_margin_score,
            st_kilo.vital_margin_score
        );
    }
}
