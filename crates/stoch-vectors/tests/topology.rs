//! The topology of Part V: `binom(7, 3) = 35` triples, the Vandermonde partition
//! `12 + 9 + 9 + 4 + 1 = 35`, and Table B.1 reconciled row by row.
//!
//! This is an explorer, not a gate on the operator: nothing here feeds a verdict.
//! The table is read from the manuscript at test time, so a divergence is a
//! divergence between the book and the enumeration, not between two copies of a
//! constant in the harness.

use stoch_vectors::topology;

fn sources() -> (String, String) {
    let table = std::fs::read_to_string(stoch_vectors::appendix_b_path())
        .unwrap_or_else(|error| {
            panic!(
                "Appendix B must be readable at {}: {error}",
                stoch_vectors::appendix_b_path().display()
            )
        });
    let preface = std::fs::read_to_string(stoch_vectors::preface_path())
        .unwrap_or_else(|error| {
            panic!(
                "the preface must be readable at {}: {error}",
                stoch_vectors::preface_path().display()
            )
        });
    (table, preface)
}

#[test]
fn the_table_reconciles_with_the_book() {
    let (table, preface) = sources();
    let parity = topology::reconcile(&table, &preface).unwrap_or_else(|errors| {
        for error in &errors {
            println!("divergence: {error}");
        }
        panic!("{} divergence(s) against Table B.1", errors.len())
    });
    println!("{}", topology::render(&parity));

    assert_eq!(parity.table_rows, 35, "thirty-five rows");
    assert_eq!(parity.triples_exhausted, 35, "the table exhausts binom(7, 3)");
    assert_eq!(parity.strata.len(), 7, "seven strata");
    assert_eq!(parity.anchors.len(), 7, "seven recorded anchors");
    for site in 1..=7 {
        assert_eq!(
            parity.per_site_primary[site], 5,
            "T{site} is primary in exactly five strata"
        );
    }
    for stratum in &parity.strata {
        assert_eq!(stratum.orders.len(), 5, "five orders per stratum");
        assert_eq!(stratum.primaries.len(), 5);
        let mut distinct = stratum.primaries.clone();
        distinct.sort_unstable();
        distinct.dedup();
        assert_eq!(
            distinct.len(),
            5,
            "stratum {} has five pairwise distinct primary sites",
            stratum.stratum
        );
        assert!(
            stratum.anchor.is_some(),
            "stratum {} has a recorded anchor",
            stratum.stratum
        );
        assert!(
            stratum.primaries.contains(&stratum.anchor.unwrap()),
            "the anchor of stratum {} is the primary site of one of its orders",
            stratum.stratum
        );
    }
}

#[test]
fn the_vandermonde_partition_sums_to_the_binomial() {
    // The binomial count, class by class: C(3, kE) C(2, kV) C(2, kG).
    let classes = topology::vandermonde();
    let total: u64 = classes.iter().map(|(_, count)| count).sum();
    println!("classes (E, V, G): {classes:?}");
    assert_eq!(topology::binom(7, 3), 35);
    assert_eq!(total, topology::binom(7, 3), "the partition is exhaustive");

    let count = |signature: (u8, u8, u8)| {
        classes
            .iter()
            .find(|(candidate, _)| *candidate == signature)
            .map(|(_, count)| *count)
            .unwrap_or(0)
    };
    assert_eq!(count((1, 1, 1)), 12, "trans-triadic keystones");
    assert_eq!(count((3, 0, 0)), 1, "pure epistemic concentration");
    assert_eq!(count((1, 0, 2)) + count((1, 2, 0)), 6);

    // The book's 12 + 9 + 9 + 4 + 1 is the same thirty-five, counted by the
    // exposure label each row prints rather than by raw axis occupancy: the pure
    // epistemic triple sits in both the `V = 0` and the `G = 0` class, and the
    // label gives it to neither, which is why the two nines and not two tens.
    let (table, _) = sources();
    let rows = topology::parse_table(&table).expect("the table parses");
    let labels = [
        "Trans-Triadic Keystone",
        "Vital Axial Exposure",
        "Governance Axial Exposure",
        "Epistemic Axial Exposure",
        "Pure Epistemic Concentration",
    ];
    let mut counts = [0usize; 5];
    for row in &rows {
        let index = labels
            .iter()
            .position(|label| *label == row.exposure)
            .unwrap_or_else(|| panic!("{}: unknown exposure {:?}", row.order, row.exposure));
        counts[index] += 1;
    }
    println!("labels: {labels:?} -> {counts:?}");
    assert_eq!(counts, [12, 9, 9, 4, 1], "Table B.1's five regimes");
    assert_eq!(counts.iter().sum::<usize>(), 35);
    let raw_vital_absent: u64 = classes.iter().filter(|((_, v, _), _)| *v == 0).map(|(_, c)| *c).sum::<u64>();
    let raw_governance_absent: u64 = classes.iter().filter(|((_, _, g), _)| *g == 0).map(|(_, c)| *c).sum::<u64>();
    let raw_epistemic_absent: u64 = classes.iter().filter(|((e, _, _), _)| *e == 0).map(|(_, c)| *c).sum::<u64>();
    println!(
        "raw zero-classes: V=0 {raw_vital_absent}, G=0 {raw_governance_absent}, E=0 {raw_epistemic_absent}"
    );
    assert_eq!(
        (raw_vital_absent, raw_governance_absent, raw_epistemic_absent),
        (10, 10, 4),
        "the pure triple is the overlap the label drains"
    );
    assert_eq!(
        (counts[1] as u64 + 1, counts[2] as u64 + 1, counts[3] as u64),
        (raw_vital_absent, raw_governance_absent, raw_epistemic_absent)
    );
}

#[test]
fn the_triples_are_three_element_subsets_of_the_seven_sites() {
    let triples = topology::all_triples();
    assert_eq!(triples.len(), 35);
    for triple in &triples {
        for site in triple {
            assert!((1..=7).contains(site), "T{site} is not a guard site");
        }
        assert!(triple[0] < triple[1] && triple[1] < triple[2], "{triple:?} is sorted");
    }
    let mut unique = triples.clone();
    unique.sort_unstable();
    unique.dedup();
    assert_eq!(unique.len(), 35, "the enumeration has no duplicate");
}

#[test]
fn a_perturbed_table_is_refused() {
    let (table, preface) = sources();
    let mut lines: Vec<String> = table.lines().map(str::to_string).collect();
    let first = lines
        .iter()
        .position(|line| line.starts_with("| `L01`"))
        .expect("L01 is in the table");
    let second = lines
        .iter()
        .position(|line| line.starts_with("| `L02`"))
        .expect("L02 is in the table");
    let borrowed = topology::parse_table(&format!(
        "{}\n",
        lines[second]
    ))
    .expect("L02 parses")
    .remove(0);
    let original = topology::parse_table(&format!("{}\n", lines[first]))
        .expect("L01 parses")
        .remove(0);
    assert_ne!(borrowed.triple, original.triple);

    let from = format!(
        "{{T{}, T{}, T{}}}",
        original.triple[0], original.triple[1], original.triple[2]
    );
    let to = format!(
        "{{T{}, T{}, T{}}}",
        borrowed.triple[0], borrowed.triple[1], borrowed.triple[2]
    );
    lines[first] = lines[first].replacen(&from, &to, 1);
    let perturbed = lines.join("\n") + "\n";

    let errors = topology::reconcile(&perturbed, &preface).expect_err("a duplicate triple diverges");
    for error in &errors {
        println!("perturbed: {error}");
    }
    assert!(!errors.is_empty());
    assert!(
        errors.iter().any(|error| error.contains("L01")
            || error.contains("twice")
            || error.contains("no order")
            || error.contains("duplicate")),
        "the divergence names the row or the duplication: {errors:?}"
    );
}
