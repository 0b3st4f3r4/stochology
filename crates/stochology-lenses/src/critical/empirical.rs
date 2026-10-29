//! Real-Data Empirical Coupler for **Critical Stochastic Rationality** (`EmpiricalCriticalRationalityAudit`).

use crate::analog::EmpiricalWaveAnalogAudit;
use crate::invariance::{run_nonlinear_icp_nile, CausalEnvironment};
use crate::tail::{expected_shortfall_99, HillTailDiagnostic};
use stochology_core::{mean, sample_std, ChouldechovaAudit};

/// Real-Data Empirical Coupler for **Critical Stochastic Rationality** across `danish_fire.csv` ($n=2\text{,}167$),
/// `toronto_arrests.csv` ($n=5\text{,}226$), `atlantic_storms.csv` ($n=20\text{,}778$), and `vic_elec.csv` ($n=52\text{,}608$).
#[derive(Debug, Clone, PartialEq)]
pub struct EmpiricalCriticalRationalityAudit {
    /// Hill tail index $\hat{\alpha}_{k,n} \in (1, 2]$ on `danish_fire.csv` (infinite theoretical variance regime).
    pub danish_fire_hill_alpha: f64,
    /// True Expected Shortfall $\mathrm{ES}_{0.99}$ on `danish_fire.csv` vs. Gaussian $2\sigma$-truncated $\mathrm{ES}_{0.99}^{\text{trunc}}$.
    pub danish_fire_true_vs_truncated_es99: (f64, f64),
    /// Fraction of real `danish_fire.csv` tail risk liquidated by forced Gaussian identity ($> 65\%$).
    pub danish_fire_non_identical_erasure_fraction: f64,
    /// Chouldechova FPR disparity ratio ($\mathrm{FPR}_A / \mathrm{FPR}_B$) on `toronto_arrests.csv` base rates.
    pub toronto_arrests_preemptive_fpr_ratio: f64,
    /// Ratio of spurious latitude proxy penalty to invariant barometric core penalty on `atlantic_storms.csv` ($> 80\times$).
    pub atlantic_storms_spurious_to_invariant_penalty_ratio: f64,
    /// Wave-interference compression ratio and PKU 24-bit `HP-INV` spectral error on `vic_elec.csv`.
    pub vic_elec_wave_compression_and_hp_inv_error: (f64, f64),
    /// Whether all four empirical real-data audits are verified.
    pub empirical_critical_verified: bool,
}

impl EmpiricalCriticalRationalityAudit {
    /// Audits the real datasets (`danish_fire.csv`, `toronto_arrests.csv`, `atlantic_storms.csv`, and `vic_elec.csv`).
    pub fn evaluate_real_datasets(
        danish_fire_losses: &[f64],
        toronto_base_rates: (f64, f64),
        atlantic_storm_envs: &[CausalEnvironment],
        vic_elec_demand: &[f64],
    ) -> Self {
        // 1. Adorno's Liquidation of the Non-Identical on `danish_fire.csv`
        let hill = HillTailDiagnostic::audit(danish_fire_losses, Some(100));
        let es99_true = expected_shortfall_99(danish_fire_losses);
        let mu_fire = mean(danish_fire_losses);
        let sd_fire = sample_std(danish_fire_losses);
        let fire_cutoff = mu_fire + 2.0 * sd_fire;
        let fire_trunc: Vec<f64> = danish_fire_losses
            .iter()
            .copied()
            .filter(|&x| x <= fire_cutoff)
            .collect();
        let es99_trunc = expected_shortfall_99(&fire_trunc);
        let danish_fire_non_identical_erasure_fraction = 1.0 - (es99_trunc / es99_true.max(1e-12));

        // 2. Rouvroy's Algorithmic Preemption & Reification on `toronto_arrests.csv`
        let (p_a, p_b) = toronto_base_rates;
        let (_, _, toronto_arrests_preemptive_fpr_ratio) =
            ChouldechovaAudit::fpr_ratio_under_equal_ppv_and_fnr(p_a, p_b, 0.65, 0.25);

        // 3. Marcuse's Operational Closure vs. Causal Invariance on `atlantic_storms.csv`
        let nile = run_nonlinear_icp_nile(atlantic_storm_envs, 0.15);
        let p_core = nile.nonlinear_nile_irm_penalties[0].max(1e-9);
        let p_spurious = nile.nonlinear_nile_irm_penalties[1];
        let atlantic_storms_spurious_to_invariant_penalty_ratio = p_spurious / p_core;

        // 4. Material Collapse & Wave/Analog Overcoming on `vic_elec.csv`
        let emp_wave = EmpiricalWaveAnalogAudit::evaluate_series(vic_elec_demand);
        let vic_elec_wave_compression_and_hp_inv_error = (
            emp_wave.empirical_compression_ratio,
            emp_wave.empirical_hp_inv_relative_error,
        );

        let empirical_critical_verified = hill.alpha_hill > 1.0
            && hill.alpha_hill <= 2.0
            && danish_fire_non_identical_erasure_fraction > 0.65
            && toronto_arrests_preemptive_fpr_ratio > 5.0
            && atlantic_storms_spurious_to_invariant_penalty_ratio > 80.0
            && emp_wave.empirical_wave_verified;

        Self {
            danish_fire_hill_alpha: hill.alpha_hill,
            danish_fire_true_vs_truncated_es99: (es99_true, es99_trunc),
            danish_fire_non_identical_erasure_fraction,
            toronto_arrests_preemptive_fpr_ratio,
            atlantic_storms_spurious_to_invariant_penalty_ratio,
            vic_elec_wave_compression_and_hp_inv_error,
            empirical_critical_verified,
        }
    }
}
