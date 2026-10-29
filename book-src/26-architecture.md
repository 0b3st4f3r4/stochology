# 26. Architectural Mapping of the Pure-Rust Apparatus


## 26.1 Why Pure Rust with Zero External Dependencies?

Most scientific computing projects depend on hundreds of transitive third-party packages (`pip`, `npm`, or external crates). Over a five-year horizon, transitive dependencies suffer from **supply-chain drift**: an upstream linear-algebra routine changes its tie-breaking rule, a random-number library alters its normal-distribution sampler, or a YAML parser changes how it handles floating-point precision, silently breaking reproducibility.

To guarantee that `seed = 2026` produces the **exact same bit-for-bit mathematical audit today and twenty years from today**, the entire Stochology Protocol is implemented in **100% pure Rust (2021 edition) with zero external crate dependencies**:
- **Deterministic Pseudo-Randomness & Online Moments (`stochology_core::rng`):** Implements Sebastiano Vigna's 64-bit `SplitMix64` generator with lock-free `.split(stream_id)` and `.split_fork_join_pair(branch_tag)` substream derivation (Steele, Lea & Flood 2014; Steele & Vigna 2021 LXM, verified in `MonteCarloSplitTreeAudit`), exact Box-Muller normal, Marsaglia-Tsang (2000) Gamma, Beta, Student-$t$, Exponential, and Knuth/Atkinson Poisson samplers, Welford-Terriberry 4th-order streaming moments (`OnlineMoments` with skewness, excess kurtosis, and Jarque-Bera test), and Künsch (1989) moving-block bootstrap (`block_bootstrap_ci`) directly from IEEE-754 primitives.
- **Cryptographic Hashing & Keyed Authentication (`stochology_core::sha256`):** Implements the full NIST **FIPS 180-4 SHA-256** digest and **RFC 2104 `HMAC-SHA256`** authentication (`hmac_sha256_hex`) from scratch to seal pre-registrations (`preregistration_hash`), temporal vaults (`TemporalVault`), and empirical datasets (`data/SHA256SUMS`).
- **Structured Card Parser (`stochology_core::card`):** Parses and validates `cards/*.yaml` directly into strongly typed Rust structs (`EpistemologicalCard`) without external serialization macros.

---

## 26.2 The Five-Crate Workspace Architecture

The repository is organized as a layered Cargo workspace where dependency arrows flow strictly downward from the CLI to the core crate:

<figure class="stoch-figure">
  <svg xmlns="http://www.w3.org/2000/svg" width="1060" height="420" viewBox="0 0 1060 420" fill="none" role="img" aria-labelledby="title-fig-12-crates">
  <title id="title-fig-12-crates">Five-Crate Rust Workspace Dependency Architecture</title>
  <defs>
  <marker id="cr-sky" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6.5" markerHeight="6.5" orient="auto">
  <path d="M 0 1 L 9 5 L 0 9 z" fill="var(--sky)"/>
  </marker>
  <marker id="cr-violet" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6.5" markerHeight="6.5" orient="auto">
  <path d="M 0 1 L 9 5 L 0 9 z" fill="var(--violet)"/>
  </marker>
  <marker id="cr-emerald" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6.5" markerHeight="6.5" orient="auto">
  <path d="M 0 1 L 9 5 L 0 9 z" fill="var(--emerald)"/>
  </marker>
  <style>text { font-family: "Inter", system-ui, -apple-system, "Segoe UI", Roboto, sans-serif; } .mono { font-family: "JetBrains Mono", ui-monospace, "Cascadia Code", monospace; }</style>
  </defs>
  <rect x="1" y="1" width="1058" height="418" rx="14" fill="var(--canvas)" stroke="var(--border)" stroke-width="1.5"/>
  <rect x="270" y="16" width="520" height="86" rx="12" fill="var(--elevated)" stroke="var(--amber)" stroke-width="2.4"/>
  <text x="530" y="45" text-anchor="middle" fill="var(--ink)" class="mono" font-size="16" font-weight="800">stochology-cli</text>
  <text x="530" y="68" text-anchor="middle" fill="var(--amber)" class="mono" font-size="11.5" font-weight="700">Binaries: labs (7 Labs) &amp; gates (8 Gates G1–G8)</text>
  <text x="530" y="88" text-anchor="middle" fill="var(--muted)" font-size="11.5">Empirical CSV loaders &amp; SHA-256 verification</text>
  <line x1="380" y1="102" x2="188" y2="134" stroke="var(--sky)" stroke-width="2.2" marker-end="url(#cr-sky)"/>
  <line x1="530" y1="102" x2="530" y2="134" stroke="var(--violet)" stroke-width="2.2" marker-end="url(#cr-violet)"/>
  <line x1="680" y1="102" x2="872" y2="134" stroke="var(--emerald)" stroke-width="2.2" marker-end="url(#cr-emerald)"/>
  <rect x="24" y="140" width="320" height="128" rx="12" fill="var(--elevated)" stroke="var(--sky)" stroke-width="2.2"/>
  <text x="184" y="168" text-anchor="middle" fill="var(--ink)" class="mono" font-size="14.5" font-weight="800">stochology-lenses</text>
  <text x="184" y="191" text-anchor="middle" fill="var(--sky)" class="mono" font-size="11" font-weight="700">5 Transversal Lenses + Machine</text>
  <text x="184" y="213" text-anchor="middle" fill="var(--muted)" class="mono" font-size="11">information · time · tail</text>
  <text x="184" y="233" text-anchor="middle" fill="var(--muted)" class="mono" font-size="11">invariance · decision (CORP)</text>
  <text x="184" y="253" text-anchor="middle" fill="var(--muted)" class="mono" font-size="10.5">machine · relational · analog · critical · stocks · triad</text>
  <rect x="370" y="140" width="320" height="128" rx="12" fill="var(--elevated)" stroke="var(--violet)" stroke-width="2.2"/>
  <text x="530" y="168" text-anchor="middle" fill="var(--ink)" class="mono" font-size="14.5" font-weight="800">stochology-siga</text>
  <text x="530" y="191" text-anchor="middle" fill="var(--violet)" class="mono" font-size="11" font-weight="700">Funnel &amp; Operational Roadmap</text>
  <text x="530" y="213" text-anchor="middle" fill="var(--muted)" class="mono" font-size="11">funnel (4 Eliminatory Filters)</text>
  <text x="530" y="233" text-anchor="middle" fill="var(--muted)" class="mono" font-size="11">roadmap (8-Step SIGA, SHELF,</text>
  <text x="530" y="253" text-anchor="middle" fill="var(--muted)" class="mono" font-size="11">SBC, e-process, Gigerenzer)</text>
  <rect x="716" y="140" width="320" height="128" rx="12" fill="var(--elevated)" stroke="var(--emerald)" stroke-width="2.2"/>
  <text x="876" y="168" text-anchor="middle" fill="var(--ink)" class="mono" font-size="14.5" font-weight="800">stochology-governance</text>
  <text x="876" y="191" text-anchor="middle" fill="var(--emerald)" class="mono" font-size="11" font-weight="700">Production Audit &amp; Drift</text>
  <text x="876" y="213" text-anchor="middle" fill="var(--muted)" class="mono" font-size="11">clock (Bitemporal + Canary)</text>
  <text x="876" y="233" text-anchor="middle" fill="var(--muted)" class="mono" font-size="11">vault (Single-Opening SHA-256)</text>
  <text x="876" y="253" text-anchor="middle" fill="var(--muted)" class="mono" font-size="11">drift (CORP, ExTRA, 4 Shift)</text>
  <line x1="188" y1="268" x2="365" y2="300" stroke="var(--sky)" stroke-width="2.2" marker-end="url(#cr-sky)"/>
  <line x1="530" y1="268" x2="530" y2="300" stroke="var(--violet)" stroke-width="2.2" marker-end="url(#cr-violet)"/>
  <line x1="872" y1="268" x2="695" y2="300" stroke="var(--emerald)" stroke-width="2.2" marker-end="url(#cr-emerald)"/>
  <rect x="40" y="306" width="980" height="94" rx="12" fill="var(--surface)" stroke="var(--sky)" stroke-width="2.5"/>
  <text x="530" y="336" text-anchor="middle" fill="var(--ink)" class="mono" font-size="16" font-weight="800">stochology-core (Zero External Dependencies)</text>
  <text x="530" y="360" text-anchor="middle" fill="var(--sky)" class="mono" font-size="11.5" font-weight="700">layers (L₁–L₃, Meng, Manski, Δ_J, CRPS, RRM) · uncertainty (GP, Info-Gap + Wald)</text>
  <text x="530" y="383" text-anchor="middle" fill="var(--muted)" class="mono" font-size="11.5">invariants (Φ₁–Φ₅, Moulton, Chouldechova) · card (YAML &amp; audit) · rng (SplitMix64) · sha256</text>
</svg>
  <figcaption><strong>Figure 26.1 — Five-Crate Rust Workspace Dependency Architecture</strong></figcaption>
</figure>

---

## 26.3 Complete Mapping from Textbook Chapters to Rust Modules

| Textbook Chapter & Protocol Section | Mathematical Concept / Governing Equation | Rust Module & Primary Types | Verification Command |
| --- | --- | --- | --- |
| **Ch. 10 (§1.1)** — Bridge $L\_1 \to L\_2$ | Meng's Identity ($\rho\_{R,Y}$), Rubin (`MCAR`/`MAR`/`MNAR`), Stratified Manski Bounds, Lee (2009) Trimming Bounds, and $W\_1$ | `stochology_core::layers::{MengAudit, ManskiBounds, MissingnessMechanism, empirical_wasserstein_1}` | `cargo run --release --bin labs -- --layers` |
| **Ch. 15 (§1.1)** — Bridge $L\_2 \to L\_3$ | Jensen's Gap $\Delta\_J = \mathbb{E}[f(X)] - f(\mathbb{E}[X])$, quantiles, and exact $O(n\log n)$ LOO $\mathrm{CRPS}$ | `stochology_core::layers::{NonlinearSummary, exact_loo_empirical_crps}` | `cargo run --release --bin labs -- --layers` |
| **Ch. 19 (§1.1)** — Bridge $L\_3 \to L\_1$ | Performative Prediction ($\theta\_{\mathrm{SL}}, \theta\_{\mathrm{PS}}, \theta\_{\mathrm{PO}}$), `PerfGD`, RRM contraction $\frac{\varepsilon\beta}{\gamma} < 1$, and 4 Variants of Goodhart's Law | `stochology_core::layers::{PerformativeSystem, PerformativeDiagnostic, GoodhartVariant, GoodhartTaxonomyAudit}` | `labs --layers` & `gates -- --defect performativity` |
| **Ch. 2, 11, 20 (§1.2)** — Uncertainty Typing | Cholesky Empirical Bayes Kennedy-O'Hagan GP $\delta(x)$ and Info-Gap ($\hat{\alpha}, \hat{\beta}$) + Sniedovich/Wald Safeguard | `stochology_core::uncertainty::{KennedyOHaganDiscrepancy, cholesky_decompose, InfoGapOption}` | `labs --uncertainty` & `gates -- --defect sniedovich` |
| **Ch. 9 (§1.3)** — Lens 1 (Information) | 3-step DPI Markov chain, SDPI hierarchy ($\eta\_{\chi^2} \le \eta\_{\mathrm{KL}} \le \eta\_{\mathrm{TV}}$), Blahut-Arimoto $C(W)$, and Fano's floor | `stochology_lenses::information::{DpiMarkovChain, SdpiHierarchy, blahut_arimoto_capacity, fano_error_lower_bound}` | `cargo run --release --bin labs -- --lenses` |
| **Ch. 9 (§1.3)** — Lens 2 (Time) | Itô variance drain $g\_{\text{time}} = \mu - \frac{1}{2}\sigma^2$, asymmetric & empirical Kelly $f^{\ast}$ vs. Merton, and LSI mixing | `stochology_lenses::time::{TimeKellyDiagnostic, empirical_kelly_optimal_fraction, MixingTimeAudit}` | `cargo run --release --bin labs -- --lenses` |
| **Ch. 9 (§1.3)** — Lens 3 (Scale & Tail) | Hill $\hat{\alpha}\_{k,n}$, Dekkers-Einmahl-de Haan $\hat{\xi}$, POT GPD, $\mathrm{ES}\_{0.99}$, and `tail_n_eff` | `stochology_lenses::tail::{HillTailDiagnostic, dedh_moment_tail_index, gpd_pot_fit, expected_shortfall_99}` | `cargo run --release --bin labs -- --lenses` |
| **Ch. 9 (§1.3)** — Lens 4 (Invariance) | Pearl's Ladder, linear ICP $\hat{S}^{\ast}$, Chow $F$-test, nonlinear Hermite NILE/IRM, Rosenbaum $\Lambda^{\ast}$ & Cinelli-Hazlett $RV\_q$ | `stochology_lenses::invariance::{run_icp, chow_invariance_f_stat, run_nonlinear_icp_nile, cinelli_hazlett_robustness_value}` | `cargo run --release --bin labs -- --lenses` |
| **Ch. 9 (§1.3)** — Lens 5 (Decision) | Cost-Loss $p^{\ast} = \frac{C}{\eta L}$, $\mathrm{EVPI}$, Richardson $V(p^{\ast})$, Log Score, isotonic PAV CORP-Murphy, and Fair-Coin Benchmark | `stochology_lenses::decision::{CostLossRule, CorpPavDecomposition, logarithmic_score, ExactCoinCalibrationBenchmark}` | `cargo run --release --bin labs -- --lenses` |
| **Ch. 11 (§1.3)** — Machine & Authentic Form | $(\varepsilon, \delta)$-Almost-Perfect Machine $M=(T,\Phi,\theta)$ (`P1`–`P5`) and `AuthenticModelForm` ($\delta(x)+\hat{S}^{\ast}$) | `stochology_lenses::machine::{AlmostPerfectMachine, AuthenticModelForm, IdentityClosureAudit}` | `gates` (`G8`) & `gates -- --defect machine` |
| **Ch. 22 (§1.3.7)** — Relational Ontology | *Śūnyatā*/RQM/OSR, Prigogine Brusselator RK4 + Schnakenberg $d\_i S$, Schelling tipping, *Homo clausus*, and *Daisyworld* RK4 | `stochology_lenses::relational::{RelationalComplexSystemsSynthesis, DissipativeAutopoieticAudit, DaisyworldSystem}` | `labs --lenses` & `gates` (`G8`) |
| **Ch. 13 (§1.3.8)** — Analog & Wave Computing | Horowitz energy wall, RLC-mechanical RK4 isomorphism, AIMC "Analog Wall" vs. NAT, PKU `HP-INV` + `BlockAMC`, `ABNAT / HaLoRA`, `SIREN`/Wave RNN, and Sensory DPI + Turing decibans | `stochology_lenses::analog::{AnalogWaveComputingSynthesis, AimcAnalogWallHpInvAudit, EmpiricalWaveAnalogAudit}` | `labs --lenses`, `gates` (`G8`), `--defect analog_wall` |
| **Ch. 18 (§1.3.9)** — Critical Stochastic Rationality | Adorno's *Nichtidentische* $\Delta_{\text{Nichtidentische}}$, Marcuse/Rouvroy preemption, "Stochastic Parrots" + 4 Goodhart Variants, 300T token autophagy & Alia noise cliff, and neurosymbolic/wave overcoming | `stochology_lenses::critical::{CriticalStochasticRationalitySynthesis, EmpiricalCriticalRationalityAudit}` | `labs --lenses`, `labs --real-data`, `gates` (`G8`), `--defect instrumental_reason` |
| **Ch. 14, 23 (§1.3.10–11)** — Stochary-Conjectural Synthesis, Supply-Chain Physics, Cybernetics & Triadic ($\mathbb{C} \times \text{Fractals} \times \mathbb{T}^d$) Technium | 150-node cybernetic graph, 4 Ontological Stock Axioms, Powell's 4 policy classes, Friston $G(\pi)$, Pearl Front-Door, Dierickx-Cool, Scarf $(S,s)$, Kingman VUT & MEIO decoupling (`SupplyChainPhysicsAudit`), Simchi-Levi $\mathrm{TTS}/\mathrm{TTR}$, $\beta$-CVaR + WDRO, Ashby/Conant/Beer `VSM` S1–S5/`S3*`/Algedonic + `PLAT`/`PCCP`, `PATE` Q1–Q4 matrix, Complex Gabor `WIRE` vs. `tanh` on the discrete Helmholtz PDE, $\mathbb{C}\hookrightarrow\mathbb{R}$ `BlockAMC` & admissible hierarchical compression, simulated `CTRW` scaling & trimmed analog ECC decoding, `PDHTE` refusal gate & `TailWelsch-DML`, Golden Weyl `mixGamma` & Hermite-Mehler `DFM`/`MLLS+BCTS`, and forced-logistic `SNA`/`RLHF` spectra/`PerfGD`/`FHRR`/`LTC` (identities closed in both modes; refuted claims recorded as `discarded_findings`) | `stochology_lenses::{stocks::{StocharyConjecturalCyberneticSynthesis, SupplyChainPhysicsAudit, EmpiricalStocharyCyberneticAudit}, triad::{ComplexFractalToroidalTechniumSynthesis, EmpiricalTriadicTechniumAudit}}` | `labs --lenses`, `labs --real-data`, `gates` (`G8`), `--defect stochary_cybernetics`, `--defect triadic_technium` |
| **Ch. 10, 11, 12, 15, 16, 17, 19, 20, 21, 22 (§1.3.12)** — Strategic AI Six-Pair Accommodation (taught axis by axis; which pair each axis is read under is an authorial mapping, a reading frame over existing code) | Minsky-Papert `XOR`/connectivity & *Society of Mind* voters, observational vs. interventional `SHAP`, Rubin MI vs. single-mean imputation & DEFF-aware power, RAG `DpiMarkovChain`/`PPI` judge/cost-loss cascade/outlier-protected INT4, hub-buffer cascade & Centola contagion, `AHP` rank reversal/`ICE` bias/regulatory proxy checklist/PKM provenance, Huber-Kalman & Tube-MPC (identities closed in both modes; refuted claims recorded as `discarded_findings`) | `stochology_lenses::strategic::{StrategicAiAccommodationSynthesis, MinskyPapertGeometrySocietyOfMindAudit, ShapleyCausalRubinMiDoeAudit, GenerativeRagLlmJudgeCompressionAudit, ComplexNetworkHubContagionAudit, MultiCriteriaAhpRegulatoryPkmAudit, RobustKalmanClosedLoopControlAudit, EmpiricalStrategicAccommodationAudit}` | `labs --lenses`, `gates` (`G8`), `--defect strategic_accommodation` |
| **Ch. 22 (§1.3.13)** — Coupled Public-Private Stock-Flow Consistent Society Synthesis | Closed-economy `SfcFlowMatrix` (Kalecki-Levy & Godley Three-Balances $< 10^{-12}$, exact $L\_\infty == 0.0$ permutation invariance), `H1a` $2\times 2$ Closed- vs. Open-Loop Regulator Coupling DiD (`H1aRegulatorCouplingAblation`), `H1b` Bester (1985) collateral screening vs. Stiglitz-Weiss pooling (`H1bCreditScreeningAblation`), `H2` Clarke VCG surplus routing, Green-Laffont $O(1/M)$ indirect tax leakage & Ostrom commons (`H2VcgGreenLaffontCommonsAblation`), and 3-CSV coupler (`EmpiricalCoupledSocietyAudit`; virgin confirmation block `9_000_200..=9_000_399`, `K = 6`) | `stochology_lenses::society::{CoupledPublicPrivateSocietySynthesis, EmpiricalCoupledSocietyAudit, SfcFlowMatrix, H1aRegulatorCouplingAblation, H1bCreditScreeningAblation, H2VcgGreenLaffontCommonsAblation}` | `labs --lenses`, `gates` (`G8`), `--defect coupled_society` |
| **Ch. 12, 15, 17, 19, 20 (§2)** — Contract, $\Phi\_1$–$\Phi\_5$ & Card | Prohibitions 1–4, `SplitMix64::split` (`MonteCarloSplitTreeAudit`), Moulton-Campbell $\tilde{m}$, exceedance profile $\rho\_I(p)$, Prospective Clustered MDE (`prospective_clustered_mde`), Chouldechova + KMR, `EpistemicPkmProvenanceAudit`, and YAML Card | `stochology_core::{rng::MonteCarloSplitTreeAudit, invariants::{ExceedanceDesignEffect, ChouldechovaAudit, prospective_clustered_mde}, layers::{RubinMultipleImputationAudit, EpistemicPkmProvenanceAudit}, card::EpistemologicalCard}` | `labs --invariants`, `gates`, `--defect cromwell/phi5/moulton/chouldechova` |
| **Ch. 12, 16, 24 (§3)** — Funnel & SIGA Roadmap | 4 Eliminatory Filters, Lentz Incomplete Beta & SHELF $\mathrm{Beta}(\hat{\alpha},\hat{\beta})$, 8-Step SIGA, SBC $\chi^2$, GRAPA *e-processes*, Gigerenzer trees, and `AlertFatigueAudit` ($10\text{ bit/s}$ + Turing decibans) | `stochology_siga::funnel::{evaluate_four_filter_funnel, fit_shelf_beta_prior}` & `stochology_siga::roadmap::{EvidenceProcess, NaturalFrequencyMemo, AlertFatigueAudit}` | `cargo run --release --bin labs -- --siga` |
| **Ch. 21 (§4)** — Governance, Vault & Drift | Bitemporal Clock ($t\_{\text{disp}}$), Canary, `HMAC-SHA256` Vault (`SealTampered`), CORP Breaker, `ExTRA-WCP-T`, Online `ACI`, 4 Label Shift Estimators, `PredeterminedChangeControlPlan`, `ProfitLossAttributionTest`, `EvaluatedCompetenceCriterion`, and `RegulatoryComplianceAudit` (EU AI Act + LGPD Art. 20) | `stochology_governance::{clock::*, vault::TemporalVault, drift::{AdaptiveConformalInference, PredeterminedChangeControlPlan, ProfitLossAttributionTest, EvaluatedCompetenceCriterion, RegulatoryComplianceAudit, *}}` | `labs --governance`, `gates -- --defect clock/vault/concept_drift` |
| **Ch. 25 (§5)** — Empirical Synthesis (Lab 7) | End-to-end audit across all 13 empirical CSV datasets and 4 canonical YAML cards (`PublicInfrastructureCard04Audit`) | `stochology_cli` (`src/lib.rs` & `src/bin/labs.rs`) | `cargo run --release --bin labs -- --real-data` |

---

## 26.4 Model-Agnostic Multi-Agent Governance Architecture (`AGENTS.md` & `.agents/`)

To enforce Filter 4's strict separation of authorship (`null_model` / `decision_rule` vs. `adversarial_reviewer`) and Invariant $\Phi\_3$'s cryptographic protection across any autonomous coding agent or multi-agent harness, the workspace defines three specialized subagents and a pre-tool cryptographic hook in `.agents/` alongside the root contract in `AGENTS.md`:

1. **`stochology-modeler` (`.agents/agents/stochology-modeler.md`, Mode `branch`):** Operates in an isolated Git worktree to expand `stochology-core`, `stochology-lenses`, `stochology-siga`, and `stochology-governance` in pure Rust (`std` only) anchored to `SplitMix64` (`seed = 2026`).
2. **`stochology-adversary` (`.agents/agents/stochology-adversary.md`, Mode `inherit`):** Acts as the independent VSM audit channel `S3*` and `adversarial_reviewer`, executing the 8 positive gates (`gates`) and all 21 negative defect proofs (`gates -- --defect <name>`).
3. **`stochology-scribe` (`.agents/agents/stochology-scribe.md`, Mode `inherit`):** Maintains documentary and pedagogical parity across the Rust crates, `docs/PROTOCOL.md`, `book-src/appendix-protocol.md`, and the 28 chapters of `book-src/`.
4. **Cryptographic Protection Hook (`.agents/hooks.json` & `.agents/scripts/phi3_cryptographic_gate.sh`):** Enforces autonomous execution (`Allow List`) for `scripts/verify.sh`, `cargo fmt`, `cargo clippy`, `cargo test`, `labs`, and `gates`, while requiring explicit human inline-diff approval (`Request Review` / `force_ask`) before any modification to `data/SHA256SUMS`, `cards/*.yaml`, or `crates/stochology-governance/src/vault.rs`.

---

## 26.5 Interactive Playground Architecture (`/lab` Bridge on GitHub Pages)

Every chapter of the compiled textbook (`book/`) integrates directly with the **Stochology Interactive Laboratory (`/lab`)** via `book-src/lab/index.html`, `book-src/lab/embed/index.html`, `mathjax.js`, and `theme/book.css`:

1. **Declarative DOM Contract (`.stoch-lab`):**
   Chapters declare interactive modules via `<div class="stoch-lab" data-lab="<module-id>" data-title="..." data-subtitle="..."></div>` elements. At page load, the engine hydrates each container with parameter sliders, a real-time vector stage (`<svg viewBox="0 0 680 230">`), four quantitative telemetry readouts, a one-click CLI audit copy button, a synchronized deep-link button (`Open in /lab ↗`), and an optional embedded sandbox toggle (`Sandbox (/lab/embed)`).
2. **Deterministic URL & `postMessage` Synchronization (`seed = 2026`):**
   Every slider mutation updates the query string `/lab/?module=<id>&seed=2026&<k1>=<v1>&...` and, when the sandbox `iframe` (`/lab/embed/?...`) is active, dispatches a structured `window.postMessage` payload (`{ type: "stochology:lab:state", seed: 2026, module, state, metrics, cli }`).

```mermaid
sequenceDiagram
    participant Reader as Human Reader (book/*.html)
    participant Widget as In-Page Engine (mathjax.js)
    participant Lab as Interactive Sandbox (/lab &amp; /lab/embed)
    participant CLI as Pure-Rust Binary (labs / gates)
    Reader->>Widget: Adjusts parameter sliders (seed = 2026)
    Widget->>Widget: Recomputes exact SVG curves &amp; gate status
    Widget->>Lab: Updates deep-link URL (/lab/?...) &amp; postMessage("stochology:lab:state")
    Reader->>CLI: Copies &amp; runs cargo run --release --bin labs / gates
    CLI-->>Reader: Reproduces bit-for-bit terminal audit (exit code 0 or 1)
```

<div class="stoch-lab"
     data-lab="spectral-telgarsky-deff"
     data-title="Workspace Architecture Lab: Almost-Perfect Machine M = (T, Φ, θ)"
     data-subtitle="Inspect Telgarsky depth 2^D, Gram effective dimension d_eff, and Barron rate across crates">
</div>

## Computational Audit in Rust

Verify the zero-dependency workspace structure, compile all five crates under `-D warnings`, and run the workspace test suite and all seven laboratories:

~~~bash
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo run --release --bin labs -- --all
~~~

## Exercises

1. **Zero-Dependency Crate Graph (`Cargo.toml`):** Inspect why `stochology-core` sits at the root of the dependency DAG (`stochology-core` $\to$ `stochology-lenses` $\to$ `stochology-siga` / `stochology-governance` $\to$ `stochology-cli`). Why does enforcing zero external crate dependencies across all five crates protect long-term reproducibility ($\Phi\_1$) and cryptographic auditability ($\Phi\_3$)?
2. **Interactive Deep-Link Reproducibility (`mathjax.js` & `/lab`):** Open any `.stoch-lab` widget in the textbook, adjust a slider, and inspect the generated `/lab/?module=...&seed=2026` query string. How does coupling the browser sandbox to the `Copy CLI Audit` button bridge visual intuition and terminal verification?

## Summary

- **Five pure-Rust crates** (`stochology-core`, `stochology-lenses`, `stochology-siga`, `stochology-governance`, and `stochology-cli`) implement the entire CAPIBA Protocol using only the Rust standard library (`std`), anchored to `SplitMix64` (`seed = 2026`).
- **Two CLI binaries** (`labs` for the 7 empirical/synthetic laboratories and `gates` for the 8 epistemological gates, `--audit-card <card.yaml>`, and the 21 negative defect proofs) pair with the 16 interactive `/lab` modules in `mathjax.js`.
