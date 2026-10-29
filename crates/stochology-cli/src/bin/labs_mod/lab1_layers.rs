//! Lab 1: Dynamics of the Three Ontological Layers ($L_1 \to L_2 \to L_3 \to L_1$), Meng, Manski, Jensen, and Performativity.

use std::path::Path;
use stochology_cli::{load_bank_call_clusters, load_lalonde_nsw_cps};
use stochology_core::{
    empirical_wasserstein_1, GoodhartTaxonomyAudit, ManskiBounds, MengAudit, NonlinearSummary,
    PerformativeSystem, DEFAULT_SEED,
};

/// Lab 1: Dynamics of the Three Ontological Layers ($L_1 \to L_2 \to L_3 \to L_1$), Meng, Manski, Jensen, and Performativity.
pub fn lab_layers(root: &Path) {
    println!("=== LAB 1: THREE ONTOLOGICAL LAYERS AND BRIDGE MECHANICS (L1 -> L2 -> L3 -> L1) ===");
    let (nsw_t, nsw_c, cps_c) = load_lalonde_nsw_cps(root);
    let mut pop_y = Vec::with_capacity(nsw_c.len() + cps_c.len());
    let mut rec = Vec::with_capacity(nsw_c.len() + cps_c.len());
    for &y in &cps_c {
        pop_y.push(y);
        rec.push(true);
    }
    for &y in &nsw_c {
        pop_y.push(y);
        rec.push(false);
    }
    let meng = MengAudit::from_population(&rec, &pop_y);
    let manski = ManskiBounds::compute(&nsw_t, nsw_t.len() + 30, 0.0, 60_000.0);
    let (lee_lo, lee_hi, lee_q) =
        ManskiBounds::lee_trimming_bounds(&nsw_t, nsw_t.len() + 15, &nsw_c, nsw_c.len() + 30);
    let w1_nsw = empirical_wasserstein_1(&nsw_t, &nsw_c);
    println!(
        "  [Bridge L1->L2] Meng (CPS n={} vs NSW n={}): rho_R_Y={:+.4}, selection_err={:+.2}, n_eff_Meng={:.2} | Manski MNAR=[{:.1}, {:.1}] | LeeBounds(q={:.3})=[{:.1}, {:.1}], W1={:.1}",
        cps_c.len(),
        nsw_c.len(),
        meng.rho_r_y,
        meng.estimation_error,
        meng.n_eff_meng,
        manski.lower_bound,
        manski.upper_bound,
        lee_q,
        lee_lo,
        lee_hi,
        w1_nsw
    );

    let clusters = load_bank_call_clusters(root);
    let daily_totals: Vec<f64> = clusters.iter().map(|c| c.iter().sum::<f64>()).collect();
    let split_idx = daily_totals.len() * 4 / 5;
    // Convex congestion cost f(x) = x^2 evaluated with out-of-sample CRPS on held-out days
    let jensen = NonlinearSummary::evaluate_out_of_sample(
        &daily_totals[..split_idx],
        &daily_totals[split_idx..],
        |x| x * x,
        2.0,
    );
    println!(
        "  [Bridge L2->L3] Jensen Gap (bank_calls, {} days): f(E[X])={:.2}, E[f(X)]={:.2}, Delta_J={:+.2}, median={:.2}, q05={:.2}, q95={:.2}, OOS_CRPS={:.2}",
        daily_totals.len(),
        jensen.f_at_mean,
        jensen.expected_f,
        jensen.jensen_gap,
        jensen.median_f,
        jensen.q05_f,
        jensen.q95_f,
        jensen.empirical_crps
    );

    let perf_sys = PerformativeSystem::new(1.0, 0.2, 0.5, 1.0, 1.5);
    let perf = perf_sys.diagnose();
    let perfgd = perf_sys.perfgd_trajectory(0.0, 0.5, 50);
    let theta_perfgd = *perfgd.last().unwrap_or(&0.0);
    let osc = PerformativeSystem::simulate_rrm_oscillation(5);
    let goodhart = GoodhartTaxonomyAudit::evaluate(DEFAULT_SEED, 45.0, 6.2);
    println!(
        "  [Bridge L3->L1] Performative Prediction: theta_SL={:.4}, theta_PS={:.4}, theta_PO={:.4} (PerfGD={:.4}), PR(PS)={:.4} vs PR(PO)={:.4}, r_RRM={:.3} (converges={}), oscillation_violated={:?} | Goodhart4(Reg={:+.2}, Ext={:+.2}, Causal_NILE={:.1}x, Adv_grad1={:+.3}, verified={})",
        perf.theta_sl,
        perf.theta_ps,
        perf.theta_po,
        theta_perfgd,
        perf.risk_at_ps,
        perf.risk_at_po,
        perf.rrm_contraction_radius,
        perf.rrm_converges,
        osc,
        goodhart.regressional_winners_curse_inflation,
        goodhart.extremal_tail_decoupling_drop,
        goodhart.causal_nile_penalty_ratio,
        goodhart.adversarial_grad1_dpr_at_ps,
        goodhart.all_four_variants_audited
    );
}
