# SWARM_REVIEW.md — Multi-Agent Swarm Audit Charter & Partition Map (Stochology)

This document is the operational charter and master partition map for an **autonomous multi-agent swarm** conducting a comprehensive review of the **Stochology** repository. Every agent in the swarm must read [`AGENTS.md`](AGENTS.md) and this file before inspecting or modifying any file.

---

## I. Mission & Non-Negotiable Constitutional Invariants

**Stochology** (*stókhos*, aim or conjecture, + *lógos*, reasoned account) is an integrative framework and transversal research program for the theory and ethics of aim under structural uncertainty when action must precede the outcome, whose distinct unit of analysis is the **full mediation circuit across World, Archive, and Institution**.

Every agent in the review swarm is bound by eight non-negotiable invariants:

1. **Zero External Dependencies (`std` only):**
   Both workspace crates (`crates/stochology` and `crates/stochology-cli`) use pure Rust (`std` only, zero external crates). All pseudo-random streams are anchored to deterministic [`SplitMix64`](crates/stochology/src/kernel.rs) (`DEFAULT_SEED = 2026`).
2. **The Leontief Nuclear Triad & Three Collapse Propositions:**
   $$\mathcal{T}_{\mathrm{Stochology}} = \bigl(S_{\mathrm{epi}},\; S_{\mathrm{vital}},\; S_{\mathrm{gov}}\bigr), \qquad M_{\mathrm{Leontief}} = \min\bigl(S_{\mathrm{epi}},\, S_{\mathrm{vital}},\, S_{\mathrm{gov}}\bigr)$$
   - **Proposition 1 ($\neg S_{\mathrm{epi}} \implies \text{Blind Undersizing}$):** `CollapseProposition::Proposition1BlindUndersizing` (`StochologicalAgent::without_epistemic_margin()`, cell `011`).
   - **Proposition 2 ($\neg S_{\mathrm{vital}} \implies \text{Impotent Lucidity Before Ruin}$):** `CollapseProposition::Proposition2ImpotentLucidity` (`StochologicalAgent::epistemic_only()`, cell `101`).
   - **Proposition 3 ($\neg S_{\mathrm{gov}} \implies \text{Cynical Capture and Pillaging}$):** `CollapseProposition::Proposition3CynicalCapture` (`StochologicalAgent::without_governance()`, cell `110`).
3. **One-Way Authority Arrow:**
   - **Notes (`notes/*.md`) $\to$ Code (`crates/*/src/*.rs`) $\to$ Book (`book-src/*.md`):**
     A textbook chapter may cite a note and must cite code symbols (`file.rs::Symbol`); a note never cites a textbook chapter as authority; Rust source code never cites chapter numbers.
4. **Human-Gated Cryptographic Seal Protection (`.agents/hooks.json`):**
   - **DO NOT** modify [`data/SHA256SUMS`](data/SHA256SUMS), [`notes/The Ethics of Aim.md`](notes/The%20Ethics%20of%20Aim.md), or `TemporalVault` in [`crates/stochology/src/governance.rs`](crates/stochology/src/governance.rs) without explicit human inline-diff confirmation.
5. **Bidirectional CLI Output Parity:**
   - The fenced `text` blocks containing `=== CHAPTER ` in [`README.md`](README.md), [`book-src/appendix-b-architecture-verification.md`](book-src/appendix-b-architecture-verification.md), and the 9 Framework Chapters (`book-src/42-*.md`..`50-*.md`, marked by the banner `> Laboratory N of the `stochology` CLI: `--chapter N`.`) must match `cargo run --release --bin stochology` line-for-line in both directions.
   - Educational chapters (`01`–`41` and `51`–`85`) must **never** contain the laboratory banner or `=== CHAPTER `, and no chapter may contain an HTML comment (`<!--`).
6. **Claim Provenance Discipline:**
   Every quantitative claim in `book-src/`, `notes/`, and `README.md` must carry one of the four canonical provenance tags: `[Measured]` (on a named dataset in `data/*.csv`), `[Simulated]` (`SplitMix64`), `[Assumed]` (analytical/benchmark parameter), or `[Literature]` (external citation). Non-canonical tags such as `[Identity]` or `[Analytical ...]` are forbidden.
7. **Symbol & Line-Anchor Integrity (`PYSYM` & `book_anchors.rs`):**
   - Every backticked `Type::member` in `book-src/*.md`, `notes/*.md`, `README.md`, and `AGENTS.md` must resolve to an existing `pub` symbol/member in `crates/*/src/*.rs`.
   - Every backticked `file.rs::Symbol` or `file.rs::Type::method` in educational chapters (`01`–`41`, `51`–`85`) is checked by `tests/book_anchors.rs`.
   - Every markdown link of the form `` [`Symbol`](crates/...#L10-L20) `` is checked by `scripts/verify.sh` against the actual declaration line window.
8. **Markdown & KaTeX Hygiene (`PYKATEX`):**
   - **Never** write `file://` or `/home/` paths inside repository `.md` files on disk (use relative links such as `crates/stochology/src/grammar.rs`).
   - Inline math `$...$` must have no leading/trailing space immediately inside `$` and no word/digit character immediately after the closing `$`. Never use `\_` outside `\texttt{...}` or `\*` inside math mode.

---

## II. Conflict-Free Swarm Topology (8 Parallel Review Cells)

To allow a swarm of agents to audit and refine the repository concurrently without file lock contention or merge collisions, the workspace is partitioned into **8 disjoint cells**:

| Cell ID | Role / Agent Profile | Owned Files (Write Scope) | Primary Verification Command |
| :---: | :--- | :--- | :--- |
| **Cell 1** | `Swarm-Notes-Auditor` | `notes/Aiming Between Mirrors.md`<br>`notes/The Ethics of Aim.md` *(human-gated)*<br>`notes/The Grammar of Aim.md`<br>`notes/What Must Be Known When We Do Not Know.md`<br>`notes/The Multilayer Graph of Knowledge and Aim.md` | `cargo test --test grammar_spec --test multilayer_spec` |
| **Cell 2** | `Swarm-Rust-Core-Auditor` | `crates/stochology/src/*.rs`<br>`crates/stochology/tests/*.rs`<br>`crates/stochology-cli/src/*.rs`<br>`data/SOURCES.md` | `cargo clippy --all-targets -- -D warnings && cargo test` |
| **Cell 3** | `Swarm-Book-Parts-I-II` | `book-src/00-preface.md`<br>`book-src/01-*.md` .. `book-src/26-*.md`<br>*(Preface + Part I `01`–`08` + Part II `09`–`26`)* | `cargo test --test book_anchors` |
| **Cell 4** | `Swarm-Book-Part-III` | `book-src/27-*.md` .. `book-src/41-*.md`<br>*(Part III: Foundations of Judgment `27`–`41`)* | `cargo test --test book_anchors` |
| **Cell 5** | `Swarm-Book-Part-IV-Labs` | `book-src/42-*.md` .. `book-src/50-*.md`<br>`book-src/appendix-a-mathematics.md`<br>`book-src/appendix-b-architecture-verification.md`<br>`book-src/appendix-c-sources.md`<br>`book-src/lab/index.html`<br>`book-src/figures/*.svg` | `cargo run --release --bin stochology` + `scripts/verify.sh` |
| **Cell 6** | `Swarm-Book-Part-V-A` | `book-src/51-*.md` .. `book-src/70-*.md`<br>*(Part V Strata `S1`–`S4`: Layers `L01`–`L20`)* | `cargo test --test book_anchors --test multilayer_spec` |
| **Cell 7** | `Swarm-Book-Part-V-B` | `book-src/71-*.md` .. `book-src/85-*.md`<br>*(Part V Strata `S5`–`S7`: Layers `L21`–`L35`)* | `cargo test --test book_anchors --test multilayer_spec` |
| **Cell 8** | `Swarm-S3-Star-Adversary` | `README.md`, `AGENTS.md`, `CITATION.cff`, `CONTRIBUTING.md`, `Cargo.toml`, `book.toml`, `book-src/SUMMARY.md`, `.agents/**`, `scripts/verify.sh`, `SWARM_REVIEW.md` | `./scripts/verify.sh` *(Full 6-stage gate)* |

---

## III. Canonical Ontological Reference Tables

### 1. The Seven Guard Sites (`T1`–`T7`), `CircuitTransit`s ($3 + 3 + 1 = 7$), and Canonical Strata (`S1`–`S7`)

Every agent auditing `notes/`, `grammar.rs`, `multilayer.rs`, or `book-src/` must enforce this exact 1-to-1 structure:

| Site (`GuardSite`) | Triad Axis | Circuit Transit (`GuardSite::transit`) | Transit Kind | Primary Plane (`GuardSite::plane`) | Canonical Stratum (`KnowledgeStratum::canonical_guard_site`) | Primary Orders (`KnowledgeOrder::primary_guard_site`, 5 each) |
| :--- | :---: | :--- | :--- | :---: | :--- | :--- |
| **`T1Framing`** | $S_{\mathrm{epi}}$ | `Institution -> Archive` | Reflexive Lock 1 | `Plane::Institution` | `S2` (`S2FormalMetrological`) | `L06, L15, L20, L24, L30` |
| **`T2Severity`** | $S_{\mathrm{epi}}$ | `Archive -> World` | Reflexive Lock 2 | `Plane::World` | `S4` (`S4ClinicalFormative`) | `L09, L11, L18, L27, L31` |
| **`T3Archive`** | $S_{\mathrm{epi}}$ | `World -> Archive` | Direct Transit 1 | `Plane::Archive` | `S5` (`S5SemioticArchival`) | `L04, L08, L17, L22, L29` |
| **`T4Formalization`** | $S_{\mathrm{vital}}$ | `Archive -> Institution` | Direct Transit 2 | `Plane::Archive` | `S3` (`S3LivingEvolutionary`) | `L05, L07, L14, L16, L34` |
| **`T5Temporal`** | $S_{\mathrm{vital}}$ | `World -> Institution` | Reflexive Lock 3 | `Plane::World` | `S1` (`S1PhysicalDynamical`) | `L01, L12, L23, L26, L33` |
| **`T6Performative`** | $S_{\mathrm{gov}}$ | `Institution -> World` | Direct Transit 3 | `Plane::Institution` | `S6` (`S6StrategicEconomic`) | `L03, L19, L25, L28, L35` |
| **`T7Liability`** | $S_{\mathrm{gov}}$ | `World <-> Archive <-> Institution` | Integral Closure | `Plane::Institution` | `S7` (`S7ConstitutionalBiospheric`) | `L02, L10, L13, L21, L32` |

**Structural invariants verified by `grammar.rs` and `multilayer.rs`:**
- **Axial partition (`3 / 2 / 2`):** $|S_{\mathrm{epi}}| = 3$ (`T1, T2, T3`), $|S_{\mathrm{vital}}| = 2$ (`T4, T5`), $|S_{\mathrm{gov}}| = 2$ (`T6, T7`).
- **Vandermonde decomposition ($\binom{7}{3} = 35$):**
  $$\binom{7}{3} = \underbrace{\binom{3}{1}\binom{2}{1}\binom{2}{1}}_{(1,1,1)\,=\,12} + \underbrace{\left[\binom{3}{2}\binom{2}{1}\binom{2}{0} + \binom{3}{1}\binom{2}{2}\binom{2}{0}\right]}_{(2,1,0)+(1,2,0)\,=\,9} + \underbrace{\left[\binom{3}{2}\binom{2}{0}\binom{2}{1} + \binom{3}{1}\binom{2}{0}\binom{2}{2}\right]}_{(2,0,1)+(1,0,2)\,=\,9} + \underbrace{\left[\binom{3}{0}\binom{2}{2}\binom{2}{1} + \binom{3}{0}\binom{2}{1}\binom{2}{2}\right]}_{(0,2,1)+(0,1,2)\,=\,4} + \underbrace{\binom{3}{3}\binom{2}{0}\binom{2}{0}}_{(3,0,0)\,=\,1} = 12 + 9 + 9 + 4 + 1 = 35$$
  The five bracketed groups are exactly the five `TriadicExposureRegime` variants; the same identity is stated in `notes/The Multilayer Graph of Knowledge and Aim.md` (§Vandermonde) and in the `TriadicExposureRegime` documentation in `multilayer.rs`.
- **Circuit transit bijection (`3 + 3 + 1 = 7`):** `GuardSite::is_bijection_over_circuit_transits() == true`.
- **Stratum-to-Guard-Site bijection:** `KnowledgeStratum::canonical_guard_site()` and `KnowledgeStratum::from_canonical_guard_site()` form an exact 7-to-7 bijection.
- **Balanced primary bottleneck assignment ($7 \times 5 = 35$):**
  1. Every `GuardSite` is `primary_guard_site()` for **exactly 5** `KnowledgeOrder`s (`MultilayerGraph2::is_balanced_five_orders_per_primary_site() == true`).
  2. Within every `KnowledgeStratum` (`S1`–`S7`), its 5 `KnowledgeOrder`s have **5 pairwise-distinct** `primary_guard_site()` values (`KnowledgeStratum::has_distinct_primary_guard_sites() == true`), always including the stratum's `canonical_guard_site()`.
  3. Within every `KnowledgeOrder`, `primary_guard_site()` is a member of that order's 3-element `guard_sites()` combination.

### 2. Complete 35-Order Multilayer Map (`L01`–`L35` $\leftrightarrow$ Chapters `51`–`85`)

| Code | Ch | Stratum | Order of Knowledge (`KnowledgeOrder`) | Textbook File (`book-src/`) | Primary Site | Primary Axis | Unique Guard Triple (`guard_sites`) | Signature $(E,V,G)$ |
| :--- | :-: | :---: | :--- | :--- | :---: | :---: | :---: | :---: |
| `L01` | `51` | `S1` | `L01MicroKinetic` | `51-layer-micro-kinetic-sensorimotor-order.md` | `T5` | `Vital` | `[T1, T5, T7]` | `(1,1,1)` |
| `L02` | `52` | `S1` | `L02MesoBufferedQueue` | `52-layer-meso-buffered-queue-dispatch-order.md` | `T7` | `Governance` | `[T4, T6, T7]` | `(0,1,2)` |
| `L03` | `53` | `S1` | `L03MacroCoupledTopology` | `53-layer-macro-coupled-network-contagion-order.md` | `T6` | `Governance` | `[T4, T5, T6]` | `(0,2,1)` |
| `L04` | `54` | `S1` | `L04GeophysicalChaosDataAssimilation` | `54-layer-geophysical-chaos-data-assimilation-order.md` | `T3` | `Epistemic` | `[T3, T4, T5]` | `(1,2,0)` |
| `L05` | `55` | `S1` | `L05ContinuumFractureStructural` | `55-layer-fracture-continuum-structural-order.md` | `T4` | `Vital` | `[T2, T4, T7]` | `(1,1,1)` |
| `L06` | `56` | `S2` | `L06FormalComputableLinearType` | `56-layer-formal-computable-type-order.md` | `T1` | `Epistemic` | `[T1, T4, T5]` | `(1,2,0)` |
| `L07` | `57` | `S2` | `L07ProbabilityCredenceMeasurement` | `57-layer-probability-credence-measurement-order.md` | `T4` | `Vital` | `[T1, T2, T4]` | `(2,1,0)` |
| `L08` | `58` | `S2` | `L08MetrologicalInfoGeometric` | `58-layer-metrological-information-geometric-order.md` | `T3` | `Epistemic` | `[T3, T4, T6]` | `(1,1,1)` |
| `L09` | `59` | `S2` | `L09ExperimentalSamplingCausal` | `59-layer-experimental-sampling-causal-order.md` | `T2` | `Epistemic` | `[T1, T2, T3]` | `(3,0,0)` |
| `L10` | `60` | `S2` | `L10CryptographicZeroKnowledgeAdversarial` | `60-layer-cryptographic-zero-knowledge-adversarial-order.md` | `T7` | `Governance` | `[T2, T6, T7]` | `(1,0,2)` |
| `L11` | `61` | `S3` | `L11BiochemicalKineticMolecular` | `61-layer-biochemical-kinetic-molecular-order.md` | `T2` | `Epistemic` | `[T2, T5, T6]` | `(1,1,1)` |
| `L12` | `62` | `S3` | `L12MorphogeneticDevelopmental` | `62-layer-morphogenetic-evolutionary-developmental-order.md` | `T5` | `Vital` | `[T1, T5, T6]` | `(1,1,1)` |
| `L13` | `63` | `S3` | `L13ImmunologicalSelfGovernance` | `63-layer-immunological-self-governance-order.md` | `T7` | `Governance` | `[T2, T5, T7]` | `(1,1,1)` |
| `L14` | `64` | `S3` | `L14TaxonomySystematicsOntology` | `64-layer-taxonomy-systematics-ontology-order.md` | `T4` | `Vital` | `[T1, T3, T4]` | `(2,1,0)` |
| `L15` | `65` | `S3` | `L15StratigraphyPaleobiologyDeepTime` | `65-layer-stratigraphy-paleobiology-deep-time-order.md` | `T1` | `Epistemic` | `[T1, T3, T5]` | `(2,1,0)` |
| `L16` | `66` | `S4` | `L16PharmacologicalClinicalDiagnostic` | `66-layer-pharmacological-clinical-diagnostic-order.md` | `T4` | `Vital` | `[T2, T4, T5]` | `(1,2,0)` |
| `L17` | `67` | `S4` | `L17EpidemiologyOutbreakSurveillance` | `67-layer-epidemiology-outbreak-surveillance-order.md` | `T3` | `Epistemic` | `[T2, T3, T5]` | `(2,1,0)` |
| `L18` | `68` | `S4` | `L18PharmacovigilanceRegulatoryToxicology` | `68-layer-pharmacovigilance-regulatory-toxicology-order.md` | `T2` | `Epistemic` | `[T2, T3, T7]` | `(2,0,1)` |
| `L19` | `69` | `S4` | `L19MetascienceScientometricsReplication` | `69-layer-metascience-scientometrics-replication-order.md` | `T6` | `Governance` | `[T2, T3, T6]` | `(2,0,1)` |
| `L20` | `70` | `S4` | `L20AcousticAestheticTragicPedagogical` | `70-layer-acoustic-aesthetic-tragic-pedagogical-order.md` | `T1` | `Epistemic` | `[T1, T4, T6]` | `(1,1,1)` |
| `L21` | `71` | `S5` | `L21LinguisticEvidentialPragmatic` | `71-layer-linguistic-evidential-pragmatic-order.md` | `T7` | `Governance` | `[T3, T6, T7]` | `(1,0,2)` |
| `L22` | `72` | `S5` | `L22PhilologicalEpigraphicTextual` | `72-layer-philological-epigraphic-textual-order.md` | `T3` | `Epistemic` | `[T2, T3, T4]` | `(2,1,0)` |
| `L23` | `73` | `S5` | `L23TaphonomicHistoriographical` | `73-layer-taphonomic-archaeological-historiographical-order.md` | `T5` | `Vital` | `[T3, T5, T7]` | `(1,1,1)` |
| `L24` | `74` | `S5` | `L24CriticalArchivalTransitionalJustice` | `74-layer-critical-archival-transitional-justice-order.md` | `T1` | `Epistemic` | `[T1, T3, T7]` | `(2,0,1)` |
| `L25` | `75` | `S5` | `L25MediaEcologyInformationRetrieval` | `75-layer-media-ecology-information-retrieval-order.md` | `T6` | `Governance` | `[T1, T3, T6]` | `(2,0,1)` |
| `L26` | `76` | `S6` | `L26SequentialDecisionBanditRealOptions` | `76-layer-sequential-decision-bandit-real-options-order.md` | `T5` | `Vital` | `[T1, T2, T5]` | `(2,1,0)` |
| `L27` | `77` | `S6` | `L27GameTheoryStrategicAdversarial` | `77-layer-game-theory-strategic-adversarial-order.md` | `T2` | `Epistemic` | `[T1, T2, T6]` | `(2,0,1)` |
| `L28` | `78` | `S6` | `L28MacroFinanceEconometricsActuarial` | `78-layer-macro-finance-econometrics-actuarial-order.md` | `T6` | `Governance` | `[T2, T4, T6]` | `(1,1,1)` |
| `L29` | `79` | `S6` | `L29OfficialStatisticsForensicAccounting` | `79-layer-official-statistics-forensic-accounting-order.md` | `T3` | `Epistemic` | `[T3, T4, T7]` | `(1,1,1)` |
| `L30` | `80` | `S6` | `L30CorporateGovernancePrincipalAgent` | `80-layer-corporate-governance-principal-agent-order.md` | `T1` | `Epistemic` | `[T1, T6, T7]` | `(1,0,2)` |
| `L31` | `81` | `S7` | `L31CriminalProofForensicEpistemology` | `81-layer-criminal-proof-forensic-epistemology-order.md` | `T2` | `Epistemic` | `[T1, T2, T7]` | `(2,0,1)` |
| `L32` | `82` | `S7` | `L32EvidentiaryTortConstitutional` | `82-layer-evidentiary-tort-constitutional-legal-order.md` | `T7` | `Governance` | `[T1, T4, T7]` | `(1,1,1)` |
| `L33` | `83` | `S7` | `L33DistributedConsensusSre` | `83-layer-distributed-consensus-sre-order.md` | `T5` | `Vital` | `[T5, T6, T7]` | `(0,1,2)` |
| `L34` | `84` | `S7` | `L34HighHazardInfrastructureSafety` | `84-layer-infrastructure-nuclear-hydraulic-safety-order.md` | `T4` | `Vital` | `[T4, T5, T7]` | `(0,2,1)` |
| `L35` | `85` | `S7` | `L35PolycentricCommonsBiospheric` | `85-layer-polycentric-commons-urban-biospheric-order.md` | `T6` | `Governance` | `[T3, T5, T6]` | `(1,1,1)` |

---

## IV. Cell-by-Cell Audit Checklists

### Cell 1 (`Swarm-Notes-Auditor` — `notes/*.md`)
1. Verify that each of the 5 notes fulfills its distinct role without duplicating or contradicting another note:
   - `Aiming Between Mirrors.md`: Three ethical refusals (panoptic certainty, bureaucratic fatalism, unbuffered optimization).
   - `The Ethics of Aim.md` *(human-gated)*: Three-plane circuit, Nuclear Triad, 3 Collapse Propositions, 4 Pre-Registered Falsifiers (`F1`–`F4`), numerical parity with CLI output.
   - `The Grammar of Aim.md`: Seven Guard Sites (`3 + 3 + 1 = 7` `CircuitTransit`s), 4 Verdicts (`GrammarVerdict`), EBNF grammar, `grammar-anchors` block resolved by `tests/grammar_spec.rs`.
   - `What Must Be Known When We Do Not Know.md`: Regress termination on *télos*, `3` Trunk Questions $\to$ `7` Guard Sites, and `3` Regimes of Action.
   - `The Multilayer Graph of Knowledge and Aim.md`: $\mathcal{G}_1 \leftrightarrow \mathcal{G}_2$, Vandermonde $\binom{7}{3} = 12 + 9 + 9 + 4 + 1 = 35$, $7 \times 5 = 35$, `multilayer-anchors` block resolved by `tests/multilayer_spec.rs`.
2. Ensure zero KaTeX errors, zero phantom symbols (`Type::member`), and zero `file://` or `/home/` URIs.

### Cell 2 (`Swarm-Rust-Core-Auditor` — `crates/stochology` & `crates/stochology-cli`)
1. **Structural & Combinatorial Invariants (`grammar.rs` & `multilayer.rs`):**
   - `GuardSite::is_bijection_over_circuit_transits() == true` ($3 + 3 + 1 = 7$ `CircuitTransit`s across `World`, `Archive`, `Institution`).
   - `MultilayerGraph2::is_bijection_over_35_guard_combinations() == true` ($\binom{7}{3} = 12 + 9 + 9 + 4 + 1 = 35$).
   - `MultilayerGraph2::every_stratum_covers_all_seven_sites() == true` and `KnowledgeStratum::has_distinct_primary_guard_sites() == true` for all 7 strata (`S1`–`S7`).
   - `MultilayerGraph2::is_balanced_five_orders_per_primary_site() == true` ($7 \times 5 = 35$, with `order.guard_sites().contains(&order.primary_guard_site())` for every `order` in `L01`–`L35`).
   - `MultilayerGraph1::every_ancestor_cone_spans_all_three_axes() == true`.
2. Verify all unit tests, integration tests (`book_anchors.rs`, `grammar_spec.rs`, `multilayer_spec.rs`), `cargo clippy --all-targets -- -D warnings`, and `cargo llvm-cov` ($\ge 90\%$ lines, regions, and functions).

### Cell 3 & Cell 4 (`Swarm-Book-Parts-I-III` — `book-src/00`–`41`)
1. Verify every educational chapter (`01`–`41`) has:
   - Exact heading `## By the end of this chapter you can` followed by numbered outcomes `1. `, `2. `, `3. `.
   - Exact closing section `## Bridge` leading cleanly into the next chapter.
   - Valid `file.rs::Symbol` anchors resolving to real symbols in `crates/stochology/src/*.rs`.
   - Pedagogical progression from intuitive motivation (Part I `01`–`08`), mathematical/philosophical/scientific bases (Part II `09`–`26`), to foundations of judgment (Part III `27`–`41`), preparing the reader for Part IV (`42`–`50`) and Part V (`51`–`85`).

### Cell 5 (`Swarm-Book-Part-IV-Labs` — `book-src/42`–`50`, `appendix-a..c.md`, `lab/index.html`)
1. Verify each of the 9 Framework Chapters (`42`–`50`) opens on its `# ` title followed by the banner `> Laboratory N of the `stochology` CLI: `--chapter N`.` ($N = \text{chapter} - 41$), has $\ge 10$ canonical provenance tags, and matches the `=== CHAPTER N:` CLI output verbatim.
2. Verify `book-src/appendix-a-mathematics.md`, `book-src/appendix-b-architecture-verification.md`, and `book-src/appendix-c-sources.md` cover all exported types, functions, and datasets, including `grammar.rs` and `multilayer.rs`.
3. Verify `book-src/lab/index.html` matches `cargo run --quiet --release --bin stochology -- --export-lab-json` and all SVG figures in `book-src/figures/*.svg` use `theme/book.css` CSS variables without hardcoded hex colors outside `<style>`.

### Cell 6 & Cell 7 (`Swarm-Book-Part-V` — `book-src/51`–`85`)
1. Verify each of the 35 chapters (`51`–`85`) corresponds 1-to-1 to `KnowledgeOrder::ALL` (`L01`–`L35`):
   - Correct Stratum (`S1`–`S7`), 3-Site Guard Combination (`{T_a, T_b, T_c}`), Triadic Signature (`(n_epi, n_vital, n_gov)`), and `TriadicExposureRegime` against §III.2 below; then `dag_parents` (a field emitted by `multilayer.rs` but not tabulated here) against `KnowledgeOrder::parent_layers()` and the chapter's own "builds on" header line. There is no `P1`–`P5` phase field anywhere in `multilayer.rs`, the note, or the chapters; the five-fold axis is the five `KnowledgeOrder`s per stratum.
   - Where a chapter discusses its primary guard site (`KnowledgeOrder::primary_guard_site`) or its stratum's canonical guard site (`KnowledgeStratum::canonical_guard_site`), ensure it matches `multilayer.rs` and `notes/The Multilayer Graph of Knowledge and Aim.md`.
   - Every chapter has `## By the end of this chapter you can` (`1. `, `2. `, `3. `), `## Bridge`, and valid `file.rs::Symbol` anchors.

### Cell 8 (`Swarm-S3-Star-Adversary` — Root & Full Verification Gate)
1. After all cells complete their pass, run:
   ```bash
   ./scripts/verify.sh
   ```
2. Confirm exit code `0` (`ALL STOCHOLOGY CHECKS PASSED (0 WARNINGS, 14 SEALED DATASETS, 9 LABS)`).
