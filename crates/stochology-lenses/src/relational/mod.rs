//! Relational Ontology and Complex Systems (§1.3.7 — *Ontologia Relacional e Sistemas Complexos*):
//!
//! Formalizes in pure Rust the six transdisciplinary axes linking relational ontology,
//! non-equilibrium thermodynamics, emergent phase transitions, cybernetic homeostasis,
//! and the Ontological Stack to the Stochology Protocol (`seed = 2026`).

pub mod daisyworld;
pub mod figuration_autopoiesis;
pub mod schelling;

pub use daisyworld::{DaisyworldEquilibriumPoint, DaisyworldHomeostasisAudit, DaisyworldSystem};
pub use figuration_autopoiesis::{DissipativeAutopoieticAudit, RelationalFigurationAudit};
pub use schelling::{SchellingCell, SchellingPhaseTransitionAudit, SchellingSegregationModel};

use stochology_core::NonlinearSummary;

/// Axis 6: Complete Transdisciplinary Synthesis of Relational Ontology and Complex Systems
/// across all 6 axes of the Ontological Stack (Morin, Mirowski, Arthur, Beinhocker — `seed = 2026`).
#[derive(Debug, Clone, PartialEq)]
pub struct RelationalComplexSystemsSynthesis {
    /// Axis 1 & 4: Relational Vacuity (*Śūnyatā* / RQM / OSR) and *Homo clausus* vs. *Homines aperti*.
    pub relational_figuration: RelationalFigurationAudit,
    /// Axis 2: Prigogine Dissipative Structure ($dS = d_i S + d_e S$) and Maturana-Varela Autopoiesis.
    pub dissipative_autopoiesis: DissipativeAutopoieticAudit,
    /// Axis 3: Schelling (1971) Emergent Segregation Phase Transition (*More Is Different*).
    pub schelling_emergence: SchellingPhaseTransitionAudit,
    /// Axis 5: Watson & Lovelock (1983) *Daisyworld* Cybernetic Homeostasis.
    pub daisyworld_homeostasis: DaisyworldHomeostasisAudit,
    /// Axis 6: Nonlinearity Jensen Gap $\Delta_J = \mathbb{E}[f(X)] - f(\mathbb{E}[X])$ refuting Neoclassical equilibrium atomism.
    pub ontological_stack_jensen_gap: f64,
    /// Whether all 6 axes of the Relational & Complex Systems synthesis pass verification.
    pub all_six_axes_verified: bool,
}

impl RelationalComplexSystemsSynthesis {
    /// Evaluates the complete 6-axis synthesis deterministically under `seed = 2026`.
    pub fn evaluate_canonical(seed: u64) -> Self {
        let rel = RelationalFigurationAudit::evaluate(seed);
        let diss = DissipativeAutopoieticAudit::simulate_vortex(60, 0.12);
        let schelling = SchellingSegregationModel::audit_phase_transition(seed);
        let daisy = DaisyworldSystem::canonical().audit_homeostasis();

        let macro_states = [
            daisy.faint_sun_point.lifeless_bare_temp_c,
            daisy.present_sun_point.lifeless_bare_temp_c,
            daisy.bright_sun_point.lifeless_bare_temp_c,
            daisy.post_collapse_point.lifeless_bare_temp_c,
        ];
        let jensen = NonlinearSummary::evaluate(&macro_states, |t| (t - 22.5) * (t - 22.5), 2.0);

        let all_ok = rel.relational_vacuity_verified
            && diss.dissipative_autopoiesis_valid
            && schelling.phase_transition_verified
            && daisy.homeostasis_verified
            && jensen.jensen_gap > 50.0;

        Self {
            relational_figuration: rel,
            dissipative_autopoiesis: diss,
            schelling_emergence: schelling,
            daisyworld_homeostasis: daisy,
            ontological_stack_jensen_gap: jensen.jensen_gap,
            all_six_axes_verified: all_ok,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use stochology_core::DEFAULT_SEED;

    #[test]
    fn relational_vacuity_and_homo_clausus_audit_holds() {
        let audit = RelationalFigurationAudit::evaluate(DEFAULT_SEED);
        assert!(audit.relational_vacuity_verified);
        assert!(audit.relational_mutual_information_nats > 0.30);
        assert!(audit.isolated_svabhava_mutual_information_nats < 0.005);
        assert!(audit.homo_clausus_pseudoreplication_deff > 5.0);
        assert!(
            audit.co_evolutionary_stackelberg_theta_po > audit.puddle_passive_stability_theta_ps
        );
    }

    #[test]
    fn prigogine_dissipative_and_autopoietic_closure_holds() {
        let vortex = DissipativeAutopoieticAudit::simulate_vortex(60, 0.12);
        assert!(vortex.dissipative_autopoiesis_valid);
        assert!(vortex.internal_entropy_production_di_s > 0.0);
        assert!(vortex.boundary_entropy_flux_de_s < 0.0);
        assert!(vortex.net_open_entropy_change_ds <= 0.0);
        assert!(vortex.material_turnover_fraction > 0.999);
        assert!(vortex.autopoietic_pattern_fidelity > 0.999);
    }

    #[test]
    fn schelling_segregation_exhibits_abrupt_phase_transition_under_mild_tolerance() {
        let audit = SchellingSegregationModel::audit_phase_transition(DEFAULT_SEED);
        assert!(audit.phase_transition_verified);
        assert!(
            audit.integrated_similarity_at_low_tau < 0.60,
            "sub-critical tau={} must remain integrated, got {}",
            audit.low_threshold_tau,
            audit.integrated_similarity_at_low_tau
        );
        assert!(
            audit.emergent_segregation_at_mild_tau >= 0.78,
            "mild tau={} must trigger tipping cascade into >= 78% segregation, got {}",
            audit.mild_threshold_tau,
            audit.emergent_segregation_at_mild_tau
        );
        assert!(audit.heteropathic_emergence_gap > 0.38);
    }

    #[test]
    fn watson_lovelock_daisyworld_self_regulates_planetary_temperature_around_optimum() {
        let audit = DaisyworldSystem::canonical().audit_homeostasis();
        assert!(audit.homeostasis_verified);
        assert!(
            audit.faint_sun_point.planetary_temp_c
                > audit.faint_sun_point.lifeless_bare_temp_c + 5.0
        );
        assert!(
            audit.bright_sun_point.planetary_temp_c
                < audit.bright_sun_point.lifeless_bare_temp_c - 5.0
        );
        assert!(audit.homeostatic_mean_abs_error_c < 3.0);
        assert!(audit.lifeless_mean_abs_error_c > 8.0);
        assert!(audit.coexistence_feedback_slope_dt_dl <= 0.0);
        assert!(audit.post_collapse_point.black_daisy_area_alpha_b < 1e-4);
        assert!(audit.post_collapse_point.white_daisy_area_alpha_w < 1e-4);
    }

    #[test]
    fn full_six_axis_relational_complex_systems_synthesis_passes() {
        let syn = RelationalComplexSystemsSynthesis::evaluate_canonical(DEFAULT_SEED);
        assert!(syn.all_six_axes_verified);
    }
}
