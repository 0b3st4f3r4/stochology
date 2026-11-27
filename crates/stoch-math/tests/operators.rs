//! The Three Canonical Operators of Part IV in `stoch-math`: arithmetic, survival,
//! differential sign, and structural isolation guarantees.

use std::fs;
use std::path::{Path, PathBuf};

use stoch_math::{epistemic, governance, integer_root_i64, vital, OperatorError, UNIT};

const HALF: i64 = UNIT / 2;

#[test]
fn design_effects_compose_additively_and_the_product_is_only_a_bound() {
    let effects = [2 * UNIT, 3 * UNIT, 4 * UNIT];
    let additive = epistemic::compose_design_effects(&effects).expect("composes");
    let bound = epistemic::design_effect_product_bound(&effects).expect("bounded");
    assert_eq!(additive, 7 * UNIT);
    assert_eq!(bound, 24 * UNIT);
    assert!(additive < bound, "the product is loose: {additive} vs {bound}");

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

    for left in 1..=5_i64 {
        for right in 1..=5_i64 {
            let pair = [left * UNIT, right * UNIT];
            let sum = epistemic::compose_design_effects(&pair).unwrap();
            let product = epistemic::design_effect_product_bound(&pair).unwrap();
            let exact = (left + right - 1) * UNIT;
            assert_eq!(sum, exact, "additive composition of {pair:?}");
            assert_eq!(product, left * right * UNIT);
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
        OperatorError::BelowUnit
    );
    assert_eq!(epistemic::effective_sample(-1, UNIT).unwrap_err().code(), -3);
}

#[test]
fn repulsive_sub_unit_design_effect_expands_effective_sample() {
    let deff = epistemic::negative_icc_cluster_deff(5, -200_000).unwrap();
    assert_eq!(deff, 200_000);
    assert_eq!(epistemic::repulsive_effective_sample(100, deff).unwrap(), 500);
    assert_eq!(epistemic::negative_icc_cluster_deff(1, -UNIT).unwrap(), UNIT);
    assert_eq!(
        epistemic::negative_icc_cluster_deff(5, -250_000).unwrap_err(),
        OperatorError::OutOfRange("negative_icc_cluster_deff: rho_icc violates Kish lower bound")
    );
    assert_eq!(
        epistemic::negative_icc_cluster_deff(5, 1).unwrap_err(),
        OperatorError::OutOfRange("negative_icc_cluster_deff: rho_icc outside [-UNIT, 0]")
    );
    assert_eq!(
        epistemic::repulsive_effective_sample(100, UNIT + 1).unwrap_err(),
        OperatorError::OutOfRange("repulsive_effective_sample: design_effect exceeds UNIT")
    );
    assert_eq!(
        epistemic::repulsive_effective_sample(100, 0).unwrap_err(),
        OperatorError::NonPositive("repulsive_effective_sample: design_effect")
    );
}

#[test]
fn the_meng_identity_holds_in_the_squared_domain() {
    for rho in [-UNIT, -HALF, 0, HALF, UNIT] {
        for fraction in [UNIT / 10, HALF, UNIT] {
            let parameter = epistemic::Nonresponse::new(rho, 1000, fraction).unwrap();
            let squared = epistemic::meng_bias_squared(parameter).unwrap();
            let bias = epistemic::meng_bias(parameter).unwrap();

            let unit = i128::from(UNIT);
            let r = i128::from(rho);
            let f = i128::from(fraction);
            let expected = (r * r * 1000 * 1000 * (unit - f)) / (unit * unit * f);
            assert_eq!(i128::from(squared), expected, "rho {rho}, f {fraction}");

            assert!(bias * bias <= squared);
            assert!(squared < (bias + 1) * (bias + 1));
            assert_eq!(bias, integer_root_i64(squared).unwrap());
        }
    }

    let small = epistemic::meng_bias(epistemic::Nonresponse::new(HALF, 1000, UNIT).unwrap()).unwrap();
    assert_eq!(small, 0);

    let ignorable = epistemic::Nonresponse::new(0, 1000, HALF).unwrap();
    assert!(ignorable.ignorable());
    assert_eq!(epistemic::meng_bias(ignorable).unwrap(), 0);
    assert_eq!(epistemic::corrected_level(700_000, ignorable).unwrap(), 700_000);
    assert_eq!(epistemic::data_defect_index(ignorable).unwrap(), 0);

    let positive = epistemic::Nonresponse::new(HALF, 1000, HALF).unwrap();
    let negative = epistemic::Nonresponse::new(-HALF, 1000, HALF).unwrap();
    let bias = epistemic::meng_bias(positive).unwrap();
    assert!(bias > 0);
    assert_eq!(epistemic::signed_bias(positive).unwrap(), bias);
    assert_eq!(epistemic::signed_bias(negative).unwrap(), -bias);
    assert_eq!(epistemic::corrected_level(700_000, positive).unwrap(), 700_000 - bias);
    assert_eq!(epistemic::corrected_level(700_000, negative).unwrap(), 700_000 + bias);

    assert_eq!(
        epistemic::Nonresponse::new(UNIT, 26_000_000_000, UNIT / 2)
            .unwrap_err()
            .code(),
        -3
    );
    let extreme = epistemic::Nonresponse::new(UNIT, epistemic::SIGMA_CAP, 1).unwrap();
    assert_eq!(epistemic::meng_bias_squared(extreme).unwrap_err(), OperatorError::Overflow);

    let tied = [UNIT + 1, UNIT + 1];
    assert_eq!(epistemic::compose_design_effects(&tied).unwrap(), UNIT + 2);
    assert_eq!(epistemic::design_effect_product_bound(&tied).unwrap(), UNIT + 2);
    assert_eq!(
        epistemic::data_defect_index(epistemic::Nonresponse::new(1001, 1, UNIT).unwrap()).unwrap(),
        1
    );
}

#[test]
fn volatility_drag_and_recovery_time_hold() {
    assert_eq!(vital::volatility_drag(0).unwrap(), 0);
    assert_eq!(vital::volatility_drag(2 * UNIT).unwrap(), 2 * UNIT);

    let drift = 500_000;
    let heavy = 2 * UNIT;
    let ensemble = vital::ensemble_growth(drift);
    let time = vital::time_growth(drift, heavy).unwrap();
    assert!(time < ensemble);
    assert_eq!(vital::non_ergodic_gap(drift, heavy).unwrap(), ensemble - time);

    let dynamics = vital::ReserveDynamics::new(1200, 1000, 2 * UNIT).unwrap();
    assert_eq!(vital::reserve_after_tail(dynamics, 3 * UNIT, 2).unwrap(), -UNIT);
    assert!(vital::ruined_by_tail(dynamics, 3 * UNIT, 2).unwrap());
    assert!(!vital::ruined_by_tail(dynamics, 3 * UNIT, 1).unwrap());

    let recovery_dyn = vital::ReserveDynamics::new(120, 100, 0).unwrap();
    assert_eq!(vital::recovery_time(recovery_dyn, 0, 100, 10).unwrap(), Some(5));
    assert_eq!(vital::recovery_time(recovery_dyn, 0, 100, 4).unwrap(), None);
    assert_eq!(
        vital::survival_outcome(5, Some(5)),
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
}

#[test]
fn governance_fixes_the_sign_and_is_not_a_third_reading() {
    let low = HALF - 1;
    let high = HALF + 1;
    assert_eq!(
        governance::differential_sign(low, HALF).unwrap(),
        governance::DifferentialSign::Negative
    );
    assert_eq!(
        governance::differential_sign(HALF, HALF).unwrap(),
        governance::DifferentialSign::Neutral
    );
    assert_eq!(governance::differential_sign(HALF, HALF).unwrap().factor(), 0);
    assert_eq!(governance::differential_sign(HALF, HALF).unwrap().tag(), 0);
    assert_eq!(governance::signed_differential(7, HALF, HALF).unwrap(), 0);
    assert_eq!(
        governance::differential_sign(high, HALF).unwrap(),
        governance::DifferentialSign::Positive
    );
    assert_eq!(governance::differential_sign(high, HALF).unwrap().tag(), 1);
    assert_eq!(governance::differential_sign(low, HALF).unwrap().tag(), -1);

    let readings = governance::Bitemporal::new(900_000, 400_000, 700_000).unwrap();
    assert_eq!(readings.responsibility(), 400_000);
    assert_eq!(readings.ex_ante(), 900_000);
    assert_eq!(readings.kairos(), 400_000);
    assert_eq!(readings.ex_post(), 700_000);

    let over = governance::Integrity::new(10, 12).unwrap();
    assert_eq!(over.ratio().unwrap(), 1_200_000);
    assert!(over.gate(UNIT).unwrap());
}

fn collect_rs(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).expect("readable src dir") {
        let path = entry.expect("dir entry").path();
        if path.is_dir() {
            collect_rs(&path, out);
        } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
            out.push(path);
        }
    }
}

fn code_lines(source: &str) -> String {
    source
        .lines()
        .filter(|line| {
            let trimmed = line.trim_start();
            !(trimmed.starts_with("//") || trimmed.starts_with('*'))
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn contains_ident(haystack: &str, ident: &str) -> bool {
    let bytes = haystack.as_bytes();
    let needle = ident.as_bytes();
    let is_id = |b: u8| b.is_ascii_alphanumeric() || b == b'_';
    let mut idx = 0;
    while idx + needle.len() <= bytes.len() {
        if &bytes[idx..idx + needle.len()] == needle {
            let left_ok = idx == 0 || !is_id(bytes[idx - 1]);
            let right_idx = idx + needle.len();
            let right_ok = right_idx == bytes.len() || !is_id(bytes[right_idx]);
            if left_ok && right_ok {
                return true;
            }
        }
        idx += 1;
    }
    false
}

#[test]
fn the_operators_do_not_reach_the_step_or_the_accept_path() {
    let src_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut files = Vec::new();
    collect_rs(&src_dir, &mut files);
    files.sort();
    assert!(files.len() >= 4, "expected lib, epistemic, vital, governance");

    for path in files {
        let source = fs::read_to_string(&path).expect("readable");
        let code = code_lines(&source);
        for forbidden in ["step(", "accept(", "stoch_core::"] {
            assert!(
                !code.contains(forbidden),
                "{}: contains forbidden {forbidden:?}",
                path.display()
            );
        }
        for core_type in ["Sentence", "Transcript", "State", "Regime", "Verdict"] {
            assert!(
                !contains_ident(&code, core_type),
                "{}: references core type {core_type:?}",
                path.display()
            );
        }
    }
}

#[test]
fn no_artifact_folds_the_three_margins() {
    let src_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut files = Vec::new();
    collect_rs(&src_dir, &mut files);
    files.sort();

    let mut saw_governance = false;
    for path in files {
        let source = fs::read_to_string(&path).expect("readable");
        let code = code_lines(&source);
        for forbidden in [
            "s_epi", "s_vital", "s_gov", "weighted", "average", "sum(", "fold3", "score",
        ] {
            assert!(
                !code.contains(forbidden),
                "{}: mentions {forbidden:?}",
                path.display()
            );
        }
        if path.file_name().and_then(|n| n.to_str()) == Some("governance.rs") {
            assert!(code.contains("responsibility"));
            assert!(code.contains("ex_ante") && code.contains("kairos") && code.contains("ex_post"));
            assert!(!code.contains("min(s_epi"));
            saw_governance = true;
        }
    }
    assert!(saw_governance, "governance.rs must be present in src/");
}
