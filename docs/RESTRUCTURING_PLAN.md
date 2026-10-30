# Restructuring Plan — Book and Protocol

> **Status: Phases 0–4 applied; Phase 5 awaiting a maintainer decision.** See §7 for what was executed and where it differs from the proposal. Each phase is a separate change set reviewed under Filter 4 (modeler authors, adversary audits, scribe keeps parity). Phases that touch `data/SHA256SUMS`, `cards/*.yaml`, `crates/stochology-governance/src/vault.rs`, or any of the 60 claim-defining files sealed in `data/preregistration/acceptance_registry.txt` require human review and a recorded re-seal (`AGENTS.md` §V.3).

## 1. Goal

Align the book and the normative documents with the three-tempo pedagogy of [`docs/SYLLABUS.md`](SYLLABUS.md) (Detachment → Abstraction → Return) **without** changing any claim, number, gate, or seal. The restructuring is organizational: it moves and splits text, it does not alter what the code computes.

## 2. Diagnosis (current state, verified against the repository)

### 2.1 Book (`book-src/`)

| # | Finding | Evidence |
| :-: | --- | --- |
| B1 | The Strategic AI six-pair synthesis has **no chapter**. | `StrategicAiAccommodationSynthesis` appears only in `00-preface.md`, `12-…`, `26-architecture.md`, `27-verification.md`. |
| B2 | The coupled public-private SFC society synthesis is a subsection (§22.7) of the relational-ontology chapter, although it is a full pre-registered synthesis with its own virgin confirmation block. | `22-relational-ontology.md:242`. |
| B3 | Part VI mixes **operation** (Funnel/SIGA, Lab 7) with **reference** (architecture, verification, sources). | `SUMMARY.md`, Part VI. |
| B4 | Synthetic-recursion collapse (Gerstgrasser, Accumulate vs. Replace) is a $L_3 \to L_1$ mechanism (Pillar P), but it is discussed in Ch. 13 and Ch. 18, not in Ch. 19. | `grep Gerstgrasser book-src` → 00, 13, 18, 27, 28. |
| B5 | The Preface (472 lines) carries foundational material (five interpretations of probability, the Dutch Book, reproducible chance) that the syllabus treats as Movement I content. The Dutch Book appears nowhere else. | `00-preface.md` §3–§4. |
| B6 | Tools taught under the Time and Invariance lenses live in the strategic synthesis: Huber-Kalman (`RobustKalmanControlAudit`) and `do`-Shapley (`ShapleyCausalAudit`). `ShapleyCausalAudit` is not named in any chapter. | `crates/stochology-lenses/src/strategic/`. |
| B7 | Chapters have no uniform internal shape; the Detachment/Abstraction/Return tempos are implicit. | Chapter headings vary (e.g. Ch. 22 opens with a narrative section, then numbered axes, then "Computational Audit in Rust", then "Exercises"). |

### 2.2 Protocol (`AGENTS.md`, `docs/PROTOCOL.md`)

| # | Finding |
| :-: | --- |
| P1 | `AGENTS.md` §III is titled "The Five Transversal Lenses" but enumerates 13 items: 5 lenses, the Almost-Perfect Machine, and 7 frontier syntheses. The normative kernel and the frontier are not separated. |
| P2 | `docs/PROTOCOL.md` is organized by apparatus (diagnosis, contract, funnel, governance, Lab 7, architecture), not by the six ordered pairs, so a reader cannot see the pair → symbol → gate → defect chain in one place. |
| P3 | Each new synthesis grows `AGENTS.md` by a long paragraph; the contract is becoming a changelog. |
| P4 | `docs/SYLLABUS.md` is outside the overclaim blacklist and the symbol-parity checks in `scripts/verify.sh`. |

## 3. Target Architecture

### 3.1 Protocol: four tiers

| Tier | Content | Normative? | Change policy |
| --- | --- | :---: | --- |
| **0 — Kernel** | Four Prohibitions, $\Phi_1$–$\Phi_5$, Epistemological Card, gates `G1`–`G7`, claim-provenance discipline (§II.C) | Yes | Human review |
| **1 — Lenses and uncertainty** | The five transversal lenses, Kennedy-O'Hagan, Info-Gap + Sniedovich, Almost-Perfect Machine and Authentic Form | Yes | Human review |
| **2 — Bridges** | The three implemented forward bridges and the three reverse reading frames, each with its pair → symbol → gate → defect row | Yes | Human review |
| **3 — Syntheses** | Analog, critical, stocks, triad, strategic, society, relational: each a self-contained module gated by `G8` multi-seed acceptance, with its own pre-registration and `discarded_claims` | Gated, not axiomatic | Pre-registration + re-seal |

`AGENTS.md` keeps Tiers 0–2 in full and reduces each Tier 3 synthesis to a one-row registry entry (name, symbols, datasets, gated/discarded claim counts, defect name, link). The long paragraphs move to `docs/syntheses/<name>.md`.

### 3.2 `docs/PROTOCOL.md`: reorganized by pair

1. Kernel (Tier 0).
2. Lenses and uncertainty typing (Tier 1).
3. Six pairs (Tier 2), one section per pair, each closing with a table *pair → symbols → gates → defect proofs → book chapters*.
4. Operation: Funnel, SIGA, governance, Lab 7.
5. Syntheses registry (Tier 3), linking to `docs/syntheses/`.
6. Architecture mapping.

The CAPIBA six-pillar table stays verbatim (it is checked by `scripts/verify.sh`).

### 3.3 Book: Movements as Parts

| Part | Title (Movement) | Chapters (proposed) | Change from today |
| --- | --- | --- | --- |
| I | The Act of Conjecture | Preface (slimmed); 1 Layers & Six Pairs; 2 Typology of Uncertainty (absorbs Preface §3: five interpretations, Dutch Book) | Content moves from Preface (B5) |
| II | Five Laws of the Real | 3–9 unchanged; Ch. 9 adds cross-references to Huber-Kalman and `do`-Shapley as authorial links (B6) | Cross-references only |
| III | World ⇄ Model | 10–14 unchanged | Gerstgrasser material in Ch. 13 shrinks to a pointer (B4) |
| IV | Model ⇄ Decision | 15–18 unchanged | Gerstgrasser material in Ch. 18 shrinks to a pointer (B4) |
| V | Decision ⇄ World | 19 (absorbs Accumulate vs. Replace, B4); 20; 21; 22 Relational Ontology (without §22.7); 23 Stochary-Conjectural Cybernetics | §22.7 leaves (B2) |
| VI | **Cross-Pair Syntheses** (new) | 24 Strategic AI Six-Pair Accommodation (**new**, B1); 25 Coupled Public-Private SFC Society (**promoted** from §22.7, B2) | New Part |
| VII | Operation | 26 Funnel & SIGA; 27 Transversal Empirical Synthesis (Lab 7) | Split from old Part VI (B3) |
| Appendices | Reference | A Architecture; B Verification and Gates; C Data Sources; D Full Specification; E Course Syllabus | Old Ch. 26–28 become appendices (B3) |

**Uniform chapter template (B7).** Every chapter in Parts I–VI follows:
`Detachment` → `Abstraction` (layer, pair, uncertainty class) → `Return` (equations and symbols) → `Computational Audit in Rust` (regenerated `labs`/`gates` output) → `Negative Proof` (`--defect`) → `Provenance Ledger` (every number with its class) → `Exercises`.

## 4. Phased Execution

| Phase | Scope | Touches seals? | Owner → auditor |
| :-: | --- | :---: | --- |
| **0** | Tooling: add `docs/SYLLABUS.md` and `docs/syntheses/*.md` to `BLACKLIST_DOCS` and to the symbol-parity loop in `scripts/verify.sh`. | No | modeler → adversary |
| **1** | Protocol docs: Tier split of `AGENTS.md` §III; `docs/syntheses/` extraction; pair-ordered `docs/PROTOCOL.md`. Pure text moves; every quoted number re-checked against current `labs`/`gates` output. | No | scribe → adversary |
| **2** | Book, non-renumbering moves: Preface §3 → Ch. 2 (B5); Gerstgrasser → Ch. 19 (B4); Ch. 9 cross-references (B6); chapter template headings (B7). | No | scribe → adversary |
| **3** | Book, new chapters: write Ch. 24 (Strategic) and promote §22.7 to Ch. 25 (Society). Numbers regenerated from `labs -- --all` and `gates`, never hand-edited (§II.C rule 8). | No | scribe → adversary |
| **4** | Book renumbering and new Parts VI–VII and appendices. Update `SUMMARY.md`, every "Chapter N" reference, relative links, the `28-sources.md` ↔ registry sync check in `scripts/verify.sh`, the "28-chapter book" wording in `AGENTS.md` §V.2/§VII and in `.agents/agents/stochology-scribe.md`. | No (but `verify.sh` edits) | scribe → adversary, **human review** |
| **5** | *(Optional, deferred)* Code-side mirror: only if a module path must change. Moving any of the 60 claim-defining source files changes the sealed hashes, so `G8` blocks until `acceptance_registry.txt` and `data/SHA256SUMS` are re-sealed. **Recommendation: do not do this.** The restructuring is documentary; module paths stay where they are. | **Yes** | modeler → adversary, **human review + re-seal** |

Each phase ends with `scripts/verify.sh` green (all 21 negative proofs `exit 1`, `gates` `exit 0`).

## 5. Invariants the Restructuring Must Preserve

- No change to any claim, threshold, seed set, pass rate, gated or discarded claim, or dataset.
- The 21 negative proofs and the CAPIBA partition `3 + 4 + 2 + 3 + 2 + 7` stay as they are.
- `book-src/00-preface.md` keeps the CAPIBA expansion and the dedication checked by `verify.sh`; normative documents stay free of the preface's metaphorical vocabulary.
- The three reverse pairs remain reading frames, not bridges (§II.C rule 7); the three-tempo cycle and the five Movements remain an authorial pedagogical extension.

## 6. Maintainer Decisions

1. Renumbering (Phase 4): **approved** and applied.
2. Moving Tier 3 synthesis detail out of `AGENTS.md` into `docs/syntheses/`: **approved** and applied.
3. Phase 5 (code moves and re-seal): **open**. The answer received ("de forma alguma") is ambiguous between "do not leave it out" and "do not do it"; nothing in Phase 5 has been executed.

## 7. Execution Log (Phases 0–4)

| Phase | Done | Deviation from §3–§4 |
| :-: | --- | --- |
| 0 | `scripts/verify.sh`: `docs/SYLLABUS.md` and `docs/syntheses/*.md` added to the overclaim blacklist; `SUMMARY.md` link check now covers appendices; new checks that every numbered chapter opens with the three-tempo block and that mdBook `{{#include}}` targets exist. | The 51-symbol parity loop was left on `docs/PROTOCOL.md` alone: every symbol still appears there (architecture table and registry). |
| 1 | `docs/PROTOCOL.md` reorganized into §1 Kernel, §2 Lenses, §3 Six Pairs (with the pair map), §4 Operation, §5 Syntheses Registry, §6 Architecture; items 7–13 of the former §1.3 moved verbatim to `docs/syntheses/{relational,analog,critical,stocks,triad,strategic,society}.md`, each with the matching former `AGENTS.md` §III paragraph as "Contract Summary". `AGENTS.md` §III split into Tier 1 (lenses, uncertainty) and §III.B Tier 3 registry. | None. |
| 2 | Preface §3 moved to Ch. 2 §2.1 (old §2.1–2.3 became §2.2–2.4); the Accumulate-vs.-Replace derivation moved from Ch. 13 and Ch. 18 to Ch. 19 §19.3, with pointers left behind; Ch. 9 gained authorial cross-references to Huber-Kalman and `do`-Shapley (Ch. 24). | The chapter template (B7) is implemented as a "Three tempos" opening block in every chapter (Detachment, Abstraction, Return, Negative proof), enforced by `verify.sh`. Existing chapter bodies were not re-sectioned, and a Provenance Ledger section exists only in the new Ch. 24–25. |
| 3 | New Ch. 24 (Strategic AI Six-Pair Accommodation) and Ch. 25 (Coupled Society, promoted from §22.7). Quoted `labs`/`gates` lines are copied from current output and pass `book_snapshot_parity`. | None. |
| 4 | Old Ch. 24–25 → Ch. 26–27 (Part VII Operation); old Ch. 26–28 → Appendices A–C; full specification → Appendix D; syllabus → Appendix E (an mdBook include of `docs/SYLLABUS.md`). `book.toml` redirects, `lab/index.html` slugs and labels, `README.md`, `AGENTS.md`, and `.agents/` updated. | `appendix-protocol.md` keeps its filename (only its title changed) to avoid churning references. |

`scripts/verify.sh` passes end to end after Phases 0–4 (all 8 gates exit `0`, all 21 negative proofs exit `1`, `book_snapshot_parity` green). `mdbook build` was not run locally (the binary is not installed in this environment); the Pages workflow builds the book with mdBook 0.5.3.
