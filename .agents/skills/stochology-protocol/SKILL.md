---
name: stochology-protocol
description: End-to-end engineering, mathematical modeling, empirical verification, and textbook synchronization workflow for Stochology (2 crates: stochology and stochology-cli, 10 chapters: 00 to 09).
---

# Stochology Workflow Skill

Use this skill when adding or auditing Rust primitives in `crates/stochology`, chapter laboratories in `crates/stochology-cli`, or textbook chapters in `book-src/`.

## 1. Check Alignment with the Treatise (`notes/The Ethics of Aim.md`)
- Identify which axis of the **Nuclear Triad** (`Epistemic Margin`, `Vital Margin`, `Institutional Governance`) and which of the **7 Constitutive Tensions** (`inquiry`, `identification`, `asymmetry`, `archive`, `performativity`, `vital`, `governance`, `synthesis`) the change belongs to.
- Connect the mathematical primitive both to the evolving sequential environment (`CommunityReserveEnv` in `crates/stochology/src/engine.rs`) and to the sealed empirical datasets (`data/*.csv`).

## 2. Pure-Rust Implementation (`crates/stochology` & `crates/stochology-cli`)
- Use only the Rust standard library (`std`).
- Anchor all stochastic simulations to `SplitMix64` (`crates/stochology/src/rng.rs`).
- Write clear unit tests in the module's `#[cfg(test)] mod tests` verifying both the positive safeguard and the failure of the naive/unbuffered baseline.

## 3. Textbook & Interactive Lab Synchronization (`book-src/`)
- Ensure numbers and CLI snippets in `book-src/00-preface.md`..`09-methodological-families-reflexivity.md` and `book-src/appendix-b-architecture-verification.md` match the output of `cargo run --release --bin stochology`.
- Label quantitative claims honestly (`Measured` on a named dataset in `data/*.csv`, `Simulated`, `Assumed/Analytical`, or `Literature/Informative`).

## 4. Full Verification
Run `./scripts/verify.sh` and confirm 0 warnings and exit code `0`.
