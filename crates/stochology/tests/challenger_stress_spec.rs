//! Adversarial stress test suite written by Challenger 2.
//!
//! Empirically stress-tests combinatorial invariants, Vandermonde partitions,
//! dual stratification, ancestor cone triadic spanning, grammar judgment edge cases,
//! and mutation resilience for Stochology.

use std::collections::{BTreeSet, HashMap};
use stochology::grammar::{
    BOUNDARY_BAND, CircuitTransit, GrammarBudget, GrammarVerdict, GuardReading, GuardSite, Plane,
    Sentence, Shot, UnparsableReason, judge, parse_ebnf_grammar, shortest_violation,
};
use stochology::kernel::SplitMix64;
use stochology::multilayer::{
    KnowledgeOrder, KnowledgeStratum, LayerTriadReading, MultilayerGraph1, MultilayerGraph2,
    MultilayerHomomorphism, TriadicExposureRegime,
};
use stochology::synthesis::CollapseProposition;

#[test]
fn test_exhaustive_combinatorial_oracle_and_vandermonde_partition() {
    // 1. Generate all theoretical 3-combinations from {0..6}
    let mut theoretical_combos: Vec<[GuardSite; 3]> = Vec::new();
    let all_sites = GuardSite::ALL;
    for i in 0..7 {
        for j in (i + 1)..7 {
            for k in (j + 1)..7 {
                theoretical_combos.push([all_sites[i], all_sites[j], all_sites[k]]);
            }
        }
    }
    assert_eq!(
        theoretical_combos.len(),
        35,
        "Theoretical 7 choose 3 must be exactly 35"
    );

    // 2. Check each KnowledgeOrder::guard_sites()
    let mut order_combos: Vec<[GuardSite; 3]> = Vec::with_capacity(35);
    for order in KnowledgeOrder::ALL {
        let sites = order.guard_sites();
        // Check strict ascending order
        assert!(
            (sites[0] as u8) < (sites[1] as u8) && (sites[1] as u8) < (sites[2] as u8),
            "Order {:?} guard sites must be strictly ordered: {:?}",
            order,
            sites
        );
        assert!(
            !order_combos.contains(&sites),
            "Duplicate combination found for {:?}: {:?}",
            order,
            sites
        );
        order_combos.push(sites);
    }
    assert_eq!(order_combos.len(), 35);

    // 3. Verify bijection between theoretical combos and order combos
    for combo in &theoretical_combos {
        assert!(
            order_combos.contains(combo),
            "Missing combination in KnowledgeOrder: {:?}",
            combo
        );
    }

    // 4. Theoretical Vandermonde partitioning:
    // S_epi has 3 sites (T1, T2, T3)
    // S_vital has 2 sites (T4, T5)
    // S_gov has 2 sites (T6, T7)
    let mut regime_counts: HashMap<TriadicExposureRegime, usize> = HashMap::new();
    let mut signature_counts: HashMap<(u8, u8, u8), usize> = HashMap::new();

    for order in KnowledgeOrder::ALL {
        let (e, v, g) = order.triadic_occupancy();
        assert_eq!(
            e + v + g,
            3,
            "Occupancy signature must sum to 3 for {:?}",
            order
        );
        *signature_counts.entry((e, v, g)).or_insert(0) += 1;
        let regime = order.exposure_regime();
        *regime_counts.entry(regime).or_insert(0) += 1;

        // Verify regime classification matches signature
        match (e, v, g) {
            (1, 1, 1) => assert_eq!(regime, TriadicExposureRegime::TransTriadicKeystone),
            (2, 1, 0) | (1, 2, 0) => {
                assert_eq!(regime, TriadicExposureRegime::GovernanceBlindSpot)
            }
            (2, 0, 1) | (1, 0, 2) => assert_eq!(regime, TriadicExposureRegime::VitalBlindSpot),
            (0, 2, 1) | (0, 1, 2) => assert_eq!(regime, TriadicExposureRegime::EpistemicBlindSpot),
            (3, 0, 0) => assert_eq!(regime, TriadicExposureRegime::PureEpistemicConcentration),
            other => panic!("Unexpected signature {:?} for {:?}", other, order),
        }
    }

    // Expected Vandermonde partition counts:
    // (1,1,1) = C(3,1)*C(2,1)*C(2,1) = 3 * 2 * 2 = 12
    assert_eq!(signature_counts.get(&(1, 1, 1)), Some(&12));
    // (2,1,0) = C(3,2)*C(2,1)*C(2,0) = 3 * 2 * 1 = 6
    assert_eq!(signature_counts.get(&(2, 1, 0)), Some(&6));
    // (1,2,0) = C(3,1)*C(2,2)*C(2,0) = 3 * 1 * 1 = 3
    assert_eq!(signature_counts.get(&(1, 2, 0)), Some(&3));
    // (2,0,1) = C(3,2)*C(2,0)*C(2,1) = 3 * 1 * 2 = 6
    assert_eq!(signature_counts.get(&(2, 0, 1)), Some(&6));
    // (1,0,2) = C(3,1)*C(2,0)*C(2,2) = 3 * 1 * 1 = 3
    assert_eq!(signature_counts.get(&(1, 0, 2)), Some(&3));
    // (0,2,1) = C(3,0)*C(2,2)*C(2,1) = 1 * 1 * 2 = 2
    assert_eq!(signature_counts.get(&(0, 2, 1)), Some(&2));
    // (0,1,2) = C(3,0)*C(2,1)*C(2,2) = 1 * 2 * 1 = 2
    assert_eq!(signature_counts.get(&(0, 1, 2)), Some(&2));
    // (3,0,0) = C(3,3)*C(2,0)*C(2,0) = 1 * 1 * 1 = 1
    assert_eq!(signature_counts.get(&(3, 0, 0)), Some(&1));

    // Regimes sum: 12 + (6+3) + (6+3) + (2+2) + 1 = 12 + 9 + 9 + 4 + 1 = 35
    assert_eq!(
        regime_counts.get(&TriadicExposureRegime::TransTriadicKeystone),
        Some(&12)
    );
    assert_eq!(
        regime_counts.get(&TriadicExposureRegime::GovernanceBlindSpot),
        Some(&9)
    );
    assert_eq!(
        regime_counts.get(&TriadicExposureRegime::VitalBlindSpot),
        Some(&9)
    );
    assert_eq!(
        regime_counts.get(&TriadicExposureRegime::EpistemicBlindSpot),
        Some(&4)
    );
    assert_eq!(
        regime_counts.get(&TriadicExposureRegime::PureEpistemicConcentration),
        Some(&1)
    );
}

#[test]
fn test_dual_stratification_and_primary_guard_site_invariants() {
    let g2 = MultilayerGraph2::canonical();
    assert!(g2.is_bijection_over_35_guard_combinations());
    assert!(g2.every_stratum_covers_all_seven_sites());
    assert!(g2.is_balanced_five_orders_per_primary_site());

    // 1. Check that each stratum has 5 orders and covers all 7 guard sites
    for stratum in KnowledgeStratum::ALL {
        let orders = stratum.orders();
        assert_eq!(orders.len(), 5);

        let mut site_set: BTreeSet<GuardSite> = BTreeSet::new();
        let mut primary_set: BTreeSet<GuardSite> = BTreeSet::new();

        for order in orders {
            assert_eq!(order.stratum(), stratum);
            for s in order.guard_sites() {
                site_set.insert(s);
            }
            assert!(
                primary_set.insert(order.primary_guard_site()),
                "Primary guard site must be distinct within stratum {:?}: {:?}",
                stratum,
                order.primary_guard_site()
            );
            assert!(
                order.guard_sites().contains(&order.primary_guard_site()),
                "Order {:?} primary guard site {:?} must be in its guard_sites {:?}",
                order,
                order.primary_guard_site(),
                order.guard_sites()
            );
        }

        assert_eq!(
            site_set.len(),
            7,
            "Stratum {:?} must cover all 7 sites",
            stratum
        );
        assert_eq!(
            primary_set.len(),
            5,
            "Stratum {:?} must have 5 distinct primary sites",
            stratum
        );
        assert!(
            primary_set.contains(&stratum.canonical_guard_site()),
            "Stratum {:?} must contain its canonical anchor {:?}",
            stratum,
            stratum.canonical_guard_site()
        );
    }

    // 2. Check 5 orders per primary site across all 35 orders
    let mut primary_site_counts: HashMap<GuardSite, usize> = HashMap::new();
    for order in KnowledgeOrder::ALL {
        *primary_site_counts
            .entry(order.primary_guard_site())
            .or_insert(0) += 1;
    }
    for site in GuardSite::ALL {
        assert_eq!(
            primary_site_counts.get(&site),
            Some(&5),
            "Site {:?} must be primary for exactly 5 orders",
            site
        );
    }

    // 3. Check bijection between KnowledgeStratum and GuardSite
    for site in GuardSite::ALL {
        let stratum = KnowledgeStratum::from_canonical_guard_site(site);
        assert_eq!(stratum.canonical_guard_site(), site);
    }
    for stratum in KnowledgeStratum::ALL {
        let site = stratum.canonical_guard_site();
        assert_eq!(KnowledgeStratum::from_canonical_guard_site(site), stratum);
    }
}

#[test]
fn test_g1_dag_and_ancestor_cone_triadic_spanning() {
    let g1 = MultilayerGraph1::canonical();
    assert!(g1.is_strict_dag());
    assert!(g1.every_ancestor_cone_spans_all_three_axes());

    // Verify properties of ancestor cones
    for order in KnowledgeOrder::ALL {
        let cone = g1.ancestors_of(order);
        // Reflexive: order itself must be in cone
        assert!(
            cone.contains(&order),
            "Ancestor cone must contain target node"
        );
        // Transitive: parents must be in cone
        for &parent in order.parent_layers() {
            assert!(
                cone.contains(&parent),
                "Ancestor cone must contain all direct parents"
            );
        }
        // Strict index ordering: all ancestors except target have strictly lower layer_index
        for &anc in &cone {
            if anc != order {
                assert!(anc.layer_index() < order.layer_index());
            }
        }

        // Triadic spanning check
        let mut has_epi = false;
        let mut has_vital = false;
        let mut has_gov = false;
        for anc in &cone {
            let (e, v, g) = anc.triadic_occupancy();
            if e > 0 {
                has_epi = true;
            }
            if v > 0 {
                has_vital = true;
            }
            if g > 0 {
                has_gov = true;
            }
        }
        assert!(
            has_epi && has_vital && has_gov,
            "Ancestor cone for {:?} failed to span all three axes!",
            order
        );
    }
}

#[test]
fn test_circuit_transits_bijection_and_plane_topology() {
    assert!(GuardSite::is_bijection_over_circuit_transits());

    let mut direct = Vec::new();
    let mut reflexive = Vec::new();
    let mut integral = Vec::new();

    for site in GuardSite::ALL {
        match site.transit() {
            CircuitTransit::Directed {
                from,
                to,
                reflexive_lock,
            } => {
                assert_ne!(from, to, "Transit endpoints must be distinct");
                if reflexive_lock {
                    reflexive.push((from, to));
                } else {
                    direct.push((from, to));
                }
            }
            CircuitTransit::IntegralCircuit => {
                integral.push(site);
            }
        }
    }

    assert_eq!(direct.len(), 3, "Exactly 3 direct loop transits");
    assert_eq!(reflexive.len(), 3, "Exactly 3 reflexive locks");
    assert_eq!(integral.len(), 1, "Exactly 1 integral closure");
    assert_eq!(integral[0], GuardSite::T7Liability);

    // Direct loop transits form a closed directed 3-cycle:
    // World -> Archive (T3), Archive -> Institution (T4), Institution -> World (T6)
    assert!(direct.contains(&(Plane::World, Plane::Archive)));
    assert!(direct.contains(&(Plane::Archive, Plane::Institution)));
    assert!(direct.contains(&(Plane::Institution, Plane::World)));

    // Reflexive locks are the exact reversals:
    // Archive -> World (T2), Institution -> Archive (T1), World -> Institution (T5)
    assert!(reflexive.contains(&(Plane::Archive, Plane::World)));
    assert!(reflexive.contains(&(Plane::Institution, Plane::Archive)));
    assert!(reflexive.contains(&(Plane::World, Plane::Institution)));
}

#[test]
fn test_adversarial_grammar_judgment_edge_cases() {
    let threshold = 0.50;
    let base_shot = Shot::canonical(1, 0.70);

    let valid_guards = vec![
        GuardReading::new(GuardSite::T1Framing, 0.80, None),
        GuardReading::new(GuardSite::T2Severity, 0.80, None),
        GuardReading::new(GuardSite::T3Archive, 0.80, None),
        GuardReading::new(GuardSite::T4Formalization, 0.80, None),
        GuardReading::new(GuardSite::T5Temporal, 0.80, None),
        GuardReading::new(GuardSite::T6Performative, 0.80, None),
        GuardReading::new(GuardSite::T7Liability, 0.80, None),
    ];

    // 1. Healthy sentence -> Interior
    let sentence = Sentence::new(vec![base_shot], valid_guards.clone());
    match judge(&sentence, threshold) {
        GrammarVerdict::Interior => {}
        other => panic!("Expected Interior, got {:?}", other),
    }

    // 2. Near-critical boundary test: margin between [tau, tau + BOUNDARY_BAND)
    let mut boundary_guards = valid_guards.clone();
    boundary_guards[0].margin = threshold + BOUNDARY_BAND / 2.0; // 0.525
    let boundary_sentence = Sentence::new(vec![base_shot], boundary_guards);
    match judge(&boundary_sentence, threshold) {
        GrammarVerdict::Boundary { site, margin } => {
            assert_eq!(site, GuardSite::T1Framing);
            assert!((margin - 0.525).abs() < 1e-9);
        }
        other => panic!("Expected Boundary, got {:?}", other),
    }

    // 3. Exact boundary threshold: tau + BOUNDARY_BAND -> Interior
    let mut frontier_guards = valid_guards.clone();
    frontier_guards[0].margin = threshold + BOUNDARY_BAND; // 0.55
    let frontier_sentence = Sentence::new(vec![base_shot], frontier_guards);
    match judge(&frontier_sentence, threshold) {
        GrammarVerdict::Interior => {}
        other => panic!("Expected Interior at exact boundary band, got {:?}", other),
    }

    // 4. Exact tau threshold: tau (0.50) -> Boundary (since [0.50, 0.55))
    let mut tau_guards = valid_guards.clone();
    tau_guards[0].margin = threshold; // 0.50
    let tau_sentence = Sentence::new(vec![base_shot], tau_guards);
    match judge(&tau_sentence, threshold) {
        GrammarVerdict::Boundary { site, margin } => {
            assert_eq!(site, GuardSite::T1Framing);
            assert!((margin - 0.50).abs() < 1e-9);
        }
        other => panic!("Expected Boundary at tau, got {:?}", other),
    }

    // 5. Margin below tau -> Collapse
    let mut collapse_guards = valid_guards.clone();
    collapse_guards[0].margin = threshold - 0.01; // 0.49 on T1 (Epistemic)
    let collapse_sentence = Sentence::new(vec![base_shot], collapse_guards);
    match judge(&collapse_sentence, threshold) {
        GrammarVerdict::Collapse {
            proposition, at, ..
        } => {
            assert_eq!(at, Some(GuardSite::T1Framing));
            assert_eq!(
                proposition,
                CollapseProposition::Proposition1BlindUndersizing
            );
        }
        other => panic!("Expected Collapse, got {:?}", other),
    }

    // 6. Adversarial input: NaN margin -> Unparsable(NonFiniteMargin)
    let mut nan_guards = valid_guards.clone();
    nan_guards[2].margin = f64::NAN;
    let nan_sentence = Sentence::new(vec![base_shot], nan_guards);
    match judge(&nan_sentence, threshold) {
        GrammarVerdict::Unparsable(UnparsableReason::NonFiniteMargin(site)) => {
            assert_eq!(site, GuardSite::T3Archive);
        }
        other => panic!("Expected Unparsable(NonFiniteMargin), got {:?}", other),
    }

    // 7. Adversarial input: Infinite margin -> Unparsable(NonFiniteMargin)
    let mut inf_guards = valid_guards.clone();
    inf_guards[3].margin = f64::INFINITY;
    let inf_sentence = Sentence::new(vec![base_shot], inf_guards);
    match judge(&inf_sentence, threshold) {
        GrammarVerdict::Unparsable(UnparsableReason::NonFiniteMargin(site)) => {
            assert_eq!(site, GuardSite::T4Formalization);
        }
        other => panic!("Expected Unparsable(NonFiniteMargin), got {:?}", other),
    }

    // 8. Adversarial input: Empty sentence -> Unparsable(EmptySentence)
    let empty_sentence = Sentence {
        shots: vec![],
        guards: vec![],
        actions: vec![],
        reads: vec![],
        token_budget: None,
        tool_call_budget: None,
        seal: stochology::governance::TemporalVault::seal(b""),
    };
    match judge(&empty_sentence, threshold) {
        GrammarVerdict::Unparsable(UnparsableReason::EmptySentence) => {}
        other => panic!("Expected Unparsable(EmptySentence), got {:?}", other),
    }

    // 9. Adversarial input: Missing an entire axis (remove T6 and T7 -> Institutional axis unguarded)
    let axis_unguarded: Vec<GuardReading> = valid_guards
        .iter()
        .filter(|g| g.site != GuardSite::T6Performative && g.site != GuardSite::T7Liability)
        .copied()
        .collect();
    let un_sentence = Sentence::new(vec![base_shot], axis_unguarded);
    match judge(&un_sentence, threshold) {
        GrammarVerdict::Unparsable(UnparsableReason::UnguardedTransit(site)) => {
            assert_eq!(site, GuardSite::T6Performative);
        }
        other => panic!("Expected Unparsable(UnguardedTransit), got {:?}", other),
    }

    // 10. Adversarial input: Missing an entire plane (remove T2 and T5 -> World plane unguarded)
    let plane_unguarded: Vec<GuardReading> = valid_guards
        .iter()
        .filter(|g| g.site != GuardSite::T2Severity && g.site != GuardSite::T5Temporal)
        .copied()
        .collect();
    let un_sentence_plane = Sentence::new(vec![base_shot], plane_unguarded);
    match judge(&un_sentence_plane, threshold) {
        GrammarVerdict::Unparsable(UnparsableReason::UnguardedTransit(site)) => {
            assert_eq!(site, GuardSite::T2Severity);
        }
        other => panic!("Expected Unparsable(UnguardedTransit), got {:?}", other),
    }
}

#[test]
fn test_mutation_resilience_of_graph_validators() {
    let mut g2 = MultilayerGraph2::canonical();

    // Baseline passes
    assert!(g2.is_bijection_over_35_guard_combinations());
    assert!(g2.is_balanced_five_orders_per_primary_site());

    // Mutation A: Corrupt one order's combination to create a duplicate
    let original_combo = g2.nodes[1].guard_sites;
    g2.nodes[1].guard_sites = g2.nodes[0].guard_sites; // duplicate of L01
    assert!(
        !g2.is_bijection_over_35_guard_combinations(),
        "is_bijection must detect duplicate 3-combination"
    );
    // Restore
    g2.nodes[1].guard_sites = original_combo;
    assert!(g2.is_bijection_over_35_guard_combinations());

    // Mutation B: Corrupt ordering in combination (non-monotonic)
    g2.nodes[2].guard_sites = [original_combo[1], original_combo[0], original_combo[2]];
    assert!(
        !g2.is_bijection_over_35_guard_combinations(),
        "is_bijection must detect unsorted combination"
    );
    // Reset from canonical
    let mut g2 = MultilayerGraph2::canonical();

    // Mutation C: Primary guard site not in guard_sites()
    g2.nodes[0].primary_site = GuardSite::T4Formalization; // not in [T1, T5, T7]
    assert!(
        !g2.is_balanced_five_orders_per_primary_site(),
        "is_balanced must detect primary site not in guard_sites"
    );
    let mut g2 = MultilayerGraph2::canonical();

    // Mutation D: Break 5-orders-per-primary-site balance (6 vs 4)
    g2.nodes[1].primary_site = g2.nodes[0].primary_site; // duplicate primary site in S1
    assert!(
        !g2.is_balanced_five_orders_per_primary_site(),
        "is_balanced must detect imbalance"
    );

    // Mutation E: G1 DAG cycle or reverse edge
    let mut g1 = MultilayerGraph1::canonical();
    assert!(g1.is_strict_dag());
    // Create an artificial node with a backward parent
    let original_parents = g1.nodes[1].parents;
    g1.nodes[1].parents = &[KnowledgeOrder::L05ContinuumFractureStructural]; // L05 has index 5 > 2!
    assert!(
        !g1.is_strict_dag(),
        "is_strict_dag must detect backward parent"
    );
    g1.nodes[1].parents = original_parents;
    assert!(g1.is_strict_dag());
}

#[test]
fn test_multilayer_homomorphism_monotonicity_and_collapse_propagation() {
    let g1 = MultilayerGraph1::canonical();
    let mut rng = SplitMix64::new(2026);

    // Test over 50 randomized margin profiles
    for trial in 0..50 {
        let readings = KnowledgeOrder::ALL.map(|order| LayerTriadReading {
            order,
            s_epi: rng.next_f64(),
            s_vital: rng.next_f64(),
            s_gov: rng.next_f64(),
        });

        let hom = MultilayerHomomorphism::evaluate(&g1, &readings);

        for order in KnowledgeOrder::ALL {
            let slot = (order.layer_index() - 1) as usize;
            let local_leontief = hom.local[slot].leontief_minimum;
            let cum_leontief = hom.cumulative[slot].leontief_minimum;

            // Monotonicity: Cumulative Leontief floor cannot exceed local Leontief floor
            assert!(
                cum_leontief <= local_leontief + 1e-12,
                "Trial {trial}, {:?}: cumulative floor ({cum_leontief}) exceeds local ({local_leontief})",
                order
            );

            // Inherited constraint: Cumulative floor cannot exceed parent's cumulative floor
            for &parent in order.parent_layers() {
                let p_slot = (parent.layer_index() - 1) as usize;
                let parent_cum = hom.cumulative[p_slot].leontief_minimum;
                assert!(
                    cum_leontief <= parent_cum + 1e-12,
                    "Trial {trial}, {:?}: cumulative floor ({cum_leontief}) exceeds parent {:?} ({parent_cum})",
                    order,
                    parent
                );
            }
        }
    }
}

#[test]
fn test_ebnf_parser_corruption_resilience() {
    // 1. Valid canonical EBNF block should parse cleanly
    let valid_ebnf = r#"```ebnf
Sentence ::= Framing Counting Body
Framing ::= "frame" [ "insulate" | "open" ] [ "type3" ]
Counting ::= "count" Interval [ "deff" ] [ "vintages" ]
Body ::= Decision [ Brake ] ( Write | Read )* [ Halt ] [ Step* ]
Step ::= Framing Counting Body
Decision ::= "act" "lex" "(" "survival" "," "minimax_regret" ")"
Brake ::= "brake" "tts" ">=" "ttr"
Write ::= "write" Scope
Read ::= "read" Scope
Halt ::= "halt"
Interval ::= "[" Scalar "," Scalar "]"
```"#;
    assert!(parse_ebnf_grammar(valid_ebnf).is_ok());

    // 2. Corrupt by dropping one production (e.g. Halt)
    let missing_halt = r#"```ebnf
Sentence ::= Framing Counting Body
Framing ::= "frame" [ "insulate" | "open" ] [ "type3" ]
Counting ::= "count" Interval [ "deff" ] [ "vintages" ]
Body ::= Decision [ Brake ] ( Write | Read )* [ Halt ] [ Step* ]
Step ::= Framing Counting Body
Decision ::= "act" "lex" "(" "survival" "," "minimax_regret" ")"
Brake ::= "brake" "tts" ">=" "ttr"
Write ::= "write" Scope
Read ::= "read" Scope
Interval ::= "[" Scalar "," Scalar "]"
```"#;
    assert!(
        parse_ebnf_grammar(missing_halt).is_err(),
        "Parser must reject incomplete EBNF grammar missing a production"
    );

    // 3. Corrupt by changing a production LHS (e.g. Sentence ::= banana)
    let corrupted_lhs = r#"```ebnf
MalformedSentence ::= Framing Counting Body
Framing ::= "frame" [ "insulate" | "open" ] [ "type3" ]
Counting ::= "count" Interval [ "deff" ] [ "vintages" ]
Body ::= Decision [ Brake ] ( Write | Read )* [ Halt ] [ Step* ]
Step ::= Framing Counting Body
Decision ::= "act" "lex" "(" "survival" "," "minimax_regret" ")"
Brake ::= "brake" "tts" ">=" "ttr"
Write ::= "write" Scope
Read ::= "read" Scope
Halt ::= "halt"
Interval ::= "[" Scalar "," Scalar "]"
```"#;
    assert!(
        parse_ebnf_grammar(corrupted_lhs).is_err(),
        "Parser must reject corrupted LHS in EBNF grammar"
    );
}

#[test]
fn test_shortest_violation_depth_and_boundary_reachability() {
    static VIOLATION_GRID: [f64; 4] = [0.0, 0.40, 0.60, 0.95];
    let budget1 = GrammarBudget {
        max_depth: 3,
        threshold: 0.50,
        grid: &VIOLATION_GRID,
    };
    let (s1, site1) = shortest_violation(&budget1).expect("violation exists");
    assert_eq!(site1, GuardSite::T1Framing);
    assert_eq!(s1.shots.len(), 1, "Shortest violation found at depth 1");

    // When grid contains only values in the Boundary band [0.50, 0.55),
    // depth 1 yields a Boundary verdict (not a Collapse), so BFS reaches depth 2!
    static BOUNDARY_ONLY_GRID: [f64; 2] = [0.52, 0.85];
    let budget2 = GrammarBudget {
        max_depth: 2,
        threshold: 0.50,
        grid: &BOUNDARY_ONLY_GRID,
    };
    let (s2, site2) = shortest_violation(&budget2).expect("depth 2 violation exists");
    assert_eq!(site2, GuardSite::T1Framing);
    assert_eq!(s2.shots.len(), 2, "Reached depth 2 for shortest collapse");

    // When grid is strictly >= 0.80, no violation can be generated
    static SAFE_GRID: [f64; 2] = [0.80, 0.95];
    let budget_safe = GrammarBudget {
        max_depth: 2,
        threshold: 0.50,
        grid: &SAFE_GRID,
    };
    assert!(
        shortest_violation(&budget_safe).is_none(),
        "Safe grid must yield no violations"
    );
}
