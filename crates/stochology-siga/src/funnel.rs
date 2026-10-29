//! The 4 Eliminatory Filters (§3.1): Sequential Pruning Before Inference.
//!
//! - **Filter 1 (Decision & Resolution):** $p^* = \frac{C}{\eta L} \in (0, 1)$, $\mathrm{EVPI} > C_{\text{total}}$, and Invariant $\Phi_5$.
//! - **Filter 2 (Effective Sample Size & Causal Specification):** Moulton-Campbell $n_{\mathrm{eff}}(p) \ge n_{\min}$,
//!   declarative Markov Blanket specification check, and ban on conditioning on colliders (Berkson's paradox).
//! - **Filter 3 (Generative Physics & Prior Predictive Check):** Simulation-Based Calibration (SBC $\chi^2$
//!   rank uniformity) and declarative SHELF quartile domain coherence check.
//! - **Filter 4 (Cryptographic Pre-Registration & Cross-Authorship):** SHA-256 seal, independent
//!   cross-authorship (`null_model != decision_rule != adversarial_reviewer`), and numeric exit clause.

use stochology_core::{is_valid_sha256_hex, verify_resolution_phi5};
use stochology_lenses::decision::CostLossRule;

/// Candidate hypothesis/model submitted to the 4 Eliminatory Filters (§3.1).
#[derive(Debug, Clone)]
pub struct FunnelCandidate {
    // Filter 1
    pub who_decides: String,
    pub altered_action: String,
    pub independent_unit: String,
    pub prior_probability_p: f64,
    pub protection_cost_c: f64,
    pub event_loss_l: f64,
    pub efficacy_eta: f64,
    pub decision_volume_units: f64,
    pub total_modeling_cost: f64,

    // Filter 2
    pub effective_tail_n_eff_p: f64,
    pub minimum_required_n_eff: f64,
    pub markov_blanket_mapped: bool,
    pub conditions_on_collider_berkson: bool,

    // Filter 3
    pub sbc_rank_histogram: Vec<usize>,
    pub sbc_chi2_critical: f64,
    pub shelf_quartiles_q25_q50_q75: (f64, f64, f64),
    pub physical_domain_bounds: (f64, f64),

    // Filter 4
    pub preregistered_plan_text: String,
    pub declared_sha256_seal: String,
    pub author_null_model: String,
    pub author_decision_rule: String,
    pub author_adversarial_reviewer: String,
    pub numeric_exit_clause: String,
}

/// Verdict of the 4 Eliminatory Filters.
#[derive(Debug, Clone, PartialEq)]
pub struct FunnelResult {
    pub calculated_evpi: f64,
    pub filter_1_approved: bool,
    pub filter_2_approved: bool,
    pub sbc_chi2_statistic: f64,
    pub filter_3_approved: bool,
    pub filter_4_approved: bool,
    /// First filter (1..=4) where the candidate was pruned, or `None` if all 4 passed.
    pub pruned_at_filter: Option<u8>,
    pub pruning_reason: Option<String>,
    pub approved_all_filters: bool,
}

/// Evaluates the uniformity of Simulation-Based Calibration (SBC) rank counts via Pearson's $\chi^2$.
pub fn sbc_rank_chi2(rank_bins: &[usize]) -> f64 {
    assert!(!rank_bins.is_empty());
    let total: f64 = rank_bins.iter().map(|&obs| obs as f64).sum();
    if total <= 0.0 {
        return f64::INFINITY;
    }
    let expected = total / (rank_bins.len() as f64);
    rank_bins
        .iter()
        .map(|&obs| {
            let diff = (obs as f64) - expected;
            (diff * diff) / expected
        })
        .sum()
}

/// Lanczos approximation for $\ln \Gamma(z)$ ($z > 0$) with 9 coefficients (precision $\sim 10^{-15}$).
pub fn ln_gamma_lanczos(z: f64) -> f64 {
    assert!(z > 0.0, "ln_gamma_lanczos requires z > 0");
    const COEFFS: [f64; 9] = [
        0.999_999_999_999_809_9,
        676.520_368_121_885_1,
        -1_259.139_216_722_402_8,
        771.323_428_777_653_1,
        -176.615_029_162_140_6,
        12.507_343_278_686_905,
        -0.138_571_095_265_720_12,
        9.984_369_578_019_572e-6,
        1.505_632_735_149_311_6e-7,
    ];
    let z_minus_1 = z - 1.0;
    let mut x = COEFFS[0];
    for (i, &c) in COEFFS.iter().enumerate().skip(1) {
        x += c / (z_minus_1 + (i as f64));
    }
    let t = z_minus_1 + 7.5;
    0.5 * std::f64::consts::TAU.ln() + (z_minus_1 + 0.5) * t.ln() - t + x.ln()
}

/// Regularized incomplete Beta function $I_x(a, b) = \frac{B(x; a, b)}{B(a, b)}$ evaluated via
/// Lentz's continued-fraction algorithm (DLMF §8.17.22).
pub fn regularized_incomplete_beta(x: f64, a: f64, b: f64) -> f64 {
    assert!(a > 0.0 && b > 0.0, "requires a > 0 and b > 0");
    if x <= 0.0 {
        return 0.0;
    }
    if x >= 1.0 {
        return 1.0;
    }
    let sym_threshold = (a + 1.0) / (a + b + 2.0);
    if x > sym_threshold {
        return 1.0 - regularized_incomplete_beta(1.0 - x, b, a);
    }
    let ln_beta = ln_gamma_lanczos(a) + ln_gamma_lanczos(b) - ln_gamma_lanczos(a + b);
    let front = (a * x.ln() + b * (1.0 - x).ln() - ln_beta).exp() / a;

    // Modified Lentz continued fraction for I_x(a, b)
    let tiny = 1e-30_f64;
    let mut c = 1.0_f64;
    let mut d = 1.0 - ((a + b) * x / (a + 1.0));
    if d.abs() < tiny {
        d = tiny;
    }
    d = 1.0 / d;
    let mut f = d;

    for m in 1..=200 {
        let m_f = m as f64;
        let two_m = 2.0 * m_f;
        // Even step d_{2m}
        let num_even = (m_f * (b - m_f) * x) / ((a + two_m - 1.0) * (a + two_m));
        d = 1.0 + num_even * d;
        if d.abs() < tiny {
            d = tiny;
        }
        c = 1.0 + num_even / c;
        if c.abs() < tiny {
            c = tiny;
        }
        d = 1.0 / d;
        f *= c * d;

        // Odd step d_{2m+1}
        let num_odd = -((a + m_f) * (a + b + m_f) * x) / ((a + two_m) * (a + two_m + 1.0));
        d = 1.0 + num_odd * d;
        if d.abs() < tiny {
            d = tiny;
        }
        c = 1.0 + num_odd / c;
        if c.abs() < tiny {
            c = tiny;
        }
        d = 1.0 / d;
        let delta = c * d;
        f *= delta;
        if (delta - 1.0).abs() < 1e-14 {
            break;
        }
    }
    (front * f).clamp(0.0, 1.0)
}

/// Fits a conjugate Beta prior $\mathrm{Beta}(\hat{\alpha}, \hat{\beta})$ to SHELF elicited quartiles
/// $(q_{25}, q_{50}, q_{75})$ (O'Hagan / Oakley — Filter 3, §3.1) by minimizing the exact CDF residual
/// $\bigl(I_{q_{25}}(\alpha, \beta) - 0.25\bigr)^2 + \bigl(I_{q_{50}}(\alpha, \beta) - 0.50\bigr)^2 + \bigl(I_{q_{75}}(\alpha, \beta) - 0.75\bigr)^2$.
/// Returns `(alpha_hat, beta_hat, cdf_rmse)`.
pub fn fit_shelf_beta_prior(q25: f64, q50: f64, q75: f64) -> (f64, f64, f64) {
    assert!(
        0.0 < q25 && q25 < q50 && q50 < q75 && q75 < 1.0,
        "requires 0 < q25 < q50 < q75 < 1"
    );
    // Method-of-moments warm start from median and IQR
    let iqr = (q75 - q25).max(1e-4);
    let sd_approx = (iqr / 1.349).max(1e-4);
    let mu0 = q50.clamp(0.01, 0.99);
    let conc0 = ((mu0 * (1.0 - mu0)) / (sd_approx * sd_approx) - 1.0).clamp(1.0, 400.0);

    let obj = |a: f64, b: f64| -> f64 {
        let r25 = regularized_incomplete_beta(q25, a, b) - 0.25;
        let r50 = regularized_incomplete_beta(q50, a, b) - 0.50;
        let r75 = regularized_incomplete_beta(q75, a, b) - 0.75;
        ((r25 * r25 + r50 * r50 + r75 * r75) / 3.0).sqrt()
    };

    let mut best_a = (mu0 * conc0).max(0.2);
    let mut best_b = ((1.0 - mu0) * conc0).max(0.2);
    let mut best_err = obj(best_a, best_b);

    for scale in [0.5, 0.2, 0.08, 0.02, 0.005] {
        for da in -4..=4 {
            for db in -4..=4 {
                let cand_a = (best_a * (1.0 + (da as f64) * scale)).clamp(0.1, 1000.0);
                let cand_b = (best_b * (1.0 + (db as f64) * scale)).clamp(0.1, 1000.0);
                let err = obj(cand_a, cand_b);
                if err < best_err {
                    best_err = err;
                    best_a = cand_a;
                    best_b = cand_b;
                }
            }
        }
    }
    (best_a, best_b, best_err)
}

pub use stochology_core::prospective_clustered_mde;

/// Prospective gold-set sample size $n_{\text{gold}}^*$ for Prediction-Powered Inference (PPI;
/// Angelopoulos et al., 2023) at Filter 2 before deploying an `LLM-as-a-Judge` evaluator:
/// $$n_{\text{gold}}^* = \left\lceil \frac{\sigma_{\Delta}^2}{\bigl(h_{\text{target}} / z_{1-\alpha/2}\bigr)^2 - \sigma_f^2 / N} \right\rceil,$$
/// where $\sigma_{\Delta}^2 = \mathrm{Var}(Y - f(X))$ is the rectifier residual variance, $\sigma_f^2 = \mathrm{Var}(f(\tilde{X}))$
/// is the judge variance on $N$ unlabeled records, and $h_{\text{target}}$ is the maximum tolerated half-width.
/// Returns `usize::MAX` if the unlabeled variance floor $\sigma_f^2 / N$ already exceeds $(h_{\text{target}} / z_{1-\alpha/2})^2$.
pub fn prospective_ppi_gold_sample_size(
    target_half_width: f64,
    z_alpha_half: f64,
    unlabeled_n: usize,
    var_f: f64,
    var_rectifier_diff: f64,
) -> usize {
    assert!(
        target_half_width.is_finite()
            && target_half_width > 0.0
            && z_alpha_half.is_finite()
            && z_alpha_half > 0.0
            && unlabeled_n > 0
            && var_f.is_finite()
            && var_f >= 0.0
            && var_rectifier_diff.is_finite()
            && var_rectifier_diff > 0.0,
        "prospective_ppi_gold_sample_size requires positive finite arguments"
    );
    let target_var = (target_half_width / z_alpha_half).powi(2);
    let unlabeled_var = var_f / (unlabeled_n as f64);
    let residual_budget = target_var - unlabeled_var;
    if residual_budget <= 1e-15 {
        return usize::MAX;
    }
    ((var_rectifier_diff / residual_budget).ceil() as usize).max(2)
}

/// Effective sample-size multiplier of Prediction-Powered Inference relative to classical gold-only
/// estimation on $n_{\text{gold}}$ human-labeled records:
/// $$\mathrm{ESS}_{\text{mult}} = \frac{\sigma_Y^2 / n_{\text{gold}}}{\sigma_f^2 / N + \sigma_{\Delta}^2 / n_{\text{gold}}}.$$
pub fn ppi_effective_sample_size_multiplier(
    var_y: f64,
    var_f: f64,
    var_rectifier_diff: f64,
    unlabeled_n: usize,
    gold_n: usize,
) -> f64 {
    assert!(
        var_y.is_finite()
            && var_y > 0.0
            && var_f.is_finite()
            && var_f >= 0.0
            && var_rectifier_diff.is_finite()
            && var_rectifier_diff > 0.0
            && unlabeled_n > 0
            && gold_n > 0,
        "ppi_effective_sample_size_multiplier requires positive finite variances and sample sizes"
    );
    let classical_var = var_y / (gold_n as f64);
    let ppi_var = var_f / (unlabeled_n as f64) + var_rectifier_diff / (gold_n as f64);
    classical_var / ppi_var.max(1e-15)
}

/// Filter 2 Causal Resolution of Simpson's Paradox (`SimpsonCausalParadoxAudit`, Pearl 2009 / $\Phi_3$):
///
/// Given a $2 \times 2 \times 2$ contingency table $(A \in \{0,1\}, Y \in \{0,1\}, Z \in \{0,1\})$ where
/// treatment $A=1$ increases success within every stratum $Z=z$ ($\Delta_z > 0$) yet reverses sign in the
/// coarse pooled aggregate ($\Delta_{\text{pooled}} < 0$), pure Rung-1 statistics cannot decide whether to
/// stratify or aggregate:
/// - **Confounder DAG ($Z \to A, Z \to Y$):** Back-door `do`-adjustment
///   $P(Y=1 \mid \mathrm{do}(A=a)) = \sum_z P(Y=1 \mid A=a, Z=z)P(Z=z)$ yields $\tau_{\text{confounder}}^{\mathrm{do}} > 0$
///   (stratification is mandatory; coarse pooling lies).
/// - **Mediator DAG ($A \to Z \to Y$):** Front-door / total-effect `do`-distribution
///   $P(Y=1 \mid \mathrm{do}(A=a)) = P(Y=1 \mid A=a)$ yields $\tau_{\text{mediator}}^{\mathrm{do}} < 0$
///   (pooling is mandatory; conditioning on post-treatment $Z$ blocks the causal pathway and lies).
#[derive(Debug, Clone, PartialEq)]
pub struct SimpsonCausalParadoxAudit {
    /// Stratum $Z=0$ risk difference $P(Y=1 \mid A=1, Z=0) - P(Y=1 \mid A=0, Z=0)$.
    pub stratum_z0_diff: f64,
    /// Stratum $Z=1$ risk difference $P(Y=1 \mid A=1, Z=1) - P(Y=1 \mid A=0, Z=1)$.
    pub stratum_z1_diff: f64,
    /// Coarse pooled risk difference $P(Y=1 \mid A=1) - P(Y=1 \mid A=0)$.
    pub pooled_diff: f64,
    /// Exact Law of Total Probability identity residual $|P(Y=1\mid A=a) - \sum_z P(Y=1\mid A=a,Z=z)P(Z=z\mid A=a)|$.
    pub total_probability_identity_error: f64,
    /// True causal effect $\tau_{\text{confounder}}^{\mathrm{do}}$ under the Confounder DAG ($Z \to A, Z \to Y$).
    pub confounder_do_ate: f64,
    /// True causal effect $\tau_{\text{mediator}}^{\mathrm{do}}$ under the Mediator DAG ($A \to Z \to Y$).
    pub mediator_do_ate: f64,
    /// Whether Simpson's sign reversal holds ($\Delta_0 > 0, \Delta_1 > 0$, yet $\Delta_{\text{pooled}} < 0$)
    /// and the two Rung-2 DAGs demand opposite decisions from the exact same Rung-1 numbers.
    pub simpson_reversal_and_dag_dichotomy_verified: bool,
}

impl SimpsonCausalParadoxAudit {
    /// Evaluates a $2 \times 2 \times 2$ table given cell counts `(successes, total)` for
    /// `(A=0, Z=0)`, `(A=0, Z=1)`, `(A=1, Z=0)`, and `(A=1, Z=1)`.
    pub fn evaluate_counts(
        a0_z0: (usize, usize),
        a0_z1: (usize, usize),
        a1_z0: (usize, usize),
        a1_z1: (usize, usize),
    ) -> Self {
        assert!(
            a0_z0.1 > 0 && a0_z1.1 > 0 && a1_z0.1 > 0 && a1_z1.1 > 0,
            "SimpsonCausalParadoxAudit requires positive cell denominators"
        );
        let p_y_a0_z0 = (a0_z0.0 as f64) / (a0_z0.1 as f64);
        let p_y_a0_z1 = (a0_z1.0 as f64) / (a0_z1.1 as f64);
        let p_y_a1_z0 = (a1_z0.0 as f64) / (a1_z0.1 as f64);
        let p_y_a1_z1 = (a1_z1.0 as f64) / (a1_z1.1 as f64);

        let n_a0 = (a0_z0.1 + a0_z1.1) as f64;
        let n_a1 = (a1_z0.1 + a1_z1.1) as f64;
        let n_total = n_a0 + n_a1;

        let p_y_a0_pooled = ((a0_z0.0 + a0_z1.0) as f64) / n_a0;
        let p_y_a1_pooled = ((a1_z0.0 + a1_z1.0) as f64) / n_a1;

        let p_z0_given_a0 = (a0_z0.1 as f64) / n_a0;
        let p_z1_given_a0 = (a0_z1.1 as f64) / n_a0;
        let p_z0_given_a1 = (a1_z0.1 as f64) / n_a1;
        let p_z1_given_a1 = (a1_z1.1 as f64) / n_a1;

        let recon_a0 = p_y_a0_z0 * p_z0_given_a0 + p_y_a0_z1 * p_z1_given_a0;
        let recon_a1 = p_y_a1_z0 * p_z0_given_a1 + p_y_a1_z1 * p_z1_given_a1;
        let total_probability_identity_error = (p_y_a0_pooled - recon_a0)
            .abs()
            .max((p_y_a1_pooled - recon_a1).abs());

        let p_z0_marginal = ((a0_z0.1 + a1_z0.1) as f64) / n_total;
        let p_z1_marginal = ((a0_z1.1 + a1_z1.1) as f64) / n_total;

        let stratum_z0_diff = p_y_a1_z0 - p_y_a0_z0;
        let stratum_z1_diff = p_y_a1_z1 - p_y_a0_z1;
        let pooled_diff = p_y_a1_pooled - p_y_a0_pooled;

        let do_y_a1_conf = p_y_a1_z0 * p_z0_marginal + p_y_a1_z1 * p_z1_marginal;
        let do_y_a0_conf = p_y_a0_z0 * p_z0_marginal + p_y_a0_z1 * p_z1_marginal;
        let confounder_do_ate = do_y_a1_conf - do_y_a0_conf;
        let mediator_do_ate = pooled_diff;

        let verified = stratum_z0_diff > 0.0
            && stratum_z1_diff > 0.0
            && pooled_diff < 0.0
            && confounder_do_ate > 0.0
            && mediator_do_ate < 0.0
            && total_probability_identity_error < 1e-12;

        Self {
            stratum_z0_diff,
            stratum_z1_diff,
            pooled_diff,
            total_probability_identity_error,
            confounder_do_ate,
            mediator_do_ate,
            simpson_reversal_and_dag_dichotomy_verified: verified,
        }
    }

    /// Canonical kidney-stone / clinical triage benchmark (Charig et al. 1986; Pearl 2009, Table 6.1):
    /// $N = 700$ patients ($350$ per arm), where $Z=0$ is small stones and $Z=1$ is large stones.
    pub fn canonical_pearl_benchmark() -> Self {
        Self::evaluate_counts((234, 270), (55, 80), (81, 87), (192, 263))
    }
}

/// Filter 2 Long-Context Attention Dilution vs. Markov Blanket Sufficiency Audit (`LongContextDpiDilutionAudit`):
///
/// Audits the "100 pages in 10 seconds" extended-context workflow by measuring softmax attention signal
/// retention when $k_{\text{blanket}}$ relevant chunks of the Markov Blanket $M(Y)$ (at logit advantage $\beta \Delta_s$)
/// are buried inside $T_{\text{raw}} \gg k_{\text{blanket}}$ total document pages (*Lost-in-the-Middle* dilution).
#[derive(Debug, Clone, PartialEq)]
pub struct LongContextDpiDilutionAudit {
    /// Total unpruned document chunks/pages $T_{\text{raw}}$ (e.g. $100$ pages).
    pub raw_context_pages: usize,
    /// Sufficient Markov Blanket chunks $k_{\text{blanket}} \ll T_{\text{raw}}$.
    pub markov_blanket_pages: usize,
    /// Softmax attention mass captured by the true Markov Blanket in the unpruned $T_{\text{raw}}$-page context.
    pub raw_blanket_attention_mass: f64,
    /// Softmax attention mass captured by the true Markov Blanket after Filter 2 pruning to $k_{\text{blanket}} + k_{\text{guard}}$ pages.
    pub pruned_blanket_attention_mass: f64,
    /// Effective signal-to-noise ratio (in decibans) before vs. after Markov Blanket pruning.
    pub snr_gain_decibans: f64,
    /// Exact softmax partition identity residual $|\sum_{i=1}^{T_{\text{raw}}} \alpha_i - 1|$.
    pub softmax_partition_identity_error: f64,
    /// Whether Markov Blanket pruning restores signal dominance ($\alpha_{\text{pruned}} \ge 0.80 > \alpha_{\text{raw}}$).
    pub markov_blanket_pruning_approved: bool,
}

impl LongContextDpiDilutionAudit {
    /// Evaluates long-context softmax dilution at inverse temperature $\beta$ and signal logit margin $\Delta_s$.
    pub fn evaluate(
        raw_context_pages: usize,
        markov_blanket_pages: usize,
        pruned_guard_distractors: usize,
        logit_signal_margin: f64,
        positional_middle_penalty: f64,
    ) -> Self {
        assert!(
            raw_context_pages > markov_blanket_pages && markov_blanket_pages > 0,
            "LongContextDpiDilutionAudit requires raw_context_pages > markov_blanket_pages > 0"
        );
        let k = markov_blanket_pages as f64;
        let n_distract_raw = (raw_context_pages - markov_blanket_pages) as f64;
        // In a long unpruned context, relevant clauses sitting in the middle suffer U-shaped positional attenuation
        let w_sig_raw = (logit_signal_margin - positional_middle_penalty.max(0.0)).exp();
        let z_raw = k * w_sig_raw + n_distract_raw * 1.0;
        let raw_blanket_attention_mass = (k * w_sig_raw) / z_raw;
        let raw_distract_mass = n_distract_raw / z_raw;
        let softmax_partition_identity_error =
            ((raw_blanket_attention_mass + raw_distract_mass) - 1.0).abs();

        // After Filter 2 Markov Blanket pruning, middle positional decay is eliminated and distractors are bounded
        let w_sig_pruned = logit_signal_margin.exp();
        let n_distract_pruned = (pruned_guard_distractors as f64).max(1.0);
        let z_pruned = k * w_sig_pruned + n_distract_pruned;
        let pruned_blanket_attention_mass = (k * w_sig_pruned) / z_pruned;

        let snr_raw = raw_blanket_attention_mass / (1.0 - raw_blanket_attention_mass).max(1e-12);
        let snr_pruned =
            pruned_blanket_attention_mass / (1.0 - pruned_blanket_attention_mass).max(1e-12);
        let snr_gain_decibans = 10.0 * (snr_pruned / snr_raw.max(1e-12)).log10();

        Self {
            raw_context_pages,
            markov_blanket_pages,
            raw_blanket_attention_mass,
            pruned_blanket_attention_mass,
            snr_gain_decibans,
            softmax_partition_identity_error,
            markov_blanket_pruning_approved: pruned_blanket_attention_mass >= 0.80
                && pruned_blanket_attention_mass > raw_blanket_attention_mass + 0.40
                && softmax_partition_identity_error < 1e-12,
        }
    }
}

/// Evaluates the 4 Eliminatory Filters in strict sequential order (Filter 1 -> 2 -> 3 -> 4).
pub fn evaluate_four_filter_funnel(c: &FunnelCandidate) -> FunnelResult {
    let params_valid = c.protection_cost_c.is_finite()
        && c.protection_cost_c > 0.0
        && c.event_loss_l.is_finite()
        && c.event_loss_l > 0.0
        && c.efficacy_eta.is_finite()
        && c.efficacy_eta > 0.0
        && c.efficacy_eta <= 1.0
        && c.prior_probability_p.is_finite()
        && c.prior_probability_p > 0.0
        && c.prior_probability_p < 1.0
        && c.decision_volume_units.is_finite()
        && c.decision_volume_units > 0.0
        && c.total_modeling_cost.is_finite()
        && c.total_modeling_cost >= 0.0;

    let denom = c.efficacy_eta * c.event_loss_l;
    let raw_p_star = if params_valid && denom > 0.0 && denom.is_finite() {
        c.protection_cost_c / denom
    } else {
        f64::NAN
    };
    let p_star_in_unit_interval = raw_p_star.is_finite() && raw_p_star > 0.0 && raw_p_star < 1.0;

    let evpi = if params_valid && p_star_in_unit_interval {
        let rule = CostLossRule::new(c.protection_cost_c, c.event_loss_l, c.efficacy_eta);
        rule.calculate_evpi(c.prior_probability_p, c.decision_volume_units)
    } else {
        0.0
    };

    // Filter 1: Decision & Resolution (p* in (0, 1) + Phi5 + EVPI > C_total)
    let f1_ok = !c.who_decides.trim().is_empty()
        && !c.altered_action.trim().is_empty()
        && params_valid
        && p_star_in_unit_interval
        && verify_resolution_phi5(&c.independent_unit, &c.altered_action)
        && evpi > c.total_modeling_cost;

    if !f1_ok {
        return FunnelResult {
            calculated_evpi: evpi,
            filter_1_approved: false,
            filter_2_approved: false,
            sbc_chi2_statistic: 0.0,
            filter_3_approved: false,
            filter_4_approved: false,
            pruned_at_filter: Some(1),
            pruning_reason: Some(
                "Pruned at Filter 1: p* outside (0, 1), EVPI <= total_modeling_cost, or Phi5 resolution violation"
                    .into(),
            ),
            approved_all_filters: false,
        };
    }

    // Filter 2: Effective Tail Sample Size n_eff(p) & Markov Blanket without Berkson Collider
    let f2_ok = c.effective_tail_n_eff_p >= c.minimum_required_n_eff
        && c.markov_blanket_mapped
        && !c.conditions_on_collider_berkson;

    if !f2_ok {
        return FunnelResult {
            calculated_evpi: evpi,
            filter_1_approved: true,
            filter_2_approved: false,
            sbc_chi2_statistic: 0.0,
            filter_3_approved: false,
            filter_4_approved: false,
            pruned_at_filter: Some(2),
            pruning_reason: Some(
                "Pruned at Filter 2: insufficient tail n_eff(p) or conditioning on a Berkson collider"
                    .into(),
            ),
            approved_all_filters: false,
        };
    }

    // Filter 3: Generative Physics (SBC rank uniformity + SHELF quartile coherence)
    let chi2 = sbc_rank_chi2(&c.sbc_rank_histogram);
    let (q25, q50, q75) = c.shelf_quartiles_q25_q50_q75;
    let (lo, hi) = c.physical_domain_bounds;
    let shelf_ok = lo <= q25 && q25 < q50 && q50 < q75 && q75 <= hi;
    let f3_ok = chi2 <= c.sbc_chi2_critical && shelf_ok;

    if !f3_ok {
        return FunnelResult {
            calculated_evpi: evpi,
            filter_1_approved: true,
            filter_2_approved: true,
            sbc_chi2_statistic: chi2,
            filter_3_approved: false,
            filter_4_approved: false,
            pruned_at_filter: Some(3),
            pruning_reason: Some(
                "Pruned at Filter 3: SBC rank histogram non-uniform or SHELF prior violates physical domain"
                    .into(),
            ),
            approved_all_filters: false,
        };
    }

    // Filter 4: Cryptographic SHA-256 Pre-Registration, Cross-Authorship, and Numeric Exit Clause
    let expected_hash = stochology_core::sha256_hex(c.preregistered_plan_text.as_bytes());
    let seal_ok =
        is_valid_sha256_hex(&c.declared_sha256_seal) && c.declared_sha256_seal == expected_hash;
    let authors_ok = !c.author_null_model.trim().is_empty()
        && !c.author_decision_rule.trim().is_empty()
        && !c.author_adversarial_reviewer.trim().is_empty()
        && c.author_null_model != c.author_decision_rule
        && c.author_adversarial_reviewer != c.author_null_model
        && c.author_adversarial_reviewer != c.author_decision_rule;
    let exit_ok = !c.numeric_exit_clause.trim().is_empty()
        && c.numeric_exit_clause.chars().any(|ch| ch.is_ascii_digit());
    let f4_ok = seal_ok && authors_ok && exit_ok;

    if !f4_ok {
        return FunnelResult {
            calculated_evpi: evpi,
            filter_1_approved: true,
            filter_2_approved: true,
            sbc_chi2_statistic: chi2,
            filter_3_approved: true,
            filter_4_approved: false,
            pruned_at_filter: Some(4),
            pruning_reason: Some(
                "Pruned at Filter 4: invalid SHA-256 pre-registration, non-independent cross-authorship, or missing exit clause"
                    .into(),
            ),
            approved_all_filters: false,
        };
    }

    FunnelResult {
        calculated_evpi: evpi,
        filter_1_approved: true,
        filter_2_approved: true,
        sbc_chi2_statistic: chi2,
        filter_3_approved: true,
        filter_4_approved: true,
        pruned_at_filter: None,
        pruning_reason: None,
        approved_all_filters: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use stochology_core::sha256_hex;

    fn valid_candidate() -> FunnelCandidate {
        let plan = "siga_plan_coastal_evacuation_shift_seed_2026";
        FunnelCandidate {
            who_decides: "Coastal Emergency Operations Director".into(),
            altered_action: "Trigger mandatory coastal evacuation order within 6h synoptic shift"
                .into(),
            independent_unit: "6h synoptic storm shift".into(),
            prior_probability_p: 0.05,
            protection_cost_c: 1_800.0,
            event_loss_l: 34_000.0,
            efficacy_eta: 1.0,
            decision_volume_units: 100.0,
            total_modeling_cost: 15_000.0,
            effective_tail_n_eff_p: 1_300.0,
            minimum_required_n_eff: 100.0,
            markov_blanket_mapped: true,
            conditions_on_collider_berkson: false,
            sbc_rank_histogram: vec![20, 20, 20, 20, 20],
            sbc_chi2_critical: 9.49,
            shelf_quartiles_q25_q50_q75: (0.02, 0.05, 0.09),
            physical_domain_bounds: (0.001, 0.999),
            preregistered_plan_text: plan.into(),
            declared_sha256_seal: sha256_hex(plan.as_bytes()),
            author_null_model: "inference".into(),
            author_decision_rule: "decision".into(),
            author_adversarial_reviewer: "independent_auditor".into(),
            numeric_exit_clause: "Suspend if REL_CORP > 0.015 over 14 days".into(),
        }
    }

    #[test]
    fn sound_candidate_passes_all_four_filters() {
        let c = valid_candidate();
        let res = evaluate_four_filter_funnel(&c);
        assert!(res.approved_all_filters);
        assert_eq!(res.pruned_at_filter, None);

        let (a_hat, b_hat, rmse) = fit_shelf_beta_prior(0.02, 0.05, 0.09);
        assert!(a_hat > 0.5 && b_hat > 5.0 && rmse < 0.03);
    }

    #[test]
    fn funnel_prunes_berkson_collider_at_filter2_and_same_author_at_filter4() {
        let mut c2 = valid_candidate();
        c2.conditions_on_collider_berkson = true;
        let r2 = evaluate_four_filter_funnel(&c2);
        assert_eq!(r2.pruned_at_filter, Some(2));

        let mut c4 = valid_candidate();
        c4.author_adversarial_reviewer = "inference".into();
        let r4 = evaluate_four_filter_funnel(&c4);
        assert_eq!(r4.pruned_at_filter, Some(4));

        let mut c4_no_digit = valid_candidate();
        c4_no_digit.numeric_exit_clause = "Suspend model when calibration degrades".into();
        let r4_no_digit = evaluate_four_filter_funnel(&c4_no_digit);
        assert_eq!(r4_no_digit.pruned_at_filter, Some(4));

        let mut c1_bad_pstar = valid_candidate();
        c1_bad_pstar.protection_cost_c = 40_000.0; // C >= eta * L -> p* >= 1.0
        let r1_bad_pstar = evaluate_four_filter_funnel(&c1_bad_pstar);
        assert_eq!(r1_bad_pstar.pruned_at_filter, Some(1));

        // Out-of-domain cost/loss/eta/NaN values short-circuit cleanly at Filter 1 without panicking
        let mut c1_bad_eta = valid_candidate();
        c1_bad_eta.efficacy_eta = 1.5;
        assert_eq!(
            evaluate_four_filter_funnel(&c1_bad_eta).pruned_at_filter,
            Some(1)
        );

        let mut c1_nonpos_cost = valid_candidate();
        c1_nonpos_cost.protection_cost_c = 0.0;
        assert_eq!(
            evaluate_four_filter_funnel(&c1_nonpos_cost).pruned_at_filter,
            Some(1)
        );

        let mut c1_nan_loss = valid_candidate();
        c1_nan_loss.event_loss_l = f64::NAN;
        assert_eq!(
            evaluate_four_filter_funnel(&c1_nan_loss).pruned_at_filter,
            Some(1)
        );
    }

    #[test]
    fn prospective_ppi_gold_sizing_and_ess_multiplier_resolve_finite_sample_refutation() {
        // At K = 5, n_gold = 160 with var_rectifier_diff ~ 0.22 has 95% half-width ~ 0.073 (> 0.06).
        // Prospective sizing computes the exact gold sample size needed to guarantee half-width <= 0.06:
        let n_star = prospective_ppi_gold_sample_size(0.06, 1.959_964, 2_000, 0.21, 0.22);
        assert!(n_star > 160 && n_star < 400);
        let ess_mult = ppi_effective_sample_size_multiplier(0.24, 0.21, 0.11, 2_000, n_star);
        assert!(ess_mult > 1.5);
    }

    #[test]
    fn simpson_paradox_and_long_context_dpi_dilution_verify_filter2_causal_pruning() {
        let simpson = SimpsonCausalParadoxAudit::canonical_pearl_benchmark();
        assert!(simpson.simpson_reversal_and_dag_dichotomy_verified);
        assert!(simpson.stratum_z0_diff > 0.05 && simpson.stratum_z1_diff > 0.04);
        assert!(simpson.pooled_diff < -0.04);
        assert!(simpson.confounder_do_ate > 0.04);
        assert!(simpson.mediator_do_ate < -0.04);
        assert!(simpson.total_probability_identity_error < 1e-12);

        let ctx = LongContextDpiDilutionAudit::evaluate(100, 3, 1, 2.2, 0.9);
        assert!(ctx.markov_blanket_pruning_approved);
        assert!(ctx.raw_blanket_attention_mass < 0.12);
        assert!(ctx.pruned_blanket_attention_mass > 0.95);
        assert!(ctx.snr_gain_decibans > 20.0);
        assert!(ctx.softmax_partition_identity_error < 1e-12);
    }
}
