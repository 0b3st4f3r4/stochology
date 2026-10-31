//! **Chapter 1 & Transversal Spine — The Sequential Agent–World–Institution Loop (`CommunityReserveEnv`).**
//!
//! Inspired by the agent–environment loop of Reinforcement Learning (Sutton & Barto),
//! this module implements the evolving didactic environment **"The Community and the Reserve"**
//! (`CommunityReserveEnv`), the [`AgentPolicy`] trait, the baseline [`NaiveOptimizer`],
//! and the progressive [`StochologicalAgent`].
//!
//! Unlike idealized RL environments, `CommunityReserveEnv` exhibits the four real-world properties
//! that make Stochology necessary:
//! 1. **Mutilated Archive:** Observations $O_t$ omit a vulnerable fraction of the population ($\rho_{R,Y} \ne 0$)
//!    and arrive in correlated clusters;
//! 2. **Heavy Tails & Convex Harm:** Demand surges follow power-law tails ($\alpha < 2$), and queue delay /
//!    unmet need grows convexly via Kingman's heavy-traffic factor $\frac{\rho}{1 - \rho}$;
//! 3. **Performative Reactivity:** Aggressive proxy targets trigger strategic gaming and erode true service quality;
//! 4. **Irreversible Ruin & Institutional Mediation:** Exhausting the vital reserve $R_t \le 0$ during a crisis
//!    breaches the absorbing survival barrier ($\mathrm{TTS} < \mathrm{TTR}$).

use crate::archive::{ClusterDesignEffect, MengBigDataDefect};
use crate::asymmetry::NonlinearSummary;
use crate::governance::{
    AlgedonicChannel, DiscursiveCommittee, SosaJudgmentGrade, TelicEvaluation,
};
use crate::identification::ManskiBounds;
use crate::inquiry::cromwell_clamp;
use crate::rng::{SplitMix64, mean};
use crate::synthesis::TriadicIntegrity;
use crate::vital::{EpistemicKellyAllocation, VitalSlackBuffer};

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
    /// Whether the true latent demand fell inside the agent's declared epistemic interval $[L_t, U_t]$.
    pub epistemic_bounds_covered_truth: bool,
    /// Realized utilization $\rho_t = S_t / C_t$.
    pub realized_utilization: f64,
    /// Realized convex queue & unmet-need harm $H_t \ge 0$.
    pub realized_harm: f64,
    /// Updated vital reserve $R_{t+1} \ge 0$.
    pub next_reserve: f64,
    /// True if the system hit the absorbing ruin / collapse barrier ($R_{t+1} \le 0$) at this step.
    pub hit_ruin_barrier: bool,
    /// Sosa judgment grade for this step's decision and outcome.
    pub sosa_grade: SosaJudgmentGrade,
}

/// Summary report of a full trajectory of length $T$ in [`CommunityReserveEnv`].
#[derive(Debug, Clone, PartialEq)]
pub struct TrajectoryReport {
    /// Name of the evaluated policy.
    pub policy_name: String,
    /// Total steps simulated $T$.
    pub total_steps: usize,
    /// Empirical coverage rate of the agent's epistemic bounds over the true latent demand.
    pub epistemic_coverage_rate: f64,
    /// Mean recognized effective sample size $\bar{n}_{\mathrm{eff}}$ vs. raw archive size.
    pub mean_recognized_n_eff: f64,
    /// Mean realized convex social harm $\frac{1}{T}\sum_{t=1}^T H_t$.
    pub mean_social_harm: f64,
    /// Worst-case single-step harm $\max_t H_t$.
    pub max_step_harm: f64,
    /// Terminal vital reserve $R_T$.
    pub terminal_reserve: f64,
    /// Step at which irreversible ruin occurred (`None` if the community survived the entire horizon).
    pub ruin_step: Option<usize>,
    /// Fraction of steps graded as `FullyAptPerformance` or `AdroitExAnteUnluckyOutcome` under Sosa's taxonomy.
    pub apt_or_adroit_rate: f64,
    /// Triadic structural integrity diagnosis over the trajectory.
    pub triadic_integrity: TriadicIntegrity,
    /// Step-by-step trajectory log.
    pub steps: Vec<StepOutcome>,
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
        let raw_n = all_obs.len() as f64;
        // Optimizes for the lone mean with 96% utilization ("lean efficiency") and tiny 2% margin
        AgentDecision {
            epistemic_demand_bounds: (raw_mean - 0.5, raw_mean + 0.5),
            recognized_n_eff: raw_n,
            provisioned_capacity: raw_mean / 0.96,
            vital_reserve_allocation: 0.02 * obs.current_reserve,
            growth_leverage_f: 1.35,
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
    /// **Safeguard 1 — Epistemic Margin:** enables Manski partial-identification bounds,
    /// Moulton cluster correction, and Meng selection-defect adjustment.
    pub use_epistemic_margin: bool,
    /// **Safeguard 2 — Vital Margin:** enables Jensen convexity adjustment, Kingman queue slack ($\rho \le 0.78$),
    /// $\mathrm{TTS} \ge \mathrm{TTR}$ reserve buffering, and Baker–McHale epistemic Kelly shrinkage.
    pub use_vital_margin: bool,
    /// **Safeguard 3 — Institutional Governance:** eliminates high-powered proxy gaming, removes Fricker's
    /// testimonial discount on the algedonic channel ($\delta_{\mathrm{cred}} = 0$), and enforces the
    /// List–Pettit / Beer $S3^*$ constitutional conjunctive brake.
    pub use_institutional_governance: bool,
}

impl StochologicalAgent {
    /// Full Stochological Agent with all three axes of the Nuclear Triad active.
    pub const fn full() -> Self {
        Self {
            use_epistemic_margin: true,
            use_vital_margin: true,
            use_institutional_governance: true,
        }
    }

    /// Agent with only the Epistemic Margin active (illustrating **Collapse Proposition 2**: *impotent lucidity* when Vital Margin is missing).
    pub const fn epistemic_only() -> Self {
        Self {
            use_epistemic_margin: true,
            use_vital_margin: false,
            use_institutional_governance: true,
        }
    }

    /// Agent missing only the Epistemic Margin (illustrating **Collapse Proposition 1**: *blind undersizing*).
    pub const fn without_epistemic_margin() -> Self {
        Self {
            use_epistemic_margin: false,
            use_vital_margin: true,
            use_institutional_governance: true,
        }
    }

    /// Agent missing only Institutional Governance (illustrating **Collapse Proposition 3**: *cynical capture and pillaging*).
    pub const fn without_governance() -> Self {
        Self {
            use_epistemic_margin: true,
            use_vital_margin: true,
            use_institutional_governance: false,
        }
    }
}

impl AgentPolicy for StochologicalAgent {
    fn name(&self) -> &str {
        match (
            self.use_epistemic_margin,
            self.use_vital_margin,
            self.use_institutional_governance,
        ) {
            (true, true, true) => "StochologicalAgent(FullTriad)",
            (false, true, true) => "DefectiveAgent(NoEpistemicMargin->Prop1)",
            (true, false, true) => "DefectiveAgent(NoVitalMargin->Prop2)",
            (true, true, false) => "DefectiveAgent(NoGovernance->Prop3)",
            _ => "PartialStochologicalAgent",
        }
    }

    fn act(&mut self, obs: &ArchiveObservation) -> AgentDecision {
        let all_obs: Vec<f64> = obs
            .observed_clusters
            .iter()
            .flat_map(|c| c.iter().copied())
            .collect();
        let raw_mean = mean(&all_obs);
        let raw_n = all_obs.len() as f64;

        // 1. Epistemic Margin (Chapters 2, 3, 5): Moulton cluster DEFF + Meng selection + Manski bounds
        let (bounds, n_eff) = if self.use_epistemic_margin {
            let deff = ClusterDesignEffect::evaluate(&obs.observed_clusters, 0.75);
            let meng = MengBigDataDefect::evaluate(
                (all_obs.len() + obs.missing_count + 500) as f64,
                raw_n,
                -0.18,
                8.0,
            );
            let manski = ManskiBounds::evaluate(
                &all_obs,
                obs.missing_count,
                obs.demand_support.0,
                obs.demand_support.1,
                0.05,
            );
            let n_joint = deff.effective_n.min(meng.effective_sample_size).max(4.0);
            (
                (
                    manski.imbens_manski_lower,
                    manski.imbens_manski_upper + meng.estimation_error.abs(),
                ),
                n_joint,
            )
        } else {
            ((raw_mean - 0.5, raw_mean + 0.5), raw_n)
        };

        // 2. Vital Margin (Chapters 4, 7): Nonlinear convex summary + Kingman slack + epistemic Kelly
        let design_demand = if self.use_epistemic_margin {
            0.35 * bounds.0 + 0.65 * bounds.1
        } else {
            raw_mean
        };
        let nonlin = NonlinearSummary::evaluate(&all_obs, |x| x * x);
        let jensen_correction = (nonlin.expected_eval.sqrt() - nonlin.sample_mean).max(0.0);

        let (capacity, reserve_alloc, leverage_f) = if self.use_vital_margin {
            let kelly = EpistemicKellyAllocation::evaluate(0.08, 0.16, n_eff);
            // Buffer capacity to keep Kingman utilization rho <= 0.76 even under upper epistemic demand + Jensen spread
            let cap = (design_demand + jensen_correction) / 0.76;
            let res = (0.30 * obs.current_reserve).max(12.0);
            (cap, res, kelly.epistemic_kelly_fraction)
        } else {
            // Impotent lucidity: knows the wide epistemic bounds, yet sizes capacity for the point mean at 96% utilization with over-leverage
            (raw_mean / 0.96, 0.02 * obs.current_reserve, 1.55)
        };

        // 3. Institutional Governance (Chapters 6, 8, 9): causal multitask protection + undiscounted algedonic brake
        let (proxy_pressure, discount, brake) = if self.use_institutional_governance {
            (0.10, 0.0, true)
        } else {
            (0.85, 0.60, false)
        };

        AgentDecision {
            epistemic_demand_bounds: bounds,
            recognized_n_eff: n_eff,
            provisioned_capacity: capacity,
            vital_reserve_allocation: reserve_alloc,
            growth_leverage_f: leverage_f,
            proxy_incentive_pressure: proxy_pressure,
            frontline_testimonial_discount: discount,
            constitutional_brake_active: brake,
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
        let rng = SplitMix64::new(self.seed);
        let mut reserve = self.initial_reserve;
        let mut ruin_step = None;
        let mut steps = Vec::with_capacity(self.horizon_steps);
        let mut covered_count = 0usize;
        let mut sum_n_eff = 0.0_f64;
        let mut sum_harm = 0.0_f64;
        let mut max_harm = 0.0_f64;
        let mut apt_count = 0usize;
        let mut performative_erosion = 0.0_f64;

        for t in 0..self.horizon_steps {
            let mut step_rng = rng.split(t as u64);
            // 6 community clusters of size 10; 2 vulnerable high-need clusters have 50% missingness in the archive
            let base_demand = 22.0 + performative_erosion;
            let is_shock_step = t % 6 == 4;
            let tail_surge = if is_shock_step {
                step_rng.pareto(4.0, 1.45).min(22.0)
            } else {
                0.0
            };

            let mut obs_clusters = Vec::with_capacity(6);
            let mut true_all = Vec::with_capacity(60);
            let mut missing_count = 0usize;

            for c in 0..6 {
                let cluster_shift = if c >= 4 {
                    12.0 + 0.6 * tail_surge
                } else {
                    -2.5 + 0.2 * tail_surge
                };
                let shared_shock = step_rng.normal(0.0, 2.2);
                let mut c_obs = Vec::new();
                for _ in 0..10 {
                    let val =
                        (base_demand + cluster_shift + shared_shock + step_rng.normal(0.0, 1.2))
                            .clamp(5.0, 65.0);
                    true_all.push(val);
                    // Vulnerable clusters (c >= 4) suffer selective non-recording (MNAR)
                    let record_prob = if c >= 4 { 0.40 } else { 0.95 };
                    if step_rng.bernoulli(record_prob) {
                        c_obs.push(val);
                    } else {
                        missing_count += 1;
                    }
                }
                if c_obs.is_empty() {
                    c_obs.push(base_demand);
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
            let covered = true_mean >= decision.epistemic_demand_bounds.0
                && true_mean <= decision.epistemic_demand_bounds.1;
            if covered {
                covered_count += 1;
            }
            sum_n_eff += decision.recognized_n_eff;

            // Evaluate algedonic emergency detection and Kingman queue congestion
            let algedonic = AlgedonicChannel::evaluate(
                frontline_e.max(1.05),
                10.0,
                decision.frontline_testimonial_discount,
                6.0,
            );
            let effective_cap = if decision.constitutional_brake_active
                && !algedonic.fatal_testimonial_delay
                && is_shock_step
            {
                decision.provisioned_capacity + 0.50 * decision.vital_reserve_allocation
            } else {
                decision.provisioned_capacity
            };

            let rho = cromwell_clamp(true_mean / effective_cap.max(1.0), 0.01);
            let slack = VitalSlackBuffer::evaluate(
                rho,
                1.0,
                decision.vital_reserve_allocation,
                (true_mean - decision.provisioned_capacity).max(1.0),
                3.0,
            );

            // Performative erosion: high proxy pressure degrades next-step baseline and inflates harm
            let gaming_penalty = 8.0 * decision.proxy_incentive_pressure;
            performative_erosion =
                (0.75 * performative_erosion + 2.2 * decision.proxy_incentive_pressure).min(14.0);

            let harm = slack.kingman_wait_multiplier + gaming_penalty;
            sum_harm += harm;
            if harm > max_harm {
                max_harm = harm;
            }

            // Reserve dynamics: replenishment from safe growth minus overload drain and over-leverage variance drain
            let overload_drain = (true_mean - effective_cap).max(0.0) * 1.8
                + if rho > 0.90 { 8.5 } else { 0.0 }
                + if !decision.constitutional_brake_active {
                    3.5 * decision.proxy_incentive_pressure
                } else {
                    0.0
                };
            let kelly_net = 3.2 * decision.growth_leverage_f
                - 2.4 * decision.growth_leverage_f * decision.growth_leverage_f;
            reserve = (reserve + kelly_net - overload_drain).max(0.0);

            let hit_ruin = reserve <= 1e-6;
            if hit_ruin && ruin_step.is_none() {
                ruin_step = Some(t);
            }

            let ex_ante_comp = if covered && rho <= 0.85 && decision.constitutional_brake_active {
                0.92
            } else {
                0.35
            };
            let telic = TelicEvaluation::evaluate(
                ex_ante_comp,
                covered && decision.constitutional_brake_active,
                !hit_ruin && harm < 12.0,
                0.45,
            );
            if matches!(
                telic.sosa_grade,
                SosaJudgmentGrade::FullyAptPerformance
                    | SosaJudgmentGrade::AdroitExAnteUnluckyOutcome
            ) {
                apt_count += 1;
            }

            steps.push(StepOutcome {
                step_t: t,
                true_latent_demand: true_mean,
                raw_archive_mean: raw_mean,
                epistemic_bounds_covered_truth: covered,
                realized_utilization: rho,
                realized_harm: harm,
                next_reserve: reserve,
                hit_ruin_barrier: hit_ruin,
                sosa_grade: telic.sosa_grade,
            });
        }

        let tf = self.horizon_steps.max(1) as f64;
        let cov_rate = (covered_count as f64) / tf;
        let mean_harm = sum_harm / tf;
        let apt_rate = (apt_count as f64) / tf;

        // Score the three axes of the Nuclear Triad in [0, 1]
        let last_dec = policy.act(&ArchiveObservation {
            step_t: 0,
            observed_clusters: vec![vec![20.0, 22.0], vec![25.0, 28.0]],
            missing_count: 4,
            demand_support: (5.0, 65.0),
            current_reserve: self.initial_reserve,
            frontline_distress_e_factor: 1.5,
        });
        let s_epi = cov_rate.clamp(0.0, 1.0);
        let s_vital =
            if last_dec.vital_reserve_allocation >= 10.0 && last_dec.growth_leverage_f <= 1.0 {
                if ruin_step.is_none() { 0.92 } else { 0.78 }
            } else {
                0.08
            };
        let s_gov = if last_dec.constitutional_brake_active
            && last_dec.frontline_testimonial_discount < 0.20
            && last_dec.proxy_incentive_pressure < 0.30
        {
            0.92
        } else {
            0.12
        };
        let triad = TriadicIntegrity::evaluate(s_epi, s_vital, s_gov, 0.50);
        let _ = DiscursiveCommittee::evaluate(&[[true, true, true]]);

        TrajectoryReport {
            policy_name: policy.name().to_string(),
            total_steps: self.horizon_steps,
            epistemic_coverage_rate: cov_rate,
            mean_recognized_n_eff: sum_n_eff / tf,
            mean_social_harm: mean_harm,
            max_step_harm: max_harm,
            terminal_reserve: reserve,
            ruin_step,
            apt_or_adroit_rate: apt_rate,
            triadic_integrity: triad,
            steps,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::synthesis::CollapseProposition;

    #[test]
    fn stochological_agent_survives_and_dominates_naive_and_partial_agents() {
        let env = CommunityReserveEnv::new(2026, 24, 45.0);

        let mut naive = NaiveOptimizer;
        let rep_naive = env.run_trajectory(&mut naive);
        assert!(rep_naive.epistemic_coverage_rate < 0.25);
        assert!(rep_naive.ruin_step.is_some());

        let mut full = StochologicalAgent::full();
        let rep_full = env.run_trajectory(&mut full);
        assert!(rep_full.epistemic_coverage_rate > 0.85);
        assert!(rep_full.ruin_step.is_none());
        assert!(rep_full.mean_social_harm < rep_naive.mean_social_harm * 0.25);
        assert_eq!(
            rep_full.triadic_integrity.diagnose,
            CollapseProposition::IntactTriad
        );

        let mut no_epi = StochologicalAgent::without_epistemic_margin();
        let rep_p1 = env.run_trajectory(&mut no_epi);
        assert_eq!(
            rep_p1.triadic_integrity.diagnose,
            CollapseProposition::Proposition1BlindUndersizing
        );

        let mut no_vital = StochologicalAgent::epistemic_only();
        let rep_p2 = env.run_trajectory(&mut no_vital);
        assert_eq!(
            rep_p2.triadic_integrity.diagnose,
            CollapseProposition::Proposition2ImpotentLucidity
        );

        let mut no_gov = StochologicalAgent::without_governance();
        let rep_p3 = env.run_trajectory(&mut no_gov);
        assert_eq!(
            rep_p3.triadic_integrity.diagnose,
            CollapseProposition::Proposition3CynicalCapture
        );
    }
}
