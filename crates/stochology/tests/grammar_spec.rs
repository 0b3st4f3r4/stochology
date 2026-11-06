//! Specification test binding [`notes/The Grammar of Aim.md`] to `crates/stochology/src/grammar.rs`.
//!
//! The document is the formal artifact; this test is the guarantee that it is not prose. Four
//! independent checks: the anchor block resolves against the real sources, the lexicon of the
//! document matches the lexicon of the code (seven sites, four verdicts), the worked example quotes
//! names the engine actually emits, and the recognizer never disagrees with the engine across the
//! eight cells of the $2 \times 2 \times 2$ factorial cube.

use std::path::{Path, PathBuf};

use stochology::DEFAULT_SEED;
use stochology::engine::{CommunityReserveEnv, StochologicalAgent};
use stochology::grammar::{
    AgentAction, AgentActionKind, BOUNDARY_BAND, GrammarBudget, GrammarVerdict, GuardSite,
    Sentence, Shot, TriadAxis, UnparsableReason, assess_report, enumerate, enumerate_with_coverage,
    judge, parse_ebnf_grammar, shortest_violation, shortest_violation_with_coverage,
};
use stochology::synthesis::CollapseProposition;

const THRESHOLD: f64 = 0.50;

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root resolves")
}

fn grammar_note() -> String {
    let path = workspace_root().join("notes/The Grammar of Aim.md");
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()))
}

/// Extracts the fenced `grammar-anchors` block: lines of the form `Name -> path :: Symbol`.
fn anchors(note: &str) -> Vec<(String, String, String)> {
    let mut out = Vec::new();
    let mut inside = false;
    for line in note.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("```") {
            inside = !inside && trimmed.contains("grammar-anchors");
            continue;
        }
        if !inside || trimmed.is_empty() {
            continue;
        }
        let (name, rest) = trimmed
            .split_once("->")
            .unwrap_or_else(|| panic!("anchor line without '->': {trimmed}"));
        let (path, symbol) = rest
            .split_once("::")
            .unwrap_or_else(|| panic!("anchor line without '::': {trimmed}"));
        out.push((
            name.trim().to_string(),
            path.trim().to_string(),
            symbol.trim().to_string(),
        ));
    }
    out
}

/// Extracts all brace-balanced `pub struct/enum/trait {owner} { ... }` and `impl {owner} { ... }` blocks from `text`.
fn extract_owner_blocks(text: &str, owner: &str) -> String {
    let mut blocks = String::new();
    let prefixes = [
        format!("pub struct {owner}"),
        format!("pub enum {owner}"),
        format!("pub trait {owner}"),
        format!("impl {owner}"),
    ];
    for prefix in &prefixes {
        let mut search_from = 0usize;
        while let Some(rel_idx) = text[search_from..].find(prefix) {
            let start = search_from + rel_idx;
            let after_kw = start + prefix.len();
            if let Some(next_ch) = text[after_kw..].chars().next()
                && (next_ch.is_alphanumeric() || next_ch == '_')
            {
                search_from = after_kw;
                continue;
            }
            if let Some(open_rel) = text[after_kw..].find('{') {
                let open_pos = after_kw + open_rel;
                let mut depth = 0i32;
                let mut end_pos = text.len();
                for (offset, ch) in text[open_pos..].char_indices() {
                    if ch == '{' {
                        depth += 1;
                    } else if ch == '}' {
                        depth -= 1;
                        if depth == 0 {
                            end_pos = open_pos + offset + 1;
                            break;
                        }
                    }
                }
                blocks.push_str(&text[start..end_pos]);
                blocks.push('\n');
                search_from = end_pos;
            } else {
                break;
            }
        }
    }
    blocks
}

#[test]
fn every_anchor_resolves_against_the_source_tree() {
    let note = grammar_note();
    let list = anchors(&note);
    assert!(
        list.len() >= 12,
        "expected at least 12 anchors, found {}",
        list.len()
    );
    let root = workspace_root();
    for (name, rel_path, symbol) in &list {
        let source = root.join(rel_path);
        let text = std::fs::read_to_string(&source)
            .unwrap_or_else(|e| panic!("anchor {name}: cannot read {}: {e}", source.display()));
        if let Some((owner, member)) = symbol.split_once("::") {
            let scope_slice = extract_owner_blocks(&text, owner);
            assert!(
                !scope_slice.is_empty(),
                "anchor {name}: owner type '{owner}' not found in {rel_path}"
            );
            let member_pats = [
                format!("pub fn {member}("),
                format!("pub fn {member}<"),
                format!("pub const fn {member}("),
                format!("fn {member}("),
                format!("pub {member}:"),
            ];
            assert!(
                member_pats.iter().any(|p| scope_slice.contains(p)),
                "anchor {name} is stale: qualified member '{owner}::{member}' not found inside {owner} blocks in {rel_path}"
            );
        } else {
            let decl_patterns = [
                format!("pub fn {symbol}("),
                format!("pub struct {symbol}"),
                format!("pub enum {symbol}"),
                format!("pub trait {symbol}"),
                format!("pub const {symbol}"),
                format!("pub type {symbol}"),
            ];
            assert!(
                decl_patterns.iter().any(|pat| text.contains(pat)),
                "anchor {name} is stale: top-level declaration for '{symbol}' not found in {rel_path}"
            );
        }
    }
}

#[test]
fn note_lexicon_matches_the_code_lexicon() {
    let note = grammar_note();
    for site in GuardSite::ALL {
        let spelled = format!("{site:?}");
        assert!(
            note.contains(&spelled),
            "guard site '{spelled}' is absent from the note"
        );
        let expected_axis_tok = match site.axis() {
            TriadAxis::Epistemic => "S_{\\mathrm{epi}}",
            TriadAxis::Vital => "S_{\\mathrm{vital}}",
            TriadAxis::Institutional => "S_{\\mathrm{gov}}",
        };
        let table_line = note
            .lines()
            .find(|l| l.contains(&format!("| `{spelled}` |")))
            .unwrap_or_else(|| panic!("Table 1 row missing for {spelled}"));
        assert!(
            table_line.contains(expected_axis_tok),
            "Table 1 row for {spelled} must map to axis {expected_axis_tok}: {table_line}"
        );
    }
    assert!(
        note.contains("\\mathrm{judge}(S, G) = \\mathrm{judge}(S', G')"),
        "§5 must state the formal non-commutation equation of judge(S, G)"
    );
    // M14: Verify §2 guard count & 3/2/2 partition, §4 strict evaluation order, and §7.3 cell-by-cell agreement rules
    assert!(
        note.contains("**Guard sites.** Seven, one per constitutive tension")
            && note.contains("partition is $3/2/2$"),
        "§2 must state Seven guard sites and the 3/2/2 partition"
    );
    let sec4_steps = [
        "1. **Skeleton.**",
        "2. **Totality of the annotation.**",
        "3. **Finite readings.**",
        "4. **The Leontief fold.**",
        "5. **The frontier band.**",
    ];
    let mut prev_pos = 0usize;
    for step_marker in sec4_steps {
        let pos = note
            .find(step_marker)
            .unwrap_or_else(|| panic!("§4 is missing ordered evaluation rule: {step_marker}"));
        assert!(
            pos > prev_pos,
            "§4 evaluation rules are out of order at {step_marker}"
        );
        prev_pos = pos;
    }
    assert!(
        note.contains("### **7.3 Agreement with the engine, cell by cell**")
            && note.contains("if the grammar returns `Collapse`, its proposition must equal the engine's diagnosis")
            && note.contains("if the grammar returns `Interior` or `Boundary`, the engine's diagnosis must be `IntactTriad`")
            && note.contains("if the grammar returns `Unparsable` for a trajectory the engine successfully produced, the test fails"),
        "§7.3 must state all three cell-by-cell agreement clauses"
    );
    for verdict in ["Interior", "Boundary", "Collapse", "Unparsable"] {
        assert!(
            note.contains(verdict),
            "verdict '{verdict}' is absent from the note"
        );
    }
    for reason in [
        "SkeletonMismatch",
        "UnguardedTransit",
        "NonFiniteMargin",
        "EmptySentence",
        "MissingControlTape",
        "OutOfScope",
        "UnauthorizedRead",
        "BudgetExhausted",
    ] {
        assert!(
            note.contains(reason),
            "quarantine reason '{reason}' is absent from the note"
        );
    }
    assert!(
        note.contains("Proposition1BlindUndersizing") && note.contains("IntactTriad"),
        "the note must quote the collapse propositions by their engine names"
    );
    assert!(
        note.contains("```ebnf"),
        "the note must carry the productions as an EBNF block"
    );
}

#[test]
fn seven_sites_partition_the_triad_three_two_two() {
    assert_eq!(GuardSite::ALL.len(), 7);
    let count = |axis: TriadAxis| GuardSite::ALL.iter().filter(|s| s.axis() == axis).count();
    assert_eq!(count(TriadAxis::Epistemic), 3);
    assert_eq!(count(TriadAxis::Vital), 2);
    assert_eq!(count(TriadAxis::Institutional), 2);
    assert!(GuardSite::is_bijection_over_circuit_transits());
}

#[test]
fn recognition_never_disagrees_with_the_engine() {
    let env = CommunityReserveEnv::new(DEFAULT_SEED, 24, 45.0);
    let cells = [
        ("111", true, true, true),
        ("011", false, true, true),
        ("101", true, false, true),
        ("110", true, true, false),
        ("100", true, false, false),
        ("010", false, true, false),
        ("001", false, false, true),
        ("000", false, false, false),
    ];
    for (key, e, v, g) in cells {
        let report = env.run_trajectory(&mut StochologicalAgent::with_axes(e, v, g));
        let engine = report.triadic_integrity.diagnose;
        match assess_report(&report, THRESHOLD) {
            GrammarVerdict::Collapse { proposition, .. } => assert_eq!(
                proposition, engine,
                "cell {key}: recogniser and engine disagree on the collapse proposition"
            ),
            GrammarVerdict::Interior | GrammarVerdict::Boundary { .. } => assert_eq!(
                engine,
                CollapseProposition::IntactTriad,
                "cell {key}: engine reports a collapse the recogniser did not see"
            ),
            GrammarVerdict::Unparsable(reason) => {
                panic!("cell {key}: a produced trajectory was quarantined ({reason:?})")
            }
        }
    }
}

#[test]
fn worked_example_in_the_note_is_reproducible() {
    let note = grammar_note();
    let env = CommunityReserveEnv::new(DEFAULT_SEED, 24, 45.0);
    let report = env.run_trajectory(&mut StochologicalAgent::without_epistemic_margin());
    match assess_report(&report, THRESHOLD) {
        GrammarVerdict::Collapse {
            proposition,
            at,
            step_t,
        } => {
            assert_eq!(
                proposition,
                CollapseProposition::Proposition1BlindUndersizing
            );
            assert_eq!(at, Some(GuardSite::T1Framing));
            assert_eq!(step_t, Some(0));
            assert!(
                note.contains("Proposition1BlindUndersizing") && note.contains("T1Framing"),
                "the note's worked example must name the proposition and the site"
            );
            assert!(
                (report.epistemic_coverage_rate - 0.25).abs() < 1e-9,
                "the note quotes 25.0% coverage for cell 011"
            );
            assert!(
                (report.triadic_integrity.leontief_minimum - 0.235).abs() < 5e-4,
                "the note quotes M_Leontief = 0.235 for cell 011 (got {})",
                report.triadic_integrity.leontief_minimum
            );
        }
        other => panic!("expected Collapse for the 011 ablation, got {other:?}"),
    }
}

#[test]
fn non_commutation_property_holds_for_the_acceptance_predicate() {
    let guards = GuardSite::ALL
        .iter()
        .map(|s| stochology::grammar::GuardReading::new(*s, 0.95, Some(0)))
        .collect::<Vec<_>>();
    let mut left = Shot::canonical(0, 0.80);
    let mut right = left;
    // Permute the contingency tape only.
    right.skopos.true_latent_demand = 40.0;
    right.skopos.raw_archive_mean = 0.10;
    right.skopos.realized_harm = 999.0;
    right.skopos.next_reserve = 0.0;
    right.skopos.hit_ruin_barrier = true;
    right.skopos.epistemic_bounds_covered_truth = false;
    left.telos = right.telos;
    let a = Sentence::new(vec![left], guards.clone());
    let b = Sentence::new(vec![right], guards.clone());
    assert_eq!(judge(&a, THRESHOLD), judge(&b, THRESHOLD));
    assert_eq!(judge(&a, THRESHOLD), GrammarVerdict::Interior);

    // P1 / B3 · NCOM-01: Two shots with provisioned_capacity == 0.0 differing ONLY in hit_ruin_barrier
    // must receive the exact same verdict (`Unparsable(SkeletonMismatch)`).
    let mut z_left = Shot::canonical(0, 0.80);
    z_left.telos.provisioned_capacity = 0.0;
    z_left.skopos.hit_ruin_barrier = false;
    let mut z_right = z_left;
    z_right.skopos.hit_ruin_barrier = true;
    let za = Sentence::new(vec![z_left], guards.clone());
    let zb = Sentence::new(vec![z_right], guards);
    assert_eq!(judge(&za, THRESHOLD), judge(&zb, THRESHOLD));
    assert_eq!(
        judge(&za, THRESHOLD),
        GrammarVerdict::Unparsable(UnparsableReason::SkeletonMismatch)
    );
}

#[test]
fn quarantine_is_reachable_and_is_not_a_collapse() {
    let guards = GuardSite::ALL
        .iter()
        .map(|s| stochology::grammar::GuardReading::new(*s, f64::NAN, None))
        .collect::<Vec<_>>();
    let sentence = Sentence::new(vec![Shot::canonical(0, 0.80)], guards);
    match judge(&sentence, THRESHOLD) {
        GrammarVerdict::Unparsable(UnparsableReason::NonFiniteMargin(site)) => {
            assert_eq!(site, GuardSite::T1Framing)
        }
        other => panic!("expected quarantine, got {other:?}"),
    }

    // P1 / B2 · FITA-01: Missing control tape in StepOutcome is quarantined as MissingControlTape
    let env = CommunityReserveEnv::new(DEFAULT_SEED, 24, 45.0);
    let mut report = env.run_trajectory(&mut StochologicalAgent::full());
    report.steps[0].attested_decision = None;
    assert_eq!(
        assess_report(&report, THRESHOLD),
        GrammarVerdict::Unparsable(UnparsableReason::MissingControlTape)
    );
}

#[test]
fn generative_movement_is_bounded_and_sealed() {
    let note = grammar_note();
    let ebnf =
        parse_ebnf_grammar(&note).expect("note EBNF must parse into 11 canonical productions");
    assert_eq!(ebnf.rules.len(), 11);

    // Mutating the EBNF block to `Sentence ::= banana` must fail the executable EBNF parser (B10 / §7 Criterion 5)
    let broken_note = "```ebnf\nSentence ::= banana\n```\n";
    assert!(parse_ebnf_grammar(broken_note).is_err());

    static GRID: [f64; 3] = [0.0, 0.60, 0.95];
    let budget = GrammarBudget {
        max_depth: 2,
        threshold: THRESHOLD,
        grid: &GRID,
    };
    let (sentences, cov3) = enumerate(&budget);
    assert_eq!(sentences.len(), 3_usize.pow(7));
    assert!(!cov3.truncated);
    assert!(sentences.iter().all(|s| s.verify_seal()));
    // Verify surface EBNF recognition matches judge() on the generated language
    for s in sentences.iter().take(64) {
        let surface = s.to_ebnf_surface();
        let parsed = ebnf
            .parse_sentence(&surface, s.guards.clone())
            .expect("generated EBNF surface must parse");
        assert_eq!(judge(&parsed, THRESHOLD), judge(s, THRESHOLD));
    }
    let collapses = sentences
        .iter()
        .filter(|s| matches!(judge(s, THRESHOLD), GrammarVerdict::Collapse { .. }))
        .count();
    assert_eq!(collapses, 3_usize.pow(7) - 2_usize.pow(7));
    let (counterexample, site) = shortest_violation(&budget).expect("a counterexample exists");
    assert_eq!(site, GuardSite::T1Framing);
    assert_eq!(counterexample.shots.len(), 1);
    assert_eq!(BOUNDARY_BAND, 0.05);
    let boundary_guards = GuardSite::ALL
        .iter()
        .map(|s| stochology::grammar::GuardReading::new(*s, 0.52, Some(0)))
        .collect::<Vec<_>>();
    let boundary_sentence = Sentence::new(vec![Shot::canonical(0, 0.80)], boundary_guards);
    assert!(matches!(
        judge(&boundary_sentence, THRESHOLD),
        GrammarVerdict::Boundary { .. }
    ));

    // Verify g = 5 yields 5^7 = 78,125 sentences without dropping the highest grid value (B10)
    static GRID5: [f64; 5] = [0.0, 0.30, 0.50, 0.70, 0.95];
    let budget5 = GrammarBudget {
        max_depth: 1,
        threshold: THRESHOLD,
        grid: &GRID5,
    };
    let (sentences5, cov5) = enumerate(&budget5);
    assert_eq!(sentences5.len(), 5_usize.pow(7));
    assert!(!cov5.truncated);
    assert!(
        sentences5
            .last()
            .expect("non-empty")
            .guards
            .iter()
            .all(|g| (g.margin - 0.95).abs() < 1e-12)
    );

    // P2 / M1 · COB-01: g = 7 returns 6^7 = 279,936 with coverage.truncated == true
    static GRID7: [f64; 7] = [0.0, 0.15, 0.30, 0.45, 0.60, 0.75, 0.90];
    let budget7 = GrammarBudget {
        max_depth: 1,
        threshold: THRESHOLD,
        grid: &GRID7,
    };
    let (violation7, cov7) = shortest_violation_with_coverage(&budget7);
    assert!(violation7.is_some());
    assert_eq!(cov7.requested, 7_usize.pow(7));
    assert_eq!(cov7.enumerated, 279_936);
    assert_eq!(cov7.distinct_levels, 6);
    assert!(cov7.truncated);

    // Duplicate grid levels deduplicate before checking MAX_ENUMERATION
    static DUP_GRID: [f64; 7] = [0.0, 0.0, 0.30, 0.30, 0.70, 0.70, 0.95];
    let dup_budget = GrammarBudget {
        max_depth: 1,
        threshold: THRESHOLD,
        grid: &DUP_GRID,
    };
    let (dup_sentences, dup_cov) = enumerate_with_coverage(&dup_budget);
    assert_eq!(dup_cov.distinct_levels, 4);
    assert_eq!(dup_sentences.len(), 4_usize.pow(7));
    assert!(!dup_cov.truncated);
}

#[test]
fn agents_differing_only_in_action_permission_receive_distinct_verdicts() {
    let guards = GuardSite::ALL
        .iter()
        .map(|s| stochology::grammar::GuardReading::new(*s, 0.95, Some(0)))
        .collect::<Vec<_>>();
    let shots = vec![Shot::canonical(0, 0.80)];

    let permitted = Sentence::with_agent_audit(
        shots.clone(),
        guards.clone(),
        vec![AgentAction {
            kind: AgentActionKind::Write,
            scope: vec!["reserve/vault".to_string()],
            principal: "agent-01".to_string(),
            on_behalf_of: "citizen-board".to_string(),
            contract_id: "contract-2026".to_string(),
            authorized_scopes: vec!["reserve/vault".to_string()],
            can_halt: true,
            token_cost: 10,
        }],
        Vec::new(),
        Some(100),
        Some(10),
    );
    let forbidden = Sentence::with_agent_audit(
        shots,
        guards,
        vec![AgentAction {
            kind: AgentActionKind::Write,
            scope: vec!["reserve/vault".to_string()],
            principal: "agent-01".to_string(),
            on_behalf_of: "citizen-board".to_string(),
            contract_id: "contract-2026".to_string(),
            authorized_scopes: vec!["telemetry/read".to_string()],
            can_halt: true,
            token_cost: 10,
        }],
        Vec::new(),
        Some(100),
        Some(10),
    );
    assert_eq!(judge(&permitted, THRESHOLD), GrammarVerdict::Interior);
    assert_eq!(
        judge(&forbidden, THRESHOLD),
        GrammarVerdict::Unparsable(UnparsableReason::OutOfScope)
    );
}
