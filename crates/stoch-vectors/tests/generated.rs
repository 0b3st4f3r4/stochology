//! The generated corpus: a second, disjoint set of rows that reaches every item
//! of the minimum coverage list, and is executed by the same harness.
//!
//! The fixture is the declaration of record; the generated corpus is the check
//! that the declaration is not the only thing the operator can answer. The two
//! are compared separately so a divergence names its own corpus.

use stoch_vectors::{fixture, generator, schema};

#[test]
fn the_generated_corpus_agrees_with_its_declaration() {
    let text = generator::corpus();
    let rows = schema::parse(&text).expect("the generated corpus parses");
    let report = fixture::run_rows(&rows);
    let mismatches = report.mismatches();
    for mismatch in &mismatches {
        println!("{} declared: {}", mismatch.id, mismatch.expected);
        println!("{} returned: {}", mismatch.id, mismatch.actual);
    }
    assert!(report.agrees(), "{} generated rows diverge", mismatches.len());
    assert_eq!(rows.len(), generator::cases().len(), "every case is rendered");
    assert!(rows.len() >= 40, "the corpus is not a sample: {} rows", rows.len());
}

#[test]
fn the_generated_corpus_covers_the_minimum_set() {
    let rows = schema::parse(&generator::corpus()).expect("parses");
    let census = generator::census(&rows);
    let missing = census.missing();
    assert!(missing.is_empty(), "uncovered: {missing:?}");

    println!("rows: {}", census.rows);
    println!("regimes (Buffered..NoAction): {:?}", census.regimes);
    println!("verdicts (Interior..Unparsable): {:?}", census.verdicts);
    println!("collapse sites 1..7: {:?}", &census.collapse_sites[1..]);
    println!("quarantine slots 1..7: {:?}", &census.quarantine_slots[1..]);
    println!("bindings (none, E, V, G): {:?}", census.bindings);
    println!("error codes: {:?}", census.errors);

    // The named conditions of §6.3, each one asserted rather than inferred from
    // the buckets: a corpus can fill a histogram without ever testing the edge.
    assert!(census.saw_l_equals_c, "L == C");
    assert!(census.saw_u_equals_c, "U == C");
    assert!(census.saw_tts_equals_ttr, "TTS == TTR");
    assert!(census.saw_absorption, "absorption");
    assert!(census.saw_truncation, "truncating division");
    assert!(census.saw_underflow, "probe underflow");
    assert!(census.saw_irreversible_probe, "irreversible probe refusal");
    assert!(census.saw_missing_score, "a missing score");
    assert!(census.saw_q1_before_q2, "instrument before latency");
    assert!(census.saw_q2_before_q3, "latency before geometry");
    assert!(census.saw_extent_below_capacity, "U < C");
    assert!(census.saw_unnamed_collapse, "a collapse that cannot name itself");
    assert!(census.saw_mismatch, "site/proposition mismatch");

    // Every regime, every verdict, every site, every slot, every binding.
    for (tag, count) in census.regimes.iter().enumerate() {
        assert!(*count > 0, "regime {tag} is not exercised");
    }
    for (tag, count) in census.verdicts.iter().enumerate() {
        assert!(*count > 0, "verdict {tag} is not exercised");
    }
    for site in 1..=7 {
        assert!(census.collapse_sites[site] > 0, "site {site} has no collapse");
        assert!(census.quarantine_slots[site] > 0, "slot {site} is not quarantined");
    }
    assert!(census.bindings[0] > 0, "no diagnosis without a binding axis");
    for axis in 1..=3 {
        assert!(census.bindings[axis] > 0, "axis {axis} never binds");
    }
}

#[test]
fn every_generated_case_names_what_it_covers() {
    let cases = generator::cases();
    let mut ids: Vec<&str> = cases.iter().map(|case| case.id).collect();
    ids.sort_unstable();
    let before = ids.len();
    ids.dedup();
    assert_eq!(before, ids.len(), "case ids are unique");

    let rows = schema::parse(&generator::corpus()).expect("parses");
    for (case, row) in cases.iter().zip(rows.iter()) {
        assert_eq!(case.id, row.id, "the case and its row agree on the id");
        assert!(
            !case.covers.trim().is_empty(),
            "{} does not say what it covers",
            case.id
        );
    }
}

#[test]
fn the_generated_corpus_is_not_the_normative_fixture() {
    let generated = generator::corpus();
    let normative = std::fs::read_to_string(stoch_vectors::fixture_path()).expect("readable");
    assert_ne!(generated, normative, "the corpus is a second declaration");
    let generated_rows = schema::parse(&generated).expect("parses");
    let normative_rows = schema::parse(&normative).expect("parses");
    for row in &generated_rows {
        assert!(
            !normative_rows.iter().any(|other| other.id == row.id),
            "{} collides with a fixture id",
            row.id
        );
    }
}
