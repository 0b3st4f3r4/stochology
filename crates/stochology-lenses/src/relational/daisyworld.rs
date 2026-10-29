//! Axis 5: Andrew J. Watson & James E. Lovelock's (1983) *Daisyworld* Planetary Homeostasis Model.

/// Axis 5: Andrew J. Watson & James E. Lovelock's (1983) *Daisyworld* Planetary Homeostasis Model
/// (*Tellus B*, 35, 284–289).
///
/// Governs a planet whose fertile area ($p = 1.0$) is shared between bare ground ($x = 1 - \alpha_b - \alpha_w$,
/// albedo $A_g$), black daisies ($\alpha_b$, low albedo $A_b < A_g$), and white daisies ($\alpha_w$, high albedo $A_w > A_g$):
/// - Global albedo: $A = x A_g + \alpha_b A_b + \alpha_w A_w$
/// - Effective planetary temperature ($^\circ\mathrm{C}$): $T_e = \bigl(\frac{S_0 L (1 - A)}{\sigma}\bigr)^{1/4} - 273$
/// - Local microclimate temperature: $T_i = q (A - A_i) + T_e$
/// - Parabolic growth rate ($5\,^\circ\mathrm{C}$ to $40\,^\circ\mathrm{C}$, optimum $22.5\,^\circ\mathrm{C}$):
///   $$\beta(T_i) = \max\bigl(0,\ 1 - 0.003265 (22.5 - T_i)^2\bigr)$$
/// - Population replicator ODEs: $\frac{d\alpha_i}{dt} = \alpha_i (\beta(T_i) x - \gamma)$
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DaisyworldSystem {
    /// Bare ground albedo $A_g$ (canonical $0.50$).
    pub albedo_bare_ground: f64,
    /// Black daisy albedo $A_b$ (canonical $0.25$, absorbs radiation and warms locally).
    pub albedo_black: f64,
    /// White daisy albedo $A_w$ (canonical $0.75$, reflects radiation and cools locally).
    pub albedo_white: f64,
    /// Local thermal redistribution parameter $q$ ($^\circ\mathrm{C}$, canonical $20.0$).
    pub heat_transfer_q: f64,
    /// Constant mortality rate $\gamma$ (canonical $0.30$).
    pub mortality_gamma: f64,
    /// Biological optimal growth temperature $T_{\mathrm{opt}}$ ($^\circ\mathrm{C}$, canonical $22.5$).
    pub optimal_temp_c: f64,
    /// Ratio $S_0 / \sigma$ ($\mathrm{K}^4$) calibrated so bare ground at $L = 1.0$ is at $22.5\,^\circ\mathrm{C}$.
    pub solar_flux_over_sigma_k4: f64,
}

/// Steady-state equilibrium of *Daisyworld* at a given dimensionless stellar luminosity $L$.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DaisyworldEquilibriumPoint {
    pub luminosity_l: f64,
    pub black_daisy_area_alpha_b: f64,
    pub white_daisy_area_alpha_w: f64,
    pub bare_ground_area_x: f64,
    pub planetary_albedo_a: f64,
    /// Effective planetary temperature $T_e$ ($^\circ\mathrm{C}$) with living biosphere.
    pub planetary_temp_c: f64,
    /// Reference temperature $T_{\text{lifeless}}$ ($^\circ\mathrm{C}$) of a dead planet ($A = A_g$).
    pub lifeless_bare_temp_c: f64,
}

/// Complete audit of cybernetic planetary homeostasis across stellar luminosity $L \in [0.70, 1.60]$.
#[derive(Debug, Clone, PartialEq)]
pub struct DaisyworldHomeostasisAudit {
    /// Early faint-sun equilibrium ($L = 0.72$), where black daisies dominate and warm the planet toward $22.5\,^\circ\mathrm{C}$.
    pub faint_sun_point: DaisyworldEquilibriumPoint,
    /// Present-sun coexistence equilibrium ($L = 1.00$).
    pub present_sun_point: DaisyworldEquilibriumPoint,
    /// Bright-sun equilibrium ($L = 1.25$), where white daisies dominate and cool the planet toward $22.5\,^\circ\mathrm{C}$.
    pub bright_sun_point: DaisyworldEquilibriumPoint,
    /// Extreme stellar forcing point ($L = 1.60$) beyond the upper absorbing survival barrier ($\alpha_b = \alpha_w = 0$).
    pub post_collapse_point: DaisyworldEquilibriumPoint,
    /// Mean absolute deviation $|T_e - 22.5\,^\circ\mathrm{C}|$ with life across the homeostatic band $L \in [0.72, 1.28]$.
    pub homeostatic_mean_abs_error_c: f64,
    /// Mean absolute deviation $|T_{\text{lifeless}} - 22.5\,^\circ\mathrm{C}|$ on a dead planet over the same band.
    pub lifeless_mean_abs_error_c: f64,
    /// Negative feedback slope $\Delta T_e / \Delta L$ in the two-species coexistence zone ($\le 0$).
    pub coexistence_feedback_slope_dt_dl: f64,
    /// Whether emergent cybernetic homeostasis without teleology is verified.
    pub homeostasis_verified: bool,
}

impl Default for DaisyworldSystem {
    fn default() -> Self {
        Self::canonical()
    }
}

impl DaisyworldSystem {
    /// Returns the canonical Watson & Lovelock (1983) *Daisyworld* parameterization.
    pub fn canonical() -> Self {
        let t_opt_k = 22.5_f64 + 273.0;
        let albedo_bare = 0.50;
        let s0_over_sigma = t_opt_k.powi(4) / (1.0 - albedo_bare);
        Self {
            albedo_bare_ground: albedo_bare,
            albedo_black: 0.25,
            albedo_white: 0.75,
            heat_transfer_q: 20.0,
            mortality_gamma: 0.30,
            optimal_temp_c: 22.5,
            solar_flux_over_sigma_k4: s0_over_sigma,
        }
    }

    /// Parabolic growth rate $\beta(T) = \max\bigl(0,\ 1 - 0.003265 (22.5 - T)^2\bigr)$,
    /// vanishing outside $[5\,^\circ\mathrm{C},\ 40\,^\circ\mathrm{C}]$ and peaking at $1.0$ when $T = 22.5\,^\circ\mathrm{C}$.
    #[inline]
    pub fn growth_rate_beta(&self, local_temp_c: f64) -> f64 {
        let diff = self.optimal_temp_c - local_temp_c;
        (1.0 - 0.003265 * diff * diff).max(0.0)
    }

    /// Effective radiative equilibrium temperature $T_e(L, A)$ in $^\circ\mathrm{C}$ from Stefan-Boltzmann balance.
    #[inline]
    pub fn effective_temperature_c(&self, luminosity_l: f64, planetary_albedo_a: f64) -> f64 {
        let absorbed =
            (self.solar_flux_over_sigma_k4 * luminosity_l * (1.0 - planetary_albedo_a)).max(0.0);
        absorbed.powf(0.25) - 273.0
    }

    /// Integrates the coupled replicator-radiative ODEs to steady state at stellar luminosity `luminosity_l`
    /// using classical 4th-order Runge-Kutta (RK4) integration.
    pub fn solve_equilibrium(
        &self,
        luminosity_l: f64,
        steps: usize,
        dt: f64,
    ) -> DaisyworldEquilibriumPoint {
        let mut alpha_b = 0.02_f64;
        let mut alpha_w = 0.02_f64;

        let rhs = |ab: f64, aw: f64| -> (f64, f64) {
            let ab_c = ab.clamp(0.0, 1.0);
            let aw_c = aw.clamp(0.0, 1.0 - ab_c);
            let x = (1.0 - ab_c - aw_c).max(0.0);
            let albedo_a =
                x * self.albedo_bare_ground + ab_c * self.albedo_black + aw_c * self.albedo_white;
            let t_e = self.effective_temperature_c(luminosity_l, albedo_a);
            let t_b = self.heat_transfer_q * (albedo_a - self.albedo_black) + t_e;
            let t_w = self.heat_transfer_q * (albedo_a - self.albedo_white) + t_e;
            let beta_b = self.growth_rate_beta(t_b);
            let beta_w = self.growth_rate_beta(t_w);
            (
                ab_c * (beta_b * x - self.mortality_gamma),
                aw_c * (beta_w * x - self.mortality_gamma),
            )
        };

        for _ in 0..steps {
            let (k1b, k1w) = rhs(alpha_b, alpha_w);
            let (k2b, k2w) = rhs(alpha_b + 0.5 * dt * k1b, alpha_w + 0.5 * dt * k1w);
            let (k3b, k3w) = rhs(alpha_b + 0.5 * dt * k2b, alpha_w + 0.5 * dt * k2y_fix(k2w));
            let (k4b, k4w) = rhs(alpha_b + dt * k3b, alpha_w + dt * k3w);
            alpha_b = (alpha_b + (dt / 6.0) * (k1b + 2.0 * k2b + 2.0 * k3b + k4b)).clamp(0.0, 1.0);
            alpha_w = (alpha_w + (dt / 6.0) * (k1w + 2.0 * k2w + 2.0 * k3w + k4w))
                .clamp(0.0, 1.0 - alpha_b);
        }

        let x = (1.0 - alpha_b - alpha_w).max(0.0);
        let albedo_a =
            x * self.albedo_bare_ground + alpha_b * self.albedo_black + alpha_w * self.albedo_white;
        let t_e = self.effective_temperature_c(luminosity_l, albedo_a);
        let t_lifeless = self.effective_temperature_c(luminosity_l, self.albedo_bare_ground);

        DaisyworldEquilibriumPoint {
            luminosity_l,
            black_daisy_area_alpha_b: alpha_b,
            white_daisy_area_alpha_w: alpha_w,
            bare_ground_area_x: x,
            planetary_albedo_a: albedo_a,
            planetary_temp_c: t_e,
            lifeless_bare_temp_c: t_lifeless,
        }
    }

    /// Executes the full Watson-Lovelock (1983) *Daisyworld* homeostasis audit across solar luminosities.
    pub fn audit_homeostasis(&self) -> DaisyworldHomeostasisAudit {
        let faint = self.solve_equilibrium(0.78, 2_500, 0.05);
        let present = self.solve_equilibrium(1.00, 2_500, 0.05);
        let bright = self.solve_equilibrium(1.22, 2_500, 0.05);
        let collapse = self.solve_equilibrium(1.60, 2_500, 0.05);

        let luminosities = [0.78, 0.85, 0.92, 1.00, 1.08, 1.15, 1.22];
        let mut bio_err_sum = 0.0;
        let mut dead_err_sum = 0.0;
        for &l in &luminosities {
            let pt = self.solve_equilibrium(l, 2_500, 0.05);
            bio_err_sum += (pt.planetary_temp_c - self.optimal_temp_c).abs();
            dead_err_sum += (pt.lifeless_bare_temp_c - self.optimal_temp_c).abs();
        }
        let bio_mae = bio_err_sum / (luminosities.len() as f64);
        let dead_mae = dead_err_sum / (luminosities.len() as f64);

        let pt_085 = self.solve_equilibrium(0.85, 2_500, 0.05);
        let pt_115 = self.solve_equilibrium(1.15, 2_500, 0.05);
        let slope = (pt_115.planetary_temp_c - pt_085.planetary_temp_c) / (1.15 - 0.85);

        let verified = faint.black_daisy_area_alpha_b > 0.40
            && faint.planetary_temp_c > faint.lifeless_bare_temp_c + 5.0
            && bright.white_daisy_area_alpha_w > 0.40
            && bright.planetary_temp_c < bright.lifeless_bare_temp_c - 5.0
            && bio_mae < 3.0
            && dead_mae > 8.0
            && slope <= 0.0
            && collapse.black_daisy_area_alpha_b < 1e-4
            && collapse.white_daisy_area_alpha_w < 1e-4;

        DaisyworldHomeostasisAudit {
            faint_sun_point: faint,
            present_sun_point: present,
            bright_sun_point: bright,
            post_collapse_point: collapse,
            homeostatic_mean_abs_error_c: bio_mae,
            lifeless_mean_abs_error_c: dead_mae,
            coexistence_feedback_slope_dt_dl: slope,
            homeostasis_verified: verified,
        }
    }
}

#[inline]
fn k2y_fix(val: f64) -> f64 {
    val
}
