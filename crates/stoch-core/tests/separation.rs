//! Separation: the three functions are three types, and the type system is what keeps
//! them apart.
//!
//! The compile-time half of this claim lives in the `compile_fail` doctests on the
//! crate root and on [`stoch_core::Diagnosis`]: a diagnosis cannot be passed to `step`,
//! cannot be built by a struct literal, and cannot be read field by field. This file
//! is the runtime half — it scans the production source for the constructions and the
//! cross-references that would break the separation, and it checks the two algebraic
//! properties the predicate must have: determinism, and independence from the
//! contingency record.
//!
//! The scans read the source as text because the properties are negative: there is no
//! value to assert on when the claim is "this construction exists once" or "this type
//! is never named here". A text scan over the production half of the tree is the
//! cheapest faithful form of that assertion, and it fails loudly when the layout it
//! assumes is no longer there.

use std::fs;
use std::path::PathBuf;

use stoch_core::{
    accept, Assessment, AxisReading, AxialReadings, Bounds, Budget, Capacity, ContingencyTape,
    ControlTape, Declared, Guards, Instrument, Params, Score, Sentence, SevenReadings, Site,
    Thresholds, Transcript, Verdict,
};

fn manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn source(name: &str) -> String {
    let path = manifest_dir().join("src").join(name);
    fs::read_to_string(&path).unwrap_or_else(|error| panic!("{}: {}", path.display(), error))
}

/// The production half of a file: everything before its first test module.
///
/// Cutting at the first `#[cfg(test)]` alone would be wrong: the crate root carries
/// `#[cfg(test)] extern crate std;` in the middle of its imports, and cutting there
/// would leave the scan reading an empty file and passing by vacuity. A test module is
/// the attribute immediately followed by `mod`, and every one in this crate is last.
fn production(text: &str) -> &str {
    let mut offset = 0usize;
    let mut attribute: Option<usize> = None;
    for line in text.split_inclusive('\n') {
        let trimmed = line.trim();
        if attribute.is_some() && trimmed.starts_with("mod ") {
            return &text[..attribute.unwrap_or(0)];
        }
        attribute = if trimmed == "#[cfg(test)]" { Some(offset) } else { None };
        offset += line.len();
    }
    text
}

/// The code of a file with comments removed, one line at a time.
///
/// The doc comments of this crate argue *about* the types they separate — they name
/// the type a function refuses — so a scan over raw text would flag the explanation.
/// Cutting each line at its first `//` keeps the argument and drops the mention.
fn code(text: &str) -> String {
    text.lines()
        .map(|line| line.split("//").next().unwrap_or(""))
        .collect::<Vec<_>>()
        .join("\n")
}

fn source_files() -> Vec<(String, String)> {
    let mut files = Vec::new();
    for entry in fs::read_dir(manifest_dir().join("src")).expect("src/ is readable") {
        let path = entry.expect("an entry").path();
        if path.extension().map(|e| e == "rs").unwrap_or(false) {
            let name = path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default();
            let text = fs::read_to_string(&path).expect("UTF-8");
            files.push((name, text));
        }
    }
    files.sort_by(|a, b| a.0.cmp(&b.0));
    assert!(files.len() >= 5, "the crate has at least five sources");
    files
}

fn count(haystack: &str, needle: &str) -> usize {
    haystack.matches(needle).count()
}

#[test]
fn no_action_is_constructed_in_exactly_one_site() {
    let mut sites = Vec::new();
    for (name, text) in source_files() {
        let body = code(production(&text));
        for (index, line) in body.lines().enumerate() {
            let trimmed = line.trim();
            let Some(at) = trimmed.find("Regime::NoAction") else {
                continue;
            };
            // A pattern carries the variant *before* its arrow; a construction
            // carries it after it. `(Less, Less) => Ok(Regime::NoAction)` builds the
            // variant, `Regime::NoAction => ActionSet::None` only reads it.
            if trimmed.find("=>").map(|arrow| at < arrow).unwrap_or(false) {
                continue;
            }
            sites.push(format!("{}:{}: {}", name, index + 1, trimmed));
        }
    }
    assert_eq!(
        sites.len(),
        1,
        "the variant must be constructed once, found: {:#?}",
        sites
    );
    assert!(
        sites[0].starts_with("step.rs:") && sites[0].contains("Ok(Regime::NoAction)"),
        "the single construction is the operator's `U < C` answer, found: {}",
        sites[0]
    );
}

#[test]
fn none_is_produced_by_exactly_one_variant() {
    let mut arms = Vec::new();
    for (name, text) in source_files() {
        let body = code(production(&text));
        for (index, line) in body.lines().enumerate() {
            let trimmed = line.trim();
            let Some(at) = trimmed.find("ActionSet::None") else {
                continue;
            };
            // Built after the arrow, or assigned outright — never before it.
            if trimmed.find("=>").map(|arrow| at < arrow).unwrap_or(false) {
                continue;
            }
            {
                arms.push(format!("{}:{}: {}", name, index + 1, trimmed));
            }
        }
    }
    assert_eq!(
        arms.len(),
        1,
        "exactly one variant produces the empty action set, found: {:#?}",
        arms
    );
    assert!(
        arms[0].contains("Regime::NoAction => ActionSet::None"),
        "and it is `NoAction`, not a quarantine or a halt: {}",
        arms[0]
    );
}

#[test]
fn the_three_functions_do_not_name_each_others_types() {
    // `accept` is a predicate over a transcript: it never sees a state.
    let accept_code = code(production(&source("accept.rs")));
    assert_eq!(
        count(&accept_code, "State"),
        0,
        "accept must not take or mention a State"
    );

    // `step` emits a regime and nothing else: no verdict, no diagnosis.
    let step_code = code(production(&source("step.rs")));
    assert_eq!(count(&step_code, "Verdict"), 0, "step must not name a Verdict");
    assert_eq!(
        count(&step_code, "Diagnosis"),
        0,
        "step must not name a Diagnosis"
    );

    // `diagnose` reports; it has no path to an action at all.
    let diagnose_code = code(production(&source("diagnose.rs")));
    assert_eq!(
        count(&diagnose_code, "Regime"),
        0,
        "diagnose must not name a Regime"
    );
    assert_eq!(
        count(&diagnose_code, "ActionSet"),
        0,
        "diagnose must not name an ActionSet"
    );
    assert_eq!(
        count(&diagnose_code, "Verdict"),
        0,
        "diagnose must not name a Verdict either: the fold is not a verdict"
    );
}

#[test]
fn the_source_carries_no_binary_floating_point_and_no_unsafe() {
    // The two forbidden tokens are assembled here so that this file does not carry
    // them either: the scan is over `src/`, and a scan whose own witness contains the
    // forbidden spelling is a scan nobody can extend.
    let single = String::from_utf8(vec![b'f', b'3', b'2']).unwrap();
    let double = String::from_utf8(vec![b'f', b'6', b'4']).unwrap();
    let undefined = String::from_utf8(vec![b'u', b'n', b's', b'a', b'f', b'e']).unwrap();

    for (name, text) in source_files() {
        let body = code(production(&text));
        // A scan that silently reads nothing is not a scan: every production file is
        // longer than the test modules this crate appends to it.
        assert!(
            body.len() > 1_000,
            "{}: the scan reads {} characters of production code",
            name,
            body.len()
        );
        assert_eq!(count(&body, &single), 0, "{}: binary float in the core", name);
        assert_eq!(count(&body, &double), 0, "{}: binary float in the core", name);
        // The only legal spelling is the deny attribute, which carries the token as
        // part of its own name.
        assert_eq!(
            count(&body, &undefined),
            count(&body, &format!("{}_code", undefined)),
            "{}: a raw block in the core",
            name
        );
        assert_eq!(count(&body, "extern \"C\""), 0, "{}: no ABI here", name);
        assert_eq!(count(&body, "#[no_mangle]"), 0, "{}: no exported symbol", name);
        assert_eq!(count(&body, "wasm_bindgen"), 0, "{}: no binding layer", name);
        assert_eq!(count(&body, "serde"), 0, "{}: no serialisation crate", name);
    }
}

#[test]
fn the_crate_has_no_dependency_and_no_wire_target() {
    let cargo = fs::read_to_string(manifest_dir().join("Cargo.toml")).expect("Cargo.toml");
    assert_eq!(count(&cargo, "[dependencies]"), 0, "no runtime dependency");
    assert_eq!(count(&cargo, "[dev-dependencies]"), 0, "no test dependency");
    assert_eq!(count(&cargo, "[build-dependencies]"), 0, "no build dependency");
    assert_eq!(count(&cargo, "cdylib"), 0, "the wire target is a later phase");
    assert_eq!(count(&cargo, "rlib"), 1, "one static library target");
    assert!(
        !manifest_dir().join("build.rs").exists(),
        "no build script: the crate is auditable as it stands"
    );
    let lock = manifest_dir().join("Cargo.lock");
    if lock.exists() {
        let text = fs::read_to_string(lock).expect("Cargo.lock");
        let packages = count(&text, "[[package]]");
        assert_eq!(packages, 1, "the lock file holds the crate and nothing else");
    }
}

fn sentence(contingency: u64, axes: [i64; 3], site: Option<Site>) -> Sentence {
    Sentence::new(
        ControlTape::Attested,
        ContingencyTape::new(contingency),
        Bounds::new(120, 400, 1000, 0),
        Capacity::new(200, 0, 1000),
        Budget::new(1000, 10),
        AxialReadings::new(
            AxisReading::new(1, Score::Present(axes[0])),
            AxisReading::new(1, Score::Present(axes[1])),
            AxisReading::new(1, Score::Present(axes[2])),
        ),
        Declared::new(None, site, None),
        Assessment::new(100_000, 50_000),
    )
}

#[test]
fn the_predicate_is_deterministic_and_ignores_the_contingency_record() {
    let transcript = Transcript::seal(sentence(7, [500_000; 3], None)).expect("seals");
    let first = accept(&transcript);
    let second = accept(&transcript);
    assert_eq!(first, second, "the same transcript answers the same twice");
    assert_eq!(first, Ok(Verdict::Interior));

    // Appendix B.1: the verdict does not commute with the contingency record. Two
    // transcripts that differ only in that record must answer identically, which is
    // what makes the order of the predicate's stages observable at all.
    let left = Transcript::seal(sentence(1, [50_000, 900_000, 900_000], Some(Site::T1)))
        .expect("seals");
    let right = Transcript::seal(sentence(9, [50_000, 900_000, 900_000], Some(Site::T1)))
        .expect("seals");
    assert_eq!(left.contingency().seal(), 1);
    assert_eq!(right.contingency().seal(), 9);
    assert_eq!(accept(&left), accept(&right));
}

#[test]
fn the_fold_does_not_privilege_an_axis() {
    // The minimum is symmetric in its three arguments, so a permutation of the same
    // three readings leaves the fold where it was: at 100_000, whichever slot carries
    // it. What a permutation does move is the *axis* of the fold, and B.1 line 163
    // makes the declared site answer for that — the verdict names the site that
    // carries the fold, so a coherent pair names the slot the minimum landed on. That
    // is a property of the pair, not a privilege of the first axis; a predicate that
    // folded in sequence or short-circuited on one axis would fail below.
    let permutations = [
        ([100_000i64, 300_000, 500_000], Site::T1),
        ([300_000, 500_000, 100_000], Site::T7),
        ([500_000, 100_000, 300_000], Site::T4),
        ([100_000, 500_000, 300_000], Site::T1),
    ];
    for (axes, site) in permutations {
        let transcript = Transcript::seal(sentence(7, axes, Some(site))).expect("seals");
        assert_eq!(
            accept(&transcript),
            Ok(Verdict::Boundary {
                site,
                margin: 100_000
            }),
            "axes {:?} carry the fold on {:?}",
            axes,
            site
        );
    }

    // Holding the site fixed while the fold moves away from its axis is the coherence
    // clause: an epistemic fold declared on a vital site is not a boundary.
    let transcript = Transcript::seal(sentence(7, [100_000, 300_000, 500_000], Some(Site::T4)))
        .expect("seals");
    assert_ne!(
        accept(&transcript).map(|verdict| verdict.tag()),
        Ok(1),
        "a vital site cannot name an epistemic fold"
    );
}

#[test]
fn a_state_is_equal_to_itself_and_the_operator_is_a_function() {
    use stoch_core::{step, Capacity as C, State};

    let seal = || {
        State::seal(
            ControlTape::Attested,
            Bounds::new(400, 800, 1000, 0),
            C::new(200, 0, 1000),
            Guards::new(Instrument::Sealed(SevenReadings::new([500_000; 7]))),
            Params::new(10, 10, true, false, false, false, Thresholds::new(100_000, 50_000)),
        )
        .expect("seals")
    };
    let a = seal();
    let b = seal();
    assert_eq!(a, b, "sealing is a function of the record");
    assert_eq!(step(&a), step(&b), "the operator is deterministic");
    assert_eq!(step(&a).map(|r| r.tag()), Ok(0));
}

#[test]
fn no_public_constructor_of_the_diagnosis_appears_in_the_source() {
    // The opaque type has no `pub` constructor and no public field: the only `pub fn`
    // on it is the observation. This is the runtime echo of the `compile_fail` tests.
    let diagnose_code = code(production(&source("diagnose.rs")));
    assert_eq!(
        count(&diagnose_code, "pub fn observe"),
        1,
        "one way out of a diagnosis"
    );
    assert_eq!(
        count(&diagnose_code, "impl Diagnosis"),
        1,
        "one impl block on the diagnosis"
    );
    assert_eq!(
        count(&diagnose_code, "pub struct Diagnosis {"),
        1,
        "one declaration"
    );
    // Every field of the diagnosis is private: the struct block carries no `pub`.
    let start = diagnose_code.find("pub struct Diagnosis {").expect("declared");
    let block = &diagnose_code[start..];
    let block = &block[..block.find('}').expect("a block")];
    assert_eq!(count(block, "pub "), 1, "only the declaration itself is public");
    assert_eq!(count(block, ":"), 7, "and it has seven private fields");
    for forbidden in ["impl From<Diagnosis", "impl Deref", "derive(Debug)"] {
        assert_eq!(count(&diagnose_code, forbidden), 0, "{}", forbidden);
    }
}
