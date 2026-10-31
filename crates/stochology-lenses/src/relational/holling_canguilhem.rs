//! Holling's (1973) **Engineering vs. Ecological Resilience** and Canguilhem's (1943/2009)
//! **Biological Normativity** (*"une marge de tolérance des infidélités du milieu"*)
//! (`notes/A Ética da Mira.md` §5.3, lines 229–238).
//!
//! Models a bistable biological/ecological system with an Allee threshold and extraction pressure $h$:
//! $$\frac{dx}{dt} = f(x; h) = r\,x\left(1 - \frac{x}{K}\right)\left(\frac{x}{A} - 1\right) - h\,x$$
//! proving that hyper-optimizing short-term extraction ($h \to h_{\text{crit}}$) shrinks the
//! **Ecological Basin of Attraction** $\mathcal{B}_{\mathrm{eco}}(h) = x^*(h) - x_{\text{sep}}(h)$
//! below the amplitude of environmental infidelities, causing irreversible collapse into the
//! absorbing ruin state $x = 0$ under 4th-order Runge-Kutta (`RK4`) integration.

/// Audit of **Holling's (1973) Engineering vs. Ecological Resilience** and **Canguilhem's (1943/2009)
/// Normative Margin of Tolerance for Environmental Infidelities** (`notes/A Ética da Mira.md` §5.3).
#[derive(Debug, Clone, PartialEq)]
pub struct HollingCanguilhemResilienceAudit {
    /// Upper stable equilibrium $x^*(h)$ for `(buffered_vital_regime, tight_zero_slack_regime)`.
    pub upper_equilibrium_buffered_vs_tight: (f64, f64),
    /// Unstable Allee tipping-point separatrix $x_{\text{sep}}(h)$ for `(buffered_vital_regime, tight_zero_slack_regime)`.
    pub tipping_separatrix_buffered_vs_tight: (f64, f64),
    /// Local **Engineering Resilience** $\lambda_{\mathrm{eng}}(h) = -f'(x^*(h)) > 0$ for `(buffered, tight)`.
    /// Both regimes are locally stable against infinitesimal perturbations!
    pub engineering_resilience_buffered_vs_tight: (f64, f64),
    /// **Holling Ecological Resilience** (basin width $\mathcal{B}_{\mathrm{eco}}(h) = x^*(h) - x_{\text{sep}}(h)$)
    /// for `(buffered, tight)`.
    pub ecological_basin_width_buffered_vs_tight: (f64, f64),
    /// **Canguilhem Normative Margin** $M_{\mathrm{Canguilhem}} = \mathcal{B}_{\mathrm{eco}}(h) / \Delta x_{\text{shock}}$
    /// against an environmental infidelity shock $\Delta x_{\text{shock}}$ for `(buffered, tight)`.
    pub canguilhem_tolerance_ratio_buffered_vs_tight: (f64, f64),
    /// Post-shock `RK4` trajectory final state $x(T)$ after environmental shock $\Delta x_{\text{shock}}$ for `(buffered, tight)`.
    pub post_shock_rk4_final_state_buffered_vs_tight: (f64, f64),
    /// Exact discriminant identity residual $|\bigl(x^*(h) - x_{\text{sep}}(h)\bigr)^2 - \bigl((K - A)^2 - 4AKh/r\bigr)|$.
    pub basin_discriminant_identity_error: f64,
    /// True if both regimes have positive local engineering resilience ($\lambda_{\mathrm{eng}} > 0$),
    /// yet only the buffered Canguilhem regime ($M_{\mathrm{Canguilhem}} > 1$) survives the environmental infidelity shock.
    pub holling_canguilhem_resilience_verified: bool,
}

impl HollingCanguilhemResilienceAudit {
    fn rhs(x: f64, r: f64, k: f64, a: f64, h: f64) -> f64 {
        if x <= 0.0 {
            return 0.0;
        }
        r * x * (1.0 - x / k) * (x / a - 1.0) - h * x
    }

    fn rk4_integrate(x0: f64, r: f64, k: f64, a: f64, h: f64, dt: f64, steps: usize) -> f64 {
        let mut x = x0.max(0.0);
        for _ in 0..steps {
            let k1 = Self::rhs(x, r, k, a, h);
            let k2 = Self::rhs((x + 0.5 * dt * k1).max(0.0), r, k, a, h);
            let k3 = Self::rhs((x + 0.5 * dt * k2).max(0.0), r, k, a, h);
            let k4 = Self::rhs((x + dt * k3).max(0.0), r, k, a, h);
            x = (x + (dt / 6.0) * (k1 + 2.0 * k2 + 2.0 * k3 + k4)).max(0.0);
        }
        x
    }

    /// Evaluates Holling's engineering vs. ecological resilience and Canguilhem's margin of tolerance
    /// across a buffered vital-margin regime (`h_buffered`) and a zero-idleness regime (`h_tight`)
    /// subjected to an environmental infidelity shock `shock_delta_x`.
    pub fn evaluate(
        r: f64,
        carrying_capacity_k: f64,
        allee_threshold_a: f64,
        h_buffered: f64,
        h_tight: f64,
        shock_delta_x: f64,
    ) -> Self {
        let h_crit = r * (carrying_capacity_k - allee_threshold_a).powi(2)
            / (4.0 * allee_threshold_a * carrying_capacity_k);
        assert!(
            r > 0.0
                && carrying_capacity_k > allee_threshold_a
                && allee_threshold_a > 0.0
                && h_buffered >= 0.0
                && h_buffered < h_tight
                && h_tight < h_crit
                && shock_delta_x > 0.0,
            "requires 0 <= h_buffered < h_tight < h_crit and positive shock"
        );

        let solve_regime = |h: f64| -> (f64, f64, f64, f64, f64) {
            let disc = (carrying_capacity_k - allee_threshold_a).powi(2)
                - 4.0 * allee_threshold_a * carrying_capacity_k * (h / r);
            let root_disc = disc.max(0.0).sqrt();
            let x_star = 0.5 * (carrying_capacity_k + allee_threshold_a + root_disc);
            let x_sep = 0.5 * (carrying_capacity_k + allee_threshold_a - root_disc);
            let basin = x_star - x_sep;
            // Exact derivative -f'(x*) at upper root of quadratic
            let lambda_eng = (r * x_star * basin) / (allee_threshold_a * carrying_capacity_k);
            let id_err = (basin * basin - disc).abs();
            (x_star, x_sep, basin, lambda_eng, id_err)
        };

        let (x_star_b, x_sep_b, basin_b, lambda_b, err_b) = solve_regime(h_buffered);
        let (x_star_t, x_sep_t, basin_t, lambda_t, err_t) = solve_regime(h_tight);
        let basin_discriminant_identity_error = err_b.max(err_t);

        let cang_b = basin_b / shock_delta_x;
        let cang_t = basin_t / shock_delta_x;

        let final_b = Self::rk4_integrate(
            (x_star_b - shock_delta_x).max(0.0),
            r,
            carrying_capacity_k,
            allee_threshold_a,
            h_buffered,
            0.02,
            1_000,
        );
        let final_t = Self::rk4_integrate(
            (x_star_t - shock_delta_x).max(0.0),
            r,
            carrying_capacity_k,
            allee_threshold_a,
            h_tight,
            0.02,
            1_000,
        );

        let verified = lambda_b > 0.0
            && lambda_t > 0.0
            && cang_b > 1.50
            && cang_t < 0.80
            && (final_b - x_star_b).abs() < 1e-3
            && final_t < 1e-3
            && basin_discriminant_identity_error < 1e-12;

        Self {
            upper_equilibrium_buffered_vs_tight: (x_star_b, x_star_t),
            tipping_separatrix_buffered_vs_tight: (x_sep_b, x_sep_t),
            engineering_resilience_buffered_vs_tight: (lambda_b, lambda_t),
            ecological_basin_width_buffered_vs_tight: (basin_b, basin_t),
            canguilhem_tolerance_ratio_buffered_vs_tight: (cang_b, cang_t),
            post_shock_rk4_final_state_buffered_vs_tight: (final_b, final_t),
            basin_discriminant_identity_error,
            holling_canguilhem_resilience_verified: verified,
        }
    }

    /// Assumed: canonical Holling (1973) & Canguilhem (1943/2009) benchmark ($r = 1, K = 10, A = 2$,
    /// $h_{\text{buffered}} = 0.20$ vs. $h_{\text{tight}} = 0.72$, environmental shock $\Delta x = 3.80$).
    pub fn canonical_benchmark() -> Self {
        Self::evaluate(1.0, 10.0, 2.0, 0.20, 0.72, 3.80)
    }
}

/// Donald MacKenzie's (2006, *An Engine, Not a Camera*) four sociotechnical regimes of model performativity
/// on Bridge $L_3 \to L_1$ (`notes/A Ética da Mira.md` §5.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MacKenziePerformativityRegime {
    /// **Camera / Passive ($\varepsilon \approx 0$):** Model deployment leaves the world distribution invariant.
    CameraPassive,
    /// **Effective Performativity:** Deployment alters practical processes in alignment with substantive reality ($\Delta \hat{M} \cdot \Delta \hat{Y}^* > 0$).
    EffectivePerformative,
    /// **Barnesian Self-Fulfilling Prophecy:** Apparent predictive error shrinks ($|\hat{M}_t - \theta_t| \to 0$) only because the rule deprived the evaluated group of resources ($\Delta \hat{Y}^* \le 0$).
    BarnesianSelfFulfilling,
    /// **Counterperformativity:** Control pressure improves the official proxy ($\Delta \hat{M} > 0$) while destroying the substantive reality audited by $S3^*$ ($\Delta \hat{Y}^* < 0$).
    CounterperformativeDestructive,
}

impl MacKenziePerformativityRegime {
    /// Classifies the performative regime from Wasserstein shift `w1_shift`, proxy change `delta_proxy`,
    /// substantive $S3^*$ outcome change `delta_substantive`, and apparent prediction error reduction `error_shrunk`.
    pub fn classify(
        w1_shift: f64,
        delta_proxy: f64,
        delta_substantive: f64,
        error_shrunk: bool,
    ) -> Self {
        if w1_shift.abs() < 1e-6 {
            return Self::CameraPassive;
        }
        if delta_proxy > 0.0 && delta_substantive < 0.0 {
            return Self::CounterperformativeDestructive;
        }
        if error_shrunk && delta_substantive <= 0.0 {
            return Self::BarnesianSelfFulfilling;
        }
        Self::EffectivePerformative
    }
}

/// Unified audit of **Filter III (Constitutive Tensions 5 and 6)** across all fields of knowledge
/// (`notes/A Ética da Mira.md` §5.1–5.3):
/// 1. **Tension 5 (Passive Observation vs. Performative Intervention):** MacKenzie's (2006) four performativity regimes
///    (`CounterperformativeDestructive` vs. `BarnesianSelfFulfilling`), Miller, Milli & Hardt (2020) / Kleinberg & Raghavan (2020)
///    causal improvement vs. cosmetic gaming & strategic social burden gap, Hudgens & Halloran (2008) SUTVA spillover and
///    general-equilibrium scale-up attenuation, and Tian & Pearl (2000) exact non-parametric counterfactual bounds on
///    $\mathrm{PN}$ (*Probability of Necessity*), $\mathrm{PS}$ (*Probability of Sufficiency*), and $\mathrm{PNS}$;
/// 2. **Tension 6 (Ensemble Average vs. Single Trajectory in Irreversible Time):** Scheffer et al. (2009) *Critical Slowing Down*
///    early warning signals ($\rho_1(h) \to 1$ and $\sigma_X^2(h)$ inflation) on the Holling-Canguilhem basin, Samuelson (1971, 1979) &
///    Baker-McHale (2013) epistemic fractional Kelly shrinkage, Abraham Wald (1943) bomber survivorship inversion plus parasitic vs.
///    legitimate survival audit, and Bruce McEwen (1998) allostatic cumulative wear eroding carrying capacity $K_t$.
#[derive(Debug, Clone, PartialEq)]
pub struct FilterThreeTensionsFiveAndSixAudit {
    /// Tension 5: classified MacKenzie regimes for `(counterperformative_episode, barnesian_episode, effective_episode)`.
    pub tension5_mackenzie_regimes: (
        MacKenziePerformativityRegime,
        MacKenziePerformativityRegime,
        MacKenziePerformativityRegime,
    ),
    /// Tension 5: Kleinberg-Raghavan causal-vs-gaming net payoff under `(proxy_rule, causal_icp_rule)` and social burden ratio $\mathrm{Burden}_A / \mathrm{Burden}_B$.
    pub tension5_causal_effort_payoff_and_burden_ratio: (f64, f64, f64),
    /// Tension 5: Hudgens-Halloran pilot effect $\tau(0.05)$ vs. full-scale congestion-attenuated effect $\tau(1.00)$.
    pub tension5_spillover_pilot_vs_scale_effect: (f64, f64),
    /// Tension 5: Tian & Pearl (2000) exact non-parametric counterfactual bounds `(pn_lo, pn_hi, ps_lo, ps_hi, pns_lo, pns_hi)`.
    pub tension5_tian_pearl_pn_ps_pns_bounds: (f64, f64, f64, f64, f64, f64),
    /// Tension 6: Scheffer *Critical Slowing Down* lag-1 autocorrelation $\rho_1$ and variance ratio $\sigma_{\mathrm{tight}}^2 / \sigma_{\mathrm{buffered}}^2$.
    pub tension6_scheffer_lag1_autocorr_and_var_ratio: (f64, f64, f64),
    /// Tension 6: naive plug-in full Kelly $f_{\mathrm{plug\text{-}in}}^*$ vs. Baker-McHale/Manski epistemic fractional Kelly $f_{\mathrm{epist}}^*$.
    pub tension6_plugin_vs_epistemic_kelly_fraction: (f64, f64),
    /// Tension 6: Abraham Wald (1943) reconstructed conditional lethality $\mathbb{P}(\text{Downed}\mid\text{Hit})$ for `(wings_visible, engine_silent)`.
    pub tension6_wald_wing_vs_engine_lethality: (f64, f64),
    /// Tension 6: parasitic externalizing strategy `(private_time_growth, systemic_commons_time_growth)`.
    pub tension6_parasitic_private_vs_systemic_growth: (f64, f64),
    /// Tension 6: McEwen allostatic capacity erosion `(initial_basin_width, worn_basin_width_after_chronic_overload)`.
    pub tension6_allostatic_basin_erosion: (f64, f64),
    /// True if all checks and identities across Tensions 5 and 6 hold.
    pub all_tensions_5_and_6_verified: bool,
}

impl FilterThreeTensionsFiveAndSixAudit {
    /// Evaluates the canonical multi-disciplinary audit for Constitutive Tensions 5 and 6.
    pub fn canonical_benchmark() -> Self {
        // --- Tension 5: MacKenzie Regimes, Strategic Effort vs Gaming, SUTVA Spillover, Tian-Pearl Bounds ---
        let reg_counter = MacKenziePerformativityRegime::classify(0.42, 0.25, -0.30, true);
        let reg_barnesian = MacKenziePerformativityRegime::classify(0.35, -0.20, -0.22, true);
        let reg_effective = MacKenziePerformativityRegime::classify(0.30, 0.18, 0.16, true);

        // Kleinberg & Raghavan (2020) / Miller, Milli & Hardt (2020):
        // Agent allocates 1 unit of effort between cosmetic proxy gaming (cost c_g = 0.4) and causal parent improvement (cost c_c = 1.0).
        // Under naive proxy rule (weights w_g = 0.80, w_c = 0.30), marginal return of gaming (0.80/0.40 = 2.0) beats causal effort (0.30/1.0 = 0.30).
        // Under ICP-screened rule (w_g = 0.05, w_c = 0.90), causal return (0.90) beats gaming (0.125).
        let net_causal_advantage_proxy_rule = (0.30 / 1.0) - (0.80 / 0.40); // -1.70 (gaming dominates)
        let net_causal_advantage_icp_rule = (0.90 / 1.0) - (0.05 / 0.40); // +0.775 (genuine causal improvement dominates)
        // Milli et al. (2019) Social Burden ratio when disadvantaged group A faces marginal distance 1.8x group B:
        let social_burden_ratio = 1.80_f64;

        // Hudgens & Halloran (2008) SUTVA Spillover / Queue Congestion on scale-up:
        // Direct treatment gain = +0.22, negative congestion spillover = -0.18 * p_sat
        let tau_at_sat = |p_sat: f64| -> f64 { 0.22 - 0.18 * p_sat };
        let tau_pilot = tau_at_sat(0.05); // +0.211 at 5% pilot saturation
        let tau_scale = tau_at_sat(1.00); // +0.040 at 100% universal rollout

        // Tian & Pearl (2000) exact counterfactual bounds combining experimental P(Y_1=1), P(Y_0=1)
        // and observational P(X=1, Y=1), P(X=0, Y=0), P(Y=1):
        // Experimental: P(Y_1 = 1) = 0.65, P(Y_0 = 1) = 0.15 (P(Y_0 = 0) = 0.85, P(Y_1 = 0) = 0.35)
        // Observational: P(X=1) = 0.50, P(X=1, Y=1) = 0.35, P(X=0, Y=1) = 0.05 -> P(Y=1) = 0.40, P(Y=0) = 0.60, P(X=0, Y=0) = 0.45
        let (p_y1_1, p_y0_1) = (0.65_f64, 0.15_f64);
        let p_y0_0 = 1.0 - p_y0_1;
        let (p_x1_y1, p_x0_y0) = (0.35_f64, 0.45_f64);
        let p_y_1 = 0.40_f64;

        // Tian & Pearl (2000, Eqs. 15-21):
        // PNS bounds:
        let pns_lo = (0.0_f64)
            .max(p_y1_1 - p_y0_1)
            .max(p_y_1 - p_y0_1)
            .max(p_y1_1 - p_y_1);
        let pns_hi = p_y1_1
            .min(p_y0_0)
            .min(p_x1_y1 + p_x0_y0)
            .min(p_y1_1 - p_y0_1 + (p_x1_y1 - p_y0_1).max(0.0).min(p_y0_0));
        // PN bounds:
        let pn_lo = ((p_y_1 - p_y0_1).max(0.0)) / p_x1_y1;
        let pn_hi = ((p_y0_0 - p_x0_y0).min(p_x1_y1)) / p_x1_y1;
        // PS bounds:
        let ps_lo = ((p_y1_1 - p_y_1).max(0.0)) / p_x0_y0;
        let ps_hi = ((p_y1_1 - p_x1_y1).min(p_x0_y0)) / p_x0_y0;

        // --- Tension 6: Scheffer Critical Slowing Down, Epistemic Kelly, Wald Survivorship, Allostatic Wear ---
        let hc = HollingCanguilhemResilienceAudit::canonical_benchmark();
        let (lambda_buf, lambda_tight) = hc.engineering_resilience_buffered_vs_tight;
        let dt_obs = 0.50_f64;
        let rho1_buf = (-lambda_buf * dt_obs).exp();
        let rho1_tight = (-lambda_tight * dt_obs).exp();
        let var_ratio_tight_over_buf = lambda_buf / lambda_tight;

        // Epistemic Fractional Kelly (Samuelson 1971, 1979; Baker & McHale 2013):
        // Point estimate p_hat = 0.58, Manski lower bound p_L = 0.51, gain b = 1.0, loss a = 1.0, n_eff = 30, n_0 = 30
        let f_plugin = (0.58_f64 / 1.0) - (0.42_f64 / 1.0); // 0.16 (overbets under epistemic ambiguity!)
        let f_manski_lower = ((0.51_f64 / 1.0) - (0.49_f64 / 1.0)).max(0.0); // 0.02
        let f_epistemic = (30.0 / (30.0 + 30.0)) * f_manski_lower; // 0.01

        // Abraham Wald (1943) Bomber Survivorship Inversion:
        // Overall return rate P(S=1) = 0.80. Prior hit exposure: P(Hit_wing) = 0.50, P(Hit_engine) = 0.25.
        // Observed among returning planes (S=1): P(Hit_wing | S=1) = 0.58, P(Hit_engine | S=1) = 0.05.
        // By Bayes: P(S=1 | Hit_k) = P(S=1) * P(Hit_k | S=1) / P(Hit_k), so lethality P(S=0 | Hit_k) = 1 - P(S=1 | Hit_k):
        let p_return = 0.80_f64;
        let lethality_wing = 1.0 - (p_return * 0.58) / 0.50; // 0.072 (7.2% fatal)
        let lethality_engine = 1.0 - (p_return * 0.05) / 0.25; // 0.840 (84.0% fatal!)

        // Normative Survivorship Fallacy (Parasitic Externalization):
        // Strategy wins +18% (p=0.5) or loses -22% (p=0.5), but externalizes 60% of losses onto the commons.
        // Private loss = 0.40 * 0.22 = 0.088 -> private log growth = 0.5 * ln(1.18) + 0.5 * ln(0.912) > 0!
        // Systemic loss = 0.22 -> systemic log growth = 0.5 * ln(1.18) + 0.5 * ln(0.78) < 0!
        let g_private = 0.5 * (1.18_f64).ln() + 0.5 * (1.0 - 0.088_f64).ln();
        let g_systemic = 0.5 * (1.18_f64).ln() + 0.5 * (1.0 - 0.22_f64).ln();

        // Bruce McEwen (1998) Allostatic Cumulative Wear eroding carrying capacity K_t:
        // Initial K_0 = 10.0, A = 2.0, r = 1.0, h = 0.20 -> initial basin width = hc.ecological_basin_width_buffered_vs_tight.0 (~ 6.93 > shock 3.80)
        // After 12 periods of chronic overload (u_t = 0.96 > u_homeo = 0.75, gamma_wear = 6.0):
        let mut k_worn = 10.0_f64;
        for _ in 0..12 {
            let overload = (0.96_f64 - 0.75_f64).max(0.0);
            k_worn -= 6.0 * overload * overload;
        }
        let disc_worn = (k_worn - 2.0).powi(2) - 4.0 * 2.0 * k_worn * (0.20 / 1.0);
        let basin_worn = disc_worn.max(0.0).sqrt(); // ~ 3.52 < shock 3.80!
        let basin_init = hc.ecological_basin_width_buffered_vs_tight.0;

        let verified = reg_counter == MacKenziePerformativityRegime::CounterperformativeDestructive
            && reg_barnesian == MacKenziePerformativityRegime::BarnesianSelfFulfilling
            && reg_effective == MacKenziePerformativityRegime::EffectivePerformative
            && net_causal_advantage_proxy_rule < 0.0
            && net_causal_advantage_icp_rule > 0.0
            && tau_pilot > 0.20
            && tau_scale < 0.05
            && pn_lo <= pn_hi
            && pn_lo > 0.70
            && ps_lo <= ps_hi
            && pns_lo <= pns_hi
            && rho1_tight > rho1_buf
            && var_ratio_tight_over_buf > 1.80
            && f_epistemic < f_plugin * 0.15
            && lethality_engine > lethality_wing * 10.0
            && g_private > 0.0
            && g_systemic < 0.0
            && basin_worn < basin_init * 0.55;

        Self {
            tension5_mackenzie_regimes: (reg_counter, reg_barnesian, reg_effective),
            tension5_causal_effort_payoff_and_burden_ratio: (
                net_causal_advantage_proxy_rule,
                net_causal_advantage_icp_rule,
                social_burden_ratio,
            ),
            tension5_spillover_pilot_vs_scale_effect: (tau_pilot, tau_scale),
            tension5_tian_pearl_pn_ps_pns_bounds: (pn_lo, pn_hi, ps_lo, ps_hi, pns_lo, pns_hi),
            tension6_scheffer_lag1_autocorr_and_var_ratio: (
                rho1_buf,
                rho1_tight,
                var_ratio_tight_over_buf,
            ),
            tension6_plugin_vs_epistemic_kelly_fraction: (f_plugin, f_epistemic),
            tension6_wald_wing_vs_engine_lethality: (lethality_wing, lethality_engine),
            tension6_parasitic_private_vs_systemic_growth: (g_private, g_systemic),
            tension6_allostatic_basin_erosion: (basin_init, basin_worn),
            all_tensions_5_and_6_verified: verified,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn holling_ecological_resilience_and_canguilhem_normativity_separate_survival_from_local_stability()
     {
        let audit = HollingCanguilhemResilienceAudit::canonical_benchmark();
        assert!(audit.holling_canguilhem_resilience_verified);
        assert!(audit.engineering_resilience_buffered_vs_tight.0 > 0.0);
        assert!(audit.engineering_resilience_buffered_vs_tight.1 > 0.0);
        assert!(audit.canguilhem_tolerance_ratio_buffered_vs_tight.0 > 1.80);
        assert!(audit.canguilhem_tolerance_ratio_buffered_vs_tight.1 < 0.70);
        assert!(audit.post_shock_rk4_final_state_buffered_vs_tight.0 > 9.0);
        assert!(audit.post_shock_rk4_final_state_buffered_vs_tight.1 < 1e-4);
        assert!(audit.basin_discriminant_identity_error < 1e-12);
    }

    #[test]
    fn filter_three_tensions_5_and_6_mackenzie_tian_pearl_scheffer_wald_and_mcewen_hold() {
        let audit = FilterThreeTensionsFiveAndSixAudit::canonical_benchmark();
        assert!(audit.all_tensions_5_and_6_verified);
        assert_eq!(
            audit.tension5_mackenzie_regimes.0,
            MacKenziePerformativityRegime::CounterperformativeDestructive
        );
        assert_eq!(
            audit.tension5_mackenzie_regimes.1,
            MacKenziePerformativityRegime::BarnesianSelfFulfilling
        );
        assert_eq!(
            MacroCheck::passive(),
            MacKenziePerformativityRegime::CameraPassive
        );
        assert!(audit.tension5_tian_pearl_pn_ps_pns_bounds.0 > 0.70);
        assert!(
            audit.tension6_wald_wing_vs_engine_lethality.1
                > audit.tension6_wald_wing_vs_engine_lethality.0 * 10.0
        );
        assert!(audit.tension6_parasitic_private_vs_systemic_growth.0 > 0.0);
        assert!(audit.tension6_parasitic_private_vs_systemic_growth.1 < 0.0);
    }

    struct MacroCheck;
    impl MacroCheck {
        fn passive() -> MacKenziePerformativityRegime {
            MacKenziePerformativityRegime::classify(0.0, 0.0, 0.0, false)
        }
    }
}
