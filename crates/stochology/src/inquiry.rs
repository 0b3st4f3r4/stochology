//! **Chapter 2 (Filter I · Tension 1) — Inquiry, Framing, and Antidogmatic Openness.**
//!
//! Mediates the tension between situational interest / selective framing and
//! antidogmatic openness:
//! - Lindley's Cromwell's Rule (`cromwell_clamp`);
//! - Cost-loss decision threshold $p^* = \frac{C}{\eta L}$, EVPI, and Longino–Harding
//!   strong-objectivity framing gap (`CostLossFraming`);
//! - Mayo's severe statistical power vs. Proctor–Cassam agnotological underpowering (`SeverePowerAnalysis`);
//! - Kimball–Mitroff Type III error of solving the wrong narrow problem (`TypeThreeFramingAudit`);
//! - Wang–Ramdas *e-BH* false discovery control (`e_bh_discoveries`) and Steegen's multiverse
//!   specification curve (`SpecificationCurve`).

use crate::math::standard_normal_cdf;
use crate::rng::{mean, median, sample_std};

/// Clamps a probability $p$ into the open-interior Cromwell interval $[\varepsilon, 1 - \varepsilon]$
/// (Lindley, 1985), preventing dogmatic $0\%$ or $100\%$ probabilities that paralyze Bayesian updating.
#[inline]
pub fn cromwell_clamp(p: f64, epsilon_floor: f64) -> f64 {
    let eps = epsilon_floor.clamp(1e-12, 0.49);
    p.clamp(eps, 1.0 - eps)
}

/// Checks whether a probability slice strictly obeys Cromwell's Rule for `epsilon_floor`.
pub fn obeys_cromwell_rule(probs: &[f64], epsilon_floor: f64) -> bool {
    let eps = epsilon_floor.clamp(1e-12, 0.49);
    !probs.is_empty()
        && probs
            .iter()
            .all(|&p| p.is_finite() && p >= eps && p <= 1.0 - eps)
}

/// Economic cost-loss decision threshold $p^* = \frac{C}{\eta L}$ and Expected Value of Perfect Information (EVPI).
///
/// Compares the institutional framing $(C_{\mathrm{inst}}, L_{\mathrm{inst}})$ against the affected
/// community's framing $(C_{\mathrm{affected}}, L_{\mathrm{affected}})$ (Longino, 1990; Harding, 1991).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CostLossFraming {
    /// Action cost $C > 0$.
    pub cost_c: f64,
    /// Mitigation effectiveness $\eta \in (0, 1]$.
    pub efficiency_eta: f64,
    /// Unmitigated harm loss $L > 0$.
    pub loss_l: f64,
    /// Optimal probability threshold $p^* = \frac{C}{\eta L} \in (0, 1)$.
    pub threshold_p_star: f64,
    /// Expected Value of Perfect Information $\mathrm{EVPI}(\pi_0)$ at base rate $\pi_0$.
    pub evpi: f64,
}

impl CostLossFraming {
    /// Evaluates the cost-loss threshold $p^* = \frac{C}{\eta L}$ and $\mathrm{EVPI}$ at base rate `base_rate_pi`.
    pub fn evaluate(cost_c: f64, efficiency_eta: f64, loss_l: f64, base_rate_pi: f64) -> Self {
        assert!(
            cost_c > 0.0 && efficiency_eta > 0.0 && efficiency_eta <= 1.0 && loss_l > 0.0,
            "CostLossFraming requires C > 0, eta in (0, 1], L > 0"
        );
        let eff_loss = efficiency_eta * loss_l;
        let p_star = (cost_c / eff_loss).clamp(1e-9, 1.0 - 1e-9);
        let pi = base_rate_pi.clamp(0.0, 1.0);
        // Prior Bayes action without information: act if pi >= p_star, incurring min(C, pi * eta * L)
        // With perfect information: act only when event occurs (prob pi), incurring pi * min(C, eta * L)
        let risk_prior = cost_c.min(pi * eff_loss);
        let risk_perfect = pi * cost_c.min(eff_loss);
        let evpi = (risk_prior - risk_perfect).max(0.0);
        Self {
            cost_c,
            efficiency_eta,
            loss_l,
            threshold_p_star: p_star,
            evpi,
        }
    }

    /// Computes the Longino–Harding framing gap $|p^*_{\mathrm{inst}} - p^*_{\mathrm{affected}}|$
    /// between two stakeholder loss valuations.
    pub fn framing_gap(&self, other: &Self) -> f64 {
        (self.threshold_p_star - other.threshold_p_star).abs()
    }
}

/// Severe statistical power audit against structural agnotology (Proctor, 2008; Cassam, 2019; Mayo, 2018).
///
/// An underpowered study ($n_{\mathrm{actual}} \ll n_{\mathrm{severe}}$) has low probability of detecting
/// a minimum socially relevant harm $\delta_{\min} > 0$, turning "non-significance" into a manufactured
/// certificate of safety.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SeverePowerAnalysis {
    /// Minimum socially or clinically relevant harm $\delta_{\min} > 0$.
    pub min_relevant_harm: f64,
    /// Outcome standard deviation $\sigma > 0$.
    pub std_dev: f64,
    /// Actual sample size $n$.
    pub sample_n: usize,
    /// One-sided significance level $\alpha \in (0, 0.5)$.
    pub alpha_one_sided: f64,
    /// Statistical power $\mathbb{P}(\text{reject } H_0 \mid \delta = \delta_{\min})$ at $n$.
    pub power_at_min_harm: f64,
    /// Severity of a "no harm" claim when $H_0$ is not rejected: $\mathrm{SEV}(\delta < \delta_{\min}) \approx \text{power}$.
    pub severity_of_null_claim: f64,
}

impl SeverePowerAnalysis {
    /// Evaluates the power and Mayo severity of a one-sided $z$-test for a minimum relevant harm $\delta_{\min}$.
    pub fn evaluate(
        min_relevant_harm: f64,
        std_dev: f64,
        sample_n: usize,
        alpha_one_sided: f64,
    ) -> Self {
        assert!(
            min_relevant_harm > 0.0 && std_dev > 0.0 && sample_n >= 1,
            "SeverePowerAnalysis requires delta > 0, sigma > 0, n >= 1"
        );
        // Approximate z_{1 - alpha} for common one-sided alpha (or via inverse normal)
        let z_alpha = inverse_standard_normal_cdf(1.0 - alpha_one_sided.clamp(1e-6, 0.49));
        let signal_to_noise = min_relevant_harm / (std_dev / (sample_n as f64).sqrt());
        let power = 1.0 - standard_normal_cdf(z_alpha - signal_to_noise);
        Self {
            min_relevant_harm,
            std_dev,
            sample_n,
            alpha_one_sided,
            power_at_min_harm: power,
            severity_of_null_claim: power,
        }
    }
}

/// Rational approximation to the inverse standard normal CDF $\Phi^{-1}(p)$ for $p \in (0, 1)$
/// (Peter Acklam / Beasley-Springer-Moro).
#[allow(clippy::excessive_precision)]
pub fn inverse_standard_normal_cdf(p: f64) -> f64 {
    let q = p.clamp(1e-12, 1.0 - 1e-12) - 0.5;
    if q.abs() <= 0.425 {
        let r = 0.180625 - q * q;
        q * (((((((2.5090809287301227e3 * r + 3.3430575583588128e4) * r + 6.7265770927008700e4)
            * r
            + 4.5921953931549871e4)
            * r
            + 1.3731693765509461e4)
            * r
            + 1.9715909503065514e3)
            * r
            + 1.3314166789178438e2)
            * r
            + 3.3871328727963666e0)
            / (((((((5.2264952788528545e3 * r + 2.8729085735721942e4) * r
                + 3.9307895800092710e4)
                * r
                + 2.1213794301586596e4)
                * r
                + 5.3941960214247511e3)
                * r
                + 6.8718700749205791e2)
                * r
                + 4.2313330701600911e1)
                * r
                + 1.0)
    } else {
        let r = if q < 0.0 { p } else { 1.0 - p };
        let s = (-r.clamp(1e-15, 1.0).ln()).sqrt();
        let val = (2.426679552305318 + s * (0.2153685100182 + s * 0.001300645))
            / (1.0 + s * (0.28969444 + s * 0.005310));
        if q < 0.0 { -val } else { val }
    }
}

/// Kimball (1957) & Mitroff (1974) Type III error audit: solving the wrong, overly narrow focal problem
/// ($\Delta U_{\mathrm{focal}} > 0$) while inflicting negative spillover on coupled subsystems ($\Delta U_{\mathrm{global}} < 0$).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TypeThreeFramingAudit {
    /// Gain on the narrow focal metric $\Delta U_{\mathrm{focal}}$.
    pub focal_gain: f64,
    /// Unaccounted negative externality / spillover cost on coupled domains $D_{\mathrm{spillover}} \ge 0$.
    pub spillover_damage: f64,
    /// Net global utility $\Delta U_{\mathrm{global}} = \Delta U_{\mathrm{focal}} - D_{\mathrm{spillover}}$.
    pub global_net_utility: f64,
    /// True if a Type III framing error is present ($\Delta U_{\mathrm{focal}} > 0$ yet $\Delta U_{\mathrm{global}} < 0$).
    pub is_type_three_error: bool,
}

impl TypeThreeFramingAudit {
    /// Evaluates whether optimizing the narrow metric causes a net global loss.
    pub fn evaluate(focal_gain: f64, spillover_damage: f64) -> Self {
        let global = focal_gain - spillover_damage;
        Self {
            focal_gain,
            spillover_damage,
            global_net_utility: global,
            is_type_three_error: focal_gain > 0.0 && global < 0.0,
        }
    }
}

/// Wang & Ramdas (2022) *e-BH* procedure for False Discovery Rate (FDR) control at level $\alpha$
/// across $K$ tested hypotheses or specifications under arbitrary dependence.
///
/// Returns the sorted indices ($0$-based) of the rejected hypotheses.
pub fn e_bh_discoveries(e_values: &[f64], alpha: f64) -> Vec<usize> {
    if e_values.is_empty() || alpha <= 0.0 {
        return Vec::new();
    }
    let k = e_values.len();
    let mut indexed: Vec<(usize, f64)> = e_values.iter().copied().enumerate().collect();
    indexed.sort_by(|a, b| b.1.total_cmp(&a.1)); // descending order of e-values

    let mut max_rank = 0usize;
    for (rank_zero, &(_, e_val)) in indexed.iter().enumerate() {
        let rank = rank_zero + 1;
        let threshold = (k as f64) / ((rank as f64) * alpha);
        if e_val >= threshold {
            max_rank = rank;
        }
    }
    let mut discoveries: Vec<usize> = indexed[..max_rank].iter().map(|&(idx, _)| idx).collect();
    discoveries.sort_unstable();
    discoveries
}

/// Steegen et al. (2016) Multiverse Specification Curve across $K \ge 1$ defensible analytical choices.
///
/// Audits whether an empirical conclusion is robust across the full space of reasonable specifications
/// or an artifact of selective reporting (*Garden of Forking Paths*, Gelman & Loken 2013).
#[derive(Debug, Clone, PartialEq)]
pub struct SpecificationCurve {
    /// Number of defensible specifications $K \ge 1$ evaluated.
    pub num_specifications_k: usize,
    /// Effect estimates $\hat{\theta}_1, \dots, \hat{\theta}_K$ across the multiverse.
    pub effects: Vec<f64>,
    /// Evidence values $E_1, \dots, E_K$ across the multiverse.
    pub e_values: Vec<f64>,
    /// Median effect across all $K$ specifications.
    pub median_effect: f64,
    /// Standard deviation of effect estimates across specifications.
    pub specification_std: f64,
    /// Proportion of specifications sharing the dominant sign and clearing $E_s \ge 1/\alpha$.
    pub consistency_ratio: f64,
    /// Multiverse average *e-value* $\bar{E}_{\mathcal{S}} = \frac{1}{K}\sum_{s=1}^K E_s$
    /// (valid *e-value* under arbitrary correlation among specifications).
    pub average_e_value: f64,
}

impl SpecificationCurve {
    /// Evaluates a multiverse of $K \ge 1$ effect estimates and their corresponding *e-values* at level $\alpha$.
    pub fn evaluate(effects: &[f64], e_values: &[f64], alpha: f64) -> Self {
        assert!(
            !effects.is_empty() && effects.len() == e_values.len(),
            "SpecificationCurve requires non-empty equal-length slices"
        );
        let k = effects.len();
        let med = median(effects);
        let std_spec = if k >= 2 { sample_std(effects) } else { 0.0 };
        let sign = if med >= 0.0 { 1.0 } else { -1.0 };
        let threshold = 1.0 / alpha.clamp(1e-6, 0.5);
        let consistent_count = effects
            .iter()
            .zip(e_values.iter())
            .filter(|&(&eff, &ev)| eff * sign > 0.0 && ev >= threshold)
            .count();
        let avg_e = mean(e_values);

        Self {
            num_specifications_k: k,
            effects: effects.to_vec(),
            e_values: e_values.to_vec(),
            median_effect: med,
            specification_std: std_spec,
            consistency_ratio: (consistent_count as f64) / (k as f64),
            average_e_value: avg_e,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cromwell_rule_rejects_dogmatic_extremes_and_clamps_interior() {
        assert!(!obeys_cromwell_rule(&[0.0, 0.5], 1e-4));
        assert!(!obeys_cromwell_rule(&[0.5, 1.0], 1e-4));
        let clamped = [cromwell_clamp(0.0, 1e-4), cromwell_clamp(1.0, 1e-4)];
        assert!(obeys_cromwell_rule(&clamped, 1e-4));
    }

    #[test]
    fn cost_loss_threshold_and_severe_power_expose_framing_and_agnotology() {
        let inst = CostLossFraming::evaluate(100.0, 1.0, 200.0, 0.20);
        let affected = CostLossFraming::evaluate(100.0, 1.0, 2000.0, 0.20);
        assert!((inst.threshold_p_star - 0.50).abs() < 1e-12);
        assert!((affected.threshold_p_star - 0.05).abs() < 1e-12);
        assert!((inst.framing_gap(&affected) - 0.45).abs() < 1e-12);
        assert!(affected.evpi > inst.evpi);

        let underpowered = SeverePowerAnalysis::evaluate(0.30, 1.0, 16, 0.05);
        let severe = SeverePowerAnalysis::evaluate(0.30, 1.0, 140, 0.05);
        assert!(underpowered.power_at_min_harm < 0.35);
        assert!(severe.power_at_min_harm > 0.90);
    }

    #[test]
    fn multiverse_specification_curve_and_e_bh_separate_robust_from_forking_paths() {
        let robust_eff = [0.28, 0.31, 0.26, 0.29, 0.33, 0.27];
        let robust_e = [32.0, 41.0, 25.0, 36.0, 48.0, 28.0];
        let curve_robust = SpecificationCurve::evaluate(&robust_eff, &robust_e, 0.05);
        assert!((curve_robust.consistency_ratio - 1.0).abs() < 1e-12);
        assert!(curve_robust.average_e_value > 20.0);

        let cherry_eff = [0.42, -0.04, 0.01, -0.02, 0.03, -0.01];
        let cherry_e = [21.5, 0.6, 0.8, 0.5, 0.9, 0.7];
        let curve_cherry = SpecificationCurve::evaluate(&cherry_eff, &cherry_e, 0.05);
        assert!(curve_cherry.consistency_ratio < 0.20);
        assert!(curve_cherry.average_e_value < 5.0);
        // Under e-BH multiplicity control across the 6 attempted specifications, the lone E=21.5 < 6/0.05=120 is not discovered!
        assert!(e_bh_discoveries(&cherry_e, 0.05).is_empty());
    }
}
