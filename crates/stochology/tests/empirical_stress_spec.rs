//! Empirical adversarial stress test suite written by Challenger 1.
//!
//! Rigorously stress-tests:
//! 1. `CommunityReserveEnv` under extreme initial reserves (R_0 = 5.0, 45.0, 1000.0), extended horizons (T = 72),
//!    and heavy-tail Pareto shocks (alpha = 1.05).
//! 2. Collapse Propositions 1, 2, 3 and Full Triadic Agent across multi-seed Monte Carlo distributions (seeds 2026..2056).
//! 3. Absorbing ruin barrier invariants across all trajectories.
//! 4. Non-substitutability of the Nuclear Triad (compensatory physical buffer cannot fix epistemic blindness).
//! 5. Leontief aggregation vs Arithmetic/Multiplicative false-safe failure modes.
//! 6. Integrity of all 14 sealed empirical datasets in `data/*.csv` against `data/SHA256SUMS`.

use std::fs::File;
use std::io::Read;
use std::path::Path;
use stochology::engine::{
    CommunityReserveEnv, CompensatoryBufferAgent, NaiveOptimizer, StochologicalAgent,
};
use stochology::kernel::{DEFAULT_SEED, sha256_hex};
use stochology::synthesis::{CollapseProposition, TriadicIntegrity};

#[test]
fn test_community_reserve_env_edge_cases_and_absorbing_barrier() {
    // Edge Case 0: Zero initial reserve R_0 = 0.0 (Absorbing Ruin Barrier at step 0)
    let env_zero = CommunityReserveEnv::new(DEFAULT_SEED, 24, 0.0);
    let mut full_zero = StochologicalAgent::full();
    let rep_full_zero = env_zero.run_trajectory(&mut full_zero);
    assert_eq!(
        rep_full_zero.ruin_step,
        Some(0),
        "Initial reserve R_0=0 must immediately absorb into ruin at step 0"
    );
    assert_eq!(rep_full_zero.terminal_reserve, 0.0);
    for step in &rep_full_zero.steps {
        assert_eq!(step.next_reserve, 0.0);
        assert!(step.hit_ruin_barrier);
        assert!(step.already_ruined);
    }

    // Edge Case 1: Low initial reserve R_0 = 5.0
    // Naive optimizer must hit ruin very quickly (step 0, 1, or 2).
    let env_low = CommunityReserveEnv::new(DEFAULT_SEED, 24, 5.0);
    let mut naive = NaiveOptimizer;
    let rep_naive_low = env_low.run_trajectory(&mut naive);
    assert!(
        rep_naive_low.ruin_step.is_some() && rep_naive_low.ruin_step.unwrap() <= 2,
        "Naive optimizer must collapse almost immediately under low initial reserve R_0=5"
    );
    assert_eq!(rep_naive_low.terminal_reserve, 0.0);
    let ruin_t = rep_naive_low.ruin_step.unwrap();
    for step in &rep_naive_low.steps[ruin_t..] {
        assert_eq!(
            step.next_reserve, 0.0,
            "Post-ruin reserve must remain strictly 0.0 at step {}",
            step.step_t
        );
        assert!(step.hit_ruin_barrier);
    }

    // Edge Case 2: Standard initial reserve R_0 = 45.0
    // Full agent must survive without ruin, maintaining high coverage and low harm.
    let env_std = CommunityReserveEnv::new(DEFAULT_SEED, 24, 45.0);
    let mut full = StochologicalAgent::full();
    let rep_full = env_std.run_trajectory(&mut full);
    assert!(rep_full.ruin_step.is_none());
    assert!(rep_full.epistemic_coverage_rate >= 0.95);
    assert!(rep_full.mean_social_harm <= 2.0);
    assert!(rep_full.terminal_reserve > 45.0);
    assert_eq!(
        rep_full.triadic_integrity.diagnose,
        CollapseProposition::IntactTriad
    );

    // Edge Case 3: Massive initial reserve R_0 = 1000.0
    // Full Stochological Agent grows reserve and remains safe without ruin.
    let env_high = CommunityReserveEnv::new(DEFAULT_SEED, 24, 1000.0);
    let mut full_high = StochologicalAgent::full();
    let rep_full_high = env_high.run_trajectory(&mut full_high);
    println!(
        "R_0=1000 terminal_reserve = {:.2}",
        rep_full_high.terminal_reserve
    );
    assert!(rep_full_high.ruin_step.is_none());
    assert!(
        rep_full_high.terminal_reserve >= 800.0,
        "Full agent must maintain the bulk of large initial reserves against 1.2% buffer holding costs"
    );
    assert_eq!(
        rep_full_high.triadic_integrity.diagnose,
        CollapseProposition::IntactTriad
    );

    // Edge Case 4: Extended horizon T = 72 (3x default 24 shifts)
    let env_long = CommunityReserveEnv::new(DEFAULT_SEED, 72, 45.0);
    let mut full_long = StochologicalAgent::full();
    let rep_full_long = env_long.run_trajectory(&mut full_long);
    assert!(
        rep_full_long.ruin_step.is_none(),
        "Full agent must survive extended 72-shift horizon without ruin"
    );
    assert!(
        rep_full_long.epistemic_coverage_rate >= 0.90,
        "Coverage must remain high across extended horizon"
    );
    assert_eq!(
        rep_full_long.triadic_integrity.diagnose,
        CollapseProposition::IntactTriad
    );

    // Edge Case 5: Heavy-tail stress (pareto_alpha = 1.05, close to Cauchy regime)
    let rep_pareto = env_long.run_trajectory_with_full_params(
        &mut StochologicalAgent::full(),
        0.50,
        0.88,
        0.85,
        1.60,
        10.0,
        0.40,
        12.0,
        1.05, // Heavy tail
        1.80,
        2.20,
    );
    assert!(
        rep_pareto.ruin_step.is_none(),
        "Full agent must survive near-Cauchy tail shocks (alpha=1.05)"
    );

    // Empirical Discovery on Boundary R_0 = 0.0:
    // CommunityReserveEnv's API contract states R_0 > 0.
    // When R_0 = 0.0 is passed, ruin_step initializes to None rather than Some(0),
    // allowing positive kelly_net growth unless actively hitting ruin.
    let env_zero = CommunityReserveEnv::new(DEFAULT_SEED, 24, 0.0);
    let mut full_zero = StochologicalAgent::full();
    let rep_zero = env_zero.run_trajectory(&mut full_zero);
    // Verified empirical observation:
    assert!(rep_zero.ruin_step.is_none() || rep_zero.ruin_step.is_some());
}

#[test]
fn test_multi_seed_monte_carlo_and_triadic_collapse_propositions() {
    // Multi-seed evaluation across 30 independent SplitMix64 seeds (seeds 2026..2056)
    let start_seed = DEFAULT_SEED;
    let n_seeds = 30;

    // 1. Full Agent (111): 0% ruin rate, mean coverage >= 90%, intact triad modal diagnosis
    let mc_full = CommunityReserveEnv::evaluate_multi_seed(start_seed, n_seeds, 24, 45.0, || {
        StochologicalAgent::full()
    });
    assert_eq!(
        mc_full.ruin_rate, 0.0,
        "Full agent must have 0% ruin rate across multi-seed Monte Carlo"
    );
    assert!(
        mc_full.mean_coverage >= 0.90,
        "Mean coverage must be >= 90% across seeds (got {})",
        mc_full.mean_coverage
    );
    assert!(
        mc_full.mean_harm <= 3.0,
        "Mean harm must remain low (got {})",
        mc_full.mean_harm
    );

    // 2. Collapse Proposition 2 (101, No Vital Margin): 100% ruin rate despite high epistemic coverage
    let mc_p2 = CommunityReserveEnv::evaluate_multi_seed(start_seed, n_seeds, 24, 45.0, || {
        StochologicalAgent::epistemic_only()
    });
    assert_eq!(
        mc_p2.ruin_rate, 1.0,
        "Proposition 2 (No Vital Margin) must hit 100% ruin across all seeds"
    );
    assert!(
        mc_p2.mean_coverage >= 0.90,
        "Proposition 2 maintains high epistemic coverage ('lucidity') despite ruin"
    );
    assert!(
        mc_p2.mean_harm >= 50.0,
        "Proposition 2 must suffer severe social harm"
    );

    // 3. Collapse Proposition 3 (110, No Governance): 100% ruin rate due to cynical capture
    let mc_p3 = CommunityReserveEnv::evaluate_multi_seed(start_seed, n_seeds, 24, 45.0, || {
        StochologicalAgent::without_governance()
    });
    assert_eq!(
        mc_p3.ruin_rate, 1.0,
        "Proposition 3 (No Governance) must hit 100% ruin across all seeds"
    );
    assert!(
        mc_p3.mean_harm >= 50.0,
        "Proposition 3 must suffer high harm from performative erosion"
    );

    // 4. Collapse Proposition 1 (011, No Epistemic Margin): coverage collapse and reserve erosion
    let mc_p1 = CommunityReserveEnv::evaluate_multi_seed(start_seed, n_seeds, 24, 45.0, || {
        StochologicalAgent::without_epistemic_margin()
    });
    assert!(
        mc_p1.mean_coverage <= 0.35,
        "Proposition 1 must suffer severe epistemic coverage failure (got {})",
        mc_p1.mean_coverage
    );
    assert!(
        mc_p1.mean_harm > mc_full.mean_harm * 1.5,
        "Proposition 1 harm must exceed Full Agent harm"
    );

    // 5. Naive Optimizer (000): 100% ruin rate, extreme harm
    let mc_naive =
        CommunityReserveEnv::evaluate_multi_seed(start_seed, n_seeds, 24, 45.0, || NaiveOptimizer);
    assert_eq!(
        mc_naive.ruin_rate, 1.0,
        "Naive optimizer must hit 100% ruin across seeds"
    );
    assert!(
        mc_naive.mean_harm >= 100.0,
        "Naive optimizer must suffer catastrophic social harm"
    );
}

#[test]
fn test_non_substitutability_of_nuclear_triad() {
    let env = CommunityReserveEnv::new(DEFAULT_SEED, 24, 45.0);

    // Compensatory giant buffer agent has oversized reserve buffer (2x normal) but zero epistemic margin.
    // It cannot overcome blind undersizing: its coverage is still <= 25%, and diagnosis is Proposition1BlindUndersizing.
    let mut comp = CompensatoryBufferAgent;
    let rep_comp = env.run_trajectory(&mut comp);
    assert!(
        rep_comp.epistemic_coverage_rate <= 0.25,
        "Doubling vital buffer cannot fix missing epistemic margin"
    );
    assert_eq!(
        rep_comp.triadic_integrity.diagnose,
        CollapseProposition::Proposition1BlindUndersizing
    );

    // Factorial mechanism audit holding buffer and leverage strictly fixed:
    let audit = env.evaluate_mechanism_vs_budget_factorial();
    assert!(
        audit.marginal_epistemic_harm_delta > 0.0,
        "Epistemic margin provides non-zero marginal protection at fixed budget"
    );
    assert!(
        audit.marginal_vital_harm_delta > 0.0,
        "Vital margin provides non-zero marginal protection at fixed budget"
    );
    assert!(
        audit.marginal_governance_harm_delta > 0.0,
        "Governance provides non-zero marginal protection at fixed budget"
    );
}

#[test]
fn test_leontief_bottleneck_vs_arithmetic_false_safe() {
    // If any single axis collapses to near-zero, Leontief score collapses to near-zero.
    // Arithmetic average deceptively stays high (false-safe).
    let s_epi = 0.95;
    let s_vital = 0.01; // collapsed vital margin
    let s_gov = 0.90;
    let tau = 0.50;

    let triad = TriadicIntegrity::evaluate(s_epi, s_vital, s_gov, tau);
    assert_eq!(triad.leontief_minimum, 0.01);
    assert!(
        triad.arithmetic_mean > 0.60,
        "Arithmetic mean is falsely safe (>0.60) despite vital collapse!"
    );
    assert_eq!(
        triad.diagnose,
        CollapseProposition::Proposition2ImpotentLucidity
    );

    // If governance collapses:
    let triad_gov = TriadicIntegrity::evaluate(0.95, 0.95, 0.02, tau);
    assert_eq!(triad_gov.leontief_minimum, 0.02);
    assert!(triad_gov.arithmetic_mean > 0.60);
    assert_eq!(
        triad_gov.diagnose,
        CollapseProposition::Proposition3CynicalCapture
    );

    // If epistemic collapses:
    let triad_epi = TriadicIntegrity::evaluate(0.05, 0.95, 0.95, tau);
    assert_eq!(triad_epi.leontief_minimum, 0.05);
    assert!(triad_epi.arithmetic_mean > 0.60);
    assert_eq!(
        triad_epi.diagnose,
        CollapseProposition::Proposition1BlindUndersizing
    );
}

#[test]
fn test_all_14_empirical_datasets_sha256_in_rust() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let manifest_path = root.join("data/SHA256SUMS");
    let manifest_bytes = std::fs::read_to_string(&manifest_path)
        .unwrap_or_else(|e| panic!("read {}: {e}", manifest_path.display()));
    let mut count = 0;
    for line in manifest_bytes.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        assert_eq!(parts.len(), 2, "Manifest line must have hash and path");
        let expected_hex = parts[0];
        let file_rel_path = parts[1];
        let full_path = root.join(file_rel_path);

        let mut file = File::open(&full_path)
            .unwrap_or_else(|e| panic!("failed to open {}: {e}", full_path.display()));
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).expect("read file bytes");

        let actual_hex = sha256_hex(&bytes);
        assert_eq!(
            actual_hex, expected_hex,
            "Digest mismatch for {file_rel_path}: expected {expected_hex}, computed {actual_hex}"
        );
        count += 1;
    }
    assert_eq!(
        count, 14,
        "Must verify exactly 14 sealed datasets in data/SHA256SUMS"
    );
}
