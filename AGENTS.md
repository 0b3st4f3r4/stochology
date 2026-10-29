# AGENTS.md — Governance, Engineering, and Orchestration Contract (Stochology — The CAPIBA Protocol)

This file is the binding contract for any agent or contributor working in this repository. The system built here is an **auditable epistemological machine**: claim, generating process, executable null model, and severe test live in the same place, with a fixed seed (`seed = 2026`), a structured Epistemological Card, and algorithmic gates that reject flawed claims. The final reader of the output is a **human** (decision-maker, student, instructor, or auditor).

This repository implements the **CAPIBA Protocol** (*Calibrated, Anytime-valid, Performative-aware, Invariant, Bitemporal Auditing Protocol*, the scientific and operational auditing protocol of **Stochology**) in **pure Rust (zero external dependencies)**:

- Full specification and architectural mapping: [`docs/PROTOCOL.md`](docs/PROTOCOL.md)

---

## I. Golden Rules — The Four Prohibitions

1. **Ban on unstructured free natural language between agents (`PROHIBITION_1`).** All communication between agents or modules passes through structured **Epistemological Cards** (`EpistemologicalCard`, in `crates/stochology-core/src/card.rs`), serialized in YAML (`cards/*.yaml`) and validated by `EpistemologicalCard::audit()`.
2. **Ban on the lone mean (`PROHIBITION_2`).** When the loss or payoff function is nonlinear, dynamics are multiplicative, or the tail is heavy, reporting only the mean is forbidden. Report the evaluation at the mean $f(\mathbb{E}[X])$, the simulated expectation $\mathbb{E}[f(X)]$, Jensen's Gap $\Delta_J = \mathbb{E}[f(X)] - f(\mathbb{E}[X])$, the median, tail quantiles ($q_{05}, q_{95}$), and the *Continuous Ranked Probability Score* ($\mathrm{CRPS}$, in `crates/stochology-core/src/layers.rs`).
3. **Ban on confusing learning with performativity (`PROHIBITION_3`).** Small predictive error does not prove that a model is true. On Bridge $L_3 \to L_1$, separate **Performative Stability** ($\theta_{\mathrm{PS}}$) from **Performative Optimality** ($\theta_{\mathrm{PO}}$, Stackelberg equilibrium), measure Wasserstein-1 sensitivity ($\varepsilon$), audit the strict contraction condition of Repeated Risk Minimization ($\frac{\varepsilon \beta}{\gamma} < 1$, in `PerformativeSystem`), and classify metric collapse across the **4 Variants of Goodhart's Law** (`GoodhartTaxonomyAudit`: Regressional, Extremal, Causal, Adversarial).
4. **Ban on dogmatic extremes and omitted attempts (`PROHIBITION_4`).** No probability may be $0\%$ or $100\%$ (Cromwell's Rule: $p \in [\varepsilon_{\text{floor}}, 1 - \varepsilon_{\text{floor}}]$). Evaluate every forecast out-of-sample via the isotonic **CORP-Murphy** decomposition using *Pool-Adjacent-Violators* ($\mathrm{BS} = \mathrm{REL}_{\mathrm{CORP}} - \mathrm{RES}_{\mathrm{CORP}} + \mathrm{UNC}_0$, in `crates/stochology-lenses/src/decision.rs`), and declare and validate the counter of attempted specifications $K \ge 1$, the evidence process (*e-process* $E_t$), and discarded findings.

---

## II. Boundary Invariants ($\Phi_1$ to $\Phi_5$)

| Invariant | Formal Requirement | Code Location |
| --- | --- | --- |
| $\Phi_1$ — Simulability | Fixed seed (`seed = 2026`, `SplitMix64`), lock-free `.split(stream_id)` / `.split_fork_join_pair(branch_tag)` substreams (`MonteCarloSplitTreeAudit`), and executable null model before any fitting | `crates/stochology-core/src/rng.rs` & `card.rs` |
| $\Phi_2$ — Independent Unit | Margin over the true independent unit; Moulton-Campbell correction for unequal clusters $\tilde{m} = \bar{m}(1 + \mathrm{CV}^2)$, threshold exceedance ICC $\rho_I(p)$, and $n_{\mathrm{eff}}(p) = \frac{n}{1 + (\tilde{m}-1)\rho_I(p)}$ | `crates/stochology-core/src/invariants.rs` (`ExceedanceDesignEffect`, `OperationalWeightingGap`) |
| $\Phi_3$ — Severe Test & Missingness | Explicit Rubin missingness classification (`MCAR`, `MAR`, `MNAR`), Meng's Identity ($\rho_{R,Y}$), non-parametric Manski bounds, SHA-256 pre-registration, and Pearl's Ladder rung | `crates/stochology-core/src/layers.rs`, `sha256.rs` & `card.rs` |
| $\Phi_4$ — Scope, Fairness & Exit | Economic boundary $p^* = \frac{C}{\eta L}$, algebraic audit of Chouldechova's trilemma ($\mathrm{FPR}_A / \mathrm{FPR}_B$ under unequal base rates), and automatic numeric exit clause | `crates/stochology-core/src/invariants.rs` (`ChouldechovaAudit`) & `card.rs` |
| $\Phi_5$ — Question Resolution | Ban on answering a daily/shift operational decision with coarse annual/monthly aggregates ($R_0$) | `crates/stochology-core/src/invariants.rs` (`verify_resolution_phi5`) |

The `gates` binary enforces invariants $\Phi_1$–$\Phi_5$ and methodological locks across eight gates (`G1` to `G8`) and exits with code `0` (approved) or `1` (blocked). For the simulated syntheses (`critical.rs`, `stocks.rs`, `triad.rs`), `G8` applies multi-seed acceptance (`stochology_lenses::acceptance`): every named claim must pass on at least 95% of the development seeds `2026..=2045` (used to decide what to keep) and on at least 95% of 200 confirmation seeds `9_000_000..=9_000_199` evaluated once after the claim set was frozen; claims marked `.det.` do not depend on the seed and must pass on every seed, and identities must close on every seed. Claims that fail either set are reported as `discarded_claims` (refuted, never approved), and the corresponding negative proofs must fail on all 20 development seeds through seed-dependent claims. `data/preregistration/acceptance_registry.txt` seals the seed sets, the pass rate, every claim name, the SHA-256 of the 18 claim-defining source files (the module closure of the three syntheses, checked by a unit test), and the confirmation pass rates; its SHA-256 is listed in `data/SHA256SUMS`, and `G8` regenerates it (`gates -- --print-acceptance-registry`) and blocks on any mismatch, so editing a claim-defining source requires re-sealing under human review.

### II.B. The Six Mnemonic Pillars of the CAPIBA Protocol

**Stochology** is the discipline; the **CAPIBA Protocol** (*Calibrated, Anytime-valid, Performative-aware, Invariant, Bitemporal Auditing Protocol*) organizes **what is verified** into **6 mnemonic pillars** (`C-A-P-I-B-A`), orthogonal to the **5 Boundary Invariants** ($\Phi_1$–$\Phi_5$), the **8 Epistemological Gates** (`G1`–`G8`), and the **8-Step SIGA Operational Roadmap** (*how the verification cycle runs*). Each pillar maps to explicit gates and a disjoint subset of the **19 negative defect proofs** (`3 + 4 + 2 + 3 + 2 + 5 = 19`):

| Pillar | Formal Verification Scope | Rust Primitives | Gates | Negative Defect Proofs (`--defect`, `exit 1`) |
| :---: | --- | --- | :---: | --- |
| **C** — **Calibrated** | Isotonic PAV `CORP` decomposition (Brier & Log scores), `MurphyDiagram` ($\int_0^1 \bar{S}_\theta\,d\theta = \frac{1}{2}\mathrm{BS}$), and Cromwell bounds ($p \in [\varepsilon, 1-\varepsilon]$). | `CorpPavDecomposition`, `MurphyDiagram`, `CorpRocMurphyTriptych`, `apply_cromwell_floor` | `G6` | `cromwell`, `brier_dominance`, `concept_drift` |
| **A** — **Anytime-valid** | Non-parametric `aGRAPA` / `GRO` betting martingales $E_t$, `SAVI` confidence sequences, `GREE` VaR/ES e-backtesting, Moulton-Campbell tail $n_{\mathrm{eff}}(p)$, and $\Phi_5$ resolution parity. | `EvidenceProcess`, `SaviConfidenceSequence`, `VarEsEBacktest`, `ExceedanceDesignEffect`, `verify_resolution_phi5` | `G2/G3/G6` | `moulton`, `peeking`, `var_underreport`, `phi5` |
| **P** — **Performative-aware** | Performative Stability ($\theta_{\mathrm{PS}}$) vs. Stackelberg Optimality ($\theta_{\mathrm{PO}}$, `PerfGD`), RRM contraction ($\frac{\varepsilon\beta}{\gamma}<1$), Goodhart's 4 variants, and `Accumulate` vs. `Replace` synthetic recursion bounds. | `PerformativeSystem`, `GoodhartTaxonomyAudit`, `GerstgrasserAccumulateAudit`, `SyntheticDataPolicy` | `G6/G8` | `performativity`, `replace_collapse` |
| **I** — **Invariant** | Causal invariance across environments (`ICP` / `NILE`), Meng's `ddc`/`DDI` + fractional Lee (2009) trimming bounds & tipping point $\delta_1^*$, anchored Kennedy-O'Hagan GP, and Info-Gap + Sniedovich Maximin floor. | `MengAudit`, `ManskiBounds`, `run_nonlinear_icp_nile`, `KennedyOHaganDiscrepancy`, `InfoGapOption`, `AuthenticModelForm` | `G3/G6/G8` | `lee_point_estimate`, `sniedovich`, `machine` |
| **B** — **Bitemporal** | Bitemporal Clock ($t_{\text{disp}} \ge t_{\text{ev}}$), synthetic canary mutual-information check ($I(\text{Canary}; Y)=0$), and single-opening SHA-256 `TemporalVault` with `PCCP` / `PLAT` governance. | `BitemporalRecord`, `BitemporalClockAudit`, `TemporalVault`, `PredeterminedChangeControlPlan`, `ProfitLossAttributionTest` | `G7` | `clock`, `vault` |
| **A** — **Auditing** | Eliminatory 4-Filter Funnel, Filter 4 cross-authorship, Chouldechova & KMR fairness impossibility audits, physical/analog `HP-INV` & wave computing, critical stochastic rationality, VSM `S3*` / `PATE` cybernetics, and Triadic ($\mathbb{C} \times \text{Fractals} \times \mathbb{T}^d$) & Technium synthesis. | `ChouldechovaAudit`, `AlertFatigueAudit`, `AnalogWaveComputingSynthesis`, `CriticalStochasticRationalitySynthesis`, `StocharyConjecturalCyberneticSynthesis`, `ComplexFractalToroidalTechniumSynthesis`, `EmpiricalTriadicTechniumAudit` | `G4/G8` | `chouldechova`, `analog_wall`, `instrumental_reason`, `stochary_cybernetics`, `triadic_technium` |

### II.C. Claim Provenance Discipline

Every quantitative claim that leaves the code — in `book-src/`, `docs/PROTOCOL.md`, `README.md`, card commentary, or any derived text (summaries, posts, slides, notebook "remixes") — carries exactly one provenance class, stated next to the number:

| Class | Meaning | Required wording |
| --- | --- | --- |
| **Measured** | Computed by the code on a real dataset in `data/*.csv` | Name the dataset (e.g. `danish_fire.csv`) |
| **Simulated** | Output of a seeded generative model | Say "simulated" and give the seed; if `G8` reports the claim in `discarded_claims`, say it is discarded |
| **Assumed** | A hand-set input or design target (e.g. the Stochological operating point $\mathrm{TPR} = 0.82$, $\mathrm{FPR} = 0.045$ in `AlertFatigueAudit`) | Say "assumed"; never present it as a result |
| **Literature (informative)** | A constant transcribed from a source and not computed (Horowitz's $640\text{ pJ}$, the IEA's $945\text{ TWh}$, Epoch AI's $300\text{T}$ tokens, Ferran Alia's noise curve, Tsinghua `ACCEL`) | Cite the source and mark it informative |

Rules, each derived from an audited failure:

1. Never set a measured value against an assumed or simulated one as if they were the same kind of evidence.
2. A toy simulation supports no claim about a named real system, product, or population.
3. Do not invent causal links between literature constants that their sources do not state, and do not turn a power ratio into an efficiency claim.
4. Give units and relative size next to a large absolute number (Jensen's Gap $\Delta_J = +9.03 \times 10^6$ squared daily calls in `bank_calls.csv` is $\approx 0.85\%$ of $f(\mathbb{E}[X])$).
5. One study does not license a claim about a whole class ("$88\%$ of the Epic Sepsis Model's alerts", not "$88\%$ of AI alerts").
6. Frame narratives as thought experiments or worked examples; never narrate a scenario from the code as a first-person event.
7. Present concepts, frameworks, or bridges that the repository does not implement (the bridges are $L_1 \to L_2$, $L_2 \to L_3$, and $L_3 \to L_1$) as authorial extensions, not as the protocol.
8. Numbers in the book must match what the current code prints; regenerate quoted `labs`/`gates` output blocks instead of editing them by hand.

`stochology-adversary` applies this table to every derived text before publication (claim, provenance class, source location, verdict); `stochology-scribe` applies it to `book-src/` and `docs/`; `stochology-modeler` tags transcribed constants `Informative:` and hand-set inputs `Assumed:` in doc comments.

---

## III. The Five Transversal Lenses, Almost-Perfect Machine, and Severe Uncertainty

The five transversal lenses and the Almost-Perfect Machine live in `crates/stochology-lenses`:

1. **Information Lens (`information.rs`):** Data Processing Inequality (DPI, 3-step Markov chain `DpiMarkovChain`), exact hierarchy of strong contraction constants ($\eta_{\chi^2}(W) = \rho_m^2 \le \eta_{\mathrm{KL}}(W) \le \eta_{\mathrm{TV}}(W)$), amplitude-constrained bound, and Fano's error floor.
2. **Time Lens (`time.rs`):** Itô variance drain ($g_{\text{time}} = \mu - \frac{1}{2}\sigma^2$), optimal Kelly fraction under asymmetric returns ($f^* = \frac{p}{a} - \frac{1-p}{b}$), absorbing ruin barriers, and Langevin/MCMC mixing time under Log-Sobolev Inequality.
3. **Scale and Tail Lens (`tail.rs`):** Hill estimator ($\hat{\alpha}_{k,n}$), moment existence regime classification ($\alpha \le 1$, $1 < \alpha \le 2$, $\alpha > 2$), and quantiles with effective tail sample size.
4. **Structure and Invariance Lens (`invariance.rs`):** Invariant Causal Prediction (ICP by intersection across environments $e \in \mathcal{E}$), environment OLS invariance penalty (`env_ols_invariance_penalties`), nonlinear ICP and NILE/IRM risk-gradient penalty over orthonormal Hermite basis (`run_nonlinear_icp_nile`), 2nd-order worst-case loss, and critical hidden confounding threshold $\Lambda^*$.
5. **Decision and Calibration Lens (`decision.rs`):** Cost-loss rule $p^* = \frac{C}{\eta L}$, Expected Value of Perfect Information ($\mathrm{EVPI}$), quantile score (pinball loss), isotonic **CORP-Murphy** decomposition via the PAV algorithm, and exact fair-coin calibration vs. decision benchmark (`ExactCoinCalibrationBenchmark`).
6. **Almost-Perfect Machine and Authentic Model Form (`machine.rs`):** $(\varepsilon, \delta)$-Almost-Perfect Machine $M = (T, \Phi, \theta)$ (`AlmostPerfectMachine`, auditing universal Barron density, DPI $\delta$-sufficiency, Fano/minimax rate optimality, Telgarsky depth-$D$ compositional efficiency $2^D$, and Gram effective dimension $d_{\mathrm{eff}} = \frac{(\mathrm{tr}\,G)^2}{\|G\|_F^2}$ across `P1`–`P5`); and First-Class Authentic Model Form (`AuthenticModelForm`, $\delta(x) + \hat{S}^*$, explicitly named discarded spurious atoms `DiscardedAtomRecord`, simultaneous closure of the 6 numerical identities `IdentityClosureAudit`, and the 5-part operational authenticity criterion $A(\text{model})$ `AuthenticityCriterionA`), enforced in Gate `G8` and `--defect machine`.
7. **Relational Ontology and Complex Systems (`relational.rs`):** 6-axis relational synthesis (`RelationalComplexSystemsSynthesis`: Nāgārjuna/RQM/OSR, Prigogine Brusselator RK4 + Schnakenberg $d_i S$, Schelling tipping, *Homo clausus*, and *Daisyworld* RK4), enforced in Gate `G8`.
8. **Physical, Analog, and Wave-Interference Computing (`analog.rs`):** 5-axis physical/analog synthesis (`AnalogWaveComputingSynthesis`: Horowitz 45nm CMOS energy wall & synthetic Model Collapse vs. neurosymbolic anchoring, exact RK4 RLC-mechanical 2nd-order ODE isomorphism & LTC networks, AIMC crossbar Ohm/Kirchhoff physics with Ferran Alia's "Analog Wall" threshold cliff vs. Noise-Aware Training (NAT), Analog ECC, Peking University's 24-bit `HP-INV` solver + recursive $4\times 4$ `BlockAMC` block solver + `ABNAT / HaLoRA` attention stability, `SIREN` $\sin(\omega_0 W x + b)$ vs. ReLU on the 2nd-order Helmholtz PDE & Stanford's Wave-Physics RNN, and the 20W brain sensory DPI funnel $1.6\text{ Gb/s} \to 10\text{ bit/s}$ + Turing deciban threshold + organoid/hybrid/photonic audit) and real-data coupler `EmpiricalWaveAnalogAudit` (`vic_elec.csv`), enforced in Gate `G8` and `--defect analog_wall`.
9. **Critical Theory of Stochastic Rationality & Algorithmic Governmentality (`critical.rs`):** 4-axis philosophical-mathematical synthesis (`CriticalStochasticRationalitySynthesis`: Adorno & Horkheimer's liquidation of the *Non-Identical* $\Delta_{\text{Nichtidentische}}$ & eternalization of the historical present, Marcuse's operational closure & Rouvroy/Berns's algorithmic preemption vs. "Stochastic Parrots" $P(w_t \mid w_{1:t-1})$ + Manheim & Garrabrant's 4 Goodhart variants `GoodhartTaxonomyAudit`, the material collapse triad of 300T human token exhaustion / $711\times$ DRAM wall / Ferran Alia's noise cliff, and dialectical overcoming via neurosymbolic logic, NAT/A-ECC/PKU 24-bit `HP-INV`, and Gibram Raul's continuous wave-interference physics) and multi-dataset coupler `EmpiricalCriticalRationalityAudit` (`danish_fire.csv`, `toronto_arrests.csv`, `atlantic_storms.csv`, `vic_elec.csv`). Claim checks run the naive positivist method when `grounded_mode = false` (truncated-Gaussian VaR, bare simulator, RRM, parrot predictor, unconstrained recursion, raw drifted readout, one-shot inversion, ReLU), identities (`all_identities_closed`) close in both modes, and token, energy, Ferran Alia, and `NAT` figures are informative. Enforced in Gate `G8` and `--defect instrumental_reason`.
10. **Stochary-Conjectural Synthesis, Ontological Stock Axioms, Supply-Chain Physics & Cybernetic Governance (`stocks.rs`):** 4-axis synthesis uniting $\sigma\tau\omicron\chi\alpha\sigma\mu\acute{o}\varsigma$ (*Technai Stochastikai* / Bernoulli *Ars Conjectandi*) and *Stocks/Buffers* (`StocharyConjecturalCyberneticSynthesis`: 150-node acyclic tree $H_{L1} = 3.4242, J_{L1} = 0.9898, H_{L2} = 4.9142, J_{L2} = 0.9828 \to$ recursive cybernetic graph + the **Four Ontological Axioms of the Stock** [Low-Pass Filter/Anti-Bullwhip, Peters 2019 Non-Ergodicity $+50\%/-40\%$ & Gambler's Ruin survival, Taleb-Douady Tail Vega convexity, Statefulness], Warren Powell's 4 universal policy meta-classes `PFA/CFA/VFA/DLA`, Karl Friston's Variational Free Energy & a simulated preposterior value-of-information check (Expected Free Energy $G(\pi)$ is cited, not computed), Judea Pearl's `do-Calculus`/Front-Door/Transportability, Dierickx & Cool's 5 accumulation laws, Scarf's $K$-convexity of $(S, s)$ policies, Simchi-Levi's $\mathrm{TTS}/\mathrm{TTR}/\mathrm{REI}$, Rockafellar-Uryasev $\beta$-CVaR + Esfahani-Kuhn WDRO, classical/modern **Supply-Chain Physics** [`SupplyChainPhysicsAudit`: Kingman VUT & Little's Law, Sterman Supply-Line & Lee-Whang Bullwhip floor, Clark-Scarf / Graves-Willems MEIO decoupling points, Eppen Risk-Pooling tail breakdown, Allon-Van Mieghem TBS dual sourcing, and Ivanov Ripple Effect VSC], Ashby/Conant/Beer `VSM` S1–S5/`S3*`/Algedonic Channel + `PLAT`/`PCCP`/`EvaluatedCompetenceCriterion`, Kahneman judgment noise audit, the 4-Quadrant **PATE** Telos-Skopos matrix, and the 5-step `StocharyStressTestNotebook`) plus 5-dataset coupler `EmpiricalStocharyCyberneticAudit` (`djia_1885_2023.csv`, `bank_calls.csv`, `danish_fire.csv`, `vic_elec.csv`, `atlantic_storms.csv`). Claim checks run the naive method when `cybernetic_shielded = false` (canonical tree, panic ordering, full exposure, myopic CFA, always-act agent, Rung-1 slope, untransported effect, $u = 0.985$ queue, silo placement, single sourcing, 4-day buffer, uncertified optimum, persistence regulator, ungated S1 channel, raw judges, outcome-only review, factor-omitting `PLAT`), identities (`all_identities_closed`) close in both modes, declared varieties and loop counts are informative, and the refuted chronological Optimizer's Curse on `danish_fire.csv` and the non-robust `vic_elec.csv` reserve coverage (0.907 forward, 0.880 with the halves swapped) are reported in `discarded_findings`. Enforced in Gate `G8` and `--defect stochary_cybernetics`.
11. **Triadic Mathematical & Physical Synthesis ($\mathbb{C} \times \text{Fractals} \times \mathbb{T}^d$), Technium / Technosphere Evolutionary Ontology, and Spring-and-Horizon Multidimensional Spiral (`triad.rs`):** 6-bridge synthesis (`ComplexFractalToroidalTechniumSynthesis`: complex Gabor `WIRE` with learned frequency vs. real `tanh` dictionary on the discrete 2D Helmholtz PDE, `Improved SIREN` phase completeness & Hughes-Minkov-Shi-Fan adjoint gradient identity; $\mathbb{C}^{n\times n} \hookrightarrow \mathbb{R}^{2n\times 2n}$ inversion identity, two-stage `BlockAMC` + `HP-INV` refinement & $\eta$-admissible hierarchical compression; $G\_0 = \frac{2e^2}{h}$ identity, state-aware PCM drift compensation, simulated Montroll-Weiss `CTRW` scaling $c^{\beta/\alpha}$ vs. Fickian $c^{1/2}$ & residual-trimmed analog ECC decoding; `PDHTE` refusal gate, Welsch-IRLS vs. OLS & Dey-Donoho $\pi^2/6$ identity; preface cycle $x_{t+1} = (5x_t+3)\bmod 16$ from $x_0 = 2026 \bmod 16 = 10$, golden Weyl discrepancy, `mixGamma` transitions, Hermite-Mehler `DFM` vs. random Fourier features & `MLLS+BCTS` vs. `BBSE` vs. classify-and-count; forced-logistic `SNA` diagnostics, `RLHF` companion-polynomial spectra, `PerfGD` vs. RRM, Phasor `FHRR`/`GC-VSA` identities, Hasani `LTC` Fused Solver vs. explicit Euler & Taleb-Douady $H(x,\Delta)$ signs) plus 5-dataset coupler `EmpiricalTriadicTechniumAudit`. Claim checks run the naive method in `shielded = false` mode, identities (`all_identities_closed`) must close in both modes, literature constants are informative only, and refuted claims are reported as `discarded_findings` (cascade curvature tolerance, `PDHTE+JK` superiority, lag-induced `RLHF` instability, out-of-time storm label shift). Enforced in Gate `G8` and `--defect triadic_technium`.

Structural and severe uncertainty modeling lives in `crates/stochology-core/src/uncertainty.rs`:

- **Kennedy-O'Hagan Structural Discrepancy (`KennedyOHaganDiscrepancy`):** Gaussian Process $\delta(x) \sim \mathcal{GP}(0, k(x, x'))$ with an anchoring constraint that prevents parameter pseudo-convergence and extrapolation overconfidence.
- **Info-Gap Decision Theory with Sniedovich's Safeguard (`InfoGapOption`):** Robustness horizon $\hat{\alpha}(q, r_c)$, opportuneness horizon $\hat{\beta}(q, r_w)$, *zero-ing* property, and Wald's global Maximin safeguard against "No-Man's Land".

---

## IV. 4-Filter Funnel, 8-Step SIGA Roadmap, and Production Governance

- **Eliminatory Funnel and SIGA Roadmap (`crates/stochology-siga`):**
  - `funnel.rs`: Evaluates the **4 Eliminatory Filters** in sequence (Filter 1: $\mathrm{EVPI}$ and $\Phi_5$; Filter 2: Moulton-Campbell $n_{\mathrm{eff}}(p)$, Markov Blanket, and Berkson collider ban; Filter 3: Simulation-Based Calibration SBC $\chi^2$ and SHELF elicitation; Filter 4: SHA-256 pre-registration, independent cross-authorship, and exit clause).
  - `roadmap.rs`: Runs the **8-Step SIGA Roadmap** (**S**urvey the Layer & Bridge; **I**nterrogate the Decision-Maker; **G**enerate the Null & Noise Physics; **A**ct with Threshold, Memory & Exit), cumulative evidence processes (*e-processes* $E_t = \prod L_i$), executive memoranda in integer natural frequencies, and `AlertFatigueAudit` on Bridge $L_2 \to L_3$ coupling the $10\text{ bit/s}$ human conscious bottleneck (Zheng & Meister) and Turing-Good additive deciban weights ($W = 10\log_{10}\mathrm{LR}$) to shift-level $\mathrm{PPV}$.
- **Bitemporal Governance, Vault, and Drift (`crates/stochology-governance`):**
  - `clock.rs`: Bitemporal Clock ($t_{\text{disp}} \ge t_{\text{ev}}$) and synthetic canary column ($I(\text{Canary}; Y) = 0$).
  - `vault.rs`: Single-opening Temporal Vault (`TemporalVault`) with SHA-256 seal and automatic downgrade to exploratory status on failure.
  - `drift.rs`: Murphy-CORP Circuit Breaker (`MurphyCorpCircuitBreaker`), adaptation to *Covariate Drift* via exponential tilt and *ExTRA-WCP-T* with Lei & Candès bound ($1 - \alpha - d_{\mathrm{TV}}$), adaptation to *Label Shift* via the 4 canonical estimators (**BBSE**, **MLLS/SLD**, **DFM**, and **OTTER**), and **Predetermined Change Control Plan (`PredeterminedChangeControlPlan`)** coupled to the **Profit and Loss Attribution Test (`ProfitLossAttributionTest` — Spearman $r_s$ + KS $D_{\mathrm{KS}}$ traffic-light)** and **`EvaluatedCompetenceCriterion`**.

---

## V. Model-Agnostic Multi-Agent Orchestration & Cryptographic Policy

This contract is model- and harness-agnostic. It governs any autonomous coding agent, LLM orchestrator, or multi-agent runtime operating on this repository (via `AGENTS.md` and `.agents/`):

1. **Pre-Implementation Structural Diagnosis (The Three Questions):**
   Before proposing or writing code, answer:
   - **Ontological Layer & Bridge:** Where can the system fail ($L_1$ World, $L_2$ Model, $L_3$ Agent, or Bridges $L_1 \to L_2$, $L_2 \to L_3$, $L_3 \to L_1$)?
   - **Stirling-Walker Uncertainty Class:** Is uncertainty `aleatory`, `epistemic_parameter`, `epistemic_specification` (anchored `KennedyOHaganDiscrepancy`), or `knightian` (`InfoGapOption` + Sniedovich's global Wald Maximin floor)?
   - **Severe Test:** What executable generative null model (`seed = 2026`, $\Phi_1$) and negative defect proof (`gates -- --defect <name>`) can falsify the claim?
2. **Filter 4 Separation of Authorship (`.agents/agents/`):**
   Eliminatory Filter 4 (`crates/stochology-siga/src/funnel.rs`) and `FieldAuthorship` (`crates/stochology-core/src/card.rs`) forbid a single agent role from both authoring a model and approving its audit:
   - **`stochology-modeler` (`.agents/agents/stochology-modeler.md`, isolated `branch` worktree):** Expands `stochology-core`, `stochology-lenses`, `stochology-siga`, and `stochology-governance` in pure Rust (`std` only, zero external crates) anchored to `SplitMix64` (`seed = 2026`, $\Phi_1$), tagging transcribed constants `Informative:` and hand-set inputs `Assumed:` (§II.C).
   - **`stochology-adversary` (`.agents/agents/stochology-adversary.md`, `inherit` mode — VSM Channel $S3^*$):** Acts as the independent sporadic audit channel $S3^*$ and `adversarial_reviewer`, running the 8 positive gates (`gates`, exit code `0`), the 19 negative defect proofs (`gates -- --defect <name>`, exit code `1`), and `IdentityClosureAudit`, and auditing the claim provenance (§II.C) of every derived text before publication.
   - **`stochology-scribe` (`.agents/agents/stochology-scribe.md`, `inherit` mode):** Keeps documentary and pedagogical parity between exported Rust symbols, `docs/PROTOCOL.md`, `book-src/appendix-protocol.md`, and `book-src/00-preface.md`..`24-sources.md`, including claim provenance (§II.C) and numeric parity with current `labs`/`gates` output.
3. **Terminal Execution Policy & Cryptographic Pre-Registration Protection ($\Phi_3$):**
   - **Allow List (Autonomous Execution):** `scripts/verify.sh`, `cargo fmt`, `cargo clippy -- -D warnings`, `cargo test`, `cargo run --release --bin labs -- --all`, and `cargo run --release --bin gates` (plus `--defect <name>`).
   - **Request Review (Mandatory Human Confirmation via Inline Diff):** Any modification to `data/SHA256SUMS`, `cards/*.yaml`, or `crates/stochology-governance/src/vault.rs` (`TemporalVault`, enforced by `.agents/scripts/phi3_cryptographic_gate.sh` and `.agents/hooks.json`) requires human review so no agent mutates pre-registration seals or increments the attempt counter $K \ge 1$ unrecorded.

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
cargo run --release --bin labs -- --real-data                   # Lab 7: Transversal Synthesis across all 13 CSVs & 4 YAML Cards
cargo run --release --bin gates                                 # The 8 Epistemological Gates (G1-G8): exits with 0
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
cargo run --release --bin gates -- --defect replace_collapse    # Negative proof (Pure Synthetic Replacement Collapse vs Accumulate): exits with 1
cargo run --release --bin gates -- --defect triadic_technium    # Negative proof (Naive Triadic Methods vs Simulated Ground Truth): exits with 1
git tag --list                                                  # Must be empty
~~~

---

## VII. Project Status (Protocol)

| Layer | Location | Status |
| --- | --- | --- |
| Protocol Specification | `docs/PROTOCOL.md` | Consolidated specification and code mapping |
| Rust Apparatus | `crates/stochology-core`, `crates/stochology-lenses`, `crates/stochology-siga`, `crates/stochology-governance`, `crates/stochology-cli` | 5 pure-Rust crates (zero external dependencies), 2021 edition |
| Epistemological Cards | `cards/card_01_atlantic_storms_evacuation.yaml`, `cards/card_02_bank_queue_staffing.yaml`, `cards/card_03_nyc_flights_holding.yaml`, `cards/card_04_dpi_civil_defense_triage.yaml` | 4 canonical cards anchored to `data/*.csv` and audited in `gates` and `labs --siga` |
| Multi-Agent Governance Harness | `.agents/agents/{stochology-modeler,stochology-adversary,stochology-scribe}.md`, `.agents/rules/AGENTS.md`, `.agents/skills/stochology-protocol/SKILL.md`, `.agents/hooks.json` | Model-agnostic 3-subagent harness (Filter 4 separation of authorship), $\Phi_3$ cryptographic hook, and terminal execution policy |
| Empirical Datasets | `data/*.csv` (`danish_fire.csv`, `bank_calls.csv`, `nsw_experimental.csv`, `cps_observational.csv`, `djia_1885_2023.csv`, `atlantic_storms.csv`, `toronto_arrests.csv`, `nyc_flights_2013.csv`, `nyc_flights_2023.csv`, `vic_elec.csv`; plus sealed out-of-time/out-of-domain reserves `nyc_flights_2013_h1.csv`, `nyc_flights_2023_h1.csv`, `houston_flights_2011.csv`) | Preserved and exercised by the laboratories |
| Verification | `scripts/verify.sh` | Complete positive and negative test suite |
