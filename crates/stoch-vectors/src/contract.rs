//! §V.9 of the contract, as a check that reads the contract.
//!
//! AGENTS.md carries a table of four propositions, each with one canonical string
//! and a list of contradicting variants. The check is mechanical: after collapsing
//! runs of whitespace, the canonical string must occur somewhere in `book-src/`,
//! and no file of `book-src/` may contain one of that row's variants. The same
//! strings may not appear in crate sources either — `01` §7.1 forbids the core and
//! its harness from carrying a copy of the contract's prose.
//!
//! The file is read at test time from the workspace root, not embedded: a missing
//! AGENTS.md must fail loudly rather than silently checking nothing. That is why
//! `read_contract` returns the path it tried in its error.

use std::fs;
use std::path::{Path, PathBuf};

/// One row of the contract's concordance table.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConcordanceRow {
    pub proposition: String,
    pub canonical: String,
    pub variants: Vec<String>,
}

/// What a sweep established.
#[derive(Clone, Debug)]
pub struct Sweep {
    pub rows: Vec<ConcordanceRow>,
    pub book_files: usize,
    /// For each proposition, the files of `book-src/` that carry its canonical
    /// string.
    pub canonical_sites: Vec<(String, Vec<PathBuf>)>,
    pub errors: Vec<String>,
}

impl Sweep {
    pub fn passed(&self) -> bool {
        self.errors.is_empty()
    }
}

/// Read the workspace root from the harness's own location: `CARGO_MANIFEST_DIR`
/// is `crates/stoch-vectors`, and the workspace is two levels above it.
pub fn workspace_root() -> PathBuf {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest
        .parent()
        .and_then(Path::parent)
        .map(Path::to_path_buf)
        .unwrap_or(manifest)
}

/// Collapse every run of whitespace to one space, so that a reflowed paragraph
/// is not a contradiction.
pub fn collapse(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Read AGENTS.md and parse the concordance table.
pub fn read_contract(root: &Path) -> Result<Vec<ConcordanceRow>, String> {
    let path = root.join("AGENTS.md");
    let text = fs::read_to_string(&path).map_err(|error| {
        format!(
            "the contract must be read at test time, and it is missing: {} ({error})",
            path.display()
        )
    })?;
    parse_contract(&text)
}

/// Parse the four rows of the concordance table out of the contract text.
pub fn parse_contract(text: &str) -> Result<Vec<ConcordanceRow>, String> {
    let marker = "9. **Concordance.**";
    let mut lines = text.lines();
    let mut found_marker = false;
    for line in lines.by_ref() {
        if line.trim_start().starts_with(marker) {
            found_marker = true;
            break;
        }
    }
    if !found_marker {
        return Err(format!("the contract carries no `{marker}` section"));
    }

    let mut rows = Vec::new();
    let mut in_table = false;
    for line in lines {
        let line = line.trim();
        if !line.starts_with('|') {
            if in_table {
                break;
            }
            continue;
        }
        in_table = true;
        let cells: Vec<&str> = line.split('|').map(str::trim).collect();
        if cells.len() < 5 {
            continue;
        }
        if cells[1].starts_with("Proposition") || cells[1].starts_with(":---") {
            continue;
        }
        let canonical = backticked(cells[2]);
        let variants = backticked(cells[3]);
        if canonical.len() != 1 {
            return Err(format!(
                "row {:?} declares {} canonical strings, expected exactly one",
                cells[1],
                canonical.len()
            ));
        }
        if variants.is_empty() {
            return Err(format!(
                "row {:?} declares no contradicting variant, so the row cannot fail",
                cells[1]
            ));
        }
        rows.push(ConcordanceRow {
            proposition: cells[1].to_string(),
            canonical: canonical[0].clone(),
            variants,
        });
    }
    if rows.len() != 4 {
        return Err(format!(
            "the concordance table carries {} rows, expected 4",
            rows.len()
        ));
    }
    Ok(rows)
}

/// Every backticked span in a cell, in order.
fn backticked(cell: &str) -> Vec<String> {
    let mut spans = Vec::new();
    let mut rest = cell;
    while let Some(open) = rest.find('`') {
        let after = &rest[open + 1..];
        match after.find('`') {
            Some(close) => {
                spans.push(after[..close].to_string());
                rest = &after[close + 1..];
            }
            None => break,
        }
    }
    spans
}

/// Every markdown file under `book-src/`.
pub fn book_files(root: &Path) -> Result<Vec<PathBuf>, String> {
    let book = root.join("book-src");
    if !book.is_dir() {
        return Err(format!("book-src/ is missing: {}", book.display()));
    }
    let mut files = Vec::new();
    collect(&book, "md", &mut files)?;
    files.sort();
    Ok(files)
}

fn collect(directory: &Path, extension: &str, out: &mut Vec<PathBuf>) -> Result<(), String> {
    let entries = fs::read_dir(directory)
        .map_err(|error| format!("cannot read {}: {error}", directory.display()))?;
    for entry in entries {
        let entry = entry.map_err(|error| format!("cannot read an entry: {error}"))?;
        let path = entry.path();
        if path.is_dir() {
            collect(&path, extension, out)?;
        } else if path.extension().and_then(|e| e.to_str()) == Some(extension) {
            out.push(path);
        }
    }
    Ok(())
}

/// Run the sweep: canonical present, no variant present, and no contract prose in
/// the crate sources.
pub fn sweep(root: &Path) -> Result<Sweep, String> {
    let rows = read_contract(root)?;
    let files = book_files(root)?;
    let mut errors = Vec::new();
    let mut canonical_sites = Vec::new();

    let mut collapsed: Vec<(PathBuf, String)> = Vec::new();
    for path in &files {
        let text = fs::read_to_string(path)
            .map_err(|error| format!("cannot read {}: {error}", path.display()))?;
        collapsed.push((path.clone(), collapse(&text)));
    }

    for row in &rows {
        let canonical = collapse(&row.canonical);
        let sites: Vec<PathBuf> = collapsed
            .iter()
            .filter(|(_, text)| text.contains(&canonical))
            .map(|(path, _)| path.clone())
            .collect();
        if sites.is_empty() {
            errors.push(format!(
                "canonical string {:?} occurs nowhere in book-src/",
                row.canonical
            ));
        }
        canonical_sites.push((row.proposition.clone(), sites));

        for variant in &row.variants {
            let needle = collapse(variant);
            for (path, text) in &collapsed {
                if text.contains(&needle) {
                    errors.push(format!(
                        "{}: contradicts {:?} with variant {:?}",
                        path.display(),
                        row.proposition,
                        variant
                    ));
                }
            }
        }
    }

    // `01` §7.1: the crates carry no copy of the contract's prose.
    let mut sources = Vec::new();
    let crates = root.join("crates");
    if crates.is_dir() {
        collect(&crates, "rs", &mut sources)?;
    }
    for path in &sources {
        let text = collapse(
            &fs::read_to_string(path)
                .map_err(|error| format!("cannot read {}: {error}", path.display()))?,
        );
        for row in &rows {
            if text.contains(&collapse(&row.canonical)) {
                errors.push(format!(
                    "{}: carries the canonical string of {:?}",
                    path.display(),
                    row.proposition
                ));
            }
            for variant in &row.variants {
                if text.contains(&collapse(variant)) {
                    errors.push(format!(
                        "{}: carries a contradicting variant of {:?}",
                        path.display(),
                        row.proposition
                    ));
                }
            }
        }
    }

    Ok(Sweep {
        rows,
        book_files: files.len(),
        canonical_sites,
        errors,
    })
}
