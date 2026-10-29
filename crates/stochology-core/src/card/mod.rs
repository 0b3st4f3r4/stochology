//! Structured Epistemological Card (`EpistemologicalCard`, §2.3),
//! zero-dependency YAML parser/serializer, typed Value Objects, and Specification Pattern validator
//! (`audit()`) enforcing the 4 Prohibitions and the 5 Boundary Invariants ($\Phi_1$ to $\Phi_5$).

pub mod canonical;
pub mod model;
pub mod specs;
pub mod value_objects;
pub mod yaml;

pub use model::{
    BridgePerformativity, CardParseError, CausalLadder, ContractViolation, DataAndAuditContract,
    DecisionMakerQuestion, DecisionRuleAndScope, DominantEvent, EpistemologicalCard,
    EpistemologicalHeader, FieldAuthorship, FindingsRegistry, InfoGapRobustness,
    MissingnessMechanism, MurphyCorpDecomposition, NoisePhysics, UncertaintyBudgetAndAuthorship,
    UnitAndMargin, VarianceBudget,
};
pub use specs::{
    CANONICAL_CARD_SPECIFICATIONS, CardSpecification, Filter4CrossAuthorshipSpec,
    InfoGapSniedovichSpec, Lens5CostLossBoundarySpec, Lens5MurphyCorpAndFindingsSpec,
    Phi1SimulabilitySpec, Phi2IndependentUnitSpec, Phi3ProvenanceAndMissingnessSpec,
    Phi4ChouldechovaAndScopeSpec, Phi5ResolutionSpec, Prohibition2LoneMeanSpec,
    Prohibition3PerformativitySpec, Prohibition4CromwellAndEProcessSpec, Step2QuestionSpec,
    VarianceBudgetAndDominantEventSpec,
};
pub use value_objects::{
    ChouldechovaRatioMetrics, EProcessMetrics, MoultonClusterMetrics, NonlinearSummaryMetrics,
    PerformativeDivergenceMetrics, RrmContractionMetrics,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_card_passes_audit_and_yaml_roundtrip() {
        let c = EpistemologicalCard::canonical_example();
        assert_eq!(c.audit(), Ok(()));
        let yaml = c.to_yaml();
        let parsed = EpistemologicalCard::from_yaml(&yaml).expect("YAML parse must succeed");
        assert_eq!(parsed.audit(), Ok(()));
        assert_eq!(parsed, c);
    }

    #[test]
    fn card_rejects_cromwell_murphy_and_authorship_violations() {
        let mut c = EpistemologicalCard::canonical_example();
        c.decision_rule_and_scope.event_probability_p = 0.0; // Violates Cromwell
        c.uncertainty_budget_and_authorship
            .field_authorship
            .adversarial_reviewer = "inference".into(); // Violates Filter 4
        c.data_and_audit_contract
            .murphy_corp_decomposition
            .total_score_brier_or_crps = 0.999; // Violates Murphy-CORP
        let errs = c.audit().expect_err("must reject defective card");
        assert!(errs.iter().any(|e| e.gate_code == "PROHIBITION_4_CROMWELL"));
        assert!(
            errs.iter()
                .any(|e| e.gate_code == "FILTER4_CROSS_AUTHORSHIP")
        );
        assert!(errs.iter().any(|e| e.gate_code == "LENS5_MURPHY_CORP"));
    }

    #[test]
    fn card_audit_enforces_numeric_values_for_seed_jensen_rrm_eprocess_and_nan() {
        let mut c = EpistemologicalCard::canonical_example();
        c.data_and_audit_contract.seed_and_environment = "seed=42; rustc=1.97".into();
        c.epistemological_header.nonlinearity =
            "f(E[X])=1420.0, E[f(X)]=1890.0, Jensen gap Delta_J=+999.0, median=1510.0, q05=820.0, q95=4120.0, CRPS=215.4".into();
        c.epistemological_header
            .noise_physics
            .bridge_l3_l1_performativity
            .rrm_convergence_condition =
            "eps * beta / gamma = 0.80 * 1.50 / 1.00 = 1.20 < 1".into();
        c.data_and_audit_contract.e_process_multiplicity =
            "Cumulative e-process E_1 = 5.0 (threshold 1/alpha = 20.0 for alpha = 0.05)".into();
        c.uncertainty_budget_and_authorship.variance_budget.inputs = f64::NAN;

        let errs = c.audit().expect_err("must reject numeric violations");
        assert!(errs.iter().any(|e| e.gate_code == "PHI1_SEED"));
        assert!(
            errs.iter()
                .any(|e| e.gate_code == "PROHIBITION_2_LONE_MEAN")
        );
        assert!(
            errs.iter()
                .any(|e| e.gate_code == "PROHIBITION_3_RRM_CONTRACTION")
        );
        assert!(
            errs.iter()
                .any(|e| e.gate_code == "PROHIBITION_4_E_PROCESS")
        );
        assert!(errs.iter().any(|e| e.gate_code == "VARIANCE_BUDGET"));
    }

    #[test]
    fn yaml_escapes_quotes_colons_backslashes_and_newlines_and_rejects_nan() {
        let mut c = EpistemologicalCard::canonical_example();
        c.decision_maker_question.altered_action =
            "Action with \"quotes\", colon: value, backslash \\ and\nnewline".into();
        let yaml = c.to_yaml();
        let parsed = EpistemologicalCard::from_yaml(&yaml).expect("escaped YAML must parse");
        assert_eq!(parsed, c);

        let nan_yaml = yaml.replace("inputs: 0.45", "inputs: NaN");
        assert!(EpistemologicalCard::from_yaml(&nan_yaml).is_err());
    }

    #[test]
    fn trojan_horse_card_is_rejected_across_all_defective_gates() {
        let trojan_yaml = r#"id: "trojan_horse_adversarial_bypass"
layer: "L2"
uncertainty: "epistemic_specification"

decision_maker_question:
  who_decides: "The System / Data Science Team"
  altered_action: "Set a single annual staffing average across all shifts"
  question: "Should we deploy the model?"

epistemological_header:
  generating_process: "None"
  nonlinearity: "f(E[X])=100.0, E[f(X)]=100.0, Jensen gap Delta_J=0.0, median=10.0, q05=10.0, q95=10.001, CRPS=999999.0"
  unit_and_margin:
    independent_unit: "transactional row in a 5-minute window"
    nominal_sample_size_n: 25028
    weighted_cluster_mean_m_tilde: "m_tilde = 1.0 (Moulton-Campbell ignored, CV = 0)"
    threshold_exceedance_icc: "rho_I(0.90) = 0.85 -> DEFF = 1.0 (unadjusted)"
    effective_sample_size_n_eff: 25028
    unit_margin_of_error: "0.0%"
  noise_physics:
    stochastic_regime: "Gaussian i.i.d."
    absorbing_states: "None"
    bridge_L3_L1_performativity:
      agent_type: "performative"
      wasserstein_sensitivity_epsilon: 0.99
      rrm_convergence_condition: "eps * beta / gamma = 50.0 / 1.0 = 0.99 < 1 (oscillatory divergence masked as 3-number string)"
      stability_optimality_divergence: "theta_PS = 0.0 vs theta_PO = 0.00001"
  causal_ladder:
    pearl_rung: "1-See"
    identification_strategy: "Observational correlation conditioned on collider (hospitalized patients only); Markov blanket not mapped"
    untestable_assumption: "None"

uncertainty_budget_and_authorship:
  variance_budget:
    inputs: 1.00
    parameters: 0.00
    covariance: 0.00
    structural_discrepancy_gp: "None (delta(x) = 0 assumed)"
  dominant_event:
    description: "Negligible noise event"
    probability_p: 0.50
    base_rate_floor_epsilon: 0.001
    cost_share: 0.01
  info_gap_robustness:
    critical_requirement_rc: "None"
    robustness_horizon_alpha_hat: "alpha_hat = 0.0 (fragile around nominal estimate)"
    opportuneness_horizon_beta_hat: "beta_hat = 999.0"
    global_maximin_safeguard: "No-Man's Land: Wald global Maximin bound = -999999.0 (unbounded ruin, irreversible action)"
  field_authorship:
    null_model: "Author A"
    decision_rule: "Author A ( Decision )"
    adversarial_reviewer: "Author A ( Reviewer )"

data_and_audit_contract:
  dataset_path: "none"
  dataset_sha256: "invalid_hash"
  missingness_mechanism: "MNAR (70% missing; complete-case analysis used, Manski bounds ignored)"
  seed_and_environment: "seed=2026"
  null_model: "none (skipped)"
  preregistration_hash: "0000000000000000000000000000000000000000000000000000000000000000"
  attempt_counter_K: 50
  e_process_multiplicity: "E_50 = 50.1 (threshold = 1.001)"
  murphy_corp_decomposition:
    total_score_brier_or_crps: 0.500
    miscalibration_REL_MCB: 0.250
    resolution_RES_DSC: 0.000
    base_uncertainty_UNC0: 0.250
  findings:
    registered: 1
    used: 1
    unused:
      []

decision_rule_and_scope:
  event_probability_p: 0.50
  base_rate_floor_epsilon: 0.001
  protection_cost_C: 10.0
  event_loss_L: 100.0
  mitigation_efficacy_eta: 1.0
  critical_threshold_p_star: 0.10000
  declared_act: "protect"
  scope_statement: "Applies universally everywhere forever"
  who_pays_for_error: "Chouldechova not audited; model assumed equally fair on PPV, FPR, and FNR simultaneously"
  exit_clause: "We will review the model at management discretion when convenient"
"#;

        let card = EpistemologicalCard::from_yaml(trojan_yaml).expect("Trojan YAML must parse");
        let violations = card
            .audit()
            .expect_err("Trojan Horse card must be rejected");
        assert!(
            violations.len() >= 10,
            "Expected at least 10 gate violations, got {}: {violations:?}",
            violations.len()
        );
        for expected_gate in [
            "STEP2_DECISION_MAKER",
            "PROHIBITION_2_LONE_MEAN",
            "PROHIBITION_3_RRM_CONTRACTION",
            "PHI1_PHI3_NULL_MODEL",
            "PHI2_PSEUDOREPLICATION",
            "PHI2_MOULTON_CAMPBELL",
            "PHI2_EXCEEDANCE_ICC",
            "PHI2_N_EFF",
            "PHI3_DATASET_PROVENANCE",
            "PHI3_PREREGISTRATION_HASH",
            "PHI3_PEARL_LADDER",
            "VARIANCE_BUDGET",
            "DOMINANT_EVENT_COST_SHARE",
            "INFO_GAP_SNIEDOVICH",
            "PROHIBITION_4_E_PROCESS",
            "LENS5_MURPHY_CORP",
            "PHI4_CHOULDECHOVA",
            "PHI4_SCOPE_AND_EXIT",
            "PHI5_RESOLUTION",
        ] {
            assert!(
                violations.iter().any(|v| v.gate_code == expected_gate),
                "Missing expected violation for gate '{expected_gate}' in {violations:?}"
            );
        }
    }
}
