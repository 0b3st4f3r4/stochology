//! Triadic Mathematical & Physical Synthesis ($\mathbb{C} \times \text{Fractals} \times \mathbb{T}^d$),
//! Technium / Technosphere Evolutionary Ontology, and Spring-and-Horizon Multidimensional Spiral (`stochology_lenses::triad`).

pub mod ctrw_pdhte;
pub mod empirical;
pub mod helmholtz_hmatrix;
pub mod weyl_sna_rlhf;

pub use ctrw_pdhte::{FractionalCtrwAnalogEccAudit, MultifractalPdhteRagCollapseAudit};
pub use empirical::EmpiricalTriadicTechniumAudit;
pub use helmholtz_hmatrix::{ComplexRealIsomorphismHMatrixAudit, ComplexWaveGaborHelmholtzAudit};
pub use weyl_sna_rlhf::{SnaRlhfFhrrCohomologyTechniumAudit, WeylTorusHermiteMehlerDfmAudit};

use crate::acceptance::{MultiSeedAcceptance, NamedCheck};

/// Complete 6-Bridge Triadic Mathematical & Physical Synthesis ($\mathbb{C} \times \text{Fractals} \times \mathbb{T}^d$),
/// Technium / Technosphere Evolutionary Ontology, and Spring-and-Horizon Multidimensional Spiral.
#[derive(Debug, Clone)]
pub struct ComplexFractalToroidalTechniumSynthesis {
    /// Axis 1 ($\mathbb{C}$): Complex Gabor Wavelets (`WIRE` with learned frequency, `Improved SIREN`) on 2D Helmholtz PDE & Adjoint Photonic Backpropagation.
    pub complex_wave_gabor_helmholtz: ComplexWaveGaborHelmholtzAudit,
    /// Axis 2 ($\mathbb{C} \leftrightarrow \text{Fractals}$): $\mathbb{C}^{n\times n} \hookrightarrow \mathbb{R}^{2n\times 2n}$ Isomorphism, Two-Stage `BlockAMC`, & Hierarchical Kernel Compression.
    pub complex_real_isomorphism_hmatrix: ComplexRealIsomorphismHMatrixAudit,
    /// Axis 3 ($\text{Fractals} \leftrightarrow \mathbb{R}^n$): Atomic Conductance $G_0$, PCM Drift, Simulated `CTRW` Scaling, & Analog ECC Decoding.
    pub fractional_ctrw_analog_ecc: FractionalCtrwAnalogEccAudit,
    /// Axis 4 ($\text{Fractals} \leftrightarrow \text{Scaling Laws}$): Multifractal Cascade, Eichi `PDHTE+JK` & `TailWelsch-DML`, & $\pi^2/6$ Bound.
    pub multifractal_pdhte_rag_collapse: MultifractalPdhteRagCollapseAudit,
    /// Axis 5 ($\mathbb{T}^d \leftrightarrow \mathbb{C}$): Preface $\mathbb{Z}/16\mathbb{Z}$ Cycle, Weyl `mixGamma`, Hermite-Mehler `DFM`, & `MLLS+BCTS`.
    pub weyl_torus_hermite_mehler_dfm: WeylTorusHermiteMehlerDfmAudit,
    /// Axis 6 ($\mathbb{C} \times \text{Fractals} \times \mathbb{T}^d$): Forced-Logistic `SNA`, `RLHF` Loop, `PerfGD`, `FHRR`/`GC-VSA`, `LTC` Fused Solver, & Technium Heuristics.
    pub sna_rlhf_fhrr_cohomology_technium: SnaRlhfFhrrCohomologyTechniumAudit,
    /// True if every axis's identity checks close (expected in both modes).
    pub all_identities_closed: bool,
    /// Discarded findings collected from Axes 4 and 6 (Prohibition 4).
    pub discarded_findings: Vec<&'static str>,
    /// True if all 6 triadic bridges pass simultaneously.
    pub all_six_bridges_verified: bool,
}

impl ComplexFractalToroidalTechniumSynthesis {
    /// Evaluates the canonical shielded Triadic & Technium Synthesis under `seed = 2026`.
    pub fn evaluate_canonical(seed: u64) -> Self {
        Self::evaluate_with_mode(seed, true)
    }

    /// Evaluates the Triadic & Technium Synthesis deploying either the CAPIBA methods (`shielded = true`)
    /// or the naive methods (`shielded = false`, used by `--defect triadic_technium`).
    pub fn evaluate_with_mode(seed: u64, shielded: bool) -> Self {
        let a1 = ComplexWaveGaborHelmholtzAudit::evaluate(seed, shielded);
        let a2 = ComplexRealIsomorphismHMatrixAudit::evaluate(seed, shielded);
        let a3 = FractionalCtrwAnalogEccAudit::evaluate(seed, shielded);
        let a4 = MultifractalPdhteRagCollapseAudit::evaluate(seed, shielded);
        let a5 = WeylTorusHermiteMehlerDfmAudit::evaluate(seed, shielded);
        let a6 = SnaRlhfFhrrCohomologyTechniumAudit::evaluate(seed, shielded);
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
        let discarded_findings = a4
            .discarded_findings
            .iter()
            .chain(&a6.discarded_findings)
            .copied()
            .collect();
        Self {
            complex_wave_gabor_helmholtz: a1,
            complex_real_isomorphism_hmatrix: a2,
            fractional_ctrw_analog_ecc: a3,
            multifractal_pdhte_rag_collapse: a4,
            weyl_torus_hermite_mehler_dfm: a5,
            sna_rlhf_fhrr_cohomology_technium: a6,
            all_identities_closed,
            discarded_findings,
            all_six_bridges_verified: all_ok,
        }
    }

    /// Claims removed from the gate after failing multi-seed acceptance.
    pub fn discarded_claim_checks(&self) -> Vec<NamedCheck> {
        self.weyl_torus_hermite_mehler_dfm.discarded_claims.clone()
    }

    /// All gated claims of the six bridges, in axis order.
    pub fn claim_checks(&self) -> Vec<NamedCheck> {
        [
            &self.complex_wave_gabor_helmholtz.claims,
            &self.complex_real_isomorphism_hmatrix.claims,
            &self.fractional_ctrw_analog_ecc.claims,
            &self.multifractal_pdhte_rag_collapse.claims,
            &self.weyl_torus_hermite_mehler_dfm.claims,
            &self.sna_rlhf_fhrr_cohomology_technium.claims,
        ]
        .into_iter()
        .flatten()
        .copied()
        .collect()
    }

    /// Per-claim pass rates over `seeds` in the given mode (see `crate::acceptance`).
    pub fn multi_seed_acceptance(seeds: &[u64], shielded: bool) -> MultiSeedAcceptance {
        MultiSeedAcceptance::evaluate(seeds, |seed| {
            let syn = Self::evaluate_with_mode(seed, shielded);
            (syn.claim_checks(), syn.all_identities_closed)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::weyl_sna_rlhf::{fit_bcts, rlhf_nonconsensus_spectral_radius};
    use super::*;
    use stochology_core::rng::{Rng, DEFAULT_SEED};

    #[test]
    fn test_triadic_synthesis_shielded_and_unshielded() {
        let shielded = ComplexFractalToroidalTechniumSynthesis::evaluate_canonical(DEFAULT_SEED);
        assert!(
            shielded.complex_wave_gabor_helmholtz.axis1_verified,
            "a1={:?}",
            shielded.complex_wave_gabor_helmholtz
        );
        assert!(
            shielded.complex_real_isomorphism_hmatrix.axis2_verified,
            "a2={:?}",
            shielded.complex_real_isomorphism_hmatrix
        );
        assert!(
            shielded.fractional_ctrw_analog_ecc.axis3_verified,
            "a3={:?}",
            shielded.fractional_ctrw_analog_ecc
        );
        assert!(
            shielded.multifractal_pdhte_rag_collapse.axis4_verified,
            "a4={:?}",
            shielded.multifractal_pdhte_rag_collapse
        );
        assert!(
            shielded.weyl_torus_hermite_mehler_dfm.axis5_verified,
            "a5={:?}",
            shielded.weyl_torus_hermite_mehler_dfm
        );
        assert!(
            shielded.sna_rlhf_fhrr_cohomology_technium.axis6_verified,
            "a6={:?}",
            shielded.sna_rlhf_fhrr_cohomology_technium
        );
        assert!(shielded.all_identities_closed);
        assert!(shielded.all_six_bridges_verified);
        assert_eq!(
            shielded.discarded_findings.len(),
            3,
            "{:?}",
            shielded.discarded_findings
        );

        let unshielded =
            ComplexFractalToroidalTechniumSynthesis::evaluate_with_mode(DEFAULT_SEED, false);
        assert!(unshielded.all_identities_closed);
        assert!(!unshielded.all_six_bridges_verified);
        assert!(!unshielded.complex_wave_gabor_helmholtz.axis1_verified);
        assert!(!unshielded.complex_real_isomorphism_hmatrix.axis2_verified);
        assert!(!unshielded.fractional_ctrw_analog_ecc.axis3_verified);
        assert!(!unshielded.multifractal_pdhte_rag_collapse.axis4_verified);
        assert!(!unshielded.weyl_torus_hermite_mehler_dfm.axis5_verified);
        assert!(!unshielded.sna_rlhf_fhrr_cohomology_technium.axis6_verified);
    }

    #[test]
    fn test_fit_bcts_recovers_calibration_and_refuses_degenerate_labels() {
        let mut rng = Rng::new(DEFAULT_SEED);
        let (mut z, mut y) = (Vec::new(), Vec::new());
        for _ in 0..20_000 {
            let label = if rng.next_f64() < 0.5 { 1.0 } else { 0.0 };
            z.push(4.0 * ((2.0 * label - 1.0) + rng.standard_normal()) - 0.5);
            y.push(label);
        }
        let (a, b) = fit_bcts(&z, &y).expect("converges on non-separable data");
        assert!(
            (a - 0.5).abs() < 0.03 && (b - 0.25).abs() < 0.05,
            "a={a} b={b}"
        );
        assert!(fit_bcts(&z, &vec![1.0; z.len()]).is_none());
    }

    #[test]
    fn test_rlhf_companion_roots_match_undelayed_closed_form() {
        let rho0 = rlhf_nonconsensus_spectral_radius(1.2, 0.7, 0.6, 0);
        assert!((rho0 - 0.48_f64.sqrt()).abs() < 1e-9, "rho0={rho0}");
    }
}
