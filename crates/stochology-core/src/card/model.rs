//! Domain model structs and typed errors for [`EpistemologicalCard`] (§2.3).

use std::fmt;

use crate::layers::{Layer, UncertaintyClass};

use super::value_objects::{
    ChouldechovaRatioMetrics, EProcessMetrics, MoultonClusterMetrics, NonlinearSummaryMetrics,
    PerformativeDivergenceMetrics, RrmContractionMetrics,
};

/// Missingness mechanism classification according to Donald Rubin (Invariant $\Phi_3$).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MissingnessMechanism {
    Mcar,
    Mar,
    Mnar,
}

impl MissingnessMechanism {
    pub fn from_prefix(s: &str) -> Option<Self> {
        let t = s.trim();
        if t.starts_with("MCAR") {
            Some(Self::Mcar)
        } else if t.starts_with("MAR") {
            Some(Self::Mar)
        } else if t.starts_with("MNAR") {
            Some(Self::Mnar)
        } else {
            None
        }
    }
}

/// Decision-maker's 3-part operational question (Step 2 of SIGA & Prohibition 1).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionMakerQuestion {
    pub who_decides: String,
    pub altered_action: String,
    pub question: String,
}

/// Independent unit, Moulton-Campbell cluster correction, and exceedance ICC (Invariant $\Phi_2$).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnitAndMargin {
    pub independent_unit: String,
    pub nominal_sample_size_n: usize,
    pub weighted_cluster_mean_m_tilde: String,
    pub threshold_exceedance_icc: String,
    pub effective_sample_size_n_eff: usize,
    pub unit_margin_of_error: String,
}

impl UnitAndMargin {
    /// Parses the structured [`MoultonClusterMetrics`] Value Object from the declaration fields.
    pub fn parsed_moulton_metrics(&self) -> Option<MoultonClusterMetrics> {
        MoultonClusterMetrics::parse_from_declarations(
            &self.weighted_cluster_mean_m_tilde,
            &self.threshold_exceedance_icc,
        )
    }
}

/// Performative prediction audit on Bridge $L_3 \to L_1$ (Prohibition 3).
#[derive(Debug, Clone, PartialEq)]
pub struct BridgePerformativity {
    pub agent_type: String,
    pub wasserstein_sensitivity_epsilon: f64,
    pub rrm_convergence_condition: String,
    pub stability_optimality_divergence: String,
}

impl BridgePerformativity {
    /// Parses the structured [`RrmContractionMetrics`] Value Object from `rrm_convergence_condition`.
    pub fn parsed_rrm_metrics(&self) -> Option<RrmContractionMetrics> {
        RrmContractionMetrics::parse_from_declaration(&self.rrm_convergence_condition)
    }

    /// Parses the structured [`PerformativeDivergenceMetrics`] Value Object from `stability_optimality_divergence`.
    pub fn parsed_divergence_metrics(&self) -> Option<PerformativeDivergenceMetrics> {
        PerformativeDivergenceMetrics::parse_from_declaration(&self.stability_optimality_divergence)
    }
}

/// Noise physics, absorbing barriers, and performative dynamics.
#[derive(Debug, Clone, PartialEq)]
pub struct NoisePhysics {
    pub stochastic_regime: String,
    pub absorbing_states: String,
    pub bridge_l3_l1_performativity: BridgePerformativity,
}

/// Pearl's Causal Ladder declaration (Lens 4 & Invariant $\Phi_3$).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CausalLadder {
    pub pearl_rung: String,
    pub identification_strategy: String,
    pub untestable_assumption: String,
}

/// Epistemological header of the card (§2.3).
#[derive(Debug, Clone, PartialEq)]
pub struct EpistemologicalHeader {
    pub generating_process: String,
    pub nonlinearity: String,
    pub unit_and_margin: UnitAndMargin,
    pub noise_physics: NoisePhysics,
    pub causal_ladder: CausalLadder,
}

impl EpistemologicalHeader {
    /// Parses the structured [`NonlinearSummaryMetrics`] Value Object from `nonlinearity`.
    pub fn parsed_nonlinearity_metrics(&self) -> Option<NonlinearSummaryMetrics> {
        NonlinearSummaryMetrics::parse_from_declaration(&self.nonlinearity)
    }
}

/// Variance budget and Kennedy-O'Hagan Gaussian Process structural discrepancy (§1.2).
#[derive(Debug, Clone, PartialEq)]
pub struct VarianceBudget {
    pub inputs: f64,
    pub parameters: f64,
    pub covariance: f64,
    pub structural_discrepancy_gp: String,
}

/// Dominant cost-driving event under Cromwell's Rule.
#[derive(Debug, Clone, PartialEq)]
pub struct DominantEvent {
    pub description: String,
    pub probability_p: f64,
    pub base_rate_floor_epsilon: f64,
    pub cost_share: f64,
}

/// Info-Gap robustness/opportuneness horizons and Sniedovich's Global Maximin Safeguard (§1.2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InfoGapRobustness {
    pub critical_requirement_rc: String,
    pub robustness_horizon_alpha_hat: String,
    pub opportuneness_horizon_beta_hat: String,
    pub global_maximin_safeguard: String,
}

/// Independent cross-authorship and adversarial review (Filter 4).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldAuthorship {
    pub null_model: String,
    pub decision_rule: String,
    pub adversarial_reviewer: String,
}

/// Uncertainty budget, Info-Gap robustness, and cross-authorship section.
#[derive(Debug, Clone, PartialEq)]
pub struct UncertaintyBudgetAndAuthorship {
    pub variance_budget: VarianceBudget,
    pub dominant_event: DominantEvent,
    pub info_gap_robustness: InfoGapRobustness,
    pub field_authorship: FieldAuthorship,
}

/// Isotonic CORP-Murphy decomposition via PAV ($\mathrm{BS} = \mathrm{REL}_{\mathrm{CORP}} - \mathrm{RES}_{\mathrm{CORP}} + \mathrm{UNC}_0$, Lens 5).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MurphyCorpDecomposition {
    pub total_score_brier_or_crps: f64,
    pub miscalibration_rel_mcb: f64,
    pub resolution_res_dsc: f64,
    pub base_uncertainty_unc0: f64,
}

/// Accounting of registered, used, and discarded findings (Prohibition 4).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FindingsRegistry {
    pub registered: usize,
    pub used: usize,
    pub unused: Vec<String>,
}

/// Data contract, dataset provenance (`data/*.csv`), pre-registration seal, e-process multiplicity, and CORP-Murphy audit.
#[derive(Debug, Clone, PartialEq)]
pub struct DataAndAuditContract {
    pub dataset_path: String,
    pub dataset_sha256: String,
    pub missingness_mechanism: String,
    pub seed_and_environment: String,
    pub null_model: String,
    pub preregistration_hash: String,
    pub attempt_counter_k: usize,
    pub e_process_multiplicity: String,
    pub murphy_corp_decomposition: MurphyCorpDecomposition,
    pub findings: FindingsRegistry,
}

impl DataAndAuditContract {
    /// Parses the structured [`EProcessMetrics`] Value Object from `e_process_multiplicity`.
    pub fn parsed_e_process_metrics(&self) -> Option<EProcessMetrics> {
        EProcessMetrics::parse_from_declaration(&self.e_process_multiplicity)
    }
}

/// Cost-loss decision boundary $p^* = \frac{C}{\eta L}$, Chouldechova fairness audit, and numeric exit clause ($\Phi_4$).
#[derive(Debug, Clone, PartialEq)]
pub struct DecisionRuleAndScope {
    pub event_probability_p: f64,
    pub base_rate_floor_epsilon: f64,
    pub protection_cost_c: f64,
    pub event_loss_l: f64,
    pub mitigation_efficacy_eta: f64,
    pub critical_threshold_p_star: f64,
    pub declared_act: String,
    pub scope_statement: String,
    pub who_pays_for_error: String,
    pub exit_clause: String,
}

impl DecisionRuleAndScope {
    /// Parses the structured [`ChouldechovaRatioMetrics`] Value Object from `who_pays_for_error`.
    pub fn parsed_chouldechova_metrics(&self) -> Option<ChouldechovaRatioMetrics> {
        ChouldechovaRatioMetrics::parse_from_declaration(&self.who_pays_for_error)
    }
}

/// Complete Epistemological Card.
#[derive(Debug, Clone, PartialEq)]
pub struct EpistemologicalCard {
    pub id: String,
    pub layer: Layer,
    pub uncertainty: UncertaintyClass,
    pub decision_maker_question: DecisionMakerQuestion,
    pub epistemological_header: EpistemologicalHeader,
    pub uncertainty_budget_and_authorship: UncertaintyBudgetAndAuthorship,
    pub data_and_audit_contract: DataAndAuditContract,
    pub decision_rule_and_scope: DecisionRuleAndScope,
}

/// Formal contract violation returned by [`EpistemologicalCard::audit`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContractViolation {
    pub gate_code: &'static str,
    pub description: String,
}

impl fmt::Display for ContractViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.gate_code, self.description)
    }
}

impl std::error::Error for ContractViolation {}

/// Typed error returned when parsing an [`EpistemologicalCard`] from YAML fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CardParseError {
    MissingField(String),
    InvalidNumber { field: String, detail: String },
    NonFiniteNumber { field: String, value: String },
    InvalidInteger { field: String, detail: String },
    InvalidLayer(String),
    InvalidUncertaintyClass(String),
}

impl fmt::Display for CardParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingField(k) => write!(f, "missing mandatory YAML field: '{k}'"),
            Self::InvalidNumber { field, detail } => {
                write!(f, "invalid numeric value in '{field}': {detail}")
            }
            Self::NonFiniteNumber { field, value } => {
                write!(f, "non-finite numeric value in '{field}': {value}")
            }
            Self::InvalidInteger { field, detail } => {
                write!(f, "invalid integer in '{field}': {detail}")
            }
            Self::InvalidLayer(v) => write!(f, "invalid layer '{v}' (use L1, L2, or L3)"),
            Self::InvalidUncertaintyClass(v) => write!(f, "invalid uncertainty class '{v}'"),
        }
    }
}

impl std::error::Error for CardParseError {}

impl EpistemologicalCard {
    /// Builds the canonical pre-analysis specification payload hashed into `preregistration_hash`.
    ///
    /// Covers only pre-analysis design fields (`id`, `layer`, `independent_unit`, `dataset_path`,
    /// `dataset_sha256`, `null_model`, `critical_threshold_p_star`) so the pre-registration seal
    /// is independent of post-hoc observed statistics.
    pub fn compute_preregistration_payload(&self) -> String {
        format!(
            "stochology_prereg_v1|id={}|layer={}|unit={}|dataset={}|sha256={}|null={}|p_star={:.5}",
            self.id.trim(),
            self.layer.code(),
            self.epistemological_header
                .unit_and_margin
                .independent_unit
                .trim(),
            self.data_and_audit_contract.dataset_path.trim(),
            self.data_and_audit_contract.dataset_sha256.trim(),
            self.data_and_audit_contract.null_model.trim(),
            self.decision_rule_and_scope.critical_threshold_p_star,
        )
    }

    /// Computes the expected SHA-256 hex digest of [`Self::compute_preregistration_payload`].
    pub fn compute_preregistration_hash(&self) -> String {
        crate::sha256::sha256_hex(self.compute_preregistration_payload().as_bytes())
    }
}
