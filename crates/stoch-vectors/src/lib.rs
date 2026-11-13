//! The normative harness of the Stochology core.
//!
//! Five instruments, one per obligation of the harness task:
//!
//! * [`fixture`] — run the 44 normative vectors and compare canonical output;
//! * [`generator`] — the coverage corpus of `01` §6.3, verified by census;
//! * [`contract`] — §V.9's concordance, read from AGENTS.md at run time;
//! * [`topology`] — $\binom{7}{3} = 35$, the Vandermonde partition and Table B.1;
//! * [`schema`] — the fixture's `key=value` record, its parser and its renderer.
//!
//! The crate has one dependency, the core it exercises, and no opinion of its own
//! about `step`, `accept` or `diagnose`: every observation in this crate is a call
//! into `stoch-core` followed by a rendering of what came back.
//!
//! The harness cannot feed a verdict back into the operator that produced it,
//! which is the type-level half of the seal test of Chapter 41 §4:
//!
//! ```compile_fail
//! use stoch_core::{accept, Verdict};
//! let report = Verdict::Interior;
//! let _ = accept(&report);
//! ```
//!
//! Nor can a diagnosis stand where a state does:
//!
//! ```compile_fail
//! use stoch_core::{diagnose, step, Score, StepReading, Trajectory};
//! let steps = [StepReading::new(Score::Present(1), Score::Present(1), Score::Present(1), false, 1)];
//! let diagnosis = diagnose(&Trajectory::seal(&steps));
//! let _ = step(&diagnosis);
//! ```

pub mod contract;
pub mod fixture;
pub mod generator;
pub mod operators;
pub mod schema;
pub mod topology;

use std::path::PathBuf;

/// The repository root: two directories above this crate.
pub fn workspace_root() -> PathBuf {
    contract::workspace_root()
}

/// The normative fixture, as the proposal states it. The core embeds the same
/// bytes; [`fixture`] can be pointed at either.
pub fn fixture_path() -> PathBuf {
    workspace_root().join("crates/stoch-core/vectors.kv")
}

/// Appendix B, which carries Table B.1.
pub fn appendix_b_path() -> PathBuf {
    workspace_root().join("book-src/appendix-b-architecture-verification.md")
}

/// The preface, which records the seven stratum anchors.
pub fn preface_path() -> PathBuf {
    workspace_root().join("book-src/00-preface.md")
}
