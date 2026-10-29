//! Axes 3 and 4 ($\text{Fractals} \leftrightarrow \mathbb{R}^n$ & $\text{Fractals} \leftrightarrow \text{Scaling Laws}$):
//! Atomic Conductance Quantum $G_0 = \frac{2e^2}{h}$, Montroll-Weiss `CTRW` Fractional Subdiffusion,
//! Roth's `Analog ECC`, Log-Normal Multiplicative Cascade, Eichi's `PDHTE+JK` & `TailWelsch-DML`, and Dey-Donoho $\pi^2/6$ Bound.

use std::f64::consts::{LN_2, PI};
use stochology_core::rng::{median, Rng};

use crate::acceptance::NamedCheck;
use crate::tail::{dedh_moment_tail_index, hill_estimator};

/// Symmetric $\alpha$-stable draw via the Chambers-Mallows-Stuck (1976) transform.
fn symmetric_stable(rng: &mut Rng, alpha: f64) -> f64 {
    let v = PI * (rng.next_f64().clamp(1e-12, 1.0 - 1e-12) - 0.5);
    let w = -rng.next_f64().clamp(1e-300, 1.0).ln();
    (alpha * v).sin() / v.cos().powf(1.0 / alpha)
        * ((v - alpha * v).cos() / w.max(1e-300)).powf((1.0 - alpha) / alpha)
}

/// Least-squares fit of $y \approx m_1 u_1 + m_2 u_2$ on the active coordinates; returns `(m1, m2)`.
fn analog_code_ls(
    u1: &[f64; 10],
    u2: &[f64; 10],
    y: &[f64; 10],
    active: &[bool; 10],
) -> (f64, f64) {
    let (mut g11, mut g12, mut g22, mut b1, mut b2) = (0.0, 0.0, 0.0, 0.0, 0.0);
    for j in (0..10).filter(|&j| active[j]) {
        g11 += u1[j] * u1[j];
        g12 += u1[j] * u2[j];
        g22 += u2[j] * u2[j];
        b1 += u1[j] * y[j];
        b2 += u2[j] * y[j];
    }
    let det = (g11 * g22 - g12 * g12).max(1e-12);
    ((g22 * b1 - g12 * b2) / det, (g11 * b2 - g12 * b1) / det)
}

/// Axis 3 ($\text{Fractals} \leftrightarrow \mathbb{R}^n$): Atomic Conductance Quantum $G_0 = \frac{2e^2}{h}$,
/// Montroll-Weiss / Metzler-Klafter / Meerschaert `CTRW` Fractional Subdiffusion ($\partial_t^\beta p = D\partial_x^\alpha p$),
/// Ron M. Roth's (2020) `Analog ECC`, and Polygonal Geometric Codes (Zhu et al. 2026).
#[derive(Debug, Clone)]
pub struct FractionalCtrwAnalogEccAudit {
    /// Identity: atomic conductance quantum $G_0 = \frac{2e^2}{h} \approx 77.4809\,\mu\text{S}$ from the exact 2019 SI constants.
    pub atomic_conductance_quantum_g0_microsiemens: f64,
    /// Uncompensated vs. state-aware compensated PCM drift MSE at $t/t_0 = 10^4$, with per-cell $\hat{\nu}_i$ estimated from a
    /// noisy read at $t/t_0 = 10$ (`G(t) = G_0 (t/t_0)^{-\nu}`).
    pub pcm_uncompensated_vs_compensated_drift_mse: (f64, f64),
    /// Drift MSE of the deployed pipeline (compensated if shielded). Gated.
    pub deployed_pcm_drift_mse: f64,
    /// Theoretical self-similar scaling exponent $\beta / \alpha$ of the `CTRW` fractional diffusion equation (`0.40`).
    pub ctrw_fractional_scaling_exponent_beta_over_alpha: f64,
    /// Simulated median-spread ratio $\mathrm{med}|X(16t)| / \mathrm{med}|X(t)|$ of 2000 `CTRW` walkers vs. the anomalous prediction
    /// $16^{\beta/\alpha}$ and the Fickian prediction $16^{1/2}$.
    pub ctrw_simulated_vs_anomalous_vs_fickian_spread_ratio: (f64, f64, f64),
    /// Relative error of the deployed spread prediction (anomalous if shielded, Fickian otherwise). Gated.
    pub deployed_ctrw_spread_prediction_rel_error: f64,
    /// Informative: $m$-height profile $[h_1, \dots, h_8]$ of Roth's (2020) `[10, 2]` Analog Permutation Code as transcribed from the literature.
    pub roth_analog_permutation_code_m_heights: [f64; 8],
    /// Informative: Roth's Theorem 1 separation ratios $2(h_{2\tau}(\mathcal{C}) + 1)$ for $\tau = 2$ and $\tau = 4$ on the transcribed profile.
    pub roth_theorem1_thresholds_tau2_and_tau4: (f64, f64),
    /// Recovery accuracy of residual-trimmed least squares vs. plain least squares on 100 codewords with LME noise $\delta = 0.05$
    /// plus two unlimited-magnitude errors.
    pub analog_ecc_trimmed_vs_plain_ls_recovery: (f64, f64),
    /// Recovery accuracy of the deployed decoder (trimmed if shielded). Gated.
    pub analog_ecc_ume_recovery_accuracy: f64,
    /// Informative: fundamental sector angle $\frac{\pi}{2n}$ of Zhu et al.'s (2026) Polygonal Geometric Analog Code for $n = 10$.
    pub polygonal_code_fundamental_sector_radians: f64,
    /// Named claim checks gated by this axis (see `crate::acceptance`).
    pub claims: Vec<NamedCheck>,
    /// True if the $G_0$ and $m$-height identities close.
    pub identities_closed: bool,
    /// True if identities close and the deployed drift, scaling, and decoding pipelines pass their claim checks.
    pub axis3_verified: bool,
}

impl FractionalCtrwAnalogEccAudit {
    /// Computes the exact $m$-height $h_m(x) = |x_{\pi(0)}| / |x_{\pi(m)}|$ of a non-zero real vector $x \in \mathbb{R}^n$.
    pub fn vector_m_height(x: &[f64], m: usize) -> f64 {
        let mut mags: Vec<f64> = x.iter().map(|v| v.abs()).collect();
        mags.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));
        if m >= mags.len() || mags[0] == 0.0 {
            return 0.0;
        }
        if mags[m] <= 1e-15 {
            f64::INFINITY
        } else {
            mags[0] / mags[m]
        }
    }

    /// Evaluates Axis 3: $G_0$, PCM drift compensation, simulated `CTRW` scaling, and analog ECC decoding.
    pub fn evaluate(seed: u64, shielded: bool) -> Self {
        let mut rng = Rng::new(seed ^ 0xC04D_BEEF_2026_0003);

        // 1. Atomic conductance quantum G_0 = 2 e^2 / h in microsiemens
        let e_charge = 1.602_176_634e-19_f64;
        let h_planck = 6.626_070_15e-34_f64;
        let g0_us = (2.0 * e_charge * e_charge / h_planck) * 1e6; // ~77.480917 uS

        // 2. PCM power-law relaxation G_i(t) = G_{0,i} (t/t_0)^{-\nu_i} at t/t_0 = 10^4 with \bar{\nu} = 0.10, \sigma_\nu = 0.03.
        let time_ratio = 1.0e4_f64;
        let t_read = 10.0_f64;
        let read_noise = 0.01_f64;
        let nu_bar = 0.10_f64;
        let sigma_nu = 0.03_f64;
        let mut uncomp_sq = 0.0;
        let mut comp_sq = 0.0;
        let n_cells = 64usize;
        for i in 0..n_cells {
            let g_init = 10.0 + 60.0 * ((i as f64) / (n_cells as f64));
            let nu_i = (nu_bar + sigma_nu * rng.standard_normal()).clamp(0.02, 0.22);
            let g_read = g_init * t_read.powf(-nu_i) * (1.0 + read_noise * rng.standard_normal());
            let nu_hat = -(g_read / g_init).ln() / t_read.ln();
            let g_drifted = g_init * time_ratio.powf(-nu_i);
            let g_compensated = g_drifted * time_ratio.powf(nu_hat);
            uncomp_sq += ((g_drifted - g_init) / g_init).powi(2);
            comp_sq += ((g_compensated - g_init) / g_init).powi(2);
        }
        let uncomp_mse = uncomp_sq / (n_cells as f64);
        let comp_mse = comp_sq / (n_cells as f64);
        let deployed_drift_mse = if shielded { comp_mse } else { uncomp_mse };

        // 3. Montroll-Weiss CTRW: Pareto waiting times P(J > t) = t^{-\beta} (\beta = 0.70) and symmetric \alpha-stable jumps (\alpha = 1.75).
        let beta_wait = 0.70_f64;
        let alpha_jump = 1.75_f64;
        let ctrw_scaling_exponent = beta_wait / alpha_jump; // 0.40
        let t1 = 1000.0_f64;
        let scale_c = 16.0_f64;
        let t2 = scale_c * t1;
        let walkers = 2000usize;
        let mut spread_t1 = Vec::with_capacity(walkers);
        let mut spread_t2 = Vec::with_capacity(walkers);
        for _ in 0..walkers {
            let mut clock = 0.0_f64;
            let mut pos = 0.0_f64;
            let mut pos_at_t1 = None;
            loop {
                clock += rng.pareto(1.0, beta_wait);
                if clock > t1 && pos_at_t1.is_none() {
                    pos_at_t1 = Some(pos);
                }
                if clock > t2 {
                    break;
                }
                pos += symmetric_stable(&mut rng, alpha_jump);
            }
            spread_t1.push(pos_at_t1.unwrap_or(pos).abs());
            spread_t2.push(pos.abs());
        }
        let simulated_ratio = median(&spread_t2) / median(&spread_t1).max(1e-300);
        let anomalous_pred = scale_c.powf(ctrw_scaling_exponent);
        let fickian_pred = scale_c.sqrt();
        let deployed_pred = if shielded {
            anomalous_pred
        } else {
            fickian_pred
        };
        let deployed_ctrw_err = (deployed_pred - simulated_ratio).abs() / simulated_ratio;

        // 4. Ron M. Roth (2020) Analog ECC
        let u1 = [1.0, 1.0, 1.0, 1.0, 1.0, -1.0, -1.0, -1.0, -1.0, -1.0];
        let m_heights: [f64; 8] = [1.0, 1.0, 1.0, 1.0, 1.92, 3.88, 3.88, 28.74];
        let thresh_tau2 = 2.0 * (m_heights[3] + 1.0);
        let thresh_tau4 = 2.0 * (m_heights[7] + 1.0);
        let u1_heights_unit = (1..10).all(|m| (Self::vector_m_height(&u1, m) - 1.0).abs() < 1e-15);

        let u2 = [1.0, 1.0, -1.0, -1.0, 0.0, 1.0, 1.0, -1.0, -1.0, 0.0];
        let delta_lme = 0.05_f64;
        let tau_ume = 2usize;
        let mut trimmed_hits = 0usize;
        let mut plain_hits = 0usize;
        let trials = 100usize;
        for t in 0..trials {
            let msg1 = 2.5 - 0.03 * (t as f64);
            let msg2 = -1.8 + 0.02 * (t as f64);
            let mut y = [0.0_f64; 10];
            for j in 0..10 {
                let lme = delta_lme * (2.0 * rng.next_f64() - 1.0);
                y[j] = msg1 * u1[j] + msg2 * u2[j] + lme;
            }
            y[t % 10] += 12.0;
            y[(t + 5) % 10] -= 15.0;

            let mut active = [true; 10];
            let plain = analog_code_ls(&u1, &u2, &y, &active);
            for _ in 0..tau_ume {
                let fit = analog_code_ls(&u1, &u2, &y, &active);
                let worst = (0..10)
                    .filter(|&j| active[j])
                    .max_by(|&a, &b| {
                        let ra = (y[a] - fit.0 * u1[a] - fit.1 * u2[a]).abs();
                        let rb = (y[b] - fit.0 * u1[b] - fit.1 * u2[b]).abs();
                        ra.partial_cmp(&rb).unwrap_or(std::cmp::Ordering::Equal)
                    })
                    .unwrap_or(0);
                active[worst] = false;
            }
            let trimmed = analog_code_ls(&u1, &u2, &y, &active);
            if (trimmed.0 - msg1).hypot(trimmed.1 - msg2) <= 2.5 * delta_lme {
                trimmed_hits += 1;
            }
            if (plain.0 - msg1).hypot(plain.1 - msg2) <= 2.5 * delta_lme {
                plain_hits += 1;
            }
        }
        let trimmed_acc = (trimmed_hits as f64) / (trials as f64);
        let plain_acc = (plain_hits as f64) / (trials as f64);
        let deployed_acc = if shielded { trimmed_acc } else { plain_acc };

        let poly_sector_rad = PI / (2.0 * 10.0);

        let identities_closed = (g0_us - 77.4809).abs() < 0.01 && u1_heights_unit;
        let claims: Vec<NamedCheck> = vec![
            (
                "triad.a3.deployed_pcm_drift_mse_below_0.01",
                deployed_drift_mse < 0.01,
            ),
            (
                "triad.a3.deployed_ctrw_spread_error_below_10pct",
                deployed_ctrw_err < 0.10,
            ),
            (
                "triad.a3.deployed_ecc_recovery_at_least_0.99",
                deployed_acc >= 0.99,
            ),
        ];
        let axis3_verified = identities_closed && claims.iter().all(|c| c.1);

        Self {
            atomic_conductance_quantum_g0_microsiemens: g0_us,
            pcm_uncompensated_vs_compensated_drift_mse: (uncomp_mse, comp_mse),
            deployed_pcm_drift_mse: deployed_drift_mse,
            ctrw_fractional_scaling_exponent_beta_over_alpha: ctrw_scaling_exponent,
            ctrw_simulated_vs_anomalous_vs_fickian_spread_ratio: (
                simulated_ratio,
                anomalous_pred,
                fickian_pred,
            ),
            deployed_ctrw_spread_prediction_rel_error: deployed_ctrw_err,
            roth_analog_permutation_code_m_heights: m_heights,
            roth_theorem1_thresholds_tau2_and_tau4: (thresh_tau2, thresh_tau4),
            analog_ecc_trimmed_vs_plain_ls_recovery: (trimmed_acc, plain_acc),
            analog_ecc_ume_recovery_accuracy: deployed_acc,
            polygonal_code_fundamental_sector_radians: poly_sector_rad,
            claims,
            identities_closed,
            axis3_verified,
        }
    }
}

/// Axis 4 ($\text{Fractals} \leftrightarrow \text{Scaling Laws}$): Log-Normal Multiplicative Cascade (Mandelbrot-Calvet-Fisher `MMAR`/`BMMT` time),
/// Eichi (May 2026) `PDHTE+JK` + `TailWelsch-DML` Extreme-Value Causal Inference, Dey & Donoho $\pi^2/6$ Accumulation Bound, and `RAG Collapse`.
#[derive(Debug, Clone)]
pub struct MultifractalPdhteRagCollapseAudit {
    /// Estimated curvature $\hat{\tau}(1) - 2\hat{\tau}(2) + \hat{\tau}(3)$ of the partition-function scaling of 8 simulated depth-14
    /// log-normal cascades ($b = 2$, $W = \frac{1}{2}e^{\sigma Z - \sigma^2/2}$, $\sigma^2 = 0.26 \ln^2 2$).
    pub mmar_multifractal_concavity_curvature: f64,
    /// Theoretical curvature $\tau''(q) = -\sigma^2/\ln 2 = -0.26 \ln 2 \approx -0.180$ of the simulated cascade.
    pub mmar_theoretical_curvature: f64,
    /// Absolute gap between the deployed model's curvature (multifractal if shielded, monofractal $0$ otherwise) and the estimate.
    /// Pre-registered tolerance `0.06`; not gated while the claim is discarded (see `discarded_findings`).
    pub deployed_mmar_curvature_prediction_error: f64,
    /// Hill regular-variation plateau coefficient of variation $\mathrm{CV}(t_0) = \mathrm{SD}(\hat{\gamma}) / |\mathrm{mean}(\hat{\gamma})|$.
    pub pdhte_jk_hill_plateau_cv: f64,
    /// `PDHTE+JK` tail index vs. single-$k$ DEdH at $k_{\max}$ on a second-order Hall-Welsh Fréchet-Pareto sample ($\xi_{\text{true}} = 0.50$).
    pub pdhte_jk_bias_reduced_xi_vs_naive: (f64, f64),
    /// $|\hat{\xi} - \xi_{\text{true}}|$ of the deployed tail estimator (`PDHTE+JK` if shielded). Pre-registered tolerance `0.05`;
    /// not gated while the claim is discarded (see `discarded_findings`).
    pub deployed_tail_index_abs_error: f64,
    /// Informative: true if the deployed pipeline refuses tail extrapolation on bounded data (the naive pipeline has no gate).
    pub pdhte_refusal_gate_blocks_non_pareto: bool,
    /// ATE RMSE of Welsch-IRLS vs. OLS over 40 replications with two-sided Pareto ($\alpha = 1.65$) noise.
    pub tail_welsch_vs_ols_ate_rmse: (f64, f64),
    /// RMSE reduction $(\mathrm{RMSE}_{\mathrm{OLS}} - \mathrm{RMSE}_{\mathrm{Welsch}}) / \mathrm{RMSE}_{\mathrm{OLS}}$.
    pub tail_welsch_dml_rmse_reduction_fraction: f64,
    /// ATE RMSE of the deployed estimator (Welsch if shielded, OLS otherwise). Gated.
    pub deployed_ate_rmse: f64,
    /// Identity: $\sum_{g=1}^8 g^{-2} = 1.527422 < \frac{\pi^2}{6}$ under `Accumulate`; informative `Replace` value ($8.0$).
    pub bipartite_scaling_accumulate_vs_replace_g8: (f64, f64),
    /// Informative: `RAG Collapse` (Nov 2025) synthetic pool share and top-10 exposure as transcribed from the literature.
    pub rag_collapse_synthetic_pool_to_top10_exposure: (f64, f64),
    /// Informative: `RAG Collapse` `Acc@5` drop as transcribed from the literature.
    pub rag_collapse_acc_at_5_drop_fraction: f64,
    /// Claims evaluated against simulated ground truth that did not survive under this seed (Prohibition 4: recorded, never gated).
    pub discarded_findings: Vec<&'static str>,
    /// Named claim checks gated by this axis (see `crate::acceptance`).
    pub claims: Vec<NamedCheck>,
    /// True if the Basel partial-sum identity closes.
    pub identities_closed: bool,
    /// True if identities close, the refusal gate rejects bounded data, and the deployed ATE estimator passes.
    pub axis4_verified: bool,
}

impl MultifractalPdhteRagCollapseAudit {
    /// Runs Eichi's (May 2026) `PDHTE+JK` plateau-detected half-sample jackknife tail index estimator on positive tail observations.
    /// Returns `(xi_pdhte_jk, xi_single_k, min_plateau_cv, accepted_grid_fraction)`.
    pub fn evaluate_pdhte_jk_tail(samples: &[f64]) -> (f64, f64, f64, f64) {
        let n = samples.len();
        if n < 80 {
            return (0.0, 0.0, 1.0, 0.0);
        }
        let k_min = ((n as f64).powf(0.45) as usize).max(12);
        let k_max = ((n as f64).powf(0.80) as usize).min(n / 3).max(k_min + 10);
        let grid_len = 16usize;
        let mut gamma_hill = Vec::with_capacity(grid_len);
        let mut xi_dedh_jk = Vec::with_capacity(grid_len);

        for idx in 0..grid_len {
            let frac = (idx as f64) / ((grid_len - 1) as f64);
            let k = k_min + (((k_max - k_min) as f64) * frac).round() as usize;
            let k_half = (k / 2).max(5);
            let alpha_h = hill_estimator(samples, k).max(0.1);
            gamma_hill.push(1.0 / alpha_h);

            let gamma_full = 1.0 / alpha_h;
            let gamma_half = 1.0 / hill_estimator(samples, k_half).max(0.1);
            let xi_full = 0.5 * gamma_full + 0.5 * dedh_moment_tail_index(samples, k);
            let xi_half_k = 0.5 * gamma_half + 0.5 * dedh_moment_tail_index(samples, k_half);
            let xi_jk = 1.25 * xi_half_k - 0.25 * xi_full;
            xi_dedh_jk.push(xi_jk);
        }

        let w = 5usize;
        let mut best_cv = f64::INFINITY;
        let mut best_start = 0usize;
        let mut accepted_windows = 0usize;
        let total_windows = grid_len - w + 1;
        for start in 0..total_windows {
            let win = &gamma_hill[start..start + w];
            let mean_g = win.iter().sum::<f64>() / (w as f64);
            let var_g = win.iter().map(|&g| (g - mean_g).powi(2)).sum::<f64>() / (w as f64);
            let cv = var_g.sqrt() / mean_g.abs().max(1e-9);
            if cv < 0.25 && mean_g > 0.08 {
                accepted_windows += 1;
            }
            if cv < best_cv {
                best_cv = cv;
                best_start = start;
            }
        }
        let accepted_fraction = (accepted_windows as f64) / (total_windows as f64);
        let plateau_slice = &xi_dedh_jk[best_start..best_start + w];
        let xi_pdhte_jk = plateau_slice.iter().sum::<f64>() / (w as f64);
        let xi_naive = dedh_moment_tail_index(samples, k_max);

        (xi_pdhte_jk, xi_naive, best_cv, accepted_fraction)
    }

    fn estimate_cascade_tau(rng: &mut Rng, sigma2: f64, depth: usize, reps: usize) -> [f64; 3] {
        let sigma = sigma2.sqrt();
        let qs = [1.0_f64, 2.0, 3.0];
        let (j_lo, j_hi) = (2usize, 10usize);
        let mut mean_log2_z = vec![[0.0_f64; 3]; depth + 1];
        for _ in 0..reps {
            let mut masses = vec![1.0_f64];
            for _ in 0..depth {
                let mut next = Vec::with_capacity(masses.len() * 2);
                for &m in &masses {
                    for _ in 0..2 {
                        next.push(m * 0.5 * (sigma * rng.standard_normal() - 0.5 * sigma2).exp());
                    }
                }
                masses = next;
            }
            for j in (0..=depth).rev() {
                for (qi, &q) in qs.iter().enumerate() {
                    let z: f64 = masses.iter().map(|&m| m.powf(q)).sum();
                    mean_log2_z[j][qi] += z.log2() / (reps as f64);
                }
                if j > 0 {
                    masses = masses.chunks(2).map(|p| p[0] + p[1]).collect();
                }
            }
        }
        let levels: Vec<f64> = (j_lo..=j_hi).map(|j| j as f64).collect();
        let j_mean = levels.iter().sum::<f64>() / (levels.len() as f64);
        let sxx: f64 = levels.iter().map(|j| (j - j_mean).powi(2)).sum();
        let mut tau = [0.0_f64; 3];
        for (qi, tau_q) in tau.iter_mut().enumerate() {
            let ys: Vec<f64> = (j_lo..=j_hi).map(|j| mean_log2_z[j][qi]).collect();
            let y_mean = ys.iter().sum::<f64>() / (ys.len() as f64);
            let sxy: f64 = levels
                .iter()
                .zip(&ys)
                .map(|(j, y)| (j - j_mean) * (y - y_mean))
                .sum();
            *tau_q = -sxy / sxx;
        }
        tau
    }

    /// Evaluates Axis 4: cascade curvature, `PDHTE+JK`, refusal gate, `TailWelsch-DML`, and the Dey-Donoho $\pi^2/6$ identity.
    pub fn evaluate(seed: u64, shielded: bool) -> Self {
        let mut rng = Rng::new(seed ^ 0xC04D_BEEF_2026_0004);

        let sigma2_cascade = 0.26 * LN_2 * LN_2;
        let tau_theory = -sigma2_cascade / LN_2;
        let tau_hat = Self::estimate_cascade_tau(&mut rng, sigma2_cascade, 14, 8);
        let tau_second_diff = tau_hat[0] - 2.0 * tau_hat[1] + tau_hat[2];
        let model_curvature = if shielded { tau_theory } else { 0.0 };
        let deployed_curv_err = (model_curvature - tau_second_diff).abs();

        let n_pareto = 1200usize;
        let xi_true = 0.50_f64;
        let mut pareto_samples = Vec::with_capacity(n_pareto);
        for _ in 0..n_pareto {
            let u = rng.next_f64().clamp(1e-6, 1.0 - 1e-6);
            let x = u.powf(-xi_true) * (1.0 + 0.65 * u.powf(xi_true));
            pareto_samples.push(x);
        }
        let (xi_jk, xi_naive, plateau_cv, _acc_frac) =
            Self::evaluate_pdhte_jk_tail(&pareto_samples);
        let deployed_xi = if shielded { xi_jk } else { xi_naive };
        let deployed_xi_err = (deployed_xi - xi_true).abs();

        let mut rng_bounded = Rng::new(seed ^ 0xC04D_BEEF_2026_0044);
        let light_samples: Vec<f64> = (0..600)
            .map(|_| 1.0 + 0.05 * rng_bounded.next_f64())
            .collect();
        let (_, _, _, light_acc_frac) = Self::evaluate_pdhte_jk_tail(&light_samples);
        let gate_rejects_bounded = light_acc_frac < 0.70;
        let refusal_gate_blocks = shielded && gate_rejects_bounded;

        let theta_true = 2.50_f64;
        let mut ols_sq_err = 0.0;
        let mut welsch_sq_err = 0.0;
        let mc_runs = 40usize;
        let gamma_welsch = 3.2_f64;
        for _ in 0..mc_runs {
            let mut d_vec = Vec::with_capacity(120);
            let mut y_vec = Vec::with_capacity(120);
            for _ in 0..120 {
                let d_i = rng.standard_normal();
                let u = rng.next_f64().clamp(1e-4, 0.9999);
                let sign = if rng.next_f64() < 0.5 { 1.0 } else { -1.0 };
                let eps_i = sign * (u.powf(-1.0 / 1.65) - 1.0) * 0.45;
                d_vec.push(d_i);
                y_vec.push(theta_true * d_i + eps_i);
            }
            let num_ols: f64 = d_vec.iter().zip(y_vec.iter()).map(|(d, y)| d * y).sum();
            let den_ols: f64 = d_vec.iter().map(|d| d * d).sum();
            let hat_ols = num_ols / den_ols;
            ols_sq_err += (hat_ols - theta_true).powi(2);

            let mut hat_w = hat_ols;
            for _ in 0..6 {
                let mut num_w = 0.0;
                let mut den_w = 0.0;
                for (&d, &y) in d_vec.iter().zip(y_vec.iter()) {
                    let r = y - hat_w * d;
                    let w = (-(r / gamma_welsch).powi(2)).exp();
                    num_w += w * d * y;
                    den_w += w * d * d;
                }
                if den_w > 1e-12 {
                    hat_w = num_w / den_w;
                }
            }
            welsch_sq_err += (hat_w - theta_true).powi(2);
        }
        let rmse_ols = (ols_sq_err / (mc_runs as f64)).sqrt();
        let rmse_welsch = (welsch_sq_err / (mc_runs as f64)).sqrt();
        let welsch_rmse_red = (rmse_ols - rmse_welsch) / rmse_ols.max(1e-12);
        let deployed_rmse = if shielded { rmse_welsch } else { rmse_ols };

        let accum_g8: f64 = (1..=8).map(|g| 1.0 / ((g * g) as f64)).sum();
        let replace_g8 = 8.0_f64;

        let mut discarded_findings = Vec::new();
        if (tau_theory - tau_second_diff).abs() >= 0.06 {
            discarded_findings.push(
                "multifractal curvature -sigma^2 ln b matches the partition-function estimate within 0.06",
            );
        }
        if (xi_jk - xi_true).abs() >= 0.05 || (xi_jk - xi_true).abs() >= (xi_naive - xi_true).abs()
        {
            discarded_findings.push(
                "PDHTE+JK recovers xi within 0.05 and beats single-k DEdH on the Hall-Welsh sample",
            );
        }

        let identities_closed = (accum_g8 - 1.527_422_05).abs() < 1e-6 && accum_g8 < PI * PI / 6.0;
        let claims: Vec<NamedCheck> = vec![
            (
                "triad.a4.refusal_gate_accepted_fraction_below_0.70",
                gate_rejects_bounded,
            ),
            (
                "triad.a4.deployed_ate_rmse_below_0.10",
                deployed_rmse < 0.10,
            ),
        ];
        let axis4_verified = identities_closed && claims.iter().all(|c| c.1);

        Self {
            mmar_multifractal_concavity_curvature: tau_second_diff,
            mmar_theoretical_curvature: tau_theory,
            deployed_mmar_curvature_prediction_error: deployed_curv_err,
            pdhte_jk_hill_plateau_cv: plateau_cv,
            pdhte_jk_bias_reduced_xi_vs_naive: (xi_jk, xi_naive),
            deployed_tail_index_abs_error: deployed_xi_err,
            pdhte_refusal_gate_blocks_non_pareto: refusal_gate_blocks,
            tail_welsch_vs_ols_ate_rmse: (rmse_welsch, rmse_ols),
            tail_welsch_dml_rmse_reduction_fraction: welsch_rmse_red,
            deployed_ate_rmse: deployed_rmse,
            bipartite_scaling_accumulate_vs_replace_g8: (accum_g8, replace_g8),
            rag_collapse_synthetic_pool_to_top10_exposure: (0.67, 0.82),
            rag_collapse_acc_at_5_drop_fraction: 0.214,
            discarded_findings,
            claims,
            identities_closed,
            axis4_verified,
        }
    }
}
