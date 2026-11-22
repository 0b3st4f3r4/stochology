//! Formal property-based tests for `stoch-core`, `stoch-abi`, `stoch-math`, and `stoch-crdt`.

use proptest::prelude::*;
use stoch_abi::{
    accept_at, diagnose_at, step_at, vectors_hash_at, DiagnosisOut, RegimeOut, StateRec,
    StepTriple, VerdictOut, E_CALLER_CONTRACT, MAX_STEPS,
};
use stoch_core::{
    accept, diagnose, probe_fraction, step, Assessment, AxialReadings, Axis, AxisReading, Bounds,
    Budget, Capacity, Constraint, ContingencyTape, ControlTape, Declared, Error, Guards, HaltCause,
    Instrument, MissingSlot, Params, Proposition, Regime, Score, Sentence, SevenReadings, Site,
    State, StepReading, Thresholds, Trajectory, Transcript, UnparsableReason, Verdict, MAX_ABS,
    SCALE,
};
use stoch_crdt::{
    cross_witness_median, cross_witness_reconcile, CrdtError, EquivocationCertificate, ExilePhase,
    JoinSemilattice, LeontiefEpochState, RehabLattice, RehabPolicy, SignedWitness,
    StoreCarryBundle, StoreCarryQueue, WitnessOutcome,
};
use stoch_host::disclosure::{BindingAxisTag, CoddStatusTag, Level1Cue, RegimeTag};
use stoch_host::hyparview::HyParViewOverlay;
use stoch_host::metrics::LockFreeMetrics;
use stoch_host::solar::{SolarReading, SolarWatchdogConfig, SolarWatchdogEngine};
use stoch_math::paraconsistent::{AnnotatedEvidence, FdeState, Filter1Claim, LocalConsistency};
use stoch_math::{
    chao1_lower_bound, chao2_two_list_lower_bound, combine_deff_additive, epistemic, governance,
    integer_root, kish_design_effect, moulton_cluster_deff, thm3_max_delay, widen_before_seal,
    CoddValue, Interval, UNIT,
};

fn valid_bound_l() -> impl Strategy<Value = i64> {
    prop_oneof![
        (-MAX_ABS..=-2i64),
        (0i64..=MAX_ABS),
    ]
}

fn arb_score() -> impl Strategy<Value = Score> {
    prop_oneof![
        1 => Just(Score::Missing),
        3 => (0i64..=SCALE).prop_map(Score::Present),
    ]
}

fn arb_instrument() -> impl Strategy<Value = Instrument> {
    prop_oneof![
        (1u8..=7u8).prop_map(|slot| Instrument::Incomplete(MissingSlot::new(slot).expect("1..=7"))),
        prop::array::uniform7(0i64..=SCALE).prop_map(|m| Instrument::Sealed(SevenReadings::new(m))),
    ]
}

#[repr(C, align(8))]
struct AlignedScratch([u8; 512]);

fn sentinel_regime_out() -> RegimeOut {
    RegimeOut {
        payload_0: -7,
        payload_1: -7,
        payload_2: -7,
        reserved: [0x7F; 48],
    }
}

fn regime_out_unchanged(out: &RegimeOut) -> bool {
    out.payload_0 == -7
        && out.payload_1 == -7
        && out.payload_2 == -7
        && out.reserved == [0x7F; 48]
}

fn sentinel_verdict_out() -> VerdictOut {
    VerdictOut {
        proposition: -7,
        site: -7,
        margin: -7,
        at_step: -7,
        reason: -7,
        reserved: [0x7F; 36],
    }
}

fn verdict_out_unchanged(out: &VerdictOut) -> bool {
    out.proposition == -7
        && out.site == -7
        && out.margin == -7
        && out.at_step == -7
        && out.reason == -7
        && out.reserved == [0x7F; 36]
}

fn sentinel_diagnosis_out() -> DiagnosisOut {
    DiagnosisOut {
        binding_axis: -7,
        absorbed: -7,
        s_epi: -7,
        s_vital: -7,
        s_gov: -7,
        ruin_step: -7,
        terminal_reserve: -7,
        theta_basin: -7,
    }
}

fn diagnosis_out_unchanged(out: &DiagnosisOut) -> bool {
    out.binding_axis == -7
        && out.absorbed == -7
        && out.s_epi == -7
        && out.s_vital == -7
        && out.s_gov == -7
        && out.ruin_step == -7
        && out.terminal_reserve == -7
        && out.theta_basin == -7
}

#[allow(clippy::too_many_arguments)]
fn make_sealed_state(
    l: i64,
    span: i64,
    c: i64,
    b: i64,
    r: i64,
    tts: i64,
    ttr_eff: i64,
    reversible: bool,
    vital_floor: bool,
    gov_floor: bool,
    gov_corrupt: bool,
    lethal_omission: bool,
    instrument: Instrument,
) -> Option<State> {
    let u = l.checked_add(span)?;
    if u > MAX_ABS || l < -MAX_ABS || c <= 0 || c > MAX_ABS || !(0..=MAX_ABS).contains(&r) {
        return None;
    }
    let bounds = Bounds::new(l, u, 1000, 0);
    let cap = Capacity::new(c, b, r);
    let params = Params::new(
        tts,
        ttr_eff,
        reversible,
        vital_floor,
        gov_floor,
        gov_corrupt,
        Thresholds::new(100_000, 50_000),
    )
    .with_lethal_omission(lethal_omission);
    State::seal(ControlTape::Attested, bounds, cap, Guards::new(instrument), params).ok()
}

proptest! {
    #[test]
    fn prop_step_survival_latency_preempts_all(
        l in valid_bound_l(),
        span in 0i64..=10_000i64,
        c in 1i64..=10_000i64,
        ttr_eff in -10_000i64..=10_000i64,
        deficit in 1i64..=10_000i64,
        reversible in any::<bool>(),
        vital_floor in any::<bool>(),
        gov_floor in any::<bool>(),
        gov_corrupt in any::<bool>(),
        lethal_omission in any::<bool>(),
        instrument in arb_instrument(),
    ) {
        let tts = ttr_eff - deficit;
        if let Some(state) = make_sealed_state(
            l, span, c, 0, 100, tts, ttr_eff,
            reversible, vital_floor, gov_floor, gov_corrupt, lethal_omission, instrument,
        ) {
            let expected = if gov_corrupt {
                HaltCause::SurvivalAndGovernance
            } else {
                HaltCause::SurvivalLatency
            };
            prop_assert_eq!(step(&state), Ok(Regime::AlgedonicHalt { cause: expected }));
        }
    }

    #[test]
    fn prop_step_incomplete_quarantines_when_surviving(
        l in valid_bound_l(),
        span in 0i64..=10_000i64,
        c in 1i64..=10_000i64,
        ttr_eff in -10_000i64..=10_000i64,
        headroom in 0i64..=10_000i64,
        slot in 1u8..=7u8,
        gov_corrupt in any::<bool>(),
        vital_floor in any::<bool>(),
        gov_floor in any::<bool>(),
    ) {
        let tts = ttr_eff + headroom;
        let missing = MissingSlot::new(slot).expect("1..=7");
        if let Some(state) = make_sealed_state(
            l, span, c, 0, 100, tts, ttr_eff,
            true, vital_floor, gov_floor, gov_corrupt, false,
            Instrument::Incomplete(missing),
        ) {
            prop_assert_eq!(step(&state), Ok(Regime::Quarantine { missing }));
        }
    }

    #[test]
    fn prop_step_q2_q3_precedence_and_probe_range(
        l in valid_bound_l(),
        span in 0i64..=100_000i64,
        c in 1i64..=100_000i64,
        ttr_eff in -1_000i64..=1_000i64,
        headroom in 0i64..=1_000i64,
        reversible in any::<bool>(),
        vital_floor in any::<bool>(),
        gov_floor in any::<bool>(),
        gov_corrupt in any::<bool>(),
        lethal_omission in any::<bool>(),
        margins in prop::array::uniform7(0i64..=SCALE),
    ) {
        let tts = ttr_eff + headroom;
        let instrument = Instrument::Sealed(SevenReadings::new(margins));
        if let Some(state) = make_sealed_state(
            l, span, c, 0, 100, tts, ttr_eff,
            reversible, vital_floor, gov_floor, gov_corrupt, lethal_omission, instrument,
        ) {
            let u = l + span;
            let res = step(&state);
            if gov_corrupt {
                prop_assert_eq!(res, Ok(Regime::AlgedonicHalt { cause: HaltCause::GovernanceChannel }));
            } else if vital_floor {
                prop_assert_eq!(res, Ok(Regime::InwardAction { breached: Constraint::VitalFloor }));
            } else if gov_floor {
                prop_assert_eq!(res, Ok(Regime::InwardAction { breached: Constraint::GovernanceFloor }));
            } else if l > c || (l == c && u > c) {
                prop_assert_eq!(res, Ok(Regime::BufferedAction));
            } else if u <= c {
                prop_assert_eq!(res, Ok(Regime::NoAction));
            } else if !reversible {
                let expected = if lethal_omission {
                    HaltCause::LethalOmissionImpasse
                } else {
                    HaltCause::IrreversibleProbeRefused
                };
                prop_assert_eq!(res, Ok(Regime::AlgedonicHalt { cause: expected }));
            } else {
                let pf = probe_fraction(Bounds::new(l, u, 1000, 0), c);
                match pf {
                    Ok(frac) => {
                        prop_assert!((1..SCALE).contains(&frac));
                        prop_assert_eq!(res, Ok(Regime::ReversibleFractionalProbe { fraction: frac }));
                    }
                    Err(Error::ProbeUnderflow) => prop_assert_eq!(res, Err(Error::ProbeUnderflow)),
                    other => prop_assert!(false, "unexpected probe_fraction result: {:?}", other),
                }
            }
        }
    }

    #[test]
    fn prop_step_prefix_invariance_reserve_irrelevant(
        l in valid_bound_l(),
        span in 1i64..=10_000i64,
        c in 1i64..=10_000i64,
        r1 in 0i64..=MAX_ABS,
        r2 in 0i64..=MAX_ABS,
        b1 in -MAX_ABS..=MAX_ABS,
        b2 in -MAX_ABS..=MAX_ABS,
        reversible in any::<bool>(),
    ) {
        let inst = Instrument::Sealed(SevenReadings::new([500_000; 7]));
        let s1 = make_sealed_state(l, span, c, b1, r1, 100, 50, reversible, false, false, false, false, inst);
        let s2 = make_sealed_state(l, span, c, b2, r2, 100, 50, reversible, false, false, false, false, inst);
        if let (Some(a), Some(b)) = (s1, s2) {
            prop_assert_eq!(step(&a), step(&b));
        }
    }

    #[test]
    fn prop_accept_contingency_tape_invariance(
        c1 in any::<u64>(),
        c2 in any::<u64>(),
        epi in arb_score(),
        vital in arb_score(),
        gov in arb_score(),
        site_code in 0u8..=7u8,
        prop_code in 0i64..=3i64,
        step_idx in prop::option::of(0u32..1000u32),
        reopen in any::<bool>(),
    ) {
        let mk = |contingency| {
            Sentence::new(
                ControlTape::Attested,
                ContingencyTape::new(contingency),
                Bounds::new(120, 400, 1000, 0),
                Capacity::new(200, 0, 500),
                Budget::new(1000, 10),
                AxialReadings::new(
                    AxisReading::new(1, epi),
                    AxisReading::new(1, vital),
                    AxisReading::new(1, gov),
                ),
                Declared::new(Proposition::from_code(prop_code), Site::from_code(site_code), step_idx)
                    .with_reopen_sealed(reopen),
                Assessment::new(100_000, 50_000),
            )
        };
        let t1 = Transcript::seal(mk(c1)).expect("valid sentence");
        let t2 = Transcript::seal(mk(c2)).expect("valid sentence");
        prop_assert_eq!(accept(&t1), accept(&t2));
    }

    #[test]
    fn prop_accept_non_commutation_stage_order(
        collapse_margin in 0i64..50_000i64,
        zero_coverage in any::<bool>(),
        budget_overrun in any::<bool>(),
        reopen in any::<bool>(),
    ) {
        // Ensure at least one early Stage 1/2/3 fault fires alongside Stage 4 collapse margins.
        let readings_epi = if zero_coverage { 0 } else { 1 };
        let vital_score = if !zero_coverage && !budget_overrun && !reopen {
            Score::Missing
        } else {
            Score::Present(collapse_margin)
        };
        let elapsed = if budget_overrun { 1001 } else { 10 };
        let sentence = Sentence::new(
            ControlTape::Attested,
            ContingencyTape::new(42),
            Bounds::new(120, 400, 1000, 0),
            Capacity::new(200, 0, 500),
            Budget::new(1000, elapsed),
            AxialReadings::new(
                AxisReading::new(readings_epi, Score::Present(collapse_margin)),
                AxisReading::new(1, vital_score),
                AxisReading::new(1, Score::Present(collapse_margin)),
            ),
            Declared::new(Some(Proposition::P1Epistemic), Some(Site::T1), Some(0))
                .with_reopen_sealed(reopen),
            Assessment::new(100_000, 50_000),
        );
        let transcript = Transcript::seal(sentence).expect("valid sentence");
        let verdict = accept(&transcript).expect("valid skeleton axis pair");
        prop_assert!(
            matches!(verdict, Verdict::Unparsable { .. }),
            "expected Unparsable, got {:?}",
            verdict
        );
        if budget_overrun {
            prop_assert_eq!(verdict, Verdict::Unparsable { reason: UnparsableReason::BudgetExceeded });
        } else if reopen {
            prop_assert_eq!(verdict, Verdict::Unparsable { reason: UnparsableReason::CounterInductiveReopen });
        } else if zero_coverage {
            prop_assert_eq!(verdict, Verdict::Unparsable { reason: UnparsableReason::AxialCoverage });
        } else {
            prop_assert_eq!(verdict, Verdict::Unparsable { reason: UnparsableReason::MarginMissing });
        }
    }

    #[test]
    fn prop_diagnose_missing_poisons_and_theta_basin_independent(
        steps in prop::collection::vec(
            (arb_score(), arb_score(), arb_score(), arb_score(), any::<bool>(), -1000i64..=1000i64),
            1..=32
        ),
        alt_basin in arb_score(),
    ) {
        let readings: Vec<StepReading> = steps
            .iter()
            .map(|&(e, v, g, b, abs, r)| StepReading::new(e, v, g, b, abs, r))
            .collect();
        let view1 = diagnose(&Trajectory::seal(&readings)).observe();
        let view2 = diagnose(&Trajectory::seal(&readings)).observe();
        prop_assert_eq!(view1, view2);

        let alt_readings: Vec<StepReading> = steps
            .iter()
            .map(|&(e, v, g, _, abs, r)| StepReading::new(e, v, g, alt_basin, abs, r))
            .collect();
        let view_alt = diagnose(&Trajectory::seal(&alt_readings)).observe();
        prop_assert_eq!(view1.binding_axis, view_alt.binding_axis);
        prop_assert_eq!(view1.s_epi, view_alt.s_epi);
        prop_assert_eq!(view1.s_vital, view_alt.s_vital);
        prop_assert_eq!(view1.s_gov, view_alt.s_gov);

        let has_missing_epi = readings.iter().any(|s| s.s_epi == Score::Missing);
        let has_missing_gov = readings.iter().any(|s| s.s_gov == Score::Missing);
        if has_missing_epi || has_missing_gov {
            prop_assert_eq!(view1.binding_axis, None);
        }

        let first_abs = readings.iter().position(|s| s.absorbed).map(|i| i as u32);
        prop_assert_eq!(view1.absorbed, first_abs.is_some());
        prop_assert_eq!(view1.ruin_step, first_abs);
        if !readings.is_empty() && !view1.absorbed {
            let has_missing_vital = readings.iter().any(|s| s.s_vital == Score::Missing);
            if has_missing_vital {
                prop_assert_eq!(view1.binding_axis, None);
            } else if let (Score::Present(e), Score::Present(v), Score::Present(g)) =
                (view1.s_epi, view1.s_vital, view1.s_gov)
            {
                let expected = if e <= v && e <= g {
                    Axis::Epistemic
                } else if v <= g {
                    Axis::Vital
                } else {
                    Axis::Governance
                };
                prop_assert_eq!(view1.binding_axis, Some(expected));
            }
        }
    }

    #[test]
    fn prop_abi_fuzz_never_panics_and_preserves_out_on_error(
        instrument in -2i32..=3i32,
        missing_slot in -1i32..=9i32,
        l in -2_000_000_000i64..=2_000_000_000i64,
        u in -2_000_000_000i64..=2_000_000_000i64,
        c in -100i64..=100_000i64,
        r in -100i64..=100_000i64,
        tts in -100i64..=100_000i64,
        ttr_eff in -100i64..=100_000i64,
        flag_bits in 0u8..=63u8,
        tape in -1i32..=2i32,
        site in -1i32..=9i32,
        prop_code in -1i32..=5i32,
        g0 in -2i64..=(SCALE + 2),
    ) {
        let rec = StateRec {
            instrument,
            missing_slot,
            l,
            u,
            c,
            b: 0,
            r,
            tts,
            ttr_eff,
            n_eff: 1000,
            defect: 0,
            reversible: i32::from(flag_bits & 1),
            gov_corrupt: i32::from((flag_bits >> 1) & 1),
            vital_floor: i32::from((flag_bits >> 2) & 1),
            gov_floor: i32::from((flag_bits >> 3) & 1),
            tape,
            site,
            accept_threshold: 100_000,
            band_width: 50_000,
            guard: [g0, 500_000, 500_000, 500_000, 500_000, 500_000, 500_000],
            budget: 1000,
            elapsed: 10,
            proposition: prop_code,
            step_index: i32::from((flag_bits >> 4) & 1) - 1,
            lethal_omission: i32::from((flag_bits >> 5) & 1),
            reopen_sealed: 0,
            reserved1: [0u8; 48],
        };

        let mut r_out = sentinel_regime_out();
        // SAFETY: `rec` and `r_out` are live aligned stack locals of full size.
        let step_tag = unsafe { step_at(&rec, &mut r_out) };
        if step_tag < 0 {
            prop_assert!(regime_out_unchanged(&r_out));
        } else {
            prop_assert!((0..=5).contains(&step_tag));
        }

        let mut v_out = sentinel_verdict_out();
        // SAFETY: `rec` and `v_out` are live aligned stack locals of full size.
        let acc_tag = unsafe { accept_at(&rec, &mut v_out) };
        if acc_tag < 0 {
            prop_assert!(verdict_out_unchanged(&v_out));
        } else {
            prop_assert!((0..=3).contains(&acc_tag));
        }
    }

    #[test]
    fn prop_abi_null_misaligned_and_oversize_refused(
        offset in 1usize..8usize,
        bad_len in prop_oneof![(-10i32..0i32), ((MAX_STEPS as i32 + 1)..=(MAX_STEPS as i32 + 64))],
    ) {
        let scratch = AlignedScratch([0u8; 512]);
        let misaligned_rec = scratch.0.as_ptr().wrapping_add(offset) as *const StateRec;
        let misaligned_triples = scratch.0.as_ptr().wrapping_add(offset) as *const StepTriple;

        let mut r_out = sentinel_regime_out();
        // SAFETY: intentionally misaligned or null pointers must be rejected by alignment/null gates prior to read.
        prop_assert_eq!(unsafe { step_at(core::ptr::null(), &mut r_out) }, E_CALLER_CONTRACT);
        prop_assert!(regime_out_unchanged(&r_out));
        // SAFETY: `misaligned_rec` is refused before dereference.
        prop_assert_eq!(unsafe { step_at(misaligned_rec, &mut r_out) }, E_CALLER_CONTRACT);
        prop_assert!(regime_out_unchanged(&r_out));

        let mut v_out = sentinel_verdict_out();
        // SAFETY: null/misaligned inputs refused before dereference.
        prop_assert_eq!(unsafe { accept_at(core::ptr::null(), &mut v_out) }, E_CALLER_CONTRACT);
        prop_assert!(verdict_out_unchanged(&v_out));
        // SAFETY: `misaligned_rec` refused before dereference.
        prop_assert_eq!(unsafe { accept_at(misaligned_rec, &mut v_out) }, E_CALLER_CONTRACT);
        prop_assert!(verdict_out_unchanged(&v_out));

        let mut d_out = sentinel_diagnosis_out();
        let valid_triple = StepTriple {
            s_epi: 500_000,
            s_vital: 500_000,
            s_gov: 500_000,
            theta_basin: 500_000,
            absorbed: 0,
            reserve: 100,
        };
        // SAFETY: out-of-range length or misaligned input pointer refused without writing `d_out`.
        prop_assert_eq!(unsafe { diagnose_at(&valid_triple, bad_len, &mut d_out) }, E_CALLER_CONTRACT);
        prop_assert!(diagnosis_out_unchanged(&d_out));
        // SAFETY: `misaligned_triples` refused before dereference.
        prop_assert_eq!(unsafe { diagnose_at(misaligned_triples, 1, &mut d_out) }, E_CALLER_CONTRACT);
        prop_assert!(diagnosis_out_unchanged(&d_out));

        // SAFETY: null hash buffer refused cleanly.
        prop_assert_eq!(unsafe { vectors_hash_at(core::ptr::null_mut()) }, E_CALLER_CONTRACT);
    }

    #[test]
    fn prop_math_widen_before_seal_monotone_and_symmetric(
        a in -10_000_000i64..=10_000_000i64,
        width in 0i64..=10_000_000i64,
        meng1 in 0i64..=1_000_000i64,
        meng_delta in 0i64..=1_000_000i64,
        deff1 in UNIT..=(10 * UNIT),
        deff_delta in 0i64..=(10 * UNIT),
        loss1 in 0i64..=1_000_000i64,
    ) {
        let raw = Interval::new(a, a + width).expect("ordered interval");
        let w_id = widen_before_seal(raw, 0, UNIT, 0).expect("identity");
        prop_assert_eq!(w_id, raw);

        let w1 = widen_before_seal(raw, meng1, deff1, loss1).expect("widened 1");
        let w2 = widen_before_seal(raw, meng1 + meng_delta, deff1 + deff_delta, loss1).expect("widened 2");

        prop_assert!(w1.lo <= raw.lo && w1.hi >= raw.hi);
        prop_assert!(w2.lo <= w1.lo && w2.hi >= w1.hi);
        prop_assert_eq!(i128::from(w1.lo) + i128::from(w1.hi), i128::from(raw.lo) + i128::from(raw.hi));
        prop_assert_eq!(i128::from(w2.lo) + i128::from(w2.hi), i128::from(raw.lo) + i128::from(raw.hi));
    }

    #[test]
    fn prop_math_integer_root_exact_bounds(v in 0i128..=i128::MAX) {
        let r = integer_root(v).expect("non-negative root");
        prop_assert!(r >= 0);
        let sq = r.checked_mul(r).expect("r^2 <= v <= i128::MAX");
        prop_assert!(sq <= v);
        if let Some(next_sq) = (r + 1).checked_mul(r + 1) {
            prop_assert!(next_sq > v);
        }
    }

    #[test]
    fn prop_math_thm3_delay_doppler_tightening_and_monotonicity(
        mu_plus in 0i64..(UNIT - 1),
        mu_minus in 0i64..=(2 * UNIT),
        rho in 1i64..=UNIT,
        gamma_cap in (UNIT / 10)..=(10 * UNIT),
        gamma_extra in 0i64..=(5 * UNIT),
        lambda_max in (UNIT / 10)..=(10 * UNIT),
    ) {
        let plain = thm3_max_delay(mu_plus, mu_minus, rho, gamma_cap, lambda_max, false).expect("plain");
        let doppler = thm3_max_delay(mu_plus, mu_minus, rho, gamma_cap, lambda_max, true).expect("doppler");
        prop_assert!(doppler >= 0);
        prop_assert!(doppler <= plain);

        let stiffer = thm3_max_delay(mu_plus, mu_minus, rho, gamma_cap + gamma_extra, lambda_max, false).expect("stiffer");
        prop_assert!(stiffer <= plain);
    }

    #[test]
    fn prop_kish_deff_at_least_unit_and_scale_invariant(
        weights in prop::collection::vec(1i64..=5_000i64, 1..=24),
        k in 1i64..=100i64,
        uniform_w in 1i64..=10_000i64,
        uniform_len in 1usize..=20usize,
    ) {
        let scaled_inputs: Vec<i64> = weights.iter().map(|w| w * UNIT).collect();
        let d_base = kish_design_effect(&scaled_inputs).expect("valid weights");
        prop_assert!(d_base >= UNIT);

        let k_scaled: Vec<i64> = scaled_inputs.iter().map(|w| w * k).collect();
        let d_scaled = kish_design_effect(&k_scaled).expect("scaled weights");
        prop_assert_eq!(d_scaled, d_base);

        let uniform = vec![uniform_w; uniform_len];
        prop_assert_eq!(kish_design_effect(&uniform), Ok(UNIT));
    }

    #[test]
    fn prop_moulton_cluster_and_additive_deff_monotonicity(
        cluster_sizes in prop::collection::vec(1u64..=50u64, 1..=16),
        rho1 in 0i64..=UNIT,
        rho_delta in 0i64..=UNIT,
        weights in prop::collection::vec(1i64..=5_000i64, 1..=12),
    ) {
        let rho2 = (rho1 + rho_delta).min(UNIT);
        let d1 = moulton_cluster_deff(&cluster_sizes, rho1).expect("rho1 valid");
        let d2 = moulton_cluster_deff(&cluster_sizes, rho2).expect("rho2 valid");
        prop_assert!(d1 >= UNIT);
        prop_assert!(d2 >= d1);

        let scaled_weights: Vec<i64> = weights.iter().map(|w| w * UNIT).collect();
        let dk = kish_design_effect(&scaled_weights).expect("kish valid");
        let total = combine_deff_additive(d2, dk).expect("combined valid");
        prop_assert_eq!(i128::from(total), i128::from(d2) + i128::from(dk) - i128::from(UNIT));
        let product_majorant = (i128::from(d2) * i128::from(dk)) / i128::from(UNIT);
        prop_assert!(i128::from(total) <= product_majorant);
    }

    #[test]
    fn prop_chao1_and_chao2_monotonic_and_finite(
        n10 in 0u64..=5_000u64,
        n01 in 0u64..=5_000u64,
        m1 in 0u64..=1_000u64,
        m_delta in 0u64..=1_000u64,
        f1_extra in 0u64..=1_000u64,
    ) {
        let f1 = n10 + n01;
        let c1_lo = chao1_lower_bound(f1, m1).expect("chao1 finite");
        let c1_hi = chao1_lower_bound(f1 + f1_extra, m1).expect("chao1 higher f1");
        prop_assert!(c1_lo >= 0);
        prop_assert!(c1_hi >= c1_lo);

        let c2_base = chao2_two_list_lower_bound(n10, n01, m1).expect("chao2 finite even at m=0");
        prop_assert!(c2_base >= 0);
        if m1 > 0 {
            let c2_more_overlap = chao2_two_list_lower_bound(n10, n01, m1 + m_delta).expect("chao2");
            prop_assert!(c2_base >= c2_more_overlap);
        }
    }

    #[test]
    fn prop_leontief_epoch_state_aci(
        e1 in 0u64..=5u64,
        e2 in 0u64..=5u64,
        e3 in 0u64..=5u64,
        s1 in prop::array::uniform3(0i64..=SCALE),
        s2 in prop::array::uniform3(0i64..=SCALE),
        s3 in prop::array::uniform3(0i64..=SCALE),
        caps1 in prop::collection::vec(any::<u8>(), 0..=4),
        caps2 in prop::collection::vec(any::<u8>(), 0..=4),
        caps3 in prop::collection::vec(any::<u8>(), 0..=4),
    ) {
        let build = |e, s: [i64; 3], caps: &[u8]| {
            let mut st = LeontiefEpochState::new(e, s[0], s[1], s[2]);
            for &b in caps {
                st.spent_capabilities.consume([b; 32]);
            }
            st
        };
        let a = build(e1, s1, &caps1);
        let b = build(e2, s2, &caps2);
        let c = build(e3, s3, &caps3);

        // Commutative.
        let mut ab = a.clone();
        ab.join(&b);
        let mut ba = b.clone();
        ba.join(&a);
        prop_assert_eq!(&ab, &ba);

        // Associative.
        let mut ab_c = ab.clone();
        ab_c.join(&c);
        let mut bc = b.clone();
        bc.join(&c);
        let mut a_bc = a.clone();
        a_bc.join(&bc);
        prop_assert_eq!(ab_c, a_bc);

        // Idempotent & capability-monotone.
        let mut aa = a.clone();
        aa.join(&a);
        prop_assert_eq!(aa, a.clone());
        for &byte in caps1.iter().chain(caps2.iter()) {
            prop_assert!(ab.spent_capabilities.is_spent(&[byte; 32]));
        }
    }

    #[test]
    fn prop_rehab_lattice_aci_and_zero_probation_witness_weight(
        g1 in 0u64..=5u64,
        g2 in 0u64..=5u64,
        g3 in 0u64..=5u64,
        p1 in 0u8..=3u8,
        p2 in 0u8..=3u8,
        p3 in 0u8..=3u8,
        nominal_weight in 1u64..=1_000u64,
        t_base in 1u64..=1_000u64,
        q_fraud in 1usize..=5usize,
        q_extra in 1usize..=5usize,
        cond_active in 100_000i64..=900_000i64,
        cond_probation in 1i64..=99_999i64,
    ) {
        let map_phase = |p: u8| match p {
            0 => ExilePhase::Quarantined,
            1 => ExilePhase::Canary,
            2 => ExilePhase::Pharmacovigilance,
            _ => ExilePhase::Active,
        };
        let a = RehabLattice::at_state(g1, map_phase(p1));
        let b = RehabLattice::at_state(g2, map_phase(p2));
        let c = RehabLattice::at_state(g3, map_phase(p3));

        // ACI laws over RehabLattice.
        let mut ab = a.clone();
        ab.join(&b);
        let mut ba = b.clone();
        ba.join(&a);
        prop_assert_eq!(&ab, &ba);

        let mut ab_c = ab.clone();
        ab_c.join(&c);
        let mut bc = b.clone();
        bc.join(&c);
        let mut a_bc = a.clone();
        a_bc.join(&bc);
        prop_assert_eq!(ab_c, a_bc);

        let mut aa = a.clone();
        aa.join(&a);
        prop_assert_eq!(aa, a.clone());

        // Strictly zero witness weight outside Active.
        for phase in [ExilePhase::Quarantined, ExilePhase::Canary, ExilePhase::Pharmacovigilance] {
            let st = RehabLattice::at_state(1, phase);
            prop_assert!(!phase.has_witness_power());
            prop_assert_eq!(st.witness_weight(nominal_weight), 0);
            prop_assert!(phase.conductance_cap_millionths() < UNIT);
        }

        // Exponential backoff and zero cross-witness weight during Canary / Pharmacovigilance.
        let policy = RehabPolicy::new(q_fraud, q_fraud + q_extra, t_base).expect("policy");
        prop_assert!(policy.vigil_duration(2).expect("g2") >= policy.vigil_duration(1).expect("g1"));

        let w_active = SignedWitness {
            issuer: [1; 32],
            subject: [9; 32],
            epoch: 1,
            conductance_millionths: cond_active,
            digest: [7; 32],
        };
        let w_canary = SignedWitness {
            issuer: [2; 32],
            subject: [9; 32],
            epoch: 1,
            conductance_millionths: cond_probation,
            digest: [8; 32],
        };
        let w_vigil = SignedWitness {
            issuer: [3; 32],
            subject: [9; 32],
            epoch: 1,
            conductance_millionths: cond_probation,
            digest: [9; 32],
        };
        let med = cross_witness_median(
            &[9; 32],
            1,
            &[
                (&w_active, ExilePhase::Active),
                (&w_canary, ExilePhase::Canary),
                (&w_vigil, ExilePhase::Pharmacovigilance),
            ],
        )
        .expect("active witness yields median");
        prop_assert_eq!(med, cond_active);
    }

    #[test]
    fn prop_dtn_store_carry_backpressure_preserves_urgent_headroom(
        shed_threshold in 1usize..=8usize,
        extra_cap in 1usize..=8usize,
        events in prop::collection::vec((any::<bool>(), 0u64..=10u64, any::<bool>()), 1..=32),
    ) {
        let capacity = shed_threshold + extra_cap;
        let mut q = StoreCarryQueue::new(capacity, shed_threshold).expect("queue");
        let mut now: u64 = 1;
        for (idx, (urgent, ttl_delta, do_pop)) in events.into_iter().enumerate() {
            now += 1;
            let expiry = now + ttl_delta;
            if do_pop {
                if let Some(popped) = q.pop_ready(now) {
                    prop_assert!(popped.expiry_epoch >= now);
                }
            } else {
                let _ = q.purge_expired(now);
                let len_before = q.len();
                let res = q.enqueue(
                    StoreCarryBundle {
                        payload: idx,
                        expiry_epoch: expiry,
                        urgent,
                    },
                    now,
                );
                if !urgent && len_before >= shed_threshold {
                    prop_assert_eq!(res, Err(CrdtError::CapacityFull));
                }
            }
            prop_assert!(q.len() <= capacity);
        }
    }

    #[test]
    fn prop_hyparview_2pc_zero_orphan_stubs_and_leaked_locks(
        initial_active in 1usize..=4usize,
        ops in prop::collection::vec((1u8..=8u8, any::<bool>(), 0u8..=2u8, 1usize..=4usize), 1..=12),
    ) {
        let mut overlay = HyParViewOverlay::new(initial_active, 8).expect("overlay");
        let mut initial_peers = Vec::new();
        for idx in 1..=(initial_active as u8) {
            let pid = [idx; 32];
            overlay.seed_peer(pid);
            initial_peers.push(pid);
        }

        for (step_idx, (cand_byte, approve, outcome_mode, quorum)) in ops.into_iter().enumerate() {
            let proposal_id = (step_idx + 1) as u64;
            let cand = [cand_byte.saturating_add(20); 32];
            let displace = if overlay.active_len() >= initial_active {
                initial_peers.first().copied()
            } else {
                None
            };
            if overlay.propose_phase_a(proposal_id, cand, displace).is_err() {
                continue;
            }
            let _ = overlay.acquire_phase_b1_locks(proposal_id);
            for (v_idx, voter) in initial_peers.iter().enumerate() {
                let vote = approve && (v_idx < quorum);
                let _ = overlay.vote_phase_b1(proposal_id, *voter, vote);
            }
            match outcome_mode {
                0 => {
                    let _ = overlay.commit_phase_b2(proposal_id, quorum);
                }
                _ => {
                    let _ = overlay.abort_or_timeout(proposal_id);
                }
            }
            // Invariant after every terminal step: zero orphan stubs and zero leaked locks.
            prop_assert_eq!(overlay.pending_stubs_count(), 0);
            prop_assert_eq!(overlay.active_locks_count(), 0);
            prop_assert!(overlay.active_len() <= initial_active);
            if let Some(&first_active) = overlay.is_active(&cand).then_some(&cand) {
                if let Some(d) = displace {
                    if d != first_active && outcome_mode == 0 {
                        // Refresh displace tracker if swapped.
                        initial_peers[0] = first_active;
                    }
                }
            }
        }
    }

    #[test]
    fn prop_level1_cue_roundtrip_within_8_bits(
        reg_idx in 0u8..=5u8,
        ax_idx in 0u8..=3u8,
        algedonic in any::<bool>(),
        codd_idx in 0u8..=3u8,
    ) {
        let regime = match reg_idx {
            0 => RegimeTag::BufferedAction,
            1 => RegimeTag::ReversibleFractionalProbe,
            2 => RegimeTag::InwardAction,
            3 => RegimeTag::Quarantine,
            4 => RegimeTag::AlgedonicHalt,
            _ => RegimeTag::NoAction,
        };
        let binding_axis = match ax_idx {
            0 => BindingAxisTag::None,
            1 => BindingAxisTag::Epistemic,
            2 => BindingAxisTag::Vital,
            _ => BindingAxisTag::Governance,
        };
        let codd_status = match codd_idx {
            0 => CoddStatusTag::Known,
            1 => CoddStatusTag::ApplicableUnknown,
            2 => CoddStatusTag::Inapplicable,
            _ => CoddStatusTag::NotAskable,
        };
        let cue = Level1Cue {
            regime,
            binding_axis,
            algedonic,
            codd_status,
        };
        let packed = cue.pack();
        let decoded = Level1Cue::unpack(packed).expect("valid cue bits");
        prop_assert_eq!(decoded, cue);
    }

    #[test]
    fn prop_solar_watchdog_never_emits_fraud_and_bounds_kappa(
        readings in prop::collection::vec((-100_000i64..=1_200_000i64, -100_000i64..=1_200_000i64, any::<bool>()), 1..=16),
    ) {
        use std::sync::atomic::{AtomicU64, Ordering};
        static PROP_DB_ID: AtomicU64 = AtomicU64::new(0);

        let dir = std::env::temp_dir().join(format!(
            "stoch-proptest-solar-{}-{}",
            std::process::id(),
            PROP_DB_ID.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir_all(&dir).expect("create temp dir");
        {
            let ledger = stoch_custody::CustodyLedger::open(dir.join("solar.redb")).expect("open");
            let metrics = LockFreeMetrics::new();
            let mut engine = SolarWatchdogEngine::new(SolarWatchdogConfig::default_calibrated())
                .expect("valid default config");
            for (step, (v, soc, powered)) in readings.into_iter().enumerate() {
                let ts = (step + 1) as u64;
                let out = engine
                    .step_reading(
                        SolarReading {
                            voltage_scaled: v,
                            soc_scaled: soc,
                            bus_powered: powered,
                        },
                        ts,
                        ts,
                        &ledger,
                        &metrics,
                    )
                    .expect("watchdog step");
                prop_assert!((0..=UNIT).contains(&out.energy_kappa_scale_millionths));
                if !powered {
                    prop_assert_eq!(out.voltage_codd, CoddValue::ApplicableUnknown);
                }
            }
        }
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn prop_fde_info_join_aci_and_filter1_blocks_gaps_and_gluts(
        a_idx in 0u8..=3u8,
        b_idx in 0u8..=3u8,
        c_idx in 0u8..=3u8,
        v1 in -1_000_000i64..=1_000_000i64,
        v2 in -1_000_000i64..=1_000_000i64,
        kind1 in 0u8..=3u8,
        kind2 in 0u8..=3u8,
    ) {
        let map_fde = |idx: u8| match idx {
            0 => FdeState::Neither,
            1 => FdeState::True,
            2 => FdeState::False,
            _ => FdeState::Both,
        };
        let a = map_fde(a_idx);
        let b = map_fde(b_idx);
        let c = map_fde(c_idx);

        // Belnap-Dunn information join <=_k is associative, commutative, idempotent.
        prop_assert_eq!(a.info_join(b), b.info_join(a));
        prop_assert_eq!(a.info_join(b).info_join(c), a.info_join(b.info_join(c)));
        prop_assert_eq!(a.info_join(a), a);

        let map_codd = |kind: u8, v: i64| match kind {
            0 => CoddValue::Known(v),
            1 => CoddValue::ApplicableUnknown,
            2 => CoddValue::Inapplicable,
            _ => CoddValue::NotAskable,
        };
        let c1 = Filter1Claim::new(map_codd(kind1, v1), a, LocalConsistency::Consistent);
        let c2 = Filter1Claim::new(map_codd(kind2, v2), b, LocalConsistency::Consistent);
        let merged = c1.merge_claim(c2);

        if matches!(c1.value, CoddValue::NotAskable) || matches!(c2.value, CoddValue::NotAskable) {
            prop_assert_eq!(merged.value, CoddValue::NotAskable);
            prop_assert!(!merged.can_seal());
        }
        if merged.fde.is_gap() || merged.fde.is_glut() || !merged.value.is_known() {
            prop_assert!(!merged.can_seal());
        }
        if let (CoddValue::Known(x), CoddValue::Known(y)) = (c1.value, c2.value) {
            if x != y {
                prop_assert_eq!(merged.fde, FdeState::Both);
                prop_assert_eq!(merged.consistency, LocalConsistency::Contested);
                prop_assert!(!merged.can_seal());
            }
        }
    }

    #[test]
    fn prop_lpa2v_certainty_and_contradiction_within_unit_bounds(
        mu in 0i64..=UNIT,
        lambda in 0i64..=UNIT,
    ) {
        let ev = AnnotatedEvidence::new(mu, lambda).expect("mu and lambda inside [0, UNIT]");
        let gc = ev.certainty_degree();
        let gct = ev.contradiction_degree();
        prop_assert!((-UNIT..=UNIT).contains(&gc));
        prop_assert!((-UNIT..=UNIT).contains(&gct));
        prop_assert_eq!(ev.is_overdetermined(), mu + lambda > UNIT);
        prop_assert_eq!(ev.is_underdetermined(), mu + lambda < UNIT);
    }

    #[test]
    fn prop_part4_deff_additive_le_product_bound_and_bitemporal_min(
        effects in prop::collection::vec(UNIT..=(20 * UNIT), 1..=6),
        ex_ante in 0i64..=UNIT,
        kairos in 0i64..=UNIT,
        ex_post in 0i64..=UNIT,
        gov in 0i64..=UNIT,
        pivot in 1i64..UNIT,
        mag in 0i64..=10_000_000i64,
    ) {
        let additive = epistemic::compose_design_effects(&effects).expect("additive deff");
        let bound = epistemic::design_effect_product_bound(&effects).expect("product majorant");
        prop_assert!(additive >= UNIT);
        prop_assert!(additive <= bound);

        let bt = governance::Bitemporal::new(ex_ante, kairos, ex_post).expect("valid readings");
        prop_assert_eq!(bt.responsibility(), ex_ante.min(kairos).min(ex_post));

        let sign = governance::differential_sign(gov, pivot).expect("valid sign");
        let diff = governance::signed_differential(mag, gov, pivot).expect("signed diff");
        prop_assert_eq!(diff, mag * sign.factor());
        if gov == pivot {
            prop_assert_eq!(diff, 0);
        }
    }

    #[test]
    fn prop_cross_witness_reconcile_emits_verified_equivocation_certificates(
        c1 in 1i64..=SCALE,
        c2 in 1i64..=SCALE,
        c3 in 1i64..=SCALE,
        d1 in any::<u8>(),
        d2 in any::<u8>(),
    ) {
        let subject = [99u8; 32];
        let w1 = SignedWitness {
            issuer: [1u8; 32],
            subject,
            epoch: 7,
            conductance_millionths: c1,
            digest: [d1; 32],
        };
        let w2_a = SignedWitness {
            issuer: [2u8; 32],
            subject,
            epoch: 7,
            conductance_millionths: c2,
            digest: [d1; 32],
        };
        let w2_b = SignedWitness {
            issuer: [2u8; 32],
            subject,
            epoch: 7,
            conductance_millionths: c3,
            digest: [d2; 32],
        };

        let forward = cross_witness_reconcile(
            &subject,
            7,
            &[
                (&w1, ExilePhase::Active),
                (&w2_a, ExilePhase::Active),
                (&w2_b, ExilePhase::Active),
            ],
        )
        .expect("reconciles");
        let reverse = cross_witness_reconcile(
            &subject,
            7,
            &[
                (&w2_b, ExilePhase::Active),
                (&w2_a, ExilePhase::Active),
                (&w1, ExilePhase::Active),
            ],
        )
        .expect("reconciles reverse");

        prop_assert_eq!(&forward, &reverse);
        if (c2, [d1; 32]) != (c3, [d2; 32]) {
            match forward {
                WitnessOutcome::Conflicted {
                    conservative_min_millionths,
                    certificates,
                } => {
                    prop_assert_eq!(conservative_min_millionths, c1.min(c2).min(c3));
                    prop_assert_eq!(certificates.len(), 1);
                    let cert: &EquivocationCertificate = &certificates[0];
                    prop_assert_eq!(cert.verify(), Ok(()));
                    prop_assert!(cert.claim_a < cert.claim_b);
                }
                WitnessOutcome::Consistent { .. } => {
                    prop_assert!(false, "conflicting claims must surface WitnessOutcome::Conflicted");
                }
            }
        } else {
            let is_consistent = matches!(forward, WitnessOutcome::Consistent { .. });
            prop_assert!(is_consistent);
        }
    }
}
