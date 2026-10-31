//! **Chapter 7 (Filter III · Tension 6) — Irreversible Time, Non-Ergodicity, and the Vital Margin.**
//!
//! Mediates the tension between static ensemble optimization and survival in irreversible entropic time
//! (Canguilhem, 1943/1966; Wald, 1943, 1945; Itô, 1951; Kelly, 1956; Scarf, 1960; Breiman, 1961;
//! Kingman, 1961; Merton, 1969; Georgescu-Roegen, 1971; Samuelson, 1971, 1979; Holling, 1973;
//! McEwen, 1998; Ben-Haim, 2006; Sniedovich, 2007, 2012; Scheffer et al., 2009;
//! Baker & McHale, 2013; Simchi-Levi et al., 2014; Peters, 2019):
//! - Non-ergodic Itô variance drain and Baker–McHale epistemic Kelly allocation (`EpistemicKellyAllocation`);
//! - Canguilhem–Holling resilience basin, Scheffer's Critical Slowing Down, and McEwen's allostatic load (`CanguilhemHollingBasin`);
//! - Kingman heavy-traffic queue amplification, Scarf $(s, S)$ buffers, and Simchi-Levi $\mathrm{TTS} \ge \mathrm{TTR}$ (`VitalSlackBuffer`);
//! - Abraham Wald's 1943 bomber survivorship inversion (`WaldSurvivorshipAudit`);
//! - Info-Gap robustness horizon guarded by Wald–Sniedovich global Maximin (`InfoGapMaximinSafeguard`).

use crate::rng::{mean, sample_variance};

/// Non-ergodic multiplicative growth and Baker & McHale (2013) epistemic Kelly allocation
/// under estimation uncertainty (Itô, 1951; Kelly, 1956; Samuelson, 1971, 1979; Peters, 2019).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EpistemicKellyAllocation {
    /// Ensemble arithmetic drift $\hat{\mu}$.
    pub ensemble_drift_mu: f64,
    /// Return variance $\sigma^2$.
    pub return_variance_sigma2: f64,
    /// Variance of the drift estimator $\sigma_{\hat{\mu}}^2 = \sigma^2 / n_{\mathrm{eff}}$.
    pub estimation_variance_mu: f64,
    /// Unleveraged ($f = 1$) Itô time-average growth rate $g_{\mathrm{time}}(1) = \hat{\mu} - \frac{1}{2}\sigma^2$.
    pub unleveraged_time_growth: f64,
    /// Naive plug-in Kelly fraction $f^*_{\mathrm{plug\text{-}in}} = \hat{\mu} / \sigma^2$.
    pub plugin_kelly_fraction: f64,
    /// Baker–McHale epistemic shrinkage factor $s_{\mathrm{epistemic}} = \frac{1}{1 + \sigma_{\hat{\mu}}^2 / \sigma^2} \in (0, 1]$.
    pub epistemic_shrinkage: f64,
    /// Epistemically shrunk Kelly fraction $f^*_{\mathrm{epistemic}} = s_{\mathrm{epistemic}} \cdot f^*_{\mathrm{plug\text{-}in}}$.
    pub epistemic_kelly_fraction: f64,
    /// Expected long-run time-average growth rate at $f^*_{\mathrm{epistemic}}$ accounting for parameter variance.
    pub expected_growth_at_epistemic: f64,
    /// Expected long-run time-average growth rate at $2 f^*_{\mathrm{plug\text{-}in}}$ (overbetting ruin threshold).
    pub expected_growth_at_double_plugin: f64,
}

impl EpistemicKellyAllocation {
    /// Evaluates non-ergodic time-average growth and the Baker–McHale epistemic Kelly fraction
    /// given drift `mu_hat`, return variance `sigma2`, and effective sample size `n_eff`.
    pub fn evaluate(mu_hat: f64, sigma2: f64, n_eff: f64) -> Self {
        assert!(
            sigma2 > 0.0 && n_eff >= 1.0,
            "requires sigma2 > 0 and n_eff >= 1"
        );
        let sigma_mu2 = sigma2 / n_eff;
        let g_unlev = mu_hat - 0.5 * sigma2;
        let f_plugin = (mu_hat / sigma2).max(0.0);
        let shrink = 1.0 / (1.0 + sigma_mu2 / sigma2);
        let f_epi = shrink * f_plugin;

        let eval_expected_g = |f: f64| -> f64 { f * mu_hat - 0.5 * f * f * (sigma2 + sigma_mu2) };
        Self {
            ensemble_drift_mu: mu_hat,
            return_variance_sigma2: sigma2,
            estimation_variance_mu: sigma_mu2,
            unleveraged_time_growth: g_unlev,
            plugin_kelly_fraction: f_plugin,
            epistemic_shrinkage: shrink,
            epistemic_kelly_fraction: f_epi,
            expected_growth_at_epistemic: eval_expected_g(f_epi),
            expected_growth_at_double_plugin: eval_expected_g(2.0 * f_plugin),
        }
    }

    /// Evaluates non-ergodic drift and volatility directly from an empirical return series `returns`.
    pub fn from_return_series(returns: &[f64]) -> Self {
        assert!(returns.len() >= 2, "requires at least 2 returns");
        let mu = mean(returns);
        let var = sample_variance(returns).max(1e-12);
        Self::evaluate(mu, var, returns.len() as f64)
    }
}

/// Canguilhem (1943/1966) vital normativity and Holling (1973) ecological resilience over a bistable
/// resource system with harvesting / throughput pressure $h$:
/// $$f(x; h, K) = r x\left(1 - \frac{x}{K}\right) - \frac{h x^2}{x^2 + s^2}$$
/// Coupled with Scheffer et al. (2009) Critical Slowing Down indicators and McEwen (1998) allostatic wear.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CanguilhemHollingBasin {
    /// Harvesting / extraction pressure $h \ge 0$.
    pub harvesting_pressure_h: f64,
    /// Carrying capacity $K > 0$ (after any cumulative allostatic wear).
    pub carrying_capacity_k: f64,
    /// Upper healthy equilibrium $x^*_{\mathrm{high}}$.
    pub healthy_equilibrium: f64,
    /// Unstable Allee / fold tipping threshold $x^*_{\mathrm{tipping}}$.
    pub tipping_threshold: f64,
    /// Engineering resilience $\lambda_{\mathrm{eng}}(h) = -f'(x^*_{\mathrm{high}}) > 0$ (local recovery speed).
    pub engineering_resilience_lambda: f64,
    /// Ecological resilience / Canguilhem vital margin $W_{\mathrm{eco}}(h) = x^*_{\mathrm{high}} - x^*_{\mathrm{tipping}}$
    /// (maximum shock $\Delta x$ the system can absorb without collapsing into the degraded attractor).
    pub vital_basin_radius: f64,
    /// Scheffer et al. (2009) lag-1 autocorrelation $\rho_1(h) = \exp(-\lambda_{\mathrm{eng}}(h))$ (approaches $1$ near tipping).
    pub critical_slowing_down_rho1: f64,
    /// Stationary fluctuation variance $\mathrm{Var}_\infty(h) = \frac{\sigma^2}{2\lambda_{\mathrm{eng}}(h)}$.
    pub stationary_variance: f64,
}

impl CanguilhemHollingBasin {
    /// Solves for the equilibria, recovery speed, vital basin radius, and Critical Slowing Down indicators
    /// at extraction pressure `harvesting_h` and carrying capacity `carrying_capacity_k`
    /// (with intrinsic growth `r = 1.0`, half-saturation `s = 1.0`, and noise `sigma = 0.25`).
    pub fn evaluate(harvesting_h: f64, carrying_capacity_k: f64) -> Self {
        let r = 1.0_f64;
        let s2 = 1.0_f64;
        let sigma2 = 0.0625_f64;
        let k = carrying_capacity_k.max(2.5);
        let h = harvesting_h.max(0.0);

        let drift = |x: f64| -> f64 { r * x * (1.0 - x / k) - (h * x * x) / (x * x + s2) };
        let d_drift = |x: f64| -> f64 {
            r * (1.0 - (2.0 * x) / k) - (2.0 * h * x * s2) / ((x * x + s2) * (x * x + s2))
        };

        // Locate upper stable root near k and middle unstable root via grid scan + bisection
        let mut x_high = k;
        for step in 0..40 {
            let fx = drift(x_high);
            let dfx = d_drift(x_high);
            if dfx.abs() > 1e-9 {
                x_high = (x_high - fx / dfx).clamp(1.0, k);
            }
            if step > 10 && fx.abs() < 1e-11 {
                break;
            }
        }

        // Scan [0.2, x_high - 0.1] for sign change of drift (from negative to positive)
        let mut x_tip = 0.0_f64;
        let n_grid = 400usize;
        let hi_scan = (x_high - 0.05).max(0.25);
        for i in 0..n_grid {
            let a = 0.10 + (hi_scan - 0.10) * ((i as f64) / (n_grid as f64));
            let b = 0.10 + (hi_scan - 0.10) * (((i + 1) as f64) / (n_grid as f64));
            let fa = drift(a);
            let fb = drift(b);
            if fa <= 0.0 && fb >= 0.0 {
                let mut lo = a;
                let mut hi = b;
                for _ in 0..40 {
                    let mid = 0.5 * (lo + hi);
                    if drift(mid) <= 0.0 {
                        lo = mid;
                    } else {
                        hi = mid;
                    }
                }
                x_tip = 0.5 * (lo + hi);
                break;
            }
        }

        let lambda = (-d_drift(x_high)).max(1e-4);
        let radius = (x_high - x_tip).max(0.0);
        let rho1 = (-lambda).exp();
        let stat_var = sigma2 / (2.0 * lambda);

        Self {
            harvesting_pressure_h: h,
            carrying_capacity_k: k,
            healthy_equilibrium: x_high,
            tipping_threshold: x_tip,
            engineering_resilience_lambda: lambda,
            vital_basin_radius: radius,
            critical_slowing_down_rho1: rho1,
            stationary_variance: stat_var,
        }
    }

    /// Evaluates Bruce McEwen's (1998) allostatic load: repeated stress episodes erode carrying capacity
    /// from `initial_k` to `initial_k - wear_damage`, shrinking the vital basin radius.
    pub fn with_allostatic_wear(
        harvesting_h: f64,
        initial_k: f64,
        cumulative_wear_damage: f64,
    ) -> (Self, Self) {
        let fresh = Self::evaluate(harvesting_h, initial_k);
        let worn_k = (initial_k - cumulative_wear_damage.max(0.0)).max(3.0);
        let worn = Self::evaluate(harvesting_h, worn_k);
        (fresh, worn)
    }
}

/// Kingman (1961) heavy-traffic queue delay, Scarf (1960) $(s, S)$ inventory buffer, and
/// Simchi-Levi et al. (2014) $\mathrm{TTS} \ge \mathrm{TTR}$ supply-chain survival audit.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VitalSlackBuffer {
    /// Utilization ratio $\rho = \lambda / \mu_{\mathrm{cap}} \in (0, 1)$.
    pub utilization_rho: f64,
    /// Kingman congestion multiplier $\frac{\rho}{1 - \rho} \cdot \frac{c_a^2 + c_s^2}{2}$.
    pub kingman_wait_multiplier: f64,
    /// Local reserve buffer $B \ge 0$ (e.g., emergency beds, stock units, liquidity).
    pub reserve_buffer: f64,
    /// Crisis net burn rate $d_{\mathrm{crisis}} > 0$ per unit time.
    pub crisis_burn_rate: f64,
    /// Time-to-Survive $\mathrm{TTS} = B / d_{\mathrm{crisis}}$.
    pub time_to_survive_tts: f64,
    /// Time-to-Recover $\mathrm{TTR} > 0$ (lead time to restore capacity or resupply).
    pub time_to_recover_ttr: f64,
    /// Survival margin $\mathrm{TTS} - \mathrm{TTR}$ (must be $\ge 0$ to avoid irreversible stockout/collapse).
    pub survival_margin: f64,
}

impl VitalSlackBuffer {
    /// Evaluates Kingman queue congestion and the Simchi-Levi $\mathrm{TTS} \ge \mathrm{TTR}$ survival margin.
    pub fn evaluate(
        utilization_rho: f64,
        variability_factor: f64,
        reserve_buffer: f64,
        crisis_burn_rate: f64,
        time_to_recover_ttr: f64,
    ) -> Self {
        let rho = utilization_rho.clamp(0.01, 0.999);
        let kingman = (rho / (1.0 - rho)) * variability_factor.max(0.1);
        let burn = crisis_burn_rate.max(1e-9);
        let tts = reserve_buffer.max(0.0) / burn;
        let ttr = time_to_recover_ttr.max(0.0);
        Self {
            utilization_rho: rho,
            kingman_wait_multiplier: kingman,
            reserve_buffer,
            crisis_burn_rate: burn,
            time_to_survive_tts: tts,
            time_to_recover_ttr: ttr,
            survival_margin: tts - ttr,
        }
    }

    /// True if the vital reserve survives the recovery lead time ($\mathrm{TTS} \ge \mathrm{TTR}$).
    #[inline]
    pub const fn survives_disruption(&self) -> bool {
        self.survival_margin >= 0.0
    }
}

/// Abraham Wald (1943) conditional lethality reconstruction from returning survivors.
///
/// Given $N$ missions where $S$ aircraft return and $D = N - S$ are lost, if returning aircraft exhibit
/// $S_A$ hits in zone $A$ (wings/fuselage) and $S_B \ll S_A$ hits in zone $B$ (engines), naive empiricism
/// armors zone $A$. Wald reconstructs $\mathbb{P}(\text{Lost} \mid \text{Hit}_z)$ via equal exposure
/// $H_z = N p_z = S_z + D_z$, proving zone $B$ has the highest lethality.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WaldSurvivorshipAudit {
    /// Naive conditional hit rate among survivors in zone $A$ (wings): $S_A / S$.
    pub survivor_hit_rate_zone_a: f64,
    /// Naive conditional hit rate among survivors in zone $B$ (engines): $S_B / S$.
    pub survivor_hit_rate_zone_b: f64,
    /// Wald reconstructed conditional loss probability $\mathbb{P}(\text{Lost} \mid \text{Hit}_A) = D_A / H_A$.
    pub lethality_given_hit_zone_a: f64,
    /// Wald reconstructed conditional loss probability $\mathbb{P}(\text{Lost} \mid \text{Hit}_B) = D_B / H_B$.
    pub lethality_given_hit_zone_b: f64,
    /// True if the naive survivor ranking ($S_A > S_B$) is inverted by true lethality ($\mathbb{P}(\text{Lost}\mid B) > \mathbb{P}(\text{Lost}\mid A)$).
    pub survivorship_inversion_present: bool,
}

impl WaldSurvivorshipAudit {
    /// Reconstructs conditional lethality across two zones `(hits_on_survivors_a, total_exposed_hits_a)`
    /// and `(hits_on_survivors_b, total_exposed_hits_b)` out of `survivors_s` returning units.
    pub fn evaluate(
        survivors_s: usize,
        survivor_hits_a: usize,
        total_hits_a: usize,
        survivor_hits_b: usize,
        total_hits_b: usize,
    ) -> Self {
        assert!(
            survivors_s >= 1 && total_hits_a >= survivor_hits_a && total_hits_b >= survivor_hits_b,
            "WaldSurvivorshipAudit requires valid hit counts"
        );
        let s = survivors_s as f64;
        let rate_a = (survivor_hits_a as f64) / s;
        let rate_b = (survivor_hits_b as f64) / s;
        let lost_a = (total_hits_a - survivor_hits_a) as f64;
        let lost_b = (total_hits_b - survivor_hits_b) as f64;
        let leth_a = lost_a / (total_hits_a.max(1) as f64);
        let leth_b = lost_b / (total_hits_b.max(1) as f64);
        Self {
            survivor_hit_rate_zone_a: rate_a,
            survivor_hit_rate_zone_b: rate_b,
            lethality_given_hit_zone_a: leth_a,
            lethality_given_hit_zone_b: leth_b,
            survivorship_inversion_present: rate_a > rate_b && leth_b > leth_a,
        }
    }
}

/// Yakov Ben-Haim (2006) Info-Gap local robustness horizon $\hat{\alpha}(q, r_c)$ guarded by
/// Abraham Wald (1945) & Moshe Sniedovich (2007, 2012) global Maximin floor against "No-Man's Land".
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InfoGapMaximinSafeguard {
    /// Nominal performance at the focal point estimate $\hat{u}_0$ (zero-ing property: $\hat{\alpha}(q, R(q, \hat{u}_0)) = 0$).
    pub nominal_payoff: f64,
    /// Critical survival requirement $r_c$.
    pub critical_requirement: f64,
    /// Local Info-Gap robustness horizon $\hat{\alpha}(q, r_c) = \max\{\alpha \ge 0 : \min_{u \in U(\alpha)} R(q, u) \ge r_c\}$.
    pub local_robustness_alpha: f64,
    /// Global worst-case payoff over the entire physical support $\Omega$: $W_{\mathrm{global}}(q) = \min_{u \in \Omega} R(q, u)$.
    pub global_worst_case_payoff: f64,
    /// Global survival floor $r_{\mathrm{global}}$ demanded by Wald–Sniedovich safeguard.
    pub global_maximin_floor: f64,
    /// True if the option is a "No-Man's Land" mirage (high local $\hat{\alpha}$ around a false focal estimate,
    /// but catastrophic collapse below $r_{\mathrm{global}}$ under severe surprise).
    pub is_no_mans_land_mirage: bool,
}

impl InfoGapMaximinSafeguard {
    /// Evaluates local Info-Gap robustness and checks Sniedovich's global Wald Maximin floor.
    pub fn evaluate(
        nominal_payoff: f64,
        sensitivity_slope: f64,
        critical_requirement: f64,
        global_worst_case_payoff: f64,
        global_maximin_floor: f64,
    ) -> Self {
        let slope = sensitivity_slope.max(1e-9);
        let alpha = ((nominal_payoff - critical_requirement) / slope).max(0.0);
        let mirage = alpha > 0.0 && global_worst_case_payoff < global_maximin_floor;
        Self {
            nominal_payoff,
            critical_requirement,
            local_robustness_alpha: alpha,
            global_worst_case_payoff,
            global_maximin_floor,
            is_no_mans_land_mirage: mirage,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn epistemic_kelly_canguilhem_holling_and_wald_hold() {
        let kelly = EpistemicKellyAllocation::evaluate(0.10, 0.20, 4.0);
        assert!(kelly.epistemic_kelly_fraction < kelly.plugin_kelly_fraction);
        assert!(kelly.expected_growth_at_epistemic > kelly.expected_growth_at_double_plugin);

        let (fresh, worn) = CanguilhemHollingBasin::with_allostatic_wear(1.80, 10.0, 2.5);
        assert!(worn.vital_basin_radius < fresh.vital_basin_radius);
        let stressed = CanguilhemHollingBasin::evaluate(2.45, 10.0);
        assert!(stressed.critical_slowing_down_rho1 > fresh.critical_slowing_down_rho1);
        assert!(stressed.stationary_variance > fresh.stationary_variance);

        let wald = WaldSurvivorshipAudit::evaluate(800, 232, 250, 40, 250);
        assert!(wald.survivorship_inversion_present);
        assert!((wald.lethality_given_hit_zone_b - 0.84).abs() < 1e-12);
    }

    #[test]
    fn kingman_slack_and_sniedovich_safeguard_detect_fragility() {
        let tight = VitalSlackBuffer::evaluate(0.98, 1.0, 2.0, 1.0, 5.0);
        let safe = VitalSlackBuffer::evaluate(0.80, 1.0, 8.0, 1.0, 5.0);
        assert!(tight.kingman_wait_multiplier > 10.0 * safe.kingman_wait_multiplier);
        assert!(!tight.survives_disruption());
        assert!(safe.survives_disruption());

        let mirage = InfoGapMaximinSafeguard::evaluate(100.0, 10.0, 60.0, -500.0, -50.0);
        assert!(mirage.is_no_mans_land_mirage);
    }
}
