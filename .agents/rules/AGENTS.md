---
trigger: always_on
description: Stochology governance rules — Nuclear Triad, 3 Collapse Propositions, 3 Canonical Operators, 7 Guard Sites, 35-Layer Multilayer Knowledge Graph, 2 pure-Rust crates (stochology + stochology-cli), 5 foundational notes, 86-chapter textbook, and verification pipeline.
---

# Stochology — Engineering & Pedagogical Rules

You operate within the discipline of **Stochology** (`notes/The Ethics of Aim.md`):

1. **Zero External Dependencies & Deterministic Simulability:**
   All code in `crates/stochology` and `crates/stochology-cli` uses pure Rust (`std` only, zero external crates) anchored to `SplitMix64` (`DEFAULT_SEED = 2026`).
2. **1-to-1 Architecture (Treatise $\leftrightarrow$ Code $\leftrightarrow$ Textbook):**
   - `crates/stochology/src/{lib, kernel, engine, epistemic, vital, governance, synthesis, grammar, multilayer}.rs` mirrors the 5 notes in `notes/` (`Aiming Between Mirrors.md`, `The Ethics of Aim.md`, `The Grammar of Aim.md`, `What Must Be Known When We Do Not Know.md`, `The Multilayer Graph of Knowledge and Aim.md`) and `book-src/00-preface.md`..`85-layer-polycentric-commons-urban-biospheric-order.md` + `appendix-a..c.md`:
     - **Filter I · Axis 1 ($S_{\mathrm{epi}}$):** Framework Chapters `43`–`45` / CLI Labs 2–4 (`epistemic.rs` — `EpistemicMarginState`)
     - **Filter II · Axis 2 ($S_{\mathrm{vital}}$):** Framework Chapters `46`–`47` / CLI Labs 5–6 (`vital.rs` — `VitalMarginState`)
     - **Filter III · Axis 3 ($S_{\mathrm{gov}}$):** Framework Chapters `48`–`49` / CLI Labs 7–8 (`governance.rs` — `GovernanceState`, `TemporalVault`)
     - **Triadic Synthesis ($M_{\mathrm{Leontief}}$):** Framework Chapter `50` / CLI Lab 9 (`synthesis.rs` — `TriadicIntegrity`, `CollapseProposition`, `ProcessTracingInference`, `ReflexiveBrakeAudit`)
     - **Grammar of Aim (syntax layer, all chapters):** [`notes/The Grammar of Aim.md`](../../notes/The%20Grammar%20of%20Aim.md) (`grammar.rs` — `CircuitTransit`, `GuardSite`, `GrammarVerdict`, `Sentence`, `judge`, `enumerate`, `shortest_violation`), frozen by `tests/grammar_spec.rs`
     - **Multilayer Knowledge Graph ($\mathcal{G}_1 \leftrightarrow \mathcal{G}_2$, Part V Chapters `51`–`85`):** [`notes/The Multilayer Graph of Knowledge and Aim.md`](../../notes/The%20Multilayer%20Graph%20of%20Knowledge%20and%20Aim.md) (`multilayer.rs` — `KnowledgeStratum`, `KnowledgeOrder`, `MultilayerGraph1`, `MultilayerGraph2`, $\binom{7}{3} = 12 + 9 + 9 + 4 + 1 = 35$ and $7 \times 5 = 35$), frozen by `tests/multilayer_spec.rs`
   - `CommunityReserveEnv` (`engine.rs`, Framework Chapter `42` / CLI Lab 1) is the unifying sequential environment comparing `NaiveOptimizer` against the progressive `StochologicalAgent` (and its three partial ablations demonstrating Collapse Propositions 1, 2, and 3).
3. **Subagent Roles (`.agents/agents/`):**
   - `stochology-modeler` (`workspace: branch`): Implements mathematical, physical, institutional, grammatical, and multilayer graph models in `crates/stochology` and `crates/stochology-cli`.
   - `stochology-adversary` (`workspace: inherit`): Runs `cargo test`, `cargo run --release --bin stochology`, and `scripts/verify.sh`, verifying the 3 Collapse Propositions, the 4 Reflexive Goodhart Brakes, grammar/multilayer parity, and claim provenance.
   - `stochology-scribe` (`workspace: inherit`): Keeps exported Rust symbols and CLI outputs synchronized with `book-src/*.md`, `notes/*.md`, `README.md`, `CITATION.cff`, and `AGENTS.md`.
4. **Verification & Seal Protection:**
   - Run `./scripts/verify.sh` before marking any task complete.
   - Any modification to `data/SHA256SUMS` or `notes/The Ethics of Aim.md` requires human confirmation.
