//! Real-time data vintages as bitemporal records (Croushore & Stark 2001): event time = observation
//! quarter, availability time = vintage. A backtest that feeds later vintages into past decisions is
//! temporal leakage, caught by the bitemporal clock (§4.1).

use std::collections::BTreeMap;

use crate::clock::{BitemporalRecord, query_with_bitemporal_clock};

/// Quarter index `4 * year + (q - 1)` parsed from `"YYYYQq"`.
pub fn parse_quarter(label: &str) -> Option<u32> {
    let (year, q) = label.split_once('Q')?;
    let (year, q): (u32, u32) = (year.parse().ok()?, q.parse().ok()?);
    (1..=4).contains(&q).then_some(4 * year + q - 1)
}

/// `"YYYYQq"` label of a quarter index.
pub fn quarter_label(index: u32) -> String {
    format!("{}Q{}", index / 4, index % 4 + 1)
}

/// Real-time backtest of a naive growth forecast on vintage data (Croushore & Stark 2001,
/// *J. Econometrics* 105(1):111–130; Croushore 2011, *J. Econ. Lit.* 49(1):72–100, informative).
/// At decision quarter $t$ the forecaster sees vintage $t$ only (data through $t - 1$) and forecasts the
/// annualized growth of quarter $t$ as the mean of the last four growth rates in that vintage. Growth is
/// computed inside a vintage, because levels change base year across vintages.
#[derive(Debug, Clone, PartialEq)]
pub struct RealTimeVintageAudit {
    pub first_decision: u32,
    pub last_decision: u32,
    pub latest_vintage: u32,
    pub decisions: usize,
    /// RMSE (annualized % points) of real-time forecasts against the first release.
    pub rmse_real_time_vs_first_release: f64,
    /// RMSE of real-time forecasts against the latest vintage.
    pub rmse_real_time_vs_latest: f64,
    /// RMSE of the leaky backtest (latest-vintage inputs) against the latest vintage.
    pub rmse_leaky_vs_latest: f64,
    /// Mean and SD of the revision (latest minus first release) of quarterly growth.
    pub mean_revision: f64,
    pub sd_revision: f64,
    /// Bitemporal clock alarms over all decision quarters.
    pub real_time_clock_alarm: bool,
    pub leaky_clock_alarm: bool,
    /// Inputs with availability after their decision quarter in the leaky backtest.
    pub leaky_inputs_after_decision: usize,
}

impl RealTimeVintageAudit {
    /// `rows` holds `(vintage, obs_quarter, level)` quarter indices and levels.
    pub fn evaluate(rows: &[(u32, u32, f64)], first_decision: u32, last_decision: u32) -> Self {
        let mut table: BTreeMap<(u32, u32), f64> = BTreeMap::new();
        for &(v, o, level) in rows {
            table.insert((v, o), level);
        }
        let latest_vintage = rows.iter().map(|r| r.0).max().expect("non-empty vintages");
        let growth = |v: u32, o: u32| -> Option<f64> {
            let (now, before) = (table.get(&(v, o))?, table.get(&(v, o - 1))?);
            Some(100.0 * ((now / before).powi(4) - 1.0))
        };

        let (mut se_first, mut se_latest, mut se_leaky) = (0.0, 0.0, 0.0);
        let mut revisions = Vec::new();
        let mut real_time_records = Vec::new();
        let mut leaky_records = Vec::new();
        let (mut real_time_alarm, mut leaky_alarm) = (false, false);
        let mut leaky_after = 0usize;
        for t in first_decision..=last_decision {
            let inputs_rt: Option<Vec<f64>> = (t - 4..t).map(|o| growth(t, o)).collect();
            let inputs_leaky: Option<Vec<f64>> =
                (t - 4..t).map(|o| growth(latest_vintage, o)).collect();
            let (Some(rt), Some(leaky), Some(first), Some(latest)) = (
                inputs_rt,
                inputs_leaky,
                growth(t + 1, t),
                growth(latest_vintage, t),
            ) else {
                continue;
            };
            let f_rt = rt.iter().sum::<f64>() / 4.0;
            let f_leaky = leaky.iter().sum::<f64>() / 4.0;
            se_first += (f_rt - first).powi(2);
            se_latest += (f_rt - latest).powi(2);
            se_leaky += (f_leaky - latest).powi(2);
            revisions.push(latest - first);

            real_time_records.clear();
            leaky_records.clear();
            for o in t - 4..t {
                real_time_records.push(Self::record(o, t));
                leaky_records.push(Self::record(o, latest_vintage));
                leaky_after += usize::from(latest_vintage > t);
            }
            real_time_alarm |= query_with_bitemporal_clock(&real_time_records, u64::from(t), false)
                .temporal_intrusion_alarm;
            leaky_alarm |= query_with_bitemporal_clock(&leaky_records, u64::from(t), false)
                .temporal_intrusion_alarm;
        }
        let n = revisions.len();
        assert!(n > 1, "no decision quarter has complete vintages");
        let mean_revision = revisions.iter().sum::<f64>() / n as f64;
        let sd_revision = (revisions
            .iter()
            .map(|r| (r - mean_revision).powi(2))
            .sum::<f64>()
            / (n - 1) as f64)
            .sqrt();
        Self {
            first_decision,
            last_decision,
            latest_vintage,
            decisions: n,
            rmse_real_time_vs_first_release: (se_first / n as f64).sqrt(),
            rmse_real_time_vs_latest: (se_latest / n as f64).sqrt(),
            rmse_leaky_vs_latest: (se_leaky / n as f64).sqrt(),
            mean_revision,
            sd_revision,
            real_time_clock_alarm: real_time_alarm,
            leaky_clock_alarm: leaky_alarm,
            leaky_inputs_after_decision: leaky_after,
        }
    }

    /// A vintage value as a bitemporal record: event = observation quarter, availability = vintage.
    fn record(obs_quarter: u32, vintage: u32) -> BitemporalRecord {
        BitemporalRecord {
            row_id: obs_quarter as usize,
            t_event: u64::from(obs_quarter),
            t_availability: u64::from(vintage),
            feature_x: 0,
            outcome_y: 0,
            canary_column: 0,
            t_availability_canary: u64::from(vintage),
        }
    }

    /// How much better the leaky backtest looks than the honest one against the same target.
    pub fn leakage_flattery(&self) -> f64 {
        self.rmse_real_time_vs_latest - self.rmse_leaky_vs_latest
    }
}
