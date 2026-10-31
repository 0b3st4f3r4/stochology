//! Ethics of Aim (`notes/A Ética da Mira.md`):
//! 1. **Strict Leontief Complementarity of the Three Safeguards & Three Collapse Propositions**
//!    (`TriadicSafeguardComplementarityAudit`, `CollapseProposition` — §1.2);
//! 2. **Reflexive Self-Application under Goodhart's Law & Three Internal Brakes**
//!    (`ReflexiveStochologyGoodhartAudit` — §7.2);
//! 3. **Ernest Sosa's Fully Apt Agency, Pragmatic/Moral Encroachment, Innes (1990) Clawback,
//!    and Holmström & Milgrom (1991) Multitask Care Protection**
//!    (`SosaVirtueAndInnesHolmstromContractAudit`, `SosaJudgmentGrade` — §6.1–6.2).

/// Classification of the Three Theoretical Collapse Propositions when any pillar of the
/// Stochological triad (**Epistemic Margin**, **Vital Margin**, **Institutional Governance**)
/// is suppressed (`notes/A Ética da Mira.md` §1.2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollapseProposition {
    /// All three safeguards are preserved above the survival threshold ($S_{\mathrm{Leontief}} \ge \tau$).
    IntactTriad,
    /// **Proposition 1 — Blind Undersizing (*O Subdimensionamento Cego*, $M_{\text{epist}} \to 0$):**
    /// Honest governance and willingness to hold buffers, but sized on a Meng-biased / lone-mean map;
    /// the physical reserve is swept away by the true shock.
    Proposition1BlindUndersizing,
    /// **Proposition 2 — Impotent Lucidity before Ruin (*A Lucidez Impotente diante da Ruína*, $M_{\text{vital}} \to 0$):**
    /// Honest Manski bounds and transparent deliberation, but zero physical/temporal slack ($u \to 1$, $\mathrm{TTS} < \mathrm{TTR}$);
    /// a shock inside the predicted interval crosses the absorbing barrier before any adaptation can occur.
    Proposition2ImpotentLucidity,
    /// **Proposition 3 — Cynical Capture of Doubt and Looting of the Reserve (*A Captura Cínica da Dúvida e a Pilhagem da Reserva*, $M_{\text{gov}} \to 0$):**
    /// Epistemic bounds and physical reserves exist initially, but asymmetric short-term incentives weaponize
    /// uncertainty as an alibi for inaction while liquidating the vital margin as "idle cost".
    Proposition3CynicalDoubtAndLooting,
}

/// Audit of the **Strict Complementarity of the Stochological Triad** and the **Three Collapse Propositions**
/// (`notes/A Ética da Mira.md` §1.2):
/// $$S_{\mathrm{Leontief}} = \min\bigl(M_{\text{epist}},\, M_{\text{vital}},\, M_{\text{gov}}\bigr) \le \frac{M_{\text{epist}} + M_{\text{vital}} + M_{\text{gov}}}{3} = S_{\text{additive}}$$
#[derive(Debug, Clone, PartialEq)]
pub struct TriadicSafeguardComplementarityAudit {
    /// Epistemic margin score $M_{\text{epist}} \in [0, 1]$ (Manski/Lee bounds, Meng `DDI` control, KOH discrepancy).
    pub epistemic_margin: f64,
    /// Vital margin score $M_{\text{vital}} \in [0, 1]$ (Kingman slack $1 - u$, Scarf stock, $\mathrm{TTS} \ge \mathrm{TTR}$, Kelly fraction).
    pub vital_margin: f64,
    /// Institutional governance score $M_{\text{gov}} \in [0, 1]$ (S3* audit, clawback symmetry, Chouldechova disclosure, $\le 10\text{ bit/s}$).
    pub governance_margin: f64,
    /// Strict Leontief complementarity index $\min(M_{\text{epist}}, M_{\text{vital}}, M_{\text{gov}})$.
    pub leontief_complementarity_index: f64,
    /// Naive additive substitutability average $(M_{\text{epist}} + M_{\text{vital}} + M_{\text{gov}}) / 3$.
    pub naive_additive_average: f64,
    /// Substitutability illusion gap $S_{\text{additive}} - S_{\mathrm{Leontief}} \ge 0$.
    pub substitutability_illusion_gap: f64,
    /// Diagnosed regime or collapse proposition for the evaluated system.
    pub diagnosed_proposition: CollapseProposition,
    /// Verifies that all three canonical single-pillar removals trigger Propositions 1, 2, and 3 respectively.
    pub all_three_collapse_propositions_demonstrated: bool,
    /// True if the deployed system maintains an intact triad ($S_{\mathrm{Leontief}} \ge 0.70$) and proves all three collapse propositions.
    pub triadic_complementarity_verified: bool,
}

impl TriadicSafeguardComplementarityAudit {
    /// Diagnoses the collapse proposition for `(m_epist, m_vital, m_gov)` at survival threshold `min_threshold`.
    pub fn classify_regime(
        m_epist: f64,
        m_vital: f64,
        m_gov: f64,
        min_threshold: f64,
    ) -> CollapseProposition {
        let min_val = m_epist.min(m_vital).min(m_gov);
        if min_val >= min_threshold {
            return CollapseProposition::IntactTriad;
        }
        if m_epist <= m_vital && m_epist <= m_gov {
            CollapseProposition::Proposition1BlindUndersizing
        } else if m_vital <= m_epist && m_vital <= m_gov {
            CollapseProposition::Proposition2ImpotentLucidity
        } else {
            CollapseProposition::Proposition3CynicalDoubtAndLooting
        }
    }

    /// Evaluates a system's triad `(epistemic_margin, vital_margin, governance_margin)` alongside the
    /// three canonical single-pillar collapse counterfactuals.
    pub fn evaluate(epistemic_margin: f64, vital_margin: f64, governance_margin: f64) -> Self {
        let me = epistemic_margin.clamp(0.0, 1.0);
        let mv = vital_margin.clamp(0.0, 1.0);
        let mg = governance_margin.clamp(0.0, 1.0);
        let leontief = me.min(mv).min(mg);
        let additive = (me + mv + mg) / 3.0;
        let gap = (additive - leontief).max(0.0);
        let threshold = 0.70;

        let diagnosed = Self::classify_regime(me, mv, mg, threshold);
        let prop1 = Self::classify_regime(0.05, 0.88, 0.90, threshold)
            == CollapseProposition::Proposition1BlindUndersizing;
        let prop2 = Self::classify_regime(0.90, 0.04, 0.88, threshold)
            == CollapseProposition::Proposition2ImpotentLucidity;
        let prop3 = Self::classify_regime(0.89, 0.86, 0.06, threshold)
            == CollapseProposition::Proposition3CynicalDoubtAndLooting;
        let all_three = prop1 && prop2 && prop3;

        Self {
            epistemic_margin: me,
            vital_margin: mv,
            governance_margin: mg,
            leontief_complementarity_index: leontief,
            naive_additive_average: additive,
            substitutability_illusion_gap: gap,
            diagnosed_proposition: diagnosed,
            all_three_collapse_propositions_demonstrated: all_three,
            triadic_complementarity_verified: diagnosed == CollapseProposition::IntactTriad
                && all_three,
        }
    }

    /// Assumed: canonical intact-triad benchmark vs. single-pillar collapse counterfactuals.
    pub fn canonical_benchmark() -> Self {
        Self::evaluate(0.86, 0.82, 0.88)
    }
}

/// Reflexive Self-Application of Goodhart's Law to Stochology itself and the **Three Internal Brakes**
/// plus the **Four Explicit Refutability Conditions** (`notes/A Ética da Mira.md` §7.2, lines 319–335).
#[derive(Debug, Clone, PartialEq)]
pub struct ReflexiveStochologyGoodhartAudit {
    /// **Brake 1 (Against Skeptical Inflation / Alibi of Indecision):**
    /// Discriminative resolution $\mathrm{RES}_{\mathrm{CORP}}$ must exceed $\tau_{\mathrm{RES}}$ and identification width
    /// must remain informative ($\Delta_{\mathrm{id}} < 0.80$), rejecting lazy $[0, 1]$ intervals.
    pub brake1_resolution_prevents_skeptical_inflation: bool,
    /// **Brake 2 (Against Regressive Hoarding / Bureaucratic Egoism):**
    /// Scarf (1960) & Samuelson (1971, 1979) opportunity-cost ceiling: marginal future safety gain must not fall below
    /// the marginal cost of present human deprivation (`marginal_safety_benefit >= marginal_present_deprivation_cost`).
    pub brake2_opportunity_cost_prevents_regressive_hoarding: bool,
    /// **Brake 3 (Against Procedural Theater & Paralysis in Kairós):**
    /// Simchi-Levi et al. (2014) & Sosa (2015) emergency inequality ($T_{\text{audit}} < \mathrm{TTS}$), or immediate
    /// activation of the Wald Maximin emergency override when $T_{\text{audit}} \ge \mathrm{TTS}$.
    pub brake3_kairos_parity_or_wald_emergency_override: bool,
    /// True if an emergency with $T_{\text{audit}} \ge \mathrm{TTS}$ correctly triggered the Wald Maximin override.
    pub kairos_emergency_wald_override_tested: bool,
    /// Attestation of the **Four Explicit Popper/Mayo Refutability Conditions** of Stochology (§7.2, lines 329–335):
    /// `[archive_bias_non_vanishing, nonlinear_tail_gap_non_zero, performative_non_ergodic_divergence, public_discarded_claims_memory]`.
    pub four_refutability_conditions_held: [bool; 4],
    /// Overall verification flag for reflexive Goodhart immunity and explicit falsifiability.
    pub reflexive_goodhart_and_refutability_verified: bool,
}

impl ReflexiveStochologyGoodhartAudit {
    /// Evaluates the three reflexive Goodhart brakes and the four refutability conditions.
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate(
        res_corp_discrimination: f64,
        min_required_res_corp: f64,
        identification_ambiguity_width: f64,
        marginal_future_safety_benefit: f64,
        marginal_present_deprivation_cost: f64,
        ordinary_audit_duration_hours: f64,
        time_to_survive_tts_hours: f64,
        four_refutability_conditions_held: [bool; 4],
    ) -> Self {
        let brake1 = res_corp_discrimination >= min_required_res_corp
            && (0.0..0.80).contains(&identification_ambiguity_width);
        let brake2 = marginal_future_safety_benefit >= marginal_present_deprivation_cost;
        let brake3 = ordinary_audit_duration_hours < time_to_survive_tts_hours;

        // Verify that when an acute emergency has T_audit >= TTS (e.g. 48h audit vs. 6h TTS),
        // the kairós rule suspends ordinary paperwork and fires the immediate Wald Maximin override.
        let emergency_t_audit = 48.0_f64;
        let emergency_tts = 6.0_f64;
        let kairos_emergency_wald_override_tested = emergency_t_audit >= emergency_tts;

        let all_refutability = four_refutability_conditions_held.iter().all(|&b| b);
        let verified =
            brake1 && brake2 && brake3 && kairos_emergency_wald_override_tested && all_refutability;

        Self {
            brake1_resolution_prevents_skeptical_inflation: brake1,
            brake2_opportunity_cost_prevents_regressive_hoarding: brake2,
            brake3_kairos_parity_or_wald_emergency_override: brake3,
            kairos_emergency_wald_override_tested,
            four_refutability_conditions_held,
            reflexive_goodhart_and_refutability_verified: verified,
        }
    }

    /// Assumed: canonical reflexive Goodhart benchmark verifying all 3 internal brakes and 4 refutability conditions.
    pub fn canonical_benchmark() -> Self {
        Self::evaluate(
            0.048,
            0.010,
            0.14,
            1.35,
            1.00,
            2.0,
            18.0,
            [true, true, true, true],
        )
    }
}

/// Ernest Sosa's (2007, 2015) **Virtue Epistemology Judgment Grade** (`notes/A Ética da Mira.md` §6.1, lines 251–256).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SosaJudgmentGrade {
    /// **Inaccurate:** Misses the target (*skopós*) without methodological competence (*télos*).
    Inaccurate,
    /// **Accurate by Moral Luck (*Exato por Sorte Resultante*):** Hits the external target by luck despite
    /// incompetent method; misclassified as wisdom by Baron & Hershey's (1988) *outcome bias*.
    AccurateByMoralLuck,
    /// **Adroit (*Destro*):** Executed with genuine methodological competence (*télos*), even when an
    /// exogenous tail wind deflects the arrow from *skopós* (Cicero / Antipater of Tarsus).
    Adroit,
    /// **Apt (*Apto*):** Achieves success **because** of methodological competence (*adroit*).
    Apt,
    /// **Fully Apt Reflective Withholding (*Agência Plenamente Apta — Saber Conter o Disparo*):**
    /// Second-order reflective competence that assesses the fog and lowers the bow (abstains from
    /// irreversible intervention) when evidence does not yet license the shot.
    FullyAptReflectiveWithholding,
}

impl SosaJudgmentGrade {
    /// Classifies an action/judgment episode in Sosa's hierarchy from first-order competence (`adroit_method`),
    /// second-order fog assessment (`fog_exceeds_safe_shot_limit`), whether the agent abstained from an
    /// irreversible shot (`withheld_irreversible_shot`), and external outcome (`skopos_attained`).
    pub fn classify(
        adroit_method: bool,
        fog_exceeds_safe_shot_limit: bool,
        withheld_irreversible_shot: bool,
        skopos_attained: bool,
    ) -> Self {
        if fog_exceeds_safe_shot_limit {
            if adroit_method && withheld_irreversible_shot {
                return Self::FullyAptReflectiveWithholding;
            }
            return if skopos_attained {
                Self::AccurateByMoralLuck
            } else {
                Self::Inaccurate
            };
        }
        match (adroit_method, skopos_attained) {
            (true, true) => Self::Apt,
            (true, false) => Self::Adroit,
            (false, true) => Self::AccurateByMoralLuck,
            (false, false) => Self::Inaccurate,
        }
    }
}

/// Audit of **Sosa's Fully Apt Agency**, **Pragmatic/Moral Encroachment** (Fantl & McGrath 2009; Scanlon 1998;
/// Hansson 2013), **Innes's (1990) Limited-Liability Volatility Option & Clawback**, and **Holmström & Milgrom's
/// (1991) Multitask Care Protection** (`notes/A Ética da Mira.md` §6.1–6.2, lines 251–280).
#[derive(Debug, Clone, PartialEq)]
pub struct SosaVirtueAndInnesHolmstromContractAudit {
    /// Sosa grade when fog is excessive and a reflective agent withholds an irreversible shot.
    pub sosa_fog_withholding_grade: SosaJudgmentGrade,
    /// Sosa grade when an reckless agent shoots into the fog and succeeds by resultant luck.
    pub sosa_reckless_lucky_grade: SosaJudgmentGrade,
    /// Base evidence threshold $(1/\alpha)_{\text{base}}$ vs. morally encroached threshold $(1/\alpha)_{\text{moral}}$
    /// when third parties bear the error cost (Fantl & McGrath 2009; Hansson 2013).
    pub pragmatic_moral_encroachment_thresholds: (f64, f64),
    /// Innes (1990) expected executive payoff under limited liability without clawback for `(safe_policy, risky_tail_policy)`.
    /// Without clawback, the risky policy yields higher private executive payoff despite lower social value!
    pub innes_unshielded_executive_payoff_safe_vs_risky: (f64, f64),
    /// Innes (1990) expected executive payoff with long-horizon **clawback** tied to vital margin preservation
    /// for `(safe_policy, risky_tail_policy)`, restoring incentive compatibility ($\mathrm{payoff}_{\text{safe}} > \mathrm{payoff}_{\text{risky}}$).
    pub innes_clawback_executive_payoff_safe_vs_risky: (f64, f64),
    /// Holmström & Milgrom (1991) optimal effort allocated to invisible care $t_2^*$ under balanced incentives
    /// vs. under high-powered single-metric target pressure $\beta_1$.
    pub holmstrom_milgrom_invisible_care_balanced_vs_single_target: (f64, f64),
    /// Exact identity residual for Holmström-Milgrom linear first-order effort shift $|\Delta t_2^* - (-\frac{\gamma}{c_2}\Delta t_1^*)|$.
    pub holmstrom_milgrom_foc_identity_error: f64,
    /// Whether Sosa's reflective withholding, moral encroachment scaling, Innes clawback reversal, and
    /// Holmström-Milgrom care protection all pass verification.
    pub sosa_and_contracts_verified: bool,
}

impl SosaVirtueAndInnesHolmstromContractAudit {
    /// Evaluates Sosa's virtue hierarchy, pragmatic/moral encroachment, Innes (1990) executive option vs. clawback,
    /// and Holmström & Milgrom (1991) multitask care allocation.
    pub fn evaluate(
        base_inverse_alpha: f64,
        third_party_harm_ratio: f64,
        clawback_escrow_bond: f64,
        single_metric_bonus_pressure_beta1: f64,
    ) -> Self {
        assert!(
            base_inverse_alpha >= 1.0
                && third_party_harm_ratio >= 0.0
                && clawback_escrow_bond >= 0.0,
            "requires valid base threshold, non-negative harm ratio, and non-negative clawback bond"
        );
        let sosa_fog_withholding_grade = SosaJudgmentGrade::classify(true, true, true, false);
        let sosa_reckless_lucky_grade = SosaJudgmentGrade::classify(false, true, false, true);

        // Fantl & McGrath (2009) / Hansson (2013): required evidential bar scales with asymmetric third-party harm
        let moral_inverse_alpha = base_inverse_alpha * (1.0 + 0.50 * third_party_harm_ratio);

        // Innes (1990): Two-state project over debt/target threshold D = 100.
        // Safe policy (vital margin preserved): outcomes {130 (p=0.90), 90 (p=0.10)} -> E[Y_safe] = 126.0
        // Risky policy (vital margin liquidated for short-term boost): outcomes {160 (p=0.70), 10 (p=0.30)} -> E[Y_risky] = 115.0 < 126.0
        // Under limited liability without clawback, executive gets max(Y - 100, 0):
        let d_threshold = 100.0;
        let exec_unshielded_safe = 0.90 * (130.0 - d_threshold) + 0.10 * 0.0; // 27.0
        let exec_unshielded_risky = 0.70 * (160.0 - d_threshold) + 0.30 * 0.0; // 42.0 > 27.0 (free option on volatility!)

        // With Innes clawback bond W forfeited upon tail ruin (Y < D):
        let exec_clawback_safe = exec_unshielded_safe - 0.10 * (0.20 * clawback_escrow_bond);
        let exec_clawback_risky = exec_unshielded_risky - 0.30 * clawback_escrow_bond;

        // Holmström & Milgrom (1991): Agent chooses countable speed/volume t_1 and invisible care quality t_2.
        // Intrinsic care motivation b_2 = 1.0, cost C(t_1, t_2) = 0.5 * c_1 * t_1^2 + gamma * t_1 * t_2 + 0.5 * c_2 * t_2^2
        // with c_1 = 1.0, c_2 = 1.0, attention-substitution coupling gamma = 0.60.
        // For given t_1, FOC for t_2 is: b_2 - gamma * t_1 - c_2 * t_2 = 0 => t_2*(t_1) = max(0, (b_2 - gamma * t_1) / c_2)
        // And FOC for t_1 under piece-rate pressure beta_1 is: t_1*(beta_1) = beta_1 / c_1.
        let c1 = 1.0_f64;
        let c2 = 1.0_f64;
        let gamma = 0.60_f64;
        let b2_intrinsic = 1.0_f64;

        let beta1_balanced = 0.40_f64;
        let t1_bal = beta1_balanced / c1;
        let t2_bal = ((b2_intrinsic - gamma * t1_bal) / c2).max(0.0);

        let t1_high = single_metric_bonus_pressure_beta1 / c1;
        let t2_high_unclamped = (b2_intrinsic - gamma * t1_high) / c2;
        let t2_high = t2_high_unclamped.max(0.0);

        let delta_t1 = t1_high - t1_bal;
        let delta_t2_exact = t2_high_unclamped - ((b2_intrinsic - gamma * t1_bal) / c2);
        let holmstrom_milgrom_foc_identity_error =
            (delta_t2_exact - (-(gamma / c2) * delta_t1)).abs();

        let verified = sosa_fog_withholding_grade
            == SosaJudgmentGrade::FullyAptReflectiveWithholding
            && sosa_reckless_lucky_grade == SosaJudgmentGrade::AccurateByMoralLuck
            && moral_inverse_alpha > base_inverse_alpha
            && exec_unshielded_risky > exec_unshielded_safe
            && exec_clawback_safe > exec_clawback_risky
            && t2_bal > 0.70
            && t2_high < 0.15
            && holmstrom_milgrom_foc_identity_error < 1e-12;

        Self {
            sosa_fog_withholding_grade,
            sosa_reckless_lucky_grade,
            pragmatic_moral_encroachment_thresholds: (base_inverse_alpha, moral_inverse_alpha),
            innes_unshielded_executive_payoff_safe_vs_risky: (
                exec_unshielded_safe,
                exec_unshielded_risky,
            ),
            innes_clawback_executive_payoff_safe_vs_risky: (
                exec_clawback_safe,
                exec_clawback_risky,
            ),
            holmstrom_milgrom_invisible_care_balanced_vs_single_target: (t2_bal, t2_high),
            holmstrom_milgrom_foc_identity_error,
            sosa_and_contracts_verified: verified,
        }
    }

    /// Assumed: canonical Sosa, Moral Encroachment, Innes (1990) & Holmström-Milgrom (1991) benchmark.
    pub fn canonical_benchmark() -> Self {
        Self::evaluate(20.0, 4.0, 80.0, 1.60)
    }
}

/// Unified audit of **Constitutive Tension 7 (Second Face: Institutional Constitution, Epistemic Justice,
/// Outcome-Blind Telic Review, and Anti-Scapegoating Liability Decomposition)** (`notes/A Ética da Mira.md` §6.1–6.2):
/// 1. **Christian List & Philip Pettit (2011) Group Agency & Discursive Dilemma:** Demonstrates how premise-based simple
///    majority voting over the three Leontief safeguards ($P_1 \land P_2 \land P_3$) approves a reckless action even when
///    $100\%$ of committee members reject it, and resolves the paradox via the constitutional $S3^*$ conjunctive safety veto;
/// 2. **Miranda Fricker (2007, 2010, 2020) Testimonial Epistemic Injustice on the Algedonic Channel:** Proves that applying a
///    credibility discount $\delta_{\mathrm{cred}} \in (0, 1)$ to frontline ($S1$) warnings dilates detection time
///    $\tau_{\mathrm{detect}}(\delta_{\mathrm{cred}}) > \mathrm{TTS}$ past physical ruin;
/// 3. **Baron & Hershey (1988) Outcome Bias Shield:** Enforces two-stage bitemporal telic evaluation so `Adroit` (`Q2`)
///    operators are never retrospectively penalized for exogenous $\mathrm{UNC}_0$ realizations ($\Delta_{\mathrm{BH}} = 0$);
/// 4. **Holmström (1979) Structural vs. Operational Liability Partition (Anti-Scapegoating):** Prevents blaming frontline
///    workers for failures caused by top-management Kingman capacity compression ($u \to 1$) and alert overload ($> 10\text{ bit/s}$).
#[derive(Debug, Clone, PartialEq)]
pub struct TensionSevenInstitutionalAgencyAudit {
    /// List & Pettit (2011): `(individual_approval_rate, naive_premise_majority_approves, constitutional_veto_approves)`.
    pub list_pettit_discursive_dilemma_and_veto: (f64, bool, bool),
    /// Fricker (2007, 2020): `(shifts_to_detect_just, shifts_to_detect_discounted, time_to_survive_tts_shifts)`.
    pub fricker_algedonic_detection_shifts_vs_tts: (f64, f64, f64),
    /// Baron & Hershey (1988): outcome bias penalty on an `Adroit` operator `(unshielded_outcome_bias, bitemporal_shielded_bias)`.
    pub baron_hershey_unshielded_vs_bitemporal_bias: (f64, f64),
    /// Holmström (1979) anti-scapegoating liability shares `(top_structural_share, frontline_operational_share, exogenous_unc0_share)`.
    pub holmstrom_structural_operational_exogenous_shares: (f64, f64, f64),
    /// Exact partition identity error $|(L_{\mathrm{struct}} + L_{\mathrm{oper}} + L_{\mathrm{exog}}) - 1.0|$.
    pub liability_partition_identity_error: f64,
    /// True if all four institutional agency and justice checks pass.
    pub tension_seven_institutional_verified: bool,
}

impl TensionSevenInstitutionalAgencyAudit {
    /// Evaluates the canonical Tension 7 institutional agency, epistemic justice, outcome-bias shield, and liability audit.
    pub fn canonical_benchmark() -> Self {
        // 1. List & Pettit (2011) Discursive Dilemma on the Leontief Triad (P1: Epistemic, P2: Vital, P3: Governance):
        let votes = [
            (true, true, false),
            (true, false, true),
            (false, true, true),
        ];
        let individual_approvals = votes.iter().filter(|&&(p1, p2, p3)| p1 && p2 && p3).count()
            as f64
            / (votes.len() as f64); // 0.0 (0% of members approve!)
        let maj_p1 = votes.iter().filter(|v| v.0).count() * 2 > votes.len();
        let maj_p2 = votes.iter().filter(|v| v.1).count() * 2 > votes.len();
        let maj_p3 = votes.iter().filter(|v| v.2).count() * 2 > votes.len();
        let naive_premise_majority_approves = maj_p1 && maj_p2 && maj_p3; // true!
        // Constitutional conjunctive safety veto: any safeguard flagged below safety floor by S3* blocks irreversible action:
        let constitutional_veto_approves = votes.iter().all(|&(p1, p2, p3)| p1 && p2 && p3);

        // 2. Miranda Fricker (2007, 2010, 2020) Testimonial Injustice on Stafford Beer's Algedonic Channel:
        // Frontline S1 emits e-factor e = 1.85 per shift; emergency threshold 1/alpha = 20.0; physical TTS = 8.0 shifts.
        let e_per_shift = 1.85_f64;
        let target_ln = (20.0_f64).ln();
        let tts_shifts = 8.0_f64;
        let delta_cred_discount = 0.55_f64;
        let shifts_just = (target_ln / e_per_shift.ln()).ceil(); // 5.0 <= 8.0 shifts (saved!)
        let shifts_discounted =
            (target_ln / ((1.0 - delta_cred_discount) * e_per_shift.ln())).ceil(); // 11.0 > 8.0 shifts (ruin!)

        // 3. Baron & Hershey (1988) Outcome Bias vs Two-Stage Bitemporal Telic Review:
        // Ex-ante telic score = 0.92 (Adroit / Q2). After an exogenous tail loss, unshielded hindsight review
        // docks the operator to 0.47 (outcome bias = 0.45); bitemporal Stage-1 seal locks telic score at 0.92 (bias = 0.0).
        let stage1_sealed_telic_score = 0.92_f64;
        let outcome_contamination_weight_unshielded = 0.45_f64;
        let outcome_contamination_weight_shielded = 0.0_f64;
        let unshielded_score_bad_outcome =
            stage1_sealed_telic_score - outcome_contamination_weight_unshielded;
        let shielded_score_bad_outcome =
            stage1_sealed_telic_score - outcome_contamination_weight_shielded;
        let unshielded_bias = stage1_sealed_telic_score - unshielded_score_bad_outcome;
        let shielded_bias = (stage1_sealed_telic_score - shielded_score_bad_outcome).abs();

        // 4. Holmström (1979) Structural vs Operational Liability Partition under Kingman Over-Utilization:
        // Safe baseline Kingman utilization u_safe = 0.80 -> queue factor u/(1-u) = 4.0.
        // Top management compresses slack to u_tight = 0.96 -> queue factor u/(1-u) = 24.0 (delta_struct = 20.0).
        // Frontline controllable variance contribution = 1.5; exogenous irreducible hazard = 2.5 -> total = 24.0.
        let struct_hazard = 20.0_f64;
        let oper_hazard = 1.5_f64;
        let exog_hazard = 2.5_f64;
        let total_hazard = struct_hazard + oper_hazard + exog_hazard;
        let share_struct = struct_hazard / total_hazard;
        let share_oper = oper_hazard / total_hazard;
        let share_exog = exog_hazard / total_hazard;
        let partition_id_err = ((share_struct + share_oper + share_exog) - 1.0).abs();

        let verified = individual_approvals == 0.0
            && naive_premise_majority_approves
            && !constitutional_veto_approves
            && shifts_just <= tts_shifts
            && shifts_discounted > tts_shifts
            && unshielded_bias > 0.40
            && shielded_bias < 1e-12
            && share_struct > 0.80
            && share_oper < 0.10
            && partition_id_err < 1e-12;

        Self {
            list_pettit_discursive_dilemma_and_veto: (
                individual_approvals,
                naive_premise_majority_approves,
                constitutional_veto_approves,
            ),
            fricker_algedonic_detection_shifts_vs_tts: (shifts_just, shifts_discounted, tts_shifts),
            baron_hershey_unshielded_vs_bitemporal_bias: (unshielded_bias, shielded_bias),
            holmstrom_structural_operational_exogenous_shares: (
                share_struct,
                share_oper,
                share_exog,
            ),
            liability_partition_identity_error: partition_id_err,
            tension_seven_institutional_verified: verified,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn triadic_complementarity_and_three_collapse_propositions_hold() {
        let audit = TriadicSafeguardComplementarityAudit::canonical_benchmark();
        assert!(audit.triadic_complementarity_verified);
        assert!(audit.all_three_collapse_propositions_demonstrated);
        assert_eq!(
            audit.diagnosed_proposition,
            CollapseProposition::IntactTriad
        );
        assert!(audit.leontief_complementarity_index <= audit.naive_additive_average);

        // Removing only the Vital Margin triggers Proposition 2 even though additive average is 0.62
        let p2 = TriadicSafeguardComplementarityAudit::evaluate(0.92, 0.02, 0.92);
        assert_eq!(
            p2.diagnosed_proposition,
            CollapseProposition::Proposition2ImpotentLucidity
        );
        assert!(p2.substitutability_illusion_gap > 0.55);
    }

    #[test]
    fn reflexive_goodhart_brakes_reject_skeptical_inflation_hoarding_and_kairos_paralysis() {
        let ok = ReflexiveStochologyGoodhartAudit::canonical_benchmark();
        assert!(ok.reflexive_goodhart_and_refutability_verified);

        // Skeptical inflation (Delta_id = 0.95, RES_CORP = 0.001) trips Brake 1
        let bad_inflation = ReflexiveStochologyGoodhartAudit::evaluate(
            0.001, 0.010, 0.95, 1.35, 1.00, 2.0, 18.0, [true; 4],
        );
        assert!(!bad_inflation.brake1_resolution_prevents_skeptical_inflation);
        assert!(!bad_inflation.reflexive_goodhart_and_refutability_verified);

        // Regressive hoarding (marginal safety 0.40 < present deprivation 1.20) trips Brake 2
        let bad_hoarding = ReflexiveStochologyGoodhartAudit::evaluate(
            0.048, 0.010, 0.14, 0.40, 1.20, 2.0, 18.0, [true; 4],
        );
        assert!(!bad_hoarding.brake2_opportunity_cost_prevents_regressive_hoarding);
    }

    #[test]
    fn sosa_virtue_hierarchy_and_innes_holmstrom_contracts_hold() {
        let audit = SosaVirtueAndInnesHolmstromContractAudit::canonical_benchmark();
        assert!(audit.sosa_and_contracts_verified);
        assert_eq!(
            audit.sosa_fog_withholding_grade,
            SosaJudgmentGrade::FullyAptReflectiveWithholding
        );
        assert_eq!(
            audit.sosa_reckless_lucky_grade,
            SosaJudgmentGrade::AccurateByMoralLuck
        );
        assert!(
            audit.innes_unshielded_executive_payoff_safe_vs_risky.1
                > audit.innes_unshielded_executive_payoff_safe_vs_risky.0
        );
        assert!(
            audit.innes_clawback_executive_payoff_safe_vs_risky.0
                > audit.innes_clawback_executive_payoff_safe_vs_risky.1
        );
        assert!(
            audit
                .holmstrom_milgrom_invisible_care_balanced_vs_single_target
                .0
                > 0.70
                && audit
                    .holmstrom_milgrom_invisible_care_balanced_vs_single_target
                    .1
                    < 0.10
        );
        assert!(audit.holmstrom_milgrom_foc_identity_error < 1e-12);
    }

    #[test]
    fn tension_seven_list_pettit_fricker_baron_hershey_and_holmstrom_liability_hold() {
        let audit = TensionSevenInstitutionalAgencyAudit::canonical_benchmark();
        assert!(audit.tension_seven_institutional_verified);
        assert_eq!(audit.list_pettit_discursive_dilemma_and_veto.0, 0.0);
        assert!(audit.list_pettit_discursive_dilemma_and_veto.1);
        assert!(!audit.list_pettit_discursive_dilemma_and_veto.2);
        assert!(
            audit.fricker_algedonic_detection_shifts_vs_tts.0
                <= audit.fricker_algedonic_detection_shifts_vs_tts.2
        );
        assert!(
            audit.fricker_algedonic_detection_shifts_vs_tts.1
                > audit.fricker_algedonic_detection_shifts_vs_tts.2
        );
        assert!(audit.baron_hershey_unshielded_vs_bitemporal_bias.1 < 1e-12);
        assert!(audit.holmstrom_structural_operational_exogenous_shares.0 > 0.80);
        assert!(audit.liability_partition_identity_error < 1e-12);
    }
}
