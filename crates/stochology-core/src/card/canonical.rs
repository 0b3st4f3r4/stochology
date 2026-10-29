//! Canonical empirically grounded Epistemological Card factory (`data/atlantic_storms.csv`).

use crate::layers::{Layer, UncertaintyClass};

use super::model::{
    BridgePerformativity, CausalLadder, DataAndAuditContract, DecisionMakerQuestion,
    DecisionRuleAndScope, DominantEvent, EpistemologicalCard, EpistemologicalHeader,
    FieldAuthorship, FindingsRegistry, InfoGapRobustness, MurphyCorpDecomposition, NoisePhysics,
    UncertaintyBudgetAndAuthorship, UnitAndMargin, VarianceBudget,
};

impl EpistemologicalCard {
    /// Returns a canonical, empirically grounded Epistemological Card (`data/atlantic_storms.csv`).
    pub fn canonical_example() -> Self {
        let mut card = Self {
            id: "atlantic_storms_coastal_evacuation".into(),
            layer: Layer::ModelL2,
            uncertainty: UncertaintyClass::EpistemicSpecification,
            decision_maker_question: DecisionMakerQuestion {
                who_decides: "Coastal Emergency Operations Director".into(),
                altered_action: "Trigger mandatory coastal evacuation order within 6h synoptic cycle when calibrated major-hurricane probability exceeds threshold p*".into(),
                question: "Does the calibrated probability of Category 2+ major hurricane intensity (wind > 85 kt) in the synoptic cycle exceed the cost-loss threshold p* = 0.12000?".into(),
            },
            epistemological_header: EpistemologicalHeader {
                generating_process: "Barometric pressure-wind gradient dynamics across Atlantic tropical cyclone tracks with Kennedy-O'Hagan GP discrepancy".into(),
                nonlinearity: "f(E[X])=4849.59, E[f(X)]=5819.49, Jensen gap Delta_J=+969.91, median=3600.00, q05=900.00, q95=16900.00, CRPS=2826.59".into(),
                unit_and_margin: UnitAndMargin {
                    independent_unit: "Atlantic storm synoptic track (cluster of 6-hourly observations)".into(),
                    nominal_sample_size_n: 20_778,
                    weighted_cluster_mean_m_tilde: "m_tilde = m_bar * (1 + CV^2) = 29.98 * 1.377 = 41.29 (Moulton-Campbell, CV = 0.614)".into(),
                    threshold_exceedance_icc: "rho_I(0.90) = 0.2634 -> DEFF_exceedance = 11.61".into(),
                    effective_sample_size_n_eff: 1_789,
                    unit_margin_of_error: "+/- 1.4 percentage points over 1,789 effective synoptic storm observations (693 storm tracks)".into(),
                },
                noise_physics: NoisePhysics {
                    stochastic_regime: "Non-ergodic eye-wall intensification cascade with heavy-tailed kinetic energy severity (alpha_Hill = 1.60)".into(),
                    absorbing_states: "Coastal landfall barrier and irreversible infrastructure inundation threshold".into(),
                    bridge_l3_l1_performativity: BridgePerformativity {
                        agent_type: "performative".into(),
                        wasserstein_sensitivity_epsilon: 0.18,
                        rrm_convergence_condition: "eps * beta / gamma = 0.18 * 1.50 / 1.00 = 0.27 < 1 (strict contraction)".into(),
                        stability_optimality_divergence: "theta_PS = 1.8293 vs Stackelberg theta_PO = 1.9631 (audited)".into(),
                    },
                },
                causal_ladder: CausalLadder {
                    pearl_rung: "2-Act".into(),
                    identification_strategy: "Invariant Causal Prediction (ICP) across multidecadal Atlantic basin regimes; Markov blanket verified without collider".into(),
                    untestable_assumption: "Hidden confounding bounded by sensitivity threshold Lambda* <= 1.50".into(),
                },
            },
            uncertainty_budget_and_authorship: UncertaintyBudgetAndAuthorship {
                variance_budget: VarianceBudget {
                    inputs: 0.45,
                    parameters: 0.35,
                    covariance: 0.20,
                    structural_discrepancy_gp: "Gaussian Process delta(x) ~ GP(0, k_anchored(x,x')) per Kennedy-O'Hagan".into(),
                },
                dominant_event: DominantEvent {
                    description: "Category 2+ major hurricane wind exceedance above 85 knots (90th percentile)".into(),
                    probability_p: 0.09770,
                    base_rate_floor_epsilon: 0.001,
                    cost_share: 1.00,
                },
                info_gap_robustness: InfoGapRobustness {
                    critical_requirement_rc: "Coastal evacuation clearance completion rate above 92% before 64-kt gale arrival (robust satisficing)".into(),
                    robustness_horizon_alpha_hat: "alpha_hat = 3.50 structural standard deviations tolerated without violating r_c".into(),
                    opportuneness_horizon_beta_hat: "beta_hat = 1.20 deviations for windfall early track recurvature r_w".into(),
                    global_maximin_safeguard: "Wald global Maximin bound >= 85% outside U(alpha, u_hat) with reversible staged shelter protocol".into(),
                },
                field_authorship: FieldAuthorship {
                    null_model: "inference".into(),
                    decision_rule: "decision".into(),
                    adversarial_reviewer: "independent_adversarial_auditor".into(),
                },
            },
            data_and_audit_contract: DataAndAuditContract {
                dataset_path: "data/atlantic_storms.csv".into(),
                dataset_sha256: "f7c58accf7472a32ee92218afba12843ae3952529ce8247ba7ae0db6c7602b6c".into(),
                missingness_mechanism: "MAR (early pre-dropsonde central pressure records missing; Meng correlation rho_R_Y = 0.003; Manski bounds verified)".into(),
                seed_and_environment: "seed=2026; rustc=1.97; linux-x86_64; container_sha256=ba7816bf".into(),
                null_model: "crates/stochology-cli#evaluate_empirical_card_severe_test".into(),
                preregistration_hash: String::new(),
                attempt_counter_k: 1,
                e_process_multiplicity: "Cumulative Ville e-process E_30 = 40741.2 (threshold 1/alpha = 20.0 for alpha = 0.05)".into(),
                murphy_corp_decomposition: MurphyCorpDecomposition {
                    total_score_brier_or_crps: 0.03210,
                    miscalibration_rel_mcb: 0.00578,
                    resolution_res_dsc: 0.06183,
                    base_uncertainty_unc0: 0.08815,
                },
                findings: FindingsRegistry {
                    registered: 2,
                    used: 1,
                    unused: vec![
                        "Latitude-only polynomial interaction discarded due to insufficient tail n_eff(0.90) in Filter 2".into(),
                    ],
                },
            },
            decision_rule_and_scope: DecisionRuleAndScope {
                event_probability_p: 0.09770,
                base_rate_floor_epsilon: 0.001,
                protection_cost_c: 3_600.0,
                event_loss_l: 30_000.0,
                mitigation_efficacy_eta: 1.0,
                critical_threshold_p_star: 0.12000,
                declared_act: "do_not_protect".into(),
                scope_statement: "Valid for North Atlantic basin tropical cyclone synoptic tracks (HURDAT2); does not apply to inland flash-flooding or Pacific typhoons".into(),
                who_pays_for_error: "Chouldechova trilemma audit: high-vulnerability vs fortified coastal zones (p_A=0.3926 vs p_B=0.1119); equalizing PPV at 0.65 & FNR at 0.25 implies monitored ratio FPR_A/FPR_B = 5.13".into(),
                exit_clause: "Immediately suspend automated synoptic advisory if REL_CORP > 0.015 or RES_CORP < 0.020 over a 14-day rolling window".into(),
            },
        };
        card.data_and_audit_contract.preregistration_hash = card.compute_preregistration_hash();
        card
    }
}
