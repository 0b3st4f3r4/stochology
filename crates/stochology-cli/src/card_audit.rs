//! Strategy Pattern (`CardDatasetStrategy`) and empirical card ground-truth auditor
//! (`compute_empirical_dataset_audit`, `verify_card_against_empirical_data`, `evaluate_empirical_card_severe_test`).

use crate::csv::CsvTable;
use crate::error::CliError;
use crate::loaders::load_sha256_manifest_at;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use stochology_core::card::{DatasetColumns, JensenLoss, ProtocolProfile};
use stochology_core::{
    EpistemologicalCard, ExceedanceDesignEffect, NonlinearSummary, Rng, apply_cromwell_floor, mean,
    sha256_hex,
};
use stochology_lenses::CorpPavDecomposition;
use stochology_siga::{
    SigaExecutionReport, run_full_siga_roadmap_empirical_with_profile,
    simulate_sbc_beta_binomial_ranks,
};

/// Empirical ground-truth metrics computed directly from a card's declared `data/*.csv` file.
#[derive(Debug, Clone)]
pub struct EmpiricalDatasetAudit {
    pub dataset_path: String,
    pub dataset_sha256: String,
    pub n_nominal: usize,
    pub num_clusters: usize,
    pub m_bar: f64,
    pub m_tilde: f64,
    pub rho_exceedance_90: f64,
    pub deff_exceedance_90: f64,
    pub n_eff_exceedance_90: usize,
    pub jensen_summary: NonlinearSummary,
    pub event_probability_p: f64,
    pub corp_pav: CorpPavDecomposition,
    pub confusion_tpr_fpr_at_p_star: (f64, f64),
    pub sbc_ranks: Vec<f64>,
    pub observed_test_statistic: f64,
    pub null_distribution_samples: Vec<f64>,
    pub ville_e_values: Vec<f64>,
}

/// Extracted raw cluster series and forecasts produced by a [`CardDatasetStrategy`].
pub struct ExtractedCardDataset {
    pub clusters_raw: Vec<Vec<f64>>,
    pub jensen: NonlinearSummary,
    pub probs: Vec<f64>,
    pub outcomes: Vec<f64>,
    pub cluster_probs: Vec<Vec<f64>>,
    pub cluster_outcomes: Vec<Vec<f64>>,
}

/// Strategy Pattern: encapsulates dataset-specific extraction of clusters, Jensen gap, and forecasts
/// for a canonical Epistemological Card.
pub trait CardDatasetStrategy: Sync {
    /// Relative path in `data/*.csv` handled by this strategy.
    fn dataset_path(&self) -> &'static str;

    /// Extracts clustered observations, out-of-sample Jensen summary, and cluster-level forecasts/outcomes.
    fn extract(&self, root_dir: &Path, csv_content: &str) -> ExtractedCardDataset;
}

struct AtlanticStormsCardStrategy;
impl CardDatasetStrategy for AtlanticStormsCardStrategy {
    fn dataset_path(&self) -> &'static str {
        "data/atlantic_storms.csv"
    }

    fn extract(&self, _root_dir: &Path, content: &str) -> ExtractedCardDataset {
        let table = CsvTable::from_string(content.to_string());
        let mut by_storm: BTreeMap<String, Vec<(f64, f64)>> = BTreeMap::new();
        let mut raw_pairs = Vec::new();
        for row in table.rows() {
            if let (Some(name), Some(year), Some(w), Some(p)) =
                (row.str_at(1), row.str_at(2), row.f64_at(10), row.f64_at(11))
            {
                let key = format!("{name}_{year}");
                by_storm.entry(key).or_default().push((w, p));
                raw_pairs.push((w, p));
            }
        }
        let clusters_raw: Vec<Vec<f64>> = by_storm
            .values()
            .map(|v| v.iter().map(|&(w, _)| w).collect())
            .collect();
        let storm_peaks: Vec<f64> = clusters_raw
            .iter()
            .map(|c| c.iter().copied().fold(0.0_f64, f64::max))
            .collect();
        let sp = storm_peaks.len() * 4 / 5;
        let jensen = NonlinearSummary::evaluate_out_of_sample(
            &storm_peaks[..sp],
            &storm_peaks[sp..],
            |x| x * x,
            2.0,
        );
        let mut probs = Vec::with_capacity(raw_pairs.len());
        let mut outcomes = Vec::with_capacity(raw_pairs.len());
        for &(w, p) in &raw_pairs {
            let z = (968.0 - p) * 0.11;
            probs.push(apply_cromwell_floor(1.0 / (1.0 + (-z).exp()), 0.001));
            outcomes.push(if w > 85.0 { 1.0 } else { 0.0 });
        }
        let mut cluster_probs = Vec::with_capacity(by_storm.len());
        let mut cluster_outcomes = Vec::with_capacity(by_storm.len());
        for v in by_storm.values() {
            cluster_probs.push(
                v.iter()
                    .map(|&(_, p)| {
                        let z = (968.0 - p) * 0.11;
                        apply_cromwell_floor(1.0 / (1.0 + (-z).exp()), 0.001)
                    })
                    .collect(),
            );
            cluster_outcomes.push(
                v.iter()
                    .map(|&(w, _)| if w > 85.0 { 1.0 } else { 0.0 })
                    .collect(),
            );
        }
        ExtractedCardDataset {
            clusters_raw,
            jensen,
            probs,
            outcomes,
            cluster_probs,
            cluster_outcomes,
        }
    }
}

struct BankCallsCardStrategy;
impl CardDatasetStrategy for BankCallsCardStrategy {
    fn dataset_path(&self) -> &'static str {
        "data/bank_calls.csv"
    }

    fn extract(&self, _root_dir: &Path, content: &str) -> ExtractedCardDataset {
        let table = CsvTable::from_string(content.to_string());
        let mut by_day: BTreeMap<String, Vec<f64>> = BTreeMap::new();
        for row in table.rows() {
            if let (Some(ts), Some(calls)) = (row.str_at(1), row.f64_at(2)) {
                let date = ts.split('T').next().unwrap_or("").to_string();
                by_day.entry(date).or_default().push(calls);
            }
        }
        let clusters_raw: Vec<Vec<f64>> = by_day.into_values().collect();
        let daily_totals: Vec<f64> = clusters_raw.iter().map(|c| c.iter().sum::<f64>()).collect();
        let sp = daily_totals.len() * 4 / 5;
        let jensen = NonlinearSummary::evaluate_out_of_sample(
            &daily_totals[..sp],
            &daily_totals[sp..],
            |x| x * x,
            2.0,
        );
        let mut probs = Vec::new();
        let mut outcomes = Vec::new();
        let mut cluster_probs = Vec::with_capacity(clusters_raw.len());
        let mut cluster_outcomes = Vec::with_capacity(clusters_raw.len());
        for d in &clusters_raw {
            let mut prev = d[0];
            let mut cp = Vec::with_capacity(d.len());
            let mut cy = Vec::with_capacity(d.len());
            for &x in d {
                let z = -0.35 + (prev - 286.0) * 0.025;
                let p = apply_cromwell_floor(1.0 / (1.0 + (-z).exp()), 0.001);
                let y = if x > 286.0 { 1.0 } else { 0.0 };
                probs.push(p);
                outcomes.push(y);
                cp.push(p);
                cy.push(y);
                prev = x;
            }
            cluster_probs.push(cp);
            cluster_outcomes.push(cy);
        }
        ExtractedCardDataset {
            clusters_raw,
            jensen,
            probs,
            outcomes,
            cluster_probs,
            cluster_outcomes,
        }
    }
}

struct NycFlightsCardStrategy;
impl CardDatasetStrategy for NycFlightsCardStrategy {
    fn dataset_path(&self) -> &'static str {
        "data/nyc_flights_2013.csv"
    }

    fn extract(&self, _root_dir: &Path, content: &str) -> ExtractedCardDataset {
        let table = CsvTable::from_string(content.to_string());
        let mut by_day: BTreeMap<(u32, u32), Vec<(f64, f64)>> = BTreeMap::new();
        for row in table.rows() {
            if let (Some(m), Some(d), Some(sched_dep), Some(delay)) =
                (row.u32_at(0), row.u32_at(1), row.f64_at(2), row.f64_at(8))
            {
                by_day.entry((m, d)).or_default().push((sched_dep, delay));
            }
        }
        let clusters_raw: Vec<Vec<f64>> = by_day
            .values()
            .map(|v| v.iter().map(|&(_, delay)| delay).collect())
            .collect();
        let day_pos_delays: Vec<f64> = clusters_raw
            .iter()
            .map(|c| c.iter().map(|&d| d.max(0.0)).sum::<f64>() / (c.len().max(1) as f64))
            .collect();
        let sp = day_pos_delays.len() * 4 / 5;
        let jensen = NonlinearSummary::evaluate_out_of_sample(
            &day_pos_delays[..sp],
            &day_pos_delays[sp..],
            |x| x * x,
            2.0,
        );
        let mut probs = Vec::new();
        let mut outcomes = Vec::new();
        let mut cluster_probs = Vec::with_capacity(by_day.len());
        let mut cluster_outcomes = Vec::with_capacity(by_day.len());
        for day_rows in by_day.values() {
            let mut ema_delay = 0.0;
            let mut cp = Vec::with_capacity(day_rows.len());
            let mut cy = Vec::with_capacity(day_rows.len());
            for &(sched_dep, delay) in day_rows {
                let hr = sched_dep / 100.0;
                let z = -3.1 + 0.08 * (hr - 12.0) + 0.035 * ema_delay;
                let p = apply_cromwell_floor(1.0 / (1.0 + (-z).exp()), 0.001);
                let y = if delay > 41.0 { 1.0 } else { 0.0 };
                probs.push(p);
                outcomes.push(y);
                cp.push(p);
                cy.push(y);
                ema_delay = 0.85 * ema_delay + 0.15 * delay;
            }
            cluster_probs.push(cp);
            cluster_outcomes.push(cy);
        }
        ExtractedCardDataset {
            clusters_raw,
            jensen,
            probs,
            outcomes,
            cluster_probs,
            cluster_outcomes,
        }
    }
}

struct CpsObservationalCardStrategy;
impl CardDatasetStrategy for CpsObservationalCardStrategy {
    fn dataset_path(&self) -> &'static str {
        "data/cps_observational.csv"
    }

    fn extract(&self, _root_dir: &Path, content: &str) -> ExtractedCardDataset {
        let table = CsvTable::from_string(content.to_string());
        type CpsCohortRecord = (f64, f64, f64, f64);
        let mut by_cohort: BTreeMap<(u32, u32), Vec<CpsCohortRecord>> = BTreeMap::new();
        for row in table.rows() {
            if let (Some(age), Some(educ), Some(nodeg), Some(re74), Some(re75), Some(re78)) = (
                row.u32_at(3),
                row.u32_at(4),
                row.f64_at(8),
                row.f64_at(9),
                row.f64_at(10),
                row.f64_at(11),
            ) {
                by_cohort
                    .entry((age, educ))
                    .or_default()
                    .push((nodeg, re74, re75, re78));
            }
        }
        let clusters_raw: Vec<Vec<f64>> = by_cohort
            .values()
            .map(|v| {
                v.iter()
                    .map(|&(nodeg, re74, re75, re78)| {
                        ((26_000.0 - re78).max(0.0)) / 1_000.0
                            + 4.0 * nodeg
                            + if re75 < 1_500.0 { 6.0 } else { 0.0 }
                            + if re74 < 1_500.0 { 4.0 } else { 0.0 }
                    })
                    .collect()
            })
            .collect();
        let cohort_totals: Vec<f64> = clusters_raw.iter().map(|c| c.iter().sum::<f64>()).collect();
        let sp = cohort_totals.len() * 4 / 5;
        let jensen = NonlinearSummary::evaluate_out_of_sample(
            &cohort_totals[..sp],
            &cohort_totals[sp..],
            |x| x * x,
            2.0,
        );
        let mut probs = Vec::new();
        let mut outcomes = Vec::new();
        let mut cluster_probs = Vec::with_capacity(by_cohort.len());
        let mut cluster_outcomes = Vec::with_capacity(by_cohort.len());
        for rows in by_cohort.values() {
            let mut cp = Vec::with_capacity(rows.len());
            let mut cy = Vec::with_capacity(rows.len());
            for &(nodeg, re74, re75, re78) in rows {
                let z = -0.35 - 0.00018 * re75 - 0.00007 * re74 + 0.45 * nodeg;
                let p = apply_cromwell_floor(1.0 / (1.0 + (-z).exp()), 0.001);
                let y = if re78 < 1_500.0 { 1.0 } else { 0.0 };
                probs.push(p);
                outcomes.push(y);
                cp.push(p);
                cy.push(y);
            }
            cluster_probs.push(cp);
            cluster_outcomes.push(cy);
        }
        ExtractedCardDataset {
            clusters_raw,
            jensen,
            probs,
            outcomes,
            cluster_probs,
            cluster_outcomes,
        }
    }
}

/// Dataset strategy for an application's own CSV, driven by the `dataset_columns` of its
/// [`ProtocolProfile`] instead of a hand-written extractor: rows are grouped into independent clusters
/// by the `cluster` column, forecasts come from `forecast` (floored by Cromwell's `epsilon`), binary
/// outcomes from `outcome`, and the Jensen summary applies the profile's loss to `value`, fitted on the
/// first 80% of rows and scored out-of-sample on the last 20%.
pub struct ColumnMappedCsvStrategy<'a> {
    pub columns: &'a DatasetColumns,
    pub cromwell_epsilon: f64,
}

impl ColumnMappedCsvStrategy<'_> {
    /// Extracts clusters, forecasts, outcomes and the Jensen summary, or explains which column is unusable.
    pub fn extract(&self, csv_content: &str) -> Result<ExtractedCardDataset, String> {
        let header = csv_content.lines().next().unwrap_or_default();
        let names: Vec<&str> = header.split(',').map(str::trim).collect();
        let index_of = |col: &str| -> Result<usize, String> {
            names
                .iter()
                .position(|n| *n == col)
                .ok_or_else(|| format!("dataset_columns names '{col}', absent from the CSV header"))
        };
        let (ci, fi, oi, vi) = (
            index_of(&self.columns.cluster)?,
            index_of(&self.columns.forecast)?,
            index_of(&self.columns.outcome)?,
            index_of(&self.columns.value)?,
        );
        let table = CsvTable::from_string(csv_content.to_string());
        let mut by_cluster: BTreeMap<String, Vec<(f64, f64, f64)>> = BTreeMap::new();
        let mut values = Vec::new();
        for (line, row) in table.rows().enumerate() {
            let (Some(c), Some(p), Some(y), Some(x)) = (
                row.str_at(ci),
                row.f64_at(fi),
                row.f64_at(oi),
                row.f64_at(vi),
            ) else {
                return Err(format!(
                    "CSV data row {} is incomplete or non-numeric",
                    line + 1
                ));
            };
            if !(0.0..=1.0).contains(&p) || (y != 0.0 && y != 1.0) {
                return Err(format!(
                    "CSV data row {}: forecast must lie in [0, 1] and outcome must be 0 or 1",
                    line + 1
                ));
            }
            let p = apply_cromwell_floor(p, self.cromwell_epsilon);
            by_cluster.entry(c.to_string()).or_default().push((p, y, x));
            values.push(x);
        }
        if by_cluster.len() < 2 || values.len() < 10 {
            return Err("column-mapped dataset needs at least 2 clusters and 10 rows".into());
        }
        let split = values.len() * 4 / 5;
        let loss = self.columns.jensen_loss;
        let f_second = match loss {
            JensenLoss::Square => 2.0,
            JensenLoss::Exp => mean(&values[..split]).exp(),
            JensenLoss::Hinge => 0.0,
        };
        let jensen = NonlinearSummary::evaluate_out_of_sample(
            &values[..split],
            &values[split..],
            |x| loss.apply(x),
            f_second,
        );
        let clusters_raw = by_cluster
            .values()
            .map(|v| v.iter().map(|r| r.2).collect())
            .collect();
        let cluster_probs: Vec<Vec<f64>> = by_cluster
            .values()
            .map(|v| v.iter().map(|r| r.0).collect())
            .collect();
        let cluster_outcomes: Vec<Vec<f64>> = by_cluster
            .values()
            .map(|v| v.iter().map(|r| r.1).collect())
            .collect();
        Ok(ExtractedCardDataset {
            clusters_raw,
            jensen,
            probs: cluster_probs.concat(),
            outcomes: cluster_outcomes.concat(),
            cluster_probs,
            cluster_outcomes,
        })
    }
}

const CARD_DATASET_STRATEGIES: [&dyn CardDatasetStrategy; 4] = [
    &AtlanticStormsCardStrategy,
    &BankCallsCardStrategy,
    &NycFlightsCardStrategy,
    &CpsObservationalCardStrategy,
];

/// Computes the 30-block Ville betting martingale factors $L_1, \dots, L_{30}$ directly from
/// empirical forecasts `probs` and binary outcomes `outcomes` under $H_0: \mathbb{E}[Y_i \mid \hat{p}_i] = \bar{Y}$.
pub fn compute_ville_betting_e_values(
    probs: &[f64],
    outcomes: &[f64],
    num_blocks: usize,
) -> Vec<f64> {
    assert!(!outcomes.is_empty() && probs.len() == outcomes.len() && num_blocks >= 1);
    let n = outcomes.len();
    let block_size = (n / num_blocks).max(1);
    let mut factors = Vec::with_capacity(num_blocks);
    let mut past_y_sum = 0.0_f64;
    let mut past_y_count = 0.0_f64;

    for b in 0..num_blocks {
        let start = b * block_size;
        let end = if b + 1 == num_blocks {
            n
        } else {
            ((b + 1) * block_size).min(n)
        };
        if start >= end {
            break;
        }
        let m_b = (end - start) as f64;
        let p_bar_b = probs[start..end].iter().sum::<f64>() / m_b;
        let p0_pred = if b == 0 {
            p_bar_b.clamp(1e-4, 1.0 - 1e-4)
        } else {
            ((past_y_sum + 1.0) / (past_y_count + 2.0)).clamp(1e-4, 1.0 - 1e-4)
        };
        let var0_pred = (p0_pred * (1.0 - p0_pred)).max(1e-4);

        let b_minus = (probs[start..end]
            .iter()
            .filter(|&&p| p < p_bar_b)
            .map(|&p| p_bar_b - p)
            .sum::<f64>()
            / m_b)
            .max(1e-9);
        let lambda_b = (1.0 / var0_pred).min(0.95 / b_minus);

        let mut z_sum = 0.0;
        for i in start..end {
            let d_i = probs[i] - p_bar_b;
            z_sum += d_i * (outcomes[i] - p0_pred);
            past_y_sum += outcomes[i];
            past_y_count += 1.0;
        }
        let lb = 1.0 + lambda_b * (z_sum / m_b);
        factors.push(lb);
    }
    factors
}

/// Computes the un-clamped cluster-permutation test statistic $T_{\text{obs}}$ and 200 null
/// replications $T^{(1)}, \dots, T^{(200)}$ (`seed = 2026`) by permuting cluster outcome rates
/// across the independent clusters.
pub fn compute_unclamped_cluster_permutation_null(
    cluster_probs: &[Vec<f64>],
    cluster_outcomes: &[Vec<f64>],
    seed: u64,
    num_replications: usize,
) -> (f64, Vec<f64>) {
    assert!(!cluster_probs.is_empty() && cluster_probs.len() == cluster_outcomes.len());
    let c_len = cluster_probs.len();
    let mut total_n = 0usize;
    let mut sum_p = 0.0;
    let mut sum_y = 0.0;
    let mut cluster_mean_p = Vec::with_capacity(c_len);
    let mut cluster_mean_y = Vec::with_capacity(c_len);
    let mut cluster_weights = Vec::with_capacity(c_len);

    for (cp, cy) in cluster_probs.iter().zip(cluster_outcomes.iter()) {
        let m = cp.len().max(1);
        total_n += m;
        let sp: f64 = cp.iter().sum();
        let sy: f64 = cy.iter().sum();
        sum_p += sp;
        sum_y += sy;
        cluster_mean_p.push(sp / (m as f64));
        cluster_mean_y.push(sy / (m as f64));
        cluster_weights.push(m as f64);
    }

    let n_f = total_n.max(1) as f64;
    let p_bar = sum_p / n_f;
    let y_bar = sum_y / n_f;

    let mut obs_cov = 0.0;
    for (cp, cy) in cluster_probs.iter().zip(cluster_outcomes.iter()) {
        for (&p, &y) in cp.iter().zip(cy.iter()) {
            obs_cov += (p - p_bar) * (y - y_bar);
        }
    }
    let t_obs = obs_cov / n_f;

    let mut rng = Rng::new(seed);
    let mut indices: Vec<usize> = (0..c_len).collect();
    let mut null_samples = Vec::with_capacity(num_replications);

    for _ in 0..num_replications {
        for i in (1..c_len).rev() {
            let j = (rng.next_u64() as usize) % (i + 1);
            indices.swap(i, j);
        }
        let mut null_cov = 0.0;
        for (c_idx, &perm_idx) in indices.iter().enumerate() {
            let w = cluster_weights[c_idx];
            let dp = cluster_mean_p[c_idx] - p_bar;
            let dy = cluster_mean_y[perm_idx] - y_bar;
            null_cov += w * dp * dy;
        }
        null_samples.push(null_cov / n_f);
    }

    (t_obs, null_samples)
}

fn compute_confusion_tpr_fpr(probs: &[f64], outcomes: &[f64], p_star: f64) -> (f64, f64) {
    let mut tp = 0usize;
    let mut fn_c = 0usize;
    let mut fp = 0usize;
    let mut tn = 0usize;
    for (&p, &y) in probs.iter().zip(outcomes.iter()) {
        let alarm = p >= p_star;
        if y >= 0.5 {
            if alarm {
                tp += 1;
            } else {
                fn_c += 1;
            }
        } else if alarm {
            fp += 1;
        } else {
            tn += 1;
        }
    }
    let tpr = (tp as f64) / ((tp + fn_c).max(1) as f64);
    let fpr = (fp as f64) / ((fp + tn).max(1) as f64);
    (tpr, fpr)
}

/// Computes the complete [`EmpiricalDatasetAudit`] directly from the CSV declared in `card.data_and_audit_contract.dataset_path`.
pub fn compute_empirical_dataset_audit(
    root_dir: &Path,
    card: &EpistemologicalCard,
) -> Result<EmpiricalDatasetAudit, String> {
    compute_empirical_dataset_audit_with_profile(root_dir, card, &ProtocolProfile::canonical())
}

/// As [`compute_empirical_dataset_audit`] under an application's [`ProtocolProfile`]: the dataset is
/// checked against the profile's `dataset_manifest`, read through [`ColumnMappedCsvStrategy`] when it
/// is not one of the canonical four, and the SBC, permutation null and Ville e-values use the
/// profile's seed and replication counts.
pub fn compute_empirical_dataset_audit_with_profile(
    root_dir: &Path,
    card: &EpistemologicalCard,
    profile: &ProtocolProfile,
) -> Result<EmpiricalDatasetAudit, String> {
    let rel_csv = card.data_and_audit_contract.dataset_path.trim();
    let full_csv = root_dir.join(rel_csv);
    let raw_bytes = fs::read(&full_csv).map_err(|e| {
        String::from(CliError::Io {
            path: full_csv.display().to_string(),
            message: e.to_string(),
        })
    })?;
    let actual_sha256 = sha256_hex(&raw_bytes);

    let manifest = load_sha256_manifest_at(root_dir, &profile.dataset_manifest)?;
    let Some(manifest_sha) = manifest.get(rel_csv) else {
        return Err(String::from(CliError::ManifestMismatch {
            dataset_path: rel_csv.to_string(),
            actual_sha256,
            expected_sha256: None,
        }));
    };
    if &actual_sha256 != manifest_sha {
        return Err(String::from(CliError::ManifestMismatch {
            dataset_path: rel_csv.to_string(),
            actual_sha256,
            expected_sha256: Some(manifest_sha.clone()),
        }));
    }

    let content = String::from_utf8(raw_bytes).map_err(|e| {
        String::from(CliError::Utf8 {
            path: rel_csv.to_string(),
            message: e.to_string(),
        })
    })?;

    let p_star = card.decision_rule_and_scope.critical_threshold_p_star;

    let extracted = match CARD_DATASET_STRATEGIES
        .iter()
        .find(|s| s.dataset_path() == rel_csv)
    {
        Some(strategy) => strategy.extract(root_dir, &content),
        None => {
            let columns = profile
                .dataset_columns
                .as_ref()
                .ok_or_else(|| String::from(CliError::UnsupportedDataset(rel_csv.to_string())))?;
            ColumnMappedCsvStrategy {
                columns,
                cromwell_epsilon: card.decision_rule_and_scope.base_rate_floor_epsilon,
            }
            .extract(&content)
            .map_err(|detail| {
                String::from(CliError::EmpiricalMismatch {
                    card_id: card.id.clone(),
                    detail,
                })
            })?
        }
    };
    let ExtractedCardDataset {
        clusters_raw,
        jensen,
        probs,
        outcomes,
        cluster_probs,
        cluster_outcomes,
    } = extracted;
    let st = &profile.severe_test;

    let deff = ExceedanceDesignEffect::from_clustered_observations(&clusters_raw, 0.90);
    let n_eff_rounded = deff.n_effective_p.round() as usize;
    let event_prob_p = mean(&outcomes);
    let corp_pav = CorpPavDecomposition::compute(&probs, &outcomes);
    let confusion = compute_confusion_tpr_fpr(&probs, &outcomes, p_star);

    let alpha_prior = (20.0 * p_star.clamp(0.01, 0.99)).max(0.5);
    let beta_prior = (20.0 * (1.0 - p_star.clamp(0.01, 0.99))).max(0.5);
    let sbc_ranks = simulate_sbc_beta_binomial_ranks(
        profile.seed,
        st.sbc_replications,
        st.sbc_max_rank,
        alpha_prior,
        beta_prior,
        n_eff_rounded.clamp(20, 120),
    );

    let (t_obs, null_samples) = compute_unclamped_cluster_permutation_null(
        &cluster_probs,
        &cluster_outcomes,
        profile.seed,
        st.permutation_replications,
    );
    let ville_e_values = compute_ville_betting_e_values(&probs, &outcomes, st.ville_blocks);

    Ok(EmpiricalDatasetAudit {
        dataset_path: rel_csv.to_string(),
        dataset_sha256: actual_sha256,
        n_nominal: deff.n_nominal,
        num_clusters: deff.num_clusters_c,
        m_bar: deff.m_bar,
        m_tilde: deff.m_tilde_moulton,
        rho_exceedance_90: deff.rho_exceedance_p,
        deff_exceedance_90: deff.deff_exceedance,
        n_eff_exceedance_90: n_eff_rounded,
        jensen_summary: jensen,
        event_probability_p: event_prob_p,
        corp_pav,
        confusion_tpr_fpr_at_p_star: confusion,
        sbc_ranks,
        observed_test_statistic: t_obs,
        null_distribution_samples: null_samples,
        ville_e_values,
    })
}

/// Audits an [`EpistemologicalCard`] against both structural invariants (`card.audit()`) and the
/// empirical ground truth recomputed directly from its declared `data/*.csv` file, then runs the
/// un-clamped 8-Step SIGA Roadmap.
pub fn verify_card_against_empirical_data(
    root_dir: &Path,
    card: &EpistemologicalCard,
) -> Result<(EmpiricalDatasetAudit, SigaExecutionReport), String> {
    verify_card_against_empirical_data_with_profile(root_dir, card, &ProtocolProfile::canonical())
}

/// As [`verify_card_against_empirical_data`] under an application's [`ProtocolProfile`]
/// ([`EpistemologicalCard::audit_with`], the profile-sealed pre-registration payload, and
/// [`run_full_siga_roadmap_empirical_with_profile`]).
pub fn verify_card_against_empirical_data_with_profile(
    root_dir: &Path,
    card: &EpistemologicalCard,
    profile: &ProtocolProfile,
) -> Result<(EmpiricalDatasetAudit, SigaExecutionReport), String> {
    if let Err(violations) = card.audit_with(profile) {
        return Err(String::from(CliError::ContractAudit {
            card_id: card.id.clone(),
            violations,
        }));
    }
    let emp = compute_empirical_dataset_audit_with_profile(root_dir, card, profile)?;

    if card.data_and_audit_contract.dataset_sha256.trim() != emp.dataset_sha256 {
        return Err(String::from(CliError::EmpiricalMismatch {
            card_id: card.id.clone(),
            detail: format!(
                "declared dataset_sha256 '{}' != actual CSV SHA-256 '{}'",
                card.data_and_audit_contract.dataset_sha256, emp.dataset_sha256
            ),
        }));
    }

    let um = &card.epistemological_header.unit_and_margin;
    if um.nominal_sample_size_n != emp.n_nominal {
        return Err(String::from(CliError::EmpiricalMismatch {
            card_id: card.id.clone(),
            detail: format!(
                "declared nominal_sample_size_n={} != CSV n={}",
                um.nominal_sample_size_n, emp.n_nominal
            ),
        }));
    }
    if (um.effective_sample_size_n_eff as isize - emp.n_eff_exceedance_90 as isize).abs() > 1 {
        return Err(String::from(CliError::EmpiricalMismatch {
            card_id: card.id.clone(),
            detail: format!(
                "declared effective_sample_size_n_eff={} != CSV n_eff={}",
                um.effective_sample_size_n_eff, emp.n_eff_exceedance_90
            ),
        }));
    }
    let Some(moulton) = um.parsed_moulton_metrics() else {
        return Err(String::from(CliError::EmpiricalMismatch {
            card_id: card.id.clone(),
            detail: "failed to parse Moulton-Campbell cluster metrics from unit_and_margin".into(),
        }));
    };
    if (moulton.weighted_cluster_mean_m_tilde - emp.m_tilde).abs() > 0.02
        || (moulton.threshold_exceedance_icc_rho_i - emp.rho_exceedance_90).abs() > 5e-4
    {
        return Err(String::from(CliError::EmpiricalMismatch {
            card_id: card.id.clone(),
            detail: format!(
                "declared Moulton metrics (m_tilde={:.2}, rho_I={:.4}) diverge from CSV (m_tilde={:.2}, rho_I={:.4})",
                moulton.weighted_cluster_mean_m_tilde,
                moulton.threshold_exceedance_icc_rho_i,
                emp.m_tilde,
                emp.rho_exceedance_90
            ),
        }));
    }

    let dr = &card.decision_rule_and_scope;
    if (dr.event_probability_p - emp.event_probability_p).abs() > 5e-4 {
        return Err(String::from(CliError::EmpiricalMismatch {
            card_id: card.id.clone(),
            detail: format!(
                "declared event_probability_p={:.5} != CSV empirical p={:.5}",
                dr.event_probability_p, emp.event_probability_p
            ),
        }));
    }
    let dom = &card.uncertainty_budget_and_authorship.dominant_event;
    if (dom.probability_p - emp.event_probability_p).abs() > 5e-4 {
        return Err(String::from(CliError::EmpiricalMismatch {
            card_id: card.id.clone(),
            detail: format!(
                "declared dominant_event.probability_p={:.5} != CSV empirical p={:.5}",
                dom.probability_p, emp.event_probability_p
            ),
        }));
    }

    let mc = &card.data_and_audit_contract.murphy_corp_decomposition;
    if (mc.total_score_brier_or_crps - emp.corp_pav.brier_total).abs() > 5e-4
        || (mc.miscalibration_rel_mcb - emp.corp_pav.rel_miscalibration).abs() > 5e-4
        || (mc.resolution_res_dsc - emp.corp_pav.res_resolution).abs() > 5e-4
        || (mc.base_uncertainty_unc0 - emp.corp_pav.unc0_uncertainty).abs() > 5e-4
    {
        return Err(String::from(CliError::EmpiricalMismatch {
            card_id: card.id.clone(),
            detail: format!(
                "declared CORP-PAV ({:.5}, {:.5}, {:.5}, {:.5}) diverges from CSV CORP-PAV ({:.5}, {:.5}, {:.5}, {:.5})",
                mc.total_score_brier_or_crps,
                mc.miscalibration_rel_mcb,
                mc.resolution_res_dsc,
                mc.base_uncertainty_unc0,
                emp.corp_pav.brier_total,
                emp.corp_pav.rel_miscalibration,
                emp.corp_pav.res_resolution,
                emp.corp_pav.unc0_uncertainty
            ),
        }));
    }

    let Some(nl) = card.epistemological_header.parsed_nonlinearity_metrics() else {
        return Err(String::from(CliError::EmpiricalMismatch {
            card_id: card.id.clone(),
            detail: "failed to parse Prohibition 2 nonlinearity metrics".into(),
        }));
    };
    let js = &emp.jensen_summary;
    if (nl.f_at_mean - js.f_at_mean).abs() > 0.05
        || (nl.expected_f - js.expected_f).abs() > 0.05
        || (nl.jensen_gap_delta_j - js.jensen_gap).abs() > 0.05
        || (nl.median - js.median_f).abs() > 0.05
        || (nl.q05 - js.q05_f).abs() > 0.05
        || (nl.q95 - js.q95_f).abs() > 0.05
        || (nl.crps - js.empirical_crps).abs() > 0.05
    {
        return Err(String::from(CliError::EmpiricalMismatch {
            card_id: card.id.clone(),
            detail: format!(
                "declared nonlinearity '{}' does not match CSV Jensen summary (f(E[X])={:.2}, E[f(X)]={:.2}, Delta_J={:.2}, median={:.2}, q05={:.2}, q95={:.2}, CRPS={:.2})",
                card.epistemological_header.nonlinearity,
                js.f_at_mean,
                js.expected_f,
                js.jensen_gap,
                js.median_f,
                js.q05_f,
                js.q95_f,
                js.empirical_crps
            ),
        }));
    }

    let plan_payload = profile.preregistration_payload_for(card);
    let siga_rep = run_full_siga_roadmap_empirical_with_profile(
        card,
        profile,
        &plan_payload,
        &emp.sbc_ranks,
        emp.observed_test_statistic,
        &emp.null_distribution_samples,
        &emp.ville_e_values,
        emp.confusion_tpr_fpr_at_p_star,
    );
    if !siga_rep.approved_end_to_end {
        return Err(String::from(CliError::EmpiricalMismatch {
            card_id: card.id.clone(),
            detail: format!(
                "failed 8-Step SIGA Roadmap on empirical CSV: steps={:?}, p_null={:.4}, E_t={:.2}",
                siga_rep.steps_1_to_8_completed,
                siga_rep.null_p_value,
                siga_rep.e_process.final_e_value
            ),
        }));
    }

    Ok((emp, siga_rep))
}

/// Canonical severe-test entry point declared in `EpistemologicalCard::data_and_audit_contract.null_model`
/// (`crates/stochology-cli#evaluate_empirical_card_severe_test`).
///
/// Recomputes the un-clamped cluster-permutation null distribution (`seed = 2026`), verifies all
/// empirical quantities against the declared `data/*.csv` file, and executes the 8-Step SIGA Roadmap.
pub fn evaluate_empirical_card_severe_test(
    root_dir: &Path,
    card: &EpistemologicalCard,
) -> Result<(EmpiricalDatasetAudit, SigaExecutionReport), String> {
    verify_card_against_empirical_data(root_dir, card)
}

/// As [`evaluate_empirical_card_severe_test`] under an application's [`ProtocolProfile`].
pub fn evaluate_empirical_card_severe_test_with_profile(
    root_dir: &Path,
    card: &EpistemologicalCard,
    profile: &ProtocolProfile,
) -> Result<(EmpiricalDatasetAudit, SigaExecutionReport), String> {
    verify_card_against_empirical_data_with_profile(root_dir, card, profile)
}
