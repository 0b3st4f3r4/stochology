//! The harness proper: one fixture row in, one canonical line out.
//!
//! The harness does not know what a regime *means*. It decodes a row into the
//! public types of `stoch-core` exactly as `01` §6.4 declares the keys, calls
//! the entry point the row names, and renders what came back into a single
//! canonical line. The expectation keys of the same row are rendered by the
//! same spelling rules, so the comparison is string equality on two lines
//! produced independently: one from the fixture's declaration, one from the
//! operator's answer.
//!
//! A corrupt cell therefore cannot pass by being unread: if the fixture declares
//! something and the operator returns something else, the two lines differ and
//! the harness prints both.

use stoch_core::{
    accept, diagnose, step, Assessment, AxialReadings, AxisReading, Bounds, Budget, Capacity,
    ContingencyTape, ControlTape, Declared, Guards, Instrument,
    MissingSlot, Params, Proposition, Score, Sentence, SevenReadings, Site, State, StepReading,
    Thresholds, Trajectory, Transcript, Verdict,
};

use crate::schema::{self, Row, C_ABSORBED, C_C, C_EPI, C_EXPECT_BINDING, C_EXPECT_CAUSE,
    C_EXPECT_CONSTRAINT, C_EXPECT_ERROR, C_EXPECT_FRACTION, C_EXPECT_REASON, C_EXPECT_VARIANT,
    C_EXPECT_VERDICT, C_GOV, C_GOV_CORRUPT, C_GOV_FLOOR, C_INSTRUMENT, C_L, C_M_EPI, C_M_GOV,
    C_M_VITAL,
    C_MISSING_SLOT, C_PROPOSITION, C_R, C_REVERSIBLE, C_SITE, C_STEP_INDEX, C_TAPE, C_TTR, C_TTS,
    C_U, C_VITAL, C_VITAL_FLOOR, FN_ACCEPT, FN_DIAGNOSE, FN_SEAL, FN_STEP};

/// The two constants the fixture fixes: the derivation budget of a parse and the
/// caller-declared acceptance band. `01` §10 leaves both to the caller; the
/// fixture declares them once so that every row is judged against the same band.
const BUDGET_LIMIT: i64 = 1000;
const BUDGET_ELAPSED: i64 = 10;
const ACCEPT_THRESHOLD: i64 = 100_000;
const BAND_WIDTH: i64 = 50_000;
const N_EFF: i64 = 1000;
const DEFECT: i64 = 0;
const CONTINGENCY: u64 = 7;

/// One row's declaration and one row's answer, side by side.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Outcome {
    pub id: String,
    pub expected: String,
    pub actual: String,
}

impl Outcome {
    pub fn agrees(&self) -> bool {
        self.expected == self.actual
    }
}

/// A total run: every row, in file order.
#[derive(Clone, Debug, Default)]
pub struct Report {
    pub outcomes: Vec<Outcome>,
    pub functions: [usize; 4],
}

impl Report {
    pub fn mismatches(&self) -> Vec<&Outcome> {
        self.outcomes.iter().filter(|o| !o.agrees()).collect()
    }

    pub fn agrees(&self) -> bool {
        self.outcomes.iter().all(Outcome::agrees)
    }

    pub fn rows(&self) -> usize {
        self.outcomes.len()
    }
}

/// Run the whole fixture text.
pub fn run(text: &str) -> Result<Report, schema::ParseError> {
    let rows = schema::parse(text)?;
    Ok(run_rows(&rows))
}

pub fn run_rows(rows: &[Row]) -> Report {
    let mut report = Report::default();
    for row in rows {
        if let Some(slot) = report.functions.get_mut(row.function() as usize) {
            *slot += 1;
        }
        report.outcomes.push(Outcome {
            id: row.id.clone(),
            expected: expected_line(row),
            actual: observed_line(row),
        });
    }
    report
}

/// The fixture's own declaration, rendered by the harness's spelling rules.
pub fn expected_line(row: &Row) -> String {
    let error = row.at(C_EXPECT_ERROR);
    if error != 0 {
        return match row.function() {
            FN_STEP => format!("step err={error}"),
            FN_ACCEPT => format!("accept err={error}"),
            FN_SEAL => format!("seal err={error}"),
            FN_DIAGNOSE => format!("diagnose err={error}"),
            other => format!("fn={other} err={error}"),
        };
    }
    match row.function() {
        FN_STEP => format!(
            "step regime={} cause={} constraint={} fraction={} missing={}",
            row.at(C_EXPECT_VARIANT),
            dash(row.at(C_EXPECT_CAUSE)),
            dash(row.at(C_EXPECT_CONSTRAINT)),
            dash(row.at(C_EXPECT_FRACTION)),
            if row.at(C_EXPECT_VARIANT) == 3 {
                row.at(C_MISSING_SLOT).to_string()
            } else {
                "-".to_string()
            },
        ),
        FN_ACCEPT => {
            let verdict = row.at(C_EXPECT_VERDICT);
            let mut line = format!("accept verdict={verdict}");
            match verdict {
                0 => {}
                1 => line.push_str(&format!(
                    " site={} margin={}",
                    row.at(C_SITE),
                    axis_margin(row, row.at(C_SITE))
                )),
                2 => line.push_str(&format!(
                    " proposition={} site={} step={}",
                    row.at(C_PROPOSITION),
                    row.at(C_SITE),
                    row.at(C_STEP_INDEX)
                )),
                3 => line.push_str(&format!(" reason={}", row.at(C_EXPECT_REASON))),
                other => line.push_str(&format!(" <no verdict {other}>")),
            }
            line
        }
        FN_DIAGNOSE => {
            let absorbed = row.at(C_ABSORBED) == 1;
            format!(
                "diagnose binding={} absorbed={} ruin={} reserve={} epi={} vital={} gov={}",
                if row.at(C_EXPECT_BINDING) <= 0 {
                    "-".to_string()
                } else {
                    row.at(C_EXPECT_BINDING).to_string()
                },
                absorbed as i64,
                if absorbed { "0" } else { "-" },
                row.at(C_R).max(0),
                score_cell(row.at(C_EPI)),
                if absorbed { "-".to_string() } else { score_cell(row.at(C_VITAL)) },
                score_cell(row.at(C_GOV)),
            )
        }
        FN_SEAL => "seal sealed".to_string(),
        other => format!("fn={other}"),
    }
}

/// The operator's answer, rendered by the same rules.
pub fn observed_line(row: &Row) -> String {
    match row.function() {
        FN_STEP => match run_step(row) {
            Err(error) => format!("step err={error}"),
            Ok(regime) => format!(
                "step regime={} cause={} constraint={} fraction={} missing={}",
                regime.tag(),
                opt(regime.cause().map(|c| c.code() as i64)),
                opt(regime.constraint().map(|c| c.code() as i64)),
                opt(regime.fraction()),
                opt(regime.missing().map(|m| m.get() as i64)),
            ),
        },
        FN_ACCEPT => match run_accept(row) {
            Err(error) => format!("accept err={error}"),
            Ok(verdict) => {
                let mut line = format!("accept verdict={}", verdict.tag());
                match verdict {
                    Verdict::Interior => {}
                    Verdict::Boundary { site, margin } => {
                        line.push_str(&format!(" site={} margin={}", site.code(), margin))
                    }
                    Verdict::Collapse {
                        proposition,
                        site,
                        step,
                    } => line.push_str(&format!(
                        " proposition={} site={} step={}",
                        proposition.code(),
                        site.code(),
                        step
                    )),
                    Verdict::Unparsable { reason } => {
                        line.push_str(&format!(" reason={}", reason.code()))
                    }
                }
                line
            }
        },
        FN_DIAGNOSE => {
            let view = run_diagnose(row);
            format!(
                "diagnose binding={} absorbed={} ruin={} reserve={} epi={} vital={} gov={}",
                opt(view.binding_axis.map(|a| match a {
                    stoch_core::Axis::Epistemic => 1,
                    stoch_core::Axis::Vital => 2,
                    stoch_core::Axis::Governance => 3,
                })),
                view.absorbed as i64,
                opt(view.ruin_step.map(|s| s as i64)),
                match view.terminal_reserve {
                    Score::Present(v) => v.to_string(),
                    Score::Missing => "-".to_string(),
                },
                score_cell(score_code(view.s_epi)),
                score_cell(score_code(view.s_vital)),
                score_cell(score_code(view.s_gov)),
            )
        }
        FN_SEAL => match run_seal(row) {
            Ok(()) => "seal sealed".to_string(),
            Err(error) => format!("seal err={error}"),
        },
        other => format!("fn={other}"),
    }
}

fn run_step(row: &Row) -> Result<stoch_core::Regime, i32> {
    let state = seal_state(row, false).map_err(|e| e.code())?;
    step(&state).map_err(|e| e.code())
}

fn run_accept(row: &Row) -> Result<Verdict, i32> {
    let axes = AxialReadings::new(
        AxisReading::new(row.at(C_EPI).max(0) as usize, score(row.at(C_M_EPI))),
        AxisReading::new(row.at(C_VITAL).max(0) as usize, score(row.at(C_M_VITAL))),
        AxisReading::new(row.at(C_GOV).max(0) as usize, score(row.at(C_M_GOV))),
    );
    let declared = Declared::new(
        proposition(row.at(C_PROPOSITION)),
        site(row.at(C_SITE)),
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
        ContingencyTape::new(CONTINGENCY),
        Bounds::new(row.at(C_L), row.at(C_U), N_EFF, DEFECT),
        Capacity::new(row.at(C_C), 0, row.at(C_R).max(0)),
        Budget::new(BUDGET_LIMIT, BUDGET_ELAPSED),
        axes,
        declared,
        Assessment::new(ACCEPT_THRESHOLD, BAND_WIDTH),
    );
    let transcript = Transcript::seal(sentence).map_err(|e| e.code())?;
    accept(&transcript).map_err(|e| e.code())
}

fn run_diagnose(row: &Row) -> stoch_core::DiagnosisView {
    let absorbed = row.at(C_ABSORBED) == 1;
    let steps = [StepReading::new(
        score(row.at(C_EPI)),
        score(row.at(C_VITAL)),
        score(row.at(C_GOV)),
        absorbed,
        row.at(C_R).max(0),
    )];
    diagnose(&Trajectory::seal(&steps)).observe()
}

fn run_seal(row: &Row) -> Result<(), i32> {
    seal_state(row, true).map(|_| ()).map_err(|e| e.code())
}

fn seal_state(row: &Row, flat_margin: bool) -> Result<State, stoch_core::Error> {
    let instrument = if row.at(C_INSTRUMENT) == 1 {
        Instrument::Incomplete(
            MissingSlot::new(row.at(C_MISSING_SLOT) as u8).expect("a slot in 1..=7"),
        )
    } else {
        Instrument::Sealed(SevenReadings::new([
            if flat_margin { 500_000 } else { row.at(C_M_EPI).max(0) };
            7
        ]))
    };
    State::seal(
        tape(row.at(C_TAPE)),
        Bounds::new(row.at(C_L), row.at(C_U), N_EFF, DEFECT),
        Capacity::new(row.at(C_C), 0, row.at(C_R)),
        Guards::new(instrument),
        Params::new(
            row.at(C_TTS),
            row.at(C_TTR),
            row.flag(C_REVERSIBLE),
            row.flag(C_VITAL_FLOOR),
            row.flag(C_GOV_FLOOR),
            row.flag(C_GOV_CORRUPT),
            Thresholds::new(ACCEPT_THRESHOLD, BAND_WIDTH),
        ),
    )
}

/// The margin the fixture declares for a site: the reading of that site's own
/// axis. A boundary names one site and its own margin, so the expectation must be
/// read from the axis the site sits on, never from a single flat cell.
fn axis_margin(row: &Row, site: i64) -> i64 {
    match site {
        1..=3 => row.at(C_M_EPI),
        4..=5 => row.at(C_M_VITAL),
        6..=7 => row.at(C_M_GOV),
        _ => -1,
    }
}

// ---- decoders: the fixture's integers, in the types the core requires ----

fn tape(value: i64) -> ControlTape {
    if value == 1 {
        ControlTape::Attested
    } else {
        ControlTape::Unattested
    }
}

fn site(value: i64) -> Option<Site> {
    if value <= 0 {
        None
    } else {
        Site::from_code(value as u8)
    }
}

fn proposition(value: i64) -> Option<Proposition> {
    if value <= 0 {
        None
    } else {
        Proposition::from_code(value)
    }
}

fn score(value: i64) -> Score {
    if value < 0 {
        Score::Missing
    } else {
        Score::Present(value)
    }
}

fn score_code(score: Score) -> i64 {
    match score {
        Score::Present(value) => value,
        Score::Missing => -1,
    }
}

fn score_cell(value: i64) -> String {
    if value < 0 {
        "-".to_string()
    } else {
        value.to_string()
    }
}

fn opt(value: Option<i64>) -> String {
    match value {
        Some(value) => value.to_string(),
        None => "-".to_string(),
    }
}

fn dash(value: i64) -> String {
    if value < 0 {
        "-".to_string()
    } else {
        value.to_string()
    }
}
