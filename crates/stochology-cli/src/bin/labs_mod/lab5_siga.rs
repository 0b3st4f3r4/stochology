//! Lab 5: 4-Filter Eliminatory Funnel, 8-Step SIGA Roadmap, and Natural-Frequency Executive Memoranda on the 4 Canonical YAML Cards.

use std::path::Path;
use stochology_cli::{load_cards_from_disk, verify_card_against_empirical_data};
use stochology_siga::roadmap::OperatingPointSource;
use stochology_siga::{AlertFatigueAudit, EvidenceProcess, fit_shelf_beta_prior};

/// Renders the §II.C provenance class of an `AlertFatigueAudit` operating point.
fn describe_operating_point_source(source: &OperatingPointSource) -> String {
    match source {
        OperatingPointSource::Literature { source } => format!("Literature({source})"),
        OperatingPointSource::Assumed => {
            "Assumed (design operating point, not measured)".to_string()
        }
    }
}

/// Lab 5: 4-Filter Eliminatory Funnel, 8-Step SIGA Roadmap, and Natural-Frequency Executive Memoranda on the 4 Canonical YAML Cards.
pub fn lab_siga(root: &Path) {
    println!("=== LAB 5: 4-FILTER ELIMINATORY FUNNEL AND 8-STEP SIGA ROADMAP ===");
    let (shelf_a, shelf_b, shelf_rmse) = fit_shelf_beta_prior(0.02, 0.05, 0.09);
    let grapa_demo = EvidenceProcess::from_grapa_betting(&vec![0.45; 35], 0.05, 1)
        .expect("bounded GRAPA demo payoffs");
    let (epic_sepsis, stoch_triage) =
        AlertFatigueAudit::canonical_epic_sepsis_vs_stochological_gate();
    println!(
        "  [Filter 3 SHELF Beta Fit & Step 6 GRAPA] Beta(alpha={:.2}, beta={:.2}, CDF_RMSE={:.4}) | GRAPA E_35={:.2} (rejects_H0={}) | [Bridge L2->L3 AlertFatigueAudit (10 bit/s & Turing Decibans)] EpicSepsis(PPV={:.1}%, FP/TP={:.1}, approved={}) vs StochologicalTarget(assumed TPR/FPR, PPV={:.1}%, W={:+.2} dB, load={:.1} bit/s <= 10, approved={})",
        shelf_a,
        shelf_b,
        shelf_rmse,
        grapa_demo.final_e_value,
        grapa_demo.rejects_null_hypothesis,
        epic_sepsis.positive_predictive_value_ppv * 100.0,
        epic_sepsis.false_alarms_per_true_case,
        epic_sepsis.approved_bridge_l2_l3,
        stoch_triage.positive_predictive_value_ppv * 100.0,
        stoch_triage.turing_weight_of_evidence_decibans,
        stoch_triage.cognitive_triage_load_bps,
        stoch_triage.approved_bridge_l2_l3
    );
    println!(
        "  [AlertFatigueAudit provenance] EpicSepsis={} | StochologicalTarget={}",
        describe_operating_point_source(&epic_sepsis.operating_point_source),
        describe_operating_point_source(&stoch_triage.operating_point_source)
    );
    let cards = load_cards_from_disk(root).expect("load canonical YAML cards from disk");
    for (rel_path, card) in cards {
        let (_audit, rep) = verify_card_against_empirical_data(root, &card)
            .expect("verify card against empirical dataset");
        println!(
            "  [SIGA {}] dataset={}, EVPI={:.1}, p_null={:.4} (unclamped), chi2_SBC={:.2}, E_30={:.1}, steps_1_to_8={:?}, approved={}",
            rel_path,
            card.data_and_audit_contract.dataset_path,
            rep.funnel_result.calculated_evpi,
            rep.null_p_value,
            rep.funnel_result.sbc_chi2_statistic,
            rep.e_process.final_e_value,
            rep.steps_1_to_8_completed,
            rep.approved_end_to_end
        );
        for line in rep.natural_frequency_memo.formatted_summary.lines() {
            println!("    {line}");
        }
    }
}
