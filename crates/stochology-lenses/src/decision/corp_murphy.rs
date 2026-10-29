//! Isotonic **CORP** (*Consistent, Optimal, Reproducible, PAV-calibrated*) Decomposition,
//! Murphy Diagram, CORP–ROC–Murphy Evaluation Triptych, and Exact Fair-Coin Calibration vs. Decision Benchmark.

use super::cost_loss::{CostLossRule, brier_score};

/// Exact Isotonic **CORP** (*Consistent, Optimal, Reproducible, PAV-calibrated*) Decomposition
/// of the Brier Score (Dimitriadis, Gneiting & Jordan / Arnold et al. — §1.3.5):
/// $$\mathrm{BS}(p, y) = \underbrace{\bigl(\mathrm{BS}(p, y) - \mathrm{BS}(q, y)\bigr)}_{\mathrm{REL}_{\mathrm{CORP}}\text{ (Miscalibration)}} - \underbrace{\frac{1}{n}\sum_{i=1}^n (q_i - \bar{y})^2}_{\mathrm{RES}_{\mathrm{CORP}}\text{ (Resolution)}} + \underbrace{\bar{y}(1 - \bar{y})}_{\mathrm{UNC}_0\text{ (Uncertainty)}}$$
/// where $q_1 \le q_2 \le \dots \le q_n$ are the isotonically recalibrated probabilities via PAV.
#[derive(Debug, Clone, PartialEq)]
pub struct CorpPavDecomposition {
    pub brier_total: f64,
    pub rel_miscalibration: f64,
    pub res_resolution: f64,
    pub unc0_uncertainty: f64,
    /// Exact numerical residual $|\mathrm{BS} - (\mathrm{REL} - \mathrm{RES} + \mathrm{UNC}_0)|$.
    pub identity_error: f64,
    /// Isotonically recalibrated PAV probabilities $q_i$ sorted by original forecast order.
    pub recalibrated_probabilities_q: Vec<f64>,
}

impl CorpPavDecomposition {
    /// Computes the exact CORP decomposition using the Pool-Adjacent-Violators (PAV) algorithm.
    pub fn compute(forecasts_p: &[f64], outcomes_y: &[f64]) -> Self {
        assert_eq!(forecasts_p.len(), outcomes_y.len());
        assert!(!forecasts_p.is_empty());
        let n = forecasts_p.len();

        let mut idx: Vec<usize> = (0..n).collect();
        idx.sort_by(|&i, &j| {
            forecasts_p[i]
                .total_cmp(&forecasts_p[j])
                .then_with(|| outcomes_y[i].total_cmp(&outcomes_y[j]))
        });

        let mut initial_blocks: Vec<(usize, usize, f64, usize)> = Vec::new();
        for pos in 0..n {
            let orig = idx[pos];
            let p_val = forecasts_p[orig];
            let y_val = outcomes_y[orig];
            if let Some(last) = initial_blocks.last_mut() {
                let prev_orig = idx[last.1];
                if (forecasts_p[prev_orig] - p_val).abs() < 1e-15 {
                    last.1 = pos;
                    last.2 += y_val;
                    last.3 += 1;
                    continue;
                }
            }
            initial_blocks.push((pos, pos, y_val, 1));
        }

        let mut blocks: Vec<(usize, usize, f64, usize)> = Vec::with_capacity(initial_blocks.len());
        for blk in initial_blocks {
            blocks.push(blk);
            while blocks.len() >= 2 {
                let len = blocks.len();
                let mean_prev = blocks[len - 2].2 / (blocks[len - 2].3 as f64);
                let mean_curr = blocks[len - 1].2 / (blocks[len - 1].3 as f64);
                if mean_prev > mean_curr {
                    let removed = blocks.pop().unwrap();
                    let prev = blocks.last_mut().unwrap();
                    prev.1 = removed.1;
                    prev.2 += removed.2;
                    prev.3 += removed.3;
                } else {
                    break;
                }
            }
        }

        let mut q_orig = vec![0.0; n];
        for &(start, end, sum_y, count) in &blocks {
            let q_val = sum_y / (count as f64);
            for &orig_i in &idx[start..=end] {
                q_orig[orig_i] = q_val;
            }
        }

        let n_f = n as f64;
        let y_bar = outcomes_y.iter().sum::<f64>() / n_f;
        let bs = brier_score(forecasts_p, outcomes_y);
        let bs_q = brier_score(&q_orig, outcomes_y);
        let rel = (bs - bs_q).max(0.0);
        let res = q_orig.iter().map(|&q| (q - y_bar).powi(2)).sum::<f64>() / n_f;
        let unc0 = y_bar * (1.0 - y_bar);
        let reconstructed = rel - res + unc0;

        Self {
            brier_total: bs,
            rel_miscalibration: rel,
            res_resolution: res,
            unc0_uncertainty: unc0,
            identity_error: (bs - reconstructed).abs(),
            recalibrated_probabilities_q: q_orig,
        }
    }

    /// Computes the universal CORP decomposition under the **Logarithmic Score** (cross-entropy)
    /// sharing the exact PAV isotonic recalibrated probabilities $q_i$ (Dimitriadis, Gneiting & Jordan 2021).
    pub fn compute_logarithmic(forecasts_p: &[f64], outcomes_y: &[f64]) -> Self {
        let base = Self::compute(forecasts_p, outcomes_y);
        let q_orig = base.recalibrated_probabilities_q;
        let n_f = forecasts_p.len() as f64;
        let y_bar = outcomes_y.iter().sum::<f64>() / n_f;

        let bernoulli_log_loss = |p: f64, y: f64| -> f64 {
            let t1 = if y <= 0.0 {
                0.0
            } else {
                -y * p.max(1e-15).ln()
            };
            let t0 = if y >= 1.0 {
                0.0
            } else {
                -(1.0 - y) * (1.0 - p).max(1e-15).ln()
            };
            t1 + t0
        };

        let s_log: f64 = forecasts_p
            .iter()
            .zip(outcomes_y.iter())
            .map(|(&p, &y)| bernoulli_log_loss(p, y))
            .sum::<f64>()
            / n_f;
        let s_log_q: f64 = q_orig
            .iter()
            .zip(outcomes_y.iter())
            .map(|(&q, &y)| bernoulli_log_loss(q, y))
            .sum::<f64>()
            / n_f;
        let unc_log = bernoulli_log_loss(y_bar, y_bar);
        let rel_log = (s_log - s_log_q).max(0.0);
        let res_log = (unc_log - s_log_q).max(0.0);
        let reconstructed = rel_log - res_log + unc_log;

        Self {
            brier_total: s_log,
            rel_miscalibration: rel_log,
            res_resolution: res_log,
            unc0_uncertainty: unc_log,
            identity_error: (s_log - reconstructed).abs(),
            recalibrated_probabilities_q: q_orig,
        }
    }
}

/// Ehm, Gneiting, Jordan & Krüger (2016) **Murphy Diagram** of elementary cost-loss scores across all
/// economic thresholds $\theta \in (0, 1)$.
#[derive(Debug, Clone, PartialEq)]
pub struct MurphyDiagram {
    /// Sorted unique breakpoints $0 = t_0 < t_1 < \dots < t_m = 1$ of the piecewise-linear curve $\bar{S}_\theta$.
    pub breakpoints_theta: Vec<f64>,
    /// Midpoint thresholds $\theta_k = \frac{1}{2}(t_{k-1} + t_k)$ of each linear segment.
    pub segment_midpoints_theta: Vec<f64>,
    /// Mean elementary score $\bar{S}_{\theta_k}$ at each segment midpoint.
    pub segment_midpoint_scores: Vec<f64>,
    /// Exact piecewise-linear integral $\int_0^1 \bar{S}_\theta(x, y)\,d\theta$.
    pub area_under_murphy_curve: f64,
    /// Half of the Brier Score $\frac{1}{2}\,\mathrm{BS}(x, y)$.
    pub half_brier_score: f64,
    /// Exact numerical residual $\bigl|\int_0^1 \bar{S}_\theta\,d\theta - \frac{1}{2}\mathrm{BS}\bigr|$.
    pub area_identity_error: f64,
    /// Elementary score at symmetric cost-loss threshold $\theta = 0.5$: $\bar{S}_{0.5}(x, y) = \frac{1}{2}\mathrm{Err}_{0.5}$.
    pub score_at_half: f64,
    /// Half of the classification error rate at threshold $0.5$.
    pub half_classification_error_at_half: f64,
}

impl MurphyDiagram {
    /// Evaluates the exact elementary score $S_\theta(x, y)$ for a single forecast-outcome pair at threshold $\theta \in [0, 1]$.
    #[inline]
    pub fn elementary_score(forecast_x: f64, outcome_y: f64, theta: f64) -> f64 {
        if outcome_y < 0.5 {
            if forecast_x > theta { theta } else { 0.0 }
        } else if forecast_x <= theta {
            1.0 - theta
        } else {
            0.0
        }
    }

    /// Evaluates the sample mean elementary score $\bar{S}_\theta(x, y) = \frac{1}{n}\sum_{i=1}^n S_\theta(x_i, y_i)$.
    pub fn mean_elementary_score(forecasts_p: &[f64], outcomes_y: &[f64], theta: f64) -> f64 {
        assert_eq!(forecasts_p.len(), outcomes_y.len());
        assert!(!forecasts_p.is_empty());
        let sum: f64 = forecasts_p
            .iter()
            .zip(outcomes_y.iter())
            .map(|(&x, &y)| Self::elementary_score(x, y, theta))
            .sum();
        sum / (forecasts_p.len() as f64)
    }

    /// Constructs the exact piecewise-linear Murphy Diagram over all forecast breakpoints.
    pub fn compute(forecasts_p: &[f64], outcomes_y: &[f64]) -> Self {
        assert_eq!(forecasts_p.len(), outcomes_y.len());
        assert!(!forecasts_p.is_empty());

        let mut pts = Vec::with_capacity(forecasts_p.len() + 2);
        pts.push(0.0);
        pts.push(1.0);
        for &p in forecasts_p {
            pts.push(p.clamp(0.0, 1.0));
        }
        pts.sort_by(|a, b| a.total_cmp(b));
        pts.dedup_by(|a, b| (*a - *b).abs() < 1e-14);

        let mut midpoints = Vec::with_capacity(pts.len().saturating_sub(1));
        let mut mid_scores = Vec::with_capacity(pts.len().saturating_sub(1));
        let mut area = 0.0;

        for w in pts.windows(2) {
            let a = w[0];
            let b = w[1];
            let width = b - a;
            if width > 1e-15 {
                let mid = 0.5 * (a + b);
                let s_mid = Self::mean_elementary_score(forecasts_p, outcomes_y, mid);
                midpoints.push(mid);
                mid_scores.push(s_mid);
                area += width * s_mid;
            }
        }

        let half_bs = 0.5 * brier_score(forecasts_p, outcomes_y);
        let s_half = Self::mean_elementary_score(forecasts_p, outcomes_y, 0.5);
        let err_half = forecasts_p
            .iter()
            .zip(outcomes_y.iter())
            .filter(|&(&x, &y)| (y >= 0.5 && x <= 0.5) || (y < 0.5 && x > 0.5))
            .count() as f64
            / (forecasts_p.len() as f64);
        let half_err = 0.5 * err_half;

        Self {
            breakpoints_theta: pts,
            segment_midpoints_theta: midpoints,
            segment_midpoint_scores: mid_scores,
            area_under_murphy_curve: area,
            half_brier_score: half_bs,
            area_identity_error: (area - half_bs).abs(),
            score_at_half: s_half,
            half_classification_error_at_half: half_err,
        }
    }

    /// Checks whether `forecasts_a` uniformly dominates `forecasts_b` across all cost-loss thresholds $\theta \in (0, 1)$.
    pub fn check_uniform_dominance(
        forecasts_a: &[f64],
        forecasts_b: &[f64],
        outcomes_y: &[f64],
    ) -> (bool, f64, f64) {
        assert_eq!(forecasts_a.len(), outcomes_y.len());
        assert_eq!(forecasts_b.len(), outcomes_y.len());
        let mut pts = Vec::with_capacity(forecasts_a.len() + forecasts_b.len() + 2);
        pts.push(0.0);
        pts.push(1.0);
        for &p in forecasts_a.iter().chain(forecasts_b.iter()) {
            pts.push(p.clamp(0.0, 1.0));
        }
        pts.sort_by(|a, b| a.total_cmp(b));
        pts.dedup_by(|a, b| (*a - *b).abs() < 1e-14);

        let mut max_excess = 0.0_f64;
        let mut worst_theta = 0.5_f64;

        for w in pts.windows(2) {
            let a = w[0];
            let b = w[1];
            if b - a > 1e-14 {
                for frac in [1e-7, 0.5, 1.0 - 1e-7] {
                    let th = a + frac * (b - a);
                    let diff = Self::mean_elementary_score(forecasts_a, outcomes_y, th)
                        - Self::mean_elementary_score(forecasts_b, outcomes_y, th);
                    if diff > max_excess {
                        max_excess = diff;
                        worst_theta = th;
                    }
                }
            }
        }
        (max_excess <= 1e-12, max_excess, worst_theta)
    }
}

/// Complete **CORP–ROC–Murphy Evaluation Triptych** (Dimitriadis, Gneiting & Jordan 2021; Ehm et al. 2016),
/// uniting Brier CORP, Logarithmic CORP, the Murphy Diagram, and the PAV-calibrated concave ROC AUC.
#[derive(Debug, Clone, PartialEq)]
pub struct CorpRocMurphyTriptych {
    pub brier_corp: CorpPavDecomposition,
    pub log_corp: CorpPavDecomposition,
    pub murphy: MurphyDiagram,
    /// Exact Mann-Whitney / Wilcoxon area under the PAV-calibrated concave ROC curve.
    pub pav_concave_roc_auc: f64,
    /// True if Brier CORP, Log CORP, and Murphy integral identities all close within $10^{-12}$.
    pub all_triptych_identities_closed: bool,
}

impl CorpRocMurphyTriptych {
    pub fn evaluate(forecasts_p: &[f64], outcomes_y: &[f64]) -> Self {
        let brier_corp = CorpPavDecomposition::compute(forecasts_p, outcomes_y);
        let log_corp = CorpPavDecomposition::compute_logarithmic(forecasts_p, outcomes_y);
        let murphy = MurphyDiagram::compute(forecasts_p, outcomes_y);

        let q = &brier_corp.recalibrated_probabilities_q;
        let mut pos_q = Vec::new();
        let mut neg_q = Vec::new();
        for (&qi, &yi) in q.iter().zip(outcomes_y.iter()) {
            if yi >= 0.5 {
                pos_q.push(qi);
            } else {
                neg_q.push(qi);
            }
        }
        let auc = if pos_q.is_empty() || neg_q.is_empty() {
            0.5
        } else {
            let mut wins = 0.0;
            for &qp in &pos_q {
                for &qn in &neg_q {
                    if (qp - qn).abs() < 1e-14 {
                        wins += 0.5;
                    } else if qp > qn {
                        wins += 1.0;
                    }
                }
            }
            wins / ((pos_q.len() * neg_q.len()) as f64)
        };

        let closed = brier_corp.identity_error < 1e-12
            && log_corp.identity_error < 1e-12
            && murphy.area_identity_error < 1e-12
            && (murphy.score_at_half - murphy.half_classification_error_at_half).abs() < 1e-12;

        Self {
            brier_corp,
            log_corp,
            murphy,
            pav_concave_roc_auc: auc,
            all_triptych_identities_closed: closed,
        }
    }
}

/// Exact zero-cost benchmark proving that *Calibrated Decisions Are Not Calibrated Probabilities*
/// (anchored to a known ground-truth fair coin $p_0 = 0.50$ with `seed = 2026`).
#[derive(Debug, Clone, PartialEq)]
pub struct ExactCoinCalibrationBenchmark {
    pub seed: u64,
    pub sample_size_n: usize,
    pub true_coin_probability: f64,
    pub preregistration_sha256: String,
    pub empirical_outcome_mean: f64,
    pub calibrated_corp: CorpPavDecomposition,
    pub overconfident_corp: CorpPavDecomposition,
    /// Fraction of trials where both forecasters take the exact same binary action under $p^*_A = 0.30$.
    pub decision_agreement_rate_at_p_star_a: f64,
    /// Excess economic regret per unit suffered by the overconfident forecaster when the cost-loss threshold shifts to $p^*_B = 0.70$.
    pub shifted_threshold_excess_regret_per_unit: f64,
    pub separates_decision_from_probability_calibration: bool,
}

impl ExactCoinCalibrationBenchmark {
    /// Executes the exact fair-coin ($p_0 = 0.50$) calibration vs. decision audit (`seed = 2026`).
    pub fn evaluate(seed: u64, sample_size_n: usize) -> Self {
        let n = sample_size_n.max(100);
        let payload = format!(
            "exact_coin_calibration_v1|seed={seed}|p_true=0.50000|n={n}|p_star_a=0.30000|p_star_b=0.70000"
        );
        let prereg_sha = stochology_core::sha256_hex(payload.as_bytes());

        let mut rng = stochology_core::Rng::new(seed);
        let mut p_cal = Vec::with_capacity(n);
        let mut p_over = Vec::with_capacity(n);
        let mut y_out = Vec::with_capacity(n);

        for i in 0..n {
            let delta = if i.is_multiple_of(2) { -0.05 } else { 0.05 };
            let p_true_i = 0.50 + delta;
            let y = if rng.bernoulli(p_true_i) { 1.0 } else { 0.0 };
            p_cal.push(p_true_i);
            p_over.push(0.85 + delta);
            y_out.push(y);
        }

        let rule_a = CostLossRule::new(30.0, 100.0, 1.0);
        let rule_b = CostLossRule::new(70.0, 100.0, 1.0);

        let mut agree_a = 0usize;
        let mut loss_cal_b = 0.0;
        let mut loss_over_b = 0.0;

        for i in 0..n {
            let act_cal_a = rule_a.recommend_action(p_cal[i]);
            let act_over_a = rule_a.recommend_action(p_over[i]);
            if act_cal_a == act_over_a {
                agree_a += 1;
            }

            let act_cal_b = rule_b.recommend_action(p_cal[i]);
            let act_over_b = rule_b.recommend_action(p_over[i]);
            loss_cal_b += if act_cal_b == "protect" {
                rule_b.protection_cost_c
            } else {
                y_out[i] * rule_b.event_loss_l
            };
            loss_over_b += if act_over_b == "protect" {
                rule_b.protection_cost_c
            } else {
                y_out[i] * rule_b.event_loss_l
            };
        }

        let n_f = n as f64;
        let agree_rate_a = (agree_a as f64) / n_f;
        let excess_regret_b = (loss_over_b - loss_cal_b) / n_f;
        let y_bar = y_out.iter().sum::<f64>() / n_f;

        let cal_corp = CorpPavDecomposition::compute(&p_cal, &y_out);
        let over_corp = CorpPavDecomposition::compute(&p_over, &y_out);

        let separates = (agree_rate_a - 1.0).abs() < 1e-12
            && over_corp.rel_miscalibration > 0.08
            && cal_corp.rel_miscalibration < 0.01
            && (cal_corp.res_resolution - over_corp.res_resolution).abs() < 1e-12
            && excess_regret_b > 10.0;

        Self {
            seed,
            sample_size_n: n,
            true_coin_probability: 0.50,
            preregistration_sha256: prereg_sha,
            empirical_outcome_mean: y_bar,
            calibrated_corp: cal_corp,
            overconfident_corp: over_corp,
            decision_agreement_rate_at_p_star_a: agree_rate_a,
            shifted_threshold_excess_regret_per_unit: excess_regret_b,
            separates_decision_from_probability_calibration: separates,
        }
    }
}
