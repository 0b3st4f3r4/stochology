//! Statutory Regulatory Compliance Gate (`RegulatoryComplianceAudit`, EU AI Act & LGPD) and
//! Cloud API vs. Private Local Deployment (`LoRA / QLoRA`) Architecture Audit (`CloudLocalLoraArchitectureAudit`, §4.2).

/// Statutory Regulatory Compliance Gate (`RegulatoryComplianceAudit` — EU AI Act Regulation (EU) 2024/1689
/// High-Risk Annex III Articles 9, 10, 14, 15 & Brazilian LGPD Law No. 13.709/2018 Article 20):
///
/// Maps qualitative legal mandates to quantitative, falsifiable Stochology invariants:
/// - **EU AI Act Art. 9 (Risk Management System):** Non-zero Info-Gap robustness horizon $\hat{\alpha}(q, r_c) > 0$ and approved Sniedovich Wald Maximin floor.
/// - **EU AI Act Art. 10 & LGPD Art. 6 III (Data Governance & Minimization):** Bounded Meng `DDI`, explicit Manski/Lee non-response bounds, Moulton-Campbell $n_{\mathrm{eff}}(p) \ge n_{\min}$, and mapped Markov Blanket without Berkson colliders.
/// - **EU AI Act Art. 14 & LGPD Art. 20 (Effective Human Oversight & Right to Explanation):** Shift burst cognitive load $\le 10\text{ bit/s}$ (`AlertFatigueAudit`), Turing deciban weight $W \ge +10\text{ dB}$, and integer Gigerenzer natural-frequency explanation.
/// - **EU AI Act Art. 15 & Chouldechova/KMR ($\Phi_4$) (Accuracy, Robustness & Non-Discrimination):** Isotonic $\mathrm{REL}_{\mathrm{CORP}} \le 0.02$, active `PCCP`/`PLAT` Green/Amber zone, and audited Chouldechova error-bearer declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct RegulatoryComplianceAudit {
    /// EU AI Act Art. 9 (Risk Management via Info-Gap $\hat{\alpha} > 0$ + Sniedovich Maximin floor).
    pub eu_ai_act_art9_risk_management_ok: bool,
    /// EU AI Act Art. 10 & LGPD Art. 6 III (Data Governance via Meng/Manski/Lee + $n_{\mathrm{eff}}(p)$ + Markov Blanket minimization).
    pub eu_ai_act_art10_lgpd_minimization_ok: bool,
    /// EU AI Act Art. 14 & LGPD Art. 20 (Effective Human Oversight $\le 10\text{ bit/s}$ + Gigerenzer natural-frequency explanation).
    pub eu_ai_act_art14_lgpd_art20_human_oversight_ok: bool,
    /// EU AI Act Art. 15 & $\Phi_4$ (Calibration $\mathrm{REL}_{\mathrm{CORP}} \le 0.02$ + `PCCP`/`PLAT` + Chouldechova audit).
    pub eu_ai_act_art15_robustness_fairness_ok: bool,
    /// True if all four statutory pillars pass simultaneously.
    pub all_statutory_requirements_approved: bool,
}

impl RegulatoryComplianceAudit {
    /// Evaluates EU AI Act (Arts. 9, 10, 14, 15) and LGPD (Arts. 6 III, 20) compliance from verified operational metrics.
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate(
        info_gap_alpha_hat: f64,
        sniedovich_approved: bool,
        meng_bounds_and_markov_blanket_ok: bool,
        effective_tail_n_eff: f64,
        human_oversight_bits_per_sec: f64,
        turing_decibans: f64,
        natural_frequency_explanation_attached: bool,
        rel_corp_miscalibration: f64,
        pccp_approved: bool,
        chouldechova_audited: bool,
    ) -> Self {
        let art9 = info_gap_alpha_hat > 0.0 && sniedovich_approved;
        let art10 = meng_bounds_and_markov_blanket_ok && effective_tail_n_eff >= 100.0;
        let art14 = human_oversight_bits_per_sec <= 10.0
            && turing_decibans >= 10.0
            && natural_frequency_explanation_attached;
        let art15 = rel_corp_miscalibration <= 0.02 && pccp_approved && chouldechova_audited;
        Self {
            eu_ai_act_art9_risk_management_ok: art9,
            eu_ai_act_art10_lgpd_minimization_ok: art10,
            eu_ai_act_art14_lgpd_art20_human_oversight_ok: art14,
            eu_ai_act_art15_robustness_fairness_ok: art15,
            all_statutory_requirements_approved: art9 && art10 && art14 && art15,
        }
    }
}

/// Bridge $L_2 \leftrightarrow L_3$ Cloud API vs. Private Local Deployment (`LoRA / QLoRA`) Architecture Audit
/// (`CloudLocalLoraArchitectureAudit`):
///
/// Couples two core engineering decisions from *Estrategista* (Week 4: *Unit Economics & API vs. Local Duel*)
/// and *Cientista Sábio* (Weeks 12 & 19–20: *Fine-Tuning & `LoRA / QLoRA` Private Infrastructure*):
/// 1. **Stochastic Unit Economics & Tailored Base-Surge (`TBS`) vs. Naive Break-Even:**
///    Deterministic break-even $V_{\text{naive}}^* = C_{\text{fixed}} / (c_{\text{API}} - c_{\text{local}})$
///    ignores Kingman queueing congestion ($\frac{u}{1-u}\frac{c_a^2+c_s^2}{2}$) when local GPU utilization
///    $u \to 1$. Serving base volume at $u_{\text{base}} \le 0.78$ locally and routing burst surges to API
///    (`TBS`) strictly dominates both pure-API and overloaded pure-local deployment.
/// 2. **Eckart-Young-Mirsky Spectral Truncation & Gram Effective Dimension for `LoRA` ($\Delta W = BA$):**
///    Computes the Gram effective dimension $d_{\mathrm{eff}}(G) = \frac{(\sum_i \sigma_i^2)^2}{\sum_i \sigma_i^4}$
///    of the task update singular spectrum $\sigma_1 \ge \dots \ge \sigma_d$, the relative Frobenius truncation
///    error $\varepsilon_{\mathrm{LoRA}}(r) = \frac{\sum_{i>r}\sigma_i^2}{\sum_{i=1}^d \sigma_i^2}$, and verifies
///    the exact Pythagorean energy partition $\sum_{i\le r}\sigma_i^2 + \sum_{i>r}\sigma_i^2 = \|\Delta W\|_F^2$.
#[derive(Debug, Clone, PartialEq)]
pub struct CloudLocalLoraArchitectureAudit {
    /// Naive deterministic break-even token volume (ignoring queueing delay).
    pub naive_break_even_volume: f64,
    /// Total monthly cost under pure Cloud API billing.
    pub pure_api_total_cost: f64,
    /// Total monthly cost (CapEx + OpEx + SLA congestion penalty) under overloaded pure local deployment ($u = 0.96$).
    pub overloaded_pure_local_total_cost: f64,
    /// Total monthly cost under Tailored Base-Surge (`TBS`) hybrid local + API spillover routing.
    pub tbs_hybrid_total_cost: f64,
    /// Gram effective dimension $d_{\mathrm{eff}}(G) = \frac{(\mathrm{tr}\,G)^2}{\|G\|_F^2}$ of the task update spectrum.
    pub gram_effective_dimension_d_eff: f64,
    /// Relative Frobenius energy retained by the chosen `LoRA` rank $r \ge \lceil d_{\mathrm{eff}}\rceil$ vs. rank-1 truncation.
    pub lora_retained_energy_ratio: f64,
    pub rank1_retained_energy_ratio: f64,
    /// Exact Eckart-Young Pythagorean energy identity residual $|E_{\le r} + E_{> r} - \|\Delta W\|_F^2|$.
    pub eckart_young_energy_identity_error: f64,
    /// Whether `TBS` hybrid routing beats both pure-API and overloaded pure-local cost, and `LoRA` rank $r$
    /// retains $\ge 95\%$ of the task update Frobenius energy.
    pub architecture_and_lora_approved: bool,
}

impl CloudLocalLoraArchitectureAudit {
    /// Evaluates Cloud API vs. Local GPU `LoRA` unit economics and spectral truncation at rank `lora_rank`.
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate(
        monthly_token_volume_millions: f64,
        local_fixed_monthly_cost: f64,
        local_marginal_cost_per_million: f64,
        api_cost_per_million: f64,
        burst_cv_squared_sum: f64,
        sla_delay_penalty_scale: f64,
        task_singular_values: &[f64],
        lora_rank: usize,
    ) -> Self {
        assert!(
            monthly_token_volume_millions > 0.0
                && api_cost_per_million > local_marginal_cost_per_million
                && !task_singular_values.is_empty()
                && lora_rank > 0
                && lora_rank <= task_singular_values.len(),
            "CloudLocalLoraArchitectureAudit requires valid unit economics and non-empty singular spectrum"
        );
        let unit_saving = (api_cost_per_million - local_marginal_cost_per_million).max(1e-9);
        let naive_break_even_volume = local_fixed_monthly_cost / unit_saving;
        let pure_api_total_cost = monthly_token_volume_millions * api_cost_per_million;

        // Overloaded pure-local deployment running at u = 0.96 suffers Kingman VUT queue blowup
        let u_overloaded = 0.96;
        let kingman_overloaded =
            (u_overloaded / (1.0 - u_overloaded)) * (0.5 * burst_cv_squared_sum);
        let overloaded_pure_local_total_cost = local_fixed_monthly_cost
            + monthly_token_volume_millions * local_marginal_cost_per_million
            + sla_delay_penalty_scale * kingman_overloaded;

        // Tailored Base-Surge (TBS): 78% base load on local QLoRA server (u_base = 0.75) + 22% burst surge spilled to API
        let base_share = 0.78;
        let u_base = 0.75;
        let kingman_base = (u_base / (1.0 - u_base)) * (0.5 * burst_cv_squared_sum);
        let tbs_hybrid_total_cost = local_fixed_monthly_cost
            + (monthly_token_volume_millions * base_share) * local_marginal_cost_per_million
            + (monthly_token_volume_millions * (1.0 - base_share)) * api_cost_per_million
            + sla_delay_penalty_scale * kingman_base;

        let energies: Vec<f64> = task_singular_values.iter().map(|&s| s * s).collect();
        let total_energy: f64 = energies.iter().sum();
        let sum_sq_energy: f64 = energies.iter().map(|&e| e * e).sum();
        let gram_effective_dimension_d_eff =
            (total_energy * total_energy) / sum_sq_energy.max(1e-15);

        let energy_le_r: f64 = energies.iter().take(lora_rank).sum();
        let energy_gt_r: f64 = energies.iter().skip(lora_rank).sum();
        let eckart_young_energy_identity_error = ((energy_le_r + energy_gt_r) - total_energy).abs();
        let lora_retained_energy_ratio = energy_le_r / total_energy.max(1e-15);
        let rank1_retained_energy_ratio = energies[0] / total_energy.max(1e-15);

        let approved = tbs_hybrid_total_cost < pure_api_total_cost
            && tbs_hybrid_total_cost < overloaded_pure_local_total_cost
            && lora_retained_energy_ratio >= 0.95
            && lora_retained_energy_ratio > rank1_retained_energy_ratio + 0.30
            && eckart_young_energy_identity_error < 1e-12;

        Self {
            naive_break_even_volume,
            pure_api_total_cost,
            overloaded_pure_local_total_cost,
            tbs_hybrid_total_cost,
            gram_effective_dimension_d_eff,
            lora_retained_energy_ratio,
            rank1_retained_energy_ratio,
            eckart_young_energy_identity_error,
            architecture_and_lora_approved: approved,
        }
    }

    /// Canonical Cloud API vs. Local `QLoRA` architecture benchmark.
    pub fn canonical_benchmark() -> Self {
        let spectrum = [4.0, 2.8, 1.9, 1.2, 0.45, 0.25, 0.15, 0.08];
        Self::evaluate(1_200.0, 2_400.0, 0.60, 4.50, 3.2, 140.0, &spectrum, 4)
    }
}
