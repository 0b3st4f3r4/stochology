//! The topology of the thirty-five orders, and the parity of Table B.1.
//!
//! This module is an *explorer*, not a judge of the corpus: it enumerates the
//! $\binom{7}{3} = 35$ triples of guard sites, derives the Vandermonde partition,
//! parses Appendix B's Table B.1 and the recorded stratum anchors out of the
//! manuscript, and reconciles the three against each other. It lives outside the
//! acceptance path — no verdict of the core is computed here, and nothing here
//! can make `accept` return anything.
//!
//! The anchor list is a *recorded designation* (§V-10, Appendix B.2, the preface's
//! own wording): the table does not derive it and neither does this module. What
//! is checked is that the manuscript's record is internally consistent — seven
//! anchors, one per stratum, each the primary site of exactly one order in that
//! stratum — and that the table's triples exhaust the combinatorial space.

use stoch_core::Axis;

/// The seven guard sites, as the manuscript's `T1`..`T7`.
pub const SITES: [u8; 7] = [1, 2, 3, 4, 5, 6, 7];

/// The axis partition of the circuit: `3/2/2` over epistemic, vital, governance.
///
/// `T1`–`T3` are the epistemic transits, `T4`–`T5` the vital ones, `T6`–`T7` the
/// governance ones (Appendix E's Table E.1 with the Filter I/II/III grouping of
/// Appendix B; corroborated by every signature in Table B.1).
pub fn axis_of(site: u8) -> Axis {
    match site {
        1..=3 => Axis::Epistemic,
        4..=5 => Axis::Vital,
        6..=7 => Axis::Governance,
        other => panic!("no guard site T{other}"),
    }
}

/// `binom(n, k)`, integer division exact because it is built by the multiplicative
/// form with the divisor removed before the multiply.
pub fn binom(n: u64, k: u64) -> u64 {
    if k > n {
        return 0;
    }
    let k = k.min(n - k);
    let mut result = 1u64;
    for i in 0..k {
        result = result * (n - i) / (i + 1);
    }
    result
}

/// All $\binom{7}{3}$ triples of distinct sites, in lexicographic order. This is
/// the enumeration the table must exhaust, generated here rather than copied from
/// the table, so that agreement is a check and not a tautology.
pub fn all_triples() -> Vec<[u8; 3]> {
    let mut triples = Vec::new();
    for a in SITES {
        for b in SITES {
            for c in SITES {
                if a < b && b < c {
                    triples.push([a, b, c]);
                }
            }
        }
    }
    triples
}

/// The `(E, V, G)` signature of a triple: how many of its sites lie on each axis.
pub fn signature(triple: [u8; 3]) -> (u8, u8, u8) {
    let mut counts = (0u8, 0u8, 0u8);
    for site in triple {
        match axis_of(site) {
            Axis::Epistemic => counts.0 += 1,
            Axis::Vital => counts.1 += 1,
            Axis::Governance => counts.2 += 1,
        }
    }
    counts
}

/// The Vandermonde partition, computed from the axis partition rather than
/// quoted: $\sum_{k_E+k_V+k_G=3}\binom{3}{k_E}\binom{2}{k_V}\binom{2}{k_G}$.
pub fn vandermonde() -> Vec<((u8, u8, u8), u64)> {
    let mut classes = Vec::new();
    for ke in 0..=3u8 {
        for kv in 0..=2u8 {
            for kg in 0..=2u8 {
                if ke + kv + kg != 3 {
                    continue;
                }
                let count = binom(3, ke as u64) * binom(2, kv as u64) * binom(2, kg as u64);
                classes.push(((ke, kv, kg), count));
            }
        }
    }
    classes
}

/// The exposure regime the manuscript names for a signature.
pub fn exposure_of(signature: (u8, u8, u8)) -> &'static str {
    match signature {
        (1, 1, 1) => "Trans-Triadic Keystone",
        (3, 0, 0) => "Pure Epistemic Concentration",
        (0, _, _) => "Epistemic Axial Exposure",
        (_, 0, _) => "Vital Axial Exposure",
        (_, _, 0) => "Governance Axial Exposure",
        _ => "unclassified",
    }
}

/// One row of Appendix B's Table B.1.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TableRow {
    pub order: String,
    pub chapter: u32,
    pub stratum: u8,
    pub title: String,
    pub primary: u8,
    pub triple: [u8; 3],
    pub signature: (u8, u8, u8),
    pub exposure: String,
}

/// One recorded stratum anchor: the site a stratum is named by, and the chapter
/// run the designation accompanies.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Anchor {
    pub stratum: u8,
    pub first_chapter: u32,
    pub last_chapter: u32,
    pub site: u8,
}

/// Parse Table B.1 out of the appendix text. Only the order rows are read; the
/// header and the alignment row carry no designation.
pub fn parse_table(text: &str) -> Result<Vec<TableRow>, String> {
    let mut rows = Vec::new();
    for (index, line) in text.lines().enumerate() {
        let line = line.trim();
        if !line.starts_with("| `L") {
            continue;
        }
        let cells: Vec<&str> = line.split('|').map(str::trim).collect();
        // cells[0] is empty (leading pipe); the row has nine cells plus that one.
        if cells.len() < 10 {
            return Err(format!(
                "table line {}: expected at least 9 cells, found {}",
                index + 1,
                cells.len() - 1
            ));
        }
        let order = cells[1].trim_matches('`').to_string();
        let chapter: u32 = cells[2]
            .parse()
            .map_err(|_| format!("table line {}: chapter {:?}", index + 1, cells[2]))?;
        let stratum = cells[3]
            .trim_matches('`')
            .strip_prefix('S')
            .and_then(|s| s.parse::<u8>().ok())
            .ok_or_else(|| format!("table line {}: stratum {:?}", index + 1, cells[3]))?;
        let primary = site_cell(cells[5])
            .ok_or_else(|| format!("table line {}: primary site {:?}", index + 1, cells[5]))?;
        let triple = triple_cell(cells[6])
            .ok_or_else(|| format!("table line {}: triple {:?}", index + 1, cells[6]))?;
        let signature = signature_cell(cells[7])
            .ok_or_else(|| format!("table line {}: signature {:?}", index + 1, cells[7]))?;
        rows.push(TableRow {
            order,
            chapter,
            stratum,
            title: cells[4].to_string(),
            primary,
            triple,
            signature,
            exposure: cells[8].to_string(),
        });
    }
    Ok(rows)
}

/// Parse the preface's recorded anchor list: one `Stratum S_n (Chapters a–b) ↔ T_m`
/// bullet per stratum.
pub fn parse_anchors(text: &str) -> Result<Vec<Anchor>, String> {
    let mut anchors = Vec::new();
    for line in text.lines() {
        let trimmed = line.trim_start();
        if !trimmed.starts_with("- **Stratum ") {
            continue;
        }
        let stratum = trimmed
            .split("S\\_")
            .nth(1)
            .and_then(|rest| rest.chars().next())
            .and_then(|c| c.to_digit(10))
            .ok_or_else(|| format!("anchor line without a stratum number: {line}"))? as u8;
        let chapters = trimmed
            .split("Chapters ")
            .nth(1)
            .ok_or_else(|| format!("anchor line without a chapter run: {line}"))?;
        // The run is written `51–55` (en dash); a hyphen is accepted too.
        let run = chapters
            .split(')')
            .next()
            .ok_or_else(|| format!("anchor line without a chapter run: {line}"))?;
        let separator = if run.contains('\u{2013}') { '\u{2013}' } else { '-' };
        let mut parts = run.split(separator);
        let first: String = parts
            .next()
            .ok_or_else(|| format!("anchor line without a first chapter: {line}"))?
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect();
        let last: String = parts
            .next()
            .ok_or_else(|| format!("anchor line without a last chapter: {line}"))?
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect();
        let first_chapter: u32 = first
            .parse()
            .map_err(|_| format!("anchor line without a first chapter: {line}"))?;
        let last_chapter: u32 = last
            .parse()
            .map_err(|_| format!("anchor line without a last chapter: {line}"))?;
        let site = trimmed
            .split("leftrightarrow T\\_")
            .nth(1)
            .and_then(|rest| rest.chars().next())
            .and_then(|c| c.to_digit(10))
            .ok_or_else(|| format!("anchor line without a site: {line}"))? as u8;
        anchors.push(Anchor {
            stratum,
            first_chapter,
            last_chapter,
            site,
        });
    }
    // The list is read in the order it is written; a missing stratum is reported
    // by the reconciler rather than hidden by a sort.
    anchors.sort_by_key(|anchor| anchor.stratum);
    Ok(anchors)
}

/// Per-stratum summary, for the explorer's printed output.
#[derive(Clone, Debug)]
pub struct StratumReport {
    pub stratum: u8,
    pub orders: Vec<String>,
    pub primaries: Vec<u8>,
    pub anchor: Option<u8>,
    pub signatures: Vec<(u8, u8, u8)>,
}

/// Everything the reconciler established, so the caller can print it.
#[derive(Clone, Debug)]
pub struct Parity {
    pub table_rows: usize,
    pub triples_exhausted: usize,
    pub classes: Vec<((u8, u8, u8), u64)>,
    pub strata: Vec<StratumReport>,
    pub anchors: Vec<Anchor>,
    pub per_site_primary: [usize; 8],
}

/// Reconcile Table B.1 with the enumeration, the Vandermonde partition and the
/// recorded anchors. Returns every divergence it found, not just the first.
pub fn reconcile(table_text: &str, preface_text: &str) -> Result<Parity, Vec<String>> {
    let mut errors = Vec::new();
    let rows = match parse_table(table_text) {
        Ok(rows) => rows,
        Err(error) => return Err(vec![error]),
    };
    let anchors = match parse_anchors(preface_text) {
        Ok(anchors) => anchors,
        Err(error) => return Err(vec![error]),
    };

    if rows.len() != 35 {
        errors.push(format!("Table B.1 has {} rows, expected 35", rows.len()));
    }

    // Orders are L01..L35 and chapters are 51..85, both monotone and one-to-one.
    for (index, row) in rows.iter().enumerate() {
        let expected_order = format!("L{:02}", index + 1);
        if row.order != expected_order {
            errors.push(format!(
                "row {} is {:?}, expected {:?}",
                index + 1,
                row.order,
                expected_order
            ));
        }
        let expected_chapter = 51 + index as u32;
        if row.chapter != expected_chapter {
            errors.push(format!(
                "{}: chapter {}, expected {}",
                row.order, row.chapter, expected_chapter
            ));
        }
        let expected_stratum = ((row.chapter - 51) / 5 + 1) as u8;
        if row.stratum != expected_stratum {
            errors.push(format!(
                "{}: stratum S{}, but chapter {} belongs to S{}",
                row.order, row.stratum, row.chapter, expected_stratum
            ));
        }
        if !row.triple.contains(&row.primary) {
            errors.push(format!(
                "{}: primary T{} is not in {:?}",
                row.order, row.primary, row.triple
            ));
        }
        let derived = signature(row.triple);
        if row.signature != derived {
            errors.push(format!(
                "{}: signature {:?} but triple {:?} gives {:?}",
                row.order, row.signature, row.triple, derived
            ));
        }
        let exposure = exposure_of(derived);
        if row.exposure != exposure {
            errors.push(format!(
                "{}: exposure {:?}, but signature {:?} is {:?}",
                row.order, row.exposure, derived, exposure
            ));
        }
    }

    // The triples must be exactly the combinatorial space: 35 distinct triples,
    // each carried by one order.
    let universe = all_triples();
    let mut seen: Vec<[u8; 3]> = Vec::new();
    for row in &rows {
        if row.triple[0] >= row.triple[1] || row.triple[1] >= row.triple[2] {
            errors.push(format!("{}: {:?} is not a sorted triple", row.order, row.triple));
        }
        if !(1..=7).contains(&row.triple[0])
            || !(1..=7).contains(&row.triple[2])
        {
            errors.push(format!("{}: {:?} names a site outside T1..T7", row.order, row.triple));
        }
        if seen.contains(&row.triple) {
            errors.push(format!("{}: triple {:?} is carried twice", row.order, row.triple));
        }
        seen.push(row.triple);
    }
    seen.sort();
    seen.dedup();
    if seen != universe {
        for triple in &universe {
            if !seen.contains(triple) {
                errors.push(format!("triple {:?} is carried by no order", triple));
            }
        }
    }

    // The Vandermonde partition, against the table's own signatures.
    let classes = vandermonde();
    let mut actual: Vec<((u8, u8, u8), u64)> = Vec::new();
    for row in &rows {
        match actual.iter_mut().find(|(key, _)| *key == row.signature) {
            Some((_, count)) => *count += 1,
            None => actual.push((row.signature, 1)),
        }
    }
    for (key, count) in &classes {
        let found = actual
            .iter()
            .find(|(candidate, _)| candidate == key)
            .map(|(_, count)| *count)
            .unwrap_or(0);
        if found != *count {
            errors.push(format!(
                "Vandermonde class {key:?}: {count} predicted, {found} in the table"
            ));
        }
    }
    let total: u64 = classes.iter().map(|(_, count)| count).sum();
    if total != 35 {
        errors.push(format!("the Vandermonde classes sum to {total}, expected 35"));
    }
    let table_total: u64 = actual.iter().map(|(_, count)| count).sum();
    if table_total != 35 {
        errors.push(format!("the table's classes sum to {table_total}, expected 35"));
    }

    // Per stratum: five orders, five distinct primary sites.
    let mut strata = Vec::new();
    for stratum in 1..=7u8 {
        let members: Vec<&TableRow> = rows.iter().filter(|r| r.stratum == stratum).collect();
        let orders: Vec<String> = members.iter().map(|r| r.order.clone()).collect();
        let primaries: Vec<u8> = members.iter().map(|r| r.primary).collect();
        let signatures: Vec<(u8, u8, u8)> = members.iter().map(|r| r.signature).collect();
        if members.len() != 5 {
            errors.push(format!("S{stratum} carries {} orders, expected 5", members.len()));
        }
        let mut distinct = primaries.clone();
        distinct.sort();
        distinct.dedup();
        if distinct.len() != primaries.len() {
            errors.push(format!("S{stratum}: two orders share a primary site"));
        }
        strata.push(StratumReport {
            stratum,
            orders,
            primaries,
            anchor: anchors.iter().find(|a| a.stratum == stratum).map(|a| a.site),
            signatures,
        });
    }

    // Each of the seven sites is primary in exactly five strata — equivalently,
    // of exactly five orders.
    let mut per_site_primary = [0usize; 8];
    for row in &rows {
        if (1..=7).contains(&row.primary) {
            per_site_primary[row.primary as usize] += 1;
        }
    }
    for site in 1..=7usize {
        if per_site_primary[site] != 5 {
            errors.push(format!(
                "T{site} is primary in {} orders, expected 5",
                per_site_primary[site]
            ));
        }
    }

    // The recorded anchors: seven, one per stratum, each the primary site of
    // exactly one order of its own stratum. The designation is read, not derived.
    if anchors.len() != 7 {
        errors.push(format!("{} stratum anchors recorded, expected 7", anchors.len()));
    }
    let mut anchor_sites = Vec::new();
    for anchor in &anchors {
        anchor_sites.push(anchor.site);
        let members: Vec<&TableRow> = rows
            .iter()
            .filter(|r| r.stratum == anchor.stratum)
            .collect();
        let carrying = members.iter().filter(|r| r.primary == anchor.site).count();
        if carrying != 1 {
            errors.push(format!(
                "S{}: anchor T{} is the primary site of {carrying} orders, expected 1",
                anchor.stratum, anchor.site
            ));
        }
        let expected_first = 51 + (anchor.stratum as u32 - 1) * 5;
        if anchor.first_chapter != expected_first || anchor.last_chapter != expected_first + 4 {
            errors.push(format!(
                "S{}: anchor names chapters {}–{}, expected {}–{}",
                anchor.stratum,
                anchor.first_chapter,
                anchor.last_chapter,
                expected_first,
                expected_first + 4
            ));
        }
    }
    anchor_sites.sort();
    anchor_sites.dedup();
    if anchor_sites.len() != anchors.len() {
        errors.push("two strata are recorded with the same anchor site".to_string());
    }

    if errors.is_empty() {
        Ok(Parity {
            table_rows: rows.len(),
            triples_exhausted: seen.len(),
            classes,
            strata,
            anchors,
            per_site_primary,
        })
    } else {
        Err(errors)
    }
}

/// The explorer's report: what the topology is, stratum by stratum.
pub fn render(parity: &Parity) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "Table B.1: {} rows, {} distinct triples of binom(7,3) = {}\n",
        parity.table_rows,
        parity.triples_exhausted,
        binom(7, 3)
    ));
    let mut total = 0u64;
    out.push_str("Vandermonde partition:\n");
    for (key, count) in &parity.classes {
        total += count;
        out.push_str(&format!(
            "  (E,V,G) = {key:?}: {count}  [{}, {}]\n",
            exposure_of(*key),
            match key {
                (1, 1, 1) => "trans-triadic keystone",
                (3, 0, 0) => "pure epistemic concentration",
                (0, _, _) => "epistemic-exposed",
                (_, 0, _) => "vital-exposed",
                _ => "governance-exposed",
            }
        ));
    }
    out.push_str(&format!("  total: {total}\n"));
    for stratum in &parity.strata {
        out.push_str(&format!(
            "S{} anchor T{}  orders {}  primaries {:?}\n",
            stratum.stratum,
            stratum.anchor.unwrap_or(0),
            stratum.orders.join(","),
            stratum.primaries
        ));
    }
    out.push_str("primary counts:");
    for site in 1..=7 {
        out.push_str(&format!(" T{site}={}", parity.per_site_primary[site]));
    }
    out.push('\n');
    out
}

fn site_cell(cell: &str) -> Option<u8> {
    cell.trim_matches('`')
        .strip_prefix('T')
        .and_then(|rest| rest.parse::<u8>().ok())
        .filter(|site| (1..=7).contains(site))
}

fn triple_cell(cell: &str) -> Option<[u8; 3]> {
    let inner = cell.trim_matches('`').trim();
    let inner = inner.strip_prefix('{')?.strip_suffix('}')?;
    let mut sites = [0u8; 3];
    let mut count = 0;
    for part in inner.split(',') {
        if count == 3 {
            return None;
        }
        sites[count] = site_cell(part.trim())?;
        count += 1;
    }
    if count != 3 {
        return None;
    }
    Some(sites)
}

fn signature_cell(cell: &str) -> Option<(u8, u8, u8)> {
    let inner = cell.trim_matches('`').trim();
    let inner = inner.strip_prefix('(')?.strip_suffix(')')?;
    let mut values = [0u8; 3];
    let mut count = 0;
    for part in inner.split(',') {
        if count == 3 {
            return None;
        }
        values[count] = part.trim().parse::<u8>().ok()?;
        count += 1;
    }
    if count != 3 {
        return None;
    }
    Some((values[0], values[1], values[2]))
}
