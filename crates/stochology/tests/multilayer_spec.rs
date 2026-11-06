//! Specification tests for `notes/The Multilayer Graph of Knowledge and Aim.md` and `multilayer.rs`.
//!
//! Enforces 1-to-1 parity between the formal note, the 35-layer Rust types (`KnowledgeOrder`,
//! `KnowledgeStratum`, `TriadicExposureRegime`, `MultilayerGraph1`, `MultilayerGraph2`,
//! `MultilayerHomomorphism`), the exact $\binom{7}{3} = 35$ guard-site bijection, and the
//! 35 Part V chapters (`book-src/51-*.md`..`85-*.md`).

use std::fs;
use std::path::{Path, PathBuf};
use stochology::{
    CollapseProposition, KnowledgeOrder, KnowledgeStratum, LayerTriadReading, MultilayerGraph1,
    MultilayerGraph2, MultilayerHomomorphism, TriadicExposureRegime,
};

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("workspace root")
        .to_path_buf()
}

#[test]
fn every_multilayer_anchor_resolves_against_the_source_tree() {
    let root = workspace_root();
    let note_path = root.join("notes/The Multilayer Graph of Knowledge and Aim.md");
    let note = fs::read_to_string(&note_path).expect("read multilayer note");

    let block_start = note
        .find("```text\nmultilayer-anchors\n")
        .expect("multilayer-anchors block present");
    let after_header = &note[block_start + "```text\nmultilayer-anchors\n".len()..];
    let block_end = after_header.find("```").expect("closing fence");
    let block = &after_header[..block_end];

    let mut count = 0usize;
    for raw_line in block.lines() {
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }
        count += 1;
        let parts: Vec<&str> = line.split("::").collect();
        assert!(
            parts.len() == 2 || parts.len() == 3,
            "invalid anchor format: {line}"
        );
        let file_path = root.join(parts[0]);
        let src = fs::read_to_string(&file_path)
            .unwrap_or_else(|_| panic!("missing source file for anchor: {line}"));
        let sym = parts[1];
        let decl_found = src.contains(&format!("pub struct {sym}"))
            || src.contains(&format!("pub enum {sym}"))
            || src.contains(&format!("pub fn {sym}("))
            || src.contains(&format!("pub const fn {sym}("));
        assert!(decl_found, "symbol `{sym}` not declared in {}", parts[0]);
        if parts.len() == 3 {
            let member = parts[2];
            let member_found = src.contains(&format!("pub fn {member}("))
                || src.contains(&format!("pub const fn {member}("));
            assert!(
                member_found,
                "member `{member}` of `{sym}` not found in {}",
                parts[0]
            );
        }
    }
    assert!(count >= 20, "expected at least 20 multilayer anchors");
}

#[test]
fn note_table_and_part_v_chapters_match_all_35_knowledge_orders() {
    let root = workspace_root();
    let note = fs::read_to_string(root.join("notes/The Multilayer Graph of Knowledge and Aim.md"))
        .expect("read multilayer note");
    let book_dir = root.join("book-src");

    for order in KnowledgeOrder::ALL {
        let code = order.code();
        let ch = order.chapter_number();
        let st = order.stratum().code();
        let p_site = order.primary_guard_site().code();
        let needle = format!("| `{code}` | `{ch}` | `{st}` | `{order:?}` | `{p_site}` |");
        assert!(
            note.contains(&needle),
            "note Table 1 missing or mismatched row `{needle}` for {:?}",
            order
        );

        let chapter_path = book_dir.join(order.chapter_file());
        assert!(
            chapter_path.is_file(),
            "missing Part V chapter file `{}` for {:?}",
            order.chapter_file(),
            order
        );
    }
}

#[test]
fn multilayer_bijection_dual_stratification_and_leontief_homomorphism_hold() {
    let g1 = MultilayerGraph1::canonical();
    let g2 = MultilayerGraph2::canonical();

    assert!(g1.is_strict_dag());
    assert_eq!(g1.nodes.len(), 35);
    assert_eq!(g2.nodes.len(), 35);
    assert!(g1.every_ancestor_cone_spans_all_three_axes());
    assert!(g2.is_bijection_over_35_guard_combinations());
    assert!(g2.every_stratum_covers_all_seven_sites());
    assert!(g2.is_balanced_five_orders_per_primary_site());

    for stratum in KnowledgeStratum::ALL {
        assert_eq!(stratum.orders().len(), 5);
        assert!(stratum.covers_all_seven_guard_sites());
        assert!(stratum.has_distinct_primary_guard_sites());
    }

    let regime_totals: usize = TriadicExposureRegime::ALL
        .iter()
        .map(|r| r.expected_count())
        .sum();
    assert_eq!(regime_totals, 35);

    let mut readings = KnowledgeOrder::ALL.map(|order| LayerTriadReading {
        order,
        s_epi: 0.90,
        s_vital: 0.86,
        s_gov: 0.92,
    });
    // Trigger Collapse Proposition 1 (Epistemic failure) at Layer 09 (Experimental/Sampling/Causal).
    let l09_slot = (KnowledgeOrder::L09ExperimentalSamplingCausal.layer_index() - 1) as usize;
    readings[l09_slot].s_epi = 0.12;

    let hom = MultilayerHomomorphism::evaluate(&g1, &readings);
    // Layer 35 (Polycentric Commons & Biospheric Order) transitively depends on Layer 09.
    assert_eq!(
        hom.cumulative[34].diagnose,
        CollapseProposition::Proposition1BlindUndersizing
    );
    assert_eq!(
        hom.bottleneck_ancestor[34],
        KnowledgeOrder::L09ExperimentalSamplingCausal
    );
}
