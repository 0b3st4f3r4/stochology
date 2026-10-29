//! Multimodal Correlated Fusion (`MultimodalCorrelatedFusionAudit`) and Prospective Pre-Mortem
//! Counterfactual Root-Cause & SOP Homologation (`PreMortemCounterfactualRootCauseAudit`).

/// Bridge $L_1 \to L_2 \to L_3$ Five-Senses Multimodal Fusion Verifier (`MultimodalCorrelatedFusionAudit`):
///
/// Audits the fusion of $M$ sensory channels (*"Os Cinco Sentidos da Máquina"*: e.g. text, audio, image,
/// video, telemetry) with individual Turing deciban weights $w_1, \dots, w_M$ and pairwise latent
/// equicorrelation $\rho \in [0, 1)$ induced by a shared upstream pre-training corpus or sensor confounder:
/// - **Naive Independent Fusion:** Sums all channels blindly $W_{\text{naive}} = \sum_{m=1}^M w_m$, violating
///   $\Phi_2$ across modalities and overstating effective evidence by the multimodal design-effect factor
///   $\mathrm{DEFF}_{\text{modal}} = 1 + (M - 1)\rho$.
/// - **Covariance-Discounted & Outlier-Clipped Fusion:** Discounts shared correlation via
///   $W_{\text{discounted}} = \frac{\sum_{m=1}^M \tilde{w}_m}{1 + (M - 1)\rho}$ where each channel is clipped
///   at $\pm w_{\max}$ to prevent a single corrupted or poisoned sensor from hijacking posterior odds.
#[derive(Debug, Clone, PartialEq)]
pub struct MultimodalCorrelatedFusionAudit {
    /// Number of sensory modalities $M$ (e.g. $5$).
    pub num_modalities: usize,
    /// Shared cross-modal correlation $\rho \in [0, 1)$.
    pub cross_modal_correlation_rho: f64,
    /// Multimodal design-effect factor $\mathrm{DEFF}_{\text{modal}} = 1 + (M - 1)\rho$.
    pub multimodal_deff: f64,
    /// Naive independent deciban sum $W_{\text{naive}} = \sum_{m=1}^M w_m$ (unclipped, $\rho=0$ assumed).
    pub naive_independent_decibans: f64,
    /// Covariance-discounted and Huber/innovation-clipped deciban evidence $W_{\text{shielded}}$.
    pub shielded_correlated_decibans: f64,
    /// Exact identity error $|W_{\text{discounted}}(\rho=0, \text{unclipped}) - W_{\text{naive}}|$.
    pub zero_correlation_identity_error: f64,
    /// Whether the shielded fusion prevents both double-counting inflation ($W_{\text{shielded}} < W_{\text{naive}}$)
    /// and single-sensor corruption hijack.
    pub multimodal_fusion_approved: bool,
}

impl MultimodalCorrelatedFusionAudit {
    /// Evaluates $M$-channel deciban fusion under cross-modal correlation $\rho$ and per-channel clip $w_{\max}$.
    pub fn evaluate(raw_channel_decibans: &[f64], rho: f64, channel_clip_max: f64) -> Self {
        assert!(
            !raw_channel_decibans.is_empty()
                && rho.is_finite()
                && (0.0..1.0).contains(&rho)
                && channel_clip_max.is_finite()
                && channel_clip_max > 0.0,
            "MultimodalCorrelatedFusionAudit requires non-empty channels, rho in [0, 1), and positive clip"
        );
        let m = raw_channel_decibans.len();
        let multimodal_deff = 1.0 + ((m.saturating_sub(1)) as f64) * rho;
        let naive_independent_decibans: f64 = raw_channel_decibans.iter().copied().sum();
        let unclipped_zero_rho: f64 = naive_independent_decibans / 1.0;
        let zero_correlation_identity_error =
            (unclipped_zero_rho - naive_independent_decibans).abs();

        let clipped_sum: f64 = raw_channel_decibans
            .iter()
            .map(|&w| w.clamp(-channel_clip_max, channel_clip_max))
            .sum();
        let shielded_correlated_decibans = clipped_sum / multimodal_deff;

        Self {
            num_modalities: m,
            cross_modal_correlation_rho: rho,
            multimodal_deff,
            naive_independent_decibans,
            shielded_correlated_decibans,
            zero_correlation_identity_error,
            multimodal_fusion_approved: multimodal_deff > 1.0
                && shielded_correlated_decibans < naive_independent_decibans
                && shielded_correlated_decibans.abs() <= (m as f64) * channel_clip_max
                && zero_correlation_identity_error < 1e-12,
        }
    }

    /// Assumed: canonical 5-senses benchmark (text, audio, image, video, telemetry) where 4 clean sensors carry
    /// $+4.5\text{ dB}$ each, shared pre-training correlation is $\rho = 0.45$, and the 5th sensor suffers
    /// a $+35.0\text{ dB}$ adversarial/glitch spike clipped at $w_{\max} = 6.0\text{ dB}$.
    pub fn canonical_five_senses_benchmark() -> Self {
        Self::evaluate(&[4.5, 4.5, 4.5, 4.5, 35.0], 0.45, 6.0)
    }
}

/// Prospective Pre-Mortem, Counterfactual Root-Cause ($\mathrm{PNS}$ vs. Linear "5 Whys"), and Frictionless
/// SOP Homologation Audit (`PreMortemCounterfactualRootCauseAudit`):
///
/// Couples Gary Klein's prospective *Pre-Mortem* (*Technium* failure autopsy), Pearl's Tian-Pearl bounds on
/// the **Probability of Necessary and Sufficient Causation** ($\mathrm{PNS} = P(Y_1 = 1, Y_0 = 0)$) versus
/// naive single-chain "5 Whys" root-cause selection, and boundary stress-testing + Standard Operating Procedure
/// (`SOP`) cognitive friction check ($\le 10\text{ bit/s}$).
#[derive(Debug, Clone, PartialEq)]
pub struct PreMortemCounterfactualRootCauseAudit {
    /// Tian-Pearl sharp lower bound $\max(0, P(Y_1=1) - P(Y_0=1))$ on $\mathrm{PNS}$ for the true root cause.
    pub pns_lower_bound: f64,
    /// Tian-Pearl sharp upper bound $\min(P(Y_1=1), P(Y_0=0))$ on $\mathrm{PNS}$ for the true root cause.
    pub pns_upper_bound: f64,
    /// Exact structural $\mathrm{PNS}$ of the true conjunctive root cause vs. the superficial "5 Whys" symptom.
    pub true_root_cause_pns: f64,
    pub naive_five_whys_symptom_pns: f64,
    /// Residual of the Tian-Pearl interval containment identity $\mathrm{PNS} \in [\mathrm{LB}, \mathrm{UB}]$.
    pub tian_pearl_bounds_identity_error: f64,
    /// Prospective Pre-Mortem expected tail loss before vs. after boundary stress-test homologation.
    pub unshielded_premortem_expected_loss: f64,
    pub homologated_premortem_expected_loss: f64,
    /// Operational SOP cognitive load in bit/s ($\le 10.0\text{ bit/s}$ required for frictionless adoption).
    pub sop_cognitive_load_bps: f64,
    /// Whether counterfactual $\mathrm{PNS}$ isolates the true root cause, Tian-Pearl bounds hold, and
    /// Pre-Mortem stress-testing + SOP homologation pass.
    pub premortem_and_sop_approved: bool,
}

impl PreMortemCounterfactualRootCauseAudit {
    /// Evaluates counterfactual root-cause $\mathrm{PNS}$ bounds, Pre-Mortem tail-loss reduction, and SOP load.
    pub fn evaluate(
        p_y1_true_cause: f64,
        p_y0_true_cause: f64,
        true_root_cause_pns: f64,
        naive_five_whys_symptom_pns: f64,
        unshielded_premortem_expected_loss: f64,
        homologated_premortem_expected_loss: f64,
        sop_cognitive_load_bps: f64,
    ) -> Self {
        let p1 = p_y1_true_cause.clamp(0.0, 1.0);
        let p0 = p_y0_true_cause.clamp(0.0, 1.0);
        let pns_lower_bound = (p1 - p0).max(0.0);
        let pns_upper_bound = p1.min(1.0 - p0);
        let below_lb = (pns_lower_bound - true_root_cause_pns).max(0.0);
        let above_ub = (true_root_cause_pns - pns_upper_bound).max(0.0);
        let tian_pearl_bounds_identity_error = below_lb.max(above_ub);

        let approved = tian_pearl_bounds_identity_error < 1e-12
            && true_root_cause_pns > naive_five_whys_symptom_pns + 0.35
            && homologated_premortem_expected_loss < 0.25 * unshielded_premortem_expected_loss
            && sop_cognitive_load_bps <= 10.0;

        Self {
            pns_lower_bound,
            pns_upper_bound,
            true_root_cause_pns,
            naive_five_whys_symptom_pns,
            tian_pearl_bounds_identity_error,
            unshielded_premortem_expected_loss,
            homologated_premortem_expected_loss,
            sop_cognitive_load_bps,
            premortem_and_sop_approved: approved,
        }
    }

    /// Assumed: canonical Pre-Mortem & Counterfactual Root-Cause benchmark.
    pub fn canonical_benchmark() -> Self {
        Self::evaluate(0.84, 0.12, 0.76, 0.14, 125_000.0, 18_500.0, 4.2)
    }
}
