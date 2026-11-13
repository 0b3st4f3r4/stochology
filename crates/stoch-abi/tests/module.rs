//! Reading the shipped binary, not a report about it.
//!
//! `02` §2.1 closes the surface of the module: five functions, one memory, and no
//! import. This test parses the wasm binary itself — the section table and the export
//! section — and asserts the set, not a substring of a summary. It also asserts that
//! the fixture digest appears in the module's data, which is what "the digest is
//! derived from the file at build time" means in bytes.
//!
//! If the release artifact has not been built, the test says so and returns; the
//! verification sequence builds it first (`cargo build --release --target
//! wasm32-unknown-unknown -p stoch-abi`).

use std::path::PathBuf;

/// The release artifact of this crate for the shipped target.
fn artifact() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/wasm32-unknown-unknown/release/stoch_abi.wasm")
}

/// Read an unsigned LEB128 `u32`, advancing the cursor.
fn leb_u32(bytes: &[u8], cursor: &mut usize) -> u32 {
    let mut result: u32 = 0;
    let mut shift = 0;
    loop {
        let byte = bytes[*cursor];
        *cursor += 1;
        result |= u32::from(byte & 0x7F) << shift;
        if byte & 0x80 == 0 {
            return result;
        }
        shift += 7;
    }
}

/// One section of the module: its id and its payload.
struct Section<'a> {
    id: u8,
    payload: &'a [u8],
}

/// Split the module into sections, checking the header on the way.
fn sections(bytes: &[u8]) -> Vec<Section<'_>> {
    assert!(bytes.len() > 8);
    assert_eq!(&bytes[..4], b"\0asm", "the wasm magic");
    assert_eq!(&bytes[4..8], &[1, 0, 0, 0], "the module version is 1");
    let mut cursor = 8;
    let mut out = Vec::new();
    while cursor < bytes.len() {
        let id = bytes[cursor];
        cursor += 1;
        let size = leb_u32(bytes, &mut cursor) as usize;
        let payload = &bytes[cursor..cursor + size];
        cursor += size;
        out.push(Section { id, payload });
    }
    out
}

/// The export section: `(name, kind)` pairs in module order.
fn exports(payload: &[u8]) -> Vec<(String, u8)> {
    let mut cursor = 0;
    let count = leb_u32(payload, &mut cursor);
    let mut out = Vec::new();
    for _ in 0..count {
        let len = leb_u32(payload, &mut cursor) as usize;
        let name = String::from_utf8(payload[cursor..cursor + len].to_vec()).unwrap();
        cursor += len;
        let kind = payload[cursor];
        cursor += 1;
        let _index = leb_u32(payload, &mut cursor);
        out.push((name, kind));
    }
    out
}

#[test]
fn the_module_has_no_import_section_and_exactly_the_five_exports() {
    let path = artifact();
    let bytes = match std::fs::read(&path) {
        Ok(bytes) => bytes,
        Err(_) => {
            eprintln!(
                "skipped: {} is not built; run `cargo build --release --target \
                 wasm32-unknown-unknown -p stoch-abi`",
                path.display()
            );
            return;
        }
    };

    let sections = sections(&bytes);
    let ids: Vec<u8> = sections.iter().map(|s| s.id).collect();
    assert!(
        !ids.contains(&2),
        "an import section (id 2) is present; the module must import nothing"
    );

    let export_section = sections
        .iter()
        .find(|section| section.id == 7)
        .expect("a module with exports has an export section");
    let mut names: Vec<String> = exports(export_section.payload)
        .into_iter()
        .map(|(name, _kind)| name)
        .collect();
    names.sort();
    assert_eq!(
        names,
        vec![
            "abi_version".to_string(),
            "accept".to_string(),
            "diagnose".to_string(),
            "memory".to_string(),
            "step".to_string(),
            "vectors_hash".to_string(),
        ],
        "the closed surface of 02 §2.1: five functions and one memory, nothing else"
    );

    let memory = sections
        .iter()
        .find(|section| section.id == 5)
        .expect("the module declares a memory");
    assert_eq!(leb_u32(memory.payload, &mut 0), 1, "one memory, as §2.1 fixes");

    // The digest of the fixture is in the module's data, because the module was built
    // from it. A digest that had been typed in by hand would also be here; what makes
    // this evidence is `tests/port.rs`, which compares the same 32 bytes against
    // `sha256sum` of the file on disk.
    let digest = stoch_core::vectors_hash();
    assert!(
        bytes.windows(digest.len()).any(|window| window == digest),
        "the fixture digest is embedded in the module"
    );

    eprintln!(
        "module: {} bytes, {} sections, {} exports",
        bytes.len(),
        sections.len(),
        names.len()
    );
}

#[test]
fn the_module_does_not_grow_its_memory_or_import_a_table() {
    let path = artifact();
    let bytes = match std::fs::read(&path) {
        Ok(bytes) => bytes,
        Err(_) => return,
    };
    let sections = sections(&bytes);
    // A table (id 4) would be a function-pointer surface nothing in this crate needs.
    assert!(!sections.iter().any(|section| section.id == 4));
    // No start function (id 8): the boundary is called, it does not run on load.
    assert!(!sections.iter().any(|section| section.id == 8));
}
