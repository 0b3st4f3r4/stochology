//! Bridge $L_1 \to L_2$ (Invisible Filter & Missingness): Meng's Fundamental Identity (`MengAudit`),
//! Non-parametric Manski & Lee (2009) Identification Bounds (`ManskiBounds`), and
//! Rubin's Multiple Imputation Combination Rules (`RubinMultipleImputationAudit`).

use crate::rng::mean;

/// Audit of Bridge $L_1 \to L_2$ (Invisible Filter) via Meng's Fundamental Identity (§1.1 and §1.3.1):
/// $$\bar{Y}_n - \bar{Y}_N = \underbrace{\rho_{R, Y}}_{\text{Selection Quality}} \times \underbrace{\sqrt{\frac{N - n}{n}}}_{\text{Data Quantity}} \times \underbrace{\sigma_Y}_{\text{Problem Difficulty}}$$
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MengAudit {
    /// Total population size $N$.
    pub population_n_total: usize,
    /// Recorded sample size $n$.
    pub sample_n: usize,
    /// Recording correlation $\rho_{R, Y}$ between recording indicator $R_i \in \{0, 1\}$ and outcome $Y_i$.
    pub rho_r_y: f64,
    /// Data quantity factor $\sqrt{(N - n) / n}$.
    pub data_quantity_factor: f64,
    /// Population standard deviation $\sigma_Y$.
    pub sigma_y: f64,
    /// Exact estimation error $\bar{Y}_n - \bar{Y}_N$.
    pub estimation_error: f64,
    /// Meng effective sample size equivalent to simple random sampling ($n_{\text{eff, Meng}}$).
    pub n_eff_meng: f64,
}

impl MengAudit {
    /// Computes Meng's decomposition from known parameters $(N, n, \rho_{R,Y}, \sigma_Y)$.
    pub fn from_parameters(
        population_n_total: usize,
        sample_n: usize,
        rho_r_y: f64,
        sigma_y: f64,
    ) -> Self {
        assert!(
            sample_n > 0 && sample_n <= population_n_total,
            "0 < n <= N required"
        );
        let n_pop = population_n_total as f64;
        let n_smp = sample_n as f64;
        let data_quantity_factor = ((n_pop - n_smp) / n_smp).sqrt();
        let estimation_error = rho_r_y * data_quantity_factor * sigma_y;
        let n_eff_meng = if rho_r_y.abs() < 1e-12 {
            n_smp
        } else {
            let mse_ratio = (n_pop - n_smp) * rho_r_y * rho_r_y;
            n_smp / (1.0 + mse_ratio)
        };
        Self {
            population_n_total,
            sample_n,
            rho_r_y,
            data_quantity_factor,
            sigma_y,
            estimation_error,
            n_eff_meng,
        }
    }

    /// Computes Meng's Identity directly from population vectors $R \in \{0, 1\}^N$ and $Y \in \mathbb{R}^N$.
    pub fn from_population(recorded: &[bool], y: &[f64]) -> Self {
        assert_eq!(recorded.len(), y.len());
        assert!(y.len() >= 2);
        let n_total = y.len();
        let n_obs = recorded.iter().filter(|&&r| r).count();
        assert!(n_obs > 0 && n_obs < n_total);

        let y_bar_pop = mean(y);
        let var_pop: f64 =
            y.iter().map(|&v| (v - y_bar_pop).powi(2)).sum::<f64>() / (n_total as f64);
        let sigma_y = var_pop.sqrt();

        let r_vec: Vec<f64> = recorded
            .iter()
            .map(|&r| if r { 1.0 } else { 0.0 })
            .collect();
        let r_bar = mean(&r_vec);
        let sigma_r = (r_bar * (1.0 - r_bar)).sqrt();

        let cov_ry: f64 = r_vec
            .iter()
            .zip(y.iter())
            .map(|(&ri, &yi)| (ri - r_bar) * (yi - y_bar_pop))
            .sum::<f64>()
            / (n_total as f64);
        let rho_r_y = if sigma_r * sigma_y > 0.0 {
            cov_ry / (sigma_r * sigma_y)
        } else {
            0.0
        };
        Self::from_parameters(n_total, n_obs, rho_r_y, sigma_y)
    }

    /// Data Defect Correlation ($\mathrm{ddc} = \rho_{R, Y}$, Meng 2018).
    #[inline]
    pub const fn data_defect_correlation_ddc(&self) -> f64 {
        self.rho_r_y
    }

    /// Data Defect Index ($\mathrm{DDI} = \rho_{R, Y}^2 = \mathbb{E}[\rho_{R, Y}^2]$, Meng 2018).
    #[inline]
    pub fn data_defect_index_ddi(&self) -> f64 {
        self.rho_r_y * self.rho_r_y
    }

    /// Law of Large Populations (LLP) standardized $Z$-score error magnification (Meng 2018, Theorem 1):
    /// $$Z_n = \frac{\bar{Y}_n - \bar{Y}_N}{\sqrt{(1 - f)\sigma_Y^2 / n}} = \sqrt{N - 1}\,\rho_{R, Y}$$
    #[inline]
    pub fn law_of_large_populations_z(&self) -> f64 {
        ((self.population_n_total.saturating_sub(1)) as f64).sqrt() * self.rho_r_y
    }

    /// Asymptotic SRS-equivalent effective sample size $n_{\mathrm{eff, SRS}}^* = \frac{f}{1 - f}\frac{1}{\mathrm{DDI}}$.
    #[inline]
    pub fn n_eff_srs_asymptotic(&self) -> f64 {
        let ddi = self.data_defect_index_ddi();
        let n_unobs = (self.population_n_total.saturating_sub(self.sample_n)) as f64;
        if ddi <= 1e-15 || n_unobs <= 0.0 {
            self.sample_n as f64
        } else {
            (self.sample_n as f64) / (n_unobs * ddi)
        }
    }

    /// Canonical $N = 10{,}000{,}000$, $n = 8{,}000{,}000$ ($80\%$ coverage) Big Data Paradox benchmark
    /// with $\rho_{R, Y} = 0.005$ ($\mathrm{DDI} = 2.5 \times 10^{-5}$), collapsing $8\times 10^6$ biased records to
    /// $n_{\mathrm{eff, SRS}}^* = 160{,}000$ ($1.6\%$ of the population) with $Z_{n,N} = \sqrt{N-1}\,\rho_{R,Y} \approx 15.81$.
    pub fn dpi_benchmark_10m_80pct() -> Self {
        Self::from_parameters(10_000_000, 8_000_000, 0.005, 0.50)
    }
}

/// Non-parametric Manski Identification Bounds under MNAR missingness (Invariant $\Phi_3$).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ManskiBounds {
    pub observed_rate: f64,
    pub missing_rate: f64,
    pub observed_mean: f64,
    pub lower_bound: f64,
    pub upper_bound: f64,
    pub ambiguity_width: f64,
}

impl ManskiBounds {
    /// Computes worst-case Manski bounds for an outcome $Y \in [y_{\min}, y_{\max}]$.
    pub fn compute(y_observed: &[f64], n_total: usize, y_min: f64, y_max: f64) -> Self {
        assert!(!y_observed.is_empty() && y_observed.len() <= n_total);
        assert!(y_min <= y_max);
        let p_obs = (y_observed.len() as f64) / (n_total as f64);
        let p_miss = 1.0 - p_obs;
        let obs_mean = mean(y_observed);
        let lower = p_obs * obs_mean + p_miss * y_min;
        let upper = p_obs * obs_mean + p_miss * y_max;
        Self {
            observed_rate: p_obs,
            missing_rate: p_miss,
            observed_mean: obs_mean,
            lower_bound: lower,
            upper_bound: upper,
            ambiguity_width: upper - lower,
        }
    }

    /// Covariate-stratified Manski bounds across $S$ strata `(y_observed_s, n_total_s)`, tightening the unstratified
    /// ambiguity interval whenever missingness or conditional means vary across observable strata $X = s$.
    pub fn compute_stratified(strata: &[(&[f64], usize)], y_min: f64, y_max: f64) -> Self {
        assert!(!strata.is_empty(), "requires at least 1 stratum");
        let mut total_n = 0usize;
        let mut total_obs = 0usize;
        let mut sum_obs_y = 0.0;
        let mut weighted_lower = 0.0;
        let mut weighted_upper = 0.0;

        for &(y_s, n_s) in strata {
            assert!(!y_s.is_empty() && y_s.len() <= n_s);
            let b_s = Self::compute(y_s, n_s, y_min, y_max);
            let w_s = n_s as f64;
            total_n += n_s;
            total_obs += y_s.len();
            sum_obs_y += y_s.iter().sum::<f64>();
            weighted_lower += w_s * b_s.lower_bound;
            weighted_upper += w_s * b_s.upper_bound;
        }
        let n_f = total_n as f64;
        let p_obs = (total_obs as f64) / n_f;
        let lower = weighted_lower / n_f;
        let upper = weighted_upper / n_f;
        Self {
            observed_rate: p_obs,
            missing_rate: 1.0 - p_obs,
            observed_mean: sum_obs_y / (total_obs as f64),
            lower_bound: lower,
            upper_bound: upper,
            ambiguity_width: upper - lower,
        }
    }

    /// Exact continuous fractional trimming helper over sorted observations `sorted`, keeping fraction `keep_frac`
    /// ($K = \text{keep\_frac} \cdot m = k_{\mathrm{int}} + \mathrm{rem}$) from the lower and upper tails without
    /// $O(1/m)$ integer rounding discontinuities.
    fn fractional_trimmed_means(sorted: &[f64], keep_frac: f64) -> (f64, f64) {
        let m = sorted.len();
        let k_exact = (keep_frac * (m as f64)).clamp(1e-9, m as f64);
        let k_int = (k_exact.floor() as usize).min(m);
        let rem = k_exact - (k_int as f64);

        let mut lower_sum: f64 = sorted[..k_int].iter().sum();
        let mut upper_sum: f64 = sorted[m - k_int..].iter().sum();
        if k_int < m && rem > 0.0 {
            lower_sum += rem * sorted[k_int];
            upper_sum += rem * sorted[m - 1 - k_int];
        }
        (lower_sum / k_exact, upper_sum / k_exact)
    }

    /// David Lee (2009) Sharp Trimming Bounds for the treatment effect among always-responders ($R(1)=1, R(0)=1$)
    /// under monotonic differential attrition ($R(1) \ge R(0)$ or $R(0) \ge R(1)$), using exact continuous
    /// fractional quantile trimming $K = (1 - q)m$.
    /// Returns `(lower_ate_bound, upper_ate_bound, trimming_fraction_q)`.
    pub fn lee_trimming_bounds(
        y_treated_obs: &[f64],
        n_treated_total: usize,
        y_control_obs: &[f64],
        n_control_total: usize,
    ) -> (f64, f64, f64) {
        assert!(!y_treated_obs.is_empty() && y_treated_obs.len() <= n_treated_total);
        assert!(!y_control_obs.is_empty() && y_control_obs.len() <= n_control_total);
        let p1 = (y_treated_obs.len() as f64) / (n_treated_total as f64);
        let p0 = (y_control_obs.len() as f64) / (n_control_total as f64);

        if (p1 - p0).abs() < 1e-12 {
            let diff = mean(y_treated_obs) - mean(y_control_obs);
            return (diff, diff, 0.0);
        }

        if p1 > p0 {
            let q = ((p1 - p0) / p1).clamp(0.0, 0.999);
            let mut sorted = y_treated_obs.to_vec();
            sorted.sort_by(|a, b| a.total_cmp(b));
            let (lower_mean, upper_mean) = Self::fractional_trimmed_means(&sorted, 1.0 - q);
            let c_mean = mean(y_control_obs);
            (lower_mean - c_mean, upper_mean - c_mean, q)
        } else {
            let q = ((p0 - p1) / p0).clamp(0.0, 0.999);
            let mut sorted = y_control_obs.to_vec();
            sorted.sort_by(|a, b| a.total_cmp(b));
            let (c_lower_mean, c_upper_mean) = Self::fractional_trimmed_means(&sorted, 1.0 - q);
            let t_mean = mean(y_treated_obs);
            (t_mean - c_upper_mean, t_mean - c_lower_mean, q)
        }
    }

    /// Sensitivity tipping-point shift $\delta_1^* = -\frac{\bar{Y}_1^{\mathrm{obs}} - \bar{Y}_0^{\mathrm{obs}}}{1 - p_1}$
    /// required in the unobserved treated outcomes ($\mathbb{E}[Y(1)\mid R(1)=0] = \bar{Y}_1^{\mathrm{obs}} + \delta_1$)
    /// to reduce the population ATE to zero when control outcomes are MAR ($\delta_0 = 0$).
    pub fn lee_tipping_point_delta(
        y_treated_obs: &[f64],
        n_treated_total: usize,
        y_control_obs: &[f64],
        n_control_total: usize,
    ) -> f64 {
        assert!(!y_treated_obs.is_empty() && y_treated_obs.len() <= n_treated_total);
        assert!(!y_control_obs.is_empty() && y_control_obs.len() <= n_control_total);
        let p1 = (y_treated_obs.len() as f64) / (n_treated_total as f64);
        let missing_1 = (1.0 - p1).max(1e-12);
        let diff_obs = mean(y_treated_obs) - mean(y_control_obs);
        -diff_obs / missing_1
    }
}

/// Donald B. Rubin's (1987, *Multiple Imputation for Nonresponse in Surveys*) combination rules
/// on Bridge $L_1 \to L_2$, contrasting proper between-imputation uncertainty $B_M$ against
/// naive single mean imputation variance shrinkage and non-parametric `ManskiBounds` under `MNAR`.
#[derive(Debug, Clone, PartialEq)]
pub struct RubinMultipleImputationAudit {
    /// Number of imputations $M \ge 2$.
    pub num_imputations_m: usize,
    /// Pooled point estimate $\bar{Q}_M = \frac{1}{M}\sum_{m=1}^M \hat{Q}_m$.
    pub pooled_estimate_q_bar: f64,
    /// Average within-imputation variance $\bar{U}_M = \frac{1}{M}\sum_{m=1}^M U_m$.
    pub within_variance_u_bar: f64,
    /// Between-imputation variance $B_M = \frac{1}{M-1}\sum_{m=1}^M (\hat{Q}_m - \bar{Q}_M)^2$.
    pub between_variance_b_m: f64,
    /// Total Rubin variance $T_M = \bar{U}_M + \left(1 + \frac{1}{M}\right)B_M$.
    pub total_variance_t_m: f64,
    /// Relative increase in variance due to nonresponse $r_M = \frac{(1 + M^{-1})B_M}{\bar{U}_M}$.
    pub relative_variance_increase_r_m: f64,
    /// Fraction of Missing Information $\gamma_{\mathrm{FMI}} = \frac{(1 + M^{-1})B_M}{T_M} \in [0, 1]$.
    pub fraction_missing_information_fmi: f64,
    /// Exact identity residual $\bigl|T_M - \bigl(\bar{U}_M + (1 + M^{-1})B_M\bigr)\bigr|$.
    pub variance_identity_error: f64,
}

impl RubinMultipleImputationAudit {
    /// Combines $M \ge 2$ completed-data estimates $\hat{Q}_m$ and their within-imputation sampling variances $U_m$.
    pub fn combine_imputations(estimates: &[f64], within_variances: &[f64]) -> Self {
        assert!(
            estimates.len() >= 2 && estimates.len() == within_variances.len(),
            "Rubin MI requires M >= 2 equal-length estimate and within-variance slices"
        );
        let m = estimates.len();
        let m_f = m as f64;
        let q_bar = mean(estimates);
        let u_bar = mean(within_variances).max(0.0);
        let b_m = estimates.iter().map(|&q| (q - q_bar).powi(2)).sum::<f64>() / ((m - 1) as f64);
        let inflation = (1.0 + 1.0 / m_f) * b_m;
        let t_m = u_bar + inflation;
        let r_m = if u_bar > 1e-15 {
            inflation / u_bar
        } else {
            0.0
        };
        let fmi = if t_m > 1e-15 {
            (inflation / t_m).clamp(0.0, 1.0)
        } else {
            0.0
        };
        let identity_err = (t_m - (u_bar + (1.0 + 1.0 / m_f) * b_m)).abs();
        Self {
            num_imputations_m: m,
            pooled_estimate_q_bar: q_bar,
            within_variance_u_bar: u_bar,
            between_variance_b_m: b_m,
            total_variance_t_m: t_m,
            relative_variance_increase_r_m: r_m,
            fraction_missing_information_fmi: fmi,
            variance_identity_error: identity_err,
        }
    }

    /// Quantifies the exact population variance amputation caused by naive single mean imputation:
    /// filling $N - n$ missing slots with $\bar{Y}_{\text{obs}}$ shrinks the population variance by
    /// $\frac{\sigma^2_{\text{mean-imp}}}{\sigma^2_{\text{obs}}} = \frac{n}{N} = 1 - \pi_{\text{miss}}$
    /// (with sample variance ratio $\frac{n - 1}{N - 1}$). Returns `(shrunk_pop_var, exact_ratio_n_over_N)`.
    pub fn single_mean_imputation_variance_shrinkage(
        observed: &[f64],
        total_n: usize,
    ) -> (f64, f64) {
        assert!(
            !observed.is_empty() && total_n >= observed.len(),
            "requires non-empty observed slice and total_n >= observed.len()"
        );
        let n_obs = observed.len() as f64;
        let n_tot = total_n as f64;
        let mu = mean(observed);
        let ss_obs: f64 = observed.iter().map(|&y| (y - mu).powi(2)).sum();
        let pop_var_obs = (ss_obs / n_obs).max(1e-15);
        // Imputing (total_n - observed.len()) values at exactly `mu` adds 0.0 to the sum of squares `ss_obs`!
        let pop_var_imp = ss_obs / n_tot;
        (pop_var_imp, pop_var_imp / pop_var_obs)
    }
}
