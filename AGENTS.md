# AGENTS.md — Governance, Engineering, and Pedagogical Contract (Stochology)

This file is the binding contract for any agent or contributor working in this repository. **Stochology** (from *stókhos*, aim or conjecture, and *lógos*, reasoned account) is an **integrative framework and transversal research program for the theory and ethics of aim** (*stókhos*, from Plato's *Philebus* 55e–56c, Hippocratic *sēmeîa*, *kairós*, and *mētis*, Galenic *stokhastikē tékhnē*, and Antipater of Tarsus's archer to Bernoulli's *Ars Conjectandi*) for judgment and decision-making under structural uncertainty when action must precede the outcome, whose distinct unit of analysis is the **full mediation circuit across world, archive, and institution**.

Emerging from the three foundational ethical refusals of [`notes/Aiming Between Mirrors.md`](notes/Aiming%20Between%20Mirrors.md), formalized in [`notes/The Ethics of Aim.md`](notes/The%20Ethics%20of%20Aim.md), given its syntax layer in [`notes/The Grammar of Aim.md`](notes/The%20Grammar%20of%20Aim.md) (seven guard sites across three direct loop transits, three reflexive locks, and one integral three-plane closure, four verdicts, generation and recognition over one set of productions, executed in `crates/stochology/src/grammar.rs` and frozen against drift by `crates/stochology/tests/grammar_spec.rs`), given its reflexive core in [`notes/What Must Be Known When We Do Not Know.md`](notes/What%20Must%20Be%20Known%20When%20We%20Do%20Not%20Know.md) (the termination of the regress of higher-order uncertainty on the agent's own circuit, the `3 → 7` inventory of what must be known before the shot, and the three regimes of action in the dark), and organized across the 35-layer Multilayer Knowledge Graph in [`notes/The Multilayer Graph of Knowledge and Aim.md`](notes/The%20Multilayer%20Graph%20of%20Knowledge%20and%20Aim.md) ($\mathcal{G}_1 \leftrightarrow \mathcal{G}_2$, $\binom{7}{3} = 12 + 9 + 9 + 4 + 1 = 35$ and $7 \times 5 = 35$, executed in `crates/stochology/src/multilayer.rs` and frozen against drift by `crates/stochology/tests/multilayer_spec.rs`), its implementation and textbook are organized 1-to-1 around the **Sequential Agent–World–Institution Loop** (`CommunityReserveEnv`) and the **Three Canonical Mathematical Operators** (`EpistemicMarginState`, `VitalMarginState`, `GovernanceState`) in **pure Rust (`std` only, zero external dependencies)**.

---

## I. The Nuclear Triad and the Three Collapse Propositions

Stochology unites three historical lineages (Epistemological/Statistical, Physical/Cybernetic, and Critical/Ethical) into a **Nuclear Triad** of three non-substitutable safeguards:

$$\mathcal{T}_{\mathrm{Stochology}} = \bigl(\text{Epistemic Margin},\; \text{Vital Margin},\; \text{Institutional Governance}\bigr), \qquad M_{\mathrm{Leontief}} = \min\bigl(S_{\mathrm{epi}},\, S_{\mathrm{vital}},\, S_{\mathrm{gov}}\bigr)$$

Removing any single axis triggers a specific structural collapse (`CollapseProposition` and `TriadicIntegrity` in `crates/stochology/src/synthesis.rs`, demonstrated on `CommunityReserveEnv` in `crates/stochology/src/engine.rs`):

1. **Collapse Proposition 1 ($\neg\text{Epistemic Margin} \implies \text{Blind Undersizing}$):** Even an institution that maintains physical reserves sizes them against a false point certainty when it ignores non-random missingness ($\rho_{R,Y} \ne 0$), cluster design effects ($\mathrm{DEFF}$), and partial-identification bounds (`StochologicalAgent::without_epistemic_margin()`).
2. **Collapse Proposition 2 ($\neg\text{Vital Margin} \implies \text{Impotent Lucidity Before Ruin}$):** Knowing the exact width of uncertainty while operating at $96\text{--}100\%$ utilization or excess leverage with $\mathrm{TTS}_{\mathrm{survival}} < \mathrm{TTR}_{\mathrm{recovery}}$ leads to immediate absorption at the ruin barrier (`StochologicalAgent::epistemic_only()`).
3. **Collapse Proposition 3 ($\neg\text{Institutional Governance} \implies \text{Cynical Capture and Pillaging}$):** Epistemic doubt is weaponized as an agnotological alibi while common vital reserves are pillaged under limited-liability moral hazard, high-powered proxy gaming, and testimonial discounting of frontline warnings (`StochologicalAgent::without_governance()`).

---

## II. 1-to-1 Architecture: Treatise $\leftrightarrow$ Code (`crates/stochology`) $\leftrightarrow$ Book (`book-src/`)

The workspace consists of exactly **two pure-Rust crates**:
- `crates/stochology`: the core mathematical, physical, and institutional library (`std` only).
- `crates/stochology-cli`: the single binary `stochology` that verifies `data/SHA256SUMS` and runs the 9 progressive chapter laboratories over `CommunityReserveEnv` and the 14 real-world CSV datasets in `data/*.csv`.

**The five notes in `notes/` do five different jobs, and none is a draft of another.** *Aiming Between Mirrors* holds the ethical refusals; *The Ethics of Aim* maps the three-plane circuit, the Nuclear Triad and the three collapse propositions; *The Grammar of Aim* fixes the syntax layer over trajectories (`CircuitTransit` and `GuardSite` across $3 + 3 + 1 = 7$ circuit transits); *What Must Be Known When We Do Not Know* isolates the epistemic transformation that precedes the shot — why the regress of higher-order uncertainty terminates only on the agent's own circuit (*télos*), the `3` trunk questions that unfold into the `7` guard sites, and the three regimes of action (buffered action, reversible fractional probe, lowering the bow); *The Multilayer Graph of Knowledge and Aim* unifies the $\binom{7}{3} = 35$ triadic combinations of the seven guard sites ($\mathcal{G}_1$, `MultilayerGraph1`, decomposed by Vandermonde into $12 + 9 + 9 + 4 + 1 = 35$) with the $7 \times 5 = 35$ stratification of human knowledge ($\mathcal{G}_2$, `MultilayerGraph2`, across seven strata `S1`–`S7` in 1-to-1 bijection with `T1`–`T7` via `KnowledgeStratum::canonical_guard_site`, balanced at five `KnowledgeOrder`s per stratum and five `KnowledgeOrder`s per primary guard site via `KnowledgeOrder::primary_guard_site`). A chapter may cite a note; a note may not cite a chapter as authority.

**Two kinds of chapter live in `book-src/`, and they obey different contracts.**

The book is one numbered sequence, `00`–`85`, in five parts: *Thinking Under Uncertainty* (01–08),
*The Bases* (09–26, in three blocks: mathematics 09–15, philosophy and epistemology 16–20, natural
science 21–26), *Foundations of Judgment* (27–41), *The Framework* (42–50) and *The Multilayer
Architecture of Knowledge and Action* (51–85, in seven strata of five orders: `S1` 51–55, `S2` 56–60,
`S3` 61–65, `S4` 66–70, `S5` 71–75, `S6` 76–80, `S7` 81–85), with the preface at 00 and three
appendices. Numbering is editorial and carries no machine meaning.

- **Framework chapters** — `42`–`50`, the nine that correspond 1-to-1 to the code in the table below — open on their `# ` title followed by the laboratory banner `> Laboratory N of the `stochology` CLI: `--chapter N`.` (with $N = \text{chapter} - 41$), and contain a fenced `text` block introduced by `=== CHAPTER ` that must match the CLI output verbatim. `scripts/verify.sh` selects them by the banner — never by filename — and enforces the match in both directions. No chapter carries an HTML marker: upload content sniffers classify a file beginning with `<!--` as `text/html` and refuse it as markdown.
- **Educational chapters** — `01`–`26`, `27`–`41` and `51`–`85` — are markdown only. They carry the prerequisites a framework chapter presupposes and the 35 multilayer orders of Part V, they contain **no** `=== CHAPTER ` block, and their contract is enforced by `crates/stochology/tests/book_anchors.rs` (and, for `51`–`85`, `crates/stochology/tests/multilayer_spec.rs`): dense numbering, three stated outcomes, exactly one closing `## Bridge`, and every cited `file.rs::Symbol` anchor resolving to a real symbol.

A concept enters the code only when a claim in the educational arc cannot be verified without it. The arrow runs one way, and it protects authority rather than the absence of names: (i) a note never cites a chapter as authority; (ii) Rust source never cites a chapter as authority and never defers a definition to prose — a definition lives in the code; (iii) code may carry the `KnowledgeOrder` $\leftrightarrow$ chapter bijection as model data and may name the chapter that exercises a symbol, in doc comments and error strings alike. A chapter number inside code is a CLI laboratory number `N` of `--chapter N`, whose framework chapter is `41 + N`; the pre-renumbering numbers that `vital.rs` and `governance.rs` record in brackets are edition history, not the current mapping.

| Chapter | Textbook (`book-src/`) | Contiguous Triad Axis & Treatise (`notes/The Ethics of Aim.md` & `notes/The Multilayer Graph of Knowledge and Aim.md`) | Module (`crates/stochology/src/`) | Empirical Datasets (`data/*.csv`) |
| :---: | :--- | :--- | :--- | :--- |
| **00** | `00-preface.md` | §I–§II (Genealogy, Nuclear Triad, 3 Collapse Propositions, 3 Canonical Operators) | `lib.rs` | — |
| **42** | `42-sequential-loop-naive-agent.md` | §II.2 (Sequential Mediation Loop) | `kernel.rs`, `engine.rs` (`CommunityReserveEnv`, `NaiveOptimizer`, `StochologicalAgent`) | — |
| **43** | `43-inquiry-openness-multiverse.md` | §III.1 (**Filter I · Axis 1 [$S_{\mathrm{epi}}$]:** Framing, Cromwell, Agnotology vs. Severity, Type III, *e-BH*, Multiverse) | `epistemic.rs` (`EpistemicMarginState::from_inquiry_framing`) | — |
| **44** | `44-underdetermination-partial-identification.md` | §III.2 (**Filter I · Axis 1 [$S_{\mathrm{epi}}$]:** *e-processes*, Lakatos, Two-Arm Manski & Lee ATE Bounds, Peterson) | `epistemic.rs` (`EpistemicMarginState::from_ate_bounds`) | `nsw_experimental.csv`, `cps_observational.csv` |
| **45** | `45-world-to-archive-big-data-paradox.md` | §III.3 (**Filter I · Axis 1 [$S_{\mathrm{epi}}$]:** Keynes $V$, Meng $\rho_{R,Y}$, Campbell–Moulton $\mathrm{DEFF}$, Kish, Chao $\hat{n}_{00}$, Bitemporal Vintages) | `epistemic.rs` (`EpistemicMarginState::from_archive_clusters`, `from_meng_big_data`) | `cps_observational.csv`, `bank_calls.csv`, `rtdsm_routput_vintages.csv` |
| **46** | `46-non-identical-asymmetry-heavy-tails.md` | §IV.1 (**Filter II · Axis 2 [$S_{\mathrm{vital}}$]:** Dialectical Penumbra, Ordinal Invariance, Jensen $\Delta_J$, CRPS, Hill $\hat{\alpha}$) | `vital.rs` (`VitalMarginState::from_heavy_tails_and_asymmetry`) | `danish_fire.csv`, `nyc_flights_2013.csv`, `nyc_flights_2023.csv`, `nyc_flights_2013_h1.csv`, `nyc_flights_2023_h1.csv`, `houston_flights_2011.csv` |
| **47** | `47-irreversible-time-vital-margin.md` | §IV.2 (**Filter II · Axis 2 [$S_{\mathrm{vital}}$]:** Itô Drain, Baker–McHale Kelly, Canguilhem–Holling, Kingman, $\mathrm{TTS}_{\mathrm{survival}}\ge\mathrm{TTR}_{\mathrm{recovery}}$, Wald, Sniedovich) | `vital.rs` (`VitalMarginState::from_queue_basin_and_survival`, `from_queue_basin_and_derived_ttr`, `from_non_ergodic_returns`) | `djia_1885_2023.csv`, `vic_elec.csv`, `nsw_experimental.csv` |
| **48** | `48-reactive-worlds-performativity-goodhart.md` | §V.1 (**Filter III · Axis 3 [$S_{\mathrm{gov}}$]:** CORP-PAV, ICP, Kennedy–O'Hagan GP, Tian–Pearl, Perdomo/MacKenzie, Goodhart) | `governance.rs` (`GovernanceState::from_ex_ante_gate`) | `atlantic_storms.csv` |
| **49** | `49-moral-luck-apt-agency-institutions.md` | §V.2–§VI (**Filter III · Axis 3 [$S_{\mathrm{gov}}$]:** Sosa/Baron–Hershey, List–Pettit, Fricker/Beer Algedonic, Holmström–Innes, Chouldechova, Zheng–Meister, `TemporalVault`) | `governance.rs` (`GovernanceState::from_real_time_and_ex_post_gates`, `TemporalVault`) | `toronto_arrests.csv` |
| **50** | `50-methodological-families-reflexivity.md` | §VII–§VIII (**Triadic Synthesis [$M_{\mathrm{Leontief}}$]:** 5 Methodological Families, Process Tracing, 4 Reflexive Brakes, 4 Refutability Conditions) | `synthesis.rs` (`TriadicIntegrity`, `CollapseProposition`, `ProcessTracingInference`, `ReflexiveBrakeAudit`) | `atlantic_storms.csv` |
| **51–85** | `51-layer-micro-kinetic-sensorimotor-order.md` .. `85-layer-polycentric-commons-urban-biospheric-order.md` | **Multilayer Knowledge Graph ($\mathcal{G}_1 \leftrightarrow \mathcal{G}_2$):** 35 Orders (`L01`–`L35`) across 7 Strata (`S1`–`S7` $\leftrightarrow$ `T1`–`T7`), 5 Orders per Stratum and 5 primary Orders per Guard Site | `multilayer.rs` (`KnowledgeStratum`, `KnowledgeOrder`, `MultilayerGraph1`, `MultilayerGraph2`) | — |

---

## III. Claim Provenance Discipline

Every quantitative claim in `book-src/`, `README.md`, or derived text must carry its honest provenance:
1. **Measured:** Computed by `stochology-cli` on a sealed dataset in `data/*.csv` (name the dataset, e.g., `danish_fire.csv`).
2. **Simulated:** Output of a deterministic `SplitMix64` simulation under a pre-registered seed (e.g., `CommunityReserveEnv` under `DEFAULT_SEED = 2026`).
3. **Assumed / Analytical:** A hand-set parameter or closed-form benchmark input (e.g., $C = 100, L_{\mathrm{inst}} = 200$ in `EpistemicMarginState::from_inquiry_framing`).
4. **Literature (Informative):** An external empirical finding cited from the literature (e.g., Wong et al. 2021 on the Epic Sepsis Model's $88\%$ false-alarm rate ($1 - \mathrm{PPV}$), or Zheng & Meister 2025 on the $\sim 10\text{ bits/s}$ human behavioral/sensory-motor information throughput limit).

---

## IV. Verification & Execution Policy

- **Zero External Dependencies:** Both `crates/stochology` and `crates/stochology-cli` use pure Rust (`std` only) and deterministic `SplitMix64` streams.
- **Verification Pipeline (`scripts/verify.sh`):**
  1. `cargo fmt --all -- --check`
  2. `cargo clippy --all-targets -- -D warnings`
  3. `cargo test`
  4. `cargo llvm-cov --workspace --summary-only --fail-under-lines 90 --fail-under-regions 90 --fail-under-functions 90`
  5. `cargo run --release --bin stochology`
  6. `sha256sum -c --quiet data/SHA256SUMS`, followed by the structural, provenance, symbol and hygiene stages that make the printed laboratory output binding: bidirectional `=== CHAPTER ` parity for `README.md`, Appendix B and the nine framework chapters (each documented block must be one contiguous run of CLI output), the frozen cross-domain audit report, `book-src/lab/index.html` trajectory and multilayer-node parity, the `data/*.csv` loader inventory, `PYSYM` symbol parity (`Type::member` members, every backticked bare identifier against the implementation surface, and every declaration-window link), root-document link resolution, canonical provenance tags, `PYKATEX` hygiene, and relative-path discipline.
     - **Writing a declaration-window link.** A link carries a backticked symbol in square brackets, then in parentheses the repository-relative file, a `#L` fragment and the line window — the shape shown by the illustrative `` [`Symbol`](crates/...#L10-L20) `` above. The cited file is opened, the window must be well formed, and the declaration must lie inside it. A path whose segment is written literally as `...` is read as an illustrative example and skipped without opening anything, which is exactly why the example above is safe. Every other path must exist: a missing target, an inverted window (`#L20-L10`), or a window running past the end of the file fails the gate. Nothing inside a fenced code block is resolved at all, and a citation wrapped in its own inline code span is rendered as literal text, so both of those are safe places to document the syntax.
- **Grammar Parity (`tests/grammar_spec.rs`):** `cargo test` also parses `notes/The Grammar of Aim.md`, resolves its `grammar-anchors` block against the sources, and requires the recognizer verdict to agree with `TrajectoryReport::triadic_integrity.diagnose` across all eight cells of the $2\times 2\times 2$ cube. The note and `grammar.rs` are not permitted to drift apart.
- **Multilayer Graph Parity (`tests/multilayer_spec.rs`):** `cargo test` parses `notes/The Multilayer Graph of Knowledge and Aim.md`, resolves its `multilayer-anchors` block against the sources, verifies the $\binom{7}{3} = 35$ bijection ($\mathcal{G}_1 \leftrightarrow \mathcal{G}_2$), the 1-to-1 bijection between `GuardSite` (`T1`–`T7`) and `KnowledgeStratum` (`S1`–`S7`), the $7 \times 5 = 35$ primary guard site balance, and the 1-to-1 alignment with Part V chapters `51`–`85`.
- **Human Review Protection:** Modifying `data/SHA256SUMS` or `notes/The Ethics of Aim.md` requires explicit human confirmation.
