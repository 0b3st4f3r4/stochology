//! Portable Epistemological Card auditing (`gates --audit-card <card.yaml>`).

use std::path::Path;
use stochology_cli::evaluate_empirical_card_severe_test;
use stochology_core::{EpistemologicalCard, MissingnessMechanism};
use stochology_governance::{CloudLocalLoraArchitectureAudit, RegulatoryComplianceAudit};
use stochology_siga::{
    AlertFatigueAudit, LongContextDpiDilutionAudit, MultimodalCorrelatedFusionAudit,
    PreMortemCounterfactualRootCauseAudit, SimpsonCausalParadoxAudit,
};

fn parse_alpha_hat_from_declaration(decl: &str) -> f64 {
    let lower = decl.to_ascii_lowercase();
    let Some(idx) = lower.find("alpha_hat") else {
        return 0.0;
    };
    let rest = &lower[idx + "alpha_hat".len()..];
    let after_eq = rest
        .find('=')
        .map(|eq| &rest[eq + 1..])
        .unwrap_or(rest)
        .trim_start();
    let token: String = after_eq
        .chars()
        .take_while(|c| c.is_ascii_digit() || *c == '.' || *c == '+' || *c == '-')
        .collect();
    token
        .parse::<f64>()
        .ok()
        .filter(|v| v.is_finite())
        .unwrap_or(0.0)
}

/// Audits a single Epistemological Card (`--audit-card <card.yaml>`) end-to-end across contract specs,
/// empirical data, the 8-Step SIGA Roadmap, statutory compliance, and blueprint resonances.
pub fn run_audit_card(root: &Path, card_arg: &str) -> Result<(), String> {
    let candidate_path = Path::new(card_arg);
    let full_path = if candidate_path.exists() {
        candidate_path.to_path_buf()
    } else {
        root.join(card_arg)
    };
    let yaml_content = std::fs::read_to_string(&full_path)
        .map_err(|e| format!("Failed to read card '{}': {e}", full_path.display()))?;
    let card = EpistemologicalCard::from_yaml(&yaml_content)
        .map_err(|e| format!("Failed to parse YAML card '{}': {e}", full_path.display()))?;
    card.audit().map_err(|violations| {
        format!(
            "Card '{}' failed EpistemologicalCard::audit(): {:?}",
            card.id, violations
        )
    })?;

    let (emp, siga_rep) = evaluate_empirical_card_severe_test(root, &card)?;
    let (tpr, fpr) = emp.confusion_tpr_fpr_at_p_star;
    let p_star = card.decision_rule_and_scope.critical_threshold_p_star;
    let alert_audit =
        AlertFatigueAudit::evaluate(emp.event_probability_p, tpr, fpr, p_star, 3.0, 32.0, true);
    let (_, stoch_gate) = AlertFatigueAudit::canonical_epic_sepsis_vs_stochological_gate();
    let triage_decibans = if siga_rep.approved_end_to_end
        && alert_audit.positive_predictive_value_ppv >= p_star
        && alert_audit.cognitive_triage_load_bps <= 10.0
        && alert_audit.turing_weight_of_evidence_decibans > 3.0
    {
        alert_audit
            .turing_weight_of_evidence_decibans
            .max(stoch_gate.turing_weight_of_evidence_decibans)
    } else {
        alert_audit.turing_weight_of_evidence_decibans
    };

    let ig = &card.uncertainty_budget_and_authorship.info_gap_robustness;
    let alpha_hat = parse_alpha_hat_from_declaration(&ig.robustness_horizon_alpha_hat);
    let safeguard_lower = ig.global_maximin_safeguard.to_ascii_lowercase();
    let sniedovich_ok = safeguard_lower.contains("wald")
        && safeguard_lower.contains("reversible")
        && !safeguard_lower.contains("irreversible")
        && !safeguard_lower.contains("no-man's land")
        && !safeguard_lower.contains("-999");
    let mm_ok =
        MissingnessMechanism::from_prefix(&card.data_and_audit_contract.missingness_mechanism)
            .is_some()
            && card
                .epistemological_header
                .causal_ladder
                .identification_strategy
                .to_ascii_lowercase()
                .contains("markov blanket");
    let chouldechova_ok = card
        .decision_rule_and_scope
        .parsed_chouldechova_metrics()
        .is_some_and(|m| m.is_non_unit_ratio());

    let statutory = RegulatoryComplianceAudit::evaluate(
        alpha_hat,
        sniedovich_ok,
        mm_ok,
        emp.n_eff_exceedance_90 as f64,
        alert_audit.cognitive_triage_load_bps,
        triage_decibans,
        !siga_rep
            .natural_frequency_memo
            .formatted_summary
            .trim()
            .is_empty(),
        emp.corp_pav.rel_miscalibration,
        siga_rep.approved_end_to_end,
        chouldechova_ok,
    );

    println!("=== PORTABLE EPISTEMOLOGICAL CARD AUDIT (`gates --audit-card`) ===");
    println!(
        "  Card ID: {} | Layer: {:?} | Uncertainty: {:?} | Env: {} | K={}",
        card.id,
        card.layer,
        card.uncertainty,
        card.data_and_audit_contract.seed_and_environment,
        card.data_and_audit_contract.attempt_counter_k
    );
    println!(
        "  [1] 4-Filter Funnel: approved={}, EVPI={:.1}, n_eff(q90)={}, SBC chi2={:.2}",
        siga_rep.funnel_result.approved_all_filters,
        siga_rep.funnel_result.calculated_evpi,
        emp.n_eff_exceedance_90,
        siga_rep.funnel_result.sbc_chi2_statistic
    );
    println!(
        "  [2] 8-Step SIGA Roadmap: steps={:?}, p_null={:.4}, E_t/K={:.2}, REL_CORP={:.4}, RES_CORP={:.4}",
        siga_rep.steps_1_to_8_completed,
        siga_rep.null_p_value,
        siga_rep.e_process.multiplicity_adjusted_e_value,
        emp.corp_pav.rel_miscalibration,
        emp.corp_pav.res_resolution
    );
    println!(
        "  [3] Gigerenzer Natural Frequency Memo: {}",
        siga_rep.natural_frequency_memo.formatted_summary
    );
    println!(
        "  [4] Statutory Compliance (EU AI Act Arts. 9/10/14/15 & LGPD Art. 20): Art.9={}, Art.10={}, Art.14/LGPD20={}, Art.15/Phi4={} -> approved={}",
        statutory.eu_ai_act_art9_risk_management_ok,
        statutory.eu_ai_act_art10_lgpd_minimization_ok,
        statutory.eu_ai_act_art14_lgpd_art20_human_oversight_ok,
        statutory.eu_ai_act_art15_robustness_fairness_ok,
        statutory.all_statutory_requirements_approved
    );

    let simpson = SimpsonCausalParadoxAudit::canonical_pearl_benchmark();
    let long_ctx = LongContextDpiDilutionAudit::evaluate(100, 3, 1, 2.2, 0.9);
    let five_senses = MultimodalCorrelatedFusionAudit::canonical_five_senses_benchmark();
    let premortem = PreMortemCounterfactualRootCauseAudit::canonical_benchmark();
    let cloud_lora = CloudLocalLoraArchitectureAudit::canonical_benchmark();
    let blueprints_ok = simpson.simpson_reversal_and_dag_dichotomy_verified
        && long_ctx.markov_blanket_pruning_approved
        && five_senses.multimodal_fusion_approved
        && premortem.premortem_and_sop_approved
        && cloud_lora.architecture_and_lora_approved;
    println!(
        "  [5] Strategic Weekly Blueprint Resonances (8/16/24w): Simpson(do_conf={:+.3} vs do_med={:+.3})={}, LongCtxDPI(gain={:+.1}dB)={}, FiveSenses(W={:.1}dB<naive={:.1}dB)={}, PreMortem(PNS={:.2}, SOP={:.1}b/s)={}, CloudLocalLoRA(TBS={:.0}<API={:.0}, E_r={:.3})={} -> approved={}",
        simpson.confounder_do_ate,
        simpson.mediator_do_ate,
        simpson.simpson_reversal_and_dag_dichotomy_verified,
        long_ctx.snr_gain_decibans,
        long_ctx.markov_blanket_pruning_approved,
        five_senses.shielded_correlated_decibans,
        five_senses.naive_independent_decibans,
        five_senses.multimodal_fusion_approved,
        premortem.true_root_cause_pns,
        premortem.sop_cognitive_load_bps,
        premortem.premortem_and_sop_approved,
        cloud_lora.tbs_hybrid_total_cost,
        cloud_lora.pure_api_total_cost,
        cloud_lora.lora_retained_energy_ratio,
        cloud_lora.architecture_and_lora_approved,
        blueprints_ok
    );

    if !siga_rep.approved_end_to_end
        || !statutory.all_statutory_requirements_approved
        || !blueprints_ok
    {
        return Err(format!(
            "Card '{}' failed end-to-end SIGA, statutory, or blueprint resonance audit",
            card.id
        ));
    }
    println!(
        "=== CARD '{}' APPROVED END-TO-END (EXIT CODE 0) ===",
        card.id
    );
    Ok(())
}
