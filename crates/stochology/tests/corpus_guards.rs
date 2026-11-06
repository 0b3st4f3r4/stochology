//! Executable guards over the prose corpus and over the hand-maintained export inventory.
//!
//! Both guards exist because a defect crossed the entire gate while every stage stayed green.
//!
//! **Guard 1 — control characters in prose.** `book-src/46` once carried two literal BEL bytes
//! (`0x07`) inside `$\hat{\alpha}$`, so the LaTeX silently rendered `\hat{lpha}`. A control byte is
//! neither whitespace nor an ASCII escape sequence: `cargo fmt`, `clippy`, every test, the KaTeX
//! hygiene stage and the final `verify.sh` stage all passed, and a byte-level search for `\\hat{`
//! cannot see it either. This guard scans every byte of the prose corpus through `char::is_control`
//! and allows exactly two control characters — `U+000A` LF and `U+0009` TAB — which are the only ones
//! the corpus uses, so no legitimate file is banned by accident.
//!
//! **Guard 2 — Appendix B.8 export-inventory drift.** B.8 is a hand-maintained table that claims to
//! enumerate every public item of both crates, module by module. A table with a completeness claim
//! that nothing checks drifts on the first `pub fn` anyone adds. This guard parses B.8 and compares it
//! against the sources in both directions at item level: no undocumented item, no stale entry, no
//! wrong per-module count, no wrong total, and no module with public surface left without a row.
//!
//! The pinned definition of a **live pub item** is the counting rule stated in B.8 itself, under
//! "**Counting rule (the contract the drift guard pins).**" — the prose paragraph and this parser are
//! checked against each other by `the_prose_counting_rule_and_the_parser_cannot_diverge`, so neither
//! side can move without the other:
//!
//! - an item is identified by `(module file, name)`; a name redeclared in a second `impl` block of
//!   the same module is one item, which is how B.8 lists and counts it;
//! - counted, in five categories: `pub struct` / `pub enum` / `pub trait`; `pub fn` (including
//!   `pub const fn` and a trait method implemented inside an `impl … for …` block); `pub const NAME:`;
//!   `pub static NAME:`; and a `pub type` alias;
//! - not counted: `pub use` re-exports, `pub mod` declarations, enum variants (they belong to their
//!   `enum` row), and everything after the first `#[cfg(test)]` in a file — test scaffolding is not
//!   exported surface;
//! - `pub type` aliases ARE counted: `ResourceId` (grammar.rs), `Rng` (kernel.rs) and
//!   `SosaJudgmentGrade` (governance.rs) each have their own `Aliases:` line and their heading counts
//!   are alias-inclusive.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root resolves")
}

// ---------------------------------------------------------------------------------------------
// Guard 1 — control characters in prose
// ---------------------------------------------------------------------------------------------

/// The only control characters the corpus may contain: `U+000A` LINE FEED and `U+0009` CHARACTER TABULATION.
const ALLOWED_CONTROLS: [char; 2] = ['\n', '\t'];

/// One control character found in prose.
#[derive(Debug, PartialEq, Eq)]
struct ControlDefect {
    /// 1-based line number of the offending character.
    line: usize,
    /// 0-based byte offset of the offending character in the file.
    byte_offset: usize,
    /// Its code point.
    code_point: u32,
    /// The offending line, with control characters rendered as escapes.
    context: String,
}

/// Reports every control character in `body` except [`ALLOWED_CONTROLS`].
fn control_defects(body: &str) -> Vec<ControlDefect> {
    let mut out = Vec::new();
    let mut line = 1usize;
    let mut line_start = 0usize;
    for (offset, ch) in body.char_indices() {
        if ch == '\n' {
            line += 1;
            line_start = offset + 1;
            continue;
        }
        if !ch.is_control() || ALLOWED_CONTROLS.contains(&ch) {
            continue;
        }
        let line_end = body[line_start..]
            .find('\n')
            .map(|n| line_start + n)
            .unwrap_or(body.len());
        out.push(ControlDefect {
            line,
            byte_offset: offset,
            code_point: ch as u32,
            context: body[line_start..line_end].escape_debug().to_string(),
        });
    }
    out
}

/// Every markdown-adjacent file the prose contract covers, recursively under `book-src/` and
/// `notes/`, plus `README.md` and `AGENTS.md`, as repository-relative POSIX paths.
fn corpus_files() -> Vec<String> {
    fn walk(root: &Path, dir: &Path, out: &mut Vec<String>) {
        let mut entries: Vec<PathBuf> = std::fs::read_dir(dir)
            .unwrap_or_else(|_| panic!("read directory {}", dir.display()))
            .map(|entry| entry.expect("dir entry").path())
            .collect();
        entries.sort();
        for path in entries {
            if path.is_dir() {
                walk(root, &path, out);
            } else if let Ok(rel) = path.strip_prefix(root) {
                out.push(rel.to_string_lossy().replace('\\', "/"));
            }
        }
    }

    let root = workspace_root();
    let mut out = Vec::new();
    for dir in ["book-src", "notes"] {
        walk(&root, &root.join(dir), &mut out);
    }
    for file in ["README.md", "AGENTS.md"] {
        if root.join(file).is_file() {
            out.push(file.to_string());
        }
    }
    out.sort();
    out
}

#[test]
fn prose_corpus_carries_no_control_characters() {
    let root = workspace_root();
    let files = corpus_files();
    assert!(
        files.len() >= 100,
        "the prose corpus should cover book-src/**, notes/**, README.md and AGENTS.md; found {} files",
        files.len()
    );

    let mut failures = Vec::new();
    for rel in &files {
        let bytes = std::fs::read(root.join(rel)).expect("read corpus file");
        let Ok(text) = String::from_utf8(bytes) else {
            failures.push(format!("{rel}: not valid UTF-8"));
            continue;
        };
        for defect in control_defects(&text) {
            failures.push(format!(
                "{rel}:{}: byte offset {} = 0x{:02x}: control character U+{:04X}\n    line: {}",
                defect.line,
                defect.byte_offset,
                defect.byte_offset,
                defect.code_point,
                defect.context
            ));
        }
    }
    assert!(
        failures.is_empty(),
        "control characters in prose (allowlist: U+0009 TAB and U+000A LF only):\n{}",
        failures.join("\n")
    );
}

#[test]
fn the_control_character_guard_sees_an_embedded_bel() {
    // The exact historical defect: a BEL byte (0x07) between `\hat{` and `alpha}` in chapter 46,
    // which rendered `\hat{lpha}` and defeated every escape-based check.
    let broken = "text before\n  and $\\hat{\u{7}alpha}$ loses its backslash\n";
    let defects = control_defects(broken);
    assert_eq!(defects.len(), 1, "{defects:?}");
    assert_eq!(defects[0].line, 2);
    assert_eq!(defects[0].code_point, 0x07);
    assert_eq!(
        defects[0].byte_offset,
        broken.find('\u{7}').expect("BEL present")
    );
    assert!(
        defects[0].context.contains("\\u{7}"),
        "a failure must render the byte, got {:?}",
        defects[0].context
    );

    // DEL and a C1 control are refused too, and the two allowed controls pass.
    assert_eq!(control_defects("a\u{7f}b").len(), 1);
    assert_eq!(control_defects("a\u{85}b").len(), 1);
    assert!(control_defects("$\\hat{\\alpha}$\n\tthree digits: \u{3b1} \u{2014}\n").is_empty());
}

// ---------------------------------------------------------------------------------------------
// Guard 2 — Appendix B.8 export-inventory drift
// ---------------------------------------------------------------------------------------------

/// The kinds of declaration the counting rule admits as one inventory row.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PubKind {
    /// `pub struct`
    Struct,
    /// `pub enum`
    Enum,
    /// `pub trait`
    Trait,
    /// `pub fn`, including `pub const fn`
    Function,
    /// `pub const NAME:`
    Constant,
    /// `pub static NAME:`
    Static,
    /// `pub type NAME = …`
    TypeAlias,
}

impl PubKind {
    /// True for the alias category, which B.8 lists on its own `Aliases:` line.
    fn is_alias(self) -> bool {
        matches!(self, Self::TypeAlias)
    }
}

/// One B.8 row: one module, its claimed count, and the names it lists.
#[derive(Debug)]
struct InventoryRow {
    /// Module file name as B.8 writes it (`engine.rs`, `loaders.rs`).
    module: String,
    /// Count claimed in the section heading (`### B.8.1 `engine.rs` — 31 public items`).
    claimed_count: usize,
    /// Names listed under `Types:`, `Functions:`, `Constants:` and `Aliases:`.
    listed: Vec<String>,
    /// Names listed under `Aliases:` only — counted items, but with their own drift messages.
    aliases: Vec<String>,
}

/// Reads the backticked names out of one `…:` line of a B.8 section.
fn listed_names(line: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = line;
    while let Some(open) = rest.find('`') {
        let after = &rest[open + 1..];
        let Some(close) = after.find('`') else { break };
        let name = &after[..close];
        if !name.is_empty() {
            out.push(name.to_string());
        }
        rest = &after[close + 1..];
    }
    out
}

/// Reads the alias names out of an `Aliases:` line.
///
/// An alias line carries an explanation after the alias itself —
/// `` `SosaJudgmentGrade` (governance.rs:53) — an alias of `TelicGrade`, … `` — so only the leading
/// head before the em dash is harvested, and only tokens that are identifiers survive: the
/// explanation may name the aliased target (`TelicGrade`, `SplitMix64`), which is a crate type but
/// not an item of this module's inventory.
fn alias_names(line: &str) -> Vec<String> {
    let head = match line.split_once('—') {
        Some((head, _)) => head,
        None => line,
    };
    let head = match head.split_once(" - ") {
        Some((head, _)) => head,
        None => head,
    };
    listed_names(head)
        .into_iter()
        .filter(|name| !name.is_empty() && name.chars().all(|c| c.is_alphanumeric() || c == '_'))
        .collect()
}

/// Parses every `### B.8.N `file.rs` — M public items` section of Appendix B.
fn parse_inventory(appendix: &str) -> Vec<InventoryRow> {
    let mut rows: Vec<InventoryRow> = Vec::new();
    let mut in_section = false;
    for line in appendix.lines() {
        if let Some(rest) = line.strip_prefix("### B.8.") {
            let Some(after_number) = rest.split_once('`').map(|(_, tail)| tail) else {
                in_section = false;
                continue;
            };
            let Some((module, tail)) = after_number.split_once('`') else {
                in_section = false;
                continue;
            };
            let Some(after_dash) = tail.split_once('—').map(|(_, tail)| tail.trim()) else {
                in_section = false;
                continue;
            };
            let count: usize = after_dash
                .split_whitespace()
                .next()
                .and_then(|n| n.parse().ok())
                .unwrap_or(0);
            rows.push(InventoryRow {
                module: module.to_string(),
                claimed_count: count,
                listed: Vec::new(),
                aliases: Vec::new(),
            });
            in_section = true;
            continue;
        }
        if line.starts_with("## ") || line.starts_with("### ") {
            in_section = false;
            continue;
        }
        if !in_section {
            continue;
        }
        let Some(row) = rows.last_mut() else {
            continue;
        };
        if line.starts_with("Aliases: ") {
            let names = alias_names(line);
            row.listed.extend(names.iter().cloned());
            row.aliases.extend(names);
        } else if line.starts_with("Types: ")
            || line.starts_with("Functions: ")
            || line.starts_with("Constants: ")
        {
            row.listed.extend(listed_names(line));
        }
    }
    rows
}

/// Every live public item of one module file: name to kind and declaration line.
fn live_pub_items(source: &str) -> BTreeMap<String, (PubKind, usize)> {
    let body = match source.find("#[cfg(test)]") {
        Some(at) => &source[..at],
        None => source,
    };
    let is_ident = |c: char| c.is_alphanumeric() || c == '_';
    let mut out: BTreeMap<String, (PubKind, usize)> = BTreeMap::new();
    let mut search = 0usize;
    while let Some(found) = body[search..].find("pub") {
        let at = search + found;
        search = at + 3;
        let before_ok = body[..at].chars().next_back().is_none_or(|c| !is_ident(c));
        let after = &body[at + 3..];
        let after_ok = after.chars().next().is_none_or(|c| !is_ident(c));
        if !before_ok || !after_ok {
            continue;
        }
        // Skip `pub(crate)`-style visibility modifiers, then read up to four leading tokens.
        let rest = match after.strip_prefix('(') {
            Some(after_open) => match after_open.find(')') {
                Some(close) => &after_open[close + 1..],
                None => continue,
            },
            None => after,
        };
        let mut tokens: Vec<&str> = Vec::new();
        let mut cursor = rest;
        while tokens.len() < 4 {
            cursor = cursor.trim_start();
            let end = cursor.find(|c: char| !is_ident(c)).unwrap_or(cursor.len());
            if end == 0 {
                break;
            }
            tokens.push(&cursor[..end]);
            cursor = &cursor[end..];
            if tokens.len() == 1 && !cursor.starts_with(char::is_whitespace) {
                break;
            }
        }
        let kind = tokens.first().copied().unwrap_or_default();
        let (name, pub_kind) = match kind {
            "struct" => (tokens.get(1).copied(), Some(PubKind::Struct)),
            "enum" => (tokens.get(1).copied(), Some(PubKind::Enum)),
            "trait" => (tokens.get(1).copied(), Some(PubKind::Trait)),
            "fn" => (tokens.get(1).copied(), Some(PubKind::Function)),
            "static" => (tokens.get(1).copied(), Some(PubKind::Static)),
            "type" => (tokens.get(1).copied(), Some(PubKind::TypeAlias)),
            "const" => match tokens.get(1).copied() {
                Some("fn") => (tokens.get(2).copied(), Some(PubKind::Function)),
                Some(name) => (Some(name), Some(PubKind::Constant)),
                None => (None, None),
            },
            "async" | "unsafe" | "extern" => match tokens.get(1).copied() {
                Some("fn") => (tokens.get(2).copied(), Some(PubKind::Function)),
                Some("const") => match tokens.get(2).copied() {
                    Some("fn") => (tokens.get(3).copied(), Some(PubKind::Function)),
                    Some(name) => (Some(name), Some(PubKind::Constant)),
                    None => (None, None),
                },
                _ => (None, None),
            },
            // `pub use` and `pub mod` add no declaration of their own, so they never enter the
            // comparison; enum variants are not `pub` and cannot reach this branch.
            _ => (None, None),
        };
        if let (Some(name), Some(pub_kind)) = (name, pub_kind)
            && is_ident(name.chars().next().unwrap_or(' '))
        {
            let line = body[..at].matches('\n').count() + 1;
            out.entry(name.to_string()).or_insert((pub_kind, line));
        }
    }
    out
}

/// The two crate source directories as `(module file name, source path)`.
fn crate_modules() -> Vec<(String, PathBuf)> {
    let root = workspace_root();
    let mut out = Vec::new();
    for dir in ["crates/stochology/src", "crates/stochology-cli/src"] {
        let mut entries: Vec<PathBuf> = std::fs::read_dir(root.join(dir))
            .expect("crate src directory")
            .map(|entry| entry.expect("dir entry").path())
            .collect();
        entries.sort();
        for path in entries {
            if path.extension().and_then(|e| e.to_str()) == Some("rs") {
                let name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .expect("file name")
                    .to_string();
                out.push((name, path));
            }
        }
    }
    out
}

/// Counts the declarations and the type aliases of the whole scanned surface.
fn surface_totals(modules: &[(String, PathBuf)]) -> (usize, usize) {
    let mut declarations = 0usize;
    let mut aliases = 0usize;
    for (_, path) in modules {
        let source = std::fs::read_to_string(path).expect("read module source");
        for (kind, _) in live_pub_items(&source).values() {
            if kind.is_alias() {
                aliases += 1;
            } else {
                declarations += 1;
            }
        }
    }
    (declarations, aliases)
}

/// Compares parsed inventory rows against the crate sources in both directions.
/// Returns `(failures, documented_total, source_total)`.
fn inventory_drift(
    rows: &[InventoryRow],
    modules: &[(String, PathBuf)],
) -> (Vec<String>, usize, usize) {
    let mut failures = Vec::new();
    let mut documented_total = 0usize;
    let mut source_total = 0usize;

    for row in rows {
        documented_total += row.claimed_count;
        let Some((_, path)) = modules.iter().find(|(name, _)| *name == row.module) else {
            failures.push(format!(
                "{}: B.8 inventories a module that does not exist under crates/*/src",
                row.module
            ));
            continue;
        };
        let source = std::fs::read_to_string(path).expect("read module source");
        let actual = live_pub_items(&source);
        source_total += actual.len();

        if row.listed.len() != row.claimed_count {
            failures.push(format!(
                "{}: heading claims {} public items, the section lists {}",
                row.module,
                row.claimed_count,
                row.listed.len()
            ));
        }

        // Aliases first: B.8 keeps them on their own line, so name them by that category.
        for name in &row.aliases {
            match actual.get(name) {
                Some((kind, _)) if kind.is_alias() => {}
                Some((_, line)) => failures.push(format!(
                    "{}: `{name}` is listed under `Aliases:` but line {line} declares it as something other than a `pub type` alias",
                    row.module
                )),
                None => failures.push(format!(
                    "{}: documented alias with no declaration: `Aliases:` lists `{name}` but the module declares no `pub type {name}`",
                    row.module
                )),
            }
        }
        for (name, (kind, line)) in &actual {
            if kind.is_alias() && !row.aliases.contains(name) {
                failures.push(format!(
                    "{}: declared alias with no row: `pub type {name}` (line {line}) is missing from the section's `Aliases:` line",
                    row.module
                ));
            }
        }

        if row.claimed_count != actual.len() {
            let example = actual
                .iter()
                .find(|(name, _)| !row.listed.contains(name))
                .map(|(name, (kind, line))| (name.clone(), *line, Some(*kind)))
                .or_else(|| {
                    row.listed.iter().find_map(|name| {
                        (!actual.contains_key(name)).then(|| (name.clone(), 0usize, None))
                    })
                });
            let example_text = match example {
                Some((name, line, Some(kind))) => {
                    format!("`{name}` ({:?} at crates source line {line})", kind)
                }
                Some((name, _, None)) => {
                    format!("`{name}` (listed in B.8 but absent from the source)")
                }
                None => "no single symbol accounts for it".to_string(),
            };
            failures.push(format!(
                "{}: expected {} public items, found {} — example: {}",
                row.module,
                row.claimed_count,
                actual.len(),
                example_text
            ));
        }

        for (name, (kind, line)) in &actual {
            if kind.is_alias() || row.listed.contains(name) {
                continue;
            }
            failures.push(format!(
                "{}: missing from B.8: `{name}` ({kind:?}, line {line}) — the source declares it `pub` but the inventory omits it",
                row.module
            ));
        }
        for name in &row.listed {
            if row.aliases.contains(name) || actual.contains_key(name) {
                continue;
            }
            failures.push(format!(
                "{}: stale B.8 entry: `{name}` is listed but no `pub` declaration of that name survives in the module",
                row.module
            ));
        }
    }

    // No module with public surface may be absent from the inventory, so a new file cannot slip in.
    for (module, path) in modules {
        if rows.iter().any(|row| row.module == *module) {
            continue;
        }
        let source = std::fs::read_to_string(path).expect("read module source");
        let actual = live_pub_items(&source);
        if !actual.is_empty() {
            let (example, (kind, line)) = actual.iter().next().expect("non-empty");
            failures.push(format!(
                "{module}: no B.8 row for a module exporting {} public items — example: `{example}` ({kind:?}, line {line})",
                actual.len()
            ));
        }
    }

    (failures, documented_total, source_total)
}

/// The paragraph of B.8 that states the counting rule the guard pins.
const COUNTING_RULE_LEAD: &str = "**Counting rule (the contract the drift guard pins).**";

/// The categories the parser counts, exactly as the counting rule must name them.
const COUNTED_CATEGORIES: [&str; 9] = [
    "`pub struct`",
    "`pub enum`",
    "`pub trait`",
    "`pub fn`",
    "`pub const fn`",
    "impl … for …",
    "`pub const NAME:`",
    "`pub static NAME:`",
    "`pub type`",
];

/// The names the parser refuses, exactly as the counting rule must exclude them.
const EXCLUDED_NAMES: [&str; 5] = [
    "`pub use`",
    "`pub mod`",
    "enum variants",
    "`#[cfg(test)]`",
    "counts once",
];

/// Extracts the counting-rule paragraph, from its lead sentence to the next blank-line-then-heading.
fn counting_rule_paragraph(appendix: &str) -> Option<&str> {
    let start = appendix.find(COUNTING_RULE_LEAD)?;
    let rest = &appendix[start..];
    let end = rest.find("\n\n### ").unwrap_or(rest.len());
    Some(&rest[..end])
}

/// The bolded integers the counting-rule paragraph states, in order.
fn stated_totals(paragraph: &str) -> Vec<usize> {
    let mut out = Vec::new();
    let mut rest = paragraph;
    while let Some(open) = rest.find("**") {
        let after = &rest[open + 2..];
        let Some(close) = after.find("**") else { break };
        if let Ok(n) = after[..close].parse::<usize>() {
            out.push(n);
        }
        rest = &after[close + 2..];
    }
    out
}

#[test]
fn appendix_b8_export_inventory_matches_the_public_surface() {
    let root = workspace_root();
    let appendix_path = root.join("book-src/appendix-b-architecture-verification.md");
    let appendix = std::fs::read_to_string(&appendix_path).expect("read appendix B");
    let rows = parse_inventory(&appendix);
    assert!(
        rows.len() >= 11,
        "Appendix B.8 must inventory all eight library modules and the three CLI modules; parsed {} rows",
        rows.len()
    );

    let modules = crate_modules();
    let (failures, documented_total, source_total) = inventory_drift(&rows, &modules);
    // Guard against a vacuous comparison: the scan must reach the whole documented surface.
    assert!(
        documented_total >= 300 && source_total >= 300,
        "the inventory comparison is suspiciously small: {documented_total} documented vs {source_total} declared"
    );
    // Aliases are counted items now: if the parser silently stopped seeing them, this fails.
    let (declarations, aliases) = surface_totals(&modules);
    assert!(
        aliases >= 1,
        "the scan found no `pub type` aliases at all — the alias category was dropped"
    );
    assert_eq!(
        aliases,
        rows.iter().map(|row| row.aliases.len()).sum::<usize>(),
        "the `Aliases:` lines of B.8 and the `pub type` declarations of the crates disagree in number"
    );
    assert_eq!(
        documented_total,
        declarations + aliases,
        "Appendix B.8 documents {documented_total} public items; the crates declare {declarations} declarations plus {aliases} aliases"
    );
    assert_eq!(
        documented_total, source_total,
        "Appendix B.8 documents {documented_total} public items; the two crates declare {source_total}"
    );
    assert!(
        failures.is_empty(),
        "Appendix B.8 export inventory has drifted from the sources:\n{}",
        failures.join("\n")
    );
}

#[test]
fn the_prose_counting_rule_and_the_parser_cannot_diverge() {
    let root = workspace_root();
    let appendix =
        std::fs::read_to_string(root.join("book-src/appendix-b-architecture-verification.md"))
            .expect("read appendix B");
    let paragraph = counting_rule_paragraph(&appendix).unwrap_or_else(|| {
        panic!("Appendix B.8 no longer carries the paragraph led by {COUNTING_RULE_LEAD}")
    });

    // Every category the parser counts must be named by the rule, and every name it refuses must be
    // excluded by the rule: a prose edit that drops `pub type` from the counted categories, or a
    // parser edit that starts counting `pub use`, breaks this test instead of drifting silently.
    for category in COUNTED_CATEGORIES {
        assert!(
            paragraph.contains(category),
            "B.8's counting rule no longer names the counted category {category}; the guard would be \
             counting something the prose does not claim, or vice versa"
        );
    }
    for excluded in EXCLUDED_NAMES {
        assert!(
            paragraph.contains(excluded),
            "B.8's counting rule no longer states the exclusion `{excluded}`, which the parser \
             enforces"
        );
    }
    // Heading counts must include aliases, since the parser compares headings against alias-inclusive
    // live counts.
    assert!(
        paragraph.contains("aliases included"),
        "B.8's counting rule no longer states that `### B.8.N` headings are alias-inclusive"
    );

    // The prose states its totals in bold. They must be internally consistent and must equal what the
    // scan finds, so neither side can move alone.
    let stated = stated_totals(paragraph);
    assert_eq!(
        stated.len(),
        3,
        "B.8's counting rule should state total, declarations and aliases (e.g. **380** = **377** + \
         **3**); found {stated:?}"
    );
    let (total, declarations, aliases) = (stated[0], stated[1], stated[2]);
    assert_eq!(
        total,
        declarations + aliases,
        "B.8's counting rule states {total} items but {declarations} + {aliases} does not add up"
    );
    let modules = crate_modules();
    let (live_declarations, live_aliases) = surface_totals(&modules);
    assert_eq!(
        (declarations, aliases),
        (live_declarations, live_aliases),
        "B.8's counting rule states {declarations} declarations and {aliases} aliases; the crates \
         declare {live_declarations} and {live_aliases}"
    );
    assert_eq!(
        total,
        live_declarations + live_aliases,
        "B.8's counting rule states {total} public items; the crates declare {} plus {}",
        live_declarations,
        live_aliases
    );
}

#[test]
fn the_inventory_parser_reads_rows_counts_and_names() {
    // A section in B.8's own format, plus a following section that must not leak into it.
    let appendix = "## B.8 Complete Export Inventory of the Two Crates\n\n\
        ### B.8.1 `engine.rs` — 4 public items\n\n\
        Types: `AgentDecision`, `StepOutcome`\n\n\
        Functions: `new`\n\n\
        Aliases: `Grade` (engine.rs:9) — an alias of `TelicGrade`, the exported moral-luck name.\n\n\
        ### B.8.2 `csv.rs` — 1 public items (CLI crate)\n\n\
        Types: `CsvTable`\n\n";
    let rows = parse_inventory(appendix);
    assert_eq!(rows.len(), 2, "{rows:?}");
    assert_eq!(rows[0].module, "engine.rs");
    assert_eq!(rows[0].claimed_count, 4);
    assert_eq!(
        rows[0].listed,
        vec![
            "AgentDecision".to_string(),
            "StepOutcome".to_string(),
            "new".to_string(),
            "Grade".to_string()
        ]
    );
    // The explanation after the em dash names the aliased target and a file:line; only the leading
    // identifier is an inventory name.
    assert_eq!(rows[0].aliases, vec!["Grade".to_string()]);
    assert_eq!(rows[1].module, "csv.rs");
    assert_eq!(rows[1].claimed_count, 1);
    assert_eq!(rows[1].listed, vec!["CsvTable".to_string()]);
    assert!(rows[1].aliases.is_empty());

    // And the item scanner on a synthetic module: inherent, generic, associated const, alias,
    // re-export, type alias, trait method and a test-only declaration.
    let source = "pub struct A;\npub enum B { X }\npub trait C {}\n\
        impl A {\n    pub fn new() -> Self { A }\n    pub const ALL: [u8; 1] = [0];\n}\n\
        impl Default for A {\n    fn default() -> Self { A }\n}\n\
        pub type Alias = u8;\n\
        #[cfg(test)]\nmod tests {\n    pub struct Hidden;\n}\n";
    let items = live_pub_items(source);
    let names: Vec<(&str, PubKind)> = items
        .iter()
        .map(|(name, (kind, _))| (name.as_str(), *kind))
        .collect();
    assert_eq!(
        names,
        vec![
            ("A", PubKind::Struct),
            ("ALL", PubKind::Constant),
            ("Alias", PubKind::TypeAlias),
            ("B", PubKind::Enum),
            ("C", PubKind::Trait),
            ("new", PubKind::Function)
        ]
    );
    assert!(
        !items.contains_key("Hidden"),
        "test-only items are excluded"
    );
    assert!(
        !items.contains_key("default"),
        "trait methods are not `pub`"
    );
}

#[test]
fn the_inventory_comparison_reports_a_drifted_row_in_both_directions() {
    let engine_only: Vec<(String, PathBuf)> = crate_modules()
        .into_iter()
        .filter(|(name, _)| name == "engine.rs")
        .collect();
    let engine_source = std::fs::read_to_string(&engine_only[0].1).expect("read engine.rs source");
    let real_row = InventoryRow {
        module: "engine.rs".to_string(),
        claimed_count: 31,
        listed: live_pub_items(&engine_source).keys().cloned().collect(),
        aliases: Vec::new(),
    };
    let (clean, documented, declared) =
        inventory_drift(std::slice::from_ref(&real_row), &engine_only);
    assert!(clean.is_empty(), "{clean:?}");
    assert_eq!(documented, declared);

    let mut listed = real_row.listed.clone();
    let dropped = listed.pop().expect("non-empty");
    listed.push("GhostItem".to_string());
    let drifted = InventoryRow {
        module: "engine.rs".to_string(),
        claimed_count: 31,
        listed,
        aliases: Vec::new(),
    };
    let (failures, _, _) = inventory_drift(std::slice::from_ref(&drifted), &engine_only);
    let joined = failures.join("\n");
    assert_eq!(failures.len(), 2, "{joined}");
    assert!(
        joined.contains(&format!("missing from B.8: `{dropped}`")),
        "{joined}"
    );
    assert!(joined.contains("stale B.8 entry: `GhostItem`"), "{joined}");

    // A wrong heading count is reported as well, even when the item sets are intact.
    let miscounted = InventoryRow {
        module: "engine.rs".to_string(),
        claimed_count: 30,
        listed: real_row.listed.clone(),
        aliases: Vec::new(),
    };
    let (count_failures, documented, declared) =
        inventory_drift(std::slice::from_ref(&miscounted), &engine_only);
    let joined = count_failures.join("\n");
    assert_eq!(documented, 30);
    assert_eq!(declared, 31);
    assert!(
        joined.contains("heading claims 30 public items, the section lists 31"),
        "{joined}"
    );
    assert!(
        joined.contains("expected 30 public items, found 31"),
        "{joined}"
    );
}

#[test]
fn the_inventory_comparison_rejects_a_stale_alias_row() {
    // grammar.rs owns exactly one `pub type` alias, `ResourceId`; the guard must notice when B.8
    // renames it and when a declared alias loses its `Aliases:` line.
    let grammar_only: Vec<(String, PathBuf)> = crate_modules()
        .into_iter()
        .filter(|(name, _)| name == "grammar.rs")
        .collect();
    let grammar_source =
        std::fs::read_to_string(&grammar_only[0].1).expect("read grammar.rs source");
    let actual = live_pub_items(&grammar_source);
    let declared_alias = actual
        .iter()
        .find(|(_, (kind, _))| *kind == PubKind::TypeAlias)
        .map(|(name, (_, line))| (name.clone(), *line))
        .expect("grammar.rs declares a pub type alias");
    let alias_count = actual
        .values()
        .filter(|(kind, _)| *kind == PubKind::TypeAlias)
        .count();
    let alias_free = InventoryRow {
        module: "grammar.rs".to_string(),
        claimed_count: actual.len(),
        listed: actual.keys().cloned().collect(),
        aliases: Vec::new(),
    };
    let (failures, _, _) = inventory_drift(std::slice::from_ref(&alias_free), &grammar_only);
    let joined = failures.join("\n");
    assert_eq!(failures.len(), alias_count, "{joined}");
    assert!(
        joined.contains(&format!(
            "declared alias with no row: `pub type {}` (line {})",
            declared_alias.0, declared_alias.1
        )),
        "{joined}"
    );

    // A ghost alias row: the name is documented, and no `pub type` of that name exists.
    let mut listed = actual.keys().cloned().collect::<Vec<String>>();
    listed.retain(|name| *name != declared_alias.0);
    listed.push("GhostAlias".to_string());
    let ghosted = InventoryRow {
        module: "grammar.rs".to_string(),
        claimed_count: actual.len(),
        listed,
        aliases: vec!["GhostAlias".to_string()],
    };
    let (ghost_failures, _, _) = inventory_drift(std::slice::from_ref(&ghosted), &grammar_only);
    let joined = ghost_failures.join("\n");
    assert!(
        joined.contains("documented alias with no declaration: `Aliases:` lists `GhostAlias`"),
        "{joined}"
    );
    assert!(
        joined.contains(&format!(
            "declared alias with no row: `pub type {}` (line {})",
            declared_alias.0, declared_alias.1
        )),
        "{joined}"
    );

    // The real row, untouched, is clean.
    let real_row = InventoryRow {
        module: "grammar.rs".to_string(),
        claimed_count: actual.len(),
        listed: actual.keys().cloned().collect(),
        aliases: vec![declared_alias.0.clone()],
    };
    let (clean, documented, declared) =
        inventory_drift(std::slice::from_ref(&real_row), &grammar_only);
    assert!(clean.is_empty(), "{clean:?}");
    assert_eq!(documented, declared);
}

// ---------------------------------------------------------------------------------------------
// Guard 3 — the one-way authority arrow (AGENTS.md §II, SWARM_REVIEW.md §I.3)
// ---------------------------------------------------------------------------------------------
//
// The amended invariant protects authority, not the absence of names. Rust source may carry the
// `KnowledgeOrder` <-> chapter bijection as model data and may name the chapter that exercises a
// symbol, in doc comments and error strings alike (`Run Chapter 4:`, `Chapter 2 Operator — …`,
// `Chapters 2–4`, `Chapter 1–9 laboratories`, `--chapter 8`, `41 + N`, `[formerly Ch. 4]`,
// `Chapters 51–85`). What it may never do is hand authority or a definition back to the prose:
// send the reader to the book, cite the book as the place where a claim is shown, or make a chapter
// the definition of a symbol. Those are the citations below.

/// Citations that defer authority or a definition to the prose.
///
/// Each entry is built with [`concat!`] so the guard's own source text never contains the assembled
/// needle: the guard scans `crates/**/*.rs`, its own file included, and must not flag itself.
const FORBIDDEN_CITATIONS: [&str; 11] = [
    concat!("see ", "chapter"),
    concat!("see ", "chapters"),
    concat!("per ", "chapter"),
    concat!("cf. ", "chapter"),
    concat!("cf ", "chapter"),
    concat!("according to ", "chapter"),
    concat!("refer to ", "chapter"),
    concat!("consult ", "chapter"),
    concat!("as shown in ", "chapter"),
    concat!("defined in ", "chapter"),
    concat!("for the ", "definition"),
];

/// One citation that violates the arrow.
#[derive(Debug, PartialEq, Eq)]
struct ArrowViolation {
    /// 1-based line number of the offending comment or string.
    line: usize,
    /// The phrase that fired.
    phrase: String,
    /// The offending line, trimmed, for the failure message.
    text: String,
}

/// Reports every authority-deferring citation in a Rust source.
///
/// The last entry of [`FORBIDDEN_CITATIONS`] — the one that names a definition — only fires on a
/// line that also names a chapter; a legend such as "the definition of `S_epi` lives here" is legal.
fn arrow_violations(body: &str) -> Vec<ArrowViolation> {
    // Assembled at runtime for the same reason the needles are: this file must not trip the guard.
    let chapter_word = concat!("chap", "ter");
    let mut out = Vec::new();
    for (idx, raw) in body.lines().enumerate() {
        let lower = raw.to_ascii_lowercase();
        let names_a_chapter = lower.contains(chapter_word);
        for phrase in FORBIDDEN_CITATIONS {
            if !lower.contains(phrase) {
                continue;
            }
            if phrase == FORBIDDEN_CITATIONS[FORBIDDEN_CITATIONS.len() - 1] && !names_a_chapter {
                continue;
            }
            out.push(ArrowViolation {
                line: idx + 1,
                phrase: phrase.to_string(),
                text: raw.trim().to_string(),
            });
        }
        // The `Chapter N <verb>` idiom, where the book is made the definition of the symbol: the
        // verb may sit anywhere in the following clause.
        if names_a_chapter {
            for at in lower.match_indices(chapter_word).map(|(at, _)| at) {
                let tail = &lower[at..];
                let window = &tail[..tail.len().min(80)];
                for verb in ["defines", "gives the definition", "states the definition"] {
                    if window.contains(verb) {
                        out.push(ArrowViolation {
                            line: idx + 1,
                            phrase: format!("{chapter_word} … {verb}"),
                            text: raw.trim().to_string(),
                        });
                        break;
                    }
                }
            }
        }
    }
    out
}

/// Every Rust source of the workspace, repository-relative: both crates, sources and tests.
fn rust_sources() -> Vec<String> {
    fn walk(root: &Path, dir: &Path, out: &mut Vec<String>) {
        let mut entries: Vec<PathBuf> = std::fs::read_dir(dir)
            .unwrap_or_else(|_| panic!("read directory {}", dir.display()))
            .map(|entry| entry.expect("dir entry").path())
            .collect();
        entries.sort();
        for path in entries {
            if path.is_dir() {
                walk(root, &path, out);
            } else if path.extension().and_then(|e| e.to_str()) == Some("rs")
                && let Ok(rel) = path.strip_prefix(root)
            {
                out.push(rel.to_string_lossy().replace('\\', "/"));
            }
        }
    }

    let root = workspace_root();
    let mut out = Vec::new();
    walk(&root, &root.join("crates"), &mut out);
    out.sort();
    out
}

#[test]
fn rust_source_never_defers_authority_to_the_textbook() {
    let root = workspace_root();
    let files = rust_sources();
    assert!(
        files.len() >= 20,
        "expected both crates, sources and tests; found {} Rust files",
        files.len()
    );

    let mut failures = Vec::new();
    let mut mentions = 0usize;
    for rel in &files {
        let body = std::fs::read_to_string(root.join(rel)).expect("read Rust source");
        // The legal population must actually be present, otherwise this guard proves nothing.
        let lower = body.to_ascii_lowercase();
        mentions += lower.matches(concat!("chap", "ter")).count();
        for violation in arrow_violations(&body) {
            failures.push(format!(
                "{rel}:{}: `{}` — {}",
                violation.line, violation.phrase, violation.text
            ));
        }
    }
    assert!(
        mentions >= 50,
        "only {mentions} chapter mentions found across the crates; the scan is not reaching the \
         legal naming that the amended invariant allows"
    );
    assert!(
        failures.is_empty(),
        "the one-way authority arrow is broken — source may name the chapter that exercises a symbol \
         but must not defer authority or a definition to it:\n{}",
        failures.join("\n")
    );
}

#[test]
fn the_arrow_guard_separates_deferral_from_lawful_naming() {
    // Assembled at runtime: this file is inside the scanned corpus and must not trip the guard.
    let word = concat!("Chap", "ter");
    let noun = concat!("def", "inition");
    let verb = concat!("def", "ines");

    // Every line below is lawful under AGENTS.md §II (iii): naming the chapter that exercises a
    // symbol, CLI laboratory numbers, the KnowledgeOrder bijection, and the bracketed edition history.
    let lawful = [
        format!("/// Run {word} 4: Filter I · Axis 1 — the archive paradox."),
        format!("    /// **{word} 2 Operator — Framing, Cost-Loss Threshold and Type III.**"),
        format!(
            "    /// **Safeguard 1 — Epistemic Margin (`S_epi`, {word}s 2–4):** enables bounds."
        ),
        format!("//! Progressive {word} 1–9 executable laboratories over the environment."),
        format!(
            "        Some(other) => Err(format!(\"{word} must be between 1 and 9 (got {{other}})\")),"
        ),
        format!(
            "        return Err(\"{word} 9 process tracing or reflexive brake audit failed\");"
        ),
        format!("    /// Stratum I (`L01`–`L05`, {word}s `51`–`55`): basal regimes."),
        format!("//! **{word} 5 [formerly Ch. 4]** — scale invariance and heavy tails."),
        format!("    /// Run with `--{word} 8` or with the laboratory number `41 + N`."),
        format!(
            "    /// The 2x2x2 ablation is demonstrated in {word} 9; this module implements it."
        ),
        "    /// `KnowledgeOrder::chapter_number()` returns `50 + layer_index`.".to_string(),
    ];
    for line in &lawful {
        assert!(
            arrow_violations(line).is_empty(),
            "lawful naming was flagged: {line} -> {:?}",
            arrow_violations(line)
        );
    }

    // Every line below hands authority or the definition of a symbol to the prose.
    let unlawful = [
        (
            format!("//! see {word} 44 for the sharp ATE bounds."),
            concat!("see ", "chapter"),
        ),
        (
            format!("    /// per {word} 4, the archive bias is bounded."),
            concat!("per ", "chapter"),
        ),
        (
            format!("//! as shown in {word} 45, `n_eff` collapses."),
            concat!("as shown in ", "chapter"),
        ),
        (
            format!("    /// the Hill index is defined in {word} 5."),
            concat!("defined in ", "chapter"),
        ),
        (
            format!("//! cf. {word} 7 for the performativity argument."),
            concat!("cf. ", "chapter"),
        ),
        (
            format!("    /// the ATE bounds are tabulated for the {noun} in {word} 44."),
            concat!("for the ", "definition"),
        ),
        (
            format!("    /// {word} 44 {verb} the sharp bounds; do not restate them."),
            concat!("chap", "ter", " … ", "def", "ines"),
        ),
    ];
    for (line, expected) in &unlawful {
        let violations = arrow_violations(line);
        assert_eq!(
            violations.len(),
            1,
            "expected exactly one violation in {line}: {violations:?}"
        );
        assert_eq!(
            violations[0].phrase, *expected,
            "expected phrase `{expected}` for {line}, got `{}`",
            violations[0].phrase
        );
        assert_eq!(violations[0].line, 1);
    }
}
