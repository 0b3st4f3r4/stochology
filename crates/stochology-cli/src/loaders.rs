//! Empirical CSV dataset loaders (`data/*.csv`) and YAML Card / SHA-256 manifest readers.

use crate::csv::CsvTable;
use crate::error::CliError;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use stochology_core::{EpistemologicalCard, apply_cromwell_floor};
use stochology_lenses::CausalEnvironment;

/// Resolves the workspace/distribution root directory containing `data/SHA256SUMS` and `cards/*.yaml`.
///
/// Checks, in order:
/// 1. `std::env::current_dir()` if `data/SHA256SUMS` exists there;
/// 2. Ancestor directories of `std::env::current_exe()` if `data/SHA256SUMS` exists there (for prebuilt release archives);
/// 3. Compile-time `CARGO_MANIFEST_DIR/../..` fallback.
pub fn resolve_workspace_root() -> PathBuf {
    if let Ok(cwd) = std::env::current_dir() {
        if cwd.join("data/SHA256SUMS").is_file() {
            return cwd;
        }
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

/// Paths of the 4 canonical Epistemological Cards on disk (`cards/*.yaml`).
pub const CANONICAL_CARD_PATHS: [&str; 4] = [
    "cards/card_01_atlantic_storms_evacuation.yaml",
    "cards/card_02_bank_queue_staffing.yaml",
    "cards/card_03_nyc_flights_holding.yaml",
    "cards/card_04_dpi_civil_defense_triage.yaml",
];

/// Loads and parses the 4 canonical YAML cards directly from disk (`cards/*.yaml`).
pub fn load_cards_from_disk(root_dir: &Path) -> Result<Vec<(String, EpistemologicalCard)>, String> {
    let mut out = Vec::with_capacity(CANONICAL_CARD_PATHS.len());
    for rel_path in CANONICAL_CARD_PATHS {
        let full_path = root_dir.join(rel_path);
        let content = fs::read_to_string(&full_path).map_err(|e| {
            String::from(CliError::Io {
                path: full_path.display().to_string(),
                message: e.to_string(),
            })
        })?;
        let card = EpistemologicalCard::from_yaml_typed(&content).map_err(|error| {
            String::from(CliError::ParseCard {
                path: rel_path.to_string(),
                error,
            })
        })?;
        out.push((rel_path.to_string(), card));
    }
    Ok(out)
}

/// Reads `data/SHA256SUMS` and returns a map of `relative_path -> expected_sha256`.
pub fn load_sha256sums_manifest(root_dir: &Path) -> Result<BTreeMap<String, String>, String> {
    load_sha256_manifest_at(root_dir, "data/SHA256SUMS")
}

/// Reads the SHA-256 manifest at `root_dir.join(rel_manifest)` (a profile's `dataset_manifest`) and
/// returns a map of `relative_path -> expected_sha256`.
pub fn load_sha256_manifest_at(
    root_dir: &Path,
    rel_manifest: &str,
) -> Result<BTreeMap<String, String>, String> {
    let path = root_dir.join(rel_manifest);
    let content = fs::read_to_string(&path).map_err(|e| {
        String::from(CliError::Io {
            path: path.display().to_string(),
            message: e.to_string(),
        })
    })?;
    let mut map = BTreeMap::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let mut parts = trimmed.split_whitespace();
        if let (Some(hash), Some(rel)) = (parts.next(), parts.next()) {
            map.insert(rel.to_string(), hash.to_string());
        }
    }
    Ok(map)
}

/// Reads real Danish Fire Insurance losses (`data/danish_fire.csv`).
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

/// Reads real `re78` earnings from the NSW experiment (`data/nsw_experimental.csv`, treated vs. control)
/// and the observational CPS sample (`data/cps_observational.csv`).
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

/// Reads `data/cps_observational.csv` grouped by `(age, educ)` administrative demographic-educational cohort clusters,
/// returning the cluster-level vulnerability score vectors used in Card 04 and `EmpiricalCoupledSocietyAudit`.
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

/// Reads daily arithmetic returns $(S_t - S_{t-1}) / S_{t-1}$ of the Dow Jones Industrial Average (`data/djia_1885_2023.csv`)
/// so that ensemble drift $\mu_{\text{ens}}$ and Itô time-average growth $g_{\text{time}} = \mu_{\text{ens}} - \frac{1}{2}\sigma^2$
/// are evaluated without double-subtracting the variance drain.
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

/// Reads all Atlantic storm observations (`data/atlantic_storms.csv`), returning uncalibrated
/// barometric hurricane-intensity probabilities (`wind >= 64` knots) and binary outcomes (`0.0` or `1.0`)
/// used to demonstrate PAV isotonic recalibration in Lab 3.
pub fn load_atlantic_storms_forecasts(root_dir: &Path) -> (Vec<f64>, Vec<f64>) {
    let table = CsvTable::open(root_dir, "data/atlantic_storms.csv")
        .expect("failed to read data/atlantic_storms.csv");
    let mut probs = Vec::new();
    let mut outcomes = Vec::new();
    for row in table.rows() {
        if let (Some(wind), Some(pressure)) = (row.f64_at(10), row.f64_at(11)) {
            let z = (1005.0 - pressure) * 0.12;
            let p = apply_cromwell_floor(1.0 / (1.0 + (-z).exp()), 0.001);
            let y = if wind >= 64.0 { 1.0 } else { 0.0 };
            probs.push(p);
            outcomes.push(y);
        }
    }
    (probs, outcomes)
}

/// Splits `data/atlantic_storms.csv` ($n = 20\,778$) into 3 climatological seasonal regimes
/// (`storms_fringe_season`, `storms_shoulder_season_aug_oct`, `storms_peak_season_sep`)
/// to audit nonlinear physical invariance of cyclostrophic barometric pressure deficit (`X0`)
/// vs. a regime-shifted latitude proxy (`X1`) predicting normalized maximum sustained wind (`Y`).
pub fn load_atlantic_storms_decade_environments(root_dir: &Path) -> Vec<CausalEnvironment> {
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

/// Reads all half-hourly electricity demand observations (MW) of Victoria, Australia (`data/vic_elec.csv`).
pub fn load_vic_elec_demand(root_dir: &Path) -> Vec<f64> {
    let table =
        CsvTable::open(root_dir, "data/vic_elec.csv").expect("failed to read data/vic_elec.csv");
    table.rows().filter_map(|r| r.f64_at(2)).collect()
}

/// Reads Toronto arrest records (`data/toronto_arrests.csv`) and returns empirical base rates
/// $(p_A, p_B)$ of non-release (`released == "No"`) across two demographic groups with unequal base rates.
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

/// Reads standardized distance / arrival-delay features from `data/nyc_flights_2013.csv` (source $P$)
/// and `data/nyc_flights_2023.csv` (target $Q$), systematically stratified across the full annual files.
pub fn load_nyc_flights_2013_2023(root_dir: &Path) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
    let (x13, y13) = load_flight_stream(root_dir, "data/nyc_flights_2013.csv", 1_500);
    let (x23, y23) = load_flight_stream(root_dir, "data/nyc_flights_2023.csv", 1_500);
    (x13, y13, x23, y23)
}

/// Reads the pre-departure intraday delay-cascade feature (standardized scheduled departure hour plus
/// causally lagged rolling arrival delay of prior flights) and binary delay outcome ($\text{arr\_delay} > 15\text{ min}$)
/// from `data/nyc_flights_2013.csv` (source $P$) and `data/nyc_flights_2023.csv` (target $Q$).
pub fn load_nyc_flights_cascade_2013_2023(
    root_dir: &Path,
) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
    let load_cascade = |rel_path: &str, max_rows: usize| -> (Vec<f64>, Vec<f64>) {
        let table = CsvTable::open(root_dir, rel_path).expect("failed to read flight CSV");
        let mut all_x = Vec::new();
        let mut all_y = Vec::new();
        let mut ema_delay = 0.0_f64;
        let mut prev_day = (0u32, 0u32);
        for row in table.rows() {
            if let (Some(month), Some(day), Some(sched_dep), Some(arr_delay)) =
                (row.u32_at(0), row.u32_at(1), row.f64_at(2), row.f64_at(8))
            {
                if (month, day) != prev_day {
                    ema_delay = 0.0;
                    prev_day = (month, day);
                }
                let hr = (sched_dep / 100.0).floor();
                let feat = 0.35 * ((hr - 13.0) / 5.0) + 0.65 * (ema_delay / 25.0);
                all_x.push(feat.clamp(-3.0, 5.0));
                all_y.push(if arr_delay > 15.0 { 1.0 } else { 0.0 });
                ema_delay = 0.80 * ema_delay + 0.20 * arr_delay;
            }
        }
        if max_rows == 0 || all_x.len() <= max_rows {
            return (all_x, all_y);
        }
        let n = all_x.len();
        let mut x = Vec::with_capacity(max_rows);
        let mut y = Vec::with_capacity(max_rows);
        for i in 0..max_rows {
            let idx = (i * n) / max_rows;
            x.push(all_x[idx]);
            y.push(all_y[idx]);
        }
        (x, y)
    };
    let (x13, y13) = load_cascade("data/nyc_flights_2013.csv", 1_500);
    let (x23, y23) = load_cascade("data/nyc_flights_2023.csv", 1_500);
    (x13, y13, x23, y23)
}

/// Reads standardized distance $((d - 1000)/1000)$ and binary arrival-delay indicator $(\text{arr\_delay} > 15\text{ min})$
/// from any flight CSV in `data/` (`nyc_flights_2013.csv`, `nyc_flights_2023.csv`, `nyc_flights_2013_h1.csv`,
/// `nyc_flights_2023_h1.csv`, or `houston_flights_2011.csv`), scanning the entire file and taking a deterministic
/// equispaced sample of `max_rows` across the full temporal span.
pub fn load_flight_stream(
    root_dir: &Path,
    rel_path: &str,
    max_rows: usize,
) -> (Vec<f64>, Vec<f64>) {
    let table = CsvTable::open(root_dir, rel_path).expect("failed to read flight CSV");
    let mut all_x = Vec::new();
    let mut all_y = Vec::new();
    for row in table.rows() {
        if let (Some(distance), Some(arr_delay)) = (row.f64_at(7), row.f64_at(8)) {
            all_x.push((distance - 1_000.0) / 1_000.0);
            all_y.push(if arr_delay > 15.0 { 1.0 } else { 0.0 });
        }
    }
    if max_rows == 0 || all_x.len() <= max_rows {
        return (all_x, all_y);
    }
    let n = all_x.len();
    let mut x = Vec::with_capacity(max_rows);
    let mut y = Vec::with_capacity(max_rows);
    for i in 0..max_rows {
        let idx = (i * n) / max_rows;
        x.push(all_x[idx]);
        y.push(all_y[idx]);
    }
    (x, y)
}

/// Reads the 3 out-of-time / out-of-domain flight reserve datasets (`data/nyc_flights_2013_h1.csv`,
/// `data/nyc_flights_2023_h1.csv`, and `data/houston_flights_2011.csv`).
#[allow(clippy::type_complexity)]
pub fn load_oot_ood_flight_reserves(
    root_dir: &Path,
) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
    let (x13_h1, y13_h1) = load_flight_stream(root_dir, "data/nyc_flights_2013_h1.csv", 1_500);
    let (x23_h1, y23_h1) = load_flight_stream(root_dir, "data/nyc_flights_2023_h1.csv", 1_500);
    let (x_hou, y_hou) = load_flight_stream(root_dir, "data/houston_flights_2011.csv", 1_500);
    (x13_h1, y13_h1, x23_h1, y23_h1, x_hou, y_hou)
}
