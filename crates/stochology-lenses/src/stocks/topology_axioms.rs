//! Axis 1: Morphological Audit of the 150-Node Conceptual Tree (Acyclic Tree $\to$ Recursive Cybernetic Graph)
//! and Formal Verification of the Four Ontological Axioms of the Stock.

use crate::acceptance::NamedCheck;
use stochology_core::{Rng, mean, sample_variance};

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

fn shannon_round(v: f64) -> f64 {
    (v * 10_000.0).round() / 10_000.0
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
                "stocks.a1.det.buffered_tail_vega_above_5",
                buffered_convex_tail_vega > 5.0,
            ),
            (
                "stocks.a1.det.jit_tail_vega_below_-10",
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
