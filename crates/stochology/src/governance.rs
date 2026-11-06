//! **Block III (Filter III: Reactive Worlds and Institutions · Axis 3: Institutional Governance $S_{\mathrm{gov}}$ · Chapters 7–8).**
//!
//! Unifies all reactive-world and institutional-governance operators across **Chapter 7 [formerly Ch. 6]**
//! (Reactive Worlds, CORP-PAV Calibration, Invariant Causal Prediction, Performativity & Goodhart's Law)
//! and **Chapter 8** (Moral Luck, Apt Agency, Algedonic $S3^*$ Channel & Institutional Design)
//! into **Canonical Operator III (Three Temporal Gates)** and the Type-State structure [`GovernanceState`]:
//!
//! 1. **Gate 1 (Ex-Ante / Pre-Action):** Isotonic CORP-PAV Brier decomposition ($\mathrm{BS} = \mathrm{REL} - \mathrm{RES} + \mathrm{UNC}$),
//!    Invariant Causal Prediction (ICP), Kennedy–O'Hagan structural discrepancy, Tian–Pearl bounds, and
//!    Perdomo/MacKenzie Stackelberg Performative Optimality ($\theta_{\mathrm{PO}}$ vs. $\theta_{\mathrm{PS}}$);
//! 2. **Gate 2 (Real-Time / Execution):** Beer's $S3^*$ algedonic emergency channel with zero Fricker testimonial
//!    discount ($\delta_{\mathrm{cred}} = 0$, preserving $\mathrm{TTS}_{\mathrm{survival}} \ge \mathrm{TTR}_{\mathrm{recovery}}$),
//!    List–Pettit constitutional conjunctive veto, and Gigerenzer / Zheng–Meister natural-frequency alert filtering;
//! 3. **Gate 3 (Ex-Post / Accountability):** Bitemporal SHA-256 [`TemporalVault`], Sosa/Baron–Hershey [`TelicGrade`],
//!    Holmström–Innes structural liability vs. Simon–Girard scapegoating & Bateson double-bind ($DB_t$),
//!    and Chouldechova's fairness trilemma ($\frac{\mathrm{FPR}_A}{\mathrm{FPR}_B} = \frac{p_A/(1-p_A)}{p_B/(1-p_B)}$).

use crate::kernel::{
    SplitMix64, hmac_sha256_hex, mean, ols_ridge_fit, pav_isotonic_non_decreasing, sample_variance,
    sha256_bytes, sha256_hex, solve_cholesky_spd,
};

/// Ernest Sosa (2007, 2015) and Jonathan Baron & John C. Hershey (1988) four-grade bitemporal
/// taxonomy of judgment and agency under uncertainty.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TelicGrade {
    /// Failed both ex-ante competence (*télos*) and ex-post outcome (*skopós*).
    #[default]
    InaptAndInaccurate,
    /// Hit the target (*accurate*) purely by moral luck despite negligent ex-ante method (reckless speculation).
    LuckyAccurateOnly,
    /// Executed with rigorous ex-ante competence (*adroit*) and vital margin, yet suffered an exogenous tail shock.
    AdroitExAnteUnluckyOutcome,
    /// Achieved success *because* of competence, or exercised second-order reflective wisdom (*fully apt*)
    /// by abstaining when the epistemic margin could not clear the moral stakes of harm.
    FullyAptPerformance,
}

impl TelicGrade {
    /// Classifies an action into Sosa's $2\times 2$ bitemporal judgment table from `(ex_ante_adroit, ex_post_accurate)`.
    #[inline]
    pub const fn classify(ex_ante_adroit: bool, ex_post_accurate: bool) -> Self {
        match (ex_ante_adroit, ex_post_accurate) {
            (true, true) => Self::FullyAptPerformance,
            (true, false) => Self::AdroitExAnteUnluckyOutcome,
            (false, true) => Self::LuckyAccurateOnly,
            (false, false) => Self::InaptAndInaccurate,
        }
    }
}

/// Alias preserving compatibility with Sosa's terminology in trajectories and reports.
pub type SosaJudgmentGrade = TelicGrade;

/// Donald MacKenzie (2006) classification of performativity regimes when a model or rule $\theta$
/// acts upon the socio-economic world (*An Engine, Not a Camera*).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MacKenzieRegime {
    /// `CameraPassive`: Wasserstein shift $\varepsilon \approx 0$; deploying the model does not alter the world.
    #[default]
    CameraPassive,
    /// `EffectivePerformative`: Deploying the rule alters the world and improves substantive social welfare.
    EffectivePerformative,
    /// `BarnesianSelfFulfilling`: Repeating the rule drives predictive residuals toward zero ($\theta \to \theta_{\mathrm{PS}}$)
    /// by molding agents to fit the formula, without reaching the substantive optimum ($\theta_{\mathrm{PO}}$).
    BarnesianSelfFulfilling,
    /// `CounterperformativeDestructive`: Deploying the rule triggers feedback that destabilizes the world.
    CounterperformativeDestructive,
}

impl MacKenzieRegime {
    /// Classifies the performative feedback regime from Wasserstein sensitivity `epsilon`, performative gap `gap = |theta_PS - theta_PO|`,
    /// and whether repeated deployment destabilizes risk (`risk_diverges`).
    #[inline]
    pub fn classify(epsilon: f64, gap: f64, risk_diverges: bool) -> Self {
        if risk_diverges || epsilon < -0.15 {
            Self::CounterperformativeDestructive
        } else if epsilon.abs() <= 1e-3 {
            Self::CameraPassive
        } else if gap > 0.05 {
            Self::BarnesianSelfFulfilling
        } else {
            Self::EffectivePerformative
        }
    }
}

/// Manheim & Garrabrant (2018) taxonomy of the four mechanisms by which a proxy metric $M$
/// decouples from the true substantive goal $G$ when optimized (Goodhart, 1975; Strathern, 1997).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GoodhartVariant {
    /// Selecting the tail of a noisy proxy $M = G + \varepsilon$ selects for extreme noise $\varepsilon$.
    Regressional,
    /// Pushing $M$ into an extreme tail leaves the domain where $M$ and $G$ were historically correlated.
    Extremal,
    /// Intervening on a downstream symptom $M$ ($G \to M$) does not alter the upstream cause $G$.
    Causal,
    /// Strategic agents actively game $M$ to capture rewards while hollowing out $G$.
    #[default]
    Adversarial,
}

impl GoodhartVariant {
    /// Classifies the dominant Goodhart failure mode from mechanism indicators.
    #[inline]
    pub fn classify(
        adversarial_gaming: bool,
        causal_symptom_intervention: bool,
        tail_extremeness_z: f64,
    ) -> Self {
        if adversarial_gaming {
            Self::Adversarial
        } else if causal_symptom_intervention {
            Self::Causal
        } else if tail_extremeness_z > 2.5 {
            Self::Extremal
        } else {
            Self::Regressional
        }
    }
}

/// Beer's (1972) $S3^*$ Algedonic Channel coupled with Miranda Fricker's (2007) testimonial injustice discount $\delta_{\mathrm{cred}}$.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AlgedonicChannel {
    /// Frontline shift likelihood ratio / distress factor $e > 1$.
    pub frontline_e_factor: f64,
    /// Circuit-breaker trip threshold $E_{\mathrm{trip}} > 1$.
    pub trip_threshold: f64,
    /// Fricker testimonial credibility discount $\delta_{\mathrm{cred}} \in [0, 1)$.
    pub testimonial_discount: f64,
    /// Shifts required to trip $S3^*$ when $\delta_{\mathrm{cred}} = 0$.
    pub ttr_undiscounted: f64,
    /// Shifts required to trip $S3^*$ under discount $\delta_{\mathrm{cred}}$.
    pub ttr_discounted: f64,
    /// Physical Time-to-Survival $\mathrm{TTS}_{\mathrm{survival}}$.
    pub tts_survival: f64,
    /// True iff $\mathrm{TTR}_{\mathrm{undiscounted}} \le \mathrm{TTS}_{\mathrm{survival}} < \mathrm{TTR}_{\mathrm{discounted}}$.
    pub fatal_testimonial_delay: bool,
}

impl AlgedonicChannel {
    /// Evaluates undiscounted and discounted algedonic circuit-breaker latencies against `tts_survival`.
    pub fn evaluate(
        frontline_e_factor: f64,
        trip_threshold: f64,
        testimonial_discount: f64,
        tts_survival: f64,
    ) -> Self {
        let target_ln = trip_threshold.max(1.01).ln();
        let step_ln = frontline_e_factor.max(1.01).ln();
        let delta = testimonial_discount.clamp(0.0, 0.99);
        let ttr_just = (target_ln / step_ln).ceil();
        let ttr_disc = (target_ln / ((1.0 - delta) * step_ln)).ceil();
        let tts = tts_survival.max(0.0);
        Self {
            frontline_e_factor,
            trip_threshold,
            testimonial_discount: delta,
            ttr_undiscounted: ttr_just,
            ttr_discounted: ttr_disc,
            tts_survival: tts,
            fatal_testimonial_delay: ttr_just <= tts && ttr_disc > tts,
        }
    }
}

/// Gregory Bateson's (1956/1972) institutional double-bind audit:
/// $\mathrm{DB}_t = \beta_{\mathrm{proxy}} \cdot \delta_{\mathrm{cred}} \cdot (1 - \mathbf{1}\{\text{brake}\})$.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BatesonDoubleBindAudit {
    /// High-powered proxy pressure $\beta_{\mathrm{proxy}} \in [0, 1]$.
    pub proxy_pressure: f64,
    /// Testimonial discount $\delta_{\mathrm{cred}} \in [0, 1]$.
    pub testimonial_discount: f64,
    /// Whether the $S3^*$ constitutional veto is active.
    pub constitutional_brake_active: bool,
    /// Synergistic double-bind intensity $\mathrm{DB}_t \in [0, 1]$.
    pub double_bind_intensity: f64,
    /// True if $\mathrm{DB}_t > 0.20$.
    pub is_double_bind_trap: bool,
}

impl BatesonDoubleBindAudit {
    /// Evaluates Bateson's double-bind intensity.
    pub fn evaluate(
        proxy_pressure: f64,
        testimonial_discount: f64,
        constitutional_brake_active: bool,
    ) -> Self {
        let beta = proxy_pressure.clamp(0.0, 1.0);
        let delta = testimonial_discount.clamp(0.0, 1.0);
        let no_brake = if constitutional_brake_active {
            0.0
        } else {
            1.0
        };
        let db = beta * delta * no_brake;
        Self {
            proxy_pressure: beta,
            testimonial_discount: delta,
            constitutional_brake_active,
            double_bind_intensity: db,
            is_double_bind_trap: db > 0.20,
        }
    }
}

/// Gerd Gigerenzer & Ulrich Hoffrage (1995) natural-frequency tree and Turing–Good deciban filter (`CognitiveAlertFilter`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CognitiveAlertFilter {
    /// Turing–Good weight of evidence $W = 10 \log_{10}(\mathrm{TPR}/\mathrm{FPR})$ in decibans.
    pub deciban_weight: f64,
    /// Positive Predictive Value $\mathrm{PPV} = \mathbb{P}(\text{Crisis}\mid\text{Alert})$.
    pub ppv: f64,
    /// False Discovery Rate $\mathrm{FDR} = 1 - \mathrm{PPV}$.
    pub fdr: f64,
    /// Reference cohort size $N$ (e.g., $1\,000$ shifts).
    pub cohort_n: u32,
    /// Expected true crisis shifts $n_{\mathrm{crisis}}$ out of $N$.
    pub natural_freq_crises: u32,
    /// Expected true alerts $a$ out of $n_{\mathrm{crisis}}$.
    pub natural_freq_true_alerts: u32,
    /// Expected false alarms $b$ out of $N - n_{\mathrm{crisis}}$.
    pub natural_freq_false_alarms: u32,
}

impl CognitiveAlertFilter {
    /// Evaluates the deciban weight, PPV/FDR, and integer natural-frequency tree for `(base_rate_pi, tpr, fpr, cohort_n)`.
    pub fn evaluate(base_rate_pi: f64, tpr: f64, fpr: f64, cohort_n: u32) -> Self {
        let pi = base_rate_pi.clamp(1e-6, 1.0 - 1e-6);
        let sens = tpr.clamp(1e-6, 1.0);
        let fp = fpr.clamp(1e-6, 1.0);
        let decibans = 10.0 * (sens / fp).log10();
        let ppv = (pi * sens) / (pi * sens + (1.0 - pi) * fp);
        let n_f = cohort_n.max(1) as f64;
        let crises = (n_f * pi).round() as u32;
        let true_alerts = ((crises as f64) * sens).round() as u32;
        let false_alarms = (((cohort_n.saturating_sub(crises)) as f64) * fp).round() as u32;
        Self {
            deciban_weight: decibans,
            ppv,
            fdr: 1.0 - ppv,
            cohort_n,
            natural_freq_crises: crises,
            natural_freq_true_alerts: true_alerts,
            natural_freq_false_alarms: false_alarms,
        }
    }
}

/// Bengt Holmström (1979) & Robert D. Innes (1990) variance decomposition of tail deficit
/// paired with Herbert Simon & René Girard's scapegoat blame transfer (`ContractAndLiabilityAudit`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ContractAndLiabilityAudit {
    /// Share of total deficit variance caused by executive structural sizing $L_{\mathrm{struct}} = \mathrm{Var}(D_{\mathrm{struct}}) / \mathrm{Var}_{\mathrm{total}}$.
    pub structural_liability_share: f64,
    /// Share of total deficit variance caused by frontline operational noise $L_{\mathrm{oper}} = \mathrm{Var}(D_{\mathrm{oper}}) / \mathrm{Var}_{\mathrm{total}}$.
    pub frontline_liability_share: f64,
    /// Share of total deficit variance caused by exogenous shocks $L_{\mathrm{exog}}$.
    pub exogenous_liability_share: f64,
    /// Blame share assigned to frontline operators by management $b_{\mathrm{frontline}} \in [0, 1]$.
    pub assigned_frontline_blame: f64,
    /// Excess blame shifted onto frontline operators $\Delta_{\mathrm{scapegoat}} = \max(0, b_{\mathrm{frontline}} - L_{\mathrm{oper}})$.
    pub scapegoat_blame_transfer: f64,
    /// Blame inversion ratio $b_{\mathrm{frontline}} / L_{\mathrm{oper}}$.
    pub blame_inversion_ratio: f64,
    /// True if $L_{\mathrm{struct}} > 0.50$ and $\Delta_{\mathrm{scapegoat}} > 0.25$.
    pub is_scapegoating: bool,
}

impl ContractAndLiabilityAudit {
    /// Decomposes empirical variance across `structural_deficits`, `frontline_deficits`, and `exogenous_deficits`
    /// and audits assigned frontline blame `assigned_frontline_blame`.
    pub fn from_deficit_series(
        structural_deficits: &[f64],
        frontline_deficits: &[f64],
        exogenous_deficits: &[f64],
        assigned_frontline_blame: f64,
    ) -> Self {
        let v_s = if structural_deficits.len() >= 2 {
            sample_variance(structural_deficits)
        } else {
            0.0
        };
        let v_o = if frontline_deficits.len() >= 2 {
            sample_variance(frontline_deficits)
        } else {
            0.0
        };
        let v_e = if exogenous_deficits.len() >= 2 {
            sample_variance(exogenous_deficits)
        } else {
            0.0
        };
        let v_tot = (v_s + v_o + v_e).max(1e-12);
        let l_struct = v_s / v_tot;
        let l_oper = v_o / v_tot;
        let l_exog = v_e / v_tot;
        let b_front = assigned_frontline_blame.clamp(0.0, 1.0);
        let transfer = (b_front - l_oper).max(0.0);
        let inv_ratio = b_front / l_oper.max(1e-9);
        Self {
            structural_liability_share: l_struct,
            frontline_liability_share: l_oper,
            exogenous_liability_share: l_exog,
            assigned_frontline_blame: b_front,
            scapegoat_blame_transfer: transfer,
            blame_inversion_ratio: inv_ratio,
            is_scapegoating: l_struct > 0.50 && transfer > 0.25,
        }
    }
}

/// One observational or interventional environment $e \in \mathcal{E}$ for Invariant Causal Prediction
/// (Peters, Bühlmann & Meinshausen, 2016).
#[derive(Debug, Clone, PartialEq)]
pub struct CausalEnvironment {
    /// Human-readable regime name.
    pub name: String,
    /// Design matrix rows $X_{i,\cdot}$ ($n_e \times d$).
    pub x: Vec<Vec<f64>>,
    /// Outcome vector $Y_i$ ($n_e$).
    pub y: Vec<f64>,
}

/// Finite-sample power and identification classification for Invariant Causal Prediction (ICP).
///
/// Prevents the silent collapse of $\hat{S}_{\mathrm{ICP}} = \bigcap_{S:\,H_{0,S}\text{ not rejected}} S \to \emptyset$
/// under low effective sample size ($n_{\mathrm{eff}} / \text{env} < 150$) from being weaponized as an
/// agnotological alibi against protective buffers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IcpIdentificationStatus {
    /// $n_{\mathrm{eff}}/\text{env} \ge 150$ and an invariant causal parent set survived intersection.
    #[default]
    IdentifiedInvariantSet,
    /// $n_{\mathrm{eff}}/\text{env} < 150$: statistical power is insufficient to reject $H_{0,\emptyset}$;
    /// mandates precautionary sizing at the upper partial-identification bound $U_t$.
    EpistemicUnderpoweredSet,
    /// $n_{\mathrm{eff}}/\text{env} \ge 150$ and all candidate sets were refuted across environments.
    RefutedAllCandidates,
}

/// **Synchronous Kairós Decision Plane (`KairosSynchronousPanel`, $\le 30\text{ bits}$ / 6 scalars):**
///
/// Enforces the Zheng & Meister (2024/2025) human conscious decision throughput limit
/// ($C_{\mathrm{human}} = 10.0\text{ bits/s}$) as a compile-time structural separation from the
/// 36-scalar asynchronous deliberative audit (`GovernanceState` + `EpistemicMarginState` + `VitalMarginState`),
/// guaranteeing $\tau_{\mathrm{cog}} = 3.0\text{ s}$, $\rho_{\mathrm{cog}} = 0.12 \le 0.76$, and
/// $\mathrm{TTR}_{\mathrm{total}} = 18.51\text{ s} \le \mathrm{TTS}_{\mathrm{crisis}} = 45.0\text{ s}$.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KairosSynchronousPanel {
    /// Scalar 1: Epistemic margin score $S_{\mathrm{epi}} \in [0, 1]$.
    pub s_epi: f64,
    /// Scalar 2: Vital margin score $S_{\mathrm{vital}} \in [0, 1]$.
    pub s_vital: f64,
    /// Scalar 3: Institutional governance nested Leontief score $S_{\mathrm{gov}} \in [0, 1]$.
    pub s_gov: f64,
    /// Scalar 4: Leontief bottleneck minimum $M_{\mathrm{Leontief}} = \min(S_{\mathrm{epi}}, S_{\mathrm{vital}}, S_{\mathrm{gov}})$.
    pub m_leontief: f64,
    /// Scalar 5: Physical net survival margin $\mathrm{TTS}_{\mathrm{survival}} - \mathrm{TTR}_{\mathrm{recovery}}$ (seconds or shifts).
    pub net_survival_margin: f64,
    /// Scalar 6: Binary state of Beer's $S3^*$ constitutional conjunctive emergency brake.
    pub s3_star_brake_active: bool,
}

impl KairosSynchronousPanel {
    /// Maximum semantic information payload of the 6-scalar Kairós panel ($30.0\text{ bits}$, 5 bits/scalar).
    pub const MAX_KAIROS_BITS: f64 = 30.0;

    /// Constructs the 6-scalar compressed Kairós panel from the triad scores, survival slack, and $S3^*$ state.
    pub fn new(
        s_epi: f64,
        s_vital: f64,
        s_gov: f64,
        tts_survival: f64,
        ttr_recovery: f64,
        s3_star_brake_active: bool,
    ) -> Self {
        let e = s_epi.clamp(0.0, 1.0);
        let v = s_vital.clamp(0.0, 1.0);
        let g = s_gov.clamp(0.0, 1.0);
        Self {
            s_epi: e,
            s_vital: v,
            s_gov: g,
            m_leontief: e.min(v).min(g),
            net_survival_margin: tts_survival - ttr_recovery,
            s3_star_brake_active,
        }
    }
}

/// Cybernetic queue audit coupling the Zheng & Meister (2024/2025) $10\text{ bits/s}$ human conscious
/// channel limit ($C_{\mathrm{human}}$) with Kingman's heavy-traffic queue formula and Simchi-Levi's
/// survival condition $\mathrm{TTS}_{\mathrm{crisis}} \ge \mathrm{TTR}_{\mathrm{total}}$.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ZhengMeisterQueueAudit {
    /// Number of scalar indicators inspected per alert cycle.
    pub scalar_count: usize,
    /// Semantic information payload $B_{\mathrm{audit}}$ (in bits) required per decision alert.
    pub payload_bits: f64,
    /// Base human cognitive service time $\tau_{\mathrm{cog}} = B_{\mathrm{audit}} / C_{\mathrm{human}}$ (seconds).
    pub cognitive_service_time_s: f64,
    /// Operator cognitive utilization $\rho_{\mathrm{cog}} = \lambda_a \tau_{\mathrm{cog}}$.
    pub cognitive_utilization_rho: f64,
    /// Kingman heavy-traffic attention queue delay $W_q = \bigl(\frac{\rho_{\mathrm{cog}}}{1-\rho_{\mathrm{cog}}}\bigr)\bigl(\frac{c_a^2+c_s^2}{2}\bigr)\tau_{\mathrm{cog}}$ (seconds).
    pub kingman_attention_wait_s: f64,
    /// Total Time-to-Recovery $\mathrm{TTR}_{\mathrm{total}} = \tau_{\mathrm{cog}} + W_q + \tau_{\mathrm{act}}$ (seconds).
    pub total_recovery_ttr_s: f64,
    /// Available physical Time-to-Survival $\mathrm{TTS}_{\mathrm{crisis}}$ (seconds).
    pub crisis_survival_tts_s: f64,
    /// True iff $\rho_{\mathrm{cog}} \le 0.76$ (Kingman slack ceiling) and $\mathrm{TTR}_{\mathrm{total}} \le \mathrm{TTS}_{\mathrm{crisis}}$.
    pub survives_kairos: bool,
}

impl ZhengMeisterQueueAudit {
    /// Human conscious decision throughput $C_{\mathrm{human}} = 10.0\text{ bits/s}$ (Zheng & Meister, 2024/2025).
    pub const HUMAN_BITS_PER_SEC: f64 = 10.0;

    /// Evaluates the cognitive queue dynamics for a given `scalar_count` and `payload_bits` under crisis parameters
    /// $(\lambda_a,\, c_a^2,\, c_s^2,\, \tau_{\mathrm{act}},\, \mathrm{TTS}_{\mathrm{crisis}})$.
    pub fn evaluate(
        scalar_count: usize,
        payload_bits: f64,
        alert_arrival_rate_hz: f64,
        arrival_cv_sq: f64,
        service_cv_sq: f64,
        actuation_latency_s: f64,
        crisis_survival_tts_s: f64,
    ) -> Self {
        let tau_cog = payload_bits.max(0.0) / Self::HUMAN_BITS_PER_SEC;
        let rho_cog = (alert_arrival_rate_hz.max(0.0) * tau_cog).clamp(0.0, 0.999);
        let v_factor = 0.5 * (arrival_cv_sq.max(0.0) + service_cv_sq.max(0.0));
        let w_q = (rho_cog / (1.0 - rho_cog)) * v_factor * tau_cog;
        let ttr_tot = tau_cog + w_q + actuation_latency_s.max(0.0);
        let tts = crisis_survival_tts_s.max(0.0);

        Self {
            scalar_count,
            payload_bits,
            cognitive_service_time_s: tau_cog,
            cognitive_utilization_rho: rho_cog,
            kingman_attention_wait_s: w_q,
            total_recovery_ttr_s: ttr_tot,
            crisis_survival_tts_s: tts,
            survives_kairos: rho_cog <= 0.76 && ttr_tot <= tts,
        }
    }

    /// Evaluates the three canonical complexity regimes under the benchmark crisis parameters
    /// ($\lambda_a = 0.04\text{ s}^{-1}$, $c_a^2 = 1.5$, $c_s^2 = 1.0$, $\tau_{\mathrm{act}} = 15.0\text{ s}$, $\mathrm{TTS}_{\mathrm{crisis}} = 45.0\text{ s}$):
    /// 1. Compressed Kairós Triad (6 scalars, $30\text{ bits}$);
    /// 2. Standard Epistemic Card (18 scalars, $120\text{ bits}$);
    /// 3. Uncompressed Full Audit (36 scalars, $240\text{ bits}$).
    pub fn evaluate_three_canonical_regimes() -> [Self; 3] {
        [
            Self::evaluate(6, 30.0, 0.04, 1.5, 1.0, 15.0, 45.0),
            Self::evaluate(18, 120.0, 0.04, 1.5, 1.0, 15.0, 45.0),
            Self::evaluate(36, 240.0, 0.04, 1.5, 1.0, 15.0, 45.0),
        ]
    }
}

/// Default institutional $S3^*$ audit key used when sealing a [`TemporalVault`] via [`TemporalVault::seal`]
/// for deterministic corruption/tamper detection in reproducible runs; external key authentication uses
/// [`TemporalVault::seal_with_key`] and [`TemporalVault::unseal_with_key`].
pub const DEFAULT_VAULT_HMAC_KEY: &[u8] = b"stochology-s3star-bitemporal-audit-key-v1";

/// Cryptographic SHA-256 & RFC 2104 HMAC-SHA256 Single-Opening Temporal Vault (`TemporalVault`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemporalVault {
    /// Pre-registered 32-byte SHA-256 commitment of the analysis specification and holdout data.
    pub seal_bytes: [u8; 32],
    /// Pre-registered 64-hex SHA-256 commitment string.
    pub expected_sha256: String,
    /// Pre-registered 64-hex RFC 2104 HMAC-SHA256 authentication tag under the institutional audit key.
    pub expected_hmac_sha256: String,
    /// Ex-ante shift timestamp at which the vault was sealed.
    pub seal_shift: usize,
    /// Ex-post shift timestamp at which the vault was unsealed (`Some(t)` with `t > seal_shift` in two-shift lifecycle).
    pub unseal_shift: Option<usize>,
    /// Number of times the vault has been opened.
    pub open_count: usize,
    /// Whether the vault remains valid for confirmatory claims (`open_count == 1` with matching SHA-256 and HMAC-SHA256).
    pub is_confirmatory: bool,
}

impl TemporalVault {
    /// Creates a new sealed `TemporalVault` committed to `payload_bytes` using [`DEFAULT_VAULT_HMAC_KEY`].
    pub fn seal(payload_bytes: &[u8]) -> Self {
        Self::seal_with_key(DEFAULT_VAULT_HMAC_KEY, payload_bytes)
    }

    /// Creates a new sealed `TemporalVault` committed to `payload_bytes` and authenticated with `hmac_key`.
    pub fn seal_with_key(hmac_key: &[u8], payload_bytes: &[u8]) -> Self {
        Self {
            seal_bytes: sha256_bytes(payload_bytes),
            expected_sha256: sha256_hex(payload_bytes),
            expected_hmac_sha256: hmac_sha256_hex(hmac_key, payload_bytes),
            seal_shift: 0,
            unseal_shift: None,
            open_count: 0,
            is_confirmatory: false,
        }
    }

    /// Attempts to open the vault with `candidate_bytes` using [`DEFAULT_VAULT_HMAC_KEY`].
    pub fn unseal(&mut self, candidate_bytes: &[u8]) -> Result<(), &'static str> {
        self.unseal_with_key(DEFAULT_VAULT_HMAC_KEY, candidate_bytes)
    }

    /// Attempts to open the vault with `candidate_bytes` and verify its keyed RFC 2104 HMAC-SHA256 tag.
    pub fn unseal_with_key(
        &mut self,
        hmac_key: &[u8],
        candidate_bytes: &[u8],
    ) -> Result<(), &'static str> {
        if self.open_count >= 1 {
            self.open_count += 1;
            self.is_confirmatory = false;
            return Err(
                "TemporalVault violation: holdout vault opened more than once (exploratory downgrade)",
            );
        }
        let actual_sha = sha256_hex(candidate_bytes);
        let actual_hmac = hmac_sha256_hex(hmac_key, candidate_bytes);
        if actual_sha != self.expected_sha256 || actual_hmac != self.expected_hmac_sha256 {
            self.is_confirmatory = false;
            return Err("TemporalVault violation: SHA-256 / HMAC-SHA256 pre-registration mismatch");
        }
        self.open_count += 1;
        self.is_confirmatory = true;
        Ok(())
    }

    /// Executes a two-stage bitemporal lifecycle: seals `pre_reg_payload` at ex-ante shift `seal_shift`
    /// and unseals `realized_payload` at ex-post shift `unseal_shift` (requiring `unseal_shift > seal_shift`).
    pub fn verify_two_shift_lifecycle(
        seal_shift: usize,
        unseal_shift: usize,
        pre_reg_payload: &[u8],
        realized_payload: &[u8],
    ) -> Result<Self, &'static str> {
        if unseal_shift <= seal_shift {
            return Err(
                "TemporalVault violation: unseal_shift must be strictly after ex-ante seal_shift",
            );
        }
        let mut vault = Self::seal(pre_reg_payload);
        vault.seal_shift = seal_shift;
        vault.unseal(realized_payload)?;
        vault.unseal_shift = Some(unseal_shift);
        Ok(vault)
    }
}

/// Empirical Alexandra Chouldechova (2017) binary classifier confusion-matrix and fairness-trilemma audit
/// (`[Measured]` on real arrest/outcome records + algebraic verification of Eq. 2.5).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct EmpiricalChouldechovaAudit {
    /// Sample size in Group A ($n_A$).
    pub n_a: usize,
    /// Sample size in Group B ($n_B$).
    pub n_b: usize,
    /// Measured base rate $\pi_A = \mathbb{P}(Y = 1 \mid G = A)$.
    pub prevalence_a: f64,
    /// Measured base rate $\pi_B = \mathbb{P}(Y = 1 \mid G = B)$.
    pub prevalence_b: f64,
    /// Measured Positive Predictive Value $\mathrm{PPV}_A = \mathbb{P}(Y = 1 \mid \hat{Y} = 1, G = A)$.
    pub ppv_a: f64,
    /// Measured Positive Predictive Value $\mathrm{PPV}_B = \mathbb{P}(Y = 1 \mid \hat{Y} = 1, G = B)$.
    pub ppv_b: f64,
    /// Measured False Negative Rate $\mathrm{FNR}_A = \mathbb{P}(\hat{Y} = 0 \mid Y = 1, G = A)$.
    pub fnr_a: f64,
    /// Measured False Negative Rate $\mathrm{FNR}_B = \mathbb{P}(\hat{Y} = 0 \mid Y = 1, G = B)$.
    pub fnr_b: f64,
    /// Measured False Positive Rate $\mathrm{FPR}_A = \mathbb{P}(\hat{Y} = 1 \mid Y = 0, G = A)$.
    pub fpr_a: f64,
    /// Measured False Positive Rate $\mathrm{FPR}_B = \mathbb{P}(\hat{Y} = 1 \mid Y = 0, G = B)$.
    pub fpr_b: f64,
    /// Measured False Positive Rate disparity $\mathrm{FPR}_A / \mathrm{FPR}_B$.
    pub empirical_fpr_ratio: f64,
    /// Exact algebraic closure error $\max_g \bigl|\mathrm{FPR}_g - \frac{\pi_g}{1-\pi_g}\frac{1-\mathrm{PPV}_g}{\mathrm{PPV}_g}(1-\mathrm{FNR}_g)\bigr|$.
    pub identity_closure_error: f64,
}

impl EmpiricalChouldechovaAudit {
    /// Evaluates the confusion matrix `(outcome_y, predicted_high_risk)` for two demographic groups
    /// and verifies Chouldechova's (2017, Eq. 2.5) exact identity on the empirical rates.
    /// If either group is empty or lacks positive/negative conditioning events, `identity_closure_error`
    /// is set to `1.0` so degenerate inputs never silently pass closure verification.
    pub fn evaluate(group_a: &[(bool, bool)], group_b: &[(bool, bool)]) -> Self {
        let stats = |rows: &[(bool, bool)]| -> (usize, f64, f64, f64, f64, f64) {
            if rows.is_empty() {
                return (0, 0.0, 0.0, 0.0, 0.0, 1.0);
            }
            let n = rows.len();
            let mut tp = 0usize;
            let mut fp = 0usize;
            let mut fn_cnt = 0usize;
            let mut tn = 0usize;
            for &(y, y_hat) in rows {
                match (y, y_hat) {
                    (true, true) => tp += 1,
                    (false, true) => fp += 1,
                    (true, false) => fn_cnt += 1,
                    (false, false) => tn += 1,
                }
            }
            if tp + fn_cnt == 0 || fp + tn == 0 || tp == 0 {
                let pos = (tp + fn_cnt).max(1) as f64;
                let neg = (fp + tn).max(1) as f64;
                let pred_pos = (tp + fp).max(1) as f64;
                let prev = ((tp + fn_cnt) as f64) / (n as f64);
                let ppv = (tp as f64) / pred_pos;
                let fnr = (fn_cnt as f64) / pos;
                let fpr = (fp as f64) / neg;
                return (n, prev, ppv, fnr, fpr, 1.0);
            }
            let pos = (tp + fn_cnt) as f64;
            let neg = (fp + tn) as f64;
            let pred_pos = (tp + fp) as f64;
            let prev = pos / (n as f64);
            let ppv = (tp as f64) / pred_pos;
            let fnr = (fn_cnt as f64) / pos;
            let fpr = (fp as f64) / neg;
            let rhs = (prev / (1.0 - prev)) * ((1.0 - ppv) / ppv) * (1.0 - fnr);
            (n, prev, ppv, fnr, fpr, (fpr - rhs).abs())
        };
        let (n_a, prev_a, ppv_a, fnr_a, fpr_a, err_a) = stats(group_a);
        let (n_b, prev_b, ppv_b, fnr_b, fpr_b, err_b) = stats(group_b);
        Self {
            n_a,
            n_b,
            prevalence_a: prev_a,
            prevalence_b: prev_b,
            ppv_a,
            ppv_b,
            fnr_a,
            fnr_b,
            fpr_a,
            fpr_b,
            empirical_fpr_ratio: fpr_a / fpr_b.max(1e-9),
            identity_closure_error: err_a.max(err_b),
        }
    }
}

/// Output of [`GovernanceState::corp_pav_crps_decomposition`]: the CRPS of a deterministic forecast
/// decomposed into the three CORP-PAV components, with two explicit error diagnostics.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CrpsDecomposition {
    /// Grid integral of the Brier path, $\int \mathrm{BS}(\tau)\,d\tau$ — the CRPS of the point forecast.
    pub crps: f64,
    /// Integrated CORP-PAV reliability $\mathrm{REL}_{\mathrm{CORP}}$: the resolvable calibration deficit.
    pub reliability: f64,
    /// Integrated CORP-PAV resolution $\mathrm{RES}_{\mathrm{CORP}}$: discriminative power around the base rate.
    pub resolution: f64,
    /// Integrated Bernoulli uncertainty $\bar{y}(1-\bar{y})$ across thresholds.
    pub uncertainty: f64,
    /// Worst of the algebraic closure error $\lvert \mathrm{CRPS} - (\mathrm{REL} - \mathrm{RES} + \mathrm{UNC}) \rvert$
    /// and the worst per-threshold PAV closure error.
    pub closure_error: f64,
    /// Gap between the grid integral and the closed-form mean absolute error; a property of the
    /// quadrature, not of the forecast.
    pub grid_discretization_error: f64,
    /// Number of points in the threshold grid that was integrated.
    pub threshold_grid_len: usize,
}

/// **Canonical Type-State Structure III — `GovernanceState` ($S_{\mathrm{gov}}$).**
///
/// Encapsulates **Canonical Operator III (The Three Temporal Gates of Institutional Governance)**:
/// 1. **Gate 1 (Ex-Ante):** CORP-PAV isotonic calibration (`corp_resolution`), Invariant Causal Prediction (`icp_*`),
///    Kennedy–O'Hagan discrepancy (`koh_*`), Tian–Pearl bounds (`tian_pearl_*`), and Stackelberg Performative
///    Optimality (`theta_po`, `theta_ps`, `performative_gap`);
/// 2. **Gate 2 (Real-Time):** Beer's $S3^*$ Algedonic Channel with zero Fricker testimonial discount
///    (`algedonic_discount`, `survival_slack_tts`, `recovery_delay_ttr_*`), List–Pettit conjunctive veto,
///    and Gigerenzer / Zheng–Meister natural-frequency alert triage;
/// 3. **Gate 3 (Ex-Post):** Bitemporal `vault_seal`, Sosa/Baron–Hershey `telic_grade`, Holmström–Innes
///    `structural_liability_share` vs. Simon–Girard `scapegoat_blame_transfer` & Bateson `double_bind_intensity`,
///    and Chouldechova's fairness trilemma (`chouldechova_fpr_disparity`).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct GovernanceState {
    // --- Gate 1: Ex-Ante (Pre-Action) ---
    /// Mean Brier Score $\mathrm{BS} = \mathrm{REL}_{\mathrm{CORP}} - \mathrm{RES}_{\mathrm{CORP}} + \mathrm{UNC}_0$.
    pub corp_brier_score: f64,
    /// CORP Reliability (miscalibration loss) $\mathrm{REL}_{\mathrm{CORP}} \ge 0$.
    pub corp_reliability: f64,
    /// CORP Resolution (discriminative power) $\mathrm{RES}_{\mathrm{CORP}} > 0$ (Reflexive Goodhart Brake 1).
    pub corp_resolution: f64,
    /// Marginal base-rate uncertainty $\mathrm{UNC}_0 = \bar{y}(1 - \bar{y})$.
    pub corp_uncertainty: f64,
    /// Exact algebraic closure error $|\mathrm{BS} - (\mathrm{REL} - \mathrm{RES} + \mathrm{UNC}_0)|$.
    pub corp_closure_error: f64,
    /// Invariant Causal Prediction (ICP) slope range for physical predictor across epochs.
    pub icp_invariant_slope_range: f64,
    /// ICP slope range for spurious proxy predictor across epochs.
    pub icp_spurious_slope_range: f64,
    /// Number of candidate predictor subsets $S \subseteq \{0,\dots,d-1\}$ accepted by the ICP Chow invariance test.
    pub icp_accepted_subsets_count: usize,
    /// Bitmask of predictors in the ICP intersection $\hat{S}_{\mathrm{ICP}} = \bigcap_{S\text{ accepted}} S$ (bit $j = 1 \iff X_j \in \hat{S}_{\mathrm{ICP}}$).
    pub icp_identified_mask: u32,
    /// Kennedy–O'Hagan Gaussian Process posterior mean inside historical anchor support ($x_* = 0.0$).
    pub koh_mean_in_sample: f64,
    /// Kennedy–O'Hagan Gaussian Process posterior mean outside historical support ($x_* = 8.0$).
    pub koh_mean_out_of_sample: f64,
    /// Kennedy–O'Hagan Gaussian Process posterior std inside historical anchor support ($x_* = 0.0$).
    pub koh_std_in_sample: f64,
    /// Kennedy–O'Hagan Gaussian Process posterior std outside historical support ($x_* = 8.0$).
    pub koh_std_out_of_sample: f64,
    /// Tian–Pearl (2000) sharp bounds on Probability of Necessity $\mathrm{PN}$.
    pub tian_pearl_pn_bounds: [f64; 2],
    /// Tian–Pearl (2000) sharp bounds on Probability of Sufficiency $\mathrm{PS}$.
    pub tian_pearl_ps_bounds: [f64; 2],
    /// Tian–Pearl (2000) sharp bounds on Probability of Necessity and Sufficiency $\mathrm{PNS}$.
    pub tian_pearl_pns_bounds: [f64; 2],
    /// Perdomo et al. (2020) Performative Stability fixed point $\theta_{\mathrm{PS}}$.
    pub theta_ps: f64,
    /// Perdomo et al. (2020) Stackelberg Performative Optimality point $\theta_{\mathrm{PO}}$.
    pub theta_po: f64,
    /// Performative risk evaluated at $\theta_{\mathrm{PS}}$.
    pub risk_at_ps: f64,
    /// Performative risk evaluated at $\theta_{\mathrm{PO}}$ ($\le \text{risk at }\theta_{\mathrm{PS}}$).
    pub risk_at_po: f64,
    /// Performative gap $|\theta_{\mathrm{PO}} - \theta_{\mathrm{PS}}|$.
    pub performative_gap: f64,
    /// Classified MacKenzie (2006) performativity regime.
    pub mackenzie_regime: MacKenzieRegime,
    /// Dominant Goodhart variant.
    pub goodhart_variant: GoodhartVariant,
    /// True if optimizing the proxy metric caused substantive goal collapse ($\Delta M > 0, \Delta G \le 0$).
    pub goodhart_metric_collapsed: bool,

    // --- Gate 2: Real-Time (Execution) ---
    /// High-powered proxy pressure $\beta_{\mathrm{proxy}} \in [0, 1]$.
    pub proxy_incentive_pressure: f64,
    /// Fricker testimonial credibility discount $\delta_{\mathrm{cred}} \in [0, 1)$ on Beer's $S3^*$ algedonic channel.
    pub algedonic_discount: f64,
    /// Whether the constitutional conjunctive $S3^*$ safety brake is active.
    pub constitutional_brake_active: bool,
    /// **Time-to-Survival Buffer ($\mathrm{TTS}_{\mathrm{survival}}$):** autonomous survival horizon (in shifts).
    pub survival_slack_tts: f64,
    /// **Time-to-Recovery ($\mathrm{TTR}_{\mathrm{recovery}}$) undiscounted:** shifts to trip $S3^*$ when $\delta_{\mathrm{cred}} = 0$.
    pub recovery_delay_ttr_undiscounted: f64,
    /// **Time-to-Recovery ($\mathrm{TTR}_{\mathrm{recovery}}$) discounted:** shifts to trip $S3^*$ under Fricker discount $\delta_{\mathrm{cred}}$.
    pub recovery_delay_ttr_discounted: f64,
    /// True if testimonial injustice delays recovery past survival ($\mathrm{TTR}_{\mathrm{discounted}} > \mathrm{TTS}_{\mathrm{survival}} \ge \mathrm{TTR}_{\mathrm{undiscounted}}$).
    pub fatal_testimonial_delay: bool,
    /// List–Pettit premise-by-premise majority approval.
    pub premise_majority_approves: bool,
    /// List–Pettit constitutional conjunctive approval.
    pub constitutional_conjunctive_approves: bool,
    /// True if List–Pettit discursive dilemma is present.
    pub discursive_dilemma_present: bool,
    /// Unfiltered noisy alarm False Discovery Rate ($\mathrm{FDR} = 1 - \mathrm{PPV}$).
    pub noisy_alert_fdr: f64,
    /// Zheng–Meister / Gigerenzer filtered shift-level Positive Predictive Value ($\mathrm{PPV}$).
    pub filtered_alert_ppv: f64,
    /// Gigerenzer natural-frequency true alerts per $N = 1\,000$ shifts.
    pub natural_freq_true_alerts: u32,
    /// Gigerenzer natural-frequency false alarms per $N = 1\,000$ shifts.
    pub natural_freq_false_alarms: u32,

    // --- Gate 3: Ex-Post (Accountability) ---
    /// SHA-256 ex-ante commitment seal from [`TemporalVault`].
    pub vault_seal: [u8; 32],
    /// True if the bitemporal vault was opened exactly once with matching SHA-256 digest.
    pub vault_confirmatory: bool,
    /// Sosa / Baron–Hershey bitemporal judgment grade.
    pub telic_grade: TelicGrade,
    /// Hindsight outcome bias removed by Stage-1 bitemporal sealing.
    pub eliminated_outcome_bias: f64,
    /// Holmström–Innes share of overload hazard caused by executive slack compression $L_{\mathrm{struct}} \in [0, 1]$.
    pub structural_liability_share: f64,
    /// Share of overload hazard caused by frontline operational variance $L_{\mathrm{oper}} \in [0, 1]$.
    pub frontline_liability_share: f64,
    /// Simon–Girard excess blame shifted onto frontline operators ($\Delta_{\mathrm{scapegoat}} = \max(0, b_{\mathrm{frontline}} - L_{\mathrm{oper}})$).
    pub scapegoat_blame_transfer: f64,
    /// Ratio of assigned frontline blame to true frontline causal variance ($b_{\mathrm{frontline}} / L_{\mathrm{oper}}$).
    pub blame_inversion_ratio: f64,
    /// True if crisis attribution reflects "adulterated air" fraud and scapegoating.
    pub is_adulterated_air_scapegoating: bool,
    /// Bateson synergistic double-bind intensity $\mathrm{DB}_t = \beta_{\mathrm{proxy}} \cdot \delta_{\mathrm{cred}} \cdot (1 - \mathbf{1}\{\text{brake}\}) \in [0, 1]$.
    pub double_bind_intensity: f64,
    /// True if frontline operator is trapped in a Bateson double bind ($\mathrm{DB}_t > 0.20$).
    pub is_double_bind_trap: bool,
    /// Chouldechova group $A$ base rate $p_A$.
    pub chouldechova_prevalence_a: f64,
    /// Chouldechova group $B$ base rate $p_B$.
    pub chouldechova_prevalence_b: f64,
    /// Chouldechova group $A$ induced classifier false-positive rate $\mathrm{FPR}_A = \frac{p_A}{1-p_A}\frac{1-\mathrm{PPV}}{\mathrm{PPV}}(1-\mathrm{FNR}_A)$ (Eq. 2.5).
    pub chouldechova_classifier_fpr_a: f64,
    /// Chouldechova group $B$ induced classifier false-positive rate $\mathrm{FPR}_B = \frac{p_B}{1-p_B}\frac{1-\mathrm{PPV}}{\mathrm{PPV}}(1-\mathrm{FNR}_B)$ (Eq. 2.5).
    pub chouldechova_classifier_fpr_b: f64,
    /// Chouldechova false-positive disparity $\frac{\mathrm{FPR}_A}{\mathrm{FPR}_B}$ (which reduces algebraically to the base-rate odds ratio $\frac{p_A/(1-p_A)}{p_B/(1-p_B)}$ under equal $\mathrm{PPV}$ and $\mathrm{FNR}_A = \mathrm{FNR}_B$).
    pub chouldechova_fpr_disparity: f64,

    // --- Reflexive Regularization & Homothetic Gate Aggregation ---
    /// Empirical split-half cross-validated PAV isotonic calibration optimism $\hat{b}_{\mathrm{PAV}}^{\mathrm{CV}}$.
    pub corp_pav_finite_bias: f64,
    /// Debiased CORP Reliability $\mathrm{REL}_{\mathrm{CORP}}^{\mathrm{debiased}} = \max\bigl(0,\, \mathrm{REL}_{\mathrm{CORP}} - \hat{b}_{\mathrm{PAV}}^{\mathrm{CV}}\bigr)$.
    pub corp_reliability_debiased: f64,
    /// Effective sample size per causal environment $n_{\mathrm{eff}}/\text{env}$ used in ICP.
    pub icp_effective_n_per_env: f64,
    /// ICP identification and statistical power status (`EpistemicUnderpoweredSet` when $n_{\mathrm{eff}}/\text{env} < 150$).
    pub icp_status: IcpIdentificationStatus,
    /// Gate 1 sub-margin 1: CORP-PAV calibration & discriminative resolution score $\in [0, 1]$.
    pub ex_ante_calibration_score: f64,
    /// Gate 1 sub-margin 2: Multi-environment ICP invariance & performative stability score $\in [0, 1]$.
    pub ex_ante_invariance_score: f64,
    /// Gate 1 sub-margin 3: Kennedy–O'Hagan anchor discrepancy & extrapolation discipline score $\in [0, 1]$.
    pub ex_ante_discrepancy_score: f64,
    /// Normalized Gate 1 (Ex-Ante) quality score $g_{\text{ex-ante}} \in [0, 1]$.
    pub gate1_ex_ante_score: f64,
    /// Normalized Gate 2 (Real-Time Kairós) quality score $g_{\text{kairos}} \in [0, 1]$.
    pub gate2_kairos_score: f64,
    /// Normalized Gate 3 (Ex-Post Accountability) quality score $g_{\text{ex-post}} \in [0, 1]$.
    pub gate3_ex_post_score: f64,
    /// Homothetic degree-1 nested Leontief governance score $S_{\mathrm{gov}} = \min(g_{\text{ex-ante}}, g_{\text{kairos}}, g_{\text{ex-post}})$.
    pub nested_leontief_score: f64,
    /// Homothetic degree-1 geometric mean governance score $(g_{\text{ex-ante}} \cdot g_{\text{kairos}} \cdot g_{\text{ex-post}})^{1/3}$.
    pub geometric_gate_score: f64,
}

impl GovernanceState {
    /// **Empirical Split-Half Cross-Validated PAV Isotonic Calibration Optimism:**
    /// Estimates finite-sample in-sample PAV overfitting optimism $\hat{b}_{\mathrm{PAV}}^{\mathrm{CV}}$ directly from
    /// `(forecasts, outcomes)` without label leakage across ties: sorts by `(forecasts[i], i)`, splits into
    /// interleaved even/odd folds, pools tied forecasts within each fold before PAV isotonic calibration, and
    /// compares in-sample vs. cross-fold squared calibration error (halved to match full sample $n$).
    pub fn evaluate_corp_pav_optimism_cv(forecasts: &[f64], outcomes: &[f64]) -> f64 {
        if forecasts.len() < 4 || forecasts.len() != outcomes.len() {
            return 0.0;
        }
        let n = forecasts.len();
        let mut order: Vec<usize> = (0..n).collect();
        order.sort_by(|&i, &j| {
            forecasts[i]
                .total_cmp(&forecasts[j])
                .then_with(|| i.cmp(&j))
        });
        let p_sorted: Vec<f64> = order.iter().map(|&i| forecasts[i]).collect();
        let y_sorted: Vec<f64> = order.iter().map(|&i| outcomes[i]).collect();
        let p_even: Vec<f64> = p_sorted.iter().step_by(2).copied().collect();
        let y_even: Vec<f64> = y_sorted.iter().step_by(2).copied().collect();
        let p_odd: Vec<f64> = p_sorted.iter().skip(1).step_by(2).copied().collect();
        let y_odd: Vec<f64> = y_sorted.iter().skip(1).step_by(2).copied().collect();

        let fit_fold = |p_fold: &[f64], y_fold: &[f64]| -> Vec<f64> {
            let m = p_fold.len();
            let mut idx: Vec<usize> = (0..m).collect();
            idx.sort_by(|&a, &b| {
                p_fold[a]
                    .total_cmp(&p_fold[b])
                    .then_with(|| y_fold[b].total_cmp(&y_fold[a]))
            });
            let y_tie_pooled: Vec<f64> = idx.iter().map(|&k| y_fold[k]).collect();
            let pi_tie_pooled = pav_isotonic_non_decreasing(&y_tie_pooled);
            let mut pi_out = vec![0.0; m];
            for (pos, &orig_k) in idx.iter().enumerate() {
                pi_out[orig_k] = pi_tie_pooled[pos];
            }
            pi_out
        };

        let pi_even = fit_fold(&p_even, &y_even);
        let pi_odd = fit_fold(&p_odd, &y_odd);
        let m = y_even.len().min(y_odd.len());
        if m == 0 {
            return 0.0;
        }
        let mut err_in = 0.0_f64;
        let mut err_oos = 0.0_f64;
        for k in 0..m {
            err_in += (y_even[k] - pi_even[k]).powi(2) + (y_odd[k] - pi_odd[k]).powi(2);
            err_oos += (y_even[k] - pi_odd[k]).powi(2) + (y_odd[k] - pi_even[k]).powi(2);
        }
        let half_opt = ((err_oos - err_in) / ((2 * m) as f64)).max(0.0);
        0.5 * half_opt
    }

    /// Evaluates the exact PAV isotonic CORP Brier decomposition together with the empirical split-half
    /// cross-validated PAV calibration optimism `(bs, rel_raw, res, unc, closure_err, pav_bias_cv, rel_debiased)`.
    pub fn evaluate_corp_pav_regularized(
        forecasts: &[f64],
        outcomes: &[f64],
        _n_eff: f64,
    ) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (bs, rel, res, unc, err) = Self::evaluate_corp_pav(forecasts, outcomes);
        let bias = Self::evaluate_corp_pav_optimism_cv(forecasts, outcomes);
        let rel_deb = (rel - bias).max(0.0);
        (bs, rel, res, unc, err, bias, rel_deb)
    }

    /// Classifies ICP identification status, explicitly returning [`IcpIdentificationStatus::EpistemicUnderpoweredSet`]
    /// when $n_{\mathrm{eff}} / \text{env} < 150.0$ to prevent silent collapse of $\hat{S}_{\mathrm{ICP}} \to \emptyset$.
    #[inline]
    pub fn classify_icp_power(
        n_eff_per_env: f64,
        invariant_slope_range: f64,
        spurious_slope_range: f64,
    ) -> IcpIdentificationStatus {
        if n_eff_per_env < 150.0 {
            IcpIdentificationStatus::EpistemicUnderpoweredSet
        } else if invariant_slope_range <= 0.05 && invariant_slope_range < spurious_slope_range {
            IcpIdentificationStatus::IdentifiedInvariantSet
        } else {
            IcpIdentificationStatus::RefutedAllCandidates
        }
    }

    /// Classifies ICP identification status by combining the subset-intersection mask (`identified_mask`,
    /// `accepted_subsets_count`) from [`Self::evaluate_icp_subset_intersection`] with effective per-environment power.
    #[inline]
    pub fn classify_icp_from_intersection(
        n_eff_per_env: f64,
        identified_mask: u32,
        accepted_subsets_count: usize,
        invariant_slope_range: f64,
        spurious_slope_range: f64,
    ) -> IcpIdentificationStatus {
        if n_eff_per_env < 150.0 {
            IcpIdentificationStatus::EpistemicUnderpoweredSet
        } else if accepted_subsets_count > 0
            && identified_mask != 0
            && invariant_slope_range < spurious_slope_range
        {
            IcpIdentificationStatus::IdentifiedInvariantSet
        } else {
            IcpIdentificationStatus::RefutedAllCandidates
        }
    }

    /// Computes the upper $(1 - \alpha)$ critical value of the $F(df_1, df_2)$ distribution via the
    /// Paulson (1942) / Wilson–Hilferty cube-root normal transformation.
    #[inline]
    pub fn icp_f_critical_value(df1: f64, df2: f64, alpha: f64) -> f64 {
        let d1 = df1.max(1.0);
        let d2 = df2.max(1.0);
        let z = crate::kernel::inverse_standard_normal_cdf(1.0 - alpha.clamp(1e-6, 0.49));
        let a = 2.0 / (9.0 * d1);
        let b = 2.0 / (9.0 * d2);
        let num = (1.0 - b) * (1.0 - a)
            + z * ((1.0 - b).powi(2) * a + (1.0 - a).powi(2) * b - a * b * z * z)
                .max(0.0)
                .sqrt();
        let den = ((1.0 - b).powi(2) - b * z * z).max(0.05);
        (num / den).max(0.5).powi(3)
    }

    /// Fallible version of [`Self::evaluate_icp_subset_intersection`] that rejects predictor dimensions
    /// $d > 12$ instead of silently truncating columns, and uses degrees-of-freedom-adaptive $F_{0.95}(df_1, df_2)$
    /// critical values whenever `f_crit <= 0.0`.
    pub fn try_evaluate_icp_subset_intersection(
        environments: &[CausalEnvironment],
        f_crit: f64,
    ) -> Result<(u32, usize, Vec<f64>), &'static str> {
        if environments.len() < 2 || environments[0].x.is_empty() {
            return Err("ICP requires at least 2 non-empty causal environments");
        }
        let d = environments[0].x[0].len();
        if d == 0 || d > 12 {
            return Err("ICP predictor count d must lie in 1..=12 (no silent column truncation)");
        }
        Ok(Self::evaluate_icp_subset_intersection(environments, f_crit))
    }

    /// **Peters, Bühlmann & Meinshausen (2016) Invariant Causal Prediction (ICP):**
    /// Enumerates candidate subsets $S \subseteq \{0,\dots,d-1\}$, tests cross-environment structural
    /// slope invariance $H_{0,S}: \beta_S^1 = \dots = \beta_S^{|\mathcal{E}|}$ via the dimensionless Chow $F$-statistic,
    /// and returns `(identified_mask, accepted_subsets_count, subset_chow_f_stats)` where
    /// $\hat{S}_{\mathrm{ICP}} = \bigcap_{S : H_{0,S}\text{ accepted}} S$.
    /// When `f_crit <= 0.0`, evaluates each subset against its own degrees-of-freedom-calibrated
    /// critical value [`Self::icp_f_critical_value`]`(df1, df2, 0.05)`.
    pub fn evaluate_icp_subset_intersection(
        environments: &[CausalEnvironment],
        f_crit: f64,
    ) -> (u32, usize, Vec<f64>) {
        if environments.len() < 2 || environments[0].x.is_empty() {
            return (0, 0, Vec::new());
        }
        let d = environments[0].x[0].len();
        if d == 0 || d > 12 {
            return (0, 0, Vec::new());
        }
        let num_subsets = 1usize << d;
        let mut subset_f_stats = Vec::with_capacity(num_subsets);
        let mut accepted_masks = Vec::new();
        let num_envs = environments.len();
        let total_n: usize = environments.iter().map(|e| e.y.len()).sum();

        // Mask 0 (empty set): dimensionless marginal F-test of whether Y is uncorrelated with all predictors.
        let mut f_empty = 0.0_f64;
        let mut df_empty_res = 10.0_f64;
        for j in 0..d {
            for env in environments {
                let n_e = env.y.len();
                if n_e < 3 {
                    continue;
                }
                let y_bar = mean(&env.y);
                let tss: f64 = env.y.iter().map(|&yi| (yi - y_bar).powi(2)).sum();
                let design: Vec<Vec<f64>> = env.x.iter().map(|row| vec![1.0, row[j]]).collect();
                let beta = ols_ridge_fit(&design, &env.y, 1e-8).unwrap_or_else(|| vec![0.0, 0.0]);
                let rss: f64 = env
                    .x
                    .iter()
                    .zip(env.y.iter())
                    .map(|(row, &yi)| {
                        let pred = beta[0] + beta[1] * row[j];
                        (yi - pred).powi(2)
                    })
                    .sum();
                let df_res = (n_e - 2) as f64;
                df_empty_res = df_res;
                let f_je = if rss <= 1e-12 {
                    if (tss - rss).abs() <= 1e-10 { 0.0 } else { 1e6 }
                } else {
                    ((tss - rss).max(0.0)) / (rss / df_res)
                };
                f_empty = f_empty.max(f_je);
            }
        }
        subset_f_stats.push(f_empty);
        let crit_empty = if f_crit > 0.0 {
            f_crit
        } else {
            Self::icp_f_critical_value(1.0, df_empty_res, 0.05)
        };
        if f_empty <= crit_empty {
            accepted_masks.push(0u32);
        }

        for mask in 1..num_subsets {
            let cols: Vec<usize> = (0..d).filter(|&j| (mask & (1 << j)) != 0).collect();
            let k = cols.len();

            // Environment-centered pooled design for common slopes beta_S across environments
            let mut pooled_x = Vec::with_capacity(total_n);
            let mut pooled_y = Vec::with_capacity(total_n);
            let mut rss_unrestricted = 0.0_f64;

            for env in environments {
                let n_e = env.y.len().max(1) as f64;
                let y_bar = mean(&env.y);
                let mut x_bars = vec![0.0; k];
                for (idx, &col) in cols.iter().enumerate() {
                    x_bars[idx] = env.x.iter().map(|r| r[col]).sum::<f64>() / n_e;
                }
                let mut env_cx = Vec::with_capacity(env.y.len());
                let mut env_cy = Vec::with_capacity(env.y.len());
                for (row, &yi) in env.x.iter().zip(env.y.iter()) {
                    let crow: Vec<f64> = cols
                        .iter()
                        .enumerate()
                        .map(|(idx, &col)| row[col] - x_bars[idx])
                        .collect();
                    pooled_x.push(crow.clone());
                    pooled_y.push(yi - y_bar);
                    env_cx.push(crow);
                    env_cy.push(yi - y_bar);
                }
                let beta_e = ols_ridge_fit(&env_cx, &env_cy, 1e-8).unwrap_or_else(|| vec![0.0; k]);
                for (crow, &cyi) in env_cx.iter().zip(env_cy.iter()) {
                    let pred: f64 = crow.iter().zip(beta_e.iter()).map(|(&x, &b)| x * b).sum();
                    let res = cyi - pred;
                    rss_unrestricted += res * res;
                }
            }

            let beta_pool =
                ols_ridge_fit(&pooled_x, &pooled_y, 1e-8).unwrap_or_else(|| vec![0.0; k]);
            let mut rss_restricted = 0.0_f64;
            for (crow, &cyi) in pooled_x.iter().zip(pooled_y.iter()) {
                let pred: f64 = crow
                    .iter()
                    .zip(beta_pool.iter())
                    .map(|(&x, &b)| x * b)
                    .sum();
                let res = cyi - pred;
                rss_restricted += res * res;
            }

            // On the centered data (`pooled_x`, `pooled_y` with no intercept column), the unrestricted
            // fit estimates `num_envs * k` slope parameters vs `k` pooled slopes (`df1 = (num_envs - 1) * k`),
            // with residual degrees of freedom `df2 = total_n - num_envs * k` on the centered system.
            let df1 = ((num_envs - 1) * k).max(1) as f64;
            let df2 = total_n.saturating_sub(num_envs * k).max(1) as f64;
            let diff_rss = (rss_restricted - rss_unrestricted).max(0.0);
            let f_chow = if rss_unrestricted <= 1e-12 {
                if diff_rss <= 1e-10 { 0.0 } else { 1e6 }
            } else {
                (diff_rss / df1) / (rss_unrestricted / df2)
            };
            subset_f_stats.push(f_chow);
            let crit_sub = if f_crit > 0.0 {
                f_crit
            } else {
                Self::icp_f_critical_value(df1, df2, 0.05)
            };
            if f_chow <= crit_sub {
                accepted_masks.push(mask as u32);
            }
        }

        let identified_mask = if accepted_masks.is_empty() {
            0
        } else {
            accepted_masks
                .iter()
                .copied()
                .fold((1u32 << d) - 1, |acc, m| acc & m)
        };
        (identified_mask, accepted_masks.len(), subset_f_stats)
    }

    /// **Perdomo et al. (2020) & MacKenzie (2006) Performative Risk Evaluator:**
    /// Evaluates performative stability $\theta_{\mathrm{PS}}$, performative optimality $\theta_{\mathrm{PO}}$,
    /// their risks `(risk_ps, risk_po)`, gap $|\theta_{\mathrm{PS}} - \theta_{\mathrm{PO}}|$, and [`MacKenzieRegime`]
    /// (including `CounterperformativeDestructive` when performative feedback $\varepsilon$ causes `risk_ps > 2 * risk_po`).
    pub fn evaluate_performative_system(
        mu0: f64,
        eps: f64,
        lam: f64,
        sig2: f64,
    ) -> (f64, f64, f64, f64, f64, MacKenzieRegime) {
        let denom_ps =
            (1.0 + lam - eps).abs().max(1e-6) * if 1.0 + lam - eps < 0.0 { -1.0 } else { 1.0 };
        let theta_ps = mu0 / denom_ps;
        let one_minus_eps = 1.0 - eps;
        let theta_po = (one_minus_eps * mu0) / (one_minus_eps * one_minus_eps + lam.max(1e-6));
        let eval_risk = |th: f64| -> f64 {
            let bias = (mu0 + eps * th) - th;
            0.5 * (bias * bias + sig2.max(0.0)) + 0.5 * lam.max(0.0) * th * th
        };
        let risk_ps = eval_risk(theta_ps);
        let risk_po = eval_risk(theta_po);
        let gap = (theta_ps - theta_po).abs();
        let regime = MacKenzieRegime::classify(eps, gap, risk_ps > 2.0 * risk_po);
        (theta_ps, theta_po, risk_ps, risk_po, gap, regime)
    }

    /// **Tian & Pearl (2000) Sharp Counterfactual Bounds:**
    /// Computes sharp bounds on Probability of Necessity $\mathrm{PN}$, Probability of Sufficiency $\mathrm{PS}$,
    /// and Probability of Necessity and Sufficiency $\mathrm{PNS}$ combining experimental
    /// ($P(y_x), P(y_{x'})$) and observational ($P(x,y), P(x,y'), P(x',y), P(x',y')$) data.
    pub fn evaluate_tian_pearl_bounds(
        exp_y1: f64,
        exp_y0: f64,
        p_x1_y1: f64,
        p_x1_y0: f64,
        p_x0_y1: f64,
        p_x0_y0: f64,
    ) -> ([f64; 2], [f64; 2], [f64; 2]) {
        let p_y = p_x1_y1 + p_x0_y1;
        let pns_lo = (exp_y1 - exp_y0)
            .max(p_y - exp_y0)
            .max(exp_y1 - p_y)
            .max(0.0);
        let pns_hi = exp_y1
            .min(1.0 - exp_y0)
            .min(p_x1_y1 + p_x0_y0)
            .min(exp_y1 - exp_y0 + p_x1_y0 + p_x0_y1)
            .clamp(pns_lo, 1.0);
        let denom_pn = p_x1_y1.max(1e-12);
        let pn_lo = ((p_y - exp_y0).max(0.0) / denom_pn).clamp(0.0, 1.0);
        let pn_hi = ((1.0 - exp_y0 - p_x0_y0).clamp(0.0, p_x1_y1) / denom_pn).clamp(pn_lo, 1.0);
        let denom_ps = p_x0_y0.max(1e-12);
        let ps_lo = ((exp_y1 - p_y).max(0.0) / denom_ps).clamp(0.0, 1.0);
        let ps_hi = ((exp_y1 - p_x1_y1).clamp(0.0, p_x0_y0) / denom_ps).clamp(ps_lo, 1.0);
        ([pn_lo, pn_hi], [ps_lo, ps_hi], [pns_lo, pns_hi])
    }

    /// **Chouldechova (2017) Classifier False-Positive Disparity (Eq. 2.5):**
    /// Evaluates induced group false-positive rates
    /// $\mathrm{FPR}_g = \frac{p_g}{1-p_g}\frac{1-\mathrm{PPV}}{\mathrm{PPV}}(1-\mathrm{FNR}_g)$
    /// and their ratio $\mathrm{FPR}_A / \mathrm{FPR}_B$, which reduces algebraically to the base-rate
    /// odds ratio $\frac{p_A/(1-p_A)}{p_B/(1-p_B)}$ whenever $\mathrm{PPV}$ and $\mathrm{FNR}$ are equal across groups.
    #[inline]
    pub fn evaluate_chouldechova_classifier_fpr(
        prevalence_a: f64,
        prevalence_b: f64,
        ppv: f64,
        fnr_a: f64,
        fnr_b: f64,
    ) -> (f64, f64, f64) {
        let pa = prevalence_a.clamp(1e-6, 1.0 - 1e-6);
        let pb = prevalence_b.clamp(1e-6, 1.0 - 1e-6);
        let ppv_c = ppv.clamp(1e-6, 1.0 - 1e-6);
        let odds_a = pa / (1.0 - pa);
        let odds_b = pb / (1.0 - pb);
        let ppv_factor = (1.0 - ppv_c) / ppv_c;
        let fpr_a = odds_a * ppv_factor * (1.0 - fnr_a.clamp(0.0, 0.999));
        let fpr_b = odds_b * ppv_factor * (1.0 - fnr_b.clamp(0.0, 0.999));
        let disparity = fpr_a / fpr_b.max(1e-12);
        (fpr_a, fpr_b, disparity)
    }

    /// **Canonical Operator III Score Aggregator ($S_{\mathrm{gov}} \in [0, 1]$):**
    /// Combines the three dimensionless temporal gate margins (`gate1_ex_ante`, `gate2_kairos`, `gate3_ex_post`)
    /// into the homothetic degree-1 nested Leontief score $S_{\mathrm{gov}} = \min(g_1, g_2, g_3)$
    /// and the geometric gate mean $(g_1 g_2 g_3)^{1/3}$.
    #[inline]
    pub fn canonical_score(
        gate1_ex_ante: f64,
        gate2_kairos: f64,
        gate3_ex_post: f64,
    ) -> (f64, f64) {
        if ![gate1_ex_ante, gate2_kairos, gate3_ex_post]
            .iter()
            .all(|x| x.is_finite())
        {
            return (0.0, 0.0);
        }
        let g1 = gate1_ex_ante.clamp(0.0, 1.0);
        let g2 = gate2_kairos.clamp(0.0, 1.0);
        let g3 = gate3_ex_post.clamp(0.0, 1.0);
        let nested_min = g1.min(g2).min(g3);
        let geom_score = (g1 * g2 * g3).cbrt();
        (nested_min, geom_score)
    }

    /// Validates that `outcomes` are binary $\{0, 1\}$ before evaluating the PAV isotonic CORP Brier decomposition.
    pub fn try_evaluate_corp_pav(
        forecasts: &[f64],
        outcomes: &[f64],
    ) -> Result<(f64, f64, f64, f64, f64), &'static str> {
        if forecasts.is_empty() || forecasts.len() != outcomes.len() {
            return Err("CORP-PAV requires non-empty equal-length forecast and outcome slices");
        }
        let is_binary = outcomes
            .iter()
            .all(|&y| y.abs() <= 1e-9 || (y - 1.0).abs() <= 1e-9);
        if !is_binary {
            return Err(
                "CORP-PAV Bernoulli uncertainty y_bar*(1-y_bar) requires binary outcomes y in {0, 1}",
            );
        }
        Ok(Self::evaluate_corp_pav(forecasts, outcomes))
    }

    /// Evaluates the PAV isotonic CORP Brier decomposition `(brier_score, resolvable_calibration_deficit, resolution, uncertainty, closure_error)`.
    ///
    /// Here `reliability` ($\mathrm{REL}_{\mathrm{CORP}} = \mathrm{BS}(p) - \mathrm{BS}(\pi)$) is the PAV resolvable calibration deficit.
    /// For binary outcomes $y_i \in \{0, 1\}$, $\mathrm{UNC}_0 = \bar{y}(1 - \bar{y}) = \frac{1}{n}\sum (y_i - \bar{y})^2$ and
    /// PAV block-orthogonality implies $\mathrm{BS}(\pi) = \mathrm{UNC}_0 - \mathrm{RES}_{\mathrm{CORP}}$.
    /// For non-binary outcomes $y_i \notin \{0, 1\}$, $\bar{y}(1-\bar{y}) \ne \frac{1}{n}\sum (y_i - \bar{y})^2$, yielding a non-zero `closure_error`.
    pub fn evaluate_corp_pav(forecasts: &[f64], outcomes: &[f64]) -> (f64, f64, f64, f64, f64) {
        if forecasts.is_empty() || forecasts.len() != outcomes.len() {
            return (0.0, 0.0, 0.0, 0.0, 1.0);
        }
        let n = forecasts.len();
        let nf = n as f64;
        let mut order: Vec<usize> = (0..n).collect();
        order.sort_by(|&i, &j| {
            forecasts[i]
                .total_cmp(&forecasts[j])
                .then_with(|| outcomes[j].total_cmp(&outcomes[i]))
        });

        let y_sorted: Vec<f64> = order.iter().map(|&i| outcomes[i]).collect();
        let p_sorted: Vec<f64> = order.iter().map(|&i| forecasts[i]).collect();
        let pi_sorted = pav_isotonic_non_decreasing(&y_sorted);

        let y_bar = mean(outcomes);
        let unc = y_bar * (1.0 - y_bar);
        let bs = p_sorted
            .iter()
            .zip(y_sorted.iter())
            .map(|(&p, &y)| (p - y) * (p - y))
            .sum::<f64>()
            / nf;
        let bs_cal = pi_sorted
            .iter()
            .zip(y_sorted.iter())
            .map(|(&pi, &y)| (pi - y) * (pi - y))
            .sum::<f64>()
            / nf;
        let rel = (bs - bs_cal).max(0.0);
        let res = pi_sorted
            .iter()
            .map(|&pi| (pi - y_bar) * (pi - y_bar))
            .sum::<f64>()
            / nf;
        let is_binary = outcomes
            .iter()
            .all(|&y| y.abs() <= 1e-9 || (y - 1.0).abs() <= 1e-9);
        let raw_err = (bs - (rel - res + unc)).abs();
        let err = if is_binary {
            raw_err
        } else {
            let emp_var = y_sorted
                .iter()
                .map(|&y| (y - y_bar) * (y - y_bar))
                .sum::<f64>()
                / nf;
            raw_err.max((unc - emp_var).abs())
        };
        (bs, rel, res, unc, err)
    }

    /// **Continuous Ranked Probability Score of a deterministic forecast, decomposed by the same
    /// CORP-PAV machinery as the Brier score.**
    ///
    /// Hersbach's identity writes the CRPS of a predictive distribution $F$ against an observation $y$ as
    /// an integral of Brier scores over the threshold axis:
    /// $$\mathrm{CRPS}(F, y) = \int \bigl(F(\tau) - \mathbf{1}\{y \le \tau\}\bigr)^2 \, d\tau .$$
    /// For a *point* forecast $f$ the distribution is the step $F(\tau) = \mathbf{1}\{f \le \tau\}$, so every
    /// threshold contributes a binary Brier score whose full CORP-PAV decomposition
    /// (reliability, resolution, uncertainty) is available, and the four quantities integrate with the
    /// **same** quadrature weights. Linearity of that quadrature is what makes the decomposition hold to
    /// floating-point precision rather than approximately.
    ///
    /// The reported `crps` is the grid integral of the Brier path, not the closed form mean absolute
    /// error; `grid_discretization_error` reports the gap between the two, which vanishes as the grid is
    /// refined and is a property of the quadrature rather than of the forecast.
    pub fn corp_pav_crps_decomposition(
        forecasts: &[f64],
        outcomes: &[f64],
        threshold_grid: &[f64],
    ) -> Result<CrpsDecomposition, &'static str> {
        if forecasts.is_empty() || forecasts.len() != outcomes.len() {
            return Err(
                "CRPS decomposition requires non-empty equal-length forecast and outcome slices",
            );
        }
        if forecasts.len() < 2 {
            return Err("CRPS decomposition requires at least two cases");
        }
        if threshold_grid.len() < 2 {
            return Err("CRPS decomposition requires a threshold grid with at least two points");
        }
        if !threshold_grid.windows(2).all(|w| w[1] > w[0]) {
            return Err("threshold grid must be strictly increasing");
        }
        if !forecasts
            .iter()
            .chain(outcomes.iter())
            .chain(threshold_grid.iter())
            .all(|v| v.is_finite())
        {
            return Err("all CRPS inputs must be finite");
        }

        let n = forecasts.len();
        let mut binary_forecasts = vec![0.0_f64; n];
        let mut binary_outcomes = vec![0.0_f64; n];
        let (mut crps, mut rel, mut res, mut unc) = (0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64);
        let mut threshold_closure = 0.0_f64;

        for cell in threshold_grid.windows(2) {
            let tau = 0.5 * (cell[0] + cell[1]);
            let weight = cell[1] - cell[0];
            for i in 0..n {
                binary_forecasts[i] = if forecasts[i] <= tau { 1.0 } else { 0.0 };
                binary_outcomes[i] = if outcomes[i] <= tau { 1.0 } else { 0.0 };
            }
            let (bs, r, s, u, e) = Self::evaluate_corp_pav(&binary_forecasts, &binary_outcomes);
            crps += weight * bs;
            rel += weight * r;
            res += weight * s;
            unc += weight * u;
            threshold_closure = threshold_closure.max(e);
        }

        let mae = forecasts
            .iter()
            .zip(outcomes.iter())
            .map(|(&f, &y)| (f - y).abs())
            .sum::<f64>()
            / n as f64;
        let identity_error = (crps - (rel - res + unc)).abs();

        Ok(CrpsDecomposition {
            crps,
            reliability: rel,
            resolution: res,
            uncertainty: unc,
            closure_error: identity_error.max(threshold_closure),
            grid_discretization_error: (crps - mae).abs(),
            threshold_grid_len: threshold_grid.len(),
        })
    }

    /// **Gate 1 (Ex-Ante Operator · Chapter 7 [formerly Ch. 6]):**
    /// Evaluates CORP-PAV calibration, Invariant Causal Prediction across environments,
    /// Kennedy–O'Hagan Gaussian Process discrepancy, Tian–Pearl bounds, Perdomo/MacKenzie
    /// Stackelberg Performative Optimality ($\theta_{\mathrm{PO}}$ vs. $\theta_{\mathrm{PS}}$), and Goodhart collapse.
    pub fn from_ex_ante_gate(
        forecasts: &[f64],
        outcomes: &[f64],
        environments: &[CausalEnvironment],
    ) -> Self {
        if forecasts.is_empty()
            || forecasts.len() != outcomes.len()
            || environments.len() < 2
            || environments[0].x.is_empty()
        {
            return Self::default();
        }
        let deff_seq = if forecasts.len() >= 10 {
            let m_f = mean(forecasts);
            let mut num = 0.0_f64;
            let mut den = 0.0_f64;
            for i in 0..forecasts.len() {
                let d0 = forecasts[i] - m_f;
                den += d0 * d0;
                if i + 1 < forecasts.len() {
                    num += d0 * (forecasts[i + 1] - m_f);
                }
            }
            let rho1 = if den > 1e-12 {
                (num / den).clamp(0.0, 0.92)
            } else {
                0.0
            };
            ((1.0 + rho1) / (1.0 - rho1)).clamp(1.0, 25.0)
        } else {
            1.0
        };
        let n_eff_for_corp = ((forecasts.len() as f64) / deff_seq).max(1.0);
        let (bs, rel, res, unc, err, pav_bias, rel_debiased) =
            Self::evaluate_corp_pav_regularized(forecasts, outcomes, n_eff_for_corp);

        let d = environments[0].x[0].len();
        let mut slope_ranges = Vec::with_capacity(d);
        for j in 0..d {
            let mut slopes = Vec::with_capacity(environments.len());
            for env in environments {
                let design: Vec<Vec<f64>> = env.x.iter().map(|row| vec![1.0, row[j]]).collect();
                let beta = ols_ridge_fit(&design, &env.y, 1e-8).unwrap_or_else(|| vec![0.0, 0.0]);
                slopes.push(beta[1]);
            }
            let min_s = slopes.iter().copied().fold(f64::INFINITY, f64::min);
            let max_s = slopes.iter().copied().fold(f64::NEG_INFINITY, f64::max);
            slope_ranges.push((max_s - min_s).max(0.0));
        }
        let inv_range = slope_ranges.first().copied().unwrap_or(0.0);
        let spur_range = slope_ranges.get(1).copied().unwrap_or(0.0);
        let (icp_mask, icp_acc_count, _) =
            Self::evaluate_icp_subset_intersection(environments, 0.0);

        // Kennedy-O'Hagan anchored Gaussian Process discrepancy (anchor_x=[-1,0,1], empirical tercile residuals)
        let anchor_x = [-1.0_f64, 0.0, 1.0];
        let anchor_r = if forecasts.len() >= 3 && forecasts.len() == outcomes.len() {
            let mut pairs: Vec<(f64, f64)> = forecasts
                .iter()
                .copied()
                .zip(outcomes.iter().copied())
                .collect();
            pairs.sort_by(|a, b| a.0.total_cmp(&b.0));
            let third = (pairs.len() / 3).max(1);
            let mid_end = (2 * third).min(pairs.len());
            let r0 = pairs[..third].iter().map(|&(f, y)| y - f).sum::<f64>() / (third as f64);
            let r1 = pairs[third..mid_end]
                .iter()
                .map(|&(f, y)| y - f)
                .sum::<f64>()
                / ((mid_end - third).max(1) as f64);
            let r2 = pairs[mid_end..].iter().map(|&(f, y)| y - f).sum::<f64>()
                / ((pairs.len() - mid_end).max(1) as f64);
            [r0, r1, r2]
        } else {
            [0.08_f64, -0.03, 0.05]
        };
        let anchor_rms =
            ((anchor_r[0].powi(2) + anchor_r[1].powi(2) + anchor_r[2].powi(2)) / 3.0).sqrt();
        // Prior kernel scale s2 = 0.50^2 [Assumed], observation residual variance n2 coupled to empirical tercile discrepancy
        let s2 = 0.50_f64 * 0.50;
        let l2 = 0.65_f64 * 0.65;
        let n2 = (0.04 + 0.15 * anchor_rms).powi(2);
        let mut gram = vec![vec![0.0; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                let diff = anchor_x[i] - anchor_x[j];
                gram[i][j] = s2 * (-0.5 * diff * diff / l2).exp() + if i == j { n2 } else { 0.0 };
            }
        }
        let alpha_gp = solve_cholesky_spd(&gram, &anchor_r).unwrap_or_else(|| vec![0.0; 3]);
        let koh_posterior = |x_star: f64| -> (f64, f64) {
            let k_star: Vec<f64> = anchor_x
                .iter()
                .map(|&xi| {
                    let diff = x_star - xi;
                    s2 * (-0.5 * diff * diff / l2).exp()
                })
                .collect();
            let mean_star: f64 = k_star
                .iter()
                .zip(alpha_gp.iter())
                .map(|(&k, &a)| k * a)
                .sum();
            let v = solve_cholesky_spd(&gram, &k_star).unwrap_or_else(|| vec![0.0; 3]);
            let red: f64 = k_star.iter().zip(v.iter()).map(|(&k, &vi)| k * vi).sum();
            let std_star = (s2 - red).max(1e-12).sqrt();
            (mean_star, std_star)
        };
        let (mean_in, std_in) = koh_posterior(0.0);
        let (mean_out, std_out) = koh_posterior(8.0);

        // Tian-Pearl counterfactual bounds (exp_y1=0.70, exp_y0=0.20, p11=0.40, p10=0.10, p01=0.10, p00=0.40) [Assumed benchmark]
        let (pn_bounds, ps_bounds, pns_bounds) =
            Self::evaluate_tian_pearl_bounds(0.70, 0.20, 0.40, 0.10, 0.10, 0.40);

        // Perdomo/MacKenzie Performative System (mu0=1.0, eps=0.45, lambda=0.20, sigma2=0.10) [Assumed benchmark]
        let (theta_ps, theta_po, risk_ps, risk_po, gap, regime) =
            Self::evaluate_performative_system(1.0, 0.45, 0.20, 0.10);

        // Goodhart audit (proxy_gain = +0.35, substantive_goal_change = -0.12) [Assumed benchmark]
        let proxy_gain = 0.35_f64;
        let substantive_goal_change = -0.12_f64;
        let collapsed = proxy_gain > 0.0 && substantive_goal_change <= 0.0;
        let goodhart_var = GoodhartVariant::classify(true, false, 3.0);

        let min_env_n = environments.iter().map(|e| e.y.len()).min().unwrap_or(1) as f64;
        let icp_n_eff = (min_env_n / deff_seq).max(1.0);
        let icp_stat = Self::classify_icp_from_intersection(
            icp_n_eff,
            icp_mask,
            icp_acc_count,
            inv_range,
            spur_range,
        );

        let res_ratio = (res / unc.max(1e-6)).clamp(0.0, 1.0);
        let cal_score =
            ((1.0 - rel_debiased).clamp(0.0, 1.0) * (0.85 + 0.15 * res_ratio)).clamp(0.0, 1.0);
        let icp_stability =
            (1.0 - inv_range / (inv_range + spur_range + 0.05).max(1e-6)).clamp(0.0, 1.0);
        let mask_factor = if icp_mask != 0 { 1.0 } else { 0.85 };
        let inv_score = ((1.0 - (risk_ps - risk_po)).clamp(0.0, 1.0)
            * (0.88 + 0.12 * icp_stability)
            * mask_factor)
            .clamp(0.0, 1.0);
        let koh_shift = 0.5 * (mean_in.abs() + mean_out.abs());
        let koh_var_inflation = ((std_out - std_in).max(0.0) / std_out.max(1e-6)).clamp(0.0, 1.0);
        let disc_score =
            (1.0 - anchor_rms - 0.15 * koh_shift - 0.08 * koh_var_inflation).clamp(0.0, 1.0);
        let (nested_min, geom_score) = Self::canonical_score(cal_score, inv_score, disc_score);

        Self {
            corp_brier_score: bs,
            corp_reliability: rel,
            corp_resolution: res,
            corp_uncertainty: unc,
            corp_closure_error: err,
            icp_invariant_slope_range: inv_range,
            icp_spurious_slope_range: spur_range,
            icp_accepted_subsets_count: icp_acc_count,
            icp_identified_mask: icp_mask,
            koh_mean_in_sample: mean_in,
            koh_mean_out_of_sample: mean_out,
            koh_std_in_sample: std_in,
            koh_std_out_of_sample: std_out,
            tian_pearl_pn_bounds: pn_bounds,
            tian_pearl_ps_bounds: ps_bounds,
            tian_pearl_pns_bounds: pns_bounds,
            theta_ps,
            theta_po,
            risk_at_ps: risk_ps,
            risk_at_po: risk_po,
            performative_gap: gap,
            mackenzie_regime: regime,
            goodhart_variant: goodhart_var,
            goodhart_metric_collapsed: collapsed,
            constitutional_brake_active: true,
            corp_pav_finite_bias: pav_bias,
            corp_reliability_debiased: rel_debiased,
            icp_effective_n_per_env: icp_n_eff,
            icp_status: icp_stat,
            ex_ante_calibration_score: cal_score,
            ex_ante_invariance_score: inv_score,
            ex_ante_discrepancy_score: disc_score,
            gate1_ex_ante_score: nested_min,
            gate2_kairos_score: 1.0,
            gate3_ex_post_score: 1.0,
            nested_leontief_score: nested_min,
            geometric_gate_score: geom_score,
            ..Self::default()
        }
    }

    /// **Gate 2 & Gate 3 (Real-Time & Ex-Post Operators · Chapter 8):**
    /// Evaluates Beer's $S3^*$ Algedonic Channel under Fricker testimonial injustice ($\mathrm{TTS}_{\mathrm{survival}} \ge \mathrm{TTR}_{\mathrm{recovery}}$),
    /// List–Pettit discursive dilemma, Zheng–Meister & Gigerenzer natural-frequency alert filter,
    /// Sosa/Baron–Hershey `TelicGrade`, Holmström–Innes structural liability vs. Simon–Girard scapegoating &
    /// Bateson double-bind ($DB_t$), Chouldechova's fairness trilemma, and `TemporalVault`.
    pub fn from_real_time_and_ex_post_gates(
        prevalence_a: f64,
        prevalence_b: f64,
        frontline_shift_e_factor: f64,
        trip_threshold: f64,
        testimonial_discount_delta: f64,
        survival_slack_tts: f64,
        vault_payload: &[u8],
    ) -> Result<Self, &'static str> {
        // Chouldechova fairness trilemma (with classifier PPV = 0.65, equal FNR = 0.20 across groups)
        let pa = prevalence_a.clamp(1e-6, 1.0 - 1e-6);
        let pb = prevalence_b.clamp(1e-6, 1.0 - 1e-6);
        let (fpr_a, fpr_b, fpr_disparity) =
            Self::evaluate_chouldechova_classifier_fpr(pa, pb, 0.65, 0.20, 0.20);

        // List-Pettit 3-member committee votes: [[true, true, false], [true, false, true], [false, true, true]]
        let votes = [
            [true, true, false],
            [true, false, true],
            [false, true, true],
        ];
        let (premise_maj, const_ok, _) = Self::evaluate_list_pettit_votes(&votes);

        // Beer S3* Algedonic Channel & Fricker delay via AlgedonicChannel
        let algedonic = AlgedonicChannel::evaluate(
            frontline_shift_e_factor,
            trip_threshold,
            testimonial_discount_delta,
            survival_slack_tts,
        );

        // Sosa / Baron-Hershey telic evaluation (ex-ante=0.92, cleared=true, outcome=false, hindsight_penalty=0.45)
        let telic_grade = TelicGrade::classify(true, false);
        let bias_removed = 0.45_f64;

        // Holmström-Innes structural vs operational liability via ContractAndLiabilityAudit
        let liability = ContractAndLiabilityAudit::from_deficit_series(
            &[-3.162_277_660, 3.162_277_660],
            &[-0.866_025_404, 0.866_025_404],
            &[-1.118_033_989, 1.118_033_989],
            0.85,
        );

        // Bateson double-bind under Collapse Proposition 3 (beta=0.85, delta=0.60, brake=false)
        let bateson = BatesonDoubleBindAudit::evaluate(0.85, 0.60, false);

        // Zheng-Meister & Gigerenzer natural-frequency alert filter (N=1000, pi=0.02)
        let noisy_filter = CognitiveAlertFilter::evaluate(0.02, 0.80, 0.25, 1_000);
        let clean_filter = CognitiveAlertFilter::evaluate(0.02, 0.82, 0.015, 1_000);

        // TemporalVault two-shift bitemporal lifecycle (seal at t_0 = 0, unseal at t_1 = 1)
        let vault = TemporalVault::verify_two_shift_lifecycle(0, 1, vault_payload, vault_payload)?;

        let g1_score = (1.0 - 0.02 * (fpr_disparity - 1.0).max(0.0)).clamp(0.50, 0.98);
        let g2_score = (1.0 - algedonic.testimonial_discount).clamp(0.0, 1.0);
        let g3_score = if vault.is_confirmatory {
            (1.0 - liability.frontline_liability_share).clamp(0.0, 1.0)
        } else {
            0.0
        };
        let (nested_min, geom_score) = Self::canonical_score(g1_score, g2_score, g3_score);

        Ok(Self {
            proxy_incentive_pressure: bateson.proxy_pressure,
            algedonic_discount: algedonic.testimonial_discount,
            constitutional_brake_active: const_ok,
            survival_slack_tts: algedonic.tts_survival,
            recovery_delay_ttr_undiscounted: algedonic.ttr_undiscounted,
            recovery_delay_ttr_discounted: algedonic.ttr_discounted,
            fatal_testimonial_delay: algedonic.fatal_testimonial_delay,
            premise_majority_approves: premise_maj,
            constitutional_conjunctive_approves: const_ok,
            discursive_dilemma_present: premise_maj && !const_ok,
            noisy_alert_fdr: noisy_filter.fdr,
            filtered_alert_ppv: clean_filter.ppv,
            natural_freq_true_alerts: clean_filter.natural_freq_true_alerts,
            natural_freq_false_alarms: clean_filter.natural_freq_false_alarms,
            vault_seal: vault.seal_bytes,
            vault_confirmatory: vault.is_confirmatory,
            telic_grade,
            eliminated_outcome_bias: bias_removed,
            structural_liability_share: liability.structural_liability_share,
            frontline_liability_share: liability.frontline_liability_share,
            scapegoat_blame_transfer: liability.scapegoat_blame_transfer,
            blame_inversion_ratio: liability.blame_inversion_ratio,
            is_adulterated_air_scapegoating: liability.is_scapegoating,
            double_bind_intensity: bateson.double_bind_intensity,
            is_double_bind_trap: bateson.is_double_bind_trap,
            chouldechova_prevalence_a: pa,
            chouldechova_prevalence_b: pb,
            chouldechova_classifier_fpr_a: fpr_a,
            chouldechova_classifier_fpr_b: fpr_b,
            chouldechova_fpr_disparity: fpr_disparity,
            gate1_ex_ante_score: g1_score,
            gate2_kairos_score: g2_score,
            gate3_ex_post_score: g3_score,
            nested_leontief_score: nested_min,
            geometric_gate_score: geom_score,
            ..Self::default()
        })
    }

    /// Unifies a Gate-1 (`from_ex_ante_gate`) state with a Gate-2/Gate-3 (`from_real_time_and_ex_post_gates`)
    /// state through the single canonical nested Leontief aggregator
    /// $S_{\mathrm{gov}} = \min(g_{\text{ex-ante}},\, g_{\text{kairos}},\, g_{\text{ex-post}})$ (`G1`).
    pub fn from_all_three_gates(ex_ante: &Self, real_time_and_ex_post: &Self) -> Self {
        let g1 = ex_ante.gate1_ex_ante_score;
        let g2 = real_time_and_ex_post.gate2_kairos_score;
        let g3 = real_time_and_ex_post.gate3_ex_post_score;
        let (nested_min, geom_score) = Self::canonical_score(g1, g2, g3);
        let mut combined = *real_time_and_ex_post;
        combined.corp_brier_score = ex_ante.corp_brier_score;
        combined.corp_reliability = ex_ante.corp_reliability;
        combined.corp_resolution = ex_ante.corp_resolution;
        combined.corp_uncertainty = ex_ante.corp_uncertainty;
        combined.corp_closure_error = ex_ante.corp_closure_error;
        combined.icp_invariant_slope_range = ex_ante.icp_invariant_slope_range;
        combined.icp_spurious_slope_range = ex_ante.icp_spurious_slope_range;
        combined.icp_accepted_subsets_count = ex_ante.icp_accepted_subsets_count;
        combined.icp_identified_mask = ex_ante.icp_identified_mask;
        combined.koh_mean_in_sample = ex_ante.koh_mean_in_sample;
        combined.koh_mean_out_of_sample = ex_ante.koh_mean_out_of_sample;
        combined.koh_std_in_sample = ex_ante.koh_std_in_sample;
        combined.koh_std_out_of_sample = ex_ante.koh_std_out_of_sample;
        combined.tian_pearl_pn_bounds = ex_ante.tian_pearl_pn_bounds;
        combined.tian_pearl_ps_bounds = ex_ante.tian_pearl_ps_bounds;
        combined.tian_pearl_pns_bounds = ex_ante.tian_pearl_pns_bounds;
        combined.theta_ps = ex_ante.theta_ps;
        combined.theta_po = ex_ante.theta_po;
        combined.risk_at_ps = ex_ante.risk_at_ps;
        combined.risk_at_po = ex_ante.risk_at_po;
        combined.performative_gap = ex_ante.performative_gap;
        combined.mackenzie_regime = ex_ante.mackenzie_regime;
        combined.goodhart_variant = ex_ante.goodhart_variant;
        combined.goodhart_metric_collapsed = ex_ante.goodhart_metric_collapsed;
        combined.corp_pav_finite_bias = ex_ante.corp_pav_finite_bias;
        combined.corp_reliability_debiased = ex_ante.corp_reliability_debiased;
        combined.icp_effective_n_per_env = ex_ante.icp_effective_n_per_env;
        combined.icp_status = ex_ante.icp_status;
        combined.ex_ante_calibration_score = ex_ante.ex_ante_calibration_score;
        combined.ex_ante_invariance_score = ex_ante.ex_ante_invariance_score;
        combined.ex_ante_discrepancy_score = ex_ante.ex_ante_discrepancy_score;
        combined.gate1_ex_ante_score = g1;
        combined.gate2_kairos_score = g2;
        combined.gate3_ex_post_score = g3;
        combined.nested_leontief_score = nested_min;
        combined.geometric_gate_score = geom_score;
        combined
    }

    /// Evaluates Beer's $S3^*$ Algedonic Channel and Sosa's [`TelicGrade`] for a single shift $t$ in `engine.rs`.
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate_shift_governance(
        frontline_e_factor: f64,
        trip_threshold: f64,
        testimonial_discount: f64,
        survival_slack_tts: f64,
        proxy_pressure: f64,
        constitutional_brake_active: bool,
        ex_ante_competence: f64,
        cleared_moral_stakes: bool,
        outcome_succeeded: bool,
    ) -> Self {
        let target_ln = trip_threshold.max(1.01).ln();
        let step_ln = frontline_e_factor.max(1.01).ln();
        let delta = testimonial_discount.clamp(0.0, 0.99);
        let ttr_just = (target_ln / step_ln).ceil();
        let ttr_disc = (target_ln / ((1.0 - delta) * step_ln)).ceil();
        let tts = survival_slack_tts.max(0.0);
        let fatal_delay = ttr_just <= tts && ttr_disc > tts;

        let adroit = ex_ante_competence >= 0.75 && cleared_moral_stakes;
        let grade = match (adroit, outcome_succeeded) {
            (true, true) => TelicGrade::FullyAptPerformance,
            (true, false) => TelicGrade::AdroitExAnteUnluckyOutcome,
            (false, true) => TelicGrade::LuckyAccurateOnly,
            (false, false) => TelicGrade::InaptAndInaccurate,
        };
        let brake_ind = if constitutional_brake_active {
            1.0
        } else {
            0.0
        };
        let db = proxy_pressure.clamp(0.0, 1.0) * delta * (1.0 - brake_ind);

        let delay_excess = (ttr_disc - ttr_just).max(0.0);
        let tts_denom = tts.max(ttr_just).max(1.0);
        let kairos_slack = (1.0 - delay_excess / tts_denom).clamp(0.0, 1.0);
        let tts_ratio = (tts / (tts + 0.35 * ttr_disc).max(1e-6)).clamp(0.0, 1.0);
        let succ_ind = if outcome_succeeded { 1.0 } else { 0.0 };
        let comp_clamped = ex_ante_competence.clamp(0.0, 1.0);
        let g1 =
            ((1.0 - proxy_pressure.clamp(0.0, 1.0)) * (0.72 + 0.28 * comp_clamped)).clamp(0.0, 1.0);
        let g2 = ((1.0 - delta) * (0.68 + 0.32 * tts_ratio) * (0.55 + 0.45 * kairos_slack))
            .clamp(0.0, 1.0);
        let g3 = if constitutional_brake_active {
            (0.78 + 0.14 * succ_ind + 0.08 * comp_clamped).clamp(0.0, 1.0)
        } else {
            ((0.32 - 0.22 * db) * (0.45 + 0.35 * succ_ind + 0.20 * tts_ratio)).clamp(0.0, 1.0)
        };
        let (nested_min, geom_score) = Self::canonical_score(g1, g2, g3);

        Self {
            proxy_incentive_pressure: proxy_pressure,
            algedonic_discount: delta,
            constitutional_brake_active,
            survival_slack_tts: tts,
            recovery_delay_ttr_undiscounted: ttr_just,
            recovery_delay_ttr_discounted: ttr_disc,
            fatal_testimonial_delay: fatal_delay,
            telic_grade: grade,
            double_bind_intensity: db,
            is_double_bind_trap: db > 0.20,
            gate1_ex_ante_score: g1,
            gate2_kairos_score: g2,
            gate3_ex_post_score: g3,
            nested_leontief_score: nested_min,
            geometric_gate_score: geom_score,
            ..Self::default()
        }
    }

    /// Evaluates a parameterized $3\times 3$ List–Pettit committee vote matrix (`M20 · CONT-01`),
    /// returning `(premise_majority_approves, constitutional_conjunctive_approves, discursive_dilemma_present)`.
    pub fn evaluate_list_pettit_votes(votes: &[[bool; 3]; 3]) -> (bool, bool, bool) {
        let mut p_counts = [0usize; 3];
        let mut conj_count = 0usize;
        for v in votes {
            for j in 0..3 {
                if v[j] {
                    p_counts[j] += 1;
                }
            }
            if v[0] && v[1] && v[2] {
                conj_count += 1;
            }
        }
        let premise_maj = p_counts.iter().all(|&c| (c as f64) / 3.0 > 0.5);
        let const_ok = (conj_count as f64) / 3.0 > 0.5;
        (premise_maj, const_ok, premise_maj && !const_ok)
    }
}

impl TemporalVault {
    /// Creates a new sealed `TemporalVault` committed to `payload_bytes` at an explicit ex-ante shift
    /// `seal_shift` and authenticated with a non-empty `hmac_key` (`M19 · SELO-01`).
    pub fn seal_at_shift_with_key(
        hmac_key: &[u8],
        seal_shift: usize,
        payload_bytes: &[u8],
    ) -> Result<Self, &'static str> {
        if hmac_key.is_empty() {
            return Err("TemporalVault violation: hmac_key must not be empty");
        }
        let mut vault = Self::seal_with_key(hmac_key, payload_bytes);
        vault.seal_shift = seal_shift;
        Ok(vault)
    }
}

/// One immutable record in an append-only cryptographic [`AuditLog`] (`M19 · SELO-01`, §10).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditLogEntry {
    /// Monotonic $0$-based sequence number.
    pub seq: usize,
    /// Shift or step index at which the entry was committed.
    pub shift_t: usize,
    /// 64-hex SHA-256 hash of the preceding [`AuditLogEntry`] (`"0".repeat(64)` for `seq == 0`).
    pub prev_hash: String,
    /// 64-hex SHA-256 digest of the entry payload.
    pub payload_sha256: String,
    /// 64-hex RFC 2104 HMAC-SHA256 tag over the chained canonical record.
    pub hmac_sha256: String,
    /// 64-hex SHA-256 chain hash committing `(seq, shift_t, prev_hash, payload_sha256, hmac_sha256)`.
    pub entry_hash: String,
}

/// Append-only cryptographic hash-chain ledger (`AuditLog`) for multi-step institutional and agentic audits (`M19 · SELO-01`, §10).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AuditLog {
    /// Ordered append-only chain of committed entries.
    pub entries: Vec<AuditLogEntry>,
}

impl AuditLog {
    /// Creates an empty append-only audit log.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Appends a new entry at `shift_t` authenticated with `hmac_key` (rejects empty keys or regressing shifts).
    pub fn append_with_key(
        &mut self,
        hmac_key: &[u8],
        shift_t: usize,
        payload_bytes: &[u8],
    ) -> Result<&AuditLogEntry, &'static str> {
        if hmac_key.is_empty() {
            return Err("AuditLog violation: hmac_key must not be empty");
        }
        if let Some(last) = self.entries.last()
            && shift_t < last.shift_t
        {
            return Err("AuditLog violation: shift_t must be non-decreasing");
        }
        let seq = self.entries.len();
        let prev_hash = self
            .entries
            .last()
            .map(|e| e.entry_hash.clone())
            .unwrap_or_else(|| "0".repeat(64));
        let payload_sha256 = sha256_hex(payload_bytes);
        let chain_material = format!("{seq}:{shift_t}:{prev_hash}:{payload_sha256}");
        let hmac_sha256 = hmac_sha256_hex(hmac_key, chain_material.as_bytes());
        let entry_material = format!("{chain_material}:{hmac_sha256}");
        let entry_hash = sha256_hex(entry_material.as_bytes());
        self.entries.push(AuditLogEntry {
            seq,
            shift_t,
            prev_hash,
            payload_sha256,
            hmac_sha256,
            entry_hash,
        });
        Ok(self.entries.last().expect("just pushed"))
    }

    /// Verifies the entire hash chain and HMAC signatures from genesis (`seq == 0`) to tip.
    pub fn verify_chain(&self, hmac_key: &[u8]) -> bool {
        if hmac_key.is_empty() {
            return false;
        }
        let mut expected_prev = "0".repeat(64);
        let mut prev_shift = 0usize;
        for (idx, e) in self.entries.iter().enumerate() {
            if e.seq != idx || e.prev_hash != expected_prev || (idx > 0 && e.shift_t < prev_shift) {
                return false;
            }
            let chain_material = format!(
                "{}:{}:{}:{}",
                e.seq, e.shift_t, e.prev_hash, e.payload_sha256
            );
            let expected_hmac = hmac_sha256_hex(hmac_key, chain_material.as_bytes());
            if e.hmac_sha256 != expected_hmac {
                return false;
            }
            let entry_material = format!("{chain_material}:{expected_hmac}");
            let expected_entry_hash = sha256_hex(entry_material.as_bytes());
            if e.entry_hash != expected_entry_hash {
                return false;
            }
            expected_prev = e.entry_hash.clone();
            prev_shift = e.shift_t;
        }
        true
    }
}

// ---------------------------------------------------------------------------------------------
// Latent world-model exploitation ("the agent that cheats inside the dream")
// ---------------------------------------------------------------------------------------------

/// Configuration of [`LatentModelExploitationAudit`].
///
/// The audit fits a linear surrogate $\hat{M}(z) = b_0 + b_1 z$ by ridge-free least squares on an
/// anchor grid confined to $[-s, s]$, then optimises a policy *inside* $\hat{M}$ and executes the
/// same actions against a world that carries one constraint the surrogate never observed: an
/// absorbing wall at $z_{\text{wall}}$.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LatentWorldModelConfig {
    /// Planning horizon inside the dream $H$.
    pub horizon_steps: usize,
    /// Radius $s$ of the anchor support $[-s, s]$ where the surrogate is fitted.
    pub support_radius: f64,
    /// Half-width of the symmetric action grid.
    pub action_half_width: f64,
    /// Number of actions in the grid (odd, at least three).
    pub action_count: usize,
    /// Absorbing boundary $z_{\text{wall}}$ of the true world (unobserved by the surrogate).
    pub unmodelled_wall: f64,
    /// Number of consecutive steps the world may spend beyond the boundary before it is absorbed.
    pub wall_patience: usize,
    /// Per-step harm charged while the world sits beyond the boundary.
    pub wall_penalty: f64,
    /// Lower end of the Boltzmann temperature sweep; the sweep is log-spaced up to `temperature_max`.
    pub temperature_min: f64,
    /// Upper end of the Boltzmann temperature sweep.
    pub temperature_max: f64,
    /// Deterministic PRNG seed for the temperature sweep rollouts.
    pub seed: u64,
    /// Initial latent state $z_0$.
    pub initial_state: f64,
}

impl LatentWorldModelConfig {
    /// Canonical audit configuration (seed `2026`, horizon `12`, support `1.0`, wall `1.0`,
    /// temperature sweep `0.05 … 4.0`).
    pub const fn canonical() -> Self {
        Self {
            horizon_steps: 12,
            support_radius: 1.0,
            action_half_width: 0.25,
            action_count: 9,
            unmodelled_wall: 1.0,
            wall_patience: 3,
            wall_penalty: 2.0,
            temperature_min: 0.05,
            temperature_max: 12.8,
            seed: 2026,
            initial_state: 0.0,
        }
    }

    /// Number of temperatures in the sweep.
    pub const SWEEP_LEN: usize = 5;
}

impl Default for LatentWorldModelConfig {
    fn default() -> Self {
        Self::canonical()
    }
}

/// Rolling record of one arm of the audit.
#[derive(Debug, Clone, Copy, PartialEq)]
struct LatentRollout {
    /// Return accumulated inside the surrogate $\hat{M}$.
    dream_return: f64,
    /// Return realised in the world, truncated at the absorbing wall.
    realized_return: f64,
    /// Step at which the world hit the absorbing wall (`None` if it survived the horizon).
    ruin_step: Option<usize>,
    /// Latent state the surrogate believes it reached at the end of the horizon.
    dream_final_state: f64,
    /// Fraction of dream steps spent outside the anchor support.
    out_of_support_fraction: f64,
}

/// Structural discrepancy $\delta(z) = |f(z) - \hat{f}(z)| / (1 + |f(z)| + |\hat{f}(z)|) \in [0, 1)$
/// between the world and the surrogate.
fn latent_discrepancy(world: f64, surrogate: f64) -> f64 {
    let gap = (world - surrogate).abs();
    let scale = 1.0 + world.abs() + surrogate.abs();
    (gap / scale).clamp(0.0, 1.0)
}

/// **Latent world-model exploitation audit** — the Stochological formalisation of the
/// "agent that cheats inside the dream".
///
/// A policy optimised only against a learned surrogate drifts, step by step, towards latent states
/// where the surrogate has no data and the world has constraints. The optimum of the surrogate is
/// therefore a **Goodhart extremal/adversarial** point: the dreamed return rises while the realised
/// return collapses and the world reaches an absorbing boundary the model never represented. The
/// Boltzmann temperature $\tau$ of the action policy is the regulariser: raising $\tau$ withdraws
/// the commitment to the single action the surrogate over-rewards, keeps the trajectory inside the
/// anchor support, and shrinks the structural discrepancy $\delta(z)$.
#[derive(Debug, Clone, PartialEq)]
pub struct LatentModelExploitationAudit {
    /// Planning horizon inside the dream $H$.
    pub horizon_steps: usize,
    /// Radius of the anchor support $s$.
    pub support_radius: f64,
    /// Fitted surrogate slope $b_1$.
    pub surrogate_slope: f64,
    /// Fitted surrogate intercept $b_0$.
    pub surrogate_intercept: f64,
    /// Absorbing wall $z_{\text{wall}}$ the surrogate never observed.
    pub unmodelled_wall: f64,
    /// Dreamed return of the unconstrained (greedy) policy.
    pub dream_return: f64,
    /// Realised return of the unconstrained (greedy) policy, truncated at the wall.
    pub realized_return_greedy: f64,
    /// Step at which the greedy policy destroyed the world (`None` if it survived).
    pub ruin_step_greedy: Option<usize>,
    /// Extrapolation depth $|z_H| / s$ reached inside the dream by the greedy policy.
    pub extrapolation_depth: f64,
    /// Fraction of greedy dream steps spent outside the anchor support.
    pub out_of_support_fraction: f64,
    /// Structural discrepancy $\delta(z_H)$ at the dreamed terminal state of the greedy policy.
    pub discrepancy_at_dream_end: f64,
    /// Goodhart variant exhibited by the greedy policy.
    pub goodhart_variant: GoodhartVariant,
    /// Boltzmann temperatures swept, from `temperature_min` to `temperature_max`.
    pub temperature_grid: [f64; LatentWorldModelConfig::SWEEP_LEN],
    /// Realised return per temperature.
    pub realized_return_by_tau: [f64; LatentWorldModelConfig::SWEEP_LEN],
    /// Ruin step per temperature.
    pub ruin_step_by_tau: [Option<usize>; LatentWorldModelConfig::SWEEP_LEN],
    /// Structural discrepancy at the dreamed terminal state, per temperature.
    pub discrepancy_by_tau: [f64; LatentWorldModelConfig::SWEEP_LEN],
}

impl LatentModelExploitationAudit {
    /// World transition $f(z) = 1.25 z - 0.35 z^{3}$: two stable branches and one unstable
    /// interior fixed point, none of which the linear surrogate represents.
    fn world_transition(z: f64) -> f64 {
        1.25 * z - 0.35 * z * z * z
    }

    /// Fits $\hat{M}(z) = b_0 + b_1 z$ on a uniform anchor grid over $[-s, s]$.
    fn fit_surrogate(config: &LatentWorldModelConfig) -> (f64, f64) {
        let anchors = config.action_count.max(3);
        let mut rows = Vec::with_capacity(anchors);
        let mut targets = Vec::with_capacity(anchors);
        for k in 0..anchors {
            let t = k as f64 / (anchors - 1) as f64;
            let z = -config.support_radius + 2.0 * config.support_radius * t;
            rows.push(vec![1.0, z]);
            targets.push(1.25 * z - 0.35 * z * z * z);
        }
        match ols_ridge_fit(&rows, &targets, 0.0) {
            Some(coeffs) if coeffs.len() == 2 => (coeffs[0], coeffs[1]),
            _ => (0.0, 0.0),
        }
    }

    /// Builds the symmetric action grid.
    fn action_grid(config: &LatentWorldModelConfig) -> Vec<f64> {
        let n = config.action_count.max(3);
        (0..n)
            .map(|k| {
                let t = k as f64 / (n - 1) as f64;
                -config.action_half_width + 2.0 * config.action_half_width * t
            })
            .collect()
    }

    /// Rolls one arm out. `temperature = None` is the greedy (fully committed) policy.
    fn roll_out(
        config: &LatentWorldModelConfig,
        slope: f64,
        intercept: f64,
        temperature: Option<f64>,
    ) -> LatentRollout {
        let actions = Self::action_grid(config);
        let mut rng = SplitMix64::new(config.seed);
        let mut dream_state = config.initial_state;
        let mut world_state = config.initial_state;
        let mut dream_return = 0.0;
        let mut realized_return = 0.0;
        let mut ruin_step = None;
        let mut outside = 0usize;
        let mut violation_streak = 0usize;

        for step in 1..=config.horizon_steps {
            let dreamed_next: Vec<f64> = actions
                .iter()
                .map(|&u| intercept + slope * dream_state + u)
                .collect();
            let action = match temperature {
                None => {
                    let mut best = 0usize;
                    for (i, &q) in dreamed_next.iter().enumerate() {
                        if q > dreamed_next[best] {
                            best = i;
                        }
                    }
                    actions[best]
                }
                Some(tau) => {
                    let tau = tau.max(1e-9);
                    let peak = dreamed_next
                        .iter()
                        .copied()
                        .fold(f64::NEG_INFINITY, f64::max);
                    let weights: Vec<f64> = dreamed_next
                        .iter()
                        .map(|&q| ((q - peak) / tau).exp())
                        .collect();
                    let total: f64 = weights.iter().sum();
                    let mut draw = rng.next_f64() * total;
                    let mut chosen = actions[actions.len() - 1];
                    for (i, &w) in weights.iter().enumerate() {
                        if draw < w {
                            chosen = actions[i];
                            break;
                        }
                        draw -= w;
                    }
                    chosen
                }
            };

            let dreamed_value = intercept + slope * dream_state + action;
            dream_return += dreamed_value;
            if dream_state.abs() > config.support_radius {
                outside += 1;
            }
            dream_state = dreamed_value;

            if ruin_step.is_none() {
                let realised_value = Self::world_transition(world_state) + action;
                realized_return += realised_value;
                world_state = realised_value;
                if world_state > config.unmodelled_wall {
                    violation_streak += 1;
                    realized_return -= config.wall_penalty * (world_state - config.unmodelled_wall);
                } else {
                    violation_streak = 0;
                }
                if violation_streak >= config.wall_patience.max(1) {
                    ruin_step = Some(step);
                }
            }
        }

        LatentRollout {
            dream_return,
            realized_return,
            ruin_step,
            dream_final_state: dream_state,
            out_of_support_fraction: outside as f64 / config.horizon_steps.max(1) as f64,
        }
    }

    /// Evaluates the audit: greedy arm, temperature sweep, Goodhart classification.
    pub fn evaluate(config: LatentWorldModelConfig) -> Self {
        let (intercept, slope) = Self::fit_surrogate(&config);
        let greedy = Self::roll_out(&config, slope, intercept, None);
        let dreamed_world = Self::world_transition(greedy.dream_final_state);
        let dreamed_surrogate = intercept + slope * greedy.dream_final_state;
        let discrepancy_at_dream_end = latent_discrepancy(dreamed_world, dreamed_surrogate);
        let extrapolation_depth = greedy.dream_final_state.abs() / config.support_radius.max(1e-12);

        let mut temperature_grid = [0.0_f64; LatentWorldModelConfig::SWEEP_LEN];
        let mut realized_return_by_tau = [0.0_f64; LatentWorldModelConfig::SWEEP_LEN];
        let mut ruin_step_by_tau = [None; LatentWorldModelConfig::SWEEP_LEN];
        let mut discrepancy_by_tau = [0.0_f64; LatentWorldModelConfig::SWEEP_LEN];
        for (i, slot) in temperature_grid.iter_mut().enumerate() {
            let t = i as f64 / (LatentWorldModelConfig::SWEEP_LEN - 1) as f64;
            let tau =
                config.temperature_min * (config.temperature_max / config.temperature_min).powf(t);
            *slot = tau;
            let arm = Self::roll_out(&config, slope, intercept, Some(tau));
            let world_end = Self::world_transition(arm.dream_final_state);
            let surrogate_end = intercept + slope * arm.dream_final_state;
            realized_return_by_tau[i] = arm.realized_return;
            ruin_step_by_tau[i] = arm.ruin_step;
            discrepancy_by_tau[i] = latent_discrepancy(world_end, surrogate_end);
        }

        Self {
            horizon_steps: config.horizon_steps,
            support_radius: config.support_radius,
            surrogate_slope: slope,
            surrogate_intercept: intercept,
            unmodelled_wall: config.unmodelled_wall,
            dream_return: greedy.dream_return,
            realized_return_greedy: greedy.realized_return,
            ruin_step_greedy: greedy.ruin_step,
            extrapolation_depth,
            out_of_support_fraction: greedy.out_of_support_fraction,
            discrepancy_at_dream_end,
            goodhart_variant: GoodhartVariant::classify(true, false, extrapolation_depth),
            temperature_grid,
            realized_return_by_tau,
            ruin_step_by_tau,
            discrepancy_by_tau,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_operator_iii_verifies_three_temporal_gates() {
        let forecasts = [0.1, 0.2, 0.8, 0.9];
        let outcomes = [0.0, 0.0, 1.0, 1.0];
        let envs = vec![
            CausalEnvironment {
                name: "E1".to_string(),
                x: vec![vec![1.0, 2.0], vec![2.0, 3.0], vec![3.0, 5.0]],
                y: vec![2.0, 4.0, 6.0],
            },
            CausalEnvironment {
                name: "E2".to_string(),
                x: vec![vec![1.0, 9.0], vec![2.0, 1.0], vec![3.0, 4.0]],
                y: vec![2.0, 4.0, 6.0],
            },
        ];
        let g1 = GovernanceState::from_ex_ante_gate(&forecasts, &outcomes, &envs);
        assert!(g1.corp_closure_error < 1e-12);
        assert!(g1.icp_invariant_slope_range < g1.icp_spurious_slope_range);
        assert_eq!(
            g1.icp_status,
            IcpIdentificationStatus::EpistemicUnderpoweredSet
        );
        assert_eq!(
            GovernanceState::classify_icp_power(500.0, 0.009, 0.076),
            IcpIdentificationStatus::IdentifiedInvariantSet
        );
        assert!(
            g1.corp_pav_finite_bias >= 0.0 && g1.corp_reliability_debiased <= g1.corp_reliability
        );
        assert_eq!(
            GovernanceState::canonical_score(f64::NAN, 0.9, 0.9),
            (0.0, 0.0)
        );

        // §7 Criterion 3 & B6: S_gov(A) != S_gov(B) for distinct datasets A and B by a large margin (> 0.30)
        let forecasts_b = [0.95, 0.85, 0.15, 0.05];
        let outcomes_b = [0.0, 1.0, 0.0, 1.0];
        let g1_b = GovernanceState::from_ex_ante_gate(&forecasts_b, &outcomes_b, &envs);
        assert!(
            (g1.nested_leontief_score - g1_b.nested_leontief_score).abs() > 0.30,
            "S_gov(A)={} must differ substantially from S_gov(B)={}",
            g1.nested_leontief_score,
            g1_b.nested_leontief_score
        );

        // M7 & M8: CounterperformativeDestructive is reachable, and ICP rejects d > 12 without silent truncation
        let (_, _, r_ps_dest, r_po_dest, _, reg_dest) =
            GovernanceState::evaluate_performative_system(1.0, 0.95, 0.20, 0.10);
        assert!(r_ps_dest > 2.0 * r_po_dest);
        assert_eq!(reg_dest, MacKenzieRegime::CounterperformativeDestructive);
        let wide_env = vec![
            CausalEnvironment {
                name: "W1".to_string(),
                x: vec![vec![0.0; 13]; 4],
                y: vec![1.0; 4],
            },
            CausalEnvironment {
                name: "W2".to_string(),
                x: vec![vec![0.0; 13]; 4],
                y: vec![1.0; 4],
            },
        ];
        assert!(GovernanceState::try_evaluate_icp_subset_intersection(&wide_env, 0.0).is_err());
        assert!(GovernanceState::icp_f_critical_value(2.0, 100.0, 0.05) > 2.5);

        // B7: Non-binary outcomes are rejected by try_evaluate_corp_pav and produce positive closure_error
        assert!(
            GovernanceState::try_evaluate_corp_pav(&[0.2, 0.5, 0.8], &[0.3, 0.25, 0.8]).is_err()
        );
        let (_, _, _, _, non_bin_err) =
            GovernanceState::evaluate_corp_pav(&[0.2, 0.5, 0.8], &[0.3, 0.25, 0.8]);
        assert!(non_bin_err > 0.05);

        // A9: Two-shift TemporalVault lifecycle rejects same-shift unseal and second opening
        assert!(TemporalVault::verify_two_shift_lifecycle(1, 1, b"payload", b"payload").is_err());
        let mut vault_ok =
            TemporalVault::seal_with_key(b"custom-hmac-key-2026", b"telic-commitment-payload");
        assert!(
            vault_ok
                .unseal_with_key(b"custom-hmac-key-2026", b"telic-commitment-payload")
                .is_ok()
        );
        assert!(
            vault_ok
                .unseal_with_key(b"custom-hmac-key-2026", b"telic-commitment-payload")
                .is_err()
        );
        assert!(!vault_ok.is_confirmatory);
        let mut vault_bad =
            TemporalVault::seal_with_key(b"custom-hmac-key-2026", b"telic-commitment-payload");
        assert!(
            vault_bad
                .unseal_with_key(b"wrong-hmac-key", b"telic-commitment-payload")
                .is_err()
        );
        assert!(g1.koh_std_out_of_sample > 2.5 * g1.koh_std_in_sample);
        assert!(g1.risk_at_po < g1.risk_at_ps);
        assert!(g1.goodhart_metric_collapsed);

        let g23 = GovernanceState::from_real_time_and_ex_post_gates(
            0.393,
            0.112,
            1.85,
            20.0,
            0.55,
            8.0,
            b"stochology-stage1-telic-commitment",
        )
        .expect("vault unseal");
        assert!((g23.chouldechova_fpr_disparity - 5.13).abs() < 0.05);
        assert!(g23.discursive_dilemma_present);
        assert!(g23.fatal_testimonial_delay);
        assert!(g23.structural_liability_share > 0.80);
        assert!(g23.is_adulterated_air_scapegoating);
        assert!(g23.is_double_bind_trap);
        assert!(g23.vault_confirmatory);
        let g_all = GovernanceState::from_all_three_gates(&g1, &g23);
        assert!(
            (g_all.nested_leontief_score
                - g1.gate1_ex_ante_score
                    .min(g23.gate2_kairos_score)
                    .min(g23.gate3_ex_post_score))
            .abs()
                < 1e-12
        );

        let panel = KairosSynchronousPanel::new(0.879, 1.000, 0.900, 45.0, 18.5, true);
        assert!((panel.m_leontief - 0.879).abs() < 1e-12);
        let regimes = ZhengMeisterQueueAudit::evaluate_three_canonical_regimes();
        assert!(regimes[0].survives_kairos);
        assert!((regimes[0].total_recovery_ttr_s - 18.51).abs() < 0.05);
        assert!(regimes[1].survives_kairos);
        assert!((regimes[1].total_recovery_ttr_s - 40.85).abs() < 0.05);
        assert!(!regimes[2].survives_kairos);
        assert!((regimes[2].total_recovery_ttr_s - 759.0).abs() < 0.5);

        let emp_ch = EmpiricalChouldechovaAudit::evaluate(
            &[(true, true), (true, false), (false, true), (false, false)],
            &[
                (true, true),
                (false, true),
                (false, false),
                (false, false),
                (false, false),
            ],
        );
        assert!(emp_ch.identity_closure_error < 1e-12);
        assert!(emp_ch.empirical_fpr_ratio > 1.5);
        // M3: Empty group must not pass closure error check
        let emp_empty = EmpiricalChouldechovaAudit::evaluate(&[], &[(true, true), (false, false)]);
        assert!(emp_empty.identity_closure_error >= 1.0);

        // M19 & M20: AuditLog chain verification, seal_at_shift_with_key, and parameterized List-Pettit votes
        assert!(TemporalVault::seal_at_shift_with_key(b"", 3, b"payload").is_err());
        let shifted_vault =
            TemporalVault::seal_at_shift_with_key(b"key-2026", 4, b"payload").expect("valid key");
        assert_eq!(shifted_vault.seal_shift, 4);

        let mut log = AuditLog::new();
        assert!(log.append_with_key(b"", 0, b"step-0").is_err());
        log.append_with_key(b"audit-key", 1, b"step-1")
            .expect("entry 0");
        log.append_with_key(b"audit-key", 2, b"step-2")
            .expect("entry 1");
        assert!(log.append_with_key(b"audit-key", 1, b"regress").is_err());
        assert!(log.verify_chain(b"audit-key"));
        assert!(!log.verify_chain(b"wrong-key"));
        assert!(!log.verify_chain(b""));
        let mut tampered_log = log.clone();
        tampered_log.entries[0].payload_sha256 = "f".repeat(64);
        assert!(!tampered_log.verify_chain(b"audit-key"));

        let unanimous = [[true, true, true]; 3];
        assert_eq!(
            GovernanceState::evaluate_list_pettit_votes(&unanimous),
            (true, true, false)
        );
        let rejected = [[false, false, false]; 3];
        assert_eq!(
            GovernanceState::evaluate_list_pettit_votes(&rejected),
            (false, false, false)
        );
    }
}

#[cfg(test)]
mod crps_decomposition_tests {
    use super::*;

    fn grid() -> Vec<f64> {
        (0..=200).map(|i| i as f64 / 200.0).collect()
    }

    #[test]
    fn crps_decomposition_closes_and_tracks_the_point_forecast_error() {
        let forecasts = [0.10, 0.35, 0.60, 0.90];
        let outcomes = [0.00, 1.00, 0.00, 1.00];
        let out = GovernanceState::corp_pav_crps_decomposition(&forecasts, &outcomes, &grid())
            .expect("valid decomposition");
        assert!(
            out.closure_error < 1e-12,
            "CRPS identity must close, got {}",
            out.closure_error
        );
        assert!(
            out.grid_discretization_error < 2e-3,
            "grid integral must approximate the mean absolute error, gap {}",
            out.grid_discretization_error
        );
        assert_eq!(out.threshold_grid_len, 201);
        let mae = forecasts
            .iter()
            .zip(outcomes.iter())
            .map(|(f, y)| (f - y).abs())
            .sum::<f64>()
            / forecasts.len() as f64;
        assert!((out.crps - mae).abs() < 2e-3);
    }

    #[test]
    fn a_constant_forecast_has_no_resolution() {
        let forecasts = [0.42; 6];
        let outcomes = [0.0, 1.0, 0.0, 1.0, 0.0, 1.0];
        let out = GovernanceState::corp_pav_crps_decomposition(&forecasts, &outcomes, &grid())
            .expect("valid decomposition");
        assert!(
            out.resolution < 1e-12,
            "an uninformative constant forecast resolves nothing, got {}",
            out.resolution
        );
        assert!(out.reliability >= -1e-12);
    }

    #[test]
    fn crps_decomposition_rejects_degenerate_inputs() {
        let g = grid();
        assert!(GovernanceState::corp_pav_crps_decomposition(&[], &[], &g).is_err());
        assert!(GovernanceState::corp_pav_crps_decomposition(&[0.5], &[1.0], &g).is_err());
        assert!(GovernanceState::corp_pav_crps_decomposition(&[0.2, 0.8], &[0.0], &g).is_err());
        assert!(
            GovernanceState::corp_pav_crps_decomposition(&[0.2, 0.8], &[0.0, 1.0], &[0.5]).is_err()
        );
        assert!(
            GovernanceState::corp_pav_crps_decomposition(&[0.2, 0.8], &[0.0, 1.0], &[0.5, 0.5])
                .is_err()
        );
        assert!(
            GovernanceState::corp_pav_crps_decomposition(&[0.2, f64::NAN], &[0.0, 1.0], &g)
                .is_err()
        );
    }
}
