---
name: stochology-protocol
description: End-to-end engineering, mathematical modeling, empirical verification, and textbook synchronization workflow for Stochology (2 crates: stochology and stochology-cli, 5 foundational notes, 86 chapters: 00 to 85, 9 CLI laboratories: 42 to 50, 35 multilayer orders: 51 to 85).
---

# Stochology Workflow Skill

Use this skill when adding or auditing Rust primitives in `crates/stochology`, chapter laboratories in `crates/stochology-cli`, foundational notes in `notes/`, or textbook chapters in `book-src/`.

## 1. Check Alignment with the Five Foundational Notes (`notes/`)
- Identify which axis of the **Nuclear Triad** (`Epistemic Margin`, `Vital Margin`, `Institutional Governance`) and which module (`epistemic.rs`, `vital.rs`, `governance.rs`, `synthesis.rs`, `grammar.rs`, `multilayer.rs`) the change belongs to.
- Connect the mathematical primitive both to the evolving sequential environment (`CommunityReserveEnv` in `crates/stochology/src/engine.rs`), to the Seven Guard Sites (`GuardSite` and `CircuitTransit` in `crates/stochology/src/grammar.rs`) and 35-layer Multilayer Knowledge Graph (`KnowledgeStratum`, `KnowledgeOrder`, `MultilayerGraph1`, `MultilayerGraph2` in `crates/stochology/src/multilayer.rs`), and to the sealed empirical datasets (`data/*.csv`).

## 2. Pure-Rust Implementation (`crates/stochology` & `crates/stochology-cli`)
- Use only the Rust standard library (`std`).
- Anchor all stochastic simulations to `SplitMix64` (`crates/stochology/src/kernel.rs`).
- Write clear unit tests in the module's `#[cfg(test)] mod tests` and integration tests (`tests/book_anchors.rs`, `tests/grammar_spec.rs`, `tests/multilayer_spec.rs`) verifying both the positive safeguard and the failure of the naive/unbuffered baseline.

## 3. Textbook & Interactive Lab Synchronization (`book-src/`)
- Ensure numbers and CLI snippets in the 9 Framework chapters (`book-src/42-sequential-loop-naive-agent.md`..`50-methodological-families-reflexivity.md`, marked by the banner `> Laboratory N of the `stochology` CLI: `--chapter N`.`), `book-src/appendix-b-architecture-verification.md`, and `README.md` match the output of `cargo run --release --bin stochology`.
- Ensure Educational chapters (`01`–`41` and Part V Multilayer chapters `51`–`85`) satisfy `tests/book_anchors.rs` and `tests/multilayer_spec.rs` (three stated outcomes, closing `## Bridge`, and resolved `file.rs::Symbol` anchors).
- Label quantitative claims honestly (`Measured` on a named dataset in `data/*.csv`, `Simulated`, `Assumed/Analytical`, or `Literature/Informative`).

## 4. Full Verification
Run `./scripts/verify.sh` and confirm 0 warnings and exit code `0`.
