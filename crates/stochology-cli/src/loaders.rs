//! Empirical CSV dataset loaders (`data/*.csv`) and SHA-256 manifest verifier.

use crate::csv::CsvTable;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use stochology::{CausalEnvironment, cromwell_clamp, parse_quarter, sha256_hex};

/// Resolves the workspace root directory containing `data/SHA256SUMS`.
pub fn resolve_workspace_root() -> PathBuf {
    if let Ok(cwd) = std::env::current_dir()
        && cwd.join("data/SHA256SUMS").is_file()
    {
        return cwd;
    }
    if let Ok(exe) = std::env::current_exe() {
        for ancestor in exe.ancestors().skip(1) {
            if ancestor.join("data/SHA256SUMS").is_file() {
                return ancestor.to_path_buf();
            }
        }
    }
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .expect("workspace root")
        .to_path_buf()
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

/// Reads `data/cps_observational.csv` grouped by `(age, educ)` demographic-educational cohort clusters.
pub fn load_cps_cohort_clusters(root_dir: &Path) -> Vec<Vec<f64>> {
    let table = CsvTable::open(root_dir, "data/cps_observational.csv")
        .expect("failed to read data/cps_observational.csv");
    let mut by_cohort: BTreeMap<(u32, u32), Vec<f64>> = BTreeMap::new();
    for row in table.rows() {
        if let (Some(age), Some(educ), Some(nodeg), Some(re74), Some(re75), Some(re78)) = (
            row.u32_at(3),
            row.u32_at(4),
            row.f64_at(8),
            row.f64_at(9),
            row.f64_at(10),
            row.f64_at(11),
        ) {
            let vuln_score = ((26_000.0 - re78).max(0.0)) / 1_000.0
                + 4.0 * nodeg
                + if re75 < 1_500.0 { 6.0 } else { 0.0 }
                + if re74 < 1_500.0 { 4.0 } else { 0.0 };
            by_cohort.entry((age, educ)).or_default().push(vuln_score);
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

/// Reads Atlantic storm observations (`data/atlantic_storms.csv`), returning barometric hurricane-intensity
/// forecasts (`wind >= 64` knots) and binary outcomes (`0.0` or `1.0`).
pub fn load_atlantic_storms_forecasts(root_dir: &Path) -> (Vec<f64>, Vec<f64>) {
    let table = CsvTable::open(root_dir, "data/atlantic_storms.csv")
        .expect("failed to read data/atlantic_storms.csv");
    let mut probs = Vec::new();
    let mut outcomes = Vec::new();
    for row in table.rows() {
        if let (Some(wind), Some(pressure)) = (row.f64_at(10), row.f64_at(11)) {
            let z = (1005.0 - pressure) * 0.12;
            let p = cromwell_clamp(1.0 / (1.0 + (-z).exp()), 0.001);
            let y = if wind >= 64.0 { 1.0 } else { 0.0 };
            probs.push(p);
            outcomes.push(y);
        }
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
    let table = CsvTable::open(root_dir, "data/toronto_arrests.csv")
        .expect("failed to read data/toronto_arrests.csv");
    let mut a_pos = 0usize;
    let mut a_tot = 0usize;
    let mut b_pos = 0usize;
    let mut b_tot = 0usize;
    for row in table.rows() {
        if row.len() >= 8 {
            let not_released = row.str_at(1) == Some("No");
            let group = row.str_at(2).unwrap_or("");
            let employed = row.str_at(6) == Some("Yes");
            if group == "Black" && !employed {
                a_tot += 1;
                if not_released {
                    a_pos += 1;
                }
            } else if group == "White" && employed {
                b_tot += 1;
                if not_released {
                    b_pos += 1;
                }
            }
        }
    }
    let p_a = (a_pos as f64) / (a_tot.max(1) as f64);
    let p_b = (b_pos as f64) / (b_tot.max(1) as f64);
    (p_a, p_b)
}

/// Reads arrival delays (minutes, positive = late) from `data/nyc_flights_2013.csv` and `data/nyc_flights_2023.csv`.
pub fn load_nyc_flight_delays(root_dir: &Path, max_rows: usize) -> (Vec<f64>, Vec<f64>) {
    let read_delays = |rel: &str| -> Vec<f64> {
        let table = CsvTable::open(root_dir, rel).expect("failed to read flight CSV");
        let all: Vec<f64> = table
            .rows()
            .filter_map(|r| r.f64_at(8))
            .map(|d| d.max(0.0))
            .collect();
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
