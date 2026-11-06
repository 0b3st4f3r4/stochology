//! **Block IV (Chapter 9) — Five Methodological Families, Process Tracing, Reflexive Self-Application,
//! and Leontief Complementarity ($M_{\mathrm{Leontief}}$).**
//!
//! Completes the disciplinary arc of Stochology:
//! - The Nuclear Triad and the Three Collapse Propositions (`CollapseProposition`, `TriadicIntegrity`);
//! - Qualitative, historical, legal, and clinical *Process Tracing* with Galenic *sēmeîa*
//!   and Keynes's evidential weight $V$ (`ProcessTracingTestKind`, `DiagnosticClue`, `ProcessTracingInference`);
//! - Reflexive self-application: the Four Internal Goodhart Brakes and Four Conditions of Refutability (`ReflexiveBrakeAudit`).

use crate::epistemic::EpistemicMarginState;
use crate::governance::GovernanceState;
use crate::kernel::{cromwell_clamp, inverse_standard_normal_cdf, mean, standard_normal_cdf};
use crate::vital::VitalMarginState;

/// The Collapse Propositions across the $2 \times 2 \times 2$ factorial cube when one, two, or all three
/// axes of the Nuclear Triad fall below their survival threshold.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CollapseProposition {
    /// **Collapse Proposition 1 (`011`, $\neg\text{Epistemic Margin}$):** *Blind Undersizing* —
    /// sizing reserves on a false point certainty underestimates selection defects and cluster echoes.
    Proposition1BlindUndersizing,
    /// **Collapse Proposition 2 (`101`, $\neg\text{Vital Margin}$):** *Impotent Lucidity Before Ruin* —
    /// knowing the exact width of uncertainty while operating at $100\%$ utilization or excess leverage
    /// leads to immediate absorption at the ruin barrier.
    Proposition2ImpotentLucidity,
    /// **Collapse Proposition 3 (`110`, $\neg\text{Institutional Governance}$):** *Cynical Capture and Pillaging* —
    /// epistemic doubt is weaponized as an alibi for inaction ("more studies are needed") while common
    /// vital reserves are privatized under limited-liability moral hazard.
    Proposition3CynicalCapture,
    /// **Combined Propositions 1 & 2 (`001`, $\neg S_{\mathrm{epi}} \land \neg S_{\mathrm{vital}}$):**
    /// *Accountable Institution with Zero Physical & Epistemic Buffer*.
    CombinedPropositions1And2,
    /// **Combined Propositions 1 & 3 (`010`, $\neg S_{\mathrm{epi}} \land \neg S_{\mathrm{gov}}$):**
    /// *Blindly Sized Reserves Pillaged by Moral Hazard*.
    CombinedPropositions1And3,
    /// **Combined Propositions 2 & 3 (`100`, $\neg S_{\mathrm{vital}} \land \neg S_{\mathrm{gov}}$):**
    /// *Lucid & Unprotected Before Ruin*.
    CombinedPropositions2And3,
    /// **Total Triadic Collapse (`000`, $\neg S_{\mathrm{epi}} \land \neg S_{\mathrm{vital}} \land \neg S_{\mathrm{gov}}$):**
    /// *Simultaneous Blindness, Congestion Ruin, and Unaccountable Capture*.
    /// Default fail-closed state for uninitialized structures.
    #[default]
    TotalTriadicCollapse,
    /// All three safeguards (`111`) are active and above their survival thresholds.
    IntactTriad,
}

/// Evaluates the Leontief structural complementarity of the Nuclear Triad
/// $(S_{\mathrm{epi}}, S_{\mathrm{vital}}, S_{\mathrm{gov}}) \in [0, 1]^3$:
/// $$M_{\mathrm{Leontief}} = \min\bigl(S_{\mathrm{epi}},\, S_{\mathrm{vital}},\, S_{\mathrm{gov}}\bigr)$$
/// alongside compensatory alternatives (arithmetic mean $M_{\mathrm{arith}}$ and Constant Elasticity of
/// Substitution $M_{\mathrm{CES}}(\sigma)$) to test false-safe classification rates near the ruin boundary.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct TriadicIntegrity {
    /// Epistemic margin score $S_{\mathrm{epi}} \in [0, 1]$.
    pub epistemic_margin: f64,
    /// Vital margin score $S_{\mathrm{vital}} \in [0, 1]$.
    pub vital_margin: f64,
    /// Institutional governance score $S_{\mathrm{gov}} \in [0, 1]$.
    pub institutional_governance: f64,
    /// Leontief bottleneck minimum $M_{\mathrm{Leontief}} = \min(S_{\mathrm{epi}}, S_{\mathrm{vital}}, S_{\mathrm{gov}})$.
    pub leontief_minimum: f64,
    /// Index of the binding bottleneck axis (`0` = Epistemic, `1` = Vital, `2` = Governance).
    pub bottleneck_axis: u8,
    /// Compensatory arithmetic mean $M_{\mathrm{arith}} = \frac{1}{3}(S_{\mathrm{epi}} + S_{\mathrm{vital}} + S_{\mathrm{gov}})$.
    pub arithmetic_mean: f64,
    /// Geometric / Cobb–Douglas mean $M_{\mathrm{geom}} = (S_{\mathrm{epi}} S_{\mathrm{vital}} S_{\mathrm{gov}})^{1/3}$ ($\sigma = 1$).
    pub geometric_mean: f64,
    /// Constant Elasticity of Substitution (CES) score at strong interior complementarity ($\sigma = 0.25$, $r = (\sigma - 1)/\sigma = -3$; $\sigma \to 0^+$ is the strict Leontief limit).
    pub ces_complement_score: f64,
    /// Active collapse proposition across the $2 \times 2 \times 2$ factorial cube if any axis falls below `min_threshold`.
    pub diagnose: CollapseProposition,
}

impl TriadicIntegrity {
    /// Evaluates the structural integrity of a decision system across the three axes of the Nuclear Triad.
    /// Computes `leontief_minimum = min(e, v, g)` and `bottleneck_axis` first, then derives `diagnose` from the Leontief minimum.
    pub fn evaluate(
        epistemic_margin: f64,
        vital_margin: f64,
        institutional_governance: f64,
        min_threshold: f64,
    ) -> Self {
        let clean_axis = |x: f64| -> f64 {
            if x.is_finite() {
                x.clamp(0.0, 1.0)
            } else {
                0.0
            }
        };
        let e = clean_axis(epistemic_margin);
        let v = clean_axis(vital_margin);
        let g = clean_axis(institutional_governance);
        let thresh = if min_threshold.is_finite() {
            min_threshold.clamp(0.05, 0.95)
        } else {
            0.50
        };
        let leontief_min = e.min(v).min(g);
        // Exact deterministic lexicographic tie-breaking (0: Epistemic -> 1: Vital -> 2: Governance)
        let bottleneck_axis = if e <= v && e <= g {
            0
        } else if v <= e && v <= g {
            1
        } else {
            2
        };
        let diag = if leontief_min >= thresh {
            CollapseProposition::IntactTriad
        } else {
            match (e >= thresh, v >= thresh, g >= thresh) {
                (false, true, true) => CollapseProposition::Proposition1BlindUndersizing,
                (true, false, true) => CollapseProposition::Proposition2ImpotentLucidity,
                (true, true, false) => CollapseProposition::Proposition3CynicalCapture,
                (false, false, true) => CollapseProposition::CombinedPropositions1And2,
                (false, true, false) => CollapseProposition::CombinedPropositions1And3,
                (true, false, false) => CollapseProposition::CombinedPropositions2And3,
                _ => CollapseProposition::TotalTriadicCollapse,
            }
        };
        let arith = (e + v + g) / 3.0;
        let geom = (e * v * g).cbrt();
        let ces = Self::ces_aggregate(e, v, g, 0.25);
        Self {
            epistemic_margin: e,
            vital_margin: v,
            institutional_governance: g,
            leontief_minimum: leontief_min,
            bottleneck_axis,
            arithmetic_mean: arith,
            geometric_mean: geom,
            ces_complement_score: ces,
            diagnose: diag,
        }
    }

    /// Evaluates `TriadicIntegrity` after normalizing each axis against its own quantile-calibrated
    /// survival threshold `axis_thresholds = [tau_epi, tau_vital, tau_gov]` Mapped to canonical threshold `0.50`.
    pub fn evaluate_quantile_calibrated(
        epistemic_margin: f64,
        vital_margin: f64,
        institutional_governance: f64,
        axis_thresholds: [f64; 3],
    ) -> Self {
        let map_axis = |score: f64, tau_i: f64| -> f64 {
            if !score.is_finite() || !tau_i.is_finite() {
                return 0.0;
            }
            let t = tau_i.clamp(0.05, 0.95);
            let s = score.clamp(0.0, 1.0);
            if s <= t {
                0.50 * (s / t)
            } else {
                0.50 + 0.50 * ((s - t) / (1.0 - t))
            }
        };
        Self::evaluate(
            map_axis(epistemic_margin, axis_thresholds[0]),
            map_axis(vital_margin, axis_thresholds[1]),
            map_axis(institutional_governance, axis_thresholds[2]),
            0.50,
        )
    }

    /// Evaluates `TriadicIntegrity` directly from the Three Canonical Type-State structures
    /// (`EpistemicMarginState`, `VitalMarginState`, `GovernanceState`) via their canonical $[0, 1]$ margin scores
    /// `(epi.epistemic_margin_score, vital.vital_margin_score, gov.nested_leontief_score)`.
    pub fn from_canonical_states(
        epi: &EpistemicMarginState,
        vital: &VitalMarginState,
        gov: &GovernanceState,
        min_threshold: f64,
    ) -> Self {
        Self::evaluate(
            epi.epistemic_margin_score,
            vital.vital_margin_score,
            gov.nested_leontief_score,
            min_threshold,
        )
    }

    /// Evaluates the symmetric Constant Elasticity of Substitution (CES) aggregator
    /// $M_{\mathrm{CES}}(\sigma) = \bigl(\frac{1}{3}(e^\rho + v^\rho + g^\rho)\bigr)^{1/\rho}$
    /// where $\rho = (\sigma - 1)/\sigma$ (and $\lim_{\sigma \to 0^+} M_{\mathrm{CES}}(\sigma) = \min(e, v, g)$).
    pub fn ces_aggregate(e: f64, v: f64, g: f64, elasticity_sigma: f64) -> f64 {
        if ![e, v, g, elasticity_sigma].iter().all(|x| x.is_finite()) {
            return 0.0;
        }
        let min_val = e.min(v).min(g).max(0.0);
        if elasticity_sigma <= 1e-3 || (elasticity_sigma <= 1.0 && min_val == 0.0) {
            return min_val;
        }
        if (elasticity_sigma - 1.0).abs() < 1e-4 {
            return (e.max(0.0) * v.max(0.0) * g.max(0.0)).cbrt();
        }
        let rho = (elasticity_sigma - 1.0) / elasticity_sigma;
        let e_c = if rho > 0.0 { e.max(0.0) } else { e.max(1e-12) };
        let v_c = if rho > 0.0 { v.max(0.0) } else { v.max(1e-12) };
        let g_c = if rho > 0.0 { g.max(0.0) } else { g.max(1e-12) };
        let mean_pow = (e_c.powf(rho) + v_c.powf(rho) + g_c.powf(rho)) / 3.0;
        mean_pow.powf(1.0 / rho).clamp(0.0, 1.0)
    }
}

/// Diagnostic test classification in qualitative, historical, legal, and clinical *Process Tracing*
/// (Galenic *sēmeîa*; Van Evera, 1997; Bennett & Checkel, 2015).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessTracingTestKind {
    /// Neither necessary nor sufficient ($\mathrm{LR}^+ > 1$ modest, $\mathrm{LR}^- < 1$ modest).
    StrawInTheWind,
    /// Necessary condition ($\mathrm{LR}^- \ll 1$): failing a Hoop Test eliminates the hypothesis.
    HoopTest,
    /// Sufficient / pathognomonic sign ($\mathrm{LR}^+ \gg 1$): passing a Smoking Gun confirms the hypothesis.
    SmokingGun,
    /// Both necessary and sufficient ($\mathrm{LR}^+ \gg 1$ and $\mathrm{LR}^- \ll 1$).
    DoublyDecisive,
}

/// One diagnostic clue (historical document, ethnographic observation, or clinical sign *sēmeîon*)
/// evaluated in Process Tracing.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DiagnosticClue {
    /// Sensitivity $\mathbb{P}(E \mid H) \in (0, 1)$: probability of observing the clue if the hypothesis is true.
    pub prob_given_hypothesis: f64,
    /// False-positive rate $\mathbb{P}(E \mid \neg H) \in (0, 1)$: probability of observing the clue under the rival explanation.
    pub prob_given_rival: f64,
    /// Whether the clue was actually observed (`true`) or absent (`false`) in the archive/field.
    pub clue_observed: bool,
    /// Evidential weight contribution $\Delta K > 0$ (depth of archival/clinical verification).
    pub weight_contribution: f64,
}

impl DiagnosticClue {
    /// Classifies the diagnostic clue into one of the four Process Tracing test kinds.
    pub fn classify(&self) -> ProcessTracingTestKind {
        let p_h = self.prob_given_hypothesis.clamp(1e-6, 1.0 - 1e-6);
        let p_r = self.prob_given_rival.clamp(1e-6, 1.0 - 1e-6);
        let high_necessity = p_h >= 0.85;
        let high_sufficiency = p_r <= 0.10;
        match (high_necessity, high_sufficiency) {
            (true, true) => ProcessTracingTestKind::DoublyDecisive,
            (true, false) => ProcessTracingTestKind::HoopTest,
            (false, true) => ProcessTracingTestKind::SmokingGun,
            (false, false) => ProcessTracingTestKind::StrawInTheWind,
        }
    }

    /// Computes the active Bayes likelihood ratio ($\mathrm{LR}^+$ if observed, $\mathrm{LR}^-$ if absent).
    pub fn active_likelihood_ratio(&self) -> f64 {
        let p_h = self.prob_given_hypothesis.clamp(1e-6, 1.0 - 1e-6);
        let p_r = self.prob_given_rival.clamp(1e-6, 1.0 - 1e-6);
        if self.clue_observed {
            p_h / p_r
        } else {
            (1.0 - p_h) / (1.0 - p_r)
        }
    }
}

/// Process Tracing inference combining Bayesian likelihood updating with Keynes's (1921) evidential weight $V$
/// and Popper–Mayo refutability requirements for qualitative, historical, and clinical inquiry.
#[derive(Debug, Clone, PartialEq)]
pub struct ProcessTracingInference {
    /// Prior probability $p_0 \in (0, 1)$.
    pub prior_probability: f64,
    /// Posterior probability $p_{\mathrm{post}} \in (0, 1)$ after evaluating all diagnostic clues.
    pub posterior_probability: f64,
    /// Cumulative Turing–Good weight of evidence in decibans $W = 10 \log_{10}(\mathrm{LR}_{\mathrm{total}})$.
    pub cumulative_decibans: f64,
    /// Initial Keynes evidential weight $V_0 = \frac{K_0}{K_0 + I_0}$.
    pub initial_keynes_weight_v: f64,
    /// Posterior Keynes evidential weight $V_{\mathrm{post}} = \frac{K_0 + \sum \Delta K}{K_0 + \sum \Delta K + I_0}$.
    pub posterior_keynes_weight_v: f64,
    /// True if at least one Hoop/Doubly Decisive clue was evaluated AND all evaluated Hoop/Doubly Decisive clues passed (`clue_observed == true`).
    pub has_refutable_hoop_condition: bool,
    /// True if any Hoop or Doubly Decisive clue failed (`clue_observed == false`), eliminating the hypothesis.
    pub hoop_refuted: bool,
}

impl ProcessTracingInference {
    /// Evaluates a sequence of diagnostic clues starting from `prior_p`, initial knowledge `k0`, and ignorance `i0`.
    pub fn evaluate(
        prior_p: f64,
        initial_k0: f64,
        ignorance_i0: f64,
        clues: &[DiagnosticClue],
    ) -> Self {
        let p0 = cromwell_clamp(prior_p, 1e-4);
        let mut odds = p0 / (1.0 - p0);
        let mut total_lr = 1.0_f64;
        let mut added_k = 0.0_f64;
        let mut has_hoop = false;
        let mut hoop_failed = false;

        for clue in clues {
            let kind = clue.classify();
            if matches!(
                kind,
                ProcessTracingTestKind::HoopTest | ProcessTracingTestKind::DoublyDecisive
            ) {
                has_hoop = true;
                if !clue.clue_observed {
                    hoop_failed = true;
                }
            }
            let lr = clue.active_likelihood_ratio();
            odds *= lr;
            total_lr *= lr;
            added_k += clue.weight_contribution.max(0.0);
        }

        let post_p = cromwell_clamp(odds / (1.0 + odds), 1e-6);
        let k0 = initial_k0.max(0.1);
        let i0 = ignorance_i0.max(0.1);
        let v0 = k0 / (k0 + i0);
        let v_post = (k0 + added_k) / (k0 + added_k + i0);

        Self {
            prior_probability: p0,
            posterior_probability: post_p,
            cumulative_decibans: 10.0 * total_lr.max(1e-30).log10(),
            initial_keynes_weight_v: v0,
            posterior_keynes_weight_v: v_post,
            has_refutable_hoop_condition: has_hoop && !hoop_failed,
            hoop_refuted: hoop_failed,
        }
    }
}

/// Reflexive self-application of Stochology under Goodhart's Law:
/// audits the Four Internal Goodhart Brakes that prevent each axis of the triad from degenerating
/// when turned into an isolated bureaucratic target, and prevent the composite itself from becoming
/// one.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ReflexiveBrakeAudit {
    /// **Brake 1 (Epistemic Margin):** CORP discriminative resolution $\mathrm{RES}_{\mathrm{CORP}}$ of a sharp
    /// Stochological interval vs. a lazy $[0, 1]$ "skeptical inflation" interval ($\mathrm{RES} = 0$).
    pub sharp_resolution: f64,
    /// Resolution of a lazy uninformative interval that predicts the constant base rate everywhere.
    pub skeptical_inflation_resolution: f64,
    /// **Brake 2 (Vital Margin):** Net social welfare when reserves are capped by Scarf/Samuelson marginal
    /// opportunity cost vs. regressive bureaucratic hoarding that starves present urgent needs.
    pub optimal_buffer_social_welfare: f64,
    /// Net social welfare under regressive hoarding beyond the opportunity-cost boundary.
    pub regressive_hoarding_welfare: f64,
    /// **Brake 3 (Institutional Governance):** Emergency fast-path Time-to-Recovery $\mathrm{TTR}_{\mathrm{fast}} \le \mathrm{TTS}_{\mathrm{survival}}$
    /// vs. procedural-theater delay $\mathrm{TTR}_{\mathrm{theater}} > \mathrm{TTS}_{\mathrm{survival}}$.
    pub recovery_delay_ttr_fast_path: f64,
    /// Slow procedural-theater audit duration ($\mathrm{TTR}_{\mathrm{theater}}$).
    pub recovery_delay_ttr_theater: f64,
    /// Physical Time-to-Survival Buffer ($\mathrm{TTS}_{\mathrm{survival}}$).
    pub survival_slack_tts: f64,
    /// **Brake 4 (Anti-Checklist Constitutional Rule):** $M_{\mathrm{Leontief}}$ is never declared as a
    /// scalar reward target or compensation multiplier. It operates exclusively as a diagnostic
    /// circuit-breaker whose component premises are audited directly. `true` iff no scalar target was
    /// declared; the flag is *declared* by the institution and never inferred from data.
    pub leontief_scalar_target_absent: bool,
}

impl ReflexiveBrakeAudit {
    /// Evaluates the four internal Goodhart brakes on a candidate forecast, reserve level, and emergency
    /// clock, assuming no scalar target has been declared over the composite.
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate(
        sharp_forecasts: &[f64],
        outcomes: &[f64],
        reserve_level: f64,
        hoarding_reserve_level: f64,
        urgent_present_marginal_value: f64,
        precautionary_benefit_scale: f64,
        recovery_delay_ttr_fast_path: f64,
        recovery_delay_ttr_theater: f64,
        survival_slack_tts: f64,
    ) -> Self {
        Self::evaluate_with_declaration(
            sharp_forecasts,
            outcomes,
            reserve_level,
            hoarding_reserve_level,
            urgent_present_marginal_value,
            precautionary_benefit_scale,
            recovery_delay_ttr_fast_path,
            recovery_delay_ttr_theater,
            survival_slack_tts,
            false,
        )
    }

    /// Same audit as [`Self::evaluate`], with the institutional declaration that brake 4 must check:
    /// `leontief_scalar_target_declared` is `true` when the composite has been turned into a scalar
    /// reward target, a compensation multiplier, or a certification badge — the degeneracy the brake
    /// exists to forbid.
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate_with_declaration(
        sharp_forecasts: &[f64],
        outcomes: &[f64],
        reserve_level: f64,
        hoarding_reserve_level: f64,
        urgent_present_marginal_value: f64,
        precautionary_benefit_scale: f64,
        recovery_delay_ttr_fast_path: f64,
        recovery_delay_ttr_theater: f64,
        survival_slack_tts: f64,
        leontief_scalar_target_declared: bool,
    ) -> Self {
        let (_, _, res_sharp, _, _) = GovernanceState::evaluate_corp_pav(sharp_forecasts, outcomes);
        let y_bar = mean(outcomes);
        // Pure constant base-rate forecast (uninformative [0, 1] skeptical inflation -> RES_CORP = 0.0)
        // Note: a 25%-contaminated every-4th-step strawman yields RES ~ 0.029, so the pure constant
        // baseline is used here to match the declared zero-resolution specification.
        let lazy_forecasts: Vec<f64> = sharp_forecasts
            .iter()
            .map(|_| if y_bar.is_finite() { y_bar } else { 0.0 })
            .collect();
        let (_, _, res_lazy, _, _) = GovernanceState::evaluate_corp_pav(&lazy_forecasts, outcomes);

        let eval_welfare = |r: f64| -> f64 {
            precautionary_benefit_scale * (1.0 + r.max(0.0)).ln()
                - urgent_present_marginal_value * r.max(0.0)
        };

        Self {
            sharp_resolution: res_sharp,
            skeptical_inflation_resolution: res_lazy,
            optimal_buffer_social_welfare: eval_welfare(reserve_level),
            regressive_hoarding_welfare: eval_welfare(hoarding_reserve_level),
            recovery_delay_ttr_fast_path,
            recovery_delay_ttr_theater,
            survival_slack_tts,
            leontief_scalar_target_absent: !leontief_scalar_target_declared,
        }
    }

    /// True if all four internal Goodhart brakes hold:
    /// 1. $\mathrm{RES}_{\mathrm{sharp}} > \mathrm{RES}_{\mathrm{lazy}} + 0.01$ (non-trivial discriminative resolution);
    /// 2. Optimal vital reserve welfare exceeds regressive hoarding welfare;
    /// 3. Emergency Maximin fast-path acts within $\mathrm{TTS}_{\mathrm{survival}}$ ($\mathrm{TTR}_{\mathrm{fast}} \le \mathrm{TTS}_{\mathrm{survival}}$)
    ///    and strictly faster than slow procedural theater ($\mathrm{TTR}_{\mathrm{fast}} < \mathrm{TTR}_{\mathrm{theater}}$);
    /// 4. the composite is not declared as a scalar reward target (`leontief_scalar_target_absent`).
    pub fn all_brakes_active(&self) -> bool {
        self.sharp_resolution > self.skeptical_inflation_resolution + 0.01
            && self.optimal_buffer_social_welfare > self.regressive_hoarding_welfare
            && self.recovery_delay_ttr_fast_path <= self.survival_slack_tts
            && self.recovery_delay_ttr_fast_path < self.recovery_delay_ttr_theater
            && self.leontief_scalar_target_absent
    }
}

/// Analytical and $3^5 = 243$ factorial audit of Pure Nested Leontief aggregation
/// ($\min_i g_i$, homothetic of degree 1) vs. continuous multiplicative gate products ($\prod_i g_i$).
///
/// Quantifies both uncalibrated threshold comparisons ($\tau = 0.50$) and volume/power-recalibrated
/// threshold comparisons ($\tau_{\mathrm{vol}} \approx 0.2951$, $\tau_{\mathrm{pow},5} = \tau^5 = 0.03125$).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LeontiefVsMultiplicativeAudit {
    /// Safety threshold $\tau \in (0, 1)$.
    pub threshold_tau: f64,
    /// Analytical Lebesgue feasible volume $V_{\min}(\tau) = (1 - \tau)^3$ in $[0, 1]^3$.
    pub leontief_volume_3d: f64,
    /// Analytical Lebesgue feasible volume $V_{\mathrm{prod}}(\tau) = 1 - \tau + \tau \ln \tau - \frac{\tau}{2}(\ln \tau)^2$ in $[0, 1]^3$.
    pub multiplicative_volume_3d: f64,
    /// Fraction of feasible parameter volume destroyed by uncalibrated multiplicative damping: $1 - V_{\mathrm{prod}}(\tau) / V_{\min}(\tau)$.
    pub volume_contraction_loss: f64,
    /// Volume-matched 3D product threshold $\tau_{\mathrm{vol}}$ satisfying $V_{\mathrm{prod}}(\tau_{\mathrm{vol}}) = V_{\min}(\tau)$.
    pub volume_matched_prod_tau: f64,
    /// Total factorial configurations tested on $\{0.60, 0.75, 0.90\}^5$ ($243$).
    pub grid_total: usize,
    /// False collapses under Architecture 1 (Pure Nested Leontief $\min_{i=1..5} \theta_i \ge \tau$).
    pub pure_leontief_false_collapses: usize,
    /// False collapse rate under Architecture 1 ($0.0$).
    pub pure_leontief_false_collapse_rate: f64,
    /// False collapses under Architecture 2 (Uncalibrated Hybrid Multiplicative Governance $\min(\theta_1, \theta_2, \theta_3 \theta_4 \theta_5) \ge \tau$).
    pub hybrid_gov_product_false_collapses: usize,
    /// False collapse rate under Architecture 2 ($180 / 243 \approx 0.74074$).
    pub hybrid_gov_product_false_collapse_rate: f64,
    /// False collapses under Architecture 3 (Uncalibrated 5-Invariant Product $\prod_{i=1}^5 \theta_i \ge \tau$).
    pub five_invariant_product_false_collapses: usize,
    /// False collapse rate under Architecture 3 ($242 / 243 \approx 0.99588$).
    pub five_invariant_product_false_collapse_rate: f64,
    /// False collapses on the 5-factor product when threshold is recalibrated to $\tau_{\mathrm{vol}}$ ($192 / 243$).
    pub volume_matched_prod5_false_collapses: usize,
    /// False collapses on the 5-factor product when threshold is power-calibrated to $\tau^5$ (homothetic equivalence to geometric mean: $0 / 243$).
    pub power_calibrated_prod5_false_collapses: usize,
    /// Total single-axis ruin boundary configurations tested ($3 \times 3 \times 2 \times 2 = 36$ where one axis $\in \{0.00, 0.15, 0.30\} < \tau$ and two axes $\in \{0.80, 0.90\}$).
    pub boundary_ruin_total: usize,
    /// False-safe diagnoses under compensatory arithmetic mean $M_{\mathrm{arith}} = (e + v + g)/3 \ge \tau$ on the single-axis ruin boundary ($36 / 36$).
    pub arithmetic_false_safe_collapses: usize,
    /// False-safe diagnosis rate under compensatory arithmetic mean on the single-axis ruin boundary ($1.0$).
    pub arithmetic_false_safe_rate: f64,
    /// False-safe diagnoses under Pure Nested Leontief $M_{\mathrm{Leontief}} = \min(e, v, g) \ge \tau$ on the single-axis ruin boundary ($0 / 36$).
    pub leontief_false_safe_collapses: usize,
    /// Collapses on the mixed threshold-crossing grid $\{0.35, 0.55, 0.80\}^5$ ($3^5 - 2^5 = 211 / 243$).
    pub crossing_grid_leontief_collapses: usize,
    /// False-safe arithmetic-mean diagnoses on the mixed threshold-crossing grid $\{0.35, 0.55, 0.80\}^5$ where $\min < \tau \le \text{mean}$.
    pub crossing_grid_arith_false_safe: usize,
}

impl LeontiefVsMultiplicativeAudit {
    /// Evaluates the exact analytical Lebesgue volumes, the $3^5 = 243$ interior factorial grid
    /// (both uncalibrated and volume/power-recalibrated), and the $36$-point single-axis ruin boundary grid at `threshold_tau`.
    pub fn evaluate(threshold_tau: f64) -> Self {
        let tau = threshold_tau.clamp(0.05, 0.95);
        let v_min = (1.0 - tau).powi(3);
        let eval_v_prod = |t: f64| -> f64 {
            let tc = t.clamp(1e-12, 1.0 - 1e-12);
            let l = tc.ln();
            1.0 - tc + tc * l - 0.5 * tc * l * l
        };
        let v_prod = eval_v_prod(tau);
        let loss = (1.0 - (v_prod / v_min.max(1e-15))).clamp(0.0, 1.0);

        // Solve V_prod(tau_vol) = V_min(tau) via bisection
        let mut lo = 1e-6_f64;
        let mut hi = tau;
        for _ in 0..60 {
            let mid = 0.5 * (lo + hi);
            if eval_v_prod(mid) > v_min {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        let tau_vol = 0.5 * (lo + hi);
        let tau_pow5 = tau.powi(5);

        let levels = [0.60_f64, 0.75, 0.90];
        let mut total = 0usize;
        let mut false_arch1 = 0usize;
        let mut false_arch2 = 0usize;
        let mut false_arch3 = 0usize;
        let mut false_vol_matched = 0usize;
        let mut false_pow_cal = 0usize;

        for &t1 in &levels {
            for &t2 in &levels {
                for &t3 in &levels {
                    for &t4 in &levels {
                        for &t5 in &levels {
                            total += 1;
                            let pure_min = t1.min(t2).min(t3).min(t4).min(t5);
                            if pure_min < tau {
                                false_arch1 += 1;
                            }
                            let hybrid_gov = t1.min(t2).min(t3 * t4 * t5);
                            if hybrid_gov < tau {
                                false_arch2 += 1;
                            }
                            let prod5 = t1 * t2 * t3 * t4 * t5;
                            if prod5 < tau {
                                false_arch3 += 1;
                            }
                            if prod5 < tau_vol {
                                false_vol_matched += 1;
                            }
                            if prod5 < tau_pow5 {
                                false_pow_cal += 1;
                            }
                        }
                    }
                }
            }
        }

        let ruin_levels = [0.00_f64, 0.15, 0.30];
        let safe_levels = [0.80_f64, 0.90];
        let mut boundary_total = 0usize;
        let mut arith_false_safe = 0usize;
        let mut leontief_false_safe = 0usize;

        for axis in 0..3 {
            for &r in &ruin_levels {
                for &s1 in &safe_levels {
                    for &s2 in &safe_levels {
                        boundary_total += 1;
                        let (e, v, g) = match axis {
                            0 => (r, s1, s2),
                            1 => (s1, r, s2),
                            _ => (s1, s2, r),
                        };
                        if (e + v + g) / 3.0 >= tau {
                            arith_false_safe += 1;
                        }
                        if e.min(v).min(g) >= tau {
                            leontief_false_safe += 1;
                        }
                    }
                }
            }
        }

        let tf = total.max(1) as f64;
        let bf = boundary_total.max(1) as f64;
        let crossing_levels = [0.35_f64, 0.55, 0.80];
        let mut crossing_leo_collapses = 0usize;
        let mut crossing_arith_false_safe = 0usize;
        for &c1 in &crossing_levels {
            for &c2 in &crossing_levels {
                for &c3 in &crossing_levels {
                    for &c4 in &crossing_levels {
                        for &c5 in &crossing_levels {
                            let c_min = c1.min(c2).min(c3).min(c4).min(c5);
                            let c_mean = (c1 + c2 + c3 + c4 + c5) / 5.0;
                            if c_min < tau {
                                crossing_leo_collapses += 1;
                                if c_mean >= tau {
                                    crossing_arith_false_safe += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
        Self {
            threshold_tau: tau,
            leontief_volume_3d: v_min,
            multiplicative_volume_3d: v_prod,
            volume_contraction_loss: loss,
            volume_matched_prod_tau: tau_vol,
            grid_total: total,
            pure_leontief_false_collapses: false_arch1,
            pure_leontief_false_collapse_rate: (false_arch1 as f64) / tf,
            hybrid_gov_product_false_collapses: false_arch2,
            hybrid_gov_product_false_collapse_rate: (false_arch2 as f64) / tf,
            five_invariant_product_false_collapses: false_arch3,
            five_invariant_product_false_collapse_rate: (false_arch3 as f64) / tf,
            volume_matched_prod5_false_collapses: false_vol_matched,
            power_calibrated_prod5_false_collapses: false_pow_cal,
            boundary_ruin_total: boundary_total,
            arithmetic_false_safe_collapses: arith_false_safe,
            arithmetic_false_safe_rate: (arith_false_safe as f64) / bf,
            leontief_false_safe_collapses: leontief_false_safe,
            crossing_grid_leontief_collapses: crossing_leo_collapses,
            crossing_grid_arith_false_safe: crossing_arith_false_safe,
        }
    }
}

/// Explicit pre-registered domain preconditions required before each of the Four Chapter 9 Falsifiers is testable:
/// - $F_1$ ($S_{\mathrm{epi}}$): specification multiverse size $K \ge 10$ or cluster $\mathrm{DEFF} \ge 1.5$;
/// - $F_2$ ($S_{\mathrm{vital}}$): clustered/heavy-tailed regimes $M \ge 20$ with horizon $T \ge 5$;
/// - $F_3$ ($S_{\mathrm{gov}}$): organizational units $J \ge 15$ under proxy pressure $\beta_{\mathrm{proxy}} \ge 0.70$;
/// - $F_4$ ($M_{\mathrm{Leontief}}$): joint multi-axis stress across all preconditions.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RefutabilityPreconditions {
    /// Number of plausible specification branches $K$ ($F_1$ requires $K \ge 10$ or $\mathrm{DEFF} \ge 1.5$).
    pub multiverse_specs_k: usize,
    /// Cluster / temporal design effect $\mathrm{DEFF}$ ($F_1$ requires $\mathrm{DEFF} \ge 1.5$ or $K \ge 10$).
    pub cluster_deff: f64,
    /// Number of operational regimes $M$ ($F_2$ requires $M \ge 20$).
    pub regimes_m: usize,
    /// Sequential horizon length $T$ ($F_2$ requires $T \ge 5$).
    pub horizon_t: usize,
    /// Number of organizational units $J$ ($F_3$ requires $J \ge 15$).
    pub units_j: usize,
    /// High-powered proxy incentive intensity $\beta_{\mathrm{proxy}}$ ($F_3$ requires $\beta_{\mathrm{proxy}} \ge 0.70$).
    pub proxy_pressure_beta: f64,
}

impl RefutabilityPreconditions {
    /// Returns true iff the domain preconditions for falsifier `falsifier_id` ($1..=4$) hold.
    pub fn is_satisfied_for(&self, falsifier_id: u8) -> bool {
        let f1_ok = self.multiverse_specs_k >= 10 || self.cluster_deff >= 1.5;
        let f2_ok = self.regimes_m >= 20 && self.horizon_t >= 5;
        let f3_ok = self.units_j >= 15 && self.proxy_pressure_beta >= 0.70;
        match falsifier_id {
            1 => f1_ok,
            2 => f2_ok,
            3 => f3_ok,
            4 => f1_ok && f2_ok && f3_ok,
            _ => false,
        }
    }
}

/// Reason why a [`PreregisteredFalsifier`] cannot be empirically adjudicated on the supplied data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotTestable {
    /// No paired observation differences were supplied.
    EmptyObservations,
    /// Pre-registered domain precondition (`K >= 10`, `DEFF >= 1.5`, `M >= 20`, `T >= 5`, `J >= 15`, `beta_proxy >= 0.70`) failed.
    PreconditionNotMet { falsifier_id: u8 },
    /// Effective independent sample size $n_{\mathrm{eff}} = \lfloor n / \mathrm{DEFF} \rfloor$ is below `required_n_eff`.
    InsufficientEffectiveSampleSize {
        observed_n_eff: usize,
        required_n_eff: usize,
    },
}

/// Empirical post-data verdict returned when a [`PreregisteredFalsifier`] is confronted with observed paired differences.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FalsifierVerdict {
    /// Hypothesis is refuted by the observed paired differences ($\bar{d} \le d_{\mathrm{crit}}$) with Mayo post-data severity $\mathrm{SEV}_{\mathrm{fals}}$.
    Refuted {
        observed_mean_d: f64,
        post_data_severity: f64,
    },
    /// Hypothesis is corroborated by the observed paired differences ($\bar{d} \ge d_{\mathrm{corrob}}$) with Mayo severity $\mathrm{SEV}(\Delta \ge \delta_{\min}) \ge 0.95$.
    Corroborated {
        observed_mean_d: f64,
        post_data_severity: f64,
    },
    /// Observed improvement lies in the indeterminate zone $(d_{\mathrm{crit}}, d_{\mathrm{corrob}})$.
    Indeterminate {
        observed_mean_d: f64,
        post_data_severity: f64,
    },
}

/// Pre-registered Empirical Falsifier ($F_1$ to $F_4$) with Burden-of-Proof Inversion
/// ($H_0^{\mathrm{fals}}: \Delta_{\mathrm{harm}} \ge \delta_{\min}$ vs. $H_1^{\mathrm{fals}}: \Delta_{\mathrm{harm}} < \delta_{\min}$)
/// and Deborah Mayo (2018) Post-Data Severity ($\mathrm{SEV} \ge 0.95$).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PreregisteredFalsifier {
    /// Falsifier index ($1..=4$).
    pub id: u8,
    /// Minimum substantively relevant harm reduction $\delta_{\min} > 0$.
    pub delta_min: f64,
    /// Paired difference standard deviation $\sigma_d > 0$.
    pub sigma_d: f64,
    /// Required effective independent sample size $n_{\mathrm{eff}}^{\mathrm{req}} = \lceil [(|z_\alpha| + z_{1-\beta})\sigma_d / \delta_{\min}]^2 \rceil$.
    pub required_n_eff: usize,
    /// Standard error at $n_{\mathrm{eff}}^{\mathrm{req}}$: $\mathrm{SE} = \sigma_d / \sqrt{n_{\mathrm{eff}}^{\mathrm{req}}}$.
    pub standard_error: f64,
    /// Critical threshold $d_{\mathrm{crit}} = \delta_{\min} - |z_\alpha| \mathrm{SE}$ below which Stochology is falsified at $\alpha = 0.05$.
    pub falsification_critical_d: f64,
    /// Observed improvement $d_{\mathrm{corrob}} = \delta_{\min} + |z_\alpha| \mathrm{SE}$ required for Mayo corroboration $\mathrm{SEV}(\Delta \ge \delta_{\min}) \ge 0.95$.
    pub mayo_corroboration_d: f64,
    /// Post-data falsification severity $\mathrm{SEV}_{\mathrm{fals}}(d_{\mathrm{null}})$ when observed gain is near zero ($d_{\mathrm{null}} = 0.02 \delta_{\min}$).
    pub falsification_severity_at_null: f64,
}

impl PreregisteredFalsifier {
    /// Designs a pre-registered falsifier with burden-of-proof inversion at significance $\alpha = 0.05$
    /// and power $1 - \beta = 0.90$.
    pub fn design(id: u8, delta_min: f64, sigma_d: f64) -> Self {
        if !delta_min.is_finite() || delta_min <= 0.0 || !sigma_d.is_finite() || sigma_d <= 0.0 {
            return Self {
                id,
                delta_min: 0.0,
                sigma_d: 0.0,
                required_n_eff: usize::MAX,
                standard_error: f64::INFINITY,
                falsification_critical_d: f64::INFINITY,
                mayo_corroboration_d: f64::INFINITY,
                falsification_severity_at_null: 0.0,
            };
        }
        let d_min = delta_min;
        let sig = sigma_d;
        let z_alpha_abs = inverse_standard_normal_cdf(1.0 - 0.05);
        let z_power = inverse_standard_normal_cdf(0.90);
        let ratio = ((z_alpha_abs + z_power) * sig) / d_min;
        let n_req = (ratio * ratio).ceil() as usize;
        let se = sig / (n_req.max(1) as f64).sqrt();
        let d_crit = d_min - z_alpha_abs * se;
        let d_corrob = d_min + z_alpha_abs * se;
        let d_null = 0.02 * d_min;
        let sev_fals = standard_normal_cdf((d_min - d_null) / se);

        Self {
            id,
            delta_min: d_min,
            sigma_d: sig,
            required_n_eff: n_req,
            standard_error: se,
            falsification_critical_d: d_crit,
            mayo_corroboration_d: d_corrob,
            falsification_severity_at_null: sev_fals,
        }
    }

    /// Returns all four canonical pre-registered falsifiers ($F_1$ for $S_{\mathrm{epi}}$, $F_2$ for $S_{\mathrm{vital}}$,
    /// $F_3$ for $S_{\mathrm{gov}}$, and $F_4$ for $M_{\mathrm{Leontief}}$).
    pub fn canonical_four() -> [Self; 4] {
        [
            Self::design(1, 0.15, 0.25),
            Self::design(2, 1.50, 2.20),
            Self::design(3, 0.20, 0.30),
            Self::design(4, 0.25, 0.35),
        ]
    }

    /// Evaluates real observed paired differences `paired_differences` against this falsifier's
    /// pre-registered critical thresholds and domain `preconditions`.
    /// Without observations or when domain/sample preconditions fail, returns `Err(NotTestable)`.
    pub fn evaluate_observations(
        &self,
        paired_differences: &[f64],
        deff: f64,
        preconditions: &RefutabilityPreconditions,
    ) -> Result<FalsifierVerdict, NotTestable> {
        if paired_differences.is_empty() {
            return Err(NotTestable::EmptyObservations);
        }
        if !preconditions.is_satisfied_for(self.id) {
            return Err(NotTestable::PreconditionNotMet {
                falsifier_id: self.id,
            });
        }
        let n_eff = ((paired_differences.len() as f64) / deff.max(1.0)).floor() as usize;
        if n_eff < self.required_n_eff {
            return Err(NotTestable::InsufficientEffectiveSampleSize {
                observed_n_eff: n_eff,
                required_n_eff: self.required_n_eff,
            });
        }
        let d_bar = mean(paired_differences);
        let emp_var = if paired_differences.len() >= 2 {
            let nf = paired_differences.len() as f64;
            paired_differences
                .iter()
                .map(|&x| (x - d_bar) * (x - d_bar))
                .sum::<f64>()
                / (nf - 1.0)
        } else {
            self.sigma_d * self.sigma_d
        };
        let se_obs = (emp_var.max(1e-12).sqrt() / (n_eff as f64).sqrt()).max(1e-9);
        if d_bar <= self.falsification_critical_d {
            let sev = standard_normal_cdf((self.delta_min - d_bar) / se_obs);
            Ok(FalsifierVerdict::Refuted {
                observed_mean_d: d_bar,
                post_data_severity: sev,
            })
        } else if d_bar >= self.mayo_corroboration_d {
            let sev = standard_normal_cdf((d_bar - self.delta_min) / se_obs);
            Ok(FalsifierVerdict::Corroborated {
                observed_mean_d: d_bar,
                post_data_severity: sev,
            })
        } else {
            let sev = standard_normal_cdf((d_bar - self.delta_min) / se_obs);
            Ok(FalsifierVerdict::Indeterminate {
                observed_mean_d: d_bar,
                post_data_severity: sev,
            })
        }
    }
}

/// Closed-form analytical benchmark scorecard for the Three Collapse Propositions
/// (`[Assumed]` benchmark parameters evaluated on closed-form identities):
/// - **Proposition 1 (Epistemic Coverage Collapse under Non-Random Selection):** Under Meng's (2018) exact identity
///   $|\bar{Y}_n - \bar{Y}_N| = |\rho_{R,Y}|\sqrt{(1-f)/f}\,\sigma_Y > 0$, as $n \to N f$, the nominal i.i.d.
///   z-statistic $|Z_{\mathrm{naive}}| = |\rho_{R,Y}|\sqrt{n(1-f)/f}$ diverges as $\mathcal{O}(\sqrt{n})$, driving
///   nominal confidence-interval coverage $\mathbb{P}(\bar{Y}_N \in \bar{Y}_n \pm z_{1-\alpha/2}\sigma_Y/\sqrt{n})$ to $0$.
/// - **Proposition 2 (Queue Explosion, Over-Leverage Drain & Ruin Absorption under Zero Vital Margin):**
///   Under Kingman (1961), $\lim_{\rho \to 1^-} \frac{\rho}{1-\rho} = +\infty$; under Itô (1951) & Kelly (1956),
///   any leverage $f > 2(\mu - r)/\sigma^2$ yields negative excess logarithmic growth $g(f) - r < 0$; and under
///   Simchi-Levi (2014), whenever $\mathrm{TTS}_{\mathrm{survival}} = B_0/\dot{D} < \mathrm{TTR}_{\mathrm{recovery}}$,
///   unfiltered reserve balance $B_0 - \dot{D}\,\mathrm{TTR}_{\mathrm{recovery}} < 0$ crosses $0$ at $t^* = \mathrm{TTS}_{\mathrm{survival}} < \mathrm{TTR}_{\mathrm{recovery}}$.
/// - **Proposition 3 (Algedonic Deafness & Multitask Care Erosion under Ungoverned Incentives):**
///   Under Fricker (2007) & Ville (1939), testimonial discount $\delta_{\mathrm{cred}} > 1 - \frac{\ln(1/\alpha)}{\mathrm{TTS}_{\mathrm{survival}}\ln e}$
///   forces alarm stopping time $\tau_{\mathrm{alarm}}(\delta_{\mathrm{cred}}) > \mathrm{TTS}_{\mathrm{survival}}$, while under
///   Holmström–Milgrom (1991) multitask substitution with cross-cost $c_{12} > 0$, raising uncompensated proxy power
///   $\beta_{\mathrm{proxy}} > 0$ strictly reduces unmeasured care effort ($\Delta a_{\mathrm{care}}^* = -\frac{c_{12}}{c_{11}c_{22}-c_{12}^2}\beta_{\mathrm{proxy}} < 0$).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StructuralCollapseProofAudit {
    /// Theoretical i.i.d. coverage in the finite-sample `CommunityReserveEnv` regime ($n = 46, f = 46/60, |\rho_{R,Y}| = 0.62 \implies \mathrm{Cov} \approx 0.34$).
    pub prop1_finite_engine_coverage: f64,
    /// Monotone nominal i.i.d. coverage decay curve across sample-size ladder $n \in \{46, 500, 5\,000, 50\,000, 2\,300\,000\}$.
    pub prop1_coverage_ladder: [f64; 5],
    /// Nominal i.i.d. coverage probability under Meng selection bias at large $n$ ($\to 0$).
    pub prop1_naive_asymptotic_coverage: f64,
    /// Finite-sample Imbens–Manski coverage over Bhatia–Davis bounded support when $|\rho_{R,Y}| \le 1$.
    pub prop1_manski_support_coverage: f64,
    /// Excess logarithmic growth rate $g(f) - r$ under over-leverage $f > 2 f^*$ ($< 0$).
    pub prop2_overleveraged_excess_growth: f64,
    /// Unfiltered reserve balance $B_0 - \dot{D}\,\mathrm{TTR}_{\mathrm{recovery}}$ at recovery time $t = \mathrm{TTR}_{\mathrm{recovery}}$ (with $B_0 = 45.0, \dot{D} = B_0/\mathrm{TTS}_{\mathrm{survival}}$; $< 0$ when $\mathrm{TTS}_{\mathrm{survival}} < \mathrm{TTR}_{\mathrm{recovery}}$).
    pub prop2_reserve_at_ttr: f64,
    /// Discounted algedonic alarm delay minus survival time $\tau_{\mathrm{alarm}}(\delta_{\mathrm{cred}}) - \mathrm{TTS}_{\mathrm{survival}}$ ($> 0$).
    pub prop3_algedonic_delay_overflow: f64,
    /// Multitask unmeasured care effort change $\Delta a_{\mathrm{care}}^*$ under high-powered proxy incentive $\beta_{\mathrm{proxy}} > 0$ ($< 0$).
    pub prop3_multitask_care_change: f64,
}

impl StructuralCollapseProofAudit {
    /// Evaluates the closed-form analytical benchmark inequalities for all three Collapse Propositions
    /// across both the finite-sample simulator regime ($n = 46$) and the asymptotic sample-size ladder.
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate_analytical(
        sample_n: f64,
        sampling_fraction_f: f64,
        rho_ry: f64,
        excess_drift: f64,
        variance_sigma2: f64,
        over_leverage_f: f64,
        tts_survival: f64,
        ttr_recovery: f64,
        testimonial_discount: f64,
        per_step_e_value: f64,
        proxy_power_beta: f64,
        multitask_cross_cost_c12: f64,
    ) -> Self {
        let z_crit = inverse_standard_normal_cdf(0.975);
        let eval_cov = |n_val: f64, f_val: f64, rho_val: f64| -> f64 {
            let f_c = f_val.clamp(1e-9, 1.0 - 1e-9);
            let zb = rho_val.abs() * (n_val.max(1.0) * (1.0 - f_c) / f_c).sqrt();
            (standard_normal_cdf(z_crit - zb) - standard_normal_cdf(-z_crit - zb)).clamp(0.0, 1.0)
        };
        let cov_engine_46 = eval_cov(46.0, 46.0 / 60.0, 0.62);
        let ladder_ns = [46.0, 500.0, 5_000.0, 50_000.0, sample_n.max(2_300_000.0)];
        let mut cov_ladder = [0.0; 5];
        for (idx, &nv) in ladder_ns.iter().enumerate() {
            cov_ladder[idx] = eval_cov(nv, sampling_fraction_f, rho_ry);
        }
        let cov_naive = eval_cov(sample_n, sampling_fraction_f, rho_ry);
        let f_m = sampling_fraction_f.clamp(1e-9, 1.0 - 1e-9);
        let manski_slack_z = (1.0 - rho_ry.abs()) * (sample_n.max(1.0) * (1.0 - f_m) / f_m).sqrt();
        let cov_manski = (standard_normal_cdf(z_crit + manski_slack_z)
            - standard_normal_cdf(-z_crit - manski_slack_z))
        .clamp(0.0, 1.0);

        let g_excess = over_leverage_f * excess_drift
            - 0.5 * over_leverage_f * over_leverage_f * variance_sigma2;
        let initial_reserve_b0 = 45.0;
        let burn_rate_d = initial_reserve_b0 / tts_survival.max(1e-9);
        let res_at_ttr = initial_reserve_b0 - burn_rate_d * ttr_recovery;

        let eff_ln_e =
            (1.0 - testimonial_discount.clamp(0.0, 0.999)) * per_step_e_value.max(1.01).ln();
        let tau_alarm = (20.0_f64.ln() / eff_ln_e.max(1e-12)).ceil();
        let delay_overflow = tau_alarm - tts_survival;

        let det_c = (1.0 - multitask_cross_cost_c12 * multitask_cross_cost_c12).max(1e-6);
        let delta_care = -(multitask_cross_cost_c12.max(0.0) / det_c) * proxy_power_beta.max(0.0);

        Self {
            prop1_finite_engine_coverage: cov_engine_46,
            prop1_coverage_ladder: cov_ladder,
            prop1_naive_asymptotic_coverage: cov_naive,
            prop1_manski_support_coverage: cov_manski,
            prop2_overleveraged_excess_growth: g_excess,
            prop2_reserve_at_ttr: res_at_ttr,
            prop3_algedonic_delay_overflow: delay_overflow,
            prop3_multitask_care_change: delta_care,
        }
    }

    /// Returns true iff all closed-form analytical benchmark collapse conditions hold simultaneously.
    pub fn all_benchmark_conditions_met(&self) -> bool {
        let monotone_decay = self.prop1_coverage_ladder.windows(2).all(|w| w[0] >= w[1])
            && self.prop1_coverage_ladder[0] > 0.80
            && self.prop1_coverage_ladder[4] < 1e-6;
        monotone_decay
            && self.prop1_finite_engine_coverage < 0.40
            && self.prop1_naive_asymptotic_coverage < 1e-6
            && self.prop1_manski_support_coverage >= 0.95
            && self.prop2_overleveraged_excess_growth < 0.0
            && self.prop2_reserve_at_ttr <= 0.0
            && self.prop3_algedonic_delay_overflow > 0.0
            && self.prop3_multitask_care_change < 0.0
    }

    /// Alias retained for API compatibility; delegates to [`Self::all_benchmark_conditions_met`].
    pub fn all_three_propositions_proven(&self) -> bool {
        self.all_benchmark_conditions_met()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn triadic_integrity_process_tracing_and_reflexive_brakes_hold() {
        // A11: Default TriadicIntegrity is fail-closed (TotalTriadicCollapse)
        let def_triad = TriadicIntegrity::default();
        assert_eq!(
            def_triad.diagnose,
            CollapseProposition::TotalTriadicCollapse
        );

        let intact = TriadicIntegrity::evaluate(0.90, 0.85, 0.88, 0.50);
        assert_eq!(intact.diagnose, CollapseProposition::IntactTriad);
        assert!((intact.leontief_minimum - 0.85).abs() < 1e-12);
        assert_eq!(intact.bottleneck_axis, 1);

        // §7 Criterion 4 & B6: Monotone axis transformation shifts bottleneck_axis and diagnose together
        let p2 = TriadicIntegrity::evaluate(0.879, 0.007, 0.900, 0.50);
        assert_eq!(p2.bottleneck_axis, 1);
        assert_eq!(
            p2.diagnose,
            CollapseProposition::Proposition2ImpotentLucidity
        );
        assert!(p2.arithmetic_mean > 0.50 && p2.leontief_minimum < 0.05);

        let p1_shifted =
            TriadicIntegrity::evaluate_quantile_calibrated(0.35, 0.60, 0.85, [0.50, 0.40, 0.50]);
        assert_eq!(p1_shifted.bottleneck_axis, 0);
        assert_eq!(
            p1_shifted.diagnose,
            CollapseProposition::Proposition1BlindUndersizing
        );

        let epi_state = EpistemicMarginState::for_shift_observation(
            &[vec![20.0, 21.0, 22.0], vec![23.0, 24.0, 25.0]],
            2,
            (5.0, 65.0),
        );
        let vital_state = VitalMarginState::from_heavy_tails_and_asymmetry(
            &[1.0, 2.0, 3.0, 5.0, 8.0],
            2,
            (0.20, 0.80),
            |x| x * x,
        );
        let gov_state = GovernanceState::evaluate_shift_governance(
            1.2, 10.0, 0.0, 6.0, 0.10, true, 0.90, true, true,
        );
        let from_states =
            TriadicIntegrity::from_canonical_states(&epi_state, &vital_state, &gov_state, 0.50);
        assert_eq!(from_states.diagnose, CollapseProposition::IntactTriad);
        assert!(from_states.leontief_minimum >= 0.50);

        let clues = [
            DiagnosticClue {
                prob_given_hypothesis: 0.95,
                prob_given_rival: 0.45,
                clue_observed: true,
                weight_contribution: 15.0,
            },
            DiagnosticClue {
                prob_given_hypothesis: 0.35,
                prob_given_rival: 0.02,
                clue_observed: true,
                weight_contribution: 25.0,
            },
        ];
        let pt = ProcessTracingInference::evaluate(0.25, 10.0, 40.0, &clues);
        assert!(
            pt.posterior_probability > 0.90 && pt.has_refutable_hoop_condition && !pt.hoop_refuted
        );

        // B11: Failing a HoopTest sets hoop_refuted = true and has_refutable_hoop_condition = false
        let failed_hoop = [DiagnosticClue {
            prob_given_hypothesis: 0.95,
            prob_given_rival: 0.45,
            clue_observed: false,
            weight_contribution: 15.0,
        }];
        let pt_failed = ProcessTracingInference::evaluate(0.25, 10.0, 40.0, &failed_hoop);
        assert!(pt_failed.hoop_refuted && !pt_failed.has_refutable_hoop_condition);

        let brakes = ReflexiveBrakeAudit::evaluate(
            &[0.08, 0.15, 0.82, 0.91],
            &[0.0, 0.0, 1.0, 1.0],
            3.0,
            25.0,
            1.0,
            4.0,
            1.5,
            12.0,
            4.0,
        );
        assert!(brakes.all_brakes_active());
        assert!(brakes.leontief_scalar_target_absent);

        // Brake 4: declaring the composite as a scalar target voids the audit, whatever the other three say.
        let degenerated = ReflexiveBrakeAudit::evaluate_with_declaration(
            &[0.08, 0.15, 0.82, 0.91],
            &[0.0, 0.0, 1.0, 1.0],
            3.0,
            25.0,
            1.0,
            4.0,
            1.5,
            12.0,
            4.0,
            true,
        );
        assert!(!degenerated.leontief_scalar_target_absent);
        assert!(!degenerated.all_brakes_active());

        let vol_audit = LeontiefVsMultiplicativeAudit::evaluate(0.50);
        assert!((vol_audit.leontief_volume_3d - 0.125).abs() < 1e-12);
        assert!((vol_audit.multiplicative_volume_3d - 0.033313).abs() < 1e-5);
        assert!((vol_audit.volume_contraction_loss - 0.7335).abs() < 1e-4);
        assert_eq!(vol_audit.pure_leontief_false_collapses, 0);
        assert_eq!(vol_audit.hybrid_gov_product_false_collapses, 180);
        assert_eq!(vol_audit.five_invariant_product_false_collapses, 242);
        assert_eq!(vol_audit.volume_matched_prod5_false_collapses, 192);
        assert_eq!(vol_audit.power_calibrated_prod5_false_collapses, 0);
        assert_eq!(vol_audit.boundary_ruin_total, 36);
        assert_eq!(vol_audit.arithmetic_false_safe_collapses, 36);
        assert!((vol_audit.arithmetic_false_safe_rate - 1.0).abs() < 1e-12);
        assert_eq!(vol_audit.leontief_false_safe_collapses, 0);
        assert_eq!(vol_audit.crossing_grid_leontief_collapses, 243 - 32);
        assert!(
            vol_audit.crossing_grid_arith_false_safe > 100
                && vol_audit.crossing_grid_arith_false_safe < 211
        );

        let ces_comp = TriadicIntegrity::ces_aggregate(0.0, 0.90, 0.90, 0.25);
        let ces_sub = TriadicIntegrity::ces_aggregate(0.0, 0.90, 0.90, 2.0);
        assert_eq!(ces_comp, 0.0);
        assert!(ces_sub > 0.35);

        let falsifiers = PreregisteredFalsifier::canonical_four();
        assert_eq!(falsifiers[0].required_n_eff, 24);
        assert!((falsifiers[0].falsification_critical_d - 0.0661).abs() < 1e-3);
        assert!((falsifiers[0].mayo_corroboration_d - 0.2339).abs() < 1e-3);
        assert_eq!(falsifiers[1].required_n_eff, 19);
        assert!((falsifiers[1].falsification_critical_d - 0.6698).abs() < 1e-3);
        assert_eq!(falsifiers[2].required_n_eff, 20);
        assert!((falsifiers[2].falsification_critical_d - 0.0897).abs() < 1e-3);
        assert_eq!(falsifiers[3].required_n_eff, 17);
        assert!((falsifiers[3].falsification_critical_d - 0.1104).abs() < 1e-3);
        assert!(
            falsifiers
                .iter()
                .all(|f| f.falsification_severity_at_null >= 0.998)
        );

        // B11: Empirical falsifier adjudication on observed data, with NotTestable when empty or preconditions fail
        let preconds_ok = RefutabilityPreconditions {
            multiverse_specs_k: 12,
            cluster_deff: 2.0,
            regimes_m: 24,
            horizon_t: 24,
            units_j: 20,
            proxy_pressure_beta: 0.85,
        };
        assert_eq!(
            falsifiers[0].evaluate_observations(&[], 1.0, &preconds_ok),
            Err(NotTestable::EmptyObservations)
        );
        let preconds_bad = RefutabilityPreconditions {
            multiverse_specs_k: 3,
            cluster_deff: 1.1,
            regimes_m: 5,
            horizon_t: 2,
            units_j: 4,
            proxy_pressure_beta: 0.20,
        };
        assert_eq!(
            falsifiers[0].evaluate_observations(&[0.30; 30], 1.0, &preconds_bad),
            Err(NotTestable::PreconditionNotMet { falsifier_id: 1 })
        );
        let corrob_obs = vec![0.35; 30];
        assert!(matches!(
            falsifiers[0].evaluate_observations(&corrob_obs, 1.0, &preconds_ok),
            Ok(FalsifierVerdict::Corroborated { .. })
        ));
        let refuted_obs = vec![0.01; 30];
        assert!(matches!(
            falsifiers[0].evaluate_observations(&refuted_obs, 1.0, &preconds_ok),
            Ok(FalsifierVerdict::Refuted { .. })
        ));

        let proof_audit = StructuralCollapseProofAudit::evaluate_analytical(
            2_300_000.0,
            0.23,
            -0.05,
            0.08,
            0.12,
            1.40,
            2.0,
            5.5,
            0.60,
            2.0,
            0.85,
            0.40,
        );
        assert!(proof_audit.all_benchmark_conditions_met());
        assert!(proof_audit.prop2_reserve_at_ttr < -50.0);
        assert!((vol_audit.volume_matched_prod_tau - 0.2951).abs() < 1e-4);

        let safe_reserve_audit = StructuralCollapseProofAudit::evaluate_analytical(
            2_300_000.0,
            0.23,
            -0.05,
            0.08,
            0.12,
            1.40,
            9.0,
            5.5,
            0.60,
            2.0,
            0.85,
            0.40,
        );
        assert!(safe_reserve_audit.prop2_reserve_at_ttr > 10.0);
        assert!(!safe_reserve_audit.all_benchmark_conditions_met());

        // S-04: Non-finite axis scores or falsifier designs fail closed (never IntactTriad)
        for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            let t_e = TriadicIntegrity::evaluate(bad, 0.9, 0.9, 0.5);
            let t_v = TriadicIntegrity::evaluate(0.9, bad, 0.9, 0.5);
            let t_g = TriadicIntegrity::evaluate(0.9, 0.9, bad, 0.5);
            assert_eq!(t_e.leontief_minimum, 0.0);
            assert_eq!(
                t_e.diagnose,
                CollapseProposition::Proposition1BlindUndersizing
            );
            assert_eq!(t_v.leontief_minimum, 0.0);
            assert_eq!(
                t_v.diagnose,
                CollapseProposition::Proposition2ImpotentLucidity
            );
            assert_eq!(t_g.leontief_minimum, 0.0);
            assert_eq!(
                t_g.diagnose,
                CollapseProposition::Proposition3CynicalCapture
            );
            assert_eq!(TriadicIntegrity::ces_aggregate(bad, 0.9, 0.9, 0.25), 0.0);
            let bad_fals = PreregisteredFalsifier::design(1, bad, 0.25);
            assert_eq!(bad_fals.required_n_eff, usize::MAX);
            assert_eq!(bad_fals.falsification_severity_at_null, 0.0);
        }
    }
}
