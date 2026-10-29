//! Triadic Mathematical & Physical Synthesis ($\mathbb{C} \times \text{Fractals} \times \mathbb{T}^d$),
//! Technium / Technosphere Evolutionary Ontology, and Spring-and-Horizon Multidimensional Spiral (`stochology_lenses::triad`).
//!
//! Implements the 6-bridge mathematical-physical synthesis and real-data empirical coupler for Gate `G8` and Lab 3 / Lab 7:
//! 1. **Axis 1 ($\mathbb{C}$ — Complex Phase-Amplitude Wave Physics & Gabor INRs):** 2D Helmholtz PDE $(\nabla^2 + k^2)\Phi = q$
//!    over $\Phi = u + iv \in \mathbb{C}$ on a discrete 5-point Laplacian, fitting a complex Gabor atom (`WIRE`) whose frequency is
//!    learned by gradient ascent (`LSSA`-style spectral adaptation) against a spectrally-biased real `tanh` dictionary, the
//!    `Improved SIREN` $\sin/\cos$ phase-complete basis against single-phase sine, and Hughes, Minkov, Shi & Fan (*Optica* 2018)
//!    adjoint *in-situ* photonic backpropagation.
//! 2. **Axis 2 ($\mathbb{C} \leftrightarrow \text{Fractals}$ — Complex-to-Real Matrix Isomorphism, Two-Stage `BlockAMC`, & Hackbusch $\mathcal{H}$-Matrices):**
//!    Algebraic isomorphism $\mathbb{C}^{n\times n} \hookrightarrow \mathbb{R}^{2n\times 2n}$, two-stage recursive Schur-complement `BlockAMC`
//!    with 24-bit `HP-INV` refinement (`crate::analog`), and an $\eta$-admissible hierarchical low-rank approximation of a two-cluster
//!    kernel matrix against naive compression of every block.
//! 3. **Axis 3 ($\text{Fractals} \leftrightarrow \mathbb{R}^n$ — Fractional `CTRW` Subdiffusion, Atomic Conductance Quantum $G_0$, & Analog ECC):**
//!    $G_0 = \frac{2e^2}{h}$ from exact SI constants, state-aware PCM drift compensation $G(t) = G_0(t/t_0)^{-\nu}$ from an early read,
//!    a simulated Montroll-Weiss `CTRW` (Pareto waiting times $\beta = 0.70$, symmetric $\alpha$-stable jumps $\alpha = 1.75$) whose spread
//!    scaling $c^{\beta/\alpha}$ is tested against the Fickian prediction $c^{1/2}$, and residual-trimmed least-squares decoding of a
//!    $[10, 2]$ analog code under bounded LME noise plus $\tau = 2$ unlimited-magnitude errors.
//! 4. **Axis 4 ($\text{Fractals} \leftrightarrow \text{Scaling Laws}$ — Multifractal Cascade, Eichi `PDHTE+JK` & `TailWelsch-DML`, & the $\pi^2/6$ Bound):**
//!    A simulated log-normal multiplicative cascade whose partition-function curvature $\hat{\tau}''(q)$ is tested against the
//!    multifractal prediction $-\sigma^2 \ln b$ and the monofractal prediction $0$, the `PDHTE+JK` Hill-plateau jackknife tail index
//!    against single-$k$ DEdH, the global refusal gate ($< 0.70$) on thin-tailed data, Welsch-IRLS against OLS under Pareto noise,
//!    and the Dey & Donoho (2024) partial Basel sum $\sum_{g=1}^8 g^{-2} < \pi^2/6$.
//! 5. **Axis 5 ($\mathbb{T}^d \leftrightarrow \mathbb{C}$ — Preface Modular Cycle, Golden Weyl Torus `SplitMix`/`LXM`, Hermite-Mehler RKHS `DFM`, & `MLLS+BCTS`):**
//!    Preface full-period cycle $x_{t+1} = (5x_t + 3) \bmod 16$ from $x_0 = 2026 \bmod 16 = 10$, golden vs. rational Weyl star discrepancy
//!    on $\mathbb{T}^1$, Steele-Lea-Flood `mixGamma` vs. unmixed odd increments, Dussap et al. (2023) `DFM` contaminated label shift with
//!    Hermite-Mehler features vs. random Fourier features, and Garg et al. (NeurIPS 2020) `MLLS+BCTS` vs. `BBSE` vs. classify-and-count.
//! 6. **Axis 6 ($\mathbb{C} \times \text{Fractals} \times \mathbb{T}^d$ — `SNA` Fractal Torus, Delayed `RLHF` Loop, Izzo `PerfGD`, Phasor `FHRR`/`GC-VSA`, `LTC` Fused Solver, & Technium):**
//!    Schnakenberg cycle entropy production $\Pi = \sum_e J_e A_e \ge 0$, the quasiperiodically forced logistic map (Lyapunov exponent,
//!    phase-sensitivity growth, orbital synchronization), the 3-stage `RLHF` feedback loop with its delayed companion polynomial,
//!    oracle-free `PerfGD` vs. Repeated Risk Minimization on `PerformativeSystem`, Phasor `FHRR` binding on $\mathbb{T}^D \subset \mathbb{C}^D$,
//!    Hasani's `LTC` Fused Solver vs. explicit Euler at a stiff step, and the Taleb-Douady fragility heuristic $H(x,\Delta)$.
//!
//! # Verification semantics
//!
//! Every axis separates three kinds of quantities:
//! - **Claim checks** compare the output of the *deployed* method against simulated ground truth. With `shielded = true` the
//!   CAPIBA method is deployed; with `shielded = false` the naive method is actually run (real `tanh` dictionary, single-phase sine,
//!   one-shot inversion without refinement, compression without admissibility, uncompensated drift, Fickian scaling, plain least
//!   squares, monofractal model, single-$k$ DEdH, no refusal gate, OLS, rational Weyl step, unmixed gammas, random Fourier features,
//!   classify-and-count, Repeated Risk Minimization, explicit Euler). `--defect triadic_technium` therefore fails because the naive
//!   method misses the ground truth, not because a flag overwrites the result.
//! - **Identity checks** (`identities_closed`) are algebraic or numerical closures that must hold in both modes; they detect
//!   implementation bugs, not empirical failures.
//! - **Informative constants** transcribed from the cited literature (or closed-form arithmetic on them) are carried for
//!   traceability and never enter a verification boolean.
//! - **Discarded findings** (`discarded_findings`) are claims that missed the tolerance fixed in the code before the run at
//!   `seed = 2026`; **discarded claims** (`discarded_claims`) failed multi-seed acceptance (`crate::acceptance`). Both are
//!   reported (Prohibition 4) and never counted as verified.

use std::f64::consts::{LN_2, PI};
use stochology_core::layers::PerformativeSystem;
use stochology_core::rng::{median, Rng};

use crate::acceptance::{MultiSeedAcceptance, NamedCheck};
use crate::tail::{dedh_moment_tail_index, hill_estimator};

/// Complex number as `(re, im)`.
type C64 = (f64, f64);

#[inline]
fn c_mul(a: C64, b: C64) -> C64 {
    (a.0 * b.0 - a.1 * b.1, a.0 * b.1 + a.1 * b.0)
}

/// Returns $\bar{a} b$.
#[inline]
fn c_conj_mul(a: C64, b: C64) -> C64 {
    (a.0 * b.0 + a.1 * b.1, a.0 * b.1 - a.1 * b.0)
}

#[inline]
fn c_div(a: C64, b: C64) -> C64 {
    let norm2 = (b.0 * b.0 + b.1 * b.1).max(1e-300);
    let p = c_conj_mul(b, a);
    (p.0 / norm2, p.1 / norm2)
}

/// Relative squared error $\lVert a - t\rVert^2 / \lVert t\rVert^2$ between two complex fields.
fn relative_sq_error(approx: &[C64], target: &[C64]) -> f64 {
    let num: f64 = approx
        .iter()
        .zip(target)
        .map(|(a, t)| (a.0 - t.0).powi(2) + (a.1 - t.1).powi(2))
        .sum();
    let den: f64 = target.iter().map(|t| t.0 * t.0 + t.1 * t.1).sum();
    num / den.max(1e-300)
}

/// Discrete Helmholtz operator $(\nabla_h^2 + k^2)$ (5-point Laplacian) on the interior nodes of an $n \times n$ grid with spacing $h$.
fn discrete_helmholtz(field: &[C64], n: usize, h: f64, k: f64) -> Vec<C64> {
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
            let lap_re = (nb.iter().map(|v| v.0).sum::<f64>() - 4.0 * c.0) * inv_h2;
            let lap_im = (nb.iter().map(|v| v.1).sum::<f64>() - 4.0 * c.1) * inv_h2;
            out.push((lap_re + k * k * c.0, lap_im + k * k * c.1));
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
                target.push((e * (k * x).cos(), e * (k * x).sin()));
            }
        }
        let source_q = discrete_helmholtz(&target, n, h, k);

        // Complex Gabor atom with least-squares complex amplitude; returns the fitted field and the log normalized correlation.
        let fit_gabor = |omega: f64| -> (Vec<C64>, f64) {
            let mut atom = Vec::with_capacity(n * n);
            for (i, &x) in coords.iter().enumerate() {
                let (phase_re, phase_im) = ((omega * x).cos(), (omega * x).sin());
                for &e in &env[i * n..(i + 1) * n] {
                    atom.push((e * phase_re, e * phase_im));
                }
            }
            let mut inner = (0.0, 0.0);
            let mut atom_sq = 0.0;
            let mut target_sq = 0.0;
            for (a, t) in atom.iter().zip(&target) {
                let p = c_conj_mul(*a, *t);
                inner.0 += p.0;
                inner.1 += p.1;
                atom_sq += a.0 * a.0 + a.1 * a.1;
                target_sq += t.0 * t.0 + t.1 * t.1;
            }
            let amp = (inner.0 / atom_sq, inner.1 / atom_sq);
            let log_corr = ((inner.0 * inner.0 + inner.1 * inner.1) / (atom_sq * target_sq)).ln();
            (atom.iter().map(|&a| c_mul(amp, a)).collect(), log_corr)
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
            b_re[m] = atoms[m].iter().zip(&target).map(|(p, t)| p * t.0).sum();
            b_im[m] = atoms[m].iter().zip(&target).map(|(p, t)| p * t.1).sum();
        }
        let coef_re = solve_real_linear_system(&gram, &b_re);
        let coef_im = solve_real_linear_system(&gram, &b_im);
        let tanh_fit: Vec<C64> = (0..n * n)
            .map(|idx| {
                (0..atom_count).fold((0.0, 0.0), |acc, m| {
                    (
                        acc.0 + coef_re[m] * atoms[m][idx],
                        acc.1 + coef_im[m] * atoms[m][idx],
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
        // Complex diagonal scattering perturbation: G(\varepsilon) = (\Omega_0 - k_0^2 \mathrm{diag}(\varepsilon))^{-1}.
        // For scalar/decoupled complex modes E_j(\varepsilon_j) = s_j / (d_j - k_0^2 \varepsilon_j),
        // the exact derivative of L = \frac{1}{2} \sum_j |E_j - T_j|^2 w.r.t. \varepsilon_j is
        // \partial L / \partial \varepsilon_j = \mathrm{Re}\{ k_0^2 E_{\mathrm{fwd},j} E_{\mathrm{adj},j} \} where E_{\mathrm{adj},j} = (E_j - T_j)^* / (d_j^* - k_0^2 \varepsilon_j).
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
            // Adjoint backpropagated field E_adj = G^T (E_fwd - T)^*
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

/// Solves a dense real linear system $A x = b$ via Gaussian elimination with partial pivoting.
fn solve_real_linear_system(a: &[Vec<f64>], b: &[f64]) -> Vec<f64> {
    let n = b.len();
    let mut aug = vec![vec![0.0; n + 1]; n];
    for i in 0..n {
        for j in 0..n {
            aug[i][j] = a[i][j];
        }
        aug[i][n] = b[i];
    }
    for col in 0..n {
        let mut pivot = col;
        let mut max_abs = aug[col][col].abs();
        for (row, aug_row) in aug.iter().enumerate().skip(col + 1) {
            if aug_row[col].abs() > max_abs {
                max_abs = aug_row[col].abs();
                pivot = row;
            }
        }
        aug.swap(col, pivot);
        let diag = if aug[col][col].abs() < 1e-15 {
            1e-15
        } else {
            aug[col][col]
        };
        for val in aug[col].iter_mut().take(n + 1).skip(col) {
            *val /= diag;
        }
        let pivot_row = aug[col].clone();
        for (row_idx, row_vec) in aug.iter_mut().enumerate() {
            if row_idx != col {
                let factor = row_vec[col];
                for (val, &piv) in row_vec
                    .iter_mut()
                    .zip(pivot_row.iter())
                    .take(n + 1)
                    .skip(col)
                {
                    *val -= factor * piv;
                }
            }
        }
    }
    aug.into_iter().map(|r| r[n]).collect()
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
            let sol = solve_real_linear_system(&iso_a, &e_re);
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
        // Rank-3 separable Taylor expansion in x about x_c: 1/(y - x) = \sum_{m<3} (x - x_c)^m / (y - x_c)^{m+1} + O(r^3).
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

/// Symmetric $\alpha$-stable draw via the Chambers-Mallows-Stuck (1976) transform.
fn symmetric_stable(rng: &mut Rng, alpha: f64) -> f64 {
    let v = PI * (rng.next_f64().clamp(1e-12, 1.0 - 1e-12) - 0.5);
    let w = -rng.next_f64().clamp(1e-300, 1.0).ln();
    (alpha * v).sin() / v.cos().powf(1.0 / alpha)
        * ((v - alpha * v).cos() / w.max(1e-300)).powf((1.0 - alpha) / alpha)
}

/// Least-squares fit of $y \approx m_1 u_1 + m_2 u_2$ on the active coordinates; returns `(m1, m2)`.
fn analog_code_ls(u1: &[f64; 10], u2: &[f64; 10], y: &[f64; 10], active: &[bool; 10]) -> C64 {
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
        // State-aware compensation estimates \hat{\nu}_i from one read at t/t_0 = 10 with 1% multiplicative read noise.
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
        // Positions are recorded at t_1 = 1000 and t_2 = 16 t_1; the median spread ratio is compared with 16^{\beta/\alpha} and 16^{1/2}.
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

        // 4. Ron M. Roth (2020) Analog ECC: literature m-height profile of the [10, 2] Analog Permutation Code (informative).
        let u1 = [1.0, 1.0, 1.0, 1.0, 1.0, -1.0, -1.0, -1.0, -1.0, -1.0];
        let m_heights: [f64; 8] = [1.0, 1.0, 1.0, 1.0, 1.92, 3.88, 3.88, 28.74];
        let thresh_tau2 = 2.0 * (m_heights[3] + 1.0); // 2 * (h_4 + 1) = 4.0
        let thresh_tau4 = 2.0 * (m_heights[7] + 1.0); // 2 * (h_8 + 1) = 59.48
        let u1_heights_unit = (1..10).all(|m| (Self::vector_m_height(&u1, m) - 1.0).abs() < 1e-15);

        // Decode 100 codewords y = m_1 u_1 + m_2 u_2 + LME (|e| <= \delta = 0.05) with \tau = 2 UMEs (+12, -15).
        // Trimmed decoder: refit least squares after discarding, one at a time, the coordinate with the largest residual.
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

        // 5. Polygonal Geometric Analog Codes (Zhu et al., March 2026): fundamental sector \pi / (2n) (informative)
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
    /// The gated claim is the mode-independent property that the gate rejects 600 random bounded draws.
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
        // Logarithmic k-grid in [max(12, n^0.45), min(n/3, n^0.80)]
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

        // Sliding window of width w = 5 over the log-grid to detect the regular-variation plateau (CV(t_0) < 0.25)
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

    /// Estimates $\hat{\tau}(q)$ for $q \in \{1, 2, 3\}$ from the partition functions $Z_j(q) = \sum_{\text{boxes at level } j} \mu^q$
    /// of simulated log-normal cascades, fitting $\log_2 Z_j(q) = -j\,\tau(q) + c$ over levels $j \in [2, 10]$.
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

        // 1. Log-normal cascade (b = 2): \tau(q) = q - 1 - \sigma^2 (q^2 - q) / (2 \ln 2) \implies \tau''(q) = -\sigma^2 / \ln 2.
        // \sigma^2 = 0.26 \ln^2 2 reproduces the Mandelbrot-Calvet-Fisher curvature -\sigma_{MCF}^2 \ln b with \sigma_{MCF}^2 = 0.26.
        let sigma2_cascade = 0.26 * LN_2 * LN_2;
        let tau_theory = -sigma2_cascade / LN_2;
        let tau_hat = Self::estimate_cascade_tau(&mut rng, sigma2_cascade, 14, 8);
        let tau_second_diff = tau_hat[0] - 2.0 * tau_hat[1] + tau_hat[2];
        let model_curvature = if shielded { tau_theory } else { 0.0 };
        let deployed_curv_err = (model_curvature - tau_second_diff).abs();

        // 2. Eichi (May 2026) PDHTE+JK on a second-order Hall-Welsh Fréchet-Pareto sample (\xi_true = 0.50, \alpha = 2.0):
        // X_i = U_i^{-\xi} (1 + 0.65 U_i^{\xi}) where U_i ~ Uniform(0, 1).
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

        // Refusal gate on 600 random bounded draws U(1.00, 1.05) (Weibull domain, xi = -1): the gate refuses when the accepted
        // plateau fraction is < 0.70; the naive pipeline has no gate and always emits a tail index.
        let mut rng_bounded = Rng::new(seed ^ 0xC04D_BEEF_2026_0044);
        let light_samples: Vec<f64> = (0..600)
            .map(|_| 1.0 + 0.05 * rng_bounded.next_f64())
            .collect();
        let (_, _, _, light_acc_frac) = Self::evaluate_pdhte_jk_tail(&light_samples);
        let gate_rejects_bounded = light_acc_frac < 0.70;
        let refusal_gate_blocks = shielded && gate_rejects_bounded;

        // 3. TailWelsch-DML: estimate treatment effect \theta_0 = 2.50 under heavy-tailed Pareto noise (\alpha = 1.65)
        let theta_true = 2.50_f64;
        let mut ols_sq_err = 0.0;
        let mut welsch_sq_err = 0.0;
        let mc_runs = 40usize;
        let gamma_welsch = 3.2_f64; // Tail-adaptive Welsch scale \gamma(\hat{\xi})
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

            // IRLS with Welsch weight w(r) = \exp(-(r/\gamma)^2), initialized at OLS
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

        // 4. Dey & Donoho (2024) / Dohmatob et al. (2024) universal Basel accumulation bound at G = 8:
        // \sum_{g=1}^8 1/g^2 = 1.527422052154195 < \pi^2 / 6 = 1.6449340668482264 vs Replace = 8.0
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

/// Star discrepancy $D_N^*$ of a point set on $\mathbb{T}^1 = [0, 1)$.
fn star_discrepancy_1d(points: &[f64]) -> f64 {
    let mut sorted = points.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let n = sorted.len() as f64;
    sorted
        .iter()
        .enumerate()
        .map(|(i, &u)| {
            (u - (i as f64) / n)
                .abs()
                .max((u - ((i + 1) as f64) / n).abs())
        })
        .fold(0.0, f64::max)
}

/// Orthonormal Hermite-Mehler moment map $\psi_c(x) = \sqrt{\lambda_c}\,He_c(x)/\sqrt{c!}\,e^{-x^2/4}$, $c = 0..3$, $\lambda_c = (1-\rho)\rho^c$.
fn hermite_mehler_features(x: f64, rho: f64) -> [f64; 4] {
    let env = (-0.25 * x * x).exp();
    let w0 = (1.0 - rho).sqrt();
    let w1 = (w0 * rho.sqrt()) * x;
    let w2 = (w0 * rho) * (x * x - 1.0) / 2.0_f64.sqrt();
    let w3 = (w0 * rho.powf(1.5)) * (x.powi(3) - 3.0 * x) / 6.0_f64.sqrt();
    [w0 * env, w1 * env, w2 * env, w3 * env]
}

/// Kernel mean matching for a two-component mixture: solves $\min_\alpha \lVert \mu_Q - \alpha_1 \mu_{P_1} - \alpha_2 \mu_{P_2}\rVert^2$.
fn dfm_two_class_weights(mu1: &[f64], mu2: &[f64], mu_q: &[f64]) -> C64 {
    let dot = |a: &[f64], b: &[f64]| a.iter().zip(b).map(|(p, q)| p * q).sum::<f64>();
    let (m11, m12, m22) = (dot(mu1, mu1), dot(mu1, mu2), dot(mu2, mu2));
    let (b1, b2) = (dot(mu1, mu_q), dot(mu2, mu_q));
    let det = (m11 * m22 - m12 * m12).max(1e-12);
    ((m22 * b1 - m12 * b2) / det, (m11 * b2 - m12 * b1) / det)
}

/// Fits bias-corrected temperature scaling $p = \sigma(a z + b)$ on logits $z$ by damped Newton steps (backtracking line search
/// on the numerically stable log loss). Converged when the Newton decrement $\lambda^2/2 = g^\top H^{-1} g / 2$ falls below $10^{-10}$;
/// returns `None` if that does not happen within 100 iterations.
fn fit_bcts(logits: &[f64], labels: &[f64]) -> Option<C64> {
    let log_loss = |a: f64, b: f64| -> f64 {
        logits
            .iter()
            .zip(labels)
            .map(|(&z, &y)| {
                let s = a * z + b;
                // log(1 + e^s) - y s, evaluated without overflow
                s.max(0.0) + (-s.abs()).exp().ln_1p() - y * s
            })
            .sum()
    };
    let n = logits.len() as f64;
    let (mut a, mut b) = (1.0_f64, 0.0_f64);
    for _ in 0..100 {
        let (mut ga, mut gb, mut haa, mut hab, mut hbb) = (0.0, 0.0, 0.0, 0.0, 0.0);
        for (&z, &y) in logits.iter().zip(labels) {
            let p = 1.0 / (1.0 + (-(a * z + b)).exp());
            let w = p * (1.0 - p);
            ga += (p - y) * z;
            gb += p - y;
            haa += w * z * z;
            hab += w * z;
            hbb += w;
        }
        let det = haa * hbb - hab * hab;
        let (da, db) = if det > 1e-12 {
            ((hbb * ga - hab * gb) / det, (haa * gb - hab * ga) / det)
        } else {
            (ga / n, gb / n)
        };
        if det > 1e-12 && 0.5 * (ga * da + gb * db) < 1e-10 {
            return Some((a, b));
        }
        let current = log_loss(a, b);
        let mut step = 1.0;
        while step > 1e-10 && log_loss(a - step * da, b - step * db) > current {
            step *= 0.5;
        }
        a -= step * da;
        b -= step * db;
    }
    None
}

/// Axis 5 ($\mathbb{T}^d \leftrightarrow \mathbb{C}$): Preface Modular Cycle in $\mathbb{Z}/16\mathbb{Z}$,
/// Golden Weyl Torus (`SplitMix64` / `LXM` `mixGamma`), Hermite-Mehler RKHS Features vs. Random Fourier Features in `DFM`
/// (Dussap, Blanchard & Chérif-Abdellatif 2023), and `MLLS+BCTS` vs. `BBSE` vs. Classify-and-Count (Garg et al. 2020).
#[derive(Debug, Clone)]
pub struct WeylTorusHermiteMehlerDfmAudit {
    /// Identity: 17-element full-period Hull-Dobell sequence of $x_{t+1} = (5x_t + 3) \bmod 16$ starting at $x_0 = 2026 \bmod 16 = 10$.
    pub preface_lcg_mod16_cycle: [u8; 17],
    /// Star discrepancy $D_{256}^*$ on $\mathbb{T}^1$ of the golden Weyl sequence $\theta_n = (n \varphi^{-1}) \bmod 1$ vs. the rational step $1/8$.
    pub golden_weyl_vs_rational_discrepancy: (f64, f64),
    /// Star discrepancy of the deployed Weyl increment (golden if shielded, rational otherwise). Gated.
    pub deployed_weyl_discrepancy: f64,
    /// Informative: minimum bit-transition count `(gamma ^ (gamma >> 1)).count_ones()` over 64 deployed stream gammas
    /// (`mixGamma` if shielded, unmixed odd increments $2s + 1$ otherwise). `mixGamma` guarantees `>= 24` by construction,
    /// which is checked under `identities_closed`.
    pub mixgamma_min_transition_bits: u32,
    /// Identity: Mercer eigenvalue mass $\sum_{c=0}^{64} (1-\rho)\rho^c = 1 - \rho^{65}$ of the Hermite-Mehler basis ($\rho = 0.50$).
    pub hermite_mehler_eigenvalue_sum: f64,
    /// $L_2$ error $\lVert \hat{\alpha} - \alpha^*\rVert_2$ under contaminated label shift $\mathbb{Q} = 0.55\,\mathbb{P}_1 + 0.35\,\mathbb{P}_2 + 0.10\,\mathbb{Q}_0$
    /// with 4 Hermite-Mehler features vs. 4 random Fourier features.
    pub dfm_hermite_vs_rff_l2_error: (f64, f64),
    /// $L_2$ error of the deployed feature map (Hermite-Mehler if shielded, random Fourier otherwise). Gated.
    pub deployed_dfm_l2_error: f64,
    /// Prior estimation $D_{\mathrm{KL}}(q_Y \parallel \hat{q}_Y)$ of `MLLS+BCTS` vs. hard-thresholded `BBSE` on 10 000 target draws.
    pub mlls_bcts_vs_bbse_kl_divergence: (f64, f64),
    /// Prior estimation KL divergence of classify-and-count with the uncalibrated classifier.
    pub classify_and_count_kl_divergence: f64,
    /// KL divergence of the deployed prior estimator (`MLLS+BCTS` if shielded, classify-and-count otherwise). Gated.
    pub deployed_label_shift_kl: f64,
    /// Named claim checks gated by this axis (see `crate::acceptance`).
    pub claims: Vec<NamedCheck>,
    /// Claims removed from the gate after failing multi-seed acceptance (`crate::acceptance`); still computed and reported.
    pub discarded_claims: Vec<NamedCheck>,
    /// True if the Hull-Dobell, Mercer, and `mixGamma` transition identities close.
    pub identities_closed: bool,
    /// True if identities close and the deployed Weyl step, `DFM` features, and prior estimator pass their claim checks.
    pub axis5_verified: bool,
}

impl WeylTorusHermiteMehlerDfmAudit {
    /// Steele, Lea & Flood (OOPSLA 2014) `mixGamma` function: `mix64variant13(z) | 1`, ensuring odd $\gamma$ ($\gcd(\gamma, 2^{64}) = 1$)
    /// and at least 24 bit transitions in `z ^ (z >> 1)`.
    pub fn mix_gamma(mut z: u64) -> u64 {
        z = (z ^ (z >> 33)).wrapping_mul(0xff51_afd7_ed55_8ccd);
        z = (z ^ (z >> 33)).wrapping_mul(0xc4ce_b9fe_1a85_ec53);
        z = (z ^ (z >> 33)) | 1;
        let n = (z ^ (z >> 1)).count_ones();
        if n < 24 {
            z ^= 0xaaaa_aaaa_aaaa_aaaa;
        }
        z
    }

    /// Evaluates Axis 5: Preface $\mathbb{Z}/16\mathbb{Z}$ cycle, Weyl torus & `mixGamma`, Hermite-Mehler `DFM`, and label-shift prior estimation.
    pub fn evaluate(seed: u64, shielded: bool) -> Self {
        let mut rng = Rng::new(seed ^ 0xC04D_BEEF_2026_0005);

        // 1. Preface full-period modular cycle x_{t+1} = (5 x_t + 3) mod 16 from x_0 = 2026 mod 16 = 10
        let mut cycle = [0u8; 17];
        let mut x_mod = (2026u64 % 16) as u8;
        for slot in &mut cycle {
            *slot = x_mod;
            x_mod = (5 * x_mod + 3) % 16;
        }
        let expected_cycle: [u8; 17] = [10, 5, 12, 15, 14, 9, 0, 3, 2, 13, 4, 7, 6, 1, 8, 11, 10];

        // 2. Golden vs. rational Weyl rotation on T^1, plus Steele-Lea-Flood mixGamma vs. unmixed odd increments
        let phi_inv = (5.0_f64.sqrt() - 1.0) * 0.5;
        let n_weyl = 256usize;
        let golden_pts: Vec<f64> = (0..n_weyl)
            .map(|n| ((n as f64) * phi_inv).fract())
            .collect();
        let rational_pts: Vec<f64> = (0..n_weyl).map(|n| ((n as f64) / 8.0).fract()).collect();
        let disc_golden = star_discrepancy_1d(&golden_pts);
        let disc_rational = star_discrepancy_1d(&rational_pts);
        let deployed_disc = if shielded { disc_golden } else { disc_rational };

        let transitions = |g: u64| (g ^ (g >> 1)).count_ones();
        let mixed_min = (1..=64u64)
            .map(|s| {
                transitions(Self::mix_gamma(
                    seed.wrapping_add(s.wrapping_mul(0x9e37_79b9_7f4a_7c15)),
                ))
            })
            .min()
            .unwrap_or(0);
        let unmixed_min = (1..=64u64)
            .map(|s| transitions(2 * s + 1))
            .min()
            .unwrap_or(0);
        let min_transitions = if shielded { mixed_min } else { unmixed_min };

        // 3. Contaminated label shift Q = \alpha_1^* P_1 + \alpha_2^* P_2 + \alpha_0^* Q_0 with \alpha^* = (0.55, 0.35), \alpha_0^* = 0.10:
        // P_1 ~ N(-1.2, 0.5^2), P_2 ~ N(+1.2, 0.5^2), contamination Q_0 ~ N(4.5, 0.3^2).
        let rho_mehler = 0.50_f64;
        let eig_sum: f64 = (0..=64)
            .map(|c| (1.0 - rho_mehler) * rho_mehler.powi(c))
            .sum();
        let alpha1_true = 0.55_f64;
        let alpha2_true = 0.35_f64;
        let n_mc = 400usize;
        let mut xs_p1 = Vec::with_capacity(n_mc);
        let mut xs_p2 = Vec::with_capacity(n_mc);
        let mut xs_q = Vec::with_capacity(n_mc);
        for i in 0..n_mc {
            let z = rng.standard_normal();
            xs_p1.push(-1.2 + 0.5 * z);
            xs_p2.push(1.2 + 0.5 * z);
            let frac = (i as f64) / (n_mc as f64);
            xs_q.push(if frac < alpha1_true {
                -1.2 + 0.5 * rng.standard_normal()
            } else if frac < alpha1_true + alpha2_true {
                1.2 + 0.5 * rng.standard_normal()
            } else {
                4.5 + 0.3 * rng.standard_normal()
            });
        }
        // Random Fourier features \sqrt{2/D} \cos(\omega_d x + b_d), \omega_d ~ N(0, 1), b_d ~ U(0, 2\pi), D = 4.
        let rff_params: Vec<C64> = (0..4)
            .map(|_| (rng.standard_normal(), 2.0 * PI * rng.next_f64()))
            .collect();
        let rff_features = |x: f64| -> [f64; 4] {
            let mut f = [0.0; 4];
            for (slot, &(w, b)) in f.iter_mut().zip(&rff_params) {
                *slot = (0.5_f64).sqrt() * (w * x + b).cos();
            }
            f
        };
        let mean_map = |xs: &[f64], map: &dyn Fn(f64) -> [f64; 4]| -> [f64; 4] {
            let mut mu = [0.0; 4];
            for &x in xs {
                for (m, v) in mu.iter_mut().zip(map(x)) {
                    *m += v / (xs.len() as f64);
                }
            }
            mu
        };
        let hm_map = |x: f64| hermite_mehler_features(x, rho_mehler);
        let dfm_error = |map: &dyn Fn(f64) -> [f64; 4]| -> f64 {
            let (a1, a2) = dfm_two_class_weights(
                &mean_map(&xs_p1, map),
                &mean_map(&xs_p2, map),
                &mean_map(&xs_q, map),
            );
            (a1 - alpha1_true).hypot(a2 - alpha2_true)
        };
        let err_hm = dfm_error(&hm_map);
        let err_rff = dfm_error(&rff_features);
        let deployed_dfm_err = if shielded { err_hm } else { err_rff };

        // 4. Garg et al. (NeurIPS 2020): label shift from source prior 0.5 to target prior q = 0.72, x | y ~ N(\pm 1, 1).
        // The deployed classifier f(x) = \sigma(4x - 0.5) is overconfident and biased relative to the true posterior \sigma(2x).
        let q_true = 0.72_f64;
        let classifier_logit = |x: f64| 4.0 * x - 0.5;
        let n_val = 2000usize;
        let mut val_logits = Vec::with_capacity(n_val);
        let mut val_labels = Vec::with_capacity(n_val);
        for _ in 0..n_val {
            let y = if rng.next_f64() < 0.5 { 1.0 } else { 0.0 };
            let x = (2.0 * y - 1.0) + rng.standard_normal();
            val_logits.push(classifier_logit(x));
            val_labels.push(y);
        }
        let n_target = 10_000usize;
        let target_logits: Vec<f64> = (0..n_target)
            .map(|_| {
                let y = if rng.next_f64() < q_true { 1.0 } else { 0.0 };
                classifier_logit((2.0 * y - 1.0) + rng.standard_normal())
            })
            .collect();
        let pi_source = val_labels.iter().sum::<f64>() / (n_val as f64);

        // MLLS (Saerens-Latinne-Decaestecker EM) on BCTS-calibrated posteriors; an unconverged calibration refuses (no estimate).
        let q_mlls = fit_bcts(&val_logits, &val_labels).map(|(bcts_a, bcts_b)| {
            let calibrated: Vec<f64> = target_logits
                .iter()
                .map(|&z| 1.0 / (1.0 + (-(bcts_a * z + bcts_b)).exp()))
                .collect();
            let mut q = pi_source;
            for _ in 0..500 {
                q = calibrated
                    .iter()
                    .map(|&p| {
                        let pos = q / pi_source * p;
                        pos / (pos + (1.0 - q) / (1.0 - pi_source) * (1.0 - p))
                    })
                    .sum::<f64>()
                    / (n_target as f64);
            }
            q
        });
        // Hard-thresholded BBSE: invert the source confusion rates
        let (mut tp, mut fp, mut n_pos) = (0.0, 0.0, 0.0);
        for (&z, &y) in val_logits.iter().zip(&val_labels) {
            let pred = if z > 0.0 { 1.0 } else { 0.0 };
            tp += pred * y;
            fp += pred * (1.0 - y);
            n_pos += y;
        }
        let tpr = tp / n_pos;
        let fpr = fp / ((n_val as f64) - n_pos);
        let target_pos_rate =
            target_logits.iter().filter(|&&z| z > 0.0).count() as f64 / (n_target as f64);
        let q_bbse = ((target_pos_rate - fpr) / (tpr - fpr)).clamp(1e-3, 1.0 - 1e-3);
        let q_cc = target_pos_rate;
        let kl_bin = |p: f64, q: f64| -> f64 {
            let p_c = p.clamp(1e-6, 1.0 - 1e-6);
            let q_c = q.clamp(1e-6, 1.0 - 1e-6);
            p_c * (p_c / q_c).ln() + (1.0 - p_c) * ((1.0 - p_c) / (1.0 - q_c)).ln()
        };
        let kl_mlls = q_mlls.map_or(f64::INFINITY, |q| kl_bin(q_true, q));
        let kl_bbse = kl_bin(q_true, q_bbse);
        let kl_cc = kl_bin(q_true, q_cc);
        let deployed_kl = if shielded { kl_mlls } else { kl_cc };

        let identities_closed = cycle == expected_cycle
            && (eig_sum - (1.0 - rho_mehler.powi(65))).abs() < 1e-12
            && mixed_min >= 24;
        let claims: Vec<NamedCheck> = vec![
            (
                "triad.a5.det.deployed_weyl_discrepancy_below_0.02",
                deployed_disc < 0.02,
            ),
            (
                "triad.a5.deployed_dfm_error_below_0.05",
                deployed_dfm_err < 0.05,
            ),
        ];
        // MLLS+BCTS reaches KL < 1e-3 on 18 of 20 acceptance seeds (sampling error ~0.02 in the prior at n = 2000 + 10 000).
        let discarded_claims: Vec<NamedCheck> = vec![(
            "triad.a5.deployed_label_shift_kl_below_1e-3",
            deployed_kl < 1e-3,
        )];
        let axis5_verified = identities_closed && claims.iter().all(|c| c.1);

        Self {
            preface_lcg_mod16_cycle: cycle,
            golden_weyl_vs_rational_discrepancy: (disc_golden, disc_rational),
            deployed_weyl_discrepancy: deployed_disc,
            mixgamma_min_transition_bits: min_transitions,
            hermite_mehler_eigenvalue_sum: eig_sum,
            dfm_hermite_vs_rff_l2_error: (err_hm, err_rff),
            deployed_dfm_l2_error: deployed_dfm_err,
            mlls_bcts_vs_bbse_kl_divergence: (kl_mlls, kl_bbse),
            classify_and_count_kl_divergence: kl_cc,
            deployed_label_shift_kl: deployed_kl,
            claims,
            discarded_claims,
            identities_closed,
            axis5_verified,
        }
    }
}

/// Multiplies two polynomials given by ascending coefficients.
fn poly_mul(a: &[f64], b: &[f64]) -> Vec<f64> {
    let mut out = vec![0.0; a.len() + b.len() - 1];
    for (i, &ai) in a.iter().enumerate() {
        for (j, &bj) in b.iter().enumerate() {
            out[i + j] += ai * bj;
        }
    }
    out
}

/// Roots of a monic real polynomial (ascending coefficients, leading coefficient `1`) via Durand-Kerner iteration.
fn polynomial_roots(coeffs: &[f64]) -> Vec<C64> {
    let deg = coeffs.len() - 1;
    let eval = |z: C64| -> C64 {
        coeffs[..deg]
            .iter()
            .rev()
            .fold((coeffs[deg], 0.0), |acc, &c| {
                let m = c_mul(acc, z);
                (m.0 + c, m.1)
            })
    };
    let mut roots = Vec::with_capacity(deg);
    let mut seed_root = (1.0, 0.0);
    for _ in 0..deg {
        roots.push(seed_root);
        seed_root = c_mul(seed_root, (0.4, 0.9));
    }
    for _ in 0..1000 {
        for i in 0..deg {
            let zi = roots[i];
            let den = roots
                .iter()
                .enumerate()
                .filter(|&(j, _)| j != i)
                .fold((1.0, 0.0), |acc, (_, &zj)| {
                    c_mul(acc, (zi.0 - zj.0, zi.1 - zj.1))
                });
            let step = c_div(eval(zi), den);
            roots[i] = (zi.0 - step.0, zi.1 - step.1);
        }
    }
    roots
}

/// Spectral radius, excluding the consensus root $z = 1$, of the 3-stage loop $h \to \phi \to a \to h$ with gains $(\delta, \gamma, \eta)$
/// when $h$ reads $a$ with a lag of `delay` steps: $z^{\tau}(z - 1 + \delta)(z - 1 + \gamma)(z - 1 + \eta) - \delta\gamma\eta = 0$.
fn rlhf_nonconsensus_spectral_radius(delta: f64, gamma: f64, eta: f64, delay: usize) -> f64 {
    let mut p = vec![1.0];
    for _ in 0..delay {
        p = poly_mul(&p, &[0.0, 1.0]);
    }
    for gain in [delta, gamma, eta] {
        p = poly_mul(&p, &[gain - 1.0, 1.0]);
    }
    p[0] -= delta * gamma * eta;
    let roots = polynomial_roots(&p);
    let consensus = roots
        .iter()
        .enumerate()
        .min_by(|a, b| {
            let da = (a.1 .0 - 1.0).hypot(a.1 .1);
            let db = (b.1 .0 - 1.0).hypot(b.1 .1);
            da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|(i, _)| i)
        .unwrap_or(0);
    roots
        .iter()
        .enumerate()
        .filter(|&(i, _)| i != consensus)
        .map(|(_, z)| z.0.hypot(z.1))
        .fold(0.0, f64::max)
}

/// Axis 6 ($\mathbb{C} \times \text{Fractals} \times \mathbb{T}^d$): Strange Nonchaotic Attractor Diagnostics on the Quasiperiodically Forced Logistic Map,
/// the Delayed 3-Stage `RLHF` Feedback Loop, Izzo `PerfGD` vs. Repeated Risk Minimization, Phasor `FHRR` / `GC-VSA` Binding,
/// Hasani's `LTC` Fused Solver, and the Kevin Kelly / W. Brian Arthur / Taleb-Douady Technium Heuristics.
#[derive(Debug, Clone)]
pub struct SnaRlhfFhrrCohomologyTechniumAudit {
    /// Identity: Schnakenberg (1976) cycle entropy production $\Pi = \sum_e (J_e^+ - J_e^-)\ln(J_e^+/J_e^-) > 0$ for a driven 3-state cycle.
    pub schnakenberg_cycle_entropy_production_pi: f64,
    /// Phase-sensitivity growth exponent $\mu = \ln(\Gamma_{2000}/\Gamma_{100}) / \ln 20$ with $\Gamma_N = \max_{n \le N}|\partial x_n/\partial\theta|$,
    /// and largest Lyapunov exponent $\lambda$ of $x_{n+1} = a x_n(1 - x_n) + \varepsilon\sin(2\pi\theta_n)$ at $a = 3.0, \varepsilon = 0.1553$.
    pub sna_fractal_exponent_and_negative_lyapunov: (f64, f64),
    /// Orbital synchronization error $|x_N^{(1)} - x_N^{(2)}|$ of two initial conditions driven by the same toroidal phase $\theta_n$.
    pub sna_orbital_synchronization_error: f64,
    /// Number of sign flips of $h - a$ in the undelayed 3-stage `RLHF` loop ($\delta=\gamma=\eta=0.3$ from $(1, 0, -1)$ over 40 steps).
    pub rlhf_delay0_damped_sign_flips: usize,
    /// Non-consensus spectral radius of the `RLHF` loop with $(\delta, \gamma, \eta) = (1.2, 0.7, 0.6)$ at lag $\tau = 0$ vs. $\tau = 1$,
    /// computed from the companion polynomial roots. Informative: the delay-destabilization claim is not gated (see `discarded_findings`).
    pub rlhf_delay0_vs_delayed_spectral_radius: (f64, f64),
    /// Final iterate over $\theta_{\mathrm{PO}}$ of Repeated Risk Minimization vs. oracle-free `PerfGD` on `PerformativeSystem(2, 0.5, 0.5, 1, 1)`.
    pub izzo_rrm_vs_perfgd_ratio_to_opt: (f64, f64),
    /// $|\theta_{\text{final}} - \theta_{\mathrm{PO}}|$ of the deployed optimizer (`PerfGD` if shielded, RRM otherwise). Gated.
    pub deployed_performative_gap_to_opt: f64,
    /// Phasor `FHRR` single-pair unbinding cosine (identity, `1.0`) and 2-pair bundled unbinding cosine on $\mathbb{T}^{128} \subset \mathbb{C}^{128}$.
    pub fhrr_phasor_exact_and_bundled_unbinding_cosine: (f64, f64),
    /// Informative `GC-VSA` scale ratio $s_{i+1}/s_i = 1.42$ and identity path-integration closure error
    /// $\lVert V_X^{\odot 3}\odot V_Y^{\odot 7}\odot V_X^{\odot 2}\odot V_Y^{\odot 1} - V_X^{\odot 5}\odot V_Y^{\odot 8}\rVert_\infty$.
    pub gc_vsa_scale_ratio_and_path_closure_error: (f64, f64),
    /// Informative: circular correlation of a polar-angle baseline on a noisy circle. Perea's persistent-cohomology circular
    /// coordinates $H^1(X;\mathbb{Z}) \cong [X, S^1]$ are not implemented here, so this value is never gated.
    pub polar_baseline_circular_coordinate_correlation: f64,
    /// Informative: Hasani et al. (AAAI 2021) latent trajectory lengths as transcribed from the literature (`LTC`, Neural ODE, CT-RNN).
    pub hasani_ltc_vs_neural_ode_and_ctrnn_trajectory_lengths: (f64, f64, f64),
    /// Maximum $|x(t)|$ of the `LTC` Fused Solver vs. explicit Euler at the stiff step $\Delta t = 2.5$.
    pub ltc_fused_solver_max_state_vs_euler: (f64, f64),
    /// Informative: maximum $|x(t)|$ of the deployed solver (fused if shielded, Euler otherwise). The fused-solver bound
    /// $|x| \le A$ for positive conductance is a theorem and is checked under `identities_closed`.
    pub deployed_solver_max_state: f64,
    /// Informative Arthur combinatorial growth factor $M_8 / M_0$, and identity Taleb-Douady signs
    /// $H(x, \Delta) = \frac{f(x-\Delta)+f(x+\Delta)}{2} - f(x)$ (`(concave < 0, convex > 0)`).
    pub technium_arthur_growth_and_taleb_fragility_h: (f64, f64, f64),
    /// Claims evaluated against computed ground truth that did not survive (Prohibition 4: recorded, never gated).
    pub discarded_findings: Vec<&'static str>,
    /// Named claim checks gated by this axis (see `crate::acceptance`).
    pub claims: Vec<NamedCheck>,
    /// True if the Schnakenberg, phasor, path-closure, `LTC` bound, and Taleb-Douady identities close.
    pub identities_closed: bool,
    /// True if identities close, the mode-independent dynamical claims hold, and the deployed optimizer passes its check.
    pub axis6_verified: bool,
}

impl SnaRlhfFhrrCohomologyTechniumAudit {
    /// Evaluates Axis 6: Schnakenberg thermodynamics, forced-logistic `SNA` diagnostics, `RLHF` loop spectra,
    /// `PerfGD` vs. RRM, Phasor `FHRR`/`GC-VSA`, `LTC` Fused Solver vs. Euler, and Technium heuristics.
    pub fn evaluate(seed: u64, shielded: bool) -> Self {
        let mut rng = Rng::new(seed ^ 0xC04D_BEEF_2026_0006);

        // 1. Schnakenberg (1976) Network Thermodynamics cycle entropy production \Pi = \sum_e (J_e^+ - J_e^-) \ln(J_e^+ / J_e^-) > 0
        // across a 3-state non-equilibrium Markov cycle 1 -> 2 -> 3 -> 1 driven by external chemical potential affinity.
        let j_plus = [1.8_f64, 1.5_f64, 2.1_f64];
        let j_minus = [0.6_f64, 0.5_f64, 0.7_f64];
        let schnakenberg_pi: f64 = j_plus
            .iter()
            .zip(j_minus.iter())
            .map(|(&jp, &jm)| (jp - jm) * (jp / jm).ln())
            .sum();

        // 2. Quasiperiodically forced logistic map x_{n+1} = a x_n (1 - x_n) + \varepsilon \sin(2\pi \theta_n),
        // \theta_{n+1} = (\theta_n + \omega) mod 1, golden rotation \omega = (\sqrt{5}-1)/2, a = 3.0, \varepsilon = 0.1553.
        // Lyapunov exponent \lambda = <\ln|a (1 - 2 x_n)|>, phase sensitivity \partial x_{n+1}/\partial\theta = a(1-2x_n) \partial x_n/\partial\theta + 2\pi\varepsilon\cos(2\pi\theta_n).
        let a_sna = 3.0_f64;
        let eps_sna = 0.1553_f64;
        let omega_gold = (5.0_f64.sqrt() - 1.0) * 0.5;
        let mut x1 = 0.42_f64;
        let mut x2 = 0.78_f64;
        let mut dx_dtheta = 0.0_f64;
        let mut lyap_sum = 0.0_f64;
        let n_sna = 2000usize;
        let mut gamma_running = 0.0_f64;
        let mut gamma_early = 0.0_f64;
        for step in 0..n_sna {
            let theta = ((step as f64) * omega_gold).fract();
            let deriv = a_sna * (1.0 - 2.0 * x1);
            lyap_sum += deriv.abs().max(1e-300).ln();
            dx_dtheta = deriv * dx_dtheta + 2.0 * PI * eps_sna * (2.0 * PI * theta).cos();
            x1 = a_sna * x1 * (1.0 - x1) + eps_sna * (2.0 * PI * theta).sin();
            x2 = a_sna * x2 * (1.0 - x2) + eps_sna * (2.0 * PI * theta).sin();
            gamma_running = gamma_running.max(dx_dtheta.abs());
            if step + 1 == 100 {
                gamma_early = gamma_running;
            }
        }
        let lyap_sna = lyap_sum / (n_sna as f64);
        let fractal_exp = (gamma_running / gamma_early.max(1e-300)).ln() / 20.0_f64.ln();
        let sync_err = (x1 - x2).abs();

        // 3. Delayed 3-Stage RLHF Feedback Loop:
        // Stage 1: \tau = 0, \delta = \gamma = \eta = 0.3 from (h_0, \phi_0, a_0) = (1, 0, -1) over 40 steps.
        let mut h = 1.0_f64;
        let mut phi = 0.0_f64;
        let mut a_val = -1.0_f64;
        let mut sign_flips = 0usize;
        let initial_gap = h - a_val;
        let mut prev_diff = initial_gap;
        for _ in 0..40 {
            let h_next = h + 0.3 * (a_val - h);
            let phi_next = phi + 0.3 * (h - phi);
            let a_next = a_val + 0.3 * (phi - a_val);
            h = h_next;
            phi = phi_next;
            a_val = a_next;
            let diff = h - a_val;
            if diff * prev_diff < 0.0 {
                sign_flips += 1;
            }
            prev_diff = diff;
        }
        let rlhf_damped = sign_flips >= 2 && prev_diff.abs() < initial_gap.abs();
        // Stage 2: (\delta, \gamma, \eta) = (1.2, 0.7, 0.6); companion polynomial roots at lag 0 and lag 1 (informative).
        let rho_delay0 = rlhf_nonconsensus_spectral_radius(1.2, 0.7, 0.6, 0);
        let rho_delayed = rlhf_nonconsensus_spectral_radius(1.2, 0.7, 0.6, 1);

        // 4. Izzo, Ying & Zou (ICML 2021): oracle-free PerfGD vs. Repeated Risk Minimization on a performative quadratic.
        let perf = PerformativeSystem::new(2.0, 0.5, 0.5, 1.0, 1.0);
        let theta_po = perf.diagnose().theta_po;
        let perfgd_final = *perf
            .perfgd_finite_difference_trajectory(0.0, 1.0, 0.5, 60)
            .last()
            .unwrap_or(&0.0);
        let mut rrm_final = 0.0_f64;
        for _ in 0..60 {
            rrm_final = perf.rrm_update_step(rrm_final);
        }
        let deployed_theta = if shielded { perfgd_final } else { rrm_final };
        let deployed_perf_gap = (deployed_theta - theta_po).abs();

        // 5. Phasor FHRR (\mathbb{T}^D \subset \mathbb{C}^D, D = 128) & Entorhinal GC-VSA (s_i = s_{\min} * 1.42^i):
        let dim_vsa = 128usize;
        let mut a_ang = vec![0.0_f64; dim_vsa];
        let mut b_ang = vec![0.0_f64; dim_vsa];
        let mut c_ang = vec![0.0_f64; dim_vsa];
        let mut d_ang = vec![0.0_f64; dim_vsa];
        for j in 0..dim_vsa {
            a_ang[j] = 2.0 * PI * rng.next_f64() - PI;
            b_ang[j] = 2.0 * PI * rng.next_f64() - PI;
            c_ang[j] = 2.0 * PI * rng.next_f64() - PI;
            d_ang[j] = 2.0 * PI * rng.next_f64() - PI;
        }
        // Single-pair exact unbinding: (a \odot b) \odot b^* has phase (a_j + b_j - b_j) = a_j \implies Hermitian similarity = 1.0
        let exact_unbind_cos: f64 = (0..dim_vsa)
            .map(|j| ((a_ang[j] + b_ang[j] - b_ang[j]) - a_ang[j]).cos())
            .sum::<f64>()
            / (dim_vsa as f64);

        // Bundled 2-pair memory S = a \odot b + c \odot d; unbind with b^* -> S \odot b^* = a + c \odot d \odot b^*
        let mut bundled_cos_sum = 0.0_f64;
        for j in 0..dim_vsa {
            let s_re = (a_ang[j] + b_ang[j]).cos() + (c_ang[j] + d_ang[j]).cos();
            let s_im = (a_ang[j] + b_ang[j]).sin() + (c_ang[j] + d_ang[j]).sin();
            // Multiply by exp(-i b_ang[j]) and normalize to unit phasor on T^D
            let u_re = s_re * b_ang[j].cos() + s_im * b_ang[j].sin();
            let u_im = s_im * b_ang[j].cos() - s_re * b_ang[j].sin();
            let u_phase = u_im.atan2(u_re);
            bundled_cos_sum += (u_phase - a_ang[j]).cos();
        }
        let bundled_unbind_cos = bundled_cos_sum / (dim_vsa as f64);

        // Entorhinal GC-VSA (March 2025, arXiv:2503.08608): s_i = s_min * 1.42^i and 2D path integration
        // V_X^{\odot 3} \odot V_Y^{\odot 7} \odot V_X^{\odot 2} \odot V_Y^{\odot 1} == V_X^{\odot 5} \odot V_Y^{\odot 8}
        let gc_scale_ratio = 1.42_f64;
        let mut max_path_err = 0.0_f64;
        for j in 0..dim_vsa {
            let vx = a_ang[j];
            let vy = b_ang[j];
            let step_by_step = 3.0 * vx + 7.0 * vy + 2.0 * vx + 1.0 * vy;
            let direct = 5.0 * vx + 8.0 * vy;
            let err = (step_by_step.cos() - direct.cos()).hypot(step_by_step.sin() - direct.sin());
            max_path_err = max_path_err.max(err);
        }

        // 6. Polar-angle baseline on a noisy circle point cloud in \mathbb{R}^2 (informative; not Perea's cohomology algorithm).
        let n_circle = 32usize;
        let circ_corr: f64 = (0..n_circle)
            .map(|i| {
                let ang = 2.0 * PI * (i as f64) / (n_circle as f64);
                let x_pt = ang.cos() + 0.03 * (3.0 * ang).sin();
                let y_pt = ang.sin() + 0.03 * (3.0 * ang).cos();
                (ang - y_pt.atan2(x_pt).rem_euclid(2.0 * PI)).cos()
            })
            .sum::<f64>()
            / (n_circle as f64);

        // 7. Hasani et al. (AAAI 2021) Liquid Time-Constant (LTC) Fused Solver vs. explicit Euler at a stiff step:
        // x(t + \Delta t) = [x(t) + \Delta t f(x, I, t, \theta) A] / [1 + \Delta t (1/\tau + f(x, I, t, \theta))]
        let ltc_lengths = (463.1858_f64, 102.9654_f64, 39.9587_f64);
        let tau_base = 1.0_f64;
        let a_rev = 2.0_f64;
        let dt_stiff = 2.5_f64;
        let mut x_ltc = 0.5_f64;
        let mut x_euler = 0.5_f64;
        let mut max_ltc_abs = 0.5_f64;
        let mut max_euler_abs = 0.5_f64;
        for step in 0..20 {
            let f_gate = 1.2 + 0.8 * ((step as f64) * 0.5).sin().abs(); // Positive synaptic conductance f >= 1.2
            x_ltc =
                (x_ltc + dt_stiff * f_gate * a_rev) / (1.0 + dt_stiff * (1.0 / tau_base + f_gate));
            let dx_dt = -(1.0 / tau_base + f_gate) * x_euler + f_gate * a_rev;
            x_euler += dt_stiff * dx_dt;
            max_ltc_abs = max_ltc_abs.max(x_ltc.abs());
            max_euler_abs = max_euler_abs.max(x_euler.abs());
        }
        let deployed_max_state = if shielded { max_ltc_abs } else { max_euler_abs };

        // 8. Kevin Kelly's Technium / W. Brian Arthur's combinatorial autopoiesis (informative recurrence) and
        // Taleb-Douady model-free fragility H(x, \Delta) = [f(x-\Delta) + f(x+\Delta)]/2 - f(x) (Jensen sign identity).
        let mut m_tech = 10.0_f64;
        for _ in 0..8 {
            let pairs = 0.5 * m_tech * (m_tech - 1.0);
            m_tech += 0.015 * pairs;
        }
        let arthur_growth = m_tech / 10.0;
        let delta_shock = 1.0_f64;
        let f_fragile = |s: f64| -s.exp();
        let f_convex = |s: f64| s.exp();
        let h_fragile = 0.5 * (f_fragile(-delta_shock) + f_fragile(delta_shock)) - f_fragile(0.0);
        let h_antifragile = 0.5 * (f_convex(-delta_shock) + f_convex(delta_shock)) - f_convex(0.0);

        let mut discarded_findings = Vec::new();
        if rho_delayed <= 1.0 {
            discarded_findings.push(
                "a one-step lag pushes the (1.2, 0.7, 0.6) RLHF loop past the unit circle (companion radius > 1)",
            );
        }

        let identities_closed = schnakenberg_pi > 0.0
            && (exact_unbind_cos - 1.0).abs() < 1e-12
            && max_path_err < 1e-12
            && h_fragile < 0.0
            && h_antifragile > 0.0
            && max_ltc_abs <= a_rev;
        let claims: Vec<NamedCheck> = vec![
            ("triad.a6.det.sna_lyapunov_below_-0.04", lyap_sna < -0.04),
            (
                "triad.a6.det.sna_phase_growth_exponent_above_0.15",
                fractal_exp > 0.15,
            ),
            ("triad.a6.det.sna_sync_error_below_1e-10", sync_err < 1e-10),
            (
                "triad.a6.det.rlhf_at_least_2_sign_flips_and_decay",
                rlhf_damped,
            ),
            (
                "triad.a6.det.rlhf_undelayed_radius_below_1",
                rho_delay0 < 1.0,
            ),
            (
                "triad.a6.fhrr_bundled_unbinding_above_0.55",
                bundled_unbind_cos > 0.55,
            ),
            (
                "triad.a6.det.deployed_performative_gap_below_1e-3",
                deployed_perf_gap < 1e-3,
            ),
        ];
        let axis6_verified = identities_closed && claims.iter().all(|c| c.1);

        Self {
            schnakenberg_cycle_entropy_production_pi: schnakenberg_pi,
            sna_fractal_exponent_and_negative_lyapunov: (fractal_exp, lyap_sna),
            sna_orbital_synchronization_error: sync_err,
            rlhf_delay0_damped_sign_flips: sign_flips,
            rlhf_delay0_vs_delayed_spectral_radius: (rho_delay0, rho_delayed),
            izzo_rrm_vs_perfgd_ratio_to_opt: (rrm_final / theta_po, perfgd_final / theta_po),
            deployed_performative_gap_to_opt: deployed_perf_gap,
            fhrr_phasor_exact_and_bundled_unbinding_cosine: (exact_unbind_cos, bundled_unbind_cos),
            gc_vsa_scale_ratio_and_path_closure_error: (gc_scale_ratio, max_path_err),
            polar_baseline_circular_coordinate_correlation: circ_corr,
            hasani_ltc_vs_neural_ode_and_ctrnn_trajectory_lengths: ltc_lengths,
            ltc_fused_solver_max_state_vs_euler: (max_ltc_abs, max_euler_abs),
            deployed_solver_max_state: deployed_max_state,
            technium_arthur_growth_and_taleb_fragility_h: (arthur_growth, h_fragile, h_antifragile),
            discarded_findings,
            claims,
            identities_closed,
            axis6_verified,
        }
    }
}

/// Complete 6-Bridge Triadic Mathematical & Physical Synthesis ($\mathbb{C} \times \text{Fractals} \times \mathbb{T}^d$),
/// Technium / Technosphere Evolutionary Ontology, and Spring-and-Horizon Multidimensional Spiral.
#[derive(Debug, Clone)]
pub struct ComplexFractalToroidalTechniumSynthesis {
    /// Axis 1 ($\mathbb{C}$): Complex Gabor Wavelets (`WIRE` with learned frequency, `Improved SIREN`) on 2D Helmholtz PDE & Adjoint Photonic Backpropagation.
    pub complex_wave_gabor_helmholtz: ComplexWaveGaborHelmholtzAudit,
    /// Axis 2 ($\mathbb{C} \leftrightarrow \text{Fractals}$): $\mathbb{C}^{n\times n} \hookrightarrow \mathbb{R}^{2n\times 2n}$ Isomorphism, Two-Stage `BlockAMC`, & Hierarchical Kernel Compression.
    pub complex_real_isomorphism_hmatrix: ComplexRealIsomorphismHMatrixAudit,
    /// Axis 3 ($\text{Fractals} \leftrightarrow \mathbb{R}^n$): Atomic Conductance $G_0$, PCM Drift, Simulated `CTRW` Scaling, & Analog ECC Decoding.
    pub fractional_ctrw_analog_ecc: FractionalCtrwAnalogEccAudit,
    /// Axis 4 ($\text{Fractals} \leftrightarrow \text{Scaling Laws}$): Multifractal Cascade, Eichi `PDHTE+JK` & `TailWelsch-DML`, & $\pi^2/6$ Bound.
    pub multifractal_pdhte_rag_collapse: MultifractalPdhteRagCollapseAudit,
    /// Axis 5 ($\mathbb{T}^d \leftrightarrow \mathbb{C}$): Preface $\mathbb{Z}/16\mathbb{Z}$ Cycle, Weyl `mixGamma`, Hermite-Mehler `DFM`, & `MLLS+BCTS`.
    pub weyl_torus_hermite_mehler_dfm: WeylTorusHermiteMehlerDfmAudit,
    /// Axis 6 ($\mathbb{C} \times \text{Fractals} \times \mathbb{T}^d$): Forced-Logistic `SNA`, `RLHF` Loop, `PerfGD`, `FHRR`/`GC-VSA`, `LTC` Fused Solver, & Technium Heuristics.
    pub sna_rlhf_fhrr_cohomology_technium: SnaRlhfFhrrCohomologyTechniumAudit,
    /// True if every axis's identity checks close (expected in both modes).
    pub all_identities_closed: bool,
    /// Discarded findings collected from Axes 4 and 6 (Prohibition 4).
    pub discarded_findings: Vec<&'static str>,
    /// True if all 6 triadic bridges pass simultaneously.
    pub all_six_bridges_verified: bool,
}

impl ComplexFractalToroidalTechniumSynthesis {
    /// Evaluates the canonical shielded Triadic & Technium Synthesis under `seed = 2026`.
    pub fn evaluate_canonical(seed: u64) -> Self {
        Self::evaluate_with_mode(seed, true)
    }

    /// Evaluates the Triadic & Technium Synthesis deploying either the CAPIBA methods (`shielded = true`)
    /// or the naive methods (`shielded = false`, used by `--defect triadic_technium`).
    pub fn evaluate_with_mode(seed: u64, shielded: bool) -> Self {
        let a1 = ComplexWaveGaborHelmholtzAudit::evaluate(seed, shielded);
        let a2 = ComplexRealIsomorphismHMatrixAudit::evaluate(seed, shielded);
        let a3 = FractionalCtrwAnalogEccAudit::evaluate(seed, shielded);
        let a4 = MultifractalPdhteRagCollapseAudit::evaluate(seed, shielded);
        let a5 = WeylTorusHermiteMehlerDfmAudit::evaluate(seed, shielded);
        let a6 = SnaRlhfFhrrCohomologyTechniumAudit::evaluate(seed, shielded);
        let all_identities_closed = a1.identities_closed
            && a2.identities_closed
            && a3.identities_closed
            && a4.identities_closed
            && a5.identities_closed
            && a6.identities_closed;
        let all_ok = a1.axis1_verified
            && a2.axis2_verified
            && a3.axis3_verified
            && a4.axis4_verified
            && a5.axis5_verified
            && a6.axis6_verified;
        let discarded_findings = a4
            .discarded_findings
            .iter()
            .chain(&a6.discarded_findings)
            .copied()
            .collect();
        Self {
            complex_wave_gabor_helmholtz: a1,
            complex_real_isomorphism_hmatrix: a2,
            fractional_ctrw_analog_ecc: a3,
            multifractal_pdhte_rag_collapse: a4,
            weyl_torus_hermite_mehler_dfm: a5,
            sna_rlhf_fhrr_cohomology_technium: a6,
            all_identities_closed,
            discarded_findings,
            all_six_bridges_verified: all_ok,
        }
    }

    /// Claims removed from the gate after failing multi-seed acceptance.
    pub fn discarded_claim_checks(&self) -> Vec<NamedCheck> {
        self.weyl_torus_hermite_mehler_dfm.discarded_claims.clone()
    }

    /// All gated claims of the six bridges, in axis order.
    pub fn claim_checks(&self) -> Vec<NamedCheck> {
        [
            &self.complex_wave_gabor_helmholtz.claims,
            &self.complex_real_isomorphism_hmatrix.claims,
            &self.fractional_ctrw_analog_ecc.claims,
            &self.multifractal_pdhte_rag_collapse.claims,
            &self.weyl_torus_hermite_mehler_dfm.claims,
            &self.sna_rlhf_fhrr_cohomology_technium.claims,
        ]
        .into_iter()
        .flatten()
        .copied()
        .collect()
    }

    /// Per-claim pass rates over `seeds` in the given mode (see `crate::acceptance`).
    pub fn multi_seed_acceptance(seeds: &[u64], shielded: bool) -> MultiSeedAcceptance {
        MultiSeedAcceptance::evaluate(seeds, |seed| {
            let syn = Self::evaluate_with_mode(seed, shielded);
            (syn.claim_checks(), syn.all_identities_closed)
        })
    }
}

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
        // 1. vic_elec.csv: phase-amplitude wave R^2
        let emp_wave = crate::analog::EmpiricalWaveAnalogAudit::evaluate_series(vic_demand);
        let vic_r2 = emp_wave.wave_explained_variance_r2;

        // 2. danish_fire.csv: Eichi (2026) PDHTE+JK tail index \hat{\xi}_{PDHTE+JK} and Hill plateau CV(t_0)
        let (xi_fire_jk, _xi_fire_naive, cv_fire, _acc_fire) =
            MultifractalPdhteRagCollapseAudit::evaluate_pdhte_jk_tail(fire_losses);

        // 3. atlantic_storms.csv (chronological): first half is the labeled source, second half the unlabeled target.
        // Forecasts are standardized on the source and mapped to Hermite-Mehler features; the target prior is the projection
        // of the target feature mean onto the segment between the source class-conditional means.
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

        // 4. djia_1885_2023.csv: Empirical MMAR multifractal concavity across scales \Delta t \in {1, 16}
        // Estimate \zeta(q) for q \in {1, 2, 3}; multifractal intermittency forces \zeta(3) - 2\zeta(2) + \zeta(1) < 0.
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

        // 5. bank_calls.csv: LTC Fused Solver driven by daily call intensities (input-independent boundedness identity)
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

#[cfg(test)]
mod tests {
    use super::*;
    use stochology_core::rng::DEFAULT_SEED;

    #[test]
    fn test_triadic_synthesis_shielded_and_unshielded() {
        let shielded = ComplexFractalToroidalTechniumSynthesis::evaluate_canonical(DEFAULT_SEED);
        assert!(
            shielded.complex_wave_gabor_helmholtz.axis1_verified,
            "a1={:?}",
            shielded.complex_wave_gabor_helmholtz
        );
        assert!(
            shielded.complex_real_isomorphism_hmatrix.axis2_verified,
            "a2={:?}",
            shielded.complex_real_isomorphism_hmatrix
        );
        assert!(
            shielded.fractional_ctrw_analog_ecc.axis3_verified,
            "a3={:?}",
            shielded.fractional_ctrw_analog_ecc
        );
        assert!(
            shielded.multifractal_pdhte_rag_collapse.axis4_verified,
            "a4={:?}",
            shielded.multifractal_pdhte_rag_collapse
        );
        assert!(
            shielded.weyl_torus_hermite_mehler_dfm.axis5_verified,
            "a5={:?}",
            shielded.weyl_torus_hermite_mehler_dfm
        );
        assert!(
            shielded.sna_rlhf_fhrr_cohomology_technium.axis6_verified,
            "a6={:?}",
            shielded.sna_rlhf_fhrr_cohomology_technium
        );
        assert!(shielded.all_identities_closed);
        assert!(shielded.all_six_bridges_verified);
        // Cascade curvature tolerance, PDHTE+JK superiority, and lag-induced RLHF instability are refuted at seed 2026.
        assert_eq!(
            shielded.discarded_findings.len(),
            3,
            "{:?}",
            shielded.discarded_findings
        );

        // The naive methods fail on their own outputs while every identity still closes.
        let unshielded =
            ComplexFractalToroidalTechniumSynthesis::evaluate_with_mode(DEFAULT_SEED, false);
        assert!(unshielded.all_identities_closed);
        assert!(!unshielded.all_six_bridges_verified);
        assert!(!unshielded.complex_wave_gabor_helmholtz.axis1_verified);
        assert!(!unshielded.complex_real_isomorphism_hmatrix.axis2_verified);
        assert!(!unshielded.fractional_ctrw_analog_ecc.axis3_verified);
        assert!(!unshielded.multifractal_pdhte_rag_collapse.axis4_verified);
        assert!(!unshielded.weyl_torus_hermite_mehler_dfm.axis5_verified);
        assert!(!unshielded.sna_rlhf_fhrr_cohomology_technium.axis6_verified);
    }

    #[test]
    fn test_fit_bcts_recovers_calibration_and_refuses_degenerate_labels() {
        // Logits z = 4x - 0.5 of x | y ~ N(2y - 1, 1) with balanced labels: the calibrated map is sigma(z / 2 + 0.25).
        let mut rng = Rng::new(DEFAULT_SEED);
        let (mut z, mut y) = (Vec::new(), Vec::new());
        for _ in 0..20_000 {
            let label = if rng.next_f64() < 0.5 { 1.0 } else { 0.0 };
            z.push(4.0 * ((2.0 * label - 1.0) + rng.standard_normal()) - 0.5);
            y.push(label);
        }
        let (a, b) = fit_bcts(&z, &y).expect("converges on non-separable data");
        assert!(
            (a - 0.5).abs() < 0.03 && (b - 0.25).abs() < 0.05,
            "a={a} b={b}"
        );
        // Constant labels have no finite maximum-likelihood calibration.
        assert!(fit_bcts(&z, &vec![1.0; z.len()]).is_none());
    }

    #[test]
    fn test_rlhf_companion_roots_match_undelayed_closed_form() {
        // At lag 0 the non-consensus roots satisfy \mu^2 + 2.5\mu + 1.98 = 0 with z = 1 + \mu, so |z| = \sqrt{0.48}.
        let rho0 = rlhf_nonconsensus_spectral_radius(1.2, 0.7, 0.6, 0);
        assert!((rho0 - 0.48_f64.sqrt()).abs() < 1e-9, "rho0={rho0}");
    }
}
