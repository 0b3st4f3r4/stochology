//! Real-Data Multi-Dataset Coupler (`djia_1885_2023.csv`, `bank_calls.csv`, `danish_fire.csv`,
//! `vic_elec.csv`, and `atlantic_storms.csv`) auditing the Stochary-Conjectural & Cybernetic Synthesis.

use super::supply_chain_plant::rockafellar_uryasev_cvar;
use crate::decision::CorpPavDecomposition;
use stochology_core::math::stats::empirical_wasserstein_1_grid;
use stochology_core::{mean, quantile, sample_std, sample_variance};

/// Real-Data Multi-Dataset Coupler (`djia_1885_2023.csv`, `bank_calls.csv`, `danish_fire.csv`,
/// `vic_elec.csv`, and `atlantic_storms.csv`) auditing the Stochary-Conjectural & Cybernetic Synthesis.
#[derive(Debug, Clone, PartialEq)]
pub struct EmpiricalStocharyCyberneticAudit {
    /// **`djia_1885_2023.csv`:** annualized non-ergodic variance drain $\frac{1}{2}\sigma^2 \times 252$. Gated (`> 0.01`).
    pub djia_non_ergodic_variance_drain: f64,
    /// **`bank_calls.csv`:** variance ratio of an EWMA ($\alpha = 0.20$) buffer over daily call totals. Gated (`< 0.30`).
    pub bank_calls_low_pass_variance_ratio: f64,
    /// Informative: $(s, S) = (\mu + 0.45\sigma, \mu + 1.35\sigma)$ daily call-buffer thresholds.
    pub bank_calls_scarf_s_and_big_s: (f64, f64),
    /// **`danish_fire.csv`:** full-sample $(\mathrm{VaR}_{0.95}, \mathrm{CVaR}_{0.95})$ (identity: $\mathrm{CVaR} \ge \mathrm{VaR}$).
    pub danish_fire_var95_and_cvar95: (f64, f64),
    /// Chronological split (first third train, rest test): $\mathrm{CVaR}^{\text{test}}_{0.95} - \mathrm{CVaR}^{\text{train}}_{0.95}$ of the empirical
    /// estimate, and of the WDRO certificate $\mathrm{CVaR}^{\text{train}} + \varepsilon/(1-\beta)$ with $\varepsilon$ the $W_1$ distance between
    /// the two halves of the training window. Informative: the pre-registered Optimizer's Curse claim (`> 1.0`) is discarded at the
    /// observed chronological gap (see `discarded_findings`), which makes the certificate claim vacuous, so it is not gated either.
    pub danish_fire_naive_vs_wdro_disappointment: (f64, f64),
    /// **`vic_elec.csv`:** share of test-half excursions above the training $q_{95}$ whose excess energy is covered by a reserve of
    /// $6.5\,\mathrm{h} \times (q_{99} - q_{95})$ sized on the training half, for (first half -> second half, second half -> first half).
    /// Informative: the `>= 0.90` claim holds in the first direction by one excursion and fails in the second, so it is discarded.
    pub vic_elec_reserve_excursion_coverage: (f64, f64),
    /// **`atlantic_storms.csv`:** out-of-sample CORP Telos score $\mathrm{REL} - \mathrm{RES}$ of the second half recalibrated by the
    /// first-half isotonic map, and $\mathrm{UNC}_0$. Gated (`< 0` and `> 0.02`).
    pub atlantic_storms_telos_skill_vs_skopos_unc0: (f64, f64),
    /// Data-dependent claims that did not survive on the real datasets (Prohibition 4: recorded, never gated).
    pub discarded_findings: Vec<&'static str>,
    /// Whether all surviving real-data claims hold.
    pub empirical_stochary_verified: bool,
}

impl EmpiricalStocharyCyberneticAudit {
    /// Evaluates the Stochary-Conjectural & Cybernetic synthesis across the 5 real CSV datasets.
    pub fn evaluate_real_datasets(
        djia_returns: &[f64],
        bank_call_clusters: &[Vec<f64>],
        danish_fire_losses: &[f64],
        vic_elec_demand: &[f64],
        storm_forecasts: (&[f64], &[f64]),
    ) -> Self {
        // 1. DJIA 1885-2023: Non-ergodic variance drain (1/2)*sigma_annual^2
        let sigma_daily = sample_std(djia_returns);
        let djia_non_ergodic_variance_drain = 0.5 * sigma_daily * sigma_daily * 252.0;

        // 2. Bank Calls: low-pass stock smoothing across daily arrival totals
        let daily_totals: Vec<f64> = bank_call_clusters
            .iter()
            .map(|c| c.iter().sum::<f64>())
            .collect();
        let mut smoothed = Vec::with_capacity(daily_totals.len());
        let mut s_buf = mean(&daily_totals);
        for &d in &daily_totals {
            s_buf = 0.80 * s_buf + 0.20 * d;
            smoothed.push(s_buf);
        }
        let bank_calls_low_pass_variance_ratio =
            sample_variance(&smoothed) / sample_variance(&daily_totals).max(1e-9);
        let mu_calls = mean(&daily_totals);
        let sd_calls = sample_std(&daily_totals);
        let bank_calls_scarf_s_and_big_s = (mu_calls + 0.45 * sd_calls, mu_calls + 1.35 * sd_calls);

        // 3. Danish Fire: chronological split; the WDRO radius is the W1 drift observed inside the training window
        let beta = 0.95;
        let (var95, cvar95) = rockafellar_uryasev_cvar(danish_fire_losses, beta);
        let split = danish_fire_losses.len() / 3;
        let (train, test) = danish_fire_losses.split_at(split);
        let (_, cvar_train) = rockafellar_uryasev_cvar(train, beta);
        let (_, cvar_test) = rockafellar_uryasev_cvar(test, beta);
        let epsilon = empirical_wasserstein_1_grid(&train[..split / 2], &train[split / 2..], 1000);
        let naive_disappointment = cvar_test - cvar_train;
        let wdro_disappointment = cvar_test - (cvar_train + epsilon / (1.0 - beta));

        // 4. Victoria Electricity Demand: reserve sized on one half, tested on contiguous excursions of the other half
        let reserve_coverage = |train: &[f64], test: &[f64]| -> f64 {
            let q95_train = quantile(train, 0.95);
            let q99_train = quantile(train, 0.99);
            let reserve_mwh = 6.5 * (q99_train - q95_train).max(1.0);
            let (mut excursions, mut covered) = (0usize, 0usize);
            let mut excess_mwh = 0.0_f64;
            let mut in_excursion = false;
            for &d in test.iter().chain(std::iter::once(&f64::NEG_INFINITY)) {
                if d > q95_train {
                    excess_mwh += (d - q95_train) * 0.5;
                    in_excursion = true;
                } else if in_excursion {
                    excursions += 1;
                    if excess_mwh <= reserve_mwh {
                        covered += 1;
                    }
                    excess_mwh = 0.0;
                    in_excursion = false;
                }
            }
            covered as f64 / excursions.max(1) as f64
        };
        let (vic_first, vic_second) = vic_elec_demand.split_at(vic_elec_demand.len() / 2);
        let vic_elec_reserve_coverage = (
            reserve_coverage(vic_first, vic_second),
            reserve_coverage(vic_second, vic_first),
        );

        // 5. Atlantic Storms: isotonic recalibration fitted on the first half and applied out of sample to the second half
        let (probs, outcomes) = storm_forecasts;
        let storm_half = probs.len().min(outcomes.len()) / 2;
        let corp_train =
            CorpPavDecomposition::compute(&probs[..storm_half], &outcomes[..storm_half]);
        let mut iso_map: Vec<(f64, f64)> = probs[..storm_half]
            .iter()
            .copied()
            .zip(corp_train.recalibrated_probabilities_q.iter().copied())
            .collect();
        iso_map.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        let recalibrate = |p: f64| -> f64 {
            let idx = iso_map.partition_point(|&(x, _)| x <= p);
            iso_map[idx.saturating_sub(1)].1.clamp(1e-3, 1.0 - 1e-3)
        };
        let test_probs: Vec<f64> = probs[storm_half..outcomes.len().min(probs.len())]
            .iter()
            .map(|&p| recalibrate(p))
            .collect();
        let corp_test = CorpPavDecomposition::compute(
            &test_probs,
            &outcomes[storm_half..storm_half + test_probs.len()],
        );
        let telos_net_score = corp_test.rel_miscalibration - corp_test.res_resolution;
        let skopos_unc0 = corp_test.unc0_uncertainty;

        let mut discarded_findings = Vec::new();
        if naive_disappointment <= 1.0 {
            discarded_findings.push(
                "danish_fire: the training-window CVaR_95 underestimates the later CVaR_95 by more than 1.0 (Optimizer's Curse)",
            );
        }
        if wdro_disappointment > 0.0 {
            discarded_findings.push(
                "danish_fire: the WDRO certificate CVaR_train + W1/(1 - beta) covers the later CVaR_95",
            );
        }
        // The first-direction coverage clears 0.90 by one excursion and the swapped direction does not, so the claim is not robust.
        if vic_elec_reserve_coverage.0.min(vic_elec_reserve_coverage.1) < 0.90 {
            discarded_findings.push(
                "vic_elec: a 6.5 h x (q99 - q95) reserve sized on one half covers 90% of peak excursions of the other half in both directions",
            );
        }
        if telos_net_score >= 0.0 {
            discarded_findings.push(
                "atlantic_storms: first-half isotonic recalibration keeps positive net skill (REL - RES < 0) on the second half",
            );
        }

        let empirical_stochary_verified = djia_non_ergodic_variance_drain > 0.010
            && bank_calls_low_pass_variance_ratio < 0.30
            && cvar95 >= var95
            && skopos_unc0 > 0.02
            && telos_net_score < 0.0;

        Self {
            djia_non_ergodic_variance_drain,
            bank_calls_low_pass_variance_ratio,
            bank_calls_scarf_s_and_big_s,
            danish_fire_var95_and_cvar95: (var95, cvar95),
            danish_fire_naive_vs_wdro_disappointment: (naive_disappointment, wdro_disappointment),
            vic_elec_reserve_excursion_coverage: vic_elec_reserve_coverage,
            atlantic_storms_telos_skill_vs_skopos_unc0: (telos_net_score, skopos_unc0),
            discarded_findings,
            empirical_stochary_verified,
        }
    }
}
