//! Purity checks for `stoch-crdt`: integer-only, zero dependencies, and no unsafe code blocks.

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
fn stoch_crdt_is_integer_only_and_unsafe_free() {
    let float_32 = format!("f{}", 32);
    let float_64 = format!("f{}", 64);
    let raw_kw = format!("un{}", "safe");
    let allow_attr = format!("allow({raw_kw}_code)");
    let deny_attr = format!("deny({raw_kw}_code)");

    let src_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let lib_text = fs::read_to_string(src_dir.join("lib.rs")).expect("lib.rs readable");
    assert!(
        lib_text.contains(&deny_attr),
        "src/lib.rs must carry #![deny(...)]"
    );
    assert!(
        lib_text.contains("#![no_std]"),
        "src/lib.rs must remain #![no_std]"
    );

    let mut scanned = 0;
    for path in crate_sources() {
        let text = fs::read_to_string(&path).expect("readable");
        for needle in [&float_32, &float_64] {
            assert!(
                !text.contains(needle.as_str()),
                "{}: contains {needle:?}",
                path.display()
            );
        }
        if path.starts_with(&src_dir) {
            assert!(
                !text.contains(&allow_attr),
                "{}: must not allow unsafe_code",
                path.display()
            );
            for (line_no, raw_line) in text.lines().enumerate() {
                let code = raw_line.split("//").next().unwrap_or("");
                let trimmed = code.trim();
                if trimmed.starts_with("#![") {
                    continue;
                }
                for opener in [
                    format!("{raw_kw} {{"),
                    format!("{raw_kw} fn "),
                    format!("{raw_kw} impl"),
                    format!("{raw_kw} trait"),
                    format!("{raw_kw} extern"),
                ] {
                    assert!(
                        !code.contains(&opener),
                        "{}:{}: contains {opener:?}",
                        path.display(),
                        line_no + 1
                    );
                }
            }
        }
        scanned += 1;
    }
    assert!(scanned >= 2, "expected at least src/lib.rs and tests/purity.rs");
}

#[test]
fn stoch_crdt_has_zero_dependencies_and_no_build_script() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let manifest = fs::read_to_string(manifest_dir.join("Cargo.toml")).expect("readable");
    assert!(
        !manifest.contains("[dependencies]"),
        "stoch-crdt must declare zero external dependencies"
    );
    assert!(
        !manifest.contains("[dev-dependencies]"),
        "stoch-crdt must declare zero dev-dependencies"
    );
    assert!(
        !manifest_dir.join("build.rs").exists(),
        "stoch-crdt must not ship a build script"
    );
}
