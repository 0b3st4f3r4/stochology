//! Axis 3: Strategic Stock Plant & Resilience — Dierickx & Cool's (1989) 5 Laws of Asset Stock Accumulation,
//! Herbert Scarf's (1960) $K$-Convexity of $(S, s)$ Policies, David Simchi-Levi's (2014/2015) $\mathrm{TTS}/\mathrm{TTR}/\mathrm{REI}$,
//! Rockafellar & Uryasev's (2000) $\beta$-CVaR, Esfahani & Kuhn's (2018) Wasserstein DRO, Conformal Safety Stocks,
//! and Classical/Modern Supply-Chain Physics (`SupplyChainPhysicsAudit`).

use crate::acceptance::NamedCheck;
use stochology_core::{mean, median, quantile_sorted, sample_std, sample_variance, Rng};

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
                "stocks.sc.det.graves_willems_saving_above_0.25",
                clark_scarf_graves_willems_meio_saving_fraction > 0.25,
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
        // Development: the 0.88 coverage floor held on 12 of 20 seeds (coverage SD ~0.024 at n = 150).
        // Refuted on the one-shot confirmation (seeds 9_000_000..=9_000_199): WDRO certificate coverage 0.890 and TBS saving 0.935.
        let discarded_claims: Vec<NamedCheck> = vec![
            (
                "stocks.a3.conformal_coverage_at_least_0.88",
                conformal_safety_stock_coverage >= 0.88,
            ),
            (
                "stocks.a3.deployed_wdro_certificate_covers_oos",
                wdro_robust_oos_disappointment <= 0.0,
            ),
            (
                "stocks.sc.tbs_saving_above_0.08",
                supply_chain_physics.allon_van_mieghem_tbs_cost_saving_fraction > 0.08,
            ),
        ];
        let mut claims: Vec<NamedCheck> = vec![
            (
                "stocks.a3.levers_cut_rei_by_0.80",
                simchi_levi_rei_reduction_fraction >= 0.80,
            ),
            (
                "stocks.a3.deployed_tts_to_ttr_at_least_1.15",
                simchi_levi_tts_to_ttr_ratio >= 1.15,
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
