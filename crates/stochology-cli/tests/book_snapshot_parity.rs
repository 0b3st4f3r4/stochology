//! Book snapshot parity (§II.C rule 8): every `labs`/`gates` output line quoted inside a fenced block of
//! `book-src/*.md` must appear verbatim (trimmed) in the output that the current code prints.
//!
//! Quoted output lines are recognized by their trimmed prefix: `[`, `NEGATIVE PROOF`, `=== `, or `Out of every`.
//! The collected output is `labs --all`, `gates`, `gates --defect <name>` for each of the 21 negative proofs, and
//! `gates --audit-card` on the shaped application of `examples/clay/` (Protocol Profile worked example).

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread;

use stochology_cli::all_defect_names;

/// Trimmed prefixes that mark a fenced-block line as quoted `labs`/`gates` output.
const QUOTED_OUTPUT_PREFIXES: [&str; 4] = ["[", "NEGATIVE PROOF", "=== ", "Out of every"];

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .expect("workspace root")
        .to_path_buf()
}

/// Runs one binary invocation from the workspace root and returns stdout followed by stderr.
fn run_capture(bin: &'static str, args: Vec<String>, root: PathBuf) -> String {
    let out = Command::new(bin)
        .args(&args)
        .current_dir(&root)
        .output()
        .unwrap_or_else(|e| panic!("failed to spawn {bin} {args:?}: {e}"));
    let mut text = String::from_utf8_lossy(&out.stdout).into_owned();
    text.push('\n');
    text.push_str(&String::from_utf8_lossy(&out.stderr));
    text
}

/// Collects the trimmed output lines of `labs --all`, `gates`, the 21 `gates --defect <name>` runs, and the
/// shaped-application `gates --audit-card` run.
fn collect_output_lines(root: &Path) -> HashSet<String> {
    let labs_bin: &'static str = env!("CARGO_BIN_EXE_labs");
    let gates_bin: &'static str = env!("CARGO_BIN_EXE_gates");
    let mut invocations: Vec<(&'static str, Vec<String>)> = vec![
        (labs_bin, vec!["--all".to_string()]),
        (gates_bin, Vec::new()),
        (
            gates_bin,
            vec![
                "--audit-card".to_string(),
                "examples/clay/card_clay_night_shift_overflow.yaml".to_string(),
            ],
        ),
    ];
    for defect in all_defect_names() {
        invocations.push((gates_bin, vec!["--defect".to_string(), defect.to_string()]));
    }
    // The invocations are independent and deterministic (`seed = 2026`), so they run concurrently.
    let handles: Vec<_> = invocations
        .into_iter()
        .map(|(bin, args)| {
            let root = root.to_path_buf();
            thread::spawn(move || run_capture(bin, args, root))
        })
        .collect();
    let mut lines = HashSet::new();
    for handle in handles {
        let text = handle.join().expect("binary capture thread panicked");
        for line in text.lines() {
            lines.insert(line.trim().to_string());
        }
    }
    lines
}

/// Returns `(line_number, trimmed_line)` for every quoted output line inside a fenced block of `markdown`.
fn quoted_output_lines(markdown: &str) -> Vec<(usize, String)> {
    let mut quoted = Vec::new();
    let mut open_fence: Option<&'static str> = None;
    for (idx, raw) in markdown.lines().enumerate() {
        let trimmed = raw.trim();
        match open_fence {
            None => {
                if trimmed.starts_with("```") {
                    open_fence = Some("```");
                } else if trimmed.starts_with("~~~") {
                    open_fence = Some("~~~");
                }
            }
            Some(fence) => {
                if trimmed.starts_with(fence) {
                    open_fence = None;
                } else if QUOTED_OUTPUT_PREFIXES
                    .iter()
                    .any(|prefix| trimmed.starts_with(prefix))
                {
                    quoted.push((idx + 1, trimmed.to_string()));
                }
            }
        }
    }
    quoted
}

#[test]
fn fenced_block_parser_extracts_only_quoted_output_lines() {
    let md = "[outside] not fenced\n~~~text\n  [G1] inside\nplain\n=== LAB 1 ===\n~~~\n```\nOut of every 100\n```\n";
    let got = quoted_output_lines(md);
    assert_eq!(
        got,
        vec![
            (3, "[G1] inside".to_string()),
            (5, "=== LAB 1 ===".to_string()),
            (8, "Out of every 100".to_string()),
        ]
    );
}

#[test]
fn book_quoted_output_matches_current_labs_and_gates_output() {
    let root = workspace_root();
    let output_lines = collect_output_lines(&root);

    let mut chapters: Vec<PathBuf> = fs::read_dir(root.join("book-src"))
        .expect("read book-src")
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|ext| ext == "md"))
        .collect();
    chapters.sort();

    let mut checked = 0usize;
    let mut offending = Vec::new();
    for chapter in &chapters {
        let markdown = fs::read_to_string(chapter).expect("read book chapter");
        let name = chapter
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default();
        for (line_no, line) in quoted_output_lines(&markdown) {
            checked += 1;
            if !output_lines.contains(&line) {
                offending.push(format!("book-src/{name}:{line_no}: {line}"));
            }
        }
    }

    assert!(
        checked > 0,
        "no quoted labs/gates output found in book-src/*.md"
    );
    assert!(
        offending.is_empty(),
        "{} of {} quoted output lines in book-src/*.md do not match the current labs/gates output \
         (regenerate them from the binaries, never edit by hand):\n{}",
        offending.len(),
        checked,
        offending.join("\n")
    );
}
