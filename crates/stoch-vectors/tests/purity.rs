//! The harness is integer-only and dependency-free but for the core.
//!
//! Proven by scanning its own sources and manifest, not by inspection: a later
//! edit that reaches for a float or a second crate fails here first.

use std::fs;
use std::path::{Path, PathBuf};

fn sources(directory: &Path, out: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(directory).expect("readable") {
        let path = entry.expect("entry").path();
        if path.is_dir() {
            sources(&path, out);
        } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
            out.push(path);
        }
    }
}

fn crate_sources() -> Vec<PathBuf> {
    let mut files = Vec::new();
    sources(&Path::new(env!("CARGO_MANIFEST_DIR")).join("src"), &mut files);
    sources(&Path::new(env!("CARGO_MANIFEST_DIR")).join("tests"), &mut files);
    files
}

#[test]
fn the_harness_is_integer_only() {
    // Assembled at run time so the scan does not find its own needles.
    let forbidden = [
        format!("f{}", 32),
        format!("f{}", 64),
        format!("as f{}", 64),
        format!("as f{}", 32),
        format!("un{}", "safe"),
    ];
    let mut scanned = 0;
    for path in crate_sources() {
        let text = fs::read_to_string(&path).expect("readable");
        for needle in &forbidden {
            assert!(
                !text.contains(needle.as_str()),
                "{}: contains {needle:?}",
                path.display()
            );
        }
        scanned += 1;
    }
    assert!(scanned >= 6, "the scan saw {scanned} files");
}

#[test]
fn the_harness_has_one_dependency() {
    let manifest = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml"),
    )
    .expect("the manifest is readable");
    let dependencies: Vec<&str> = manifest
        .lines()
        .skip_while(|line| line.trim() != "[dependencies]")
        .skip(1)
        .take_while(|line| !line.trim_start().starts_with('['))
        .filter(|line| !line.trim().is_empty())
        .collect();
    assert_eq!(dependencies.len(), 1, "{dependencies:?}");
    assert!(dependencies[0].contains("stoch-core"), "{dependencies:?}");
    assert!(dependencies[0].contains("path"), "a path dependency");
    for absent in ["serde", "wasm-bindgen", "rand", "clap"] {
        assert!(
            !manifest.contains(absent),
            "the manifest mentions {absent}"
        );
    }
    assert!(
        !Path::new(env!("CARGO_MANIFEST_DIR")).join("build.rs").exists(),
        "no build script"
    );
}
