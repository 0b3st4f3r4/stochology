//! The coverage corpus: the vectors the fixture must not lose.
//!
//! `01-funcionalidades.md` §6.3 states the minimum coverage of the normative
//! fixture: every regime, every verdict, every quarantine slot, one collapse per
//! guard site, the four binding-axis outcomes, the two unlicensed boundaries
//! (`L == C`, `U == C`), the equality of the two latencies, absorption, the
//! truncating probe and its underflow. This module *generates* a corpus that
//! declares those cases, and then *verifies* the corpus by running it and
//! reading the census off the answers — not off the declarations.
//!
//! That split is the point. A generator whose coverage claim is read from its own
//! labels proves nothing; here the census is computed from the canonical lines
//! the core returned, so a case that drifts into another branch shows up as a
//! missing item and the test fails.

use crate::fixture;
use crate::schema::{self, Row, C_ABSORBED, C_C, C_EPI, C_EXPECT_BINDING, C_EXPECT_CAUSE,
    C_EXPECT_CONSTRAINT, C_EXPECT_ERROR, C_EXPECT_FRACTION, C_EXPECT_REASON, C_EXPECT_VARIANT,
    C_EXPECT_VERDICT, C_FN, C_GOV, C_GOV_CORRUPT, C_GOV_FLOOR, C_INSTRUMENT, C_L, C_M_EPI,
    C_M_GOV, C_M_VITAL,
    C_MISSING_SLOT, C_PROPOSITION, C_R, C_REVERSIBLE, C_SITE, C_STEP_INDEX, C_TAPE, C_TTR, C_TTS,
    C_U, C_VITAL, C_VITAL_FLOOR, FN_ACCEPT, FN_DIAGNOSE, FN_SEAL, FN_STEP};

/// A generated case: the row, and the §6.3 item it exists to discharge.
#[derive(Clone, Debug)]
pub struct Case {
    pub id: &'static str,
    pub covers: &'static str,
    pub row: Row,
}

/// The generated corpus, in the order it is declared.
pub fn cases() -> Vec<Case> {
    let mut cases = Vec::new();

    // ---- step: the six regimes ----
    cases.push(Case {
        id: "g01",
        covers: "BufferedAction",
        row: step_row("g01", 200, 400, 120)
            .set(C_EXPECT_VARIANT, 0)
            .clone(),
    });
    // Truncating probe: the fraction of the interval above capacity is 2/3,
    // and 2 * SCALE / 3 does not divide: 666_666, floored.
    cases.push(Case {
        id: "g02",
        covers: "ReversibleFractionalProbe, truncation",
        row: probe_row("g02", 0, 3, 1, 666_666),
    });
    // The same fraction at a quarter of the interval: 250_000 exactly.
    cases.push(Case {
        id: "g03",
        covers: "ReversibleFractionalProbe, exact division",
        row: probe_row("g03", 100, 500, 400, 250_000),
    });
    // The lower bound above capacity leaves the whole interval usable: buffered.
    cases.push(Case {
        id: "g04",
        covers: "BufferedAction below the interval",
        row: step_row("g04", 700, 900, 600)
            .set(C_EXPECT_VARIANT, 0)
            .clone(),
    });
    cases.push(Case {
        id: "g05",
        covers: "InwardAction/VitalFloor",
        row: step_row("g05", 200, 400, 120)
            .set(C_VITAL_FLOOR, 1)
            .set(C_EXPECT_VARIANT, 2)
            .set(C_EXPECT_CONSTRAINT, 0)
            .clone(),
    });
    cases.push(Case {
        id: "g06",
        covers: "InwardAction/GovernanceFloor",
        row: step_row("g06", 200, 400, 120)
            .set(C_GOV_FLOOR, 1)
            .set(C_EXPECT_VARIANT, 2)
            .set(C_EXPECT_CONSTRAINT, 1)
            .clone(),
    });

    // ---- step: quarantine, one case per slot ----
    for slot in 1..=7u8 {
        let id = match slot {
            1 => "g07",
            2 => "g08",
            3 => "g09",
            4 => "g10",
            5 => "g11",
            6 => "g12",
            _ => "g13",
        };
        cases.push(Case {
            id,
            covers: "Quarantine, one per guard site",
            row: step_row(id, 200, 400, 120)
                .set(C_INSTRUMENT, 1)
                .set(C_MISSING_SLOT, slot as i64)
                .set(C_EXPECT_VARIANT, 3)
                .clone(),
        });
    }

    // ---- step: the three halts and the one no-action ----
    cases.push(Case {
        id: "g14",
        covers: "AlgedonicHalt/SurvivalLatency",
        row: step_row("g14", 300, 500, 120)
            .set(C_TTS, 5)
            .set(C_TTR, 10)
            .set(C_EXPECT_VARIANT, 4)
            .set(C_EXPECT_CAUSE, 0)
            .clone(),
    });
    cases.push(Case {
        id: "g15",
        covers: "AlgedonicHalt/GovernanceChannel",
        row: step_row("g15", 200, 400, 120)
            .set(C_GOV_CORRUPT, 1)
            .set(C_EXPECT_VARIANT, 4)
            .set(C_EXPECT_CAUSE, 1)
            .clone(),
    });
    cases.push(Case {
        id: "g16",
        covers: "AlgedonicHalt/IrreversibleProbeRefused",
        row: step_row("g16", 200, 400, 300)
            .set(C_REVERSIBLE, 0)
            .set(C_EXPECT_VARIANT, 4)
            .set(C_EXPECT_CAUSE, 2)
            .clone(),
    });
    cases.push(Case {
        id: "g17",
        covers: "NoAction (U < C)",
        row: step_row("g17", 50, 100, 200)
            .set(C_EXPECT_VARIANT, 5)
            .clone(),
    });

    // ---- step: the ordering questions and the two unlicensed boundaries ----
    // Q1 before Q2: an incomplete instrument outranks a latent latency fault.
    cases.push(Case {
        id: "g18",
        covers: "question order: instrument before latency",
        row: step_row("g18", 300, 500, 120)
            .set(C_INSTRUMENT, 1)
            .set(C_MISSING_SLOT, 2)
            .set(C_TTS, 5)
            .set(C_TTR, 10)
            .set(C_EXPECT_VARIANT, 3)
            .clone(),
    });
    // Q2 before Q3: the governance channel corrupts before the vital floor is read.
    cases.push(Case {
        id: "g19",
        covers: "question order: corruption before floors",
        row: step_row("g19", 200, 400, 120)
            .set(C_GOV_CORRUPT, 1)
            .set(C_VITAL_FLOOR, 1)
            .set(C_EXPECT_VARIANT, 4)
            .set(C_EXPECT_CAUSE, 1)
            .clone(),
    });
    // TTS == TTR is not a latency fault: the question is `tts < ttr`.
    cases.push(Case {
        id: "g20",
        covers: "TTS == TTR is not a halt",
        row: step_row("g20", 200, 400, 120)
            .set(C_TTS, 10)
            .set(C_TTR, 10)
            .set(C_EXPECT_VARIANT, 0)
            .clone(),
    });
    cases.push(Case {
        id: "g21",
        covers: "L == C is unlicensed",
        row: step_row("g21", 200, 400, 200)
            .set(C_EXPECT_ERROR, -10)
            .clone(),
    });
    cases.push(Case {
        id: "g22",
        covers: "U == C is unlicensed",
        row: step_row("g22", 200, 400, 400)
            .set(C_EXPECT_ERROR, -10)
            .clone(),
    });
    // q floors to zero: the probe would report a fraction it cannot represent.
    cases.push(Case {
        id: "g23",
        covers: "probe underflow",
        row: probe_row_err("g23", 0, 1_000_000_000, 999_999_999, -9),
    });

    // ---- seal: the skeleton refusals reachable from the fixture's domain ----
    cases.push(Case {
        id: "g24",
        covers: "seal accepts",
        row: seal_row("g24", 200, 400, 120),
    });
    cases.push(Case {
        id: "g25",
        covers: "seal/Uattested tape",
        row: seal_row("g25", 200, 400, 120)
            .set(C_TAPE, 0)
            .set(C_EXPECT_ERROR, -7)
            .clone(),
    });
    cases.push(Case {
        id: "g26",
        covers: "seal/missing extent",
        row: seal_row("g26", 200, 400, 120)
            .set(C_L, -1)
            .set(C_EXPECT_ERROR, -6)
            .clone(),
    });
    cases.push(Case {
        id: "g27",
        covers: "seal/negative reserve",
        // `-2` and not `-1`: in the fixture's spelling a key that is not written is
        // absent, and a reserve of `-1` would be indistinguishable from a record that
        // declared no reserve at all. Any negative value is the same test.
        row: seal_row("g27", 200, 400, 120)
            .set(C_R, -2)
            .set(C_EXPECT_ERROR, -4)
            .clone(),
    });
    cases.push(Case {
        id: "g28",
        covers: "seal/inverted bounds",
        row: seal_row("g28", 400, 200, 120)
            .set(C_EXPECT_ERROR, -2)
            .clone(),
    });
    cases.push(Case {
        id: "g29",
        covers: "seal/non-positive capacity",
        row: seal_row("g29", 200, 400, 0)
            .set(C_EXPECT_ERROR, -3)
            .clone(),
    });

    // ---- accept: the four verdicts ----
    cases.push(Case {
        id: "g30",
        covers: "Verdict/Interior",
        row: accept_row("g30", 1, 1, 1, 500_000, 0, -1, -1)
            .set(C_EXPECT_VERDICT, 0)
            .clone(),
    });
    cases.push(Case {
        id: "g31",
        covers: "Verdict/Boundary",
        row: accept_row("g31", 1, 1, 1, 101_000, 4, -1, -1)
            .set(C_EXPECT_VERDICT, 1)
            .clone(),
    });
    for site in 1..=7i64 {
        let id = match site {
            1 => "g32",
            2 => "g33",
            3 => "g34",
            4 => "g35",
            5 => "g36",
            6 => "g37",
            _ => "g38",
        };
        // A collapse must name a proposition on the site's own axis, or the
        // declaration contradicts itself before the fold is read.
        let axis = match site {
            1..=3 => 1,
            4..=5 => 2,
            _ => 3,
        };
        let mut row = accept_row(id, 1, 1, 1, 50_000, site, axis, 3);
        row.set(C_EXPECT_VERDICT, 2);
        cases.push(Case {
            id,
            covers: "Verdict/Collapse, one per guard site",
            row,
        });
    }
    // A fold that misses a reading is unparsable before it is a collapse: the
    // missing score never becomes a zero.
    cases.push(Case {
        id: "g39",
        covers: "Verdict/Unparsable, missing margin",
        row: accept_row("g39", 1, 1, 1, -1, 1, 1, 3)
            .set(C_EXPECT_VERDICT, 3)
            .set(C_EXPECT_REASON, 3)
            .clone(),
    });
    // No readings on an axis: the coverage stage of B.1 refuses the transcript.
    cases.push(Case {
        id: "g40",
        covers: "Verdict/Unparsable, axial coverage",
        row: accept_row("g40", 0, 1, 1, 500_000, 0, -1, -1)
            .set(C_EXPECT_VERDICT, 3)
            .set(C_EXPECT_REASON, 2)
            .clone(),
    });
    // An unattested control tape is a skeleton fault.
    cases.push(Case {
        id: "g41",
        covers: "Verdict/Unparsable, skeleton",
        row: accept_row("g41", 1, 1, 1, 500_000, 0, -1, -1)
            .set(C_TAPE, 0)
            .set(C_EXPECT_VERDICT, 3)
            .set(C_EXPECT_REASON, 1)
            .clone(),
    });
    // A collapse that does not name its proposition cannot be reported as one.
    cases.push(Case {
        id: "g42",
        covers: "Verdict/Unparsable, unnamed collapse",
        row: accept_row("g42", 1, 1, 1, 50_000, 3, -1, 5)
            .set(C_EXPECT_VERDICT, 3)
            .set(C_EXPECT_REASON, 1)
            .clone(),
    });
    // A site outside its proposition's axis: the declaration contradicts itself.
    cases.push(Case {
        id: "g43",
        covers: "site/proposition mismatch",
        row: accept_row("g43", 1, 1, 1, 50_000, 4, 1, 3)
            .set(C_EXPECT_ERROR, -11)
            .clone(),
    });

    // ---- diagnose: the four binding outcomes ----
    cases.push(Case {
        id: "g44",
        covers: "binding/Epistemic",
        row: diagnose_row("g44", 500_000, 600_000, 700_000, false, 0)
            .set(C_EXPECT_BINDING, 1)
            .clone(),
    });
    cases.push(Case {
        id: "g45",
        covers: "binding/Vital",
        row: diagnose_row("g45", 700_000, 400_000, 600_000, false, 0)
            .set(C_EXPECT_BINDING, 2)
            .clone(),
    });
    cases.push(Case {
        id: "g46",
        covers: "binding/Governance",
        row: diagnose_row("g46", 600_000, 700_000, 300_000, false, 0)
            .set(C_EXPECT_BINDING, 3)
            .clone(),
    });
    cases.push(Case {
        id: "g47",
        covers: "binding/none, missing score",
        row: diagnose_row("g47", 500_000, -1, 600_000, false, 0)
            .set(C_EXPECT_BINDING, 0)
            .clone(),
    });
    // Absorption: the vital coordinate is not read, so it cannot bind.
    cases.push(Case {
        id: "g48",
        covers: "absorption",
        row: diagnose_row("g48", 400_000, -1, 600_000, true, 0)
            .set(C_EXPECT_BINDING, 1)
            .clone(),
    });

    cases
}

fn step_row(id: &str, l: i64, u: i64, c: i64) -> Row {
    let mut row = Row::new(id);
    row.set(C_FN, FN_STEP)
        .set(C_TAPE, 1)
        .set(C_INSTRUMENT, 0)
        .set(C_MISSING_SLOT, 0)
        .set(C_TTS, 10)
        .set(C_TTR, 10)
        .set(C_L, l)
        .set(C_U, u)
        .set(C_C, c)
        .set(C_R, 0)
        .set(C_REVERSIBLE, 1)
        .set(C_GOV_CORRUPT, 0)
        .set(C_VITAL_FLOOR, 0)
        .set(C_GOV_FLOOR, 0)
        .set(C_M_EPI, 500_000)
        .set(C_M_VITAL, 500_000)
        .set(C_M_GOV, 500_000)
        .set(C_EXPECT_ERROR, 0);
    row
}

/// The seal entry point shares the step's row shape: same bounds, capacity,
/// guards and parameters, a flat margin across the seven readings.
fn seal_row(id: &str, l: i64, u: i64, c: i64) -> Row {
    let mut row = step_row(id, l, u, c);
    row.set(C_FN, FN_SEAL);
    row
}

fn probe_row(id: &str, l: i64, u: i64, c: i64, fraction: i64) -> Row {
    let mut row = step_row(id, l, u, c);
    row.set(C_EXPECT_VARIANT, 1).set(C_EXPECT_FRACTION, fraction);
    row
}

fn probe_row_err(id: &str, l: i64, u: i64, c: i64, error: i64) -> Row {
    let mut row = step_row(id, l, u, c);
    row.set(C_EXPECT_ERROR, error);
    row
}

fn accept_row(
    id: &str,
    epi: i64,
    vital: i64,
    gov: i64,
    margin: i64,
    site: i64,
    proposition: i64,
    step_index: i64,
) -> Row {
    let mut row = Row::new(id);
    row.set(C_FN, FN_ACCEPT)
        .set(C_TAPE, 1)
        .set(C_L, 200)
        .set(C_U, 400)
        .set(C_C, 120)
        .set(C_R, 0)
        .set(C_EPI, epi)
        .set(C_VITAL, vital)
        .set(C_GOV, gov)
        .set(C_M_EPI, margin)
        .set(C_M_VITAL, margin)
        .set(C_M_GOV, margin)
        .set(C_SITE, site)
        .set(C_PROPOSITION, proposition)
        .set(C_STEP_INDEX, step_index)
        .set(C_EXPECT_ERROR, 0);
    row
}

fn diagnose_row(id: &str, epi: i64, vital: i64, gov: i64, absorbed: bool, r: i64) -> Row {
    let mut row = Row::new(id);
    row.set(C_FN, FN_DIAGNOSE)
        .set(C_EPI, epi)
        .set(C_VITAL, vital)
        .set(C_GOV, gov)
        .set(C_ABSORBED, absorbed as i64)
        .set(C_R, r)
        .set(C_EXPECT_ERROR, 0);
    row
}

/// The corpus as fixture text, so it can be re-parsed and run through the same
/// harness as the normative fixture.
pub fn corpus() -> String {
    let rows: Vec<Row> = cases().into_iter().map(|case| case.row).collect();
    schema::render(&rows)
}

/// What the answers actually contain. Every field is counted from a canonical
/// line produced by `stoch-core`, never from a case label.
#[derive(Clone, Debug, Default)]
pub struct Census {
    pub rows: usize,
    pub regimes: [usize; 6],
    pub verdicts: [usize; 4],
    pub collapse_sites: [usize; 8],
    pub quarantine_slots: [usize; 8],
    pub bindings: [usize; 4],
    pub errors: Vec<i64>,
    pub saw_l_equals_c: bool,
    pub saw_u_equals_c: bool,
    pub saw_tts_equals_ttr: bool,
    pub saw_absorption: bool,
    pub saw_truncation: bool,
    pub saw_underflow: bool,
    pub saw_irreversible_probe: bool,
    pub saw_missing_score: bool,
    pub saw_q1_before_q2: bool,
    pub saw_q2_before_q3: bool,
    pub saw_extent_below_capacity: bool,
    pub saw_unnamed_collapse: bool,
    pub saw_mismatch: bool,
}

impl Census {
    /// Which §6.3 items the corpus does not in fact reach.
    pub fn missing(&self) -> Vec<String> {
        let mut missing = Vec::new();
        for (tag, count) in self.regimes.iter().enumerate() {
            if *count == 0 {
                missing.push(format!("regime {tag}"));
            }
        }
        for (tag, count) in self.verdicts.iter().enumerate() {
            if *count == 0 {
                missing.push(format!("verdict {tag}"));
            }
        }
        for site in 1..=7 {
            if self.collapse_sites[site] == 0 {
                missing.push(format!("collapse at site {site}"));
            }
            if self.quarantine_slots[site] == 0 {
                missing.push(format!("quarantine slot {site}"));
            }
        }
        for axis in 1..=3 {
            if self.bindings[axis] == 0 {
                missing.push(format!("binding axis {axis}"));
            }
        }
        if self.bindings[0] == 0 {
            missing.push("binding absent".to_string());
        }
        for code in [-2i64, -3, -4, -6, -7, -9, -10, -11] {
            if !self.errors.contains(&code) {
                missing.push(format!("error {code}"));
            }
        }
        let flags = [
            ("L == C", self.saw_l_equals_c),
            ("U == C", self.saw_u_equals_c),
            ("TTS == TTR", self.saw_tts_equals_ttr),
            ("absorption", self.saw_absorption),
            ("truncation", self.saw_truncation),
            ("probe underflow", self.saw_underflow),
            ("irreversible probe", self.saw_irreversible_probe),
            ("missing score", self.saw_missing_score),
            ("Q1 before Q2", self.saw_q1_before_q2),
            ("Q2 before Q3", self.saw_q2_before_q3),
            ("U < C", self.saw_extent_below_capacity),
            ("unnamed collapse", self.saw_unnamed_collapse),
            ("site/proposition mismatch", self.saw_mismatch),
        ];
        for (name, present) in flags {
            if !present {
                missing.push(name.to_string());
            }
        }
        missing
    }
}

/// Count the coverage of a row set by running it.
pub fn census(rows: &[Row]) -> Census {
    let report = fixture::run_rows(rows);
    let mut census = Census {
        rows: rows.len(),
        ..Census::default()
    };
    for (row, outcome) in rows.iter().zip(report.outcomes.iter()) {
        let line = outcome.actual.as_str();
        if let Some(error) = number(line, "err") {
            census.errors.push(error);
            if error == -9 {
                census.saw_underflow = true;
                census.saw_truncation = true;
            }
            if error == -11 {
                census.saw_mismatch = true;
            }
        }
        match row.function() {
            FN_STEP => {
                if let Some(regime) = number(line, "regime") {
                    if (0..6).contains(&regime) {
                        census.regimes[regime as usize] += 1;
                    }
                    match regime {
                        0 => {}
                        1 => {
                            if let Some(fraction) = number(line, "fraction") {
                                if fraction == 666_666 || fraction == 714_285 {
                                    census.saw_truncation = true;
                                }
                            }
                        }
                        3 => {
                            if let Some(slot) = number(line, "missing") {
                                if (1..=7).contains(&slot) {
                                    census.quarantine_slots[slot as usize] += 1;
                                }
                            }
                        }
                        4 => match number(line, "cause") {
                            Some(0) => {}
                            Some(1) => {}
                            Some(2) => census.saw_irreversible_probe = true,
                            _ => {}
                        },
                        5 => census.saw_extent_below_capacity = true,
                        _ => {}
                    }
                }
                if row.at(C_L) == row.at(C_C) {
                    census.saw_l_equals_c = true;
                }
                if row.at(C_U) == row.at(C_C) {
                    census.saw_u_equals_c = true;
                }
                if row.at(C_TTS) == row.at(C_TTR) && row.at(C_INSTRUMENT) == 0 {
                    census.saw_tts_equals_ttr = true;
                }
                if row.at(C_INSTRUMENT) == 1
                    && row.at(C_TTS) < row.at(C_TTR)
                    && number(line, "regime") == Some(3)
                {
                    census.saw_q1_before_q2 = true;
                }
                if row.at(C_GOV_CORRUPT) == 1
                    && row.at(C_VITAL_FLOOR) == 1
                    && number(line, "cause") == Some(1)
                {
                    census.saw_q2_before_q3 = true;
                }
            }
            FN_ACCEPT => {
                if let Some(verdict) = number(line, "verdict") {
                    if (0..4).contains(&verdict) {
                        census.verdicts[verdict as usize] += 1;
                    }
                    if verdict == 2 {
                        if let Some(site) = number(line, "site") {
                            if (1..=7).contains(&site) {
                                census.collapse_sites[site as usize] += 1;
                            }
                        }
                    }
                    if verdict == 3 && number(line, "reason") == Some(1) {
                        census.saw_unnamed_collapse = true;
                    }
                }
            }
            FN_DIAGNOSE => {
                match cell(line, "binding") {
                    Some("-") => census.bindings[0] += 1,
                    Some(value) => {
                        if let Ok(axis) = value.parse::<usize>() {
                            if axis < 4 {
                                census.bindings[axis] += 1;
                            }
                        }
                    }
                    None => {}
                }
                if number(line, "absorbed") == Some(1) {
                    census.saw_absorption = true;
                    if line.contains(" vital=-") {
                        census.saw_missing_score = true;
                    }
                }
                for key in ["epi", "vital", "gov"] {
                    if cell(line, key) == Some("-") {
                        census.saw_missing_score = true;
                    }
                }
            }
            FN_SEAL => {}
            _ => {}
        }
    }
    census
}

/// Read the raw cell `key=<value>` out of a canonical line.
fn cell<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    let prefix = format!("{key}=");
    line.split(' ')
        .find_map(|token| token.strip_prefix(&prefix))
}

/// Read `key=<int>`; `None` both for an absent key and for the `-` placeholder.
fn number(line: &str, key: &str) -> Option<i64> {
    cell(line, key).and_then(|value| value.parse().ok())
}
