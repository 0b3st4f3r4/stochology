//! **Chapter 9 — Five Methodological Families, Process Tracing, and Reflexive Self-Application.**
//!
//! Completes the disciplinary arc of Stochology (`notes/The Ethics of Aim.md` §II, §VII, §VIII):
//! - The Nuclear Triad and the Three Collapse Propositions (`CollapseProposition`, `TriadicIntegrity`);
//! - Qualitative, historical, legal, and clinical *Process Tracing* with Galenic *sēmeîa*
//!   and Keynes's evidential weight $V$ (`ProcessTracingTestKind`, `ProcessTracingInference`);
//! - Reflexive self-application: the Three Internal Goodhart Brakes and Four Conditions of Refutability (`ReflexiveBrakeAudit`).

use crate::inquiry::cromwell_clamp;

/// The Three Collapse Propositions when any axis of the Nuclear Triad is removed
/// (`notes/The Ethics of Aim.md` §2.2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollapseProposition {
    /// **Collapse Proposition 1 ($\neg\text{Epistemic Margin}$):** *Blind Undersizing* —
    /// sizing reserves on a false point certainty underestimates selection defects and cluster echoes.
    Proposition1BlindUndersizing,
    /// **Collapse Proposition 2 ($\neg\text{Vital Margin}$):** *Impotent Lucidity Before Ruin* —
    /// knowing the exact width of uncertainty while operating at $100\%$ utilization or excess leverage
    /// leads to immediate absorption at the ruin barrier.
    Proposition2ImpotentLucidity,
    /// **Collapse Proposition 3 ($\neg\text{Institutional Governance}$):** *Cynical Capture and Pillaging* —
    /// epistemic doubt is weaponized as an alibi for inaction ("more studies are needed") while common
    /// vital reserves are privatized under limited-liability moral hazard.
    Proposition3CynicalCapture,
    /// All three safeguards are active and above their survival thresholds.
    IntactTriad,
}

/// Evaluates the Leontief structural complementarity of the Nuclear Triad
/// $(S_{\mathrm{epi}}, S_{\mathrm{vital}}, S_{\mathrm{gov}}) \in [0, 1]^3$:
/// $$M_{\mathrm{Leontief}} = \min\bigl(S_{\mathrm{epi}},\, S_{\mathrm{vital}},\, S_{\mathrm{gov}}\bigr)$$
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TriadicIntegrity {
    /// Epistemic margin score $S_{\mathrm{epi}} \in [0, 1]$.
    pub epistemic_margin: f64,
    /// Vital margin score $S_{\mathrm{vital}} \in [0, 1]$.
    pub vital_margin: f64,
    /// Institutional governance score $S_{\mathrm{gov}} \in [0, 1]$.
    pub institutional_governance: f64,
    /// Leontief bottleneck minimum $M_{\mathrm{Leontief}} = \min(S_{\mathrm{epi}}, S_{\mathrm{vital}}, S_{\mathrm{gov}})$.
    pub leontief_minimum: f64,
    /// Active collapse proposition if any axis falls below `min_threshold`.
    pub diagnose: CollapseProposition,
}

impl TriadicIntegrity {
    /// Evaluates the structural integrity of a decision system across the three axes of the Nuclear Triad.
    pub fn evaluate(
        epistemic_margin: f64,
        vital_margin: f64,
        institutional_governance: f64,
        min_threshold: f64,
    ) -> Self {
        let e = epistemic_margin.clamp(0.0, 1.0);
        let v = vital_margin.clamp(0.0, 1.0);
        let g = institutional_governance.clamp(0.0, 1.0);
        let thresh = min_threshold.clamp(0.05, 0.95);
        let diag = if e < thresh && e <= v && e <= g {
            CollapseProposition::Proposition1BlindUndersizing
        } else if v < thresh && v <= e && v <= g {
            CollapseProposition::Proposition2ImpotentLucidity
        } else if g < thresh {
            CollapseProposition::Proposition3CynicalCapture
        } else {
            CollapseProposition::IntactTriad
        };
        Self {
            epistemic_margin: e,
            vital_margin: v,
            institutional_governance: g,
            leontief_minimum: e.min(v).min(g),
            diagnose: diag,
        }
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
    /// True if at least one evaluated clue was capable of refuting the hypothesis (a Hoop or Doubly Decisive test).
    pub has_refutable_hoop_condition: bool,
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

        for clue in clues {
            let kind = clue.classify();
            if matches!(
                kind,
                ProcessTracingTestKind::HoopTest | ProcessTracingTestKind::DoublyDecisive
            ) {
                has_hoop = true;
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
            has_refutable_hoop_condition: has_hoop,
        }
    }
}

/// Reflexive self-application of Stochology under Goodhart's Law (`notes/The Ethics of Aim.md` §7.2):
/// audits the Three Internal Goodhart Brakes that prevent each axis of the triad from degenerating
/// when turned into an isolated bureaucratic target.
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
    /// **Brake 3 (Institutional Governance):** Emergency fast-path response time $\tau_{\mathrm{fast}} \le \mathrm{TTS}$
    /// vs. procedural-theater delay $\tau_{\mathrm{bureaucratic}} > \mathrm{TTS}$.
    pub emergency_fast_path_time: f64,
    /// Slow procedural-theater audit duration.
    pub procedural_theater_time: f64,
    /// Physical Time-to-Survive $\mathrm{TTS}$.
    pub emergency_tts: f64,
}

impl ReflexiveBrakeAudit {
    /// Evaluates the three internal Goodhart brakes on a candidate forecast, reserve level, and emergency clock.
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate(
        sharp_forecasts: &[f64],
        outcomes: &[f64],
        reserve_level: f64,
        hoarding_reserve_level: f64,
        urgent_present_marginal_value: f64,
        precautionary_benefit_scale: f64,
        emergency_fast_path_time: f64,
        procedural_theater_time: f64,
        emergency_tts: f64,
    ) -> Self {
        let corp_sharp =
            crate::performativity::CorpPavDecomposition::evaluate(sharp_forecasts, outcomes);
        let y_bar = crate::rng::mean(outcomes);
        let lazy_forecasts = vec![y_bar; outcomes.len()];
        let corp_lazy =
            crate::performativity::CorpPavDecomposition::evaluate(&lazy_forecasts, outcomes);

        // Concave precautionary benefit B(R) = scale * ln(1 + R) minus linear opportunity cost of starving present need c * R
        let eval_welfare = |r: f64| -> f64 {
            precautionary_benefit_scale * (1.0 + r.max(0.0)).ln()
                - urgent_present_marginal_value * r.max(0.0)
        };

        Self {
            sharp_resolution: corp_sharp.resolution,
            skeptical_inflation_resolution: corp_lazy.resolution,
            optimal_buffer_social_welfare: eval_welfare(reserve_level),
            regressive_hoarding_welfare: eval_welfare(hoarding_reserve_level),
            emergency_fast_path_time,
            procedural_theater_time,
            emergency_tts,
        }
    }

    /// True if all three internal Goodhart brakes hold:
    /// 1. $\mathrm{RES}_{\mathrm{sharp}} > \mathrm{RES}_{\mathrm{lazy}} = 0$;
    /// 2. Optimal vital reserve welfare exceeds regressive hoarding welfare;
    /// 3. Emergency Maximin fast-path acts within $\mathrm{TTS}$ while procedural theater would breach $\mathrm{TTS}$.
    pub fn all_brakes_active(&self) -> bool {
        self.sharp_resolution > 0.01
            && self.skeptical_inflation_resolution < 1e-12
            && self.optimal_buffer_social_welfare > self.regressive_hoarding_welfare
            && self.emergency_fast_path_time <= self.emergency_tts
            && self.procedural_theater_time > self.emergency_tts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn triadic_integrity_diagnoses_all_three_collapse_propositions() {
        let c1 = TriadicIntegrity::evaluate(0.05, 0.90, 0.90, 0.50);
        assert_eq!(
            c1.diagnose,
            CollapseProposition::Proposition1BlindUndersizing
        );
        let c2 = TriadicIntegrity::evaluate(0.90, 0.05, 0.90, 0.50);
        assert_eq!(
            c2.diagnose,
            CollapseProposition::Proposition2ImpotentLucidity
        );
        let c3 = TriadicIntegrity::evaluate(0.90, 0.90, 0.05, 0.50);
        assert_eq!(c3.diagnose, CollapseProposition::Proposition3CynicalCapture);
        let ok = TriadicIntegrity::evaluate(0.88, 0.85, 0.91, 0.50);
        assert_eq!(ok.diagnose, CollapseProposition::IntactTriad);
    }

    #[test]
    fn process_tracing_and_reflexive_brakes_hold() {
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
        assert!(pt.posterior_probability > 0.90);
        assert!(pt.posterior_keynes_weight_v > pt.initial_keynes_weight_v);
        assert!(pt.has_refutable_hoop_condition);

        let brakes = ReflexiveBrakeAudit::evaluate(
            &[0.1, 0.2, 0.8, 0.9],
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
    }
}
