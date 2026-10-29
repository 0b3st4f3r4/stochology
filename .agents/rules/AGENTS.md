---
trigger: always_on
description: Model-agnostic Stochology Protocol orchestration contract, Three Structural Questions, Four Prohibitions, Five Boundary Invariants (Phi1-Phi5), Three Subagents, and Terminal/Cryptographic Execution Policy for any autonomous coding agent or multi-agent harness.
---

# Stochology Protocol — Model-Agnostic Multi-Agent Harness & Governance Rules

You operate under the **Stochology Protocol** (`docs/PROTOCOL.md`). Follow these rules across the workspace:

1. **Zero External Dependencies & Deterministic Simulability ($\Phi_1$):**
   All 5 Rust crates (`stochology-core`, `stochology-lenses`, `stochology-siga`, `stochology-governance`, `stochology-cli`) use pure Rust (`std` only, zero external crates) anchored to `SplitMix64` (`seed = 2026`, including lock-free `.split(stream_id)` substream trees).
2. **The Three Structural Questions & Four Prohibitions:**
   Before implementing a change, answer the **Three Structural Questions** (Ontological Layer $L_1/L_2/L_3$ & Bridge, Stirling-Walker Uncertainty Class, and Severe Test) and follow the **Four Prohibitions** (`PROHIBITION_1` no unstructured prose between modules; `PROHIBITION_2` no lone mean under nonlinearity or heavy tails; `PROHIBITION_3` no confusing learning with performativity $\theta_{\mathrm{PS}}$ vs. $\theta_{\mathrm{PO}}$ or ignoring Goodhart's 4 variants; `PROHIBITION_4` no dogmatic $0\%/100\%$ probabilities or omitted attempts $K \ge 1$).
3. **Subagent Architecture (`.agents/agents/`):**
   - `stochology-modeler` (`workspace: branch`): Implements mathematical and stochastic models (`null_model` / `decision_rule`).
   - `stochology-adversary` (`workspace: inherit`): Acts as the VSM $S3^*$ sporadic audit channel and Filter 4 `adversarial_reviewer`, running `gates` and all 21 negative defect proofs (`gates -- --defect <name>`).
   - `stochology-scribe` (`workspace: inherit`): Keeps exported Rust symbols synchronized with `docs/PROTOCOL.md`, `book-src/appendix-protocol.md`, and `book-src/00-preface.md`..`28-sources.md`.
4. **Terminal Execution Policy & Cryptographic Protection ($\Phi_3$):**
   - **Allow List (Autonomous Execution):** `scripts/verify.sh`, `cargo fmt`, `cargo clippy -- -D warnings`, `cargo test`, `cargo run --release --bin labs -- --all`, and `cargo run --release --bin gates` (plus `--defect <name>`).
   - **Request Review (Human Confirmation Required via Inline Diff):** Any modification to `data/SHA256SUMS`, `cards/*.yaml`, or `crates/stochology-governance/src/vault.rs` (`TemporalVault`) requires human review so no agent mutates pre-registration hashes or increments $K \ge 1$ unrecorded.
5. **Verification (`scripts/verify.sh`):**
   Before marking a task complete, run `scripts/verify.sh` to confirm 0 compiler or clippy warnings, passage of all 7 laboratories (`labs --all`), approval of all 8 gates (`gates`, exit code `0`), and blocking of all 21 injected defects (`gates -- --defect <name>`, exit code `1`).
6. **Claim Provenance (`AGENTS.md` §II.C):**
   Every number written outside the code (book, docs, README, derived texts) is labelled **measured** (dataset), **simulated** (seed; say so if `G8` discarded the claim), **assumed** (hand-set input), or **literature/informative** (source). Never compare an assumed or simulated value with a measured one as equal evidence, never generalize a toy simulation to a named real system, give units and relative size for large absolute numbers, frame narratives as thought experiments, keep the six ordered pairs apart (the three forward pairs $L_1 \to L_2$, $L_2 \to L_3$, $L_3 \to L_1$ are the implemented bridges; the three reverse pairs $L_2 \to L_1$, $L_3 \to L_2$, $L_1 \to L_3$ organize existing code), and present anything the repository does not implement as an authorial extension.
