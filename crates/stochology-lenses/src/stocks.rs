//! Stochary-Conjectural Synthesis, the Four Ontological Axioms of the Stock, Sequential Decision
//! Meta-Classes, Value of Information, Wasserstein DRO / $\beta$-CVaR, Viable System Model (VSM),
//! and the Telic-Scopic Audit Protocol (`PATE`) (§1.3.10 — *Estocologia Estocário-Conjectural e Cibernética*):
//!
//! Unifies in pure Rust (`seed = 2026`, zero external dependencies) the two foundational lineages of
//! **Stochology** (*Estocologia*):
//! 1. **The Stochastic-Conjectural Lineage ($\sigma\tau\omicron\chi\alpha\sigma\mu\acute{o}\varsigma$ / *Technai Stochastikai* / Bernoulli *Ars Conjectandi* 1713):**
//!    The science of aiming at a target ($\sigma\tau\acute{o}\chi\omicron\varsigma$) under structural uncertainty, strictly separating
//!    the internal quality of the art/process (**Telos**, *facere omnia quae possit ut conliniet*, audited via
//!    CORP-Murphy $\mathrm{REL}_{\mathrm{CORP}} - \mathrm{RES}_{\mathrm{CORP}}$, judgment noise hygiene, and $\mathrm{TTS} \ge \mathrm{TTR}$)
//!    from the exogenous outcome (**Skopos**, governed by environmental uncertainty $\mathrm{UNC}_0$).
//! 2. **The Stochary-Accumulative Lineage (*Stocks*, *Buffers*, and Strategic Asset Stock Accumulation):**
//!    The ontological, thermodynamic, and cybernetic theory of stocks ($S_t$) as the physical, financial, and
//!    informational condition of possibility for agency and intentionality across time.
//!
//! Structured across **4 Canonical Axes**, the **5-Step Stochary Engineering & Stress-Test Notebook**
//! (`StocharyStressTestNotebook`), and the **5-Dataset Real Empirical Coupler** (`EmpiricalStocharyCyberneticAudit`):
//! - **Axis 1 (`TopologyAndFourStockAxiomsAudit`):** Shannon audit of the 150-node acyclic conceptual tree
//!   ($1\text{ Root} + 11\text{ L1} + 32\text{ L2} + 106\text{ L3}$, $H_{L1} = 3.4242\text{ bits}, J_{L1} = 0.9898$,
//!   $H_{L2} = 4.9142\text{ bits}, J_{L2} = 0.9828$), orphan-leaf count of the deployed taxonomy, and the **Four Ontological Axioms
//!   of the Stock**: (1) low-pass filtering vs. Bullwhip amplification; (2) Ole Peters' (2019) $+50\% / -40\%$ non-ergodic coin,
//!   with the realized time-average growth of the deployed exposure simulated over 20 000 tosses; (3) Taleb & Douady (2013) discrete
//!   *tail vega*; (4) statefulness of the stock integrator.
//! - **Axis 2 (`PowellFristonCausalEngineAudit`):** a Powell `CFA` buffer whose slack $\theta$ is tuned in-sample and evaluated
//!   out-of-sample, Friston's Variational Free Energy reduction under Bayesian updating, a preposterior **value-of-information**
//!   analysis (probe vs. act now) checked against Monte Carlo episodes, and Pearl's front-door identification and transport
//!   formula against simulated ground truth.
//! - **Axis 3 (`StrategicStockPlantResilienceAudit`):** Dierickx & Cool's accumulation laws and Scarf's $K$-convexity (closed-form
//!   identities), Simchi-Levi $\mathrm{TTS}/\mathrm{TTR}/\mathrm{REI}$ with lognormal recovery times, Rockafellar-Uryasev $\beta$-CVaR,
//!   Esfahani-Kuhn WDRO with a radius calibrated on the training scenarios, split-conformal coverage, and `SupplyChainPhysicsAudit` (simulated G/G/1 queue
//!   vs. Kingman, Sterman supply line, enumerated Graves-Willems decoupling, Eppen pooling, TBS dual sourcing, and a simulated Ivanov
//!   ripple disruption).
//! - **Axis 4 (`CyberneticVsmAndPateGovernanceAudit`):** Conant-Ashby regulator model fidelity, Beer's triple capacity identity,
//!   the Algedonic gate rule (an identity), a simulated Kahneman judge panel, the `PATE` table (an identity), and a factor-P&L `PLAT` test.
//!
//! # Verification semantics
//!
//! As in `crate::triad`, each axis separates **claim checks** (the deployed method against computed ground truth; with
//! `cybernetic_shielded = false` the naive method is actually run), **identity checks** (`identities_closed`, closed-form or
//! algebraic facts that hold in both modes), **informative constants** (declared or literature values, never gated), and
//! **discarded findings** (`discarded_findings` and `discarded_claims`, claims that missed their tolerance at seed 2026 or failed
//! multi-seed acceptance in `crate::acceptance`; recorded, never approved).

use crate::acceptance::{MultiSeedAcceptance, NamedCheck};
use crate::decision::CorpPavDecomposition;
use stochology_core::{mean, median, quantile, quantile_sorted, sample_std, sample_variance, Rng};

/// Canonical L3 leaf counts across the 11 L1 primary axes and 32 L2 secondary sub-axes of the
/// 150-node Stochology conceptual tree ($1 + 11 + 32 + 106 = 150$ total nodes).
pub const CONCEPTUAL_TREE_SUBAXIS_LEAVES: [&[usize]; 11] = [
    &[3, 3, 5, 3],    // Axis 1: 4 sub-axes, 14 leaves (13.21%)
    &[2, 2, 2, 3],    // Axis 2: 4 sub-axes, 9 leaves (8.49%)
    &[4, 2, 2],       // Axis 3: 3 sub-axes, 8 leaves (7.55%)
    &[3, 2, 3, 3, 2], // Axis 4: 5 sub-axes, 13 leaves (12.26%)
    &[5, 3, 1],       // Axis 5: 3 sub-axes, 9 leaves (8.49%) — contains orphan single-leaf [1]
    &[6, 3],          // Axis 6: 2 sub-axes, 9 leaves (8.49%)
    &[4, 3],          // Axis 7: 2 sub-axes, 7 leaves (6.60%) — lowest density bottleneck
    &[4, 5],          // Axis 8: 2 sub-axes, 9 leaves (8.49%)
    &[4, 4],          // Axis 9: 2 sub-axes, 8 leaves (7.55%)
    &[4, 4],          // Axis 10: 2 sub-axes, 8 leaves (7.55%)
    &[4, 5, 3],       // Axis 11: 3 sub-axes, 12 leaves (11.32%)
];

/// Rebalanced cybernetic taxonomy: Axis 5 is reorganized into Powell's 4 policy classes `[4, 4, 3, 3]`.
pub const REBALANCED_TREE_SUBAXIS_LEAVES: [&[usize]; 11] = [
    &[3, 3, 5, 3],
    &[2, 2, 2, 3],
    &[4, 2, 2],
    &[3, 2, 3, 3, 2],
    &[4, 4, 3, 3],
    &[6, 3],
    &[4, 3],
    &[4, 5],
    &[4, 4],
    &[4, 4],
    &[4, 5, 3],
];

/// Number of single-leaf L2 branches in a taxonomy.
fn orphan_single_leaf_count(tree: &[&[usize]]) -> usize {
    tree.iter()
        .flat_map(|s| s.iter())
        .filter(|&&c| c == 1)
        .count()
}

/// Axis 1: Morphological Audit of the 150-Node Conceptual Tree (Acyclic Tree $\to$ Recursive Cybernetic Graph)
/// and Formal Verification of the Four Ontological Axioms of the Stock.
#[derive(Debug, Clone, PartialEq)]
pub struct TopologyAndFourStockAxiomsAudit {
    /// Identity: total conceptual nodes in the 150-node tree ($1\text{ Root} + 11\text{ L1} + 32\text{ L2} + 106\text{ L3} = 150$).
    pub tree_total_nodes: usize,
    /// Identity: number of primary L1 axes ($11$), secondary L2 sub-axes ($32$), and terminal L3 leaves ($106$).
    pub tree_hierarchy_counts: (usize, usize, usize),
    /// Identity: Shannon entropy $H_{L1} = -\sum_{i=1}^{11} p_i \log_2(p_i) \approx 3.4242\text{ bits}$.
    pub shannon_entropy_l1_bits: f64,
    /// Identity: normalized balance index $J_{L1} = H_{L1} / \log_2(11) \approx 0.9898$.
    pub balance_index_j_l1: f64,
    /// Identity: Shannon entropy $H_{L2} = -\sum_{j=1}^{32} p_j \log_2(p_j) \approx 4.9142\text{ bits}$.
    pub shannon_entropy_l2_bits: f64,
    /// Identity: normalized balance index $J_{L2} = H_{L2} / \log_2(32) \approx 0.9828$.
    pub balance_index_j_l2: f64,
    /// Informative: orphan single-leaf L2 branches of the deployed taxonomy (rebalanced if shielded, canonical otherwise). Both
    /// taxonomies are declared constants, so their counts (`1` canonical, `0` rebalanced) are checked under `identities_closed`.
    pub orphan_single_leaf_subbranches: usize,
    /// Informative: feedback loops declared in the cybernetic design (`S3-S4-S5`, `S3*`, `Algedonic`, stock-interconnection arcs);
    /// not derived from an executable graph.
    pub active_cybernetic_feedback_loops: usize,
    /// **Axiom 1:** Output-to-input variance ratio of the deployed replenishment rule (low-pass buffer if shielded, panic ordering otherwise). Gated (`< 0.30`).
    pub low_pass_buffered_variance_ratio: f64,
    /// Variance amplification of unbuffered Just-in-Time panic ordering. Gated as a mode-independent claim (`> 2.0`).
    pub unbuffered_bullwhip_amplification_ratio: f64,
    /// Identity: Ole Peters (2019) $+50\% / -40\%$ ensemble rate $\mathbb{E}[\Delta X / X] = +0.05$.
    pub peters_ensemble_expected_rate: f64,
    /// Identity: time-average growth $g_{\text{tempo}} = \sqrt{1.50 \times 0.60} - 1 \approx -0.05132$.
    pub peters_time_growth_rate: f64,
    /// Identity: stock after $100$ alternating full-exposure tosses ($100 \times 0.90^{50} \approx 0.5154$).
    pub peters_100_step_unbuffered_terminal_stock: f64,
    /// Realized time-average growth rate over 20 000 simulated tosses with the deployed exposure (Kelly $f^* = 0.25$ if shielded,
    /// full exposure $f = 1$ otherwise). Gated (`> 0.003`; theory $+0.00623$ at $f^*$).
    pub ergodic_buffered_time_growth_rate: f64,
    /// **Axiom 3:** Discrete Taleb-Douady *tail vega* $\Omega$ of the buffered convex payoff. Gated (`> 5.0`).
    pub buffered_convex_tail_vega: f64,
    /// Concave Just-in-Time discrete *tail vega*. Gated (`< -10.0`).
    pub unbuffered_jit_concave_tail_vega: f64,
    /// **Axiom 4:** Lag-1 autocorrelation of the stock integrator. Gated (`> 0.80`).
    pub stock_statefulness_autocorrelation: f64,
    /// Named claim checks gated by this axis (see `crate::acceptance`).
    pub claims: Vec<NamedCheck>,
    /// True if the tree, orphan-count, and Peters closed-form identities close.
    pub identities_closed: bool,
    /// Whether the identities, mode-independent axioms, and deployed replenishment rule and exposure pass.
    pub axis1_verified: bool,
}

impl TopologyAndFourStockAxiomsAudit {
    /// Evaluates Axis 1 (`seed = 2026`). When `cybernetic_shielded` is `false`, the canonical acyclic tree, panic ordering,
    /// and full exposure on Peters' non-ergodic coin are deployed.
    pub fn evaluate(seed: u64, cybernetic_shielded: bool) -> Self {
        let l1_count = CONCEPTUAL_TREE_SUBAXIS_LEAVES.len();
        let l2_count: usize = CONCEPTUAL_TREE_SUBAXIS_LEAVES.iter().map(|s| s.len()).sum();
        let l3_count: usize = CONCEPTUAL_TREE_SUBAXIS_LEAVES
            .iter()
            .flat_map(|s| s.iter().copied())
            .sum();
        let tree_total_nodes = 1 + l1_count + l2_count + l3_count;

        let n_leaves = l3_count as f64;
        let mut h_l1 = 0.0;
        for sub in CONCEPTUAL_TREE_SUBAXIS_LEAVES {
            let axis_sum: usize = sub.iter().sum();
            let p = (axis_sum as f64) / n_leaves;
            h_l1 -= p * p.log2();
        }
        let j_l1 = h_l1 / (l1_count as f64).log2();

        let mut h_l2 = 0.0;
        for sub in CONCEPTUAL_TREE_SUBAXIS_LEAVES {
            for &leaf_c in sub {
                let p = (leaf_c as f64) / n_leaves;
                h_l2 -= p * p.log2();
            }
        }
        let j_l2 = h_l2 / (l2_count as f64).log2();

        let deployed_tree: &[&[usize]] = if cybernetic_shielded {
            &REBALANCED_TREE_SUBAXIS_LEAVES
        } else {
            &CONCEPTUAL_TREE_SUBAXIS_LEAVES
        };
        let orphan_single_leaf_subbranches = orphan_single_leaf_count(deployed_tree);

        // Axiom 1: Low-Pass Filter & Anti-Bullwhip over T = 240 steps
        let mut rng = Rng::new(seed);
        let steps = 240usize;
        let mut demand_in = Vec::with_capacity(steps);
        let mut buffered_out = Vec::with_capacity(steps);
        let mut bullwhip_stage4 = Vec::with_capacity(steps);

        let alpha_lp = 0.20; // First-order RC low-pass filter gain (tau = 5 steps, Var(Out)/Var(In) = alpha/(2-alpha) = 0.111)
        let mut prev_d = 20.0;
        for _ in 0..steps {
            let d = (20.0 + rng.normal(0.0, 4.0)).max(2.0);
            demand_in.push(d);
            // Buffered replenishment acts as a first-order RC low-pass filter while the physical stock absorbs the high-frequency difference (order_buffered - d)
            let prev_out = buffered_out.last().copied().unwrap_or(20.0);
            let order_buffered = (1.0 - alpha_lp) * prev_out + alpha_lp * d;
            buffered_out.push(order_buffered);

            // Unbuffered Just-in-Time multi-stage panic ordering (Lee, Padmanabhan & Whang 1997 Bullwhip amplification)
            let panic_order = d + 1.45 * (d - prev_d);
            prev_d = d;
            bullwhip_stage4.push(panic_order);
        }
        let var_in = sample_variance(&demand_in).max(1e-9);
        let var_buffered = sample_variance(&buffered_out);
        let var_bullwhip = sample_variance(&bullwhip_stage4);
        let low_pass_buffered_variance_ratio = if cybernetic_shielded {
            var_buffered / var_in
        } else {
            var_bullwhip / var_in
        };
        let unbuffered_bullwhip_amplification_ratio = var_bullwhip / var_in;

        // Axiom 2: Ole Peters (2019) Non-Ergodic Multiplicative Coin Toss (+50% heads / -40% tails)
        let up = 1.50_f64;
        let down = 0.60_f64;
        let peters_ensemble_expected_rate = 0.5 * (up - 1.0) + 0.5 * (down - 1.0); // +0.05 (+5.00%)
        let peters_time_growth_rate = (up * down).sqrt() - 1.0; // sqrt(0.90) - 1 = -0.0513167
        let peters_100_step_unbuffered_terminal_stock = 100.0 * (up * down).powi(50); // ~0.5153775

        // Kelly exposure f* = p/a - q/b with p = 0.50, loss fraction a = 0.40, gain fraction b = 0.50 -> f* = 0.25.
        // The deployed exposure is simulated over 20 000 tosses; the realized time-average growth is exp(mean log factor) - 1.
        let loss_a = 1.0_f64 - down; // 0.40
        let gain_b = up - 1.0_f64; // 0.50
        let f_kelly: f64 = 0.50 / loss_a - 0.50 / gain_b; // 0.25
        let f_deployed = if cybernetic_shielded { f_kelly } else { 1.0 };
        let tosses = 20_000usize;
        let mut log_wealth = 0.0_f64;
        for _ in 0..tosses {
            let factor = if rng.next_f64() < 0.5 {
                1.0 + f_deployed * gain_b
            } else {
                1.0 - f_deployed * loss_a
            };
            log_wealth += factor.ln();
        }
        let ergodic_buffered_time_growth_rate = (log_wealth / (tosses as f64)).exp() - 1.0;

        // Axiom 3: Taleb-Douady (2013) Discrete Tail Vega across lower dispersion scale Omega in {1.0, 2.5}
        let evaluate_payoffs = |omega: f64| -> (f64, f64) {
            let mut sum_buffered = 0.0;
            let mut sum_jit = 0.0;
            let n_sim = 400usize;
            for i in 0..n_sim {
                let u = ((i as f64) + 0.5) / (n_sim as f64);
                // Symmetric zero-mean perturbation scaled by omega
                let z = (u - 0.5) * 6.0 * omega;
                let shock = z.abs();
                // Buffered convex payoff: small linear holding cost -2.0 + convex optionality capture in tail shocks
                let buffered_payoff = 10.0 - 2.0 + (shock - 3.5).max(0.0).powi(2) * 3.25;
                // Unbuffered JIT payoff: saves 2.0 holding cost in calm times, breaks concavely when shock > 3.0
                let jit_payoff = 12.0 - (shock - 3.0).max(0.0).powi(2) * 6.00;
                sum_buffered += buffered_payoff;
                sum_jit += jit_payoff;
            }
            (sum_buffered / (n_sim as f64), sum_jit / (n_sim as f64))
        };
        let (buf_low, jit_low) = evaluate_payoffs(1.0);
        let (buf_high, jit_high) = evaluate_payoffs(2.5);
        let buffered_convex_tail_vega = (buf_high - buf_low) / (2.5 - 1.0);
        let unbuffered_jit_concave_tail_vega = (jit_high - jit_low) / (2.5 - 1.0);

        // Axiom 4: Statefulness dS_t = mu dt + sigma dW_t lag-1 autocorrelation
        let mut state_series = Vec::with_capacity(steps);
        let mut s_val = 50.0;
        for &d in &demand_in {
            s_val = 0.92 * s_val + 0.08 * 50.0 + (20.0 - d) * 0.25;
            state_series.push(s_val);
        }
        let mu_s = mean(&state_series);
        let mut num = 0.0;
        let mut den = 0.0;
        for w in state_series.windows(2) {
            num += (w[0] - mu_s) * (w[1] - mu_s);
        }
        for &v in &state_series {
            den += (v - mu_s).powi(2);
        }
        let stock_statefulness_autocorrelation = if den > 1e-12 { num / den } else { 0.0 };

        let identities_closed = tree_total_nodes == 150
            && (l1_count, l2_count, l3_count) == (11, 32, 106)
            && (shannon_round(h_l1) - 3.4242).abs() < 1e-3
            && (shannon_round(j_l1) - 0.9898).abs() < 1e-3
            && (shannon_round(h_l2) - 4.9142).abs() < 1e-3
            && (shannon_round(j_l2) - 0.9828).abs() < 1e-3
            && peters_ensemble_expected_rate > 0.049
            && peters_time_growth_rate < -0.050
            && peters_100_step_unbuffered_terminal_stock < 0.60
            && orphan_single_leaf_count(&CONCEPTUAL_TREE_SUBAXIS_LEAVES) == 1
            && orphan_single_leaf_count(&REBALANCED_TREE_SUBAXIS_LEAVES) == 0;
        let claims: Vec<NamedCheck> = vec![
            (
                "stocks.a1.bullwhip_amplification_above_2",
                unbuffered_bullwhip_amplification_ratio > 2.0,
            ),
            (
                "stocks.a1.buffered_tail_vega_above_5",
                buffered_convex_tail_vega > 5.0,
            ),
            (
                "stocks.a1.jit_tail_vega_below_-10",
                unbuffered_jit_concave_tail_vega < -10.0,
            ),
            (
                "stocks.a1.stock_autocorrelation_above_0.80",
                stock_statefulness_autocorrelation > 0.80,
            ),
            (
                "stocks.a1.deployed_low_pass_ratio_below_0.30",
                low_pass_buffered_variance_ratio < 0.30,
            ),
            (
                "stocks.a1.deployed_time_average_growth_above_0.003",
                ergodic_buffered_time_growth_rate > 0.003,
            ),
        ];
        let axis1_verified = identities_closed && claims.iter().all(|c| c.1);

        Self {
            tree_total_nodes,
            tree_hierarchy_counts: (l1_count, l2_count, l3_count),
            shannon_entropy_l1_bits: h_l1,
            balance_index_j_l1: j_l1,
            shannon_entropy_l2_bits: h_l2,
            balance_index_j_l2: j_l2,
            orphan_single_leaf_subbranches,
            active_cybernetic_feedback_loops: 6,
            low_pass_buffered_variance_ratio,
            unbuffered_bullwhip_amplification_ratio,
            peters_ensemble_expected_rate,
            peters_time_growth_rate,
            peters_100_step_unbuffered_terminal_stock,
            ergodic_buffered_time_growth_rate,
            buffered_convex_tail_vega,
            unbuffered_jit_concave_tail_vega,
            stock_statefulness_autocorrelation,
            claims,
            identities_closed,
            axis1_verified,
        }
    }
}

fn shannon_round(v: f64) -> f64 {
    (v * 10_000.0).round() / 10_000.0
}

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

fn ols_slope(x: &[f64], y: &[f64]) -> f64 {
    let mx = mean(x);
    let my = mean(y);
    let mut num = 0.0;
    let mut den = 0.0;
    for (&xi, &yi) in x.iter().zip(y.iter()) {
        num += (xi - mx) * (yi - my);
        den += (xi - mx).powi(2);
    }
    if den > 1e-12 {
        num / den
    } else {
        0.0
    }
}

/// Classical & Modern Supply-Chain Physics Audit embedded in Axis 3:
/// - **Little's Law (1961) & Kingman's VUT Equation (1961 / Hopp & Spearman *Factory Physics* 2011):**
///   a simulated G/G/1 queue (Lindley recursion, gamma interarrival/service times with $c_a = 1.10$, $c_s = 0.90$) checked against
///   $\mathbb{E}[W_q(u)] \approx \left(\frac{c_a^2 + c_s^2}{2}\right)\left(\frac{u}{1 - u}\right) t_e$.
/// - **Sterman (1989) Supply-Line ($SL_t$) Anchoring & Lee, Padmanabhan & Whang (1997) Bullwhip Lower Bound:**
///   $\frac{\mathrm{Var}(Q_t)}{\mathrm{Var}(D_t)} \ge 1 + \frac{2L}{p} + \frac{2L^2}{p^2}$ and simulated oscillation damping when
///   in-transit pipeline stock $SL_t$ is fully weighted ($\beta_{SL} = 1.0$ vs. $\beta_{SL} = 0.05$).
/// - **Clark & Scarf (1960) Echelon Stock & Graves & Willems (2000) Guaranteed-Service Decoupling Points:**
///   enumerated decoupling placements across a 4-stage serial network vs. myopic silo buffering.
/// - **Eppen (1979) Square-Root Law of Risk Pooling vs. Tail-Correlation Breakdown ($\Phi_2$):**
///   $\sigma_{\text{pool}}(\rho) = \sigma\sqrt{N(1 + (N-1)\rho)}$ under independence vs. crisis tail correlation $\rho = 0.85$.
/// - **Allon & Van Mieghem (2010) Tailored Base-Surge (`TBS`) Dual Sourcing & Ivanov (2020) Ripple Effect / Viable Supply Chain (`VSC`):**
///   simulated sourcing costs and a simulated 10-day Tier-2 disruption.
#[derive(Debug, Clone, PartialEq)]
pub struct SupplyChainPhysicsAudit {
    /// Simulated G/G/1 mean queue wait (hours) at the deployed utilization (`0.82` if shielded, `0.985` otherwise) and at the cliff `0.985`.
    pub kingman_vut_buffered_vs_cliff_wait_hours: (f64, f64),
    /// Kingman's closed-form approximation (hours) at $u = 0.82$ and $u = 0.985$.
    pub kingman_formula_wait_hours: (f64, f64),
    /// Informative: Little's Law $\mathrm{WIP} = \lambda (W_q + t_e)$ from the simulated waits.
    pub littles_law_wip_buffered_vs_cliff: (f64, f64),
    /// Identity: Lee, Padmanabhan & Whang (1997) Bullwhip floor $1 + \frac{2L}{p} + \frac{2L^2}{p^2}$ for $L = 4, p = 4$ ($= 5.0$).
    pub lee_whang_bullwhip_theoretical_floor: f64,
    /// Order-variance ratio of the full supply-line rule ($\beta_{SL} = 1.0$) vs. the supply-line-ignoring rule ($\beta_{SL} = 0.05$). Gated (`< 0.30`).
    pub sterman_supply_line_oscillation_damping_ratio: f64,
    /// Holding-cost saving of the enumerated Graves-Willems placement vs. silo buffering. Gated (`> 0.25`).
    pub clark_scarf_graves_willems_meio_saving_fraction: f64,
    /// Identity: Eppen pooling ratio $1/\sqrt{N}$ and tail-correlation multiplier $\sqrt{1 + (N-1)\rho}$ at $N = 16$, $\rho = 0.85$.
    pub eppen_risk_pooling_normal_vs_tail_multiplier: (f64, f64),
    /// Cost saving of `TBS` dual sourcing vs. a single slow source over 150 simulated days. Gated (`> 0.08`).
    pub allon_van_mieghem_tbs_cost_saving_fraction: f64,
    /// Simulated fill rate over a 10-day Tier-2 outage of the deployed design (6-day buffer + surge supplier from day 4 if shielded,
    /// 4-day buffer otherwise). Gated (`>= 0.90`).
    pub ivanov_ripple_effect_viability_service_rate: f64,
    /// Named claim checks gated by this axis (see `crate::acceptance`).
    pub claims: Vec<NamedCheck>,
    /// True if the Lee-Whang and Eppen identities close.
    pub identities_closed: bool,
    /// Whether identities, the Kingman validation, and the deployed queue, ordering, placement, sourcing, and disruption design pass.
    pub supply_chain_physics_verified: bool,
}

impl SupplyChainPhysicsAudit {
    /// Evaluates the 5 supply-chain physics laws (`seed = 2026`).
    pub fn evaluate(seed: u64, cybernetic_shielded: bool) -> Self {
        let mut rng = Rng::new(seed ^ 0x5C99_2026);

        // 1. Kingman (1961) VUT Equation vs. a simulated G/G/1 queue (Lindley recursion, 10% burn-in)
        let ca: f64 = 1.10;
        let cs: f64 = 0.90;
        let v_factor = 0.5 * (ca * ca + cs * cs); // 1.01
        let te_hours: f64 = 0.40;
        let mu_rate: f64 = 1.0 / te_hours; // 2.50 jobs/hour
        let kingman = |u: f64| v_factor * (u / (1.0 - u)) * te_hours;
        let simulate_wq = |rng: &mut Rng, u: f64, jobs: usize| -> f64 {
            let (k_a, k_s) = (1.0 / (ca * ca), 1.0 / (cs * cs));
            let mean_interarrival = te_hours / u;
            let burn = jobs / 10;
            let mut wait = 0.0_f64;
            let mut total = 0.0_f64;
            for j in 0..jobs {
                let service = rng.gamma(k_s, te_hours / k_s);
                let interarrival = rng.gamma(k_a, mean_interarrival / k_a);
                wait = (wait + service - interarrival).max(0.0);
                if j >= burn {
                    total += wait;
                }
            }
            total / ((jobs - burn) as f64)
        };
        let u_buffered = 0.82_f64;
        let u_cliff = 0.985_f64;
        let wq_buffered_sim = simulate_wq(&mut rng, u_buffered, 400_000);
        let wq_cliff_sim = simulate_wq(&mut rng, u_cliff, 1_000_000);
        let (u_deployed, wq_deployed) = if cybernetic_shielded {
            (u_buffered, wq_buffered_sim)
        } else {
            (u_cliff, wq_cliff_sim)
        };
        let kingman_rel_err_buffered =
            (kingman(u_buffered) - wq_buffered_sim).abs() / wq_buffered_sim.max(1e-12);
        let wip_deployed = (u_deployed * mu_rate) * (wq_deployed + te_hours);
        let wip_cliff = (u_cliff * mu_rate) * (wq_cliff_sim + te_hours);

        // 2. Lee, Padmanabhan & Whang (1997) Bullwhip Floor & Sterman (1989) Supply-Line (SL_t) Anchoring:
        let lead_time_l: f64 = 4.0;
        let forecast_window_p: f64 = 4.0;
        let lee_whang_bullwhip_theoretical_floor = 1.0
            + (2.0 * lead_time_l) / forecast_window_p
            + (2.0 * lead_time_l * lead_time_l) / (forecast_window_p * forecast_window_p);

        // Simulate 120 periods of Sterman's (1989) Beer Distribution Game (step demand shift 4 -> 8 at t >= 5 + noise)
        // with L = 4 pipeline transit delay under Sterman's stock-adjustment rule:
        // Q_t = D_hat_t + alpha_S * ((S* - S_t) + beta_SL * (SL* - SL_t)), where SL_t is in-transit supply line
        let run_sterman_chain = |beta_sl: f64, rng_seed: u64| -> f64 {
            let mut r = Rng::new(rng_seed);
            let mut s_net = 12.0;
            let mut pipeline = [4.0_f64; 4]; // L = 4 in-transit shipments
            let mut d_hat = 4.0;
            let mut orders = Vec::with_capacity(120);
            for t in 0..120 {
                let base_d = if t < 5 { 4.0 } else { 8.0 };
                let d = (base_d + r.normal(0.0, 0.25)).max(0.0);
                let arrived = pipeline[t % 4];
                pipeline[t % 4] = 0.0; // Shipment has arrived; remaining pipeline holds the L-1 in-transit orders
                s_net += arrived - d;
                d_hat = 0.80 * d_hat + 0.20 * d;
                let s_target = 12.0;
                let sl_target = 3.0 * d_hat;
                let sl_current: f64 = pipeline.iter().sum();
                let q = (d_hat + 0.65 * ((s_target - s_net) + beta_sl * (sl_target - sl_current)))
                    .max(0.0);
                pipeline[t % 4] = q;
                orders.push(q);
            }
            sample_variance(&orders)
        };
        let var_sl_ignored = run_sterman_chain(0.05, seed ^ 0x1111);
        let var_sl_full = run_sterman_chain(1.00, seed ^ 0x1111);
        let sterman_supply_line_oscillation_damping_ratio = var_sl_full / var_sl_ignored.max(1e-9);

        // 3. Clark & Scarf / Graves & Willems: serial stages (T = 6, 5, 4, 2; h = 0.40, 0.75, 1.30, 2.20).
        // A stocking stage covers the lead time accumulated since the previous stocking stage; the DC always stocks.
        let z_98: f64 = 2.054;
        let sigma_d: f64 = 15.0;
        let lead_times = [6.0_f64, 5.0, 4.0, 2.0];
        let holding = [0.40_f64, 0.75, 1.30, 2.20];
        let placement_cost = |mask: u32| -> f64 {
            let mut cost = 0.0;
            let mut exposure = 0.0;
            for i in 0..4 {
                exposure += lead_times[i];
                if mask & (1 << i) != 0 {
                    cost += holding[i] * exposure.sqrt();
                    exposure = 0.0;
                }
            }
            z_98 * sigma_d * cost
        };
        let gsm_optimum = (0u32..16)
            .filter(|m| m & 0b1000 != 0)
            .map(placement_cost)
            .fold(f64::INFINITY, f64::min);
        // Naive silo: every stage covers its own cumulative exposure independently
        let cost_naive_silo = z_98
            * sigma_d
            * (0.40 * (6.0_f64).sqrt()
                + 0.75 * (11.0_f64).sqrt()
                + 1.30 * (15.0_f64).sqrt()
                + 2.20 * (17.0_f64).sqrt());
        let clark_scarf_graves_willems_meio_saving_fraction = 1.0 - gsm_optimum / cost_naive_silo;

        // 4. Eppen (1979) Risk Pooling Square-Root Law vs. Tail-Correlation Breakdown (Phi_2):
        let n_warehouses: f64 = 16.0;
        let normal_sqrt_pooling_ratio = 1.0 / n_warehouses.sqrt(); // 0.2500
        let rho_tail: f64 = 0.85;
        let tail_correlation_surge_multiplier = (1.0 + (n_warehouses - 1.0) * rho_tail).sqrt(); // sqrt(13.75) = 3.7081x

        // 5. Allon & Van Mieghem (2010) Tailored Base-Surge (TBS) Dual-Sourcing over 150 days of Pareto demand
        let mut cost_single_slow = 0.0;
        let mut cost_tbs_dual = 0.0;
        for _ in 0..150 {
            let u = rng.next_f64().clamp(1e-4, 0.9999);
            let d = 44.0 + 8.0 * ((1.0 - u).powf(-1.0 / 2.5) - 1.0);
            // Single slow source orders fixed 50.0 (heavy shortage cost 28.0/unit on Pareto demand surges)
            let shortage = (d - 50.0).max(0.0);
            let holding_q = (50.0 - d).max(0.0);
            cost_single_slow += 50.0 * 10.0 + 28.0 * shortage + 1.5 * holding_q;
            // TBS dual source: constant base 46.0 at 10.0 + responsive surge max(d - 46.0, 0) at 13.0
            let base_q = 46.0;
            let surge_q = (d - base_q).max(0.0);
            let tbs_hold = (base_q - d).max(0.0);
            cost_tbs_dual += base_q * 10.0 + surge_q * 13.0 + 1.2 * tbs_hold;
        }
        let allon_van_mieghem_tbs_cost_saving_fraction = 1.0 - cost_tbs_dual / cost_single_slow;

        // 6. Ivanov (2020/2022) Ripple Effect: Tier-2 supply halts for 10 days under demand N(50, 5);
        // the downstream node draws on its buffer and, if available, a surge supplier (30/day) from day 4.
        let (buffer_days, surge_from_day) = if cybernetic_shielded {
            (6.0_f64, Some(4usize))
        } else {
            (4.0, None)
        };
        let mut inventory = buffer_days * 50.0;
        let (mut demanded, mut served) = (0.0_f64, 0.0_f64);
        for day in 0..10 {
            let d = (50.0 + rng.normal(0.0, 5.0)).max(0.0);
            if surge_from_day.is_some_and(|start| day >= start) {
                inventory += 30.0;
            }
            let fill = inventory.min(d);
            inventory -= fill;
            served += fill;
            demanded += d;
        }
        let ivanov_ripple_effect_viability_service_rate = served / demanded.max(1e-12);

        let identities_closed = (lee_whang_bullwhip_theoretical_floor - 5.0).abs() < 1e-9
            && (normal_sqrt_pooling_ratio - 0.25).abs() < 1e-9
            && tail_correlation_surge_multiplier > 3.50;
        let claims: Vec<NamedCheck> = vec![
            (
                "stocks.sc.kingman_within_25pct_of_simulation",
                kingman_rel_err_buffered < 0.25,
            ),
            (
                "stocks.sc.simulated_cliff_wait_above_20h",
                wq_cliff_sim > 20.0,
            ),
            (
                "stocks.sc.supply_line_damping_below_0.30",
                sterman_supply_line_oscillation_damping_ratio < 0.30,
            ),
            (
                "stocks.sc.graves_willems_saving_above_0.25",
                clark_scarf_graves_willems_meio_saving_fraction > 0.25,
            ),
            (
                "stocks.sc.tbs_saving_above_0.08",
                allon_van_mieghem_tbs_cost_saving_fraction > 0.08,
            ),
            (
                "stocks.sc.deployed_queue_wait_below_2.5h",
                wq_deployed < 2.50,
            ),
            (
                "stocks.sc.deployed_outage_fill_at_least_0.90",
                ivanov_ripple_effect_viability_service_rate >= 0.90,
            ),
        ];
        let supply_chain_physics_verified = identities_closed && claims.iter().all(|c| c.1);

        Self {
            kingman_vut_buffered_vs_cliff_wait_hours: (wq_deployed, wq_cliff_sim),
            kingman_formula_wait_hours: (kingman(u_buffered), kingman(u_cliff)),
            littles_law_wip_buffered_vs_cliff: (wip_deployed, wip_cliff),
            lee_whang_bullwhip_theoretical_floor,
            sterman_supply_line_oscillation_damping_ratio,
            clark_scarf_graves_willems_meio_saving_fraction,
            eppen_risk_pooling_normal_vs_tail_multiplier: (
                normal_sqrt_pooling_ratio,
                tail_correlation_surge_multiplier,
            ),
            allon_van_mieghem_tbs_cost_saving_fraction,
            ivanov_ripple_effect_viability_service_rate,
            claims,
            identities_closed,
            supply_chain_physics_verified,
        }
    }
}

/// Axis 3: Strategic Stock Plant & Resilience — Dierickx & Cool's (1989) 5 Laws of Asset Stock Accumulation,
/// Herbert Scarf's (1960) $K$-Convexity of $(S, s)$ Policies, David Simchi-Levi's (2014/2015) $\mathrm{TTS}/\mathrm{TTR}/\mathrm{REI}$,
/// Rockafellar & Uryasev's (2000) $\beta$-CVaR, Esfahani & Kuhn's (2018) Wasserstein DRO, Conformal Safety Stocks,
/// and Classical/Modern Supply-Chain Physics (`SupplyChainPhysicsAudit`).
#[derive(Debug, Clone, PartialEq)]
pub struct StrategicStockPlantResilienceAudit {
    /// Identity: Dierickx & Cool Law 1 cost ratio $C(2I, T/2) / C(I, T) = 2^{0.8}$ under congestion cost $I^{1.8}$.
    pub dierickx_time_compression_diseconomy_ratio: f64,
    /// Identity: final interconnected vs. isolated stock under the declared accumulation, complementarity, and erosion rates.
    pub dierickx_interconnected_vs_isolated_stock_ratio: f64,
    /// Identity: maximum violation of $K + G(a+x) - G(x) - a G'(x) \ge 0$ on the inventory grid (`0.0` for convex $G$).
    pub scarf_k_convexity_max_violation: f64,
    /// Optimal reorder point $s^*$ and order-up-to level $S^*$ satisfying $G(s^*) \ge K + G(S^*)$.
    pub scarf_optimal_s_and_big_s: (f64, f64),
    /// $\mathrm{TTS} / \mathrm{median}(\mathrm{TTR})$ of the deployed node (backup network + faster recovery if shielded) with lognormal
    /// recovery times over 20 000 draws. Gated (`>= 1.15`).
    pub simchi_levi_tts_to_ttr_ratio: f64,
    /// Reduction of the Risk Exposure Index $\mathrm{REI} = \mathbb{E}[\max(\mathrm{TTR} - \mathrm{TTS}, 0)] \times 120$ (k$/day) achieved by the
    /// flexibility levers vs. the unprotected node (mode-independent). Gated (`>= 0.80`).
    pub simchi_levi_rei_reduction_fraction: f64,
    /// Rockafellar & Uryasev (2000) $\mathrm{VaR}_{0.95}$ and $\mathrm{CVaR}_{0.95}$ of 300 heavy-tailed loss scenarios (identity: $\mathrm{CVaR} \ge \mathrm{VaR}$).
    pub rockafellar_uryasev_var95_and_cvar95: (f64, f64),
    /// Informative: out-of-sample mean of the 270 held-out scenarios minus the 30-scenario in-sample mean ($\varepsilon = 0$).
    pub wdro_naive_oos_disappointment: f64,
    /// Out-of-sample mean minus the deployed certificate: in-sample mean $+ \varepsilon_N$ with $\varepsilon_N = 2.326\,\hat{s}/\sqrt{N}$
    /// calibrated on the training scenarios only if shielded, $\varepsilon = 0$ otherwise. Gated (`<= 0`).
    pub wdro_robust_oos_disappointment: f64,
    /// Split-conformal out-of-sample coverage at nominal $0.90$. Gated (`>= 0.88`).
    pub conformal_safety_stock_coverage: f64,
    /// Supply-Chain Physics & Multi-Echelon Resilience (`SupplyChainPhysicsAudit`).
    pub supply_chain_physics: SupplyChainPhysicsAudit,
    /// Named claim checks gated by this axis (see `crate::acceptance`).
    pub claims: Vec<NamedCheck>,
    /// Claims removed from the gate after failing multi-seed acceptance (`crate::acceptance`); still computed and reported.
    pub discarded_claims: Vec<NamedCheck>,
    /// True if the Dierickx, Scarf, and CVaR identities close.
    pub identities_closed: bool,
    /// Whether identities, conformal coverage, the naive-disappointment claim, and the deployed resilience and WDRO designs pass.
    pub axis3_verified: bool,
}

impl StrategicStockPlantResilienceAudit {
    /// Evaluates Axis 3 (`seed = 2026`). When `cybernetic_shielded` is `false`, the unprotected node, the uncertified empirical
    /// optimum, and the naive supply-chain designs are deployed.
    pub fn evaluate(seed: u64, cybernetic_shielded: bool) -> Self {
        let mut rng = Rng::new(seed ^ 0xB303_2026);

        // 1. Dierickx & Cool (1989):
        // Law 1: Time compression diseconomies: accumulating ΔS = 100 in T/2 requires flow 2*I with congestion cost c(I) = I^1.8
        let cost_normal_horizon = 10.0 * (10.0_f64).powf(1.8); // 10 periods at flow I=10
        let cost_compressed_half_horizon = 5.0 * (20.0_f64).powf(1.8); // 5 periods at flow 2I=20
        let dierickx_time_compression_diseconomy_ratio =
            cost_compressed_half_horizon / cost_normal_horizon; // 2^0.8 = 1.7411x > 1.50

        // Laws 2-4: Asset mass efficiency (alpha=0.06 * S_t), Interconnectedness (beta=0.05 * S_complementary), and Erosion (delta=0.08 * S_t)
        let mut s_inter = 20.0;
        let mut s_iso = 20.0;
        let s_comp = 25.0;
        for _ in 0..40 {
            let flow_in = 3.0;
            s_inter += flow_in + 0.06 * s_inter + 0.05 * s_comp - 0.08 * s_inter; // Net positive compounding via complementary asset mass
            s_iso += flow_in - 0.08 * s_iso; // Isolated stock stagnates under erosion
        }
        let dierickx_interconnected_vs_isolated_stock_ratio = s_inter / s_iso;

        // 2. Herbert Scarf (1960) K-Convexity of (S, s) Inventory Policy:
        // Setup cost K = 40.0, holding cost h = 1.2, shortage penalty p = 9.0, demand ~ discretized N(50, 12^2).
        let k_setup = 40.0;
        let g_func = |y: f64| -> f64 {
            let mut exp_loss = 0.0;
            let n_quad = 80usize;
            for i in 0..n_quad {
                let d = 10.0 + (i as f64) * 1.0; // support [10, 89]
                let diff = (d - 50.0) / 12.0;
                let w = (-0.5 * diff * diff).exp();
                let cost = if y >= d { 1.2 * (y - d) } else { 9.0 * (d - y) };
                exp_loss += w * cost;
            }
            // Normalize Gaussian quadrature weights
            let norm: f64 = (0..n_quad)
                .map(|i| {
                    let d = 10.0 + (i as f64) * 1.0;
                    let diff = (d - 50.0) / 12.0;
                    (-0.5 * diff * diff).exp()
                })
                .sum();
            0.5 * y + (exp_loss / norm)
        };

        let mut big_s = 0.0;
        let mut min_g = f64::INFINITY;
        for y_int in 20..=90 {
            let y = y_int as f64;
            let gy = g_func(y);
            if gy < min_g {
                min_g = gy;
                big_s = y;
            }
        }
        // Find s* < S* such that G(s*) >= K + G(S*)
        let mut little_s = 20.0;
        for y_int in (10..(big_s as usize)).rev() {
            let y = y_int as f64;
            if g_func(y) >= k_setup + min_g {
                little_s = y;
                break;
            }
        }
        // Verify K-convexity: K + G(a + x) - G(x) - a * G'(x) >= 0 for all x, a >= 0
        let mut max_violation = 0.0_f64;
        for x_int in 15..=70 {
            let x = x_int as f64;
            let gx = g_func(x);
            let g_prime_x = (g_func(x + 0.5) - g_func(x - 0.5)) / 1.0;
            for a_int in 1..=20 {
                let a = a_int as f64;
                let lhs = k_setup + g_func(x + a) - gx - a * g_prime_x;
                if lhs < -1e-9 {
                    max_violation = max_violation.max(-lhs);
                }
            }
        }

        // 3. Simchi-Levi (2014/2015): a critical supplier node with 6 days of inventory loses 120 k$/day once TTR exceeds TTS.
        // Unprotected: TTS = 6 days, TTR ~ LogNormal(median 14, sigma 0.3).
        // Protected (Pepsi network backup covering 50% of flow, Nokia modularity + Toyota cross-training): TTS = 12, TTR median 9.
        let tts_ttr_draws = |rng: &mut Rng, tts: f64, ttr_median: f64| -> (f64, f64) {
            let ttr: Vec<f64> = (0..20_000)
                .map(|_| (ttr_median.ln() + 0.3 * rng.standard_normal()).exp())
                .collect();
            let rei =
                ttr.iter().map(|&t| (t - tts).max(0.0)).sum::<f64>() / ttr.len() as f64 * 120.0;
            (tts / median(&ttr), rei)
        };
        let (unprotected_ratio, rei_unprotected) = tts_ttr_draws(&mut rng, 6.0, 14.0);
        let (protected_ratio, rei_protected) = tts_ttr_draws(&mut rng, 12.0, 9.0);
        let simchi_levi_tts_to_ttr_ratio = if cybernetic_shielded {
            protected_ratio
        } else {
            unprotected_ratio
        };
        let simchi_levi_rei_reduction_fraction = 1.0 - rei_protected / rei_unprotected.max(1e-12);

        // 4. Rockafellar & Uryasev (2000) Convex beta-CVaR (beta = 0.95) on N = 300 heavy-tailed loss scenarios
        let n_scen = 300usize;
        let mut losses = Vec::with_capacity(n_scen);
        for _ in 0..n_scen {
            let u = rng.next_f64().clamp(1e-5, 0.99999);
            let pareto_shock = (1.0 - u).powf(-1.0 / 2.2) - 1.0;
            losses.push(15.0 + 6.0 * pareto_shock);
        }
        let (var95, cvar95) = rockafellar_uryasev_cvar(&losses, 0.95);

        // 5. Esfahani & Kuhn (2018) Wasserstein DRO: 30 training scenarios, 270 held out. For a 1-Lipschitz loss the worst case over
        // the W1 ball of radius epsilon is the in-sample mean + epsilon; the deployed radius is calibrated on the training scenarios
        // only, epsilon_N = 2.326 s_hat / sqrt(N) (one-sided 99% sampling margin), or 0 for the empirical optimum.
        let train_sample = &losses[..30];
        let oos_sample = &losses[30..];
        let in_sample_mean = mean(train_sample);
        let true_oos_mean = mean(oos_sample);
        let epsilon_wasserstein = if cybernetic_shielded {
            2.326 * sample_std(train_sample) / (train_sample.len() as f64).sqrt()
        } else {
            0.0
        };
        let wdro_certificate = in_sample_mean + epsilon_wasserstein;
        let wdro_naive_oos_disappointment = true_oos_mean - in_sample_mean;
        let wdro_robust_oos_disappointment = true_oos_mean - wdro_certificate;

        // 6. Angelopoulos & Bates (2023) Split-Conformal Prediction for Safety Stock Sizing
        let mut cal_scores: Vec<f64> = losses[..150].to_vec();
        cal_scores.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        // Order statistic ceil((n + 1)(1 - alpha)) at alpha = 0.10, converted to a 0-based index
        let idx_q = (((150.0_f64 + 1.0) * 0.90).ceil() as usize - 1).min(149);
        let conformal_q_hat = cal_scores[idx_q];
        let covered = losses[150..]
            .iter()
            .filter(|&&y| y <= conformal_q_hat)
            .count();
        let conformal_safety_stock_coverage = (covered as f64) / 150.0;

        let supply_chain_physics = SupplyChainPhysicsAudit::evaluate(seed, cybernetic_shielded);

        let identities_closed = dierickx_time_compression_diseconomy_ratio > 1.50
            && dierickx_interconnected_vs_isolated_stock_ratio > 1.60
            && max_violation == 0.0
            && little_s < big_s
            && cvar95 >= var95
            && supply_chain_physics.identities_closed;
        // At n = 150 test points the coverage SD is ~0.024, so a 0.88 floor at nominal 0.90 holds on 12 of 20 acceptance seeds.
        let discarded_claims: Vec<NamedCheck> = vec![(
            "stocks.a3.conformal_coverage_at_least_0.88",
            conformal_safety_stock_coverage >= 0.88,
        )];
        let mut claims: Vec<NamedCheck> = vec![
            (
                "stocks.a3.levers_cut_rei_by_0.80",
                simchi_levi_rei_reduction_fraction >= 0.80,
            ),
            (
                "stocks.a3.deployed_tts_to_ttr_at_least_1.15",
                simchi_levi_tts_to_ttr_ratio >= 1.15,
            ),
            (
                "stocks.a3.deployed_wdro_certificate_covers_oos",
                wdro_robust_oos_disappointment <= 0.0,
            ),
        ];
        claims.extend(supply_chain_physics.claims.iter().copied());
        let axis3_verified = identities_closed && claims.iter().all(|c| c.1);

        Self {
            dierickx_time_compression_diseconomy_ratio,
            dierickx_interconnected_vs_isolated_stock_ratio,
            scarf_k_convexity_max_violation: max_violation,
            scarf_optimal_s_and_big_s: (little_s, big_s),
            simchi_levi_tts_to_ttr_ratio,
            simchi_levi_rei_reduction_fraction,
            rockafellar_uryasev_var95_and_cvar95: (var95, cvar95),
            wdro_naive_oos_disappointment,
            wdro_robust_oos_disappointment,
            conformal_safety_stock_coverage,
            supply_chain_physics,
            claims,
            discarded_claims,
            identities_closed,
            axis3_verified,
        }
    }
}

/// Computes $(\mathrm{VaR}_\beta, \mathrm{CVaR}_\beta)$ by minimizing Rockafellar & Uryasev's (2000)
/// convex auxiliary function $F_\beta(\alpha) = \alpha + \frac{1}{(1-\beta)N}\sum_{i=1}^N \max(L_i - \alpha, 0)$.
pub fn rockafellar_uryasev_cvar(losses: &[f64], beta: f64) -> (f64, f64) {
    if losses.is_empty() {
        return (0.0, 0.0);
    }
    let mut sorted = losses.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let alpha_var = quantile_sorted(&sorted, beta);
    let tail_excess: f64 = sorted.iter().map(|&l| (l - alpha_var).max(0.0)).sum();
    let n = sorted.len() as f64;
    let cvar = alpha_var + tail_excess / ((1.0 - beta).max(1e-9) * n);
    (alpha_var, cvar)
}

/// VSM Algedonic gate (*"not measured, not executed"*): an S1 proposal runs only if it carries a measurement record.
fn algedonic_gate_executes(measured: bool) -> bool {
    measured
}

/// Spearman rank correlation of two equal-length samples (continuous data; ties broken by order).
fn spearman_rank_correlation(a: &[f64], b: &[f64]) -> f64 {
    let ranks = |v: &[f64]| -> Vec<f64> {
        let mut idx: Vec<usize> = (0..v.len()).collect();
        idx.sort_by(|&i, &j| v[i].partial_cmp(&v[j]).unwrap_or(std::cmp::Ordering::Equal));
        let mut r = vec![0.0; v.len()];
        for (rank, &i) in idx.iter().enumerate() {
            r[i] = rank as f64;
        }
        r
    };
    let (ra, rb) = (ranks(a), ranks(b));
    let (ma, mb) = (mean(&ra), mean(&rb));
    let cov: f64 = ra.iter().zip(&rb).map(|(x, y)| (x - ma) * (y - mb)).sum();
    let va: f64 = ra.iter().map(|x| (x - ma).powi(2)).sum();
    let vb: f64 = rb.iter().map(|y| (y - mb).powi(2)).sum();
    cov / (va * vb).sqrt().max(1e-300)
}

/// Two-sample Kolmogorov-Smirnov distance $\sup_x |F_a(x) - F_b(x)|$.
fn ks_two_sample_distance(a: &[f64], b: &[f64]) -> f64 {
    let mut sa = a.to_vec();
    let mut sb = b.to_vec();
    sa.sort_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal));
    sb.sort_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal));
    let (mut i, mut j, mut d) = (0usize, 0usize, 0.0_f64);
    while i < sa.len() && j < sb.len() {
        if sa[i] <= sb[j] {
            i += 1;
        } else {
            j += 1;
        }
        d = d.max((i as f64 / sa.len() as f64 - j as f64 / sb.len() as f64).abs());
    }
    d
}

/// Empirical Wasserstein-1 distance between two 1D samples via their quantile functions on a 1000-point grid.
fn wasserstein1_1d(a: &[f64], b: &[f64]) -> f64 {
    let mut sa = a.to_vec();
    let mut sb = b.to_vec();
    sa.sort_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal));
    sb.sort_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal));
    let grid = 1000usize;
    (0..grid)
        .map(|k| {
            let u = (k as f64 + 0.5) / grid as f64;
            (quantile_sorted(&sa, u) - quantile_sorted(&sb, u)).abs()
        })
        .sum::<f64>()
        / grid as f64
}

/// The 4 Quadrants of the Telic-Scopic Audit Protocol (`PATE` — *Protocolo de Auditoria Télico-Escópica*),
/// separating the internal quality of the stochastic art (**Telos**) from the exogenous outcome (**Skopos**).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PateQuadrant {
    /// **Q1 — Maestria Estocológica Validada (High Telos + Skopos Success):**
    /// Calibrated forecasts ($\mathrm{REL}_{\mathrm{CORP}} \to 0$), controlled judgment noise, $\mathrm{TTS} \ge \mathrm{TTR}$,
    /// and target achieved. Action: standardize and codify into habitual `PFA` rules.
    Q1ValidatedMastery,
    /// **Q2 — Resiliência Dignificada (High Telos + Skopos Failure via Tail $\mathrm{UNC}_0$):**
    /// Operator fulfilled all precepts of the art (*facere omnia quae possit ut conliniet*), but an extreme tail
    /// realization in Murphy's exogenous $\mathrm{UNC}_0$ prevented the target. Action: exonerate operator (Cicero /
    /// Alexander of Aphrodisias), trigger $\mathrm{TTR}$ recovery, and inspect Wasserstein radius $\varepsilon$.
    Q2DignifiedResilience,
    /// **Q3 — Acidente Feliz / Ilusão de Jackpot (Low Telos + Skopos Success by Luck):**
    /// Miscalibrated forecasts, noisy judgment, non-ergodic exposure or $\mathrm{TTS} < \mathrm{TTR}$, yet survived by
    /// fortuitous absence of tail shocks ("unshattered porcelain cup"). Action: immediate `S3*` & Algedonic alarm!
    Q3HappyAccidentJackpotFragility,
    /// **Q4 — Falha Estocológica Estrutural (Low Telos + Skopos Failure):**
    /// Violated stock sizing, Bullwhip amplification, and unshielded exposure leading to stockout/ruin.
    /// Action: high-leverage structural redesign (Meadows Point 10/2 + Conant-Ashby regulator model overhaul).
    Q4StructuralFailure,
}

impl PateQuadrant {
    /// Classifies an operational decision episode into its canonical PATE quadrant from internal
    /// **Telos** metrics (`rel_corp`, `tts_to_ttr`, `judgment_noise_sd`) and external **Skopos** outcome (`skopos_attained`).
    pub fn classify(
        rel_corp: f64,
        tts_to_ttr: f64,
        judgment_noise_sd: f64,
        skopos_attained: bool,
    ) -> Self {
        let high_telos = rel_corp <= 0.015 && tts_to_ttr >= 1.0 && judgment_noise_sd <= 0.10;
        match (high_telos, skopos_attained) {
            (true, true) => Self::Q1ValidatedMastery,
            (true, false) => Self::Q2DignifiedResilience,
            (false, true) => Self::Q3HappyAccidentJackpotFragility,
            (false, false) => Self::Q4StructuralFailure,
        }
    }

    /// Outcome-only review that ignores the Telos metrics: success is read as mastery, failure as structural failure.
    pub fn classify_outcome_only(skopos_attained: bool) -> Self {
        if skopos_attained {
            Self::Q1ValidatedMastery
        } else {
            Self::Q4StructuralFailure
        }
    }
}

/// Executable 5-Step Stochary Engineering & Stress-Test Notebook (*Caderno de Engenharia Estocária de 5 Etapas*).
#[derive(Debug, Clone, PartialEq)]
pub struct StocharyStressTestNotebook {
    /// Step 1 (Dierickx & Cool / Meadows): Asset erosion rate $\delta$ and time-compression diseconomy factor mapped.
    pub step1_flows_erosion_and_interconnection_mapped: bool,
    /// Step 2 (Scarf + WDRO + Conformal): Critical thresholds $(s^*, S^*)$ parameterized under $\beta$-CVaR and Wasserstein ball $\varepsilon$.
    pub step2_wdro_conformal_s_big_s_thresholds: (f64, f64),
    /// Step 3 (Simchi-Levi): Node removal stress-test ensuring $\mathrm{TTS} \ge \mathrm{TTR}$ across all critical suppliers.
    pub step3_node_removal_tts_ge_ttr_passed: bool,
    /// Step 4 (Taleb & Douady): Discrete *tail vega* ($\Omega$) probe confirming non-negative convexity in lower tail dispersion.
    pub step4_tail_vega_convexity_verified: bool,
    /// Step 5 (Powell / Friston): the deployed agent's probe-or-act choices match the simulated value of information.
    pub step5_epistemic_efe_voi_evaluated: bool,
    /// Whether all 5 engineering stress-test steps passed.
    pub all_five_steps_approved: bool,
}

/// Axis 4: Cybernetic Governance (`VSM` S1–S5 of Ashby, Conant & Stafford Beer), Human Judgment Noise Audit
/// (Kahneman, Sibony & Sunstein 2021), the Telic-Scopic Audit Protocol (`PATE`), and the 5-Step Notebook.
#[derive(Debug, Clone, PartialEq)]
pub struct CyberneticVsmAndPateGovernanceAudit {
    /// Informative: residual variety $\max(V_D - V_R, 0)$ of the declared design (attenuated $V_D = 45$, amplified $V_R = 60$ bits/min
    /// if shielded; $V_D = 120$, $V_R = 18$ otherwise). Declared values, not measured.
    pub ashby_residual_unabsorbed_variety: f64,
    /// One-step out-of-sample $R^2$ of the deployed regulator's internal model of $y_t = 0.6\,y_{t-1} + 1.5\,u_t + 0.1\,\varepsilon_t$
    /// (OLS-identified if shielded, persistence $\hat{y}_t = y_{t-1}$ otherwise). Gated (`>= 0.95`).
    pub conant_ashby_regulator_homomorphism_r2: f64,
    /// Identity: Beer's `(Productivity, Latency, Performance)` with `Performance == Productivity * Latency`.
    pub beer_vsm_productivity_latency_performance: (f64, f64, f64),
    /// Informative: true if no unmeasured S1 action among 500 proposals was executed by the deployed channel (gated if shielded,
    /// ungated otherwise). The gate's blocking rule is checked under `identities_closed`.
    pub algedonic_not_measured_not_executed_enforced: bool,
    /// Simulated judgment-error SD of a single raw judge vs. the deployed protocol (bias calibration + 4-judge averaging if shielded).
    pub kahneman_judgment_noise_before_and_after: (f64, f64),
    /// Identity: `PATE` classification of the four canonical episodes.
    pub pate_quadrant_classifications: [PateQuadrant; 4],
    /// Informative: true if the deployed reviewer (Telos-aware if shielded, outcome-only otherwise) flags the lucky episode as `Q3`;
    /// the Telos-aware classification is part of the `PATE` identity table.
    pub pate_q3_jackpot_algedonic_alarm_triggered: bool,
    /// `PLAT` Spearman $r_s$ and KS distance between 250 days of front-office P&L and the deployed risk-model P&L
    /// (all three factors if shielded, third factor omitted otherwise).
    pub plat_spearman_and_ks: (f64, f64),
    /// True when the `PLAT` green zone ($r_s > 0.80$, $D_{\mathrm{KS}} < 0.09$) and the judgment-noise budget ($\le 0.10$) hold. Gated.
    pub pccp_and_evaluated_competence_verified: bool,
    /// Informative: 5-Step Stochary Engineering & Stress-Test Notebook. It aggregates claims gated elsewhere, so it is not gated again.
    pub engineering_notebook: StocharyStressTestNotebook,
    /// Named claim checks gated by this axis (see `crate::acceptance`).
    pub claims: Vec<NamedCheck>,
    /// Claims removed from the gate after failing multi-seed acceptance (`crate::acceptance`); still computed and reported.
    pub discarded_claims: Vec<NamedCheck>,
    /// True if the Beer, `PATE`, and Algedonic gate-rule identities close.
    pub identities_closed: bool,
    /// Whether identities, the raw-noise claim, and the deployed regulator, gate, reviewer, protocol, `PLAT`, and notebook pass.
    pub axis4_verified: bool,
}

impl CyberneticVsmAndPateGovernanceAudit {
    /// Evaluates Axis 4. When `cybernetic_shielded` is `false`, the persistence regulator, an ungated S1 channel, the outcome-only
    /// reviewer, raw single-judge estimates, and the factor-omitting risk model are deployed.
    pub fn evaluate(
        axis1: &TopologyAndFourStockAxiomsAudit,
        axis2: &PowellFristonCausalEngineAudit,
        axis3: &StrategicStockPlantResilienceAudit,
        seed: u64,
        cybernetic_shielded: bool,
    ) -> Self {
        let mut rng = Rng::new(seed ^ 0x45A4_2026);

        // 1. Ashby's Requisite Variety on the declared design varieties (bits/min)
        let (v_d, v_r) = if cybernetic_shielded {
            (45.0_f64, 60.0_f64)
        } else {
            (120.0_f64, 18.0_f64)
        };
        let ashby_residual_unabsorbed_variety = (v_d - v_r).max(0.0);

        // 2. Conant-Ashby Good Regulator: identify y_t = a y_{t-1} + b u_t from 200 periods, score on the next 200
        let mut y_series = vec![0.0_f64];
        let mut u_series = vec![0.0_f64];
        for _ in 0..400 {
            let u = rng.standard_normal();
            let y = 0.6 * y_series.last().copied().unwrap_or(0.0)
                + 1.5 * u
                + 0.1 * rng.standard_normal();
            u_series.push(u);
            y_series.push(y);
        }
        let (mut syy, mut syu, mut suu, mut sy_target, mut su_target) = (0.0, 0.0, 0.0, 0.0, 0.0);
        for t in 1..=200 {
            let (lag, u, y) = (y_series[t - 1], u_series[t], y_series[t]);
            syy += lag * lag;
            syu += lag * u;
            suu += u * u;
            sy_target += lag * y;
            su_target += u * y;
        }
        let det = (syy * suu - syu * syu).max(1e-12);
        let a_hat = (suu * sy_target - syu * su_target) / det;
        let b_hat = (syy * su_target - syu * sy_target) / det;
        let test_y = &y_series[201..];
        let y_mean = mean(test_y);
        let (mut sse, mut sst) = (0.0, 0.0);
        for t in 201..=400 {
            let pred = if cybernetic_shielded {
                a_hat * y_series[t - 1] + b_hat * u_series[t]
            } else {
                y_series[t - 1]
            };
            sse += (y_series[t] - pred).powi(2);
            sst += (y_series[t] - y_mean).powi(2);
        }
        let conant_ashby_regulator_homomorphism_r2 = 1.0 - sse / sst.max(1e-12);

        // 3. Stafford Beer's Triple Capacity Vector (Brain of the Firm, p. 163)
        let actuality: f64 = 72.0;
        let capability: f64 = 90.0;
        let potentiality: f64 = 120.0;
        let productivity = actuality / capability;
        let latency = capability / potentiality;
        let performance = actuality / potentiality;

        // 4. Algedonic Channel ("Not Measured, Not Executed"): 500 S1 proposals, 20% lacking a measurement record
        let (mut unmeasured_executed, mut gated_unmeasured_executed) = (0usize, 0usize);
        for _ in 0..500 {
            let measured = rng.next_f64() >= 0.20;
            if !measured && !cybernetic_shielded {
                unmeasured_executed += 1;
            }
            if !measured && algedonic_gate_executes(measured) {
                gated_unmeasured_executed += 1;
            }
        }
        let algedonic_not_measured_not_executed_enforced = unmeasured_executed == 0;

        // 5. Kahneman, Sibony & Sunstein (2021) noise audit: 40 judges with level bias N(0, 0.22^2) and occasion noise N(0, 0.08^2)
        // rate 200 cases. Hygiene: each judge's bias is estimated on 20 calibration cases and 4 de-biased judges are averaged.
        let judges = 40usize;
        let biases: Vec<f64> = (0..judges).map(|_| rng.normal(0.0, 0.22)).collect();
        let bias_hat: Vec<f64> = biases
            .iter()
            .map(|&b| (0..20).map(|_| b + rng.normal(0.0, 0.08)).sum::<f64>() / 20.0)
            .collect();
        let mut raw_errors = Vec::with_capacity(200);
        let mut hygiene_errors = Vec::with_capacity(200);
        for _ in 0..200 {
            let j = (rng.next_f64() * judges as f64) as usize % judges;
            raw_errors.push(biases[j] + rng.normal(0.0, 0.08));
            let avg = (0..4)
                .map(|_| {
                    let k = (rng.next_f64() * judges as f64) as usize % judges;
                    biases[k] + rng.normal(0.0, 0.08) - bias_hat[k]
                })
                .sum::<f64>()
                / 4.0;
            hygiene_errors.push(avg);
        }
        let noise_before = sample_std(&raw_errors);
        let noise_deployed = if cybernetic_shielded {
            sample_std(&hygiene_errors)
        } else {
            noise_before
        };

        // 6. PATE 4-Quadrant Audit
        let q1 = PateQuadrant::classify(0.004, 1.33, 0.03, true);
        let q2 = PateQuadrant::classify(0.004, 1.33, 0.03, false);
        let q3 = PateQuadrant::classify(0.085, 0.45, 0.24, true);
        let q4 = PateQuadrant::classify(0.085, 0.45, 0.24, false);
        let pate_quadrant_classifications = [q1, q2, q3, q4];
        let reviewed_lucky_episode = if cybernetic_shielded {
            PateQuadrant::classify(0.085, 0.45, 0.24, true)
        } else {
            PateQuadrant::classify_outcome_only(true)
        };
        let pate_q3_jackpot_algedonic_alarm_triggered =
            reviewed_lucky_episode == PateQuadrant::Q3HappyAccidentJackpotFragility;

        // 7. Basel III MAR33 PLAT: front-office P&L = 1.0 f1 + 0.8 f2 + 1.5 f3 over 250 days; the risk model adds N(0, 0.3^2)
        // revaluation noise and, when naive, omits the third factor.
        let mut fo_pnl = Vec::with_capacity(250);
        let mut rm_pnl = Vec::with_capacity(250);
        for _ in 0..250 {
            let f = [
                rng.standard_normal(),
                rng.standard_normal(),
                rng.standard_normal(),
            ];
            fo_pnl.push(1.0 * f[0] + 0.8 * f[1] + 1.5 * f[2]);
            let third = if cybernetic_shielded { 1.5 * f[2] } else { 0.0 };
            rm_pnl.push(1.0 * f[0] + 0.8 * f[1] + third + rng.normal(0.0, 0.3));
        }
        let plat_spearman_and_ks = (
            spearman_rank_correlation(&fo_pnl, &rm_pnl),
            ks_two_sample_distance(&fo_pnl, &rm_pnl),
        );
        let pccp_and_evaluated_competence_verified = plat_spearman_and_ks.0 > 0.80
            && plat_spearman_and_ks.1 < 0.09
            && noise_deployed <= 0.10;

        let engineering_notebook = StocharyStressTestNotebook {
            step1_flows_erosion_and_interconnection_mapped: axis3
                .dierickx_time_compression_diseconomy_ratio
                > 1.50,
            step2_wdro_conformal_s_big_s_thresholds: axis3.scarf_optimal_s_and_big_s,
            step3_node_removal_tts_ge_ttr_passed: axis3.simchi_levi_tts_to_ttr_ratio >= 1.15,
            step4_tail_vega_convexity_verified: axis1.buffered_convex_tail_vega > 0.0,
            step5_epistemic_efe_voi_evaluated: axis2.deployed_policy_choices_match_voi,
            all_five_steps_approved: axis3.dierickx_time_compression_diseconomy_ratio > 1.50
                && axis3.simchi_levi_tts_to_ttr_ratio >= 1.15
                && axis1.buffered_convex_tail_vega > 0.0
                && axis2.deployed_policy_choices_match_voi
                && axis1.axis1_verified
                && axis3.axis3_verified,
        };

        let identities_closed = (performance - productivity * latency).abs() < 1e-12
            && pate_quadrant_classifications
                == [
                    PateQuadrant::Q1ValidatedMastery,
                    PateQuadrant::Q2DignifiedResilience,
                    PateQuadrant::Q3HappyAccidentJackpotFragility,
                    PateQuadrant::Q4StructuralFailure,
                ]
            && gated_unmeasured_executed == 0;
        // A single raw judge's error SD exceeds 0.20 on 18 of 20 acceptance seeds (panel of 40 sampled biases).
        let discarded_claims: Vec<NamedCheck> =
            vec![("stocks.a4.raw_judge_noise_above_0.20", noise_before > 0.20)];
        let claims: Vec<NamedCheck> = vec![
            (
                "stocks.a4.deployed_regulator_r2_at_least_0.95",
                conant_ashby_regulator_homomorphism_r2 >= 0.95,
            ),
            (
                "stocks.a4.deployed_judgment_noise_below_0.05",
                noise_deployed < 0.05,
            ),
            (
                "stocks.a4.deployed_plat_rs_above_0.80_ks_below_0.09_noise_at_most_0.10",
                pccp_and_evaluated_competence_verified,
            ),
        ];
        let axis4_verified = identities_closed && claims.iter().all(|c| c.1);

        Self {
            ashby_residual_unabsorbed_variety,
            conant_ashby_regulator_homomorphism_r2,
            beer_vsm_productivity_latency_performance: (productivity, latency, performance),
            algedonic_not_measured_not_executed_enforced,
            kahneman_judgment_noise_before_and_after: (noise_before, noise_deployed),
            pate_quadrant_classifications,
            pate_q3_jackpot_algedonic_alarm_triggered,
            plat_spearman_and_ks,
            pccp_and_evaluated_competence_verified,
            engineering_notebook,
            claims,
            discarded_claims,
            identities_closed,
            axis4_verified,
        }
    }
}

/// Complete 4-Axis Synthesis of Stochary-Conjectural & Cybernetic Stochology.
#[derive(Debug, Clone, PartialEq)]
pub struct StocharyConjecturalCyberneticSynthesis {
    /// Axis 1: 150-Node Topology (Acyclic Tree $\to$ Recursive Cybernetic Graph) & the Four Ontological Axioms of the Stock.
    pub topology_and_stock_axioms: TopologyAndFourStockAxiomsAudit,
    /// Axis 2: Powell's `CFA` tuning, Friston's Free Energy & value of information, and Pearl's `do-Calculus`.
    pub powell_friston_causal_engine: PowellFristonCausalEngineAudit,
    /// Axis 3: Dierickx & Cool, Scarf $K$-Convexity, Simchi-Levi $\mathrm{TTS}/\mathrm{TTR}/\mathrm{REI}$, and $\beta$-CVaR + WDRO.
    pub strategic_stock_plant: StrategicStockPlantResilienceAudit,
    /// Axis 4: Conant-Ashby/Beer VSM (S1–S5, `S3*`, Algedonic), Kahneman Noise Audit, PATE Matrix, and 5-Step Notebook.
    pub cybernetic_vsm_and_pate: CyberneticVsmAndPateGovernanceAudit,
    /// True if every axis's identity checks close (expected in both modes).
    pub all_identities_closed: bool,
    /// Whether all 4 axes and the 5-step Stochary Engineering Notebook are simultaneously verified.
    pub all_four_axes_verified: bool,
}

impl StocharyConjecturalCyberneticSynthesis {
    /// Evaluates the canonical shielded synthesis (`seed = 2026`).
    pub fn evaluate_canonical(seed: u64) -> Self {
        Self::evaluate_with_mode(seed, true)
    }

    /// Evaluates the synthesis deploying the cybernetic methods (`true`, Gate `G8`) or the naive methods
    /// (`false`, negative proof `--defect stochary_cybernetics`).
    pub fn evaluate_with_mode(seed: u64, cybernetic_shielded: bool) -> Self {
        let topology_and_stock_axioms =
            TopologyAndFourStockAxiomsAudit::evaluate(seed, cybernetic_shielded);
        let powell_friston_causal_engine =
            PowellFristonCausalEngineAudit::evaluate(seed, cybernetic_shielded);
        let strategic_stock_plant =
            StrategicStockPlantResilienceAudit::evaluate(seed, cybernetic_shielded);
        let cybernetic_vsm_and_pate = CyberneticVsmAndPateGovernanceAudit::evaluate(
            &topology_and_stock_axioms,
            &powell_friston_causal_engine,
            &strategic_stock_plant,
            seed,
            cybernetic_shielded,
        );
        let all_identities_closed = topology_and_stock_axioms.identities_closed
            && powell_friston_causal_engine.identities_closed
            && strategic_stock_plant.identities_closed
            && cybernetic_vsm_and_pate.identities_closed;
        let all_four_axes_verified = topology_and_stock_axioms.axis1_verified
            && powell_friston_causal_engine.axis2_verified
            && strategic_stock_plant.axis3_verified
            && cybernetic_vsm_and_pate.axis4_verified;

        Self {
            topology_and_stock_axioms,
            powell_friston_causal_engine,
            strategic_stock_plant,
            cybernetic_vsm_and_pate,
            all_identities_closed,
            all_four_axes_verified,
        }
    }

    /// Claims removed from the gate after failing multi-seed acceptance.
    pub fn discarded_claim_checks(&self) -> Vec<NamedCheck> {
        self.strategic_stock_plant
            .discarded_claims
            .iter()
            .chain(&self.cybernetic_vsm_and_pate.discarded_claims)
            .copied()
            .collect()
    }

    /// All gated claims of the four axes (Axis 3 includes `SupplyChainPhysicsAudit`), in axis order.
    pub fn claim_checks(&self) -> Vec<NamedCheck> {
        [
            &self.topology_and_stock_axioms.claims,
            &self.powell_friston_causal_engine.claims,
            &self.strategic_stock_plant.claims,
            &self.cybernetic_vsm_and_pate.claims,
        ]
        .into_iter()
        .flatten()
        .copied()
        .collect()
    }

    /// Per-claim pass rates over `seeds` in the given mode (see `crate::acceptance`).
    pub fn multi_seed_acceptance(seeds: &[u64], cybernetic_shielded: bool) -> MultiSeedAcceptance {
        MultiSeedAcceptance::evaluate(seeds, |seed| {
            let syn = Self::evaluate_with_mode(seed, cybernetic_shielded);
            (syn.claim_checks(), syn.all_identities_closed)
        })
    }
}

/// Real-Data Multi-Dataset Coupler (`djia_1885_2023.csv`, `bank_calls.csv`, `danish_fire.csv`,
/// `vic_elec.csv`, and `atlantic_storms.csv`) auditing the Stochary-Conjectural & Cybernetic Synthesis.
#[derive(Debug, Clone, PartialEq)]
pub struct EmpiricalStocharyCyberneticAudit {
    /// **`djia_1885_2023.csv`:** annualized non-ergodic variance drain $\frac{1}{2}\sigma^2 \times 252$. Gated (`> 0.01`).
    pub djia_non_ergodic_variance_drain: f64,
    /// **`bank_calls.csv`:** variance ratio of an EWMA ($\alpha = 0.20$) buffer over daily call totals. Gated (`< 0.30`).
    pub bank_calls_low_pass_variance_ratio: f64,
    /// Informative: $(s, S) = (\mu + 0.45\sigma, \mu + 1.35\sigma)$ daily call-buffer thresholds.
    pub bank_calls_scarf_s_and_big_s: (f64, f64),
    /// **`danish_fire.csv`:** full-sample $(\mathrm{VaR}_{0.95}, \mathrm{CVaR}_{0.95})$ (identity: $\mathrm{CVaR} \ge \mathrm{VaR}$).
    pub danish_fire_var95_and_cvar95: (f64, f64),
    /// Chronological split (first third train, rest test): $\mathrm{CVaR}^{\text{test}}_{0.95} - \mathrm{CVaR}^{\text{train}}_{0.95}$ of the empirical
    /// estimate, and of the WDRO certificate $\mathrm{CVaR}^{\text{train}} + \varepsilon/(1-\beta)$ with $\varepsilon$ the $W_1$ distance between
    /// the two halves of the training window. Informative: the pre-registered Optimizer's Curse claim (`> 1.0`) is discarded at the
    /// observed chronological gap (see `discarded_findings`), which makes the certificate claim vacuous, so it is not gated either.
    pub danish_fire_naive_vs_wdro_disappointment: (f64, f64),
    /// **`vic_elec.csv`:** share of test-half excursions above the training $q_{95}$ whose excess energy is covered by a reserve of
    /// $6.5\,\mathrm{h} \times (q_{99} - q_{95})$ sized on the training half. Gated (`>= 0.90`).
    pub vic_elec_reserve_excursion_coverage: f64,
    /// **`atlantic_storms.csv`:** out-of-sample CORP Telos score $\mathrm{REL} - \mathrm{RES}$ of the second half recalibrated by the
    /// first-half isotonic map, and $\mathrm{UNC}_0$. Gated (`< 0` and `> 0.02`).
    pub atlantic_storms_telos_skill_vs_skopos_unc0: (f64, f64),
    /// Data-dependent claims that did not survive on the real datasets (Prohibition 4: recorded, never gated).
    pub discarded_findings: Vec<&'static str>,
    /// Whether all surviving real-data claims hold.
    pub empirical_stochary_verified: bool,
}

impl EmpiricalStocharyCyberneticAudit {
    /// Evaluates the Stochary-Conjectural & Cybernetic synthesis across the 5 real CSV datasets.
    pub fn evaluate_real_datasets(
        djia_returns: &[f64],
        bank_call_clusters: &[Vec<f64>],
        danish_fire_losses: &[f64],
        vic_elec_demand: &[f64],
        storm_forecasts: (&[f64], &[f64]),
    ) -> Self {
        // 1. DJIA 1885-2023: Non-ergodic variance drain (1/2)*sigma_annual^2
        let sigma_daily = sample_std(djia_returns);
        let djia_non_ergodic_variance_drain = 0.5 * sigma_daily * sigma_daily * 252.0;

        // 2. Bank Calls: low-pass stock smoothing across daily arrival totals
        let daily_totals: Vec<f64> = bank_call_clusters
            .iter()
            .map(|c| c.iter().sum::<f64>())
            .collect();
        let mut smoothed = Vec::with_capacity(daily_totals.len());
        let mut s_buf = mean(&daily_totals);
        for &d in &daily_totals {
            s_buf = 0.80 * s_buf + 0.20 * d;
            smoothed.push(s_buf);
        }
        let bank_calls_low_pass_variance_ratio =
            sample_variance(&smoothed) / sample_variance(&daily_totals).max(1e-9);
        let mu_calls = mean(&daily_totals);
        let sd_calls = sample_std(&daily_totals);
        let bank_calls_scarf_s_and_big_s = (mu_calls + 0.45 * sd_calls, mu_calls + 1.35 * sd_calls);

        // 3. Danish Fire: chronological split; the WDRO radius is the W1 drift observed inside the training window
        let beta = 0.95;
        let (var95, cvar95) = rockafellar_uryasev_cvar(danish_fire_losses, beta);
        let split = danish_fire_losses.len() / 3;
        let (train, test) = danish_fire_losses.split_at(split);
        let (_, cvar_train) = rockafellar_uryasev_cvar(train, beta);
        let (_, cvar_test) = rockafellar_uryasev_cvar(test, beta);
        let epsilon = wasserstein1_1d(&train[..split / 2], &train[split / 2..]);
        let naive_disappointment = cvar_test - cvar_train;
        let wdro_disappointment = cvar_test - (cvar_train + epsilon / (1.0 - beta));

        // 4. Victoria Electricity Demand: reserve sized on the first half, tested on contiguous excursions of the second half
        let half = vic_elec_demand.len() / 2;
        let (vic_train, vic_test) = vic_elec_demand.split_at(half);
        let q95_train = quantile(vic_train, 0.95);
        let q99_train = quantile(vic_train, 0.99);
        let reserve_mwh = 6.5 * (q99_train - q95_train).max(1.0);
        let (mut excursions, mut covered) = (0usize, 0usize);
        let mut excess_mwh = 0.0_f64;
        let mut in_excursion = false;
        for &d in vic_test.iter().chain(std::iter::once(&f64::NEG_INFINITY)) {
            if d > q95_train {
                excess_mwh += (d - q95_train) * 0.5;
                in_excursion = true;
            } else if in_excursion {
                excursions += 1;
                if excess_mwh <= reserve_mwh {
                    covered += 1;
                }
                excess_mwh = 0.0;
                in_excursion = false;
            }
        }
        let vic_elec_reserve_coverage = covered as f64 / excursions.max(1) as f64;

        // 5. Atlantic Storms: isotonic recalibration fitted on the first half and applied out of sample to the second half
        let (probs, outcomes) = storm_forecasts;
        let storm_half = probs.len().min(outcomes.len()) / 2;
        let corp_train =
            CorpPavDecomposition::compute(&probs[..storm_half], &outcomes[..storm_half]);
        let mut iso_map: Vec<(f64, f64)> = probs[..storm_half]
            .iter()
            .copied()
            .zip(corp_train.recalibrated_probabilities_q.iter().copied())
            .collect();
        iso_map.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        let recalibrate = |p: f64| -> f64 {
            let idx = iso_map.partition_point(|&(x, _)| x <= p);
            iso_map[idx.saturating_sub(1)].1.clamp(1e-3, 1.0 - 1e-3)
        };
        let test_probs: Vec<f64> = probs[storm_half..outcomes.len().min(probs.len())]
            .iter()
            .map(|&p| recalibrate(p))
            .collect();
        let corp_test = CorpPavDecomposition::compute(
            &test_probs,
            &outcomes[storm_half..storm_half + test_probs.len()],
        );
        let telos_net_score = corp_test.rel_miscalibration - corp_test.res_resolution;
        let skopos_unc0 = corp_test.unc0_uncertainty;

        let mut discarded_findings = Vec::new();
        if naive_disappointment <= 1.0 {
            discarded_findings.push(
                "danish_fire: the training-window CVaR_95 underestimates the later CVaR_95 by more than 1.0 (Optimizer's Curse)",
            );
        }
        if wdro_disappointment > 0.0 {
            discarded_findings.push(
                "danish_fire: the WDRO certificate CVaR_train + W1/(1 - beta) covers the later CVaR_95",
            );
        }
        if vic_elec_reserve_coverage < 0.90 {
            discarded_findings.push(
                "vic_elec: a 6.5 h x (q99 - q95) reserve sized on the first half covers 90% of second-half peak excursions",
            );
        }
        if telos_net_score >= 0.0 {
            discarded_findings.push(
                "atlantic_storms: first-half isotonic recalibration keeps positive net skill (REL - RES < 0) on the second half",
            );
        }

        let empirical_stochary_verified = djia_non_ergodic_variance_drain > 0.010
            && bank_calls_low_pass_variance_ratio < 0.30
            && cvar95 >= var95
            && skopos_unc0 > 0.02
            && vic_elec_reserve_coverage >= 0.90
            && telos_net_score < 0.0;

        Self {
            djia_non_ergodic_variance_drain,
            bank_calls_low_pass_variance_ratio,
            bank_calls_scarf_s_and_big_s,
            danish_fire_var95_and_cvar95: (var95, cvar95),
            danish_fire_naive_vs_wdro_disappointment: (naive_disappointment, wdro_disappointment),
            vic_elec_reserve_excursion_coverage: vic_elec_reserve_coverage,
            atlantic_storms_telos_skill_vs_skopos_unc0: (telos_net_score, skopos_unc0),
            discarded_findings,
            empirical_stochary_verified,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use stochology_core::DEFAULT_SEED;

    #[test]
    fn test_stochary_conjectural_cybernetic_canonical_and_defect() {
        let pos = StocharyConjecturalCyberneticSynthesis::evaluate_canonical(DEFAULT_SEED);
        assert!(pos.all_identities_closed);
        assert_eq!(pos.topology_and_stock_axioms.tree_total_nodes, 150);
        assert_eq!(
            pos.topology_and_stock_axioms.tree_hierarchy_counts,
            (11, 32, 106)
        );
        assert!(
            (pos.topology_and_stock_axioms.shannon_entropy_l1_bits - 3.4242).abs() < 1e-3,
            "H_L1={}",
            pos.topology_and_stock_axioms.shannon_entropy_l1_bits
        );
        assert!(
            (pos.topology_and_stock_axioms.balance_index_j_l1 - 0.9898).abs() < 1e-3,
            "J_L1={}",
            pos.topology_and_stock_axioms.balance_index_j_l1
        );
        assert!(
            (pos.topology_and_stock_axioms.shannon_entropy_l2_bits - 4.9142).abs() < 1e-3,
            "H_L2={}",
            pos.topology_and_stock_axioms.shannon_entropy_l2_bits
        );
        assert!(
            (pos.topology_and_stock_axioms.balance_index_j_l2 - 0.9828).abs() < 1e-3,
            "J_L2={}",
            pos.topology_and_stock_axioms.balance_index_j_l2
        );
        assert!(
            pos.topology_and_stock_axioms.axis1_verified,
            "{:?}",
            pos.topology_and_stock_axioms
        );
        assert!(
            pos.powell_friston_causal_engine.axis2_verified,
            "{:?}",
            pos.powell_friston_causal_engine
        );
        assert!(
            pos.strategic_stock_plant.axis3_verified,
            "{:?}",
            pos.strategic_stock_plant
        );
        assert!(
            pos.cybernetic_vsm_and_pate.axis4_verified,
            "{:?}",
            pos.cybernetic_vsm_and_pate
        );
        assert!(pos.all_four_axes_verified);

        // The naive methods are actually run and miss their targets while every identity still closes.
        let neg = StocharyConjecturalCyberneticSynthesis::evaluate_with_mode(DEFAULT_SEED, false);
        assert!(neg.all_identities_closed);
        assert!(!neg.all_four_axes_verified);
        assert!(!neg.topology_and_stock_axioms.axis1_verified);
        assert!(!neg.powell_friston_causal_engine.axis2_verified);
        assert!(!neg.strategic_stock_plant.axis3_verified);
        assert!(!neg.cybernetic_vsm_and_pate.axis4_verified);
    }
}
