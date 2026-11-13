//! The linear binary boundary of the stochology core.
//!
//! This crate is the port, not a second implementation. It reads the `#[repr(C)]`
//! records of the archived design document (`02-arquitetura.md` §2), rebuilds the
//! core's own sealed objects with them, and calls [`stoch_core`]. No predicate is restated here: the
//! five regimes, the four verdicts and the diagnostic fold are the core's, and a
//! divergence between this port and the core is a bug in the port.
//!
//! # The closed surface
//!
//! Five exports, one memory, and nothing else:
//!
//! | Export | Signature | Meaning |
//! | :--- | :--- | :--- |
//! | [`step`] | `(i32, i32) -> i32` | `StateRec*`, `RegimeOut*`; returns the regime tag |
//! | [`accept`] | `(i32, i32) -> i32` | `StateRec*`, `VerdictOut*`; returns the verdict tag |
//! | [`diagnose`] | `(i32, i32, i32) -> i32` | `StepTriple*`, `len`, `DiagnosisOut*` |
//! | [`vectors_hash`] | `(i32) -> i32` | writes the 32-byte fixture digest at the pointer |
//! | [`abi_version`] | `() -> i64` | the handshake version, [`ABI_VERSION`] |
//!
//! # Return codes are the whole error channel
//!
//! An export returns `0 ..= 5` for a regime, `0 ..= 3` for a verdict or a binding
//! axis, and a negative code for a refusal (`02` §2.3). The codes `-1 ..= -12` are
//! the core's, taken verbatim from [`stoch_core::Error::code`]; [`E_CALLER_CONTRACT`]
//! is `-14` and names a fault of the caller rather than of the sentence or the state:
//! a null pointer, a misaligned pointer, a pointer outside the module's memory, or a
//! field carrying a value the document does not define.
//!
//! **No export panics, and no export writes a partial output.** Every pointer, every
//! padding field and every flag is validated before the core is called, every
//! arithmetic operation is total on the values that pass validation, and each output
//! record is assembled in full before it is stored. On any negative return the output
//! region is untouched, so a caller that ignores the status reads its own old bytes
//! and not a half-written regime.
//!
//! # What the port adds to the core
//!
//! Two gates exist here and not in the core, because the core's doors are Rust
//! signatures and this one is a byte string:
//!
//! 1. **The field gate.** A tag, a flag or a padding byte outside its closed set is
//!    refused with [`E_CALLER_CONTRACT`] instead of being coerced. In particular the
//!    seven guard readings must each be `-1` (absent) or a margin in `0 ..= SCALE`,
//!    and an absorbed step of a trajectory must carry no vital reading at all.
//! 2. **The bounds gate.** A pointer whose range is not wholly inside the module's
//!    memory is refused before it is read, so a truncated `len` cannot walk the
//!    caller off the end of its own arena.
//!
//! The sealed-reading gate that [`stoch_core::Trajectory::seal`] does not perform
//! (it is a `const fn` over a borrow, and a borrow cannot be refused) lives in
//! [`diagnose`]: each triple's three readings are checked against `-1 ∪ 0 ..= SCALE`
//! before the fold runs. That is a refusal of the *record*, not a re-derivation of
//! the fold.
//!
//! # Absence is a wire value, and it is not zero
//!
//! `-1` carries `Score::Missing` for a reading and `Option::None` for a declared
//! site, proposition or step index. Zero is a measurement and `0` names "no site"
//! only in the fields §2.2 defines that way (`site`, `proposition`,
//! `binding_axis`). The port never turns an absent reading into a zero.
//!
//! # The boundary is one module, and the compiler holds it there
//!
//! `03` §8 asks for the crate to deny the raw-pointer token and for exactly one
//! module to allow it. This crate is that shape: the attribute below denies the
//! token for the whole crate, and `abi.rs` is the single module that lifts the
//! denial, so every raw pointer, every `extern "C"` declaration and every byte store
//! lives in one file that a reviewer can read end to end. A second boundary would
//! need a second lift, and a second lift is what the boundary test refuses — the test
//! walks `src/` at run time and fails on the token anywhere but in `abi.rs`.

#![cfg_attr(all(not(test), target_arch = "wasm32"), no_std)]
#![deny(missing_docs)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unsafe_code)]

mod abi;

pub use abi::{
    abi_version, accept, accept_at, diagnose, diagnose_at, step, step_at, store_overlaps,
    store_span, vectors_hash, vectors_hash_at, DiagnosisOut, RegimeOut, StateRec, StepTriple,
    VerdictOut, ABI_VERSION, ARENA_STEP_BYTES, E_CALLER_CONTRACT, MAX_STEPS,
};

// ---------------------------------------------------------------------------
// The split, checked at run time against the directory rather than against a
// pattern someone pasted into a report.
// ---------------------------------------------------------------------------

#[cfg(test)]
mod boundary_tests {
    use super::*;
    use core::mem::{align_of, size_of};
    use std::fs;
    use std::path::{Path, PathBuf};

    /// The one file allowed to carry the raw-pointer token.
    const BOUNDARY_MODULE: &str = "abi.rs";

    /// The least number of occurrences `abi.rs` must carry, so that the scan over
    /// the other files cannot pass by finding nothing anywhere.
    const BOUNDARY_FLOOR: usize = 18;

    /// The token, assembled at run time so that this file does not carry it and does
    /// not have to exempt itself from its own scan.
    fn token() -> String {
        ["un", "safe"].concat()
    }

    fn is_ident(byte: u8) -> bool {
        byte.is_ascii_alphanumeric() || byte == b'_'
    }

    /// Occurrences of `needle` as a whole word: `x` in `unsafe_fn` is not one, and
    /// neither is the tail of a longer identifier.
    fn count_words(text: &str, needle: &str) -> usize {
        let haystack = text.as_bytes();
        let pattern = needle.as_bytes();
        let mut count = 0usize;
        let mut index = 0usize;
        while index + pattern.len() <= haystack.len() {
            if &haystack[index..index + pattern.len()] == pattern {
                let before = index == 0 || !is_ident(haystack[index - 1]);
                let after = index + pattern.len();
                let closed = after == haystack.len() || !is_ident(haystack[after]);
                if before && closed {
                    count += 1;
                    index = after;
                    continue;
                }
            }
            index += 1;
        }
        count
    }

    /// The file with whole-line comments removed, so that prose about the token does
    /// not count as the token.
    fn code_only(text: &str) -> String {
        let mut kept = String::with_capacity(text.len());
        for line in text.lines() {
            if line.trim_start().starts_with("//") {
                continue;
            }
            kept.push_str(line);
            kept.push('\n');
        }
        kept
    }

    fn sources() -> Vec<PathBuf> {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let mut paths: Vec<PathBuf> = fs::read_dir(&dir)
            .expect("the crate's src directory")
            .map(|entry| entry.expect("a directory entry").path())
            .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("rs"))
            .collect();
        paths.sort();
        paths
    }

    #[test]
    fn the_boundary_token_lives_in_one_module_only() {
        let needle = token();
        let mut scanned = 0usize;
        let mut inside = 0usize;
        for path in sources() {
            let name = path
                .file_name()
                .and_then(|name| name.to_str())
                .expect("a source file name")
                .to_string();
            let text = fs::read_to_string(&path).expect("a readable source file");
            let found = count_words(&code_only(&text), &needle);
            scanned += 1;
            if name == BOUNDARY_MODULE {
                inside = found;
                assert!(
                    found >= BOUNDARY_FLOOR,
                    "{name} carries {found} occurrences of the token, below the floor {BOUNDARY_FLOOR}"
                );
            } else {
                assert_eq!(
                    found, 0,
                    "{name} carries {found} occurrences of the boundary token; the boundary is {BOUNDARY_MODULE}"
                );
            }
        }
        assert!(scanned >= 2, "the scan saw {scanned} source files");
        assert!(inside >= BOUNDARY_FLOOR);
    }

    #[test]
    fn the_crate_denies_the_token_and_one_module_lifts_the_denial() {
        let needle = ["un", "safe"].concat();
        let lint = ["_", "code"].concat();
        let deny = format!("#![deny({needle}{lint})]");
        let allow = format!("#![allow({needle}{lint})]");

        let lib = fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("src/lib.rs"))
            .expect("lib.rs");
        let lib = code_only(&lib);
        assert!(
            lib.contains(&deny),
            "the crate root does not deny the token: {deny}"
        );
        assert!(
            !lib.contains(&allow),
            "the crate root lifts the denial; only {BOUNDARY_MODULE} may"
        );

        let module = fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("src")
                .join(BOUNDARY_MODULE),
        )
        .expect("the boundary module");
        let module = code_only(&module);
        assert_eq!(
            module.matches(&allow).count(),
            1,
            "the boundary module must lift the denial exactly once"
        );
        assert!(
            !module.contains(&deny),
            "the boundary module re-denies the token it is allowed to use"
        );
    }

    #[test]
    fn the_documented_sizes_survive_the_move() {
        assert_eq!(size_of::<StateRec>(), 256);
        assert_eq!(align_of::<StateRec>(), 8);
        assert_eq!(size_of::<RegimeOut>(), 64);
        assert_eq!(align_of::<RegimeOut>(), 8);
        assert_eq!(size_of::<VerdictOut>(), 64);
        assert_eq!(align_of::<VerdictOut>(), 8);
        assert_eq!(size_of::<StepTriple>(), 32);
        assert_eq!(align_of::<StepTriple>(), 8);
        assert_eq!(size_of::<DiagnosisOut>(), 48);
        assert_eq!(align_of::<DiagnosisOut>(), 8);
        assert_eq!(size_of::<StateRec>() + size_of::<RegimeOut>(), ARENA_STEP_BYTES);
    }
}
