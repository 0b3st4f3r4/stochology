//! Real-Data Coupling Audit (`EmpiricalWaveAnalogAudit`) for Physical, Analog, and Wave-Interference Computing.

use crate::analog::solvers::solve_pku_hp_inv_with_solution;
use stochology_core::{mean, sample_variance};

/// Real-Data Coupling Audit (`labs --real-data`): Applies Gibram Raul's Phase-Amplitude Wave Superposition (`SIREN`)
/// and Peking University's `HP-INV` 24-Bit RRAM Iterative Matrix Solver to empirical time series from `data/*.csv`.
#[derive(Debug, Clone, PartialEq)]
pub struct EmpiricalWaveAnalogAudit {
    /// Number of raw time-series observations compressed via phase-and-amplitude wave superposition.
    pub empirical_sample_count: usize,
    /// Number of continuous wave harmonics $M$ ($3M$ scalar parameters: amplitude, frequency, phase).
    pub wave_harmonics_count: usize,
    /// Parameter compression ratio $N / (3M)$ over raw point-by-point storage.
    pub empirical_compression_ratio: f64,
    /// Fraction of empirical variance $R^2_{\text{wave}}$ captured by the phase-and-amplitude wave representation.
    pub wave_explained_variance_r2: f64,
    /// Relative residual error $\lVert G w - c\rVert_2 / \lVert c\rVert_2$ when solving the $16\times 16$ normal equations
    /// of the empirical wave basis via PKU's 3-bit RRAM `HP-INV` iterative refinement ($< 10^{-7}$).
    pub empirical_hp_inv_relative_error: f64,
    /// Whether the empirical wave compression and `HP-INV` normal-equation precision pass audit.
    pub empirical_wave_verified: bool,
}

impl EmpiricalWaveAnalogAudit {
    /// Fits a $16$-component sinusoidal wave basis to an empirical series (e.g., `vic_elec.csv` half-hourly electricity demand)
    /// and solves its $16\times 16$ Gram normal equations using PKU's bit-sliced `HP-INV` algorithm.
    pub fn evaluate_series(series: &[f64]) -> Self {
        let n_take = series.len().min(336);
        let slice = &series[..n_take];
        let mu = mean(slice);
        let sigma = sample_variance(slice).sqrt().max(1e-9);
        let centered: Vec<f64> = slice.iter().map(|&v| (v - mu) / sigma).collect();

        let freqs = [1.0, 2.0, 3.0, 7.0, 14.0, 21.0, 4.0, 6.0];
        let n_basis = 16usize;
        let mut phi_mat = vec![vec![0.0; n_basis]; n_take];
        for (t, row) in phi_mat.iter_mut().enumerate() {
            let theta = 2.0 * std::f64::consts::PI * (t as f64 / n_take as f64);
            for (k, &f) in freqs.iter().enumerate() {
                row[2 * k] = (f * theta).sin();
                row[2 * k + 1] = (f * theta).cos();
            }
        }

        let mut gram = vec![vec![0.0; n_basis]; n_basis];
        let mut rhs = vec![0.0; n_basis];
        for t in 0..n_take {
            for i in 0..n_basis {
                rhs[i] += phi_mat[t][i] * centered[t] / (n_take as f64);
                for j in 0..n_basis {
                    gram[i][j] += phi_mat[t][i] * phi_mat[t][j] / (n_take as f64);
                }
            }
        }
        for (i, row) in gram.iter_mut().enumerate() {
            row[i] += 1e-3;
        }

        let (_init_err, empirical_hp_inv_relative_error, _iters, weights) =
            solve_pku_hp_inv_with_solution(&gram, &rhs, n_basis, 10);

        let mut ss_res = 0.0;
        let mut ss_tot = 0.0;
        for t in 0..n_take {
            let mut pred = 0.0;
            for i in 0..n_basis {
                pred += phi_mat[t][i] * weights[i];
            }
            ss_res += (centered[t] - pred).powi(2);
            ss_tot += centered[t].powi(2);
        }
        let wave_explained_variance_r2 = (1.0 - ss_res / ss_tot.max(1e-12)).clamp(0.0, 1.0);
        let wave_harmonics_count = freqs.len();
        let empirical_compression_ratio = n_take as f64 / (wave_harmonics_count * 3) as f64;
        let empirical_wave_verified = empirical_compression_ratio >= 10.0
            && wave_explained_variance_r2 >= 0.55
            && empirical_hp_inv_relative_error < 1e-7;

        Self {
            empirical_sample_count: n_take,
            wave_harmonics_count,
            empirical_compression_ratio,
            wave_explained_variance_r2,
            empirical_hp_inv_relative_error,
            empirical_wave_verified,
        }
    }
}
