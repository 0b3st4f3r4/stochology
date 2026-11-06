#![forbid(unsafe_code)]
//! # Stochology — The Ethics of Aim (*Stókhos*) and Judgment under Uncertainty
//!
//! Pure-Rust (`std` only, zero external dependencies) library implementing **Stochology** —
//! an integrative framework and transversal research program for the theory and ethics of aim
//! under structural uncertainty — across the **Sequential Agent–World–Institution Loop** (`CommunityReserveEnv`), the
//! **Nuclear Triad** ($\mathcal{T}_{\mathrm{Stochology}}$), the **Three Canonical Operators**
//! (`EpistemicMarginState`, `VitalMarginState`, `GovernanceState`), and the **Three Collapse Propositions**
//! from [`notes/The Ethics of Aim.md`](../../../notes/The%20Ethics%20of%20Aim.md).
//!
//! ## Contiguous Triadic Architecture
//!
//! - **Kernel (`kernel.rs`):** Deterministic `SplitMix64` RNG (`DEFAULT_SEED = 2026`), FIPS 180-4 SHA-256 /
//!   HMAC-SHA-256, Welford online moments, Pool-Adjacent-Violators (PAV) isotonic projection, Cholesky SPD
//!   solver, and Gaussian quantile approximations.
//! - **Sequential Loop (`engine.rs`, Chapter 1):** `CommunityReserveEnv` comparing `NaiveOptimizer` against
//!   `StochologicalAgent` and its $2\times 2\times 2$ factorial ablations.
//! - **Axis 1 — Filter I (`epistemic.rs`, Chapters 2–4):** `EpistemicMarginState` — Canonical Operator I
//!   (Harmonic effective sample size $n_{\mathrm{eff}}$, Keynes evidential weight $V(n_{\mathrm{eff}})$,
//!   Fréchet–Manski & Lee partial-identification bounds, Cromwell–Ville *e*-processes, Meng Big Data Paradox,
//!   cluster $\mathrm{DEFF}$, Kish weighting, Chao $\hat{n}_{00}$, and bitemporal vintages).
//! - **Axis 2 — Filter II (`vital.rs`, Chapters 5–6):** `VitalMarginState` — Canonical Operator II
//!   (Jensen gap $\Delta_J$, Hill tail index $\hat{\alpha}$, `HollingTailRuptureAudit` under $\mathrm{DEFF}_{\mathrm{tail}}$,
//!   Itô variance drain & `KellySizingRegime` — Baker–McHale epistemic Kelly vs. tail-robust WDRO-CVaR Kelly,
//!   Kingman queue congestion, steering slack $\mathrm{TTS}_{\mathrm{survival}} \ge \mathrm{TTR}_{\mathrm{recovery}}$,
//!   Canguilhem–Holling resilience basin $D_t/D_0$, Wald survivorship, and Sniedovich No-Man's-Land).
//! - **Axis 3 — Filter III (`governance.rs`, Chapters 7–8):** `GovernanceState` — Canonical Operator III
//!   (Ex-Ante CORP-PAV calibration with Sparre Andersen harmonic degrees-of-freedom finite-sample debiasing $\hat{b}_{\mathrm{PAV}}(n_{\mathrm{eff}}) = H_{n_{\mathrm{eff}}}/(4 n_{\mathrm{eff}})$,
//!   `IcpIdentificationStatus` causal invariance, Kennedy–O'Hagan GP, Tian–Pearl, Perdomo/MacKenzie performativity &
//!   Goodhart; Real-Time Beer $S3^*$ algedonic bypass with $\delta_{\mathrm{cred}} = 0$, List–Pettit conjunctive
//!   veto, `KairosSynchronousPanel` & `ZhengMeisterQueueAudit` two-tier cognitive queue; Ex-Post `TemporalVault`,
//!   `TelicGrade`, Holmström–Innes liability vs Simon–Girard scapegoating, and Chouldechova fairness trilemma).
//! - **Grammar of Aim (`grammar.rs`):** the syntax layer over every chapter — the seven guard sites
//!   ([`GuardSite`]), the four verdicts ([`GrammarVerdict`]), the *télos*/*skópós* projection, the
//!   coverage enumerator ([`enumerate`]), and the shortest-counterexample search
//!   ([`shortest_violation`]), as formalized in `notes/The Grammar of Aim.md`.
//! - **Synthesis (`synthesis.rs`, Chapter 9):** `TriadicIntegrity` ($M_{\mathrm{Leontief}} = \min(S_{\mathrm{epi}}, S_{\mathrm{vital}}, S_{\mathrm{gov}})$),
//!   `CollapseProposition`, `StructuralCollapseProofAudit`, `ProcessTracingInference`, `ReflexiveBrakeAudit`, `LeontiefVsMultiplicativeAudit`,
//!   and `PreregisteredFalsifier`.

pub mod engine;
pub mod epistemic;
pub mod governance;
pub mod grammar;
pub mod kernel;
pub mod multilayer;
pub mod synthesis;
pub mod vital;

pub use engine::*;
pub use epistemic::*;
pub use governance::*;
pub use grammar::*;
pub use kernel::*;
pub use multilayer::*;
pub use synthesis::*;
pub use vital::*;
