---
trigger: always_on
description: Stochology governance rules — Nuclear Triad, 3 Collapse Propositions, 7 Constitutive Tensions, 2 pure-Rust crates (stochology + stochology-cli), 10-chapter textbook, and verification pipeline.
---

# Stochology — Engineering & Pedagogical Rules

You operate within the discipline of **Stochology** (`notes/The Ethics of Aim.md`):

1. **Zero External Dependencies & Deterministic Simulability:**
   All code in `crates/stochology` and `crates/stochology-cli` uses pure Rust (`std` only, zero external crates) anchored to `SplitMix64` (`DEFAULT_SEED = 2026`).
2. **1-to-1 Architecture (Treatise $\leftrightarrow$ Code $\leftrightarrow$ Textbook):**
   - `crates/stochology/src/{rng, sha256, math, engine, inquiry, identification, asymmetry, archive, performativity, vital, governance, synthesis}.rs` mirrors `notes/The Ethics of Aim.md` and `book-src/00-preface.md`..`09-methodological-families-reflexivity.md` + `appendix-a..c.md`.
   - `CommunityReserveEnv` (`engine.rs`) is the unifying sequential environment comparing `NaiveOptimizer` against the progressive `StochologicalAgent` (and its three partial ablations demonstrating Collapse Propositions 1, 2, and 3).
3. **Subagent Roles (`.agents/agents/`):**
   - `stochology-modeler` (`workspace: branch`): Implements mathematical, physical, and institutional models in `crates/stochology` and `crates/stochology-cli`.
   - `stochology-adversary` (`workspace: inherit`): Runs `cargo test`, `cargo run --release --bin stochology`, and `scripts/verify.sh`, verifying the 3 Collapse Propositions, the 3 Reflexive Goodhart Brakes, and claim provenance.
   - `stochology-scribe` (`workspace: inherit`): Keeps exported Rust symbols and CLI outputs synchronized with `book-src/*.md`, `README.md`, and `AGENTS.md`.
4. **Verification & Seal Protection:**
   - Run `./scripts/verify.sh` before marking any task complete.
   - Any modification to `data/SHA256SUMS` or `notes/The Ethics of Aim.md` requires human confirmation.
