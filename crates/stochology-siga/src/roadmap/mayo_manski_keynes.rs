//! The Mayo–Manski Formal Bridge (`ImbensManskiConfidenceInterval`, `MayoManskiSevereBoundaryAudit`),
//! Manski's Minimax Regret under Partial Identification, and Keynes's Evidential Weight &
//! Qualitative/Clinical Process Tracing (`KeynesEvidentialWeightAudit`, `ProcessTracingTestKind`).
//!
//! Implements the theoretical synthesis of `notes/A Ética da Mira.md` (§3.2 Tension 2, §4.2 Tension 4,
//! and §7.1 Family 5):
//! 1. **Imbens & Manski (2004) Confidence Intervals for Partially Identified Parameters:**
//!    Separates *identification ambiguity* $\Delta_{\mathrm{id}} = \theta_U - \theta_L$ (which never vanishes
//!    even as $n_{\mathrm{eff}} \to \infty$) from *sampling uncertainty* $O(n_{\mathrm{eff}}^{-1/2})$.
//! 2. **The Mayo–Manski Bridge & Minimax Regret:**
//!    Applies Deborah Mayo's (2018) severe-testing requirement to the conservative lower boundary of
//!    Charles Manski's (2003, 2013) / David Lee's (2009) identification interval and evaluates
//!    Manski's Minimax Regret decision rule across $[p_L, p_U]$.
//! 3. **Keynes's Weight of Argument (1921) & Process Tracing (*Sēmeîa*):**
//!    Distinguishes probability $p$ from evidential weight $V \in [0, 1]$ and classifies qualitative,
//!    historiographical, legal, and clinical diagnostic clues (*Straw-in-the-Wind*, *Hoop Test*,
//!    *Smoking Gun*, *Doubly Decisive*) via additive Turing-Good deciban weights.

/// Standard normal CDF $\Phi(z)$ via Abramowitz & Stegun (1964, 7.1.26) rational approximation
/// (maximum absolute error $\le 1.5 \times 10^{-7}$).
fn standard_normal_cdf(z: f64) -> f64 {
    if !z.is_finite() {
        return if z.is_sign_positive() { 1.0 } else { 0.0 };
    }
    let x = z.abs() / std::f64::consts::SQRT_2;
    let t = 1.0 / (1.0 + 0.3275911 * x);
    let poly = t
        * (0.254_829_592
            + t * (-0.284_496_736
                + t * (1.421_413_741 + t * (-1.453_152_027 + t * 1.061_405_429))));
    let erf_approx = 1.0 - poly * (-x * x).exp();
    if z >= 0.0 {
        0.5 * (1.0 + erf_approx)
    } else {
        0.5 * (1.0 - erf_approx)
    }
}

/// Imbens & Manski (2004, *Econometrica* 72(6): 1845–1857) confidence interval for a partially
/// identified parameter $\theta \in [\theta_L, \theta_U]$:
/// $$\Phi\left(c_n + \frac{\sqrt{n_{\mathrm{eff}}}\,\Delta_{\mathrm{id}}}{\max(\hat{\sigma}_L, \hat{\sigma}_U)}\right) - \Phi(-c_n) = 1 - \alpha$$
/// $$\mathrm{CI}_{\mathrm{IM}}(1-\alpha) = \left[\hat{\theta}_L - c_n \frac{\hat{\sigma}_L}{\sqrt{n_{\mathrm{eff}}}},\; \hat{\theta}_U + c_n \frac{\hat{\sigma}_U}{\sqrt{n_{\mathrm{eff}}}}\right]$$
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ImbensManskiConfidenceInterval {
    /// Estimated lower identification bound $\hat{\theta}_L$.
    pub lower_id_bound: f64,
    /// Estimated upper identification bound $\hat{\theta}_U$.
    pub upper_id_bound: f64,
    /// Irreducible identification ambiguity width $\Delta_{\mathrm{id}} = \hat{\theta}_U - \hat{\theta}_L \ge 0$.
    pub identification_ambiguity_width: f64,
    /// Effective independent sample size $n_{\mathrm{eff}}$ (after Moulton-Campbell / Meng discounting).
    pub effective_n: f64,
    /// Calibrated Imbens-Manski critical value $c_n \in [z_{1-\alpha}, z_{1-\alpha/2}]$.
    pub critical_value_c_n: f64,
    /// Lower confidence limit $\hat{\theta}_L - c_n \hat{\sigma}_L / \sqrt{n_{\mathrm{eff}}}$.
    pub ci_lower: f64,
    /// Upper confidence limit $\hat{\theta}_U + c_n \hat{\sigma}_U / \sqrt{n_{\mathrm{eff}}}$.
    pub ci_upper: f64,
    /// Reducible sampling uncertainty margin $(\mathrm{ci\_upper} - \mathrm{ci\_lower}) - \Delta_{\mathrm{id}}$.
    pub sampling_uncertainty_margin: f64,
    /// Share of total interval width due to irreducible identification ambiguity $\Delta_{\mathrm{id}} / (\mathrm{ci\_upper} - \mathrm{ci\_lower})$.
    pub identification_share_of_total_width: f64,
    /// Residual of the Imbens-Manski coverage equation $|\Phi(c_n + \tau_n) - \Phi(-c_n) - (1 - \alpha)|$.
    pub coverage_equation_residual: f64,
}

impl ImbensManskiConfidenceInterval {
    /// Computes the Imbens-Manski $(1 - \alpha)$ confidence interval from identification bounds
    /// `lower_id_bound <= upper_id_bound`, endpoint standard deviations `(sigma_l, sigma_u)`,
    /// effective sample size `effective_n > 0`, and significance level `alpha` (e.g. `0.05`).
    pub fn compute(
        lower_id_bound: f64,
        upper_id_bound: f64,
        sigma_l: f64,
        sigma_u: f64,
        effective_n: f64,
        alpha: f64,
    ) -> Self {
        assert!(
            lower_id_bound.is_finite()
                && upper_id_bound.is_finite()
                && lower_id_bound <= upper_id_bound
                && sigma_l >= 0.0
                && sigma_u >= 0.0
                && effective_n > 0.0
                && (0.001..0.50).contains(&alpha),
            "ImbensManskiConfidenceInterval requires finite ordered bounds, non-negative sigmas, n_eff > 0, and alpha in (0.001, 0.50)"
        );
        let delta_id = (upper_id_bound - lower_id_bound).max(0.0);
        let max_sigma = sigma_l.max(sigma_u).max(1e-12);
        let tau_n = (effective_n.sqrt() * delta_id) / max_sigma;
        let target_coverage = 1.0 - alpha;

        // Solve Phi(c + tau_n) - Phi(-c) = 1 - alpha on c in [0.0, 8.0] via 64-step bisection
        let mut lo = 0.0_f64;
        let mut hi = 8.0_f64;
        for _ in 0..64 {
            let mid = 0.5 * (lo + hi);
            let cov = standard_normal_cdf(mid + tau_n) - standard_normal_cdf(-mid);
            if cov < target_coverage {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        let c_n = 0.5 * (lo + hi);
        let achieved = standard_normal_cdf(c_n + tau_n) - standard_normal_cdf(-c_n);
        let coverage_equation_residual = (achieved - target_coverage).abs();

        let root_n = effective_n.sqrt();
        let margin_l = c_n * (sigma_l / root_n);
        let margin_u = c_n * (sigma_u / root_n);
        let ci_lower = lower_id_bound - margin_l;
        let ci_upper = upper_id_bound + margin_u;
        let sampling_uncertainty_margin = margin_l + margin_u;
        let total_width = (ci_upper - ci_lower).max(1e-15);
        let identification_share_of_total_width = (delta_id / total_width).clamp(0.0, 1.0);

        Self {
            lower_id_bound,
            upper_id_bound,
            identification_ambiguity_width: delta_id,
            effective_n,
            critical_value_c_n: c_n,
            ci_lower,
            ci_upper,
            sampling_uncertainty_margin,
            identification_share_of_total_width,
            coverage_equation_residual,
        }
    }
}

/// Bridge $L_1 \to L_2 \to L_3$ Audit uniting Deborah Mayo's (2018) **Severe Testing** with
/// Charles Manski's (2003, 2013) **Partial Identification & Minimax Regret** and David Lee's (2009)
/// **Trimming Bounds** (`notes/A Ética da Mira.md` §3.2 and §4.2).
#[derive(Debug, Clone, PartialEq)]
pub struct MayoManskiSevereBoundaryAudit {
    /// Naive point estimate $\hat{\theta}_{\mathrm{naive}}$ ignoring non-ignorable missingness (`MNAR`).
    pub naive_point_estimate: f64,
    /// Imbens-Manski $(1 - \alpha)$ confidence interval around the partial identification interval $[\theta_L, \theta_U]$.
    pub imbens_manski_ci: ImbensManskiConfidenceInterval,
    /// Critical decision threshold $\theta^* = p^* = C / (\eta L)$ (or breakeven treatment effect).
    pub decision_threshold_star: f64,
    /// Maximum regret of choosing the protective intervention ($a = 1$) across $\theta \in [\theta_L, \theta_U]$.
    pub minimax_regret_protect: f64,
    /// Maximum regret of withholding protection ($a = 0$) across $\theta \in [\theta_L, \theta_U]$.
    pub minimax_regret_withhold: f64,
    /// Whether Manski's Minimax Regret rule selects the protective action ($\mathcal{R}_{\max}(1) \le \mathcal{R}_{\max}(0)$).
    pub minimax_regret_recommends_protect: bool,
    /// Exact identity error between $\mathcal{R}_{\max}(0) - \mathcal{R}_{\max}(1)$ and $2\eta L(\frac{\theta_L+\theta_U}{2} - \theta^*)$ when $\theta^* \in [\theta_L, \theta_U]$.
    pub minimax_regret_midpoint_identity_error: f64,
    /// Whether Mayo's severe test passes at the conservative lower Imbens-Manski boundary ($\mathrm{ci\_lower} \ge \theta^*$).
    pub mayo_severe_at_lower_bound: bool,
    /// Whether Wald's Maximin floor triggers protection at the upper Imbens-Manski boundary ($\mathrm{ci\_upper} \ge \theta^*$).
    pub wald_maximin_at_upper_bound: bool,
    /// True if a naive point test falsely claims point-level certainty while the identification interval straddles or approaches the boundary.
    pub exposes_naive_point_pseudo_severity: bool,
    /// Overall verification flag for the Mayo–Manski bridge.
    pub mayo_manski_bridge_verified: bool,
}

impl MayoManskiSevereBoundaryAudit {
    /// Evaluates the Mayo–Manski severe boundary test and Minimax Regret rule for a partially identified
    /// risk or effect parameter $\theta \in [\theta_L, \theta_U]$ against protection cost $C$ and avoided loss $\eta L$.
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate(
        naive_point_estimate: f64,
        lower_id_bound: f64,
        upper_id_bound: f64,
        sigma_l: f64,
        sigma_u: f64,
        effective_n: f64,
        alpha: f64,
        protection_cost_c: f64,
        mitigated_loss_eta_l: f64,
    ) -> Self {
        assert!(
            protection_cost_c > 0.0 && mitigated_loss_eta_l > protection_cost_c,
            "requires 0 < C < eta * L"
        );
        let decision_threshold_star = protection_cost_c / mitigated_loss_eta_l;
        let imbens_manski_ci = ImbensManskiConfidenceInterval::compute(
            lower_id_bound,
            upper_id_bound,
            sigma_l,
            sigma_u,
            effective_n,
            alpha,
        );

        let regret_protect = (protection_cost_c - lower_id_bound * mitigated_loss_eta_l).max(0.0);
        let regret_withhold = (upper_id_bound * mitigated_loss_eta_l - protection_cost_c).max(0.0);
        let minimax_regret_recommends_protect = regret_protect <= regret_withhold;

        // When theta* lies inside [theta_L, theta_U], R_max(0) - R_max(1) == 2 * eta * L * (midpoint - theta*)
        let midpoint = 0.5 * (lower_id_bound + upper_id_bound);
        let minimax_regret_midpoint_identity_error =
            if (lower_id_bound..=upper_id_bound).contains(&decision_threshold_star) {
                let lhs = regret_withhold - regret_protect;
                let rhs = 2.0 * mitigated_loss_eta_l * (midpoint - decision_threshold_star);
                (lhs - rhs).abs()
            } else {
                0.0
            };

        let mayo_severe_at_lower_bound = imbens_manski_ci.ci_lower >= decision_threshold_star;
        let wald_maximin_at_upper_bound = imbens_manski_ci.ci_upper >= decision_threshold_star;
        let naive_se = sigma_l.max(sigma_u) / effective_n.sqrt();
        let naive_lower = naive_point_estimate - 1.959_964 * naive_se;
        let exposes_naive_point_pseudo_severity =
            naive_lower > imbens_manski_ci.ci_lower + 0.25 * (upper_id_bound - lower_id_bound);

        let mayo_manski_bridge_verified = imbens_manski_ci.coverage_equation_residual < 1e-6
            && minimax_regret_midpoint_identity_error < 1e-12
            && wald_maximin_at_upper_bound
            && exposes_naive_point_pseudo_severity;

        Self {
            naive_point_estimate,
            imbens_manski_ci,
            decision_threshold_star,
            minimax_regret_protect: regret_protect,
            minimax_regret_withhold: regret_withhold,
            minimax_regret_recommends_protect,
            minimax_regret_midpoint_identity_error,
            mayo_severe_at_lower_bound,
            wald_maximin_at_upper_bound,
            exposes_naive_point_pseudo_severity,
            mayo_manski_bridge_verified,
        }
    }

    /// Assumed: canonical Mayo–Manski benchmark anchored to Lee (2009) / Manski partial identification
    /// where the lower identification bound clears the cost-loss threshold ($p_L = 0.145 > p^* = 0.120$)
    /// even after Imbens-Manski sampling adjustment ($n_{\mathrm{eff}} = 1583$).
    pub fn canonical_benchmark() -> Self {
        Self::evaluate(0.215, 0.145, 0.285, 0.35, 0.42, 1583.0, 0.05, 12.0, 100.0)
    }
}

/// Qualitative, historiographical, legal, and clinical **Process Tracing** test classification
/// (Van Evera 1997; Collier 2011; Galenic *sēmeîa* in `notes/A Ética da Mira.md` §1.0 and §7.1 item 5).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessTracingTestKind {
    /// **Straw-in-the-Wind:** Weakly informative clue ($W^+ < 6\text{ dB}$ and $|W^-| < 6\text{ dB}$);
    /// neither necessary nor sufficient.
    StrawInTheWind,
    /// **Hoop Test (Necessary Condition):** High negative weight ($|W^-| \ge 10\text{ dB}$, $W^+ < 10\text{ dB}$);
    /// surviving the hoop modestly strengthens the hypothesis, but failing it refutes it severely.
    HoopTest,
    /// **Smoking Gun (Sufficient / Pathognomonic Sign):** High positive weight ($W^+ \ge 10\text{ dB}$, $|W^-| < 10\text{ dB}$);
    /// absence does not refute, but a single authentic document or pathognomonic clinical sign confirms strongly.
    SmokingGun,
    /// **Doubly Decisive:** Both necessary and sufficient ($W^+ \ge 10\text{ dB}$ and $|W^-| \ge 10\text{ dB}$).
    DoublyDecisive,
}

impl ProcessTracingTestKind {
    /// Classifies a diagnostic clue or documentary test from its sensitivity (`tpr`) and false-positive rate (`fpr`).
    pub fn classify(tpr: f64, fpr: f64) -> (Self, f64, f64) {
        let p_sens = tpr.clamp(1e-6, 1.0 - 1e-6);
        let p_fp = fpr.clamp(1e-6, 1.0 - 1e-6);
        let w_pos_db = 10.0 * (p_sens / p_fp).log10();
        let w_neg_db = 10.0 * ((1.0 - p_sens) / (1.0 - p_fp)).log10();
        let kind = match (w_pos_db >= 10.0, w_neg_db.abs() >= 10.0) {
            (true, true) => Self::DoublyDecisive,
            (true, false) => Self::SmokingGun,
            (false, true) => Self::HoopTest,
            (false, false) => Self::StrawInTheWind,
        };
        (kind, w_pos_db, w_neg_db)
    }
}

/// Keynes's (1921, *A Treatise on Probability*, Ch. 6) **Weight of Argument ($V$)** and
/// 5th Methodological Family (*Process Tracing* & Clinical/Historiographical *Sēmeîa*) Audit
/// (`notes/A Ética da Mira.md` §4.2 and §7.1 item 5).
#[derive(Debug, Clone, PartialEq)]
pub struct KeynesEvidentialWeightAudit {
    /// Identical point probability $p_1 = p_2 = p$ shared by the thin and dense arguments.
    pub shared_probability_p: f64,
    /// Evidential weight $V_{\text{thin}} \in [0, 1]$ of the thin or heavily censored archive.
    pub thin_archive_weight_v: f64,
    /// Evidential weight $V_{\text{dense}} \in [0, 1]$ of the well-identified, high-$n_{\mathrm{eff}}$ archive.
    pub dense_archive_weight_v: f64,
    /// Classification and deciban weights $(W^+, W^-)$ of a necessary **Hoop Test** clue.
    pub hoop_test: (ProcessTracingTestKind, f64, f64),
    /// Classification and deciban weights $(W^+, W^-)$ of a sufficient **Smoking Gun** (*sēmeîon*) clue.
    pub smoking_gun_test: (ProcessTracingTestKind, f64, f64),
    /// Combined posterior probability when starting from prior $p_0$ and observing both the Hoop and Smoking Gun clues.
    pub combined_process_tracing_posterior: f64,
    /// Exact Turing-Good log-odds identity residual $|\log_{10} O_{\text{post}} - (\log_{10} O_{\text{prior}} + (W^+_{\text{hoop}} + W^+_{\text{gun}})/10)|$.
    pub deciban_log_odds_identity_error: f64,
    /// Whether Keynes's weight separates equal-probability cases ($V_{\text{dense}} > V_{\text{thin}} + 0.55$)
    /// and qualitative process tracing achieves severe diagnostic resolution.
    pub keynes_and_process_tracing_verified: bool,
}

impl KeynesEvidentialWeightAudit {
    /// Computes Keynes's evidential weight index:
    /// $$V(n_{\mathrm{eff}}, \Delta_{\mathrm{id}}, n_0) = \left(\frac{n_{\mathrm{eff}}}{n_{\mathrm{eff}} + n_0}\right)\bigl(1 - \Delta_{\mathrm{id}}\bigr) \in [0, 1]$$
    pub fn evidential_weight(
        effective_n: f64,
        normalized_ambiguity_width: f64,
        half_saturation_n0: f64,
    ) -> f64 {
        assert!(
            effective_n >= 0.0 && half_saturation_n0 > 0.0,
            "requires effective_n >= 0 and n0 > 0"
        );
        let completeness = 1.0 - normalized_ambiguity_width.clamp(0.0, 1.0);
        (effective_n / (effective_n + half_saturation_n0)) * completeness
    }

    /// Evaluates Keynes's weight of argument across a thin vs. dense archive at equal probability $p$
    /// alongside a two-step Process Tracing sequence (*Hoop Test* + *Smoking Gun*).
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate(
        shared_probability_p: f64,
        thin_n_eff: f64,
        thin_ambiguity: f64,
        dense_n_eff: f64,
        dense_ambiguity: f64,
        half_saturation_n0: f64,
        hoop_tpr_fpr: (f64, f64),
        smoking_gun_tpr_fpr: (f64, f64),
    ) -> Self {
        let p = shared_probability_p.clamp(1e-6, 1.0 - 1e-6);
        let thin_archive_weight_v =
            Self::evidential_weight(thin_n_eff, thin_ambiguity, half_saturation_n0);
        let dense_archive_weight_v =
            Self::evidential_weight(dense_n_eff, dense_ambiguity, half_saturation_n0);

        let hoop_test = ProcessTracingTestKind::classify(hoop_tpr_fpr.0, hoop_tpr_fpr.1);
        let smoking_gun_test =
            ProcessTracingTestKind::classify(smoking_gun_tpr_fpr.0, smoking_gun_tpr_fpr.1);

        let prior_odds = p / (1.0 - p);
        let total_pos_db = hoop_test.1 + smoking_gun_test.1;
        let post_odds = prior_odds * 10.0_f64.powf(total_pos_db / 10.0);
        let combined_process_tracing_posterior = post_odds / (1.0 + post_odds);

        let direct_lr_odds = prior_odds
            * (hoop_tpr_fpr.0 / hoop_tpr_fpr.1)
            * (smoking_gun_tpr_fpr.0 / smoking_gun_tpr_fpr.1);
        let deciban_log_odds_identity_error = (post_odds.log10() - direct_lr_odds.log10()).abs();

        let verified = dense_archive_weight_v > thin_archive_weight_v + 0.55
            && hoop_test.0 == ProcessTracingTestKind::HoopTest
            && smoking_gun_test.0 == ProcessTracingTestKind::SmokingGun
            && combined_process_tracing_posterior > 0.90
            && deciban_log_odds_identity_error < 1e-12;

        Self {
            shared_probability_p: p,
            thin_archive_weight_v,
            dense_archive_weight_v,
            hoop_test,
            smoking_gun_test,
            combined_process_tracing_posterior,
            deciban_log_odds_identity_error,
            keynes_and_process_tracing_verified: verified,
        }
    }

    /// Assumed: canonical Keynes & Process Tracing benchmark contrasting a 2-line summary note
    /// ($p = 0.50, n_{\mathrm{eff}} = 4, \Delta_{\mathrm{id}} = 0.72 \implies V = 0.021$) against a thoroughly
    /// audited clinical/archival investigation ($p = 0.50, n_{\mathrm{eff}} = 640, \Delta_{\mathrm{id}} = 0.08 \implies V = 0.859$),
    /// coupled with a Hoop Test ($\mathrm{TPR}=0.97, \mathrm{FPR}=0.35, W^- = -10.66\text{ dB}$) and a
    /// pathognomonic SmokingGun ($\mathrm{TPR}=0.68, \mathrm{FPR}=0.02, W^+ = +15.31\text{ dB}$).
    pub fn canonical_benchmark() -> Self {
        Self::evaluate(
            0.50,
            4.0,
            0.72,
            640.0,
            0.08,
            50.0,
            (0.97, 0.35),
            (0.68, 0.02),
        )
    }
}

/// Unified audit of **Filter I & Filter II (Constitutive Tensions 1 to 4)** across all fields of knowledge
/// (`notes/A Ética da Mira.md` §3.1–4.2):
/// 1. **Tension 1 (Situational Interest vs. Intellectual Rectitude):** Cassam (2019) & Proctor (2008) agnotological
///    underpowering detection, Frankfurt (2005) formal *bullshit* ($\mathrm{EVPI}=0, \mathrm{RES}_{\mathrm{CORP}} \approx 0$),
///    Steegen et al. (2016) multiverse specification curve stability ($\bar{E}_{\mathcal{S}} = \frac{1}{K}\sum_{s=1}^K E_s$),
///    Kimball (1957) / Mitroff (1974) Type III framing error, and Longino (1990) / Harding (1991) strong-objectivity
///    cost-loss framing gap $|p_{\mathrm{inst}}^* - p_{\mathrm{affected}}^*|$;
/// 2. **Tension 2 (Accommodative Confirmation vs. Fruitful Refutation):** Lakatos (1970) progressive vs. degenerative
///    program shift ($\Delta\mathrm{RES}_{\mathrm{CORP}}^{\mathrm{OOS}}$ and $E_t / (K+1)$) and anytime-valid partial
///    identification ($\Delta_{\mathrm{id}} + 2 r_t(\alpha) \ge \Delta_{\mathrm{id}} > 0$);
/// 3. **Tension 3 (Domesticated vs. Reflexive Formalization):** Georgescu-Roegen (1971) dialectical penumbra binary-cut
///    instability and Stevens (1946) / Krantz et al. (1971) ordinal scale invariance (arithmetic mean sign reversal under
///    monotone $\phi(x) = x^2$ vs. exact invariance of Mann-Whitney dominance $\mathbb{P}(A < B)$);
/// 4. **Tension 4 (Sample Volume vs. Evidential Weight & Archive Silence):** Joint Meng-Moulton compound effective sample
///    size $n_{\mathrm{eff,joint}}(p)$, Horvitz-Thompson / Kish positivity boundary ($\hat{\pi}(s) < \varepsilon_{\mathrm{pos}}$
///    routed to Manski bounds), Chao (1987) / Lincoln-Petersen capture-recapture hidden population $\hat{n}_{00}$, and
///    Peterson (1976) / Horowitz-Manski informative censoring survival bounds $[S_L(t), S_U(t)]$.
#[derive(Debug, Clone, PartialEq)]
pub struct FilterOneAndTwoTensionsAudit {
    /// Tension 1: statistical power $(1 - \beta)$ at minimum relevant harm $\delta_{\min}$ for `(agnotological_study, severe_study)`.
    pub tension1_agnotological_vs_severe_power: (f64, f64),
    /// Tension 1: multiverse directional consistency $\pi_{\mathrm{multiverse}}$ and average e-value $\bar{E}_{\mathcal{S}}$.
    pub tension1_multiverse_consistency_and_mean_e: (f64, f64),
    /// Tension 1: Kimball/Mitroff Type III narrow focal utility vs. global utility including collateral externalities.
    pub tension1_type_three_focal_vs_global_utility: (f64, f64),
    /// Tension 1: Longino/Harding cost-loss threshold gap $|p_{\mathrm{inst}}^* - p_{\mathrm{affected}}^*|$.
    pub tension1_strong_objectivity_threshold_gap: f64,
    /// Tension 2: Lakatos multiplicity-penalized out-of-sample evidence for `(degenerative_shift, progressive_shift)`.
    pub tension2_lakatos_degenerative_vs_progressive_e: (f64, f64),
    /// Tension 2: anytime-valid partial identification total width at `(early_t, late_t)` vs. irreducible $\Delta_{\mathrm{id}}$.
    pub tension2_anytime_partial_id_widths_and_floor: (f64, f64, f64),
    /// Tension 3: Georgescu-Roegen binary classification jump across penumbra cut $\tau_0 \pm \varepsilon$.
    pub tension3_penumbra_binary_jump: f64,
    /// Tension 3: ordinal arithmetic mean difference $\bar{A} - \bar{B}$ before and after monotone $\phi(x) = x^2$ (reverses sign!).
    pub tension3_ordinal_mean_diff_raw_vs_squared: (f64, f64),
    /// Tension 3: exact invariance residual of Mann-Whitney ordinal dominance $|\mathbb{P}(A<B) - \mathbb{P}(\phi(A)<\phi(B))|$.
    pub tension3_mann_whitney_invariance_error: f64,
    /// Tension 4: `(raw_n, moulton_n_eff, meng_moulton_joint_n_eff, keynes_weight_v_joint)`.
    pub tension4_joint_meng_moulton_and_keynes_v: (f64, f64, f64, f64),
    /// Tension 4: Kish weighted $n_{\mathrm{eff,Kish}}$ under near-zero positivity vs. Manski stratified interval `(lower, upper)`.
    pub tension4_kish_n_eff_and_positivity_manski_bounds: (f64, f64, f64),
    /// Tension 4: Lincoln-Petersen and Chao (1987) capture-recapture total population estimates `(n_obs, n_hat_lp, n_hat_chao)`.
    pub tension4_capture_recapture_population_bounds: (f64, f64, f64),
    /// Tension 4: Peterson (1976) / Horowitz-Manski informative censoring survival bounds `(s_lower, s_km_naive, s_upper)` and identity error.
    pub tension4_peterson_survival_bounds_and_id_error: (f64, f64, f64, f64),
    /// True if all algebraic identities and epistemological checks across Tensions 1 to 4 hold.
    pub all_tensions_1_to_4_verified: bool,
}

impl FilterOneAndTwoTensionsAudit {
    /// Evaluates the canonical multi-disciplinary audit across Constitutive Tensions 1, 2, 3, and 4.
    pub fn canonical_benchmark() -> Self {
        // --- Tension 1: Agnotology vs Severe Power, Multiverse, Type III Framing, Strong Objectivity ---
        // Minimum socially/clinically relevant harm delta_min = 0.35 SD; alpha = 0.05 (z_0.975 = 1.95996)
        let delta_min = 0.35_f64;
        let z_crit = 1.959_963_984_540_054_f64;
        let power_at = |n_eff: f64| -> f64 {
            let signal_z = delta_min * n_eff.sqrt();
            standard_normal_cdf(signal_z - z_crit) + standard_normal_cdf(-signal_z - z_crit)
        };
        let power_agnot = power_at(12.0); // Deliberately underpowered study (power ~ 0.23)
        let power_severe = power_at(400.0); // Severely powered study (power > 0.99)

        // Multiverse across K = 6 secondary specifications ("clay" window/preprocessing choices)
        let multiverse_effects = [0.24_f64, 0.28, 0.22, 0.31, 0.26, 0.25];
        let multiverse_e_values = [28.4_f64, 34.2, 21.6, 42.0, 31.0, 26.8];
        let k_specs = multiverse_effects.len() as f64;
        let pos_count = multiverse_effects.iter().filter(|&&v| v > 0.0).count() as f64;
        let pi_multiverse = pos_count / k_specs;
        let mean_e_multiverse = multiverse_e_values.iter().sum::<f64>() / k_specs;

        // Kimball (1957) / Mitroff (1974) Type III Framing Error: narrow focal gain +45.0 vs collateral harm 72.0
        let u_focal = 45.0_f64;
        let c_collateral = 72.0_f64;
        let u_global = u_focal - c_collateral;

        // Longino (1990) / Harding (1991) Strong Objectivity cost-loss framing gap:
        // Controller C = 35, L = 100 -> p*_inst = 0.35; Affected citizen C = 8, L = 160 -> p*_affected = 0.05
        let p_star_inst = 35.0_f64 / 100.0;
        let p_star_affected = 8.0_f64 / 160.0;
        let framing_gap = (p_star_inst - p_star_affected).abs();

        // --- Tension 2: Lakatos Program Shift & Anytime-Valid Partial Identification ---
        // Baseline K = 2, E_base = 24.0 -> adjusted 12.0.
        // Degenerative ad-hoc patch (K = 3, OOS RES drops, E_deg = 25.5 -> adjusted 8.5 < 12.0).
        // Progressive shift predicting novel fact (K = 3, OOS RES rises, E_prog = 54.0 -> adjusted 18.0 > 12.0).
        let lakatos_deg_e = 25.5_f64 / 3.0;
        let lakatos_prog_e = 54.0_f64 / 3.0;

        // Anytime-Valid Partial Identification: Manski interval [0.18, 0.34] (Delta_id = 0.16) + SAVI radius
        let delta_id = 0.34_f64 - 0.18_f64;
        let sigma_p = 0.40_f64;
        let alpha = 0.05_f64;
        let savi_radius = |n_eff: f64| -> f64 {
            sigma_p * ((2.0 * (1.0 / alpha).ln() + (1.0 + n_eff).ln()) / n_eff).sqrt()
        };
        let width_early = delta_id + 2.0 * savi_radius(25.0);
        let width_late = delta_id + 2.0 * savi_radius(2_500.0);

        // --- Tension 3: Georgescu-Roegen Penumbra & Stevens Ordinal Scale Invariance ---
        let penumbra_obs = [0.485_f64, 0.492, 0.498, 0.502, 0.507, 0.514, 0.20, 0.85];
        let rate_low_cut = penumbra_obs.iter().filter(|&&x| x >= 0.48).count() as f64
            / (penumbra_obs.len() as f64);
        let rate_high_cut = penumbra_obs.iter().filter(|&&x| x >= 0.52).count() as f64
            / (penumbra_obs.len() as f64);
        let penumbra_jump = (rate_low_cut - rate_high_cut).abs();

        // Ordinal samples A = [1, 1, 5] and B = [2, 3, 3]
        let ord_a = [1.0_f64, 1.0, 5.0];
        let ord_b = [2.0_f64, 3.0, 3.0];
        let mean_diff_raw = (ord_a.iter().sum::<f64>() - ord_b.iter().sum::<f64>()) / 3.0; // -1/3 < 0
        let ord_a_sq: Vec<f64> = ord_a.iter().map(|&x| x * x).collect();
        let ord_b_sq: Vec<f64> = ord_b.iter().map(|&x| x * x).collect();
        let mean_diff_sq = (ord_a_sq.iter().sum::<f64>() - ord_b_sq.iter().sum::<f64>()) / 3.0; // +5/3 > 0 (sign flip!)

        let mw_prob = |xs: &[f64], ys: &[f64]| -> f64 {
            let mut wins = 0.0;
            for &x in xs {
                for &y in ys {
                    if x < y {
                        wins += 1.0;
                    } else if (x - y).abs() < 1e-12 {
                        wins += 0.5;
                    }
                }
            }
            wins / ((xs.len() * ys.len()) as f64)
        };
        let mw_invariance_err = (mw_prob(&ord_a, &ord_b) - mw_prob(&ord_a_sq, &ord_b_sq)).abs();

        // --- Tension 4: Joint Meng-Moulton, Positivity/Kish, Capture-Recapture, Peterson Survival Bounds ---
        // Raw n = 10,000 rows in 200 unequal clusters (m_tilde = 65.0, rho_I = 0.15) -> deff_moulton = 1 + 64 * 0.15 = 10.6
        let raw_n = 10_000.0_f64;
        let deff_moulton = 1.0 + (65.0 - 1.0) * 0.15;
        let n_eff_moulton = raw_n / deff_moulton; // ~ 943.40 independent units
        // Out of N_PSU = 5,000 total units, selection bias rho_{R,Y} = 0.08 (DDI = 0.0064)
        let n_psu_total = 5_000.0_f64;
        let rho_ry = 0.08_f64;
        let n_eff_joint = n_eff_moulton / (1.0 + (n_psu_total - n_eff_moulton) * rho_ry * rho_ry); // ~ 34.99!
        let keynes_v_joint =
            KeynesEvidentialWeightAudit::evidential_weight(n_eff_joint, 0.22, 50.0);

        // Positivity violation: weights when one stratum has pi = 0.002 vs well-covered strata pi = 0.50
        let weights = [2.0_f64, 2.0, 2.0, 2.0, 500.0];
        let sum_w: f64 = weights.iter().sum();
        let sum_w2: f64 = weights.iter().map(|&w| w * w).sum();
        let n_eff_kish = (sum_w * sum_w) / sum_w2; // Collapses to ~ 1.03 despite 5 strata!
        // Routing the shadow stratum (pop share 0.20, pi < 0.01) to Manski bounds [0, 1] while covered share 0.80 has mean 0.30:
        let manski_pos_lo = 0.80 * 0.30 + 0.20 * 0.0;
        let manski_pos_hi = 0.80 * 0.30 + 0.20 * 1.0;

        // Capture-Recapture (Dual-System Lincoln-Petersen & Chao 1987 lower bound):
        // List A and List B: n_11 = 40 (in both), n_10 = 80 (only A), n_01 = 60 (only B) -> n_obs = 180, f1 = 140, f2 = 40
        let (n11, n10, n01) = (40.0_f64, 80.0_f64, 60.0_f64);
        let n_obs_cr = n11 + n10 + n01;
        let n00_lp = (n10 * n01) / n11; // 120 hidden -> N_LP = 300
        let n_hat_lp = n_obs_cr + n00_lp;
        let f1_singletons = n10 + n01;
        let f2_doubletons = n11;
        let n_hat_chao = n_obs_cr + (f1_singletons * f1_singletons) / (2.0 * f2_doubletons); // 180 + 19600/80 = 425

        // Peterson (1976) / Horowitz-Manski informative censoring bounds at horizon t:
        // Out of N_0 = 100 patients/cases: d = 20 observed events by t, c = 25 right-censored before t, s = 55 known survivors past t.
        // Peterson lower bound (all censored failed immediately): S_L(t) = s / N_0 = 0.55.
        // Peterson upper bound (all censored survived past t): S_U(t) = (s + c) / N_0 = 0.80.
        // Naive single-interval KM assuming independent censoring among those uncensored: s / (s + d) = 55 / 75 = 0.7333.
        // Exact identity: S_U(t) - S_L(t) == c / N_0 = 0.25.
        let (n0_surv, d_events, c_cens, s_surv) = (100.0_f64, 20.0_f64, 25.0_f64, 55.0_f64);
        let s_peterson_lo = s_surv / n0_surv;
        let s_peterson_hi = (s_surv + c_cens) / n0_surv;
        let s_km_naive = s_surv / (s_surv + d_events);
        let peterson_id_err = ((s_peterson_hi - s_peterson_lo) - (c_cens / n0_surv)).abs();

        let verified = power_agnot < 0.30
            && power_severe > 0.99
            && (pi_multiverse - 1.0).abs() < 1e-12
            && mean_e_multiverse > 20.0
            && u_focal > 0.0
            && u_global < 0.0
            && framing_gap > 0.25
            && lakatos_prog_e > 12.0
            && lakatos_deg_e < 12.0
            && width_early > width_late
            && width_late > delta_id
            && penumbra_jump > 0.50
            && mean_diff_raw < 0.0
            && mean_diff_sq > 0.0
            && mw_invariance_err < 1e-12
            && n_eff_joint < n_eff_moulton * 0.10
            && n_eff_kish < 1.10
            && n_hat_lp >= n_obs_cr
            && n_hat_chao >= n_hat_lp
            && s_peterson_lo < s_km_naive
            && s_km_naive < s_peterson_hi
            && peterson_id_err < 1e-12;

        Self {
            tension1_agnotological_vs_severe_power: (power_agnot, power_severe),
            tension1_multiverse_consistency_and_mean_e: (pi_multiverse, mean_e_multiverse),
            tension1_type_three_focal_vs_global_utility: (u_focal, u_global),
            tension1_strong_objectivity_threshold_gap: framing_gap,
            tension2_lakatos_degenerative_vs_progressive_e: (lakatos_deg_e, lakatos_prog_e),
            tension2_anytime_partial_id_widths_and_floor: (width_early, width_late, delta_id),
            tension3_penumbra_binary_jump: penumbra_jump,
            tension3_ordinal_mean_diff_raw_vs_squared: (mean_diff_raw, mean_diff_sq),
            tension3_mann_whitney_invariance_error: mw_invariance_err,
            tension4_joint_meng_moulton_and_keynes_v: (
                raw_n,
                n_eff_moulton,
                n_eff_joint,
                keynes_v_joint,
            ),
            tension4_kish_n_eff_and_positivity_manski_bounds: (
                n_eff_kish,
                manski_pos_lo,
                manski_pos_hi,
            ),
            tension4_capture_recapture_population_bounds: (n_obs_cr, n_hat_lp, n_hat_chao),
            tension4_peterson_survival_bounds_and_id_error: (
                s_peterson_lo,
                s_km_naive,
                s_peterson_hi,
                peterson_id_err,
            ),
            all_tensions_1_to_4_verified: verified,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn imbens_manski_ci_converges_from_two_sided_to_one_sided_critical_value() {
        // Point-identified limit (Delta_id = 0): c_n matches two-sided z_{0.975} ~ 1.96
        let point_ci = ImbensManskiConfidenceInterval::compute(0.20, 0.20, 0.40, 0.40, 500.0, 0.05);
        assert!((point_ci.critical_value_c_n - 1.959_96).abs() < 1e-3);
        assert!(point_ci.identification_ambiguity_width.abs() < 1e-15);
        assert!(point_ci.coverage_equation_residual < 1e-6);

        // Partially identified limit (Delta_id > 0, large n_eff): c_n converges to one-sided z_{0.95} ~ 1.64485
        let partial_ci =
            ImbensManskiConfidenceInterval::compute(0.15, 0.35, 0.40, 0.40, 2_000.0, 0.05);
        assert!((partial_ci.critical_value_c_n - 1.644_85).abs() < 1e-3);
        assert!(partial_ci.identification_share_of_total_width > 0.85);
        assert!(partial_ci.coverage_equation_residual < 1e-6);
    }

    #[test]
    fn mayo_manski_bridge_and_minimax_regret_close_identities() {
        let audit = MayoManskiSevereBoundaryAudit::canonical_benchmark();
        assert!(audit.mayo_manski_bridge_verified);
        assert!(audit.mayo_severe_at_lower_bound);
        assert!(audit.wald_maximin_at_upper_bound);
        assert!(audit.minimax_regret_recommends_protect);
        assert!(audit.minimax_regret_midpoint_identity_error < 1e-12);
        assert!(audit.exposes_naive_point_pseudo_severity);

        // When p* lies inside [theta_L, theta_U], verify the exact midpoint regret identity
        let straddling = MayoManskiSevereBoundaryAudit::evaluate(
            0.22, 0.08, 0.28, 0.30, 0.30, 800.0, 0.05, 16.0, 100.0,
        );
        assert!(!straddling.mayo_severe_at_lower_bound);
        assert!(straddling.wald_maximin_at_upper_bound);
        assert!(straddling.minimax_regret_recommends_protect);
        assert!(straddling.minimax_regret_midpoint_identity_error < 1e-12);
    }

    #[test]
    fn keynes_evidential_weight_and_process_tracing_taxonomy_hold() {
        let kw = KeynesEvidentialWeightAudit::canonical_benchmark();
        assert!(kw.keynes_and_process_tracing_verified);
        assert!(kw.thin_archive_weight_v < 0.05);
        assert!(kw.dense_archive_weight_v > 0.80);
        assert_eq!(kw.hoop_test.0, ProcessTracingTestKind::HoopTest);
        assert_eq!(kw.smoking_gun_test.0, ProcessTracingTestKind::SmokingGun);
        assert!(kw.combined_process_tracing_posterior > 0.98);
        assert!(kw.deciban_log_odds_identity_error < 1e-12);

        let (straw, _, _) = ProcessTracingTestKind::classify(0.60, 0.45);
        assert_eq!(straw, ProcessTracingTestKind::StrawInTheWind);
        let (doubly, _, _) = ProcessTracingTestKind::classify(0.96, 0.03);
        assert_eq!(doubly, ProcessTracingTestKind::DoublyDecisive);
    }

    #[test]
    fn filter_one_and_two_tensions_1_to_4_close_all_identities() {
        let audit = FilterOneAndTwoTensionsAudit::canonical_benchmark();
        assert!(audit.all_tensions_1_to_4_verified);
        assert!(audit.tension1_agnotological_vs_severe_power.0 < 0.25);
        assert!(audit.tension1_agnotological_vs_severe_power.1 > 0.99);
        assert!(audit.tension1_type_three_focal_vs_global_utility.0 > 0.0);
        assert!(audit.tension1_type_three_focal_vs_global_utility.1 < 0.0);
        assert!(audit.tension3_ordinal_mean_diff_raw_vs_squared.0 < 0.0);
        assert!(audit.tension3_ordinal_mean_diff_raw_vs_squared.1 > 0.0);
        assert!(audit.tension3_mann_whitney_invariance_error < 1e-12);
        assert!(audit.tension4_peterson_survival_bounds_and_id_error.3 < 1e-12);
    }
}
