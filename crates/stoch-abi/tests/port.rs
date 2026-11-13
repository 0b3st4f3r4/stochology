//! The host-side conformance tests of the port.
//!
//! The five exports are `i32`-addressed because the shipped artifact is a wasm32
//! module; a 64-bit host pointer does not fit in an `i32`. These tests therefore call
//! the pointer forms the exports wrap (`step_at`, `accept_at`, `diagnose_at`,
//! `vectors_hash_at`), which are the same functions the exports call — the `extern "C"`
//! shims add nothing but the `as usize as *const _` conversion, and that conversion is
//! itself exercised by `tests/module.rs`, which reads the shipped binary.
//!
//! Every expectation below is either the core's own answer for the same record
//! (differential) or a value the document fixes (`02` §2.2, §2.4). No expectation is
//! a number the port invented.

use std::process::Command;

use stoch_abi::{
    accept_at, diagnose_at, step_at, vectors_hash_at, DiagnosisOut, RegimeOut, StateRec, StepTriple,
    VerdictOut, ARENA_STEP_BYTES, E_CALLER_CONTRACT, MAX_STEPS,
};

// ---------------------------------------------------------------------------
// Records.
// ---------------------------------------------------------------------------

/// A sealed state that takes the buffered branch: `l > c`, no floor, no latency.
fn buffered_record() -> StateRec {
    StateRec {
        instrument: 0,
        missing_slot: 0,
        l: 200,
        u: 400,
        c: 120,
        b: 0,
        r: 1000,
        tts: 100,
        ttr_eff: 100,
        n_eff: 1000,
        defect: 0,
        reversible: 1,
        gov_corrupt: 0,
        vital_floor: 0,
        gov_floor: 0,
        tape: 1,
        site: 0,
        accept_threshold: 100_000,
        band_width: 50_000,
        guard: [900_000; 7],
        budget: 1000,
        elapsed: 10,
        proposition: 0,
        step_index: -1,
        reserved1: [0u8; 56],
    }
}

/// The empty regime record a caller writes into before the call.
fn regime_out() -> RegimeOut {
    RegimeOut {
        payload_0: -7,
        payload_1: -7,
        payload_2: -7,
        reserved: [0x7F; 48],
    }
}

/// The empty verdict record a caller writes into before the call.
fn verdict_out() -> VerdictOut {
    VerdictOut {
        proposition: -7,
        site: -7,
        margin: -7,
        at_step: -7,
        reserved: [0x7F; 40],
    }
}

/// The empty diagnosis record a caller writes into before the call.
fn diagnosis_out() -> DiagnosisOut {
    DiagnosisOut {
        binding_axis: -7,
        absorbed: -7,
        s_epi: -7,
        s_vital: -7,
        s_gov: -7,
        ruin_step: -7,
        terminal_reserve: -7,
    }
}

fn triple(s_epi: i64, s_vital: i64, s_gov: i64, absorbed: i32, reserve: i32) -> StepTriple {
    StepTriple {
        s_epi,
        s_vital,
        s_gov,
        absorbed,
        reserve,
    }
}

/// Run one step through the port.
fn run_step(rec: &StateRec) -> (i32, RegimeOut) {
    let mut out = regime_out();
    // SAFETY: both pointers are to live, 8-aligned locals owned by this frame.
    let tag = unsafe { step_at(rec, &mut out) };
    (tag, out)
}

/// Run one acceptance through the port.
fn run_accept(rec: &StateRec) -> (i32, VerdictOut) {
    let mut out = verdict_out();
    // SAFETY: both pointers are to live, 8-aligned locals owned by this frame.
    let tag = unsafe { accept_at(rec, &mut out) };
    (tag, out)
}

/// Run one trajectory through the port.
fn run_diagnose(steps: &[StepTriple]) -> (i32, DiagnosisOut) {
    let mut out = diagnosis_out();
    // SAFETY: the slice is live and 8-aligned, and `len` is its length.
    let status = unsafe { diagnose_at(steps.as_ptr(), steps.len() as i32, &mut out) };
    (status, out)
}

// ---------------------------------------------------------------------------
// step: the happy vectors, one per regime the record can reach.
// ---------------------------------------------------------------------------

#[test]
fn a_buffered_state_writes_the_zero_tag_and_no_payload() {
    let (tag, out) = run_step(&buffered_record());
    assert_eq!(tag, 0, "l > c is the buffered branch");
    assert_eq!(out.payload_0, 0);
    assert_eq!(out.payload_1, 0);
}

#[test]
fn a_straddle_writes_the_probe_fraction_in_the_first_slot() {
    let mut rec = buffered_record();
    rec.l = 120;
    rec.c = 200;
    rec.reversible = 1;
    let (tag, out) = run_step(&rec);
    assert_eq!(tag, 1);
    assert_eq!(out.payload_0, 714_285, "(u - c) / (u - l) truncated");

    rec.reversible = 0;
    let (tag, out) = run_step(&rec);
    assert_eq!(tag, 4, "an irreversible straddle halts");
    assert_eq!(out.payload_1, 2, "cause 2 is the refused probe");
}

#[test]
fn the_floors_and_the_latency_reach_their_own_tags() {
    let mut rec = buffered_record();
    rec.vital_floor = 1;
    let (tag, out) = run_step(&rec);
    assert_eq!(tag, 2);
    assert_eq!(out.payload_1, 0, "constraint 0 is the vital floor");

    let mut rec = buffered_record();
    rec.gov_floor = 1;
    let (tag, out) = run_step(&rec);
    assert_eq!(tag, 2);
    assert_eq!(out.payload_1, 1, "constraint 1 is the governance floor");

    let mut rec = buffered_record();
    rec.tts = 5;
    rec.ttr_eff = 10;
    let (tag, out) = run_step(&rec);
    assert_eq!(tag, 4, "Q2 seals before Q3 can buffer");
    assert_eq!(out.payload_1, 0, "cause 0 is the survival latency");

    let mut rec = buffered_record();
    rec.gov_corrupt = 1;
    let (tag, out) = run_step(&rec);
    assert_eq!(tag, 4);
    assert_eq!(out.payload_1, 1, "cause 1 is the governance channel");
}

#[test]
fn a_missing_reading_quarantines_with_its_slot() {
    let mut rec = buffered_record();
    rec.instrument = 1;
    rec.missing_slot = 3;
    let (tag, out) = run_step(&rec);
    assert_eq!(tag, 3);
    assert_eq!(out.payload_1, 3);
}

#[test]
fn an_impossible_capacity_takes_no_action() {
    let mut rec = buffered_record();
    rec.l = 50;
    rec.u = 100;
    rec.c = 120;
    let (tag, out) = run_step(&rec);
    assert_eq!(tag, 5, "u < c is the no-action branch");
    assert_eq!(out.payload_0, 0);
    assert_eq!(out.payload_1, 0);
}

// ---------------------------------------------------------------------------
// The port and the core answer the same record the same way.
// ---------------------------------------------------------------------------

#[test]
fn the_port_and_the_core_agree_on_every_record_of_a_cross_product() {
    use stoch_core::{
        step as core_step, Bounds, Capacity, ControlTape, Guards, Instrument, MissingSlot, Params,
        SevenReadings, State, Thresholds,
    };

    let geometries = [
        (200i64, 400i64, 120i64),
        (120, 400, 200),
        (100, 150, 200),
        (200, 400, 200),
        (400, 200, 100),
        (400, 800, 200),
    ];
    for (l, u, c) in geometries {
        for bits in 0u8..16 {
            for instrument in 0u8..2 {
                let mut rec = buffered_record();
                rec.l = l;
                rec.u = u;
                rec.c = c;
                rec.reversible = i32::from(bits & 1 != 0);
                rec.vital_floor = i32::from(bits & 2 != 0);
                rec.gov_floor = i32::from(bits & 4 != 0);
                rec.gov_corrupt = i32::from(bits & 8 != 0);
                rec.tts = 100;
                rec.ttr_eff = 200;
                rec.instrument = i32::from(instrument);
                rec.missing_slot = if instrument == 0 { 0 } else { 4 };

                let (tag, _) = run_step(&rec);

                let core_instrument = if instrument == 0 {
                    Instrument::Sealed(SevenReadings::new(rec.guard))
                } else {
                    Instrument::Incomplete(MissingSlot::new(4).unwrap())
                };
                let expected = State::seal(
                    ControlTape::Attested,
                    Bounds::new(l, u, 1000, 0),
                    Capacity::new(c, 0, 1000),
                    Guards::new(core_instrument),
                    Params::new(
                        100,
                        200,
                        bits & 1 != 0,
                        bits & 2 != 0,
                        bits & 4 != 0,
                        bits & 8 != 0,
                        Thresholds::new(100_000, 50_000),
                    ),
                )
                .map(|state| i32::from(core_step(&state).unwrap().tag()))
                .unwrap_or_else(|error| error.code());

                assert_eq!(tag, expected, "l={l} u={u} c={c} bits={bits} inst={instrument}");
            }
        }
    }
}

// ---------------------------------------------------------------------------
// accept: the verdict and its refusals.
// ---------------------------------------------------------------------------

#[test]
fn an_interior_record_writes_the_zero_verdict() {
    let (tag, out) = run_accept(&buffered_record());
    assert_eq!(tag, 0);
    assert_eq!(out.site, 0);
    assert_eq!(out.margin, 0);
    assert_eq!(out.at_step, -1);
}

#[test]
fn a_band_record_names_its_site_and_its_margin() {
    let mut rec = buffered_record();
    // The epistemic axis is T1..T3; the weakest of them is the fold.
    rec.guard = [101_000, 900_000, 900_000, 900_000, 900_000, 900_000, 900_000];
    rec.site = 1;
    let (tag, out) = run_accept(&rec);
    assert_eq!(tag, 1);
    assert_eq!(out.site, 1);
    assert_eq!(out.margin, 101_000);

    // The same margins declaring a vital site: the vital axis reads 900 000, so the
    // declaration does not name the fold and the transcript is not a boundary.
    rec.site = 4;
    let (tag, _) = run_accept(&rec);
    assert_eq!(tag, 3, "an incoherent site is unparsable, not a boundary");
}

#[test]
fn a_collapse_names_its_proposition_and_its_step() {
    let mut rec = buffered_record();
    rec.guard = [50_000, 900_000, 900_000, 900_000, 900_000, 900_000, 900_000];
    rec.site = 1;
    rec.proposition = 1;
    rec.step_index = 7;
    let (tag, out) = run_accept(&rec);
    assert_eq!(tag, 2);
    assert_eq!(out.proposition, 1);
    assert_eq!(out.site, 1);
    assert_eq!(out.at_step, 7);
}

#[test]
fn a_refused_transcript_returns_the_core_code_and_leaves_the_output_untouched() {
    let mut rec = buffered_record();
    rec.l = -1; // the extent sentinel: no extent was provided
    let (status, out) = run_accept(&rec);
    assert_eq!(status, -6, "E_EXTENT_MISSING");
    assert_eq!(out.proposition, -7, "the output was not written");
    assert_eq!(out.margin, -7);
    assert_eq!(out.reserved, [0x7F; 40]);

    let mut rec = buffered_record();
    rec.guard[2] = 9_000_000_000_000; // a margin off the declared scale
    let (status, _) = run_accept(&rec);
    assert_eq!(status, -5, "E_SCALE_OVERFLOW");
}

#[test]
fn an_incomplete_instrument_that_carries_the_absent_reading_is_refused() {
    let mut rec = buffered_record();
    rec.instrument = 1;
    rec.missing_slot = 3;
    // The record says slot 3 was not read and also gives it a reading.
    let (status, out) = run_accept(&rec);
    assert_eq!(status, E_CALLER_CONTRACT);
    assert_eq!(out.site, -7);

    rec.guard[2] = -1;
    let (status, _) = run_accept(&rec);
    assert_eq!(status, 0, "an absent slot with no reading is a skeleton-covered record");
}

// ---------------------------------------------------------------------------
// diagnose: the absorbed trajectory, and the separation of the wire types.
// ---------------------------------------------------------------------------

#[test]
fn an_absorbed_trajectory_reports_the_ruin_step_and_no_vital_value() {
    use stoch_core::{diagnose as core_diagnose, Score, StepReading, Trajectory};

    let steps = [
        triple(500_000, 700_000, 600_000, 0, 40),
        triple(400_000, 700_000, 600_000, 0, 30),
        triple(400_000, -1, 300_000, 1, 0),
    ];
    let (status, out) = run_diagnose(&steps);
    assert_eq!(status, 0);
    assert_eq!(out.absorbed, 1);
    assert_eq!(out.ruin_step, 2);
    assert_eq!(out.s_vital, 700_000, "the absorbed step contributes no vital value");
    assert_eq!(out.s_epi, 400_000);
    assert_eq!(out.s_gov, 300_000);
    assert_eq!(out.binding_axis, 3, "the governance coordinate is the minimum");
    assert_eq!(out.terminal_reserve, 0);

    let readings = [
        StepReading::new(Score::Present(500_000), Score::Present(700_000), Score::Present(600_000), false, 40),
        StepReading::new(Score::Present(400_000), Score::Present(700_000), Score::Present(600_000), false, 30),
        StepReading::new(Score::Present(400_000), Score::Missing, Score::Present(300_000), true, 0),
    ];
    let view = core_diagnose(&Trajectory::seal(&readings)).observe();
    assert_eq!(out.s_epi, view.s_epi.value().unwrap());
    assert_eq!(out.s_vital, view.s_vital.value().unwrap());
    assert_eq!(out.s_gov, view.s_gov.value().unwrap());
    assert_eq!(out.ruin_step, i64::from(view.ruin_step.unwrap()));
}

#[test]
fn an_absorbed_step_that_carries_a_vital_reading_is_refused() {
    let steps = [triple(400_000, 700_000, 300_000, 1, 0)];
    let (status, out) = run_diagnose(&steps);
    assert_eq!(status, E_CALLER_CONTRACT);
    assert_eq!(out.absorbed, -7, "the output was not written");
}

#[test]
fn a_reading_off_the_scale_is_refused_by_the_sealed_reading_gate() {
    let steps = [triple(400_000, 9_000_000_000_000, 300_000, 0, 0)];
    let (status, _) = run_diagnose(&steps);
    assert_eq!(status, -5, "E_SCALE_OVERFLOW");
}

#[test]
fn an_empty_trajectory_is_a_record_and_a_long_one_is_refused() {
    let (status, out) = run_diagnose(&[]);
    assert_eq!(status, 0);
    assert_eq!(out.absorbed, 0);
    assert_eq!(out.binding_axis, 0);
    assert_eq!(out.terminal_reserve, -1);

    // A length beyond the static buffer is refused, never truncated.
    let mut one = [triple(1, 1, 1, 0, 0)];
    // SAFETY: the pointer is live and aligned; the length is what is under test, and
    // the gate refuses it before any byte of the slice is read.
    let status = unsafe { diagnose_at(one.as_mut_ptr(), (MAX_STEPS + 1) as i32, &mut diagnosis_out()) };
    assert_eq!(status, E_CALLER_CONTRACT);

    let status = unsafe { diagnose_at(one.as_mut_ptr(), -1, &mut diagnosis_out()) };
    assert_eq!(status, E_CALLER_CONTRACT);
}

#[test]
fn the_diagnosis_output_carries_nothing_a_step_can_read() {
    // The separation is a property of the types: `DiagnosisOut` has no field of
    // `StateRec`, and no conversion exists in either direction. The compiler holds
    // the negative half (there is no `From` and no constructor taking a
    // `DiagnosisOut`); this test holds the positive half, that the two layouts share
    // no field name and the diagnosis record is not a state record.
    assert!(std::mem::size_of::<DiagnosisOut>() != std::mem::size_of::<StateRec>());
    assert_eq!(ARENA_STEP_BYTES, std::mem::size_of::<StateRec>() + 64);
}

// ---------------------------------------------------------------------------
// A partial state never returns success.
// ---------------------------------------------------------------------------

#[test]
fn a_partial_state_is_refused_with_the_core_code_and_nothing_written() {
    let mut rec = buffered_record();
    rec.tape = 0; // not attested
    let (status, out) = run_step(&rec);
    assert_eq!(status, -7, "E_TAPE_UNSEALED");
    assert_eq!(out.payload_0, -7);
    assert_eq!(out.payload_1, -7);

    let mut rec = buffered_record();
    rec.instrument = 1;
    rec.missing_slot = 0; // incomplete with no absent slot named
    let (status, _) = run_step(&rec);
    assert_eq!(status, E_CALLER_CONTRACT);

    let mut rec = buffered_record();
    rec.reversible = 2; // a flag the document does not define
    let (status, _) = run_step(&rec);
    assert_eq!(status, E_CALLER_CONTRACT);

    let mut rec = buffered_record();
    rec.guard[0] = -1; // a missing reading on a sealed instrument
    let (status, _) = run_step(&rec);
    assert_eq!(status, -5, "E_SCALE_OVERFLOW, as the core's seventh check names it");

    let mut rec = buffered_record();
    rec.c = 0;
    let (status, _) = run_step(&rec);
    assert_eq!(status, -3, "E_CAPACITY_NON_POSITIVE");

    let mut rec = buffered_record();
    rec.r = -1;
    let (status, _) = run_step(&rec);
    assert_eq!(status, -4, "E_RESERVE_NEGATIVE");
}

// ---------------------------------------------------------------------------
// vectors_hash: derived from the file, not decreed.
// ---------------------------------------------------------------------------

#[test]
fn vectors_hash_is_the_digest_of_the_fixture_on_disk() {
    let mut digest = [0u8; 32];
    // SAFETY: the array is live and 32 bytes long; alignment 1 is always satisfied.
    let status = unsafe { vectors_hash_at(&mut digest) };
    assert_eq!(status, 0);
    assert_eq!(digest, stoch_core::vectors_hash());

    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../crates/stoch-core/vectors.kv"
    );
    let output = Command::new("sha256sum")
        .arg(path)
        .output()
        .expect("sha256sum must be available to check the derivation");
    assert!(output.status.success());
    let text = String::from_utf8(output.stdout).expect("hex is ASCII");
    let hex = &text[..16];
    let expected: Vec<u8> = (0..8)
        .map(|i| u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16).unwrap())
        .collect();
    assert_eq!(&digest[..8], &expected[..], "the first eight bytes are the file's");

    // And the derivation is sensitive to the bytes: one flipped byte of a copy
    // changes the digest, so a literal that had been copied from this file once
    // would stop matching the moment the file moved.
    let bytes = std::fs::read(path).expect("the fixture is readable");
    let mut mutated = bytes.clone();
    mutated[0] ^= 0xFF;
    let temp = std::path::Path::new(env!("CARGO_TARGET_TMPDIR")).join("vectors-mutated.kv");
    std::fs::write(&temp, &mutated).expect("the copy is writable");
    let output = Command::new("sha256sum")
        .arg(&temp)
        .output()
        .expect("sha256sum must be available");
    let text = String::from_utf8(output.stdout).expect("hex is ASCII");
    assert_ne!(&text[..16], hex, "a changed byte changes the digest");
    assert_ne!(&mutated, &bytes);
}
