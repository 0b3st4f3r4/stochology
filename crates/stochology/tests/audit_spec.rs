//! Specification tests for the four cross-domain audits and for the audit coverage map in
//! `book-src/appendix-b-architecture-verification.md`.
//!
//! The audits are deterministic fixtures: every assertion below is relational (an ordering, a
//! bound, an absorbing event), never a re-typed constant, so the prose that quotes them and the
//! code that produces them cannot drift apart silently. The coverage map is parsed out of the
//! appendix and resolved against the source tree, so a gap in the matrix is a red test rather than
//! a missing paragraph.

use std::path::{Path, PathBuf};

use stochology::{
    AutomatonRegime, CellularAutomatonAudit, ControlRegime, DriftMonitorConfig, ElementaryRule,
    FrictionEnvelopeControlAudit, LatentModelExploitationAudit, LatentWorldModelConfig,
    ModelDriftFallbackAudit, SocialNetworkSpec, TopologyCentralityAudit,
};

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root resolves")
}

fn appendix_b() -> String {
    std::fs::read_to_string(
        workspace_root().join("book-src/appendix-b-architecture-verification.md"),
    )
    .expect("read appendix B")
}

// ---------------------------------------------------------------------------------------------
// 1. Latent world-model exploitation
// ---------------------------------------------------------------------------------------------

#[test]
fn dream_optimisation_is_an_optimism_gap_and_an_adversarial_goodhart_variant() {
    let audit = LatentModelExploitationAudit::evaluate(LatentWorldModelConfig::canonical());

    assert!(
        audit.surrogate_slope > 0.95 && audit.surrogate_slope < 1.05,
        "the linear surrogate fitted on the anchor grid must reproduce the local gain, got {}",
        audit.surrogate_slope
    );
    assert!(
        audit.dream_return > 3.0 * audit.realized_return_greedy,
        "dreamed return {:.4} must exceed the realised return {:.4} by a wide margin",
        audit.dream_return,
        audit.realized_return_greedy
    );
    assert!(
        audit.ruin_step_greedy.is_some(),
        "the greedy dreamer must destroy the world inside the horizon"
    );
    assert!(
        audit.extrapolation_depth > 2.5,
        "the greedy dreamer must leave the anchor support by more than 2.5 radii, got {:.4}",
        audit.extrapolation_depth
    );
    assert!(
        audit.out_of_support_fraction > 0.5,
        "most dream steps must fall outside the fitted support, got {:.4}",
        audit.out_of_support_fraction
    );
    assert!(
        audit.discrepancy_at_dream_end > 0.5,
        "the structural discrepancy at the dreamed terminal state must be large, got {:.4}",
        audit.discrepancy_at_dream_end
    );
    assert_eq!(
        audit.goodhart_variant,
        stochology::GoodhartVariant::Adversarial
    );
}

#[test]
fn temperature_buys_time_and_shrinks_the_discrepancy_monotonically() {
    let audit = LatentModelExploitationAudit::evaluate(LatentWorldModelConfig::canonical());

    for i in 1..LatentWorldModelConfig::SWEEP_LEN {
        assert!(
            audit.temperature_grid[i] > audit.temperature_grid[i - 1],
            "temperature sweep must be increasing"
        );
        assert!(
            audit.realized_return_by_tau[i] >= audit.realized_return_by_tau[i - 1],
            "raising tau must not reduce the realised return: {:?}",
            audit.realized_return_by_tau
        );
        assert!(
            audit.discrepancy_by_tau[i] <= audit.discrepancy_by_tau[i - 1] + 1e-12,
            "raising tau must not increase the terminal discrepancy: {:?}",
            audit.discrepancy_by_tau
        );
        let survived_before = audit.ruin_step_by_tau[i - 1].is_none();
        let survived_now = audit.ruin_step_by_tau[i].is_none();
        assert!(
            survived_before <= survived_now,
            "survival must be monotone in tau: {:?}",
            audit.ruin_step_by_tau
        );
        if let (Some(prev), Some(now)) = (audit.ruin_step_by_tau[i - 1], audit.ruin_step_by_tau[i])
        {
            assert!(now >= prev, "ruin step must not retreat as tau rises");
        }
    }

    let last = LatentWorldModelConfig::SWEEP_LEN - 1;
    assert!(
        audit.ruin_step_by_tau[last].is_none(),
        "the hottest arm must survive the horizon: {:?}",
        audit.ruin_step_by_tau
    );
    assert!(
        audit.discrepancy_by_tau[last] < 0.1,
        "the hottest arm must end close to the fitted support, got {:.4}",
        audit.discrepancy_by_tau[last]
    );
    assert_eq!(audit.ruin_step_by_tau[0], audit.ruin_step_greedy);
}

#[test]
fn dream_audit_is_bitwise_deterministic() {
    let a = LatentModelExploitationAudit::evaluate(LatentWorldModelConfig::canonical());
    let b = LatentModelExploitationAudit::evaluate(LatentWorldModelConfig::default());
    assert_eq!(a, b);
}

// ---------------------------------------------------------------------------------------------
// 2. Friction-envelope control benchmark
// ---------------------------------------------------------------------------------------------

#[test]
fn model_based_control_respects_the_friction_envelope_that_learned_policies_break() {
    let audit = FrictionEnvelopeControlAudit::evaluate(0.95, 20, 10.0);

    assert_eq!(audit.model_based.regime, ControlRegime::ModelBasedMpc);
    assert!(
        audit.model_based.completed_lap,
        "the model-based controller must finish the lap"
    );
    assert_eq!(
        audit.model_based.envelope_violation_steps, 0,
        "the model-based controller must stay inside the ellipse"
    );
    assert!(
        audit.model_based.max_utilization <= 0.97,
        "the model-based controller must hold its safety margin, got {:.4}",
        audit.model_based.max_utilization
    );
    assert!(
        audit
            .model_based
            .lap_time_s
            .unwrap_or(f64::INFINITY)
            .is_finite(),
        "the completed lap must carry a finite time"
    );

    assert!(
        !audit.in_simulation_tuned.completed_lap,
        "the in-simulation-tuned policy must lose grip on the deployed surface"
    );
    assert!(
        audit.in_simulation_tuned.max_utilization > 1.0,
        "the tuned policy must exceed the ellipse"
    );

    assert!(
        audit.demonstration_fitted.max_utilization < audit.in_simulation_tuned.max_utilization,
        "imitation must be gentler than in-simulation tuning: {:.4} vs {:.4}",
        audit.demonstration_fitted.max_utilization,
        audit.in_simulation_tuned.max_utilization
    );
    assert!(
        audit.demonstration_fitted.envelope_violation_steps
            <= audit.in_simulation_tuned.envelope_violation_steps,
        "imitation must not violate the envelope more often than tuning"
    );
    assert!(
        audit.demonstration_fitted.mean_speed > audit.in_simulation_tuned.mean_speed,
        "imitation must carry more speed before it breaks"
    );
    assert_eq!(audit.simulated_mu, 1.20);
    assert_eq!(audit.identified_mu, 0.95);
}

#[test]
fn the_margin_is_a_claim_about_the_world_and_expires_with_it() {
    let optimistic = FrictionEnvelopeControlAudit::evaluate(0.95, 20, 10.0);
    let degraded = FrictionEnvelopeControlAudit::evaluate(0.85, 20, 10.0);
    assert!(optimistic.model_based.completed_lap);
    assert!(
        !degraded.model_based.completed_lap,
        "a 10% friction deficit must destroy the model-based lap: the envelope is a claim about the surface"
    );
    assert!(degraded.model_based.max_utilization > optimistic.model_based.max_utilization);
    assert_eq!(degraded.model_based.lap_time_s, None);
}

#[test]
fn control_benchmark_is_bitwise_deterministic() {
    let a = FrictionEnvelopeControlAudit::evaluate(0.95, 20, 10.0);
    let b = FrictionEnvelopeControlAudit::evaluate(0.95, 20, 10.0);
    assert_eq!(a, b);
}

// ---------------------------------------------------------------------------------------------
// 2b. Elementary cellular automata: what a fixed local rule produces
// ---------------------------------------------------------------------------------------------

#[test]
fn one_fixed_local_rule_produces_four_different_regimes() {
    let zero = CellularAutomatonAudit::run(ElementaryRule::Zero);
    let ninety = CellularAutomatonAudit::run(ElementaryRule::Ninety);
    let thirty = CellularAutomatonAudit::run(ElementaryRule::Thirty);
    let one_ten = CellularAutomatonAudit::run(ElementaryRule::OneTen);

    assert_eq!(zero.regime, AutomatonRegime::Settling);
    assert!(zero.fixed_point_step.is_some());
    assert_eq!(zero.block_entropy_final, 0.0);
    assert_eq!(zero.live_fraction_final, 0.0);

    assert_eq!(ninety.regime, AutomatonRegime::Structured);
    assert!(ninety.fixed_point_step.is_none());
    assert!(
        ninety.block_entropy_final < 3.0,
        "the fractal rule must stay far below the dispersed ceiling, got {:.4}",
        ninety.block_entropy_final
    );
    assert!(ninety.live_fraction_final < 0.1);

    assert_eq!(thirty.regime, AutomatonRegime::Dispersed);
    assert!(thirty.block_entropy_final > 6.0);
    assert!((thirty.live_fraction_final - 0.5).abs() < 0.05);

    assert!(one_ten.fixed_point_step.is_none());
    assert!(one_ten.tail_period.is_none());
    assert!(one_ten.block_entropy_final > 5.0);
}

#[test]
fn the_audit_separates_three_regimes_and_says_why_it_cannot_separate_the_fourth() {
    let thirty = CellularAutomatonAudit::run(ElementaryRule::Thirty);
    let one_ten = CellularAutomatonAudit::run(ElementaryRule::OneTen);
    let gap = (thirty.block_entropy_final - one_ten.block_entropy_final).abs();
    assert!(
        gap < 1.5,
        "the block-entropy gap between the chaotic and the universal rule must stay small \
         ({gap:.4} bits): universality is a computational property, not an entropic one"
    );
    assert!(thirty.cells == 201 && thirty.steps == 200 && thirty.cells == one_ten.cells);
}

#[test]
fn automaton_audit_is_bitwise_deterministic() {
    for rule in ElementaryRule::ALL {
        assert_eq!(
            CellularAutomatonAudit::run(rule),
            CellularAutomatonAudit::run(rule)
        );
        assert_eq!(rule.table(), rule.table());
    }
    assert_eq!(ElementaryRule::ALL.len(), 4);
    assert_eq!(ElementaryRule::OneTen.code(), "R110");
}

// ---------------------------------------------------------------------------------------------
// 3. Social-network topology
// ---------------------------------------------------------------------------------------------

#[test]
fn topology_battery_recovers_communities_and_ranks_bridges_above_interiors() {
    let audit = TopologyCentralityAudit::evaluate(SocialNetworkSpec::canonical());

    assert_eq!(audit.node_count, 160);
    assert_eq!(audit.block_count, 4);
    assert!(audit.edge_count > 500);
    assert!(audit.density > 0.0 && audit.density < 0.5);
    assert!(audit.diameter >= 2);
    assert!(audit.clustering_coefficient > audit.density);
    assert!(
        audit.modularity_planted > 0.30,
        "the planted partition must be modular, got {:.4}",
        audit.modularity_planted
    );
    assert!(
        audit.modularity_planted >= audit.modularity_label_propagation,
        "label propagation must not beat the planted partition"
    );
    assert!(audit.modularity_label_propagation > 0.30);
    assert_eq!(audit.detected_community_count, 4);
    assert!(audit.pagerank_recovers_planted_prestige);
    assert!(audit.pagerank_block_means.windows(2).all(|w| w[0] > w[1]));
    assert!(audit.eigenvector_max > 0.0 && audit.eigenvector_max < 1.0);
    assert!(audit.hits_max_hub > 0.0 && audit.hits_max_authority > 0.0);
    assert!(
        audit.bridge_betweenness_mean > audit.interior_betweenness_mean,
        "bridge nodes must carry more betweenness than interiors: {:.6} vs {:.6}",
        audit.bridge_betweenness_mean,
        audit.interior_betweenness_mean
    );
    assert!(audit.betweenness_max <= 1.0);
}

#[test]
fn topology_battery_is_bitwise_deterministic() {
    let a = TopologyCentralityAudit::evaluate(SocialNetworkSpec::canonical());
    let b = TopologyCentralityAudit::evaluate(SocialNetworkSpec::default());
    assert_eq!(a, b);
}

// ---------------------------------------------------------------------------------------------
// 3b. Model drift, detection latency and fallback
// ---------------------------------------------------------------------------------------------

#[test]
fn fallback_is_what_turns_detection_into_recovery() {
    let audit = ModelDriftFallbackAudit::evaluate(DriftMonitorConfig::canonical());

    let detection = audit
        .detection_step
        .expect("the CUSUM monitor must detect the shift");
    assert!(
        detection > audit.change_point && detection <= audit.change_point + 8,
        "detection must trail the shift closely, got step {detection} for change point {}",
        audit.change_point
    );
    assert_eq!(
        audit.false_alarm_step, None,
        "a stationary stream must not trip the monitor"
    );
    assert!(
        !audit.survival_without_fallback,
        "the degraded model alone must exhaust the reserve inside the horizon"
    );
    assert_eq!(audit.reserve_without_fallback, 0.0);
    assert!(audit.survival_with_fallback);
    assert!(audit.reserve_with_fallback > 0.0);

    let ttr = audit.ttr_steps.expect("recovery must be scheduled");
    assert_eq!(ttr, detection + 12);
    assert!(
        ttr <= audit.tts_steps,
        "survival needs ttr = {ttr} <= tts = {}",
        audit.tts_steps
    );
}

#[test]
fn retraining_latency_is_the_binding_term_of_the_vital_margin() {
    let audit = ModelDriftFallbackAudit::evaluate(DriftMonitorConfig::canonical());
    let slow = audit
        .ttr_slow_retrain_steps
        .expect("the slow institution also detects the shift");
    assert!(slow > audit.tts_steps);
    assert!(
        !audit.survival_with_slow_retrain,
        "doubling the retraining latency must break the survival inequality"
    );
    assert!(
        audit.survival_with_fallback,
        "the fast institution survives on the same drift"
    );
}

#[test]
fn drift_audit_is_bitwise_deterministic() {
    let a = ModelDriftFallbackAudit::evaluate(DriftMonitorConfig::canonical());
    let b = ModelDriftFallbackAudit::evaluate(DriftMonitorConfig::default());
    assert_eq!(a, b);
}

// ---------------------------------------------------------------------------------------------
// 4. Audit coverage map (Appendix B.7)
// ---------------------------------------------------------------------------------------------

/// A symbol is declared when one of the Rust declaration keywords introduces it and the next
/// character is not an identifier character (so `struct Triadic` cannot satisfy `TriadicIntegrity`).
fn declared(source: &str, symbol: &str) -> bool {
    ["struct", "enum", "trait", "type", "fn", "const"]
        .iter()
        .any(|keyword| {
            let needle = format!("{keyword} {symbol}");
            source.match_indices(needle.as_str()).any(|(at, _)| {
                !matches!(
                    source[at + needle.len()..].chars().next(),
                    Some(c) if c.is_alphanumeric() || c == '_'
                )
            })
        })
}

fn book_files() -> Vec<String> {
    std::fs::read_dir(workspace_root().join("book-src"))
        .expect("book-src")
        .filter_map(|e| e.ok())
        .filter_map(|e| e.file_name().to_str().map(str::to_string))
        .collect()
}

fn crate_sources() -> Vec<(String, String)> {
    let root = workspace_root();
    let mut out = Vec::new();
    for crate_dir in ["crates/stochology/src", "crates/stochology-cli/src"] {
        for entry in std::fs::read_dir(root.join(crate_dir)).expect("crate src") {
            let path = entry.expect("dir entry").path();
            if path.extension().and_then(|e| e.to_str()) == Some("rs") {
                let name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .expect("file name")
                    .to_string();
                out.push((name, std::fs::read_to_string(&path).expect("read source")));
            }
        }
    }
    out
}

#[test]
fn audit_coverage_map_is_complete_and_resolves() {
    let text = appendix_b();
    let mut rows: Vec<(String, Vec<u8>, Vec<String>, String)> = Vec::new();
    for line in text.lines() {
        let trimmed = line.trim();
        if !trimmed.starts_with("| `AC-") {
            continue;
        }
        let cells: Vec<&str> = trimmed
            .trim_matches('|')
            .split('|')
            .map(str::trim)
            .collect();
        assert_eq!(
            cells.len(),
            5,
            "coverage row must carry five cells: {trimmed}"
        );
        let id = cells[0].trim_matches('`').to_string();
        let chapters: Vec<u8> = cells[2]
            .split(',')
            .map(|c| {
                c.trim()
                    .parse::<u8>()
                    .unwrap_or_else(|_| panic!("{id}: chapter cell is not a number: {}", cells[2]))
            })
            .collect();
        let anchors: Vec<String> = cells[3]
            .split(',')
            .map(|a| a.trim().trim_matches('`').to_string())
            .collect();
        rows.push((id, chapters, anchors, cells[4].to_string()));
    }

    assert_eq!(
        rows.len(),
        16,
        "the audit coverage map must list all 16 audit domains, found {}",
        rows.len()
    );
    for expected in 1..=16u8 {
        let id = format!("AC-{expected:02}");
        assert!(
            rows.iter().any(|(row_id, ..)| row_id == &id),
            "coverage map is missing row `{id}`"
        );
    }

    let files = book_files();
    let sources = crate_sources();

    for (id, chapters, anchors, status) in &rows {
        assert!(
            matches!(status.as_str(), "medido" | "simulado" | "prosa"),
            "{id}: unknown status `{status}`"
        );
        assert!(
            !chapters.is_empty(),
            "{id}: a coverage row must name at least one chapter"
        );
        for chapter in chapters {
            let prefix = format!("{chapter:02}-");
            assert!(
                files
                    .iter()
                    .any(|f| f.starts_with(&prefix) && f.ends_with(".md")),
                "{id}: chapter {chapter} has no file in book-src"
            );
        }
        for anchor in anchors {
            let mut parts = anchor.split("::");
            let file = parts.next().unwrap_or_default();
            let path: Vec<&str> = parts.collect();
            let source = &sources
                .iter()
                .find(|(name, _)| name == file)
                .unwrap_or_else(|| panic!("{id}: anchor `{anchor}` names no known source file"))
                .1;
            let resolved = match path.as_slice() {
                [symbol] => declared(source, symbol),
                [ty, member] => declared(source, ty) && source.contains(&format!("fn {member}(")),
                _ => false,
            };
            assert!(
                resolved,
                "{id}: anchor `{anchor}` does not resolve to a declaration in {file}"
            );
        }
    }
}
