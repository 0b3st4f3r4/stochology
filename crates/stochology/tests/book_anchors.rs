//! Specification test for the book's structure and for the educational arc.
//!
//! The book is a single numbered sequence, `00`–`85`, in five parts. Nothing here generates prose:
//! what the test guarantees is that the numbering is dense and unique, that the table of contents
//! covers every chapter and links only to files that exist, that each chapter is of exactly one of
//! the two kinds the contract allows — *framework* (declares its laboratory on a banner line
//! `> Laboratory N of the `stochology` CLI: `--chapter N`.` and carries a CLI output block) or
//! *educational* (no banner, states three outcomes and closes on its bridge) — and that every code
//! anchor an educational chapter cites resolves to a real symbol. Every chapter opens on its `# `
//! title: no chapter may carry an HTML marker, because upload content sniffers classify a file
//! that begins with `<!--` as `text/html` and refuse it as markdown.

use std::path::{Path, PathBuf};

const FIRST: u8 = 0;
const LAST: u8 = 85;
const FRAMEWORK_CHAPTERS: usize = 9;

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root resolves")
}

fn book_dir() -> PathBuf {
    workspace_root().join("book-src")
}

/// Every numbered chapter, keyed by its two-digit number.
fn chapters() -> Vec<(u8, String, String)> {
    let mut out = Vec::new();
    for entry in std::fs::read_dir(book_dir()).expect("book-src directory") {
        let path = entry.expect("dir entry").path();
        let Some(name) = path
            .file_name()
            .and_then(|n| n.to_str())
            .map(str::to_string)
        else {
            continue;
        };
        let number = name
            .get(0..2)
            .filter(|_| name.len() > 3 && name.as_bytes()[2] == b'-')
            .and_then(|n| n.parse::<u8>().ok());
        if let Some(n) = number {
            let body = std::fs::read_to_string(&path).expect("read chapter");
            out.push((n, name, body));
        }
    }
    out.sort();
    out
}

/// The laboratory banner, `> Laboratory N of the `stochology` CLI: `--chapter N`.`, that only
/// framework chapters carry. Returns the laboratory number it declares.
fn laboratory_banner(body: &str) -> Option<u8> {
    body.lines().find_map(|line| {
        let rest = line.strip_prefix("> Laboratory ")?;
        let (lab, tail) = rest.split_once(' ')?;
        let lab: u8 = lab.parse().ok()?;
        (tail == format!("of the `stochology` CLI: `--chapter {lab}`.")).then_some(lab)
    })
}

fn is_framework(body: &str) -> bool {
    laboratory_banner(body).is_some()
}

/// Source files of both crates, keyed by basename (`epistemic.rs`, `grammar.rs`, ...).
fn source_files() -> Vec<(String, String)> {
    let mut out = Vec::new();
    for crate_dir in ["crates/stochology/src", "crates/stochology-cli/src"] {
        let dir = workspace_root().join(crate_dir);
        for entry in std::fs::read_dir(&dir).expect("crate src directory") {
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
    assert!(
        !out.is_empty(),
        "no Rust sources found to resolve anchors against"
    );
    out
}

/// Collects every backticked `file.rs::Symbol` anchor in a markdown body.
fn anchors_in(body: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = body;
    while let Some(open) = rest.find('`') {
        let after = &rest[open + 1..];
        let Some(close) = after.find('`') else { break };
        let candidate = &after[..close];
        if candidate.contains(".rs::") && !candidate.contains(char::is_whitespace) {
            out.push(candidate.to_string());
        }
        rest = &after[close + 1..];
    }
    out
}

fn symbol_is_declared(body: &str, symbol: &str) -> bool {
    // Each keyword must be followed by `symbol` and then by a non-identifier character,
    // so that `mod crps_decomposition_tests` cannot satisfy a claim about `crps_decomposition`
    // and `struct Triadic` cannot satisfy a claim about `TriadicIntegrity`.
    ["struct", "enum", "type", "mod", "fn", "const"]
        .iter()
        .any(|keyword| declares(body, keyword, symbol))
}

fn declares(body: &str, keyword: &str, symbol: &str) -> bool {
    let needle = format!("{keyword} {symbol}");
    body.match_indices(needle.as_str()).any(|(at, _)| {
        !matches!(
            body[at + needle.len()..].chars().next(),
            Some(c) if c.is_alphanumeric() || c == '_'
        )
    })
}

#[test]
fn chapters_run_00_to_85_without_gaps_or_duplicates() {
    let found: Vec<u8> = chapters().iter().map(|(n, _, _)| *n).collect();
    assert_eq!(
        found,
        (FIRST..=LAST).collect::<Vec<u8>>(),
        "the book must be one dense sequence, 00..=85"
    );
}

#[test]
fn each_chapter_is_exactly_one_kind() {
    let all = chapters();
    let marked: Vec<&(u8, String, String)> = all
        .iter()
        .filter(|(_, _, body)| is_framework(body))
        .collect();
    assert_eq!(
        marked.len(),
        FRAMEWORK_CHAPTERS,
        "exactly nine chapters carry the `> Laboratory N of the `stochology` CLI` banner"
    );

    let mut failures = Vec::new();
    for (number, name, body) in &all {
        if !body.starts_with("# ") {
            failures.push(format!(
                "{name}: chapter must open on its `# ` title (a leading `<!--` is sniffed as HTML)"
            ));
        }
        if body.contains("<!--") {
            failures.push(format!("{name}: chapters carry no HTML comments"));
        }
        let blocks = body.matches("=== CHAPTER ").count();
        if let Some(declared) = laboratory_banner(body) {
            if blocks == 0 {
                failures.push(format!(
                    "{name}: framework chapter without a CLI output block"
                ));
            }
            let lab = number.checked_sub(41).unwrap_or_default();
            if declared != lab {
                failures.push(format!(
                    "{name}: banner declares Laboratory {declared}, expected Laboratory {lab}"
                ));
            }
        } else {
            if blocks != 0 {
                failures.push(format!(
                    "{name}: educational chapter must never contain a CLI output block"
                ));
            }
            if *number != FIRST {
                let outcomes = body
                    .matches("## By the end of this chapter you can")
                    .count();
                if outcomes != 1 {
                    failures.push(format!(
                        "{name}: expected one outcomes heading, found {outcomes}"
                    ));
                }
                for item in ["1. ", "2. ", "3. "] {
                    if !body.contains(&format!("\n{item}")) {
                        failures.push(format!("{name}: missing numbered outcome `{item}`"));
                    }
                }
                let bridges = body.matches("## Bridge").count();
                if bridges != 1 {
                    failures.push(format!(
                        "{name}: expected exactly one `## Bridge`, found {bridges}"
                    ));
                }
            }
        }
    }
    assert!(
        failures.is_empty(),
        "chapter contracts:\n{}",
        failures.join("\n")
    );
}

#[test]
fn every_educational_anchor_resolves_to_a_real_symbol() {
    let sources = source_files();
    let educational: Vec<_> = chapters()
        .into_iter()
        .filter(|(_, _, body)| !is_framework(body))
        .collect();
    let total: usize = educational
        .iter()
        .map(|(_, _, body)| anchors_in(body).len())
        .sum();
    assert!(
        total >= 40,
        "the arc cites only {total} code anchors; the extraction is probably broken"
    );

    let mut failures = Vec::new();
    for (_, chapter, body) in &educational {
        for anchor in anchors_in(body) {
            let mut parts = anchor.split("::");
            let file = parts.next().unwrap_or_default();
            let path: Vec<&str> = parts.collect();
            let Some((_, source)) = sources.iter().find(|(name, _)| name == file) else {
                failures.push(format!(
                    "{chapter}: `{anchor}` — no source file named {file}"
                ));
                continue;
            };
            let resolved = match path.as_slice() {
                [symbol] => symbol_is_declared(source, symbol),
                [ty, member] => {
                    symbol_is_declared(source, ty)
                        && source.contains(format!("fn {member}(").as_str())
                }
                _ => false,
            };
            if !resolved {
                failures.push(format!(
                    "{chapter}: `{anchor}` — symbol path {path:?} not declared in {file}"
                ));
            }
        }
    }
    assert!(
        failures.is_empty(),
        "unresolved code anchors in the educational arc:\n{}",
        failures.join("\n")
    );
}

#[test]
fn summary_is_a_complete_and_ordered_table_of_contents() {
    let summary_path = book_dir().join("SUMMARY.md");
    let summary = std::fs::read_to_string(&summary_path).expect("read SUMMARY.md");

    for (_, name, _) in chapters() {
        assert!(
            summary.contains(&format!("({name})")),
            "SUMMARY.md does not list {name}"
        );
    }

    let mut previous = None;
    for target in summary.split("](").skip(1) {
        let Some(end) = target.find(')') else {
            continue;
        };
        let file = &target[..end];
        if file.starts_with("http") || file.is_empty() {
            continue;
        }
        assert!(
            book_dir().join(file).exists(),
            "SUMMARY.md links to missing file {file}"
        );
        if let Ok(n) = file[0..2].parse::<u8>() {
            if let Some(p) = previous {
                assert!(n > p, "SUMMARY.md is out of order at {file}");
            }
            previous = Some(n);
        }
    }

    for part in [
        "# Part I —",
        "# Part II —",
        "# Part III —",
        "# Part IV —",
        "# Part V —",
    ] {
        assert!(
            summary.contains(part),
            "SUMMARY.md lost the heading `{part}`"
        );
    }
    assert!(
        !summary.contains("Part 0") && !summary.contains("Part VI"),
        "the book has exactly five parts"
    );
}
