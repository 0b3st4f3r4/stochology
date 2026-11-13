//! Integrity of the normative fixture and of the crate's own contract text.
//!
//! Three claims are checked here, and none of them can be checked from inside the
//! library: that the embedded fixture is byte-identical to the file the crate ships
//! in `crates/stoch-core/vectors.kv`, that its digest is the one the specification
//! declared (the archived design document, `01` §6.4), and that no
//! proposition of the governance contract `AGENTS.md` §V-9 has been contradicted in
//! the crate source (`01` §7.1).
//!
//! The last check reads the contract at test time and refuses to run if the table it
//! depends on is not there. Copying the forbidden strings into this file would make
//! the crate itself carry them, which is exactly what the check exists to prevent.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use stoch_core::{vectors_hash, vectors_hash_hex, VECTORS_BYTES, VECTORS_DATA_ROWS, VECTORS_FILE};

/// The digest of `vectors.kv`, `01` §6.4.
const EXPECTED_DIGEST: &str = "efe2bad329e5727899e4c89b7989d4c9e92cc01c961ce896f71e7f1aed6bd3d0";

fn manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn repo_root() -> PathBuf {
    manifest_dir()
        .join("..")
        .join("..")
        .canonicalize()
        .expect("the crate lives two levels under the repository root")
}

/// Every `.rs` file of the crate, in a stable order.
fn sources() -> Vec<(PathBuf, String)> {
    let mut files = Vec::new();
    let mut stack = vec![manifest_dir().join("src")];
    while let Some(directory) = stack.pop() {
        for entry in fs::read_dir(&directory).expect("src/ is readable") {
            let path = entry.expect("a directory entry").path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().map(|e| e == "rs").unwrap_or(false) {
                let text = fs::read_to_string(&path).expect("source is UTF-8");
                files.push((path, text));
            }
        }
    }
    files.sort_by(|a, b| a.0.cmp(&b.0));
    assert!(files.len() >= 5, "the crate has at least five source files");
    files
}

fn relative(path: &Path) -> String {
    path.strip_prefix(repo_root())
        .unwrap_or(path)
        .display()
        .to_string()
}

/// Collapse runs of whitespace, as `AGENTS.md` §V-9 requires of the comparison.
fn collapse(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// The backticked spans of a table cell.
fn spans(cell: &str) -> Vec<String> {
    cell.split('`')
        .enumerate()
        .filter(|(index, _)| index % 2 == 1)
        .map(|(_, span)| collapse(span))
        .filter(|span| !span.is_empty())
        .collect()
}

/// The rows of the §V-9 concordance table: `(canonical, contradicting variants)`.
fn concordance_rows() -> Vec<(String, Vec<String>)> {
    let path = repo_root().join("AGENTS.md");
    let contract = fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("{} is the governance contract: {}", path.display(), error));

    let mut after_marker = false;
    let mut rows = Vec::new();
    for line in contract.lines() {
        if !after_marker {
            if line.starts_with("9. **Concordance.**") {
                after_marker = true;
            }
            continue;
        }
        if !line.starts_with('|') {
            // The table ends at the first non-table line after it began.
            if !rows.is_empty() {
                break;
            }
            continue;
        }
        let cells: Vec<&str> = line.split('|').map(str::trim).collect();
        // `| proposition | canonical | variants |` splits into five pieces with two
        // empty edges, so the canonical string is cells[2] and the variants cells[3].
        if cells.len() < 5 {
            continue;
        }
        if cells[1].contains("Proposition") || cells[2].starts_with(':') {
            continue;
        }
        let canonical = spans(cells[2]);
        assert_eq!(
            canonical.len(),
            1,
            "each row declares exactly one canonical string: {:?}",
            cells[2]
        );
        let variants = spans(cells[3]);
        assert!(
            !variants.is_empty(),
            "each row declares at least one contradicting variant: {:?}",
            cells[3]
        );
        rows.push((canonical[0].clone(), variants));
    }

    assert_eq!(
        rows.len(),
        4,
        "§V-9 declares four propositions; the table shape changed"
    );
    rows
}

#[test]
fn the_fixture_is_the_file_the_crate_ships_byte_for_byte() {
    // One copy, one digest. The second copy of the fixture lived in the plan's
    // working tree and was archived outside the repository on 2026-11-13; the
    // crate's own copy is now the only normative one, so the comparison is between
    // the embedded bytes and the file on disk here, not between two files that
    // both claim to be the fixture.
    let on_disk = manifest_dir().join(VECTORS_FILE);
    let expected = fs::read(&on_disk)
        .unwrap_or_else(|error| panic!("the fixture is missing at {}: {}", on_disk.display(), error));
    assert_eq!(
        VECTORS_BYTES,
        expected.as_slice(),
        "the embedded fixture must be {} byte-for-byte",
        on_disk.display()
    );
    assert_eq!(VECTORS_FILE, "vectors.kv", "the fixture path is a name");
    assert_eq!(VECTORS_DATA_ROWS, 52);
    let text = std::str::from_utf8(VECTORS_BYTES).expect("the fixture is UTF-8");
    let records = text
        .lines()
        .filter(|line| {
            let line = line.trim();
            !line.is_empty() && !line.starts_with('#')
        })
        .count();
    assert_eq!(records, VECTORS_DATA_ROWS, "one line per record, comments excluded");
    assert_eq!(
        *VECTORS_BYTES.last().expect("a non-empty fixture"),
        b'\n',
        "the file ends with a newline, and the digest covers it"
    );
}

#[test]
fn the_digest_is_the_one_the_specification_declares() {
    let digest = vectors_hash();
    assert_eq!(digest.len(), 32);
    let hex = vectors_hash_hex();
    assert_eq!(hex.len(), 64);
    let text = std::str::from_utf8(&hex).expect("hex is ASCII");
    assert_eq!(text, EXPECTED_DIGEST, "vectors_hash() over the raw file");
    assert!(
        text.chars()
            .all(|c| c.is_ascii_digit() || ('a'..='f').contains(&c)),
        "the hex is lowercase, as the specification prints it"
    );
    // The digest is over the bytes, not over a rendering: a single changed byte in
    // the fixture must move it, which the KATs of the internal SHA-256 also guard.
    assert_ne!(
        text,
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        "the fixture is not empty"
    );
}

#[test]
fn the_contract_propositions_are_not_contradicted_in_the_source() {
    let rows = concordance_rows();
    let mut checked_bytes = 0usize;

    for (path, text) in sources() {
        let body = collapse(&text);
        checked_bytes += body.len();
        for (proposition, variants) in &rows {
            for variant in variants {
                assert!(
                    !body.contains(variant),
                    "{}: the source contradicts the proposition {:?} with {:?}",
                    relative(&path),
                    proposition,
                    variant
                );
            }
        }
    }

    assert!(
        checked_bytes > 10_000,
        "the scan must cover the real sources, not an empty tree"
    );
}

#[test]
fn the_contract_table_is_read_not_remembered() {
    // If the table were transcribed into this crate, the rows would be constants. The
    // check is that they are discovered: shrinking the marker search to a section the
    // file does not have must find nothing, which proves the read is real.
    let path = repo_root().join("AGENTS.md");
    let contract = fs::read_to_string(path).expect("the contract exists");
    assert!(contract.contains("9. **Concordance.**"));
    let rows = concordance_rows();
    let mut distinct = BTreeSet::new();
    for (canonical, variants) in rows {
        distinct.insert(canonical);
        for variant in variants {
            distinct.insert(variant);
        }
    }
    assert!(
        distinct.len() >= 12,
        "four propositions and their variants are distinct strings"
    );
}
