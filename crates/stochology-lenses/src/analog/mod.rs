//! Physical, Analog, and Wave-Interference Computing (§1.3.8 — *Computação Analógica, Física e por Interferência de Ondas*).

pub mod empirical;
pub mod model_collapse;
pub mod ode_aimc;
pub mod solvers;
pub mod wave_organoid;

pub use empirical::EmpiricalWaveAnalogAudit;
pub use model_collapse::{
    GerstgrasserAccumulateAudit, SyntheticDataPolicy, VonNeumannModelCollapseAudit,
};
pub use ode_aimc::{AimcAnalogWallHpInvAudit, NativeOdeIsomorphismAudit};
pub use wave_organoid::{SensoryDpiOrganoidHybridAudit, WaveInterferenceLearningAudit};

/// Complete 5-Axis Physical, Analog, and Wave-Interference Computing Synthesis (`stochology_lenses::analog`).
#[derive(Debug, Clone, PartialEq)]
pub struct AnalogWaveComputingSynthesis {
    /// Axis 1: von Neumann energy wall (Horowitz 45nm CMOS) & synthetic model collapse vs. neurosymbolic anchoring.
    pub von_neumann_and_collapse: VonNeumannModelCollapseAudit,
    /// Axis 2: Mechanical Mass-Spring-Damper $\leftrightarrow$ Analog RLC Circuit ODE isomorphism & Liquid ODEs.
    pub ode_isomorphism: NativeOdeIsomorphismAudit,
    /// Axis 3: AIMC Crossbar (Ohm/Kirchhoff), Analog Wall vs. NAT, and Peking University 24-bit `HP-INV` solver.
    pub aimc_analog_wall_hp_inv: AimcAnalogWallHpInvAudit,
    /// Axis 4: Wave-Interference Learning (`SIREN` phase/amplitude Helmholtz PDE + Stanford Wave-Equation RNN).
    pub wave_interference: WaveInterferenceLearningAudit,
    /// Axis 5: The 20-Watt Brain Sensory Funnel (`DpiMarkovChain`) & Post-Digital Ecosystem trade-off.
    pub sensory_dpi_and_ecosystem: SensoryDpiOrganoidHybridAudit,
    /// Master gate flag confirming that all 5 physical, analog, and wave-computing axes pass audit.
    pub all_five_axes_verified: bool,
}

impl AnalogWaveComputingSynthesis {
    /// Evaluates all 5 axes under the canonical protocol seed (`seed = 2026`).
    pub fn evaluate_canonical(seed: u64) -> Self {
        Self::evaluate_with_mode(seed, true)
    }

    /// Evaluates all 5 axes with configurable safeguarding (`enable_safeguards = false` triggers negative proof `--defect analog_wall`).
    pub fn evaluate_with_mode(seed: u64, enable_safeguards: bool) -> Self {
        let von_neumann_and_collapse =
            VonNeumannModelCollapseAudit::evaluate(seed, enable_safeguards);
        let ode_isomorphism = NativeOdeIsomorphismAudit::evaluate();
        let aimc_analog_wall_hp_inv =
            AimcAnalogWallHpInvAudit::evaluate(seed.wrapping_add(17), enable_safeguards);
        let wave_interference = WaveInterferenceLearningAudit::evaluate();
        let sensory_dpi_and_ecosystem = SensoryDpiOrganoidHybridAudit::evaluate();

        let all_five_axes_verified = von_neumann_and_collapse.axis1_verified
            && ode_isomorphism.axis2_verified
            && aimc_analog_wall_hp_inv.axis3_verified
            && wave_interference.axis4_verified
            && sensory_dpi_and_ecosystem.axis5_verified;

        Self {
            von_neumann_and_collapse,
            ode_isomorphism,
            aimc_analog_wall_hp_inv,
            wave_interference,
            sensory_dpi_and_ecosystem,
            all_five_axes_verified,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use stochology_core::DEFAULT_SEED;

    #[test]
    fn test_axis1_von_neumann_and_model_collapse() {
        let audit = VonNeumannModelCollapseAudit::evaluate(DEFAULT_SEED, true);
        assert!(audit.dram_to_fp32_add_ratio > 711.0);
        assert_eq!(audit.dram_to_int32_add_ratio, 6400.0);
        assert!(audit.unanchored_synthetic_variance_retention < 0.25);
        assert!(audit.neurosymbolic_anchored_variance_retention > 0.90);
        assert!(audit.axis1_verified);
    }

    #[test]
    fn test_axis2_native_ode_isomorphism_and_ltc() {
        let audit = NativeOdeIsomorphismAudit::evaluate();
        assert!(audit.max_isomorphism_discrepancy < 1e-14);
        assert!(audit.rk4_vs_analytical_max_error < 1e-7);
        assert!(audit.accuracy_improvement_over_coarse_digital > 1_000.0);
        assert!(audit.axis2_verified);
    }

    #[test]
    fn test_axis3_aimc_analog_wall_nat_and_pku_hp_inv() {
        let audit = AimcAnalogWallHpInvAudit::evaluate(DEFAULT_SEED, true);
        assert!(audit.standard_accuracy_at_sigma_060 < 0.48);
        assert!(audit.nat_accuracy_at_sigma_060 >= 0.58);
        assert!(audit.pku_hp_inv_final_relative_error < 1e-7);
        assert!(audit.pku_hp_inv_iterations_to_fp32 <= 10);
        assert!(audit.axis3_verified);
    }

    #[test]
    fn test_axis4_wave_interference_siren_and_wave_rnn() {
        let audit = WaveInterferenceLearningAudit::evaluate();
        assert!(audit.phase_amplitude_compression_ratio >= 35.0);
        assert!(audit.siren_helmholtz_laplacian_mse < 1e-8);
        assert!(audit.relu_helmholtz_laplacian_mse > 500.0);
        assert!(audit.wave_rnn_target_probe_selectivity >= 0.85);
        assert!(audit.axis4_verified);
    }

    #[test]
    fn test_axis5_sensory_dpi_and_post_digital_ecosystem() {
        let audit = SensoryDpiOrganoidHybridAudit::evaluate();
        assert_eq!(audit.total_sensory_compression_ratio, 1.6e8);
        assert!(audit.sensory_dpi_chain.dpi_satisfied);
        assert!(audit.axis5_verified);
    }

    #[test]
    fn test_full_analog_wave_computing_synthesis_and_negative_defect() {
        let pos = AnalogWaveComputingSynthesis::evaluate_canonical(DEFAULT_SEED);
        assert!(pos.all_five_axes_verified);

        let neg = AnalogWaveComputingSynthesis::evaluate_with_mode(DEFAULT_SEED, false);
        assert!(!neg.all_five_axes_verified);
        assert!(!neg.von_neumann_and_collapse.axis1_verified);
        assert!(!neg.aimc_analog_wall_hp_inv.axis3_verified);

        let gerst_pos = GerstgrasserAccumulateAudit::canonical_benchmark(
            DEFAULT_SEED,
            SyntheticDataPolicy::Accumulate,
        );
        assert!(gerst_pos.declared_policy_bounded);
        assert!(gerst_pos.simulation_closure_error < 0.10);
        assert!((gerst_pos.analytical_replace_risk_n - 1.1363636).abs() < 1e-6);
        assert!((gerst_pos.analytical_accumulate_risk_n - 0.1761100).abs() < 1e-6);
        assert!((gerst_pos.asymptotic_basel_ceiling - 0.1869243).abs() < 1e-6);

        let gerst_neg = GerstgrasserAccumulateAudit::canonical_benchmark(
            DEFAULT_SEED,
            SyntheticDataPolicy::Replace,
        );
        assert!(!gerst_neg.declared_policy_bounded);
        assert!(gerst_neg.simulation_closure_error < 0.10);
        assert!(gerst_neg.replace_to_accumulate_ratio > 4.0);
    }

    #[test]
    fn gerstgrasser_verdict_comes_from_simulation_not_from_declared_policy() {
        let one_gen = GerstgrasserAccumulateAudit::evaluate(
            DEFAULT_SEED,
            1.0,
            5,
            50,
            1,
            SyntheticDataPolicy::Replace,
        );
        assert!(one_gen.declared_policy_bounded);
        assert!(
            (one_gen.simulated_replace_risk_n - one_gen.simulated_accumulate_risk_n).abs() < 1e-12
        );
    }
}
