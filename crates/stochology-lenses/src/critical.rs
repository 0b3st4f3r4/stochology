//! Critical Theory of Stochastic Rationality, Algorithmic Governmentality, and the Material Collapse &
//! Neurosymbolic/Wave Overcoming of AI (§1.3.9):
//!
//! Formalizes in pure Rust (`seed = 2026`, zero external dependencies) the philosophical and mathematical
//! synthesis between the Frankfurt School's critique of instrumental/statistical reason, contemporary
//! philosophy of technology (Algorithmic Governmentality & "Stochastic Parrots"), and the physical,
//! informational, and thermodynamic limits of binary digital AI:
//!
//! 1. **Axis 1 — Genealogy of Stochastic Reason ($\sigma\tau\acute{o}\chi\omicron\varsigma \to \sigma\tau\omicron\chi\alpha\sigma\tau\iota\kappa\acute{o}\varsigma$),
//!    Adorno & Horkheimer's Liquidation of the "Non-Identical" (*Nichtidentische*), and Eternalization of the Historical Present:**
//!    Quantifies how positivist Gaussian identity reduction (discarding singular heavy-tailed events beyond $\pm 2\sigma$ as
//!    "standard deviation / noise") liquidates the *Non-Identical* ($\Delta_{\text{Nichtidentische}} \gg 0$ and the erased share of
//!    $\mathrm{ES}_{0.99}$), backtests the resulting $\mathrm{VaR}_{0.99}$ on fresh draws of the same process, and contrasts the
//!    historical status quo ($\theta_{\mathrm{PS}}$, reached by Repeated Risk Minimization) with counterfactual Stackelberg
//!    $\theta_{\mathrm{PO}}$ (reached by `PerfGD`), plus the **Anchored Kennedy-O'Hagan Structural Discrepancy** ($\delta(x) \sim \mathcal{GP}$).
//! 2. **Axis 2 — Herbert Marcuse's Operational Closure of Language (*One-Dimensional Man*), Antoinette Rouvroy & Thomas Berns's
//!    Algorithmic Governmentality (Preemption), and the "Stochastic Parrots" Paradigm ($P(w_t \mid w_{1:t-1})$):**
//!    Demonstrates that an ungrounded autoregressive next-token predictor achieves deceptive surface fluency on stationary
//!    correlations (Pearl Rung 1 `1-See`) while suffering catastrophic counterfactual breakdown under causal/normative interventions
//!    ($\mathrm{do}(X)$, Pearl Rung 2/3), and evaluates the closed-form **Chouldechova false-positive disparity**
//!    ($\mathrm{FPR}_A / \mathrm{FPR}_B$) of preemptive correlational profiling under unequal base rates.
//! 3. **Axis 3 — The Material & Thermodynamic Collapse of Instrumental Stochastic Reason:** simulated synthetic-recursion
//!    variance and KL collapse (`crate::analog`), with Epoch AI's token ceiling, Horowitz's 45nm energy hierarchy, the H100 and IEA
//!    figures, and Ferran Alia's noise-threshold curve carried as informative literature values.
//! 4. **Axis 4 — Dialectical Overcoming:** symbolic-constraint satisfaction of a constrained vs. unconstrained synthetic recursion,
//!    A-ECC drift compensation vs. raw drift, Peking University 24-bit `HP-INV` refinement vs. one-shot inversion, and a continuous
//!    phase-amplitude (`SIREN`) Helmholtz Laplacian vs. piecewise-linear `ReLU`; `NAT` accuracies (an assumed noise-reduction law in
//!    `crate::analog`), the wave compression ratio, and Tsinghua `ACCEL` efficiency are informative.
//!
//! # Verification semantics
//!
//! As in `crate::triad`, each axis separates **claim checks** (the deployed method against computed ground truth; with
//! `grounded_mode = false` the naive method is actually run), **identity checks** (`identities_closed`, closed-form or
//! algebraic facts that hold in both modes), **informative constants** (literature values, never gated), and
//! **discarded claims** (`discarded_claims`, claims that failed multi-seed acceptance in `crate::acceptance`; recorded, never approved).

use crate::acceptance::{MultiSeedAcceptance, NamedCheck};
use crate::analog::{
    AimcAnalogWallHpInvAudit, AnalogWaveComputingSynthesis, EmpiricalWaveAnalogAudit,
};
use crate::invariance::{run_nonlinear_icp_nile, CausalEnvironment};
use crate::machine::AuthenticModelForm;
use crate::tail::{expected_shortfall_99, HillTailDiagnostic};
use stochology_core::{
    mean, quantile, sample_std, sample_variance, ChouldechovaAudit, GoodhartTaxonomyAudit,
    KennedyOHaganDiscrepancy, NonlinearSummary, PerformativeSystem, Rng,
};

/// Heavy-tailed draw of the Axis 1 generating process: Pareto ($\alpha = 1.65$) plus a structural singularity every 40th index.
fn non_identical_draw(rng: &mut Rng, index: usize) -> f64 {
    let u = rng.next_f64().clamp(1e-6, 0.999_999);
    let pareto = (1.0 - u).powf(-1.0 / 1.65);
    let singularity = if index.is_multiple_of(40) { 14.0 } else { 0.0 };
    pareto + singularity
}

/// Axis 1: Adorno & Horkheimer's Critique of the "Mathematization of Thought", Liquidation of the
/// "Non-Identical" (*Nichtidentische*, *Negative Dialectics* 1966), and Eternalization of the Historical Present.
#[derive(Debug, Clone, PartialEq)]
pub struct AdornoNonIdenticalReificationAudit {
    /// True heavy-tailed nonlinear expected loss $\mathbb{E}[f(X)]$ preserving singular tail events.
    pub true_nonlinear_expectation: f64,
    /// Naive evaluation at the lone mean $f(\mathbb{E}[X])$ (violating Prohibition 2).
    pub lone_mean_evaluation: f64,
    /// Positivist Gaussian-identity expected loss $\mathbb{E}_{\mathcal{N}_{\text{trunc}}}[f(X)]$ after amputating $> 2\sigma$ singularities as "noise".
    pub positivist_gaussian_truncated_expectation: f64,
    /// Adorno's *Non-Identical Liquidation Gap* $\Delta_{\text{Nichtidentische}} = \mathbb{E}[f(X)] - \mathbb{E}_{\mathcal{N}_{\text{trunc}}}[f(X)]$.
    pub non_identical_liquidation_gap: f64,
    /// True Expected Shortfall $\mathrm{ES}_{0.99}$ preserving the non-identical qualitative tail.
    pub true_es99: f64,
    /// Positivist truncated Expected Shortfall $\mathrm{ES}_{0.99}^{\text{trunc}}$ after outlier liquidation.
    pub positivist_truncated_es99: f64,
    /// Fraction of tail catastrophe risk erased by forced Gaussian identity ($1 - \mathrm{ES}_{0.99}^{\text{trunc}} / \mathrm{ES}_{0.99}$).
    pub es99_erasure_fraction: f64,
    /// Exceedance rate of $\mathrm{VaR}_{0.99}$ on 6000 fresh draws of the same process: empirical quantile vs. truncated Gaussian.
    pub var99_holdout_exceedance_empirical_vs_gaussian: (f64, f64),
    /// Holdout exceedance rate of the deployed $\mathrm{VaR}_{0.99}$ (empirical if grounded, truncated Gaussian otherwise). Gated (`<= 0.02`).
    pub deployed_var99_holdout_exceedance: f64,
    /// Improvement of the anchored Kennedy-O'Hagan GP over the bare simulator in extrapolation error, $\|\cdot\|_{\text{w/o GP}} - \|\cdot\|_{\text{GP}}$.
    pub koh_structural_discrepancy_norm: f64,
    /// Extrapolation error of the deployed model (simulator + GP discrepancy if grounded, bare simulator otherwise). Gated.
    pub deployed_koh_extrapolation_error: f64,
    /// Performative Risk $\mathrm{PR}(\theta_{\mathrm{PS}})$ at the status-quo fixed point (eternalizing the historical present).
    pub passive_status_quo_performative_risk: f64,
    /// Performative Risk $\mathrm{PR}(\theta_{\mathrm{PO}})$ under counterfactual Stackelberg agency ($L_3 \to L_1$).
    pub counterfactual_stackelberg_performative_risk: f64,
    /// Excess performative risk caused by reifying the historical present ($\mathrm{PR}(\theta_{\mathrm{PS}}) - \mathrm{PR}(\theta_{\mathrm{PO}})$).
    pub status_quo_reification_excess_risk: f64,
    /// $\mathrm{PR}(\theta_{\text{final}}) - \mathrm{PR}(\theta_{\mathrm{PO}})$ of the deployed optimizer (`PerfGD` if grounded, RRM otherwise). Gated.
    pub deployed_performative_excess_risk: f64,
    /// Named claim checks gated by this axis (see `crate::acceptance`).
    pub claims: Vec<NamedCheck>,
    /// Claims removed from the gate after failing multi-seed acceptance (`crate::acceptance`); still computed and reported.
    pub discarded_claims: Vec<NamedCheck>,
    /// True if the optimality identity $\mathrm{PR}(\theta_{\mathrm{PO}}) \le \mathrm{PR}(\theta_{\mathrm{PS}})$ closes.
    pub identities_closed: bool,
    /// Whether the mode-independent liquidation claims and the deployed tail, extrapolation, and performative claims hold.
    pub axis1_verified: bool,
}

impl AdornoNonIdenticalReificationAudit {
    /// Evaluates Axis 1 (`seed = 2026`). When `preserve_non_identical` is `false`, the positivist pipeline is deployed:
    /// truncated-Gaussian $\mathrm{VaR}$, the bare simulator without structural discrepancy, and Repeated Risk Minimization.
    pub fn evaluate(seed: u64, preserve_non_identical: bool) -> Self {
        let mut rng = Rng::new(seed);
        let n = 600usize;
        let raw_samples: Vec<f64> = (0..n).map(|i| non_identical_draw(&mut rng, i)).collect();

        let split = (n * 4) / 5;
        let full_summary = NonlinearSummary::evaluate_out_of_sample(
            &raw_samples[..split],
            &raw_samples[split..],
            |x| x * x,
            2.0,
        );
        let true_nonlinear_expectation = full_summary.expected_f;
        let lone_mean_evaluation = full_summary.f_at_mean;
        let true_es99 = expected_shortfall_99(&raw_samples);

        // Positivist forced-identity Gaussian reduction: amputate observations > mu + 2*sigma as "deviant noise"
        let mu_raw = mean(&raw_samples);
        let sd_raw = sample_std(&raw_samples);
        let cutoff = mu_raw + 2.0 * sd_raw;
        let truncated: Vec<f64> = raw_samples
            .iter()
            .copied()
            .filter(|&x| x <= cutoff)
            .collect();
        let mu_trunc = mean(&truncated);
        let var_trunc = sample_variance(&truncated);
        // For f(x) = x^2 under N(mu_trunc, var_trunc), E[X^2] = mu_trunc^2 + var_trunc
        let positivist_gaussian_truncated_expectation = mu_trunc * mu_trunc + var_trunc;
        let positivist_truncated_es99 = expected_shortfall_99(&truncated);
        let non_identical_liquidation_gap =
            true_nonlinear_expectation - positivist_gaussian_truncated_expectation;
        let es99_erasure_fraction = 1.0 - (positivist_truncated_es99 / true_es99.max(1e-12));

        // VaR_99 backtest on 6000 fresh draws of the same generating process
        let var99_empirical = quantile(&raw_samples, 0.99);
        let var99_gaussian = mu_trunc + 2.326_348 * var_trunc.sqrt();
        let holdout: Vec<f64> = (n..n + 6000)
            .map(|i| non_identical_draw(&mut rng, i))
            .collect();
        let exceed = |v: f64| holdout.iter().filter(|&&x| x > v).count() as f64 / 6000.0;
        let exceed_empirical = exceed(var99_empirical);
        let exceed_gaussian = exceed(var99_gaussian);
        let deployed_exceedance = if preserve_non_identical {
            exceed_empirical
        } else {
            exceed_gaussian
        };

        // Kennedy-O'Hagan GP structural discrepancy \delta(x): captures the non-identical qualitative structure
        // that the linear/positivist parametric concept \eta(x, \theta) cannot subsume.
        let x_obs = [0.1, 0.2, 0.4, 0.8, 1.2, 1.6, 2.0, 2.4];
        let y_obs: Vec<f64> = x_obs.iter().map(|&x| 0.80 * x - 0.35 * x * x).collect();
        let x_ext = [2.6, 2.8, 3.0];
        let y_ext: Vec<f64> = x_ext.iter().map(|&x| 0.80 * x - 0.35 * x * x).collect();
        let koh = KennedyOHaganDiscrepancy::calibrate(
            &x_obs, &y_obs, 0.80, 2.4, 0.85, 0.005, &x_ext, &y_ext,
        );
        let koh_structural_discrepancy_norm =
            koh.extrapolation_error_without_gp - koh.extrapolation_error_with_gp;
        let deployed_koh_error = if preserve_non_identical {
            koh.extrapolation_error_with_gp
        } else {
            koh.extrapolation_error_without_gp
        };

        // Eternalization of the historical present: RRM reaches the status-quo fixed point \theta_PS, PerfGD reaches \theta_PO
        let perf_sys = PerformativeSystem::new(1.0, 0.30, 0.80, 1.0, 1.4);
        let perf = perf_sys.diagnose();
        let passive_status_quo_performative_risk = perf.risk_at_ps;
        let counterfactual_stackelberg_performative_risk = perf.risk_at_po;
        let status_quo_reification_excess_risk =
            passive_status_quo_performative_risk - counterfactual_stackelberg_performative_risk;
        let deployed_theta = if preserve_non_identical {
            *perf_sys
                .perfgd_finite_difference_trajectory(0.0, 1.0, 0.5, 60)
                .last()
                .unwrap_or(&0.0)
        } else {
            *perf.rrm_trajectory.last().unwrap_or(&0.0)
        };
        let deployed_performative_excess_risk =
            perf_sys.performative_risk_pr(deployed_theta) - perf.risk_at_po;

        // The closed-form Stackelberg optimum cannot carry more performative risk than the stable point.
        let identities_closed =
            counterfactual_stackelberg_performative_risk <= passive_status_quo_performative_risk;
        // The liquidation gap exceeds 30 on 7 of 20 acceptance seeds (its sign is robust, its size is not).
        let discarded_claims: Vec<NamedCheck> = vec![(
            "critical.a1.liquidation_gap_above_30",
            non_identical_liquidation_gap > 30.0,
        )];
        let claims: Vec<NamedCheck> = vec![
            (
                "critical.a1.liquidation_gap_positive",
                non_identical_liquidation_gap > 0.0,
            ),
            (
                "critical.a1.es99_erasure_above_0.60",
                es99_erasure_fraction > 0.60,
            ),
            (
                "critical.a1.koh_improvement_above_0.40",
                koh_structural_discrepancy_norm > 0.40,
            ),
            (
                "critical.a1.reification_excess_above_0.05",
                status_quo_reification_excess_risk > 0.05,
            ),
            (
                "critical.a1.deployed_var99_breach_at_most_2pct",
                deployed_exceedance <= 0.02,
            ),
            (
                "critical.a1.deployed_koh_error_below_half_bare",
                deployed_koh_error < 0.5 * koh.extrapolation_error_without_gp.max(1e-12),
            ),
            (
                "critical.a1.deployed_performative_excess_below_1e-3",
                deployed_performative_excess_risk < 1e-3,
            ),
        ];
        let axis1_verified = identities_closed && claims.iter().all(|c| c.1);

        Self {
            true_nonlinear_expectation,
            lone_mean_evaluation,
            positivist_gaussian_truncated_expectation,
            non_identical_liquidation_gap,
            true_es99,
            positivist_truncated_es99,
            es99_erasure_fraction,
            var99_holdout_exceedance_empirical_vs_gaussian: (exceed_empirical, exceed_gaussian),
            deployed_var99_holdout_exceedance: deployed_exceedance,
            koh_structural_discrepancy_norm,
            deployed_koh_extrapolation_error: deployed_koh_error,
            passive_status_quo_performative_risk,
            counterfactual_stackelberg_performative_risk,
            status_quo_reification_excess_risk,
            deployed_performative_excess_risk,
            claims,
            discarded_claims,
            identities_closed,
            axis1_verified,
        }
    }
}

/// Axis 2: Herbert Marcuse's Operational Closure of Language (*One-Dimensional Man*, 1964),
/// Antoinette Rouvroy & Thomas Berns's Algorithmic Governmentality (Preemption), and the
/// "Stochastic Parrots" Paradigm ($P(w_t \mid w_1, \dots, w_{t-1})$ without causal-ontological grounding).
#[derive(Debug, Clone, PartialEq)]
pub struct MarcuseRouvroyStochasticParrotAudit {
    /// Surface next-token MSE on passive historical correlations (Pearl Rung 1 `1-See`, stationary regime $e_0$).
    pub parrot_stationary_surface_mse: f64,
    /// Counterfactual MSE of the ungrounded "Stochastic Parrot" under a causal/normative intervention ($\mathrm{do}(X)$, Pearl Rung 2/3).
    pub parrot_counterfactual_intervention_mse: f64,
    /// Counterfactual MSE of the Causal-Normative Grounded agent anchored to $\hat{S}^* = [0]$ via ICP/NILE.
    pub causal_grounded_counterfactual_mse: f64,
    /// Counterfactual MSE of the deployed predictor (causal if grounded, parrot otherwise). Gated (`< 0.03`).
    pub deployed_counterfactual_mse: f64,
    /// Ratio of NILE/IRM invariance penalty on the spurious operational proxy ($X_1$) vs. the invariant causal core ($X_0$).
    pub spurious_proxy_invariance_penalty_ratio: f64,
    /// Identity: closed-form Chouldechova False-Positive Rate disparity ($\mathrm{FPR}_A / \mathrm{FPR}_B$) at base rates $(0.32, 0.09)$
    /// with equal $\mathrm{PPV} = 0.65$ and $\mathrm{FNR} = 0.25$.
    pub preemptive_profiling_fpr_disparity_ratio: f64,
    /// Informative: whether the deployed Authentic Model Form blocks ungrounded preemption (the naive pipeline does not name its
    /// discarded atoms, so it fails by construction).
    pub authentic_form_blocks_spurious_preemption: bool,
    /// Whether the Authentic Model Form, run with named discarded atoms, approves $\hat{S}^* = [0]$ on these environments
    /// (mode-independent). Gated.
    pub authentic_form_audit_approves_causal_core: bool,
    /// Formal Manheim-Garrabrant (2018) 4-variant Goodhart taxonomy audit linking Causal collapse to ICP/NILE ($S^*$) and Adversarial gaming to $\nabla_1 \mathrm{DPR}(\theta,\theta)$ in `PerfGD`.
    pub goodhart_taxonomy: GoodhartTaxonomyAudit,
    /// Named claim checks gated by this axis (see `crate::acceptance`).
    pub claims: Vec<NamedCheck>,
    /// True if the Chouldechova identity closes.
    pub identities_closed: bool,
    /// Whether the mode-independent parrot/invariance claims and the deployed predictor and authentic form pass.
    pub axis2_verified: bool,
}

impl MarcuseRouvroyStochasticParrotAudit {
    /// Evaluates Axis 2 (`seed = 2026`). When `causal_normative_grounding` is `false`, the parrot predictor is deployed
    /// and the Authentic Model Form omits discarded-atom accounting.
    pub fn evaluate(seed: u64, causal_normative_grounding: bool) -> Self {
        let mut rng = Rng::new(seed.wrapping_add(17));
        // Build 3 environments across historical/interventional regimes:
        // X_0 = true causal/ontological driver (invariant: Y = 1.6 * X_0 + eps)
        // X_1 = operational/linguistic surface token proxy (spurious: strongly correlated in e_0, inverted under intervention e_2)
        let mut envs = Vec::with_capacity(3);
        let configs = [
            ("e0_historical_status_quo", 1.0, 1.25),
            ("e1_mild_shift", 1.6, 0.85),
            ("e2_counterfactual_intervention", 1.3, -1.35),
        ];

        for &(name, scale_x0, proxy_coupling) in &configs {
            let mut x_mat = Vec::with_capacity(240);
            let mut y_vec = Vec::with_capacity(240);
            for _ in 0..240 {
                let x0 = rng.normal(0.0, scale_x0);
                let y = 1.60 * x0 + rng.normal(0.0, 0.10);
                let x1 = proxy_coupling * y + rng.normal(0.0, 0.12);
                x_mat.push(vec![x0, x1]);
                y_vec.push(y);
            }
            envs.push(CausalEnvironment {
                name: name.to_string(),
                x: x_mat,
                y: y_vec,
            });
        }

        // Fit an operationalist "Stochastic Parrot" exclusively on surface proxy X_1 in historical environment e_0
        // (maximizing conditional co-occurrence P(w_t | w_{1:t-1}) without asking for causal structure)
        let e0 = &envs[0];
        let mut num_w1 = 0.0;
        let mut den_w1 = 0.0;
        let mut num_w0 = 0.0;
        let mut den_w0 = 0.0;
        for (row, &yi) in e0.x.iter().zip(e0.y.iter()) {
            num_w0 += row[0] * yi;
            den_w0 += row[0] * row[0];
            num_w1 += row[1] * yi;
            den_w1 += row[1] * row[1];
        }
        let beta_causal_x0 = num_w0 / den_w0.max(1e-12);
        let beta_parrot_x1 = num_w1 / den_w1.max(1e-12);

        let mut parrot_stationary_sq = 0.0;
        for (row, &yi) in e0.x.iter().zip(e0.y.iter()) {
            let err = yi - beta_parrot_x1 * row[1];
            parrot_stationary_sq += err * err;
        }
        let parrot_stationary_surface_mse = parrot_stationary_sq / (e0.y.len() as f64);

        // Evaluate both predictors under counterfactual intervention (environment e_2)
        let e2 = &envs[2];
        let mut parrot_cf_sq = 0.0;
        let mut causal_cf_sq = 0.0;
        for (row, &yi) in e2.x.iter().zip(e2.y.iter()) {
            parrot_cf_sq += (yi - beta_parrot_x1 * row[1]).powi(2);
            causal_cf_sq += (yi - beta_causal_x0 * row[0]).powi(2);
        }
        let parrot_counterfactual_intervention_mse = parrot_cf_sq / (e2.y.len() as f64);
        let causal_grounded_counterfactual_mse = causal_cf_sq / (e2.y.len() as f64);
        let deployed_counterfactual_mse = if causal_normative_grounding {
            causal_grounded_counterfactual_mse
        } else {
            parrot_counterfactual_intervention_mse
        };

        // Nonlinear ICP / NILE audit across environments
        let nile = run_nonlinear_icp_nile(&envs, 0.15);
        let pen0 = nile.nonlinear_nile_irm_penalties[0].max(1e-9);
        let pen1 = nile.nonlinear_nile_irm_penalties[1];
        let spurious_proxy_invariance_penalty_ratio = pen1 / pen0;

        // Authentic Model Form (\delta(x) + \hat{S}^* + named discarded spurious atoms)
        let auth = AuthenticModelForm::audit_environments(
            seed,
            &envs,
            &[
                "atom_0_ontological_causal_driver".to_string(),
                "atom_1_marcuse_operational_token_proxy".to_string(),
            ],
            0.15,
            causal_normative_grounding,
        );
        let authentic_form_blocks_spurious_preemption =
            auth.approved_authentic_form && auth.invariant_core_s_star == vec![0];
        let auth_named = AuthenticModelForm::audit_environments(
            seed,
            &envs,
            &[
                "atom_0_ontological_causal_driver".to_string(),
                "atom_1_marcuse_operational_token_proxy".to_string(),
            ],
            0.15,
            true,
        );
        let authentic_form_audit_approves_causal_core =
            auth_named.approved_authentic_form && auth_named.invariant_core_s_star == vec![0];

        // Rouvroy & Berns Algorithmic Governmentality: preemptive profiling under unequal historical base rates
        // (p_A = 0.32 vs p_B = 0.09) forces an algebraic Chouldechova FPR disparity
        let (_, _, preemptive_profiling_fpr_disparity_ratio) =
            ChouldechovaAudit::fpr_ratio_under_equal_ppv_and_fnr(0.32, 0.09, 0.65, 0.25);

        let goodhart_taxonomy = GoodhartTaxonomyAudit::evaluate(
            seed,
            spurious_proxy_invariance_penalty_ratio,
            parrot_counterfactual_intervention_mse - causal_grounded_counterfactual_mse,
        );

        let identities_closed = preemptive_profiling_fpr_disparity_ratio > 3.0;
        let claims: Vec<NamedCheck> = vec![
            (
                "critical.a2.parrot_surface_mse_below_0.06",
                parrot_stationary_surface_mse < 0.06,
            ),
            (
                "critical.a2.parrot_counterfactual_mse_above_5",
                parrot_counterfactual_intervention_mse > 5.0,
            ),
            (
                "critical.a2.proxy_penalty_ratio_above_20",
                spurious_proxy_invariance_penalty_ratio > 20.0,
            ),
            (
                "critical.a2.goodhart_four_variants_per_core_layers_thresholds",
                goodhart_taxonomy.all_four_variants_audited,
            ),
            (
                "critical.a2.authentic_form_criteria_c1_to_c5_approve_s_star_0",
                authentic_form_audit_approves_causal_core,
            ),
            (
                "critical.a2.deployed_counterfactual_mse_below_0.03",
                deployed_counterfactual_mse < 0.03,
            ),
        ];
        let axis2_verified = identities_closed && claims.iter().all(|c| c.1);

        Self {
            parrot_stationary_surface_mse,
            parrot_counterfactual_intervention_mse,
            causal_grounded_counterfactual_mse,
            deployed_counterfactual_mse,
            spurious_proxy_invariance_penalty_ratio,
            preemptive_profiling_fpr_disparity_ratio,
            authentic_form_blocks_spurious_preemption,
            authentic_form_audit_approves_causal_core,
            goodhart_taxonomy,
            claims,
            identities_closed,
            axis2_verified,
        }
    }
}

/// Axis 3: The Material, Informational, and Thermodynamic Collapse of Brute-Force Stochastic Reason.
#[derive(Debug, Clone, PartialEq)]
pub struct MaterialCollapseTriadAudit {
    /// Informative: Epoch AI (Villalobos et al., Nov 2024) effective human public text stock in trillions of tokens.
    pub epoch_ai_effective_human_tokens_trillions: f64,
    /// Informative: Epoch AI raw upper bound including unindexed text in trillions of tokens.
    pub epoch_ai_raw_upper_bound_tokens_trillions: f64,
    /// Informative: projected exhaustion window of human public text.
    pub human_token_exhaustion_window_years: (u32, u32),
    /// Simulated variance retention $\sigma_G^2 / \sigma_0^2$ after 8 generations of unanchored synthetic recursion (`crate::analog`). Gated (`< 0.25`).
    pub synthetic_autophagy_variance_retention: f64,
    /// Informative: Shannon entropy loss $H(P_0) - H(P_G)$ between two illustrative distributions (the collapsed one is hand-specified).
    pub synthetic_autophagy_entropy_loss_nats: f64,
    /// Simulated $D_{\mathrm{KL}}(P_0 \parallel P_G)$ after unanchored synthetic recursion (`crate::analog`). Gated (`> 0.60`).
    pub synthetic_autophagy_kl_divergence_nats: f64,
    /// Informative: $\ln 16 - \ln 2$, the information removed by mapping 16 levels onto one bit.
    pub binary_abstraction_information_loss_nats: f64,
    /// Informative: Horowitz (ISSCC 2014, 45nm CMOS) DRAM read to FP32 addition energy ratio.
    pub von_neumann_dram_to_fp32_ratio: f64,
    /// Informative: Nvidia H100 GPU power and total cooled envelope (W).
    pub h100_gpu_watts_and_cooled_watts: (f64, f64),
    /// Informative: IEA global data-center electricity demand trajectory in 2030 (TWh).
    pub iea_2030_datacenter_twh: f64,
    /// Informative: Ferran Alia (July 2026) 5-point noise threshold curve `[sigma=0.1, 0.2, 0.4, 0.6, 0.8]` as transcribed.
    pub ferran_alia_threshold_cliff_curve: [f64; 5],
    /// Informative: $\sigma_0 \sqrt{L}$ for $L = 32$ layers at $\sigma_0 = 0.15$ (independent-noise compounding).
    pub deep_llm_32_layer_compounded_noise_sigma: f64,
    /// Named claim checks gated by this axis (see `crate::acceptance`).
    pub claims: Vec<NamedCheck>,
    /// Claims removed from the gate after failing multi-seed acceptance (`crate::acceptance`); still computed and reported.
    pub discarded_claims: Vec<NamedCheck>,
    /// Whether the simulated synthetic-recursion collapse claims hold.
    pub axis3_verified: bool,
}

impl MaterialCollapseTriadAudit {
    /// Evaluates the three material contradictions (`seed = 2026`).
    pub fn evaluate(seed: u64) -> Self {
        let analog_syn = AnalogWaveComputingSynthesis::evaluate_canonical(seed);
        let synthetic_autophagy_variance_retention = analog_syn
            .von_neumann_and_collapse
            .unanchored_synthetic_variance_retention;
        let synthetic_autophagy_kl_divergence_nats = analog_syn
            .von_neumann_and_collapse
            .unanchored_synthetic_kl_divergence_nats;

        // Illustrative entropy contrast between the base distribution and a hand-specified collapsed distribution
        let p0 = [0.12_f64, 0.23, 0.30, 0.23, 0.12];
        let pg_collapsed = [0.002_f64, 0.038, 0.920, 0.038, 0.002];
        let h0: f64 = p0.iter().map(|&p| -p * p.ln()).sum();
        let hg: f64 = pg_collapsed.iter().map(|&p| -p * p.ln()).sum();

        let claims: Vec<NamedCheck> = vec![(
            "critical.a3.synthetic_kl_above_0.60",
            synthetic_autophagy_kl_divergence_nats > 0.60,
        )];
        // Variance retention falls below 0.25 on 15 of 20 acceptance seeds after 8 generations.
        let discarded_claims: Vec<NamedCheck> = vec![(
            "critical.a3.synthetic_variance_retention_below_0.25",
            synthetic_autophagy_variance_retention < 0.25,
        )];
        let axis3_verified = claims.iter().all(|c| c.1);

        Self {
            epoch_ai_effective_human_tokens_trillions: 300.0,
            epoch_ai_raw_upper_bound_tokens_trillions: 3_100.0,
            human_token_exhaustion_window_years: (2026, 2032),
            synthetic_autophagy_variance_retention,
            synthetic_autophagy_entropy_loss_nats: h0 - hg,
            synthetic_autophagy_kl_divergence_nats,
            binary_abstraction_information_loss_nats: (16.0_f64).ln() - (2.0_f64).ln(),
            von_neumann_dram_to_fp32_ratio: analog_syn
                .von_neumann_and_collapse
                .dram_to_fp32_add_ratio,
            h100_gpu_watts_and_cooled_watts: (700.0, 1_100.0),
            iea_2030_datacenter_twh: analog_syn
                .von_neumann_and_collapse
                .iea_datacenter_twh_trajectory[2],
            ferran_alia_threshold_cliff_curve: [0.98, 0.83, 0.64, 0.39, 0.25],
            deep_llm_32_layer_compounded_noise_sigma: 0.15 * (32.0_f64).sqrt(),
            claims,
            discarded_claims,
            axis3_verified,
        }
    }
}

/// Fraction of `runs` 8-generation synthetic recursions over 5 semantic states whose final distribution satisfies the symbolic
/// rules (mirror symmetry $|p_i - p_{4-i}| \le 0.02$ and variance retention $\ge 0.90$). With `constrained`, each generation samples
/// the anchored observations and projects onto the symmetric set; otherwise it refits temperature-sharpened self-samples.
fn synthetic_recursion_rule_satisfaction(rng: &mut Rng, constrained: bool, runs: usize) -> f64 {
    let p0 = [0.12_f64, 0.23, 0.30, 0.23, 0.12];
    let vals = [-2.0_f64, -1.0, 0.0, 1.0, 2.0];
    let variance = |p: &[f64; 5]| {
        let m: f64 = p.iter().zip(&vals).map(|(pi, v)| pi * v).sum();
        p.iter()
            .zip(&vals)
            .map(|(pi, v)| pi * (v - m).powi(2))
            .sum::<f64>()
    };
    let var0 = variance(&p0);
    let draw = |rng: &mut Rng, w: &[f64; 5]| -> usize {
        let u = rng.next_f64();
        let mut acc = 0.0;
        for (i, &wi) in w.iter().enumerate() {
            acc += wi;
            if u < acc {
                return i;
            }
        }
        4
    };
    let n_per_gen = 120usize;
    let mut satisfied = 0usize;
    for _ in 0..runs {
        let mut p = p0;
        for _ in 0..8 {
            let source = if constrained {
                p0
            } else {
                let mut w = p.map(|pi| pi.powf(1.35));
                let total: f64 = w.iter().sum();
                w.iter_mut().for_each(|wi| *wi /= total);
                w
            };
            let mut counts = [0usize; 5];
            for _ in 0..n_per_gen {
                counts[draw(rng, &source)] += 1;
            }
            if constrained {
                let raw = counts.map(|c| (c as f64 + 0.5) / (n_per_gen as f64 + 2.5));
                for i in 0..5 {
                    p[i] = 0.25 * 0.5 * (raw[i] + raw[4 - i]) + 0.75 * p0[i];
                }
            } else {
                p = counts.map(|c| (c as f64 + 1e-4) / (n_per_gen as f64 + 5e-4));
            }
        }
        let symmetric = (0..5).all(|i| (p[i] - p[4 - i]).abs() <= 0.02);
        if symmetric && variance(&p) / var0 >= 0.90 {
            satisfied += 1;
        }
    }
    satisfied as f64 / runs as f64
}

/// Axis 4: Paths of Overcoming — Reconciling Logical Structure (Neurosymbolic Computing),
/// Coexistence with Physical Noise (`A-ECC`, Peking University 24-Bit `HP-INV`),
/// and Continuous Wave-Interference Physics (`SIREN`).
#[derive(Debug, Clone, PartialEq)]
pub struct DialecticalOvercomingSynthesisAudit {
    /// Symbolic-rule satisfaction rate over 200 unconstrained synthetic recursions.
    pub unanchored_statistical_rule_satisfaction_rate: f64,
    /// Symbolic-rule satisfaction rate over 200 constrained (anchored + symmetric projection) recursions.
    pub neurosymbolic_rule_satisfaction_rate: f64,
    /// Rule satisfaction of the deployed recursion (constrained if overcoming is enabled). Gated (`>= 0.95`).
    pub deployed_rule_satisfaction_rate: f64,
    /// Variance retention $\sigma_G^2 / \sigma_0^2$ of the deployed recursion in `crate::analog` (anchored only if overcoming is enabled). Gated (`> 0.90`).
    pub neurosymbolic_variance_retention: f64,
    /// Informative: accuracy at $\sigma = 0.60$ under `NAT` vs. standard training; `crate::analog` models `NAT` as an assumed reduction of
    /// effective conductance noise, so these values are not trained results.
    pub nat_vs_standard_accuracy_at_sigma_060: (f64, f64),
    /// Accuracy of drifted PCM readouts with the A-ECC calibration column vs. raw drift.
    pub a_ecc_compensated_vs_raw_accuracy: (f64, f64),
    /// Accuracy of the deployed readout (compensated if overcoming is enabled). Gated (`>= 0.97`).
    pub a_ecc_drift_recovery_accuracy: f64,
    /// Relative residual of the deployed `HP-INV` solve (10 refinements if overcoming is enabled, 1 otherwise). Gated (`< 1e-7`).
    pub pku_40nm_rram_24bit_hp_inv_error: f64,
    /// Informative: phase-amplitude parameter compression ratio (480 grid points over 12 sinusoid parameters).
    pub wave_phase_amplitude_compression_ratio: f64,
    /// `SIREN` (projection on the known frequencies, an identity) vs. piecewise-linear `ReLU` Helmholtz Laplacian MSE.
    pub siren_vs_relu_helmholtz_laplacian_mse: (f64, f64),
    /// Informative: Laplacian MSE of the deployed representation (`SIREN` if overcoming is enabled, `ReLU` otherwise); the `ReLU`
    /// side is zero by construction ($\mathrm{ReLU}'' = 0$ a.e.), so it is not a test.
    pub deployed_helmholtz_laplacian_mse: f64,
    /// Informative: Tsinghua University `ACCEL` diffractive photonic efficiency in TOPS/W as transcribed.
    pub tsinghua_accel_photonic_tops_per_watt: f64,
    /// Named claim checks gated by this axis (see `crate::acceptance`).
    pub claims: Vec<NamedCheck>,
    /// True if the `SIREN` projection identity closes (Laplacian MSE $< 10^{-10}$).
    pub identities_closed: bool,
    /// Whether the deployed recursion, readout, solver, and representation pass their claim checks.
    pub axis4_verified: bool,
}

impl DialecticalOvercomingSynthesisAudit {
    /// Evaluates Axis 4 (`seed = 2026`). When `enable_overcoming` is `false`, the unconstrained recursion, raw drifted readout,
    /// one-shot inversion, and `ReLU` representation are deployed.
    pub fn evaluate(seed: u64, enable_overcoming: bool) -> Self {
        let analog = AnalogWaveComputingSynthesis::evaluate_with_mode(seed, enable_overcoming);
        let aimc: &AimcAnalogWallHpInvAudit = &analog.aimc_analog_wall_hp_inv;

        let mut rng = Rng::new(seed ^ 0xC417_1CA1_2026);
        let unanchored_rate = synthetic_recursion_rule_satisfaction(&mut rng, false, 200);
        let constrained_rate = synthetic_recursion_rule_satisfaction(&mut rng, true, 200);
        let deployed_rule_rate = if enable_overcoming {
            constrained_rate
        } else {
            unanchored_rate
        };
        let neurosymbolic_variance_retention = analog
            .von_neumann_and_collapse
            .neurosymbolic_anchored_variance_retention;

        let a_ecc_pair = (
            aimc.a_ecc_drift_recovered_accuracy,
            aimc.a_ecc_uncompensated_accuracy,
        );
        let deployed_a_ecc = if enable_overcoming {
            a_ecc_pair.0
        } else {
            a_ecc_pair.1
        };
        let siren_vs_relu = (
            analog.wave_interference.siren_helmholtz_laplacian_mse,
            analog.wave_interference.relu_helmholtz_laplacian_mse,
        );
        let deployed_laplacian = if enable_overcoming {
            siren_vs_relu.0
        } else {
            siren_vs_relu.1
        };
        let pku_40nm_rram_24bit_hp_inv_error = aimc.pku_hp_inv_final_relative_error;

        let identities_closed = siren_vs_relu.0 < 1e-10;
        let claims: Vec<NamedCheck> = vec![
            (
                "critical.a4.deployed_rule_satisfaction_at_least_0.95",
                deployed_rule_rate >= 0.95,
            ),
            (
                "critical.a4.deployed_variance_retention_above_0.90",
                neurosymbolic_variance_retention > 0.90,
            ),
            (
                "critical.a4.deployed_a_ecc_accuracy_at_least_0.97",
                deployed_a_ecc >= 0.97,
            ),
            (
                "critical.a4.deployed_hp_inv_residual_below_1e-7",
                pku_40nm_rram_24bit_hp_inv_error < 1e-7,
            ),
        ];
        let axis4_verified = identities_closed && claims.iter().all(|c| c.1);

        Self {
            unanchored_statistical_rule_satisfaction_rate: unanchored_rate,
            neurosymbolic_rule_satisfaction_rate: constrained_rate,
            deployed_rule_satisfaction_rate: deployed_rule_rate,
            neurosymbolic_variance_retention,
            nat_vs_standard_accuracy_at_sigma_060: (
                aimc.nat_accuracy_at_sigma_060,
                aimc.standard_accuracy_at_sigma_060,
            ),
            a_ecc_compensated_vs_raw_accuracy: a_ecc_pair,
            a_ecc_drift_recovery_accuracy: deployed_a_ecc,
            pku_40nm_rram_24bit_hp_inv_error,
            wave_phase_amplitude_compression_ratio: analog
                .wave_interference
                .phase_amplitude_compression_ratio,
            siren_vs_relu_helmholtz_laplacian_mse: siren_vs_relu,
            deployed_helmholtz_laplacian_mse: deployed_laplacian,
            tsinghua_accel_photonic_tops_per_watt: analog
                .sensory_dpi_and_ecosystem
                .tsinghua_accel_tops_per_watt,
            claims,
            identities_closed,
            axis4_verified,
        }
    }
}

/// Complete 4-Axis Synthesis of **Critical Theory of Stochastic Rationality, Algorithmic
/// Governmentality, and the Material Collapse & Neurosymbolic/Wave Overcoming of AI** (§1.3.9).
#[derive(Debug, Clone, PartialEq)]
pub struct CriticalStochasticRationalitySynthesis {
    /// Axis 1: Adorno & Horkheimer's Liquidation of the *Non-Identical* and Eternalization of the Present.
    pub adorno_non_identical_and_reification: AdornoNonIdenticalReificationAudit,
    /// Axis 2: Marcuse's Operational Closure, Rouvroy's Algorithmic Preemption, and "Stochastic Parrots".
    pub marcuse_rouvroy_stochastic_parrot: MarcuseRouvroyStochasticParrotAudit,
    /// Axis 3: Material Collapse Triad (Informational Autophagy, Binary Thermodynamic Wall, and Analog Noise Cliff).
    pub material_collapse_triad: MaterialCollapseTriadAudit,
    /// Axis 4: Dialectical Overcoming (Neurosymbolic Constraints, A-ECC/PKU `HP-INV`, and Continuous Wave Physics).
    pub dialectical_overcoming: DialecticalOvercomingSynthesisAudit,
    /// True if every axis's identity checks close (expected in both modes).
    pub all_identities_closed: bool,
    /// Whether all four axes pass their epistemological and mathematical gates.
    pub all_four_axes_verified: bool,
}

impl CriticalStochasticRationalitySynthesis {
    /// Evaluates the canonical 4-axis synthesis (`seed = 2026`).
    pub fn evaluate_canonical(seed: u64) -> Self {
        Self::evaluate_with_mode(seed, true)
    }

    /// Evaluates the 4-axis synthesis deploying the grounded methods (`true`) or the naive positivist methods
    /// (`false`, used by `--defect instrumental_reason`).
    pub fn evaluate_with_mode(seed: u64, grounded_mode: bool) -> Self {
        let adorno_non_identical_and_reification =
            AdornoNonIdenticalReificationAudit::evaluate(seed, grounded_mode);
        let marcuse_rouvroy_stochastic_parrot =
            MarcuseRouvroyStochasticParrotAudit::evaluate(seed, grounded_mode);
        let material_collapse_triad = MaterialCollapseTriadAudit::evaluate(seed);
        let dialectical_overcoming =
            DialecticalOvercomingSynthesisAudit::evaluate(seed, grounded_mode);

        let all_identities_closed = adorno_non_identical_and_reification.identities_closed
            && marcuse_rouvroy_stochastic_parrot.identities_closed
            && dialectical_overcoming.identities_closed;
        let all_four_axes_verified = adorno_non_identical_and_reification.axis1_verified
            && marcuse_rouvroy_stochastic_parrot.axis2_verified
            && material_collapse_triad.axis3_verified
            && dialectical_overcoming.axis4_verified;

        Self {
            adorno_non_identical_and_reification,
            marcuse_rouvroy_stochastic_parrot,
            material_collapse_triad,
            dialectical_overcoming,
            all_identities_closed,
            all_four_axes_verified,
        }
    }

    /// Claims removed from the gate after failing multi-seed acceptance.
    pub fn discarded_claim_checks(&self) -> Vec<NamedCheck> {
        self.adorno_non_identical_and_reification
            .discarded_claims
            .iter()
            .chain(&self.material_collapse_triad.discarded_claims)
            .copied()
            .collect()
    }

    /// All gated claims of the four axes, in axis order.
    pub fn claim_checks(&self) -> Vec<NamedCheck> {
        [
            &self.adorno_non_identical_and_reification.claims,
            &self.marcuse_rouvroy_stochastic_parrot.claims,
            &self.material_collapse_triad.claims,
            &self.dialectical_overcoming.claims,
        ]
        .into_iter()
        .flatten()
        .copied()
        .collect()
    }

    /// Per-claim pass rates over `seeds` in the given mode (see `crate::acceptance`).
    pub fn multi_seed_acceptance(seeds: &[u64], grounded_mode: bool) -> MultiSeedAcceptance {
        MultiSeedAcceptance::evaluate(seeds, |seed| {
            let syn = Self::evaluate_with_mode(seed, grounded_mode);
            (syn.claim_checks(), syn.all_identities_closed)
        })
    }
}

/// Real-Data Empirical Coupler for **Critical Stochastic Rationality** across `danish_fire.csv` ($n=2\text{,}167$),
/// `toronto_arrests.csv` ($n=5\text{,}226$), `atlantic_storms.csv` ($n=20\text{,}778$), and `vic_elec.csv` ($n=52\text{,}608$).
#[derive(Debug, Clone, PartialEq)]
pub struct EmpiricalCriticalRationalityAudit {
    /// Hill tail index $\hat{\alpha}_{k,n} \in (1, 2]$ on `danish_fire.csv` (infinite theoretical variance regime).
    pub danish_fire_hill_alpha: f64,
    /// True Expected Shortfall $\mathrm{ES}_{0.99}$ on `danish_fire.csv` vs. Gaussian $2\sigma$-truncated $\mathrm{ES}_{0.99}^{\text{trunc}}$.
    pub danish_fire_true_vs_truncated_es99: (f64, f64),
    /// Fraction of real `danish_fire.csv` tail risk liquidated by forced Gaussian identity ($> 65\%$).
    pub danish_fire_non_identical_erasure_fraction: f64,
    /// Chouldechova FPR disparity ratio ($\mathrm{FPR}_A / \mathrm{FPR}_B$) on `toronto_arrests.csv` base rates.
    pub toronto_arrests_preemptive_fpr_ratio: f64,
    /// Ratio of spurious latitude proxy penalty to invariant barometric core penalty on `atlantic_storms.csv` ($> 80\times$).
    pub atlantic_storms_spurious_to_invariant_penalty_ratio: f64,
    /// Wave-interference compression ratio and PKU 24-bit `HP-INV` spectral error on `vic_elec.csv`.
    pub vic_elec_wave_compression_and_hp_inv_error: (f64, f64),
    /// Whether all four empirical real-data audits are verified.
    pub empirical_critical_verified: bool,
}

impl EmpiricalCriticalRationalityAudit {
    /// Audits the real datasets (`danish_fire.csv`, `toronto_arrests.csv`, `atlantic_storms.csv`, and `vic_elec.csv`).
    pub fn evaluate_real_datasets(
        danish_fire_losses: &[f64],
        toronto_base_rates: (f64, f64),
        atlantic_storm_envs: &[CausalEnvironment],
        vic_elec_demand: &[f64],
    ) -> Self {
        // 1. Adorno's Liquidation of the Non-Identical on `danish_fire.csv`
        let hill = HillTailDiagnostic::audit(danish_fire_losses, Some(100));
        let es99_true = expected_shortfall_99(danish_fire_losses);
        let mu_fire = mean(danish_fire_losses);
        let sd_fire = sample_std(danish_fire_losses);
        let fire_cutoff = mu_fire + 2.0 * sd_fire;
        let fire_trunc: Vec<f64> = danish_fire_losses
            .iter()
            .copied()
            .filter(|&x| x <= fire_cutoff)
            .collect();
        let es99_trunc = expected_shortfall_99(&fire_trunc);
        let danish_fire_non_identical_erasure_fraction = 1.0 - (es99_trunc / es99_true.max(1e-12));

        // 2. Rouvroy's Algorithmic Preemption & Reification on `toronto_arrests.csv`
        let (p_a, p_b) = toronto_base_rates;
        let (_, _, toronto_arrests_preemptive_fpr_ratio) =
            ChouldechovaAudit::fpr_ratio_under_equal_ppv_and_fnr(p_a, p_b, 0.65, 0.25);

        // 3. Marcuse's Operational Closure vs. Causal Invariance on `atlantic_storms.csv`
        let nile = run_nonlinear_icp_nile(atlantic_storm_envs, 0.15);
        let p_core = nile.nonlinear_nile_irm_penalties[0].max(1e-9);
        let p_spurious = nile.nonlinear_nile_irm_penalties[1];
        let atlantic_storms_spurious_to_invariant_penalty_ratio = p_spurious / p_core;

        // 4. Material Collapse & Wave/Analog Overcoming on `vic_elec.csv`
        let emp_wave = EmpiricalWaveAnalogAudit::evaluate_series(vic_elec_demand);
        let vic_elec_wave_compression_and_hp_inv_error = (
            emp_wave.empirical_compression_ratio,
            emp_wave.empirical_hp_inv_relative_error,
        );

        let empirical_critical_verified = hill.alpha_hill > 1.0
            && hill.alpha_hill <= 2.0
            && danish_fire_non_identical_erasure_fraction > 0.65
            && toronto_arrests_preemptive_fpr_ratio > 5.0
            && atlantic_storms_spurious_to_invariant_penalty_ratio > 80.0
            && emp_wave.empirical_wave_verified;

        Self {
            danish_fire_hill_alpha: hill.alpha_hill,
            danish_fire_true_vs_truncated_es99: (es99_true, es99_trunc),
            danish_fire_non_identical_erasure_fraction,
            toronto_arrests_preemptive_fpr_ratio,
            atlantic_storms_spurious_to_invariant_penalty_ratio,
            vic_elec_wave_compression_and_hp_inv_error,
            empirical_critical_verified,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use stochology_core::DEFAULT_SEED;

    #[test]
    fn test_axis1_adorno_non_identical_and_reification() {
        let pos = AdornoNonIdenticalReificationAudit::evaluate(DEFAULT_SEED, true);
        assert!(pos.non_identical_liquidation_gap > 30.0);
        assert!(pos.es99_erasure_fraction > 0.60);
        assert!(pos.koh_structural_discrepancy_norm > 0.40);
        assert!(pos.status_quo_reification_excess_risk > 0.05);
        assert!(pos.axis1_verified, "{pos:?}");

        // The positivist pipeline is actually run: its VaR is breached, its extrapolation and policy miss the targets.
        let neg = AdornoNonIdenticalReificationAudit::evaluate(DEFAULT_SEED, false);
        assert!(neg.non_identical_liquidation_gap > 30.0);
        assert!(neg.deployed_var99_holdout_exceedance > 0.02, "{neg:?}");
        assert!(neg.deployed_performative_excess_risk > 1e-3);
        assert!(!neg.axis1_verified);
    }

    #[test]
    fn test_axis2_marcuse_rouvroy_stochastic_parrot() {
        let pos = MarcuseRouvroyStochasticParrotAudit::evaluate(DEFAULT_SEED, true);
        assert!(pos.parrot_stationary_surface_mse < 0.06);
        assert!(pos.parrot_counterfactual_intervention_mse > 5.0);
        assert!(pos.causal_grounded_counterfactual_mse < 0.03);
        assert!(pos.spurious_proxy_invariance_penalty_ratio > 20.0);
        assert!(pos.preemptive_profiling_fpr_disparity_ratio > 3.0);
        assert!(pos.authentic_form_blocks_spurious_preemption);
        assert!(pos.axis2_verified);

        let neg = MarcuseRouvroyStochasticParrotAudit::evaluate(DEFAULT_SEED, false);
        assert!(neg.identities_closed);
        assert!(neg.deployed_counterfactual_mse > 5.0);
        assert!(!neg.axis2_verified);
    }

    #[test]
    fn test_axis3_material_collapse_triad() {
        let m = MaterialCollapseTriadAudit::evaluate(DEFAULT_SEED);
        assert!(m.synthetic_autophagy_variance_retention < 0.25);
        assert!(m.synthetic_autophagy_kl_divergence_nats > 0.60);
        assert!(m.axis3_verified);
    }

    #[test]
    fn test_axis4_dialectical_overcoming_and_full_synthesis() {
        let syn_pos = CriticalStochasticRationalitySynthesis::evaluate_canonical(DEFAULT_SEED);
        assert!(syn_pos.all_identities_closed);
        assert!(
            syn_pos.all_four_axes_verified,
            "{:?}",
            syn_pos.dialectical_overcoming
        );
        assert!(syn_pos.dialectical_overcoming.axis4_verified);

        let syn_neg =
            CriticalStochasticRationalitySynthesis::evaluate_with_mode(DEFAULT_SEED, false);
        assert!(syn_neg.all_identities_closed);
        assert!(!syn_neg.all_four_axes_verified);
        assert!(!syn_neg.adorno_non_identical_and_reification.axis1_verified);
        assert!(!syn_neg.marcuse_rouvroy_stochastic_parrot.axis2_verified);
        assert!(!syn_neg.dialectical_overcoming.axis4_verified);
    }
}
