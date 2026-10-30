//! Threshold-wise **CORP** decomposition of the mean **CRPS** for ensemble forecasts of a continuous
//! outcome (Arnold, Walz, Ziegel & Gneiting 2024): the CORP-Murphy identity of the Brier score,
//! integrated over every threshold of the outcome.

use stochology_core::empirical_crps;

use super::corp_murphy::CorpPavDecomposition;

/// Threshold-wise CORP decomposition of the mean Continuous Ranked Probability Score
/// (Arnold, Walz, Ziegel & Gneiting 2024, *Electron. J. Stat.* 18(2):4992–5044, informative):
/// $$\overline{\mathrm{CRPS}} = \int \mathrm{BS}(z)\,dz = \mathrm{MCB} - \mathrm{DSC} + \mathrm{UNC},$$
/// where at each threshold $z$ the Brier score of the event $\{Y \le z\}$ under forecast $F_i(z)$ is
/// split by the PAV recalibration of `CorpPavDecomposition`. The grid is the union of outcomes and
/// ensemble members, on which every integrand is piecewise constant, so the integral is exact.
///
/// The isotonicity-based variant of Arnold et al. orders whole forecast distributions by stochastic
/// dominance; this threshold-wise variant orders by $F_i(z)$ at each threshold, which keeps
/// $\mathrm{MCB}, \mathrm{DSC} \ge 0$ but does not force the recalibrated $\hat{F}_i$ to be a CDF: tie
/// pools change from one threshold to the next (`max_cdf_violation` reports the largest decrease).
#[derive(Debug, Clone, PartialEq)]
pub struct CorpCrpsDecomposition {
    pub cases: usize,
    pub thresholds: usize,
    /// Mean CRPS $\int \mathrm{BS}(z)\,dz$ over the threshold grid.
    pub crps_mean: f64,
    /// Miscalibration $\mathrm{MCB} = \int \mathrm{REL}_{\mathrm{CORP}}(z)\,dz \ge 0$.
    pub mcb_miscalibration: f64,
    /// Discrimination $\mathrm{DSC} = \int \mathrm{RES}_{\mathrm{CORP}}(z)\,dz \ge 0$.
    pub dsc_discrimination: f64,
    /// Uncertainty $\mathrm{UNC} = \int \bar{y}_z(1 - \bar{y}_z)\,dz$, the CRPS of the empirical climatology.
    pub unc_uncertainty: f64,
    /// $|\overline{\mathrm{CRPS}} - (\mathrm{MCB} - \mathrm{DSC} + \mathrm{UNC})|$.
    pub identity_error: f64,
    /// $|\int \mathrm{BS}(z)\,dz - \frac{1}{n}\sum_i \mathrm{CRPS}(F_i, y_i)|$ against the closed-form ensemble CRPS.
    pub integral_error: f64,
    /// Largest decrease $\max_{i,\,z < z'} (\hat{F}_i(z) - \hat{F}_i(z'))$ of a recalibrated forecast in $z$.
    pub max_cdf_violation: f64,
    /// Lone-CRPS check: $\overline{\mathrm{CRPS}} < \mathrm{UNC}$ (equivalently $\mathrm{MCB} < \mathrm{DSC}$).
    pub beats_climatology: bool,
    /// Admissibility rule $\mathrm{MCB} \le$ `ADMISSIBLE_MCB_OVER_DSC` $\cdot\,\mathrm{DSC}$.
    pub admissible: bool,
}

impl CorpCrpsDecomposition {
    /// Assumed: miscalibration may consume at most half of the discrimination gain.
    pub const ADMISSIBLE_MCB_OVER_DSC: f64 = 0.5;

    /// Decomposes the mean CRPS of ensemble forecasts `ensembles[i]` for outcomes `outcomes[i]`.
    pub fn compute(ensembles: &[Vec<f64>], outcomes: &[f64]) -> Self {
        assert_eq!(ensembles.len(), outcomes.len());
        assert!(!outcomes.is_empty());
        assert!(ensembles.iter().all(|e| !e.is_empty()));
        let n = outcomes.len();
        let sorted_members: Vec<Vec<f64>> = ensembles
            .iter()
            .map(|e| {
                let mut s = e.clone();
                s.sort_by(f64::total_cmp);
                s
            })
            .collect();

        let mut grid: Vec<f64> = outcomes
            .iter()
            .copied()
            .chain(ensembles.iter().flatten().copied())
            .collect();
        grid.sort_by(f64::total_cmp);
        grid.dedup();

        let (mut crps, mut mcb, mut dsc, mut unc) = (0.0, 0.0, 0.0, 0.0);
        let mut forecast_z = vec![0.0; n];
        let mut event_z = vec![0.0; n];
        let mut running_max_q = vec![0.0_f64; n];
        let mut max_violation = 0.0_f64;
        for window in grid.windows(2) {
            let (z, width) = (window[0], window[1] - window[0]);
            for i in 0..n {
                let members = &sorted_members[i];
                let below = members.partition_point(|&x| x <= z);
                forecast_z[i] = below as f64 / members.len() as f64;
                event_z[i] = f64::from(u8::from(outcomes[i] <= z));
            }
            let corp = CorpPavDecomposition::compute(&forecast_z, &event_z);
            crps += width * corp.brier_total;
            mcb += width * corp.rel_miscalibration;
            dsc += width * corp.res_resolution;
            unc += width * corp.unc0_uncertainty;
            for (i, &q) in corp.recalibrated_probabilities_q.iter().enumerate() {
                max_violation = max_violation.max(running_max_q[i] - q);
                running_max_q[i] = running_max_q[i].max(q);
            }
        }

        let direct = ensembles
            .iter()
            .zip(outcomes)
            .map(|(e, &y)| empirical_crps(e, y))
            .sum::<f64>()
            / n as f64;
        Self {
            cases: n,
            thresholds: grid.len().saturating_sub(1),
            crps_mean: crps,
            mcb_miscalibration: mcb,
            dsc_discrimination: dsc,
            unc_uncertainty: unc,
            identity_error: (crps - (mcb - dsc + unc)).abs(),
            integral_error: (crps - direct).abs(),
            max_cdf_violation: max_violation,
            beats_climatology: crps < unc,
            admissible: mcb <= Self::ADMISSIBLE_MCB_OVER_DSC * dsc,
        }
    }

    /// Share of the mean CRPS that PAV recalibration would remove ($\mathrm{MCB} / \overline{\mathrm{CRPS}}$).
    pub fn miscalibration_share(&self) -> f64 {
        if self.crps_mean > 0.0 {
            self.mcb_miscalibration / self.crps_mean
        } else {
            0.0
        }
    }

    /// Same-weekday analog ensembles from a daily series: for each day `d >= lags.max()`, the members
    /// are `series[d - lag]` for every lag. Returns `(ensembles, outcomes)`.
    pub fn analog_ensembles(series: &[f64], lags: &[usize]) -> (Vec<Vec<f64>>, Vec<f64>) {
        let start = lags.iter().copied().max().unwrap_or(0);
        let mut ensembles = Vec::new();
        let mut outcomes = Vec::new();
        for d in start..series.len() {
            ensembles.push(lags.iter().map(|&lag| series[d - lag]).collect());
            outcomes.push(series[d]);
        }
        (ensembles, outcomes)
    }

    /// Distorts ensembles by a relative `bias` of the ensemble centre and a `spread_factor` around it
    /// (the sharp-but-biased forecast of the `crps_miscalibration` negative proof).
    pub fn distort_ensembles(
        ensembles: &[Vec<f64>],
        bias: f64,
        spread_factor: f64,
    ) -> Vec<Vec<f64>> {
        ensembles
            .iter()
            .map(|e| {
                let centre = e.iter().sum::<f64>() / e.len() as f64;
                e.iter()
                    .map(|&x| centre * (1.0 + bias) + spread_factor * (x - centre))
                    .collect()
            })
            .collect()
    }
}
