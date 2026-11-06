//! Empirical CSV dataset loaders (`data/*.csv`) and SHA-256 manifest verifier.

use crate::csv::CsvTable;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use stochology::{CausalEnvironment, cromwell_clamp, parse_quarter, sha256_hex};

/// Resolves the workspace root directory containing `data/SHA256SUMS`.
///
/// Anchors resolution to `CARGO_MANIFEST_DIR` first (preventing `cwd` spoofing), then executable ancestors,
/// and falls back to `current_dir()` only when running as an installed standalone binary.
pub fn resolve_workspace_root() -> PathBuf {
    if let Some(manifest_root) = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        && manifest_root.join("data/SHA256SUMS").is_file()
    {
        return manifest_root.to_path_buf();
    }
    if let Ok(exe) = std::env::current_exe() {
        for ancestor in exe.ancestors().skip(1) {
            if ancestor.join("data/SHA256SUMS").is_file() {
                return ancestor.to_path_buf();
            }
        }
    }
    if let Ok(cwd) = std::env::current_dir()
        && cwd.join("data/SHA256SUMS").is_file()
    {
        return cwd;
    }
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .expect("workspace root")
        .to_path_buf()
}

/// Pinned repository dataset schema and SHA-256 specification (`rel_path, expected_cols, expected_rows, first_col, last_col, sha256`),
/// verified at startup and optionally confronted against a live external mirror via `STOCHOLOGY_EXTERNAL_URL` or `STOCHOLOGY_EXTERNAL_CSV_PATH`.
#[allow(clippy::type_complexity)]
pub const UPSTREAM_DATASET_CONTRACTS: [(&str, usize, usize, &str, &str, &str); 14] = [
    (
        "data/atlantic_storms.csv",
        14,
        20_778,
        "rownames",
        "hurricane_force_diameter",
        "f7c58accf7472a32ee92218afba12843ae3952529ce8247ba7ae0db6c7602b6c",
    ),
    (
        "data/bank_calls.csv",
        3,
        27_716,
        "rownames",
        "Calls",
        "6e430180c3fa8a284e38b8367e9a7d6dae4921ca1b184cbaba7aae9b07a8f9a5",
    ),
    (
        "data/cps_observational.csv",
        12,
        15_992,
        "rownames",
        "re78",
        "e357e728bfefa729ff880377aca072aa01563a4b412d956dcf70f2539446cdf5",
    ),
    (
        "data/danish_fire.csv",
        2,
        2_167,
        "Date",
        "Loss",
        "82373e6e9828763324d6d20ecb0e0898f5fe2bb6a9f8290409ffbbfb9f7b606b",
    ),
    (
        "data/djia_1885_2023.csv",
        3,
        37_931,
        "rownames",
        "value",
        "3b9c0eea57ebb6a4443498798e5ec0b2fb49d2f406edd665ed973779ac15c526",
    ),
    (
        "data/houston_flights_2011.csv",
        9,
        224_591,
        "month",
        "arr_delay",
        "42770fa6ad078e3735654a5697ee566e07afb3f855f1aa277e02328924cccf2f",
    ),
    (
        "data/nsw_experimental.csv",
        12,
        445,
        "rownames",
        "re78",
        "72efd333a4b94a2bee55149d3542021d0adde03201d010ebe971e3e863360cd6",
    ),
    (
        "data/nyc_flights_2013.csv",
        9,
        141_193,
        "month",
        "arr_delay",
        "651ae28c4b9abb668eb420f23d512272ff539c060e1eb2b2741188717a8d3634",
    ),
    (
        "data/nyc_flights_2013_h1.csv",
        9,
        195_583,
        "month",
        "arr_delay",
        "7b119575c938e2b8c226da75a4508a9882e7f973bb08794df64efb2c7d94ebd7",
    ),
    (
        "data/nyc_flights_2023.csv",
        9,
        176_739,
        "month",
        "arr_delay",
        "0c6562bd0ad8daafe3216fbac457c721b7d1214625eb275471d29217695cc63b",
    ),
    (
        "data/nyc_flights_2023_h1.csv",
        9,
        258_613,
        "month",
        "arr_delay",
        "950bc37d305a3059a6379b62e6433ecbcb5ba0444d0e5b7355739ec9da396815",
    ),
    (
        "data/rtdsm_routput_vintages.csv",
        3,
        47_405,
        "vintage",
        "routput",
        "e0703a4b3cb3d0d1dd805e70f4977e3d9b03886151825a6e09543d690e87421c",
    ),
    (
        "data/toronto_arrests.csv",
        9,
        5_226,
        "rownames",
        "checks",
        "c6982b28e80bf1acc1c77e6ea2614e20671e169381655d39a34e0cf788591720",
    ),
    (
        "data/vic_elec.csv",
        6,
        52_608,
        "rownames",
        "Holiday",
        "21727117d26cf16384cbc9bfd895bb437cc797155646a39d1cc3feeff14da452",
    ),
];

/// Verifies a raw CSV byte payload against the pinned upstream schema contract for `rel_path`
/// (cryptographic SHA-256 digest, UTF-8 validity, exact column arity, exact row count, and header names).
/// Any external mutation to schema, row count, or digest fails immediately.
pub fn verify_external_csv_bytes_and_schema(rel_path: &str, bytes: &[u8]) -> Result<usize, String> {
    let Some(&(_, expected_cols, expected_rows, first_col, last_col, expected_sha)) =
        UPSTREAM_DATASET_CONTRACTS
            .iter()
            .find(|&&(p, _, _, _, _, _)| p == rel_path)
    else {
        return Err(format!(
            "Unknown dataset '{rel_path}' in UPSTREAM_DATASET_CONTRACTS"
        ));
    };
    let actual_sha = sha256_hex(bytes);
    if actual_sha != expected_sha {
        return Err(format!(
            "Upstream digest mismatch for '{rel_path}': expected {expected_sha}, got {actual_sha}"
        ));
    }
    let content = std::str::from_utf8(bytes)
        .map_err(|e| format!("Invalid UTF-8 in '{rel_path}': {e}"))?
        .to_string();
    let table = CsvTable::from_string(content);
    let rows = table.validate_schema(expected_cols)?;
    if rows != expected_rows {
        return Err(format!(
            "Row count mismatch for '{rel_path}': expected {expected_rows}, got {rows}"
        ));
    }
    let header = table
        .header()
        .ok_or_else(|| format!("Missing header in '{rel_path}'"))?;
    if header.str_at(0) != Some(first_col) || header.str_at(expected_cols - 1) != Some(last_col) {
        return Err(format!(
            "Header column name mismatch in '{rel_path}': expected ({first_col}..{last_col}), got ({:?}..{:?})",
            header.str_at(0),
            header.str_at(expected_cols - 1)
        ));
    }
    Ok(rows)
}

/// Optional live external/upstream verification hook (`STOCHOLOGY_EXTERNAL_CSV_PATH` or `STOCHOLOGY_EXTERNAL_URL`).
/// When either environment variable is set (e.g., in connected CI or external replication), fetches the external
/// artifact and verifies it against `verify_external_csv_bytes_and_schema(rel_path, &bytes)`.
pub fn verify_live_upstream_url_if_enabled(rel_path: &str) -> Result<bool, String> {
    if let Ok(ext_path) = std::env::var("STOCHOLOGY_EXTERNAL_CSV_PATH")
        && !ext_path.trim().is_empty()
    {
        let bytes = fs::read(ext_path.trim()).map_err(|e| {
            format!("Failed to read STOCHOLOGY_EXTERNAL_CSV_PATH '{ext_path}': {e}")
        })?;
        verify_external_csv_bytes_and_schema(rel_path, &bytes)?;
        return Ok(true);
    }
    if let Ok(ext_url) = std::env::var("STOCHOLOGY_EXTERNAL_URL")
        && !ext_url.trim().is_empty()
    {
        let out = std::process::Command::new("curl")
            .args(["-fsSL", "--max-time", "15", ext_url.trim()])
            .output()
            .map_err(|e| format!("Failed to invoke curl for STOCHOLOGY_EXTERNAL_URL: {e}"))?;
        if !out.status.success() {
            return Err(format!(
                "curl failed for STOCHOLOGY_EXTERNAL_URL '{}': status {}",
                ext_url.trim(),
                out.status
            ));
        }
        verify_external_csv_bytes_and_schema(rel_path, &out.stdout)?;
        return Ok(true);
    }
    Ok(false)
}

/// Verifies all 14 datasets against their pinned repository schema contracts (column arity, header names,
/// exact row counts, and SHA-256 digests) independently of `data/SHA256SUMS`, and runs live external URL/path
/// verification when `STOCHOLOGY_EXTERNAL_URL` or `STOCHOLOGY_EXTERNAL_CSV_PATH` is set.
pub fn verify_external_upstream_schema_contract(root_dir: &Path) -> Result<usize, String> {
    for &(rel_path, _, _, _, _, _) in &UPSTREAM_DATASET_CONTRACTS {
        let full_path = root_dir.join(rel_path);
        let bytes = fs::read(&full_path)
            .map_err(|e| format!("Failed to read '{rel_path}' for contract check: {e}"))?;
        verify_external_csv_bytes_and_schema(rel_path, &bytes)?;
    }
    let _ = verify_live_upstream_url_if_enabled("data/danish_fire.csv")?;
    Ok(UPSTREAM_DATASET_CONTRACTS.len())
}

/// Reads and verifies every file listed in `data/SHA256SUMS` against its cryptographic SHA-256 digest.
pub fn verify_sha256sums_manifest(root_dir: &Path) -> Result<usize, String> {
    let manifest_path = root_dir.join("data/SHA256SUMS");
    let content = fs::read_to_string(&manifest_path)
        .map_err(|e| format!("Failed to read '{}': {e}", manifest_path.display()))?;
    let mut verified_count = 0usize;
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let mut parts = trimmed.split_whitespace();
        let (Some(expected_hash), Some(rel_path)) = (parts.next(), parts.next()) else {
            return Err(format!("Malformed line in data/SHA256SUMS: '{trimmed}'"));
        };
        let file_path = root_dir.join(rel_path);
        let bytes = fs::read(&file_path)
            .map_err(|e| format!("Missing sealed file '{}': {e}", file_path.display()))?;
        let actual_hash = sha256_hex(&bytes);
        if actual_hash != expected_hash {
            return Err(format!(
                "SHA-256 mismatch for '{rel_path}': expected {expected_hash}, got {actual_hash}"
            ));
        }
        verified_count += 1;
    }
    Ok(verified_count)
}

/// Reads real Danish Fire Insurance losses (`data/danish_fire.csv`, $n = 2\,167$, in millions of DKK).
pub fn load_danish_fire(root_dir: &Path) -> Vec<f64> {
    let table = CsvTable::open(root_dir, "data/danish_fire.csv")
        .expect("failed to read data/danish_fire.csv");
    table.rows().filter_map(|r| r.f64_at(1)).collect()
}

/// Reads bank call counts grouped by operational day (`data/bank_calls.csv`),
/// returning daily clusters of 5-minute call-volume windows.
pub fn load_bank_call_clusters(root_dir: &Path) -> Vec<Vec<f64>> {
    let table = CsvTable::open(root_dir, "data/bank_calls.csv")
        .expect("failed to read data/bank_calls.csv");
    let mut by_day: BTreeMap<String, Vec<f64>> = BTreeMap::new();
    for row in table.rows() {
        if let (Some(ts), Some(calls)) = (row.str_at(1), row.f64_at(2)) {
            let date = ts.split('T').next().unwrap_or("").to_string();
            by_day.entry(date).or_default().push(calls);
        }
    }
    by_day.into_values().collect()
}

/// Reads real `re78` earnings from the LaLonde NSW randomized trial (`data/nsw_experimental.csv`,
/// treated vs. control) and the observational CPS sample (`data/cps_observational.csv`).
pub fn load_lalonde_nsw_cps(root_dir: &Path) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let read_file = |rel: &str| -> (Vec<f64>, Vec<f64>) {
        let table = CsvTable::open(root_dir, rel).expect("failed to read LaLonde CSV");
        let mut treated = Vec::new();
        let mut control = Vec::new();
        for row in table.rows() {
            if let (Some(treat_str), Some(re78)) = (row.str_at(2), row.f64_at(11)) {
                if treat_str == "1" {
                    treated.push(re78);
                } else {
                    control.push(re78);
                }
            }
        }
        (treated, control)
    };
    let (nsw_t, nsw_c) = read_file("data/nsw_experimental.csv");
    let (_, cps_c) = read_file("data/cps_observational.csv");
    (nsw_t, nsw_c, cps_c)
}

/// Reads `data/cps_observational.csv` real `re78` earnings (in thousands of 1978 USD)
/// grouped directly by `(age, educ)` demographic-educational cohort clusters without synthetic weights.
pub fn load_cps_cohort_clusters(root_dir: &Path) -> Vec<Vec<f64>> {
    let table = CsvTable::open(root_dir, "data/cps_observational.csv")
        .expect("failed to read data/cps_observational.csv");
    let mut by_cohort: BTreeMap<(u32, u32), Vec<f64>> = BTreeMap::new();
    for row in table.rows() {
        if let (Some(age), Some(educ), Some(re78)) = (row.u32_at(3), row.u32_at(4), row.f64_at(11))
        {
            by_cohort
                .entry((age, educ))
                .or_default()
                .push(re78 / 1_000.0);
        }
    }
    by_cohort.into_values().collect()
}

/// Reads daily arithmetic returns $(S_t - S_{t-1}) / S_{t-1}$ of the Dow Jones Industrial Average (`data/djia_1885_2023.csv`).
pub fn load_djia_returns(root_dir: &Path) -> Vec<f64> {
    let table = CsvTable::open(root_dir, "data/djia_1885_2023.csv")
        .expect("failed to read data/djia_1885_2023.csv");
    let closes: Vec<f64> = table
        .rows()
        .filter_map(|r| r.f64_at(2))
        .filter(|&c| c > 0.0)
        .collect();
    closes.windows(2).map(|w| (w[1] - w[0]) / w[0]).collect()
}

/// Reads Atlantic storm track observations (`data/atlantic_storms.csv`), pairing consecutive strict `+6h`
/// synoptic observations $(t \to t+1)$ along the **same `(name, year)` storm track** (`(hour_next + 24 - hour_t) % 24 == 6`),
/// fitting a logistic model for next-step (+6h lead-time) hurricane intensity $P(\text{wind}_{t+1} \ge 64\text{ kt} \mid \text{pressure}_t)$
/// on the first half of the historical track transitions, and returning `(forecasts_t_to_t1, outcomes_t1)`.
pub fn load_atlantic_storms_forecasts(root_dir: &Path) -> (Vec<f64>, Vec<f64>) {
    let table = CsvTable::open(root_dir, "data/atlantic_storms.csv")
        .expect("failed to read data/atlantic_storms.csv");
    let mut track_obs: Vec<(String, u32, u32, f64, f64)> = Vec::new();
    for row in table.rows() {
        if let (Some(name), Some(year), Some(hour), Some(wind), Some(pressure)) = (
            row.str_at(1),
            row.u32_at(2),
            row.u32_at(5),
            row.f64_at(10),
            row.f64_at(11),
        ) {
            track_obs.push((name.to_string(), year, hour, pressure, wind));
        }
    }
    let mut transitions: Vec<(f64, f64)> = Vec::with_capacity(track_obs.len());
    for pair in track_obs.windows(2) {
        let (ref name_t, year_t, hour_t, pressure_t, _) = pair[0];
        let (ref name_next, year_next, hour_next, _, wind_next) = pair[1];
        if name_t == name_next && year_t == year_next && (hour_next + 24 - (hour_t % 24)) % 24 == 6
        {
            let x_deficit = (1010.0 - pressure_t) / 20.0;
            let y_next = if wind_next >= 64.0 { 1.0 } else { 0.0 };
            transitions.push((x_deficit, y_next));
        }
    }

    // Fit logistic regression (beta0, beta1) on the first half of storm transitions via Newton-Raphson IRLS:
    let train_n = (transitions.len() / 2).max(1);
    let mut b0 = -1.5_f64;
    let mut b1 = 2.0_f64;
    for _ in 0..12 {
        let mut g0 = 0.0_f64;
        let mut g1 = 0.0_f64;
        let mut h00 = 1e-6_f64;
        let mut h01 = 0.0_f64;
        let mut h11 = 1e-6_f64;
        for &(x, y) in &transitions[..train_n] {
            let z = (b0 + b1 * x).clamp(-20.0, 20.0);
            let p = 1.0 / (1.0 + (-z).exp());
            let w = p * (1.0 - p);
            let r = y - p;
            g0 += r;
            g1 += r * x;
            h00 += w;
            h01 += w * x;
            h11 += w * x * x;
        }
        let det = h00 * h11 - h01 * h01;
        if det.abs() > 1e-12 {
            b0 += (h11 * g0 - h01 * g1) / det;
            b1 += (h00 * g1 - h01 * g0) / det;
        }
    }

    let mut probs = Vec::with_capacity(transitions.len());
    let mut outcomes = Vec::with_capacity(transitions.len());
    for &(x, y) in &transitions {
        let z = (b0 + b1 * x).clamp(-20.0, 20.0);
        let p = cromwell_clamp(1.0 / (1.0 + (-z).exp()), 0.001);
        probs.push(p);
        outcomes.push(y);
    }
    (probs, outcomes)
}

/// Splits `data/atlantic_storms.csv` into 3 seasonal regimes (`fringe`, `shoulder_aug_oct`, `peak_sep`)
/// to test Invariant Causal Prediction of barometric pressure deficit (`X0`) vs. latitude proxy (`X1`).
pub fn load_atlantic_storms_environments(root_dir: &Path) -> Vec<CausalEnvironment> {
    let table = CsvTable::open(root_dir, "data/atlantic_storms.csv")
        .expect("failed to read data/atlantic_storms.csv");
    let mut x_e0 = Vec::new();
    let mut y_e0 = Vec::new();
    let mut x_e1 = Vec::new();
    let mut y_e1 = Vec::new();
    let mut x_e2 = Vec::new();
    let mut y_e2 = Vec::new();

    for row in table.rows() {
        if let (Some(month), Some(lat), Some(wind), Some(pressure)) =
            (row.u32_at(3), row.f64_at(6), row.f64_at(10), row.f64_at(11))
        {
            let x0 = (1010.0 - pressure) / 20.0;
            let y = (wind - 50.0) / 25.0;
            let lat_z = (lat - 25.0) / 10.0;
            if !(8..=10).contains(&month) {
                x_e0.push(vec![x0, lat_z]);
                y_e0.push(y);
            } else if month == 8 || month == 10 {
                x_e1.push(vec![x0, lat_z]);
                y_e1.push(y);
            } else {
                x_e2.push(vec![x0, lat_z]);
                y_e2.push(y);
            }
        }
    }

    vec![
        CausalEnvironment {
            name: "storms_fringe_season".to_string(),
            x: x_e0,
            y: y_e0,
        },
        CausalEnvironment {
            name: "storms_shoulder_season_aug_oct".to_string(),
            x: x_e1,
            y: y_e1,
        },
        CausalEnvironment {
            name: "storms_peak_season_sep".to_string(),
            x: x_e2,
            y: y_e2,
        },
    ]
}

/// Reads half-hourly electricity demand observations (MW) of Victoria, Australia (`data/vic_elec.csv`).
pub fn load_vic_elec_demand(root_dir: &Path) -> Vec<f64> {
    let table =
        CsvTable::open(root_dir, "data/vic_elec.csv").expect("failed to read data/vic_elec.csv");
    table.rows().filter_map(|r| r.f64_at(2)).collect()
}

/// Reads the Philadelphia Fed real-time real-output vintages (`data/rtdsm_routput_vintages.csv`)
/// as `(vintage_quarter, obs_quarter, output_level)`.
pub fn load_rtdsm_routput_vintages(root_dir: &Path) -> Vec<(u32, u32, f64)> {
    let table = CsvTable::open(root_dir, "data/rtdsm_routput_vintages.csv")
        .expect("failed to read data/rtdsm_routput_vintages.csv");
    table
        .rows()
        .filter_map(|r| {
            Some((
                parse_quarter(r.str_at(0)?)?,
                parse_quarter(r.str_at(1)?)?,
                r.f64_at(2)?,
            ))
        })
        .collect()
}

/// Reads Toronto arrest records (`data/toronto_arrests.csv`) and returns empirical base rates
/// $(p_A, p_B)$ of non-release across two demographic strata with unequal base rates.
pub fn load_toronto_arrests_prevalences(root_dir: &Path) -> (f64, f64) {
    let (p_a, p_b, _, _, _, _) = load_toronto_arrests_full_summary(root_dir);
    (p_a, p_b)
}

/// Fallibly parses a `CsvTable` of Toronto arrest records (`data/toronto_arrests.csv`), rejecting short rows,
/// invalid categorical values, missing `checks` counts, or empty demographic strata.
pub fn try_load_toronto_arrests_full_summary_from_table(
    table: &CsvTable,
) -> Result<(f64, f64, f64, f64, usize, usize), String> {
    let mut a_pos = 0usize;
    let mut a_tot = 0usize;
    let mut b_pos = 0usize;
    let mut b_tot = 0usize;
    let mut black_all_pos = 0usize;
    let mut black_all_tot = 0usize;
    let mut white_all_pos = 0usize;
    let mut white_all_tot = 0usize;
    for (idx, row) in table.rows().enumerate() {
        if row.len() < 9 {
            return Err(format!(
                "Toronto arrests row {} has {} columns (expected >= 9)",
                idx + 1,
                row.len()
            ));
        }
        let not_released = match row.str_at(1) {
            Some("No") => true,
            Some("Yes") => false,
            other => {
                return Err(format!(
                    "Toronto arrests row {} has invalid released cell {:?}",
                    idx + 1,
                    other
                ));
            }
        };
        let group = row.str_at(2).unwrap_or("");
        let employed = match row.str_at(6) {
            Some("Yes") => true,
            Some("No") => false,
            other => {
                return Err(format!(
                    "Toronto arrests row {} has invalid employed cell {:?}",
                    idx + 1,
                    other
                ));
            }
        };
        if row.u32_at(8).is_none() {
            return Err(format!(
                "Toronto arrests row {} has missing or unparseable checks count {:?}",
                idx + 1,
                row.str_at(8)
            ));
        }
        match group {
            "Black" => {
                black_all_tot += 1;
                if not_released {
                    black_all_pos += 1;
                }
                if !employed {
                    a_tot += 1;
                    if not_released {
                        a_pos += 1;
                    }
                }
            }
            "White" => {
                white_all_tot += 1;
                if not_released {
                    white_all_pos += 1;
                }
                if employed {
                    b_tot += 1;
                    if not_released {
                        b_pos += 1;
                    }
                }
            }
            other => {
                return Err(format!(
                    "Toronto arrests row {} has unexpected colour group {:?}",
                    idx + 1,
                    other
                ));
            }
        }
    }
    if a_tot == 0 || b_tot == 0 || black_all_tot == 0 || white_all_tot == 0 {
        return Err("Toronto arrests dataset has an empty demographic stratum".to_string());
    }
    let p_a = (a_pos as f64) / (a_tot as f64);
    let p_b = (b_pos as f64) / (b_tot as f64);
    let p_black_all = (black_all_pos as f64) / (black_all_tot as f64);
    let p_white_all = (white_all_pos as f64) / (white_all_tot as f64);
    Ok((
        p_a,
        p_b,
        p_black_all,
        p_white_all,
        black_all_tot + white_all_tot,
        a_tot + b_tot,
    ))
}

/// Fallibly reads `data/toronto_arrests.csv` and returns the full demographic summary.
pub fn try_load_toronto_arrests_full_summary(
    root_dir: &Path,
) -> Result<(f64, f64, f64, f64, usize, usize), String> {
    let table = CsvTable::open(root_dir, "data/toronto_arrests.csv")?;
    try_load_toronto_arrests_full_summary_from_table(&table)
}

/// Reads all $n = 5\,226$ Toronto arrest records (`data/toronto_arrests.csv`) and returns:
/// 1. The intersectional race×employment poles `(p_black_unemp, p_white_emp)` ($n = 3\,577$), and
/// 2. The unconfounded full-sample race marginal `(p_black_all, p_white_all)` across all $n = 5\,226$ rows
///    ($1\,288$ Black vs. $3\,938$ White), along with `(n_total, n_stratum)`.
pub fn load_toronto_arrests_full_summary(root_dir: &Path) -> (f64, f64, f64, f64, usize, usize) {
    try_load_toronto_arrests_full_summary(root_dir)
        .expect("failed to parse data/toronto_arrests.csv")
}

/// Fallibly parses `data/toronto_arrests.csv` for the matched stratum and custody-risk classifier pairs,
/// rejecting missing or unparseable `checks`, invalid categorical fields, or empty strata.
#[allow(clippy::type_complexity)]
pub fn try_load_toronto_arrests_matched_and_classifier_from_table(
    table: &CsvTable,
) -> Result<(f64, f64, usize, Vec<(bool, bool)>, Vec<(bool, bool)>), String> {
    let mut bm_pos = 0usize;
    let mut bm_tot = 0usize;
    let mut wm_pos = 0usize;
    let mut wm_tot = 0usize;
    let mut black_pairs = Vec::new();
    let mut white_pairs = Vec::new();
    for (idx, row) in table.rows().enumerate() {
        if row.len() < 9 {
            return Err(format!(
                "Toronto arrests row {} has {} columns (expected >= 9)",
                idx + 1,
                row.len()
            ));
        }
        let not_released = match row.str_at(1) {
            Some("No") => true,
            Some("Yes") => false,
            other => {
                return Err(format!(
                    "Toronto arrests row {} has invalid released cell {:?}",
                    idx + 1,
                    other
                ));
            }
        };
        let group = row.str_at(2).unwrap_or("");
        let employed = match row.str_at(6) {
            Some("Yes") => true,
            Some("No") => false,
            other => {
                return Err(format!(
                    "Toronto arrests row {} has invalid employed cell {:?}",
                    idx + 1,
                    other
                ));
            }
        };
        let citizen = match row.str_at(7) {
            Some("Yes") => true,
            Some("No") => false,
            other => {
                return Err(format!(
                    "Toronto arrests row {} has invalid citizen cell {:?}",
                    idx + 1,
                    other
                ));
            }
        };
        let Some(checks) = row.u32_at(8) else {
            return Err(format!(
                "Toronto arrests row {} has missing or unparseable checks count {:?}",
                idx + 1,
                row.str_at(8)
            ));
        };
        let pred_high_risk = checks >= 3 || (!employed && !citizen && checks >= 1);
        match group {
            "Black" => {
                black_pairs.push((not_released, pred_high_risk));
                if !employed && citizen {
                    bm_tot += 1;
                    if not_released {
                        bm_pos += 1;
                    }
                }
            }
            "White" => {
                white_pairs.push((not_released, pred_high_risk));
                if !employed && citizen {
                    wm_tot += 1;
                    if not_released {
                        wm_pos += 1;
                    }
                }
            }
            other => {
                return Err(format!(
                    "Toronto arrests row {} has unexpected colour group {:?}",
                    idx + 1,
                    other
                ));
            }
        }
    }
    if bm_tot == 0 || wm_tot == 0 || black_pairs.is_empty() || white_pairs.is_empty() {
        return Err("Toronto arrests matched/classifier stratum is empty".to_string());
    }
    let p_bm = (bm_pos as f64) / (bm_tot as f64);
    let p_wm = (wm_pos as f64) / (wm_tot as f64);
    Ok((p_bm, p_wm, bm_tot + wm_tot, black_pairs, white_pairs))
}

/// Fallibly reads `data/toronto_arrests.csv` for matched covariates and classifier pairs.
#[allow(clippy::type_complexity)]
pub fn try_load_toronto_arrests_matched_and_classifier(
    root_dir: &Path,
) -> Result<(f64, f64, usize, Vec<(bool, bool)>, Vec<(bool, bool)>), String> {
    let table = CsvTable::open(root_dir, "data/toronto_arrests.csv")?;
    try_load_toronto_arrests_matched_and_classifier_from_table(&table)
}

/// Reads `data/toronto_arrests.csv` and returns:
/// 1. The **matched covariate stratum** holding employment and citizenship identical (`employed == "No" && citizen == "Yes"`
///    for both Black and White arrestees): `(p_black_matched, p_white_matched, n_matched)`, and
/// 2. The empirical `(not_released, high_risk_flag)` pairs for Black and White arrestees across all $n = 5\,226$ rows,
///    where `high_risk_flag` is the custody-risk threshold `checks >= 3 || (!employed && !citizen && checks >= 1)`.
#[allow(clippy::type_complexity)]
pub fn load_toronto_arrests_matched_and_classifier(
    root_dir: &Path,
) -> (f64, f64, usize, Vec<(bool, bool)>, Vec<(bool, bool)>) {
    try_load_toronto_arrests_matched_and_classifier(root_dir)
        .expect("failed to parse data/toronto_arrests.csv")
}

/// Reads untruncated arrival delays (minutes, positive = late, negative = early) from
/// `data/nyc_flights_2013.csv` and `data/nyc_flights_2023.csv`.
pub fn load_nyc_flight_delays(root_dir: &Path, max_rows: usize) -> (Vec<f64>, Vec<f64>) {
    let read_delays = |rel: &str| -> Vec<f64> {
        let table = CsvTable::open(root_dir, rel).expect("failed to read flight CSV");
        let all: Vec<f64> = table.rows().filter_map(|r| r.f64_at(8)).collect();
        if max_rows == 0 || all.len() <= max_rows {
            return all;
        }
        let n = all.len();
        (0..max_rows).map(|i| all[(i * n) / max_rows]).collect()
    };
    (
        read_delays("data/nyc_flights_2013.csv"),
        read_delays("data/nyc_flights_2023.csv"),
    )
}

/// Reads untruncated out-of-time (H1) and out-of-domain (Houston) arrival delays from
/// `data/nyc_flights_2013_h1.csv`, `data/nyc_flights_2023_h1.csv`, and `data/houston_flights_2011.csv`.
pub fn load_oot_ood_flight_delays(
    root_dir: &Path,
    max_rows: usize,
) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let read_delays = |rel: &str| -> Vec<f64> {
        let table = CsvTable::open(root_dir, rel).expect("failed to read OOT/OOD flight CSV");
        let all: Vec<f64> = table.rows().filter_map(|r| r.f64_at(8)).collect();
        if max_rows == 0 || all.len() <= max_rows {
            return all;
        }
        let n = all.len();
        (0..max_rows).map(|i| all[(i * n) / max_rows]).collect()
    };
    (
        read_delays("data/nyc_flights_2013_h1.csv"),
        read_delays("data/nyc_flights_2023_h1.csv"),
        read_delays("data/houston_flights_2011.csv"),
    )
}

/// Counts `(total_rows, valid_delay_rows, skipped_na_rows)` for a flight delay CSV (`data/*flights*.csv`).
pub fn count_flight_csv_rows_and_missing(root_dir: &Path, rel: &str) -> (usize, usize, usize) {
    let table = CsvTable::open(root_dir, rel).expect("failed to read flight CSV");
    let mut total = 0usize;
    let mut valid = 0usize;
    for row in table.rows() {
        total += 1;
        if row.f64_at(8).is_some() {
            valid += 1;
        }
    }
    (total, valid, total.saturating_sub(valid))
}

/// Counts `(total_rows, strict_6h_within_track_transition_pairs, excluded_terminal_or_non6h_rows)` in `data/atlantic_storms.csv`.
pub fn count_atlantic_storms_transitions(root_dir: &Path) -> (usize, usize, usize) {
    let table = CsvTable::open(root_dir, "data/atlantic_storms.csv")
        .expect("failed to read data/atlantic_storms.csv");
    let mut track_obs: Vec<(String, u32, u32)> = Vec::new();
    for row in table.rows() {
        if let (Some(name), Some(year), Some(hour), Some(_), Some(_)) = (
            row.str_at(1),
            row.u32_at(2),
            row.u32_at(5),
            row.f64_at(10),
            row.f64_at(11),
        ) {
            track_obs.push((name.to_string(), year, hour));
        }
    }
    let total = track_obs.len();
    let transitions = track_obs
        .windows(2)
        .filter(|pair| {
            pair[0].0 == pair[1].0
                && pair[0].1 == pair[1].1
                && (pair[1].2 + 24 - (pair[0].2 % 24)) % 24 == 6
        })
        .count();
    (total, transitions, total.saturating_sub(transitions))
}
