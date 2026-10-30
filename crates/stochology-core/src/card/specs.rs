//! Specification Pattern (`CardSpecification` + [`CANONICAL_CARD_SPECIFICATIONS`]) for auditing
//! [`EpistemologicalCard`] against the 4 Prohibitions and the 5 Boundary Invariants ($\Phi_1$–$\Phi_5$).

use crate::invariants::verify_resolution_phi5;
use crate::layers::respects_cromwell;
use crate::sha256::is_valid_sha256_hex;

use super::model::{ContractViolation, EpistemologicalCard, MissingnessMechanism};
use super::profile::{AppliedWaiver, AuditReport, ProtocolProfile, WAIVABLE_INSTRUMENTS};
use super::value_objects::{extract_number_after, parse_m_tilde, parse_seed_from_env};

/// Composable specification rule evaluated during [`EpistemologicalCard::audit`].
///
/// Implement at least one of the two methods: each defaults to the other, `check` under
/// [`ProtocolProfile::canonical`].
pub trait CardSpecification: Sync {
    /// Appends any [`ContractViolation`]s detected on `card` under the canonical profile.
    fn check(&self, card: &EpistemologicalCard, violations: &mut Vec<ContractViolation>) {
        self.check_with(card, &ProtocolProfile::canonical(), violations);
    }

    /// Appends any [`ContractViolation`]s detected on `card` under the application's `profile`.
    fn check_with(
        &self,
        card: &EpistemologicalCard,
        _profile: &ProtocolProfile,
        violations: &mut Vec<ContractViolation>,
    ) {
        self.check(card, violations);
    }
}

/// Step 2 of SIGA & Prohibition 1: 3-part decision-maker question (`who_decides`, `altered_action`, `question`).
pub struct Step2QuestionSpec;

impl CardSpecification for Step2QuestionSpec {
    fn check_with(
        &self,
        card: &EpistemologicalCard,
        profile: &ProtocolProfile,
        v: &mut Vec<ContractViolation>,
    ) {
        let q = &card.decision_maker_question;
        let dm_lower = q.who_decides.trim().to_ascii_lowercase();
        let dm_tokens: Vec<&str> = dm_lower
            .split(|c: char| !c.is_ascii_alphanumeric())
            .filter(|t| !t.is_empty())
            .collect();
        let forbidden_dm_tokens = &profile.forbidden_decision_maker_tokens;
        if dm_tokens.len() < 2
            || dm_tokens
                .iter()
                .any(|t| forbidden_dm_tokens.iter().any(|f| f == t))
        {
            v.push(ContractViolation {
                gate_code: "STEP2_DECISION_MAKER",
                description: "who_decides cannot be empty or generic".into(),
            });
        }
        if q.altered_action.trim().is_empty() {
            v.push(ContractViolation {
                gate_code: "STEP2_ACTION",
                description: "altered_action is mandatory in Step 2".into(),
            });
        }
        if !q.question.trim().ends_with('?') {
            v.push(ContractViolation {
                gate_code: "STEP2_QUESTION_MARK",
                description: "decision-maker question must end with '?'".into(),
            });
        }
    }
}

/// Prohibition 2: Ban on the lone mean ($f(\mathbb{E}[X])$, $\mathbb{E}[f(X)]$, $\Delta_J$, quantiles, $\mathrm{CRPS}$).
pub struct Prohibition2LoneMeanSpec;

impl CardSpecification for Prohibition2LoneMeanSpec {
    fn check_with(
        &self,
        card: &EpistemologicalCard,
        profile: &ProtocolProfile,
        v: &mut Vec<ContractViolation>,
    ) {
        let nl = card
            .epistemological_header
            .nonlinearity
            .to_ascii_lowercase();
        let mut nl_missing_term = false;
        for term in &profile.lone_mean_terms {
            if !nl.contains(term.as_str()) {
                nl_missing_term = true;
                v.push(ContractViolation {
                    gate_code: "PROHIBITION_2_LONE_MEAN",
                    description: format!(
                        "nonlinearity field omits '{}' required by Prohibition 2",
                        term
                    ),
                });
            }
        }
        if !nl_missing_term
            && !card
                .epistemological_header
                .parsed_nonlinearity_metrics()
                .is_some_and(|m| m.is_algebraically_coherent())
        {
            v.push(ContractViolation {
                gate_code: "PROHIBITION_2_LONE_MEAN",
                description: "nonlinearity numeric values fail non-zero Jensen gap identity (Delta_J = E[f(X)] - f(E[X])), strict quantile ordering (q05 < median < q95), or 0 < CRPS < (q95 - q05)".into(),
            });
        }
    }
}

/// Prohibition 3: Performativity on Bridge $L_3 \to L_1$ ($\varepsilon$, RRM contraction, $\theta_{\mathrm{PS}}$ vs. $\theta_{\mathrm{PO}}$).
pub struct Prohibition3PerformativitySpec;

impl CardSpecification for Prohibition3PerformativitySpec {
    fn check_with(
        &self,
        card: &EpistemologicalCard,
        _profile: &ProtocolProfile,
        v: &mut Vec<ContractViolation>,
    ) {
        let perf = &card
            .epistemological_header
            .noise_physics
            .bridge_l3_l1_performativity;
        if perf.agent_type != "passive" && perf.agent_type != "performative" {
            v.push(ContractViolation {
                gate_code: "PROHIBITION_3_AGENT_TYPE",
                description: "agent_type must be 'passive' or 'performative'".into(),
            });
        }
        if !perf.wasserstein_sensitivity_epsilon.is_finite()
            || perf.wasserstein_sensitivity_epsilon < 0.0
            || (perf.agent_type == "performative"
                && (perf.wasserstein_sensitivity_epsilon <= 0.0
                    || perf.wasserstein_sensitivity_epsilon >= 1.0))
        {
            v.push(ContractViolation {
                gate_code: "PROHIBITION_3_EPSILON",
                description: "wasserstein_sensitivity_epsilon must be finite and in (0, 1) for performative agents (or >= 0 for passive)".into(),
            });
        }
        if !perf
            .parsed_rrm_metrics()
            .is_some_and(|m| m.satisfies_strict_contraction(perf.wasserstein_sensitivity_epsilon))
        {
            v.push(ContractViolation {
                gate_code: "PROHIBITION_3_RRM_CONTRACTION",
                description:
                    "rrm_convergence_condition must numerically satisfy eps * beta / gamma < 1"
                        .into(),
            });
        }
        if perf.agent_type == "performative"
            && !perf
                .parsed_divergence_metrics()
                .is_some_and(|d| d.has_strict_divergence())
        {
            v.push(ContractViolation {
                gate_code: "PROHIBITION_3_PS_VS_PO",
                description:
                    "performative agent must explicitly report distinct numeric theta_PS and theta_PO"
                        .into(),
            });
        }
    }
}

/// Invariant $\Phi_1$: Simulability (the profile's seed, canonically `seed=2026`, and executable `null_model`).
pub struct Phi1SimulabilitySpec;

impl CardSpecification for Phi1SimulabilitySpec {
    fn check_with(
        &self,
        card: &EpistemologicalCard,
        profile: &ProtocolProfile,
        v: &mut Vec<ContractViolation>,
    ) {
        let seed_env = &card.data_and_audit_contract.seed_and_environment;
        if parse_seed_from_env(seed_env) != Some(profile.seed) {
            v.push(ContractViolation {
                gate_code: "PHI1_SEED",
                description: format!(
                    "seed_and_environment must declare the profile's 'seed={}'",
                    profile.seed
                ),
            });
        }
        let nm_trimmed = card.data_and_audit_contract.null_model.trim();
        let nm_lower = nm_trimmed.to_ascii_lowercase();
        if nm_trimmed.is_empty()
            || nm_lower == "none"
            || nm_lower.starts_with("none ")
            || (!nm_trimmed.contains('#') && !nm_trimmed.contains("::"))
        {
            v.push(ContractViolation {
                gate_code: "PHI1_PHI3_NULL_MODEL",
                description: "executable null_model (containing '#' or '::') is mandatory".into(),
            });
        }
    }
}

/// Invariant $\Phi_2$: Independent unit, Moulton-Campbell $\tilde{m}$, exceedance ICC $\rho_I(p)$, and $n_{\mathrm{eff}}(p)$.
pub struct Phi2IndependentUnitSpec;

impl CardSpecification for Phi2IndependentUnitSpec {
    fn check_with(
        &self,
        card: &EpistemologicalCard,
        _profile: &ProtocolProfile,
        v: &mut Vec<ContractViolation>,
    ) {
        let um = &card.epistemological_header.unit_and_margin;
        let u_lower = um.independent_unit.trim().to_ascii_lowercase();
        if u_lower.is_empty()
            || ((u_lower.contains("row")
                || u_lower.contains("transaction")
                || u_lower.contains("transactional"))
                && !u_lower.contains("cluster of"))
        {
            v.push(ContractViolation {
                gate_code: "PHI2_PSEUDOREPLICATION",
                description: "independent_unit cannot be the correlated transactional row".into(),
            });
        }
        let m_tilde_lower = um.weighted_cluster_mean_m_tilde.to_ascii_lowercase();
        let icc_lower = um.threshold_exceedance_icc.to_ascii_lowercase();
        let parsed_m_tilde = parse_m_tilde(&m_tilde_lower);
        let parsed_rho_i = extract_number_after(&icc_lower, "rho_i(0.90) =")
            .or_else(|| extract_number_after(&icc_lower, "rho_i"));

        if (!um.weighted_cluster_mean_m_tilde.contains("CV") && !m_tilde_lower.contains("moulton"))
            || m_tilde_lower.contains("ignored")
            || !parsed_m_tilde.is_some_and(|m| m > 1.0)
        {
            v.push(ContractViolation {
                gate_code: "PHI2_MOULTON_CAMPBELL",
                description: "weighted_cluster_mean_m_tilde must declare valid Moulton-Campbell correction (m_tilde > 1)".into(),
            });
        }
        if !um.threshold_exceedance_icc.contains("rho_I")
            || icc_lower.contains("unadjusted")
            || !parsed_rho_i.is_some_and(|r| r > 0.0 && r <= 1.0)
        {
            v.push(ContractViolation {
                gate_code: "PHI2_EXCEEDANCE_ICC",
                description: "threshold_exceedance_icc must declare valid rho_I(p) in (0, 1]"
                    .into(),
            });
        }
        let n_eff_coherent = um.parsed_moulton_metrics().is_some_and(|m| {
            m.is_coherent_with_sample_sizes(
                um.nominal_sample_size_n,
                um.effective_sample_size_n_eff,
            )
        });
        if um.effective_sample_size_n_eff == 0
            || um.effective_sample_size_n_eff >= um.nominal_sample_size_n
            || !n_eff_coherent
            || !um.unit_margin_of_error.contains("+/-")
        {
            v.push(ContractViolation {
                gate_code: "PHI2_N_EFF",
                description: format!(
                    "effective_sample_size_n_eff ({}) invalid relative to nominal n ({}) and DEFF_exceedance, or unit_margin_of_error omits '+/-'",
                    um.effective_sample_size_n_eff, um.nominal_sample_size_n
                ),
            });
        }
    }
}

/// Invariant $\Phi_3$: Dataset provenance, Rubin missingness, SHA-256 pre-registration, and Pearl's Ladder.
pub struct Phi3ProvenanceAndMissingnessSpec;

impl CardSpecification for Phi3ProvenanceAndMissingnessSpec {
    fn check_with(
        &self,
        card: &EpistemologicalCard,
        profile: &ProtocolProfile,
        v: &mut Vec<ContractViolation>,
    ) {
        let ds_path = card.data_and_audit_contract.dataset_path.trim();
        let ds_sha = card.data_and_audit_contract.dataset_sha256.trim();
        if !ds_path.starts_with(profile.dataset_root.as_str())
            || !ds_path.ends_with(".csv")
            || ds_path.contains("..")
            || !is_valid_sha256_hex(ds_sha)
        {
            v.push(ContractViolation {
                gate_code: "PHI3_DATASET_PROVENANCE",
                description: format!(
                    "dataset_path must point to '{}*.csv' and dataset_sha256 must be a valid 64-hex SHA-256",
                    profile.dataset_root
                ),
            });
        }
        let mm_raw = &card.data_and_audit_contract.missingness_mechanism;
        let mm_lower = mm_raw.to_ascii_lowercase();
        let mm_kind = MissingnessMechanism::from_prefix(mm_raw);
        let mnar_bounds_ok = if matches!(mm_kind, Some(MissingnessMechanism::Mnar)) {
            (mm_lower.contains("manski") || mm_lower.contains("lee") || mm_lower.contains("bounds"))
                && !mm_lower.contains("ignored")
                && !mm_lower.contains("without")
                && !mm_lower.contains("only")
        } else {
            true
        };
        if mm_kind.is_none() || !mnar_bounds_ok {
            v.push(ContractViolation {
                gate_code: "PHI3_MISSINGNESS_MECHANISM",
                description: "missingness_mechanism must start with MCAR, MAR, or MNAR (and MNAR must declare valid Manski or Lee bounds)".into(),
            });
        }
        if !is_valid_sha256_hex(&card.data_and_audit_contract.preregistration_hash)
            || card.data_and_audit_contract.preregistration_hash
                != profile.preregistration_hash_for(card)
        {
            v.push(ContractViolation {
                gate_code: "PHI3_PREREGISTRATION_HASH",
                description: "preregistration_hash must be a valid 64-character SHA-256 matching the profile's preregistration_payload_for(card)"
                    .into(),
            });
        }
        let cl = &card.epistemological_header.causal_ladder;
        let rung = cl.pearl_rung.as_str();
        let id_strat = cl.identification_strategy.to_ascii_lowercase();
        let untest = cl.untestable_assumption.to_ascii_lowercase();
        let rung_ok = if !card
            .decision_maker_question
            .altered_action
            .trim()
            .is_empty()
        {
            matches!(rung, "2-Act" | "3-Imagine")
        } else {
            matches!(rung, "1-See" | "2-Act" | "3-Imagine")
        };
        let mb_ok = id_strat.contains("markov blanket")
            && !id_strat.contains("not mapped")
            && !id_strat.contains("unmapped");
        let collider_ok = (id_strat.contains("without collider")
            || id_strat.contains("no collider")
            || id_strat.contains("free of collider"))
            && !id_strat.contains("conditioned on collider")
            && !id_strat.contains("without collider audit");
        let lambda_ok = untest.contains("lambda*");
        if !rung_ok || !mb_ok || !collider_ok || !lambda_ok {
            v.push(ContractViolation {
                gate_code: "PHI3_PEARL_LADDER",
                description: "causal_ladder requires '2-Act' or '3-Imagine' for interventional decisions, mapped Markov blanket without collider, and Lambda* sensitivity bound".into(),
            });
        }
    }
}

/// Variance budget ($\sum = 1.0$, Kennedy-O'Hagan GP) and Dominant Event (`PROHIBITION_4_CROMWELL`, cost share).
pub struct VarianceBudgetAndDominantEventSpec;

impl CardSpecification for VarianceBudgetAndDominantEventSpec {
    fn check_with(
        &self,
        card: &EpistemologicalCard,
        profile: &ProtocolProfile,
        v: &mut Vec<ContractViolation>,
    ) {
        let vb = &card.uncertainty_budget_and_authorship.variance_budget;
        let sum_var = vb.inputs + vb.parameters + vb.covariance;
        let gp_lower = vb.structural_discrepancy_gp.to_ascii_lowercase();
        if !vb.inputs.is_finite()
            || !vb.parameters.is_finite()
            || !vb.covariance.is_finite()
            || vb.inputs < 0.0
            || vb.parameters < 0.0
            || vb.covariance < 0.0
            || (sum_var - 1.0).abs() > 1e-6
            || (!gp_lower.contains("gp") && !gp_lower.contains("gaussian process"))
        {
            v.push(ContractViolation {
                gate_code: "VARIANCE_BUDGET",
                description: format!("variance_budget sums to {sum_var:.4} (must equal 1.0) and structural_discrepancy_gp must declare a Gaussian Process (GP)"),
            });
        }
        let dom = &card.uncertainty_budget_and_authorship.dominant_event;
        if !dom.probability_p.is_finite()
            || !dom.base_rate_floor_epsilon.is_finite()
            || !respects_cromwell(dom.probability_p, dom.base_rate_floor_epsilon)
        {
            v.push(ContractViolation {
                gate_code: "PROHIBITION_4_CROMWELL",
                description: format!(
                    "dominant_event.probability_p ({}) violates Cromwell interval [{}, {}]",
                    dom.probability_p,
                    dom.base_rate_floor_epsilon,
                    1.0 - dom.base_rate_floor_epsilon
                ),
            });
        }
        if !dom.cost_share.is_finite()
            || dom.cost_share < profile.min_dominant_cost_share
            || dom.cost_share > 1.0
        {
            v.push(ContractViolation {
                gate_code: "DOMINANT_EVENT_COST_SHARE",
                description: format!(
                    "dominant_event.cost_share must be finite and in [{:.2}, 1.0]",
                    profile.min_dominant_cost_share
                ),
            });
        }
    }
}

/// Info-Gap robustness ($\hat{\alpha} > 0$) and Sniedovich's Global Maximin Safeguard.
pub struct InfoGapSniedovichSpec;

impl CardSpecification for InfoGapSniedovichSpec {
    fn check_with(
        &self,
        card: &EpistemologicalCard,
        _profile: &ProtocolProfile,
        v: &mut Vec<ContractViolation>,
    ) {
        let ig = &card.uncertainty_budget_and_authorship.info_gap_robustness;
        let alpha_hat_lower = ig.robustness_horizon_alpha_hat.to_ascii_lowercase();
        let safeguard_lower = ig.global_maximin_safeguard.to_ascii_lowercase();
        let alpha_hat_val = extract_number_after(&alpha_hat_lower, "alpha_hat");
        if ig.critical_requirement_rc.trim().is_empty()
            || !alpha_hat_val.is_some_and(|a| a > 0.0)
            || !safeguard_lower.contains("reversible")
            || safeguard_lower.contains("irreversible")
            || safeguard_lower.contains("no-man's land")
            || safeguard_lower.contains("unbounded")
            || safeguard_lower.contains("-999")
        {
            v.push(ContractViolation {
                gate_code: "INFO_GAP_SNIEDOVICH",
                description: "info_gap_robustness requires alpha_hat > 0 and global_maximin_safeguard with reversible protocol outside No-Man's Land".into(),
            });
        }
    }
}

/// Filter 4: Independent cross-authorship (`null_model`, `decision_rule`, `adversarial_reviewer`).
pub struct Filter4CrossAuthorshipSpec;

impl CardSpecification for Filter4CrossAuthorshipSpec {
    fn check_with(
        &self,
        card: &EpistemologicalCard,
        _profile: &ProtocolProfile,
        v: &mut Vec<ContractViolation>,
    ) {
        let auth = &card.uncertainty_budget_and_authorship.field_authorship;
        if auth.null_model.trim().is_empty()
            || auth.decision_rule.trim().is_empty()
            || auth.adversarial_reviewer.trim().is_empty()
            || auth.null_model.trim() == auth.decision_rule.trim()
            || auth.adversarial_reviewer.trim() == auth.null_model.trim()
            || auth.adversarial_reviewer.trim() == auth.decision_rule.trim()
        {
            v.push(ContractViolation {
                gate_code: "FILTER4_CROSS_AUTHORSHIP",
                description:
                    "null_model, decision_rule, and adversarial_reviewer must have distinct authors"
                        .into(),
            });
        }
    }
}

/// Prohibition 4: Cromwell's Rule on `event_probability_p`, attempt counter $K \ge 1$, and e-process $E_t / K \ge 1/\alpha$.
pub struct Prohibition4CromwellAndEProcessSpec;

impl CardSpecification for Prohibition4CromwellAndEProcessSpec {
    fn check_with(
        &self,
        card: &EpistemologicalCard,
        profile: &ProtocolProfile,
        v: &mut Vec<ContractViolation>,
    ) {
        let dr = &card.decision_rule_and_scope;
        if !dr.event_probability_p.is_finite()
            || !dr.base_rate_floor_epsilon.is_finite()
            || !respects_cromwell(dr.event_probability_p, dr.base_rate_floor_epsilon)
        {
            v.push(ContractViolation {
                gate_code: "PROHIBITION_4_CROMWELL",
                description: format!(
                    "event_probability_p ({}) violates Cromwell interval [{}, {}]",
                    dr.event_probability_p,
                    dr.base_rate_floor_epsilon,
                    1.0 - dr.base_rate_floor_epsilon
                ),
            });
        }
        let k = card.data_and_audit_contract.attempt_counter_k;
        if k < 1 {
            v.push(ContractViolation {
                gate_code: "PROHIBITION_4_COUNTER_K",
                description: "attempt_counter_K must be >= 1".into(),
            });
        }
        if !card
            .data_and_audit_contract
            .parsed_e_process_metrics()
            .is_some_and(|m| {
                m.rejects_under_multiplicity_at(k, profile.e_process_min_inverse_alpha)
            })
        {
            v.push(ContractViolation {
                gate_code: "PROHIBITION_4_E_PROCESS",
                description: format!(
                    "e_process_multiplicity must declare finite E_t with E_t / K >= 1/alpha >= {:.1}",
                    profile.e_process_min_inverse_alpha
                ),
            });
        }
    }
}

/// Lens 5 (`CORP`-Murphy decomposition identity) and Findings Registry conservation.
pub struct Lens5MurphyCorpAndFindingsSpec;

impl CardSpecification for Lens5MurphyCorpAndFindingsSpec {
    fn check_with(
        &self,
        card: &EpistemologicalCard,
        profile: &ProtocolProfile,
        v: &mut Vec<ContractViolation>,
    ) {
        let mc = &card.data_and_audit_contract.murphy_corp_decomposition;
        let bs_reconstructed =
            mc.miscalibration_rel_mcb - mc.resolution_res_dsc + mc.base_uncertainty_unc0;
        if !mc.total_score_brier_or_crps.is_finite()
            || !mc.miscalibration_rel_mcb.is_finite()
            || !mc.resolution_res_dsc.is_finite()
            || !mc.base_uncertainty_unc0.is_finite()
            || mc.miscalibration_rel_mcb < 0.0
            || mc.resolution_res_dsc <= 0.0
            || mc.base_uncertainty_unc0 <= 0.0
            || mc.miscalibration_rel_mcb >= mc.resolution_res_dsc
            || (mc.total_score_brier_or_crps - bs_reconstructed).abs()
                > profile.murphy_corp_tolerance
        {
            v.push(ContractViolation {
                gate_code: "LENS5_MURPHY_CORP",
                description: format!(
                    "Murphy-CORP identity BS = REL - RES + UNC0 (with RES > REL >= 0) fails: {} vs {}",
                    mc.total_score_brier_or_crps, bs_reconstructed
                ),
            });
        }
        let fnd = &card.data_and_audit_contract.findings;
        if fnd.used.saturating_add(fnd.unused.len()) != fnd.registered
            || fnd.unused.iter().any(|s| s.trim().is_empty())
        {
            v.push(ContractViolation {
                gate_code: "CONTRACT_FINDINGS",
                description: "findings accounting (used + unused == registered) does not close"
                    .into(),
            });
        }
    }
}

/// Lens 5 & Invariant $\Phi_4$: Cost-loss economic boundary $p^* = \frac{C}{\eta L}$ and coherent `declared_act`.
pub struct Lens5CostLossBoundarySpec;

impl CardSpecification for Lens5CostLossBoundarySpec {
    fn check_with(
        &self,
        card: &EpistemologicalCard,
        profile: &ProtocolProfile,
        v: &mut Vec<ContractViolation>,
    ) {
        let dr = &card.decision_rule_and_scope;
        if !dr.protection_cost_c.is_finite()
            || !dr.event_loss_l.is_finite()
            || !dr.mitigation_efficacy_eta.is_finite()
            || !dr.critical_threshold_p_star.is_finite()
            || dr.protection_cost_c <= 0.0
            || dr.event_loss_l <= 0.0
            || dr.mitigation_efficacy_eta <= 0.0
            || dr.mitigation_efficacy_eta > 1.0
        {
            v.push(ContractViolation {
                gate_code: "LENS5_COST_LOSS_PARAMETERS",
                description: "finite C > 0, L > 0, and eta in (0, 1] are mandatory".into(),
            });
        } else {
            let expected_p_star =
                dr.protection_cost_c / (dr.mitigation_efficacy_eta * dr.event_loss_l);
            if (dr.critical_threshold_p_star - expected_p_star).abs() > profile.p_star_tolerance {
                v.push(ContractViolation {
                    gate_code: "LENS5_THRESHOLD_P_STAR",
                    description: format!(
                        "critical_threshold_p_star ({:.5}) diverges from C/(eta*L) = {:.5}",
                        dr.critical_threshold_p_star, expected_p_star
                    ),
                });
            }
            let expected_act = if dr.event_probability_p > expected_p_star {
                "protect"
            } else {
                "do_not_protect"
            };
            if dr.declared_act.trim() != expected_act {
                v.push(ContractViolation {
                    gate_code: "LENS5_INCOHERENT_ACT",
                    description: format!(
                        "declared_act '{}' incoherent with p={} vs p*={:.5} (expected '{}')",
                        dr.declared_act, dr.event_probability_p, expected_p_star, expected_act
                    ),
                });
            }
        }
    }
}

/// Invariant $\Phi_4$: Chouldechova trilemma audit ($\mathrm{FPR}_A / \mathrm{FPR}_B \ne 1$), scope exclusion, and numeric exit clause.
pub struct Phi4ChouldechovaAndScopeSpec;

impl CardSpecification for Phi4ChouldechovaAndScopeSpec {
    fn check_with(
        &self,
        card: &EpistemologicalCard,
        profile: &ProtocolProfile,
        v: &mut Vec<ContractViolation>,
    ) {
        let dr = &card.decision_rule_and_scope;
        if !dr
            .parsed_chouldechova_metrics()
            .is_some_and(|m| m.is_non_unit_ratio_beyond(profile.chouldechova_ratio_tolerance))
        {
            v.push(ContractViolation {
                gate_code: "PHI4_CHOULDECHOVA",
                description:
                    "who_pays_for_error must algebraically audit Chouldechova's trilemma (FPR, PPV, and FPR_A/FPR_B != 1)".into(),
            });
        }
        let scope_lower = dr.scope_statement.to_ascii_lowercase();
        let has_scope_exclusion =
            scope_lower.contains("does not apply") || scope_lower.contains("excluded");
        let has_numeric_exit = dr.exit_clause.chars().any(|c| c.is_ascii_digit());
        if dr.scope_statement.trim().is_empty()
            || !has_scope_exclusion
            || dr.exit_clause.trim().is_empty()
            || !has_numeric_exit
        {
            v.push(ContractViolation {
                gate_code: "PHI4_SCOPE_AND_EXIT",
                description: "scope_statement (with explicit non-applicability boundary) and numeric exit_clause are mandatory".into(),
            });
        }
    }
}

/// Invariant $\Phi_5$: Question resolution vs. independent unit (`verify_resolution_phi5`).
pub struct Phi5ResolutionSpec;

impl CardSpecification for Phi5ResolutionSpec {
    fn check_with(
        &self,
        card: &EpistemologicalCard,
        profile: &ProtocolProfile,
        v: &mut Vec<ContractViolation>,
    ) {
        let um = &card.epistemological_header.unit_and_margin;
        let q = &card.decision_maker_question;
        let resolution_ok = match &profile.resolution {
            Some(vocab) => vocab.verify_resolution(&um.independent_unit, &q.altered_action),
            None => verify_resolution_phi5(&um.independent_unit, &q.altered_action),
        };
        if !resolution_ok {
            v.push(ContractViolation {
                gate_code: "PHI5_RESOLUTION",
                description:
                    "altered_action uses coarse aggregate granularity incompatible with independent_unit"
                        .into(),
            });
        }
    }
}

/// Ordered registry of all 14 canonical [`CardSpecification`] rules evaluated by [`EpistemologicalCard::audit`].
pub const CANONICAL_CARD_SPECIFICATIONS: &[&dyn CardSpecification] = &[
    &Step2QuestionSpec,
    &Prohibition2LoneMeanSpec,
    &Prohibition3PerformativitySpec,
    &Phi1SimulabilitySpec,
    &Phi2IndependentUnitSpec,
    &Phi3ProvenanceAndMissingnessSpec,
    &VarianceBudgetAndDominantEventSpec,
    &InfoGapSniedovichSpec,
    &Filter4CrossAuthorshipSpec,
    &Prohibition4CromwellAndEProcessSpec,
    &Lens5MurphyCorpAndFindingsSpec,
    &Lens5CostLossBoundarySpec,
    &Phi4ChouldechovaAndScopeSpec,
    &Phi5ResolutionSpec,
];

impl EpistemologicalCard {
    /// Audits the Epistemological Card against the 4 Prohibitions and the 5 Boundary Invariants ($\Phi_1$–$\Phi_5$)
    /// by running every rule in [`CANONICAL_CARD_SPECIFICATIONS`] under [`ProtocolProfile::canonical`].
    pub fn audit(&self) -> Result<(), Vec<ContractViolation>> {
        self.audit_with(&ProtocolProfile::canonical()).map(|_| ())
    }

    /// Audits the card under an application's [`ProtocolProfile`]: every rule runs with the profile's
    /// numbers and vocabulary; violations of an admissibly waived instrument are moved into the
    /// returned [`AuditReport`] instead of being dropped, and defective waivers are violations.
    pub fn audit_with(
        &self,
        profile: &ProtocolProfile,
    ) -> Result<AuditReport, Vec<ContractViolation>> {
        let mut violations: Vec<ContractViolation> = profile
            .waiver_defects(self)
            .into_iter()
            .map(|(gate_code, description)| ContractViolation {
                gate_code,
                description,
            })
            .collect();
        let mut applied_waivers: Vec<AppliedWaiver> = WAIVABLE_INSTRUMENTS
            .iter()
            .filter_map(|i| {
                let w = profile.waiver_for(i.gate_code)?;
                profile
                    .waiver_defects_of(self, w)
                    .is_empty()
                    .then(|| AppliedWaiver {
                        gate_code: w.gate_code.clone(),
                        principle: i.principle,
                        reason: w.reason.trim().to_string(),
                        suppressed: Vec::new(),
                    })
            })
            .collect();

        let mut raw = Vec::new();
        for spec in CANONICAL_CARD_SPECIFICATIONS {
            spec.check_with(self, profile, &mut raw);
        }
        for violation in raw {
            match applied_waivers
                .iter_mut()
                .find(|w| w.gate_code == violation.gate_code)
            {
                Some(waiver) => waiver.suppressed.push(violation.description),
                None => violations.push(violation),
            }
        }

        if violations.is_empty() {
            Ok(AuditReport {
                profile_name: profile.name.clone(),
                applied_waivers,
            })
        } else {
            Err(violations)
        }
    }
}
