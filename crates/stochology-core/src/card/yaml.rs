//! Zero-dependency YAML serializer and parser for [`EpistemologicalCard`] (§2.3).

use std::collections::BTreeMap;

use crate::layers::{Layer, UncertaintyClass};

use super::model::{
    BridgePerformativity, CardParseError, CausalLadder, DataAndAuditContract,
    DecisionMakerQuestion, DecisionRuleAndScope, DominantEvent, EpistemologicalCard,
    EpistemologicalHeader, FieldAuthorship, FindingsRegistry, InfoGapRobustness,
    MurphyCorpDecomposition, NoisePhysics, UncertaintyBudgetAndAuthorship, UnitAndMargin,
    VarianceBudget,
};

impl EpistemologicalCard {
    /// Serializes the Epistemological Card into the structured YAML format (§2.3).
    pub fn to_yaml(&self) -> String {
        let unused_yaml = if self.data_and_audit_contract.findings.unused.is_empty() {
            "      []".to_string()
        } else {
            self.data_and_audit_contract
                .findings
                .unused
                .iter()
                .map(|item| format!("      - \"{}\"", escape_yaml_str(item)))
                .collect::<Vec<_>>()
                .join("\n")
        };

        format!(
            r#"id: "{id}"
layer: "{layer}"
uncertainty: "{uncertainty}"

decision_maker_question:
  who_decides: "{who_decides}"
  altered_action: "{altered_action}"
  question: "{question}"

epistemological_header:
  generating_process: "{generating_process}"
  nonlinearity: "{nonlinearity}"
  unit_and_margin:
    independent_unit: "{independent_unit}"
    nominal_sample_size_n: {nominal_sample_size_n}
    weighted_cluster_mean_m_tilde: "{weighted_cluster_mean_m_tilde}"
    threshold_exceedance_icc: "{threshold_exceedance_icc}"
    effective_sample_size_n_eff: {effective_sample_size_n_eff}
    unit_margin_of_error: "{unit_margin_of_error}"
  noise_physics:
    stochastic_regime: "{stochastic_regime}"
    absorbing_states: "{absorbing_states}"
    bridge_L3_L1_performativity:
      agent_type: "{agent_type}"
      wasserstein_sensitivity_epsilon: {wasserstein_sensitivity_epsilon}
      rrm_convergence_condition: "{rrm_convergence_condition}"
      stability_optimality_divergence: "{stability_optimality_divergence}"
  causal_ladder:
    pearl_rung: "{pearl_rung}"
    identification_strategy: "{identification_strategy}"
    untestable_assumption: "{untestable_assumption}"

uncertainty_budget_and_authorship:
  variance_budget:
    inputs: {inputs:.2}
    parameters: {parameters:.2}
    covariance: {covariance:.2}
    structural_discrepancy_gp: "{structural_discrepancy_gp}"
  dominant_event:
    description: "{ev_description}"
    probability_p: {ev_prob}
    base_rate_floor_epsilon: {ev_eps}
    cost_share: {ev_share:.2}
  info_gap_robustness:
    critical_requirement_rc: "{critical_requirement_rc}"
    robustness_horizon_alpha_hat: "{robustness_horizon_alpha_hat}"
    opportuneness_horizon_beta_hat: "{opportuneness_horizon_beta_hat}"
    global_maximin_safeguard: "{global_maximin_safeguard}"
  field_authorship:
    null_model: "{auth_null}"
    decision_rule: "{auth_decision}"
    adversarial_reviewer: "{auth_reviewer}"

data_and_audit_contract:
  dataset_path: "{dataset_path}"
  dataset_sha256: "{dataset_sha256}"
  missingness_mechanism: "{missingness_mechanism}"
  seed_and_environment: "{seed_and_environment}"
  null_model: "{null_model}"
  preregistration_hash: "{preregistration_hash}"
  attempt_counter_K: {attempt_counter_k}
  e_process_multiplicity: "{e_process_multiplicity}"
  murphy_corp_decomposition:
    total_score_brier_or_crps: {total_score:.5}
    miscalibration_REL_MCB: {miscalibration:.5}
    resolution_RES_DSC: {resolution:.5}
    base_uncertainty_UNC0: {base_uncertainty:.5}
  findings:
    registered: {fnd_reg}
    used: {fnd_used}
    unused:
{unused_yaml}

decision_rule_and_scope:
  event_probability_p: {dr_p}
  base_rate_floor_epsilon: {dr_eps}
  protection_cost_C: {dr_c:.1}
  event_loss_L: {dr_l:.1}
  mitigation_efficacy_eta: {dr_eta:.1}
  critical_threshold_p_star: {dr_p_star:.5}
  declared_act: "{declared_act}"
  scope_statement: "{scope_statement}"
  who_pays_for_error: "{who_pays_for_error}"
  exit_clause: "{exit_clause}"
"#,
            id = escape_yaml_str(&self.id),
            layer = self.layer.code(),
            uncertainty = self.uncertainty.code(),
            who_decides = escape_yaml_str(&self.decision_maker_question.who_decides),
            altered_action = escape_yaml_str(&self.decision_maker_question.altered_action),
            question = escape_yaml_str(&self.decision_maker_question.question),
            generating_process = escape_yaml_str(&self.epistemological_header.generating_process),
            nonlinearity = escape_yaml_str(&self.epistemological_header.nonlinearity),
            independent_unit =
                escape_yaml_str(&self.epistemological_header.unit_and_margin.independent_unit),
            nominal_sample_size_n = self
                .epistemological_header
                .unit_and_margin
                .nominal_sample_size_n,
            weighted_cluster_mean_m_tilde = escape_yaml_str(
                &self
                    .epistemological_header
                    .unit_and_margin
                    .weighted_cluster_mean_m_tilde
            ),
            threshold_exceedance_icc = escape_yaml_str(
                &self
                    .epistemological_header
                    .unit_and_margin
                    .threshold_exceedance_icc
            ),
            effective_sample_size_n_eff = self
                .epistemological_header
                .unit_and_margin
                .effective_sample_size_n_eff,
            unit_margin_of_error = escape_yaml_str(
                &self
                    .epistemological_header
                    .unit_and_margin
                    .unit_margin_of_error
            ),
            stochastic_regime =
                escape_yaml_str(&self.epistemological_header.noise_physics.stochastic_regime),
            absorbing_states =
                escape_yaml_str(&self.epistemological_header.noise_physics.absorbing_states),
            agent_type = escape_yaml_str(
                &self
                    .epistemological_header
                    .noise_physics
                    .bridge_l3_l1_performativity
                    .agent_type
            ),
            wasserstein_sensitivity_epsilon = self
                .epistemological_header
                .noise_physics
                .bridge_l3_l1_performativity
                .wasserstein_sensitivity_epsilon,
            rrm_convergence_condition = escape_yaml_str(
                &self
                    .epistemological_header
                    .noise_physics
                    .bridge_l3_l1_performativity
                    .rrm_convergence_condition
            ),
            stability_optimality_divergence = escape_yaml_str(
                &self
                    .epistemological_header
                    .noise_physics
                    .bridge_l3_l1_performativity
                    .stability_optimality_divergence
            ),
            pearl_rung = escape_yaml_str(&self.epistemological_header.causal_ladder.pearl_rung),
            identification_strategy = escape_yaml_str(
                &self
                    .epistemological_header
                    .causal_ladder
                    .identification_strategy
            ),
            untestable_assumption = escape_yaml_str(
                &self
                    .epistemological_header
                    .causal_ladder
                    .untestable_assumption
            ),
            inputs = self
                .uncertainty_budget_and_authorship
                .variance_budget
                .inputs,
            parameters = self
                .uncertainty_budget_and_authorship
                .variance_budget
                .parameters,
            covariance = self
                .uncertainty_budget_and_authorship
                .variance_budget
                .covariance,
            structural_discrepancy_gp = escape_yaml_str(
                &self
                    .uncertainty_budget_and_authorship
                    .variance_budget
                    .structural_discrepancy_gp
            ),
            ev_description = escape_yaml_str(
                &self
                    .uncertainty_budget_and_authorship
                    .dominant_event
                    .description
            ),
            ev_prob = self
                .uncertainty_budget_and_authorship
                .dominant_event
                .probability_p,
            ev_eps = self
                .uncertainty_budget_and_authorship
                .dominant_event
                .base_rate_floor_epsilon,
            ev_share = self
                .uncertainty_budget_and_authorship
                .dominant_event
                .cost_share,
            critical_requirement_rc = escape_yaml_str(
                &self
                    .uncertainty_budget_and_authorship
                    .info_gap_robustness
                    .critical_requirement_rc
            ),
            robustness_horizon_alpha_hat = escape_yaml_str(
                &self
                    .uncertainty_budget_and_authorship
                    .info_gap_robustness
                    .robustness_horizon_alpha_hat
            ),
            opportuneness_horizon_beta_hat = escape_yaml_str(
                &self
                    .uncertainty_budget_and_authorship
                    .info_gap_robustness
                    .opportuneness_horizon_beta_hat
            ),
            global_maximin_safeguard = escape_yaml_str(
                &self
                    .uncertainty_budget_and_authorship
                    .info_gap_robustness
                    .global_maximin_safeguard
            ),
            auth_null = escape_yaml_str(
                &self
                    .uncertainty_budget_and_authorship
                    .field_authorship
                    .null_model
            ),
            auth_decision = escape_yaml_str(
                &self
                    .uncertainty_budget_and_authorship
                    .field_authorship
                    .decision_rule
            ),
            auth_reviewer = escape_yaml_str(
                &self
                    .uncertainty_budget_and_authorship
                    .field_authorship
                    .adversarial_reviewer
            ),
            dataset_path = escape_yaml_str(&self.data_and_audit_contract.dataset_path),
            dataset_sha256 = escape_yaml_str(&self.data_and_audit_contract.dataset_sha256),
            missingness_mechanism =
                escape_yaml_str(&self.data_and_audit_contract.missingness_mechanism),
            seed_and_environment =
                escape_yaml_str(&self.data_and_audit_contract.seed_and_environment),
            null_model = escape_yaml_str(&self.data_and_audit_contract.null_model),
            preregistration_hash =
                escape_yaml_str(&self.data_and_audit_contract.preregistration_hash),
            attempt_counter_k = self.data_and_audit_contract.attempt_counter_k,
            e_process_multiplicity =
                escape_yaml_str(&self.data_and_audit_contract.e_process_multiplicity),
            total_score = self
                .data_and_audit_contract
                .murphy_corp_decomposition
                .total_score_brier_or_crps,
            miscalibration = self
                .data_and_audit_contract
                .murphy_corp_decomposition
                .miscalibration_rel_mcb,
            resolution = self
                .data_and_audit_contract
                .murphy_corp_decomposition
                .resolution_res_dsc,
            base_uncertainty = self
                .data_and_audit_contract
                .murphy_corp_decomposition
                .base_uncertainty_unc0,
            fnd_reg = self.data_and_audit_contract.findings.registered,
            fnd_used = self.data_and_audit_contract.findings.used,
            unused_yaml = unused_yaml,
            dr_p = self.decision_rule_and_scope.event_probability_p,
            dr_eps = self.decision_rule_and_scope.base_rate_floor_epsilon,
            dr_c = self.decision_rule_and_scope.protection_cost_c,
            dr_l = self.decision_rule_and_scope.event_loss_l,
            dr_eta = self.decision_rule_and_scope.mitigation_efficacy_eta,
            dr_p_star = self.decision_rule_and_scope.critical_threshold_p_star,
            declared_act = escape_yaml_str(&self.decision_rule_and_scope.declared_act),
            scope_statement = escape_yaml_str(&self.decision_rule_and_scope.scope_statement),
            who_pays_for_error = escape_yaml_str(&self.decision_rule_and_scope.who_pays_for_error),
            exit_clause = escape_yaml_str(&self.decision_rule_and_scope.exit_clause),
        )
    }

    /// Parses an Epistemological Card from the YAML template returning a typed [`CardParseError`].
    pub fn from_yaml_typed(yaml: &str) -> Result<Self, CardParseError> {
        let mut map: BTreeMap<String, String> = BTreeMap::new();
        let mut stack: Vec<(usize, String)> = Vec::new();
        let mut unused: Vec<String> = Vec::new();

        for raw_line in yaml.lines() {
            let trimmed = raw_line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            let indent = raw_line.len() - raw_line.trim_start().len();
            if let Some(rest) = trimmed.strip_prefix("- ") {
                while let Some(&(last_indent, _)) = stack.last() {
                    if indent < last_indent {
                        stack.pop();
                    } else {
                        break;
                    }
                }
                let current_path = stack
                    .iter()
                    .map(|(_, s)| s.as_str())
                    .collect::<Vec<_>>()
                    .join(".");
                if current_path == "data_and_audit_contract.findings.unused" {
                    unused.push(unquote(rest));
                }
                continue;
            }
            while let Some(&(last_indent, _)) = stack.last() {
                if indent <= last_indent {
                    stack.pop();
                } else {
                    break;
                }
            }
            if let Some((k, v)) = trimmed.split_once(':') {
                let key = k.trim().to_string();
                let val = v.trim();
                if val.is_empty() {
                    stack.push((indent, key));
                } else if val == "[]" {
                    // Explicit empty inline sequence
                } else {
                    let mut full_path = stack
                        .iter()
                        .map(|(_, s)| s.as_str())
                        .collect::<Vec<_>>()
                        .join(".");
                    if !full_path.is_empty() {
                        full_path.push('.');
                    }
                    full_path.push_str(&key);
                    map.insert(full_path, unquote(val));
                }
            }
        }

        let req = |k: &str| -> Result<String, CardParseError> {
            map.get(k)
                .cloned()
                .ok_or_else(|| CardParseError::MissingField(k.to_string()))
        };
        let req_f64 = |k: &str| -> Result<f64, CardParseError> {
            let raw = req(k)?;
            let v = raw
                .parse::<f64>()
                .map_err(|e| CardParseError::InvalidNumber {
                    field: k.to_string(),
                    detail: e.to_string(),
                })?;
            if !v.is_finite() {
                return Err(CardParseError::NonFiniteNumber {
                    field: k.to_string(),
                    value: raw,
                });
            }
            Ok(v)
        };
        let req_usize = |k: &str| -> Result<usize, CardParseError> {
            req(k)?
                .parse::<usize>()
                .map_err(|e| CardParseError::InvalidInteger {
                    field: k.to_string(),
                    detail: e.to_string(),
                })
        };

        let raw_layer = req("layer")?;
        let layer = Layer::from_code(&raw_layer).ok_or(CardParseError::InvalidLayer(raw_layer))?;
        let raw_unc = req("uncertainty")?;
        let uncertainty = UncertaintyClass::from_code(&raw_unc)
            .ok_or(CardParseError::InvalidUncertaintyClass(raw_unc))?;

        Ok(Self {
            id: req("id")?,
            layer,
            uncertainty,
            decision_maker_question: DecisionMakerQuestion {
                who_decides: req("decision_maker_question.who_decides")?,
                altered_action: req("decision_maker_question.altered_action")?,
                question: req("decision_maker_question.question")?,
            },
            epistemological_header: EpistemologicalHeader {
                generating_process: req("epistemological_header.generating_process")?,
                nonlinearity: req("epistemological_header.nonlinearity")?,
                unit_and_margin: UnitAndMargin {
                    independent_unit: req(
                        "epistemological_header.unit_and_margin.independent_unit",
                    )?,
                    nominal_sample_size_n: req_usize(
                        "epistemological_header.unit_and_margin.nominal_sample_size_n",
                    )?,
                    weighted_cluster_mean_m_tilde: req(
                        "epistemological_header.unit_and_margin.weighted_cluster_mean_m_tilde",
                    )?,
                    threshold_exceedance_icc: req(
                        "epistemological_header.unit_and_margin.threshold_exceedance_icc",
                    )?,
                    effective_sample_size_n_eff: req_usize(
                        "epistemological_header.unit_and_margin.effective_sample_size_n_eff",
                    )?,
                    unit_margin_of_error: req(
                        "epistemological_header.unit_and_margin.unit_margin_of_error",
                    )?,
                },
                noise_physics: NoisePhysics {
                    stochastic_regime: req(
                        "epistemological_header.noise_physics.stochastic_regime",
                    )?,
                    absorbing_states: req("epistemological_header.noise_physics.absorbing_states")?,
                    bridge_l3_l1_performativity: BridgePerformativity {
                        agent_type: req(
                            "epistemological_header.noise_physics.bridge_L3_L1_performativity.agent_type",
                        )?,
                        wasserstein_sensitivity_epsilon: req_f64(
                            "epistemological_header.noise_physics.bridge_L3_L1_performativity.wasserstein_sensitivity_epsilon",
                        )?,
                        rrm_convergence_condition: req(
                            "epistemological_header.noise_physics.bridge_L3_L1_performativity.rrm_convergence_condition",
                        )?,
                        stability_optimality_divergence: req(
                            "epistemological_header.noise_physics.bridge_L3_L1_performativity.stability_optimality_divergence",
                        )?,
                    },
                },
                causal_ladder: CausalLadder {
                    pearl_rung: req("epistemological_header.causal_ladder.pearl_rung")?,
                    identification_strategy: req(
                        "epistemological_header.causal_ladder.identification_strategy",
                    )?,
                    untestable_assumption: req(
                        "epistemological_header.causal_ladder.untestable_assumption",
                    )?,
                },
            },
            uncertainty_budget_and_authorship: UncertaintyBudgetAndAuthorship {
                variance_budget: VarianceBudget {
                    inputs: req_f64("uncertainty_budget_and_authorship.variance_budget.inputs")?,
                    parameters: req_f64(
                        "uncertainty_budget_and_authorship.variance_budget.parameters",
                    )?,
                    covariance: req_f64(
                        "uncertainty_budget_and_authorship.variance_budget.covariance",
                    )?,
                    structural_discrepancy_gp: req(
                        "uncertainty_budget_and_authorship.variance_budget.structural_discrepancy_gp",
                    )?,
                },
                dominant_event: DominantEvent {
                    description: req(
                        "uncertainty_budget_and_authorship.dominant_event.description",
                    )?,
                    probability_p: req_f64(
                        "uncertainty_budget_and_authorship.dominant_event.probability_p",
                    )?,
                    base_rate_floor_epsilon: req_f64(
                        "uncertainty_budget_and_authorship.dominant_event.base_rate_floor_epsilon",
                    )?,
                    cost_share: req_f64(
                        "uncertainty_budget_and_authorship.dominant_event.cost_share",
                    )?,
                },
                info_gap_robustness: InfoGapRobustness {
                    critical_requirement_rc: req(
                        "uncertainty_budget_and_authorship.info_gap_robustness.critical_requirement_rc",
                    )?,
                    robustness_horizon_alpha_hat: req(
                        "uncertainty_budget_and_authorship.info_gap_robustness.robustness_horizon_alpha_hat",
                    )?,
                    opportuneness_horizon_beta_hat: req(
                        "uncertainty_budget_and_authorship.info_gap_robustness.opportuneness_horizon_beta_hat",
                    )?,
                    global_maximin_safeguard: req(
                        "uncertainty_budget_and_authorship.info_gap_robustness.global_maximin_safeguard",
                    )?,
                },
                field_authorship: FieldAuthorship {
                    null_model: req("uncertainty_budget_and_authorship.field_authorship.null_model")?,
                    decision_rule: req(
                        "uncertainty_budget_and_authorship.field_authorship.decision_rule",
                    )?,
                    adversarial_reviewer: req(
                        "uncertainty_budget_and_authorship.field_authorship.adversarial_reviewer",
                    )?,
                },
            },
            data_and_audit_contract: DataAndAuditContract {
                dataset_path: req("data_and_audit_contract.dataset_path")?,
                dataset_sha256: req("data_and_audit_contract.dataset_sha256")?,
                missingness_mechanism: req("data_and_audit_contract.missingness_mechanism")?,
                seed_and_environment: req("data_and_audit_contract.seed_and_environment")?,
                null_model: req("data_and_audit_contract.null_model")?,
                preregistration_hash: req("data_and_audit_contract.preregistration_hash")?,
                attempt_counter_k: req_usize("data_and_audit_contract.attempt_counter_K")?,
                e_process_multiplicity: req("data_and_audit_contract.e_process_multiplicity")?,
                murphy_corp_decomposition: MurphyCorpDecomposition {
                    total_score_brier_or_crps: req_f64(
                        "data_and_audit_contract.murphy_corp_decomposition.total_score_brier_or_crps",
                    )?,
                    miscalibration_rel_mcb: req_f64(
                        "data_and_audit_contract.murphy_corp_decomposition.miscalibration_REL_MCB",
                    )?,
                    resolution_res_dsc: req_f64(
                        "data_and_audit_contract.murphy_corp_decomposition.resolution_RES_DSC",
                    )?,
                    base_uncertainty_unc0: req_f64(
                        "data_and_audit_contract.murphy_corp_decomposition.base_uncertainty_UNC0",
                    )?,
                },
                findings: FindingsRegistry {
                    registered: req_usize("data_and_audit_contract.findings.registered")?,
                    used: req_usize("data_and_audit_contract.findings.used")?,
                    unused,
                },
            },
            decision_rule_and_scope: DecisionRuleAndScope {
                event_probability_p: req_f64("decision_rule_and_scope.event_probability_p")?,
                base_rate_floor_epsilon: req_f64(
                    "decision_rule_and_scope.base_rate_floor_epsilon",
                )?,
                protection_cost_c: req_f64("decision_rule_and_scope.protection_cost_C")?,
                event_loss_l: req_f64("decision_rule_and_scope.event_loss_L")?,
                mitigation_efficacy_eta: req_f64(
                    "decision_rule_and_scope.mitigation_efficacy_eta",
                )?,
                critical_threshold_p_star: req_f64(
                    "decision_rule_and_scope.critical_threshold_p_star",
                )?,
                declared_act: req("decision_rule_and_scope.declared_act")?,
                scope_statement: req("decision_rule_and_scope.scope_statement")?,
                who_pays_for_error: req("decision_rule_and_scope.who_pays_for_error")?,
                exit_clause: req("decision_rule_and_scope.exit_clause")?,
            },
        })
    }

    /// Parses an Epistemological Card from the YAML template.
    pub fn from_yaml(yaml: &str) -> Result<Self, String> {
        Self::from_yaml_typed(yaml).map_err(|e| e.to_string())
    }
}

fn escape_yaml_str(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c => out.push(c),
        }
    }
    out
}

fn unquote(s: &str) -> String {
    let t = s.trim();
    if t.len() >= 2
        && ((t.starts_with('"') && t.ends_with('"')) || (t.starts_with('\'') && t.ends_with('\'')))
    {
        let inner = &t[1..t.len() - 1];
        let mut out = String::with_capacity(inner.len());
        let mut chars = inner.chars();
        while let Some(ch) = chars.next() {
            if ch == '\\' {
                match chars.next() {
                    Some('\\') => out.push('\\'),
                    Some('"') => out.push('"'),
                    Some('n') => out.push('\n'),
                    Some('r') => out.push('\r'),
                    Some('t') => out.push('\t'),
                    Some(other) => {
                        out.push('\\');
                        out.push(other);
                    }
                    None => out.push('\\'),
                }
            } else {
                out.push(ch);
            }
        }
        out
    } else {
        t.to_string()
    }
}
