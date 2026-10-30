//! Protocol Profile ("clay") integration tests over the shaped application in `examples/clay/`.
//!
//! The example dataset is **simulated** (`SplitMix64`, seed 31): a thought experiment about night-shift
//! bed overflow across hospital wards, not a claim about any real hospital (`AGENTS.md` §II.C).
//! Regenerate it with `cargo test -p stochology-cli --test clay_profile -- --ignored --nocapture`.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use stochology_cli::{
    compute_empirical_dataset_audit_with_profile, load_cards_from_disk, resolve_workspace_root,
    verify_card_against_empirical_data_with_profile,
};
use stochology_core::card::{InstrumentWaiver, ProtocolProfile};
use stochology_core::{EpistemologicalCard, Rng, sha256_hex};

const CLAY_DIR: &str = "examples/clay";
const CLAY_CARD: &str = "examples/clay/card_clay_night_shift_overflow.yaml";
const CLAY_CSV: &str = "examples/clay/night_shift_overflow.csv";
const CLAY_SEED: u64 = 31;

fn load_clay(root: &Path) -> (String, EpistemologicalCard, ProtocolProfile) {
    let yaml = fs::read_to_string(root.join(CLAY_CARD)).expect("example card must exist");
    let card = EpistemologicalCard::from_yaml(&yaml).expect("example card must parse");
    let profile = ProtocolProfile::from_card_yaml(&yaml)
        .expect("example profile must parse")
        .expect("example card must declare a protocol_profile");
    (yaml, card, profile)
}

fn gate_codes(card: &EpistemologicalCard, profile: &ProtocolProfile) -> Vec<&'static str> {
    card.audit_with(profile)
        .err()
        .unwrap_or_default()
        .iter()
        .map(|v| v.gate_code)
        .collect()
}

#[test]
fn shaped_card_is_approved_end_to_end_by_gates_audit_card() {
    let root = resolve_workspace_root();
    let (_, card, profile) = load_clay(&root);
    let report = card
        .audit_with(&profile)
        .expect("shaped card must pass audit_with under its own profile");
    assert_eq!(report.profile_name, "night_shift_ward_overflow");
    assert_eq!(report.applied_waivers.len(), 1);
    assert_eq!(report.applied_waivers[0].gate_code, "PHI4_CHOULDECHOVA");
    verify_card_against_empirical_data_with_profile(&root, &card, &profile)
        .expect("shaped card must pass the empirical severe test and SIGA roadmap");

    let out = Command::new(env!("CARGO_BIN_EXE_gates"))
        .args(["--audit-card", CLAY_CARD])
        .current_dir(&root)
        .output()
        .expect("gates binary must run");
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        out.status.success(),
        "gates --audit-card must exit 0: {stdout}\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(stdout.contains("Protocol Profile 'night_shift_ward_overflow'"));
    assert!(stdout.contains("waiver PHI4_CHOULDECHOVA (principle PHI4 stays binding)"));
}

#[test]
fn the_same_card_fails_under_the_canonical_profile() {
    let root = resolve_workspace_root();
    let (_, card, _) = load_clay(&root);
    let codes = gate_codes(&card, &ProtocolProfile::canonical());
    for expected in [
        "PHI1_SEED",
        "PHI3_DATASET_PROVENANCE",
        "PHI3_PREREGISTRATION_HASH",
        "PHI4_CHOULDECHOVA",
    ] {
        assert!(codes.contains(&expected), "missing {expected} in {codes:?}");
    }
}

#[test]
fn a_profile_cannot_waive_the_kernel_or_waive_without_reason() {
    let root = resolve_workspace_root();
    let (_, card, profile) = load_clay(&root);

    let mut seedless = profile.clone();
    seedless.waivers.push(InstrumentWaiver {
        gate_code: "PHI1_SEED".into(),
        reason: "simulation is cheap to rerun".into(),
    });
    assert!(gate_codes(&card, &seedless).contains(&"PROFILE_KERNEL_WAIVER"));

    let mut silent = profile.clone();
    silent.waivers[0].reason = "   ".into();
    let codes = gate_codes(&card, &silent);
    assert!(codes.contains(&"PROFILE_WAIVER_REASON"));
    assert!(
        codes.contains(&"PHI4_CHOULDECHOVA"),
        "a defective waiver must not suppress its instrument: {codes:?}"
    );

    let mut info_gap_waived = profile.clone();
    info_gap_waived.waivers.push(InstrumentWaiver {
        gate_code: "INFO_GAP_SNIEDOVICH".into(),
        reason: "robustness not needed".into(),
    });
    let mut knightian = card.clone();
    knightian.uncertainty = stochology_core::UncertaintyClass::Knightian;
    assert!(gate_codes(&knightian, &info_gap_waived).contains(&"PROFILE_KERNEL_WAIVER"));
}

#[test]
fn recalibrating_the_profile_breaks_the_preregistration_seal() {
    let root = resolve_workspace_root();
    let (_, card, mut profile) = load_clay(&root);
    profile.e_process_min_inverse_alpha = 20.0;
    assert!(gate_codes(&card, &profile).contains(&"PHI3_PREREGISTRATION_HASH"));
}

#[test]
fn application_vocabulary_drives_phi5_resolution() {
    let root = resolve_workspace_root();
    let (_, mut card, mut profile) = load_clay(&root);
    card.decision_maker_question.altered_action =
        "Fixar a média anual de enfermeiros volantes para todos os plantões".into();
    assert!(gate_codes(&card, &profile).contains(&"PHI5_RESOLUTION"));
    // The canonical English vocabulary does not see the coarse aggregate in Portuguese.
    profile.resolution = None;
    assert!(!gate_codes(&card, &profile).contains(&"PHI5_RESOLUTION"));
}

#[test]
fn dataset_outside_the_profile_manifest_or_tampered_is_blocked() {
    let root = resolve_workspace_root();
    let (_, card, profile) = load_clay(&root);

    let mut wrong_manifest = profile.clone();
    wrong_manifest.dataset_manifest = "data/SHA256SUMS".into();
    assert!(compute_empirical_dataset_audit_with_profile(&root, &card, &wrong_manifest).is_err());

    let tmp = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("clay_tampered");
    let tmp_dir = tmp.join(CLAY_DIR);
    fs::create_dir_all(&tmp_dir).expect("temp dir");
    fs::copy(
        root.join(CLAY_DIR).join("SHA256SUMS"),
        tmp_dir.join("SHA256SUMS"),
    )
    .expect("copy manifest");
    let mut csv = fs::read_to_string(root.join(CLAY_CSV)).expect("read csv");
    csv.push_str("W99,1,0.5000,1,0.00\n");
    fs::write(tmp.join(CLAY_CSV), csv).expect("write tampered csv");
    let err = compute_empirical_dataset_audit_with_profile(&tmp, &card, &profile)
        .expect_err("tampered dataset must be blocked by the profile manifest");
    assert!(err.contains("SHA-256"), "{err}");
}

#[test]
fn canonical_cards_keep_their_behaviour_and_seals() {
    let root = resolve_workspace_root();
    let canonical = ProtocolProfile::canonical();
    for (path, card) in load_cards_from_disk(&root).expect("canonical cards") {
        let yaml = fs::read_to_string(root.join(&path)).expect("read card");
        assert_eq!(ProtocolProfile::from_card_yaml(&yaml), Ok(None), "{path}");
        assert_eq!(
            canonical.preregistration_hash_for(&card),
            card.compute_preregistration_hash(),
            "{path}"
        );
        assert_eq!(
            card.audit_with(&canonical).map(|_| ()),
            card.audit(),
            "{path}"
        );
        let mut tampered = card.clone();
        tampered.decision_rule_and_scope.event_probability_p = 0.0;
        assert_eq!(
            tampered.audit_with(&canonical).err(),
            tampered.audit().err(),
            "{path}"
        );
    }
}

/// Regenerates `examples/clay/night_shift_overflow.csv` and its `SHA256SUMS` (simulated, seed 31),
/// then prints the empirical quantities the example card must declare.
///
/// Generating process (all inputs `Assumed:`): 48 wards with 25–75 night shifts each; ward risk
/// $u_w \sim N(0, 0.8^2)$; per-shift admission pressure $z \sim N(0, 1)$; overflow probability
/// $p = \mathrm{logistic}(-1.4 + u_w + 1.1 z)$, published as the forecast; overflow $Y \sim \mathrm{Bernoulli}(p)$;
/// excess patients over staffed capacity $X = 3 u_w + 2 z + N(0, 1.5^2)$.
#[test]
#[ignore]
fn regenerate_clay_example() {
    let root = resolve_workspace_root();
    let mut rng = Rng::new(CLAY_SEED);
    let mut csv = String::from("ward,shift,p_hat,overflow,excess_patients\n");
    for w in 0..48 {
        let shifts = 25 + (rng.next_u64() % 51) as usize;
        let u = rng.normal(0.0, 0.8);
        for s in 0..shifts {
            let z = rng.standard_normal();
            let p = 1.0 / (1.0 + (-(-1.4 + u + 1.1 * z)).exp());
            let y = u8::from(rng.bernoulli(p));
            let x = 3.0 * u + 2.0 * z + rng.normal(0.0, 1.5);
            csv.push_str(&format!("W{w:02},{},{p:.4},{y},{x:.2}\n", s + 1));
        }
    }
    fs::write(root.join(CLAY_CSV), &csv).expect("write csv");
    let sha = sha256_hex(csv.as_bytes());
    fs::write(
        root.join(CLAY_DIR).join("SHA256SUMS"),
        format!("{sha}  {CLAY_CSV}\n"),
    )
    .expect("write manifest");
    println!("dataset_sha256: {sha}");

    let Ok((_, mut card, profile)) =
        std::panic::catch_unwind(|| load_clay(&resolve_workspace_root()))
    else {
        println!("example card absent or unparsable: write it, then rerun");
        return;
    };
    card.data_and_audit_contract.dataset_sha256 = sha;
    let emp = compute_empirical_dataset_audit_with_profile(&root, &card, &profile)
        .expect("empirical audit");
    let js = &emp.jensen_summary;
    println!(
        "n={} clusters={} m_bar={:.2} m_tilde={:.2} rho_I(0.90)={:.4} DEFF={:.2} n_eff={}",
        emp.n_nominal,
        emp.num_clusters,
        emp.m_bar,
        emp.m_tilde,
        emp.rho_exceedance_90,
        emp.deff_exceedance_90,
        emp.n_eff_exceedance_90
    );
    println!(
        "nonlinearity: f(E[X])={:.2}, E[f(X)]={:.2}, Jensen gap Delta_J={:+.2}, median={:.2}, q05={:.2}, q95={:.2}, CRPS={:.2}",
        js.f_at_mean,
        js.expected_f,
        js.jensen_gap,
        js.median_f,
        js.q05_f,
        js.q95_f,
        js.empirical_crps
    );
    println!(
        "p={:.5} CORP: BS={:.5} REL={:.5} RES={:.5} UNC0={:.5} TPR/FPR@p*={:?}",
        emp.event_probability_p,
        emp.corp_pav.brier_total,
        emp.corp_pav.rel_miscalibration,
        emp.corp_pav.res_resolution,
        emp.corp_pav.unc0_uncertainty,
        emp.confusion_tpr_fpr_at_p_star
    );
    let e_t: f64 = emp.ville_e_values.iter().product();
    let exceed = emp
        .null_distribution_samples
        .iter()
        .filter(|&&s| s >= emp.observed_test_statistic)
        .count();
    println!(
        "E_t (product of {} Ville factors) = {e_t:.1}; null exceedances = {exceed}/{}",
        emp.ville_e_values.len(),
        emp.null_distribution_samples.len()
    );
    println!(
        "preregistration_hash: {}",
        profile.preregistration_hash_for(&card)
    );
}
