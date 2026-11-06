<p align="center">
  <img src="book-src/logo.svg" alt="Stochology Emblem" />
</p>

# Stochology — The Ethics of Aim (*Stókhos*) and Judgment under Uncertainty

**Stochology** (from Greek **στόχος, *stókhos***, aim or conjecture, and **λόγος, *lógos***, reasoned account) is an integrative framework and transversal research program for the theory and ethics of aim (*stókhos*, from Plato's *Philebus* 55e–56c, Hippocratic *sēmeîa*, *kairós*, and *mētis*, Galenic *stokhastikē tékhnē*, and Antipater of Tarsus's archer to Jakob Bernoulli's *Ars Conjectandi*) in judgment and decision-making under structural uncertainty when action must precede the outcome. Rather than claiming to be a standalone positive science of mechanical measure (*metrētikē* / *akríbeia*), Stochology investigates an indivisible unit of analysis that specialized fields habitually sever: the **full mediation circuit across world, archive, and institution**.

Emerging from the three foundational ethical refusals of [`notes/Aiming Between Mirrors.md`](notes/Aiming%20Between%20Mirrors.md), formalized in [`notes/The Ethics of Aim.md`](notes/The%20Ethics%20of%20Aim.md), given its formal syntax layer in [`notes/The Grammar of Aim.md`](notes/The%20Grammar%20of%20Aim.md) ([`grammar.rs`](crates/stochology/src/grammar.rs)), its pre-shot reflexive core in [`notes/What Must Be Known When We Do Not Know.md`](notes/What%20Must%20Be%20Known%20When%20We%20Do%20Not%20Know.md), and its 35-layer disciplinary topology in [`notes/The Multilayer Graph of Knowledge and Aim.md`](notes/The%20Multilayer%20Graph%20of%20Knowledge%20and%20Aim.md) ([`multilayer.rs`](crates/stochology/src/multilayer.rs)), this repository implements the complete research program in **pure Rust (`std` only, zero external Cargo dependencies; IEEE 754 floating-point elementary functions `exp`, `ln`, and `sqrt` delegate to the toolchain's `libm` within 1–2 ULP as documented in [`kernel.rs`](crates/stochology/src/kernel.rs) and [`rust-toolchain.toml`](rust-toolchain.toml))**, organized 1-to-1 with its 86-chapter (`00`–`85`) five-part textbook (`book-src/`) and interactive browser simulator (`book-src/lab/index.html`) around the **Sequential Agent–World–Institution Loop** (`CommunityReserveEnv`).

---

## 1. The Nuclear Triad, Three Collapse Propositions, and Three Canonical Operators

Stochology unites three historical lineages—**Epistemological/Statistical**, **Physical/Cybernetic**, and **Critical/Ethical**—into a single **Nuclear Triad** of safeguards that are strictly non-substitutable whenever an absorbing ruin barrier or severe archival/institutional defect is active ($\sigma \to 0^+$), whereas partial compensation is possible in interior mild-shock regimes with doubled physical reserve buffers (`CompensatoryBufferAgent`, the regime boundary explicitly tested and refuted by Falsifier `F4` in Chapter 50 / Lab 9):

$$
\mathcal{T}_{\mathrm{Stochology}} = \bigl(\text{Epistemic Margin},\; \text{Vital Margin},\; \text{Institutional Governance}\bigr), \qquad M_{\mathrm{Leontief}} = \min\bigl(S_{\mathrm{epi}},\, S_{\mathrm{vital}},\, S_{\mathrm{gov}}\bigr)
$$

Each axis is governed by a **Canonical Mathematical Operator** (`EpistemicMarginState`, `VitalMarginState`, `GovernanceState`) and a corresponding **Collapse Proposition** when removed:

| Axis | Canonical Operator & Rust Type | Missing Axis Pathology |
| :--- | :--- | :--- |
| **Axis 1** ($S_{\mathrm{epi}}$, Filter I: Ch. 43–45 / Labs 2–4) | **Operator I — Partial-Identification Set & Effective Sample Contraction:** $\mathcal{I}_{\mathrm{epi}}(\mathcal{D}_t; \alpha)$, $\frac{1}{n_{\mathrm{eff}}} = \frac{\mathrm{DEFF}}{n} + \left(\frac{1-f}{f}\right)\rho_{R,Y}^2$, $V = \frac{n_{\mathrm{eff}}}{n_{\mathrm{eff}} + n_0}$ ([`EpistemicMarginState`](crates/stochology/src/epistemic.rs)) | **Proposition 1 — Blind Undersizing & Net Reserve Erosion** ($\neg S_{\mathrm{epi}}$): Reserves are sized on a false point certainty that ignores non-random missingness ($\rho_{R,Y} \ne 0$) and cluster echoes ($\mathrm{DEFF}$), collapsing coverage to $25.0\%$ (`0.0%` on tail shocks) and eroding reserves below $R_0$ ($R_{24} = 27.21 < R_0 = 45.0$, `[Simulated]`). |
| **Axis 2** ($S_{\mathrm{vital}}$, Filter II: Ch. 46–47 / Labs 5–6) | **Operator II — Non-Ergodic Reserve & Survival-Time Barrier:** $dR_t = \bigl(u_t - D_t(\alpha_{\mathrm{Hill}}) - \Delta_J - \frac{1}{2}f_t^2\sigma^2\bigr)\,dt$ & $\mathrm{TTS}_{\mathrm{survival}} \ge \mathrm{TTR}_{\mathrm{recovery}}$ ([`VitalMarginState`](crates/stochology/src/vital.rs)) | **Proposition 2 — Impotent Lucidity Before Ruin** ($\neg S_{\mathrm{vital}}$): The analyst knows the exact width of uncertainty, yet the system operates at 96–100% utilization (`[Assumed]`) or excess leverage with $\mathrm{TTS}_{\mathrm{survival}} < \mathrm{TTR}_{\mathrm{recovery}}$ and is absorbed at the ruin barrier. |
| **Axis 3** ($S_{\mathrm{gov}}$, Filter III: Ch. 48–49 / Labs 7–8) | **Operator III — Bitemporal Accountability & Performative Integrity Gate:** $S_{\mathrm{gov}} = \min\bigl(g_{\mathrm{ex\text{-}ante}},\, g_{\mathrm{kairos}},\, g_{\mathrm{ex\text{-}post}}\bigr)$ ([`GovernanceState`](crates/stochology/src/governance.rs) & [`TemporalVault`](crates/stochology/src/governance.rs)) | **Proposition 3 — Cynical Capture and Pillaging** ($\neg S_{\mathrm{gov}}$): Epistemic doubt is weaponized as an alibi for paralysis while common vital reserves are pillaged under limited-liability moral hazard, proxy gaming, and testimonial discounting of frontline warnings. |

---

## 2. The Seven Guard Sites (`3 + 3 + 1 = 7`) and the 35-Layer Multilayer Knowledge Graph ($\mathcal{G}_1 \leftrightarrow \mathcal{G}_2$)

When first-order facts about the external world (*skopós*) are unknown, the infinite regress of higher-order uncertainty terminates only on **reflexive invariants of the agent's own mediation circuit (*télos*)** ([`notes/What Must Be Known When We Do Not Know.md`](notes/What%20Must%20Be%20Known%20When%20We%20Do%20Not%20Know.md)). Three Trunk Questions unfold into the **Seven Guard Sites (`GuardSite`, `3/2/2`)** of [`grammar.rs`](crates/stochology/src/grammar.rs), which form an exact **$3 + 3 + 1 = 7$ bijection** onto the directed transits (`CircuitTransit`) across the three planes (`World`, `Archive`, `Institution`) and a **1-to-1 bijection** onto the **Seven Ontological Strata (`KnowledgeStratum`, `S1`–`S7`)** of [`multilayer.rs`](crates/stochology/src/multilayer.rs):

| Guard Site (`grammar.rs`) | Triad Axis (`3/2/2`) | Directed Circuit Transit (`CircuitTransit`) | Primary Plane (`2/2/3`) | Canonical Knowledge Stratum (`S1`–`S7`) | Primary Bottleneck Orders ($7 \times 5 = 35$) | Governing Rust Operator |
| :--- | :---: | :--- | :---: | :--- | :--- | :--- |
| **`T1Framing`** | $S_{\mathrm{epi}}$ | `Institution -> Archive` *(Reflexive Lock 1)* | `Institution` | `S2` (`S2FormalMetrological`) | `L06, L15, L20, L24, L30` (5) | `EpistemicMarginState::from_inquiry_framing` |
| **`T2Severity`** | $S_{\mathrm{epi}}$ | `Archive -> World` *(Reflexive Lock 2)* | `World` | `S4` (`S4ClinicalFormative`) | `L09, L11, L18, L27, L31` (5) | `EpistemicMarginState::from_ate_bounds` |
| **`T3Archive`** | $S_{\mathrm{epi}}$ | `World -> Archive` *(Direct Transit 1)* | `Archive` | `S5` (`S5SemioticArchival`) | `L04, L08, L17, L22, L29` (5) | `EpistemicMarginState::from_archive_clusters` |
| **`T4Formalization`** | $S_{\mathrm{vital}}$ | `Archive -> Institution` *(Direct Transit 2)* | `Archive` | `S3` (`S3LivingEvolutionary`) | `L05, L07, L14, L16, L34` (5) | `VitalMarginState::from_heavy_tails_and_asymmetry` |
| **`T5Temporal`** | $S_{\mathrm{vital}}$ | `World -> Institution` *(Reflexive Lock 3)* | `World` | `S1` (`S1PhysicalDynamical`) | `L01, L12, L23, L26, L33` (5) | `VitalMarginState::from_queue_basin_and_survival` |
| **`T6Performative`** | $S_{\mathrm{gov}}$ | `Institution -> World` *(Direct Transit 3)* | `Institution` | `S6` (`S6StrategicEconomic`) | `L03, L19, L25, L28, L35` (5) | `GovernanceState::from_ex_ante_gate` |
| **`T7Liability`** | $S_{\mathrm{gov}}$ | `World <-> Archive <-> Institution` *(Integral)* | `Institution` | `S7` (`S7ConstitutionalBiospheric`) | `L02, L10, L13, L21, L32` (5) | `GovernanceState::from_real_time_and_ex_post_gates`, `TemporalVault` |

Taking all $\binom{7}{3} = 35$ unique 3-site combinations of `{T1, ..., T7}` induces a bijective **35-layer Directed Acyclic Graph of Human Knowledge ($\mathcal{G}_1$, `MultilayerGraph1`)** and its dual **Stochological Guard Graph ($\mathcal{G}_2$, `MultilayerGraph2`)**, mapped 1-to-1 to Part V Chapters `51`–`85` (`KnowledgeOrder::L01MicroKinetic` through `L35PolycentricCommonsBiospheric`):
- **Horizontal Stratification ($7 \times 5 = 35$):** 7 Ontological Strata (`S1`–`S7`) of 5 Orders each; within every stratum, the union of guard triples covers all 7 `GuardSite`s (`KnowledgeStratum::covers_all_seven_guard_sites`) and the 5 orders carry 5 pairwise-distinct primary guard sites (`KnowledgeStratum::has_distinct_primary_guard_sites`).
- **Vertical Exposure Stratification ($12 + 9 + 9 + 4 + 1 = 35$):** By Vandermonde's identity over the `3/2/2` partition, $23/35$ disciplines omit at least one axis of the Nuclear Triad locally (`TriadicExposureRegime`: `GovernanceBlindSpot` [9], `VitalBlindSpot` [9], `EpistemicBlindSpot` [4], `PureEpistemicConcentration` [1]) and suffer a predictable structural collapse unless coupled across their ancestor cone $\mathrm{Anc}^*(L_k)$ via the **Cumulative Leontief Homomorphism** (`MultilayerHomomorphism::evaluate`).

---

## 3. 1-to-1 Architecture: Notes ↔ Rust Crates ↔ Five-Part Textbook (`00`–`85`)

The workspace contains two pure-Rust crates and five foundational notes:
- [`crates/stochology`](crates/stochology): core mathematical, physical, institutional, syntactic, and multilayer graph library (`std` only: [`lib.rs`](crates/stochology/src/lib.rs), [`kernel.rs`](crates/stochology/src/kernel.rs), [`engine.rs`](crates/stochology/src/engine.rs), [`epistemic.rs`](crates/stochology/src/epistemic.rs), [`vital.rs`](crates/stochology/src/vital.rs), [`governance.rs`](crates/stochology/src/governance.rs), [`synthesis.rs`](crates/stochology/src/synthesis.rs), [`grammar.rs`](crates/stochology/src/grammar.rs), [`multilayer.rs`](crates/stochology/src/multilayer.rs)).
- [`crates/stochology-cli`](crates/stochology-cli): CLI runner (`stochology`) verifying [`data/SHA256SUMS`](data/SHA256SUMS) and executing the 9 progressive chapter laboratories.
- **Foundational Notes (`notes/`):**
  1. [`notes/Aiming Between Mirrors.md`](notes/Aiming%20Between%20Mirrors.md) — Ethical and phenomenological refusals;
  2. [`notes/The Ethics of Aim.md`](notes/The%20Ethics%20of%20Aim.md) — Programmatic treatise (three-plane circuit, Nuclear Triad, 3 Collapse Propositions, 7 Tensions, 4 Reflexive Brakes, 4 Falsifiers);
  3. [`notes/The Grammar of Aim.md`](notes/The%20Grammar%20of%20Aim.md) — Formal syntax layer (`grammar.rs`, frozen by `tests/grammar_spec.rs`);
  4. [`notes/What Must Be Known When We Do Not Know.md`](notes/What%20Must%20Be%20Known%20When%20We%20Do%20Not%20Know.md) — Reflexive termination of higher-order uncertainty (`3` Trunk Questions $\to$ `7` Guard Sites and `3` Regimes of Action);
  5. [`notes/The Multilayer Graph of Knowledge and Aim.md`](notes/The%20Multilayer%20Graph%20of%20Knowledge%20and%20Aim.md) — Bijective $\binom{7}{3} = 35$ multilayer knowledge graph ($\mathcal{G}_1 \leftrightarrow \mathcal{G}_2$, frozen by `tests/multilayer_spec.rs`).

The textbook (`book-src/`, verified by `tests/book_anchors.rs`) spans **86 chapters (`00`–`85`) + 3 Appendices** in five parts:
- **Preface (`00`) & Part I — Thinking Under Uncertainty (`01`–`08`):** Intuitive entry into deciding without knowing, averages vs. tails, sample volume vs. bias, outcome luck, and non-substitutable margins.
- **Part II — The Bases (`09`–`26`):** Mathematical foundations (`09`–`15`), Philosophy & Epistemology (`16`–`20`), and Natural Science (`21`–`26`).
- **Part III — Foundations of Judgment (`27`–`41`):** Aim across the three planes, probability, distributions, framing, measurement, sampling, inference, decision thresholds, time, causality, institutions, and the Leontief minimum.
- **Part IV — The Framework (`42`–`50`, CLI Laboratories 1–9):**

| Ch. | Lab | Textbook (`book-src/`) | Contiguous Triad Axis & Treatise (`notes/The Ethics of Aim.md`) | Module (`crates/stochology/src/`) | Empirical Datasets (`data/*.csv`) |
| :-: | :-: | :--- | :--- | :--- | :--- |
| **00** | — | [`00-preface.md`](book-src/00-preface.md) | §I–§II (Genealogy, Nuclear Triad, 3 Collapse Propositions, 3 Canonical Operators) | [`lib.rs`](crates/stochology/src/lib.rs) | — |
| **42** | **1** | [`42-sequential-loop-naive-agent.md`](book-src/42-sequential-loop-naive-agent.md) | §II.2 (Sequential Mediation Loop) | [`kernel.rs`](crates/stochology/src/kernel.rs), [`engine.rs`](crates/stochology/src/engine.rs) | — |
| **43** | **2** | [`43-inquiry-openness-multiverse.md`](book-src/43-inquiry-openness-multiverse.md) | §III.1 (**Filter I · Axis 1** ($S_{\mathrm{epi}}$, `T1Framing`): Framing, Cromwell, Agnotology vs. Severity, Type III, *e-BH*, Multiverse) | [`epistemic.rs`](crates/stochology/src/epistemic.rs) | — |
| **44** | **3** | [`44-underdetermination-partial-identification.md`](book-src/44-underdetermination-partial-identification.md) | §III.2 (**Filter I · Axis 1** ($S_{\mathrm{epi}}$, `T2Severity`): *e-processes*, Lakatos, Two-Arm Manski & Lee ATE Bounds, Peterson) | [`epistemic.rs`](crates/stochology/src/epistemic.rs) | `nsw_experimental.csv`, `cps_observational.csv` |
| **45** | **4** | [`45-world-to-archive-big-data-paradox.md`](book-src/45-world-to-archive-big-data-paradox.md) | §III.3 (**Filter I · Axis 1** ($S_{\mathrm{epi}}$, `T3Archive`): Keynes $V$, Meng $\rho_{R,Y}$, Campbell–Moulton $\mathrm{DEFF}$, Kish, Chao $\hat{n}_{00}$, Bitemporal Vintages) | [`epistemic.rs`](crates/stochology/src/epistemic.rs) | `cps_observational.csv`, `bank_calls.csv`, `rtdsm_routput_vintages.csv` |
| **46** | **5** | [`46-non-identical-asymmetry-heavy-tails.md`](book-src/46-non-identical-asymmetry-heavy-tails.md) | §IV.1 (**Filter II · Axis 2** ($S_{\mathrm{vital}}$, `T4Formalization`): Dialectical Penumbra, Ordinal Invariance, Jensen $\Delta_J$, CRPS, Hill $\hat{\alpha}$) | [`vital.rs`](crates/stochology/src/vital.rs) | `danish_fire.csv`, `nyc_flights_2013.csv`, `nyc_flights_2023.csv`, `nyc_flights_2013_h1.csv`, `nyc_flights_2023_h1.csv`, `houston_flights_2011.csv` |
| **47** | **6** | [`47-irreversible-time-vital-margin.md`](book-src/47-irreversible-time-vital-margin.md) | §IV.2 (**Filter II · Axis 2** ($S_{\mathrm{vital}}$, `T5Temporal`): Itô Drain, Baker–McHale Kelly, Canguilhem–Holling, Kingman, $\mathrm{TTS}_{\mathrm{survival}} \ge \mathrm{TTR}_{\mathrm{recovery}}$, Wald, Sniedovich) | [`vital.rs`](crates/stochology/src/vital.rs) | `djia_1885_2023.csv`, `vic_elec.csv`, `nsw_experimental.csv` |
| **48** | **7** | [`48-reactive-worlds-performativity-goodhart.md`](book-src/48-reactive-worlds-performativity-goodhart.md) | §V.1 (**Filter III · Axis 3** ($S_{\mathrm{gov}}$, `T6Performative`): CORP-PAV, ICP, Kennedy–O'Hagan GP, Tian–Pearl, Perdomo/MacKenzie, Goodhart) | [`governance.rs`](crates/stochology/src/governance.rs) | `atlantic_storms.csv` |
| **49** | **8** | [`49-moral-luck-apt-agency-institutions.md`](book-src/49-moral-luck-apt-agency-institutions.md) | §V.2–§VI (**Filter III · Axis 3** ($S_{\mathrm{gov}}$, `T7Liability`): Sosa/Baron–Hershey, List–Pettit, Fricker/Beer Algedonic, Holmström–Innes, Chouldechova, Zheng–Meister, `TemporalVault`) | [`governance.rs`](crates/stochology/src/governance.rs) | `toronto_arrests.csv` |
| **50** | **9** | [`50-methodological-families-reflexivity.md`](book-src/50-methodological-families-reflexivity.md) | §VII–§VIII (**Triadic Synthesis** ($M_{\mathrm{Leontief}}$): 5 Methodological Families, Process Tracing, 4 Reflexive Brakes, 4 Refutability Conditions) | [`synthesis.rs`](crates/stochology/src/synthesis.rs) | `atlantic_storms.csv` |

- **Part V — The 35 Orders of Knowledge (`51`–`85`, `multilayer.rs`):**
  - **Stratum I (`S1PhysicalDynamical`, `51`–`55`, anchor `T5Temporal`):** `L01` Micro-Kinetic & Sensorimotor (`51`), `L02` Meso-Buffered Queue & Dispatch (`52`), `L03` Macro-Coupled Network Contagion (`53`), `L04` Geophysical Chaos & Data Assimilation (`54`), `L05` Continuum Fracture & Structural Engineering (`55`).
  - **Stratum II (`S2FormalMetrological`, `56`–`60`, anchor `T1Framing`):** `L06` Formal Logic, Computability & Linear Types (`56`), `L07` Probability, Imprecise Credences & Measurement (`57`), `L08` Metrological & Information-Geometric (`58`), `L09` Experimental Design, Survey Sampling & Causal Inference (`59`), `L10` Cryptographic, Zero-Knowledge & Adversarial Verification (`60`).
  - **Stratum III (`S3LivingEvolutionary`, `61`–`65`, anchor `T4Formalization`):** `L11` Biochemical Kinetics & Molecular Proofreading (`61`), `L12` Morphogenetic & Developmental Canalization (`62`), `L13` Immunological Self-Governance (`63`), `L14` Taxonomy, Systematics & Formal Ontology (`64`), `L15` Stratigraphy, Paleobiology & Deep Time (`65`).
  - **Stratum IV (`S4ClinicalFormative`, `66`–`70`, anchor `T2Severity`):** `L16` Pharmacological & Clinical Diagnostic (`66`), `L17` Epidemiology & Outbreak Surveillance (`67`), `L18` Pharmacovigilance & Regulatory Toxicology (`68`), `L19` Metascience, Scientometrics & Replication (`69`), `L20` Acoustic, Aesthetic, Tragic & Pedagogical (`70`).
  - **Stratum V (`S5SemioticArchival`, `71`–`75`, anchor `T3Archive`):** `L21` Linguistic Evidentiality & Organizational Pragmatics (`71`), `L22` Philological, Epigraphic & Textual Criticism (`72`), `L23` Taphonomic, Archaeological & Historiographical (`73`), `L24` Critical Archival & Transitional Justice (`74`), `L25` Media Ecology & Information Retrieval (`75`).
  - **Stratum VI (`S6StrategicEconomic`, `76`–`80`, anchor `T6Performative`):** `L26` Sequential Decision, Bandits & Real Options (`76`), `L27` Game Theory & Strategic Classification (`77`), `L28` Macro-Finance, Econometrics & Actuarial Ruin (`78`), `L29` Official Statistics & Forensic Accounting (`79`), `L30` Corporate Governance & Principal-Agent Design (`80`).
  - **Stratum VII (`S7ConstitutionalBiospheric`, `81`–`85`, anchor `T7Liability`):** `L31` Criminal Proof & Forensic Epistemology (`81`), `L32` Evidentiary, Tort & Constitutional Law (`82`), `L33` Distributed Consensus & SRE (`83`), `L34` High-Hazard Nuclear & Hydraulic Safety (`84`), `L35` Polycentric Commons & Biospheric Stewardship (`85`).
- **Appendices:** [`appendix-a-mathematics.md`](book-src/appendix-a-mathematics.md), [`appendix-b-architecture-verification.md`](book-src/appendix-b-architecture-verification.md), and [`appendix-c-sources.md`](book-src/appendix-c-sources.md).

---

## 4. Quick Start & Verification

Run the complete test suite and all 9 chapter laboratories:

```bash
./scripts/verify.sh
```

Or run the CLI directly:

```bash
cargo run --release --bin stochology
cargo run --release --bin stochology -- --chapter 1
cargo run --release --bin stochology -- --export-lab-json
cargo run --release --bin stochology -- --audit-report
```

The last command emits the frozen cross-domain audit report — the latent world-model exploitation audit, the friction-envelope control benchmark, the social-network topology battery and the model-drift fallback audit — verified verbatim against [Appendix B](book-src/appendix-b-architecture-verification.md) §B.6 by `scripts/verify.sh`.

Sample output (`cargo run --release --bin stochology`, combining `[Measured]` empirical datasets, `[Simulated]` `SplitMix64` benchmarks, `[Assumed]` analytical inputs, and `[Literature]` reference values):

```text
[SEAL VERIFIED] All 14 empirical datasets in data/SHA256SUMS match their SHA-256 digests and 14 upstream schema contracts.
=== CHAPTER 1: THE SEQUENTIAL LOOP & THE NAIVE AGENT ===
  Environment [Simulated]: CommunityReserveEnv (seed = 2026, horizon = 24 shifts, R_0 = 45.0)
  NaiveOptimizer          -> coverage =  16.7%, mean_harm = 125.83, terminal_reserve =   0.00, ruin_step = Some(2)
  Prop 1 Degrad (No E-Mar)-> coverage =  25.0%, mean_harm =   4.49, terminal_reserve =  27.21, ruin_step = None
  Collapse 2 (No V-Margin)-> coverage =  95.8%, mean_harm =  77.19, terminal_reserve =   0.00, ruin_step = Some(10)
  Collapse 3 (No Gov.)    -> coverage =  95.8%, mean_harm =  68.01, terminal_reserve =   0.00, ruin_step = Some(12)
  StochologicalAgent(Full)-> coverage =  95.8% (shock_cov =  75.0%), mean_harm =   1.59, terminal_reserve =  58.17, ruin_step = None
=== CHAPTER 2: FILTER I (S_EPI) — INQUIRY, OPENNESS & MULTIVERSE ===
  Cost-Loss Thresholds [Assumed]: institutional p* = 0.588 (EVPI=12.60) vs affected-citizen p* = 0.147 (EVPI=82.00), gap = 0.441
  Mayo Severe Power [Assumed]: 1.000 | Type III Error: true (global_net=-65.0) | Simon Framing: StrategicPremiseRigging vs CommunicativeInfluence (PRI = 0.864 vs 0.000, threshold = 0.20)
  Multiverse & e-BH [Assumed]: consistency = 83.3%, interval = [1.38, 1.61], e-BH discoveries = 5
=== CHAPTER 3: FILTER I (S_EPI) — UNDERDETERMINATION & PARTIAL IDENTIFICATION ===
  LaLonde Benchmark [Measured] (data/nsw_experimental.csv vs data/cps_observational.csv):
    Experimental RCT ATE [Measured, zero survey attrition] = $1794 (n_t=185, n_c=260) vs Naive CPS Diff = $-8498 (n_cps=15992)
    Lee (2009) Positive-Wage Selection Bounds [Measured, re78>0: 140/185 vs 168/260]: [$-1142, $2621] (width=$3763)
    Manski Worst-Case Two-Arm ATE Bounds [Assumed: 15/200 & 30/290 attrition]: [$-487, $3439] (width=$3926, Imbens-Manski 95% CI=[$-1639, $4568])
    Lee Monotone Attrition Two-Arm ATE Bounds [Assumed: 15/200 & 30/290 attrition]: [$872, $1996] (width=$1123)
    Censoring worst-case survival bounds [Assumed]: [0.750, 0.860] (KM naive=0.843)
    Peterson (1976) competing-risk Frechet bounds [Assumed]: [0.750, 0.860] (indep=0.765, width=0.110)
    Ville e-Process [Assumed]: terminal E_T = 2164.62 (33.4 decibans), anytime CI = [-0.03, 2.75]
=== CHAPTER 4: FILTER I (S_EPI) — FROM WORLD TO ARCHIVE & THE BIG DATA PARADOX ===
  Canonical Operator I (Harmonic n_eff & Keynes Weight V = n_eff / (n_eff + n_0)):
  1. Meng Big Data Paradox [Assumed] (N = 10M, n = 2.3M, rho_RY = -0.05): n_eff = 119.5, meng_error = -0.0457, V = 0.046
  2. Demographic Cluster DEFF on real re78 [Measured] (data/cps_observational.csv, [Assumed n_0=25.565]): DEFF = 10.37, n_eff = 1542.0, V = 0.984
  3. Temporal Cohort DEFF [Measured] (data/bank_calls.csv, [Assumed n_0=100.0]): DEFF = 8.19, n_eff = 3384.5, V = 0.971
  4. Kish Weighting Floor [Assumed]: DEFF_Kish = 2.78, n_eff_floor = 360.0 | Chao Unseen Dark Archive [Assumed]: n00_hat = 751.6, N_Chao_LB = 2237.1, U_Chao_95 = 2719.9
  5. Bitemporal Vintages [Measured] (data/rtdsm_routput_vintages.csv, Q=241 evaluated quarters across 244 vintages): mean_rev = 1.47 pp, max_rev = 8.02 pp, sign_flips = 6.6% (causal_view_len=1)
=== CHAPTER 5: FILTER II (S_VITAL) — ASYMMETRY, SCALE INVARIANCE & HEAVY TAILS ===
  Danish Fire Losses [Measured] (data/danish_fire.csv, n = 2167):
    Hill alpha_hat = 1.39 (k=150, 95% CI=[1.17, 1.61], k-stability [50..250] = [1.36, 1.87], regime = FiniteMeanInfiniteVariance, median = 1.78, q95 = 9.97)
    Jensen Gap Delta_J = E[phi(X)] - phi(E[X]) = 1.015 (E[phi(X)] = 1.613 vs phi(E[X]) = 0.598)
    Empirical CRPS = 1.458 | Ordinal Invariance Error = 0.0000 (Mean Ranking Reversed = true) | Penumbra Mass = 0.396
    Hill-Holling DEFF_tail Coupling [Simulated] (DEFF=10.1, k_eff=11.9): P_true=0.0166, E[P_plugin]=0.0219 (Jensen +32.3%), P_debiased=0.0119, CI90=[0.00083, 0.06704] (80.3x)
  NYC Flights Conditional Delay>0 Tail Index [Measured, k=n+/20 on non-NA rows]: 2013 alpha_hat = 2.89 (95% CI=[2.78, 3.00], k=2616, n=138375, n+=52330, total_rows=141193, skipped_NA=2818, penumbra_[10,30]=12.7%) vs 2023 alpha_hat = 2.27 (95% CI=[2.18, 2.36], k=2430, n=173566, n+=48602, total_rows=176739, skipped_NA=3173, penumbra_[10,30]=8.6%) | OOT H1 (2013=3.13 [k=4033, n=188971, n+=80674, total=195583, NA=6612], 2023=2.39 [k=4648, n=249252, n+=92972, total=258613, NA=9361]) & OOD Houston 2011=2.41 (k=5346, n=223874, n+=106920, total=224591, NA=717)
=== CHAPTER 6: FILTER II (S_VITAL) — IRREVERSIBLE TIME, NON-ERGODICITY & VITAL MARGIN ===
  Canonical Operator II (Convexity, Time-Average Growth & Survival Inequality TTS_survival >= TTR_recovery):
  1. DJIA Non-Ergodic Trajectory [Measured] (data/djia_1885_2023.csv, T = 37930 trading days, emp_alpha_loss = 2.83):
     Arithmetic Ensemble Drift = +0.000243/day vs Time-Average Growth = +0.000186/day (Itô drain = 23.6%)
     Plug-in Kelly = 2.12 vs Baker-McHale Epistemic Kelly = 1.99 (Shrinkage = 0.9416)
     Heavy-Tail Kelly Regime Selector [Assumed] (alpha=1.65 <= 2.2): regime = TailRobustWdroCvarKelly, naive_quadratic_f = 1.0000 vs WDRO_CVaR95_f = 0.0314
  2. Victoria Electricity Grid [Measured] (data/vic_elec.csv, n = 52608 half-hours, q50=4635 MW, q95=6106 MW, q99=6981 MW, max=9345 MW, ramp_p99=897 MW/h, scale=1.18/1.05 [Assumed]):
     Stressed [Measured quantiles + Assumed ODE h=1.80,K=10] (rho=96.9%): Kingman Wait = 31.2x (G/G/1 E[W_q]=16.17h, var_factor=0.52, rho1_worn=0.732), Basin D_t/D_0 = 0.46, TTS_survival = 2.6h < TTR_recovery = 8.0h (survives=false)
     Buffered [Measured quantiles + Assumed ODE h=1.80,K=10] (rho=47.2%): Kingman Wait = 0.9x (G/G/1 E[W_q]=0.46h, var_factor=0.52, rho1_worn=0.593), Basin D_t/D_0 = 0.92, TTS_survival = 16.5h >= TTR_recovery = 4.8h (survives=true)
  3. Survivorship & Robustness: Wald Latent 2x2 Solver [Assumed] = true (reconstructed M_A=18, M_B=210 -> Zone B 84.0% > Zone A 7.2%) | Empirical NSW Survivor Bias [Measured] (re78>0 only): full=$1794 vs survivor=$1341 (bias=$-453) | Sniedovich Mirage [Assumed] = true
=== CHAPTER 7: FILTER III (S_GOV) — REACTIVE WORLDS, PERFORMATIVITY & GOODHART ===
  Canonical Operator III — Gate 1 (Ex-Ante Calibration, Causal Invariance & Performative Gaming Guard):
  1. CORP-PAV +6h Lead-Time Storm Forecast Brier Decomposition [Measured] (data/atlantic_storms.csv, n_transitions = 19025 from 20778 total rows, 1753 track-terminal rows [1752 unpaired + 1 tail]):
     Total Brier = 0.0731 = REL (0.0028) - RES (0.1171) + UNC (0.1873) [closure_err = 4.62e-15]
     Finite-Sample CORP-PAV & ICP Guard [Measured]: b_PAV_CV = 0.0002, REL_debiased = 0.0026, ICP(n_eff/env=184.5) = IdentifiedInvariantSet (benchmark n_eff/env=30 -> EpistemicUnderpoweredSet [Assumed])
  2. Multi-Regime Invariant Causal Prediction (ICP) [Measured]: physical slope range = 0.009 < spurious proxy range = 0.076 (accepted_subsets = 2, S_hat_mask = 0b01)
  3. Kennedy-O'Hagan GP Discrepancy [Measured tercile anchors]: in_sample (mean=-0.0283, std=0.044) vs out_of_sample (mean=-0.0000, std=0.500) | Tian-Pearl [Assumed] PN=[0.75, 1.00], PS=[0.50, 0.75], PNS=[0.50, 0.70]
  4. Perdomo Performative Stability vs Optimality [Assumed]: theta_PS = 1.333 (risk=0.263) vs theta_PO = 1.095 (risk=0.249, MacKenzie = BarnesianSelfFulfilling)
  5. Goodhart Metric Collapse [Assumed]: collapsed = true, variant = Adversarial
=== CHAPTER 8: FILTER III (S_GOV) — MORAL LUCK, APT AGENCY & INSTITUTIONAL DESIGN ===
  Canonical Operator III — Gate 2 (Real-Time S3* Algedonic Veto) & Gate 3 (Ex-Post Responsibility):
  1. Sosa / Baron-Hershey Telic Evaluation [Assumed]: grade = AdroitExAnteUnluckyOutcome, eliminated_outcome_bias = +0.45, vault_confirmatory = true (tamper_rejected=true, replay_rejected=true)
  2. List-Pettit & Beer S3* Algedonic Channel [Assumed]: premise_majority = true, conjunctive = false, discursive_dilemma = true
     Undiscounted TTR_recovery = 5 <= TTS_survival = 8 vs Discounted TTR_recovery = 12 > TTS_survival = 8 (fatal_delay=true)
  3. Holmstrom-Innes Liability & Simon-Girard Scapegoating [Assumed]: structural_share = 83.3%, scapegoat_transfer = 78.7% (blame_inversion_ratio = 13.6x), Bateson DB_t = 0.51 (brake_on = 0.00)
  4. Chouldechova Fairness Trilemma [Measured] (data/toronto_arrests.csv, n_all = 5226, n_matched = 891, n_poles = 3577): full marginal prev_Black = 0.259, prev_White = 0.142, odds_FPR_ratio = 2.11x | matched (Unemp+Citizen) prev_Black = 0.354, prev_White = 0.263, odds_FPR_ratio = 1.54x | intersectional poles prev_A = 0.393 (FPR_A=0.278), prev_B = 0.112 (FPR_B=0.054), FPR_A/FPR_B = 5.13x
     Empirical Custody-Risk Classifier [Assumed rule checks>=3 on Measured data/toronto_arrests.csv]: PPV_B=0.365/PPV_W=0.267, FNR_B=0.324/FNR_W=0.438, FPR_Black=0.409 vs FPR_White=0.256 (empirical FPR_ratio=1.60x, Chouldechova identity closure_err=1.11e-16)
  5. Zheng-Meister 10 bits/s & Gigerenzer Natural Frequencies [Assumed]: noisy FDR = 93.9% -> filtered PPV = 52.7% (16 true alerts vs 15 false alarms / 1,000 shifts)
     Two-Tier Cognitive Queue (TTS=45s): Kairos(30b,6s) rho=0.12, TTR=18.5s (ok=true) | Card(120b,18s) rho=0.48, TTR=40.8s | Full(240b,36s) rho=0.96, TTR=759.0s (ok=false)
=== CHAPTER 9: METHODOLOGICAL FAMILIES, PROCESS TRACING & REFLEXIVITY ===
  2x2x2 Factorial Ablation of the Nuclear Triad [Simulated] (M_Leontief = min(S_epi, S_vital, S_gov)):
    [E=1 V=1 G=1] StochologicalAgent(FullTriad:111)            -> M_Leontief=0.814 (M_arith=0.895), collapse=IntactTriad                 , cov= 95.8% (shock= 75.0%), harm=  1.59 (MC100=  1.55±0.01 [  1.44,   1.69], ruin=  0.0%), R_24= 58.17
    [E=0 V=1 G=1] DefectiveAgent(NoEpistemicMargin->Prop1:011) -> M_Leontief=0.235 (M_arith=0.624), collapse=Proposition1BlindUndersizing, cov= 25.0% (shock=  0.0%), harm=  4.49 (MC100=  4.91±0.12 [  4.38,   5.18], ruin=  4.0%), R_24= 27.21
    [E=1 V=0 G=1] DefectiveAgent(NoVitalMargin->Prop2:101)     -> M_Leontief=0.021 (M_arith=0.503), collapse=Proposition2ImpotentLucidity, cov= 95.8% (shock= 75.0%), harm= 77.19 (MC100= 80.99±0.27 [ 77.12,  82.77], ruin=100.0%), R_24=  0.00
    [E=1 V=1 G=0] DefectiveAgent(NoGovernance->Prop3:110)      -> M_Leontief=0.101 (M_arith=0.612), collapse=Proposition3CynicalCapture  , cov= 95.8% (shock= 75.0%), harm= 68.01 (MC100= 67.89±0.01 [ 67.69,  68.10], ruin=100.0%), R_24=  0.00
    [E=1 V=0 G=0] PartialAgent(EpistemicOnly:100)              -> M_Leontief=0.010 (M_arith=0.295), collapse=CombinedPropositions2And3   , cov= 95.8% (shock= 75.0%), harm=114.79 (MC100=114.71±0.02 [114.42, 114.98], ruin=100.0%), R_24=  0.00
    [E=0 V=1 G=0] PartialAgent(VitalOnly:010)                  -> M_Leontief=0.094 (M_arith=0.392), collapse=CombinedPropositions1And3   , cov= 20.8% (shock=  0.0%), harm= 80.50 (MC100= 85.45±0.42 [ 80.08,  91.81], ruin=100.0%), R_24=  0.00
    [E=0 V=0 G=1] PartialAgent(GovernanceOnly:001)             -> M_Leontief=0.011 (M_arith=0.288), collapse=CombinedPropositions1And2   , cov= 16.7% (shock=  0.0%), harm=109.58 (MC100=110.81±0.24 [109.50, 115.11], ruin=100.0%), R_24=  0.00
    [E=0 V=0 G=0] DefectiveAgent(AllAxesOff:000)               -> M_Leontief=0.008 (M_arith=0.085), collapse=TotalTriadicCollapse        , cov= 16.7% (shock=  0.0%), harm=125.83 (MC100=124.38±0.24 [120.32, 126.04], ruin=100.0%), R_24=  0.00
  Fixed-Budget Mechanism Isolation Factorial (2^3 x 3 = 24 cells, B_fixed in [8.0, 14.0, 20.0], f=0.65) [Simulated]: delta_harm_Epi=+5.57, delta_harm_Vital=+5.81, delta_harm_Gov=+91.28 (FullTriad mean_harm=19.86)
  Non-Substitutability & Negative Controls [Simulated]: CompensatoryGiantBuffer(NoEpi+2xBuffer) cov=25.0%, harm=3.89 (MC100=4.06±0.02), R_24=39.88, diag=Proposition1BlindUndersizing | BenignSmallWorld(NaiveOptimizer) R_24=71.40, ruin=false | 3^5=243 Threshold Invariance=100.0% | 3^5=243 Local Structural DGP Invariance=100.0% | Wide-Band Stress Grid: dom=66.7%, intact=88.9%
  Closed-Form Prop 1 Coverage Decay Ladder [Assumed]: engine(n=46,rho=0.62)=35.9% | Meng ladder(rho=-0.05): n=46->90.5%, n=500->46.6%, n=5k->0.0%, n=50k->0.00%, n=2.3M->0.0000% (benchmark_met=true, manski_cov=100.0%, reserve_at_TTR=-78.8)
  Process Tracing (Bennett-Checkel) [Assumed]: prior = 0.25 -> posterior = 0.9988 (decibans = +34.1, V: 0.20 -> 0.64)
  Four Reflexive Brakes [Measured forecasts + Assumed welfare/TTR]: RES_sharp=0.117 > RES_lazy=0.000, welfare_opt=2.55 > hoard=-11.97, fast_path (TTR_recovery=1.5 <= TTS_survival=4.0), scalar_target_absent=true (all_active = true)
  Nested Leontief vs Multiplicative Damping [Assumed] (tau=0.50, 3^5=243 grid): V_min=0.1250 vs V_prod=0.0333 (-73.35% vol) | False Collapses: PureLeontief=0.00% (0/243) vs HybridGov=74.07% (180/243) vs Prod5=99.59% (242/243; vol-matched tau_prod=0.2951: 192/243; power-p05: 0/243) | Single-Axis Ruin False-Safe: Arith=100.0% (36/36) vs Leontief=0.0% (0/36)
  Four Pre-Registered Falsifiers [Assumed design + Simulated paired MC100 evaluation] (Mayo SEV >= 0.95): F1(n_eff=24, d_crit=0.0661, d_obs=0.321, SEV=0.995, refuted=false) | F2(n_eff=19, d_crit=0.6698, d_obs=2.514, SEV=0.982, refuted=false) | F3(n_eff=20, d_crit=0.0897, d_obs=0.385, SEV=0.972, refuted=false) | F4(n_eff=17, d_crit=0.1104, d_obs=0.077, SEV=0.996, refuted=true)
[STOCHOLOGY VERIFIED] All 9 progressive chapter laboratories and 3 Collapse Propositions passed.
```

> **Interpreting the Four Pre-Registered Falsifiers (`F1`–`F4`):** Evaluated across two-stratum (benign vs. stressed) paired cohorts at $\mathrm{DEFF} = 1.5$ (`[Assumed design + Simulated paired MC100 evaluation]`), **`F1`** ($d_{\mathrm{obs}} = 0.321 > d_{\mathrm{crit}} = 0.0661, \mathrm{SEV} = 0.995, \text{refuted} = \text{false}$), **`F2`** ($d_{\mathrm{obs}} = 2.514 > d_{\mathrm{crit}} = 0.6698, \mathrm{SEV} = 0.982, \text{refuted} = \text{false}$), and **`F3`** ($d_{\mathrm{obs}} = 0.385 > d_{\mathrm{crit}} = 0.0897, \mathrm{SEV} = 0.972, \text{refuted} = \text{false}$) corroborate the necessity of $S_{\mathrm{epi}}$, $S_{\mathrm{vital}}$, and $S_{\mathrm{gov}}$ on the same order of magnitude as their rejection cutoffs without saturating at $\mathrm{SEV} = 1.000$. By contrast, **`F4`** ($d_{\mathrm{obs}} = 0.077 \le d_{\mathrm{crit}} = 0.1104, \mathrm{SEV} = 0.996, \text{refuted} = \text{true}$) **genuinely refutes** unconditional non-compensability in a mostly mild archival-defect cohort where a doubled physical reserve buffer (`CompensatoryBufferAgent`) partially absorbs epistemic undersizing—demarcating the exact regime boundary of $M_{\mathrm{Leontief}}$: strict non-substitutability holds while an absorbing survival barrier or severe defect is active, whereas partial buffer compensation is possible under moderate shocks with doubled physical slack.
