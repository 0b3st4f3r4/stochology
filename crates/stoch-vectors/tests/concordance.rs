//! §V-9 as a test: the contract's concordance table is read at test time from
//! `AGENTS.md`, and the manuscript is swept against it.
//!
//! Three things are proven here, and the third is the one that matters: that the
//! sweep passes on the repository as it stands; that a missing contract is a
//! loud failure rather than a silent skip; and that a planted contradiction is
//! caught and named.

use std::fs;
use std::path::PathBuf;

use stoch_vectors::contract;

fn scratch(name: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "stoch-vectors-{}-{name}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&path);
    fs::create_dir_all(&path).expect("scratch root");
    path
}

#[test]
fn the_concordance_table_is_satisfied_by_the_manuscript() {
    let root = contract::workspace_root();
    let sweep = contract::sweep(&root).unwrap_or_else(|error| panic!("sweep: {error}"));
    for error in &sweep.errors {
        println!("violation: {error}");
    }
    println!(
        "{} propositions, {} manuscript files",
        sweep.rows.len(),
        sweep.book_files
    );
    for (proposition, files) in &sweep.canonical_sites {
        println!("{:?}: canonical in {} file(s)", proposition, files.len());
    }
    assert!(sweep.passed(), "{} violation(s)", sweep.errors.len());
    assert_eq!(sweep.rows.len(), 4, "the table has four propositions");
    assert!(sweep.book_files > 80, "the manuscript is the sweep's subject");
    for (proposition, files) in &sweep.canonical_sites {
        assert!(
            !files.is_empty(),
            "{proposition:?} occurs nowhere in the manuscript"
        );
    }
}

#[test]
fn a_missing_contract_is_a_loud_failure() {
    let root = scratch("no-contract");
    let error = contract::read_contract(&root).expect_err("no AGENTS.md here");
    println!("missing contract: {error}");
    assert!(error.contains("AGENTS.md"), "the path is named: {error}");
    assert!(error.contains("not found") || error.contains("No such file"), "{error}");
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn a_planted_contradiction_is_caught_and_named() {
    let real = fs::read_to_string(contract::workspace_root().join("AGENTS.md"))
        .expect("the contract is readable");
    let rows = contract::parse_contract(&real).expect("the contract parses");
    assert_eq!(rows.len(), 4);

    let root = scratch("planted-variant");
    fs::write(root.join("AGENTS.md"), &real).expect("write contract");
    let book = root.join("book-src");
    fs::create_dir_all(&book).expect("book-src");

    // The four canonical strings, present, so the sweep's first clause is met.
    let mut clean = String::from("# Fixture chapter\n\n");
    for row in &rows {
        clean.push_str(&format!("{}\n\n", row.canonical));
    }
    fs::write(book.join("clean.md"), clean).expect("write clean chapter");

    // One contradicting variant, planted in one file.
    let planted = rows[1].variants[0].clone();
    fs::write(
        book.join("trap.md"),
        format!("# Trap chapter\n\n{}\n", planted),
    )
    .expect("write trap chapter");

    let sweep = contract::sweep(&root).unwrap_or_else(|error| panic!("sweep: {error}"));
    for error in &sweep.errors {
        println!("planted violation: {error}");
    }
    assert!(!sweep.passed(), "the planted variant must fail the sweep");
    assert_eq!(sweep.errors.len(), 1, "{:?}", sweep.errors);
    assert!(sweep.errors[0].contains("trap.md"), "{}", sweep.errors[0]);
    assert!(sweep.errors[0].contains(&planted), "{}", sweep.errors[0]);

    let _ = fs::remove_dir_all(&root);
}
