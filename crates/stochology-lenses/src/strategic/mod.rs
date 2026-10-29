//! Strategic AI Curriculum Accommodation & Six-Pair Methodological Synthesis (`stochology_lenses::strategic`).

pub mod contagion_ahp_kalman;
pub mod empirical;
pub mod minsky_shapley_rag;

pub use contagion_ahp_kalman::{
    ComplexNetworkHubContagionAudit, MultiCriteriaAhpRegulatoryPkmAudit,
    RobustKalmanClosedLoopControlAudit,
};
pub use empirical::EmpiricalStrategicAccommodationAudit;
pub use minsky_shapley_rag::{
    GenerativeRagLlmJudgeCompressionAudit, MinskyPapertGeometrySocietyOfMindAudit,
    ShapleyCausalRubinMiDoeAudit,
};

use crate::acceptance::{MultiSeedAcceptance, NamedCheck};

/// Complete 6-Axis Strategic AI Accommodation Synthesis (`StrategicAiAccommodationSynthesis`),
/// uniting Minsky-Papert computational geometry, causal XAI & Rubin MI, DPI-RAG & `LLM-as-a-Judge` (`PPI`),
/// complex airspace/organizational networks, executive multi-criteria `AHP` & EU AI Act/LGPD/PKM governance,
/// and robust Huber-Kalman sensor fusion & closed-loop Tube-MPC control.
#[derive(Debug, Clone)]
pub struct StrategicAiAccommodationSynthesis {
    /// Axis 1: Minsky-Papert Geometry (`XOR`, Parity, Connectivity) & Minsky's *Society of Mind*.
    pub minsky_papert_society_of_mind: MinskyPapertGeometrySocietyOfMindAudit,
    /// Axis 2: Causal `SHAP` vs. Observational `SHAP`, `PDP` vs. `ICE`, Rubin MI, & Clustered DoE / GP-EI.
    pub shapley_rubin_mi_doe: ShapleyCausalRubinMiDoeAudit,
    /// Axis 3: 4-Stage RAG `DpiMarkovChain`, `LLM-as-a-Judge` (`PPI`), FinOps Cascade, & Outlier-Protected Quantization.
    pub generative_rag_llm_judge_compression: GenerativeRagLlmJudgeCompressionAudit,
    /// Axis 4: Complex Network Laplacian $\lambda_2(L)$, Airspace Hub Cascade, & Centola Complex Contagion.
    pub complex_network_hub_contagion: ComplexNetworkHubContagionAudit,
    /// Axis 5: Saaty `AHP` Rank Reversal, `ICE` Covariance Fallacy, EU AI Act / LGPD Gate, & Epistemic PKM Provenance.
    pub multi_criteria_ahp_regulatory_pkm: MultiCriteriaAhpRegulatoryPkmAudit,
    /// Axis 6: Precision Sensor Fusion, Joseph-Form Kalman, Huber-Kalman under Pareto Glitches, & Tube-MPC Control.
    pub robust_kalman_closed_loop_control: RobustKalmanClosedLoopControlAudit,
    /// True if every axis's exact algebraic identities close (required in both `shielded` modes).
    pub all_identities_closed: bool,
    /// Discarded findings collected from severe testing (Prohibition 4).
    pub discarded_findings: Vec<&'static str>,
    /// True if all 6 axes pass simultaneously.
    pub all_six_axes_verified: bool,
}

impl StrategicAiAccommodationSynthesis {
    /// Evaluates the canonical shielded Strategic AI Accommodation Synthesis (`seed = 2026`).
    pub fn evaluate_canonical(seed: u64) -> Self {
        Self::evaluate_with_mode(seed, true)
    }

    /// Evaluates the synthesis in either shielded CAPIBA mode (`shielded = true`) or naive mode (`shielded = false`,
    /// used by `--defect strategic_accommodation`).
    pub fn evaluate_with_mode(seed: u64, shielded: bool) -> Self {
        let a1 = MinskyPapertGeometrySocietyOfMindAudit::evaluate(seed, shielded);
        let a2 = ShapleyCausalRubinMiDoeAudit::evaluate(seed, shielded);
        let a3 = GenerativeRagLlmJudgeCompressionAudit::evaluate(seed, shielded);
        let a4 = ComplexNetworkHubContagionAudit::evaluate(seed, shielded);
        let a5 = MultiCriteriaAhpRegulatoryPkmAudit::evaluate(seed, shielded);
        let a6 = RobustKalmanClosedLoopControlAudit::evaluate(seed, shielded);

        let all_identities_closed = a1.identities_closed
            && a2.identities_closed
            && a3.identities_closed
            && a4.identities_closed
            && a5.identities_closed
            && a6.identities_closed;
        let all_ok = a1.axis1_verified
            && a2.axis2_verified
            && a3.axis3_verified
            && a4.axis4_verified
            && a5.axis5_verified
            && a6.axis6_verified;
        let mut discarded_findings = a2.discarded_findings.clone();
        discarded_findings.push(
            "PPI LLM-as-a-Judge absolute error < 0.06 at n_gold = 160 (passed 19/20 dev seeds but 0.880 on 200 confirmation seeds due to SE ~ 0.035)",
        );

        Self {
            minsky_papert_society_of_mind: a1,
            shapley_rubin_mi_doe: a2,
            generative_rag_llm_judge_compression: a3,
            complex_network_hub_contagion: a4,
            multi_criteria_ahp_regulatory_pkm: a5,
            robust_kalman_closed_loop_control: a6,
            all_identities_closed,
            discarded_findings,
            all_six_axes_verified: all_ok,
        }
    }

    /// Claims removed from the gate after failing multi-seed acceptance (recorded, never approved).
    pub fn discarded_claim_checks(&self) -> Vec<NamedCheck> {
        let mut out = self.shapley_rubin_mi_doe.discarded_claims.clone();
        out.extend(
            self.generative_rag_llm_judge_compression
                .discarded_claims
                .iter()
                .copied(),
        );
        out
    }

    /// All gated claims across the 6 axes, in declaration order.
    pub fn claim_checks(&self) -> Vec<NamedCheck> {
        [
            &self.minsky_papert_society_of_mind.claims,
            &self.shapley_rubin_mi_doe.claims,
            &self.generative_rag_llm_judge_compression.claims,
            &self.complex_network_hub_contagion.claims,
            &self.multi_criteria_ahp_regulatory_pkm.claims,
            &self.robust_kalman_closed_loop_control.claims,
        ]
        .into_iter()
        .flatten()
        .copied()
        .collect()
    }

    /// Evaluates per-claim pass rates across `seeds` in the specified mode (see `crate::acceptance`).
    pub fn multi_seed_acceptance(seeds: &[u64], shielded: bool) -> MultiSeedAcceptance {
        MultiSeedAcceptance::evaluate(seeds, |seed| {
            let syn = Self::evaluate_with_mode(seed, shielded);
            (syn.claim_checks(), syn.all_identities_closed)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::acceptance::{ACCEPTANCE_SEEDS, MIN_CLAIM_PASS_RATE};

    #[test]
    fn strategic_synthesis_passes_all_dev_seeds_when_shielded_and_blocks_all_when_unshielded() {
        let syn = StrategicAiAccommodationSynthesis::evaluate_canonical(2026);
        assert!(syn.all_identities_closed);
        assert!(
            syn.all_six_axes_verified,
            "failed claims at 2026: {:?}",
            syn.claim_checks()
                .into_iter()
                .filter(|c| !c.1)
                .collect::<Vec<_>>()
        );
        assert!(!syn.discarded_findings.is_empty());

        let dev_acc =
            StrategicAiAccommodationSynthesis::multi_seed_acceptance(&ACCEPTANCE_SEEDS, true);
        assert!(
            dev_acc.accepted,
            "Strategic synthesis failed dev acceptance: {:?}",
            dev_acc.failing_claims()
        );
        for &(name, rate) in &dev_acc.claim_pass_rates {
            assert!(
                rate >= MIN_CLAIM_PASS_RATE,
                "claim {name} had dev rate {rate}"
            );
        }

        let unshielded = StrategicAiAccommodationSynthesis::evaluate_with_mode(2026, false);
        assert!(unshielded.all_identities_closed);
        assert!(!unshielded.all_six_axes_verified);
        assert!(!unshielded.minsky_papert_society_of_mind.axis1_verified);
        assert!(!unshielded.shapley_rubin_mi_doe.axis2_verified);
        assert!(
            !unshielded
                .generative_rag_llm_judge_compression
                .axis3_verified
        );
        assert!(!unshielded.complex_network_hub_contagion.axis4_verified);
        assert!(!unshielded.multi_criteria_ahp_regulatory_pkm.axis5_verified);
        assert!(!unshielded.robust_kalman_closed_loop_control.axis6_verified);

        let unshielded_acc =
            StrategicAiAccommodationSynthesis::multi_seed_acceptance(&ACCEPTANCE_SEEDS, false);
        assert!(unshielded_acc.identities_closed_on_all_seeds);
        assert!(unshielded_acc.deterministic_claims_constant);
        let fails_every_seed = ACCEPTANCE_SEEDS.iter().all(|&s| {
            let u = StrategicAiAccommodationSynthesis::evaluate_with_mode(s, false);
            u.all_identities_closed
                && u.claim_checks()
                    .iter()
                    .any(|&(name, ok)| !crate::acceptance::is_deterministic(name) && !ok)
        });
        assert!(fails_every_seed);
    }
}
