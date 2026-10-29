//! Axes 1 and 2 ($\mathbb{C}$ & $\mathbb{C} \leftrightarrow \text{Fractals}$):
//! Complex Gabor Wavelet INRs (`WIRE` with `LSSA`-style frequency learning, `Improved SIREN`) on the 2D Helmholtz PDE,
//! Hughes-Minkov-Shi-Fan Adjoint *In-Situ* Photonic Backpropagation, Algebraic Complex-to-Real Matrix Isomorphism,
//! Two-Stage `BlockAMC`, and Hackbusch's Hierarchical Matrices ($\mathcal{H}$-Matrices).

use std::f64::consts::PI;
use stochology_core::math::complex::Complex64;
use stochology_core::math::linalg::solve_linear_system;
use stochology_core::rng::Rng;

use crate::acceptance::NamedCheck;

/// Relative squared error $\lVert a - t\rVert^2 / \lVert t\rVert^2$ between two complex fields.
fn relative_sq_error(approx: &[Complex64], target: &[Complex64]) -> f64 {
    let num: f64 = approx
        .iter()
        .zip(target)
        .map(|(&a, &t)| (a - t).norm_sq())
        .sum();
    let den: f64 = target.iter().map(|t| t.norm_sq()).sum();
    num / den.max(1e-300)
}

/// Discrete Helmholtz operator $(\nabla_h^2 + k^2)$ (5-point Laplacian) on the interior nodes of an $n \times n$ grid with spacing $h$.
fn discrete_helmholtz(field: &[Complex64], n: usize, h: f64, k: f64) -> Vec<Complex64> {
    let inv_h2 = 1.0 / (h * h);
    let mut out = Vec::with_capacity((n - 2) * (n - 2));
    for i in 1..n - 1 {
        for j in 1..n - 1 {
            let c = field[i * n + j];
            let nb = [
                field[(i + 1) * n + j],
                field[(i - 1) * n + j],
                field[i * n + j + 1],
                field[i * n + j - 1],
            ];
            let lap_re = (nb.iter().map(|v| v.re).sum::<f64>() - 4.0 * c.re) * inv_h2;
            let lap_im = (nb.iter().map(|v| v.im).sum::<f64>() - 4.0 * c.im) * inv_h2;
            out.push(Complex64::new(lap_re + k * k * c.re, lap_im + k * k * c.im));
        }
    }
    out
}

/// Axis 1 ($\mathbb{C}$): Complex Phase-Amplitude Wave Physics, Complex Gabor Wavelet INRs (`WIRE` with `LSSA`-style frequency learning,
/// `Improved SIREN`) on the 2D Helmholtz PDE $(\nabla^2 + k^2)\Phi(x,y) = q(x,y)$, and Hughes-Minkov-Shi-Fan (*Optica* 2018) Adjoint *In-Situ* Photonic Backpropagation.
#[derive(Debug, Clone)]
pub struct ComplexWaveGaborHelmholtzAudit {
    /// Helmholtz wavenumber $k$ on $[-1, 1]^2$.
    pub helmholtz_wavenumber_k: f64,
    /// Relative Helmholtz residual $\lVert(\nabla_h^2 + k^2)\hat{\Phi} - q_h\rVert^2 / \lVert q_h\rVert^2$ of the least-squares fit by a
    /// spectrally-biased real `tanh` dictionary (naive baseline, always computed).
    pub tanh_mlp_helmholtz_residual_mse: f64,
    /// Relative Helmholtz residual of the complex Gabor atom $\psi(x) = \exp(i\hat{\omega} x - |s_0 r|^2)$ (`WIRE`) at the learned frequency (always computed).
    pub wire_complex_gabor_helmholtz_residual_mse: f64,
    /// Relative frequency error $|\hat{\omega}_{\mathrm{LSSA}} - k| / k$ after 60 gradient-ascent steps from $\omega_0 = k/2$.
    pub lssa_learned_frequency_relative_error: f64,
    /// Ratio of the `tanh` residual to the Gabor residual.
    pub tanh_to_lssa_helmholtz_loss_ratio: f64,
    /// Helmholtz residual of the deployed representation (Gabor if shielded, `tanh` dictionary otherwise). Gated.
    pub deployed_helmholtz_residual_rel_mse: f64,
    /// Worst relative least-squares residual over 32 phases $\varphi$ of $e^{-s_0^2 x^2}\cos(kx + \varphi)$ for the single-phase sine basis
    /// vs. the `Improved SIREN` $\{\sin, \cos\}$ pair.
    pub improved_siren_worst_phase_residual_single_vs_pair: (f64, f64),
    /// Informative: worst-phase residual of the deployed basis (pair if shielded, single sine otherwise). The pair residual is a
    /// trigonometric identity and is checked under `identities_closed`.
    pub deployed_worst_phase_residual: f64,
    /// Informative: `MIRE` (July 2025) multi-basis PSNR gain as transcribed from the literature; not reproduced here and never gated.
    pub mire_prolate_spheroidal_psnr_gain_db: f64,
    /// Identity: maximum discrepancy between the adjoint *in-situ* gradient $\mathrm{Re}\{k_0^2 E_{\mathrm{fwd}} E_{\mathrm{adj}}\}$ and central finite differences.
    pub hughes_adjoint_in_situ_gradient_max_error: f64,
    /// Named claim checks gated by this axis (see `crate::acceptance`).
    pub claims: Vec<NamedCheck>,
    /// True if the Axis 1 identities close (adjoint gradient within $10^{-6}$, $\{\sin, \cos\}$ phase completeness within $10^{-6}$).
    pub identities_closed: bool,
    /// True if identities close and the deployed representation passes its claim check. The target lies in the Gabor class, so the
    /// claim checks that the learned frequency recovers $k$, and that the `tanh` dictionary cannot.
    pub axis1_verified: bool,
}

impl ComplexWaveGaborHelmholtzAudit {
    /// Evaluates the discrete 2D Helmholtz benchmark, phase completeness of the deployed basis, and adjoint photonic backpropagation.
    pub fn evaluate(seed: u64, shielded: bool) -> Self {
        let mut rng = Rng::new(seed ^ 0xC04D_BEEF_2026_0001);
        let k = 12.0_f64;
        let s0 = 2.5_f64;

        // Ground truth \Phi(x, y) = \exp(-s_0^2 r^2) \exp(i k x) on an 81x81 grid (k h = 0.3) and its discrete source q_h.
        let n = 81usize;
        let h = 2.0 / ((n - 1) as f64);
        let coords: Vec<f64> = (0..n).map(|i| -1.0 + h * (i as f64)).collect();
        let mut env = Vec::with_capacity(n * n);
        let mut target = Vec::with_capacity(n * n);
        for &x in &coords {
            for &y in &coords {
                let e = (-s0 * s0 * (x * x + y * y)).exp();
                env.push(e);
                target.push(Complex64::new(e * (k * x).cos(), e * (k * x).sin()));
            }
        }
        let source_q = discrete_helmholtz(&target, n, h, k);

        // Complex Gabor atom with least-squares complex amplitude; returns the fitted field and the log normalized correlation.
        let fit_gabor = |omega: f64| -> (Vec<Complex64>, f64) {
            let mut atom = Vec::with_capacity(n * n);
            for (i, &x) in coords.iter().enumerate() {
                let (phase_re, phase_im) = ((omega * x).cos(), (omega * x).sin());
                for &e in &env[i * n..(i + 1) * n] {
                    atom.push(Complex64::new(e * phase_re, e * phase_im));
                }
            }
            let mut inner = Complex64::ZERO;
            let mut atom_sq = 0.0;
            let mut target_sq = 0.0;
            for (&a, &t) in atom.iter().zip(&target) {
                inner += a.conj() * t;
                atom_sq += a.norm_sq();
                target_sq += t.norm_sq();
            }
            let amp = Complex64::new(inner.re / atom_sq, inner.im / atom_sq);
            let log_corr = (inner.norm_sq() / (atom_sq * target_sq)).ln();
            (atom.iter().map(|&a| amp * a).collect(), log_corr)
        };

        // LSSA-style spectral adaptation: gradient ascent on the log correlation from \omega_0 = k / 2.
        let mut omega_lssa = 0.5 * k;
        let fd_step = 1e-4;
        for _ in 0..60 {
            let grad = (fit_gabor(omega_lssa + fd_step).1 - fit_gabor(omega_lssa - fd_step).1)
                / (2.0 * fd_step);
            omega_lssa += 2.5 * grad;
        }
        let lssa_freq_rel_err = (omega_lssa - k).abs() / k;
        let (gabor_fit, _) = fit_gabor(omega_lssa);
        let gabor_res = relative_sq_error(&discrete_helmholtz(&gabor_fit, n, h, k), &source_q);

        // Spectrally-biased real dictionary: env * {1, tanh(0.5x), tanh(1.5x), tanh(3x), x^2, y^2} with complex LS coefficients.
        let atom_count = 6usize;
        let mut atoms = vec![vec![0.0; n * n]; atom_count];
        for (i, &x) in coords.iter().enumerate() {
            for (j, &y) in coords.iter().enumerate() {
                let idx = i * n + j;
                let e = env[idx];
                atoms[0][idx] = e;
                atoms[1][idx] = e * (0.5 * x).tanh();
                atoms[2][idx] = e * (1.5 * x).tanh();
                atoms[3][idx] = e * (3.0 * x).tanh();
                atoms[4][idx] = e * x * x;
                atoms[5][idx] = e * y * y;
            }
        }
        let mut gram = vec![vec![0.0; atom_count]; atom_count];
        let mut b_re = vec![0.0; atom_count];
        let mut b_im = vec![0.0; atom_count];
        for m in 0..atom_count {
            for l in 0..atom_count {
                gram[m][l] = atoms[m].iter().zip(&atoms[l]).map(|(p, q)| p * q).sum();
            }
            b_re[m] = atoms[m].iter().zip(&target).map(|(p, t)| p * t.re).sum();
            b_im[m] = atoms[m].iter().zip(&target).map(|(p, t)| p * t.im).sum();
        }
        let coef_re = solve_linear_system(&gram, &b_re).unwrap_or_else(|| vec![0.0; atom_count]);
        let coef_im = solve_linear_system(&gram, &b_im).unwrap_or_else(|| vec![0.0; atom_count]);
        let tanh_fit: Vec<Complex64> = (0..n * n)
            .map(|idx| {
                (0..atom_count).fold(Complex64::ZERO, |acc, m| {
                    Complex64::new(
                        acc.re + coef_re[m] * atoms[m][idx],
                        acc.im + coef_im[m] * atoms[m][idx],
                    )
                })
            })
            .collect();
        let tanh_res = relative_sq_error(&discrete_helmholtz(&tanh_fit, n, h, k), &source_q);
        let deployed_res = if shielded { gabor_res } else { tanh_res };

        // Improved SIREN: phase completeness of {sin, cos} vs. single-phase sine over \varphi = 0 plus 31 random phases.
        let m1 = 201usize;
        let xs1: Vec<f64> = (0..m1)
            .map(|i| -1.0 + 2.0 * (i as f64) / ((m1 - 1) as f64))
            .collect();
        let env1: Vec<f64> = xs1.iter().map(|&x| (-s0 * s0 * x * x).exp()).collect();
        let sin_atom: Vec<f64> = xs1
            .iter()
            .zip(&env1)
            .map(|(&x, &e)| e * (k * x).sin())
            .collect();
        let cos_atom: Vec<f64> = xs1
            .iter()
            .zip(&env1)
            .map(|(&x, &e)| e * (k * x).cos())
            .collect();
        let dot = |a: &[f64], b: &[f64]| a.iter().zip(b).map(|(p, q)| p * q).sum::<f64>();
        let ss = dot(&sin_atom, &sin_atom);
        let cc = dot(&cos_atom, &cos_atom);
        let sc = dot(&sin_atom, &cos_atom);
        let mut phases = vec![0.0_f64];
        for _ in 0..31 {
            phases.push(2.0 * PI * rng.next_f64());
        }
        let mut worst_single = 0.0_f64;
        let mut worst_pair = 0.0_f64;
        for &phi in &phases {
            let t: Vec<f64> = xs1
                .iter()
                .zip(&env1)
                .map(|(&x, &e)| e * (k * x + phi).cos())
                .collect();
            let tt = dot(&t, &t);
            let st = dot(&sin_atom, &t);
            let ct = dot(&cos_atom, &t);
            let single = (1.0 - st * st / (ss * tt)).max(0.0);
            let det = ss * cc - sc * sc;
            let a = (cc * st - sc * ct) / det;
            let b = (ss * ct - sc * st) / det;
            let pair = ((tt - a * st - b * ct) / tt).max(0.0);
            worst_single = worst_single.max(single);
            worst_pair = worst_pair.max(pair);
        }
        let deployed_phase_res = if shielded { worst_pair } else { worst_single };

        // Hughes, Minkov, Shi & Fan (Optica 2018): Adjoint In-Situ Photonic Backpropagation
        let k0_sq = 4.0_f64;
        let mut max_adj_err = 0.0_f64;
        let h_fd = 1e-7_f64;
        for m in 0..8 {
            let eps_m = 0.15 + 0.04 * (m as f64);
            let d_re = 2.5 + 0.3 * (m as f64);
            let d_im = -0.45 - 0.05 * (m as f64);
            let s_re = 1.2 - 0.1 * (m as f64);
            let s_im = 0.35 * (m as f64);
            let t_re = 0.40;
            let t_im = -0.15;

            let eval_field = |e_val: f64| -> (f64, f64) {
                let denom_re = d_re - k0_sq * e_val;
                let denom_im = d_im;
                let norm2 = denom_re * denom_re + denom_im * denom_im;
                (
                    (s_re * denom_re + s_im * denom_im) / norm2,
                    (s_im * denom_re - s_re * denom_im) / norm2,
                )
            };
            let loss_fn = |e_val: f64| -> f64 {
                let (er, ei) = eval_field(e_val);
                0.5 * ((er - t_re).powi(2) + (ei - t_im).powi(2))
            };

            let fd_grad = (loss_fn(eps_m + h_fd) - loss_fn(eps_m - h_fd)) / (2.0 * h_fd);

            let (ef_re, ef_im) = eval_field(eps_m);
            let res_re = ef_re - t_re;
            let res_im = ef_im - t_im;
            let denom_re = d_re - k0_sq * eps_m;
            let denom_im = d_im;
            let norm2 = denom_re * denom_re + denom_im * denom_im;
            let eadj_re = (res_re * denom_re - res_im * denom_im) / norm2;
            let eadj_im = (-res_im * denom_re - res_re * denom_im) / norm2;
            let in_situ_grad = k0_sq * (ef_re * eadj_re - ef_im * eadj_im);
            max_adj_err = max_adj_err.max((in_situ_grad - fd_grad).abs());
        }

        let identities_closed = max_adj_err < 1e-6 && worst_pair < 1e-6;
        let claims: Vec<NamedCheck> = vec![(
            "triad.a1.det.deployed_helmholtz_residual_below_1e-2",
            deployed_res < 1e-2,
        )];
        let axis1_verified = identities_closed && claims.iter().all(|c| c.1);

        Self {
            helmholtz_wavenumber_k: k,
            tanh_mlp_helmholtz_residual_mse: tanh_res,
            wire_complex_gabor_helmholtz_residual_mse: gabor_res,
            lssa_learned_frequency_relative_error: lssa_freq_rel_err,
            tanh_to_lssa_helmholtz_loss_ratio: tanh_res / gabor_res.max(1e-300),
            deployed_helmholtz_residual_rel_mse: deployed_res,
            improved_siren_worst_phase_residual_single_vs_pair: (worst_single, worst_pair),
            deployed_worst_phase_residual: deployed_phase_res,
            mire_prolate_spheroidal_psnr_gain_db: 4.85,
            hughes_adjoint_in_situ_gradient_max_error: max_adj_err,
            claims,
            identities_closed,
            axis1_verified,
        }
    }
}

/// Axis 2 ($\mathbb{C} \leftrightarrow \text{Fractals}$): Algebraic Complex-to-Real Matrix Isomorphism $\mathbb{C}^{n\times n} \hookrightarrow \mathbb{R}^{2n\times 2n}$,
/// Two-Stage `BlockAMC` Schur-Complement Inversion (Pan, Zuo, Luo, Sun & Huang, DATE 2024 & *Nature Electronics* Oct 2025),
/// and Wolfgang Hackbusch's Hierarchical Matrices ($\mathcal{H}$-Matrices).
#[derive(Debug, Clone)]
pub struct ComplexRealIsomorphismHMatrixAudit {
    /// Identity: residual $\max \lvert A X - I\rvert$ of the complex $4\times 4$ inverse recovered from the isomorphic real $8\times 8$ system
    /// $\mathcal{I}(A) = \begin{pmatrix}\mathrm{Re}(A) & -\mathrm{Im}(A)\\ \mathrm{Im}(A) & \mathrm{Re}(A)\end{pmatrix}$.
    pub complex_4x4_to_real_8x8_inversion_discrepancy: f64,
    /// Relative residual $\lVert A x - b\rVert_2 / \lVert b\rVert_2$ of the $16\times 16$ real (8x8 complex MIMO) system solved by two-stage `BlockAMC`
    /// with 10 (shielded) or 1 (naive, no refinement) 24-bit `HP-INV` iterations. Gated.
    pub two_stage_block_amc_16x16_relative_error: f64,
    /// Informative: number of $4\times 4$ RRAM tile operations of the $16\times 16$ tiling (fixed by construction in `crate::analog`).
    pub two_stage_block_amc_4x4_tile_ops: usize,
    /// Informative: `BlockAMC` area and energy efficiency gains as transcribed from the literature (`(0.4883, 0.4000)`).
    pub block_amc_area_and_energy_efficiency_gains: (f64, f64),
    /// Informative: binary cluster-tree depth $p$ used in the storage formula ($n = 2^p = 16$).
    pub hmatrix_hierarchy_depth_p: usize,
    /// Informative: closed-form storage $N_{\text{storage}}(p) = (2p + 1)n$ vs. dense $n^2$ for $n = 16$ (`(144, 256)`).
    pub hmatrix_storage_entries_vs_dense: (usize, usize),
    /// Informative: closed-form storage saving at $p = 10$ ($n = 1024$).
    pub hmatrix_p10_compression_saving_fraction: f64,
    /// Relative Frobenius error of the rank-3 hierarchical approximation of the two-cluster kernel $K_{ij} = 1/(y_j - x_i)$ when only
    /// $\eta$-admissible blocks are compressed vs. when every block is compressed.
    pub hmatrix_admissible_vs_naive_relative_error: (f64, f64),
    /// Relative Frobenius error of the deployed approximation (admissibility-aware if shielded). Gated.
    pub hmatrix_deployed_relative_error: f64,
    /// Named claim checks gated by this axis (see `crate::acceptance`).
    pub claims: Vec<NamedCheck>,
    /// True if the complex-to-real isomorphism identity closes.
    pub identities_closed: bool,
    /// True if identities close and the deployed `BlockAMC` and $\mathcal{H}$-matrix pass their claim checks.
    pub axis2_verified: bool,
}

impl ComplexRealIsomorphismHMatrixAudit {
    /// Evaluates the $\mathbb{C}^{n\times n} \hookrightarrow \mathbb{R}^{2n\times 2n}$ isomorphism, two-stage `BlockAMC`, and hierarchical kernel compression.
    pub fn evaluate(seed: u64, shielded: bool) -> Self {
        let mut rng = Rng::new(seed ^ 0xC04D_BEEF_2026_0002);

        // 1. 4x4 Complex Hermitian-dominant Massive MIMO matrix A = A_re + i A_im mapped to 8x8 Real matrix I(A)
        let n_c = 4usize;
        let mut a_re = vec![vec![0.0; n_c]; n_c];
        let mut a_im = vec![vec![0.0; n_c]; n_c];
        for i in 0..n_c {
            for j in 0..n_c {
                if i == j {
                    a_re[i][j] = 4.5 + 0.3 * (i as f64);
                    a_im[i][j] = 0.25 * ((i as f64) + 1.0);
                } else {
                    a_re[i][j] = 0.25 * (rng.next_f64() - 0.5);
                    a_im[i][j] = 0.25 * (rng.next_f64() - 0.5);
                }
            }
        }

        // Build 8x8 real block matrix I(A) = [[Re(A), -Im(A)], [Im(A), Re(A)]]
        let n_r = 2 * n_c;
        let mut iso_a = vec![vec![0.0; n_r]; n_r];
        for i in 0..n_c {
            for j in 0..n_c {
                iso_a[i][j] = a_re[i][j];
                iso_a[i][j + n_c] = -a_im[i][j];
                iso_a[i + n_c][j] = a_im[i][j];
                iso_a[i + n_c][j + n_c] = a_re[i][j];
            }
        }

        // Verify exact complex algebraic identity (A_re + i A_im)(X_re + i X_im) = I_4 vs 8x8 real inverse
        let mut max_iso_err = 0.0_f64;
        for col in 0..n_c {
            let mut e_re = vec![0.0; n_r];
            e_re[col] = 1.0;
            let sol = solve_linear_system(&iso_a, &e_re).unwrap_or_else(|| vec![0.0; n_r]);
            let x_re = &sol[0..n_c];
            let x_im = &sol[n_c..n_r];

            for row in 0..n_c {
                let mut prod_re = 0.0;
                let mut prod_im = 0.0;
                for k in 0..n_c {
                    prod_re += a_re[row][k] * x_re[k] - a_im[row][k] * x_im[k];
                    prod_im += a_re[row][k] * x_im[k] + a_im[row][k] * x_re[k];
                }
                let target_re = if row == col { 1.0 } else { 0.0 };
                let err = (prod_re - target_re).hypot(prod_im);
                max_iso_err = max_iso_err.max(err);
            }
        }

        // 2. 8x8 Complex Massive MIMO system -> 16x16 Real system solved via Two-Stage BlockAMC (16x16 -> 8x8 -> 4x4) + HP-INV refinement
        let aimc_eval = crate::analog::AimcAnalogWallHpInvAudit::evaluate(seed, shielded);
        let (block_amc_rel_err, tile_ops) = (
            aimc_eval.pku_block_amc_relative_error,
            aimc_eval.pku_block_amc_operations_count,
        );

        // 3. Closed-form H-Matrix storage bookkeeping (informative)
        let p_depth = 4usize;
        let n_h = 1usize << p_depth; // 16
        let h_storage = (2 * p_depth + 1) * n_h; // 144 vs 256
        let dense_storage = n_h * n_h;
        let p10_n = 1024usize;
        let p10_h_storage = (2 * 10 + 1) * p10_n; // 21,504
        let p10_saving = 1.0 - (p10_h_storage as f64) / ((p10_n * p10_n) as f64);

        // 4. Hierarchical approximation of K_{ij} = 1 / (y_j - x_i) for targets X = {0, 0.1, 0.2, 0.3} against a near source cluster
        // Y_near = {0.07, ..., 0.37} (overlapping, inadmissible) and a far cluster Y_far = {2.0, ..., 2.3} (\eta = 1 admissible).
        let x_pts = [0.0_f64, 0.1, 0.2, 0.3];
        let y_blocks = [[0.07_f64, 0.17, 0.27, 0.37], [2.0, 2.1, 2.2, 2.3]];
        let x_c = 0.15_f64;
        let rank = 3;
        let taylor = |x: f64, y: f64| -> f64 {
            (0..rank)
                .map(|m| (x - x_c).powi(m) / (y - x_c).powi(m + 1))
                .sum()
        };
        let diam = |pts: &[f64]| -> f64 {
            pts.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
                - pts.iter().cloned().fold(f64::INFINITY, f64::min)
        };
        let mut exact_sq = 0.0_f64;
        let mut admissible_err_sq = 0.0_f64;
        let mut naive_err_sq = 0.0_f64;
        for y_blk in &y_blocks {
            let dist = x_pts
                .iter()
                .flat_map(|&x| y_blk.iter().map(move |&y| (y - x).abs()))
                .fold(f64::INFINITY, f64::min);
            let admissible = diam(&x_pts).max(diam(y_blk)) <= dist;
            for &x in &x_pts {
                for &y in y_blk {
                    let exact = 1.0 / (y - x);
                    let compressed = taylor(x, y);
                    exact_sq += exact * exact;
                    naive_err_sq += (exact - compressed).powi(2);
                    if admissible {
                        admissible_err_sq += (exact - compressed).powi(2);
                    }
                }
            }
        }
        let hm_admissible_err = (admissible_err_sq / exact_sq).sqrt();
        let hm_naive_err = (naive_err_sq / exact_sq).sqrt();
        let hm_deployed_err = if shielded {
            hm_admissible_err
        } else {
            hm_naive_err
        };

        let identities_closed = max_iso_err < 1e-14;
        let claims: Vec<NamedCheck> = vec![
            (
                "triad.a2.deployed_block_amc_residual_below_1e-7",
                block_amc_rel_err < 1e-7,
            ),
            (
                "triad.a2.det.deployed_hmatrix_error_below_1e-3",
                hm_deployed_err < 1e-3,
            ),
        ];
        let axis2_verified = identities_closed && claims.iter().all(|c| c.1);

        Self {
            complex_4x4_to_real_8x8_inversion_discrepancy: max_iso_err,
            two_stage_block_amc_16x16_relative_error: block_amc_rel_err,
            two_stage_block_amc_4x4_tile_ops: tile_ops,
            block_amc_area_and_energy_efficiency_gains: (0.4883, 0.4000),
            hmatrix_hierarchy_depth_p: p_depth,
            hmatrix_storage_entries_vs_dense: (h_storage, dense_storage),
            hmatrix_p10_compression_saving_fraction: p10_saving,
            hmatrix_admissible_vs_naive_relative_error: (hm_admissible_err, hm_naive_err),
            hmatrix_deployed_relative_error: hm_deployed_err,
            claims,
            identities_closed,
            axis2_verified,
        }
    }
}
