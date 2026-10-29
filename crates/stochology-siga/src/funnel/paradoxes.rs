//! Filter 2 Causal Resolution of Simpson's Paradox (`SimpsonCausalParadoxAudit`) and
//! Long-Context Attention Dilution vs. Markov Blanket Sufficiency (`LongContextDpiDilutionAudit`).

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

    /// Informative: canonical kidney-stone / clinical triage benchmark transcribed from Charig et al. (1986, *BMJ* 292:879–882)
    /// and Pearl (2009, *Causality*, Table 6.1): $N = 700$ patients ($350$ per arm), where $Z=0$ is small stones and $Z=1$ is large stones.
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
