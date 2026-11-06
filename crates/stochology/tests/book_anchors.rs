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
//!
//! Two further guarantees are enforced here because nothing else in the pipeline can:
//!
//! - a `file.rs::Type::method` anchor must resolve to a method declared **inside that type's own
//!   `impl` block**, so a correct name attached to the wrong owner is a red test; the member may be
//!   generic (`fn member<F>(...)`) as well as plain (`fn member(...)`);
//! - every markdown link `](target)` in the chapters, the notes and `README.md` must resolve to an
//!   existing repository file, must not contain whitespace (a destination split across a newline
//!   silently stops rendering as a link), and must not carry a `#Lnn-Lmm` window that is absent
//!   from the target file or inverted.

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

/// Splits a Rust source into top-level item segments. A segment starts on a non-indented line that
/// opens an item (`impl`, `pub`, `mod`, `fn`, `use`, `const`, `static`, `#`), so an indented member
/// declaration can be attributed to the single item that actually contains it.
fn item_segments(source: &str) -> Vec<&str> {
    let mut starts: Vec<usize> = Vec::new();
    let mut offset = 0usize;
    for line in source.split_inclusive('\n') {
        let opens_item = line.starts_with("impl ")
            || line.starts_with("impl<")
            || line.starts_with("pub ")
            || line.starts_with("mod ")
            || line.starts_with("fn ")
            || line.starts_with("use ")
            || line.starts_with("const ")
            || line.starts_with("static ")
            || line.starts_with('#');
        if opens_item {
            starts.push(offset);
        }
        offset += line.len();
    }
    let mut out = Vec::with_capacity(starts.len());
    for (idx, start) in starts.iter().enumerate() {
        let end = starts.get(idx + 1).copied().unwrap_or(source.len());
        out.push(&source[*start..end]);
    }
    out
}

/// The type an `impl` header line implements, reduced to its bare name
/// (`impl<'a> crate::x::Row<'a>`, `impl Default for Spec`, `impl Foo`).
fn impl_type_name(header: &str) -> Option<&str> {
    let rest = header.strip_prefix("impl")?;
    let rest = match rest.strip_prefix('<') {
        Some(after_open) => &after_open[after_open.find('>')? + 1..],
        None => rest,
    };
    let rest = match rest.rsplit_once(" for ") {
        Some((_, after_for)) => after_for,
        None => rest,
    };
    let rest = rest.split('{').next().unwrap_or(rest);
    let rest = rest.split("where").next().unwrap_or(rest).trim();
    if rest.is_empty() {
        return None;
    }
    let rest = if rest.starts_with('<') {
        rest
    } else {
        match rest.find('<') {
            Some(at) => &rest[..at],
            None => rest,
        }
    };
    let name = rest.rsplit("::").next().unwrap_or(rest).trim();
    (!name.is_empty()).then_some(name)
}

/// True if `member` is declared as a function inside an `impl` block of `owner`, in either the plain
/// `fn member(` form or the generic `fn member<...>(` form.
fn member_is_owned_by(source: &str, owner: &str, member: &str) -> bool {
    let plain = format!("fn {member}(");
    let generic = format!("fn {member}<");
    item_segments(source).iter().any(|segment| {
        let header = segment.lines().next().unwrap_or_default();
        impl_type_name(header) == Some(owner)
            && (segment.contains(plain.as_str()) || segment.contains(generic.as_str()))
    })
}

/// Resolves one `file.rs::Symbol` / `file.rs::Type::member` anchor against the crate sources.
fn anchor_resolves(sources: &[(String, String)], anchor: &str) -> bool {
    let mut parts = anchor.split("::");
    let file = parts.next().unwrap_or_default();
    let path: Vec<&str> = parts.collect();
    let Some((_, source)) = sources.iter().find(|(name, _)| name == file) else {
        return false;
    };
    match path.as_slice() {
        [symbol] => symbol_is_declared(source, symbol),
        [owner, member] => {
            symbol_is_declared(source, owner) && member_is_owned_by(source, owner, member)
        }
        _ => false,
    }
}

/// 1-based line number of a byte offset in `body`.
fn line_of(body: &str, offset: usize) -> usize {
    body[..offset].matches('\n').count() + 1
}

/// Blanks out fenced code blocks (keeping every line break) so a sample of markdown cannot be
/// mistaken for a link.
fn without_fenced_code(body: &str) -> String {
    let mut out = String::with_capacity(body.len());
    let mut in_fence = false;
    for line in body.split_inclusive('\n') {
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") {
            in_fence = !in_fence;
        }
        if in_fence || trimmed.starts_with("```") {
            out.extend(line.chars().map(|c| if c == '\n' { '\n' } else { ' ' }));
        } else {
            out.push_str(line);
        }
    }
    out
}

/// Decodes `%XX` escapes so `notes/The%20Grammar%20of%20Aim.md` resolves to the real file name.
fn percent_decode(text: &str) -> String {
    let bytes = text.as_bytes();
    let mut out: Vec<u8> = Vec::with_capacity(bytes.len());
    let mut i = 0usize;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            let hex = &text[i + 1..i + 3];
            if let Ok(byte) = u8::from_str_radix(hex, 16) {
                out.push(byte);
                i += 3;
                continue;
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

/// One broken markdown link.
#[derive(Debug, PartialEq, Eq)]
struct LinkDefect {
    /// 1-based line of the `](` that opens the link.
    line: usize,
    /// What is wrong with it.
    kind: LinkDefectKind,
    /// The offending destination (or the offending window, with the target's line count).
    detail: String,
}

/// The breakage modes the link check refuses.
#[derive(Debug, PartialEq, Eq)]
enum LinkDefectKind {
    /// The destination carries whitespace, a newline, or no closing `)` on its own line — it never
    /// renders as a link.
    Unrenderable,
    /// The destination resolves to no existing file in the repository.
    Missing,
    /// The `#Lnn-Lmm` window names a line the target file does not have.
    LineWindowOutOfRange,
    /// The `#Lnn-Lmm` window is inverted (`Lmm` before `Lnn`).
    LineWindowInverted,
}

/// Scans one markdown file for broken links. `rel_path` is repository-relative.
fn link_defects(rel_path: &str, body: &str, root: &Path) -> Vec<LinkDefect> {
    let text = without_fenced_code(body);
    let base = Path::new(rel_path)
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_default();
    let mut out = Vec::new();
    let mut search = 0usize;
    while let Some(found) = text[search..].find("](") {
        let open = search + found;
        let dest_start = open + 2;
        let line = line_of(&text, open);
        let Some(close_rel) = text[dest_start..].find(')') else {
            out.push(LinkDefect {
                line,
                kind: LinkDefectKind::Unrenderable,
                detail: text[dest_start..].to_string(),
            });
            break;
        };
        let dest = &text[dest_start..dest_start + close_rel];
        if dest.is_empty() || dest.starts_with("http://") || dest.starts_with("https://") {
            search = dest_start + close_rel + 1;
            continue;
        }
        if dest.starts_with("mailto:") || dest.starts_with("ftp:") {
            search = dest_start + close_rel + 1;
            continue;
        }
        if dest.chars().any(char::is_whitespace) {
            out.push(LinkDefect {
                line,
                kind: LinkDefectKind::Unrenderable,
                detail: dest.to_string(),
            });
            search = dest_start + close_rel + 1;
            continue;
        }
        search = dest_start + close_rel + 1;
        let (raw_target, fragment) = match dest.split_once('#') {
            Some((target, fragment)) => (target, fragment),
            None => (dest, ""),
        };
        if raw_target.is_empty() {
            continue;
        }
        let target = percent_decode(raw_target);
        let candidates = [root.join(&base).join(&target), root.join(&target)];
        let Some(resolved) = candidates.iter().find(|p| p.exists()) else {
            out.push(LinkDefect {
                line,
                kind: LinkDefectKind::Missing,
                detail: dest.to_string(),
            });
            continue;
        };
        // A window is `Lnn` or `Lnn-Lmm`; the second bound may repeat the `L` prefix or omit it.
        let Some(window) = fragment.strip_prefix('L') else {
            continue;
        };
        let mut numbers = window.split('-');
        let (Some(first), second, third) = (numbers.next(), numbers.next(), numbers.next()) else {
            continue;
        };
        if third.is_some() {
            continue;
        }
        let parse = |bound: &str| {
            bound
                .strip_prefix('L')
                .unwrap_or(bound)
                .parse::<usize>()
                .ok()
        };
        let Some(start) = parse(first) else { continue };
        let end = match second {
            Some(bound) => match parse(bound) {
                Some(end) => end,
                None => continue,
            },
            None => start,
        };
        if start > end {
            out.push(LinkDefect {
                line,
                kind: LinkDefectKind::LineWindowInverted,
                detail: format!("{dest} ({start} > {end})"),
            });
            continue;
        }
        let target_lines = std::fs::read_to_string(resolved)
            .map(|text| text.lines().count())
            .unwrap_or_default();
        if end > target_lines {
            out.push(LinkDefect {
                line,
                kind: LinkDefectKind::LineWindowOutOfRange,
                detail: format!("{dest} (target has {target_lines} lines)"),
            });
        }
    }
    out
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
            if !anchor_resolves(&sources, &anchor) {
                failures.push(format!(
                    "{chapter}: `{anchor}` — symbol path does not resolve to a declaration in the owning item"
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
fn a_generic_fn_anchor_qualifies_only_to_the_type_that_owns_it() {
    let sources = source_files();

    // `VitalMarginState::from_heavy_tails_and_asymmetry` is declared as
    // `pub fn from_heavy_tails_and_asymmetry<F>(` in `vital.rs`, so the type-qualified form must
    // resolve once the generic-parameter form is accepted. Chapters 58 and 70 currently cite the
    // type-less form because the old matcher required a literal `fn member(`.
    for anchor in [
        "vital.rs::from_heavy_tails_and_asymmetry",
        "vital.rs::VitalMarginState::from_heavy_tails_and_asymmetry",
        "multilayer.rs::MultilayerHomomorphism::evaluate",
        "governance.rs::TemporalVault::seal",
        "kernel.rs::SplitMix64::split",
    ] {
        assert!(
            anchor_resolves(&sources, anchor),
            "`{anchor}` must resolve against the crate sources"
        );
    }

    // A correct member name attached to a type that does not own that impl is still rejected.
    for anchor in [
        "vital.rs::EpistemicMarginState::from_heavy_tails_and_asymmetry",
        "vital.rs::VitalMarginState::from_inquiry_framing",
        "governance.rs::GovernanceState::from_heavy_tails_and_asymmetry",
        "vital.rs::VitalMarginState::from_heavy_tails_and_asymmetry_missing",
    ] {
        assert!(
            !anchor_resolves(&sources, anchor),
            "`{anchor}` must not resolve: the owning type does not declare that member"
        );
    }

    // Handoff evidence for the book cells: every chapter that still cites the type-less
    // `vital.rs::from_heavy_tails_and_asymmetry` resolves once the token is qualified to
    // `VitalMarginState`. The substitution is verified in memory, so no chapter is edited here, and
    // the test does not require the type-less form to exist — qualifying a chapter later cannot
    // turn it red.
    for (_, chapter, body) in chapters().into_iter().filter(|(_, _, b)| !is_framework(b)) {
        if !body.contains("`vital.rs::from_heavy_tails_and_asymmetry`") {
            continue;
        }
        let qualified = body.replace(
            "`vital.rs::from_heavy_tails_and_asymmetry`",
            "`vital.rs::VitalMarginState::from_heavy_tails_and_asymmetry`",
        );
        for anchor in anchors_in(&qualified) {
            assert!(
                anchor_resolves(&sources, &anchor),
                "{chapter}: `{anchor}` must resolve after qualification"
            );
        }
    }
}

/// Every markdown file the link contract covers: the 86 chapters and appendices, the five notes, `README.md`.
fn link_sources() -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for entry in std::fs::read_dir(book_dir()).expect("book-src directory") {
        let path = entry.expect("dir entry").path();
        if path.extension().and_then(|e| e.to_str()) == Some("md")
            && let Some(name) = path.file_name().and_then(|n| n.to_str())
        {
            out.push(format!("book-src/{name}"));
        }
    }
    for entry in std::fs::read_dir(workspace_root().join("notes")).expect("notes directory") {
        let path = entry.expect("dir entry").path();
        if path.extension().and_then(|e| e.to_str()) == Some("md")
            && let Some(name) = path.file_name().and_then(|n| n.to_str())
        {
            out.push(format!("notes/{name}"));
        }
    }
    out.push("README.md".to_string());
    out.sort();
    out
}

#[test]
fn every_markdown_link_resolves_with_an_intact_line_window() {
    let root = workspace_root();
    let files = link_sources();
    assert!(
        files.len() >= 92,
        "the link contract covers 86 chapters, the appendices, five notes and README.md; found {} files",
        files.len()
    );

    let mut failures = Vec::new();
    let mut links = 0usize;
    for rel in &files {
        let body = std::fs::read_to_string(root.join(rel)).expect("read markdown file");
        links += body.matches("](").count();
        for defect in link_defects(rel, &body, &root) {
            failures.push(format!(
                "{rel}:{}: {:?}: {}",
                defect.line, defect.kind, defect.detail
            ));
        }
    }
    assert!(
        links >= 500,
        "only {links} markdown links found; the extraction is probably broken"
    );
    assert!(
        failures.is_empty(),
        "broken markdown links (destination must resolve, must not span a newline, and must carry a real #Lnn-Lmm window):\n{}",
        failures.join("\n")
    );
}

/// The real-world instance of the first breakage mode: until Cell 6's pass, `book-src/69` line 91
/// carried `[Chapter 70 …](70-layer-acoustic-aesthetic-tragic-pedagogical-\norder.md)` — the
/// destination spanned a newline, so the chapter link never rendered and every test stayed green.
/// The checker must reject exactly that text, accept the repaired single-line form, and the file on
/// disk must already be repaired.
#[test]
fn the_newline_split_destination_from_chapter_69_is_rejected() {
    const CHAPTER: &str = "book-src/69-layer-metascience-scientometrics-replication-order.md";
    let root = workspace_root();
    let broken = "Layer 19 propagates its margins into\n\
        [Chapter 70 — Layer 20: Acoustic, Aesthetic, Tragic & Pedagogical Formative Order](70-layer-acoustic-aesthetic-tragic-pedagogical-\n\
        order.md) (`KnowledgeOrder::L20AcousticAestheticTragicPedagogical`).\n";
    let defects = link_defects(CHAPTER, broken, &root);
    assert_eq!(defects.len(), 1, "{defects:?}");
    assert_eq!(defects[0].line, 2);
    assert_eq!(defects[0].kind, LinkDefectKind::Unrenderable);
    assert_eq!(
        defects[0].detail, "70-layer-acoustic-aesthetic-tragic-pedagogical-\norder.md",
        "the checker must report the whole split destination, newline included"
    );

    let repaired = broken.replace("pedagogical-\norder.md", "pedagogical-order.md");
    assert_ne!(
        repaired, broken,
        "the fixture must actually contain the split"
    );
    assert!(
        link_defects(CHAPTER, &repaired, &root).is_empty(),
        "the repaired single-line destination must be accepted"
    );

    let on_disk = std::fs::read_to_string(root.join(CHAPTER)).expect("read chapter 69");
    assert!(
        link_defects(CHAPTER, &on_disk, &root).is_empty(),
        "chapter 69 on disk must carry no split destination"
    );
}

#[test]
fn the_link_checker_rejects_every_breakage_mode() {
    let root = workspace_root();

    // The newline-split destination has its own named case:
    // `the_newline_split_destination_from_chapter_69_is_rejected`.

    // A destination with a raw space is not a link either.
    let spaced = "see [Layer 20](70-layer order.md)\n";
    assert_eq!(
        link_defects("book-src/fixture.md", spaced, &root),
        vec![LinkDefect {
            line: 1,
            kind: LinkDefectKind::Unrenderable,
            detail: "70-layer order.md".to_string(),
        }]
    );

    // A destination that resolves to no file in the repository.
    let missing = "text [ghost](99-ghost-chapter.md) text\n";
    assert_eq!(
        link_defects("book-src/fixture.md", missing, &root),
        vec![LinkDefect {
            line: 1,
            kind: LinkDefectKind::Missing,
            detail: "99-ghost-chapter.md".to_string(),
        }]
    );

    // A window beyond the end of the target file.
    let out_of_range = "x [`GuardSite`](crates/stochology/src/grammar.rs#L1-L999999) y\n\
         [`GuardSite`](crates/stochology/src/grammar.rs#L10-L20)\n";
    let range_defects = link_defects("book-src/fixture.md", out_of_range, &root);
    assert_eq!(range_defects.len(), 1, "{range_defects:?}");
    assert_eq!(range_defects[0].line, 1);
    assert_eq!(range_defects[0].kind, LinkDefectKind::LineWindowOutOfRange);
    assert!(
        range_defects[0].detail.ends_with("(target has 1 lines)")
            || range_defects[0].detail.contains("grammar.rs#L1-L999999")
    );

    // An inverted window.
    let inverted = "x [`GuardSite`](crates/stochology/src/grammar.rs#L50-L10)\n";
    let inverted_defects = link_defects("book-src/fixture.md", inverted, &root);
    assert_eq!(inverted_defects.len(), 1);
    assert_eq!(inverted_defects[0].line, 1);
    assert_eq!(inverted_defects[0].kind, LinkDefectKind::LineWindowInverted);

    // A clean link (escaped spaces, repository-relative target, real window) is not a defect.
    let clean = "[notes](../notes/The%20Grammar%20of%20Aim.md) and [`GuardSite`](crates/stochology/src/grammar.rs#L10-L20) and [external](https://example.org/a_b)\n";
    assert!(link_defects("book-src/fixture.md", clean, &root).is_empty());

    // Fenced code samples are not links.
    let fenced = "```text\nsplit [x](a-\nb.md)\n```\n";
    assert!(link_defects("book-src/fixture.md", fenced, &root).is_empty());
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
