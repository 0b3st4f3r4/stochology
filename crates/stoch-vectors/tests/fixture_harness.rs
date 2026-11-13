//! The normative fixture, run and compared.
//!
//! The first test is the acceptance condition: 50 rows, each one executed and
//! compared against the declaration it carries. The second is the corruption
//! condition: change one cell of one row and the harness must name that row and
//! print both lines. The third proves the harness reads the *input* columns and
//! not only the expectation columns — otherwise the corruption test could pass
//! on an echo.

use stoch_vectors::fixture;
use stoch_vectors::schema;

fn normative() -> String {
    let path = stoch_vectors::fixture_path();
    std::fs::read_to_string(&path).unwrap_or_else(|error| {
        panic!(
            "the normative fixture must be readable at {}: {error}",
            path.display()
        )
    })
}

#[test]
fn the_normative_fixture_agrees_with_its_declaration() {
    let report = fixture::run(&normative()).expect("the fixture parses");
    assert_eq!(report.rows(), 52, "52 records");
    assert_eq!(report.functions, [22, 20, 6, 4], "step/accept/diagnose/seal");
    let mismatches = report.mismatches();
    for mismatch in &mismatches {
        println!("{} declared: {}", mismatch.id, mismatch.expected);
        println!("{} returned: {}", mismatch.id, mismatch.actual);
    }
    assert!(report.agrees(), "{} rows diverge", mismatches.len());
}

/// The widening exists for one reason: with a single `margin` cell the Leontief
/// fold could never discriminate, and the boundary/skeleton coherence of B.1
/// line 163 could never bite. These six rows are that proof, printed so a human
/// can read the answers the sealed artifact actually produced.
#[test]
fn the_fixture_discriminates_the_leontief_fold() {
    let rows = schema::parse(&normative()).expect("the fixture parses");
    let case = |id: &str| {
        let row = rows
            .iter()
            .find(|row| row.id == id)
            .unwrap_or_else(|| panic!("row {id} exists"));
        let observed = fixture::observed_line(row);
        println!("{id}: {}", observed);
        observed
    };

    // (i) the fold sits on the epistemic axis while a vital site is declared: the
    // declared site does not carry the fold, so the sentence is unparsable.
    assert_eq!(case("a13"), "accept verdict=3 reason=1");
    // (ii) the fold sits on the vital axis and a vital site is declared.
    assert_eq!(case("a14"), "accept verdict=1 site=4 margin=101000");
    // (iii) the mirror image: the fold on the epistemic axis, an epistemic site.
    assert_eq!(case("a15"), "accept verdict=1 site=1 margin=101000");
    // (iii, literal) the same margins as (ii) with an epistemic site: coherence
    // fails in the other direction too, which a flat cell could never show.
    assert_eq!(case("a16"), "accept verdict=3 reason=1");
    // (iv) a triple tie: every axis attains the fold, so every site is coherent.
    assert_eq!(case("a17"), "accept verdict=1 site=4 margin=101000");
    // (v) one axis below the threshold: the fold is the minimum, so it collapses.
    assert_eq!(
        case("a18"),
        "accept verdict=2 proposition=1 site=1 step=7"
    );
}

#[test]
fn the_proposal_fixture_is_byte_identical_to_the_embedded_one() {
    let text = normative();
    assert_eq!(
        text.as_bytes(),
        stoch_core::VECTORS_BYTES,
        "the proposal's fixture and the core's embedded copy must be the same bytes"
    );
    assert_eq!(
        stoch_core::vectors_hash_hex().as_slice(),
        stoch_core::VECTORS_SHA256_HEX.as_bytes()
    );
    assert_eq!(stoch_core::VECTORS_DATA_ROWS, 52);
}

#[test]
fn a_corrupted_expectation_is_named_and_shown() {
    let rows = schema::parse(&normative()).expect("the fixture parses");
    let mut detected = 0usize;

    for (index, row) in rows.iter().enumerate() {
        let Some(column) = schema::expectation_column(row) else {
            continue;
        };
        let mut corrupted = rows.clone();
        let declared = corrupted[index].at(column);
        corrupted[index].set(column, declared + 1);

        let report = fixture::run_rows(&corrupted);
        let mismatches = report.mismatches();
        assert_eq!(
            mismatches.len(),
            1,
            "{}: corrupting {} must break exactly one row, broke {}",
            row.id,
            schema::HEADER[column],
            mismatches.len()
        );
        assert_eq!(mismatches[0].id, row.id, "the named row is the corrupted one");
        println!(
            "corrupted {} {}: {} -> {} | declared {:?} returned {:?}",
            row.id,
            schema::HEADER[column],
            declared,
            declared + 1,
            mismatches[0].expected,
            mismatches[0].actual
        );
        detected += 1;
    }

    assert_eq!(detected, rows.len(), "every row carries a checkable cell");
}

#[test]
fn a_corrupted_input_changes_the_answer() {
    let mut rows = schema::parse(&normative()).expect("the fixture parses");
    let index = rows
        .iter()
        .position(|row| row.id == "s02")
        .expect("s02 exists");
    // s02 probes a capacity of 300 inside (200, 400): (400 - 300) / 200 = 500_000.
    // One unit higher and the fraction is (400 - 301) / 200 = 495_000.
    rows[index].set(schema::C_C, 301);

    let report = fixture::run_rows(&rows);
    let mismatches = report.mismatches();
    assert_eq!(mismatches.len(), 1, "only the corrupted row diverges");
    assert_eq!(mismatches[0].id, "s02");
    println!(
        "input corruption: declared {:?} returned {:?}",
        mismatches[0].expected, mismatches[0].actual
    );
    assert!(
        mismatches[0].actual.contains("fraction=495000"),
        "the operator was actually called: {}",
        mismatches[0].actual
    );
    assert!(
        mismatches[0].expected.contains("fraction=500000"),
        "the declaration still says what the file says: {}",
        mismatches[0].expected
    );
}

#[test]
fn a_record_that_drops_a_key_it_must_carry_is_a_parse_error_not_a_pass() {
    let text = normative();
    let mut records: Vec<String> = text.lines().map(|line| line.to_string()).collect();
    // A `step` record: it is judged on the extent, so dropping `l` would replace a
    // declared bound with an absence the entry point cannot see.
    let index = records
        .iter()
        .position(|line| line.contains(" fn=0 ") && line.contains(" l="))
        .expect("the fixture has a step record");
    records[index] = records[index]
        .split(' ')
        .filter(|field| !field.starts_with("l="))
        .collect::<Vec<&str>>()
        .join(" ");
    let joined = records.join("\n") + "\n";
    let error = schema::parse(&joined).expect_err("a missing required key cannot parse");
    println!("parse error: {error}");
    assert!(error.message.contains("is missing key \"l\""), "{error}");
}

