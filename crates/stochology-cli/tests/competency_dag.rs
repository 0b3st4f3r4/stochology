//! Competency DAG (`docs/competencies.toml`) verification: the graph is acyclic and honest against the
//! book, the pathways (one per negative proof, `DEFECT_COUNT`) agree with `docs/SYLLABUS.md`, and the blocks rendered
//! into `book-src/00-preface.md` and `docs/SYLLABUS.md` match the data.
//! Regenerate the blocks with `cargo test -p stochology-cli --test competency_dag -- --ignored`.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use stochology_cli::{
    ALL_DEFECTS, BookIndex, CompetencyDag, CompetencyDepth, DEFECT_COUNT, replace_marked_block,
};

const PREFACE: &str = "book-src/00-preface.md";
const SYLLABUS: &str = "docs/SYLLABUS.md";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("workspace root")
        .to_path_buf()
}

fn load() -> (PathBuf, CompetencyDag, BookIndex) {
    let root = root();
    let dag = CompetencyDag::load(&root).expect("docs/competencies.toml must parse");
    let book = BookIndex::load(&root).expect("book-src/SUMMARY.md must load");
    (root, dag, book)
}

/// Movement of a chapter token according to the schedule of `docs/SYLLABUS.md` §5.
fn movement(token: &str) -> &'static str {
    match token.parse::<u32>().unwrap_or(0) {
        1..=2 => "I",
        3..=9 => "II",
        10..=14 => "III",
        15..=18 => "IV",
        19..=23 | 25 => "V",
        _ => "P",
    }
}

fn expected_blocks(
    dag: &CompetencyDag,
    book: &BookIndex,
) -> [(&'static str, &'static str, String); 5] {
    [
        (PREFACE, "competency-pillars", dag.render_pillar_cards(book)),
        (PREFACE, "competency-dag", dag.render_mermaid(book)),
        (
            PREFACE,
            "competency-pathways",
            dag.render_pathway_table(book, true),
        ),
        (SYLLABUS, "competency-dag", dag.render_mermaid(book)),
        (
            SYLLABUS,
            "competency-pathways",
            dag.render_pathway_table(book, false),
        ),
    ]
}

#[test]
fn competency_graph_passes_its_audit() {
    let (root, dag, book) = load();
    let problems = dag.audit(&book);
    assert!(
        problems.is_empty(),
        "competency DAG problems:\n{}",
        problems.join("\n")
    );
    for n in &dag.nodes {
        if let Some(path) = n.evidence.split("--audit-card ").nth(1) {
            assert!(
                root.join(path).is_file(),
                "'{}' evidence names missing card {path}",
                n.id
            );
        }
    }
}

#[test]
fn one_distinct_pathway_covers_each_negative_proof() {
    let (_, dag, book) = load();
    let pathways = dag.pathways(&book);
    assert_eq!(pathways.len(), DEFECT_COUNT);
    let proofs: BTreeSet<&str> = pathways.iter().map(|p| p.defect.as_str()).collect();
    let registry: BTreeSet<&str> = ALL_DEFECTS.iter().map(|d| d.name).collect();
    assert_eq!(proofs, registry);
    let closures: BTreeSet<Vec<usize>> = pathways
        .iter()
        .map(|p| {
            let mut s = p.steps.clone();
            s.sort_unstable();
            s
        })
        .collect();
    assert_eq!(
        closures.len(),
        DEFECT_COUNT,
        "two pathways have the same competencies"
    );
    for p in &pathways {
        assert_eq!(
            p.steps.last(),
            Some(&p.terminal),
            "'{}' must end at its proof",
            p.defect
        );
        for (k, &s) in p.steps.iter().enumerate() {
            for r in &dag.nodes[s].requires {
                let ri = dag.index_of(r).expect("known prerequisite");
                assert!(
                    p.steps[..k].contains(&ri),
                    "pathway '{}' reaches '{}' before its prerequisite '{r}'",
                    p.defect,
                    dag.nodes[s].id
                );
            }
        }
    }
}

#[test]
fn pathways_agree_with_the_syllabus_tracks_and_movements() {
    let (root, dag, book) = load();
    let syllabus = fs::read_to_string(root.join(SYLLABUS)).expect("syllabus");
    for p in dag.pathways(&book) {
        let needle = format!("`--defect {}`", p.defect);
        let line = syllabus
            .lines()
            .find(|l| l.contains(&needle))
            .unwrap_or_else(|| panic!("syllabus never assigns '{}'", p.defect));
        let at = line.find(&needle).unwrap_or(0);
        let syllabus_full = line[..at].contains("Full");
        let expected = if syllabus_full {
            CompetencyDepth::Full
        } else {
            CompetencyDepth::Core
        };
        assert_eq!(
            p.depth, expected,
            "pathway '{}' depth disagrees with the syllabus",
            p.defect
        );
    }

    let dist = syllabus
        .lines()
        .find(|l| l.contains("distributed with no repetition"))
        .expect("syllabus distribution line");
    let counts: Vec<usize> = dist
        .split('(')
        .skip(1)
        .filter_map(|c| c.split(')').next()?.parse().ok())
        .collect();
    let mut ours = [0usize; 6];
    for p in dag.pathways(&book) {
        let token = dag.nodes[p.terminal]
            .chapters
            .last()
            .expect("chapter")
            .as_str();
        let k = ["I", "II", "III", "IV", "V", "P"]
            .iter()
            .position(|m| *m == movement(token))
            .expect("movement");
        ours[k] += 1;
    }
    assert_eq!(
        counts,
        ours.to_vec(),
        "Movement I, II, III, IV, V, Practicum"
    );
}

#[test]
fn rendered_blocks_match_the_competency_data() {
    let (root, dag, book) = load();
    for (file, marker, body) in expected_blocks(&dag, &book) {
        let doc = fs::read_to_string(root.join(file)).expect("doc");
        let regenerated =
            replace_marked_block(&doc, marker, &body).unwrap_or_else(|e| panic!("{file}: {e}"));
        assert!(
            regenerated == doc,
            "{file} block '{marker}' is stale: run `cargo test -p stochology-cli --test competency_dag -- --ignored`"
        );
    }
}

#[test]
#[ignore]
fn regenerate_competency_blocks() {
    let (root, dag, book) = load();
    for (file, marker, body) in expected_blocks(&dag, &book) {
        let path = root.join(file);
        let doc = fs::read_to_string(&path).expect("doc");
        let updated = replace_marked_block(&doc, marker, &body).expect("markers");
        fs::write(&path, updated).expect("write doc");
    }
    for p in dag.pathways(&book) {
        println!(
            "{:>2} steps · {:<5} · {}",
            p.steps.len(),
            p.depth.label(),
            p.defect
        );
    }
}
