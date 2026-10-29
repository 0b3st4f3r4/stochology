//! Lens 2 — Time: Non-Ergodicity (Itô), Asymmetric Kelly Survival,
//! and Heavy-Tailed Mixing Time (§1.3.2).
//!
//! Implements:
//! - Itô quadratic variance drain:
//!   $$g_{\text{time}} = \mathbb{E}[\ln M] = \mu - \frac{1}{2}\sigma^2 < \ln \mathbb{E}[M] = \mu = g_{\text{ensemble}}$$
//! - Optimal Kelly Fraction for asymmetric returns (gain $b$, loss $a$, win probability $p$):
//!   $$f^* = \frac{p}{a} - \frac{1 - p}{b}$$
//! - Mixing-time audit for stochastic / Langevin sampling via the log-Sobolev constant
//!   associated with $\eta_{\mathrm{KL}}$ and warm-start bounds under tail profile $H_R = \pi(w > R)$.

use stochology_core::{Rng, median};

/// Non-Ergodicity and Kelly Survival Diagnostic.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TimeKellyDiagnostic {
    /// Ensemble average growth rate $g_{\text{ensemble}} = \mu$.
    pub ensemble_rate_mu: f64,
    /// Volatility $\sigma$.
    pub volatility_sigma: f64,
    /// Compound time-average growth rate experienced by a single trajectory $g_{\text{time}} = \mu - \frac{1}{2}\sigma^2$.
    pub time_rate_ito: f64,
    /// Quadratic variance drain $\frac{1}{2}\sigma^2$.
    pub variance_drain: f64,
    /// Indicates whether the system grows in the ensemble average ($g_{\text{ensemble}} > 0$) while collapsing in time ($g_{\text{time}} < 0$).
    pub non_ergodic_trap: bool,
    /// Optimal asymmetric Kelly fraction $f^* = \frac{p}{a} - \frac{1 - p}{b}$.
    pub asymmetric_kelly_fraction: f64,
    /// Exact logarithmic growth rate evaluated at the Kelly fraction $f^*$.
    pub log_growth_at_kelly: f64,
}

/// Computes Itô's time-average growth rate $g_{\text{time}} = \mu - \frac{1}{2}\sigma^2$.
#[inline]
pub fn ito_time_growth_rate(mu: f64, sigma: f64) -> f64 {
    mu - 0.5 * sigma * sigma
}

/// Computes the Optimal Kelly Fraction for asymmetric returns (§1.3.2):
/// $$f^* = \frac{p}{a} - \frac{1 - p}{b}$$
/// where $p \in (0, 1)$ is the calibrated win probability, $b > 0$ is the fractional gain on success,
/// and $a \in (0, 1]$ is the fractional loss on failure.
#[inline]
pub fn asymmetric_kelly_fraction(p: f64, gain_b: f64, loss_a: f64) -> f64 {
    assert!(p > 0.0 && p < 1.0, "probability p must be in (0, 1)");
    assert!(
        gain_b > 0.0 && loss_a > 0.0,
        "gain b and loss a must be positive"
    );
    ((p / loss_a) - ((1.0 - p) / gain_b)).max(0.0)
}

/// Expected logarithmic growth rate $\mathbb{E}[\ln(1 + R(f))] = p \ln(1 + f b) + (1 - p)\ln(1 - f a)$.
#[inline]
pub fn asymmetric_bet_log_growth(p: f64, gain_b: f64, loss_a: f64, fraction_f: f64) -> f64 {
    if 1.0 - fraction_f * loss_a <= 0.0 || 1.0 + fraction_f * gain_b <= 0.0 {
        f64::NEG_INFINITY
    } else {
        p * (1.0 + fraction_f * gain_b).ln() + (1.0 - p) * (1.0 - fraction_f * loss_a).ln()
    }
}

impl TimeKellyDiagnostic {
    pub fn compute(mu: f64, sigma: f64, p: f64, gain_b: f64, loss_a: f64) -> Self {
        let g_time = ito_time_growth_rate(mu, sigma);
        let f_star = asymmetric_kelly_fraction(p, gain_b, loss_a);
        let g_kelly = asymmetric_bet_log_growth(p, gain_b, loss_a, f_star);
        Self {
            ensemble_rate_mu: mu,
            volatility_sigma: sigma,
            time_rate_ito: g_time,
            variance_drain: 0.5 * sigma * sigma,
            non_ergodic_trap: mu > 0.0 && g_time < 0.0,
            asymmetric_kelly_fraction: f_star,
            log_growth_at_kelly: g_kelly,
        }
    }
}

/// Simulates multiplicative trajectories with an absorbing ruin barrier (`ruin_barrier_fraction`),
/// returning `(ensemble_mean, trajectory_median, ruin_probability)`.
#[allow(clippy::too_many_arguments)]
pub fn simulate_absorbing_trajectories(
    seed: u64,
    steps: usize,
    trajectories: usize,
    p: f64,
    gain_b: f64,
    loss_a: f64,
    exposure_fraction: f64,
    ruin_barrier_fraction: f64,
) -> (f64, f64, f64) {
    if trajectories == 0 {
        return (1.0, 1.0, 0.0);
    }
    let mut rng = Rng::new(seed);
    let mut final_wealth = Vec::with_capacity(trajectories);
    let mut ruined = 0usize;
    for _ in 0..trajectories {
        let mut w = 1.0;
        if w <= ruin_barrier_fraction {
            w = ruin_barrier_fraction;
        } else {
            for _ in 0..steps {
                if rng.bernoulli(p) {
                    w *= 1.0 + exposure_fraction * gain_b;
                } else {
                    w *= (1.0 - exposure_fraction * loss_a).max(0.0);
                }
                if w <= ruin_barrier_fraction {
                    w = ruin_barrier_fraction;
                    break;
                }
            }
        }
        if w <= ruin_barrier_fraction {
            ruined += 1;
        }
        final_wealth.push(w);
    }
    let ens_mean = final_wealth.iter().sum::<f64>() / (trajectories as f64);
    let traj_med = median(&final_wealth);
    let p_ruin = (ruined as f64) / (trajectories as f64);
    (ens_mean, traj_med, p_ruin)
}

/// Solves for the exact empirical Kelly optimal leverage fraction $f_{\text{Kelly}}^\star = \arg\max_{f \ge 0} \frac{1}{n}\sum_{t=1}^n \ln(1 + f R_t)$
/// on a continuous return series `returns` via damped Newton-Raphson optimization, and compares it against
/// the continuous-time Gaussian Merton approximation $f_{\text{Merton}} = \hat{\mu} / \hat{\sigma}^2$.
/// Returns `(f_kelly_empirical, max_time_log_growth, f_gaussian_merton)`.
pub fn empirical_kelly_optimal_fraction(returns: &[f64]) -> (f64, f64, f64) {
    assert!(
        returns.len() >= 2,
        "requires at least 2 return observations"
    );
    let n = returns.len() as f64;
    let mu = stochology_core::mean(returns);
    let var = stochology_core::sample_variance(returns).max(1e-12);
    let f_merton = (mu / var).max(0.0);
    if mu <= 0.0 {
        return (0.0, 0.0, 0.0);
    }

    let min_r = returns.iter().copied().fold(0.0_f64, f64::min);
    let f_max = if min_r < -1e-9 { 0.98 / (-min_r) } else { 10.0 };

    let mut f = f_merton.min(0.8 * f_max);
    for _ in 0..50 {
        let mut g1 = 0.0;
        let mut g2 = 0.0;
        for &r in returns {
            let denom = (1.0 + f * r).max(1e-9);
            let ratio = r / denom;
            g1 += ratio;
            g2 -= ratio * ratio;
        }
        g1 /= n;
        g2 /= n;
        if g1.abs() < 1e-12 || g2.abs() < 1e-15 {
            break;
        }
        let step = g1 / (-g2);
        f = (f + step).clamp(0.0, f_max);
    }

    let g_opt: f64 = returns
        .iter()
        .map(|&r| (1.0 + f * r).max(1e-12).ln())
        .sum::<f64>()
        / n;
    (f, g_opt, f_merton)
}

/// Mixing-Time Audit for Langevin / MCMC sampling under Heavy-Tailed Posterior Profiles (§1.3.2):
/// $$d_{\mathrm{TV}}(\mu_t, \pi) \le \sqrt{\frac{1}{2} D_{\mathrm{KL}}(\mu_0 \| \pi)\, \eta_{\mathrm{KL}}^t} + H_R$$
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MixingTimeAudit {
    pub local_eta_kl: f64,
    /// Log-Sobolev constant $\alpha_{\mathrm{LSI}} = -\frac{1}{2}\ln \eta_{\mathrm{KL}}$.
    pub log_sobolev_constant: f64,
    /// Tail profile mass $H_R = \pi(w > R)$ outside the warm-start truncation ball.
    pub tail_profile_h_r: f64,
    pub initial_kl: f64,
    /// Minimum number of steps $t$ required to reach total variation tolerance $\varepsilon$
    /// (or `usize::MAX` if $\varepsilon \le H_R$ renders the tolerance unreachable).
    pub mixing_steps: usize,
}

impl MixingTimeAudit {
    pub fn compute(
        local_eta_kl: f64,
        tail_profile_h_r: f64,
        initial_kl: f64,
        tolerance_eps: f64,
    ) -> Self {
        assert!(local_eta_kl > 0.0 && local_eta_kl < 1.0);
        assert!(tolerance_eps > 0.0 && tail_profile_h_r >= 0.0);
        let alpha_lsi = -0.5 * local_eta_kl.ln();
        let steps = if tolerance_eps <= tail_profile_h_r {
            usize::MAX
        } else {
            let eff_eps = tolerance_eps - tail_profile_h_r;
            if 0.5 * initial_kl <= eff_eps * eff_eps {
                0
            } else {
                let num = (2.0 * eff_eps * eff_eps / initial_kl).ln();
                let den = local_eta_kl.ln();
                (num / den).ceil() as usize
            }
        };
        Self {
            local_eta_kl,
            log_sobolev_constant: alpha_lsi,
            tail_profile_h_r,
            initial_kl,
            mixing_steps: steps,
        }
    }
}

/// Robust Sensor Fusion, Joseph-Form Kalman Covariance Update, Huber-Kalman Filtering under
/// Heavy-Tailed Sensor Glitches, and Tube-Buffered MPC vs. Certainty Equivalence (`RobustKalmanControlAudit`).
#[derive(Debug, Clone, PartialEq)]
pub struct RobustKalmanControlAudit {
    /// Fused variance $\sigma_{\text{fused}}^2 = (\sigma_1^{-2} + \sigma_2^{-2})^{-1}$ from two sensors $(\sigma_1^2, \sigma_2^2)$.
    pub fused_sensor_variance: f64,
    /// Joseph-form covariance identity residual $\bigl|P_{t|t}^{\text{Joseph}} - (P_{t|t-1}^{-1} + H^2 R^{-1})^{-1}\bigr|$.
    pub joseph_covariance_identity_error: f64,
    /// State estimation RMSE of standard L2 Kalman filter vs. Huber innovation-clipped Kalman filter under Pareto glitches.
    pub l2_kalman_vs_huber_kalman_rmse: (f64, f64),
    /// Lap survival rate (zero barrier collisions) of naive Certainty-Equivalent control vs. Tube/CVaR-Buffered MPC.
    pub certainty_equivalent_vs_tube_mpc_survival: (f64, f64),
    /// True if precision fusion and Joseph-form covariance identities close to machine precision.
    pub identities_closed: bool,
}

impl RobustKalmanControlAudit {
    /// Evaluates precision sensor fusion, the Joseph-form Kalman covariance identity, Huber-Kalman vs. L2 Kalman
    /// state tracking on a series with heavy-tailed glitches, and Tube-MPC vs. Certainty Equivalence.
    pub fn evaluate_trajectory(
        true_states: &[f64],
        glitchy_observations: &[f64],
        process_var_q: f64,
        nominal_obs_var_r: f64,
        huber_threshold_c: f64,
        disturbances: &[f64],
        safety_buffer_u: f64,
    ) -> Self {
        // 1. Two-sensor precision fusion identity: sigma_1^2 = 0.64, sigma_2^2 = 0.36 => fused = 1 / (1/0.64 + 1/0.36) = 0.2304 < 0.36
        let s1_sq = 0.64_f64;
        let s2_sq = 0.36_f64;
        let fused_var = 1.0 / (1.0 / s1_sq + 1.0 / s2_sq);

        // 2. Joseph-form posterior covariance update P_{t|t} = (1 - K H)^2 P_{t|t-1} + K^2 R == 1 / (1/P_{t|t-1} + H^2/R)
        let p_prior = 1.25_f64;
        let h_obs = 1.40_f64;
        let r_var = 0.50_f64;
        let s_innov = h_obs * h_obs * p_prior + r_var;
        let k_gain = p_prior * h_obs / s_innov;
        let p_joseph = (1.0 - k_gain * h_obs).powi(2) * p_prior + k_gain * k_gain * r_var;
        let p_precision = 1.0 / (1.0 / p_prior + (h_obs * h_obs) / r_var);
        let joseph_err = (p_joseph - p_precision).abs();

        // 3. Standard L2 Kalman vs. Huber-clipped Innovation Kalman over `true_states` and `glitchy_observations`
        let n = true_states.len().min(glitchy_observations.len()).max(1);
        let mut x_l2 = true_states[0];
        let mut p_l2 = 1.0_f64;
        let mut x_hub = true_states[0];
        let mut p_hub = 1.0_f64;
        let mut sq_err_l2 = 0.0_f64;
        let mut sq_err_hub = 0.0_f64;

        for i in 0..n {
            let p_pred_l2 = p_l2 + process_var_q;
            let s_l2 = p_pred_l2 + nominal_obs_var_r;
            let k_l2 = p_pred_l2 / s_l2;
            let nu_l2 = glitchy_observations[i] - x_l2;
            x_l2 += k_l2 * nu_l2;
            p_l2 = (1.0 - k_l2).powi(2) * p_pred_l2 + k_l2 * k_l2 * nominal_obs_var_r;

            let p_pred_hub = p_hub + process_var_q;
            let s_hub = p_pred_hub + nominal_obs_var_r;
            let k_hub = p_pred_hub / s_hub;
            let nu_hub = glitchy_observations[i] - x_hub;
            let std_innov = nu_hub / s_hub.sqrt().max(1e-12);
            let weight_hub = if std_innov.abs() <= huber_threshold_c {
                1.0
            } else {
                huber_threshold_c / std_innov.abs()
            };
            x_hub += k_hub * (weight_hub * nu_hub);
            p_hub = (1.0 - weight_hub * k_hub).powi(2) * p_pred_hub
                + (weight_hub * k_hub).powi(2) * nominal_obs_var_r;

            sq_err_l2 += (x_l2 - true_states[i]).powi(2);
            sq_err_hub += (x_hub - true_states[i]).powi(2);
        }
        let rmse_l2 = (sq_err_l2 / (n as f64)).sqrt();
        let rmse_hub = (sq_err_hub / (n as f64)).sqrt();

        // 4. Certainty-Equivalent Control (u_CE = 0.05 nominal mean clearance) vs. Tube-Buffered MPC (u_MPC = safety_buffer_u):
        let u_ce = 0.05_f64;
        let mut surv_ce = 0usize;
        let mut surv_mpc = 0usize;
        let d_len = disturbances.len().max(1);
        for &xi in disturbances {
            if u_ce - xi > 0.0 {
                surv_ce += 1;
            }
            if safety_buffer_u - xi > 0.0 {
                surv_mpc += 1;
            }
        }
        let rate_ce = (surv_ce as f64) / (d_len as f64);
        let rate_mpc = (surv_mpc as f64) / (d_len as f64);

        let identities_closed = fused_var < s1_sq.min(s2_sq)
            && (fused_var - 0.2304).abs() < 1e-12
            && joseph_err < 1e-12;

        Self {
            fused_sensor_variance: fused_var,
            joseph_covariance_identity_error: joseph_err,
            l2_kalman_vs_huber_kalman_rmse: (rmse_l2, rmse_hub),
            certainty_equivalent_vs_tube_mpc_survival: (rate_ce, rate_mpc),
            identities_closed,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn peters_coin_and_asymmetric_kelly() {
        // Peters coin: p = 0.5, gain b = 0.50 (+50%), loss a = 0.40 (-40%)
        // Full exposure f = 1.0 has positive ensemble mean (+5%), but E[ln M] < 0!
        // Optimal Kelly fraction f* = 0.5/0.4 - 0.5/0.5 = 1.25 - 1.0 = 0.25 achieves positive time growth!
        let diag = TimeKellyDiagnostic::compute(0.05, 0.45, 0.50, 0.50, 0.40);
        assert!(diag.non_ergodic_trap);
        assert!((diag.asymmetric_kelly_fraction - 0.25).abs() < 1e-12);
        assert!(diag.log_growth_at_kelly > 0.0);

        let (_, med_overbet, ruin_overbet) =
            simulate_absorbing_trajectories(2026, 300, 1000, 0.5, 0.50, 0.40, 1.0, 0.05);
        let (_, med_kelly, ruin_kelly) =
            simulate_absorbing_trajectories(2026, 300, 1000, 0.5, 0.50, 0.40, 0.25, 0.05);
        assert!(med_kelly > 1.5 && med_overbet < 0.1);
        assert!(
            ruin_overbet > 0.80 && ruin_kelly < 0.05,
            "ruin_overbet={ruin_overbet}, ruin_kelly={ruin_kelly}"
        );
        assert_eq!(
            simulate_absorbing_trajectories(2026, 10, 0, 0.5, 0.5, 0.4, 0.25, 0.05),
            (1.0, 1.0, 0.0)
        );
        // Trajectory ruined on the final step (steps = 1, p = 0.0 -> loss drops wealth below barrier)
        // must be immediately clamped to ruin_barrier_fraction.
        let (ens_ruin, med_ruin, p_ruin) =
            simulate_absorbing_trajectories(2026, 1, 10, 0.0, 0.5, 0.98, 1.0, 0.05);
        assert!((ens_ruin - 0.05).abs() < 1e-12);
        assert!((med_ruin - 0.05).abs() < 1e-12);
        assert!((p_ruin - 1.0).abs() < 1e-12);
    }

    #[test]
    fn mixing_time_grows_as_contraction_weakens_and_respects_tail_mass_h_r() {
        let fast = MixingTimeAudit::compute(0.80, 0.001, 5.0, 0.05);
        let heavier_tail = MixingTimeAudit::compute(0.80, 0.03, 5.0, 0.05);
        let slow = MixingTimeAudit::compute(0.98, 0.01, 5.0, 0.05);
        let unreachable = MixingTimeAudit::compute(0.80, 0.06, 5.0, 0.05);
        assert!(heavier_tail.mixing_steps > fast.mixing_steps);
        assert!(slow.mixing_steps > fast.mixing_steps * 5);
        assert_eq!(unreachable.mixing_steps, usize::MAX);

        let returns = [0.08, 0.07, 0.06, 0.09, -0.32, 0.07, 0.08, 0.06];
        let (f_emp, g_emp, f_merton) = empirical_kelly_optimal_fraction(&returns);
        assert!(f_emp > 0.0 && f_emp < f_merton && g_emp > 0.0);
    }
}
