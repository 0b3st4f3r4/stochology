//! The fixture schema: one record per line, spelled `key=value`, its parser and
//! its renderer.
//!
//! This module knows the *shape* of a vector and nothing about its meaning. The
//! key names and their indices are the only vocabulary the harness shares with
//! the fixture, and they are declared once, here, so a key that moves is a compile
//! error rather than a silent mis-reading.
//!
//! The grammar, which is the whole of it:
//!
//! * a line whose first non-blank character is `#` is a comment; a blank line is
//!   ignored; every other line is exactly one record;
//! * a record is a sequence of `key=value` pairs separated by exactly one space,
//!   with no leading or trailing space, no tab and no carriage return;
//! * the keys are the names in [`HEADER`]; a key that is not one of them, a key
//!   that appears twice on a line, a field without `=` and a value that is not an
//!   integer are all errors, each naming its line and its key;
//! * a key that does not appear is absent, and absence reads as [`Row::ABSENT`];
//!   there is no second spelling of absence, so a fixture never writes the marker
//!   by hand;
//! * the canonical order of [`HEADER`] is how the file is written, so a human can
//!   read it in columns, but the parser does not depend on it: a record whose keys
//!   are shuffled parses to the same row.
//!
//! A record must carry the keys its function always carries — see
//! [`required_keys`] and the crate README. Without that rule a key dropped by
//! accident would read as absence and the row would be judged on a value it never
//! declared, which is the one failure the fixture exists to prevent.

/// Total number of columns, `id` included.
pub const COLUMNS: usize = 33;

pub const C_ID: usize = 0;
pub const C_FN: usize = 1;
pub const C_TAPE: usize = 2;
pub const C_INSTRUMENT: usize = 3;
pub const C_MISSING_SLOT: usize = 4;
pub const C_TTS: usize = 5;
pub const C_TTR: usize = 6;
pub const C_L: usize = 7;
pub const C_U: usize = 8;
pub const C_C: usize = 9;
pub const C_R: usize = 10;
pub const C_REVERSIBLE: usize = 11;
pub const C_GOV_CORRUPT: usize = 12;
pub const C_VITAL_FLOOR: usize = 13;
pub const C_GOV_FLOOR: usize = 14;
pub const C_EPI: usize = 15;
pub const C_VITAL: usize = 16;
pub const C_GOV: usize = 17;
pub const C_SITE: usize = 18;
pub const C_PROPOSITION: usize = 19;
pub const C_STEP_INDEX: usize = 20;
/// The three per-axis weakest readings of an accept row. Absence is Missing; the
/// Leontief fold is the minimum of the three, so a fixture that repeated one cell
/// on all three axes could never discriminate it.
pub const C_M_EPI: usize = 21;
pub const C_M_VITAL: usize = 22;
pub const C_M_GOV: usize = 23;
pub const C_ABSORBED: usize = 24;
pub const C_EXPECT_VARIANT: usize = 25;
pub const C_EXPECT_CAUSE: usize = 26;
pub const C_EXPECT_CONSTRAINT: usize = 27;
pub const C_EXPECT_FRACTION: usize = 28;
pub const C_EXPECT_VERDICT: usize = 29;
pub const C_EXPECT_BINDING: usize = 30;
pub const C_EXPECT_REASON: usize = 31;
pub const C_EXPECT_ERROR: usize = 32;

/// The four entry points the fixture drives. The numbering is the fixture's.
pub const FN_STEP: i64 = 0;
pub const FN_ACCEPT: i64 = 1;
pub const FN_DIAGNOSE: i64 = 2;
pub const FN_SEAL: i64 = 3;

/// The keys, in the order the file writes them.
pub const HEADER: [&str; COLUMNS] = [
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

/// The keys a `step` record always carries: the sentence the entry point reads.
const REQUIRED_STEP: &[&str] = &[
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
];

/// The keys an `accept` record always carries. The tape column is absent in these
/// rows — an accept sentence is attested by construction — so demanding it would
/// invent a field the entry point does not take from the fixture.
const REQUIRED_ACCEPT: &[&str] = &["fn", "l", "u", "c", "epi", "vital", "gov", "site"];

/// The keys a `diagnose` record always carries.
const REQUIRED_DIAGNOSE: &[&str] = &["fn", "absorbed", "expect_binding_axis", "expect_error"];

/// The keys a `seal` record always carries.
const REQUIRED_SEAL: &[&str] = &[
    "fn",
    "tape",
    "instrument",
    "missing_slot",
    "tts",
    "ttr",
    "u",
    "c",
    "r",
    "reversible",
];

/// The key every record carries whatever its function: the entry point. It is not
/// in the per-function sets because no record can be judged without it.
///
/// `expect_error` is required of `diagnose` only, and only because that entry
/// point has no other expectation column to fall back on; a step, accept or seal
/// record whose expectation is a success states it through `expect_variant`,
/// `expect_verdict` and the entry point itself.
const REQUIRED_ALWAYS: &[&str] = &["fn"];

/// The keys a record of function `function` must carry besides `id`.
///
/// The set is the intersection of what the harness dereferences for that entry
/// point and what every row of that function in the corpus states. Everything
/// else is genuinely optional — a `step` row that never probes has no
/// `expect_fraction`, and its absence is the honest spelling of that — so the
/// rule refuses an omission only where the omission would silently change the
/// judgement.
pub fn required_keys(function: i64) -> &'static [&'static str] {
    match function {
        FN_STEP => REQUIRED_STEP,
        FN_ACCEPT => REQUIRED_ACCEPT,
        FN_DIAGNOSE => REQUIRED_DIAGNOSE,
        FN_SEAL => REQUIRED_SEAL,
        _ => &["fn"],
    }
}

/// The key a boundary verdict reads its margin from: the axis the declared site
/// sits on. A boundary names one site and reads that site's own axis, so the
/// margin a `verdict=1` record must carry depends on its `site`.
pub fn axis_margin_key(site: i64) -> Option<&'static str> {
    match site {
        1..=3 => Some("m_epi"),
        4..=5 => Some("m_vital"),
        6..=7 => Some("m_gov"),
        _ => None,
    }
}

/// The keys the record's own expectation cannot be read without, beyond the ones
/// its function always carries.
///
/// A record whose `expect_error` is not `0` declares only the code it expects and
/// needs nothing more. A `step` record that expects success names the regime it
/// expects. An `accept` record names its verdict, and then, by verdict, the site's
/// own margin (1), the proposition and its step (2) or the reason (3).
fn conditional_keys(row: &Row) -> Vec<&'static str> {
    let mut keys: Vec<&'static str> = Vec::new();
    if row.at(C_EXPECT_ERROR) != 0 {
        return keys;
    }
    match row.function() {
        FN_STEP => keys.push("expect_variant"),
        FN_ACCEPT => {
            match row.at(C_EXPECT_VERDICT) {
                1 => {
                    if let Some(key) = axis_margin_key(row.at(C_SITE)) {
                        keys.push(key);
                    }
                }
                2 => {
                    keys.push("proposition");
                    keys.push("step_index");
                }
                3 => keys.push("expect_reason"),
                _ => {}
            }
            keys.push("expect_verdict");
        }
        _ => {}
    }
    keys
}

/// The column a key names, if the key is one of the declared ones.
pub fn column_of(key: &str) -> Option<usize> {
    HEADER.iter().position(|name| *name == key)
}

/// One fixture record. `values[column]` is the column's integer; `id` is the
/// record's name, which is a label and not a number.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Row {
    pub id: String,
    pub values: [i64; COLUMNS],
}

impl Row {
    /// The marker of an absent field, written by the parser and never by the
    /// fixture: a record says nothing about a field by not naming its key.
    ///
    /// The value is the one `stoch-core` already uses for a missing reading, so
    /// absence has one spelling in the whole harness.
    pub const ABSENT: i64 = -1;

    /// A row with every field absent.
    pub fn new(id: &str) -> Row {
        Row {
            id: id.to_string(),
            values: [Row::ABSENT; COLUMNS],
        }
    }

    pub fn set(&mut self, column: usize, value: i64) -> &mut Row {
        assert!(column < COLUMNS, "column {column} out of range");
        self.values[column] = value;
        self
    }

    pub fn at(&self, column: usize) -> i64 {
        assert!(column < COLUMNS, "column {column} out of range");
        self.values[column]
    }

    /// The fixture writes `1` for a set flag and says nothing for an unset one;
    /// only `1` is true.
    pub fn flag(&self, column: usize) -> bool {
        self.at(column) == 1
    }

    pub fn function(&self) -> i64 {
        self.at(C_FN)
    }

    /// Whether the record states this field at all.
    pub fn is_present(&self, column: usize) -> bool {
        self.at(column) != Row::ABSENT
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParseError {
    pub line: usize,
    pub message: String,
}

impl ParseError {
    fn new(line: usize, message: impl Into<String>) -> ParseError {
        ParseError {
            line,
            message: message.into(),
        }
    }
}

impl core::fmt::Display for ParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "vectors line {}: {}", self.line, self.message)
    }
}

/// Parse the fixture. Comments and blank lines are skipped; any other deviation
/// is an error carrying its line number and, where there is one, the key's name.
pub fn parse(text: &str) -> Result<Vec<Row>, ParseError> {
    let mut rows = Vec::new();
    for (index, raw) in text.split('\n').enumerate() {
        let line_number = index + 1;
        if raw.contains('\r') {
            return Err(ParseError::new(
                line_number,
                "carriage return: the fixture is spelled with LF only",
            ));
        }
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if line != raw {
            return Err(ParseError::new(
                line_number,
                "leading or trailing whitespace: pairs are separated by exactly one space",
            ));
        }
        if raw.contains('\t') {
            return Err(ParseError::new(
                line_number,
                "tab: pairs are separated by exactly one space",
            ));
        }
        if raw.contains(',') {
            return Err(ParseError::new(
                line_number,
                "comma: a record is key=value pairs, not CSV",
            ));
        }
        rows.push(parse_record(raw, line_number)?);
    }
    Ok(rows)
}

fn parse_record(line: &str, line_number: usize) -> Result<Row, ParseError> {
    let mut seen = [false; COLUMNS];
    let mut row = Row::new("");
    let mut function: Option<i64> = None;

    for field in line.split(' ') {
        if field.is_empty() {
            return Err(ParseError::new(
                line_number,
                "two spaces: pairs are separated by exactly one space",
            ));
        }
        let Some((key, spelling)) = field.split_once('=') else {
            return Err(ParseError::new(
                line_number,
                format!("field {field:?} is not key=value"),
            ));
        };
        let Some(column) = column_of(key) else {
            return Err(ParseError::new(
                line_number,
                format!("unknown key {key:?}; the keys are the declared schema"),
            ));
        };
        if seen[column] {
            return Err(ParseError::new(
                line_number,
                format!("key {key:?} appears twice on the same record"),
            ));
        }
        seen[column] = true;
        if column == C_ID {
            if spelling.is_empty() {
                return Err(ParseError::new(line_number, "empty id"));
            }
            row.id = spelling.to_string();
            continue;
        }
        let value: i64 = spelling.parse().map_err(|_| {
            ParseError::new(
                line_number,
                format!("key {key:?} is not an integer: {spelling:?}"),
            )
        })?;
        row.set(column, value);
        if column == C_FN {
            function = Some(value);
        }
    }

    if row.id.is_empty() {
        return Err(ParseError::new(line_number, "record without an id"));
    }
    let Some(function) = function else {
        return Err(ParseError::new(
            line_number,
            format!("record {} names no function", row.id),
        ));
    };
    if !matches!(function, FN_STEP | FN_ACCEPT | FN_DIAGNOSE | FN_SEAL) {
        return Err(ParseError::new(
            line_number,
            format!("record {}: fn={function} is not one of the four entry points", row.id),
        ));
    }
    for key in REQUIRED_ALWAYS.iter().chain(required_keys(function)) {
        let column = column_of(key).expect("a required key is a declared key");
        if !seen[column] {
            return Err(ParseError::new(
                line_number,
                format!(
                    "record {} is missing key {key:?}, which every fn={function} record carries",
                    row.id
                ),
            ));
        }
    }
    for key in conditional_keys(&row) {
        let column = column_of(key).expect("a required key is a declared key");
        if !seen[column] {
            return Err(ParseError::new(
                line_number,
                format!(
                    "record {} is missing key {key:?}, which its declared expectation reads",
                    row.id
                ),
            ));
        }
    }
    Ok(row)
}

/// Render records back to the fixture's own spelling: one record per line, keys in
/// the canonical order, absent fields omitted.
///
/// `parse(render(rows))` is the identity on rows the parser accepts, which is what
/// lets the corruption test mutate a real file rather than a synthetic one. The
/// three comment lines the shipped fixture opens with are not data and are not
/// written here, so the generated corpus does not claim to be the normative file.
pub fn render(rows: &[Row]) -> String {
    let mut out = String::new();
    for row in rows {
        out.push_str("id=");
        out.push_str(&row.id);
        for column in 1..COLUMNS {
            let value = row.at(column);
            if value == Row::ABSENT {
                continue;
            }
            out.push(' ');
            out.push_str(HEADER[column]);
            out.push('=');
            out.push_str(&value.to_string());
        }
        out.push('\n');
    }
    out
}

/// The column a row's expectation actually reads, so a corruption test can pick
/// a cell whose change must be visible. Returns `None` for a row whose only
/// expectation is that it succeeds.
pub fn expectation_column(row: &Row) -> Option<usize> {
    if row.at(C_EXPECT_ERROR) != 0 {
        return Some(C_EXPECT_ERROR);
    }
    match row.function() {
        FN_STEP => Some(C_EXPECT_VARIANT),
        FN_ACCEPT => Some(C_EXPECT_VERDICT),
        FN_DIAGNOSE => Some(C_EXPECT_BINDING),
        FN_SEAL => Some(C_EXPECT_ERROR),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// One `accept` record in the canonical order: a boundary at `T2`, which reads
    /// `m_epi` and therefore states it.
    const RECORD: &str = "id=r01 fn=1 l=200 u=400 c=120 epi=1 vital=1 gov=1 site=2 m_epi=101000 expect_verdict=1 expect_error=0";

    fn one(text: &str) -> Row {
        let mut rows = parse(text).expect("the record parses");
        assert_eq!(rows.len(), 1);
        rows.pop().expect("one record")
    }

    #[test]
    fn the_key_order_carries_no_meaning() {
        let canonical = one(&format!("{RECORD}\n"));
        let fields: Vec<&str> = RECORD.split(' ').collect();

        let mut reversed = fields.clone();
        reversed.reverse();
        let mut rotated = fields.clone();
        rotated.rotate_left(4);
        let mut swapped = fields.clone();
        swapped.swap(1, 9);

        for order in [reversed, rotated, swapped] {
            let text = format!("{}\n", order.join(" "));
            assert_eq!(one(&text), canonical, "{text}");
        }
        // And the rendering comes back in the canonical order whatever came in.
        assert_eq!(render(&[canonical]), format!("{RECORD}\n"));
    }

    #[test]
    fn an_absent_key_is_absent_and_is_not_written() {
        let row = one("id=r02 fn=3 tape=1 instrument=0 missing_slot=0 tts=10 ttr=10 u=400 c=120 r=0 reversible=1 expect_error=0\n");
        assert_eq!(row.at(C_L), Row::ABSENT, "a key the record omits is absent");
        assert!(!row.is_present(C_L));
        let text = render(&[row.clone()]);
        assert!(!text.contains(" l="), "{text}");
        assert!(!text.contains("=-1"), "no absence is written as a value: {text}");
        assert_eq!(one(&text), row, "render drops it and parse restores it");
    }

    #[test]
    fn comments_and_blank_lines_carry_no_record() {
        let text = "# a comment\n\n   \n  # an indented comment\nid=r03 fn=2 absorbed=1 expect_binding_axis=2 expect_error=0\n";
        let rows = parse(text).expect("parses");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].id, "r03");
    }

    #[test]
    fn a_required_key_that_was_dropped_is_an_error() {
        let error = parse("id=r04 fn=1 l=200 u=400 c=120 epi=1 vital=1 gov=1 expect_error=0\n")
            .expect_err("a boundary record without its site cannot be judged");
        assert_eq!(error.line, 1);
        assert!(error.message.contains("missing key \"site\""), "{error}");
        assert!(error.message.contains("fn=1"), "{error}");

    }

    #[test]
    fn a_conditional_key_that_was_dropped_is_an_error() {
        let error = parse("id=r06 fn=1 l=200 u=400 c=120 epi=1 vital=1 gov=1 site=1 m_epi=101000 expect_error=0\n")
            .expect_err("a verdict record without its verdict cannot be declared");
        assert!(error.message.contains("missing key \"expect_verdict\""), "{error}");

        let error = parse("id=r07 fn=1 l=200 u=400 c=120 epi=1 vital=1 gov=1 site=1 expect_verdict=1 expect_error=0\n")
            .expect_err("a boundary names the margin of its own axis");
        assert!(error.message.contains("missing key \"m_epi\""), "{error}");

        let error = parse("id=r08 fn=1 l=200 u=400 c=120 epi=1 vital=1 gov=1 site=1 expect_verdict=2 expect_error=0\n")
            .expect_err("a collapse names the proposition it collapses at");
        assert!(error.message.contains("proposition"), "{error}");

        let error = parse("id=r09 fn=1 l=200 u=400 c=120 epi=1 vital=1 gov=1 site=1 expect_verdict=3 expect_error=0\n")
            .expect_err("an unparsable record names its reason");
        assert!(error.message.contains("expect_reason"), "{error}");

        let error = parse("id=r10 fn=0 tape=1 instrument=0 missing_slot=0 tts=10 ttr=10 l=200 u=400 c=120 r=0 reversible=1 expect_error=0\n")
            .expect_err("a step record that is not an error names its regime");
        assert!(error.message.contains("expect_variant"), "{error}");
    }

    #[test]
    fn an_unknown_key_names_the_key_and_the_line() {
        let error = parse("id=r11 fn=2 absorbed=1 expect_binding_axis=2 expect_error=0\nid=r12 fn=2 absorbed=1 expect_binding_axis=2 expect_error=0 verdict=1\n")
            .expect_err("an unknown key is not ignored");
        assert_eq!(error.line, 2);
        assert!(error.message.contains("unknown key \"verdict\""), "{error}");
    }

    #[test]
    fn the_deviations_are_each_an_error_on_their_line() {
        let base = "id=r13 fn=2 absorbed=1 expect_binding_axis=2 expect_error=0";
        for (text, needle) in [
            (format!("{base} id=r14\n"), "appears twice"),
            (format!("{base} extra\n"), "is not key=value"),
            (format!("{base}  absorbed=0\n"), "two spaces"),
            (format!("{base} \n"), "leading or trailing whitespace"),
            ("id=r15 fn=2 absorbed=1 expect_binding_axis=2 expect_error=zero\n".to_string(), "not an integer"),
            ("id=r16 fn=2 absorbed=1,expect_binding_axis=2 expect_error=0\n".to_string(), "not CSV"),
            ("id=r17\tfn=2 absorbed=1 expect_binding_axis=2 expect_error=0\n".to_string(), "tab"),
            (format!("{base}\r\n"), "carriage return"),
            ("id=r18 fn=9 absorbed=1 expect_binding_axis=2 expect_error=0\n".to_string(), "four entry points"),
            ("absorbed=1 expect_binding_axis=2 expect_error=0\n".to_string(), "without an id"),
        ] {
            let error = parse(&text).expect_err(&text);
            assert!(error.message.contains(needle), "{text}: {error}");
        }
    }

    #[test]
    fn a_rendered_corpus_round_trips() {
        let mut step = Row::new("r19");
        step.set(C_FN, FN_STEP)
            .set(C_TAPE, 1)
            .set(C_INSTRUMENT, 0)
            .set(C_MISSING_SLOT, 0)
            .set(C_TTS, 10)
            .set(C_TTR, 10)
            .set(C_L, 200)
            .set(C_U, 400)
            .set(C_C, 300)
            .set(C_R, 0)
            .set(C_REVERSIBLE, 1)
            .set(C_EXPECT_VARIANT, 1)
            .set(C_EXPECT_ERROR, 0);
        let mut diagnose = Row::new("r20");
        diagnose
            .set(C_FN, FN_DIAGNOSE)
            .set(C_ABSORBED, 1)
            .set(C_EXPECT_BINDING, 2)
            .set(C_EXPECT_ERROR, 0);
        let rows = vec![step, diagnose];
        assert_eq!(parse(&render(&rows)).expect("round trip"), rows);
    }
}
