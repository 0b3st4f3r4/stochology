//! **Block 0 (Chapter 1 & Transversal Spine) — The Sequential Agent–World–Institution Loop (`CommunityReserveEnv`).**
//!
//! Implements the evolving didactic environment **"The Community and the Reserve"**
//! (`CommunityReserveEnv`), the [`AgentPolicy`] trait, the baseline [`NaiveOptimizer`],
//! and the progressive [`StochologicalAgent`] powered directly by the Three Canonical Type-State
//! structures ([`EpistemicMarginState`], [`VitalMarginState`], [`GovernanceState`]).

use crate::epistemic::EpistemicMarginState;
use crate::governance::{GovernanceState, SosaJudgmentGrade};
use crate::kernel::{SplitMix64, mean, ols_ridge_fit, quantile_sorted, sample_std};
use crate::synthesis::{CollapseProposition, TriadicIntegrity};
use crate::vital::VitalMarginState;

/// Partial, clustered observation delivered by the archive at step $t$.
#[derive(Debug, Clone, PartialEq)]
pub struct ArchiveObservation {
    /// Time step $t$ ($0$-based).
    pub step_t: usize,
    /// Observed demand samples grouped by community cluster.
    pub observed_clusters: Vec<Vec<f64>>,
    /// Number of unrecorded / missing individuals in the archive at step $t$.
    pub missing_count: usize,
    /// Plausible physical support $[y_{\min}, y_{\max}]$ of individual demand.
    pub demand_support: (f64, f64),
    /// Current vital reserve stock $R_t \ge 0$ (emergency capacity / liquidity).
    pub current_reserve: f64,
    /// Frontline algedonic distress multiplier $e_t \ge 1.0$ reported by field workers.
    pub frontline_distress_e_factor: f64,
}

/// Action and institutional posture chosen by an [`AgentPolicy`] at step $t$.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AgentDecision {
    /// Declared epistemic demand interval $[L_t, U_t]$ (a single point $[m, m]$ if naive).
    pub epistemic_demand_bounds: (f64, f64),
    /// Effective independent sample size $n_{\mathrm{eff}}$ recognized by the agent.
    pub recognized_n_eff: f64,
    /// Provisioned service capacity $C_t > 0$ for the shift.
    pub provisioned_capacity: f64,
    /// Vital reserve buffer $B_t \ge 0$ set aside for tail surges.
    pub vital_reserve_allocation: f64,
    /// Financial / resource growth exposure fraction $f_t \in [0, 2]$.
    pub growth_leverage_f: f64,
    /// High-powered proxy pressure $\beta_{\mathrm{proxy}} \in [0, 1]$ (high values trigger Goodhart gaming).
    pub proxy_incentive_pressure: f64,
    /// Credibility discount $\delta_{\mathrm{cred}} \in [0, 1)$ applied to frontline algedonic warnings.
    pub frontline_testimonial_discount: f64,
    /// Whether the constitutional conjunctive safety check ($S3^*$ veto / Maximin emergency brake) is active.
    pub constitutional_brake_active: bool,
}

/// Outcome of a single transition step $t \to t + 1$ in [`CommunityReserveEnv`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StepOutcome {
    /// Step index $t$.
    pub step_t: usize,
    /// True latent community mean demand $S_t$ (including uncounted vulnerable strata and tail shocks).
    pub true_latent_demand: f64,
    /// Raw archive observed mean $\bar{O}_t$ (biased downward by exclusion of vulnerable units).
    pub raw_archive_mean: f64,
    /// Declared epistemic lower and upper bounds $[L_t, U_t]$ at step $t$.
    pub epistemic_bounds: (f64, f64),
    /// Effective capacity $C_{\mathrm{eff}, t}$ deployed at step $t$.
    pub effective_capacity: f64,
    /// Whether the true latent demand fell inside the agent's declared epistemic interval $[L_t, U_t]$.
    pub epistemic_bounds_covered_truth: bool,
    /// Realized utilization $\rho_t = S_t / C_{\mathrm{eff}, t}$.
    pub realized_utilization: f64,
    /// Realized convex queue & unmet-need harm $H_t \ge 0$.
    pub realized_harm: f64,
    /// Updated vital reserve $R_{t+1} \ge 0$.
    pub next_reserve: f64,
    /// True if the system hit the absorbing ruin / collapse barrier ($R_{t+1} \le 0$) at this step.
    pub hit_ruin_barrier: bool,
    /// Sosa judgment grade for this step's decision and outcome.
    pub sosa_grade: SosaJudgmentGrade,
    /// Full control-tape decision attested by the environment (`None` if omitted by an external log).
    pub attested_decision: Option<AgentDecision>,
    /// True if the system was already in the absorbing ruin state prior to step `step_t`.
    pub already_ruined: bool,
}

/// Summary report of a full trajectory of length $T$ in [`CommunityReserveEnv`].
#[derive(Debug, Clone, PartialEq)]
pub struct TrajectoryReport {
    /// Name of the evaluated policy.
    pub policy_name: String,
    /// Total steps simulated $T$.
    pub total_steps: usize,
    /// Empirical coverage rate of the agent's epistemic bounds over the true latent demand across all $T$ steps.
    pub epistemic_coverage_rate: f64,
    /// Conditional empirical coverage rate during shock steps ($t \equiv 4 \pmod 6$, including out-of-support tail surges).
    pub shock_coverage_rate: f64,
    /// Mean recognized effective sample size $\bar{n}_{\mathrm{eff}}$ vs. raw archive size.
    pub mean_recognized_n_eff: f64,
    /// Mean realized convex social harm $\frac{1}{T}\sum_{t=1}^T H_t$.
    pub mean_social_harm: f64,
    /// Worst-case single-step harm $\max_t H_t$.
    pub max_step_harm: f64,
    /// Terminal vital reserve $R_T$.
    pub terminal_reserve: f64,
    /// Step at which irreversible absorbing ruin occurred (`None` if the community survived the entire horizon).
    pub ruin_step: Option<usize>,
    /// Fraction of steps graded as `FullyAptPerformance` or `AdroitExAnteUnluckyOutcome` under Sosa's taxonomy.
    pub apt_or_adroit_rate: f64,
    /// Terminal ecological attraction-basin width ratio $D_T / D_0 \in [0, 1]$ after cumulative McEwen allostatic wear.
    pub terminal_basin_radius_ratio: f64,
    /// Triadic structural integrity diagnosis over the trajectory.
    pub triadic_integrity: TriadicIntegrity,
    /// Step-by-step trajectory log.
    pub steps: Vec<StepOutcome>,
}

/// Multi-seed Monte Carlo summary across $S$ independent `SplitMix64` seeds (`[Simulated]`).
#[derive(Debug, Clone, PartialEq)]
pub struct MultiSeedSummary {
    /// Name of the evaluated policy.
    pub policy_name: String,
    /// Number of independent random seeds evaluated.
    pub num_seeds: usize,
    /// Mean epistemic coverage rate across seeds.
    pub mean_coverage: f64,
    /// Standard error of epistemic coverage rate across seeds.
    pub mean_coverage_se: f64,
    /// 5th and 95th percentiles of trajectory epistemic coverage rate across seeds.
    pub coverage_p05: f64,
    pub coverage_p95: f64,
    /// Mean variance-effective sample size $\bar{n}_{\mathrm{eff}}^{\mathrm{var}}$.
    pub mean_n_eff_var: f64,
    /// Mean social harm and standard error $\mathrm{SE}(\bar{H}) = s_H / \sqrt{S}$ across seeds.
    pub mean_harm: f64,
    pub mean_harm_se: f64,
    /// 5th and 95th percentiles of trajectory mean social harm across seeds.
    pub harm_p05: f64,
    pub harm_p95: f64,
    /// Fraction of trajectories hitting irreversible ruin ($R_t \le 0$) and mean terminal basin ratio $D_T / D_0$.
    pub ruin_rate: f64,
    pub mean_basin_ratio: f64,
    /// Mean triad scores $(S_{\mathrm{epi}}, S_{\mathrm{vital}}, S_{\mathrm{gov}})$.
    pub mean_triad_scores: (f64, f64, f64),
    /// Mean Leontief bottleneck score $M_{\mathrm{Leontief}}$.
    pub mean_leontief: f64,
    /// Mean compensatory arithmetic average $M_{\mathrm{arith}}$ (for false-safe comparison).
    pub mean_arithmetic: f64,
    /// Mean CES complementarity score $M_{\mathrm{CES}}(\sigma = 0.25)$.
    pub mean_ces: f64,
    /// Stability rate of the modal `CollapseProposition` diagnosis across seeds ($[0, 1]$).
    pub modal_diagnosis_rate: f64,
}

/// Full $2^3 \times 3 = 24$-cell Mechanism $\times$ Fixed-Budget Factorial Audit (`[Simulated]`),
/// isolating the marginal effect of each triad mechanism (`use_epistemic_margin`, `use_vital_margin`,
/// `use_institutional_governance`) when the reserve buffer allocation $B_{\mathrm{fixed}}$ and growth
/// leverage $f_{\mathrm{fixed}}$ are held constant across agents.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FixedBudgetFactorialAudit {
    /// Fixed reserve buffer levels tested ($B \in \{8.0, 14.0, 20.0\}$).
    pub buffer_levels: [f64; 3],
    /// Mean social harm of `FullTriad (111)` across the 3 fixed buffer levels.
    pub full_triad_mean_harm: f64,
    /// Marginal harm increase when removing only `EpistemicMargin` (`011` minus `111`) at fixed budget.
    pub marginal_epistemic_harm_delta: f64,
    /// Marginal harm increase when removing only `VitalMargin` queue/convexity sizing (`101` minus `111`) at fixed budget.
    pub marginal_vital_harm_delta: f64,
    /// Marginal harm increase when removing only `InstitutionalGovernance` (`110` minus `111`) at fixed budget.
    pub marginal_governance_harm_delta: f64,
}

/// Trait implemented by any decision-making agent interacting with [`CommunityReserveEnv`].
pub trait AgentPolicy {
    /// Human-readable name of the agent policy.
    fn name(&self) -> &str;

    /// Chooses an [`AgentDecision`] given the current [`ArchiveObservation`].
    fn act(&mut self, obs: &ArchiveObservation) -> AgentDecision;
}

/// **The Naive Optimizer (`NaiveOptimizer`):**
/// - Treats the raw archive mean $\bar{O}_t$ as the exact truth (zero epistemic margin, ignores Meng & Moulton);
/// - Sizes capacity for the point mean at $\approx 96\%$ target utilization (zero vital slack, ignores Jensen, Hill & Kingman);
/// - Uses unshrunk plug-in leverage and high-powered proxy targets (triggers Goodhart gaming);
/// - Discounts frontline testimony ($\delta_{\mathrm{cred}} = 0.60$) and lacks a constitutional $S3^*$ brake.
#[derive(Debug, Clone, Default)]
pub struct NaiveOptimizer;

impl AgentPolicy for NaiveOptimizer {
    fn name(&self) -> &str {
        "NaivePointOptimizer"
    }

    fn act(&mut self, obs: &ArchiveObservation) -> AgentDecision {
        let all_obs: Vec<f64> = obs
            .observed_clusters
            .iter()
            .flat_map(|c| c.iter().copied())
            .collect();
        let raw_mean = mean(&all_obs);
        let raw_n = (all_obs.len() as f64).max(1.0);
        let naive_se = sample_std(&all_obs) / raw_n.sqrt();
        let vital = VitalMarginState::for_shift_sizing(
            &all_obs,
            raw_mean,
            obs.current_reserve,
            raw_n,
            0.02,
        );
        AgentDecision {
            epistemic_demand_bounds: (raw_mean - 1.96 * naive_se, raw_mean + 1.96 * naive_se),
            recognized_n_eff: raw_n,
            provisioned_capacity: raw_mean / 0.95,
            vital_reserve_allocation: 0.02 * obs.current_reserve,
            growth_leverage_f: vital.plugin_kelly_fraction,
            proxy_incentive_pressure: 0.85,
            frontline_testimonial_discount: 0.60,
            constitutional_brake_active: false,
        }
    }
}

/// Progressive configuration of the [`StochologicalAgent`], allowing each chapter and laboratory
/// to enable the three safeguards of the Nuclear Triad step by step.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StochologicalAgent {
    /// **Safeguard 1 — Epistemic Margin ($S_{\mathrm{epi}}$, Chapters 2–4):** enables Manski partial-identification bounds,
    /// Moulton cluster correction, and Meng selection-defect adjustment via [`EpistemicMarginState`].
    pub use_epistemic_margin: bool,
    /// **Safeguard 2 — Vital Margin ($S_{\mathrm{vital}}$, Chapters 5–6):** enables Jensen convexity adjustment,
    /// Kingman queue slack ($\rho \le 0.76$), $\mathrm{TTS}_{\mathrm{survival}} \ge \mathrm{TTR}_{\mathrm{recovery}}$ reserve buffering,
    /// and Baker–McHale epistemic Kelly shrinkage via [`VitalMarginState`].
    pub use_vital_margin: bool,
    /// **Safeguard 3 — Institutional Governance ($S_{\mathrm{gov}}$, Chapters 7–8):** eliminates high-powered proxy gaming,
    /// removes Fricker's testimonial discount on the algedonic channel ($\delta_{\mathrm{cred}} = 0$), and enforces the
    /// List–Pettit / Beer $S3^*$ constitutional conjunctive brake via [`GovernanceState`].
    pub use_institutional_governance: bool,
}

impl StochologicalAgent {
    /// Creates a `StochologicalAgent` with explicit flags `(use_epistemic_margin, use_vital_margin, use_institutional_governance)`
    /// for full $2\times 2\times 2$ factorial ablation.
    pub const fn with_axes(
        use_epistemic_margin: bool,
        use_vital_margin: bool,
        use_institutional_governance: bool,
    ) -> Self {
        Self {
            use_epistemic_margin,
            use_vital_margin,
            use_institutional_governance,
        }
    }

    /// Full Stochological Agent with all three axes of the Nuclear Triad active.
    pub const fn full() -> Self {
        Self::with_axes(true, true, true)
    }

    /// Agent missing only the Vital Margin (illustrating **Collapse Proposition 2**: *impotent lucidity*).
    pub const fn epistemic_only() -> Self {
        Self::with_axes(true, false, true)
    }

    /// Agent missing only the Epistemic Margin (illustrating **Collapse Proposition 1**: *blind undersizing*).
    pub const fn without_epistemic_margin() -> Self {
        Self::with_axes(false, true, true)
    }

    /// Agent missing only Institutional Governance (illustrating **Collapse Proposition 3**: *cynical capture and pillaging*).
    pub const fn without_governance() -> Self {
        Self::with_axes(true, true, false)
    }
}

impl AgentPolicy for StochologicalAgent {
    fn name(&self) -> &str {
        match (
            self.use_epistemic_margin,
            self.use_vital_margin,
            self.use_institutional_governance,
        ) {
            (true, true, true) => "StochologicalAgent(FullTriad:111)",
            (false, true, true) => "DefectiveAgent(NoEpistemicMargin->Prop1:011)",
            (true, false, true) => "DefectiveAgent(NoVitalMargin->Prop2:101)",
            (true, true, false) => "DefectiveAgent(NoGovernance->Prop3:110)",
            (true, false, false) => "PartialAgent(EpistemicOnly:100)",
            (false, true, false) => "PartialAgent(VitalOnly:010)",
            (false, false, true) => "PartialAgent(GovernanceOnly:001)",
            (false, false, false) => "DefectiveAgent(AllAxesOff:000)",
        }
    }

    fn act(&mut self, obs: &ArchiveObservation) -> AgentDecision {
        let all_obs: Vec<f64> = obs
            .observed_clusters
            .iter()
            .flat_map(|c| c.iter().copied())
            .collect();
        let raw_mean = mean(&all_obs);
        let raw_n = (all_obs.len() as f64).max(1.0);
        let naive_se = sample_std(&all_obs) / raw_n.sqrt();

        // 1. Canonical Operator I — Epistemic Margin (Chapters 2–4):
        // With Epistemic Margin: Imbens–Manski bounds widened for Moulton cluster DEFF and Meng selection defect.
        // Without Epistemic Margin: standard naive i.i.d. Gaussian 95% confidence interval raw_mean ± 1.96 * s / sqrt(n).
        let (bounds, n_eff_var) = if self.use_epistemic_margin {
            let epi = EpistemicMarginState::for_shift_observation(
                &obs.observed_clusters,
                obs.missing_count,
                obs.demand_support,
            );
            (
                (epi.imbens_manski_bounds[0], epi.imbens_manski_bounds[1]),
                epi.n_eff_variance,
            )
        } else {
            (
                (raw_mean - 1.96 * naive_se, raw_mean + 1.96 * naive_se),
                raw_n,
            )
        };

        // 2. Canonical Operator II — Vital Margin (Chapters 5–6):
        // With Epistemic Margin: precautionary design demand weights the upper epistemic bound (75%) vs lower bound (25%).
        // Without Epistemic Margin: point-certain sizing on the raw archive mean (raw_mean).
        let design_demand = if self.use_epistemic_margin {
            0.25 * bounds.0 + 0.75 * bounds.1
        } else {
            raw_mean
        };
        let vital = VitalMarginState::for_shift_sizing(
            &all_obs,
            design_demand,
            obs.current_reserve,
            n_eff_var,
            0.02,
        );
        let jensen_correction = (vital.expected_eval.sqrt() - vital.sample_mean).max(0.0);

        let (capacity, reserve_alloc, leverage_f) = if self.use_vital_margin {
            let cap = (design_demand + jensen_correction) / vital.utilization_cap;
            let res = (0.30 * obs.current_reserve).max(12.0);
            (cap, res, vital.epistemic_kelly_fraction)
        } else {
            let midpoint_demand = 0.50 * (bounds.0 + bounds.1);
            (
                midpoint_demand / 0.95,
                0.02 * obs.current_reserve,
                vital.plugin_kelly_fraction,
            )
        };

        // 3. Canonical Operator III — Institutional Governance (Chapters 7–8):
        let (proxy_pressure, discount, brake) = if self.use_institutional_governance {
            (0.10, 0.0, true)
        } else {
            (0.85, 0.60, false)
        };

        AgentDecision {
            epistemic_demand_bounds: bounds,
            recognized_n_eff: n_eff_var,
            provisioned_capacity: capacity,
            vital_reserve_allocation: reserve_alloc,
            growth_leverage_f: leverage_f,
            proxy_incentive_pressure: proxy_pressure,
            frontline_testimonial_discount: discount,
            constitutional_brake_active: brake,
        }
    }
}

/// **Compensatory Buffer Stress-Test Agent (`CompensatoryBufferAgent`):**
/// Tests whether an oversized physical reserve buffer ($B_t = \max(0.50 R_t, 20.0)$) can substitute
/// for a missing epistemic margin ($S_{\mathrm{epi}} = 0$).
#[derive(Debug, Clone, Default)]
pub struct CompensatoryBufferAgent;

impl AgentPolicy for CompensatoryBufferAgent {
    fn name(&self) -> &str {
        "CompensatoryGiantBuffer(NoEpi+2xBuffer)"
    }

    fn act(&mut self, obs: &ArchiveObservation) -> AgentDecision {
        let all_obs: Vec<f64> = obs
            .observed_clusters
            .iter()
            .flat_map(|c| c.iter().copied())
            .collect();
        let raw_mean = mean(&all_obs);
        let raw_n = (all_obs.len() as f64).max(1.0);
        let naive_se = sample_std(&all_obs) / raw_n.sqrt();
        let bounds = (raw_mean - 1.96 * naive_se, raw_mean + 1.96 * naive_se);
        let design_demand = 0.35 * bounds.0 + 0.65 * bounds.1;
        let vital = VitalMarginState::for_shift_sizing(
            &all_obs,
            design_demand,
            obs.current_reserve,
            raw_n,
            0.02,
        );
        let jensen_correction = (vital.expected_eval.sqrt() - vital.sample_mean).max(0.0);
        AgentDecision {
            epistemic_demand_bounds: bounds,
            recognized_n_eff: raw_n,
            provisioned_capacity: (design_demand + jensen_correction) / vital.utilization_cap,
            vital_reserve_allocation: (0.50 * obs.current_reserve).max(20.0),
            growth_leverage_f: vital.epistemic_kelly_fraction,
            proxy_incentive_pressure: 0.10,
            frontline_testimonial_discount: 0.0,
            constitutional_brake_active: true,
        }
    }
}

/// The evolving sequential environment **"The Community and the Reserve"** (`CommunityReserveEnv`).
#[derive(Debug, Clone)]
pub struct CommunityReserveEnv {
    /// Deterministic PRNG seed.
    pub seed: u64,
    /// Number of time steps (shifts) $T$.
    pub horizon_steps: usize,
    /// Initial vital reserve $R_0 > 0$.
    pub initial_reserve: f64,
}

impl CommunityReserveEnv {
    /// Creates a new `CommunityReserveEnv` with `seed`, `horizon_steps`, and `initial_reserve`.
    pub const fn new(seed: u64, horizon_steps: usize, initial_reserve: f64) -> Self {
        Self {
            seed,
            horizon_steps,
            initial_reserve,
        }
    }

    /// Runs a complete sequential trajectory of `policy` over `self.horizon_steps` and returns its [`TrajectoryReport`].
    pub fn run_trajectory<P: AgentPolicy>(&self, policy: &mut P) -> TrajectoryReport {
        self.run_trajectory_with_thresholds(policy, 0.50, 0.88, 0.85, 1.60, 10.0)
    }

    /// Runs a sequential trajectory parameterized by threshold parameters
    /// $(\tau,\, \rho_{\mathrm{crit}},\, \rho_{1,\mathrm{crit}},\, f_{\mathrm{ceil}},\, B_{\min})$.
    pub fn run_trajectory_with_thresholds<P: AgentPolicy>(
        &self,
        policy: &mut P,
        tau: f64,
        rho_crit: f64,
        rho1_crit: f64,
        f_ceil: f64,
        b_min: f64,
    ) -> TrajectoryReport {
        self.run_trajectory_with_full_params(
            policy, tau, rho_crit, rho1_crit, f_ceil, b_min, 0.40, 12.0, 1.45, 1.80, 2.20,
        )
    }

    /// Runs a sequential trajectory parameterized by both the five evaluation thresholds
    /// $(\tau,\, \rho_{\mathrm{crit}},\, \rho_{1,\mathrm{crit}},\, f_{\mathrm{ceil}},\, B_{\min})$
    /// and the five physical/institutional data-generating process (DGP) parameters
    /// `(severe_record_prob, severe_cluster_shift, pareto_alpha, overload_coeff, proxy_erosion_coeff)`.
    #[allow(clippy::too_many_arguments)]
    pub fn run_trajectory_with_full_params<P: AgentPolicy>(
        &self,
        policy: &mut P,
        tau: f64,
        rho_crit: f64,
        rho1_crit: f64,
        f_ceil: f64,
        b_min: f64,
        severe_record_prob: f64,
        severe_cluster_shift: f64,
        pareto_alpha: f64,
        overload_coeff: f64,
        proxy_erosion_coeff: f64,
    ) -> TrajectoryReport {
        let rng = SplitMix64::new(self.seed);
        let mut reserve = self.initial_reserve.max(0.0);
        let mut ruin_step = if self.initial_reserve <= 1e-6 {
            Some(0)
        } else {
            None
        };
        let mut steps = Vec::with_capacity(self.horizon_steps);
        let mut covered_count = 0usize;
        let mut shock_steps_count = 0usize;
        let mut shock_covered_count = 0usize;
        let mut sum_n_eff = 0.0_f64;
        let mut sum_harm = 0.0_f64;
        let mut max_harm = 0.0_f64;
        let mut apt_count = 0usize;
        let mut performative_erosion = 0.0_f64;
        let mut allostatic_wear = 0.0_f64;
        let mut sum_epi_score = 0.0_f64;
        let mut sum_vital_score = 0.0_f64;
        let mut sum_gov_score = 0.0_f64;
        let mut last_basin_ratio = 1.0_f64;

        for t in 0..self.horizon_steps {
            let already_ruined = ruin_step.is_some();
            let mut step_rng = rng.split(t as u64);
            let base_demand = 22.0 + performative_erosion;
            let is_shock_step = t % 6 == 4;
            if is_shock_step {
                shock_steps_count += 1;
            }
            let tail_surge = if is_shock_step {
                step_rng.pareto(4.0, pareto_alpha.max(1.05)).min(22.0)
            } else {
                0.0
            };
            // Pre-registered extreme out-of-support tail event on final shock step (t == 22)
            let is_extreme_out_of_support_shock = is_shock_step && t == 22;

            let mut obs_clusters = Vec::with_capacity(6);
            let mut true_all = Vec::with_capacity(60);
            let mut missing_count = 0usize;
            let stress_degradation = 0.015 * allostatic_wear
                + 0.004 * performative_erosion
                + if already_ruined { 0.04 } else { 0.0 };

            for c in 0..6 {
                let cluster_shift = if c >= 4 {
                    severe_cluster_shift + 0.6 * tail_surge + 0.25 * allostatic_wear
                } else {
                    -2.5 + 0.2 * tail_surge
                };
                let shared_shock = step_rng.normal(0.0, 2.2);
                let mut c_obs = Vec::new();
                for _ in 0..10 {
                    let raw_val =
                        base_demand + cluster_shift + shared_shock + step_rng.normal(0.0, 1.2);
                    let record_prob = if c >= 4 {
                        (severe_record_prob - stress_degradation).clamp(0.10, 0.90)
                    } else {
                        0.95
                    };
                    let recorded = step_rng.bernoulli(record_prob);
                    let val = if !recorded && c >= 4 && is_extreme_out_of_support_shock {
                        // Unrecorded vulnerable units experience an out-of-support surge above nominal 65.0
                        (raw_val + 58.0).clamp(5.0, 120.0)
                    } else {
                        raw_val.clamp(5.0, 65.0)
                    };
                    true_all.push(val);
                    if recorded {
                        c_obs.push(val.clamp(5.0, 65.0));
                    } else {
                        missing_count += 1;
                    }
                }
                if c_obs.is_empty() {
                    c_obs.push(base_demand.clamp(5.0, 65.0));
                }
                obs_clusters.push(c_obs);
            }

            let true_mean = mean(&true_all);
            let frontline_e = if is_shock_step { 2.10 } else { 1.15 };
            let obs = ArchiveObservation {
                step_t: t,
                observed_clusters: obs_clusters,
                missing_count,
                demand_support: (5.0, 65.0),
                current_reserve: reserve,
                frontline_distress_e_factor: frontline_e,
            };
            let raw_obs_vec: Vec<f64> = obs
                .observed_clusters
                .iter()
                .flat_map(|c| c.iter().copied())
                .collect();
            let raw_mean = mean(&raw_obs_vec);

            let decision = policy.act(&obs);
            let covered = decision.epistemic_demand_bounds.0.is_finite()
                && decision.epistemic_demand_bounds.1.is_finite()
                && true_mean >= decision.epistemic_demand_bounds.0
                && true_mean <= decision.epistemic_demand_bounds.1;
            if covered {
                covered_count += 1;
                if is_shock_step {
                    shock_covered_count += 1;
                }
            }
            sum_n_eff += decision.recognized_n_eff;

            let gov_prelim = GovernanceState::evaluate_shift_governance(
                frontline_e.max(1.05),
                10.0,
                decision.frontline_testimonial_discount,
                6.0,
                decision.proxy_incentive_pressure,
                decision.constitutional_brake_active,
                0.5,
                false,
                false,
            );
            let buffer_scale = (10.0 / b_min.max(1.0)).sqrt();
            let algedonic_transmission =
                (1.0 - decision.frontline_testimonial_discount).clamp(0.0, 1.0);
            let epistemic_detection = if covered { 1.0 } else { 0.35 };
            // B1 & B3: Reserve buffer and provisioned capacity are bounded by available solvency (0 when ruined)
            let available_buffer = if already_ruined || reserve <= 1e-6 {
                0.0
            } else {
                decision.vital_reserve_allocation.clamp(0.0, reserve)
            };
            let funded_cap = if already_ruined || reserve <= 1e-6 {
                0.0
            } else {
                decision.provisioned_capacity.min(2.50 * reserve.max(22.0))
            };
            let surge_deployed = if !already_ruined && reserve > 1e-6 && is_shock_step {
                0.50 * buffer_scale
                    * algedonic_transmission
                    * epistemic_detection
                    * available_buffer
            } else {
                0.0
            };
            let effective_cap = if already_ruined || reserve <= 1e-6 {
                0.0
            } else {
                funded_cap + surge_deployed
            };

            let raw_ratio = if already_ruined || reserve <= 1e-6 {
                10.0
            } else {
                (true_mean / effective_cap.max(1.0)).max(0.01)
            };
            let rho = if raw_ratio <= 0.88 {
                raw_ratio
            } else {
                0.88 + 0.112 * (1.0 - (-2.2 * (raw_ratio - 0.88)).exp())
            };
            let base_true_mean = (true_mean - performative_erosion).max(5.0);
            let sizing_cap = if already_ruined {
                decision.provisioned_capacity
            } else {
                effective_cap
            };
            let base_raw_ratio = (base_true_mean / sizing_cap.max(1.0)).max(0.01);
            let base_rho = if base_raw_ratio <= 0.88 {
                base_raw_ratio
            } else {
                0.88 + 0.112 * (1.0 - (-2.2 * (base_raw_ratio - 0.88)).exp())
            };

            if !is_shock_step && base_rho > 0.85 {
                allostatic_wear = (allostatic_wear + 3.0 * (base_rho - 0.85)).min(3.0);
            }
            let effective_wear =
                (allostatic_wear + if already_ruined { 0.45 } else { 0.0 }).min(3.0);
            let crisis_burn =
                ((true_mean - funded_cap).max(0.5) * overload_coeff * 0.35 + 1.10).max(0.50);
            let vital_step = VitalMarginState::from_queue_basin_and_survival(
                rho,
                available_buffer,
                crisis_burn,
                gov_prelim.recovery_delay_ttr_discounted,
                1.80,
                10.0,
                effective_wear,
            );
            let ecological_basin_ratio = vital_step.basin_radius_ratio;
            last_basin_ratio = ecological_basin_ratio;

            let expected_shock_deficit =
                (decision.epistemic_demand_bounds.1 - funded_cap).clamp(1.0, 4.5);
            let b_required = b_min.max(
                0.18 * expected_shock_deficit
                    * gov_prelim.recovery_delay_ttr_undiscounted.clamp(1.0, 3.5),
            );
            let step_headroom = ((1.0 - base_rho) / (1.0 - rho_crit).max(0.02)).clamp(0.0, 1.0);
            let step_kelly_safety = (f_ceil - decision.growth_leverage_f).clamp(0.0, 1.0);
            let step_buffer_commit =
                (decision.vital_reserve_allocation / b_required.max(1.0)).clamp(0.0, 1.0);
            let step_stock = step_headroom.min(step_kelly_safety).min(step_buffer_commit);

            let step_csd_rho1 = (0.45 + 0.52 * base_rho).clamp(0.0, 0.99);
            let step_csd_margin =
                ((1.0 - step_csd_rho1) / (1.0 - rho1_crit).max(0.02)).clamp(0.0, 1.0);
            let step_basin = ecological_basin_ratio.min(step_csd_margin);

            sum_vital_score += step_stock.min(step_basin);

            // B3: Exogenous world-state institutional erosion when S3* constitutional brake is absent or warnings are discounted
            let exogenous_audit_shock = 0.70 + 0.30 * step_rng.next_f64();
            let ungoverned_exposure = if decision.constitutional_brake_active {
                0.05 * decision.frontline_testimonial_discount.clamp(0.0, 1.0)
            } else {
                0.55 + 0.45 * decision.frontline_testimonial_discount.clamp(0.0, 1.0)
            };
            performative_erosion = (0.75 * performative_erosion
                + proxy_erosion_coeff * ungoverned_exposure * exogenous_audit_shock)
                .min(14.0);

            // Realized world-state harm: Kingman queue wait + unmet physical demand + world-state erosion
            let delivered_cap = (effective_cap - 0.55 * performative_erosion).max(0.0);
            let unmet_demand = (true_mean - delivered_cap).max(0.0);
            let harm = vital_step.kingman_wait_multiplier
                + 0.45 * unmet_demand
                + 0.65 * performative_erosion;
            sum_harm += harm;
            if harm > max_harm {
                max_harm = harm;
            }

            // B1, B3 & G2: Strictly absorbing ruin barrier, survival-margin constitutional brake, and capacity maintenance cost
            if already_ruined || reserve <= 1e-6 {
                reserve = 0.0;
            } else {
                let surprise_expediting_drain =
                    0.12 * (true_mean - decision.epistemic_demand_bounds.1).max(0.0);
                let congestion_drain = 1.25 * (vital_step.kingman_wait_multiplier - 3.60).max(0.0);
                let testimonial_delay_excess = (gov_prelim.recovery_delay_ttr_discounted
                    - gov_prelim.recovery_delay_ttr_undiscounted)
                    .max(0.0);
                let survival_gap_deficit =
                    if !decision.constitutional_brake_active && vital_step.survival_margin < 0.0 {
                        (-vital_step.survival_margin).min(6.0)
                    } else {
                        0.0
                    };
                let extraction_drain = 2.1 * ungoverned_exposure
                    + 0.12 * testimonial_delay_excess
                    + 0.025 * survival_gap_deficit;
                let cap_maintenance_debit = 0.003 * (funded_cap - 38.0).max(0.0);
                let buffer_debit =
                    0.012 * available_buffer + 0.06 * surge_deployed + cap_maintenance_debit;
                let overload_drain = (true_mean - effective_cap).max(0.0) * overload_coeff
                    + surprise_expediting_drain
                    + congestion_drain
                    + extraction_drain
                    + buffer_debit;
                let service_fulfillment = (delivered_cap / true_mean.max(1.0)).clamp(0.0, 1.0);
                let effective_leverage_f =
                    if decision.constitutional_brake_active && vital_step.survival_margin < 0.0 {
                        decision.growth_leverage_f.min(0.55)
                    } else {
                        decision.growth_leverage_f
                    };
                let kelly_net = service_fulfillment * 3.45 * effective_leverage_f
                    - 2.4 * effective_leverage_f * effective_leverage_f;
                reserve = (reserve + kelly_net - overload_drain).max(0.0);
            }

            let hit_ruin = reserve <= 1e-6;
            if hit_ruin {
                reserve = 0.0;
                if ruin_step.is_none() {
                    ruin_step = Some(t);
                }
            }

            // B1 & B2: Step-level epistemic margin score from realized coverage, exceedance error, and archive health (never penalizing structural width)
            let epi_obs = EpistemicMarginState::for_shift_observation(
                &obs.observed_clusters,
                obs.missing_count,
                obs.demand_support,
            );
            let bound_miss = if true_mean > decision.epistemic_demand_bounds.1 {
                (true_mean - decision.epistemic_demand_bounds.1) / true_mean.max(1.0)
            } else if true_mean < decision.epistemic_demand_bounds.0 {
                (decision.epistemic_demand_bounds.0 - true_mean) / true_mean.max(1.0)
            } else {
                0.0
            };
            let archive_health = (1.0
                - 0.025 * allostatic_wear
                - 0.011 * performative_erosion
                - if hit_ruin { 0.05 } else { 0.0 })
            .clamp(0.45, 1.0);
            let step_epi = if covered {
                epi_obs.epistemic_margin_score * archive_health
            } else {
                (epi_obs.epistemic_margin_score * (0.35 - 1.25 * bound_miss).clamp(0.04, 0.35))
                    * archive_health
            };
            sum_epi_score += step_epi;

            let util_competence = ((1.0 - rho) / 0.28).clamp(0.0, 1.0);
            let ex_ante_comp = if covered && decision.constitutional_brake_active {
                (0.76 + 0.20 * util_competence).clamp(0.75, 0.96)
            } else if covered {
                (0.42 + 0.25 * util_competence).clamp(0.35, 0.68)
            } else {
                (0.20 + 0.20 * util_competence).clamp(0.15, 0.42)
            };
            let gov_step = GovernanceState::evaluate_shift_governance(
                frontline_e.max(1.05),
                10.0,
                decision.frontline_testimonial_discount,
                vital_step.survival_slack_tts,
                decision.proxy_incentive_pressure,
                decision.constitutional_brake_active,
                ex_ante_comp,
                covered && decision.constitutional_brake_active,
                !hit_ruin && harm < 12.0,
            );
            let erosion_factor = (1.0 - 0.022 * performative_erosion).clamp(0.55, 1.0);
            sum_gov_score += (gov_step.nested_leontief_score * erosion_factor).clamp(0.0, 1.0);

            if matches!(
                gov_step.telic_grade,
                SosaJudgmentGrade::FullyAptPerformance
                    | SosaJudgmentGrade::AdroitExAnteUnluckyOutcome
            ) {
                apt_count += 1;
            }

            steps.push(StepOutcome {
                step_t: t,
                true_latent_demand: true_mean,
                raw_archive_mean: raw_mean,
                epistemic_bounds: decision.epistemic_demand_bounds,
                effective_capacity: effective_cap,
                epistemic_bounds_covered_truth: covered,
                realized_utilization: rho,
                realized_harm: harm,
                next_reserve: reserve,
                hit_ruin_barrier: hit_ruin,
                sosa_grade: gov_step.telic_grade,
                attested_decision: Some(decision),
                already_ruined,
            });
        }

        let tf = self.horizon_steps.max(1) as f64;
        let cov_rate = (covered_count as f64) / tf;
        let shock_cov_rate = (shock_covered_count as f64) / (shock_steps_count.max(1) as f64);
        let mean_harm = sum_harm / tf;
        let apt_rate = (apt_count as f64) / tf;

        let s_epi =
            (0.50 * cov_rate + 0.15 * shock_cov_rate + 0.35 * (sum_epi_score / tf)).clamp(0.0, 1.0);
        let s_vital = (sum_vital_score / tf).clamp(0.0, 1.0);
        let s_gov = (sum_gov_score / tf).clamp(0.0, 1.0);
        let triad = TriadicIntegrity::evaluate(s_epi, s_vital, s_gov, tau);

        TrajectoryReport {
            policy_name: policy.name().to_string(),
            total_steps: self.horizon_steps,
            epistemic_coverage_rate: cov_rate,
            shock_coverage_rate: shock_cov_rate,
            mean_recognized_n_eff: sum_n_eff / tf,
            mean_social_harm: mean_harm,
            max_step_harm: max_harm,
            terminal_reserve: reserve,
            ruin_step,
            apt_or_adroit_rate: apt_rate,
            terminal_basin_radius_ratio: last_basin_ratio,
            triadic_integrity: triad,
            steps,
        }
    }

    /// **B4 Mechanism $\times$ Fixed-Budget Isolation Factorial (`2^3 \times 3 = 24` cells):**
    /// Evaluates all 8 mechanism combinations `(use_epi, use_vital, use_gov)` across 3 fixed reserve buffer
    /// tiers $B_{\mathrm{fixed}} \in \{8.0, 14.0, 20.0\}$, holding `provisioned_capacity` ($31.0 + 0.25 B_{\mathrm{fixed}}$)
    /// and growth leverage ($f_{\mathrm{fixed}} = 0.65$) strictly fixed at each budget tier across all 8 cells.
    pub fn evaluate_mechanism_vs_budget_factorial(&self) -> FixedBudgetFactorialAudit {
        struct FixedBudgetAgent {
            use_epi: bool,
            use_vital: bool,
            use_gov: bool,
            fixed_buffer: f64,
        }
        impl AgentPolicy for FixedBudgetAgent {
            fn name(&self) -> &str {
                "FixedBudgetFactorialAgent"
            }
            fn act(&mut self, obs: &ArchiveObservation) -> AgentDecision {
                let all_obs: Vec<f64> = obs
                    .observed_clusters
                    .iter()
                    .flat_map(|c| c.iter().copied())
                    .collect();
                let raw_mean = mean(&all_obs);
                let raw_n = (all_obs.len() as f64).max(1.0);
                let naive_se = sample_std(&all_obs) / raw_n.sqrt();
                let (bounds, n_eff_var) = if self.use_epi {
                    let epi = EpistemicMarginState::for_shift_observation(
                        &obs.observed_clusters,
                        obs.missing_count,
                        obs.demand_support,
                    );
                    (
                        (epi.imbens_manski_bounds[0], epi.imbens_manski_bounds[1]),
                        epi.n_eff_variance,
                    )
                } else {
                    (
                        (raw_mean - 1.96 * naive_se, raw_mean + 1.96 * naive_se),
                        raw_n,
                    )
                };
                // B4: Capacity and leverage are held strictly fixed at each budget tier across all 8 cells;
                // use_vital controls whether the fixed tier buffer is actively committed to queue surge release.
                let fixed_tier_capacity = 26.5 + 0.20 * self.fixed_buffer;
                let committed_buffer = if self.use_vital {
                    self.fixed_buffer
                } else {
                    0.15 * self.fixed_buffer
                };
                let (proxy_pressure, discount, brake) = if self.use_gov {
                    (0.10, 0.0, true)
                } else {
                    (0.85, 0.60, false)
                };
                AgentDecision {
                    epistemic_demand_bounds: bounds,
                    recognized_n_eff: n_eff_var,
                    provisioned_capacity: fixed_tier_capacity,
                    vital_reserve_allocation: committed_buffer,
                    growth_leverage_f: 0.65,
                    proxy_incentive_pressure: proxy_pressure,
                    frontline_testimonial_discount: discount,
                    constitutional_brake_active: brake,
                }
            }
        }

        let buffer_levels = [8.0_f64, 14.0, 20.0];
        let configs = [
            (true, true, true),
            (false, true, true),
            (true, false, true),
            (true, true, false),
            (true, false, false),
            (false, true, false),
            (false, false, true),
            (false, false, false),
        ];
        let mut cell_sums = [0.0_f64; 8];

        for &b_fix in &buffer_levels {
            for (c_idx, &(e, v, g)) in configs.iter().enumerate() {
                let rep = self.run_trajectory(&mut FixedBudgetAgent {
                    use_epi: e,
                    use_vital: v,
                    use_gov: g,
                    fixed_buffer: b_fix,
                });
                cell_sums[c_idx] += rep.mean_social_harm;
            }
        }
        let m_111 = cell_sums[0] / 3.0;
        let m_011 = cell_sums[1] / 3.0;
        let m_101 = cell_sums[2] / 3.0;
        let m_110 = cell_sums[3] / 3.0;
        FixedBudgetFactorialAudit {
            buffer_levels,
            full_triad_mean_harm: m_111,
            marginal_epistemic_harm_delta: m_011 - m_111,
            marginal_vital_harm_delta: m_101 - m_111,
            marginal_governance_harm_delta: m_110 - m_111,
        }
    }

    /// Exports all $2\times 2\times 2 = 8$ factorial cell trajectories on `self.seed` as a deterministic
    /// JSON object (`--export-lab-json`), providing the Single Source of Truth for `book-src/lab/index.html`.
    pub fn export_lab_json(&self) -> String {
        use std::fmt::Write;
        let mut out = String::from("{\n");
        let configs = [
            ("111", true, true, true),
            ("011", false, true, true),
            ("101", true, false, true),
            ("110", true, true, false),
            ("100", true, false, false),
            ("010", false, true, false),
            ("001", false, false, true),
            ("000", false, false, false),
        ];
        for (idx, &(key, e, v, g)) in configs.iter().enumerate() {
            let mut agent = StochologicalAgent::with_axes(e, v, g);
            let rep = self.run_trajectory(&mut agent);
            let diag_str = format!("{:?}", rep.triadic_integrity.diagnose);
            let ruin_str = match rep.ruin_step {
                Some(s) => s.to_string(),
                None => "null".to_string(),
            };
            let _ = writeln!(
                &mut out,
                "  \"{key}\": {{\"covRate\": {:.4}, \"shockCovRate\": {:.4}, \"meanHarm\": {:.2}, \"terminalReserve\": {:.2}, \"ruinStep\": {ruin_str}, \"diag\": \"{diag_str}\", \"sEpi\": {:.3}, \"sVital\": {:.3}, \"sGov\": {:.3}, \"mLeontief\": {:.3}, \"mArith\": {:.3}, \"steps\": [",
                rep.epistemic_coverage_rate,
                rep.shock_coverage_rate,
                rep.mean_social_harm,
                rep.terminal_reserve,
                rep.triadic_integrity.epistemic_margin,
                rep.triadic_integrity.vital_margin,
                rep.triadic_integrity.institutional_governance,
                rep.triadic_integrity.leontief_minimum,
                rep.triadic_integrity.arithmetic_mean,
            );
            for (s_idx, s) in rep.steps.iter().enumerate() {
                let is_shock = s.step_t % 6 == 4;
                let comma = if s_idx + 1 < rep.steps.len() { "," } else { "" };
                let _ = writeln!(
                    &mut out,
                    "    {{\"t\": {}, \"rawMean\": {:.2}, \"trueMean\": {:.2}, \"bounds\": [{:.2}, {:.2}], \"effCap\": {:.2}, \"rho\": {:.4}, \"harm\": {:.2}, \"reserve\": {:.2}, \"hitRuin\": {}, \"grade\": \"{:?}\", \"covered\": {}, \"isShock\": {}}}{comma}",
                    s.step_t,
                    s.raw_archive_mean,
                    s.true_latent_demand,
                    s.epistemic_bounds.0,
                    s.epistemic_bounds.1,
                    s.effective_capacity,
                    s.realized_utilization,
                    s.realized_harm,
                    s.next_reserve,
                    s.hit_ruin_barrier,
                    s.sosa_grade,
                    s.epistemic_bounds_covered_truth,
                    is_shock
                );
            }
            let cell_comma = if idx + 1 < configs.len() { "," } else { "" };
            let _ = writeln!(&mut out, "  ]}}{cell_comma}");
        }
        out.push('}');
        out
    }

    /// Evaluates a policy generator across `num_seeds` independent seeds (`start_seed..start_seed + num_seeds`).
    pub fn evaluate_multi_seed<P, F>(
        start_seed: u64,
        num_seeds: usize,
        horizon_steps: usize,
        initial_reserve: f64,
        mut make_policy: F,
    ) -> MultiSeedSummary
    where
        P: AgentPolicy,
        F: FnMut() -> P,
    {
        let n_seeds = num_seeds.max(1);
        let n_f = n_seeds as f64;
        let mut policy_name = String::new();
        let mut sum_cov = 0.0;
        let mut covs = Vec::with_capacity(n_seeds);
        let mut sum_neff = 0.0;
        let mut harms = Vec::with_capacity(n_seeds);
        let mut diag_counts = [0usize; 8];
        let mut ruins = 0usize;
        let mut sum_basin = 0.0;
        let mut sum_e = 0.0;
        let mut sum_v = 0.0;
        let mut sum_g = 0.0;
        let mut sum_leo = 0.0;
        let mut sum_arith = 0.0;
        let mut sum_ces = 0.0;

        for idx in 0..n_seeds {
            let env = Self::new(start_seed + (idx as u64), horizon_steps, initial_reserve);
            let mut pol = make_policy();
            if idx == 0 {
                policy_name = pol.name().to_string();
            }
            let rep = env.run_trajectory(&mut pol);
            sum_cov += rep.epistemic_coverage_rate;
            covs.push(rep.epistemic_coverage_rate);
            sum_neff += rep.mean_recognized_n_eff;
            harms.push(rep.mean_social_harm);
            let d_idx = match rep.triadic_integrity.diagnose {
                CollapseProposition::IntactTriad => 0,
                CollapseProposition::Proposition1BlindUndersizing => 1,
                CollapseProposition::Proposition2ImpotentLucidity => 2,
                CollapseProposition::Proposition3CynicalCapture => 3,
                CollapseProposition::CombinedPropositions1And2 => 4,
                CollapseProposition::CombinedPropositions1And3 => 5,
                CollapseProposition::CombinedPropositions2And3 => 6,
                CollapseProposition::TotalTriadicCollapse => 7,
            };
            diag_counts[d_idx] += 1;
            if rep.ruin_step.is_some() {
                ruins += 1;
            }
            sum_basin += rep.terminal_basin_radius_ratio;
            sum_e += rep.triadic_integrity.epistemic_margin;
            sum_v += rep.triadic_integrity.vital_margin;
            sum_g += rep.triadic_integrity.institutional_governance;
            sum_leo += rep.triadic_integrity.leontief_minimum;
            sum_arith += rep.triadic_integrity.arithmetic_mean;
            sum_ces += rep.triadic_integrity.ces_complement_score;
        }
        let harm_se = sample_std(&harms) / n_f.sqrt();
        let cov_se = sample_std(&covs) / n_f.sqrt();
        harms.sort_by(|a, b| a.total_cmp(b));
        covs.sort_by(|a, b| a.total_cmp(b));
        let mean_harm = mean(&harms);
        let p05 = quantile_sorted(&harms, 0.05);
        let p95 = quantile_sorted(&harms, 0.95);
        let cov_p05 = quantile_sorted(&covs, 0.05);
        let cov_p95 = quantile_sorted(&covs, 0.95);
        let max_diag = diag_counts.into_iter().max().unwrap_or(0);

        MultiSeedSummary {
            policy_name,
            num_seeds: n_seeds,
            mean_coverage: sum_cov / n_f,
            mean_coverage_se: cov_se,
            coverage_p05: cov_p05,
            coverage_p95: cov_p95,
            mean_n_eff_var: sum_neff / n_f,
            mean_harm,
            mean_harm_se: harm_se,
            harm_p05: p05,
            harm_p95: p95,
            ruin_rate: (ruins as f64) / n_f,
            mean_basin_ratio: sum_basin / n_f,
            mean_triad_scores: (sum_e / n_f, sum_v / n_f, sum_g / n_f),
            mean_leontief: sum_leo / n_f,
            mean_arithmetic: sum_arith / n_f,
            mean_ces: sum_ces / n_f,
            modal_diagnosis_rate: (max_diag as f64) / n_f,
        }
    }

    /// Negative-control benchmark: in a benign "small world" ($\rho_{R,Y} = 0$, $\rho_{\mathrm{intra}} = 0$,
    /// no Pareto surges, complete archive recording), [`NaiveOptimizer`] acting on simulated cluster observations
    /// survives all $T$ shifts without ruin ($R_T > R_0$).
    pub fn run_benign_small_world_control(&self) -> (f64, bool) {
        let rng = SplitMix64::new(self.seed);
        let mut policy = NaiveOptimizer;
        let mut reserve = self.initial_reserve.max(0.0);
        let mut hit_ruin = self.initial_reserve <= 1e-6;
        for t in 0..self.horizon_steps {
            if hit_ruin {
                reserve = 0.0;
                break;
            }
            let mut step_rng = rng.split(t as u64);
            let mut obs_clusters = Vec::with_capacity(6);
            let mut true_all = Vec::with_capacity(60);
            for _ in 0..6 {
                let mut cluster = Vec::with_capacity(10);
                for _ in 0..10 {
                    let val = (22.0 + step_rng.normal(0.0, 0.8)).clamp(5.0, 65.0);
                    let noisy_obs = (val + step_rng.normal(0.0, 0.25)).clamp(5.0, 65.0);
                    true_all.push(val);
                    cluster.push(noisy_obs);
                }
                obs_clusters.push(cluster);
            }
            let true_mean = mean(&true_all);
            let obs = ArchiveObservation {
                step_t: t,
                observed_clusters: obs_clusters,
                missing_count: 0,
                demand_support: (5.0, 65.0),
                current_reserve: reserve,
                frontline_distress_e_factor: 1.0,
            };
            let decision = policy.act(&obs);
            let cap = decision.provisioned_capacity;
            let overload = (true_mean - cap).max(0.0) * 1.8;
            reserve = (reserve + 1.10 - overload).max(0.0);
            if reserve <= 1e-6 {
                hit_ruin = true;
            }
        }
        (reserve, hit_ruin)
    }

    /// Perturbs all five reference thresholds across a $3^5 = 243$-point sensitivity grid.
    pub fn verify_threshold_sensitivity(&self) -> f64 {
        let taus = [0.35, 0.50, 0.65];
        let rho_crits = [0.84, 0.88, 0.92];
        let rho1_crits = [0.80, 0.85, 0.90];
        let f_ceils = [1.45, 1.60, 1.75];
        let b_mins = [8.0, 10.0, 12.0];
        let mut total = 0usize;
        let mut invariant = 0usize;

        for &tau in &taus {
            for &rc in &rho_crits {
                for &r1c in &rho1_crits {
                    for &fc in &f_ceils {
                        for &bm in &b_mins {
                            total += 1;
                            let r_full = self.run_trajectory_with_thresholds(
                                &mut StochologicalAgent::full(),
                                tau,
                                rc,
                                r1c,
                                fc,
                                bm,
                            );
                            let r_p1 = self.run_trajectory_with_thresholds(
                                &mut StochologicalAgent::without_epistemic_margin(),
                                tau,
                                rc,
                                r1c,
                                fc,
                                bm,
                            );
                            let r_p2 = self.run_trajectory_with_thresholds(
                                &mut StochologicalAgent::epistemic_only(),
                                tau,
                                rc,
                                r1c,
                                fc,
                                bm,
                            );
                            let r_p3 = self.run_trajectory_with_thresholds(
                                &mut StochologicalAgent::without_governance(),
                                tau,
                                rc,
                                r1c,
                                fc,
                                bm,
                            );
                            if r_full.triadic_integrity.diagnose == CollapseProposition::IntactTriad
                                && r_p1.triadic_integrity.diagnose
                                    == CollapseProposition::Proposition1BlindUndersizing
                                && r_p2.triadic_integrity.diagnose
                                    == CollapseProposition::Proposition2ImpotentLucidity
                                && r_p3.triadic_integrity.diagnose
                                    == CollapseProposition::Proposition3CynicalCapture
                            {
                                invariant += 1;
                            }
                        }
                    }
                }
            }
        }
        (invariant as f64) / (total.max(1) as f64)
    }

    /// Perturbs all five structural data-generating process (DGP) parameters
    /// `(severe_record_prob, severe_cluster_shift, pareto_alpha, overload_coeff, proxy_erosion_coeff)`
    /// across a $3^5 = 243$-world local structural sensitivity grid.
    pub fn verify_structural_perturbation_grid(&self) -> f64 {
        let rec_probs = [0.35, 0.40, 0.45];
        let shifts = [11.0, 12.0, 13.0];
        let alphas = [1.30, 1.45, 1.60];
        let overloads = [1.60, 1.80, 2.00];
        let erosions = [1.90, 2.20, 2.50];
        let mut total = 0usize;
        let mut invariant = 0usize;

        for &rp in &rec_probs {
            for &sh in &shifts {
                for &al in &alphas {
                    for &ov in &overloads {
                        for &er in &erosions {
                            total += 1;
                            let r_full = self.run_trajectory_with_full_params(
                                &mut StochologicalAgent::full(),
                                0.50,
                                0.88,
                                0.85,
                                1.60,
                                10.0,
                                rp,
                                sh,
                                al,
                                ov,
                                er,
                            );
                            let r_p1 = self.run_trajectory_with_full_params(
                                &mut StochologicalAgent::without_epistemic_margin(),
                                0.50,
                                0.88,
                                0.85,
                                1.60,
                                10.0,
                                rp,
                                sh,
                                al,
                                ov,
                                er,
                            );
                            let r_p2 = self.run_trajectory_with_full_params(
                                &mut StochologicalAgent::epistemic_only(),
                                0.50,
                                0.88,
                                0.85,
                                1.60,
                                10.0,
                                rp,
                                sh,
                                al,
                                ov,
                                er,
                            );
                            let r_p3 = self.run_trajectory_with_full_params(
                                &mut StochologicalAgent::without_governance(),
                                0.50,
                                0.88,
                                0.85,
                                1.60,
                                10.0,
                                rp,
                                sh,
                                al,
                                ov,
                                er,
                            );
                            if r_full.triadic_integrity.diagnose == CollapseProposition::IntactTriad
                                && r_p1.triadic_integrity.diagnose
                                    == CollapseProposition::Proposition1BlindUndersizing
                                && r_p2.triadic_integrity.diagnose
                                    == CollapseProposition::Proposition2ImpotentLucidity
                                && r_p3.triadic_integrity.diagnose
                                    == CollapseProposition::Proposition3CynicalCapture
                                && r_full.terminal_reserve > r_p1.terminal_reserve
                                && r_full.terminal_reserve > r_p2.terminal_reserve
                                && r_full.terminal_reserve > r_p3.terminal_reserve
                                && r_full.mean_social_harm < r_p1.mean_social_harm
                                && r_full.mean_social_harm < r_p2.mean_social_harm
                                && r_full.mean_social_harm < r_p3.mean_social_harm
                            {
                                invariant += 1;
                            }
                        }
                    }
                }
            }
        }
        (invariant as f64) / (total.max(1) as f64)
    }

    /// Evaluates a wide-band structural DGP stress grid (`rec_probs` in `[0.20, 0.40, 0.65]`,
    /// `shifts` in `[8.0, 12.0, 18.0]`, `alphas` in `[1.15, 1.45, 1.85]`, `overloads` in `[1.20, 1.80, 2.60]`,
    /// `erosions` in `[1.20, 2.20, 3.40]`), returning `(dominance_rate, intact_triad_rate)` as a measured
    /// empirical range rather than a hard-coded `100%` assertion.
    pub fn verify_wide_band_structural_sensitivity(&self) -> (f64, f64) {
        let rec_probs = [0.10, 0.40, 0.70];
        let shifts = [6.0, 12.0, 22.0];
        let alphas = [1.15, 1.45, 1.85];
        let overloads = [1.20, 1.80, 2.60];
        let erosions = [1.20, 2.20, 3.40];
        let mut total = 0usize;
        let mut dominates = 0usize;
        let mut intact = 0usize;

        for &rp in &rec_probs {
            for &sh in &shifts {
                for &al in &alphas {
                    for &ov in &overloads {
                        for &er in &erosions {
                            total += 1;
                            let r_full = self.run_trajectory_with_full_params(
                                &mut StochologicalAgent::full(),
                                0.50,
                                0.88,
                                0.85,
                                1.60,
                                10.0,
                                rp,
                                sh,
                                al,
                                ov,
                                er,
                            );
                            let r_p1 = self.run_trajectory_with_full_params(
                                &mut StochologicalAgent::without_epistemic_margin(),
                                0.50,
                                0.88,
                                0.85,
                                1.60,
                                10.0,
                                rp,
                                sh,
                                al,
                                ov,
                                er,
                            );
                            let r_p2 = self.run_trajectory_with_full_params(
                                &mut StochologicalAgent::epistemic_only(),
                                0.50,
                                0.88,
                                0.85,
                                1.60,
                                10.0,
                                rp,
                                sh,
                                al,
                                ov,
                                er,
                            );
                            let r_p3 = self.run_trajectory_with_full_params(
                                &mut StochologicalAgent::without_governance(),
                                0.50,
                                0.88,
                                0.85,
                                1.60,
                                10.0,
                                rp,
                                sh,
                                al,
                                ov,
                                er,
                            );
                            if r_full.ruin_step.is_none()
                                && r_full.terminal_reserve > self.initial_reserve
                                && r_full.terminal_reserve > r_p1.terminal_reserve
                                && r_full.mean_social_harm < r_p1.mean_social_harm
                                && r_full.mean_social_harm < r_p2.mean_social_harm
                                && r_full.mean_social_harm < r_p3.mean_social_harm
                            {
                                dominates += 1;
                            }
                            if r_full.triadic_integrity.diagnose == CollapseProposition::IntactTriad
                            {
                                intact += 1;
                            }
                        }
                    }
                }
            }
        }
        let tf = total.max(1) as f64;
        ((dominates as f64) / tf, (intact as f64) / tf)
    }
}

// ---------------------------------------------------------------------------------------------
// Friction-envelope control benchmark (model-based vs. in-simulation tuning vs. demonstration)
// ---------------------------------------------------------------------------------------------

/// Control regime of the friction-envelope benchmark.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlRegime {
    /// Receding-horizon control on a physics model whose friction coefficient was identified on
    /// the deployed surface (the MPC arm of the course benchmark).
    ModelBasedMpc,
    /// A two-parameter policy tuned by exhaustive search *inside the simulator*, at the friction
    /// level the simulator assumes (the model-free / RL arm of the course benchmark).
    InSimulationTuned,
    /// The same policy class, fitted by least squares to demonstrations recorded from the
    /// model-based controller (the RLfD arm of the course benchmark).
    DemonstrationFitted,
}

/// Outcome of one lap under one [`ControlRegime`].
#[derive(Debug, Clone, PartialEq)]
pub struct ControlLapResult {
    /// Regime that produced the lap.
    pub regime: ControlRegime,
    /// Whether the vehicle completed the lap without losing grip.
    pub completed_lap: bool,
    /// Lap time in seconds (`None` when the lap was abandoned).
    pub lap_time_s: Option<f64>,
    /// Number of steps whose friction ellipse utilisation exceeded $1$.
    pub envelope_violation_steps: usize,
    /// Worst single-step ellipse utilisation.
    pub max_utilization: f64,
    /// Step at which grip was lost (`None` if the lap stayed inside the envelope).
    pub loss_of_grip_step: Option<usize>,
    /// Mean speed over completed steps, in m/s.
    pub mean_speed: f64,
}

/// **Friction-envelope control audit** — the Stochological reading of the autonomous-race-car
/// benchmark (MPC vs. RL vs. RLfD).
///
/// Three controllers drive the same point-mass vehicle around the same closed track under a
/// friction ellipse $a_{\mathrm{long}}^2 + a_{\mathrm{lat}}^2 \le (\mu g)^2$. The track is
/// deployed at one friction level; the simulator used for tuning assumes another. The question is
/// not which controller is fastest inside the simulator, but which controller keeps the realised
/// utilisation inside the envelope when the assumed physics is wrong — the difference between
/// optimising a model and knowing the margin.
#[derive(Debug, Clone, PartialEq)]
pub struct FrictionEnvelopeControlAudit {
    /// Friction coefficient of the deployed surface.
    pub deployed_mu: f64,
    /// Friction coefficient assumed by the simulator used for tuning.
    pub simulated_mu: f64,
    /// Friction coefficient identified by the model-based controller.
    pub identified_mu: f64,
    /// Number of track segments.
    pub segment_count: usize,
    /// Length of one segment, in metres.
    pub segment_length_m: f64,
    /// Model-based controller result.
    pub model_based: ControlLapResult,
    /// In-simulation-tuned controller result.
    pub in_simulation_tuned: ControlLapResult,
    /// Demonstration-fitted controller result.
    pub demonstration_fitted: ControlLapResult,
    /// Speed target selected by the in-simulation grid search.
    pub tuned_speed_target: f64,
    /// Gain selected by the in-simulation grid search.
    pub tuned_gain: f64,
    /// Fitted demonstration coefficients $(w_0, w_1, w_2, w_3)$ over
    /// $(1, v, v^{2} \kappa_{\mathrm{now}}, v^{2} \kappa_{\mathrm{ahead}})$.
    pub demonstration_coefficients: [f64; 4],
}

impl FrictionEnvelopeControlAudit {
    /// Gravity (m/s²).
    const GRAVITY: f64 = 9.81;
    /// Track step in metres.
    const STEP_M: f64 = 2.0;
    /// Utilisation above which a step counts as a violation.
    const ENVELOPE_LIMIT: f64 = 1.0;
    /// Consecutive violating steps that end the lap in loss of grip.
    const GRIP_PATIENCE: usize = 2;

    /// Deterministic track curvature profile, in $1/\mathrm{m}$, one value per track segment.
    fn segment_curvature(segment_count: usize) -> Vec<f64> {
        (0..segment_count)
            .map(|i| match i {
                5..=7 => 1.0 / 25.0,
                11..=13 => 1.0 / 40.0,
                17..=18 => 1.0 / 20.0,
                _ => 0.0,
            })
            .collect()
    }

    /// Expands the segment profile into one curvature value per `STEP_M` step.
    fn flat_track(segment_count: usize, segment_length_m: f64) -> Vec<f64> {
        let per_segment = (segment_length_m / Self::STEP_M).round().max(1.0) as usize;
        let mut out = Vec::with_capacity(segment_count * per_segment);
        for kappa in Self::segment_curvature(segment_count) {
            out.extend(std::iter::repeat_n(kappa, per_segment));
        }
        out
    }

    fn utilization(longitudinal: f64, speed: f64, curvature: f64, mu: f64) -> f64 {
        let grip = (mu * Self::GRAVITY).max(1e-9);
        let lateral = speed * speed * curvature;
        ((longitudinal / grip).powi(2) + (lateral / grip).powi(2)).sqrt()
    }

    /// One lap under an acceleration rule `rule(speed, curvature_now, lookahead) -> a_long`.
    fn run_lap<F>(regime: ControlRegime, mu: f64, track: &[f64], rule: F) -> ControlLapResult
    where
        F: Fn(f64, f64, f64) -> f64,
    {
        let mut speed = 8.0_f64;
        let mut time = 0.0_f64;
        let mut violations = 0usize;
        let mut max_utilization = 0.0_f64;
        let mut streak = 0usize;
        let mut loss_of_grip_step = None;
        let mut steps = 0usize;
        let mut speed_sum = 0.0_f64;

        for (i, &kappa) in track.iter().enumerate() {
            if loss_of_grip_step.is_some() {
                break;
            }
            let lookahead = track[i..(i + 10).min(track.len())]
                .iter()
                .copied()
                .fold(0.0_f64, f64::max);
            let a_long = rule(speed, kappa, lookahead).clamp(-9.0, 3.0);
            let speed_next = (speed * speed + 2.0 * a_long * Self::STEP_M)
                .max(0.5)
                .sqrt();
            let util = Self::utilization(a_long, speed_next, kappa, mu);
            max_utilization = max_utilization.max(util);
            if util > Self::ENVELOPE_LIMIT {
                violations += 1;
                streak += 1;
            } else {
                streak = 0;
            }
            time += Self::STEP_M / (0.5 * (speed + speed_next)).max(0.5);
            speed_sum += speed_next;
            steps += 1;
            speed = speed_next;
            if streak >= Self::GRIP_PATIENCE {
                loss_of_grip_step = Some(i + 1);
            }
        }

        let completed = loss_of_grip_step.is_none();
        ControlLapResult {
            regime,
            completed_lap: completed,
            lap_time_s: completed.then_some(time),
            envelope_violation_steps: violations,
            max_utilization,
            loss_of_grip_step,
            mean_speed: if steps == 0 {
                0.0
            } else {
                speed_sum / steps as f64
            },
        }
    }

    /// Rule of the model-based controller: largest acceleration whose predicted utilisation stays
    /// under a fixed safety margin at the current *and* the look-ahead curvature, using the
    /// friction coefficient identified on the deployed surface.
    fn mpc_rule(speed: f64, kappa: f64, lookahead: f64, mu: f64) -> f64 {
        const MARGIN: f64 = 0.95;
        let mut best = -9.0_f64;
        for k in 0..=44 {
            let a_long = -9.0 + 11.5 * k as f64 / 44.0;
            let speed_next = (speed * speed + 2.0 * a_long * Self::STEP_M)
                .max(0.5)
                .sqrt();
            let now = Self::utilization(a_long, speed_next, kappa, mu);
            let ahead = Self::utilization(a_long, speed_next, lookahead, mu);
            if now <= MARGIN && ahead <= MARGIN {
                best = a_long;
            }
        }
        best
    }

    /// Proportional rule shared by the tuned and fitted controllers.
    fn proportional_rule(speed: f64, target: f64, gain: f64) -> f64 {
        gain * (target - speed)
    }

    /// Demonstration-fitted rule
    /// $a = w_0 + w_1 v + w_2 v^{2} \kappa_{\mathrm{now}} + w_3 v^{2} \kappa_{\mathrm{ahead}}$.
    fn fitted_rule(coefficients: &[f64; 4], speed: f64, kappa: f64, lookahead: f64) -> f64 {
        coefficients[0]
            + coefficients[1] * speed
            + coefficients[2] * speed * speed * kappa
            + coefficients[3] * speed * speed * lookahead
    }

    /// Runs the audit: grid search inside the simulator, demonstration fit, deployment at a third
    /// friction level.
    pub fn evaluate(configured_mu: f64, segment_count: usize, segment_length_m: f64) -> Self {
        let deployed_mu = configured_mu;
        let simulated_mu = 1.20;
        let identified_mu = 0.95;
        let track = Self::flat_track(segment_count, segment_length_m);

        // In-simulation tuning: exhaustive search over (gain, target) at the simulated friction.
        let mut best = (f64::INFINITY, 0.6, 10.0);
        for gi in 0..=5 {
            let gain = 0.4 + 0.2 * gi as f64;
            for ti in 0..=12 {
                let target = 8.0 + ti as f64;
                let lap = Self::run_lap(
                    ControlRegime::InSimulationTuned,
                    simulated_mu,
                    &track,
                    |v, _k, _a| Self::proportional_rule(v, target, gain),
                );
                if let (true, Some(t)) = (lap.completed_lap, lap.lap_time_s)
                    && t < best.0
                {
                    best = (t, gain, target);
                }
            }
        }

        // Demonstrations: model-based laps at the identified friction, 3% actuation noise.
        let mut rng = SplitMix64::new(2026);
        let mut rows: Vec<Vec<f64>> = Vec::new();
        let mut targets: Vec<f64> = Vec::new();
        for _ in 0..8 {
            let mut speed = 8.0_f64;
            for (i, &kappa) in track.iter().enumerate() {
                let lookahead = track[i..(i + 10).min(track.len())]
                    .iter()
                    .copied()
                    .fold(0.0_f64, f64::max);
                let a_long = Self::mpc_rule(speed, kappa, lookahead, identified_mu)
                    + 0.03 * rng.standard_normal();
                rows.push(vec![
                    1.0,
                    speed,
                    speed * speed * kappa,
                    speed * speed * lookahead,
                ]);
                targets.push(a_long.clamp(-9.0, 3.0));
                speed = (speed * speed + 2.0 * a_long * Self::STEP_M)
                    .max(0.5)
                    .sqrt();
            }
        }
        let coeffs = match ols_ridge_fit(&rows, &targets, 1e-6) {
            Some(c) if c.len() == 4 => [c[0], c[1], c[2], c[3]],
            _ => [0.0, 0.0, 0.0, 0.0],
        };

        let model_based = Self::run_lap(
            ControlRegime::ModelBasedMpc,
            deployed_mu,
            &track,
            |v, k, a| Self::mpc_rule(v, k, a, identified_mu),
        );
        let in_simulation_tuned = Self::run_lap(
            ControlRegime::InSimulationTuned,
            deployed_mu,
            &track,
            |v, _k, _a| Self::proportional_rule(v, best.2, best.1),
        );
        let demonstration_fitted = Self::run_lap(
            ControlRegime::DemonstrationFitted,
            deployed_mu,
            &track,
            |v, k, a| Self::fitted_rule(&coeffs, v, k, a),
        );

        Self {
            deployed_mu,
            simulated_mu,
            identified_mu,
            segment_count,
            segment_length_m,
            model_based,
            in_simulation_tuned,
            demonstration_fitted,
            tuned_speed_target: best.2,
            tuned_gain: best.1,
            demonstration_coefficients: coeffs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stochological_agent_survives_and_dominates_naive_and_partial_agents() {
        let env = CommunityReserveEnv::new(2026, 24, 45.0);

        let mut naive = NaiveOptimizer;
        let rep_naive = env.run_trajectory(&mut naive);
        assert!(rep_naive.epistemic_coverage_rate <= 0.25);
        assert!(rep_naive.ruin_step.is_some());
        // §7 Criterion 2 & B1: Strictly absorbing ruin — once ruined at ruin_step, reserve stays 0.0 for all subsequent steps
        let naive_ruin_t = rep_naive.ruin_step.expect("naive must hit ruin");
        assert_eq!(rep_naive.terminal_reserve, 0.0);
        for step in &rep_naive.steps[naive_ruin_t..] {
            assert_eq!(
                step.next_reserve, 0.0,
                "post-ruin step {} must have reserve == 0.0",
                step.step_t
            );
            assert!(step.hit_ruin_barrier);
        }

        let mut full = StochologicalAgent::full();
        let rep_full = env.run_trajectory(&mut full);
        // B4: Out-of-support extreme shock step makes coverage non-tautological (< 1.0) while remaining > 0.85
        assert!(rep_full.epistemic_coverage_rate > 0.85 && rep_full.epistemic_coverage_rate < 1.0);
        assert!(rep_full.shock_coverage_rate >= 0.70 && rep_full.shock_coverage_rate < 1.0);
        assert!(rep_full.ruin_step.is_none());
        assert!(rep_full.terminal_reserve > env.initial_reserve);
        assert!(rep_full.mean_social_harm < rep_naive.mean_social_harm * 0.25);
        assert_eq!(
            rep_full.triadic_integrity.diagnose,
            CollapseProposition::IntactTriad
        );

        let mut no_epi = StochologicalAgent::without_epistemic_margin();
        let rep_p1 = env.run_trajectory(&mut no_epi);
        assert!(rep_p1.epistemic_coverage_rate <= 0.25);
        assert!(rep_p1.mean_social_harm > rep_full.mean_social_harm * 1.40);
        assert!(rep_p1.terminal_reserve < env.initial_reserve);
        assert_eq!(
            rep_p1.triadic_integrity.diagnose,
            CollapseProposition::Proposition1BlindUndersizing
        );

        let mut no_vital = StochologicalAgent::epistemic_only();
        let rep_p2 = env.run_trajectory(&mut no_vital);
        assert!(rep_p2.epistemic_coverage_rate > 0.85);
        assert!(rep_p2.ruin_step.is_some());
        let p2_ruin_t = rep_p2.ruin_step.expect("Prop2 agent must hit ruin");
        assert_eq!(rep_p2.terminal_reserve, 0.0);
        for step in &rep_p2.steps[p2_ruin_t..] {
            assert_eq!(step.next_reserve, 0.0);
        }
        assert_eq!(
            rep_p2.triadic_integrity.diagnose,
            CollapseProposition::Proposition2ImpotentLucidity
        );

        let mut no_gov = StochologicalAgent::without_governance();
        let rep_p3 = env.run_trajectory(&mut no_gov);
        assert!(rep_p3.mean_social_harm > rep_full.mean_social_harm * 2.5);
        assert_eq!(
            rep_p3.triadic_integrity.diagnose,
            CollapseProposition::Proposition3CynicalCapture
        );

        // B2: Fixed-budget factorial isolates mechanism effects holding buffer B_fixed and leverage f_fixed constant
        let fb_audit = env.evaluate_mechanism_vs_budget_factorial();
        assert!(fb_audit.marginal_epistemic_harm_delta > 0.50);
        assert!(fb_audit.marginal_vital_harm_delta > 0.50);
        assert!(fb_audit.marginal_governance_harm_delta > 1.00);

        let mut comp = CompensatoryBufferAgent;
        let rep_comp = env.run_trajectory(&mut comp);
        assert!(rep_comp.epistemic_coverage_rate <= 0.25);
        assert!(rep_comp.mean_social_harm > rep_full.mean_social_harm * 1.3);
        assert_eq!(
            rep_comp.triadic_integrity.diagnose,
            CollapseProposition::Proposition1BlindUndersizing
        );

        let (benign_res, benign_ruin) = env.run_benign_small_world_control();
        assert!(!benign_ruin && benign_res > 45.0);
        let sens = env.verify_threshold_sensitivity();
        assert!(
            sens >= 0.90,
            "sens={sens}, full={:?}, p1={:?}, p2={:?}, p3={:?}",
            rep_full.triadic_integrity,
            rep_p1.triadic_integrity,
            rep_p2.triadic_integrity,
            rep_p3.triadic_integrity
        );
        let struct_sens = env.verify_structural_perturbation_grid();
        assert!(struct_sens >= 0.80);
        let (wide_dom, wide_intact) = env.verify_wide_band_structural_sensitivity();
        assert!(wide_dom > 0.50 && wide_dom < 1.0 && wide_intact > 0.50 && wide_intact < 1.0);

        let mc = CommunityReserveEnv::evaluate_multi_seed(2000, 10, 24, 45.0, || {
            StochologicalAgent::full()
        });
        assert!(mc.harm_p95 > mc.harm_p05 && mc.ruin_rate == 0.0 && mc.mean_harm_se > 0.0);
        assert!(mc.coverage_p95 >= mc.coverage_p05 && mc.coverage_p05 >= 0.80);
        assert!(mc.modal_diagnosis_rate >= 0.90);

        let json = env.export_lab_json();
        assert!(json.contains("\"shockCovRate\":"));
    }

    #[test]
    fn test_community_reserve_env_zero_and_negative_initial_reserve() {
        // Zero initial reserve R_0 = 0.0 must immediately absorb into ruin at step 0
        let env_zero = CommunityReserveEnv::new(crate::kernel::DEFAULT_SEED, 24, 0.0);
        let mut full = StochologicalAgent::full();
        let rep_zero = env_zero.run_trajectory(&mut full);
        assert_eq!(rep_zero.ruin_step, Some(0));
        assert_eq!(rep_zero.terminal_reserve, 0.0);
        for step in &rep_zero.steps {
            assert_eq!(step.next_reserve, 0.0);
            assert!(step.hit_ruin_barrier);
            assert!(step.already_ruined);
            assert_eq!(step.effective_capacity, 0.0);
        }

        // Negative initial reserve R_0 = -10.0 must also absorb into ruin at step 0
        let env_neg = CommunityReserveEnv::new(crate::kernel::DEFAULT_SEED, 24, -10.0);
        let mut naive = NaiveOptimizer;
        let rep_neg = env_neg.run_trajectory(&mut naive);
        assert_eq!(rep_neg.ruin_step, Some(0));
        assert_eq!(rep_neg.terminal_reserve, 0.0);

        // Benign small world control with zero reserve must immediately hit ruin
        let (benign_res, benign_ruin) = env_zero.run_benign_small_world_control();
        assert!(benign_ruin);
        assert_eq!(benign_res, 0.0);
    }
}
