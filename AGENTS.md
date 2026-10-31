# AGENTS.md — Governance, Engineering, and Orchestration Contract (Stochology)

This file is the binding contract for any agent or contributor working in this repository. **Stochology** is a **scientific and ethical discipline** — the science and ethics of aim (*stókhos*, from Plato's *Philebus* 55e–56c, Hippocratic *sēmeîa*, *kairós*, and *mêtis* to Bernoulli's *Ars Conjectandi*) for judgment and decision-making under uncertainty when action must precede the outcome — structured by a **disciplinary program** across a **nuclear triad** of three interdependent axes (**epistemic margin**, **vital margin**, and **fully apt agency under institutional governance**; see [`notes/A Ética da Mira.md`](notes/A%20%C3%89tica%20da%20Mira.md), [`book-src/00-preface.md`](book-src/00-preface.md) §1–§2, and [`book-src/01-layers-six-pairs.md`](book-src/01-layers-six-pairs.md) §1.4). Removing any one axis produces a specific pathology (**Collapse Proposition 1**: *blind undersizing*; **Collapse Proposition 2**: *impotent lucidity before ruin*; **Collapse Proposition 3**: *cynical capture of doubt and pillaging of reserves*), mediated across **Three Critical Lineages**, **Three Mediating Filters**, **Seven Constitutive Tensions**, **Five Methodological Families** (`book-src/26-funnel-siga.md` §26.4), and **Three Internal Goodhart Brakes & Four Conditions of Refutability** (`book-src/27-empirical-synthesis.md` §27.3). The system built here is its **auditable epistemological machine**: claim, generating process, executable null model, and severe test live in the same place, with a pre-registered seed declared before any fitting, a structured Epistemological Card, and algorithmic gates that reject flawed claims. The final reader of the output is a **human** (decision-maker, student, instructor, or auditor).

This repository implements the normative kernel, transversal lenses, operational roadmap, and verification architecture of **Stochology** in **pure Rust (zero external dependencies)**:

- Full specification and architectural mapping: [`docs/PROTOCOL.md`](docs/PROTOCOL.md)

---

## I. Golden Rules — The Four Prohibitions

1. **Ban on unstructured free natural language between agents (`PROHIBITION_1`).** All communication between agents or modules passes through structured **Epistemological Cards** (`EpistemologicalCard`, in `crates/stochology-core/src/card/`), serialized in YAML (`cards/*.yaml`) and validated by `EpistemologicalCard::audit()`.
2. **Ban on the lone mean (`PROHIBITION_2`).** When the loss or payoff function is nonlinear, dynamics are multiplicative, or the tail is heavy, reporting only the mean is forbidden. Report the evaluation at the mean $f(\mathbb{E}[X])$, the simulated expectation $\mathbb{E}[f(X)]$, Jensen's Gap $\Delta_J = \mathbb{E}[f(X)] - f(\mathbb{E}[X])$, the median, tail quantiles ($q_{05}, q_{95}$), and the *Continuous Ranked Probability Score* ($\mathrm{CRPS}$, in `crates/stochology-core/src/layers/`).
3. **Ban on confusing learning with performativity (`PROHIBITION_3`).** Small predictive error does not prove that a model is true. On Bridge $L_3 \to L_1$, separate **Performative Stability** ($\theta_{\mathrm{PS}}$) from **Performative Optimality** ($\theta_{\mathrm{PO}}$, Stackelberg equilibrium), measure Wasserstein-1 sensitivity ($\varepsilon$), audit the strict contraction condition of Repeated Risk Minimization ($\frac{\varepsilon \beta}{\gamma} < 1$, in `PerformativeSystem`), and classify metric collapse across the **4 Variants of Goodhart's Law** (`GoodhartTaxonomyAudit`: Regressional, Extremal, Causal, Adversarial).
4. **Ban on dogmatic extremes and omitted attempts (`PROHIBITION_4`).** No probability may be $0\%$ or $100\%$ (Cromwell's Rule: $p \in [\varepsilon_{\text{floor}}, 1 - \varepsilon_{\text{floor}}]$). Evaluate every forecast out-of-sample via the isotonic **CORP-Murphy** decomposition using *Pool-Adjacent-Violators* ($\mathrm{BS} = \mathrm{REL}_{\mathrm{CORP}} - \mathrm{RES}_{\mathrm{CORP}} + \mathrm{UNC}_0$, in `crates/stochology-lenses/src/decision/`), and declare and validate the counter of attempted specifications $K \ge 1$, the evidence process (*e-process* $E_t$), and discarded findings.

---

## II. Boundary Invariants ($\Phi_1$ to $\Phi_5$)

| Invariant | Formal Requirement | Code Location |
| --- | --- | --- |
| $\Phi_1$ — Simulability | Pre-registered seed declared before fitting (`ProtocolProfile::seed`, §II.D), `SplitMix64`, lock-free `.split(stream_id)` / `.split_fork_join_pair(branch_tag)` substreams (`MonteCarloSplitTreeAudit`), and executable null model before any fitting | `crates/stochology-core/src/rng.rs` & `card/` |
| $\Phi_2$ — Independent Unit | Margin over the true independent unit; Moulton-Campbell correction for unequal clusters $\tilde{m} = \bar{m}(1 + \mathrm{CV}^2)$, threshold exceedance ICC $\rho_I(p)$, and $n_{\mathrm{eff}}(p) = \frac{n}{1 + (\tilde{m}-1)\rho_I(p)}$ | `crates/stochology-core/src/invariants.rs` (`ExceedanceDesignEffect`, `OperationalWeightingGap`) |
| $\Phi_3$ — Severe Test & Missingness | Explicit Rubin missingness classification (`MCAR`, `MAR`, `MNAR`), Meng's Identity ($\rho_{R,Y}$), non-parametric Manski bounds, SHA-256 pre-registration, and Pearl's Ladder rung | `crates/stochology-core/src/layers/`, `sha256.rs` & `card/` |
| $\Phi_4$ — Scope, Fairness & Exit | Economic boundary $p^* = \frac{C}{\eta L}$, algebraic audit of Chouldechova's trilemma ($\mathrm{FPR}_A / \mathrm{FPR}_B$ under unequal base rates), and automatic numeric exit clause | `crates/stochology-core/src/invariants.rs` (`ChouldechovaAudit`) & `card/` |
| $\Phi_5$ — Question Resolution | Ban on answering a daily/shift operational decision with coarse annual/monthly aggregates ($R_0$) | `crates/stochology-core/src/invariants.rs` (`verify_resolution_phi5`) |

The `gates` binary enforces invariants $\Phi_1$–$\Phi_5$ and methodological locks across eight gates (`G1` to `G8`) and exits with code `0` (approved) or `1` (blocked). For the simulated syntheses (`critical/`, `stocks/`, `triad/`, `strategic/`, `society/`), `G8` applies multi-seed acceptance (`stochology_lenses::acceptance`): every named claim must pass on at least 95% of the 20 pre-registered development seeds (used to decide what to keep) and on at least 95% of 200 sealed confirmation seeds (`9_000_000..=9_000_199` for `critical`, `stocks`, `triad`, `strategic`, and the virgin block `9_000_200..=9_000_399` for `society`) evaluated once after the claim set was frozen; claims marked `.det.` do not depend on the seed and must pass on every seed, and identities must close on every seed. Claims that fail either set are reported as `discarded_claims` (refuted, never approved), and the corresponding negative proofs must fail on all 20 development seeds through seed-dependent claims. `data/preregistration/acceptance_registry.txt` seals the seed sets, the pass rate, every claim name, the SHA-256 of the 64 claim-defining source files (the module closure of the five syntheses, checked by a unit test), and the confirmation pass rates; its SHA-256 is listed in `data/SHA256SUMS`, and `G8` regenerates it (`gates -- --print-acceptance-registry`) and blocks on any mismatch, so editing a claim-defining source requires re-sealing under human review.

### II.B. The Six Verification Pillars of Stochology

**Stochology** organizes **what is verified** into **6 verification pillars** (`1`–`6`), orthogonal to the **5 Boundary Invariants** ($\Phi_1$–$\Phi_5$), the **8 Epistemological Gates** (`G1`–`G8`), and the **8-Step SIGA Operational Roadmap** (*how the verification cycle runs*). Each pillar maps to explicit gates and a disjoint subset of the **27 negative defect proofs** (`4 + 5 + 3 + 4 + 3 + 8 = 27`):

| Pillar | Formal Verification Scope | Rust Primitives | Gates | Negative Defect Proofs (`--defect`, `exit 1`) |
| :---: | --- | --- | :---: | --- |
| **1** — **Calibrated** | Isotonic PAV `CORP` decomposition (Brier & Log scores), `MurphyDiagram` ($\int_0^1 \bar{S}_\theta\,d\theta = \frac{1}{2}\mathrm{BS}$), Cromwell bounds ($p \in [\varepsilon, 1-\varepsilon]$), and the threshold-wise CORP decomposition of the CRPS for continuous forecasts ($\overline{\mathrm{CRPS}} = \mathrm{MCB} - \mathrm{DSC} + \mathrm{UNC}$). | `CorpPavDecomposition`, `MurphyDiagram`, `CorpRocMurphyTriptych`, `apply_cromwell_floor`, `CorpCrpsDecomposition` | `G6` | `cromwell`, `brier_dominance`, `concept_drift`, `crps_miscalibration` |
| **2** — **Anytime-valid** | Non-parametric `aGRAPA` / `GRO` betting martingales $E_t$, `SAVI` confidence sequences, `GREE` VaR/ES e-backtesting, Moulton-Campbell tail $n_{\mathrm{eff}}(p)$, $\Phi_5$ resolution parity, and e-BH false discovery control over K e-values under arbitrary dependence. | `EvidenceProcess`, `SaviConfidenceSequence`, `VarEsEBacktest`, `ExceedanceDesignEffect`, `verify_resolution_phi5`, `EBhMultiplicityAudit` | `G2/G3/G6` | `moulton`, `peeking`, `var_underreport`, `phi5`, `multiplicity` |
| **3** — **Performative-aware** | Performative Stability ($\theta_{\mathrm{PS}}$) vs. Stackelberg Optimality ($\theta_{\mathrm{PO}}$, `PerfGD`), RRM contraction ($\frac{\varepsilon\beta}{\gamma}<1$), Goodhart's 4 variants, `Accumulate` vs. `Replace` synthetic recursion bounds, and a measured lower bound on $\varepsilon$ from randomized action arms (performative power). | `PerformativeSystem`, `GoodhartTaxonomyAudit`, `GerstgrasserAccumulateAudit`, `SyntheticDataPolicy`, `PerformativePowerAudit` | `G6/G8` | `performativity`, `replace_collapse`, `understated_epsilon` |
| **4** — **Invariant** | Causal invariance across environments (`ICP` / `NILE`), Meng's `ddc`/`DDI` + fractional Lee (2009) trimming bounds & tipping point $\delta_1^*$, anchored Kennedy-O'Hagan GP, Info-Gap + Sniedovich Maximin floor, and anchor regression with the anchor-stability diagnostic. | `MengAudit`, `ManskiBounds`, `run_nonlinear_icp_nile`, `KennedyOHaganDiscrepancy`, `InfoGapOption`, `AuthenticModelForm`, `AnchorRegressionAudit` | `G3/G6/G8` | `lee_point_estimate`, `sniedovich`, `machine`, `anchor_extrapolation` |
| **5** — **Bitemporal** | Bitemporal Clock ($t_{\text{disp}} \ge t_{\text{ev}}$), synthetic canary mutual-information check ($I(\text{Canary}; Y)=0$), and single-opening SHA-256 `TemporalVault` with `PCCP` / `PLAT` governance, and real-time data vintages as bitemporal records (event = observation quarter, availability = vintage). | `BitemporalRecord`, `BitemporalClockAudit`, `TemporalVault`, `PredeterminedChangeControlPlan`, `ProfitLossAttributionTest`, `RealTimeVintageAudit` | `G7` | `clock`, `vault`, `vintage_leak` |
| **6** — **Auditing** | Eliminatory 4-Filter Funnel, Filter 4 cross-authorship, Chouldechova & KMR fairness impossibility audits, three-way multicalibration (`Violated` / `Certified` / `Inconclusive`), physical/analog `HP-INV` & wave computing, critical stochastic rationality, VSM `S3*` / `PATE` cybernetics, Triadic ($\mathbb{C} \times \text{Fractals} \times \mathbb{T}^d$) & Technium synthesis, Strategic AI Six-Pair Accommodation (`ShapleyCausalAudit`, `RubinMultipleImputationAudit`, `LlmJudgePpiRectifier`, `SaatyAhpRankReversalAudit`, `RegulatoryComplianceAudit`, `EpistemicPkmProvenanceAudit`, `RobustKalmanControlAudit`), and Coupled Public-Private Stock-Flow Consistent Society Synthesis (`CoupledPublicPrivateSocietySynthesis`, `EmpiricalCoupledSocietyAudit`, `SfcFlowMatrix`, `H1aRegulatorCouplingAblation`, `H1bCreditScreeningAblation`, `H2VcgGreenLaffontCommonsAblation`). | `ChouldechovaAudit`, `MulticalibrationAudit`, `AlertFatigueAudit`, `AnalogWaveComputingSynthesis`, `CriticalStochasticRationalitySynthesis`, `StocharyConjecturalCyberneticSynthesis`, `ComplexFractalToroidalTechniumSynthesis`, `EmpiricalTriadicTechniumAudit`, `StrategicAiAccommodationSynthesis`, `EmpiricalStrategicAccommodationAudit`, `CoupledPublicPrivateSocietySynthesis`, `EmpiricalCoupledSocietyAudit`, `SfcFlowMatrix` | `G4/G8` | `chouldechova`, `analog_wall`, `instrumental_reason`, `stochary_cybernetics`, `triadic_technium`, `strategic_accommodation`, `coupled_society`, `subgroup_miscalibration` |

### II.C. Claim Provenance Discipline

Every quantitative claim that leaves the code — in `book-src/`, `docs/PROTOCOL.md`, `README.md`, card commentary, or any derived text (summaries, posts, slides, notebook "remixes") — carries exactly one provenance class, stated next to the number:

| Class | Meaning | Required wording |
| --- | --- | --- |
| **Measured** | Computed by the code on a real dataset in `data/*.csv` | Name the dataset (e.g. `danish_fire.csv`) |
| **Simulated** | Output of a pre-registered seeded generative model | Say "simulated" (under a pre-registered seed or multi-seed audit); if `G8` reports the claim in `discarded_claims`, say it is discarded |
| **Assumed** | A hand-set input or design target (e.g. the Stochological operating point $\mathrm{TPR} = 0.82$, $\mathrm{FPR} = 0.045$ in `AlertFatigueAudit`), including every `protocol_profile` parameter (§II.D) | Say "assumed"; never present it as a result |
| **Literature (informative)** | A constant transcribed from a source and not computed (Horowitz's $640\text{ pJ}$, the IEA's $945\text{ TWh}$, Epoch AI's $300\text{T}$ tokens, Ferran Alia's noise curve, Tsinghua `ACCEL`) | Cite the source and mark it informative |

Rules, each derived from an audited failure:

1. Never set a measured value against an assumed or simulated one as if they were the same kind of evidence.
2. A toy simulation supports no claim about a named real system, product, or population.
3. Do not invent causal links between literature constants that their sources do not state, and do not turn a power ratio into an efficiency claim.
4. Give units and relative size next to a large absolute number (Jensen's Gap $\Delta_J = +9.03 \times 10^6$ squared daily calls in `bank_calls.csv` is $\approx 0.85\%$ of $f(\mathbb{E}[X])$).
5. One study does not license a claim about a whole class ("$88\%$ of the Epic Sepsis Model's alerts", not "$88\%$ of AI alerts").
6. Frame narratives as thought experiments or worked examples; never narrate a scenario from the code as a first-person event.
7. Keep the six ordered pairs of World ($L_1$), Model ($L_2$), and Decision ($L_3$) apart. The three forward pairs are the discipline's implemented bridges: World Models $L_1 \to L_2$ (`MengAudit`, `ManskiBounds`), Decision Models $L_2 \to L_3$ (`NonlinearSummary`, `AlertFatigueAudit`), and World Decisions $L_3 \to L_1$ (`PerformativeSystem`, `GoodhartTaxonomyAudit`). The three reverse pairs are a reading frame over existing code, not new bridges: Model Worlds $L_2 \to L_1$ (seeded null models, `SplitMix64` / `MonteCarloSplitTreeAudit`, multi-seed acceptance in `stochology_lenses::acceptance`), Model Decisions $L_3 \to L_2$ (attempt counter `attempt_counter_k` $K \ge 1$, named `DiscardedAtomRecord`, SHA-256 pre-registration, and the card's `who_pays_for_error` field audited against `ChouldechovaAudit`), and Decision Worlds $L_1 \to L_3$ (Stirling-Walker class with `InfoGapOption` + Sniedovich floor, `ExceedanceDesignEffect` $n_{\mathrm{eff}}(p)$, `verify_resolution_phi5`). Present any concept, framework, bridge, or pair-level mechanism that the repository does not implement as an authorial extension, not as part of the normative kernel.
8. Numbers in the book must match what the current code prints; regenerate quoted `labs`/`gates` output blocks instead of editing them by hand.

`stochology-adversary` applies this table to every derived text before publication (claim, provenance class, source location, verdict); `stochology-scribe` applies it to `book-src/` and `docs/`; `stochology-modeler` tags transcribed constants `Informative:` and hand-set inputs `Assumed:` in doc comments.

### II.D. Kernel and Clay — Application Profiles

The Four Prohibitions and $\Phi_1$–$\Phi_5$ bind as **principles**, together with their **mechanisms**: a declared seed, a manifest-sealed dataset, the SHA-256 pre-registration, $K \ge 1$, findings conservation, Cromwell's interval, the CORP-Murphy identity, cross-authorship, and a scope with a numeric exit. These are the kernel. The numbers and vocabulary used to check them are **clay**. An application shapes them in the optional `protocol_profile` section of its card (`ProtocolProfile`, `crates/stochology-core/src/card/profile.rs`; specification in `docs/PROTOCOL.md` §1.6). The shapeable items are:
- the seed and $1/\alpha$;
- the tolerances;
- the $\Phi_5$ and lone-mean vocabularies;
- the dataset root, manifest, and column mapping;
- the replication counts and Funnel thresholds.

The rules are:

1. A non-canonical profile is sealed into `preregistration_hash`. Recalibrating it after seeing data breaks `PHI3_PREREGISTRATION_HASH`.
2. A profile may waive only the instruments in `WAIVABLE_INSTRUMENTS` (`PHI4_CHOULDECHOVA`, `INFO_GAP_SNIEDOVICH`, `PROHIBITION_3_RRM_CONTRACTION`). Each waiver needs its admissibility condition and a written reason. Every waiver is reported with the findings it suppressed.
3. Every profile parameter is **Assumed** (§II.C).
4. The canonical profile reproduces the pre-profile thresholds exactly. `G1`–`G8` and the 27 negative proofs run under it; shaped applications are audited with `gates -- --audit-card <card.yaml>` (worked example: `examples/clay/`, simulated, seed 31).
5. Moving the seed or the $\Phi_5$ vocabulary inside the sealed sources (`rng.rs`, `invariants.rs`) is out of scope for profiles and requires a re-seal under human review.

---

## III. Tier 1 — The Five Transversal Lenses, Almost-Perfect Machine, and Severe Uncertainty

Stochology's architecture is organized in four tiers: **Tier 0 — Normative Kernel** (§I, §II), **Tier 1 — Lenses and Uncertainty Typing** (§III), **Tier 2 — The Six Ordered Pairs** (§II.C rule 7, `docs/PROTOCOL.md` §3), and **Tier 3 — Syntheses** (§III.B). Tiers 0–2 change only under human review; Tier 3 modules change only through pre-registration and re-sealing.

The five transversal lenses and the Almost-Perfect Machine live in `crates/stochology-lenses`:

1. **Information Lens (`information.rs`):** Data Processing Inequality (DPI, 3-step Markov chain `DpiMarkovChain`), exact hierarchy of strong contraction constants ($\eta_{\chi^2}(W) = \rho_m^2 \le \eta_{\mathrm{KL}}(W) \le \eta_{\mathrm{TV}}(W)$), amplitude-constrained bound, and Fano's error floor.
2. **Time Lens (`time.rs`):** Itô variance drain ($g_{\text{time}} = \mu - \frac{1}{2}\sigma^2$), optimal Kelly fraction under asymmetric returns ($f^* = \frac{p}{a} - \frac{1-p}{b}$), absorbing ruin barriers, and Langevin/MCMC mixing time under Log-Sobolev Inequality.
3. **Scale and Tail Lens (`tail.rs`):** Hill estimator ($\hat{\alpha}_{k,n}$), moment existence regime classification ($\alpha \le 1$, $1 < \alpha \le 2$, $\alpha > 2$), and quantiles with effective tail sample size.
4. **Structure and Invariance Lens (`invariance/`):** Invariant Causal Prediction (ICP by intersection across environments $e \in \mathcal{E}$), environment OLS invariance penalty (`env_ols_invariance_penalties`), nonlinear ICP and NILE/IRM risk-gradient penalty over orthonormal Hermite basis (`run_nonlinear_icp_nile`), 2nd-order worst-case loss, and critical hidden confounding threshold $\Lambda^*$.
5. **Decision and Calibration Lens (`decision/`):** Cost-loss rule $p^* = \frac{C}{\eta L}$, Expected Value of Perfect Information ($\mathrm{EVPI}$), quantile score (pinball loss), isotonic **CORP-Murphy** decomposition via the PAV algorithm, and exact fair-coin calibration vs. decision benchmark (`ExactCoinCalibrationBenchmark`).
6. **Almost-Perfect Machine and Authentic Model Form (`machine.rs`):** $(\varepsilon, \delta)$-Almost-Perfect Machine $M = (T, \Phi, \theta)$ (`AlmostPerfectMachine`, auditing universal Barron density, DPI $\delta$-sufficiency, Fano/minimax rate optimality, Telgarsky depth-$D$ compositional efficiency $2^D$, and Gram effective dimension $d_{\mathrm{eff}} = \frac{(\mathrm{tr}\,G)^2}{\|G\|_F^2}$ across `P1`–`P5`); and First-Class Authentic Model Form (`AuthenticModelForm`, $\delta(x) + \hat{S}^*$, explicitly named discarded spurious atoms `DiscardedAtomRecord`, simultaneous closure of the 6 numerical identities `IdentityClosureAudit`, and the 5-part operational authenticity criterion $A(\text{model})$ `AuthenticityCriterionA`), enforced in Gate `G8` and `--defect machine`.

Structural and severe uncertainty modeling lives in `crates/stochology-core/src/uncertainty.rs`:

- **Kennedy-O'Hagan Structural Discrepancy (`KennedyOHaganDiscrepancy`):** Gaussian Process $\delta(x) \sim \mathcal{GP}(0, k(x, x'))$ with an anchoring constraint that prevents parameter pseudo-convergence and extrapolation overconfidence.
- **Info-Gap Decision Theory with Sniedovich's Safeguard (`InfoGapOption`):** Robustness horizon $\hat{\alpha}(q, r_c)$, opportuneness horizon $\hat{\beta}(q, r_w)$, *zero-ing* property, and Wald's global Maximin safeguard against "No-Man's Land".

### III.B. Tier 3 — Syntheses Registry

The syntheses below are Tier 3 modules: gated by `G8` (the five multi-seed syntheses under `stochology_lenses::acceptance`), sealed in `data/preregistration/acceptance_registry.txt`, and not part of the normative kernel. Their full specifications and contract summaries live in [`docs/syntheses/`](docs/syntheses/); `docs/PROTOCOL.md` §5 carries the registry with real-data couplers. Claim counts are **simulated** and printed by `cargo run --release --bin gates`.

| # | Synthesis | Module | Entry symbol | `G8` claims (gated / discarded) | Negative proof | Book | Specification |
| :-: | --- | --- | --- | :---: | --- | --- | --- |
| 7 | Relational Ontology and Complex Systems | `relational/` | `RelationalComplexSystemsSynthesis` | single-seed audit | none | Ch. 22 | [`relational.md`](docs/syntheses/relational.md) |
| 8 | Physical, Analog, and Wave-Interference Computing | `analog/` | `AnalogWaveComputingSynthesis` | single-seed audit | `analog_wall` | Ch. 13 | [`analog.md`](docs/syntheses/analog.md) |
| 9 | Critical Theory of Stochastic Rationality & Algorithmic Governmentality | `critical/` | `CriticalStochasticRationalitySynthesis` | 16 / 3 | `instrumental_reason` | Ch. 18 | [`critical.md`](docs/syntheses/critical.md) |
| 10 | Stochary-Conjectural Synthesis, Stock Axioms, Supply-Chain Physics & Cybernetic Governance | `stocks/` | `StocharyConjecturalCyberneticSynthesis` | 22 / 4 | `stochary_cybernetics` | Ch. 23 | [`stocks.md`](docs/syntheses/stocks.md) |
| 11 | Triadic Synthesis ($\mathbb{C} \times \text{Fractals} \times \mathbb{T}^d$) & Technium | `triad/` | `ComplexFractalToroidalTechniumSynthesis` | 17 / 1 | `triadic_technium` | Ch. 14 | [`triad.md`](docs/syntheses/triad.md) |
| 12 | Strategic AI Six-Pair Methodological Accommodation | `strategic/` | `StrategicAiAccommodationSynthesis` | 15 / 2 | `strategic_accommodation` | Ch. 24 | [`strategic.md`](docs/syntheses/strategic.md) |
| 13 | Coupled Public-Private Stock-Flow Consistent Society | `society/` | `CoupledPublicPrivateSocietySynthesis` | 4 / 1 (virgin block `9_000_200..=9_000_399`) | `coupled_society` | Ch. 25 | [`society.md`](docs/syntheses/society.md) |

Discarded claims are refuted, never approved, and must be taught and quoted as discarded (§II.C). Adding a synthesis adds one row here and one page in `docs/syntheses/`; it never grows the Tier 0–2 text.

---

## IV. 4-Filter Funnel, 8-Step SIGA Roadmap, and Production Governance

- **Eliminatory Funnel and SIGA Roadmap (`crates/stochology-siga`):**
  - `funnel/`: Evaluates the **4 Eliminatory Filters** in sequence (Filter 1: $\mathrm{EVPI}$ and $\Phi_5$; Filter 2: Moulton-Campbell $n_{\mathrm{eff}}(p)$, Markov Blanket, and Berkson collider ban; Filter 3: Simulation-Based Calibration SBC $\chi^2$ and SHELF elicitation; Filter 4: SHA-256 pre-registration, independent cross-authorship, and exit clause).
  - `roadmap/`: Runs the **8-Step SIGA Roadmap** (**S**urvey the Layer & Bridge; **I**nterrogate the Decision-Maker; **G**enerate the Null & Noise Physics; **A**ct with Threshold, Memory & Exit), cumulative evidence processes (*e-processes* $E_t = \prod L_i$), executive memoranda in integer natural frequencies, `AlertFatigueAudit` on Bridge $L_2 \to L_3$ coupling the $10\text{ bit/s}$ human conscious bottleneck (Zheng & Meister) and Turing-Good additive deciban weights ($W = 10\log_{10}\mathrm{LR}$) to shift-level $\mathrm{PPV}$, the **Mayo–Manski Severe Boundary Bridge** (`MayoManskiSevereBoundaryAudit`, `ImbensManskiConfidenceInterval`), and **Keynes's Evidential Weight $V$ & 5th Methodological Family Process Tracing** (`KeynesEvidentialWeightAudit`, `ProcessTracingTestKind`).
- **Bitemporal Governance, Vault, and Drift (`crates/stochology-governance`):**
  - `clock.rs`: Bitemporal Clock ($t_{\text{disp}} \ge t_{\text{ev}}$) and synthetic canary column ($I(\text{Canary}; Y) = 0$).
  - `vault.rs`: Single-opening Temporal Vault (`TemporalVault`) with SHA-256 seal and automatic downgrade to exploratory status on failure.
  - `drift/`: Murphy-CORP Circuit Breaker (`MurphyCorpCircuitBreaker`), adaptation to *Covariate Drift* via exponential tilt and *ExTRA-WCP-T* with Lei & Candès bound ($1 - \alpha - d_{\mathrm{TV}}$), adaptation to *Label Shift* via the 4 canonical estimators (**BBSE**, **MLLS/SLD**, **DFM**, and **OTTER**), **Predetermined Change Control Plan (`PredeterminedChangeControlPlan`)** coupled to the **Profit and Loss Attribution Test (`ProfitLossAttributionTest` — Spearman $r_s$ + KS $D_{\mathrm{KS}}$ traffic-light)** and **`EvaluatedCompetenceCriterion`**, and **Ethics of Aim** (`TriadicSafeguardComplementarityAudit`, `CollapseProposition`, `ReflexiveStochologyGoodhartAudit`, `SosaVirtueAndInnesHolmstromContractAudit`, `SosaJudgmentGrade`).

---

## V. Model-Agnostic Multi-Agent Orchestration & Cryptographic Policy

This contract is model- and harness-agnostic. It governs any autonomous coding agent, LLM orchestrator, or multi-agent runtime operating on this repository (via `AGENTS.md` and `.agents/`):

1. **Pre-Implementation Structural Diagnosis (The Three Questions):**
   Before proposing or writing code, answer:
   - **Ontological Layer & Bridge:** Where can the system fail ($L_1$ World, $L_2$ Model, $L_3$ Agent, or Bridges $L_1 \to L_2$, $L_2 \to L_3$, $L_3 \to L_1$)?
   - **Stirling-Walker Uncertainty Class:** Is uncertainty `aleatory`, `epistemic_parameter`, `epistemic_specification` (anchored `KennedyOHaganDiscrepancy`), or `knightian` (`InfoGapOption` + Sniedovich's global Wald Maximin floor)?
   - **Severe Test:** What executable generative null model (under a pre-registered seed, $\Phi_1$) and negative defect proof (`gates -- --defect <name>`) can falsify the claim?
2. **Filter 4 Separation of Authorship (`.agents/agents/`):**
   Eliminatory Filter 4 (`crates/stochology-siga/src/funnel/`) and `FieldAuthorship` (`crates/stochology-core/src/card/`) forbid a single agent role from both authoring a model and approving its audit:
   - **`stochology-modeler` (`.agents/agents/stochology-modeler.md`, isolated `branch` worktree):** Expands `stochology-core`, `stochology-lenses`, `stochology-siga`, and `stochology-governance` in pure Rust (`std` only, zero external crates) anchored to `SplitMix64` (with a pre-registered seed, $\Phi_1$), tagging transcribed constants `Informative:` and hand-set inputs `Assumed:` (§II.C).
   - **`stochology-adversary` (`.agents/agents/stochology-adversary.md`, `inherit` mode — VSM Channel $S3^*$):** Acts as the independent sporadic audit channel $S3^*$ and `adversarial_reviewer`, running the 8 positive gates (`gates`, exit code `0`), the 27 negative defect proofs (`gates -- --defect <name>`, exit code `1`), and `IdentityClosureAudit`, and auditing the claim provenance (§II.C) of every derived text before publication.
   - **`stochology-scribe` (`.agents/agents/stochology-scribe.md`, `inherit` mode):** Keeps documentary and pedagogical parity between exported Rust symbols, `docs/PROTOCOL.md`, `book-src/appendix-d-protocol.md`, and the 27-chapter book with Appendices A–E (`book-src/SUMMARY.md`, `book-src/00-preface.md`..`27-empirical-synthesis.md`, `book-src/appendix-*.md`), including claim provenance (§II.C) and numeric parity with current `labs`/`gates` output.
3. **Terminal Execution Policy & Cryptographic Pre-Registration Protection ($\Phi_3$):**
   - **Allow List (Autonomous Execution):** `scripts/verify.sh`, `cargo fmt`, `cargo clippy -- -D warnings`, `cargo test`, `cargo run --release --bin labs -- --all`, and `cargo run --release --bin gates` (plus `--defect <name>`).
   - **Request Review (Mandatory Human Confirmation via Inline Diff):** Any modification to `data/SHA256SUMS`, `cards/*.yaml`, `examples/*/SHA256SUMS`, `examples/*/card_*.yaml` (shaped applications and their `protocol_profile` seals), or `crates/stochology-governance/src/vault.rs` (`TemporalVault`, enforced by `.agents/scripts/phi3_cryptographic_gate.sh` and `.agents/hooks.json`) requires human review so no agent mutates pre-registration seals or increments the attempt counter $K \ge 1$ unrecorded.

---

## VI. Mandatory Verification Before Consolidation

One command runs the full verification suite:

~~~bash
scripts/verify.sh
~~~

`scripts/verify.sh` runs every check below:

~~~bash
cargo fmt --all --check                                         # rustfmt check
cargo clippy --workspace --all-targets -- -D warnings           # zero warnings tolerated across all 5 crates
cargo test --workspace                                          # all unit and integration tests
cargo run --release --bin labs -- --all                         # all 7 methodological and empirical laboratories
cargo run --release --bin labs -- --layers                      # Lab 1: Meng, Manski, Jensen, and Performativity
cargo run --release --bin labs -- --uncertainty                 # Lab 2: Kennedy-O'Hagan GP and Info-Gap + Sniedovich
cargo run --release --bin labs -- --lenses                      # Lab 3: The Five Transversal Lenses on real data
cargo run --release --bin labs -- --invariants                  # Lab 4: Phi1-Phi5, Moulton-Campbell, Exceedance, and Chouldechova
cargo run --release --bin labs -- --siga                        # Lab 5: 4-Filter Funnel and SIGA Roadmap on the 4 YAML Cards
cargo run --release --bin labs -- --governance                  # Lab 6: Bitemporal Clock, Vault, and 4 Regimes of Drift
cargo run --release --bin labs -- --real-data                   # Lab 7: Transversal Synthesis across all 14 CSVs & 4 YAML Cards, plus one real-data instrument per pillar
cargo run --release --bin gates                                 # The 8 Epistemological Gates (G1-G8): exits with 0
cargo run --release --bin gates -- --audit-card examples/clay/card_clay_night_shift_overflow.yaml  # Shaped application under its own protocol_profile: exits with 0
cargo run --release --bin gates -- --audit-card examples/process_tracing/card_process_tracing_epidemiological_inquiry.yaml  # 5th Methodological Family (Process Tracing & Keynes Evidential Weight V) shaped application: exits with 0
cargo run --release --bin gates -- --defect cromwell            # Negative proof (Cromwell's Rule): exits with 1
cargo run --release --bin gates -- --defect phi5                # Negative proof (Phi5 Resolution): exits with 1
cargo run --release --bin gates -- --defect moulton             # Negative proof (Moulton Pseudoreplication): exits with 1
cargo run --release --bin gates -- --defect performativity      # Negative proof (RRM Divergence): exits with 1
cargo run --release --bin gates -- --defect sniedovich          # Negative proof (Info-Gap No-Man's Land): exits with 1
cargo run --release --bin gates -- --defect clock               # Negative proof (Canary Temporal Intrusion): exits with 1
cargo run --release --bin gates -- --defect vault               # Negative proof (Vault Reopening): exits with 1
cargo run --release --bin gates -- --defect concept_drift       # Negative proof (Concept Drift Circuit Breaker): exits with 1
cargo run --release --bin gates -- --defect chouldechova        # Negative proof (Chouldechova Impossibility): exits with 1
cargo run --release --bin gates -- --defect machine             # Negative proof (Almost-Perfect Machine & Authentic Form): exits with 1
cargo run --release --bin gates -- --defect analog_wall         # Negative proof (AIMC Analog Wall Threshold Cliff): exits with 1
cargo run --release --bin gates -- --defect instrumental_reason  # Negative proof (Instrumental Stochastic Reason): exits with 1
cargo run --release --bin gates -- --defect stochary_cybernetics # Negative proof (Stochary Non-Ergodicity, WDRO & PATE Q3): exits with 1
cargo run --release --bin gates -- --defect peeking             # Negative proof (Optional Stopping Peeking vs SAVI CS): exits with 1
cargo run --release --bin gates -- --defect brier_dominance     # Negative proof (Brier Ranking vs Murphy Uniform Dominance): exits with 1
cargo run --release --bin gates -- --defect var_underreport     # Negative proof (VaR/ES Underreporting via GREE E-Backtest): exits with 1
cargo run --release --bin gates -- --defect lee_point_estimate  # Negative proof (MNAR Point Estimate vs Fractional Lee Bounds): exits with 1
cargo run --release --bin gates -- --defect replace_collapse        # Negative proof (Pure Synthetic Replacement Collapse vs Accumulate): exits with 1
cargo run --release --bin gates -- --defect triadic_technium        # Negative proof (Naive Triadic Methods vs Simulated Ground Truth): exits with 1
cargo run --release --bin gates -- --defect strategic_accommodation # Negative proof (Naive Strategic AI Methods vs Six-Pair Ground Truth): exits with 1
cargo run --release --bin gates -- --defect coupled_society         # Negative proof (Naive Event-Time MNAR/RRM, SW Pooling & Rebate VCG vs Coupled SFC Society): exits with 1
cargo run --release --bin gates -- --defect crps_miscalibration     # Negative proof (Lone CRPS vs CORP-CRPS Miscalibration on vic_elec.csv): exits with 1
cargo run --release --bin gates -- --defect multiplicity            # Negative proof (Per-Hypothesis Peeking vs e-BH FDR Control): exits with 1
cargo run --release --bin gates -- --defect understated_epsilon     # Negative proof (Passive Declaration vs Measured Performative Power on nsw_experimental.csv): exits with 1
cargo run --release --bin gates -- --defect anchor_extrapolation    # Negative proof (Invariance Claim vs Anchor-Stability Gap): exits with 1
cargo run --release --bin gates -- --defect vintage_leak            # Negative proof (Latest-Vintage Backtest vs Bitemporal Clock on rtdsm_routput_vintages.csv): exits with 1
cargo run --release --bin gates -- --defect subgroup_miscalibration # Negative proof (Global CORP vs Multicalibration Violation): exits with 1
git tag --list                                                      # SemVer release tags only (vX.Y.Z)
~~~

---

## VII. Project Status

| Layer | Location | Status |
| --- | --- | --- |
| Formal Specification | `docs/PROTOCOL.md` | Consolidated specification and code mapping |
| Rust Apparatus | `crates/stochology-core`, `crates/stochology-lenses`, `crates/stochology-siga`, `crates/stochology-governance`, `crates/stochology-cli` | 5 pure-Rust crates (zero external dependencies), 2024 edition |
| Epistemological Cards | `cards/card_01_atlantic_storms_evacuation.yaml`, `cards/card_02_bank_queue_staffing.yaml`, `cards/card_03_nyc_flights_holding.yaml`, `cards/card_04_dpi_civil_defense_triage.yaml` | 4 canonical cards anchored to `data/*.csv` and audited in `gates` and `labs --siga` |
| Textbook | `book-src/SUMMARY.md`, `book-src/00-preface.md`..`27-empirical-synthesis.md`, `book-src/appendix-*.md` | Preface, 27 chapters in 7 parts (I The Act of Conjecture; II The Five Laws of the Real, tools by layer; III–V the three axes World ⇄ Model, Model ⇄ Decision, Decision ⇄ World, each split into its forward and reverse pair plus an axis synthesis; VI Cross-Pair Syntheses; VII Operation), and Appendices A–E (architecture, verification, sources, full specification, course syllabus) |
| Competency Graph | `docs/competencies.toml`, `crates/stochology-cli/src/competencies.rs`, `crates/stochology-cli/tests/competency_dag.rs` | Authorial pedagogical frame: an acyclic graph of 46 competencies replacing reader-role tracks, with 27 verified pathways (one per negative proof) and a practicum thread; the preface and syllabus blocks are generated from it and checked by `cargo test` |
| Shaped Applications (Application Profiles) | `examples/clay/`, `examples/fleet/`, `examples/credit/`, `examples/process_tracing/` | Worked examples of pre-registered `protocol_profile` configurations (`seed` 31, 40, 41, 73; including the 5th Methodological Family Process Tracing & Keynes Evidential Weight $V$) audited by `gates -- --audit-card` and `crates/stochology-cli/tests/{clay_profile,application_profiles}.rs` |
| Multi-Agent Governance Harness | `.agents/agents/{stochology-modeler,stochology-adversary,stochology-scribe}.md`, `.agents/rules/AGENTS.md`, `.agents/skills/stochology-protocol/SKILL.md`, `.agents/hooks.json` | Model-agnostic 3-subagent harness (Filter 4 separation of authorship), $\Phi_3$ cryptographic hook, and terminal execution policy |
| Empirical Datasets | `data/*.csv` (`danish_fire.csv`, `bank_calls.csv`, `nsw_experimental.csv`, `cps_observational.csv`, `djia_1885_2023.csv`, `atlantic_storms.csv`, `toronto_arrests.csv`, `nyc_flights_2013.csv`, `nyc_flights_2023.csv`, `vic_elec.csv`, `rtdsm_routput_vintages.csv` (Philadelphia Fed real-time real-output vintages); plus sealed out-of-time/out-of-domain reserves `nyc_flights_2013_h1.csv`, `nyc_flights_2023_h1.csv`, `houston_flights_2011.csv`) | Preserved and exercised by the laboratories |
| Verification | `scripts/verify.sh` | Complete positive and negative test suite |
