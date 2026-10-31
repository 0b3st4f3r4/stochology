//! # `stochology` — The Science and Ethics of Aim (*Stókhos*) under Uncertainty
//!
//! Pure-Rust (`std` only, zero external dependencies) implementation of **Stochology**,
//! mirroring the disciplinary treatise [`notes/The Ethics of Aim.md`] and the 10-chapter textbook
//! (`book-src/00-preface.md` .. `09-methodological-families-reflexivity.md`) through the
//! **Sequential Agent–World–Institution Loop**.
//!
//! ## Module Map (1-to-1 with Chapters 1–9)
//! - [`rng`], [`sha256`], [`math`] — Reproducible simulation (`SplitMix64`), SHA-256 manifest sealing,
//!   and linear/isotonic mathematical primitives.
//! - [`engine`] (**Chapter 1**) — The sequential environment (`CommunityReserveEnv`), `AgentPolicy`,
//!   `NaiveOptimizer`, and progressive `StochologicalAgent`.
//! - [`inquiry`] (**Chapter 2 · Filter I, Tension 1**) — Cromwell's Rule, cost-loss framing under
//!   strong objectivity, severe power vs. agnotology, Type III framing error, *e-BH*, and multiverse specification curves.
//! - [`identification`] (**Chapter 3 · Filter I, Tension 2**) — Betting martingales (*e-processes*),
//!   anytime-valid confidence sequences, Lakatos shift ratio, Manski & Imbens–Manski bounds,
//!   Lee attrition bounds, Peterson survival bounds, and Mayo–Manski severe boundary testing.
//! - [`asymmetry`] (**Chapter 4 · Filter II, Tension 3**) — Dialectical penumbra, ordinal scale invariance
//!   (Mann–Whitney dominance vs. arithmetic mean reversal), Jensen's Gap $\Delta_J$, empirical CRPS, and Hill's tail index.
//! - [`archive`] (**Chapter 5 · Filter II, Tension 4**) — Keynes's evidential weight $V$, Meng's Big Data
//!   Paradox identity ($\rho_{R,Y}$), Campbell–Moulton cluster design effect ($\tilde{m}, n_{\mathrm{eff}}(p)$),
//!   Kish weighting floor, Chao capture-recapture ($\hat{n}_{00}$), and bitemporal vintages.
//! - [`performativity`] (**Chapter 6 · Filter III, Tension 5**) — Isotonic CORP-PAV Brier decomposition,
//!   Invariant Causal Prediction (ICP), Kennedy–O'Hagan anchored discrepancy, Tian–Pearl counterfactual bounds,
//!   Perdomo performative stability vs. optimality, MacKenzie's 4 regimes, strategic effort vs. gaming, and Goodhart's 4 variants.
//! - [`vital`] (**Chapter 7 · Filter III, Tension 6**) — Itô variance drain, Baker–McHale epistemic Kelly
//!   allocation, Canguilhem–Holling resilience basin, Scheffer's Critical Slowing Down, McEwen's allostatic load,
//!   Kingman queue slack, $\mathrm{TTS} \ge \mathrm{TTR}$, Abraham Wald's survivorship inversion, and Wald–Sniedovich Maximin floor.
//! - [`governance`] (**Chapter 8 · Tension 7**) — Sosa's virtue epistemology grades, Baron–Hershey two-stage
//!   bitemporal telic evaluation, List–Pettit discursive dilemma and conjunctive safety veto, Beer's algedonic
//!   channel under Fricker's testimonial injustice, Innes–Holmström–Milgrom contracts, Chouldechova's fairness trilemma,
//!   Zheng–Meister & Gigerenzer cognitive alert filtering, and `TemporalVault`.
//! - [`synthesis`] (**Chapter 9**) — The Nuclear Triad, the Three Collapse Propositions, qualitative/clinical
//!   *Process Tracing* with Galenic *sēmeîa*, and the Three Internal Goodhart Brakes.

pub mod archive;
pub mod asymmetry;
pub mod engine;
pub mod governance;
pub mod identification;
pub mod inquiry;
pub mod math;
pub mod performativity;
pub mod rng;
pub mod sha256;
pub mod synthesis;
pub mod vital;

pub use archive::{
    BitemporalRecord, CaptureRecaptureAudit, ClusterDesignEffect, KeynesEvidentialWeight,
    KishWeightAudit, MengBigDataDefect, VintageRevisionAudit, bitemporal_view_at, parse_quarter,
};
pub use asymmetry::{
    DialecticalPenumbra, HillTailEstimate, NonlinearSummary, OrdinalScaleAudit, TailMomentRegime,
    crps_empirical, mann_whitney_dominance,
};
pub use engine::{
    AgentDecision, AgentPolicy, ArchiveObservation, CommunityReserveEnv, NaiveOptimizer,
    StepOutcome, StochologicalAgent, TrajectoryReport,
};
pub use governance::{
    AlgedonicChannel, ChouldechovaTrilemma, CognitiveAlertFilter, ContractAndLiabilityAudit,
    DiscursiveCommittee, SosaJudgmentGrade, TelicEvaluation, TemporalVault,
};
pub use identification::{
    EvidenceProcess, LeeAttritionBounds, ManskiBounds, PetersonSurvivalBounds,
    anytime_valid_radius, lakatos_shift_ratio,
};
pub use inquiry::{
    CostLossFraming, SeverePowerAnalysis, SpecificationCurve, TypeThreeFramingAudit,
    cromwell_clamp, e_bh_discoveries, inverse_standard_normal_cdf, obeys_cromwell_rule,
};
pub use math::{
    kolmogorov_smirnov_distance, ols_ridge_fit, pav_isotonic_non_decreasing, solve_cholesky_spd,
    spearman_rank_correlation, standard_normal_cdf,
};
pub use performativity::{
    AnchoredDiscrepancy, CausalEnvironment, CorpPavDecomposition, GoodhartAudit, GoodhartVariant,
    InvariantPredictorAudit, MacKenzieRegime, PerformativeSystem, TianPearlCounterfactualBounds,
    evaluate_invariant_predictors,
};
pub use rng::{
    DEFAULT_SEED, OnlineMoments, Rng, SplitMix64, mean, median, median_absolute_deviation,
    quantile, quantile_sorted, sample_std, sample_variance,
};
pub use sha256::{hmac_sha256_hex, is_valid_sha256_hex, sha256_bytes, sha256_hex};
pub use synthesis::{
    CollapseProposition, DiagnosticClue, ProcessTracingInference, ProcessTracingTestKind,
    ReflexiveBrakeAudit, TriadicIntegrity,
};
pub use vital::{
    CanguilhemHollingBasin, EpistemicKellyAllocation, InfoGapMaximinSafeguard, VitalSlackBuffer,
    WaldSurvivorshipAudit,
};
