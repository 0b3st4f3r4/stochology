//! Executive Memoranda in Gigerenzer Integer Natural Frequencies and Bridge $L_2 \to L_3$
//! Cognitive & Clinical Triage Verification (`AlertFatigueAudit`, §3.2 Step 8).

/// Executive Memorandum in Integer Natural Frequencies (Gigerenzer — §3.2 Step 8).
#[derive(Debug, Clone, PartialEq)]
pub struct NaturalFrequencyMemorandum {
    pub reference_cohort_size: usize,
    pub affected_with_event: usize,
    pub unaffected_without_event: usize,
    pub true_positives: usize,
    pub false_negatives: usize,
    pub false_positives: usize,
    pub true_negatives: usize,
    pub critical_threshold_p_star: f64,
    pub recommended_action: String,
    pub exit_clause: String,
    pub formatted_summary: String,
}

impl NaturalFrequencyMemorandum {
    /// Builds an integer natural frequency tree for a reference cohort (e.g., `1_000` units).
    pub fn build(
        reference_cohort_size: usize,
        base_rate_p: f64,
        sensitivity_tpr: f64,
        false_positive_rate_fpr: f64,
    ) -> Self {
        Self::build_with_decision(
            reference_cohort_size,
            base_rate_p,
            sensitivity_tpr,
            false_positive_rate_fpr,
            0.05,
            "protect",
            "Suspend if REL_CORP exceeds threshold",
        )
    }

    /// Builds an integer natural frequency tree including the economic threshold $p^*$,
    /// recommended action (`protect` / `do_not_protect`), and numeric exit clause.
    pub fn build_with_decision(
        reference_cohort_size: usize,
        base_rate_p: f64,
        sensitivity_tpr: f64,
        false_positive_rate_fpr: f64,
        critical_threshold_p_star: f64,
        recommended_action: &str,
        exit_clause: &str,
    ) -> Self {
        let p_c = if base_rate_p.is_finite() {
            base_rate_p.clamp(0.0, 1.0)
        } else {
            0.0
        };
        let tpr_c = if sensitivity_tpr.is_finite() {
            sensitivity_tpr.clamp(0.0, 1.0)
        } else {
            0.0
        };
        let fpr_c = if false_positive_rate_fpr.is_finite() {
            false_positive_rate_fpr.clamp(0.0, 1.0)
        } else {
            0.0
        };
        let affected = ((reference_cohort_size as f64) * p_c).round() as usize;
        let unaffected = reference_cohort_size.saturating_sub(affected);
        let tp = ((affected as f64) * tpr_c).round() as usize;
        let fn_c = affected.saturating_sub(tp);
        let fp = ((unaffected as f64) * fpr_c).round() as usize;
        let tn = unaffected.saturating_sub(fp);
        let total_alarms = tp.saturating_add(fp).max(1);
        let summary = format!(
            "Out of every {reference_cohort_size} independent units, {affected} experience the critical event (of which {tp} trigger the alarm at threshold p*={critical_threshold_p_star:.5} and {fn_c} are missed), while {unaffected} do not experience the event (of which {fp} trigger a false alarm and {tn} remain clear). Positive predictive frequency: {tp} out of {total_alarms} alarms. Action: {recommended_action}. Exit clause: {exit_clause}."
        );
        Self {
            reference_cohort_size,
            affected_with_event: affected,
            unaffected_without_event: unaffected,
            true_positives: tp,
            false_negatives: fn_c,
            false_positives: fp,
            true_negatives: tn,
            critical_threshold_p_star,
            recommended_action: recommended_action.to_string(),
            exit_clause: exit_clause.to_string(),
            formatted_summary: summary,
        }
    }
}

/// Provenance of the $(\mathrm{TPR}, \mathrm{FPR})$ operating point fed into `AlertFatigueAudit` (§II.C).
///
/// A `Literature` operating point is a published external validation transcribed from its source (informative,
/// not computed by this code); an `Assumed` operating point is a hand-set design target and must never be
/// presented as a result.
#[derive(Debug, Clone, PartialEq)]
pub enum OperatingPointSource {
    /// Informative: published external validation transcribed from the literature, not computed by this code.
    Literature { source: &'static str },
    /// Hand-set design operating point (Assumed: not measured on any dataset).
    Assumed,
}

/// Bridge $L_2 \to L_3$ Cognitive & Clinical Triage Verifier (`AlertFatigueAudit`):
///
/// Couples the biological conscious bottleneck of $10\text{ bit/s}$ (Zheng & Meister 2024/2025, *Neuron*),
/// Alan Turing & I. J. Good's additive weight of evidence in decibans ($W = 10\log_{10}\mathrm{LR}$, where
/// $1\text{ dB}$ is the minimum human-perceptible quantum and $+30\text{ dB}$ is the $1000:1$ Bombe threshold),
/// and Positive Predictive Value ($\mathrm{PPV}$) at the operational shift resolution ($\Phi_5$).
///
/// Rejects high-false-alarm black-box scores exemplified by the Epic Sepsis Model external validation
/// (Wong et al., *JAMA Internal Medicine* 2021: $n = 27\,697$ patients, $38\,455$ hospitalizations,
/// $\mathrm{AUC} = 0.63$, sensitivity $\mathrm{TPR} = 0.33$, $\mathrm{PPV} = 0.12$, $18\%$ hospitalization
/// alarm rate, generating $\ge 7.3$ false alarms per true case).
#[derive(Debug, Clone, PartialEq)]
pub struct AlertFatigueAudit {
    /// Conscious executive action-selection bandwidth ceiling ($10.0\text{ bit/s}$, Zheng & Meister 2024/2025).
    pub conscious_bottleneck_bps: f64,
    /// Minimum human-perceptible weight-of-evidence quantum ($1.0\text{ deciban}$, Good 1979).
    pub minimum_perceptible_decibans: f64,
    /// Additive Turing-Good weight of evidence $W(H:E) = 10\log_{10}(\mathrm{TPR}/\mathrm{FPR})$ in decibans.
    pub turing_weight_of_evidence_decibans: f64,
    /// Posterior log-odds in decibans $O_{\text{post, dB}} = 10\log_{10}\frac{\pi}{1-\pi} + W(H:E)$.
    pub posterior_log_odds_decibans: f64,
    /// Positive Predictive Value $\mathrm{PPV} = \frac{\pi \cdot \mathrm{TPR}}{\pi \cdot \mathrm{TPR} + (1 - \pi)\cdot \mathrm{FPR}}$.
    pub positive_predictive_value_ppv: f64,
    /// False-to-true alarm ratio $\frac{\mathrm{FP}}{\mathrm{TP}} = \frac{1 - \mathrm{PPV}}{\mathrm{PPV}}$.
    pub false_alarms_per_true_case: f64,
    /// Information-theoretic cognitive load (in bit/s) imposed on the shift operator during burst triage.
    pub cognitive_triage_load_bps: f64,
    /// Whether the system satisfies $\Phi_5$ shift resolution, $W \ge 8.0\text{ dB}$, $\mathrm{PPV} \ge p^*$,
    /// false-to-true ratio $\le 4.0$, and stays below the $10\text{ bit/s}$ conscious cognitive ceiling.
    pub approved_bridge_l2_l3: bool,
    /// Provenance of the $(\mathrm{TPR}, \mathrm{FPR})$ operating point (`Assumed` unless declared otherwise
    /// via `with_operating_point_source`).
    pub operating_point_source: OperatingPointSource,
}

impl AlertFatigueAudit {
    /// Evaluates an operational alerting rule at shift resolution (`seed = 2026`).
    ///
    /// The operating point defaults to `OperatingPointSource::Assumed`; declare a literature operating point
    /// with `with_operating_point_source`.
    pub fn evaluate(
        prevalence_pi: f64,
        sensitivity_tpr: f64,
        false_positive_rate_fpr: f64,
        critical_threshold_p_star: f64,
        alarms_per_minute_burst: f64,
        bits_per_alarm_inspection: f64,
        respects_phi5_shift_resolution: bool,
    ) -> Self {
        let conscious_bottleneck_bps = 10.0;
        let minimum_perceptible_decibans = 1.0;

        let pi = prevalence_pi.clamp(1e-6, 1.0 - 1e-6);
        let tpr = sensitivity_tpr.clamp(1e-6, 1.0 - 1e-6);
        let fpr = false_positive_rate_fpr.clamp(1e-6, 1.0 - 1e-6);

        let lr_pos = tpr / fpr;
        let turing_weight_of_evidence_decibans = 10.0 * lr_pos.log10();
        let prior_decibans = 10.0 * (pi / (1.0 - pi)).log10();
        let posterior_log_odds_decibans = prior_decibans + turing_weight_of_evidence_decibans;

        let tp_rate = pi * tpr;
        let fp_rate = (1.0 - pi) * fpr;
        let positive_predictive_value_ppv = tp_rate / (tp_rate + fp_rate).max(1e-12);
        let false_alarms_per_true_case =
            (1.0 - positive_predictive_value_ppv) / positive_predictive_value_ppv.max(1e-12);

        let cognitive_triage_load_bps =
            (alarms_per_minute_burst * bits_per_alarm_inspection) / 60.0;

        let approved_bridge_l2_l3 = respects_phi5_shift_resolution
            && turing_weight_of_evidence_decibans >= 8.0
            && positive_predictive_value_ppv >= critical_threshold_p_star
            && false_alarms_per_true_case <= 4.0
            && cognitive_triage_load_bps <= conscious_bottleneck_bps;

        Self {
            conscious_bottleneck_bps,
            minimum_perceptible_decibans,
            turing_weight_of_evidence_decibans,
            posterior_log_odds_decibans,
            positive_predictive_value_ppv,
            false_alarms_per_true_case,
            cognitive_triage_load_bps,
            approved_bridge_l2_l3,
            operating_point_source: OperatingPointSource::Assumed,
        }
    }

    /// Declares the provenance of the operating point without altering any computed quantity.
    pub fn with_operating_point_source(mut self, source: OperatingPointSource) -> Self {
        self.operating_point_source = source;
        self
    }

    /// Canonical comparison between the failed Epic Sepsis Model (Wong et al., *JAMA Internal Medicine* 2021:
    /// $\mathrm{AUC} = 0.63$, $\mathrm{TPR} = 0.33$, $\mathrm{PPV} = 0.12$, $18\%$ hospitalization alarm rate,
    /// $\approx 7.33$ false alarms per true case) and a CORP-calibrated Stochological triage gate.
    ///
    /// The two sides are not symmetric evidence: the Epic operating point is a published external validation,
    /// whereas the Stochological operating point ($\mathrm{TPR} = 0.82$, $\mathrm{FPR} = 0.045$) is an *assumed
    /// design target* that was never measured on the Wong et al. cohort. The comparison verifies that the audit
    /// rejects the published operating point and accepts one meeting its thresholds; it is not a performance claim.
    pub fn canonical_epic_sepsis_vs_stochological_gate() -> (Self, Self) {
        // Epic Sepsis Model (Wong et al. 2021, published external validation): prevalence ~6.6%, TPR = 0.33, FPR = 0.171 -> PPV = 0.120, W = +2.86 dB
        let epic_sepsis = Self::evaluate(0.066, 0.33, 0.171, 0.20, 18.0, 42.0, false)
            .with_operating_point_source(OperatingPointSource::Literature {
                source: "Wong et al., JAMA Internal Medicine 2021",
            });
        // Stochological CORP-calibrated multi-signal gate (3 independent channels summing deciban weights), ASSUMED operating point:
        // TPR = 0.82, FPR = 0.045 -> LR+ = 18.22 (W = +12.61 dB), PPV = 0.563 >= p* = 0.20, cognitive load = 3.2 bit/s <= 10 bit/s
        let stochological_gate = Self::evaluate(0.066, 0.82, 0.045, 0.20, 4.0, 48.0, true)
            .with_operating_point_source(OperatingPointSource::Assumed);
        (epic_sepsis, stochological_gate)
    }
}
