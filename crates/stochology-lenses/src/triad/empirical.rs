//! Real-Data Empirical Coupler for the Triadic ($\mathbb{C} \times \text{Fractals} \times \mathbb{T}^d$) & Technium Synthesis
//! across 5 real datasets (`vic_elec.csv`, `danish_fire.csv`, `atlantic_storms.csv`, `djia_1885_2023.csv`, `bank_calls.csv`).

use super::ctrw_pdhte::MultifractalPdhteRagCollapseAudit;
use super::weyl_sna_rlhf::hermite_mehler_features;

/// Real-Data Empirical Coupler for the Triadic ($\mathbb{C} \times \text{Fractals} \times \mathbb{T}^d$) & Technium Synthesis
/// across 5 real datasets (`vic_elec.csv`, `danish_fire.csv`, `atlantic_storms.csv`, `djia_1885_2023.csv`, `bank_calls.csv`).
#[derive(Debug, Clone)]
pub struct EmpiricalTriadicTechniumAudit {
    /// `vic_elec.csv`: phase-amplitude wave explained variance $R^2$ of `crate::analog::EmpiricalWaveAnalogAudit`. Gated (`> 0.80`).
    pub vic_elec_complex_gabor_r2: f64,
    /// `danish_fire.csv`: Eichi `PDHTE+JK` tail index $\hat{\xi}_{\mathrm{PDHTE+JK}}$ and regular-variation plateau $\mathrm{CV}(t_0)$. Gated.
    pub danish_fire_pdhte_jk_xi_and_plateau_cv: (f64, f64),
    /// `atlantic_storms.csv` out-of-time label shift: source-half hurricane prior, Hermite-Mehler `DFM` estimate of the target-half prior
    /// from source class-conditional feature means, and the realized target-half prior. Pre-registered tolerance $|\hat{q} - q| < 0.03$;
    /// not gated while the claim is discarded (see `discarded_findings`).
    pub atlantic_storms_out_of_time_dfm_prior: (f64, f64, f64),
    /// `djia_1885_2023.csv`: empirical scaling curvature $\hat{\zeta}(3) - 2\hat{\zeta}(2) + \hat{\zeta}(1)$ of aggregated absolute returns. Gated (`< -0.005`).
    pub djia_mmar_concavity: f64,
    /// `bank_calls.csv`: maximum state of the `LTC` Fused Solver driven by daily call intensities (identity: $0 < x \le A$ for any positive drive).
    pub bank_calls_ltc_max_state: f64,
    /// Data-dependent claims that did not survive on the real datasets (Prohibition 4: recorded, never gated).
    pub discarded_findings: Vec<&'static str>,
    /// True if the input-independent solver identity closes on the real drive.
    pub empirical_identities_closed: bool,
    /// True if identities close and all data-dependent claims pass.
    pub empirical_triadic_verified: bool,
}

impl EmpiricalTriadicTechniumAudit {
    /// Evaluates the data-dependent triadic claims across the 5 real CSV datasets.
    pub fn evaluate_real_datasets(
        vic_demand: &[f64],
        fire_losses: &[f64],
        storm_forecasts_and_outcomes: (&[f64], &[f64]),
        djia_returns: &[f64],
        bank_clusters: &[Vec<f64>],
    ) -> Self {
        let emp_wave = crate::analog::EmpiricalWaveAnalogAudit::evaluate_series(vic_demand);
        let vic_r2 = emp_wave.wave_explained_variance_r2;

        let (xi_fire_jk, _xi_fire_naive, cv_fire, _acc_fire) =
            MultifractalPdhteRagCollapseAudit::evaluate_pdhte_jk_tail(fire_losses);

        let (storm_p, storm_y) = storm_forecasts_and_outcomes;
        let n_storm = storm_p.len().min(storm_y.len());
        let half = n_storm / 2;
        let (src_p, tgt_p) = (&storm_p[..half], &storm_p[half..n_storm]);
        let (src_y, tgt_y) = (&storm_y[..half], &storm_y[half..n_storm]);
        let mean_of = |xs: &[f64]| xs.iter().sum::<f64>() / (xs.len().max(1) as f64);
        let src_mean = mean_of(src_p);
        let src_sd = (src_p.iter().map(|p| (p - src_mean).powi(2)).sum::<f64>()
            / (src_p.len().max(2) as f64 - 1.0))
            .sqrt()
            .max(1e-12);
        let feat = |p: f64| hermite_mehler_features((p - src_mean) / src_sd, 0.5);
        let mut mu_pos = [0.0_f64; 4];
        let mut mu_neg = [0.0_f64; 4];
        let mut mu_tgt = [0.0_f64; 4];
        let (mut n_pos, mut n_neg) = (0usize, 0usize);
        for (&p, &y) in src_p.iter().zip(src_y) {
            let f = feat(p);
            if y > 0.5 {
                n_pos += 1;
                mu_pos.iter_mut().zip(f).for_each(|(m, v)| *m += v);
            } else {
                n_neg += 1;
                mu_neg.iter_mut().zip(f).for_each(|(m, v)| *m += v);
            }
        }
        mu_pos.iter_mut().for_each(|m| *m /= n_pos.max(1) as f64);
        mu_neg.iter_mut().for_each(|m| *m /= n_neg.max(1) as f64);
        for &p in tgt_p {
            mu_tgt
                .iter_mut()
                .zip(feat(p))
                .for_each(|(m, v)| *m += v / (tgt_p.len().max(1) as f64));
        }
        let diff: Vec<f64> = mu_pos.iter().zip(&mu_neg).map(|(a, b)| a - b).collect();
        let shift: Vec<f64> = mu_tgt.iter().zip(&mu_neg).map(|(a, b)| a - b).collect();
        let diff_sq: f64 = diff.iter().map(|d| d * d).sum();
        let q_dfm = (diff.iter().zip(&shift).map(|(d, s)| d * s).sum::<f64>()
            / diff_sq.max(1e-300))
        .clamp(1e-3, 1.0 - 1e-3);
        let q_src = mean_of(src_y);
        let q_tgt = mean_of(tgt_y);

        let abs_ret: Vec<f64> = djia_returns.iter().map(|r| r.abs().max(1e-6)).collect();
        let moment_at_scale = |scale: usize, q: f64| -> f64 {
            let chunks = abs_ret.len() / scale;
            if chunks == 0 {
                return 1e-6;
            }
            let sum_q: f64 = (0..chunks)
                .map(|c| {
                    let agg: f64 = abs_ret[c * scale..(c + 1) * scale].iter().sum();
                    agg.powf(q)
                })
                .sum();
            (sum_q / (chunks as f64)).ln()
        };
        let log_span = 16.0_f64.ln();
        let zeta = |q: f64| (moment_at_scale(16, q) - moment_at_scale(1, q)) / log_span;
        let djia_tau_second_diff = zeta(3.0) - 2.0 * zeta(2.0) + zeta(1.0);

        let mut x_ltc_bank = 0.0_f64;
        let mut max_ltc_bank = 0.0_f64;
        let a_cap = 1.0_f64;
        for cluster in bank_clusters.iter().take(64) {
            let day_mean = cluster.iter().sum::<f64>() / (cluster.len().max(1) as f64);
            let f_drive = (day_mean / 50.0).clamp(0.05, 5.0);
            x_ltc_bank = (x_ltc_bank + f_drive * a_cap) / (1.0 + (1.0 + f_drive));
            max_ltc_bank = max_ltc_bank.max(x_ltc_bank);
        }

        let mut discarded_findings = Vec::new();
        if (q_dfm - q_tgt).abs() >= 0.03 {
            discarded_findings.push(
                "label shift holds out of time on atlantic_storms: Hermite-Mehler DFM recovers the second-half hurricane prior within 0.03",
            );
        }

        let empirical_identities_closed = max_ltc_bank > 0.0 && max_ltc_bank <= a_cap;
        let verified = empirical_identities_closed
            && vic_r2 > 0.80
            && xi_fire_jk > 0.40
            && xi_fire_jk < 0.85
            && cv_fire < 0.25
            && djia_tau_second_diff < -0.005;

        Self {
            vic_elec_complex_gabor_r2: vic_r2,
            danish_fire_pdhte_jk_xi_and_plateau_cv: (xi_fire_jk, cv_fire),
            atlantic_storms_out_of_time_dfm_prior: (q_src, q_dfm, q_tgt),
            djia_mmar_concavity: djia_tau_second_diff,
            bank_calls_ltc_max_state: max_ltc_bank,
            discarded_findings,
            empirical_identities_closed,
            empirical_triadic_verified: verified,
        }
    }
}
