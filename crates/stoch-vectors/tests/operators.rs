//! The Three Canonical Operators: arithmetic, the survival question, the sign,
//! and the two structural bans — no aggregation, no entry into the step.
//!
//! Every number below is declared in the test. None is read from the manuscript:
//! the operators take parameters, and what is being checked is the algebra, not a
//! quoted value.

use stoch_vectors::operators::{
    epistemic, governance, integer_root, vital, OperatorError, UNIT,
};

const HALF: i64 = UNIT / 2;

#[test]
fn design_effects_compose_additively_and_the_product_is_only_a_bound() {
    let effects = [2 * UNIT, 3 * UNIT, 4 * UNIT];
    let additive = epistemic::compose_design_effects(&effects).expect("composes");
    let bound = epistemic::design_effect_product_bound(&effects).expect("bounded");
    // 1 + (2 - 1) + (3 - 1) + (4 - 1) = 7, against a product of 24.
    assert_eq!(additive, 7 * UNIT);
    assert_eq!(bound, 24 * UNIT);
    assert!(additive < bound, "the product is loose: {additive} vs {bound}");

    // Equality holds only when at most one effect departs from the unit, which is
    // exactly the case in which there is nothing for the product to overstate.
    let one = [UNIT, 4 * UNIT];
    assert_eq!(
        epistemic::compose_design_effects(&one).unwrap(),
        epistemic::design_effect_product_bound(&one).unwrap()
    );
    let two = [2 * UNIT, 4 * UNIT];
    assert!(
        epistemic::compose_design_effects(&two).unwrap()
            < epistemic::design_effect_product_bound(&two).unwrap()
    );

    // Composition is additive for any list of effects above the unit, and strictly
    // below the product as soon as two of them exceed it.
    for left in 1..=5u32 {
        for right in 1..=5u32 {
            let pair = [left as i64 * UNIT, right as i64 * UNIT];
            let sum = epistemic::compose_design_effects(&pair).unwrap();
            let product = epistemic::design_effect_product_bound(&pair).unwrap();
            let exact = (left as i64 + right as i64 - 1) * UNIT;
            assert_eq!(sum, exact, "additive composition of {pair:?}");
            assert_eq!(product, left as i64 * right as i64 * UNIT);
            assert_eq!(sum < product, (left > 1) && (right > 1));
        }
    }

    assert_eq!(
        epistemic::compose_design_effects(&[]).unwrap_err(),
        OperatorError::Empty("design effects")
    );
    assert_eq!(
        epistemic::compose_design_effects(&[UNIT / 2]).unwrap_err(),
        OperatorError::BelowUnit
    );
    assert_eq!(
        epistemic::design_effect_product_bound(&[UNIT / 2]).unwrap_err(),
        OperatorError::BelowUnit
    );
}

#[test]
fn the_effective_sample_contracts_and_truncates() {
    assert_eq!(epistemic::effective_sample(1000, UNIT).unwrap(), 1000);
    assert_eq!(epistemic::effective_sample(1000, 3 * UNIT).unwrap(), 333);
    let composed = epistemic::compose_design_effects(&[2 * UNIT, 3 * UNIT]).unwrap();
    assert_eq!(composed, 4 * UNIT);
    assert_eq!(epistemic::effective_sample(1000, composed).unwrap(), 250);
    assert_eq!(
        epistemic::effective_sample(10, UNIT / 2).unwrap_err(),
        OperatorError::BelowUnit,
        "this operator contracts; it does not expand"
    );
    assert_eq!(
        epistemic::effective_sample(-1, UNIT).unwrap_err().code(),
        -3
    );
}

#[test]
fn the_meng_identity_holds_in_the_squared_domain() {
    for rho in [-UNIT, -HALF, 0, HALF, UNIT] {
        for fraction in [UNIT / 10, HALF, UNIT] {
            let parameter = epistemic::Nonresponse::new(rho, 1000, fraction).unwrap();
            let squared = epistemic::meng_bias_squared(parameter).unwrap();
            let bias = epistemic::meng_bias(parameter).unwrap();

            // The identity, recomputed here from its own statement rather than
            // compared with itself: bias^2 = rho^2 sigma^2 (1 - f) / (f), in units.
            let unit = UNIT as i128;
            let r = rho as i128;
            let f = fraction as i128;
            let expected = (r * r * 1000 * 1000 * (unit - f)) / (unit * unit * f);
            assert_eq!(squared as i128, expected, "rho {rho}, f {fraction}");

            // Declared truncation of the integer square root, and nothing else.
            assert!(bias * bias as i64 <= squared);
            assert!(squared < (bias + 1) * (bias + 1) as i64);
            assert_eq!(bias, integer_root(squared).unwrap());
        }
    }

    // The magnitude grows with the defect and with the missing share.
    let small = epistemic::meng_bias(epistemic::Nonresponse::new(HALF, 1000, UNIT).unwrap()).unwrap();
    assert_eq!(small, 0, "a complete response carries no defect");
    // Monotone in the defect, with the response share fixed ...
    let mut previous = 0;
    for rho in [UNIT / 10, UNIT / 4, HALF, UNIT] {
        let bias =
            epistemic::meng_bias(epistemic::Nonresponse::new(rho, 1000, HALF).unwrap()).unwrap();
        assert!(bias >= previous, "monotone in the defect at rho {rho}");
        previous = bias;
    }
    // ... and monotone in the missing share, with the defect fixed.
    let mut previous = 0;
    for fraction in [UNIT, HALF, UNIT / 10] {
        let bias =
            epistemic::meng_bias(epistemic::Nonresponse::new(HALF, 1000, fraction).unwrap())
                .unwrap();
        assert!(bias >= previous, "monotone in the missing share at f {fraction}");
        previous = bias;
    }

    // Ignorable non-response is exactly the zero defect: no correction is owed,
    // and the correction is the identity.
    let ignorable = epistemic::Nonresponse::new(0, 1000, HALF).unwrap();
    assert!(ignorable.ignorable());
    assert_eq!(epistemic::meng_bias(ignorable).unwrap(), 0);
    assert_eq!(epistemic::corrected_level(700_000, ignorable).unwrap(), 700_000);
    assert_eq!(epistemic::data_defect_index(ignorable).unwrap(), 0);

    // A negative defect corrects in the opposite direction, and the sign travels
    // with rho rather than with the magnitude.
    let positive = epistemic::Nonresponse::new(HALF, 1000, HALF).unwrap();
    let negative = epistemic::Nonresponse::new(-HALF, 1000, HALF).unwrap();
    let bias = epistemic::meng_bias(positive).unwrap();
    assert!(bias > 0);
    assert_eq!(epistemic::signed_bias(positive).unwrap(), bias);
    assert_eq!(epistemic::signed_bias(negative).unwrap(), -bias);
    assert_eq!(
        epistemic::corrected_level(700_000, positive).unwrap(),
        700_000 - bias
    );
    assert_eq!(
        epistemic::corrected_level(700_000, negative).unwrap(),
        700_000 + bias
    );
    assert_eq!(
        epistemic::data_defect_index(positive).unwrap(),
        epistemic::data_defect_index(negative).unwrap()
    );

    // The declared parameter refuses values outside its domain.
    assert_eq!(
        epistemic::Nonresponse::new(UNIT + 1, 1, HALF).unwrap_err().code(),
        -3
    );
    assert_eq!(
        epistemic::Nonresponse::new(0, -1, HALF).unwrap_err().code(),
        -3
    );
    assert_eq!(
        epistemic::Nonresponse::new(0, 1, 0).unwrap_err().code(),
        -3
    );
}

#[test]
fn volatility_drag_separates_the_path_from_the_cross_section() {
    assert_eq!(vital::volatility_drag(0).unwrap(), 0);
    assert_eq!(vital::volatility_drag(1000).unwrap(), 0);
    assert_eq!(vital::volatility_drag(2 * UNIT).unwrap(), 2 * UNIT);
    assert_eq!(
        vital::volatility_drag(2 * UNIT).unwrap(),
        (2 * UNIT) * (2 * UNIT) / (2 * UNIT)
    );

    let drift = 500_000;
    let heavy = 2 * UNIT;
    let ensemble = vital::ensemble_growth(drift);
    let time = vital::time_growth(drift, heavy).unwrap();
    assert_eq!(ensemble, drift);
    assert_eq!(time, drift - vital::volatility_drag(heavy).unwrap());
    assert!(time < ensemble, "the path grows slower than the cross-section");
    assert_eq!(vital::non_ergodic_gap(drift, heavy).unwrap(), ensemble - time);
    assert_eq!(vital::non_ergodic_gap(drift, 0).unwrap(), 0, "ergodic without a tail");

    // A single tail draw ruins the path while the cross-section still climbs.
    let dynamics = vital::ReserveDynamics::new(1200, 1000, 2 * UNIT).unwrap();
    assert_eq!(dynamics.net().unwrap(), 200);
    assert_eq!(vital::reserve_after_tail(dynamics, 3 * UNIT, 2).unwrap(), -UNIT);
    assert!(vital::ruined_by_tail(dynamics, 3 * UNIT, 2).unwrap());
    assert!(!vital::ruined_by_tail(dynamics, 3 * UNIT, 1).unwrap());
    assert!(vital::ensemble_growth(dynamics.net().unwrap()) > 0);
    assert!(vital::time_growth(dynamics.net().unwrap(), dynamics.tail_scale()).unwrap() < 0);
}

#[test]
fn recovery_time_is_the_least_crossing_outside_the_step() {
    let dynamics = vital::ReserveDynamics::new(120, 100, 0).unwrap();
    assert_eq!(dynamics.net().unwrap(), 20);

    let crossing = vital::recovery_time(dynamics, 0, 100, 10).unwrap();
    assert_eq!(crossing, Some(5));
    let t = crossing.unwrap();
    assert!(vital::reserve_at(dynamics, 0, t - 1).unwrap() < 100);
    assert!(vital::reserve_at(dynamics, 0, t).unwrap() >= 100);

    // Existence is decided before the search: a short horizon is a crossing that
    // does not exist, not a search that failed.
    assert_eq!(vital::recovery_time(dynamics, 0, 100, 4).unwrap(), None);
    let shrinking = vital::ReserveDynamics::new(100, 120, 0).unwrap();
    assert_eq!(vital::recovery_time(shrinking, 0, 100, 1000).unwrap(), None);
    assert_eq!(vital::recovery_time(dynamics, 500, 100, 10).unwrap(), Some(0));

    // The crossing is the least one: walking every period agrees with it.
    for horizon in 1..=12u32 {
        let searched = vital::recovery_time(dynamics, 0, 100, horizon).unwrap();
        let mut walked = None;
        for period in 0..=horizon {
            if vital::reserve_at(dynamics, 0, period).unwrap() >= 100 {
                walked = Some(period);
                break;
            }
        }
        assert_eq!(searched, walked, "horizon {horizon}");
    }
}

#[test]
fn the_survival_question_has_three_named_outcomes() {
    assert!(vital::survival_condition(5, 5), "equality survives");
    assert!(vital::survival_condition(6, 5));
    assert!(!vital::survival_condition(4, 5));

    assert_eq!(
        vital::survival_outcome(5, Some(5)),
        vital::SurvivalOutcome::RecoversInTime
    );
    assert_eq!(
        vital::survival_outcome(9, Some(5)),
        vital::SurvivalOutcome::RecoversInTime
    );
    assert_eq!(
        vital::survival_outcome(4, Some(5)),
        vital::SurvivalOutcome::CrossesAfterRuin
    );
    assert_eq!(
        vital::survival_outcome(100, None),
        vital::SurvivalOutcome::NoCrossingWithinHorizon
    );
    assert_eq!(vital::SurvivalOutcome::RecoversInTime.tag(), 0);
    assert_eq!(vital::SurvivalOutcome::CrossesAfterRuin.tag(), 1);
    assert_eq!(vital::SurvivalOutcome::NoCrossingWithinHorizon.tag(), 2);

    // The outcome is exactly the predicate plus the existence of the crossing.
    let dynamics = vital::ReserveDynamics::new(120, 100, 0).unwrap();
    let crossing = vital::recovery_time(dynamics, 0, 100, 10).unwrap();
    let ttr = crossing.map(i64::from);
    assert_eq!(vital::survival_outcome(5, ttr), vital::SurvivalOutcome::RecoversInTime);
    assert_eq!(vital::survival_outcome(4, ttr), vital::SurvivalOutcome::CrossesAfterRuin);
}

#[test]
fn governance_fixes_the_sign_and_is_not_a_third_reading() {
    let low = HALF - 1;
    let high = HALF + 1;
    assert_eq!(
        governance::differential_sign(low, HALF).unwrap(),
        governance::DifferentialSign::Negative
    );
    // The pivot is a tie with a named arm: the differential is zero there, so the
    // coordinate does not count it as permission.
    assert_eq!(
        governance::differential_sign(HALF, HALF).unwrap(),
        governance::DifferentialSign::Neutral
    );
    assert_eq!(governance::differential_sign(HALF, HALF).unwrap().factor(), 0);
    assert_eq!(governance::signed_differential(7, HALF, HALF).unwrap(), 0);
    assert_eq!(
        governance::differential_sign(high, HALF).unwrap(),
        governance::DifferentialSign::Positive
    );
    assert_eq!(
        governance::differential_sign(high, HALF).unwrap().factor(),
        1
    );
    assert_eq!(
        governance::differential_sign(low, HALF).unwrap().factor(),
        -1
    );

    // Magnitude never chooses the sign, and the coordinate never chooses the
    // magnitude: the two are read from different arguments.
    for magnitude in [0, 1, HALF, UNIT - 1] {
        assert_eq!(
            governance::signed_differential(magnitude, low, HALF).unwrap(),
            -magnitude
        );
        assert_eq!(
            governance::signed_differential(magnitude, high, HALF).unwrap(),
            magnitude
        );
    }
    assert_eq!(
        governance::signed_differential(7, low, HALF).unwrap(),
        -governance::signed_differential(7, high, HALF).unwrap()
    );

    assert_eq!(
        governance::differential_sign(UNIT + 1, HALF).unwrap_err().code(),
        -3
    );
    assert_eq!(
        governance::differential_sign(HALF, 0).unwrap_err().code(),
        -3
    );
    assert_eq!(
        governance::signed_differential(-1, HALF, HALF).unwrap_err().code(),
        -3
    );

    // Bitemporal responsibility is a fold over ONE coordinate's three readings.
    let readings = governance::Bitemporal::new(900_000, 400_000, 700_000).unwrap();
    assert_eq!(readings.responsibility(), 400_000);
    assert_eq!(readings.ex_ante(), 900_000);
    assert_eq!(readings.kairos(), 400_000);
    assert_eq!(readings.ex_post(), 700_000);
    let late = governance::Bitemporal::new(900_000, 800_000, 100_000).unwrap();
    assert_eq!(late.responsibility(), 100_000);
    let unanimous = governance::Bitemporal::new(600_000, 600_000, 600_000).unwrap();
    assert_eq!(unanimous.responsibility(), 600_000);
    assert_eq!(
        governance::Bitemporal::new(0, UNIT + 1, 0).unwrap_err().code(),
        -3
    );

    // The gate is performative integrity: what was committed against what was done.
    let punctual = governance::Integrity::new(10, 10).unwrap();
    assert_eq!(punctual.ratio().unwrap(), UNIT);
    assert!(punctual.gate(UNIT).unwrap());
    let short = governance::Integrity::new(10, 8).unwrap();
    assert_eq!(short.ratio().unwrap(), 800_000);
    assert!(short.gate(800_000).unwrap());
    assert!(!short.gate(800_001).unwrap());
    let over = governance::Integrity::new(10, 12).unwrap();
    assert_eq!(over.ratio().unwrap(), 1_200_000, "over-discharge is not clamped");
    assert_eq!(governance::Integrity::new(0, 1).unwrap_err().code(), -5);
    assert_eq!(governance::Integrity::new(1, -1).unwrap_err().code(), -3);
}

fn module_source() -> String {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/operators.rs");
    std::fs::read_to_string(&path).unwrap_or_else(|error| {
        panic!("the operator module must be readable at {}: {error}", path.display())
    })
}

fn code_lines(source: &str) -> Vec<&str> {
    source
        .lines()
        .filter(|line| {
            let trimmed = line.trim_start();
            !(trimmed.starts_with("//") || trimmed.starts_with('*'))
        })
        .collect()
}

#[test]
fn the_operators_do_not_reach_the_step_or_the_accept_path() {
    let source = module_source();
    let code = code_lines(&source).join("\n");

    for forbidden in ["step(", "accept(", "Sentence", "Transcript", "State", "Regime", "Verdict"] {
        assert!(
            !code.contains(forbidden),
            "the operator layer's code names or calls {forbidden:?}"
        );
    }
    let core_uses: Vec<&str> = code
        .match_indices("stoch_core::")
        .map(|(index, _)| &code[index..(index + "stoch_core::SCALE".len()).min(code.len())])
        .collect();
    assert_eq!(core_uses.len(), 1, "the layer reads only the unit from the core");
    assert_eq!(core_uses[0], "stoch_core::SCALE");

    // And the refusal is real at compile time: the module's own doctests are the
    // two `compile_fail` blocks (step and accept), and they run with the suite.
    assert!(source.contains("compile_fail"), "the isolation proof is a doctest");
}

#[test]
fn no_artifact_folds_the_three_margins() {
    let source = module_source();
    let code = code_lines(&source).join("\n");
    for forbidden in [
        "s_epi", "s_vital", "s_gov", "weighted", "average", "sum(", "fold3", "score",
    ] {
        assert!(
            !code.contains(forbidden),
            "the layer mentions {forbidden:?}, which is how a fold across the three axes starts"
        );
    }
    // The three temporal readings are folded over time; the three axes are never
    // even named, so no expression here can combine them.
    assert!(code.contains("responsibility"));
    assert!(code.contains("ex_ante") && code.contains("kairos") && code.contains("ex_post"));
    assert!(!code.contains("min(s_epi"));
}
