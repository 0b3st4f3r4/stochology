//! The coherence table: vectors whose three margins differ **by axis**.
//!
//! The normative fixture cannot carry these. `01` §6.4 gives an `fn = 1` row three
//! *reading counts* and a single `margin` column — "margem do sítio mais fraco" — so
//! every transcript the fixture builds has the same weakest value on all three axes,
//! and a `Boundary` that names any declared site agrees with the fold by accident. That
//! is exactly the hole the adversarial review opened: `epi = 101000` with
//! `vital = gov = 900000`, declared as `P2Vital` on `T4`, used to return
//! `Boundary { site: T4, margin: 101000 }` — a vital site printed beside a margin that
//! came from the epistemic axis.
//!
//! B.1 line 163 says the frontier verdict "names that site with its margin", and the
//! margin it names is the fold. So a declared site is coherent exactly when its own
//! axis attains the fold; a tie is coherent on every tied axis, and a strict excess is
//! not a boundary at all. The table below is the missing case, written as a table so
//! the next reader adds a row instead of a test.

use stoch_core::{
    accept, Assessment, AxialReadings, AxisReading, Bounds, Budget, Capacity, ContingencyTape,
    ControlTape, Declared, Instrument, Params, Proposition, Score, Sentence, SevenReadings, Site,
    State, Thresholds, Transcript, UnparsableReason, Verdict,
};

const THRESHOLD: i64 = 100_000;
const BAND: i64 = 50_000;

/// One vector of the coherence table.
struct Vector {
    id: &'static str,
    /// The weakest margin of each axis, in fold order: epistemic, vital, governance.
    margins: [i64; 3],
    /// The site the sentence declares.
    site: Site,
    /// The proposition that site must be coherent with, and its axis.
    proposition: Proposition,
    /// What the predicate must answer.
    expected: Verdict,
}

/// The table. Every row has three distinct margins or a deliberate tie.
const TABLE: &[Vector] = &[
    // The fold is epistemic and the declared site is epistemic: coherent, and the
    // verdict names the pair.
    Vector {
        id: "c01",
        margins: [101_000, 900_000, 900_000],
        site: Site::T1,
        proposition: Proposition::P1Epistemic,
        expected: Verdict::Boundary {
            site: Site::T1,
            margin: 101_000,
        },
    },
    // The review's repro, verbatim: the fold is epistemic, the declared site is vital
    // with a margin of its own nine times the fold. Not a boundary; an ill-formed pair.
    Vector {
        id: "c02",
        margins: [101_000, 900_000, 900_000],
        site: Site::T4,
        proposition: Proposition::P2Vital,
        expected: Verdict::Unparsable {
            reason: UnparsableReason::Skeleton,
        },
    },
    // The fold is vital and the declared site is epistemic: the incoherence runs the
    // other way, and it is refused the same way.
    Vector {
        id: "c03",
        margins: [900_000, 101_000, 900_000],
        site: Site::T1,
        proposition: Proposition::P1Epistemic,
        expected: Verdict::Unparsable {
            reason: UnparsableReason::Skeleton,
        },
    },
    // The fold is vital and the declared site is vital: coherent.
    Vector {
        id: "c04",
        margins: [900_000, 101_000, 900_000],
        site: Site::T5,
        proposition: Proposition::P2Vital,
        expected: Verdict::Boundary {
            site: Site::T5,
            margin: 101_000,
        },
    },
    // The fold is governance and the declared site is governance: coherent.
    Vector {
        id: "c05",
        margins: [900_000, 900_000, 101_000],
        site: Site::T6,
        proposition: Proposition::P3Governance,
        expected: Verdict::Boundary {
            site: Site::T6,
            margin: 101_000,
        },
    },
    // Governance site against an epistemic fold: incoherent.
    Vector {
        id: "c06",
        margins: [101_000, 900_000, 900_000],
        site: Site::T6,
        proposition: Proposition::P3Governance,
        expected: Verdict::Unparsable {
            reason: UnparsableReason::Skeleton,
        },
    },
    // Three equal margins: every axis attains the fold, so every site of every axis is
    // coherent. Equality is attainment, not an exception to it.
    Vector {
        id: "c07",
        margins: [101_000, 101_000, 101_000],
        site: Site::T1,
        proposition: Proposition::P1Epistemic,
        expected: Verdict::Boundary {
            site: Site::T1,
            margin: 101_000,
        },
    },
    Vector {
        id: "c08",
        margins: [101_000, 101_000, 101_000],
        site: Site::T4,
        proposition: Proposition::P2Vital,
        expected: Verdict::Boundary {
            site: Site::T4,
            margin: 101_000,
        },
    },
    Vector {
        id: "c09",
        margins: [101_000, 101_000, 101_000],
        site: Site::T7,
        proposition: Proposition::P3Governance,
        expected: Verdict::Boundary {
            site: Site::T7,
            margin: 101_000,
        },
    },
    // Two axes tied at the fold, the third above it: both tied sites are coherent.
    Vector {
        id: "c10",
        margins: [101_000, 101_000, 900_000],
        site: Site::T4,
        proposition: Proposition::P2Vital,
        expected: Verdict::Boundary {
            site: Site::T4,
            margin: 101_000,
        },
    },
];

fn sentence(vector: &Vector) -> Sentence {
    Sentence::new(
        ControlTape::Attested,
        ContingencyTape::new(7),
        Bounds::new(120, 400, 1000, 0),
        Capacity::new(200, 0, 1000),
        Budget::new(1000, 10),
        AxialReadings::new(
            AxisReading::new(1, Score::Present(vector.margins[0])),
            AxisReading::new(1, Score::Present(vector.margins[1])),
            AxisReading::new(1, Score::Present(vector.margins[2])),
        ),
        Declared::new(Some(vector.proposition), Some(vector.site), Some(0)),
        Assessment::new(THRESHOLD, BAND),
    )
}

#[test]
fn the_coherence_table_answers_as_declared() {
    assert_eq!(TABLE.len(), 10, "ten vectors, each one named");
    for vector in TABLE {
        let transcript = Transcript::seal(sentence(vector))
            .unwrap_or_else(|error| panic!("{}: the door refused the vector: {:?}", vector.id, error));
        assert_eq!(
            accept(&transcript),
            Ok(vector.expected),
            "{}: {:?} declared on {:?} with margins {:?}",
            vector.id,
            vector.proposition,
            vector.site,
            vector.margins
        );
    }
}

#[test]
fn the_review_repro_is_not_a_boundary_any_more() {
    // The case the review reported, written without the table so the regression is
    // also readable as a sentence: the defect must not be reachable from the public
    // API at all.
    let transcript = Transcript::seal(sentence(&TABLE[1])).expect("seals");
    let verdict = accept(&transcript).expect("a verdict");
    assert_ne!(verdict.tag(), 1, "tag 1 is Boundary: {:?}", verdict);
    assert_eq!(
        verdict,
        Verdict::Unparsable {
            reason: UnparsableReason::Skeleton
        }
    );
}

#[test]
fn a_margin_off_the_scale_is_refused_by_both_doors() {
    // F2: the range `Score::Present` documents is imposed at both seals, so the value
    // that used to fold into a comfortable interior verdict never becomes a reading.
    let off_scale = 9_000_000_000_000i64;

    let mut vector_sentence = sentence(&TABLE[0]);
    vector_sentence.axes.epi.weakest = Score::Present(off_scale);
    assert_eq!(
        Transcript::seal(vector_sentence).err(),
        Some(stoch_core::Error::ScaleOverflow),
        "the predicate's door refuses it"
    );

    let state = State::seal(
        ControlTape::Attested,
        Bounds::new(400, 800, 1000, 0),
        Capacity::new(200, 0, 1000),
        stoch_core::Guards::new(Instrument::Sealed(SevenReadings::new([off_scale; 7]))),
        Params::new(10, 10, true, false, false, false, Thresholds::new(THRESHOLD, BAND)),
    );
    assert_eq!(
        state.err(),
        Some(stoch_core::Error::ScaleOverflow),
        "the operator's door refuses it"
    );

    // A margin on the scale still seals on both sides.
    let mut on_scale = sentence(&TABLE[0]);
    on_scale.axes.vital.weakest = Score::Present(stoch_core::SCALE);
    assert!(Transcript::seal(on_scale).is_ok());
    assert!(State::seal(
        ControlTape::Attested,
        Bounds::new(400, 800, 1000, 0),
        Capacity::new(200, 0, 1000),
        stoch_core::Guards::new(Instrument::Sealed(SevenReadings::new([stoch_core::SCALE; 7]))),
        Params::new(10, 10, true, false, false, false, Thresholds::new(THRESHOLD, BAND)),
    )
    .is_ok());
}
