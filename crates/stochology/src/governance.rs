//! **Chapter 8 (Ethical & Institutional Mediation · Tension 7) — Moral Luck, Apt Agency, and Institutional Design.**
//!
//! Mediates the tension between ex-ante care of the art (*télos*), moral luck in outcomes (*skopós*),
//! and institutional architecture (Beer, 1972; Williams, 1981; Nagel, 1979; Holmström, 1979;
//! Baron & Hershey, 1988; Innes, 1990; Holmström & Milgrom, 1991; Gigerenzer & Hoffrage, 1995;
//! Scanlon, 1998; Sosa, 2007, 2015; Fricker, 2007, 2010, 2020; Fantl & McGrath, 2009;
//! List & Pettit, 2011; Hansson, 2013; Chouldechova, 2017; Wong et al., 2021; Zheng & Meister, 2025):
//! - Sosa's virtue epistemology grades and Baron–Hershey two-stage bitemporal telic evaluation (`SosaJudgmentGrade`, `TelicEvaluation`);
//! - List–Pettit group agency discursive dilemma and constitutional conjunctive safety veto (`DiscursiveCommittee`);
//! - Beer's algedonic circuit breaker under Fricker's testimonial epistemic injustice discount (`AlgedonicChannel`);
//! - Innes–Holmström–Milgrom contracts and anti-scapegoating liability partition (`ContractAndLiabilityAudit`);
//! - Chouldechova's algebraic fairness trilemma (`ChouldechovaTrilemma`);
//! - Zheng–Meister $10\text{ bits/s}$ cognitive alert filter and Gigerenzer natural frequencies (`CognitiveAlertFilter`);
//! - Single-opening cryptographic `TemporalVault`.

use crate::sha256::sha256_hex;

/// Ernest Sosa (2007, 2015) four-grade taxonomy of judgment and agency under uncertainty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SosaJudgmentGrade {
    /// Failed both ex-ante competence (*télos*) and ex-post outcome (*skopós*).
    InaptAndInaccurate,
    /// Hit the target (*accurate*) purely by moral luck despite negligent ex-ante method (reckless speculation).
    LuckyAccurateOnly,
    /// Executed with rigorous ex-ante competence (*adroit*) and vital margin, yet suffered an exogenous tail shock.
    AdroitExAnteUnluckyOutcome,
    /// Achieved success *because* of competence, or exercised second-order reflective wisdom (*fully apt*)
    /// by abstaining when the epistemic margin could not clear the moral stakes of harm.
    FullyAptPerformance,
}

/// Two-stage bitemporal telic evaluation shielding ex-ante competence assessment from
/// Jonathan Baron & John C. Hershey's (1988) hindsight outcome bias.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TelicEvaluation {
    /// Stage-1 ex-ante competence score $T_{\mathrm{ex\text{-}ante}} \in [0, 1]$, sealed before the outcome is known.
    pub stage1_sealed_competence: f64,
    /// Realized ex-post outcome success (`true`) or exogenous tail loss (`false`).
    pub outcome_succeeded: bool,
    /// Whether the conservative epistemic lower bound cleared the moral/pragmatic threshold $p^*$, or the agent abstained.
    pub cleared_moral_encroachment_or_abstained: bool,
    /// Sosa judgment grade assigned by the bitemporal protocol.
    pub sosa_grade: SosaJudgmentGrade,
    /// Unshielded hindsight evaluation score when contaminated by `hindsight_penalty` after a bad outcome.
    pub unshielded_hindsight_score: f64,
    /// Outcome bias eliminated by Stage-1 bitemporal sealing.
    pub eliminated_outcome_bias: f64,
}

impl TelicEvaluation {
    /// Evaluates an agent's decision in two bitemporal stages, separating *télos* from *skopós*.
    pub fn evaluate(
        stage1_sealed_competence: f64,
        cleared_moral_encroachment_or_abstained: bool,
        outcome_succeeded: bool,
        hindsight_penalty_if_loss: f64,
    ) -> Self {
        let comp = stage1_sealed_competence.clamp(0.0, 1.0);
        let adroit = comp >= 0.75 && cleared_moral_encroachment_or_abstained;
        let grade = match (adroit, outcome_succeeded) {
            (true, true) => SosaJudgmentGrade::FullyAptPerformance,
            (true, false) => SosaJudgmentGrade::AdroitExAnteUnluckyOutcome,
            (false, true) => SosaJudgmentGrade::LuckyAccurateOnly,
            (false, false) => SosaJudgmentGrade::InaptAndInaccurate,
        };
        let unshielded = if outcome_succeeded {
            comp
        } else {
            (comp - hindsight_penalty_if_loss.max(0.0)).max(0.0)
        };
        Self {
            stage1_sealed_competence: comp,
            outcome_succeeded,
            cleared_moral_encroachment_or_abstained,
            sosa_grade: grade,
            unshielded_hindsight_score: unshielded,
            eliminated_outcome_bias: comp - unshielded,
        }
    }
}

/// Christian List & Philip Pettit (2011) group agency discursive dilemma audit over the conjunctive
/// safety rule $D \iff (P_1 \land P_2 \land P_3)$ (Epistemic Margin $\land$ Vital Margin $\land$ Governance).
#[derive(Debug, Clone, PartialEq)]
pub struct DiscursiveCommittee {
    /// Number of committee members $M$.
    pub num_members: usize,
    /// Fraction of members who approve each individual premise $(P_1, P_2, P_3)$.
    pub premise_approval_rates: [f64; 3],
    /// Fraction of members who approve all three premises simultaneously ($P_1 \land P_2 \land P_3$).
    pub conclusion_approval_rate: f64,
    /// Whether naive premise-by-premise majority voting ($\ge 50\%$ on each $P_j$ separately) approves deployment.
    pub premise_majority_approves: bool,
    /// Whether constitutional conjunctive safety voting (requiring all 3 safeguards to pass without $S3^*$ veto) approves.
    pub constitutional_conjunctive_approves: bool,
    /// True if a discursive dilemma is present (`premise_majority_approves && !constitutional_conjunctive_approves`).
    pub discursive_dilemma_present: bool,
}

impl DiscursiveCommittee {
    /// Evaluates a committee where each member $i$ submits votes `[p1_ok, p2_ok, p3_ok]` on the three safeguards.
    pub fn evaluate(member_votes: &[[bool; 3]]) -> Self {
        assert!(
            !member_votes.is_empty(),
            "DiscursiveCommittee requires at least 1 member"
        );
        let m = member_votes.len();
        let mf = m as f64;
        let mut p_counts = [0usize; 3];
        let mut conj_count = 0usize;
        for vote in member_votes {
            for j in 0..3 {
                if vote[j] {
                    p_counts[j] += 1;
                }
            }
            if vote[0] && vote[1] && vote[2] {
                conj_count += 1;
            }
        }
        let rates = [
            (p_counts[0] as f64) / mf,
            (p_counts[1] as f64) / mf,
            (p_counts[2] as f64) / mf,
        ];
        let conc_rate = (conj_count as f64) / mf;
        let premise_maj = rates.iter().all(|&r| r > 0.5);
        // Constitutional rule: requires majority conjunctive approval AND no unanimous failure on any individual safeguard
        let const_ok = conc_rate > 0.5;
        Self {
            num_members: m,
            premise_approval_rates: rates,
            conclusion_approval_rate: conc_rate,
            premise_majority_approves: premise_maj,
            constitutional_conjunctive_approves: const_ok,
            discursive_dilemma_present: premise_maj && !const_ok,
        }
    }
}

/// Stafford Beer (1972) algedonic emergency channel coupled with Miranda Fricker's (2007, 2010, 2020)
/// audit of testimonial epistemic injustice against frontline operators and affected citizens.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AlgedonicChannel {
    /// Per-shift frontline evidence multiplier $e_{\mathrm{shift}} > 1$ during an incipient crisis.
    pub frontline_shift_e_factor: f64,
    /// Circuit-breaker trip threshold $1 / \alpha > 1$.
    pub trip_threshold: f64,
    /// Testimonial credibility discount $\delta_{\mathrm{cred}} \in [0, 1)$ imposed by hierarchy on frontline testimony.
    pub testimonial_discount_delta: f64,
    /// Shifts required to trip the algedonic circuit breaker without testimonial injustice ($\delta_{\mathrm{cred}} = 0$).
    pub shifts_to_trip_undiscounted: f64,
    /// Shifts required to trip the algedonic circuit breaker under testimonial discount $\delta_{\mathrm{cred}}$.
    pub shifts_to_trip_discounted: f64,
    /// Available physical Time-to-Survive $\mathrm{TTS}$ (in shifts).
    pub time_to_survive_shifts: f64,
    /// True if testimonial injustice delays the algedonic trip past $\mathrm{TTS}$ while undiscounted testimony would have saved the system.
    pub fatal_testimonial_delay: bool,
}

impl AlgedonicChannel {
    /// Computes the algedonic circuit-breaker detection time with and without Fricker's testimonial discount.
    pub fn evaluate(
        frontline_shift_e_factor: f64,
        trip_threshold: f64,
        testimonial_discount_delta: f64,
        time_to_survive_shifts: f64,
    ) -> Self {
        assert!(
            frontline_shift_e_factor > 1.0 && trip_threshold > 1.0,
            "requires frontline_shift_e_factor > 1 and trip_threshold > 1"
        );
        let target_ln = trip_threshold.ln();
        let step_ln = frontline_shift_e_factor.ln();
        let delta = testimonial_discount_delta.clamp(0.0, 0.99);
        let t_just = (target_ln / step_ln).ceil();
        let t_disc = (target_ln / ((1.0 - delta) * step_ln)).ceil();
        let tts = time_to_survive_shifts.max(0.0);
        Self {
            frontline_shift_e_factor,
            trip_threshold,
            testimonial_discount_delta: delta,
            shifts_to_trip_undiscounted: t_just,
            shifts_to_trip_discounted: t_disc,
            time_to_survive_shifts: tts,
            fatal_testimonial_delay: t_just <= tts && t_disc > tts,
        }
    }
}

/// Holmström (1979), Innes (1990), and Holmström & Milgrom (1991) institutional contract and liability audit:
/// 1. Eliminates top-management's free call option on tail volatility via deferred clawback (`clawback_fraction`);
/// 2. Protects unmeasured care quality against single-metric high-powered incentives (`multitask_care_effort`);
/// 3. Partitions overload hazard into structural capacity cuts vs. frontline operational variance vs. exogenous shock.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ContractAndLiabilityAudit {
    /// Expected executive payoff from a reckless tail-risk gamble under limited liability (zero clawback).
    pub unpenalized_gamble_ev: f64,
    /// Expected executive payoff from the same reckless gamble under long-horizon clawback.
    pub clawback_gamble_ev: f64,
    /// Dedicated effort to unmeasured care quality $e_{\mathrm{care}}$ under balanced vs. hyper-quantified incentives.
    pub care_effort_balanced: f64,
    /// Dedicated effort to unmeasured care quality $e_{\mathrm{care}}$ under aggressive volume-only bonus.
    pub care_effort_hyper_quantified: f64,
    /// Share of overload hazard caused by executive slack compression $L_{\mathrm{struct}} \in [0, 1]$.
    pub structural_liability_share: f64,
    /// Share of overload hazard caused by frontline controllable variance $L_{\mathrm{oper}} \in [0, 1]$.
    pub operational_liability_share: f64,
    /// Share of overload hazard caused by irreducible exogenous shock $L_{\mathrm{exog}} \in [0, 1]$.
    pub exogenous_liability_share: f64,
}

impl ContractAndLiabilityAudit {
    /// Evaluates Innes limited-liability moral hazard, Holmström–Milgrom multitask care erosion,
    /// and Holmström structural vs. operational liability partition.
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate(
        upside_prob: f64,
        upside_bonus: f64,
        downside_ruin_loss: f64,
        clawback_escrow_fraction: f64,
        volume_incentive_power: f64,
        cross_task_substitution_rho: f64,
        structural_kingman_jump: f64,
        frontline_variance_hazard: f64,
        exogenous_shock_hazard: f64,
    ) -> Self {
        let p = upside_prob.clamp(0.0, 1.0);
        let q = 1.0 - p;
        let unpenalized_ev = p * upside_bonus; // limited liability truncates downside at 0
        let clawback_ev =
            p * upside_bonus - q * clawback_escrow_fraction.clamp(0.0, 1.0) * downside_ruin_loss;

        let base_care = 1.0_f64;
        let care_balanced = (base_care - 0.10 * cross_task_substitution_rho).max(0.0);
        let care_hyper =
            (base_care - volume_incentive_power.max(0.0) * cross_task_substitution_rho).max(0.0);

        let h_struct = structural_kingman_jump.max(0.0);
        let h_oper = frontline_variance_hazard.max(0.0);
        let h_exog = exogenous_shock_hazard.max(0.0);
        let h_tot = (h_struct + h_oper + h_exog).max(1e-12);

        Self {
            unpenalized_gamble_ev: unpenalized_ev,
            clawback_gamble_ev: clawback_ev,
            care_effort_balanced: care_balanced,
            care_effort_hyper_quantified: care_hyper,
            structural_liability_share: h_struct / h_tot,
            operational_liability_share: h_oper / h_tot,
            exogenous_liability_share: h_exog / h_tot,
        }
    }
}

/// Alexandra Chouldechova (2017) exact algebraic fairness trilemma under unequal base rates $p_A \ne p_B$:
/// $$\mathrm{FPR}_g = \frac{p_g}{1 - p_g} \cdot \frac{1 - \mathrm{PPV}_g}{\mathrm{PPV}_g} \cdot (1 - \mathrm{FNR}_g)$$
/// Whenever a classifier is calibrated across groups ($\mathrm{PPV}_A = \mathrm{PPV}_B$) and equalizes
/// false-negative rates ($\mathrm{FNR}_A = \mathrm{FNR}_B$), the false-positive rate ratio is algebraically locked to:
/// $$\frac{\mathrm{FPR}_A}{\mathrm{FPR}_B} = \frac{p_A / (1 - p_A)}{p_B / (1 - p_B)}$$
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChouldechovaTrilemma {
    /// Base rate (prevalence) in group $A$: $p_A \in (0, 1)$.
    pub prevalence_a: f64,
    /// Base rate (prevalence) in group $B$: $p_B \in (0, 1)$.
    pub prevalence_b: f64,
    /// Shared Positive Predictive Value $\mathrm{PPV} \in (0, 1)$.
    pub target_ppv: f64,
    /// Shared True Positive Rate $1 - \mathrm{FNR} \in (0, 1)$.
    pub target_tpr: f64,
    /// Implied False Positive Rate in group $A$: $\mathrm{FPR}_A$.
    pub implied_fpr_a: f64,
    /// Implied False Positive Rate in group $B$: $\mathrm{FPR}_B$.
    pub implied_fpr_b: f64,
    /// Exact odds-ratio disparity $\mathrm{FPR}_A / \mathrm{FPR}_B$.
    pub fpr_disparity_ratio: f64,
}

impl ChouldechovaTrilemma {
    /// Computes the exact Chouldechova false-positive disparity forced by unequal base rates `prevalence_a` and `prevalence_b`.
    pub fn evaluate(
        prevalence_a: f64,
        prevalence_b: f64,
        target_ppv: f64,
        target_tpr: f64,
    ) -> Self {
        let pa = prevalence_a.clamp(1e-6, 1.0 - 1e-6);
        let pb = prevalence_b.clamp(1e-6, 1.0 - 1e-6);
        let ppv = target_ppv.clamp(1e-6, 1.0 - 1e-6);
        let tpr = target_tpr.clamp(1e-6, 1.0 - 1e-6);

        let odds_a = pa / (1.0 - pa);
        let odds_b = pb / (1.0 - pb);
        let factor = ((1.0 - ppv) / ppv) * tpr;
        let fpr_a = odds_a * factor;
        let fpr_b = odds_b * factor;

        Self {
            prevalence_a: pa,
            prevalence_b: pb,
            target_ppv: ppv,
            target_tpr: tpr,
            implied_fpr_a: fpr_a,
            implied_fpr_b: fpr_b,
            fpr_disparity_ratio: odds_a / odds_b,
        }
    }
}

/// Cognitive alert filter protecting the human operator's $10\text{ bits/s}$ conscious deliberation channel
/// (Zheng & Meister, 2025; Wong et al., 2021) and translating probabilities into Gigerenzer & Hoffrage (1995)
/// integer natural frequencies.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CognitiveAlertFilter {
    /// Prior base rate $\pi_0 \in (0, 1)$.
    pub base_rate_pi: f64,
    /// True Positive Rate (sensitivity) $\mathrm{TPR} \in (0, 1)$.
    pub tpr: f64,
    /// False Positive Rate $\mathrm{FPR} \in (0, 1)$.
    pub fpr: f64,
    /// Shift-level Positive Predictive Value $\mathrm{PPV} = \frac{\pi_0 \mathrm{TPR}}{\pi_0 \mathrm{TPR} + (1 - \pi_0)\mathrm{FPR}}$.
    pub shift_ppv: f64,
    /// False Discovery Rate $\mathrm{FDR} = 1 - \mathrm{PPV}$.
    pub false_alarm_rate_fdr: f64,
    /// Turing–Good weight of evidence in decibans $W = 10 \log_{10}(\mathrm{TPR} / \mathrm{FPR})$.
    pub deciban_weight: f64,
    /// Natural-frequency denominator $N_{\mathrm{ref}}$ (e.g., $1\,000$ patients/shifts).
    pub natural_freq_cohort: u32,
    /// Expected true cases in $N_{\mathrm{ref}}$.
    pub natural_freq_true_cases: u32,
    /// Expected true alerts in $N_{\mathrm{ref}}$.
    pub natural_freq_true_alerts: u32,
    /// Expected false alarms in $N_{\mathrm{ref}}$.
    pub natural_freq_false_alarms: u32,
}

impl CognitiveAlertFilter {
    /// Evaluates shift-level PPV, Turing–Good deciban weight, and Gigerenzer natural frequencies for a cohort of size `cohort_n`.
    pub fn evaluate(base_rate_pi: f64, tpr: f64, fpr: f64, cohort_n: u32) -> Self {
        let pi = base_rate_pi.clamp(1e-6, 1.0 - 1e-6);
        let sens = tpr.clamp(1e-6, 1.0 - 1e-6);
        let fp = fpr.clamp(1e-6, 1.0 - 1e-6);
        let tp_mass = pi * sens;
        let fp_mass = (1.0 - pi) * fp;
        let ppv = tp_mass / (tp_mass + fp_mass);
        let db = 10.0 * (sens / fp).log10();

        let n_f = cohort_n.max(100) as f64;
        let n_cases = (n_f * pi).round() as u32;
        let n_true_alerts = ((n_cases as f64) * sens).round() as u32;
        let n_non_cases = cohort_n.max(100).saturating_sub(n_cases);
        let n_false_alarms = ((n_non_cases as f64) * fp).round() as u32;

        Self {
            base_rate_pi: pi,
            tpr: sens,
            fpr: fp,
            shift_ppv: ppv,
            false_alarm_rate_fdr: 1.0 - ppv,
            deciban_weight: db,
            natural_freq_cohort: cohort_n.max(100),
            natural_freq_true_cases: n_cases,
            natural_freq_true_alerts: n_true_alerts,
            natural_freq_false_alarms: n_false_alarms,
        }
    }
}

/// Cryptographic SHA-256 Single-Opening Temporal Vault (`TemporalVault`).
///
/// Seals a pre-registered analysis plan and holdout dataset digest before evaluation;
/// opening the vault more than once or with a mismatched digest automatically downgrades
/// the result from confirmatory to exploratory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemporalVault {
    /// Pre-registered 64-hex SHA-256 commitment of the analysis specification and holdout data.
    pub expected_sha256: String,
    /// Number of times the vault has been opened.
    pub open_count: usize,
    /// Whether the vault remains valid for confirmatory claims (`open_count == 1` with matching hash).
    pub is_confirmatory: bool,
}

impl TemporalVault {
    /// Creates a new sealed `TemporalVault` committed to `payload_bytes`.
    pub fn seal(payload_bytes: &[u8]) -> Self {
        Self {
            expected_sha256: sha256_hex(payload_bytes),
            open_count: 0,
            is_confirmatory: false,
        }
    }

    /// Attempts to open the vault with `candidate_bytes`.
    /// Succeeds as confirmatory on the first opening iff `sha256_hex(candidate_bytes) == expected_sha256`;
    /// any second opening or hash mismatch downgrades `is_confirmatory` to `false` and returns `Err`.
    pub fn unseal(&mut self, candidate_bytes: &[u8]) -> Result<(), &'static str> {
        self.open_count += 1;
        if self.open_count > 1 {
            self.is_confirmatory = false;
            return Err(
                "TemporalVault violation: holdout vault opened more than once (exploratory downgrade)",
            );
        }
        let actual = sha256_hex(candidate_bytes);
        if actual != self.expected_sha256 {
            self.is_confirmatory = false;
            return Err("TemporalVault violation: SHA-256 pre-registration mismatch");
        }
        self.is_confirmatory = true;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discursive_dilemma_algedonic_injustice_and_telic_review_hold() {
        let votes = [
            [true, true, false],
            [true, false, true],
            [false, true, true],
        ];
        let comm = DiscursiveCommittee::evaluate(&votes);
        assert!(comm.premise_majority_approves);
        assert!(!comm.constitutional_conjunctive_approves);
        assert!(comm.discursive_dilemma_present);

        let alg = AlgedonicChannel::evaluate(1.85, 20.0, 0.55, 8.0);
        assert!(alg.fatal_testimonial_delay);

        let telic = TelicEvaluation::evaluate(0.92, true, false, 0.45);
        assert_eq!(
            telic.sosa_grade,
            SosaJudgmentGrade::AdroitExAnteUnluckyOutcome
        );
        assert!((telic.eliminated_outcome_bias - 0.45).abs() < 1e-12);
    }

    #[test]
    fn chouldechova_contracts_alerts_and_temporal_vault_hold() {
        let ch = ChouldechovaTrilemma::evaluate(0.40, 0.20, 0.60, 0.75);
        assert!((ch.fpr_disparity_ratio - ((0.4 / 0.6) / (0.2 / 0.8))).abs() < 1e-12);

        let contract =
            ContractAndLiabilityAudit::evaluate(0.70, 10.0, 40.0, 0.80, 8.0, 0.12, 20.0, 1.5, 2.5);
        assert!(contract.unpenalized_gamble_ev > 0.0);
        assert!(contract.clawback_gamble_ev < 0.0);
        assert!(contract.structural_liability_share > 0.80);

        let noisy = CognitiveAlertFilter::evaluate(0.02, 0.80, 0.25, 1000);
        let filtered = CognitiveAlertFilter::evaluate(0.02, 0.82, 0.015, 1000);
        assert!(noisy.false_alarm_rate_fdr > 0.85);
        assert!(filtered.shift_ppv > 0.50);

        let mut vault = TemporalVault::seal(b"preregistered-protocol-v1");
        assert!(vault.unseal(b"preregistered-protocol-v1").is_ok());
        assert!(vault.is_confirmatory);
        assert!(vault.unseal(b"preregistered-protocol-v1").is_err());
        assert!(!vault.is_confirmatory);
    }
}
