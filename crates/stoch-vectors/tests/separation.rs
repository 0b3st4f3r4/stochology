//! The three separation tests of Chapter 41 §4, at the harness level.
//!
//! The core's own `tests/separation.rs` proves the separation at the type level
//! (what each operator can *see*). These tests prove it at the behavioural level
//! required by the chapter: two admissible transcripts that share the sealed
//! state of one step return one regime; two that share tape and readings but not
//! the contingency tape return one verdict; and a sealed verdict is a fold after
//! the record was frozen, not a switch that can reach back into a step.

use stoch_core::{
    accept, diagnose, step, Assessment, AxialReadings, Axis, AxisReading, Budget, Bounds,
    Capacity, Constraint, ControlTape, ContingencyTape, Declared, Error, Guards, Instrument,
    Params, Proposition, Regime, Score, Sentence, SevenReadings, Site, StepReading, Thresholds,
    Trajectory, Transcript, Verdict,
};

const THRESHOLD: i64 = 100_000;
const BAND: i64 = 50_000;

fn guards(margin: i64) -> Guards {
    Guards::new(Instrument::Sealed(SevenReadings::new([margin; 7])))
}

fn params() -> Params {
    Params::new(
        10,
        10,
        true,
        false,
        false,
        false,
        Thresholds::new(THRESHOLD, BAND),
    )
}

fn state(l: i64, u: i64, c: i64, reserve: i64, margin: i64) -> stoch_core::State {
    stoch_core::State::seal(
        ControlTape::Attested,
        Bounds::new(l, u, 1000, 0),
        Capacity::new(c, 0, reserve),
        guards(margin),
        params(),
    )
    .expect("the state is admissible")
}

fn readings(count: usize, weakest: i64) -> AxisReading {
    AxisReading::new(count, Score::Present(weakest))
}

fn sentence(
    contingency: u64,
    epi: i64,
    vital: i64,
    gov: i64,
    declared: Declared,
) -> Sentence {
    Sentence::new(
        ControlTape::Attested,
        ContingencyTape::new(contingency),
        Bounds::new(200, 400, 1000, 0),
        Capacity::new(120, 0, 900),
        Budget::new(1000, 10),
        AxialReadings::new(readings(1, epi), readings(1, vital), readings(1, gov)),
        declared,
        Assessment::new(THRESHOLD, BAND),
    )
}

fn collapse_declared() -> Declared {
    Declared::new(Proposition::from_code(1), Site::from_code(1), Some(0))
}

/// §4, prefix test: the regime is a function of the sealed state of this step.
///
/// Two states that differ only in the reserve — a number that records where the
/// trajectory has already been — return the same regime, because `step` is given
/// the sealed state and nothing else. The state objects are *not* equal: the
/// difference is there, and the regime does not read it.
#[test]
fn the_regime_does_not_read_a_prefix() {
    let shallow = state(200, 400, 120, 0, 500_000);
    let deep = state(200, 400, 120, 900, 500_000);

    assert_ne!(shallow, deep, "the two states carry different histories");
    assert_eq!(
        step(&shallow).expect("admissible"),
        step(&deep).expect("admissible"),
        "the same sealed state returns the same regime whatever the prefix"
    );
    assert_eq!(step(&deep).expect("admissible"), Regime::BufferedAction);

    // The same history, but written into a field the operator *does* read,
    // moves the regime. A prefix that is meant to matter must be declared in
    // the sealed state before the step; it cannot be inferred afterwards.
    // (400 - 120) / (400 - 50) = 800_000.
    let constrained = state(50, 400, 120, 900, 500_000);
    assert_eq!(
        step(&constrained).expect("admissible"),
        Regime::ReversibleFractionalProbe { fraction: 800_000 },
        "a declared difference in the extent is read"
    );
    assert_ne!(
        step(&shallow).expect("admissible"),
        step(&constrained).expect("admissible"),
        "history the operator must see is exactly the declared part of the state"
    );

    // And a history that was never attested cannot reach the operator at all:
    // the tape is refused at the seal, so no undeclared prefix can be smuggled
    // past this test by building the state another way.
    let unwritten = stoch_core::State::seal(
        ControlTape::Unattested,
        Bounds::new(200, 400, 1000, 0),
        Capacity::new(120, 0, 900),
        guards(500_000),
        params(),
    );
    assert_eq!(unwritten.err(), Some(Error::TapeUnsealed));
}

/// §4, contingency test: the verdict reads the tape and the readings, never the
/// contingency record. Substituting the record leaves every verdict shape fixed.
#[test]
fn the_verdict_reads_no_contingency_tape() {
    let cases: [(i64, i64, i64, Declared); 4] = [
        (500_000, 500_000, 500_000, Declared::none()),
        // T1 is the epistemic site, and 120_000 is the epistemic margin, so the pair
        // is coherent and the verdict is a boundary. The case names the site that
        // carries the fold (B.1 line 163), not any site the sentence happens to name.
        (120_000, 500_000, 500_000, Declared::new(None, Site::from_code(1), None)),
        (50_000, 500_000, 500_000, collapse_declared()),
        (500_000, 0, 500_000, Declared::none()),
    ];
    let names = ["interior", "boundary", "collapse", "unparsable"];

    for (index, (epi, vital, gov, declared)) in cases.iter().enumerate() {
        let first = Transcript::seal(sentence(7, *epi, *vital, *gov, *declared))
            .expect("the skeleton is admissible");
        let second = Transcript::seal(sentence(1_000_003, *epi, *vital, *gov, *declared))
            .expect("the skeleton is admissible");

        assert_ne!(
            first.contingency(),
            second.contingency(),
            "{}: the two records really differ",
            names[index]
        );
        assert_eq!(
            accept(&first),
            accept(&second),
            "{}: the verdict does not move with the contingency tape",
            names[index]
        );
    }

    // The verdict histogram of the four cases is the one the states declare, so
    // the equality above is not the equality of four errors.
    let verdicts: Vec<Verdict> = cases
        .iter()
        .map(|(epi, vital, gov, declared)| {
            accept(
                &Transcript::seal(sentence(7, *epi, *vital, *gov, *declared))
                    .expect("admissible"),
            )
            .expect("no skeleton error")
        })
        .collect();
    assert_eq!(verdicts[0], Verdict::Interior);
    assert!(matches!(verdicts[1], Verdict::Boundary { .. }));
    assert!(matches!(verdicts[2], Verdict::Collapse { .. }));
    assert!(matches!(verdicts[3], Verdict::Unparsable { .. }));
}

/// §4, seal test: the closing report is a fold over a frozen record.
///
/// The verdict may be reported, and reporting it changes nothing: the record is
/// immutable behind `&Transcript`, the fold is a pure function of it, and a step
/// that must change is a *new* sentence. The first transcript's verdict is
/// therefore still what it was after the second was built — the revision is a
/// new declaration and not a retroactive one.
#[test]
fn a_sealed_verdict_does_not_re_enter_the_operator() {
    let frozen = Transcript::seal(sentence(
        7,
        50_000,
        500_000,
        500_000,
        collapse_declared(),
    ))
    .expect("admissible");
    let reported = accept(&frozen).expect("no skeleton error");
    assert_eq!(
        reported,
        Verdict::Collapse {
            proposition: Proposition::P1Epistemic,
            site: Site::T1,
            step: 0,
        },
        "the closing report names its step, proposition and site"
    );

    // Reporting twice is the same report.
    assert_eq!(accept(&frozen), Ok(reported));
    assert_eq!(frozen, frozen.clone());

    // A revision is a second transcript, and building it cannot revise the first.
    let revised = Transcript::seal(sentence(7, 500_000, 500_000, 500_000, collapse_declared()))
        .expect("admissible");
    assert_eq!(accept(&revised), Ok(Verdict::Interior));
    assert_eq!(
        accept(&frozen),
        Ok(reported),
        "the sealed report is what it was before the revision existed"
    );
    assert_ne!(accept(&frozen), accept(&revised));

    // The same holds for the fold over a finished trajectory: observing it does
    // not consume it, and the second observation is the first.
    let trajectory = [
        StepReading::new(Score::Present(500_000), Score::Present(50_000), Score::Present(500_000), false, 900),
        StepReading::new(Score::Present(500_000), Score::Missing, Score::Present(500_000), true, 0),
    ];
    let diagnosis = diagnose(&Trajectory::seal(&trajectory));
    let first = diagnosis.observe();
    let second = diagnosis.observe();
    // After absorption the vital coordinate is a barrier, not a reading, so the
    // fold cannot name it: the tie between the two live axes goes to the earlier.
    assert_eq!(first.binding_axis, Some(Axis::Epistemic));
    assert_eq!(first.binding_axis, second.binding_axis);
    assert_eq!(first.ruin_step, Some(1));
    assert_eq!(first.absorbed, true);

    // The diagnostic verdict is not a control law: it names an axis and trips
    // nothing. The only brake the operator knows is the halt regime the step
    // returns on its own state, and it is reached without the diagnosis.
    let halted = stoch_core::State::seal(
        ControlTape::Attested,
        Bounds::new(200, 400, 1000, 0),
        Capacity::new(120, 0, 900),
        guards(500_000),
        Params::new(10, 20, true, false, false, false, Thresholds::new(THRESHOLD, BAND)),
    )
    .expect("admissible");
    assert_eq!(
        step(&halted).expect("admissible"),
        Regime::AlgedonicHalt {
            cause: stoch_core::HaltCause::SurvivalLatency
        }
    );
    assert_eq!(
        step(&halted).expect("admissible").constraint(),
        None,
        "the halt is a cause, not a constraint"
    );
    assert_eq!(Regime::InwardAction { breached: Constraint::VitalFloor }.constraint(), Some(Constraint::VitalFloor));
}
