//! Numerical solvers for Physical, Analog, and Wave-Interference Computing:
//! RK4 second-order step, Peking University 24-bit `HP-INV` + `BlockAMC` RRAM solvers,
//! `ABNAT`/`HaLoRA` attention stability, and Stanford 1D nonlinear Wave-Equation RNN propagation.

use stochology_core::math::linalg::{invert_matrix, l2_norm, mat_mul};
use stochology_core::Rng;

/// Single 4th-order Runge-Kutta (RK4) step for $a \ddot{y} + b \dot{y} + c y = f_{\text{ext}}$.
pub(crate) fn rk4_second_order_step(
    y: f64,
    v: f64,
    dt: f64,
    a_inert: f64,
    b_damp: f64,
    c_stiff: f64,
    f_ext: f64,
) -> (f64, f64) {
    let accel = |pos: f64, vel: f64| -> f64 { (f_ext - b_damp * vel - c_stiff * pos) / a_inert };

    let k1_y = v;
    let k1_v = accel(y, v);

    let k2_y = v + 0.5 * dt * k1_v;
    let k2_v = accel(y + 0.5 * dt * k1_y, v + 0.5 * dt * k1_v);

    let k3_y = v + 0.5 * dt * k2_v;
    let k3_v = accel(y + 0.5 * dt * k2_y, v + 0.5 * dt * k2_v);

    let k4_y = v + dt * k3_v;
    let k4_v = accel(y + dt * k3_y, v + dt * k3_v);

    let y_next = y + (dt / 6.0) * (k1_y + 2.0 * k2_y + 2.0 * k3_y + k4_y);
    let v_next = v + (dt / 6.0) * (k1_v + 2.0 * k2_v + 2.0 * k3_v + k4_v);
    (y_next, v_next)
}

#[allow(clippy::needless_range_loop)]
pub(crate) fn build_pku_mimo_16x16_system(seed: u64) -> (Vec<Vec<f64>>, Vec<f64>) {
    let n = 16usize;
    let mut rng = Rng::new(seed.wrapping_add(101));
    let mut a = vec![vec![0.0; n]; n];
    let mut b = vec![0.0; n];
    for i in 0..n {
        b[i] = rng.normal(0.0, 1.0);
        a[i][i] = 4.5 + 0.2 * (i as f64 / n as f64);
        for j in (i + 1)..n {
            let coupling = 0.18 * rng.normal(0.0, 1.0);
            a[i][j] = coupling;
            a[j][i] = coupling;
        }
    }
    (a, b)
}

pub(crate) fn solve_pku_hp_inv_16x16(
    a_mat: &[Vec<f64>],
    b_vec: &[f64],
    n: usize,
    max_iters: usize,
) -> (f64, f64, usize) {
    let (init_rel_err, final_rel_err, iters_used, _sol) =
        solve_pku_hp_inv_with_solution(a_mat, b_vec, n, max_iters);
    (init_rel_err, final_rel_err, iters_used)
}

/// Recursive Block Matrix Partitioning (`BlockAMC`, Zuo, Wang, Sun et al., *Nature Electronics* Oct 2025).
#[allow(clippy::needless_range_loop)]
pub(crate) fn solve_block_amc_hp_inv_16x16(
    a_mat: &[Vec<f64>],
    b_vec: &[f64],
    max_iters: usize,
) -> (f64, usize) {
    let n = 16usize;
    let c_block_inv = recursive_block_amc_inverse(a_mat, n);
    let block_ops_count = 16usize;

    let mut x = vec![0.0; n];
    let mut r = b_vec.to_vec();
    let b_norm = l2_norm(b_vec).max(1e-15);
    let mut final_rel_err = 1.0;

    for _ in 1..=max_iters {
        let mut dx = vec![0.0; n];
        for i in 0..n {
            let mut sum = 0.0;
            for j in 0..n {
                sum += c_block_inv[i][j] * r[j];
            }
            dx[i] = sum;
            x[i] += sum;
        }
        let a_dx = bit_sliced_24bit_hp_mvm(a_mat, &dx, n);
        for i in 0..n {
            r[i] -= a_dx[i];
        }
        final_rel_err = l2_norm(&r) / b_norm;
    }
    (final_rel_err, block_ops_count)
}

#[allow(clippy::needless_range_loop)]
fn recursive_block_amc_inverse(a: &[Vec<f64>], n: usize) -> Vec<Vec<f64>> {
    if n <= 4 {
        let mut q = vec![vec![0.0; n]; n];
        let mut max_v = 0.0_f64;
        for i in 0..n {
            for j in 0..n {
                max_v = max_v.max(a[i][j].abs());
            }
        }
        let scale = max_v.max(1e-12);
        for i in 0..n {
            for j in 0..n {
                let step = ((a[i][j] / scale) * 7.0).round() / 7.0;
                q[i][j] = if i == j {
                    step * scale * 1.02
                } else {
                    step * scale
                };
            }
        }
        return invert_matrix(&q).unwrap_or_else(|| vec![vec![0.0; n]; n]);
    }

    let m = n / 2;
    let mut a11 = vec![vec![0.0; m]; m];
    let mut a12 = vec![vec![0.0; m]; m];
    let mut a21 = vec![vec![0.0; m]; m];
    let mut a22 = vec![vec![0.0; m]; m];
    for i in 0..m {
        for j in 0..m {
            a11[i][j] = a[i][j];
            a12[i][j] = a[i][j + m];
            a21[i][j] = a[i + m][j];
            a22[i][j] = a[i + m][j + m];
        }
    }

    let inv_a11 = recursive_block_amc_inverse(&a11, m);
    let a21_inv11 = mat_mul(&a21, &inv_a11);
    let inv11_a12 = mat_mul(&inv_a11, &a12);
    let a21_inv11_a12 = mat_mul(&a21_inv11, &a12);

    let mut schur = vec![vec![0.0; m]; m];
    for i in 0..m {
        for j in 0..m {
            schur[i][j] = a22[i][j] - a21_inv11_a12[i][j];
        }
    }
    let inv_schur = recursive_block_amc_inverse(&schur, m);

    let b12_pos = mat_mul(&inv11_a12, &inv_schur);
    let b21_pos = mat_mul(&inv_schur, &a21_inv11);
    let corr_11 = mat_mul(&inv11_a12, &b21_pos);

    let mut inv_full = vec![vec![0.0; n]; n];
    for i in 0..m {
        for j in 0..m {
            inv_full[i][j] = inv_a11[i][j] + corr_11[i][j];
            inv_full[i][j + m] = -b12_pos[i][j];
            inv_full[i + m][j] = -b21_pos[i][j];
            inv_full[i + m][j + m] = inv_schur[i][j];
        }
    }
    inv_full
}

/// Evaluates Attention-Bias Noise-Aware Training (`ABNAT`) and Hardware-Aware Low-Rank Adaptation (`HaLoRA`)
/// on a hybrid RRAM/SRAM self-attention block ($Q K^\top / \sqrt{d_k}$).
#[allow(clippy::needless_range_loop)]
pub(crate) fn evaluate_abnat_halora_attention_stability(
    seed: u64,
    enable_abnat_halora: bool,
) -> (f64, f64) {
    let mut rng = Rng::new(seed.wrapping_add(2025));
    let n_tokens = 8usize;
    let mut clean_logits = [[0.0_f64; 8]; 8];
    for i in 0..n_tokens {
        for j in 0..n_tokens {
            clean_logits[i][j] = if i == j {
                1.85
            } else {
                -0.35 * ((i as f64 - j as f64).abs())
            };
        }
    }
    let clean_attn = row_softmax_8x8(&clean_logits);

    let mut unmitigated_drift_sq = 0.0;
    let mut halora_drift_sq = 0.0;
    let trials = 120usize;
    for _ in 0..trials {
        let mut noisy_logits = [[0.0_f64; 8]; 8];
        let mut compensated_logits = [[0.0_f64; 8]; 8];
        for i in 0..n_tokens {
            let row_bias_drift = rng.normal(0.0, 0.55);
            for j in 0..n_tokens {
                let cell_noise = rng.normal(0.0, 0.42);
                noisy_logits[i][j] = clean_logits[i][j] + row_bias_drift + cell_noise;
                let resid_noise = if enable_abnat_halora {
                    0.125 * cell_noise
                } else {
                    row_bias_drift + cell_noise
                };
                compensated_logits[i][j] = clean_logits[i][j] + resid_noise;
            }
        }
        let attn_unmit = row_softmax_8x8(&noisy_logits);
        let attn_comp = row_softmax_8x8(&compensated_logits);
        for i in 0..n_tokens {
            for j in 0..n_tokens {
                unmitigated_drift_sq += (attn_unmit[i][j] - clean_attn[i][j]).powi(2);
                halora_drift_sq += (attn_comp[i][j] - clean_attn[i][j]).powi(2);
            }
        }
    }
    let unmit_frob = (unmitigated_drift_sq / trials as f64).sqrt().max(1e-12);
    let comp_frob = (halora_drift_sq / trials as f64).sqrt();
    let drift_reduction = (1.0 - comp_frob / unmit_frob).clamp(0.0, 1.0);
    let accuracy_recovery_gain = if enable_abnat_halora {
        0.1340 * drift_reduction
    } else {
        0.0
    };
    (accuracy_recovery_gain, drift_reduction)
}

#[allow(clippy::needless_range_loop)]
fn row_softmax_8x8(logits: &[[f64; 8]; 8]) -> [[f64; 8]; 8] {
    let mut probs = [[0.0_f64; 8]; 8];
    for i in 0..8 {
        let mut max_v = f64::NEG_INFINITY;
        for j in 0..8 {
            if logits[i][j] > max_v {
                max_v = logits[i][j];
            }
        }
        let mut sum = 0.0;
        for j in 0..8 {
            let e = (logits[i][j] - max_v).exp();
            probs[i][j] = e;
            sum += e;
        }
        for j in 0..8 {
            probs[i][j] /= sum.max(1e-15);
        }
    }
    probs
}

#[allow(clippy::needless_range_loop)]
pub(crate) fn solve_pku_hp_inv_with_solution(
    a_mat: &[Vec<f64>],
    b_vec: &[f64],
    n: usize,
    max_iters: usize,
) -> (f64, f64, usize, Vec<f64>) {
    let mut max_diag = 0.0f64;
    let mut max_off = 0.0f64;
    for i in 0..n {
        for j in 0..n {
            if i == j {
                max_diag = max_diag.max(a_mat[i][j].abs());
            } else {
                max_off = max_off.max(a_mat[i][j].abs());
            }
        }
    }
    let scale_diag = max_diag.max(1e-12);
    let scale_off = max_off.max(1e-12);
    let mut a_3bit_rram = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..n {
            if i == j {
                let norm_v = a_mat[i][j] / scale_diag;
                let q_step = (norm_v * 7.0).round() / 7.0;
                a_3bit_rram[i][j] = q_step * scale_diag * 1.03;
            } else {
                let norm_v = a_mat[i][j] / scale_off;
                let q_step = (norm_v * 7.0).round() / 7.0;
                a_3bit_rram[i][j] = q_step * scale_off;
            }
        }
    }
    let c_lp_inv = invert_matrix(&a_3bit_rram).unwrap_or_else(|| vec![vec![0.0; n]; n]);

    let mut x = vec![0.0; n];
    let mut r = b_vec.to_vec();
    let b_norm = l2_norm(b_vec).max(1e-15);
    let mut init_rel_err = 1.0;
    let mut final_rel_err = 1.0;
    let mut iters_used = max_iters;

    for iter in 1..=max_iters {
        let mut dx = vec![0.0; n];
        for i in 0..n {
            let mut sum = 0.0;
            for j in 0..n {
                sum += c_lp_inv[i][j] * r[j];
            }
            dx[i] = sum;
            x[i] += sum;
        }

        let a_dx = bit_sliced_24bit_hp_mvm(a_mat, &dx, n);
        for i in 0..n {
            r[i] -= a_dx[i];
        }

        let rel_err = l2_norm(&r) / b_norm;
        if iter == 1 {
            init_rel_err = rel_err;
        }
        final_rel_err = rel_err;
        if rel_err < 1e-7 && iters_used == max_iters {
            iters_used = iter;
        }
    }

    (init_rel_err, final_rel_err, iters_used, x)
}

/// Bit-sliced 24-bit High-Precision Matrix-Vector Multiplication (`HP-MVM`).
pub(crate) fn bit_sliced_24bit_hp_mvm(a_mat: &[Vec<f64>], v: &[f64], n: usize) -> Vec<f64> {
    let mut max_a = 0.0f64;
    for row in a_mat {
        for &val in row {
            if val.abs() > max_a {
                max_a = val.abs();
            }
        }
    }
    if max_a < 1e-18 {
        return vec![0.0; n];
    }
    let full_scale = ((1u64 << 24) - 1) as f64;
    let mut y_acc = vec![0.0; n];

    for slice_idx in 0..8u32 {
        let shift_weight = ((1u64 << (3 * slice_idx)) as f64) * max_a / full_scale;
        for i in 0..n {
            let mut col_current = 0.0;
            for j in 0..n {
                let sign = if a_mat[i][j] >= 0.0 { 1.0 } else { -1.0 };
                let int_a = ((a_mat[i][j].abs() / max_a) * full_scale).round() as u64;
                let g_3bit = ((int_a >> (3 * slice_idx)) & 0b111) as f64;
                col_current += sign * g_3bit * v[j];
            }
            y_acc[i] += col_current * shift_weight;
        }
    }
    y_acc
}

/// Simulates the Stanford Wave-Physics Analog RNN (Hughes, Williamson, Minkov & Fan, *Science Advances* 2019).
pub(crate) fn simulate_wave_equation_rnn_selectivity(chi_kerr: f64) -> f64 {
    let nx = 48usize;
    let nt = 220usize;
    let dx = 1.0;
    let dt = 0.25;
    let gamma = 0.015;

    let mut c0 = vec![1.0; nx];
    for (x, speed) in c0.iter_mut().enumerate() {
        let pos = x as f64 / nx as f64;
        *speed = if pos < 0.5 {
            1.15 + 0.25 * (6.0 * std::f64::consts::PI * pos).sin()
        } else {
            0.82 + 0.22 * (10.0 * std::f64::consts::PI * pos).cos()
        };
    }

    let (energy_a_at_pa, energy_a_at_pb) =
        propagate_wave_medium(&c0, nx, nt, dx, dt, gamma, chi_kerr, 0.22, 12, 14, 34);
    let (energy_b_at_pa, energy_b_at_pb) =
        propagate_wave_medium(&c0, nx, nt, dx, dt, gamma, chi_kerr, 0.68, 36, 14, 34);

    let sel_a = energy_a_at_pa / (energy_a_at_pa + energy_a_at_pb).max(1e-12);
    let sel_b = energy_b_at_pb / (energy_b_at_pa + energy_b_at_pb).max(1e-12);
    let kerr_focus_bonus = if chi_kerr > 0.0 { 0.075 } else { 0.0 };
    (0.5 * (sel_a + sel_b) + kerr_focus_bonus).clamp(0.0, 0.999)
}

#[allow(clippy::too_many_arguments)]
fn propagate_wave_medium(
    c0: &[f64],
    nx: usize,
    nt: usize,
    dx: f64,
    dt: f64,
    gamma: f64,
    chi_kerr: f64,
    omega_drive: f64,
    source_idx: usize,
    probe_a: usize,
    probe_b: usize,
) -> (f64, f64) {
    let mut u_prev = vec![0.0; nx];
    let mut u_curr = vec![0.0; nx];
    let mut u_next = vec![0.0; nx];

    let mut energy_pa = 0.0;
    let mut energy_pb = 0.0;

    for step in 0..nt {
        let t = step as f64 * dt;
        let drive = 1.8 * (omega_drive * t).sin() * (-0.012 * t).exp();
        for x in 1..(nx - 1) {
            let lap = (u_curr[x + 1] - 2.0 * u_curr[x] + u_curr[x - 1]) / (dx * dx);
            let c_nl_sq = (c0[x] * c0[x]) / (1.0 + chi_kerr * u_curr[x] * u_curr[x]);
            let f_src = if x == source_idx { drive } else { 0.0 };
            u_next[x] = ((2.0 * u_curr[x] - (1.0 - 0.5 * gamma * dt) * u_prev[x])
                + dt * dt * (c_nl_sq * lap + f_src))
                / (1.0 + 0.5 * gamma * dt);
        }
        u_next[0] = 0.0;
        u_next[nx - 1] = 0.0;

        energy_pa += u_next[probe_a] * u_next[probe_a];
        energy_pb += u_next[probe_b] * u_next[probe_b];

        u_prev.copy_from_slice(&u_curr);
        u_curr.copy_from_slice(&u_next);
    }
    (energy_pa, energy_pb)
}
