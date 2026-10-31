//! **Chapter 3 (Filter I · Tension 2) — Underdetermination, Partial Identification, and Severe Testing.**
//!
//! Mediates the tension between holistic underdetermination (Duhem, 1906; Quine, 1951; Lakatos, 1970)
//! and severe refutability under incomplete data:
//! - Betting martingales (*e-processes*) and anytime-valid confidence sequences (Shafer, 2021;
//!   Grünwald, de Heide & Koolen, 2024; Ramdas et al., 2023);
//! - Lakatos's progressive vs. degenerative problemshift ratio (`lakatos_shift_ratio`);
//! - Non-parametric partial identification bounds (Manski, 2003, 2013) and Imbens–Manski (2004)
//!   confidence intervals (`ManskiBounds`);
//! - Selective-attrition trimming bounds (Lee, 2009, `LeeAttritionBounds`) and competing-risk
//!   informative censoring survival bounds (Peterson, 1976, `PetersonSurvivalBounds`);
//! - The **Mayo–Manski Severe Boundary Test**, requiring the conservative boundary of the partially
//!   identified interval to clear the minimum relevant effect $\delta_{\min}$.

use crate::rng::{mean, sample_variance};

/// Sequential evidence process (*e-process* $E_t = \prod_{i=1}^t e_i$) satisfying Ville's
/// martingale inequality $\mathbb{P}_{H_0}(\sup_{t \ge 1} E_t \ge 1/\alpha) \le \alpha$
/// (Shafer, 2021; Grünwald, de Heide & Koolen, 2024; Ramdas et al., 2023).
#[derive(Debug, Clone, PartialEq)]
pub struct EvidenceProcess {
    /// Step-by-step multiplicative *e-factors* $e_1, \dots, e_t \ge 0$.
    pub factors: Vec<f64>,
    /// Cumulative wealth trajectory $E_1, \dots, E_t$.
    pub trajectory: Vec<f64>,
    /// Current terminal evidence wealth $E_t$.
    pub terminal_e_value: f64,
    /// Running maximum $\sup_{1 \le s \le t} E_s$.
    pub running_supremum: f64,
    /// Turing–Good cumulative weight of evidence in decibans $W_t = 10 \log_{10}(E_t)$.
    pub decibans: f64,
}

impl EvidenceProcess {
    /// Builds an `EvidenceProcess` from a sequence of non-negative multiplicative *e-factors*.
    pub fn from_factors(factors: &[f64]) -> Self {
        let mut wealth = 1.0_f64;
        let mut sup = 1.0_f64;
        let mut trajectory = Vec::with_capacity(factors.len());
        for &f in factors {
            let clean_f = f.max(0.0);
            wealth = (wealth * clean_f).clamp(1e-30, 1e30);
            if wealth > sup {
                sup = wealth;
            }
            trajectory.push(wealth);
        }
        let db = 10.0 * wealth.max(1e-30).log10();
        Self {
            factors: factors.to_vec(),
            trajectory,
            terminal_e_value: wealth,
            running_supremum: sup,
            decibans: db,
        }
    }

    /// Gaussian mixture / fixed-betting-fraction *e-process* for testing $H_0: \mu \le \mu_0$
    /// against $H_1: \mu > \mu_0$ on standardized increments $z_i = (x_i - \mu_0)/\sigma$.
    pub fn from_gaussian_increments(
        xs: &[f64],
        null_mean: f64,
        sigma: f64,
        lambda_bet: f64,
    ) -> Self {
        assert!(sigma > 0.0, "sigma must be > 0");
        let lam = lambda_bet.clamp(-2.0, 2.0);
        let factors: Vec<f64> = xs
            .iter()
            .map(|&x| {
                let z = (x - null_mean) / sigma;
                (lam * z - 0.5 * lam * lam).exp()
            })
            .collect();
        Self::from_factors(&factors)
    }

    /// True if the *e-process* has cleared the Ville threshold $1/\alpha$.
    #[inline]
    pub fn rejects_null_at(&self, alpha: f64) -> bool {
        self.terminal_e_value >= 1.0 / alpha.clamp(1e-9, 0.5)
    }
}

/// Anytime-valid confidence sequence half-width $r_t(\alpha)$ (Howard et al., 2021; Ramdas et al., 2023)
/// for the running mean of observations with standard deviation $\sigma$ at step $t \ge 1$:
/// $$r_t(\alpha) = \sigma \sqrt{\frac{2 (t \rho^2 + 1)}{t^2 \rho^2} \ln\!\left(\frac{\sqrt{t \rho^2 + 1}}{\alpha}\right)}$$
pub fn anytime_valid_radius(sigma: f64, step_t: usize, alpha: f64) -> f64 {
    assert!(
        sigma >= 0.0 && step_t >= 1,
        "requires sigma >= 0 and step_t >= 1"
    );
    let t = step_t as f64;
    let a = alpha.clamp(1e-9, 0.5);
    let rho2 = 0.05_f64;
    let tr2 = t * rho2 + 1.0;
    let log_term = (tr2.sqrt() / a).ln().max(0.0);
    sigma * ((2.0 * tr2 / (t * t * rho2)) * log_term).sqrt()
}

/// Lakatos (1970) problemshift ratio $E_t / (K + 1)$, penalizing cumulative evidence $E_t$
/// by the number of *ad hoc* auxiliary adjustments $K$ added to rescue the hypothesis.
#[inline]
pub fn lakatos_shift_ratio(terminal_e_value: f64, ad_hoc_adjustments_k: usize) -> f64 {
    terminal_e_value / ((ad_hoc_adjustments_k + 1) as f64)
}

/// Charles F. Manski (2003, 2013) non-parametric worst-case partial identification bounds $[L_M, U_M]$
/// for $\mathbb{E}[Y]$ when a fraction $1 - p_{\mathrm{obs}}$ of outcomes is missing in $[y_{\min}, y_{\max}]$,
/// coupled with the Imbens–Manski (2004) confidence interval and the Mayo–Manski Severe Boundary Test.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ManskiBounds {
    /// Observed sample mean $\bar{Y}_{\mathrm{obs}}$.
    pub observed_mean: f64,
    /// Response / observation rate $p_{\mathrm{obs}} \in (0, 1]$.
    pub response_rate: f64,
    /// Missingness rate $p_{\mathrm{miss}} = 1 - p_{\mathrm{obs}}$.
    pub missing_rate: f64,
    /// Manski worst-case lower bound $L_M = p_{\mathrm{obs}}\bar{Y}_{\mathrm{obs}} + (1 - p_{\mathrm{obs}})y_{\min}$.
    pub lower_bound: f64,
    /// Manski worst-case upper bound $U_M = p_{\mathrm{obs}}\bar{Y}_{\mathrm{obs}} + (1 - p_{\mathrm{obs}})y_{\max}$.
    pub upper_bound: f64,
    /// Irreducible epistemic identification width $\Delta_{\mathrm{id}} = U_M - L_M = (1 - p_{\mathrm{obs}})(y_{\max} - y_{\min})$.
    pub identification_width: f64,
    /// Imbens–Manski (2004) confidence interval lower limit $L_{\mathrm{IM}}(\alpha)$.
    pub imbens_manski_lower: f64,
    /// Imbens–Manski (2004) confidence interval upper limit $U_{\mathrm{IM}}(\alpha)$.
    pub imbens_manski_upper: f64,
}

impl ManskiBounds {
    /// Computes Manski partial identification bounds and the Imbens–Manski (2004) confidence interval
    /// from observed values `observed`, missing count `missing_count`, and outcome support `[y_min, y_max]`.
    pub fn evaluate(
        observed: &[f64],
        missing_count: usize,
        y_min: f64,
        y_max: f64,
        alpha: f64,
    ) -> Self {
        assert!(
            !observed.is_empty() && y_min <= y_max,
            "ManskiBounds requires non-empty observed slice and y_min <= y_max"
        );
        let n_obs = observed.len();
        let n_tot = n_obs + missing_count;
        let p_obs = (n_obs as f64) / (n_tot as f64);
        let p_miss = 1.0 - p_obs;
        let obs_mean = mean(observed);

        let lower = p_obs * obs_mean + p_miss * y_min;
        let upper = p_obs * obs_mean + p_miss * y_max;
        let width = (upper - lower).max(0.0);

        let obs_var = if n_obs >= 2 {
            sample_variance(observed)
        } else {
            0.0
        };
        let se_bound = p_obs * (obs_var / (n_obs as f64)).sqrt();
        // Imbens & Manski (2004): when width > 0, one-sided critical value c_n -> z_{1-alpha}
        let z_crit = crate::inquiry::inverse_standard_normal_cdf(1.0 - alpha.clamp(1e-6, 0.49));
        let im_lower = (lower - z_crit * se_bound).max(y_min);
        let im_upper = (upper + z_crit * se_bound).min(y_max);

        Self {
            observed_mean: obs_mean,
            response_rate: p_obs,
            missing_rate: p_miss,
            lower_bound: lower,
            upper_bound: upper,
            identification_width: width,
            imbens_manski_lower: im_lower,
            imbens_manski_upper: im_upper,
        }
    }

    /// **Mayo–Manski Severe Boundary Test:** returns `true` iff even the conservative lower
    /// confidence boundary $L_{\mathrm{IM}}(\alpha)$ strictly exceeds the minimum relevant effect `min_relevant_effect`.
    #[inline]
    pub fn passes_severe_boundary(&self, min_relevant_effect: f64) -> bool {
        self.imbens_manski_lower > min_relevant_effect
    }

    /// Anytime-valid partial identification interval at step $t$, expanding the Manski bounds
    /// by the sequential confidence radius $r_t(\alpha)$.
    pub fn anytime_valid_interval(&self, sigma: f64, step_t: usize, alpha: f64) -> (f64, f64) {
        let r = anytime_valid_radius(sigma, step_t, alpha);
        (self.lower_bound - r, self.upper_bound + r)
    }
}

/// David S. Lee (2009) sharp non-parametric trimming bounds for the average treatment effect
/// on always-responders under differential sample attrition.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LeeAttritionBounds {
    /// Naive untrimmed difference in observed means $\bar{Y}_1 - \bar{Y}_0$.
    pub naive_difference: f64,
    /// Trimming proportion $p_{\mathrm{trim}} = \frac{|r_1 - r_0|}{\max(r_1, r_0)}$.
    pub trimming_fraction: f64,
    /// Lee lower bound $\Delta_L$ (trimming the top tail of the less-attrited group).
    pub lower_bound: f64,
    /// Lee upper bound $\Delta_U$ (trimming the bottom tail of the less-attrited group).
    pub upper_bound: f64,
}

impl LeeAttritionBounds {
    /// Computes Lee (2009) bounds from observed outcomes in treatment (`treated_obs` out of `n_treated_assigned`)
    /// and control (`control_obs` out of `n_control_assigned`).
    pub fn evaluate(
        treated_obs: &[f64],
        n_treated_assigned: usize,
        control_obs: &[f64],
        n_control_assigned: usize,
    ) -> Self {
        assert!(
            !treated_obs.is_empty()
                && !control_obs.is_empty()
                && n_treated_assigned >= treated_obs.len()
                && n_control_assigned >= control_obs.len(),
            "LeeAttritionBounds requires valid non-empty observed slices"
        );
        let r1 = (treated_obs.len() as f64) / (n_treated_assigned as f64);
        let r0 = (control_obs.len() as f64) / (n_control_assigned as f64);
        let naive = mean(treated_obs) - mean(control_obs);

        let mut t_sorted = treated_obs.to_vec();
        t_sorted.sort_by(|a, b| a.total_cmp(b));
        let mut c_sorted = control_obs.to_vec();
        c_sorted.sort_by(|a, b| a.total_cmp(b));

        if (r1 - r0).abs() <= 1e-12 {
            return Self {
                naive_difference: naive,
                trimming_fraction: 0.0,
                lower_bound: naive,
                upper_bound: naive,
            };
        }

        if r1 > r0 {
            let p_trim = (r1 - r0) / r1;
            let keep = ((1.0 - p_trim) * (t_sorted.len() as f64)).round() as usize;
            let keep = keep.clamp(1, t_sorted.len());
            let mean_low = mean(&t_sorted[..keep]);
            let mean_high = mean(&t_sorted[t_sorted.len() - keep..]);
            let c_mean = mean(&c_sorted);
            Self {
                naive_difference: naive,
                trimming_fraction: p_trim,
                lower_bound: mean_low - c_mean,
                upper_bound: mean_high - c_mean,
            }
        } else {
            let p_trim = (r0 - r1) / r0;
            let keep = ((1.0 - p_trim) * (c_sorted.len() as f64)).round() as usize;
            let keep = keep.clamp(1, c_sorted.len());
            let c_low = mean(&c_sorted[..keep]);
            let c_high = mean(&c_sorted[c_sorted.len() - keep..]);
            let t_mean = mean(&t_sorted);
            Self {
                naive_difference: naive,
                trimming_fraction: p_trim,
                lower_bound: t_mean - c_high,
                upper_bound: t_mean - c_low,
            }
        }
    }
}

/// Arthur V. Peterson (1976) & Horowitz–Manski non-parametric survival bounds $[S_L(t), S_U(t)]$
/// under informative censoring, contrasting with the independence assumption of Kaplan–Meier.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PetersonSurvivalBounds {
    /// Worst-case lower survival probability $S_L(t)$ (all censored subjects before $t$ failed).
    pub survival_lower: f64,
    /// Best-case upper survival probability $S_U(t)$ (all censored subjects before $t$ survived past $t$).
    pub survival_upper: f64,
    /// Naive independent-censoring survival estimate $S_{\mathrm{naive}}(t)$.
    pub naive_independent_survival: f64,
    /// Identification interval width $S_U(t) - S_L(t) = p_{\mathrm{censored}}(t)$.
    pub censoring_width: f64,
}

impl PetersonSurvivalBounds {
    /// Evaluates Peterson bounds at horizon $t$ given counts of observed failures `failed_by_t`,
    /// censored before $t$ `censored_by_t`, and known survivors past $t$ `survived_past_t`.
    pub fn evaluate(failed_by_t: usize, censored_by_t: usize, survived_past_t: usize) -> Self {
        let total = failed_by_t + censored_by_t + survived_past_t;
        assert!(total > 0, "PetersonSurvivalBounds requires total > 0");
        let n = total as f64;
        let s_lower = (survived_past_t as f64) / n;
        let s_upper = ((survived_past_t + censored_by_t) as f64) / n;
        let uncensored = (failed_by_t + survived_past_t).max(1) as f64;
        let s_naive = (survived_past_t as f64) / uncensored;
        Self {
            survival_lower: s_lower,
            survival_upper: s_upper,
            naive_independent_survival: s_naive,
            censoring_width: s_upper - s_lower,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evidence_process_and_lakatos_ratio_separate_progressive_from_ad_hoc() {
        let xs = [0.6, 0.8, 0.7, 0.9, 0.75, 0.85, 0.95, 0.8];
        let ep = EvidenceProcess::from_gaussian_increments(&xs, 0.0, 1.0, 0.8);
        assert!(ep.terminal_e_value > 10.0);
        assert!(ep.decibans > 10.0);
        assert!(lakatos_shift_ratio(80.0, 1) >= 20.0);
        assert!(lakatos_shift_ratio(18.0, 9) < 2.0);
    }

    #[test]
    fn manski_lee_and_peterson_bounds_capture_partial_identification() {
        let obs = [0.7, 0.8, 0.75, 0.85, 0.8, 0.78, 0.82, 0.76];
        let manski = ManskiBounds::evaluate(&obs, 2, 0.0, 1.0, 0.05);
        assert!(manski.lower_bound < manski.observed_mean);
        assert!(manski.upper_bound > manski.observed_mean);
        assert!((manski.identification_width - 0.20).abs() < 1e-12);
        assert!(manski.passes_severe_boundary(0.40));
        assert!(!manski.passes_severe_boundary(0.65));

        let lee = LeeAttritionBounds::evaluate(&[10.0, 12.0, 14.0, 20.0], 4, &[8.0, 9.0, 10.0], 4);
        assert!(lee.lower_bound <= lee.naive_difference);
        assert!(lee.upper_bound >= lee.naive_difference);

        let pet = PetersonSurvivalBounds::evaluate(20, 25, 55);
        assert!((pet.survival_lower - 0.55).abs() < 1e-12);
        assert!((pet.survival_upper - 0.80).abs() < 1e-12);
        assert!(pet.naive_independent_survival > pet.survival_lower);
    }
}
