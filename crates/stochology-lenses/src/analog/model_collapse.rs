//! Axis 1 of Physical, Analog, and Wave-Interference Computing:
//! Mark Horowitz's (ISSCC 2014) von Neumann Energy Hierarchy, IEA Macro-Trajectory,
//! and Recursive Synthetic Data Contamination (*Model Collapse*, `VonNeumannModelCollapseAudit`, `GerstgrasserAccumulateAudit`).

use crate::information::kl_divergence;
use stochology_core::{Rng, cholesky_decompose, cholesky_solve};

/// Axis 1: Mark Horowitz's (ISSCC 2014) von Neumann Energy Hierarchy, IEA Macro-Trajectory,
/// and Recursive Synthetic Data Contamination (*Model Collapse*) vs. Neurosymbolic/Physical Anchoring.
#[derive(Debug, Clone, PartialEq)]
pub struct VonNeumannModelCollapseAudit {
    /// Energy of a 32-bit integer addition in 45nm CMOS ($0.1\text{ pJ}$).
    pub cmos45_int32_add_pj: f64,
    /// Energy of an FP32 floating-point addition in 45nm CMOS ($0.9\text{ pJ}$).
    pub cmos45_fp32_add_pj: f64,
    /// Energy of an FP32 floating-point multiplication in 45nm CMOS ($3.7\text{ pJ}$).
    pub cmos45_fp32_mul_pj: f64,
    /// Energy of an on-chip 8KB SRAM 32-bit read in 45nm CMOS ($5.0\text{ pJ}$).
    pub cmos45_sram_read_pj: f64,
    /// Energy of an off-chip 32-bit DRAM read in 45nm CMOS ($640.0\text{ pJ}$).
    pub cmos45_dram_read_pj: f64,
    /// Ratio of off-chip DRAM read energy to FP32 addition energy ($640.0 / 0.9 \approx 711.11\times$).
    pub dram_to_fp32_add_ratio: f64,
    /// Ratio of off-chip DRAM read energy to 32-bit integer addition energy ($640.0 / 0.1 = 6400\times$).
    pub dram_to_int32_add_ratio: f64,
    /// System power ratio between Nvidia H100 GPU ($700\text{ W}$) and PKU 40nm RRAM chip ($7\text{ W}$) ($100\times$).
    pub h100_to_pku_rram_power_ratio: f64,
    /// IEA global data-center electricity demand trajectory `[2020, 2025, 2030, 2035]` in TWh.
    pub iea_datacenter_twh_trajectory: [f64; 4],
    /// Ratio of generation-$G$ variance to generation-$0$ variance under pure unanchored synthetic recursion ($\sigma_G^2 / \sigma_0^2 \ll 1$).
    pub unanchored_synthetic_variance_retention: f64,
    /// KL divergence $D_{\mathrm{KL}}(P_0 \parallel P_G)$ (in nats) after $G$ generations of pure synthetic recursion ("AI contaminating AI").
    pub unanchored_synthetic_kl_divergence_nats: f64,
    /// Variance retention $\sigma_G^2 / \sigma_0^2$ when anchored by neurosymbolic constraints and physical observations.
    pub neurosymbolic_anchored_variance_retention: f64,
    /// KL divergence $D_{\mathrm{KL}}(P_0 \parallel P_G)$ (in nats) under neurosymbolic and physical anchoring.
    pub neurosymbolic_anchored_kl_divergence_nats: f64,
    /// Whether the von Neumann bottleneck asymmetry ($\ge 700\times$) and synthetic collapse prevention are both verified.
    pub axis1_verified: bool,
}

impl VonNeumannModelCollapseAudit {
    /// Evaluates the von Neumann energy wall and simulates $G = 8$ generations of recursive synthetic training (`seed = 2026`).
    pub fn evaluate(seed: u64, anchored_mode: bool) -> Self {
        let cmos45_int32_add_pj = 0.1;
        let cmos45_fp32_add_pj = 0.9;
        let cmos45_fp32_mul_pj = 3.7;
        let cmos45_sram_read_pj = 5.0;
        let cmos45_dram_read_pj = 640.0;
        let dram_to_fp32_add_ratio = cmos45_dram_read_pj / cmos45_fp32_add_pj;
        let dram_to_int32_add_ratio = cmos45_dram_read_pj / cmos45_int32_add_pj;
        let h100_to_pku_rram_power_ratio = 700.0 / 7.0;
        let iea_datacenter_twh_trajectory = [269.1, 485.1, 945.0, 1200.0];

        let p0 = [0.12, 0.23, 0.30, 0.23, 0.12];
        let vals = [-2.0, -1.0, 0.0, 1.0, 2.0];
        let var0 = discrete_variance(&p0, &vals);

        let mut rng = Rng::new(seed);
        let n_samples_per_gen = 120usize;
        let generations = 8usize;

        let mut p_unanchored = p0;
        for _ in 0..generations {
            let mut counts = [0usize; 5];
            let mut weights = [0.0; 5];
            let mut w_sum = 0.0;
            for i in 0..5 {
                weights[i] = p_unanchored[i].powf(1.35);
                w_sum += weights[i];
            }
            for w in &mut weights {
                *w /= w_sum;
            }
            for _ in 0..n_samples_per_gen {
                let idx = sample_categorical(&mut rng, &weights);
                counts[idx] += 1;
            }
            for i in 0..5 {
                p_unanchored[i] = (counts[i] as f64 + 1e-4) / (n_samples_per_gen as f64 + 5e-4);
            }
        }
        let var_unanchored = discrete_variance(&p_unanchored, &vals);
        let unanchored_synthetic_variance_retention = var_unanchored / var0;
        let unanchored_synthetic_kl_divergence_nats = kl_divergence(&p0, &p_unanchored);

        let mut p_anchored = p0;
        for _ in 0..generations {
            let mut counts = [0usize; 5];
            for _ in 0..n_samples_per_gen {
                let source = if anchored_mode { &p0 } else { &p_unanchored };
                let idx = sample_categorical(&mut rng, source);
                counts[idx] += 1;
            }
            let mut raw = [0.0; 5];
            for i in 0..5 {
                raw[i] = (counts[i] as f64 + 0.5) / (n_samples_per_gen as f64 + 2.5);
            }
            if anchored_mode {
                for i in 0..5 {
                    let sym = 0.5 * (raw[i] + raw[4 - i]);
                    p_anchored[i] = 0.25 * sym + 0.75 * p0[i];
                }
            } else {
                p_anchored = p_unanchored;
            }
        }
        let var_anchored = discrete_variance(&p_anchored, &vals);
        let neurosymbolic_anchored_variance_retention = var_anchored / var0;
        let neurosymbolic_anchored_kl_divergence_nats = kl_divergence(&p0, &p_anchored);

        let axis1_verified = dram_to_fp32_add_ratio > 700.0
            && dram_to_int32_add_ratio >= 6400.0
            && h100_to_pku_rram_power_ratio >= 100.0
            && unanchored_synthetic_variance_retention < 0.25
            && unanchored_synthetic_kl_divergence_nats > 0.60
            && neurosymbolic_anchored_variance_retention > 0.90
            && neurosymbolic_anchored_kl_divergence_nats < 0.02;

        Self {
            cmos45_int32_add_pj,
            cmos45_fp32_add_pj,
            cmos45_fp32_mul_pj,
            cmos45_sram_read_pj,
            cmos45_dram_read_pj,
            dram_to_fp32_add_ratio,
            dram_to_int32_add_ratio,
            h100_to_pku_rram_power_ratio,
            iea_datacenter_twh_trajectory,
            unanchored_synthetic_variance_retention,
            unanchored_synthetic_kl_divergence_nats,
            neurosymbolic_anchored_variance_retention,
            neurosymbolic_anchored_kl_divergence_nats,
            axis1_verified,
        }
    }
}

/// Synthetic-data training policy across recursive model generations (Gerstgrasser et al. 2024).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyntheticDataPolicy {
    /// Generation $n$ trains only on outputs of generation $n-1$; the real dataset $D_0$ is discarded.
    Replace,
    /// Generation $n$ trains on the real dataset $D_0$ plus all synthetic generations $D_1, \dots, D_{n-1}$.
    Accumulate,
}

/// Exact **Accumulate-Not-Replace** vs. **Replacement** Model Collapse Audit
/// (Shumailov et al. 2024; Gerstgrasser et al. 2024, arXiv:2404.01413, Thm. 2).
#[derive(Debug, Clone, PartialEq)]
pub struct GerstgrasserAccumulateAudit {
    pub sigma: f64,
    pub dimension_d: usize,
    pub sample_size_t: usize,
    pub generations_n: usize,
    pub monte_carlo_trials: usize,
    pub declared_policy: SyntheticDataPolicy,
    /// Single-generation inverse-Wishart prefactor $\frac{\sigma^2 d}{T - d - 1}$.
    pub base_prefactor: f64,
    /// Closed-form excess test risk under Replacement at generation $n$.
    pub analytical_replace_risk_n: f64,
    /// Closed-form excess test risk under Accumulation at generation $n$.
    pub analytical_accumulate_risk_n: f64,
    /// Basel ceiling $\frac{\sigma^2 d}{T - d - 1} \cdot \frac{\pi^2}{6}$ bounding Accumulation for all $n \ge 1$.
    pub asymptotic_basel_ceiling: f64,
    /// Monte Carlo excess risk $\mathbb{E}\|\hat{w}_n - w^*\|^2$ under recursive Replacement refitting.
    pub simulated_replace_risk_n: f64,
    /// Monte Carlo excess risk $\mathbb{E}\|\hat{w}_n - w^*\|^2$ under recursive Accumulation refitting.
    pub simulated_accumulate_risk_n: f64,
    /// Largest relative deviation of the two simulated risks from their closed forms.
    pub simulation_closure_error: f64,
    /// Simulated ratio $E^{\mathrm{Replace}}_n / E^{\mathrm{Accum}}_n$.
    pub replace_to_accumulate_ratio: f64,
    /// Simulated excess risk of the declared policy.
    pub declared_policy_risk_n: f64,
    /// True iff the simulation closes on both closed forms (within 10%) and the declared policy's simulated
    /// risk stays under the Basel ceiling (with a 10% Monte Carlo tolerance).
    pub declared_policy_bounded: bool,
}

impl GerstgrasserAccumulateAudit {
    /// Simulates both pipelines by recursive OLS refitting and audits the declared policy.
    pub fn evaluate(
        seed: u64,
        sigma: f64,
        dimension_d: usize,
        sample_size_t: usize,
        generations_n: usize,
        declared_policy: SyntheticDataPolicy,
    ) -> Self {
        assert!(
            sample_size_t > dimension_d + 1 && dimension_d >= 1 && generations_n >= 1,
            "requires T > d + 1, d >= 1, n >= 1"
        );
        let d = dimension_d;
        let d_f = d as f64;
        let t_f = sample_size_t as f64;
        let n_f = generations_n as f64;
        let prefactor = (sigma * sigma * d_f) / (t_f - d_f - 1.0);
        let analytical_replace = prefactor * n_f;
        let harmonic_sq: f64 = (1..=generations_n).map(|i| 1.0 / ((i * i) as f64)).sum();
        let analytical_accum = prefactor * harmonic_sq;
        let basel_ceiling = prefactor * (std::f64::consts::PI * std::f64::consts::PI / 6.0);

        let mut rng = Rng::new(seed);
        let trials = 2000usize;
        let mut sum_rep = 0.0;
        let mut sum_acc = 0.0;
        for _ in 0..trials {
            let x: Vec<Vec<f64>> = (0..sample_size_t)
                .map(|_| (0..d).map(|_| rng.standard_normal()).collect())
                .collect();
            let gram: Vec<Vec<f64>> = (0..d)
                .map(|a| {
                    (0..d)
                        .map(|b| x.iter().map(|row| row[a] * row[b]).sum())
                        .collect()
                })
                .collect();
            let chol = cholesky_decompose(&gram);
            let relabel_xty = |w: &[f64], rng: &mut Rng| -> Vec<f64> {
                let mut xty = vec![0.0; d];
                for row in &x {
                    let y = row.iter().zip(w).map(|(a, b)| a * b).sum::<f64>()
                        + sigma * rng.standard_normal();
                    for (acc, &r) in xty.iter_mut().zip(row) {
                        *acc += r * y;
                    }
                }
                xty
            };
            let w_star = vec![0.0; d];
            let xty_real = relabel_xty(&w_star, &mut rng);
            let mut w_rep = cholesky_solve(&chol, &xty_real);
            let mut acc_xty = xty_real;
            let mut w_acc = w_rep.clone();
            for gen_idx in 2..=generations_n {
                let xty_rep = relabel_xty(&w_rep, &mut rng);
                w_rep = cholesky_solve(&chol, &xty_rep);
                let xty_new = relabel_xty(&w_acc, &mut rng);
                for (a, b) in acc_xty.iter_mut().zip(&xty_new) {
                    *a += b;
                }
                w_acc = cholesky_solve(&chol, &acc_xty)
                    .into_iter()
                    .map(|v| v / gen_idx as f64)
                    .collect();
            }
            sum_rep += w_rep.iter().map(|v| v * v).sum::<f64>();
            sum_acc += w_acc.iter().map(|v| v * v).sum::<f64>();
        }
        let sim_rep = sum_rep / (trials as f64);
        let sim_acc = sum_acc / (trials as f64);
        let closure = ((sim_rep - analytical_replace) / analytical_replace)
            .abs()
            .max(((sim_acc - analytical_accum) / analytical_accum).abs());
        let declared_risk = match declared_policy {
            SyntheticDataPolicy::Replace => sim_rep,
            SyntheticDataPolicy::Accumulate => sim_acc,
        };

        Self {
            sigma,
            dimension_d,
            sample_size_t,
            generations_n,
            monte_carlo_trials: trials,
            declared_policy,
            base_prefactor: prefactor,
            analytical_replace_risk_n: analytical_replace,
            analytical_accumulate_risk_n: analytical_accum,
            asymptotic_basel_ceiling: basel_ceiling,
            simulated_replace_risk_n: sim_rep,
            simulated_accumulate_risk_n: sim_acc,
            simulation_closure_error: closure,
            replace_to_accumulate_ratio: sim_rep / sim_acc.max(1e-15),
            declared_policy_risk_n: declared_risk,
            declared_policy_bounded: closure <= 0.10 && declared_risk <= 1.10 * basel_ceiling,
        }
    }

    /// Canonical benchmark ($\sigma = 1, d = 5, T = 50, n = 10$):
    /// $E^{\mathrm{Replace}}_{10} = 1.1363636$ vs. $E^{\mathrm{Accum}}_{10} = 0.1761100 \le 0.1869243$.
    pub fn canonical_benchmark(seed: u64, declared_policy: SyntheticDataPolicy) -> Self {
        Self::evaluate(seed, 1.0, 5, 50, 10, declared_policy)
    }
}

fn discrete_variance(probs: &[f64], vals: &[f64]) -> f64 {
    let mut mu = 0.0;
    for (p, x) in probs.iter().zip(vals.iter()) {
        mu += p * x;
    }
    let mut var = 0.0;
    for (p, x) in probs.iter().zip(vals.iter()) {
        var += p * (x - mu).powi(2);
    }
    var
}

fn sample_categorical(rng: &mut Rng, probs: &[f64]) -> usize {
    let u = rng.next_f64();
    let mut acc = 0.0;
    for (i, &p) in probs.iter().enumerate() {
        acc += p;
        if u <= acc || i + 1 == probs.len() {
            return i;
        }
    }
    probs.len() - 1
}
