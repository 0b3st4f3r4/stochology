//! Axis 2: Epistemic & Conjectural Engine — Warren Powell's Policy Meta-Classes, Karl Friston's Variational Free Energy
//! and the preposterior value of information, and Judea Pearl's `do-Calculus` (Front-Door, Mediation, Transportability).

use crate::acceptance::NamedCheck;
use stochology_core::Rng;
use stochology_core::math::linalg::ols_slope;

/// Warren Powell's 4 Universal Meta-Classes of Sequential Decision Policies + Hybrid Class.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowellPolicyClass {
    /// Policy Function Approximation (`PFA`): direct analytical/lookup mapping $S_t \to x_t$ without embedded optimization.
    PfaLookupRule,
    /// Cost Function Approximation (`CFA`): parameterized deterministic optimization with tuned buffer/slack $\theta$.
    CfaParameterizedBuffer,
    /// Value Function Approximation (`VFA`): Bellman lookahead value approximation $\hat{V}_{t+1}(S_{t+1})$.
    VfaBellmanApproximation,
    /// Direct Lookahead Approximation (`DLA`): deterministic (`MPC`) or stochastic (`MCTS` / two-stage) rolling horizon.
    DlaRollingHorizon,
    /// Hybrid Policy (`DLA + VFA` / `CFA + PFA`): lookahead over short horizon coupled to terminal value/rule constraints.
    HybridDlaVfa,
}

/// Axis 2: Epistemic & Conjectural Engine — Warren Powell's Policy Meta-Classes, Karl Friston's Variational Free Energy
/// and the preposterior value of information, and Judea Pearl's `do-Calculus` (Front-Door, Mediation, Transportability).
#[derive(Debug, Clone, PartialEq)]
pub struct PowellFristonCausalEngineAudit {
    /// Informative: declared regime-to-class mapping for Powell's 5 canonical problem regimes (not derived by simulation here).
    pub powell_five_regime_winners: [PowellPolicyClass; 5],
    /// Slack $\theta^*$ of the `CFA` order-up-to rule selected on a 400-period training path (grid $0, 0.25, \dots, 3.0$).
    pub powell_cfa_tuned_theta: f64,
    /// Out-of-sample per-period reward gain of the tuned rule $\theta^*$ over myopic $\theta = 0$ (mode-independent). Gated (`> 5.0`).
    pub powell_cfa_tuned_vs_myopic_gain: f64,
    /// Informative: out-of-sample gain of the deployed rule over myopic (`0` by definition when the myopic rule is deployed).
    pub powell_cfa_deployed_gain: f64,
    /// Identity: Variational Free Energy drop $F(q_0) - F(q^*)$ from an uncalibrated belief to the Bayesian posterior ($\ge 0$).
    pub friston_vfe_reduction: f64,
    /// Simulated expected loss of (probe, then act) vs. (act now) under an ambiguous prior $P(\text{fragile}) = 0.50$.
    pub voi_probe_vs_act_now_loss_ambiguous: (f64, f64),
    /// Simulated expected loss of (probe, then act) vs. (act now) under a resolved prior $P(\text{fragile}) = 0.05$.
    pub voi_probe_vs_act_now_loss_resolved: (f64, f64),
    /// True if the deployed agent (preposterior analysis if shielded, always act now otherwise) picks the lower simulated loss in both regimes. Gated.
    pub deployed_policy_choices_match_voi: bool,
    /// Identity: true causal effect $\mathbb{E}[Y \mid \mathrm{do}(X=1)] - \mathbb{E}[Y \mid \mathrm{do}(X=0)] = 2.00 \times 1.40$ in the SCM.
    pub pearl_true_do_effect: f64,
    /// Absolute bias of the Rung-1 OLS slope of $Y$ on $X$ under the unobserved confounder $U$. Gated as a mode-independent claim (`> 1.0`).
    pub pearl_naive_rung1_associative_bias: f64,
    /// Causal effect reported by the deployed estimator (front-door if shielded, Rung-1 slope otherwise). Gated (`|.| - 2.80 < 0.15`).
    pub pearl_front_door_identified_effect: f64,
    /// Informative: Controlled and Natural Direct Effects in closed form from the mediation SCM coefficients.
    pub pearl_counterfactual_cde_and_nde: (f64, f64),
    /// $|\hat{\tau}(\Pi^*) - \tau_{\text{true}}(\Pi^*)|$ of the deployed estimator (transport formula if shielded, source average otherwise). Gated (`< 0.05`).
    pub pearl_transportability_error: f64,
    /// Named claim checks gated by this axis (see `crate::acceptance`).
    pub claims: Vec<NamedCheck>,
    /// True if the Free Energy and SCM identities close.
    pub identities_closed: bool,
    /// Whether identities, the Rung-1 bias claim, and the deployed `CFA`, VOI agent, and causal estimators pass.
    pub axis2_verified: bool,
}

impl PowellFristonCausalEngineAudit {
    /// Evaluates Axis 2 (`seed = 2026`). When `cybernetic_shielded` is `false`, the myopic rule, the always-act agent,
    /// the Rung-1 slope, and the untransported source effect are deployed.
    pub fn evaluate(seed: u64, cybernetic_shielded: bool) -> Self {
        let mut rng = Rng::new(seed ^ 0xA202_2026);

        let powell_five_regime_winners = [
            PowellPolicyClass::PfaLookupRule,
            PowellPolicyClass::CfaParameterizedBuffer,
            PowellPolicyClass::VfaBellmanApproximation,
            PowellPolicyClass::DlaRollingHorizon,
            PowellPolicyClass::HybridDlaVfa,
        ];

        // 1. Powell CFA: order up to 25 + theta * 8 under demand N(25, 8) with 12% spikes of +18; theta tuned on a training path.
        let draw_path = |rng: &mut Rng, periods: usize| -> Vec<f64> {
            (0..periods)
                .map(|_| {
                    let demand = (25.0 + rng.normal(0.0, 8.0)).max(5.0);
                    let spike = if rng.next_f64() < 0.12 { 18.0 } else { 0.0 };
                    demand + spike
                })
                .collect()
        };
        let mean_reward = |theta: f64, path: &[f64]| -> f64 {
            let mut stock = 40.0_f64;
            let mut reward = 0.0;
            for &load in path {
                let order = (25.0 + theta * 8.0 - stock).max(0.0);
                stock += order;
                let shortage = (load - stock).max(0.0);
                stock = (stock - load).max(0.0);
                reward += 50.0 - 0.8 * order - 0.3 * stock - 6.5 * shortage;
            }
            reward / (path.len() as f64)
        };
        let train_path = draw_path(&mut rng, 400);
        let test_path = draw_path(&mut rng, 400);
        let theta_grid: Vec<f64> = (0..=12).map(|i| 0.25 * (i as f64)).collect();
        let theta_star = theta_grid
            .iter()
            .copied()
            .max_by(|&a, &b| {
                mean_reward(a, &train_path)
                    .partial_cmp(&mean_reward(b, &train_path))
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .unwrap_or(0.0);
        let theta_deployed = if cybernetic_shielded { theta_star } else { 0.0 };
        let powell_cfa_tuned_vs_myopic_gain =
            mean_reward(theta_star, &test_path) - mean_reward(0.0, &test_path);
        let powell_cfa_deployed_gain =
            mean_reward(theta_deployed, &test_path) - mean_reward(0.0, &test_path);

        // 2. Friston Variational Free Energy F(q) = D_KL(q(s) || p(s)) - E_q[ln p(o|s)] for an observed alarm o = 1
        let prior_s = [0.50_f64, 0.50_f64];
        let likelihood_o1 = [0.15_f64, 0.90_f64];
        let vfe_for_q = |q: [f64; 2]| -> f64 {
            let kl = q[0] * (q[0] / prior_s[0]).ln() + q[1] * (q[1] / prior_s[1]).ln();
            let accuracy = q[0] * likelihood_o1[0].ln() + q[1] * likelihood_o1[1].ln();
            kl - accuracy
        };
        let evidence = prior_s[0] * likelihood_o1[0] + prior_s[1] * likelihood_o1[1];
        let posterior = [
            prior_s[0] * likelihood_o1[0] / evidence,
            prior_s[1] * likelihood_o1[1] / evidence,
        ];
        let friston_vfe_reduction = vfe_for_q([0.80, 0.20]) - vfe_for_q(posterior);

        // 3. Preposterior value of information: hidden state fragile with prior q; acting = buffer (loss 0.20) or not
        // (loss 1.0 if fragile); probing costs 0.03 and observes an alarm with P(alarm | safe, fragile) = (0.15, 0.90).
        let buffer_cost = 0.20_f64;
        let probe_cost = 0.03_f64;
        let act_loss = |q_fragile: f64| buffer_cost.min(q_fragile);
        let analytic_probe_loss = |q: f64| -> f64 {
            let p_alarm = (1.0 - q) * likelihood_o1[0] + q * likelihood_o1[1];
            let q_alarm = q * likelihood_o1[1] / p_alarm;
            let q_quiet = q * (1.0 - likelihood_o1[1]) / (1.0 - p_alarm);
            probe_cost + p_alarm * act_loss(q_alarm) + (1.0 - p_alarm) * act_loss(q_quiet)
        };
        let simulate_losses = |rng: &mut Rng, q: f64| -> (f64, f64) {
            let episodes = 20_000usize;
            let (mut probe_total, mut act_total) = (0.0, 0.0);
            let realized = |buffer: bool, fragile: bool| {
                if buffer {
                    buffer_cost
                } else if fragile {
                    1.0
                } else {
                    0.0
                }
            };
            for _ in 0..episodes {
                let fragile = rng.next_f64() < q;
                act_total += realized(q > buffer_cost, fragile);
                let alarm = rng.next_f64()
                    < if fragile {
                        likelihood_o1[1]
                    } else {
                        likelihood_o1[0]
                    };
                let p_alarm = (1.0 - q) * likelihood_o1[0] + q * likelihood_o1[1];
                let q_post = if alarm {
                    q * likelihood_o1[1] / p_alarm
                } else {
                    q * (1.0 - likelihood_o1[1]) / (1.0 - p_alarm)
                };
                probe_total += probe_cost + realized(q_post > buffer_cost, fragile);
            }
            (probe_total / episodes as f64, act_total / episodes as f64)
        };
        let mut choices_match = true;
        let mut voi_pairs = [(0.0, 0.0); 2];
        for (slot, &q) in voi_pairs.iter_mut().zip(&[0.50_f64, 0.05]) {
            let (probe_sim, act_sim) = simulate_losses(&mut rng, q);
            *slot = (probe_sim, act_sim);
            let deployed_probes = cybernetic_shielded && analytic_probe_loss(q) < act_loss(q);
            choices_match &= deployed_probes == (probe_sim < act_sim);
        }

        // 4. Judea Pearl's SCM & Front-Door identification under unobserved confounder U:
        // X = 0.85 U + eps_X, Z = 1.40 X + eps_Z, Y = 2.00 Z - 1.60 U + eps_Y; E[Y | do(X=1)] - E[Y | do(X=0)] = 2.80.
        let pearl_true_do_effect = 2.00 * 1.40;
        let n_scm = 2_000usize;
        let mut x_vec = Vec::with_capacity(n_scm);
        let mut z_vec = Vec::with_capacity(n_scm);
        let mut y_vec = Vec::with_capacity(n_scm);
        for _ in 0..n_scm {
            let u = rng.normal(0.0, 1.0);
            let x = 0.85 * u + rng.normal(0.0, 0.45);
            let z = 1.40 * x + rng.normal(0.0, 0.65);
            let y = 2.00 * z - 1.60 * u + rng.normal(0.0, 0.20);
            x_vec.push(x);
            z_vec.push(z);
            y_vec.push(y);
        }
        let naive_slope_yx = ols_slope(&x_vec, &y_vec);
        let pearl_naive_rung1_associative_bias = (naive_slope_yx - pearl_true_do_effect).abs();

        // Front-Door adjustment: X -> Z slope times Z -> Y slope controlling for X (Frisch-Waugh-Lovell residualization)
        let slope_zx = ols_slope(&x_vec, &z_vec);
        let z_resid: Vec<f64> = z_vec
            .iter()
            .zip(x_vec.iter())
            .map(|(&z, &x)| z - slope_zx * x)
            .collect();
        let y_resid: Vec<f64> = y_vec
            .iter()
            .zip(x_vec.iter())
            .map(|(&y, &x)| y - naive_slope_yx * x)
            .collect();
        let slope_yz_given_x = ols_slope(&z_resid, &y_resid);
        let pearl_front_door_identified_effect = if cybernetic_shielded {
            slope_zx * slope_yz_given_x
        } else {
            naive_slope_yx
        };

        // Counterfactual Mediation: Y = 0.90*X + 2.00*M + 0.30*X*M with M = 1.40*X + zero-mean noise:
        // CDE(m=0) = 0.90 and NDE = 0.90 + 0.30 * E[M(0)] = 0.90 in closed form.
        let pearl_counterfactual_cde_and_nde = (0.90, 0.90);

        // 5. Transportability: randomized source trial over strata S with effects tau(S) = (0.5, 1.5, 3.0),
        // source P(S) = (0.6, 0.3, 0.1) and target P*(S) = (0.1, 0.3, 0.6); true target effect = 2.30.
        let tau_s = [0.5_f64, 1.5, 3.0];
        let p_source = [0.6_f64, 0.3, 0.1];
        let p_target = [0.1_f64, 0.3, 0.6];
        let true_target_effect: f64 = tau_s.iter().zip(&p_target).map(|(t, p)| t * p).sum();
        let mut sums = [[0.0_f64; 2]; 3];
        let mut counts = [[0usize; 2]; 3];
        let mut treated_sum = 0.0;
        let mut control_sum = 0.0;
        let (mut n_treated, mut n_control) = (0usize, 0usize);
        for _ in 0..20_000 {
            let u = rng.next_f64();
            let s = if u < p_source[0] {
                0
            } else if u < p_source[0] + p_source[1] {
                1
            } else {
                2
            };
            let treated = rng.next_f64() < 0.5;
            let y = 1.0
                + 0.4 * (s as f64)
                + if treated { tau_s[s] } else { 0.0 }
                + rng.normal(0.0, 0.5);
            let arm = usize::from(treated);
            sums[s][arm] += y;
            counts[s][arm] += 1;
            if treated {
                treated_sum += y;
                n_treated += 1;
            } else {
                control_sum += y;
                n_control += 1;
            }
        }
        let transported: f64 = (0..3)
            .map(|s| {
                let diff = sums[s][1] / counts[s][1].max(1) as f64
                    - sums[s][0] / counts[s][0].max(1) as f64;
                p_target[s] * diff
            })
            .sum();
        let source_average =
            treated_sum / n_treated.max(1) as f64 - control_sum / n_control.max(1) as f64;
        let deployed_target_effect = if cybernetic_shielded {
            transported
        } else {
            source_average
        };
        let pearl_transportability_error = (deployed_target_effect - true_target_effect).abs();

        let identities_closed =
            friston_vfe_reduction >= 0.0 && (pearl_true_do_effect - 2.80).abs() < 1e-12;
        let claims: Vec<NamedCheck> = vec![
            (
                "stocks.a2.rung1_bias_above_1",
                pearl_naive_rung1_associative_bias > 1.0,
            ),
            (
                "stocks.a2.tuned_cfa_gain_above_5",
                powell_cfa_tuned_vs_myopic_gain > 5.0,
            ),
            (
                "stocks.a2.deployed_agent_matches_simulated_voi_in_both_regimes",
                choices_match,
            ),
            (
                "stocks.a2.deployed_causal_effect_within_0.15",
                (pearl_front_door_identified_effect - pearl_true_do_effect).abs() < 0.15,
            ),
            (
                "stocks.a2.deployed_transport_error_below_0.05",
                pearl_transportability_error < 0.05,
            ),
        ];
        let axis2_verified = identities_closed && claims.iter().all(|c| c.1);

        Self {
            powell_five_regime_winners,
            powell_cfa_tuned_theta: theta_star,
            powell_cfa_tuned_vs_myopic_gain,
            powell_cfa_deployed_gain,
            friston_vfe_reduction,
            voi_probe_vs_act_now_loss_ambiguous: voi_pairs[0],
            voi_probe_vs_act_now_loss_resolved: voi_pairs[1],
            deployed_policy_choices_match_voi: choices_match,
            pearl_true_do_effect,
            pearl_naive_rung1_associative_bias,
            pearl_front_door_identified_effect,
            pearl_counterfactual_cde_and_nde,
            pearl_transportability_error,
            claims,
            identities_closed,
            axis2_verified,
        }
    }
}
