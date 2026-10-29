//! Critical Theory of Stochastic Rationality, Algorithmic Governmentality, and the Material Collapse &
//! Neurosymbolic/Wave Overcoming of AI (§1.3.9).

pub mod adorno_marcuse;
pub mod collapse_overcoming;
pub mod empirical;

pub use adorno_marcuse::{AdornoNonIdenticalReificationAudit, MarcuseRouvroyStochasticParrotAudit};
pub use collapse_overcoming::{DialecticalOvercomingSynthesisAudit, MaterialCollapseTriadAudit};
pub use empirical::EmpiricalCriticalRationalityAudit;

use crate::acceptance::{MultiSeedAcceptance, NamedCheck};

/// Complete 4-Axis Synthesis of **Critical Theory of Stochastic Rationality, Algorithmic
/// Governmentality, and the Material Collapse & Neurosymbolic/Wave Overcoming of AI** (§1.3.9).
#[derive(Debug, Clone, PartialEq)]
pub struct CriticalStochasticRationalitySynthesis {
    /// Axis 1: Adorno & Horkheimer's Liquidation of the *Non-Identical* and Eternalization of the Present.
    pub adorno_non_identical_and_reification: AdornoNonIdenticalReificationAudit,
    /// Axis 2: Marcuse's Operational Closure, Rouvroy's Algorithmic Preemption, and "Stochastic Parrots".
    pub marcuse_rouvroy_stochastic_parrot: MarcuseRouvroyStochasticParrotAudit,
    /// Axis 3: Material Collapse Triad (Informational Autophagy, Binary Thermodynamic Wall, and Analog Noise Cliff).
    pub material_collapse_triad: MaterialCollapseTriadAudit,
    /// Axis 4: Dialectical Overcoming (Neurosymbolic Constraints, A-ECC/PKU `HP-INV`, and Continuous Wave Physics).
    pub dialectical_overcoming: DialecticalOvercomingSynthesisAudit,
    /// True if every axis's identity checks close (expected in both modes).
    pub all_identities_closed: bool,
    /// Whether all four axes pass their epistemological and mathematical gates.
    pub all_four_axes_verified: bool,
}

impl CriticalStochasticRationalitySynthesis {
    /// Evaluates the canonical 4-axis synthesis (`seed = 2026`).
    pub fn evaluate_canonical(seed: u64) -> Self {
        Self::evaluate_with_mode(seed, true)
    }

    /// Evaluates the 4-axis synthesis deploying the grounded methods (`true`) or the naive positivist methods
    /// (`false`, used by `--defect instrumental_reason`).
    pub fn evaluate_with_mode(seed: u64, grounded_mode: bool) -> Self {
        let adorno_non_identical_and_reification =
            AdornoNonIdenticalReificationAudit::evaluate(seed, grounded_mode);
        let marcuse_rouvroy_stochastic_parrot =
            MarcuseRouvroyStochasticParrotAudit::evaluate(seed, grounded_mode);
        let material_collapse_triad = MaterialCollapseTriadAudit::evaluate(seed);
        let dialectical_overcoming =
            DialecticalOvercomingSynthesisAudit::evaluate(seed, grounded_mode);

        let all_identities_closed = adorno_non_identical_and_reification.identities_closed
            && marcuse_rouvroy_stochastic_parrot.identities_closed
            && dialectical_overcoming.identities_closed;
        let all_four_axes_verified = adorno_non_identical_and_reification.axis1_verified
            && marcuse_rouvroy_stochastic_parrot.axis2_verified
            && material_collapse_triad.axis3_verified
            && dialectical_overcoming.axis4_verified;

        Self {
            adorno_non_identical_and_reification,
            marcuse_rouvroy_stochastic_parrot,
            material_collapse_triad,
            dialectical_overcoming,
            all_identities_closed,
            all_four_axes_verified,
        }
    }

    /// Claims removed from the gate after failing multi-seed acceptance.
    pub fn discarded_claim_checks(&self) -> Vec<NamedCheck> {
        self.adorno_non_identical_and_reification
            .discarded_claims
            .iter()
            .chain(&self.material_collapse_triad.discarded_claims)
            .copied()
            .collect()
    }

    /// All gated claims of the four axes, in axis order.
    pub fn claim_checks(&self) -> Vec<NamedCheck> {
        [
            &self.adorno_non_identical_and_reification.claims,
            &self.marcuse_rouvroy_stochastic_parrot.claims,
            &self.material_collapse_triad.claims,
            &self.dialectical_overcoming.claims,
        ]
        .into_iter()
        .flatten()
        .copied()
        .collect()
    }

    /// Per-claim pass rates over `seeds` in the given mode (see `crate::acceptance`).
    pub fn multi_seed_acceptance(seeds: &[u64], grounded_mode: bool) -> MultiSeedAcceptance {
        MultiSeedAcceptance::evaluate(seeds, |seed| {
            let syn = Self::evaluate_with_mode(seed, grounded_mode);
            (syn.claim_checks(), syn.all_identities_closed)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use stochology_core::DEFAULT_SEED;

    #[test]
    fn test_axis1_adorno_non_identical_and_reification() {
        let pos = AdornoNonIdenticalReificationAudit::evaluate(DEFAULT_SEED, true);
        assert!(pos.non_identical_liquidation_gap > 30.0);
        assert!(pos.es99_erasure_fraction > 0.60);
        assert!(pos.koh_structural_discrepancy_norm > 0.40);
        assert!(pos.status_quo_reification_excess_risk > 0.05);
        assert!(pos.axis1_verified, "{pos:?}");

        let neg = AdornoNonIdenticalReificationAudit::evaluate(DEFAULT_SEED, false);
        assert!(neg.non_identical_liquidation_gap > 30.0);
        assert!(neg.deployed_var99_holdout_exceedance > 0.02, "{neg:?}");
        assert!(neg.deployed_performative_excess_risk > 1e-3);
        assert!(!neg.axis1_verified);
    }

    #[test]
    fn test_axis2_marcuse_rouvroy_stochastic_parrot() {
        let pos = MarcuseRouvroyStochasticParrotAudit::evaluate(DEFAULT_SEED, true);
        assert!(pos.parrot_stationary_surface_mse < 0.06);
        assert!(pos.parrot_counterfactual_intervention_mse > 5.0);
        assert!(pos.causal_grounded_counterfactual_mse < 0.03);
        assert!(pos.spurious_proxy_invariance_penalty_ratio > 20.0);
        assert!(pos.preemptive_profiling_fpr_disparity_ratio > 3.0);
        assert!(pos.authentic_form_blocks_spurious_preemption);
        assert!(pos.axis2_verified);

        let neg = MarcuseRouvroyStochasticParrotAudit::evaluate(DEFAULT_SEED, false);
        assert!(neg.identities_closed);
        assert!(neg.deployed_counterfactual_mse > 5.0);
        assert!(!neg.axis2_verified);
    }

    #[test]
    fn test_axis3_material_collapse_triad() {
        let m = MaterialCollapseTriadAudit::evaluate(DEFAULT_SEED);
        assert!(m.synthetic_autophagy_variance_retention < 0.25);
        assert!(m.synthetic_autophagy_kl_divergence_nats > 0.60);
        assert!(m.axis3_verified);
    }

    #[test]
    fn test_axis4_dialectical_overcoming_and_full_synthesis() {
        let syn_pos = CriticalStochasticRationalitySynthesis::evaluate_canonical(DEFAULT_SEED);
        assert!(syn_pos.all_identities_closed);
        assert!(
            syn_pos.all_four_axes_verified,
            "{:?}",
            syn_pos.dialectical_overcoming
        );
        assert!(syn_pos.dialectical_overcoming.axis4_verified);

        let syn_neg =
            CriticalStochasticRationalitySynthesis::evaluate_with_mode(DEFAULT_SEED, false);
        assert!(syn_neg.all_identities_closed);
        assert!(!syn_neg.all_four_axes_verified);
        assert!(!syn_neg.adorno_non_identical_and_reification.axis1_verified);
        assert!(!syn_neg.marcuse_rouvroy_stochastic_parrot.axis2_verified);
        assert!(!syn_neg.dialectical_overcoming.axis4_verified);
    }
}
