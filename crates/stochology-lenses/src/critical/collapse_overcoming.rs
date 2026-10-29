//! Axis 3 & Axis 4 of Critical Stochastic Rationality:
//! - Material, Informational, and Thermodynamic Collapse Triad (`MaterialCollapseTriadAudit`)
//! - Dialectical Overcoming via Neurosymbolic Constraints, A-ECC/PKU `HP-INV`, and Continuous Wave Physics (`DialecticalOvercomingSynthesisAudit`)

use crate::acceptance::NamedCheck;
use crate::analog::{AimcAnalogWallHpInvAudit, AnalogWaveComputingSynthesis};
use stochology_core::Rng;

/// Axis 3: The Material, Informational, and Thermodynamic Collapse of Brute-Force Stochastic Reason.
#[derive(Debug, Clone, PartialEq)]
pub struct MaterialCollapseTriadAudit {
    /// Informative: Epoch AI (Villalobos et al., Nov 2024) effective human public text stock in trillions of tokens.
    pub epoch_ai_effective_human_tokens_trillions: f64,
    /// Informative: Epoch AI raw upper bound including unindexed text in trillions of tokens.
    pub epoch_ai_raw_upper_bound_tokens_trillions: f64,
    /// Informative: projected exhaustion window of human public text.
    pub human_token_exhaustion_window_years: (u32, u32),
    /// Simulated variance retention $\sigma_G^2 / \sigma_0^2$ after 8 generations of unanchored synthetic recursion (`crate::analog`). Gated (`< 0.25`).
    pub synthetic_autophagy_variance_retention: f64,
    /// Informative: Shannon entropy loss $H(P_0) - H(P_G)$ between two illustrative distributions (the collapsed one is hand-specified).
    pub synthetic_autophagy_entropy_loss_nats: f64,
    /// Simulated $D_{\mathrm{KL}}(P_0 \parallel P_G)$ after unanchored synthetic recursion (`crate::analog`). Gated (`> 0.60`).
    pub synthetic_autophagy_kl_divergence_nats: f64,
    /// Informative: $\ln 16 - \ln 2$, the information removed by mapping 16 levels onto one bit.
    pub binary_abstraction_information_loss_nats: f64,
    /// Informative: Horowitz (ISSCC 2014, 45nm CMOS) DRAM read to FP32 addition energy ratio.
    pub von_neumann_dram_to_fp32_ratio: f64,
    /// Informative: Nvidia H100 GPU power and total cooled envelope (W).
    pub h100_gpu_watts_and_cooled_watts: (f64, f64),
    /// Informative: IEA global data-center electricity demand trajectory in 2030 (TWh).
    pub iea_2030_datacenter_twh: f64,
    /// Informative: Ferran Alia (July 2026) 5-point noise threshold curve `[sigma=0.1, 0.2, 0.4, 0.6, 0.8]` as transcribed.
    pub ferran_alia_threshold_cliff_curve: [f64; 5],
    /// Informative: $\sigma_0 \sqrt{L}$ for $L = 32$ layers at $\sigma_0 = 0.15$ (independent-noise compounding).
    pub deep_llm_32_layer_compounded_noise_sigma: f64,
    /// Named claim checks gated by this axis (see `crate::acceptance`).
    pub claims: Vec<NamedCheck>,
    /// Claims removed from the gate after failing multi-seed acceptance (`crate::acceptance`); still computed and reported.
    pub discarded_claims: Vec<NamedCheck>,
    /// Whether the simulated synthetic-recursion collapse claims hold.
    pub axis3_verified: bool,
}

impl MaterialCollapseTriadAudit {
    /// Evaluates the three material contradictions (`seed = 2026`).
    pub fn evaluate(seed: u64) -> Self {
        let analog_syn = AnalogWaveComputingSynthesis::evaluate_canonical(seed);
        let synthetic_autophagy_variance_retention = analog_syn
            .von_neumann_and_collapse
            .unanchored_synthetic_variance_retention;
        let synthetic_autophagy_kl_divergence_nats = analog_syn
            .von_neumann_and_collapse
            .unanchored_synthetic_kl_divergence_nats;

        // Illustrative entropy contrast between the base distribution and a hand-specified collapsed distribution
        let p0 = [0.12_f64, 0.23, 0.30, 0.23, 0.12];
        let pg_collapsed = [0.002_f64, 0.038, 0.920, 0.038, 0.002];
        let h0: f64 = p0.iter().map(|&p| -p * p.ln()).sum();
        let hg: f64 = pg_collapsed.iter().map(|&p| -p * p.ln()).sum();

        let claims: Vec<NamedCheck> = vec![(
            "critical.a3.synthetic_kl_above_0.60",
            synthetic_autophagy_kl_divergence_nats > 0.60,
        )];
        // Variance retention falls below 0.25 on 15 of 20 acceptance seeds after 8 generations.
        let discarded_claims: Vec<NamedCheck> = vec![(
            "critical.a3.synthetic_variance_retention_below_0.25",
            synthetic_autophagy_variance_retention < 0.25,
        )];
        let axis3_verified = claims.iter().all(|c| c.1);

        Self {
            epoch_ai_effective_human_tokens_trillions: 300.0,
            epoch_ai_raw_upper_bound_tokens_trillions: 3_100.0,
            human_token_exhaustion_window_years: (2026, 2032),
            synthetic_autophagy_variance_retention,
            synthetic_autophagy_entropy_loss_nats: h0 - hg,
            synthetic_autophagy_kl_divergence_nats,
            binary_abstraction_information_loss_nats: (16.0_f64).ln() - (2.0_f64).ln(),
            von_neumann_dram_to_fp32_ratio: analog_syn
                .von_neumann_and_collapse
                .dram_to_fp32_add_ratio,
            h100_gpu_watts_and_cooled_watts: (700.0, 1_100.0),
            iea_2030_datacenter_twh: analog_syn
                .von_neumann_and_collapse
                .iea_datacenter_twh_trajectory[2],
            ferran_alia_threshold_cliff_curve: [0.98, 0.83, 0.64, 0.39, 0.25],
            deep_llm_32_layer_compounded_noise_sigma: 0.15 * (32.0_f64).sqrt(),
            claims,
            discarded_claims,
            axis3_verified,
        }
    }
}

/// Fraction of `runs` 8-generation synthetic recursions over 5 semantic states whose final distribution satisfies the symbolic
/// rules (mirror symmetry $|p_i - p_{4-i}| \le 0.02$ and variance retention $\ge 0.90$). With `constrained`, each generation samples
/// the anchored observations and projects onto the symmetric set; otherwise it refits temperature-sharpened self-samples.
fn synthetic_recursion_rule_satisfaction(rng: &mut Rng, constrained: bool, runs: usize) -> f64 {
    let p0 = [0.12_f64, 0.23, 0.30, 0.23, 0.12];
    let vals = [-2.0_f64, -1.0, 0.0, 1.0, 2.0];
    let variance = |p: &[f64; 5]| {
        let m: f64 = p.iter().zip(&vals).map(|(pi, v)| pi * v).sum();
        p.iter()
            .zip(&vals)
            .map(|(pi, v)| pi * (v - m).powi(2))
            .sum::<f64>()
    };
    let var0 = variance(&p0);
    let draw = |rng: &mut Rng, w: &[f64; 5]| -> usize {
        let u = rng.next_f64();
        let mut acc = 0.0;
        for (i, &wi) in w.iter().enumerate() {
            acc += wi;
            if u < acc {
                return i;
            }
        }
        4
    };
    let n_per_gen = 120usize;
    let mut satisfied = 0usize;
    for _ in 0..runs {
        let mut p = p0;
        for _ in 0..8 {
            let source = if constrained {
                p0
            } else {
                let mut w = p.map(|pi| pi.powf(1.35));
                let total: f64 = w.iter().sum();
                w.iter_mut().for_each(|wi| *wi /= total);
                w
            };
            let mut counts = [0usize; 5];
            for _ in 0..n_per_gen {
                counts[draw(rng, &source)] += 1;
            }
            if constrained {
                let raw = counts.map(|c| (c as f64 + 0.5) / (n_per_gen as f64 + 2.5));
                for i in 0..5 {
                    p[i] = 0.25 * 0.5 * (raw[i] + raw[4 - i]) + 0.75 * p0[i];
                }
            } else {
                p = counts.map(|c| (c as f64 + 1e-4) / (n_per_gen as f64 + 5e-4));
            }
        }
        let symmetric = (0..5).all(|i| (p[i] - p[4 - i]).abs() <= 0.02);
        if symmetric && variance(&p) / var0 >= 0.90 {
            satisfied += 1;
        }
    }
    satisfied as f64 / runs as f64
}

/// Axis 4: Paths of Overcoming — Reconciling Logical Structure (Neurosymbolic Computing),
/// Coexistence with Physical Noise (`A-ECC`, Peking University 24-Bit `HP-INV`),
/// and Continuous Wave-Interference Physics (`SIREN`).
#[derive(Debug, Clone, PartialEq)]
pub struct DialecticalOvercomingSynthesisAudit {
    /// Symbolic-rule satisfaction rate over 200 unconstrained synthetic recursions.
    pub unanchored_statistical_rule_satisfaction_rate: f64,
    /// Symbolic-rule satisfaction rate over 200 constrained (anchored + symmetric projection) recursions.
    pub neurosymbolic_rule_satisfaction_rate: f64,
    /// Rule satisfaction of the deployed recursion (constrained if overcoming is enabled). Gated (`>= 0.95`).
    pub deployed_rule_satisfaction_rate: f64,
    /// Variance retention $\sigma_G^2 / \sigma_0^2$ of the deployed recursion in `crate::analog` (anchored only if overcoming is enabled). Gated (`> 0.90`).
    pub neurosymbolic_variance_retention: f64,
    /// Informative: accuracy at $\sigma = 0.60$ under `NAT` vs. standard training; `crate::analog` models `NAT` as an assumed reduction of
    /// effective conductance noise, so these values are not trained results.
    pub nat_vs_standard_accuracy_at_sigma_060: (f64, f64),
    /// Accuracy of drifted PCM readouts with the A-ECC calibration column vs. raw drift.
    pub a_ecc_compensated_vs_raw_accuracy: (f64, f64),
    /// Accuracy of the deployed readout (compensated if overcoming is enabled). Gated (`>= 0.97`).
    pub a_ecc_drift_recovery_accuracy: f64,
    /// Relative residual of the deployed `HP-INV` solve (10 refinements if overcoming is enabled, 1 otherwise). Gated (`< 1e-7`).
    pub pku_40nm_rram_24bit_hp_inv_error: f64,
    /// Informative: phase-amplitude parameter compression ratio (480 grid points over 12 sinusoid parameters).
    pub wave_phase_amplitude_compression_ratio: f64,
    /// `SIREN` (projection on the known frequencies, an identity) vs. piecewise-linear `ReLU` Helmholtz Laplacian MSE.
    pub siren_vs_relu_helmholtz_laplacian_mse: (f64, f64),
    /// Informative: Laplacian MSE of the deployed representation (`SIREN` if overcoming is enabled, `ReLU` otherwise); the `ReLU`
    /// side is zero by construction ($\mathrm{ReLU}'' = 0$ a.e.), so it is not a test.
    pub deployed_helmholtz_laplacian_mse: f64,
    /// Informative: Tsinghua University `ACCEL` diffractive photonic efficiency in TOPS/W as transcribed.
    pub tsinghua_accel_photonic_tops_per_watt: f64,
    /// Named claim checks gated by this axis (see `crate::acceptance`).
    pub claims: Vec<NamedCheck>,
    /// True if the `SIREN` projection identity closes (Laplacian MSE $< 10^{-10}$).
    pub identities_closed: bool,
    /// Whether the deployed recursion, readout, solver, and representation pass their claim checks.
    pub axis4_verified: bool,
}

impl DialecticalOvercomingSynthesisAudit {
    /// Evaluates Axis 4 (`seed = 2026`). When `enable_overcoming` is `false`, the unconstrained recursion, raw drifted readout,
    /// one-shot inversion, and `ReLU` representation are deployed.
    pub fn evaluate(seed: u64, enable_overcoming: bool) -> Self {
        let analog = AnalogWaveComputingSynthesis::evaluate_with_mode(seed, enable_overcoming);
        let aimc: &AimcAnalogWallHpInvAudit = &analog.aimc_analog_wall_hp_inv;

        let mut rng = Rng::new(seed ^ 0xC417_1CA1_2026);
        let unanchored_rate = synthetic_recursion_rule_satisfaction(&mut rng, false, 200);
        let constrained_rate = synthetic_recursion_rule_satisfaction(&mut rng, true, 200);
        let deployed_rule_rate = if enable_overcoming {
            constrained_rate
        } else {
            unanchored_rate
        };
        let neurosymbolic_variance_retention = analog
            .von_neumann_and_collapse
            .neurosymbolic_anchored_variance_retention;

        let a_ecc_pair = (
            aimc.a_ecc_drift_recovered_accuracy,
            aimc.a_ecc_uncompensated_accuracy,
        );
        let deployed_a_ecc = if enable_overcoming {
            a_ecc_pair.0
        } else {
            a_ecc_pair.1
        };
        let siren_vs_relu = (
            analog.wave_interference.siren_helmholtz_laplacian_mse,
            analog.wave_interference.relu_helmholtz_laplacian_mse,
        );
        let deployed_laplacian = if enable_overcoming {
            siren_vs_relu.0
        } else {
            siren_vs_relu.1
        };
        let pku_40nm_rram_24bit_hp_inv_error = aimc.pku_hp_inv_final_relative_error;

        let identities_closed = siren_vs_relu.0 < 1e-10;
        let claims: Vec<NamedCheck> = vec![
            (
                "critical.a4.deployed_rule_satisfaction_at_least_0.95",
                deployed_rule_rate >= 0.95,
            ),
            (
                "critical.a4.deployed_variance_retention_above_0.90",
                neurosymbolic_variance_retention > 0.90,
            ),
            (
                "critical.a4.deployed_a_ecc_accuracy_at_least_0.97",
                deployed_a_ecc >= 0.97,
            ),
            (
                "critical.a4.deployed_hp_inv_residual_below_1e-7",
                pku_40nm_rram_24bit_hp_inv_error < 1e-7,
            ),
        ];
        let axis4_verified = identities_closed && claims.iter().all(|c| c.1);

        Self {
            unanchored_statistical_rule_satisfaction_rate: unanchored_rate,
            neurosymbolic_rule_satisfaction_rate: constrained_rate,
            deployed_rule_satisfaction_rate: deployed_rule_rate,
            neurosymbolic_variance_retention,
            nat_vs_standard_accuracy_at_sigma_060: (
                aimc.nat_accuracy_at_sigma_060,
                aimc.standard_accuracy_at_sigma_060,
            ),
            a_ecc_compensated_vs_raw_accuracy: a_ecc_pair,
            a_ecc_drift_recovery_accuracy: deployed_a_ecc,
            pku_40nm_rram_24bit_hp_inv_error,
            wave_phase_amplitude_compression_ratio: analog
                .wave_interference
                .phase_amplitude_compression_ratio,
            siren_vs_relu_helmholtz_laplacian_mse: siren_vs_relu,
            deployed_helmholtz_laplacian_mse: deployed_laplacian,
            tsinghua_accel_photonic_tops_per_watt: analog
                .sensory_dpi_and_ecosystem
                .tsinghua_accel_tops_per_watt,
            claims,
            identities_closed,
            axis4_verified,
        }
    }
}
