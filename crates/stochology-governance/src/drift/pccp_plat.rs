//! Basel III FRTB MAR33 Profit & Loss Attribution Test (`PLAT`), Evaluated Competence Criterion,
//! and Predetermined Change Control Plan (`PCCP`, §4.2).

use stochology_core::math::stats::{spearman_rank_correlation, two_sample_ks_distance};

/// Traffic-light zone of the Basel III FRTB MAR33 Profit & Loss Attribution Test (`PLAT`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlatTrafficLightZone {
    /// **Green Zone ($r_s > 0.80$ and $D_{\mathrm{KS}} < 0.09$):** Model risk-theoretical loss aligns with realized hypothetical loss.
    Green,
    /// **Amber Zone:** Intermediate divergence requiring capital surcharge and `S3*` supervision.
    Amber,
    /// **Red Zone ($r_s < 0.70$ or $D_{\mathrm{KS}} > 0.12$):** Structural breakdown; trips the VSM Algedonic Channel and forces fallback.
    Red,
}

/// Basel III FRTB MAR33 Profit & Loss Attribution Test (`PLAT` — Spearman rank correlation $r_s$
/// and two-sample Kolmogorov-Smirnov statistic $D_{\mathrm{KS}}$ between Risk-Theoretical Loss `RTPL`
/// and Hypothetical Realized Loss `HPL`).
#[derive(Debug, Clone, PartialEq)]
pub struct ProfitLossAttributionTest {
    /// Spearman rank correlation $r_s \in [-1, 1]$ between `RTPL` and `HPL`.
    pub spearman_rank_correlation: f64,
    /// Two-sample Kolmogorov-Smirnov distance $D_{\mathrm{KS}} = \sup_x |F_{\mathrm{RTPL}}(x) - F_{\mathrm{HPL}}(x)|$.
    pub kolmogorov_smirnov_distance: f64,
    /// Assigned Basel III MAR33 traffic-light zone (`Green`, `Amber`, or `Red`).
    pub traffic_light_zone: PlatTrafficLightZone,
}

impl ProfitLossAttributionTest {
    /// Evaluates the PLAT attribution test on paired `rtpl` (model-predicted loss/payoff) and `hpl` (realized loss/payoff).
    pub fn evaluate(rtpl: &[f64], hpl: &[f64]) -> Self {
        assert!(
            !rtpl.is_empty() && rtpl.len() == hpl.len(),
            "PLAT requires non-empty equal-length RTPL and HPL slices"
        );
        let r_s = spearman_rank_correlation(rtpl, hpl);
        let d_ks = two_sample_ks_distance(rtpl, hpl);
        let zone = if r_s < 0.70 || d_ks > 0.12 {
            PlatTrafficLightZone::Red
        } else if r_s > 0.80 && d_ks < 0.09 {
            PlatTrafficLightZone::Green
        } else {
            PlatTrafficLightZone::Amber
        };
        Self {
            spearman_rank_correlation: r_s,
            kolmogorov_smirnov_distance: d_ks,
            traffic_light_zone: zone,
        }
    }
}

/// Criterion of Evaluated Competence (*Critério de Competência Avaliada*):
/// Audits whether an operator or autonomous model demonstrates genuine stochastic competence (**Telos**)
/// rather than unmeasured luck (**Skopos**).
#[derive(Debug, Clone, PartialEq)]
pub struct EvaluatedCompetenceCriterion {
    pub rel_corp_miscalibration: f64,
    pub res_corp_resolution: f64,
    pub kahneman_judgment_noise_sd: f64,
    pub plat_zone: PlatTrafficLightZone,
    pub competence_verified: bool,
}

impl EvaluatedCompetenceCriterion {
    pub fn evaluate(
        rel_corp_miscalibration: f64,
        res_corp_resolution: f64,
        kahneman_judgment_noise_sd: f64,
        plat_zone: PlatTrafficLightZone,
    ) -> Self {
        let competence_verified = rel_corp_miscalibration <= 0.015
            && res_corp_resolution >= 0.010
            && kahneman_judgment_noise_sd <= 0.10
            && plat_zone != PlatTrafficLightZone::Red;
        Self {
            rel_corp_miscalibration,
            res_corp_resolution,
            kahneman_judgment_noise_sd,
            plat_zone,
            competence_verified,
        }
    }
}

/// Predetermined Change Control Plan (`PCCP`, FDA AI/ML SaMD Lifecycle & Basel III Governance Contract):
///
/// Pre-specifies the allowed adaptation region (Data Modification Protocol + Re-training Objective +
/// Performance Evaluation Protocol) and couples Ville's $e$-process ($M_t \ge 1/\alpha$) and the `PLAT`
/// traffic-light test to the VSM Algedonic Channel (*"not measured, not executed"*), triggering an
/// automatic rollback to the sealed baseline when any boundary is breached.
#[derive(Debug, Clone, PartialEq)]
pub struct PredeterminedChangeControlPlan {
    pub plan_id: String,
    pub max_allowed_parameter_l2_drift: f64,
    pub observed_parameter_l2_drift: f64,
    pub sequential_e_process_value: f64,
    pub ville_alarm_threshold_inv_alpha: f64,
    pub plat_result: ProfitLossAttributionTest,
    pub competence_criterion: EvaluatedCompetenceCriterion,
    /// VSM Algedonic Channel rule: *"not measured, not executed"* (`true` blocks unmeasured updates).
    pub algedonic_not_measured_not_executed: bool,
    /// True if the PCCP triggered an automatic rollback to the last sealed baseline.
    pub automatic_rollback_triggered: bool,
    /// True if the candidate update is approved within the pre-registered PCCP envelope.
    pub update_approved: bool,
}

impl PredeterminedChangeControlPlan {
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate_update(
        plan_id: &str,
        max_allowed_parameter_l2_drift: f64,
        observed_parameter_l2_drift: f64,
        sequential_e_process_value: f64,
        alpha: f64,
        rtpl: &[f64],
        hpl: &[f64],
        rel_corp: f64,
        res_corp: f64,
        judgment_noise_sd: f64,
        telemetry_measured: bool,
    ) -> Self {
        let ville_alarm_threshold_inv_alpha = 1.0 / alpha.clamp(1e-6, 0.5);
        let plat_result = ProfitLossAttributionTest::evaluate(rtpl, hpl);
        let competence_criterion = EvaluatedCompetenceCriterion::evaluate(
            rel_corp,
            res_corp,
            judgment_noise_sd,
            plat_result.traffic_light_zone,
        );
        let algedonic_not_measured_not_executed = !telemetry_measured;
        let boundary_breached = !telemetry_measured
            || observed_parameter_l2_drift > max_allowed_parameter_l2_drift
            || sequential_e_process_value >= ville_alarm_threshold_inv_alpha
            || plat_result.traffic_light_zone == PlatTrafficLightZone::Red
            || !competence_criterion.competence_verified;

        Self {
            plan_id: plan_id.to_string(),
            max_allowed_parameter_l2_drift,
            observed_parameter_l2_drift,
            sequential_e_process_value,
            ville_alarm_threshold_inv_alpha,
            plat_result,
            competence_criterion,
            algedonic_not_measured_not_executed,
            automatic_rollback_triggered: boundary_breached,
            update_approved: !boundary_breached,
        }
    }
}
