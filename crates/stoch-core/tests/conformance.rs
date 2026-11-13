//! Conformance over the normative fixture `crates/stoch-core/vectors.kv`.
//!
//! Every one of the 50 data rows is executed against the public API. The test does
//! not read the expectation columns as documentation: it runs the row and compares
//! what came back with what the fixture declares, and then asserts that the corpus
//! as a whole covers every value of every closed set in `01` §6.2.
//!
//! Record keys, `01` §6.4 (33 keys, one record per line, `key=value` pairs):
//! `id fn tape instrument missing_slot tts ttr l u c r reversible gov_corrupt
//!  vital_floor gov_floor epi vital gov site proposition step_index m_epi m_vital
//!  m_gov absorbed expect_variant expect_cause expect_constraint expect_fraction
//!  expect_verdict expect_binding_axis expect_reason expect_error`.
//!
//! `fn` selects the entry point: `0` step, `1` accept, `2` diagnose, `3` seal. A key
//! a record does not name is absent (`-1`), which the harness maps to the
//! absent-or-neutral value the type requires and never to a measurement.

use stoch_core::{
    accept, diagnose, probe_fraction, step, Assessment, AxisReading, AxialReadings, Bounds, Budget,
    Capacity, Constraint, ContingencyTape, ControlTape, Declared, Error, Guards, HaltCause,
    Instrument, MissingSlot, Params, Proposition, Score, Sentence, SevenReadings, Site,
    State, StepReading, Thresholds, Trajectory, Transcript, UnparsableReason, Verdict,
    VECTORS_DATA_ROWS,
};

const HEADER_COLUMNS: usize = 33;

// Column indices, in file order.
const C_ID: usize = 0;
const C_FN: usize = 1;
const C_TAPE: usize = 2;
const C_INSTRUMENT: usize = 3;
const C_MISSING_SLOT: usize = 4;
const C_TTS: usize = 5;
const C_TTR: usize = 6;
const C_L: usize = 7;
const C_U: usize = 8;
const C_C: usize = 9;
const C_R: usize = 10;
const C_REVERSIBLE: usize = 11;
const C_GOV_CORRUPT: usize = 12;
const C_VITAL_FLOOR: usize = 13;
const C_GOV_FLOOR: usize = 14;
const C_EPI: usize = 15;
const C_VITAL: usize = 16;
const C_GOV: usize = 17;
const C_SITE: usize = 18;
const C_PROPOSITION: usize = 19;
const C_STEP_INDEX: usize = 20;
const C_M_EPI: usize = 21;
const C_M_VITAL: usize = 22;
const C_M_GOV: usize = 23;
const C_ABSORBED: usize = 24;
const C_EXPECT_VARIANT: usize = 25;
const C_EXPECT_CAUSE: usize = 26;
const C_EXPECT_CONSTRAINT: usize = 27;
const C_EXPECT_FRACTION: usize = 28;
const C_EXPECT_VERDICT: usize = 29;
const C_EXPECT_BINDING: usize = 30;
const C_EXPECT_REASON: usize = 31;
const C_EXPECT_ERROR: usize = 32;

/// The fixture, embedded so the test cannot pass against a different file on disk.
const FIXTURE: &str = include_str!("../vectors.kv");

struct Row {
    id: String,
    values: Vec<i64>,
}

impl Row {
    fn at(&self, column: usize) -> i64 {
        self.values[column]
    }

    fn flag(&self, column: usize) -> bool {
        self.at(column) == 1
    }
}

/// The record keys, in the order the fixture writes them.
const HEADER_KEYS: [&str; HEADER_COLUMNS] = [
    "id",
    "fn",
    "tape",
    "instrument",
    "missing_slot",
    "tts",
    "ttr",
    "l",
    "u",
    "c",
    "r",
    "reversible",
    "gov_corrupt",
    "vital_floor",
    "gov_floor",
    "epi",
    "vital",
    "gov",
    "site",
    "proposition",
    "step_index",
    "m_epi",
    "m_vital",
    "m_gov",
    "absorbed",
    "expect_variant",
    "expect_cause",
    "expect_constraint",
    "expect_fraction",
    "expect_verdict",
    "expect_binding_axis",
    "expect_reason",
    "expect_error",
];

/// Parse the fixture as it is written: one record per line, `key=value` pairs
/// separated by a single space, `#` opening a comment and a blank line ignored.
///
/// This is a second, independent reader of the same bytes — the crate's own parser
/// is not a dependency of the core — so a grammar the writer and the harness agree
/// on but the fixture does not obey still fails here.
fn rows() -> Vec<Row> {
    let mut rows = Vec::new();
    for (index, raw) in FIXTURE.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let number = index + 1;
        let mut id = String::new();
        let mut values = vec![-1i64; HEADER_COLUMNS];
        let mut seen = [false; HEADER_COLUMNS];
        for field in line.split(' ') {
            let (key, spelling) = field
                .split_once('=')
                .unwrap_or_else(|| panic!("line {number}: {field:?} is not key=value"));
            let column = HEADER_KEYS
                .iter()
                .position(|name| *name == key)
                .unwrap_or_else(|| panic!("line {number}: unknown key {key:?}"));
            assert!(!seen[column], "line {number}: key {key:?} appears twice");
            seen[column] = true;
            if column == C_ID {
                id = spelling.to_string();
                continue;
            }
            values[column] = spelling
                .parse::<i64>()
                .unwrap_or_else(|_| panic!("line {number}: key {key:?} is not an integer: {spelling:?}"));
        }
        assert!(!id.is_empty(), "line {number}: record without an id");
        assert!(
            (0..=3).contains(&values[C_FN]),
            "line {number}: fn={} is not one of the four entry points",
            values[C_FN]
        );
        rows.push(Row { id, values });
    }
    rows
}

fn tape_of(value: i64) -> ControlTape {
    if value == 1 {
        ControlTape::Attested
    } else {
        ControlTape::Unattested
    }
}

fn constraints_of(value: i64) -> Constraint {
    if value == 1 {
        Constraint::GovernanceFloor
    } else {
        Constraint::VitalFloor
    }
}

fn cause_of(value: i64) -> HaltCause {
    match value {
        0 => HaltCause::SurvivalLatency,
        1 => HaltCause::GovernanceChannel,
        2 => HaltCause::IrreversibleProbeRefused,
        other => panic!("no halt cause {}", other),
    }
}

fn site_of(value: i64) -> Option<Site> {
    if value <= 0 {
        None
    } else {
        Some(Site::from_code(value as u8).expect("site in 1..7"))
    }
}

fn proposition_of(value: i64) -> Option<Proposition> {
    if value <= 0 {
        None
    } else {
        Some(Proposition::from_code(value).expect("proposition in 1..3"))
    }
}

fn axis_of(value: i64) -> Option<stoch_core::Axis> {
    match value {
        0 => None,
        1 => Some(stoch_core::Axis::Epistemic),
        2 => Some(stoch_core::Axis::Vital),
        3 => Some(stoch_core::Axis::Governance),
        other => panic!("no axis {}", other),
    }
}

fn score_of(value: i64) -> Score {
    if value < 0 {
        Score::Missing
    } else {
        Score::Present(value)
    }
}

fn readings(margin: i64) -> SevenReadings {
    SevenReadings::new([margin.max(0); 7])
}

/// What one row produced, so that the corpus-level coverage can be counted from the
/// evidence rather than from the expectation columns.
#[derive(Default)]
struct Observed {
    regime_tags: [usize; 6],
    verdict_tags: [usize; 4],
    collapse_sites: [usize; 8],
    slots: [usize; 8],
    binding: [usize; 4],
    errors: Vec<i32>,
    saw_q1_before_q2: bool,
    saw_q2_before_q3: bool,
    saw_channel_and_floor: bool,
    saw_extent_below_capacity: bool,
    saw_extent_equal_capacity: bool,
    saw_upper_equal_capacity: bool,
    saw_latency_equal: bool,
    saw_truncation: bool,
    saw_underflow: bool,
    saw_irreversible_probe: bool,
    saw_absorbed: bool,
    saw_missing: bool,
    saw_quarantine_every_slot: [bool; 8],
    fractions: Vec<i64>,
}

fn run_step(row: &Row, seen: &mut Observed) {
    let instrument = if row.at(C_INSTRUMENT) == 1 {
        let slot = row.at(C_MISSING_SLOT) as u8;
        Instrument::Incomplete(MissingSlot::new(slot).expect("slot in 1..7"))
    } else {
        Instrument::Sealed(readings(row.at(C_M_EPI).max(0)))
    };
    let bounds = Bounds::new(row.at(C_L), row.at(C_U), 1000, 0);
    let capacity = Capacity::new(row.at(C_C), 0, row.at(C_R).max(0));
    let guards = Guards::new(instrument);
    let params = Params::new(
        row.at(C_TTS),
        row.at(C_TTR),
        row.flag(C_REVERSIBLE),
        row.flag(C_VITAL_FLOOR),
        row.flag(C_GOV_FLOOR),
        row.flag(C_GOV_CORRUPT),
        Thresholds::new(100_000, 50_000),
    );

    let sealed = State::seal(tape_of(row.at(C_TAPE)), bounds, capacity, guards, params);
    let expect_error = row.at(C_EXPECT_ERROR) as i32;

    let state = match sealed {
        Ok(state) => state,
        Err(error) => {
            assert_eq!(error.code(), expect_error, "{}: seal error code", row.id);
            seen.errors.push(error.code());
            return;
        }
    };

    match step(&state) {
        Ok(regime) => {
            assert_eq!(expect_error, 0, "{}: step returned where an error was due", row.id);
            assert_eq!(
                regime.tag() as i64,
                row.at(C_EXPECT_VARIANT),
                "{}: regime tag",
                row.id
            );
            seen.regime_tags[regime.tag() as usize] += 1;

            if let Some(cause) = regime.cause() {
                assert_eq!(cause.code() as i64, row.at(C_EXPECT_CAUSE), "{}: cause", row.id);
                // The column names a cause that exists: the same value rebuilt from the
                // column is the value the operator returned, so a typo in the fixture
                // cannot pass as an unknown code.
                assert_eq!(cause, cause_of(row.at(C_EXPECT_CAUSE)), "{}: cause", row.id);
            }
            if let Some(breached) = regime.constraint() {
                assert_eq!(
                    breached.code() as i64,
                    row.at(C_EXPECT_CONSTRAINT),
                    "{}: constraint",
                    row.id
                );
                assert_eq!(
                    breached,
                    constraints_of(row.at(C_EXPECT_CONSTRAINT)),
                    "{}: constraint",
                    row.id
                );
            }
            if let Some(fraction) = regime.fraction() {
                assert_eq!(
                    fraction,
                    row.at(C_EXPECT_FRACTION),
                    "{}: probe fraction",
                    row.id
                );
                seen.fractions.push(fraction);
                if fraction == 714_285 {
                    seen.saw_truncation = false;
                }
            }
            if let Some(slot) = regime.missing() {
                seen.slots[slot.get() as usize] += 1;
                seen.saw_quarantine_every_slot[slot.get() as usize] = true;
            }
        }
        Err(error) => {
            assert_eq!(error.code(), expect_error, "{}: step error code", row.id);
            seen.errors.push(error.code());
        }
    }

    // The scenario flags are read from the row's own inputs, and only asserted when
    // the regime the fixture declares was actually produced.
    if row.at(C_INSTRUMENT) == 1 && row.at(C_TTS) < row.at(C_TTR) && row.at(C_EXPECT_VARIANT) == 3 {
        seen.saw_q1_before_q2 = true;
    }
    if row.flag(C_GOV_CORRUPT) && row.flag(C_VITAL_FLOOR) && row.at(C_EXPECT_CAUSE) == 1 {
        seen.saw_q2_before_q3 = true;
    }
    if row.flag(C_GOV_CORRUPT) && row.flag(C_VITAL_FLOOR) {
        seen.saw_channel_and_floor = true;
    }
    if row.at(C_U) < row.at(C_C) && row.at(C_EXPECT_VARIANT) == 5 {
        seen.saw_extent_below_capacity = true;
    }
    if row.at(C_L) == row.at(C_C) && row.at(C_EXPECT_ERROR) == -10 {
        seen.saw_extent_equal_capacity = true;
    }
    if row.at(C_U) == row.at(C_C) && row.at(C_EXPECT_ERROR) == -10 {
        seen.saw_upper_equal_capacity = true;
    }
    if row.at(C_TTS) == row.at(C_TTR) && row.at(C_EXPECT_VARIANT) != 4 {
        seen.saw_latency_equal = true;
    }
    if row.at(C_EXPECT_ERROR) == -9 {
        seen.saw_underflow = true;
    }
    if row.at(C_EXPECT_CAUSE) == 2 && row.at(C_EXPECT_VARIANT) == 4 {
        seen.saw_irreversible_probe = true;
    }
    if row.at(C_EXPECT_FRACTION) == 666_666 {
        seen.saw_truncation = true;
    }
}

/// The margin a boundary names is the reading of the declared site's own axis.
fn axis_margin(row: &Row, site: i64) -> i64 {
    match site {
        1..=3 => row.at(C_M_EPI),
        4..=5 => row.at(C_M_VITAL),
        6..=7 => row.at(C_M_GOV),
        _ => -1,
    }
}

fn run_accept(row: &Row, seen: &mut Observed) {
    let axes = AxialReadings::new(
        AxisReading::new(row.at(C_EPI).max(0) as usize, score_of(row.at(C_M_EPI))),
        AxisReading::new(row.at(C_VITAL).max(0) as usize, score_of(row.at(C_M_VITAL))),
        AxisReading::new(row.at(C_GOV).max(0) as usize, score_of(row.at(C_M_GOV))),
    );
    let declared = Declared::new(
        proposition_of(row.at(C_PROPOSITION)),
        site_of(row.at(C_SITE)),
        if row.at(C_STEP_INDEX) < 0 {
            None
        } else {
            Some(row.at(C_STEP_INDEX) as u32)
        },
    );
    let sentence = Sentence::new(
        // The tape column is `-1` in the accept rows: absence, not unattestation.
        if row.at(C_TAPE) == 0 {
            ControlTape::Unattested
        } else {
            ControlTape::Attested
        },
        ContingencyTape::new(7),
        Bounds::new(row.at(C_L), row.at(C_U), 1000, 0),
        Capacity::new(row.at(C_C), 0, row.at(C_R).max(0)),
        Budget::new(1000, 10),
        axes,
        declared,
        Assessment::new(100_000, 50_000),
    );
    let transcript = Transcript::seal(sentence).expect("an accept row seals");
    let expect_error = row.at(C_EXPECT_ERROR) as i32;

    match accept(&transcript) {
        Ok(verdict) => {
            assert_eq!(expect_error, 0, "{}: accepted where an error was due", row.id);
            assert_eq!(
                verdict.tag() as i64,
                row.at(C_EXPECT_VERDICT),
                "{}: verdict tag",
                row.id
            );
            seen.verdict_tags[verdict.tag() as usize] += 1;
            match verdict {
                Verdict::Boundary { site, margin } => {
                    assert_eq!(site.code() as i64, row.at(C_SITE), "{}: boundary site", row.id);
                    assert_eq!(
                        margin,
                        axis_margin(row, site.code() as i64),
                        "{}: boundary margin",
                        row.id
                    );
                }
                Verdict::Collapse {
                    proposition,
                    site,
                    step,
                } => {
                    assert_eq!(
                        proposition.code() as i64,
                        row.at(C_PROPOSITION),
                        "{}: collapse proposition",
                        row.id
                    );
                    assert_eq!(site.code() as i64, row.at(C_SITE), "{}: collapse site", row.id);
                    assert_eq!(step as i64, row.at(C_STEP_INDEX), "{}: collapse step", row.id);
                    seen.collapse_sites[site.code() as usize] += 1;
                }
                Verdict::Unparsable { reason } => {
                    assert_eq!(
                        reason.code() as i64,
                        row.at(C_EXPECT_REASON),
                        "{}: unparsable reason",
                        row.id
                    );
                    if reason == UnparsableReason::MarginMissing {
                        seen.saw_missing = true;
                    }
                }
                Verdict::Interior => {}
            }
        }
        Err(error) => {
            assert_eq!(error.code(), expect_error, "{}: accept error code", row.id);
            seen.errors.push(error.code());
        }
    }
}

fn run_diagnose(row: &Row, seen: &mut Observed) {
    let absorbed = row.at(C_ABSORBED) == 1;
    let steps = [StepReading::new(
        score_of(row.at(C_EPI)),
        score_of(row.at(C_VITAL)),
        score_of(row.at(C_GOV)),
        absorbed,
        row.at(C_R).max(0),
    )];
    let view = diagnose(&Trajectory::seal(&steps)).observe();

    let expected = row.at(C_EXPECT_BINDING);
    assert_eq!(
        view.binding_axis,
        axis_of(expected),
        "{}: binding axis",
        row.id
    );
    seen.binding[expected as usize] += 1;

    assert_eq!(view.s_epi, score_of(row.at(C_EPI)), "{}: epi fold", row.id);
    if absorbed {
        seen.saw_absorbed = true;
        assert_eq!(view.ruin_step, Some(0), "{}: ruin step", row.id);
        // After absorption the vital coordinate is not read, so it cannot bind, and
        // the fold reports only the steps that were not absorbed: none here.
        assert_eq!(view.s_vital, Score::Missing, "{}: vital after absorption", row.id);
    } else {
        assert_eq!(view.s_vital, score_of(row.at(C_VITAL)), "{}: vital fold", row.id);
    }
    assert_eq!(view.s_gov, score_of(row.at(C_GOV)), "{}: gov fold", row.id);
    assert_eq!(view.absorbed, absorbed, "{}: absorbed flag", row.id);
}

fn run_seal(row: &Row, seen: &mut Observed) {
    let instrument = if row.at(C_INSTRUMENT) == 1 {
        Instrument::Incomplete(
            MissingSlot::new(row.at(C_MISSING_SLOT) as u8).expect("slot in 1..7"),
        )
    } else {
        Instrument::Sealed(readings(500_000))
    };
    let result = State::seal(
        tape_of(row.at(C_TAPE)),
        Bounds::new(row.at(C_L), row.at(C_U), 1000, 0),
        Capacity::new(row.at(C_C), 0, row.at(C_R)),
        Guards::new(instrument),
        Params::new(
            row.at(C_TTS),
            row.at(C_TTR),
            row.flag(C_REVERSIBLE),
            row.flag(C_VITAL_FLOOR),
            row.flag(C_GOV_FLOOR),
            row.flag(C_GOV_CORRUPT),
            Thresholds::new(100_000, 50_000),
        ),
    );
    let expect_error = row.at(C_EXPECT_ERROR) as i32;
    match result {
        Ok(_) => assert_eq!(expect_error, 0, "{}: sealed where an error was due", row.id),
        Err(error) => {
            assert_eq!(error.code(), expect_error, "{}: seal error code", row.id);
            seen.errors.push(error.code());
        }
    }
}

#[test]
fn every_fixture_row_is_executed_and_matches_its_expectation() {
    let rows = rows();
    assert_eq!(rows.len(), VECTORS_DATA_ROWS, "{} data rows", VECTORS_DATA_ROWS);

    let mut seen = Observed::default();
    let mut functions = [0usize; 4];

    for row in &rows {
        let function = row.at(C_FN);
        assert!(
            (0..=3).contains(&function),
            "{}: unknown entry point {}",
            row.id,
            function
        );
        functions[function as usize] += 1;
        match function {
            0 => run_step(row, &mut seen),
            1 => run_accept(row, &mut seen),
            2 => run_diagnose(row, &mut seen),
            _ => run_seal(row, &mut seen),
        }
    }

    assert_eq!(functions, [22, 20, 6, 4], "rows per entry point");

    // `01` §6.2 — the regime histogram.
    assert_eq!(seen.regime_tags, [1, 2, 2, 7, 4, 1], "regime tags 0..5");
    // `01` §6.2 — the verdict histogram.
    assert_eq!(seen.verdict_tags, [1, 4, 9, 5], "verdict tags 0..3");
    // `01` §6.2 — every site collapses at least once; `a18` adds a second collapse at
    // `T1` (the fold on the site's own axis) and `a20` a third (a two-way tie), so the
    // histogram is no longer one per site.
    assert_eq!(seen.collapse_sites[1..8], [3, 1, 1, 1, 1, 1, 1]);
    // `01` §6.2 — all seven slots quarantined.
    assert_eq!(&seen.slots[1..8], &[1, 1, 1, 1, 1, 1, 1]);
    for slot in 1..=7 {
        assert!(seen.saw_quarantine_every_slot[slot], "slot {}", slot);
    }
    // `01` §6.3 — binding axis across none, epistemic, vital, governance.
    assert_eq!(&seen.binding, &[2, 2, 1, 1], "binding axis histogram");
    // `01` §6.3 — the precedence and degenerate cases are all exercised.
    assert!(seen.saw_q1_before_q2, "Q1 must precede Q2");
    assert!(seen.saw_q2_before_q3, "Q2 must precede Q3");
    assert!(seen.saw_channel_and_floor, "channel and floor together");
    assert!(seen.saw_extent_below_capacity, "U < C");
    assert!(seen.saw_extent_equal_capacity, "L == C is unlicensed");
    assert!(seen.saw_upper_equal_capacity, "U == C is unlicensed");
    assert!(seen.saw_latency_equal, "TTS == TTR passes Q2");
    assert!(seen.saw_truncation, "the probe truncates");
    assert!(seen.saw_underflow, "the probe underflows");
    assert!(seen.saw_irreversible_probe, "an irreversible probe halts");
    assert!(seen.saw_absorbed, "the absorbed trajectory is exercised");
    assert!(seen.saw_missing, "the absent margin is exercised");

    // `01` §6.3 — the error set the fixture exercises.
    let mut codes = seen.errors.clone();
    codes.sort_unstable();
    codes.dedup();
    assert_eq!(codes, vec![-11, -10, -9, -7, -6, -4, -3, -2]);
}

#[test]
fn the_harness_does_not_invent_a_row() {
    // `01` §6.4: the fixture is `VECTORS_DATA_ROWS` records under a comment header,
    // and every id is distinct with a single letter prefix.
    let rows = rows();
    assert_eq!(rows.len(), VECTORS_DATA_ROWS, "one record per data line");
    let mut ids: Vec<&str> = rows.iter().map(|row| row.id.as_str()).collect();
    let total = ids.len();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), total, "ids are unique");
    for id in &ids {
        assert!(
            id.starts_with('s') || id.starts_with('f') || id.starts_with('a') || id.starts_with('d'),
            "{}",
            id
        );
    }
}

#[test]
fn the_probe_fraction_table_is_the_one_the_fixture_states() {
    // `01` §5: u = 400, c = 200, l = 120 truncates to 714_285.
    assert_eq!(
        probe_fraction(Bounds::new(120, 400, 1000, 0), 200),
        Ok(714_285)
    );
    // The fixture's truncation row: u = 3_000_000, c = 1_000_000, l = 0 gives 666_666.
    assert_eq!(
        probe_fraction(Bounds::new(0, 3_000_000, 1000, 0), 1_000_000),
        Ok(666_666)
    );
    // The fixture's underflow row: one unit of room in a billion truncates to zero.
    assert_eq!(
        probe_fraction(Bounds::new(0, 1_000_000_000, 1000, 0), 999_999_999),
        Err(Error::ProbeUnderflow)
    );
}

#[test]
fn the_fixture_drives_the_seven_sites_and_three_propositions() {
    // The collapse rows must pair sites with the proposition of the same axis: the
    // fixture is the evidence that the -11 pre-gate is not masking a real collapse.
    for row in rows() {
        if row.at(C_FN) == 1 && row.at(C_EXPECT_VERDICT) == 2 {
            let site = Site::from_code(row.at(C_SITE) as u8).unwrap();
            let proposition = Proposition::from_code(row.at(C_PROPOSITION)).unwrap();
            assert_eq!(
                site.axis(),
                proposition.axis(),
                "{}: a collapse must be axis-consistent",
                row.id
            );
        }
    }
}
