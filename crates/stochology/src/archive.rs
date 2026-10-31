//! **Chapter 5 (Filter II · Tension 4) — From World to Archive: Evidential Weight, Big Data Paradox, and the Uncounted.**
//!
//! Mediates the tension between raw archive volume and effective evidential weight
//! (Keynes, 1921; Horvitz & Thompson, 1952; Kish, 1965; Campbell, 1979; Moulton, 1986; Chao, 1987; Meng, 2018):
//! - Keynes's evidential weight $V = \frac{K}{K + I}$ vs. raw probability $p$ (`KeynesEvidentialWeight`);
//! - Meng's Big Data Paradox identity ($\rho_{R,Y} \cdot \sqrt{(1-f)/f} \cdot \sigma_Y$) and effective $n$ (`MengBigDataDefect`);
//! - Campbell–Moulton unequal-cluster design effect $\tilde{m} = \bar{m}(1 + \mathrm{CV}^2)$ and $n_{\mathrm{eff}}(p)$ (`ClusterDesignEffect`);
//! - Horvitz–Thompson & Kish weighted effective sample size and positivity floor (`KishWeightAudit`);
//! - Lincoln–Petersen & Chao (1987) capture-recapture estimator of the uncounted dark figure (`CaptureRecaptureAudit`);
//! - Bitemporal vintage archive filtering ($t_{\mathrm{available}} \le t_{\mathrm{decision}}$, `BitemporalRecord`).

use crate::rng::{mean, quantile};

/// John Maynard Keynes (1921) separation between directional probability $p \in (0, 1)$
/// and evidential weight $V = \frac{n_{\mathrm{eff}}}{n_{\mathrm{eff}} + n_{\mathrm{ignorance}}} \in (0, 1)$.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KeynesEvidentialWeight {
    /// Directional balance of probability $p$.
    pub probability_p: f64,
    /// Effective independent sample size or verified evidence mass $K > 0$.
    pub effective_knowledge_k: f64,
    /// Unexamined structural ignorance mass $I > 0$.
    pub relevant_ignorance_i: f64,
    /// Keynes's weight of argument $V = \frac{K}{K + I} \in (0, 1)$.
    pub evidential_weight_v: f64,
}

impl KeynesEvidentialWeight {
    /// Computes Keynes's evidential weight $V$ from directional probability `p`, effective evidence `k`, and ignorance `i`.
    pub fn evaluate(
        probability_p: f64,
        effective_knowledge_k: f64,
        relevant_ignorance_i: f64,
    ) -> Self {
        let k = effective_knowledge_k.max(0.0);
        let i = relevant_ignorance_i.max(1e-12);
        Self {
            probability_p: probability_p.clamp(1e-9, 1.0 - 1e-9),
            effective_knowledge_k: k,
            relevant_ignorance_i: i,
            evidential_weight_v: k / (k + i),
        }
    }
}

/// Xiao-Li Meng (2018) exact Big Data Paradox decomposition:
/// $$\bar{Y}_n - \bar{Y}_N = \rho_{R,Y} \cdot \sqrt{\frac{1 - f}{f}} \cdot \sigma_Y$$
/// where $f = n / N$ is the sampling fraction and $\rho_{R,Y}$ is the data defect correlation between
/// inclusion $R_i \in \{0, 1\}$ and outcome $Y_i$.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MengBigDataDefect {
    /// Population size $N$.
    pub population_n: f64,
    /// Raw archive sample size $n$.
    pub sample_n: f64,
    /// Sampling fraction $f = n / N \in (0, 1)$.
    pub sampling_fraction_f: f64,
    /// Data defect correlation $\rho_{R,Y}$ between recording indicator $R$ and outcome $Y$.
    pub rho_ry: f64,
    /// Outcome standard deviation $\sigma_Y$.
    pub sigma_y: f64,
    /// Exact estimation error $\bar{Y}_n - \bar{Y}_N$.
    pub estimation_error: f64,
    /// Equivalent simple-random-sample effective size $n_{\mathrm{eff,Meng}}$ achieving the same MSE.
    pub effective_sample_size: f64,
}

impl MengBigDataDefect {
    /// Evaluates Meng's three-factor identity and effective sample size.
    pub fn evaluate(population_n: f64, sample_n: f64, rho_ry: f64, sigma_y: f64) -> Self {
        assert!(
            population_n > sample_n && sample_n >= 1.0 && sigma_y >= 0.0,
            "MengBigDataDefect requires N > n >= 1 and sigma_y >= 0"
        );
        let f = (sample_n / population_n).clamp(1e-12, 1.0 - 1e-12);
        let drop_factor = ((1.0 - f) / f).sqrt();
        let err = rho_ry * drop_factor * sigma_y;
        // MSE = ((1-f)/f) * rho^2 * sigma^2; SRS MSE = (1/n_eff) * sigma^2 => n_eff = f / ((1-f)*rho^2 + f/n)
        let denom = (1.0 - f) * rho_ry * rho_ry + (1.0 - f) / sample_n;
        let n_eff = (f / denom.max(1e-15)).min(sample_n);
        Self {
            population_n,
            sample_n,
            sampling_fraction_f: f,
            rho_ry,
            sigma_y,
            estimation_error: err,
            effective_sample_size: n_eff,
        }
    }
}

/// Campbell (1977/1979) & Moulton (1986) unequal-cluster design effect for binary threshold exceedances
/// at tail quantile $p \in (0, 1)$:
/// $$\tilde{m} = \bar{m}(1 + \mathrm{CV}_m^2), \qquad \mathrm{DEFF}(p) = 1 + (\tilde{m} - 1)\rho_I(p), \qquad n_{\mathrm{eff}}(p) = \frac{n}{\mathrm{DEFF}(p)}$$
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ClusterDesignEffect {
    /// Total observations $n = \sum_{c=1}^C m_c$.
    pub total_observations_n: usize,
    /// Number of clusters $C$.
    pub num_clusters_c: usize,
    /// Mean cluster size $\bar{m} = n / C$.
    pub mean_cluster_size: f64,
    /// Moulton–Campbell effective cluster size $\tilde{m} = \frac{\sum m_c^2}{\sum m_c} = \bar{m}(1 + \mathrm{CV}_m^2)$.
    pub moulton_effective_cluster_size: f64,
    /// Estimated intraclass correlation $\rho_I(p) \in [0, 1]$ of threshold exceedance indicators.
    pub exceedance_icc_rho: f64,
    /// Design effect $\mathrm{DEFF}(p) = 1 + (\tilde{m} - 1)\rho_I(p) \ge 1$.
    pub design_effect: f64,
    /// Effective independent sample size $n_{\mathrm{eff}}(p) = n / \mathrm{DEFF}(p)$.
    pub effective_n: f64,
}

impl ClusterDesignEffect {
    /// Computes the Campbell–Moulton exceedance design effect across `clusters` at quantile `tail_quantile_p`.
    pub fn evaluate(clusters: &[Vec<f64>], tail_quantile_p: f64) -> Self {
        let non_empty: Vec<&Vec<f64>> = clusters.iter().filter(|c| !c.is_empty()).collect();
        assert!(
            non_empty.len() >= 2,
            "ClusterDesignEffect requires at least 2 non-empty clusters"
        );
        let c_count = non_empty.len();
        let mut all_vals = Vec::new();
        let mut sum_m = 0usize;
        let mut sum_m2 = 0usize;
        for c in &non_empty {
            let m = c.len();
            sum_m += m;
            sum_m2 += m * m;
            all_vals.extend_from_slice(c);
        }
        let n_f = sum_m as f64;
        let c_f = c_count as f64;
        let m_bar = n_f / c_f;
        let m_tilde = (sum_m2 as f64) / n_f;

        let thresh = quantile(&all_vals, tail_quantile_p.clamp(0.05, 0.99));
        // ANOVA estimator of ICC on binary indicators I(x > thresh)
        let mut cluster_rates = Vec::with_capacity(c_count);
        let mut total_exceed = 0.0;
        let mut ssw = 0.0;
        for c in &non_empty {
            let k_c = c.iter().filter(|&&x| x > thresh).count() as f64;
            let m_c = c.len() as f64;
            let p_c = k_c / m_c;
            cluster_rates.push((m_c, p_c));
            total_exceed += k_c;
            ssw += m_c * p_c * (1.0 - p_c);
        }
        let p_bar = total_exceed / n_f;
        let mut ssb = 0.0;
        for &(m_c, p_c) in &cluster_rates {
            let diff = p_c - p_bar;
            ssb += m_c * diff * diff;
        }
        let msb = ssb / ((c_f - 1.0).max(1.0));
        let msw = ssw / ((n_f - c_f).max(1.0));
        let n0 = (n_f - (sum_m2 as f64) / n_f) / ((c_f - 1.0).max(1.0));
        let denom = msb + (n0 - 1.0).max(0.0) * msw;
        let rho = if denom <= 1e-12 {
            0.0
        } else {
            ((msb - msw) / denom).clamp(0.0, 1.0)
        };
        let deff = (1.0 + (m_tilde - 1.0).max(0.0) * rho).max(1.0);
        let n_eff = n_f / deff;

        Self {
            total_observations_n: sum_m,
            num_clusters_c: c_count,
            mean_cluster_size: m_bar,
            moulton_effective_cluster_size: m_tilde,
            exceedance_icc_rho: rho,
            design_effect: deff,
            effective_n: n_eff,
        }
    }

    /// Combines Moulton cluster dependence ($\mathrm{DEFF}$) and Meng selection defect ($\rho_{R,Y}$)
    /// into the joint effective sample size $n_{\mathrm{eff,joint}}$.
    pub fn joint_meng_moulton_effective_n(&self, population_n: f64, rho_ry: f64) -> f64 {
        let n_indep = self.effective_n;
        let f_eff = (n_indep / population_n.max(n_indep + 1.0)).clamp(1e-12, 1.0 - 1e-12);
        let denom = (1.0 - f_eff) * rho_ry * rho_ry + (1.0 - f_eff) / n_indep.max(1.0);
        (f_eff / denom.max(1e-15)).min(n_indep)
    }
}

/// Horvitz & Thompson (1952) and Leslie Kish (1965) inverse-propensity weighting audit.
///
/// When inclusion probabilities $\pi_i$ approach $0$ in vulnerable strata, inverse weights $w_i = 1/\pi_i$
/// explode and Kish's effective sample size $n_{\mathrm{eff,Kish}} = \frac{(\sum w_i)^2}{\sum w_i^2}$ collapses.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KishWeightAudit {
    /// Minimum inclusion propensity $\pi_{\min} = \min_i \pi_i$.
    pub min_propensity: f64,
    /// Maximum inverse weight $w_{\max} = 1 / \pi_{\min}$.
    pub max_weight: f64,
    /// Kish effective sample size $n_{\mathrm{eff,Kish}} = \frac{(\sum w_i)^2}{\sum w_i^2}$.
    pub kish_effective_n: f64,
    /// Weighting efficiency $n_{\mathrm{eff,Kish}} / n \in (0, 1]$.
    pub weighting_efficiency: f64,
}

impl KishWeightAudit {
    /// Computes the Kish effective sample size from inclusion propensities `propensities` (one per observation)
    /// or stratum pairs `(stratum_sample_count, stratum_inclusion_propensity)`.
    pub fn from_strata(strata_counts_and_propensities: &[(usize, f64)]) -> Self {
        assert!(
            !strata_counts_and_propensities.is_empty(),
            "KishWeightAudit requires at least one stratum"
        );
        let mut total_n = 0usize;
        let mut sum_w = 0.0_f64;
        let mut sum_w2 = 0.0_f64;
        let mut min_pi = 1.0_f64;
        for &(count, pi) in strata_counts_and_propensities {
            if count == 0 {
                continue;
            }
            let clean_pi = pi.clamp(1e-9, 1.0);
            if clean_pi < min_pi {
                min_pi = clean_pi;
            }
            let w = 1.0 / clean_pi;
            total_n += count;
            sum_w += (count as f64) * w;
            sum_w2 += (count as f64) * w * w;
        }
        let kish_n = if sum_w2 <= 1e-15 {
            0.0
        } else {
            (sum_w * sum_w) / sum_w2
        };
        let eff = if total_n == 0 {
            0.0
        } else {
            kish_n / (total_n as f64)
        };
        Self {
            min_propensity: min_pi,
            max_weight: 1.0 / min_pi,
            kish_effective_n: kish_n,
            weighting_efficiency: eff,
        }
    }
}

/// Lincoln–Petersen and Anne Chao (1987) capture-recapture estimator for the uncounted dark figure $n_{00}$
/// omitted by two incomplete registries ($A$ and $B$).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CaptureRecaptureAudit {
    /// Count observed in both registries ($n_{11}$).
    pub both_registries_n11: usize,
    /// Count observed only in registry $A$ ($n_{10}$).
    pub only_a_n10: usize,
    /// Count observed only in registry $B$ ($n_{01}$).
    pub only_b_n01: usize,
    /// Total observed across the union $n_{\mathrm{obs}} = n_{11} + n_{10} + n_{01}$.
    pub observed_union: usize,
    /// Estimated uncounted population $\hat{n}_{00} = \frac{n_{10} n_{01}}{n_{11}}$.
    pub estimated_uncounted_n00: f64,
    /// Estimated total population $\hat{N} = n_{\mathrm{obs}} + \hat{n}_{00}$.
    pub estimated_total_population: f64,
    /// Chao (1987) heterogeneity-robust lower bound $\hat{N}_{\mathrm{Chao}} = n_{\mathrm{obs}} + \frac{f_1^2}{2 f_2}$
    /// where $f_1 = n_{10} + n_{01}$ (singletons) and $f_2 = n_{11}$ (doubletons).
    pub chao_lower_bound_total: f64,
}

impl CaptureRecaptureAudit {
    /// Estimates the uncounted population $\hat{n}_{00}$ and Chao (1987) lower bound from registry overlap counts.
    pub fn evaluate(both_n11: usize, only_a_n10: usize, only_b_n01: usize) -> Self {
        assert!(
            both_n11 >= 1,
            "CaptureRecaptureAudit requires both_n11 >= 1"
        );
        let obs = both_n11 + only_a_n10 + only_b_n01;
        let n00 = ((only_a_n10 * only_b_n01) as f64) / (both_n11 as f64);
        let f1 = (only_a_n10 + only_b_n01) as f64;
        let f2 = both_n11 as f64;
        let chao_add = (f1 * f1) / (2.0 * f2);
        Self {
            both_registries_n11: both_n11,
            only_a_n10,
            only_b_n01,
            observed_union: obs,
            estimated_uncounted_n00: n00,
            estimated_total_population: (obs as f64) + n00,
            chao_lower_bound_total: (obs as f64) + chao_add,
        }
    }
}

/// Bitemporal observation record storing both the real-world event timestamp $t_{\mathrm{event}}$
/// and the administrative availability / publication vintage $t_{\mathrm{available}} \ge t_{\mathrm{event}}$.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BitemporalRecord {
    /// Time when the physical or social event occurred ($t_{\mathrm{event}}$).
    pub event_time: u64,
    /// Time when the record became available in the archive ($t_{\mathrm{available}}$).
    pub available_time: u64,
    /// Recorded value at this vintage.
    pub value: f64,
}

impl BitemporalRecord {
    /// True if the record respects causal bitemporal order ($t_{\mathrm{available}} \ge t_{\mathrm{event}}$).
    #[inline]
    pub const fn is_causally_valid(&self) -> bool {
        self.available_time >= self.event_time
    }
}

/// Filters a slice of bitemporal records so that a decision at `decision_time` sees only records
/// with `available_time <= decision_time`, returning the latest available vintage per `event_time`.
/// Returns `Err` if any record violates `available_time >= event_time`.
pub fn bitemporal_view_at(
    records: &[BitemporalRecord],
    decision_time: u64,
) -> Result<Vec<BitemporalRecord>, &'static str> {
    for r in records {
        if !r.is_causally_valid() {
            return Err(
                "Bitemporal violation: available_time < event_time (future look-ahead leak)",
            );
        }
    }
    let mut latest_by_event: std::collections::BTreeMap<u64, BitemporalRecord> =
        std::collections::BTreeMap::new();
    for &r in records {
        if r.available_time <= decision_time {
            match latest_by_event.get(&r.event_time) {
                Some(prev) if prev.available_time > r.available_time => {}
                _ => {
                    latest_by_event.insert(r.event_time, r);
                }
            }
        }
    }
    Ok(latest_by_event.into_values().collect())
}

/// Parses a quarterly vintage string such as `"1965:Q4"` or `"1965Q4"` into a linear quarter index `4 * year + (q - 1)`.
pub fn parse_quarter(s: &str) -> Option<u32> {
    let clean = s.trim();
    let (yr_str, q_str) = if let Some(pos) = clean.find(":Q") {
        (&clean[..pos], &clean[pos + 2..])
    } else {
        let pos = clean.find('Q')?;
        (&clean[..pos], &clean[pos + 1..])
    };
    let year: u32 = yr_str.parse().ok()?;
    let q: u32 = q_str.parse().ok()?;
    if (1..=4).contains(&q) {
        Some(4 * year + (q - 1))
    } else {
        None
    }
}

/// Audits how much the real-time initial release of an economic series differs from its final revised vintage
/// (`rtdsm_routput_vintages.csv`), proving why backtesting on final revised data leaks future information.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VintageRevisionAudit {
    /// Number of quarters compared between initial release and final revision.
    pub num_quarters: usize,
    /// Mean absolute revision growth-rate gap (annualized percentage points).
    pub mean_abs_growth_revision_pp: f64,
    /// Maximum absolute revision growth-rate gap (annualized percentage points).
    pub max_abs_growth_revision_pp: f64,
    /// Fraction of quarters where the sign of quarterly growth flipped between initial release and final revision.
    pub sign_flip_rate: f64,
}

impl VintageRevisionAudit {
    /// Evaluates revision magnitude across `(vintage_q, obs_q, level)` triples.
    pub fn evaluate(vintages: &[(u32, u32, f64)]) -> Self {
        use std::collections::BTreeMap;
        // Group byvintage_q -> map of obs_q -> level
        let mut by_vintage: BTreeMap<u32, BTreeMap<u32, f64>> = BTreeMap::new();
        for &(v_q, o_q, val) in vintages {
            if val > 0.0 {
                by_vintage.entry(v_q).or_default().insert(o_q, val);
            }
        }
        let Some((&last_vintage, final_series)) = by_vintage.iter().next_back() else {
            return Self {
                num_quarters: 0,
                mean_abs_growth_revision_pp: 0.0,
                max_abs_growth_revision_pp: 0.0,
                sign_flip_rate: 0.0,
            };
        };
        let _ = last_vintage;
        // For each vintage v_q, initial release of quarter o_q = v_q - 1 uses (o_q - 1, o_q) as seen at v_q
        let mut diffs = Vec::new();
        let mut flips = 0usize;
        for (&v_q, series) in &by_vintage {
            if v_q < 2 {
                continue;
            }
            let o_q = v_q - 1;
            let o_prev = v_q - 2;
            if let (Some(&init_curr), Some(&init_prev), Some(&fin_curr), Some(&fin_prev)) = (
                series.get(&o_q),
                series.get(&o_prev),
                final_series.get(&o_q),
                final_series.get(&o_prev),
            ) {
                let g_init = 400.0 * ((init_curr / init_prev).ln());
                let g_fin = 400.0 * ((fin_curr / fin_prev).ln());
                diffs.push((g_init - g_fin).abs());
                if g_init * g_fin < 0.0 {
                    flips += 1;
                }
            }
        }
        let n = diffs.len();
        let mean_gap = if n == 0 { 0.0 } else { mean(&diffs) };
        let max_gap = diffs.iter().copied().fold(0.0_f64, f64::max);
        let flip_rate = if n == 0 {
            0.0
        } else {
            (flips as f64) / (n as f64)
        };
        Self {
            num_quarters: n,
            mean_abs_growth_revision_pp: mean_gap,
            max_abs_growth_revision_pp: max_gap,
            sign_flip_rate: flip_rate,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn meng_moulton_kish_and_chao_capture_archive_defects() {
        let meng = MengBigDataDefect::evaluate(10_000_000.0, 2_300_000.0, -0.05, 0.50);
        assert!(meng.effective_sample_size < 500.0);

        let c1 = vec![10.0, 12.0, 11.0, 13.0, 14.0];
        let c2 = vec![1.0, 2.0, 1.5, 2.5, 1.2];
        let c3 = vec![20.0, 22.0, 21.0, 19.0, 23.0];
        let deff = ClusterDesignEffect::evaluate(&[c1, c2, c3], 0.60);
        assert!(deff.design_effect > 1.2);
        assert!(deff.effective_n < (deff.total_observations_n as f64));

        let kish = KishWeightAudit::from_strata(&[(990, 0.90), (10, 0.001)]);
        assert!(kish.kish_effective_n < 30.0);

        let cap = CaptureRecaptureAudit::evaluate(200, 300, 300);
        assert!((cap.estimated_uncounted_n00 - 450.0).abs() < 1e-12);
        assert!(cap.chao_lower_bound_total > (cap.observed_union as f64));
    }

    #[test]
    fn bitemporal_view_blocks_future_vintages_and_rejects_lookahead_leaks() {
        let recs = [
            BitemporalRecord {
                event_time: 10,
                available_time: 11,
                value: 100.0,
            },
            BitemporalRecord {
                event_time: 10,
                available_time: 15,
                value: 120.0,
            },
        ];
        let view_at_12 = bitemporal_view_at(&recs, 12).expect("valid bitemporal");
        assert_eq!(view_at_12.len(), 1);
        assert_eq!(view_at_12[0].value, 100.0);

        let leaked = [BitemporalRecord {
            event_time: 10,
            available_time: 9,
            value: 100.0,
        }];
        assert!(bitemporal_view_at(&leaked, 12).is_err());
    }
}
