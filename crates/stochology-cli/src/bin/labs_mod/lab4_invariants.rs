//! Lab 4: Invariants $\Phi_1$–$\Phi_5$, Moulton-Campbell, Exceedance ICC, and Chouldechova.

use std::path::Path;
use stochology_cli::{load_bank_call_clusters, load_toronto_arrests_prevalences};
use stochology_core::{
    ChouldechovaAudit, DEFAULT_SEED, ExceedanceDesignEffect, MonteCarloSplitTreeAudit,
    OperationalWeightingGap,
};

/// Lab 4: Invariants $\Phi_1$–$\Phi_5$, Moulton-Campbell, Exceedance ICC, and Chouldechova.
pub fn lab_invariants(root: &Path) {
    println!("=== LAB 4: INVARIANTS PHI1-PHI5, MOULTON-CAMPBELL, EXCEEDANCE, AND CHOULDECHOVA ===");
    let split_tree = MonteCarloSplitTreeAudit::evaluate(DEFAULT_SEED, 16, 1024);
    let b1 = ExceedanceDesignEffect::benchmark_calibration_25028();
    let b2 = ExceedanceDesignEffect::benchmark_pseudoreplication_500_to_86();
    let amp = OperationalWeightingGap::variance_amplification_benchmark_69x();
    println!(
        "  [Phi1 SplitMix64::split(stream_id)] streams={}, max_|rho|={:.4}, Hamming_avalanche={:.1} bits (verified={}) | [Benchmarks Phi2] n={} -> m_bar={:.2}, m_tilde={:.2}, rho_I(0.90)={:.3}, n_eff={:.1} (naive Kish={:.1}) | n={} -> n_eff={:.1} | var_amp={:.1}x",
        split_tree.num_streams,
        split_tree.max_abs_pairwise_correlation,
        split_tree.mean_hamming_avalanche_bits,
        split_tree.split_independence_verified,
        b1.n_nominal,
        b1.m_bar,
        b1.m_tilde_moulton,
        b1.rho_exceedance_p,
        b1.n_effective_p,
        b1.n_effective_naive_kish,
        b2.n_nominal,
        b2.n_effective_p,
        amp
    );

    let clusters = load_bank_call_clusters(root);
    let real_deff = ExceedanceDesignEffect::from_clustered_observations(&clusters, 0.90);
    let profile_q =
        ExceedanceDesignEffect::profile_across_quantiles(&clusters, &[0.50, 0.90, 0.99]);
    println!(
        "  [Real Data: bank_calls.csv] n={}, clusters={}, m_bar={:.2}, m_tilde={:.2}, rho_I(0.90)={:.4}, DEFF={:.2}, n_eff(0.90)={:.1} (profile q50/q90/q99 n_eff=[{:.0}, {:.0}, {:.0}])",
        real_deff.n_nominal,
        real_deff.num_clusters_c,
        real_deff.m_bar,
        real_deff.m_tilde_moulton,
        real_deff.rho_exceedance_p,
        real_deff.deff_exceedance,
        real_deff.n_effective_p,
        profile_q[0].n_effective_p,
        profile_q[1].n_effective_p,
        profile_q[2].n_effective_p
    );

    let (p_a, p_b) = load_toronto_arrests_prevalences(root);
    let (fpr_a, fpr_b, ratio) =
        ChouldechovaAudit::fpr_ratio_under_equal_ppv_and_fnr(p_a, p_b, 0.65, 0.25);
    let kmr_audit =
        ChouldechovaAudit::from_confusion_matrices((180, 90, 630, 100), (70, 55, 825, 50));
    let (_, _, kmr_var_gap) = kmr_audit.kmr_continuous_impossibility_gap();
    println!(
        "  [Phi4: Chouldechova on toronto_arrests.csv] p_A={:.4} vs p_B={:.4} -> equalizing PPV=0.65 & FNR=0.25 forces FPR_A={:.4} vs FPR_B={:.4} (ratio={:.2}x, KMR_var_gap={:.4})",
        p_a, p_b, fpr_a, fpr_b, ratio, kmr_var_gap
    );
}
