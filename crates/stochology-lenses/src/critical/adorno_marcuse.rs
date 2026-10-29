//! Axis 1 & Axis 2 of Critical Stochastic Rationality:
//! - Adorno & Horkheimer's Liquidation of the *Non-Identical* (`AdornoNonIdenticalReificationAudit`)
//! - Marcuse's Operational Closure, Rouvroy's Algorithmic Preemption & "Stochastic Parrots" (`MarcuseRouvroyStochasticParrotAudit`)

use crate::acceptance::NamedCheck;
use crate::invariance::{CausalEnvironment, run_nonlinear_icp_nile};
use crate::machine::AuthenticModelForm;
use crate::tail::expected_shortfall_99;
use stochology_core::{
    ChouldechovaAudit, GoodhartTaxonomyAudit, KennedyOHaganDiscrepancy, NonlinearSummary,
    PerformativeSystem, Rng, mean, quantile, sample_std, sample_variance,
};

/// Heavy-tailed draw of the Axis 1 generating process: Pareto ($\alpha = 1.65$) plus a structural singularity every 40th index.
pub(crate) fn non_identical_draw(rng: &mut Rng, index: usize) -> f64 {
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
        // Development: the liquidation gap exceeds 30 on 7 of 20 seeds. Refuted on the one-shot confirmation
        // (seeds 9_000_000..=9_000_199): ES99 erasure above 0.60 on 0.945 of seeds.
        let discarded_claims: Vec<NamedCheck> = vec![
            (
                "critical.a1.liquidation_gap_above_30",
                non_identical_liquidation_gap > 30.0,
            ),
            (
                "critical.a1.es99_erasure_above_0.60",
                es99_erasure_fraction > 0.60,
            ),
        ];
        let claims: Vec<NamedCheck> = vec![
            (
                "critical.a1.det.koh_improvement_above_0.40",
                koh_structural_discrepancy_norm > 0.40,
            ),
            (
                "critical.a1.det.reification_excess_above_0.05",
                status_quo_reification_excess_risk > 0.05,
            ),
            (
                "critical.a1.deployed_var99_breach_at_most_2pct",
                deployed_exceedance <= 0.02,
            ),
            (
                "critical.a1.det.deployed_koh_error_below_half_bare",
                deployed_koh_error < 0.5 * koh.extrapolation_error_without_gp.max(1e-12),
            ),
            (
                "critical.a1.det.deployed_performative_excess_below_1e-3",
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
