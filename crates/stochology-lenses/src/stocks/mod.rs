//! Stochary-Conjectural Synthesis, the Four Ontological Axioms of the Stock, Sequential Decision
//! Meta-Classes, Value of Information, Wasserstein DRO / $\beta$-CVaR, Viable System Model (VSM),
//! and the Telic-Scopic Audit Protocol (`PATE`) (§1.3.10 — *Estocologia Estocário-Conjectural e Cibernética*).

pub mod cybernetic_vsm_pate;
pub mod empirical;
pub mod powell_friston_pearl;
pub mod supply_chain_plant;
pub mod topology_axioms;

pub use cybernetic_vsm_pate::{
    CyberneticVsmAndPateGovernanceAudit, PateQuadrant, StocharyStressTestNotebook,
};
pub use empirical::EmpiricalStocharyCyberneticAudit;
pub use powell_friston_pearl::{PowellFristonCausalEngineAudit, PowellPolicyClass};
pub use supply_chain_plant::{
    rockafellar_uryasev_cvar, StrategicStockPlantResilienceAudit, SupplyChainPhysicsAudit,
};
pub use topology_axioms::{
    TopologyAndFourStockAxiomsAudit, CONCEPTUAL_TREE_SUBAXIS_LEAVES, REBALANCED_TREE_SUBAXIS_LEAVES,
};

use crate::acceptance::{MultiSeedAcceptance, NamedCheck};

/// Complete 4-Axis Synthesis of Stochary-Conjectural & Cybernetic Stochology.
#[derive(Debug, Clone, PartialEq)]
pub struct StocharyConjecturalCyberneticSynthesis {
    /// Axis 1: 150-Node Topology (Acyclic Tree $\to$ Recursive Cybernetic Graph) & the Four Ontological Axioms of the Stock.
    pub topology_and_stock_axioms: TopologyAndFourStockAxiomsAudit,
    /// Axis 2: Powell's `CFA` tuning, Friston's Free Energy & value of information, and Pearl's `do-Calculus`.
    pub powell_friston_causal_engine: PowellFristonCausalEngineAudit,
    /// Axis 3: Dierickx & Cool, Scarf $K$-Convexity, Simchi-Levi $\mathrm{TTS}/\mathrm{TTR}/\mathrm{REI}$, and $\beta$-CVaR + WDRO.
    pub strategic_stock_plant: StrategicStockPlantResilienceAudit,
    /// Axis 4: Conant-Ashby/Beer VSM (S1–S5, `S3*`, Algedonic), Kahneman Noise Audit, PATE Matrix, and 5-Step Notebook.
    pub cybernetic_vsm_and_pate: CyberneticVsmAndPateGovernanceAudit,
    /// True if every axis's identity checks close (expected in both modes).
    pub all_identities_closed: bool,
    /// Whether all 4 axes and the 5-step Stochary Engineering Notebook are simultaneously verified.
    pub all_four_axes_verified: bool,
}

impl StocharyConjecturalCyberneticSynthesis {
    /// Evaluates the canonical shielded synthesis (`seed = 2026`).
    pub fn evaluate_canonical(seed: u64) -> Self {
        Self::evaluate_with_mode(seed, true)
    }

    /// Evaluates the synthesis deploying the cybernetic methods (`true`, Gate `G8`) or the naive methods
    /// (`false`, negative proof `--defect stochary_cybernetics`).
    pub fn evaluate_with_mode(seed: u64, cybernetic_shielded: bool) -> Self {
        let topology_and_stock_axioms =
            TopologyAndFourStockAxiomsAudit::evaluate(seed, cybernetic_shielded);
        let powell_friston_causal_engine =
            PowellFristonCausalEngineAudit::evaluate(seed, cybernetic_shielded);
        let strategic_stock_plant =
            StrategicStockPlantResilienceAudit::evaluate(seed, cybernetic_shielded);
        let cybernetic_vsm_and_pate = CyberneticVsmAndPateGovernanceAudit::evaluate(
            &topology_and_stock_axioms,
            &powell_friston_causal_engine,
            &strategic_stock_plant,
            seed,
            cybernetic_shielded,
        );
        let all_identities_closed = topology_and_stock_axioms.identities_closed
            && powell_friston_causal_engine.identities_closed
            && strategic_stock_plant.identities_closed
            && cybernetic_vsm_and_pate.identities_closed;
        let all_four_axes_verified = topology_and_stock_axioms.axis1_verified
            && powell_friston_causal_engine.axis2_verified
            && strategic_stock_plant.axis3_verified
            && cybernetic_vsm_and_pate.axis4_verified;

        Self {
            topology_and_stock_axioms,
            powell_friston_causal_engine,
            strategic_stock_plant,
            cybernetic_vsm_and_pate,
            all_identities_closed,
            all_four_axes_verified,
        }
    }

    /// Claims removed from the gate after failing multi-seed acceptance.
    pub fn discarded_claim_checks(&self) -> Vec<NamedCheck> {
        self.strategic_stock_plant
            .discarded_claims
            .iter()
            .chain(&self.cybernetic_vsm_and_pate.discarded_claims)
            .copied()
            .collect()
    }

    /// All gated claims of the four axes (Axis 3 includes `SupplyChainPhysicsAudit`), in axis order.
    pub fn claim_checks(&self) -> Vec<NamedCheck> {
        [
            &self.topology_and_stock_axioms.claims,
            &self.powell_friston_causal_engine.claims,
            &self.strategic_stock_plant.claims,
            &self.cybernetic_vsm_and_pate.claims,
        ]
        .into_iter()
        .flatten()
        .copied()
        .collect()
    }

    /// Per-claim pass rates over `seeds` in the given mode (see `crate::acceptance`).
    pub fn multi_seed_acceptance(seeds: &[u64], cybernetic_shielded: bool) -> MultiSeedAcceptance {
        MultiSeedAcceptance::evaluate(seeds, |seed| {
            let syn = Self::evaluate_with_mode(seed, cybernetic_shielded);
            (syn.claim_checks(), syn.all_identities_closed)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use stochology_core::DEFAULT_SEED;

    #[test]
    fn test_stochary_conjectural_cybernetic_canonical_and_defect() {
        let pos = StocharyConjecturalCyberneticSynthesis::evaluate_canonical(DEFAULT_SEED);
        assert!(pos.all_identities_closed);
        assert_eq!(pos.topology_and_stock_axioms.tree_total_nodes, 150);
        assert_eq!(
            pos.topology_and_stock_axioms.tree_hierarchy_counts,
            (11, 32, 106)
        );
        assert!(
            (pos.topology_and_stock_axioms.shannon_entropy_l1_bits - 3.4242).abs() < 1e-3,
            "H_L1={}",
            pos.topology_and_stock_axioms.shannon_entropy_l1_bits
        );
        assert!(
            (pos.topology_and_stock_axioms.balance_index_j_l1 - 0.9898).abs() < 1e-3,
            "J_L1={}",
            pos.topology_and_stock_axioms.balance_index_j_l1
        );
        assert!(
            (pos.topology_and_stock_axioms.shannon_entropy_l2_bits - 4.9142).abs() < 1e-3,
            "H_L2={}",
            pos.topology_and_stock_axioms.shannon_entropy_l2_bits
        );
        assert!(
            (pos.topology_and_stock_axioms.balance_index_j_l2 - 0.9828).abs() < 1e-3,
            "J_L2={}",
            pos.topology_and_stock_axioms.balance_index_j_l2
        );
        assert!(
            pos.topology_and_stock_axioms.axis1_verified,
            "{:?}",
            pos.topology_and_stock_axioms
        );
        assert!(
            pos.powell_friston_causal_engine.axis2_verified,
            "{:?}",
            pos.powell_friston_causal_engine
        );
        assert!(
            pos.strategic_stock_plant.axis3_verified,
            "{:?}",
            pos.strategic_stock_plant
        );
        assert!(
            pos.cybernetic_vsm_and_pate.axis4_verified,
            "{:?}",
            pos.cybernetic_vsm_and_pate
        );
        assert!(pos.all_four_axes_verified);

        // The naive methods are actually run and miss their targets while every identity still closes.
        let neg = StocharyConjecturalCyberneticSynthesis::evaluate_with_mode(DEFAULT_SEED, false);
        assert!(neg.all_identities_closed);
        assert!(!neg.all_four_axes_verified);
        assert!(!neg.topology_and_stock_axioms.axis1_verified);
        assert!(!neg.powell_friston_causal_engine.axis2_verified);
        assert!(!neg.strategic_stock_plant.axis3_verified);
        assert!(!neg.cybernetic_vsm_and_pate.axis4_verified);
    }
}
