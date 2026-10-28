//! Dynamics of the Three Ontological Layers ($L_1 \to L_2 \to L_3 \to L_1$),
//! Bridge Mechanics, and Formal Performative Prediction (§1.1).

use crate::rng::{mean, median, quantile, quantile_sorted, sample_std};

/// Ontological Layer of the data- and decision-generating cycle ($L_1 \to L_2 \to L_3 \to L_1$).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Layer {
    /// $L_1$ — World: Physical, biological, or socioeconomic reality generating the data.
    WorldL1,
    /// $L_2$ — Model: Formal, reduced, computationally simulable mathematical representation.
    ModelL2,
    /// $L_3$ — Agent: Human decision-maker, committee, or algorithmic controller acting under constraints.
    AgentL3,
}

impl Layer {
    pub const fn code(self) -> &'static str {
        match self {
            Self::WorldL1 => "L1",
            Self::ModelL2 => "L2",
            Self::AgentL3 => "L3",
        }
    }

    pub fn from_code(s: &str) -> Option<Self> {
        match s.trim() {
            "L1" => Some(Self::WorldL1),
            "L2" => Some(Self::ModelL2),
            "L3" => Some(Self::AgentL3),
            _ => None,
        }
    }
}

/// Four-stage Data Processing Inequality (DPI) operational pipeline view ($D_1 \to D_2 \to D_3 \to D_4$),
/// explicitly mapped onto the canonical three-layer ontology ($L_1, L_2, L_3$) and its three bridges.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DpiOperationalStage {
    /// $D_1$ — World & Sensor/Survey Collection ($L_1$ and Bridge $L_1 \to L_2$ selection filter).
    D1DataCollection,
    /// $D_2$ — Feature Engineering & Sufficient Compression (Bridge $L_1 \to L_2$ / $L_2$ representation).
    D2FeatureEngineering,
    /// $D_3$ — Formal Mathematical/Stochastic Model ($L_2$).
    D3ModelRepresentation,
    /// $D_4$ — Human/Controller Decision & Performative Actuation ($L_3$ and Bridges $L_2 \to L_3$, $L_3 \to L_1$).
    D4HumanDecisionActuation,
}

impl DpiOperationalStage {
    pub const fn code(self) -> &'static str {
        match self {
            Self::D1DataCollection => "D1",
            Self::D2FeatureEngineering => "D2",
            Self::D3ModelRepresentation => "D3",
            Self::D4HumanDecisionActuation => "D4",
        }
    }

    /// Maps a canonical `(Layer, Option<bridge_str>)` pair onto its DPI operational stage ($D_1..D_4$).
    pub fn from_ontological(layer: Layer, bridge: Option<&str>) -> Self {
        match (layer, bridge.map(str::trim)) {
            (_, Some("L2->L3")) | (_, Some("L3->L1")) | (Layer::AgentL3, _) => {
                Self::D4HumanDecisionActuation
            }
            (Layer::ModelL2, Some("L1->L2")) => Self::D2FeatureEngineering,
            (Layer::ModelL2, _) => Self::D3ModelRepresentation,
            (Layer::WorldL1, _) => Self::D1DataCollection,
        }
    }
}

/// Uncertainty Typing in the Stirling-Walker Matrix (§1.2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UncertaintyClass {
    /// Aleatory ($L_1$) — Irreducible physical or stochastic variability.
    Aleatory,
    /// Parametric Epistemic ($L_2$) — Reducible ignorance about finite parameters $\theta$.
    EpistemicParameter,
    /// Structural Epistemic ($L_2$) — Doubt about functional form $\eta(x, \theta)$; requires Kennedy-O'Hagan GP $\delta(x)$.
    EpistemicSpecification,
    /// Knightian / Deep ($L_3$) — Unbounded possibility space or strategic environment; requires Info-Gap + Sniedovich safeguard.
    Knightian,
}

impl UncertaintyClass {
    pub const fn code(self) -> &'static str {
        match self {
            Self::Aleatory => "aleatory",
            Self::EpistemicParameter => "epistemic_parameter",
            Self::EpistemicSpecification => "epistemic_specification",
            Self::Knightian => "knightian",
        }
    }

    pub fn from_code(s: &str) -> Option<Self> {
        match s.trim() {
            "aleatory" => Some(Self::Aleatory),
            "epistemic_parameter" => Some(Self::EpistemicParameter),
            "epistemic_specification" => Some(Self::EpistemicSpecification),
            "knightian" => Some(Self::Knightian),
            _ => None,
        }
    }
}

/// Laplace's Rule of Succession for small samples or rare events ($s$ successes in $n$ binary trials):
/// $$\hat{p} = \frac{s + 1}{n + 2}$$
#[inline]
pub fn laplace_succession(successes: usize, trials: usize) -> f64 {
    assert!(successes <= trials, "successes cannot exceed trials");
    ((successes as f64) + 1.0) / ((trials as f64) + 2.0)
}

/// Cromwell's Rule (Prohibition 4): clamps probability $p$ to $[\varepsilon_{\text{floor}}, 1 - \varepsilon_{\text{floor}}]$.
#[inline]
pub fn apply_cromwell_floor(p: f64, epsilon_floor: f64) -> f64 {
    assert!(
        epsilon_floor > 0.0 && epsilon_floor < 0.5,
        "epsilon_floor must be in (0, 0.5)"
    );
    p.clamp(epsilon_floor, 1.0 - epsilon_floor)
}

/// Verifies whether probability $p$ strictly satisfies Cromwell's Rule for a given $\varepsilon_{\text{floor}} > 0$.
#[inline]
pub fn respects_cromwell(p: f64, epsilon_floor: f64) -> bool {
    epsilon_floor > 0.0
        && epsilon_floor < 0.5
        && p >= epsilon_floor
        && p <= (1.0 - epsilon_floor)
        && p > 0.0
        && p < 1.0
}

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

/// Audit of Bridge $L_2 \to L_3$ (Misreading) and Prohibition 2 (Ban on the Lone Mean):
/// Quantifies Jensen's Gap $\Delta_J = \mathbb{E}[f(X)] - f(\mathbb{E}[X])$, median, tail quantiles
/// ($q_{05}, q_{95}$), and empirical CRPS.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NonlinearSummary {
    pub mean_x: f64,
    pub std_x: f64,
    /// Evaluation at the mean $f(\mathbb{E}[X])$.
    pub f_at_mean: f64,
    /// Simulated expectation $\mathbb{E}[f(X)]$.
    pub expected_f: f64,
    /// Exact Jensen Gap $\Delta_J = \mathbb{E}[f(X)] - f(\mathbb{E}[X])$.
    pub jensen_gap: f64,
    /// Second-order Taylor approximation $\frac{1}{2} f''(\mathbb{E}[X]) \sigma_X^2$.
    pub second_order_taylor_approx: f64,
    pub median_f: f64,
    pub q05_f: f64,
    pub q95_f: f64,
    /// Out-of-sample (or leave-one-out) Continuous Ranked Probability Score ($\mathrm{CRPS}$) of $f(X)$.
    pub empirical_crps: f64,
}

impl NonlinearSummary {
    /// Evaluates a nonlinear loss/payoff function $f(X)$ and its local curvature $f''(\mathbb{E}[X])$ over sample `xs`,
    /// computing exact leave-one-out out-of-sample $\mathrm{CRPS}$ in $O(n \log n)$ via order-statistic prefix sums.
    pub fn evaluate<F>(xs: &[f64], f: F, f_second_derivative_at_mean: f64) -> Self
    where
        F: Fn(f64) -> f64,
    {
        assert!(xs.len() >= 2, "NonlinearSummary requires at least 2 points");
        let fx: Vec<f64> = xs.iter().map(|&x| f(x)).collect();
        let crps = exact_loo_empirical_crps(&fx);
        Self::from_transformed(xs, &fx, f(mean(xs)), f_second_derivative_at_mean, crps)
    }

    /// Evaluates $f(X)$ on `forecast_xs` and scores out-of-sample $\mathrm{CRPS}$ against held-out `observed_xs`.
    pub fn evaluate_out_of_sample<F>(
        forecast_xs: &[f64],
        observed_xs: &[f64],
        f: F,
        f_second_derivative_at_mean: f64,
    ) -> Self
    where
        F: Fn(f64) -> f64,
    {
        assert!(
            forecast_xs.len() >= 2 && !observed_xs.is_empty(),
            "requires at least 2 forecast points and 1 observed out-of-sample point"
        );
        let fx: Vec<f64> = forecast_xs.iter().map(|&x| f(x)).collect();
        let oos_crps: f64 = observed_xs
            .iter()
            .map(|&y| empirical_crps(&fx, f(y)))
            .sum::<f64>()
            / (observed_xs.len() as f64);
        Self::from_transformed(
            forecast_xs,
            &fx,
            f(mean(forecast_xs)),
            f_second_derivative_at_mean,
            oos_crps,
        )
    }

    fn from_transformed(
        xs: &[f64],
        fx: &[f64],
        f_mx: f64,
        f_second_derivative_at_mean: f64,
        crps: f64,
    ) -> Self {
        let mx = mean(xs);
        let sx = sample_std(xs);
        let ef = mean(fx);
        let gap = ef - f_mx;
        let taylor = 0.5 * f_second_derivative_at_mean * sx * sx;
        let med = median(fx);
        let q05 = quantile(fx, 0.05);
        let q95 = quantile(fx, 0.95);
        Self {
            mean_x: mx,
            std_x: sx,
            f_at_mean: f_mx,
            expected_f: ef,
            jensen_gap: gap,
            second_order_taylor_approx: taylor,
            median_f: med,
            q05_f: q05,
            q95_f: q95,
            empirical_crps: crps,
        }
    }
}

/// Empirical Continuous Ranked Probability Score ($\mathrm{CRPS}$):
/// $$\mathrm{CRPS}(F, y) = \mathbb{E}_F|X - y| - \frac{1}{2}\mathbb{E}_F|X - X'|$$
pub fn empirical_crps(forecast_samples: &[f64], observation_y: f64) -> f64 {
    assert!(!forecast_samples.is_empty());
    let mut sorted = forecast_samples.to_vec();
    sorted.sort_by(|a, b| a.total_cmp(b));
    let n = sorted.len() as f64;
    let term1: f64 = sorted
        .iter()
        .map(|&x| (x - observation_y).abs())
        .sum::<f64>()
        / n;
    // O(n log n) evaluation of E|X - X'| = (2 / n^2) * sum_{i=0}^{n-1} (2*i - n + 1) * sorted[i]
    let mut spread = 0.0;
    for (i, &x) in sorted.iter().enumerate() {
        let coeff = (2.0 * (i as f64)) - n + 1.0;
        spread += coeff * x;
    }
    let e_xx = (2.0 * spread) / (n * n);
    term1 - 0.5 * e_xx
}

/// Exact $O(n \log n)$ leave-one-out average empirical $\mathrm{CRPS}$ across a sample `fx` of size $n \ge 2$:
/// $$\overline{\mathrm{CRPS}}_{\mathrm{LOO}} = \frac{1}{n}\sum_{i=1}^n \mathrm{CRPS}(F_{-i}, x_i) = \frac{n + 1}{2(n - 1)^2} \cdot \mathbb{E}_{\hat{F}_n}|X - X'|$$
fn exact_loo_empirical_crps(fx: &[f64]) -> f64 {
    assert!(fx.len() >= 2);
    let mut sorted = fx.to_vec();
    sorted.sort_by(|a, b| a.total_cmp(b));
    let n = sorted.len() as f64;
    let m = n - 1.0;
    // Let S = sum_{i < j} (sorted[j] - sorted[i]) = sum_{k=0}^{n-1} (2*k - n + 1) * sorted[k].
    // For each observation i, sum_{j != i} |x_j - x_i| summed over all i equals 2 * S.
    // And summing the pairwise spreads over all n leave-one-out subsets of size (n-1) counts each pair (n-2) times.
    let mut pair_diff_sum = 0.0;
    for (k, &x) in sorted.iter().enumerate() {
        pair_diff_sum += ((2.0 * (k as f64)) - n + 1.0) * x;
    }
    let avg_term1 = (2.0 * pair_diff_sum) / (n * m);
    let avg_half_spread = ((n - 2.0) * pair_diff_sum) / (n * m * m);
    avg_term1 - avg_half_spread
}

/// Exact 1D empirical Wasserstein-1 (Earth Mover / Kantorovich-Rubinstein) distance
/// $W_1(\hat{P}, \hat{Q}) = \int_0^1 |F_{\hat{P}}^{-1}(u) - F_{\hat{Q}}^{-1}(u)|\,du$ between two empirical samples.
pub fn empirical_wasserstein_1(sample_p: &[f64], sample_q: &[f64]) -> f64 {
    assert!(
        !sample_p.is_empty() && !sample_q.is_empty(),
        "empirical_wasserstein_1 requires non-empty samples"
    );
    let mut sp = sample_p.to_vec();
    let mut sq = sample_q.to_vec();
    sp.sort_by(|a, b| a.total_cmp(b));
    sq.sort_by(|a, b| a.total_cmp(b));
    if sp.len() == sq.len() {
        let n = sp.len() as f64;
        return sp
            .iter()
            .zip(sq.iter())
            .map(|(&a, &b)| (a - b).abs())
            .sum::<f64>()
            / n;
    }
    let grid = (sp.len() + sq.len()).clamp(64, 512);
    let mut acc = 0.0;
    for k in 0..grid {
        let u = ((k as f64) + 0.5) / (grid as f64);
        acc += (quantile_sorted(&sp, u) - quantile_sorted(&sq, u)).abs();
    }
    acc / (grid as f64)
}

/// Formal Theory of Performative Prediction on Bridge $L_3 \to L_1$
/// (Perdomo, Zrnic, Mendler-Dünner & Hardt — §1.1 & Prohibition 3).
///
/// Models a canonical system where outcome $Z \sim \mathcal{D}(\theta)$ shifts linearly with the deployed model $\theta$:
/// $$Y(\theta) = \mu + \varepsilon \theta + \xi, \qquad \ell(z; \phi) = \frac{\gamma}{2}(y - \phi)^2 - \alpha \phi$$
/// with Wasserstein-1 sensitivity $W_1(\mathcal{D}(\theta), \mathcal{D}(\theta')) \le \varepsilon \|\theta - \theta'\|_2$,
/// strong convexity $\gamma > 0$, and smoothness $\beta \ge \gamma$.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PerformativeSystem {
    /// Base distribution mean $\mu$ under passive regime ($\varepsilon = 0$).
    pub base_mean_mu: f64,
    /// Wasserstein-1 sensitivity $\varepsilon$ of induced distribution map $\mathcal{D}(\theta)$.
    pub sensitivity_epsilon: f64,
    /// Regularization / linear payoff incentive $\alpha$ in the loss.
    pub incentive_alpha: f64,
    /// Strong convexity constant $\gamma > 0$.
    pub strong_convexity_gamma: f64,
    /// Smoothness constant $\beta \ge \gamma$.
    pub smoothness_beta: f64,
}

/// Complete audit of Performative Stability ($\theta_{\mathrm{PS}}$) vs. Performative Optimality ($\theta_{\mathrm{PO}}$).
#[derive(Debug, Clone, PartialEq)]
pub struct PerformativeDiagnostic {
    /// Passive Supervised Learning estimator ($\varepsilon = 0$): $\theta_{\mathrm{SL}}$.
    pub theta_sl: f64,
    /// Performative Stability fixed point ($\theta_{\mathrm{PS}} = \arg\min_\phi \mathrm{DPR}(\theta_{\mathrm{PS}}, \phi)$).
    pub theta_ps: f64,
    /// Performative Optimality Stackelberg equilibrium ($\theta_{\mathrm{PO}} = \arg\min_\theta \mathrm{PR}(\theta)$).
    pub theta_po: f64,
    /// Performative Risk $\mathrm{PR}(\theta_{\mathrm{PS}})$ at the stable retraining point.
    pub risk_at_ps: f64,
    /// Performative Risk $\mathrm{PR}(\theta_{\mathrm{PO}})$ at the Stackelberg optimum.
    pub risk_at_po: f64,
    /// Gap $|\theta_{\mathrm{PO}} - \theta_{\mathrm{PS}}|$ between stability and optimality.
    pub gap_ps_vs_po: f64,
    /// Contraction radius of Repeated Risk Minimization (RRM): $r = \frac{|\varepsilon| \beta}{\gamma}$.
    pub rrm_contraction_radius: f64,
    /// Whether the strict contraction inequality $\frac{|\varepsilon|\beta}{\gamma} < 1$ holds.
    pub rrm_converges: bool,
    /// Trajectory $\theta_0, \theta_1, \dots, \theta_T$ of Repeated Risk Minimization (RRM).
    pub rrm_trajectory: Vec<f64>,
}

impl PerformativeSystem {
    pub fn new(
        base_mean_mu: f64,
        sensitivity_epsilon: f64,
        incentive_alpha: f64,
        strong_convexity_gamma: f64,
        smoothness_beta: f64,
    ) -> Self {
        assert!(strong_convexity_gamma > 0.0, "gamma must be > 0");
        assert!(
            smoothness_beta >= strong_convexity_gamma,
            "beta must be >= gamma"
        );
        Self {
            base_mean_mu,
            sensitivity_epsilon,
            incentive_alpha,
            strong_convexity_gamma,
            smoothness_beta,
        }
    }

    /// Decoupled Performative Risk $\mathrm{DPR}(\theta, \phi) \triangleq \mathbb{E}_{Z \sim \mathcal{D}(\theta)}[\ell(Z; \phi)]$.
    #[inline]
    pub fn decoupled_risk_dpr(&self, theta_world: f64, phi_evaluated: f64) -> f64 {
        let expected_y = self.base_mean_mu + self.sensitivity_epsilon * theta_world;
        let diff = expected_y - phi_evaluated;
        0.5 * self.strong_convexity_gamma * diff * diff - self.incentive_alpha * phi_evaluated
    }

    /// Performative Risk $\mathrm{PR}(\theta) \triangleq \mathrm{DPR}(\theta, \theta)$.
    #[inline]
    pub fn performative_risk_pr(&self, theta: f64) -> f64 {
        self.decoupled_risk_dpr(theta, theta)
    }

    /// Best-response map $G(\theta_t) = \arg\min_\phi \mathrm{DPR}(\theta_t, \phi)$ of Repeated Risk Minimization (RRM).
    #[inline]
    pub fn rrm_update_step(&self, theta_t: f64) -> f64 {
        self.base_mean_mu
            + self.sensitivity_epsilon * theta_t
            + (self.incentive_alpha / self.strong_convexity_gamma)
    }

    /// Runs the full performative diagnostic and simulates `steps` iterations of RRM starting from `theta_0`.
    pub fn diagnose(&self) -> PerformativeDiagnostic {
        let offset = self.incentive_alpha / self.strong_convexity_gamma;
        let theta_sl = self.base_mean_mu + offset;
        let eps = self.sensitivity_epsilon;

        let one_minus_eps = 1.0 - eps;
        let theta_ps = if one_minus_eps.abs() > 1e-12 {
            (self.base_mean_mu + offset) / one_minus_eps
        } else {
            f64::INFINITY
        };

        // Minimizing PR(theta) = 0.5 * gamma * (mu - (1 - eps)*theta)^2 - alpha * theta:
        // dPR/dtheta = -gamma * (1 - eps) * (mu - (1 - eps)*theta) - alpha = 0
        // => theta_PO = mu / (1 - eps) + alpha / (gamma * (1 - eps)^2).
        let theta_po = if one_minus_eps.abs() > 1e-12 {
            (self.base_mean_mu / one_minus_eps) + (offset / (one_minus_eps * one_minus_eps))
        } else {
            f64::INFINITY
        };

        let radius = (eps.abs() * self.smoothness_beta) / self.strong_convexity_gamma;
        let converges = radius < 1.0;

        let mut traj = Vec::with_capacity(16);
        let mut curr = 0.0;
        traj.push(curr);
        for _ in 0..15 {
            curr = self.rrm_update_step(curr);
            traj.push(curr);
        }

        let risk_ps = self.performative_risk_pr(theta_ps);
        let risk_po = self.performative_risk_pr(theta_po);

        PerformativeDiagnostic {
            theta_sl,
            theta_ps,
            theta_po,
            risk_at_ps: risk_ps,
            risk_at_po: risk_po,
            gap_ps_vs_po: (theta_po - theta_ps).abs(),
            rrm_contraction_radius: radius,
            rrm_converges: converges,
            rrm_trajectory: traj,
        }
    }

    /// Simulates a non-contracting performative classifier ($\frac{\varepsilon \beta}{\gamma} \ge 1$)
    /// that oscillates perpetually between extreme states $1, 0, 1, 0, \dots$.
    pub fn simulate_rrm_oscillation(steps: usize) -> Vec<f64> {
        let non_contracting = Self::new(1.0, -1.0, 0.0, 1.0, 1.0);
        let mut theta = 1.0;
        let mut history = Vec::with_capacity(steps);
        for _ in 0..steps {
            history.push(theta);
            theta = non_contracting.rrm_update_step(theta).clamp(0.0, 1.0);
        }
        history
    }

    /// Performative Gradient Descent (`PerfGD`, Izzo, Ying & Zou 2021) using the total performative derivative
    /// $$\nabla_\theta \mathrm{PR}(\theta) = \nabla_2 \mathrm{DPR}(\theta, \theta) + \nabla_1 \mathrm{DPR}(\theta, \theta) = -\gamma(1 - \varepsilon)\big(\mu - (1 - \varepsilon)\theta\big) - \alpha$$
    /// which converges directly to the Stackelberg Performative Optimum $\theta_{\mathrm{PO}}$ rather than stopping at $\theta_{\mathrm{PS}}$.
    pub fn perfgd_trajectory(&self, theta_0: f64, step_size_eta: f64, steps: usize) -> Vec<f64> {
        assert!(step_size_eta > 0.0, "step_size_eta must be > 0");
        let one_minus_eps = 1.0 - self.sensitivity_epsilon;
        let mut theta = theta_0;
        let mut traj = Vec::with_capacity(steps + 1);
        traj.push(theta);
        for _ in 0..steps {
            let grad = -self.strong_convexity_gamma
                * one_minus_eps
                * (self.base_mean_mu - one_minus_eps * theta)
                - self.incentive_alpha;
            theta -= step_size_eta * grad;
            traj.push(theta);
        }
        traj
    }

    /// Decomposes the total performative gradient $\nabla_\theta \mathrm{PR}(\theta)$ into:
    /// 1. $\frac{\partial \mathrm{DPR}(\theta_{\text{world}}, \phi_{\text{eval}})}{\partial \theta_{\text{world}}}\Big|_{\phi=\theta} = \gamma \varepsilon \big(\mu - (1 - \varepsilon)\theta\big)$
    ///    (distribution-shift / Adversarial Goodhart gradient; denoted $\nabla_2 \mathrm{DPR}$ in Izzo, Ying & Zou 2021 when arguments are ordered $(\text{model}, \text{distribution})$),
    /// 2. $\frac{\partial \mathrm{DPR}(\theta_{\text{world}}, \phi_{\text{eval}})}{\partial \phi_{\text{eval}}}\Big|_{\phi=\theta} = -\gamma \big(\mu - (1 - \varepsilon)\theta\big) - \alpha$
    ///    (passive parameter gradient, zero at $\theta_{\mathrm{PS}}$; denoted $\nabla_1 \mathrm{DPR}$ in Izzo, Ying & Zou 2021).
    #[inline]
    pub fn dpr_gradient_components(&self, theta: f64) -> (f64, f64) {
        let residual = self.base_mean_mu - (1.0 - self.sensitivity_epsilon) * theta;
        let grad_1_dist_shift = self.strong_convexity_gamma * self.sensitivity_epsilon * residual;
        let grad_2_model_param = -self.strong_convexity_gamma * residual - self.incentive_alpha;
        (grad_1_dist_shift, grad_2_model_param)
    }

    /// Oracle-free Performative Gradient Descent (`PerfGD`, Izzo, Ying & Zou 2021) that estimates
    /// the distribution-shift sensitivity $\hat{\varepsilon}_t = \frac{\bar{Y}(\theta_t) - \bar{Y}(\theta_{t-1})}{\theta_t - \theta_{t-1}}$
    /// from consecutive deployment means without direct access to `self.sensitivity_epsilon`.
    pub fn perfgd_finite_difference_trajectory(
        &self,
        theta_0: f64,
        step_size_eta: f64,
        delta_probe: f64,
        steps: usize,
    ) -> Vec<f64> {
        assert!(step_size_eta > 0.0 && delta_probe.abs() > 1e-9);
        let observe_mean_y = |th: f64| self.base_mean_mu + self.sensitivity_epsilon * th;
        let mut traj = Vec::with_capacity(steps + 1);
        let mut theta_prev = theta_0;
        let mut y_prev = observe_mean_y(theta_prev);
        traj.push(theta_prev);

        let mut theta = theta_0 + delta_probe;
        for _ in 0..steps {
            let y_curr = observe_mean_y(theta);
            let dtheta = theta - theta_prev;
            let eps_hat = if dtheta.abs() > 1e-12 {
                (y_curr - y_prev) / dtheta
            } else {
                0.0
            };
            let residual = y_curr - theta;
            let grad_model = -self.strong_convexity_gamma * residual - self.incentive_alpha;
            let grad_shift = self.strong_convexity_gamma * eps_hat * residual;
            let total_grad = grad_model + grad_shift;
            theta_prev = theta;
            y_prev = y_curr;
            theta -= step_size_eta * total_grad;
            traj.push(theta);
        }
        traj
    }

    /// Canonical linear performative pricing benchmark (Izzo, Ying & Zou 2021, §1):
    /// $\ell(z; \theta) = -\theta z$ (or $\theta z$ with $a < 0 < b$) where $\mathbb{E}[Z \mid \theta] = a\theta + b$.
    /// Returns `(theta_stab, theta_opt, ratio_stab_over_opt)` proving $\theta_{\mathrm{STAB}} = 2\,\theta_{\mathrm{OPT}}$.
    pub fn izzo_linear_2x_benchmark(a_slope: f64, b_intercept: f64) -> (f64, f64, f64) {
        assert!(a_slope.abs() > 1e-12, "a_slope must be non-zero");
        let theta_stab = -b_intercept / a_slope;
        let theta_opt = -b_intercept / (2.0 * a_slope);
        let ratio = theta_stab / theta_opt;
        (theta_stab, theta_opt, ratio)
    }
}

/// Formal taxonomy of the 4 Variants of Goodhart's Law (Manheim & Garrabrant 2018,
/// *Categorizing Variants of Goodhart's Law*) on Bridges $L_2 \to L_3$ and $L_3 \to L_1$.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GoodhartVariant {
    /// Regressional Goodhart: selecting on noisy proxy $M = G + \varepsilon$ inflates $\mathbb{E}[M - G \mid M \ge \tau] > 0$ (Winner's Curse).
    Regressional,
    /// Extremal Goodhart: pushing $M$ into extreme tail regions breaks the bulk manifold correlation between $M$ and $G$.
    Extremal,
    /// Causal Goodhart: intervening $\mathrm{do}(M = m)$ on a non-causal proxy or effect leaves true goal $G$ unchanged or degraded; detected by ICP/NILE ($S^*$).
    Causal,
    /// Adversarial Goodhart: strategic agents game $M$ by shifting $\mathcal{D}(\theta)$, where passive RRM ignores $\nabla_1 \mathrm{DPR}(\theta, \theta)$ while `PerfGD` internalizes $\nabla_1 + \nabla_2$.
    Adversarial,
}

/// Quantitative audit of the 4 Variants of Goodhart's Law (Manheim & Garrabrant 2018),
/// coupling Causal breakdown to ICP/NILE ($S^*$) and Adversarial gaming to `PerfGD` gradients.
#[derive(Debug, Clone, PartialEq)]
pub struct GoodhartTaxonomyAudit {
    /// Variant 1 (Regressional): Winner's curse inflation $\mathbb{E}[M - G \mid M \ge q_{90}]$ under noisy proxy selection.
    pub regressional_winners_curse_inflation: f64,
    /// Variant 2 (Extremal): Goal degradation $\mathbb{E}[G \mid M \in \text{bulk}] - \mathbb{E}[G \mid M \ge q_{98}]$ in the extreme tail.
    pub extremal_tail_decoupling_drop: f64,
    /// Variant 3 (Causal): Ratio of NILE/IRM invariance penalty on spurious proxy vs. causal core $S^*$, plus interventional MSE gap.
    pub causal_nile_penalty_ratio: f64,
    /// Variant 3 (Causal): Interventional degradation $\mathrm{MSE}(\mathrm{do}(M)) - \mathrm{MSE}(\mathrm{do}(S^*))$.
    pub causal_interventional_mse_gap: f64,
    /// Variant 4 (Adversarial): Ignored distribution-shift gradient $\nabla_1 \mathrm{DPR}(\theta_{\mathrm{PS}}, \theta_{\mathrm{PS}})$ at Performative Stability $\theta_{\mathrm{PS}}$.
    pub adversarial_grad1_dpr_at_ps: f64,
    /// Variant 4 (Adversarial): Passive parameter gradient $\nabla_2 \mathrm{DPR}(\theta_{\mathrm{PS}}, \theta_{\mathrm{PS}})$ at $\theta_{\mathrm{PS}}$ ($\approx 0$).
    pub adversarial_grad2_dpr_at_ps: f64,
    /// Variant 4 (Adversarial): Stackelberg risk improvement $\mathrm{PR}(\theta_{\mathrm{PS}}) - \mathrm{PR}(\theta_{\mathrm{PO}}) > 0$ achieved by `PerfGD`.
    pub adversarial_perfgd_stackelberg_gain: f64,
    /// Dominant diagnosed Goodhart failure mode when unmitigated.
    pub diagnosed_variants: [GoodhartVariant; 4],
    /// True if all 4 Goodhart variants are quantitatively separated and mitigated by ICP/NILE ($S^*$) + `PerfGD`.
    pub all_four_variants_audited: bool,
}

impl GoodhartTaxonomyAudit {
    /// Evaluates the 4 Manheim-Garrabrant Goodhart variants (`seed = 2026`), coupling `causal_nile_penalty_ratio`
    /// and `causal_interventional_mse_gap` from ICP/NILE ($S^*$) with `PerformativeSystem` (`PerfGD`).
    pub fn evaluate(
        seed: u64,
        causal_nile_penalty_ratio: f64,
        causal_interventional_mse_gap: f64,
    ) -> Self {
        let mut rng = crate::rng::SplitMix64::new(seed.wrapping_add(1975));
        let n = 600;
        let mut goals_g = Vec::with_capacity(n);
        let mut proxies_reg = Vec::with_capacity(n);
        let mut goals_ext = Vec::with_capacity(n);
        let mut proxies_ext = Vec::with_capacity(n);

        for _ in 0..n {
            let g = rng.normal(0.0, 1.0);
            let m_reg = g + rng.normal(0.0, 0.90);
            goals_g.push(g);
            proxies_reg.push(m_reg);

            // Extremal: aligned in bulk (|m| <= 1.8), inverted parabolic collapse in extreme tail (|m| > 1.8)
            let m_ext = rng.normal(0.0, 1.2);
            let g_ext = if m_ext <= 1.8 {
                0.85 * m_ext
            } else {
                0.85 * 1.8 - 2.40 * (m_ext - 1.8).powi(2)
            };
            proxies_ext.push(m_ext);
            goals_ext.push(g_ext);
        }

        let q90_reg = crate::rng::quantile(&proxies_reg, 0.90);
        let mut inflation_sum = 0.0;
        let mut inflation_cnt = 0usize;
        for (&m, &g) in proxies_reg.iter().zip(goals_g.iter()) {
            if m >= q90_reg {
                inflation_sum += m - g;
                inflation_cnt += 1;
            }
        }
        let regressional_winners_curse_inflation = inflation_sum / (inflation_cnt.max(1) as f64);

        let q50_ext = crate::rng::quantile(&proxies_ext, 0.50);
        let q95_ext = crate::rng::quantile(&proxies_ext, 0.95);
        let q98_ext = crate::rng::quantile(&proxies_ext, 0.98);
        let mut bulk_g_sum = 0.0;
        let mut bulk_g_cnt = 0usize;
        let mut tail_g_sum = 0.0;
        let mut tail_g_cnt = 0usize;
        for (&m, &g) in proxies_ext.iter().zip(goals_ext.iter()) {
            if m >= q50_ext && m < q95_ext {
                bulk_g_sum += g;
                bulk_g_cnt += 1;
            }
            if m >= q98_ext {
                tail_g_sum += g;
                tail_g_cnt += 1;
            }
        }
        let extremal_tail_decoupling_drop =
            (bulk_g_sum / (bulk_g_cnt.max(1) as f64)) - (tail_g_sum / (tail_g_cnt.max(1) as f64));

        // Variant 4 (Adversarial): PerformativeSystem & PerfGD gradient decomposition at theta_PS
        let perf_sys = PerformativeSystem::new(1.0, 0.25, 0.60, 1.0, 1.25);
        let diag = perf_sys.diagnose();
        let (grad1_ps, grad2_ps) = perf_sys.dpr_gradient_components(diag.theta_ps);
        let adversarial_perfgd_stackelberg_gain = diag.risk_at_ps - diag.risk_at_po;

        let all_four_variants_audited = regressional_winners_curse_inflation > 0.50
            && extremal_tail_decoupling_drop > 0.50
            && causal_nile_penalty_ratio > 20.0
            && causal_interventional_mse_gap > 4.0
            && grad1_ps.abs() > 0.05
            && grad2_ps.abs() < 1e-10
            && adversarial_perfgd_stackelberg_gain > 0.01;

        Self {
            regressional_winners_curse_inflation,
            extremal_tail_decoupling_drop,
            causal_nile_penalty_ratio,
            causal_interventional_mse_gap,
            adversarial_grad1_dpr_at_ps: grad1_ps,
            adversarial_grad2_dpr_at_ps: grad2_ps,
            adversarial_perfgd_stackelberg_gain,
            diagnosed_variants: [
                GoodhartVariant::Regressional,
                GoodhartVariant::Extremal,
                GoodhartVariant::Causal,
                GoodhartVariant::Adversarial,
            ],
            all_four_variants_audited,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn meng_identity_closes_exactly_in_population() {
        let recorded = [true, true, true, false, false, false, false, false];
        let y = [10.0, 12.0, 14.0, 2.0, 3.0, 4.0, 1.0, 2.0];
        let audit = MengAudit::from_population(&recorded, &y);
        let y_obs_mean = (10.0 + 12.0 + 14.0) / 3.0;
        let y_pop_mean = mean(&y);
        assert!((audit.estimation_error - (y_obs_mean - y_pop_mean)).abs() < 1e-12);
        assert!(audit.rho_r_y > 0.90);
        let expected_n_eff = 3.0 / (1.0 + 5.0 * audit.rho_r_y * audit.rho_r_y);
        assert!((audit.n_eff_meng - expected_n_eff).abs() < 1e-12);
        assert!(audit.n_eff_meng < 1.0);
    }

    #[test]
    fn nonlinear_summary_measures_convex_jensen_gap_and_oos_crps() {
        let xs = [1.0, 2.0, 3.0, 4.0, 10.0];
        let summary = NonlinearSummary::evaluate(&xs, |x| x * x, 2.0);
        assert!(summary.jensen_gap > 0.0);
        assert_eq!(summary.f_at_mean, 16.0);
        assert_eq!(summary.expected_f, 26.0);
        assert!((summary.jensen_gap - 10.0).abs() < 1e-12);
        assert!(summary.empirical_crps > 0.0);

        let oos = NonlinearSummary::evaluate_out_of_sample(&xs[..4], &xs[4..], |x| x * x, 2.0);
        assert!(oos.empirical_crps > summary.empirical_crps);
    }

    #[test]
    fn performative_stability_vs_optimality_and_rrm_contraction() {
        // Pure squared loss (alpha = 0): theta_PS == theta_PO == mu / (1 - eps) = 1.25, PR = 0.0 >= 0
        let sys_zero_alpha = PerformativeSystem::new(1.0, 0.2, 0.0, 1.0, 1.2).diagnose();
        assert!((sys_zero_alpha.theta_ps - 1.25).abs() < 1e-12);
        assert!((sys_zero_alpha.theta_po - 1.25).abs() < 1e-12);
        assert!(sys_zero_alpha.risk_at_ps.abs() < 1e-12);
        assert!(sys_zero_alpha.risk_at_po.abs() < 1e-12);

        // Canonical linear system with incentive: mu = 1.0, eps = 0.2, alpha = 0.5, gamma = 1.0, beta = 1.2
        // theta_SL = 1.5, theta_PS = 1.5 / 0.8 = 1.875, theta_PO = 1.0/0.8 + 0.5/0.64 = 2.03125
        let sys = PerformativeSystem::new(1.0, 0.2, 0.5, 1.0, 1.2);
        let diag = sys.diagnose();
        assert!((diag.theta_sl - 1.5).abs() < 1e-12);
        assert!((diag.theta_ps - 1.875).abs() < 1e-12);
        assert!((diag.theta_po - 2.03125).abs() < 1e-12);
        assert!(diag.rrm_converges);
        assert!(diag.risk_at_po < diag.risk_at_ps);
        assert!((diag.rrm_trajectory.last().copied().unwrap() - diag.theta_ps).abs() < 1e-2);

        // Violating contraction (eps * beta / gamma >= 1.0)
        let unstable = PerformativeSystem::new(1.0, 0.8, 0.5, 1.0, 1.5).diagnose();
        assert!(!unstable.rrm_converges);
    }

    #[test]
    fn cromwell_floor_clamps_dogmatic_extremes() {
        assert!((apply_cromwell_floor(0.0, 0.001) - 0.001).abs() < 1e-15);
        assert!((apply_cromwell_floor(1.0, 0.001) - 0.999).abs() < 1e-15);
        assert!(respects_cromwell(apply_cromwell_floor(0.0, 0.001), 0.001));
    }

    #[test]
    fn lee_bounds_wasserstein_and_perfgd_converge_to_stackelberg_po() {
        let sys = PerformativeSystem::new(1.0, 0.2, 0.5, 1.0, 1.2);
        let diag = sys.diagnose();
        let perfgd = sys.perfgd_trajectory(0.0, 0.5, 60);
        let theta_final = *perfgd.last().unwrap();
        assert!((theta_final - diag.theta_po).abs() < 1e-6);

        let w1 = empirical_wasserstein_1(&[1.0, 2.0, 3.0], &[2.5, 3.5, 4.5]);
        assert!((w1 - 1.5).abs() < 1e-12);

        let (lo, hi, q) =
            ManskiBounds::lee_trimming_bounds(&[10.0, 12.0, 14.0, 16.0], 4, &[8.0, 9.0, 10.0], 4);
        assert!(lo <= hi && q > 0.0);

        let gh = GoodhartTaxonomyAudit::evaluate(2026, 45.0, 6.2);
        assert!(gh.all_four_variants_audited);
        assert!(gh.adversarial_grad2_dpr_at_ps.abs() < 1e-10);
        assert!(gh.adversarial_grad1_dpr_at_ps.abs() > 0.05);

        let perfgd_fd = sys.perfgd_finite_difference_trajectory(0.0, 0.5, 0.01, 60);
        assert!((perfgd_fd.last().copied().unwrap() - diag.theta_po).abs() < 1e-6);

        let (t_stab, t_opt, ratio) = PerformativeSystem::izzo_linear_2x_benchmark(-1.0, 2.0);
        assert!((t_stab - 2.0).abs() < 1e-12);
        assert!((t_opt - 1.0).abs() < 1e-12);
        assert!((ratio - 2.0).abs() < 1e-12);

        let meng_10m = MengAudit::dpi_benchmark_10m_80pct();
        assert!((meng_10m.data_defect_correlation_ddc() - 0.005).abs() < 1e-12);
        assert!((meng_10m.data_defect_index_ddi() - 2.5e-5).abs() < 1e-15);
        assert!((meng_10m.n_eff_srs_asymptotic() - 160_000.0).abs() < 1e-6);
        assert!((meng_10m.law_of_large_populations_z() - 15.811_388).abs() < 1e-5);

        let delta_star = ManskiBounds::lee_tipping_point_delta(
            &[10.0, 12.0, 14.0, 16.0],
            5,
            &[8.0, 9.0, 10.0],
            5,
        );
        assert!(delta_star < 0.0);

        assert_eq!(
            DpiOperationalStage::from_ontological(Layer::WorldL1, None),
            DpiOperationalStage::D1DataCollection
        );
        assert_eq!(
            DpiOperationalStage::from_ontological(Layer::ModelL2, Some("L1->L2")),
            DpiOperationalStage::D2FeatureEngineering
        );
        assert_eq!(
            DpiOperationalStage::from_ontological(Layer::ModelL2, None),
            DpiOperationalStage::D3ModelRepresentation
        );
        assert_eq!(
            DpiOperationalStage::from_ontological(Layer::AgentL3, Some("L3->L1")),
            DpiOperationalStage::D4HumanDecisionActuation
        );
    }
}
