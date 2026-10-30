# Stochology — Course Syllabus (The CAPIBA Protocol)

> **Status.** This syllabus (also rendered as Appendix E of the book) is an **authorial pedagogical extension** of the repository (`AGENTS.md` §II.C, rule 7). The *three-tempo cycle* (Detachment → Abstraction → Return) and the *five Movements* below are a teaching frame laid over the existing book (`book-src/`) and code (`crates/`). They are **not** part of the CAPIBA Protocol and add no bridge, gate, or invariant. Every formula, symbol, laboratory, and negative proof cited here exists in the current codebase; every number carries its provenance class.

---

## 1. Catalog Description

Stochology is the discipline of judgment and decision-making under uncertainty when the decision must be made before the outcome exists. The course separates three ontological layers: the World ($L_1$), the formal Model ($L_2$), and the Decision-maker who answers for the error ($L_3$). It studies the six ordered pairs that connect them. Each unit suspends the technical reflex of "fitting a model to data", raises the problem to its structural tension (finite representation, temporal irreversibility, curvature of consequence, performative reflexivity, relationality, and buffering), and returns to the equation and the executable algorithm as a rigorous, auditable materialization of that tension. Students work with a pure-Rust apparatus under a fixed seed (`seed = 2026`), structured Epistemological Cards, and algorithmic gates that reject flawed claims.

**Prerequisites.** Introductory probability and statistics; reading knowledge of a programming language (Rust fluency is not required: students run and read the laboratories, and the 90-hour track adds guided modification).

---

## 2. Conceptual Objectives

By the end of the course, the student will be able to:

1. **Type every probability by its layer** ($L_1$ frequency, $L_2$ symmetry or model output, $L_3$ credence) and refuse to treat one as another.
2. **Distinguish four kinds of not-knowing**: aleatory noise, epistemic parameter uncertainty, epistemic specification discrepancy, and Knightian uncertainty, and choose the matching tool for each.
3. **Apply the five transversal lenses** (Information, Time, Scale and Tail, Structure and Invariance, Decision and Calibration) as invariants that hold before any bridge between layers is crossed.
4. **Keep the six ordered pairs apart**: the three implemented forward bridges ($L_1 \to L_2$, $L_2 \to L_3$, $L_3 \to L_1$) and the three reverse reading frames ($L_2 \to L_1$, $L_3 \to L_2$, $L_1 \to L_3$).
5. **Obey the Four Prohibitions**: no unstructured prose between agents, no lone mean, no confusion of learning with performativity, no dogmatic extremes or omitted attempts.
6. **Write, seal, and audit an Epistemological Card** that passes the 4-Filter Eliminatory Funnel and the 8-step SIGA Roadmap, and **falsify a claim** with a negative defect proof.
7. **Label the provenance of every number** they publish (Measured, Simulated, Assumed, Literature-informative) and never set one class against another as if it were the same kind of evidence.

---

## 3. Pedagogical Architecture: the Three-Tempo Cycle

Every unit runs in three tempos.

| Tempo | Name | What happens | Exit condition |
| :---: | --- | --- | --- |
| 1 | **Detachment** (suspending the tool) | Show that no statistical or computational technique, by itself, answers where the observation came from, what was lost on the way, or what the action will do to the world. | The student can state what the familiar technique *cannot* tell them. |
| 2 | **Abstraction** (the pure structural tension) | Place the problem in the three layers, the five lenses, and the six ordered pairs. | The student can name the layer or bridge where the system fails and the Stirling-Walker uncertainty class. |
| 3 | **Return** (the formula as guardian of the nuance) | Return to the equation and the Rust symbol, run the laboratory, then run the negative defect proof that shows the naive method failing. | The student reproduces the positive result (`exit 0`) and the negative proof (`exit 1`). |

The Return is not a recipe: a tension often has more than one legitimate formalization (Manski vs. Lee bounds; BBSE vs. MLLS vs. DFM vs. OTTER under label shift). The formula is *a* rigorous, auditable materialization of the nuance, chosen and recorded on the card.

---

## 4. Scaling: Invariant Abstraction, Modulated Technical Depth

| Track | Hours | Tempos 1–2 (abstraction) | Tempo 3 (return) |
| --- | :---: | --- | --- |
| **Core** | 30 h | All five Movements in full | One **archetype equation** per pair (§6, marked ★), practiced in the Funnel on `card_01`–`card_04` |
| **Full** | 90 h | All five Movements in full | Every return listed in §6, including the frontier syntheses, plus guided modification of a laboratory |

The abstraction is preserved in full in both tracks; only the technical depth of the Return changes.

---

## 5. Schedule Overview

| Unit | Movement | Book chapters | Core (30 h) | Full (90 h) |
| :---: | --- | --- | :---: | :---: |
| 0 | Orientation: seed, gates, and the card | Preface §4–§6, App. B | 1 h | 2 h |
| I | The Act of Conjecture and the Ontology of Uncertainty | Preface, Ch. 1–2 | 4 h | 8 h |
| II | The Five Laws of Conservation and Deformation of the Real | Ch. 3–9 | 5 h | 18 h |
| III | Representation and Materiality: World ⇄ Model | Ch. 10–14 | 5 h | 16 h |
| IV | Rationality, Curvature, and Critique: Model ⇄ Decision | Ch. 15–18 | 5 h | 14 h |
| V | Praxis, Relationality, and Buffering: Decision ⇄ World | Ch. 19–23, 25 | 6 h | 20 h |
| P | Practicum thread: Funnel, SIGA, cards, verification | Ch. 24, 26–27, App. A–C | 4 h (cross-cutting) | 12 h (cross-cutting) |
| | **Total** | | **30 h** | **90 h** |

The practicum (Unit P) covers Part VII and the appendices of the book, plus the strategic synthesis of Part VI as a Full-track capstone, and runs **across** Units I–V; it is not a sixth Movement.

---

## 6. Detailed Program

Notation: ★ = archetype equation taught in the Core track; symbols are Rust items in `crates/`; `--defect <name>` is the negative proof run with `cargo run --release --bin gates -- --defect <name>` (expected `exit 1`).

### Unit 0 — Orientation (Preface §4–§6, Appendix B)

- **Content.** Reproducible chance with `SplitMix64` (`seed = 2026`, $\Phi_1$); the 8 gates `G1`–`G8`; the Four Prohibitions; `scripts/verify.sh`.
- **Practice.** `cargo run --release --bin gates` (`exit 0`); `cargo run --release --bin labs -- --all`.

### Movement I — The Act of Conjecture and the Ontology of Uncertainty (Preface, Ch. 1–2)

- **Detachment.** Suspend the idea that "probability" is a single, self-evident number computed from past data.
- **Abstraction.** Judgment precedes truth: every consequential decision is taken before the outcome exists. Aiming at an invisible target (*stókhos*) with structured reason requires separating what the world is ($L_1$), what our formal language says about it ($L_2$), and who decides and answers for the error ($L_3$). Not-knowing comes in qualitatively distinct kinds.
- **Return.**
  - ★ The five interpretations of probability, typed by layer (Ch. 2, §2.1).
  - Internal coherence vs. external calibration: the Ramsey-de Finetti Dutch Book (Ch. 2, §2.1) set against calibration (Ch. 15).
  - Base-rate neglect: natural-frequency trees (`NaturalFrequencyMemorandum`) and Turing-Good additive weight of evidence in decibans, $W = 10\log_{10}\mathrm{LR}$ (Ch. 2, §2.1; `AlertFatigueAudit`, developed in Ch. 16).
  - ★ The four kinds of uncertainty: aleatory ($L_1$); epistemic parameter ($L_2$); anchored Kennedy-O'Hagan structural discrepancy $\delta(x) \sim \mathcal{GP}(0, k)$ (`KennedyOHaganDiscrepancy`, $L_2$); Info-Gap robustness horizons with the Wald-Sniedovich global Maximin floor (`InfoGapOption`, $L_3$).
- **Laboratory.** `labs -- --uncertainty`.
- **Negative proof.** `--defect sniedovich` (Info-Gap "No-Man's Land").

### Movement II — The Five Laws of Conservation and Deformation of the Real (Ch. 3–9)

- **Detachment.** Suspend the fragmented view of separate disciplines (information theory, stochastic processes, extreme-value theory, causal inference, decision theory).
- **Abstraction.** Before crossing any bridge, five structural invariants hold:
  1. *Irreversibility of compression*: every transformation of data can only lose information about the state of the world.
  2. *Non-ergodicity of time*: the average over many parallel observers is not the trajectory lived by one agent who accumulates gains, losses, and ruin.
  3. *Dominance of the extreme*: far from Gaussian equilibrium, the tail outweighs the center.
  4. *Invariance under intervention*: correlations change with context; causal mechanisms stay stable when we act on the system.
  5. *Consequential asymmetry*: no probabilistic belief is neutral; its value depends on the curvature of harm and on separating genuine resolution from false confidence.
- **Return** (Ch. 9 is the hub; Ch. 3–8 supply the tools by layer).
  - *Irreversibility* → ★ Data Processing Inequality $I(W;Z) \le I(W;Y) \le I(W;X)$ and the strong contraction hierarchy (`DpiMarkovChain`); channel capacity (`blahut_arimoto_capacity`); Fano's error floor (`fano_error_floor`).
  - *Non-ergodicity* → ★ Itô variance drain $g_{\text{time}} = \mu - \tfrac{1}{2}\sigma^2$ (`ito_time_growth_rate`); Kelly fraction under asymmetric returns $f^* = \frac{p}{a} - \frac{1-p}{b}$ (`asymmetric_kelly_fraction`). *Authorial link (Full track):* Huber innovation-weighted Kalman filtering under heavy-tailed glitches (`RobustKalmanControlAudit`) lives in the strategic synthesis (Ch. 24, §24.6), not in the Time lens.
  - *Dominance of the extreme* → ★ Hill tail index (`hill_estimator`) and moment-existence regimes; the moment (DEdH) estimator and $\mathrm{ES}_{0.99}$ (Ch. 9).
  - *Invariance* → Pearl's ladder and DAGs (Ch. 4); ★ Invariant Causal Prediction and nonlinear ICP/NILE over an orthonormal Hermite basis (`run_nonlinear_icp_nile`). *Authorial link (Full track):* `do`-Shapley vs. Rung-1 observational Shapley (`ShapleyCausalAudit`) lives in the strategic synthesis (Ch. 24, §24.2).
  - *Consequential asymmetry* → ★ cost-loss boundary $p^* = \frac{C}{\eta L}$ (`critical_cost_loss_threshold`), EVPI (`calculate_evpi`), Richardson's relative value, and the isotonic CORP-Murphy decomposition $\mathrm{BS} = \mathrm{REL}_{\mathrm{CORP}} - \mathrm{RES}_{\mathrm{CORP}} + \mathrm{UNC}_0$ (`CorpPavDecomposition`, `MurphyDiagram`).
  - Severe testing, multiplicity, e-processes, and conformal prediction (Ch. 8) as the $L_3$ toolset.
- **Laboratory.** `labs -- --lenses`.
- **Negative proofs.** `--defect brier_dominance` (Brier ranking vs. Murphy uniform dominance; Ch. 9), `--defect peeking` and `--defect var_underreport` (optional stopping and VaR/ES underreporting; Ch. 8).

### Movement III — Representation and Materiality: World ⇄ Model (Ch. 10–14)

- **Detachment.** Let go of the illusion that a database is a faithful portrait of nature, and that computation runs in an immaterial ether at no energetic or physical cost.
- **Abstraction.**
  - *World Models, $L_1 \to L_2$:* the world reaches the model through an invisible filter. What was not observed did not vanish at random; silence has structure. Where the world fell silent, the model must choose between declaring the interval of its ignorance and lying with a point estimate. To be authentic, it must separate the invariant structural core from the specification residual.
  - *Model Worlds, $L_2 \to L_1$ (reading frame):* the model returns to the world first as simulated worlds, where chance must be reproducible and tested across many seeds, and second as physical computing worlds, where the digital separation of memory and processing meets thermodynamic and data limits.
- **Return.**
  - $L_1 \to L_2$: ★ Meng's identity $\bar{Y}_n - \bar{Y}_N = \rho_{R,Y}\sqrt{\frac{N-n}{n}}\,\sigma_Y$ (`MengAudit`). *Measured worked example:* on `cps_observational.csv` ($n = 15{,}992$) under an imposed 25% lower-tail MNAR attrition, Meng's effective sample size falls to $n_{\mathrm{eff,Meng}} = 4.5$ (`EmpiricalCoupledSocietyAudit`, `labs -- --all`). Rubin's MCAR/MAR/MNAR taxonomy; ★ non-parametric Manski bounds (`ManskiBounds`) and fractional Lee (2009) trimming bounds; Authentic Model Form $\delta(x) + \hat{S}^*$ (`AuthenticModelForm`, `DiscardedAtomRecord`) and the $(\varepsilon, \delta)$-Almost-Perfect Machine (`AlmostPerfectMachine`).
  - $L_2 \to L_1$: ★ `SplitMix64` substreams and fork-join trees (`MonteCarloSplitTreeAudit`); multi-seed acceptance (20 development seeds `2026..=2045`, 200 sealed confirmation seeds; `stochology_lenses::acceptance`). *Full track:* RK4 ODE isomorphism, the analog wall and noise-aware training, `HP-INV` / `BlockAMC`, SIREN on the Helmholtz PDE (`AnalogWaveComputingSynthesis`); the Triadic synthesis $\mathbb{C} \times \text{Fractals} \times \mathbb{T}^d$ (`ComplexFractalToroidalTechniumSynthesis`). Energy, token, and noise-curve constants in these chapters are **Literature (informative)**.
- **Laboratory.** `labs -- --layers`.
- **Negative proofs.** `--defect lee_point_estimate`, `--defect machine`; Full track adds `--defect analog_wall`, `--defect triadic_technium`.

### Movement IV — Rationality, Curvature, and Critique: Model ⇄ Decision (Ch. 15–18)

- **Detachment.** Let go of summarizing uncertainty by the expected mean, and of the positivist belief that quantitative modeling is a neutral bureaucratic act free of human choice and power.
- **Abstraction.**
  - *Decision Models, $L_2 \to L_3$:* the map from world state to human harm or benefit is rarely a straight line; evaluating a decision by the mean erases the curvature where catastrophe lives. The model's signal must also cross the narrow bottleneck of conscious human attention without alarm fatigue, and stay valid under continuous monitoring.
  - *Model Decisions, $L_3 \to L_2$ (reading frame):* every model carries the fingerprint of the analyst's prior choices: what was included, how many specifications were tried, how the result is narrated. Unrecorded, natural language launders adjectives. At a deeper level, a stochastic reason that keeps only what fits within well-behaved deviations reifies the existing and silences the singular event.
- **Return.**
  - $L_2 \to L_3$: ★ Jensen's gap $\Delta_J = \mathbb{E}[f(X)] - f(\mathbb{E}[X])$ with median, $q_{05}$, $q_{95}$, and CRPS (`NonlinearSummary`; Prohibition 2). *Measured worked example:* on `bank_calls.csv` (164 days, squared daily calls), $\Delta_J = +9.03 \times 10^6$, about $0.85\%$ of $f(\mathbb{E}[X])$. Strictly proper scores (Brier, Log, CRPS) with the Cromwell floor $p \in [\varepsilon, 1-\varepsilon]$ (`apply_cromwell_floor`); anytime-valid evidence: `aGRAPA`/`GRO` betting martingales (`EvidenceProcess`) and SAVI confidence sequences (`SaviConfidenceSequence`) against peeking; VaR/ES e-backtesting (`VarEsEBacktest`); deciban alarm triage against the human conscious bottleneck (`AlertFatigueAudit`). The $10\text{ bit/s}$ figure is **Literature (informative)** (Zheng & Meister); the operating point $\mathrm{TPR} = 0.82$, $\mathrm{FPR} = 0.045$ is **Assumed**.
  - $L_3 \to L_2$: ★ the Duhem-Quine-Lakatos attempt counter $K \ge 1$, the structured Epistemological Card (`EpistemologicalCard`) replacing free prose, and SHA-256 pre-registration. *Full track:* the critical theory of stochastic rationality (`CriticalStochasticRationalitySynthesis`): the naive positivist arm (truncated-Gaussian VaR, correlational "parrot" predictor) is a **simulated** contrast under seeded acceptance, not a measurement of any named real system.
- **Laboratory.** `labs -- --siga` (cards as $L_3 \to L_2$ records).
- **Negative proofs.** `--defect cromwell`; Full track adds `--defect instrumental_reason`. The anytime-valid proofs `--defect peeking` and `--defect var_underreport` are run in Movement II, where Ch. 8 develops the tools.

### Movement V — Praxis, Relationality, and Buffering: Decision ⇄ World (Ch. 19–23, 25)

- **Detachment.** Let go of the passive external observer (who believes that forecasting the world does not change it) and of methodological individualism (agents and events as isolated billiard balls in a static void).
- **Abstraction.**
  - *World Decisions, $L_3 \to L_1$:* acting on the world through a model moves the world. Populations react strategically to the deployed rule, indicators turn into gamed targets, and systems that feed recursively on their own synthetic outputs without physical anchoring degenerate.
  - *Decision Worlds, $L_1 \to L_3$ (reading frame):* the world returns three constraints to the decision: (i) individuals are interdependent networks, not isolated atoms, so clustering shrinks independent information in the tails; (ii) time is bitemporal, crossed by regime drift and by irreconcilable fairness criteria; (iii) in open, dissipative, non-ergodic systems, survival requires joining the art of aiming (*stókhos*) to the architecture of stocks, reserves, and buffers, judging the decision-maker by the integrity of the internal process (*Telos*) and not by the luck of the outcome (*Skopos*).
- **Return.**
  - $L_3 \to L_1$: ★ Performative Stability $\theta_{\mathrm{PS}}$ vs. Stackelberg Optimality $\theta_{\mathrm{PO}}$ and the RRM contraction condition $\frac{\varepsilon\beta}{\gamma} < 1$ (`PerformativeSystem`); the four Goodhart variants (`GoodhartTaxonomyAudit`); Accumulate vs. Replace synthetic recursion (`GerstgrasserAccumulateAudit`, Ch. 19, §19.3), whose Accumulate error is bounded by the Basel-problem series $\sum_g g^{-2} < \pi^2/6$ (unrelated to the Basel III banking accords).
  - $L_1 \to L_3$: ★ Moulton-Campbell tail design effect $n_{\mathrm{eff}}(p) = \frac{n}{1 + (\tilde{m}-1)\rho_I(p)}$ (`ExceedanceDesignEffect`); $\Phi_5$ resolution parity (`verify_resolution_phi5`); ★ the bitemporal clock $t_{\text{disp}} \ge t_{\text{ev}}$ with a synthetic canary $I(\text{Canary};Y) = 0$ (`BitemporalClockAudit`) and the single-opening `TemporalVault`; drift adaptation (ExTRA-WCP-T, online ACI, the four label-shift estimators); Chouldechova's impossibility (`ChouldechovaAudit`).
  - Axis syntheses (Full track): relational ontology and complex systems (`RelationalComplexSystemsSynthesis`: Brusselator RK4 entropy production, Schelling, Daisyworld); ★ (Core, conceptual only) the PATE Telos/Skopos matrix and, in the Full track, the full stochary-conjectural cybernetic synthesis (`StocharyConjecturalCyberneticSynthesis`: the four ontological stock axioms, TTS/TTR, VSM `S3*`); the coupled public-private stock-flow consistent society (`CoupledPublicPrivateSocietySynthesis`, `SfcFlowMatrix`; Ch. 25). Results in these syntheses are **simulated** and gated by seeded multi-seed acceptance; claims listed in `discarded_claims` must be taught as refuted.
- **Laboratories.** `labs -- --invariants`, `labs -- --governance`.
- **Negative proofs.** Core: `--defect performativity`, `--defect moulton`, `--defect phi5`, `--defect clock`, `--defect vault`, `--defect chouldechova`. Full adds `--defect replace_collapse`, `--defect concept_drift`, `--defect stochary_cybernetics`, `--defect coupled_society`.

### Unit P — Practicum Thread (Ch. 24, 26–27, Appendices A–C; runs across Units I–V)

- **Content.** The 4 Eliminatory Filters and the 8-step SIGA Roadmap (Ch. 26); the transversal empirical synthesis across the 13 datasets (Ch. 27, `labs -- --real-data`); architecture and verification (Appendices A–B); data sources and sealed reserves (Appendix C).
- **Case studies.** `card_01_atlantic_storms_evacuation.yaml`, `card_02_bank_queue_staffing.yaml`, `card_03_nyc_flights_holding.yaml`, `card_04_dpi_civil_defense_triage.yaml`, each audited with `gates -- --audit-card <card>`.
- **Full-track capstone.** The Strategic AI six-pair accommodation synthesis (`StrategicAiAccommodationSynthesis`, Ch. 24), read as an authorial frame across all six pairs; negative proof `--defect strategic_accommodation`.

Across both tracks, the 21 negative defect proofs are distributed with no repetition: Movement I (1), II (3), III (4), IV (2), V (10), Practicum (1). The same assignment appears in each chapter's three-tempo block.

---

## 7. Assessment

| Component | Weight | Description |
| --- | :---: | --- |
| Pair diagnoses | 20% | For five short cases, answer the Three Questions (layer/bridge, Stirling-Walker class, severe test). |
| Laboratory reproductions | 25% | Reproduce one positive laboratory and one negative proof per Movement; explain why the naive method fails. |
| Provenance audit | 15% | Label every number in a supplied derived text (Measured / Simulated / Assumed / Literature) and flag violations of `AGENTS.md` §II.C. |
| Capstone Epistemological Card | 40% | Write a new card on one of the 13 datasets, declare $K \ge 1$ and discarded findings, seal it with SHA-256, and pass the 4-Filter Funnel. Filter 4 cross-authorship: a peer acts as `adversarial_reviewer`. |

---

## 8. Core Bibliography

The primary text is the course book (`book-src/SUMMARY.md`, Preface, Chapters 1–27, and Appendices A–E) together with `docs/PROTOCOL.md` and `docs/syntheses/`. Primary sources are listed per chapter and in Appendix C; the empirical datasets and their provenance are in `data/SOURCES.md`.
