//! **Block I (Filter I: From World to Archive · Axis 1: Epistemic Margin $S_{\mathrm{epi}}$ · Chapters 2–4).**
//!
//! Unifies all epistemic-margin operators across **Chapter 2** (Inquiry, Framing, Cromwell & Multiverse),
//! **Chapter 3** (Underdetermination, Partial Identification & Anytime-Valid *e*-Processes), and
//! **Chapter 4** (From World to Archive, Effective Sample Size $n_{\mathrm{eff}}$ & Big Data Paradox)
//! into **Canonical Operator I** and the Type-State structure [`EpistemicMarginState`]:
//!
//! 1. **Unified Effective Sample Size ($n_{\mathrm{eff}}$) and Keynes's Evidential Weight ($V$):**
//!    $$\frac{1}{n_{\mathrm{eff}}} = \frac{\mathrm{DEFF}_{\mathrm{total}}}{n} + \left(\frac{1 - f}{f}\right)\rho_{R,Y}^2, \qquad V(n_{\mathrm{eff}}) = \frac{n_{\mathrm{eff}}}{n_{\mathrm{eff}} + n_0}$$
//! 2. **Fréchet–Manski Partial Identification Functional ($[L_t, U_t]$) and Cromwell–Ville *e*-Process ($E_t$):**
//!    $$[L_t, U_t] = \left[\inf_{P \in \mathcal{P}_t(n_{\mathrm{eff}}, \pi_{\mathrm{miss}})} \mathbb{E}_P[g(Y)],\; \sup_{P \in \mathcal{P}_t(n_{\mathrm{eff}}, \pi_{\mathrm{miss}})} \mathbb{E}_P[g(Y)]\right], \qquad E_t = \prod_{s=1}^t e_s$$

use crate::kernel::{
    cromwell_clamp, inverse_standard_normal_cdf, mean, median, quantile, sample_std,
    sample_variance, standard_normal_cdf,
};
use std::collections::BTreeMap;

/// Classification of organizational premise-setting under bounded rationality (Herbert Simon, 1947/1997;
/// Jürgen Habermas, 1984; Immanuel Kant, 1785).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CognitiveFramingRegime {
    /// **Legitimate Cognitive Influence (*Communicative Action*):** structures the operator's finite
    /// attention channel ($\le 10\text{ bits/s}$) while transparently disclosing the full partial-identification
    /// interval $[L_t, U_t]$ and the unmitigated spillover harm borne by affected citizens.
    #[default]
    CommunicativeInfluence,
    /// **Cognitive Manipulation (*Strategic Premise-Rigging / Rigged Labyrinth at the Top*):**
    /// conceals identification uncertainty or spillover costs in the decision premises while preserving
    /// the formal illusion of frontline autonomy so blame can be shifted downward if the outcome fails.
    StrategicPremiseRigging,
}

/// Bitemporal observation record storing both the real-world event timestamp $t_{\mathrm{event}}$
/// and the administrative availability / publication vintage $t_{\mathrm{available}} \ge t_{\mathrm{event}}$.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BitemporalRecord {
    /// Time when the physical or social event occurred ($t_{\mathrm{event}}$).
    pub event_time: u64,
    /// Time when the record became available in the archive ($t_{\mathrm{available}}$).
    pub available_time: u64,
    /// Recorded value at this vintage.
    pub value: f64,
}

impl BitemporalRecord {
    /// True if the record respects causal bitemporal order ($t_{\mathrm{available}} \ge t_{\mathrm{event}}$).
    #[inline]
    pub const fn is_causally_valid(&self) -> bool {
        self.available_time >= self.event_time
    }
}

/// Filters a slice of bitemporal records so that a decision at `decision_time` sees only records
/// with `available_time <= decision_time`, returning the latest available vintage per `event_time`.
pub fn bitemporal_view_at(
    records: &[BitemporalRecord],
    decision_time: u64,
) -> Result<Vec<BitemporalRecord>, &'static str> {
    for r in records {
        if !r.is_causally_valid() {
            return Err(
                "Bitemporal violation: available_time < event_time (future look-ahead leak)",
            );
        }
    }
    let mut latest_by_event: BTreeMap<u64, BitemporalRecord> = BTreeMap::new();
    for &r in records {
        if r.available_time <= decision_time {
            match latest_by_event.get(&r.event_time) {
                Some(prev) if prev.available_time > r.available_time => {}
                _ => {
                    latest_by_event.insert(r.event_time, r);
                }
            }
        }
    }
    Ok(latest_by_event.into_values().collect())
}

/// Wang & Ramdas (2022) *e-BH* procedure for False Discovery Rate (FDR) control at level $\alpha$
/// across $K$ tested hypotheses or specifications under arbitrary dependence.
pub fn e_bh_discoveries(e_values: &[f64], alpha: f64) -> Vec<usize> {
    let alpha = alpha.clamp(1e-12, 1.0 - 1e-12);
    if e_values.is_empty() {
        return Vec::new();
    }
    let k = e_values.len();
    let mut indexed: Vec<(usize, f64)> = e_values.iter().copied().enumerate().collect();
    indexed.sort_by(|a, b| b.1.total_cmp(&a.1));

    let mut max_rank = 0usize;
    for (rank_zero, &(_, e_val)) in indexed.iter().enumerate() {
        let rank = rank_zero + 1;
        let threshold = (k as f64) / ((rank as f64) * alpha);
        if e_val >= threshold {
            max_rank = rank;
        }
    }
    let mut discoveries: Vec<usize> = indexed[..max_rank].iter().map(|&(idx, _)| idx).collect();
    discoveries.sort_unstable();
    discoveries
}

/// Anytime-valid confidence sequence half-width $r_t(\alpha)$ (Howard et al., 2021; Ramdas et al., 2023)
/// for the running mean of observations with standard deviation $\sigma$ at step $t \ge 1$.
pub fn anytime_valid_radius(sigma: f64, step_t: usize, alpha: f64) -> f64 {
    let sig = sigma.max(0.0);
    let t = step_t.max(1) as f64;
    let a = alpha.clamp(1e-9, 0.5);
    let rho2 = 0.05_f64;
    let tr2 = t * rho2 + 1.0;
    let log_term = (tr2.sqrt() / a).ln().max(0.0);
    sig * ((2.0 * tr2 / (t * t * rho2)) * log_term).sqrt()
}

/// Lakatos (1970) problemshift ratio $E_t / (K + 1)$, penalizing cumulative evidence $E_t$
/// by the number of *ad hoc* auxiliary adjustments $K$ added to rescue the hypothesis.
#[inline]
pub fn lakatos_shift_ratio(terminal_e_value: f64, ad_hoc_adjustments_k: usize) -> f64 {
    terminal_e_value / ((ad_hoc_adjustments_k + 1) as f64)
}

/// Pauker & Kassirer (1975, 1980) clinical decision threshold: act when the pre-test probability of
/// the condition exceeds $p^* = C_{\mathrm{FP}} / (C_{\mathrm{FP}} + L_{\mathrm{FN}})$, where
/// $C_{\mathrm{FP}}$ is the net cost of a false alarm and $L_{\mathrm{FN}}$ the net loss of a missed
/// event. This complements the cost-loss threshold $p^* = C/(\eta L)$ in [`EpistemicMarginState`].
#[inline]
pub fn pauker_kassirer_threshold(cost_false_positive: f64, loss_false_negative: f64) -> f64 {
    let c = cost_false_positive.max(0.0);
    let l = loss_false_negative.max(0.0);
    if c + l <= 1e-15 {
        return 0.5;
    }
    (c / (c + l)).clamp(0.0, 1.0)
}

/// Canonical reference sample size $n_0 = 50.0$ for Keynes's evidential weight $V(n_{\mathrm{eff}}) = \frac{n_{\mathrm{eff}}}{n_{\mathrm{eff}} + n_0}$,
/// anchored to the two-arm design benchmark ($n_0 = 50$ independent effective units).
pub const CANONICAL_KEYNES_N0: f64 = 50.0;

/// **Canonical Type-State Structure I — `EpistemicMarginState` ($S_{\mathrm{epi}}$).**
///
/// Encapsulates **Canonical Operator I** (Effective Information Law & Anytime-Valid Partial Identification),
/// preventing uncalibrated point estimates from entering capacity sizing without their Fréchet–Manski
/// partial-identification interval $[L_t, U_t]$, unified effective sample size $n_{\mathrm{eff}}$,
/// Keynes evidential weight $V \in [0, 1)$, and Cromwell–Ville *e*-process evidence $E_t$.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct EpistemicMarginState {
    /// Fréchet–Manski / Lee / Peterson partial-identification bounds $[L_t, U_t]$.
    pub bounds: [f64; 2],
    /// Imbens–Manski (2004) finite-sample confidence interval $[L_{\mathrm{IM}}(\alpha), U_{\mathrm{IM}}(\alpha)]$.
    pub imbens_manski_bounds: [f64; 2],
    /// Irreducible partial-identification width $\Delta_{\mathrm{id}} = U_t - L_t \ge 0$.
    pub identification_width: f64,
    /// Unified MSE-effective sample size $n_{\mathrm{eff}}$ from the Meng–Moulton–Kish harmonic operator:
    /// $\frac{1}{n_{\mathrm{eff}}} = \frac{\mathrm{DEFF}_{\mathrm{total}}}{n} + \left(\frac{1-f}{f}\right)\rho_{R,Y}^2$.
    pub n_eff: f64,
    /// Variance-only effective sample size $n_{\mathrm{eff}}^{\mathrm{var}} = n / \mathrm{DEFF}_{\mathrm{total}}$
    /// (used for Imbens–Manski sampling SE and Baker–McHale Kelly shrinkage).
    pub n_eff_variance: f64,
    /// Total design effect $\mathrm{DEFF}_{\mathrm{total}} = \mathrm{DEFF}_{\mathrm{cluster}} \cdot \mathrm{DEFF}_{\mathrm{Kish}} \ge 1$.
    pub design_effect_deff: f64,
    /// Meng (2018) non-random selection error $\bar{Y}_n - \bar{Y}_N = \rho_{R,Y}\sqrt{(1-f)/f}\,\sigma_Y$.
    pub meng_estimation_error: f64,
    /// Keynes (1921) evidential weight $V(n_{\mathrm{eff}}) = \frac{n_{\mathrm{eff}}}{n_{\mathrm{eff}} + n_0} \in [0, 1)$.
    pub keynes_weight: f64,
    /// Canonical normalized Epistemic Margin score $S_{\mathrm{epi}} \in [0, 1]$.
    pub epistemic_margin_score: f64,
    /// Cromwell–Ville anytime-valid *e*-value $E_t = \prod_{s=1}^t e_s$ (penalized for multiplicity $K$).
    pub evidence_e_value: f64,
    /// Turing–Good weight of evidence in decibans $W_t = 10\log_{10}(E_t)$.
    pub decibans: f64,
    /// Cost-loss action threshold $p^* = \frac{C}{\eta L} \in (0, 1)$ (Murphy 1973; Harding 1991).
    pub threshold_p_star: f64,
    /// Expected Value of Perfect Information $\mathrm{EVPI}(\pi_0)$.
    pub evpi: f64,
    /// Mayo (2018) severe statistical power $\mathbb{P}(\text{reject } H_0 \mid \delta = \delta_{\min})$ at $n_{\mathrm{eff}}$.
    pub severe_power: f64,
    /// Simon–Habermas–Kant cognitive framing regime (`CommunicativeInfluence` vs. `StrategicPremiseRigging`).
    pub framing_regime: CognitiveFramingRegime,
    /// Premise Rigging Index $\mathrm{PRI} = \max(c_{\mathrm{epi}}, c_{\mathrm{spill}}) \in [0, 1]$ (cutoff $0.20$).
    pub pri: f64,
    /// Net global utility $\Delta U_{\mathrm{global}} = \Delta U_{\mathrm{focal}} - D_{\mathrm{spillover}}$ in Type III audit.
    pub global_net_utility: f64,
    /// True if a Kimball–Mitroff Type III framing error is present ($\Delta U_{\mathrm{focal}} > 0$ yet $\Delta U_{\mathrm{global}} < 0$).
    pub is_type_three_error: bool,
    /// Multiverse specification curve consistency ratio across $K$ specifications (Steegen et al., 2016).
    pub multiverse_consistency: f64,
    /// Number of specifications surviving Wang–Ramdas *e*-BH FDR control at level $\alpha$.
    pub e_bh_discoveries_count: usize,
    /// Lincoln–Petersen / Chao (1987) estimated uncounted tail population $\hat{n}_{00}$.
    pub uncounted_tail_n00: f64,
    /// Chao (1987) heterogeneity-robust lower bound on total population $\hat{N}_{\mathrm{Chao}}$.
    pub chao_lower_bound_total: f64,
    /// Number of bitemporal macro quarters audited in `rtdsm_routput_vintages.csv`.
    pub vintage_num_quarters: usize,
    /// Mean absolute revision (in annualized percentage points) between initial and final vintage.
    pub vintage_mean_revision_pp: f64,
    /// Maximum absolute revision (in annualized percentage points) between initial and final vintage.
    pub vintage_max_revision_pp: f64,
    /// Fraction of quarters where growth sign flipped between initial release and final revision.
    pub vintage_sign_flip_rate: f64,
}

impl EpistemicMarginState {
    /// **Canonical Operator I.A — Unified Effective Sample Size ($n_{\mathrm{eff}}$):**
    /// $$\frac{1}{n_{\mathrm{eff}}} = \frac{\mathrm{DEFF}_{\mathrm{total}}}{n} + \left(\frac{1 - f}{f}\right)\rho_{R,Y}^2$$
    /// Unifies Meng's Big Data Paradox ($\rho_{R,Y}$), Campbell–Moulton cluster correlation ($\mathrm{DEFF}_{\mathrm{cluster}}$),
    /// and Kish's unequal weighting ($\mathrm{DEFF}_{\mathrm{Kish}}$) into a single harmonic precision law.
    pub fn unified_effective_n(
        sample_n: f64,
        population_n: f64,
        deff_total: f64,
        rho_ry: f64,
    ) -> f64 {
        if ![sample_n, population_n, deff_total, rho_ry]
            .iter()
            .all(|x| x.is_finite())
            || sample_n <= 0.0
            || population_n <= 0.0
            || deff_total <= 0.0
        {
            return 0.0;
        }
        let n = sample_n.max(1.0);
        let deff = deff_total.max(1.0);
        let pop_n = population_n.max(n);
        let f = (n / pop_n).clamp(1e-12, 1.0);
        let drop_ratio = ((1.0 - f) / f).max(0.0);
        let inv_n_eff = (deff / n) + drop_ratio * (rho_ry * rho_ry);
        (1.0 / inv_n_eff.max(1e-15)).min(n / deff)
    }

    /// **Canonical Operator I.A — Keynes's Evidential Weight $V(n_{\mathrm{eff}})$:**
    /// $$V(n_{\mathrm{eff}}) = \frac{n_{\mathrm{eff}}}{n_{\mathrm{eff}} + n_0} \in [0, 1)$$
    #[inline]
    pub fn keynes_weight_from_n_eff(n_eff: f64, reference_n0: f64) -> f64 {
        if !n_eff.is_finite() || !reference_n0.is_finite() || n_eff <= 0.0 || reference_n0 <= 0.0 {
            return 0.0;
        }
        n_eff / (n_eff + reference_n0)
    }

    /// **Canonical Functional for Axis 1 ($S_{\mathrm{epi}} \in [0, 1]$):**
    /// $$S_{\mathrm{epi}} = \sqrt{V(n_{\mathrm{eff}})} \cdot \bigl(1 - 0.50\,\tilde{u}\bigr) \cdot I_{\mathrm{frame}}$$
    /// Combines Keynes's evidential weight $V(n_{\mathrm{eff}}) \in [0, 1)$ over $n_{\mathrm{eff}}$ with total relative
    /// uncertainty $\tilde{u} = \tilde{w}_{\mathrm{id}} + (1 - \tilde{w}_{\mathrm{id}})\,\tilde{\epsilon}_{\mathrm{obs}} \in [0, 1]$.
    #[inline]
    pub fn canonical_score(
        keynes_weight: f64,
        relative_sampling_noise: f64,
        framing_integrity: f64,
    ) -> f64 {
        if ![keynes_weight, relative_sampling_noise, framing_integrity]
            .iter()
            .all(|x| x.is_finite())
        {
            return 0.0;
        }
        let v = keynes_weight.clamp(0.0, 1.0).sqrt();
        let w_noise = relative_sampling_noise.clamp(0.0, 1.0);
        let f = framing_integrity.clamp(0.0, 1.0);
        (v * (1.0 - 0.50 * w_noise) * f).clamp(0.0, 1.0)
    }

    /// **Chapter 2 Operator — Framing, Cost-Loss Threshold, Severe Power, and Type III / Simon Premise Audit.**
    #[allow(clippy::too_many_arguments)]
    pub fn from_inquiry_framing(
        cost_c: f64,
        efficiency_eta: f64,
        loss_l: f64,
        base_rate_pi: f64,
        min_relevant_harm_delta: f64,
        std_dev_sigma: f64,
        effective_n: f64,
        alpha_one_sided: f64,
        focal_gain: f64,
        spillover_damage: f64,
        true_epistemic_width: f64,
        displayed_epistemic_width: f64,
        displayed_spillover_damage: f64,
    ) -> Self {
        if ![
            cost_c,
            efficiency_eta,
            loss_l,
            base_rate_pi,
            min_relevant_harm_delta,
            std_dev_sigma,
            effective_n,
            alpha_one_sided,
            focal_gain,
            spillover_damage,
            true_epistemic_width,
            displayed_epistemic_width,
            displayed_spillover_damage,
        ]
        .iter()
        .all(|x| x.is_finite())
        {
            return Self::default();
        }
        let c_pos = cost_c.max(1e-9);
        let eta = efficiency_eta.clamp(1e-6, 1.0);
        let l_pos = loss_l.max(1e-9);
        let eff_loss = eta * l_pos;
        let p_star = (c_pos / eff_loss).clamp(1e-9, 1.0 - 1e-9);
        let pi = cromwell_clamp(base_rate_pi, 1e-4);
        let risk_prior = c_pos.min(pi * eff_loss);
        let risk_perfect = pi * c_pos.min(eff_loss);
        let evpi = (risk_prior - risk_perfect).max(0.0);

        let n_eff = effective_n.max(1.0);
        let alpha_c = alpha_one_sided.clamp(1e-6, 0.49);
        let z_alpha = inverse_standard_normal_cdf(1.0 - alpha_c);
        let se_frame = std_dev_sigma.max(1e-9) / n_eff.sqrt();
        let signal_to_noise = min_relevant_harm_delta / se_frame;
        let power = 1.0 - standard_normal_cdf(z_alpha - signal_to_noise);
        let global_net = focal_gain - spillover_damage;
        let is_type3 = focal_gain > 0.0 && global_net < 0.0;

        let w_true = true_epistemic_width.max(0.0);
        let w_disp = displayed_epistemic_width.max(0.0);
        let c_epi = if w_true <= 1e-12 {
            0.0
        } else {
            1.0 - (w_disp / w_true).clamp(0.0, 1.0)
        };
        let d_true = spillover_damage.max(0.0);
        let d_disp = displayed_spillover_damage.max(0.0);
        let c_spill = if d_true <= 1e-12 {
            0.0
        } else {
            1.0 - (d_disp / d_true).clamp(0.0, 1.0)
        };
        let pri = c_epi.max(c_spill);
        let regime = if pri <= 0.20 {
            CognitiveFramingRegime::CommunicativeInfluence
        } else {
            CognitiveFramingRegime::StrategicPremiseRigging
        };
        let v_keynes = Self::keynes_weight_from_n_eff(n_eff, CANONICAL_KEYNES_N0);
        let c_im = Self::imbens_manski_critical_value(w_true, se_frame, alpha_c);
        let im_lo = -c_im * se_frame;
        let im_hi = w_true + c_im * se_frame;
        let rel_width = (1.0 - power).clamp(0.0, 1.0);

        Self {
            bounds: [0.0, w_true],
            imbens_manski_bounds: [im_lo, im_hi],
            identification_width: w_true,
            n_eff,
            n_eff_variance: n_eff,
            design_effect_deff: 1.0,
            keynes_weight: v_keynes,
            epistemic_margin_score: Self::canonical_score(v_keynes, rel_width, 1.0 - pri),
            threshold_p_star: p_star,
            evpi,
            severe_power: power,
            framing_regime: regime,
            pri,
            global_net_utility: global_net,
            is_type_three_error: is_type3,
            ..Self::default()
        }
    }

    /// Computes the Longino–Harding framing gap $|p^*_{\mathrm{inst}} - p^*_{\mathrm{affected}}|$ between two states.
    #[inline]
    pub fn framing_gap(&self, other: &Self) -> f64 {
        (self.threshold_p_star - other.threshold_p_star).abs()
    }

    /// **Chapter 2 Operator — Multiverse Specification Curve (Steegen et al., 2016) & Wang–Ramdas *e*-BH.**
    ///
    /// Multiplicity $K$ (*garden of forking paths*) is penalized both via the Wang–Ramdas *e*-BH FDR survival
    /// rate $s_{\mathrm{e\text{-}BH}} = |\mathcal{D}_{\mathrm{e\text{-}BH}}| / K$ and via the Lakatos/Steegen
    /// specification-inflation penalty $\frac{1}{1 + 0.25 \ln K}$, so duplicating specifications without stronger
    /// evidence strictly reduces $S_{\mathrm{epi}}$.
    pub fn from_multiverse(effects: &[f64], e_values: &[f64], alpha: f64) -> Self {
        let k = effects.len().min(e_values.len());
        if k == 0 {
            return Self::default();
        }
        let effects = &effects[..k];
        let e_values = &e_values[..k];
        let med = median(effects);
        let std_spec = if k >= 2 { sample_std(effects) } else { 0.0 };
        let sign = if med >= 0.0 { 1.0 } else { -1.0 };
        let threshold = 1.0 / alpha.clamp(1e-6, 0.5);
        let consistent_count = effects
            .iter()
            .zip(e_values.iter())
            .filter(|&(&eff, &ev)| eff * sign > 0.0 && ev >= threshold)
            .count();
        let avg_e = mean(e_values);
        let disc = e_bh_discoveries(e_values, alpha);
        let min_eff = effects.iter().copied().fold(f64::INFINITY, f64::min);
        let max_eff = effects.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let consistency = (consistent_count as f64) / (k as f64);
        let ebh_survival_rate = (disc.len() as f64) / (k as f64);
        let forking_penalty = 1.0 / (1.0 + 0.25 * (k as f64).max(1.0).ln());
        let n_eff_fork = (disc.len() as f64) * forking_penalty;
        let v_keynes = Self::keynes_weight_from_n_eff(n_eff_fork, 6.0);
        let rel_noise = (std_spec / med.abs().max(1.0)).clamp(0.0, 1.0);
        let framing_integrity = (consistency * ebh_survival_rate * forking_penalty).clamp(0.0, 1.0);

        Self {
            bounds: [min_eff, max_eff],
            imbens_manski_bounds: [med - std_spec, med + std_spec],
            identification_width: (max_eff - min_eff).max(0.0),
            n_eff: n_eff_fork,
            n_eff_variance: n_eff_fork,
            design_effect_deff: 1.0 / forking_penalty,
            keynes_weight: v_keynes,
            epistemic_margin_score: Self::canonical_score(v_keynes, rel_noise, framing_integrity),
            evidence_e_value: avg_e,
            decibans: 10.0 * avg_e.max(1e-30).log10(),
            multiverse_consistency: consistency,
            e_bh_discoveries_count: disc.len(),
            ..Self::default()
        }
    }

    /// Solves the Imbens–Manski (2004, *Econometrica* 72(6), Eq. 4) adaptive critical value
    /// $c_\alpha \in [z_{1-\alpha},\, z_{1-\alpha/2}]$ by bisection, interpolating between the
    /// one-sided and two-sided quantiles as identification width grows relative to sampling error:
    /// $$\Phi\bigl(c_\alpha + \Delta_{\mathrm{id}} / \hat{\sigma}_{\max}\bigr) - \Phi(-c_\alpha) = 1 - \alpha.$$
    #[inline]
    pub fn imbens_manski_critical_value(identification_width: f64, se_max: f64, alpha: f64) -> f64 {
        let alpha = alpha.clamp(1e-9, 0.5);
        let z_lo = inverse_standard_normal_cdf(1.0 - alpha);
        let z_hi = inverse_standard_normal_cdf(1.0 - 0.5 * alpha);
        if identification_width <= 0.0 || se_max <= 0.0 {
            return z_hi;
        }
        let delta = identification_width / se_max;
        let target = 1.0 - alpha;
        let (mut lo, mut hi) = (z_lo, z_hi);
        for _ in 0..200 {
            let mid = 0.5 * (lo + hi);
            let f = standard_normal_cdf(mid + delta) - standard_normal_cdf(-mid);
            if f < target {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        0.5 * (lo + hi)
    }

    /// **Chapter 3 Operator — Single-Arm Manski & Imbens–Manski Partial Identification Bounds on $\mathbb{E}[Y]$.**
    ///
    /// Incorporates both within-responder sampling variance $p^2 s_{\mathrm{obs}}^2 / n_{\mathrm{obs}}$ (using
    /// Popoviciu's $\frac{1}{4}(y_{\max} - y_{\min})^2$ bound when $n_{\mathrm{obs}} \le 1$) and the binomial
    /// response-rate variance $\frac{p(1-p)}{n_{\mathrm{tot}}}(\bar{Y}_{\mathrm{obs}} - y_{\min/\max})^2$.
    pub fn from_single_arm_bounds(
        observed: &[f64],
        missing_count: usize,
        y_min: f64,
        y_max: f64,
        alpha: f64,
    ) -> Self {
        if ![y_min, y_max, alpha].iter().all(|x| x.is_finite())
            || observed.iter().any(|x| !x.is_finite())
        {
            return Self::default();
        }
        let lo_supp = y_min.min(y_max);
        let hi_supp = y_max.max(y_min);
        let support_span = (hi_supp - lo_supp).max(1e-9);
        if observed.is_empty() {
            return Self {
                bounds: [lo_supp, hi_supp],
                imbens_manski_bounds: [lo_supp, hi_supp],
                identification_width: support_span,
                ..Self::default()
            };
        }
        let n_obs = observed.len();
        let n_tot = (n_obs + missing_count).max(1);
        let p_obs = (n_obs as f64) / (n_tot as f64);
        let p_miss = 1.0 - p_obs;
        let obs_mean = mean(observed);

        let lower = p_obs * obs_mean + p_miss * lo_supp;
        let upper = p_obs * obs_mean + p_miss * hi_supp;
        let width = (upper - lower).max(0.0);

        let obs_var = if n_obs >= 2 {
            sample_variance(observed)
        } else {
            0.25 * support_span * support_span
        };
        let var_p = (p_obs * p_miss) / (n_tot as f64);
        let se_lower = (p_obs * p_obs * (obs_var / (n_obs as f64))
            + var_p * (obs_mean - lo_supp).powi(2))
        .sqrt();
        let se_upper = (p_obs * p_obs * (obs_var / (n_obs as f64))
            + var_p * (hi_supp - obs_mean).powi(2))
        .sqrt();
        let se_bound = se_lower.max(se_upper);
        let z_crit = Self::imbens_manski_critical_value(width, se_bound, alpha);
        let im_lower = (lower - z_crit * se_lower).max(lo_supp);
        let im_upper = (upper + z_crit * se_upper).min(hi_supp);
        let n_eff = (n_obs as f64) * p_obs;
        let z_two = inverse_standard_normal_cdf(1.0 - 0.5 * alpha.clamp(1e-9, 0.5));
        let eps_obs =
            (2.0 * z_two * (obs_var / (n_obs as f64)).sqrt() / support_span).clamp(0.0, 1.0);
        let rel_id_width = (width / support_span).clamp(0.0, 1.0);
        let total_uncertainty_ratio =
            (rel_id_width + (1.0 - rel_id_width) * eps_obs).clamp(0.0, 1.0);
        let v_keynes = Self::keynes_weight_from_n_eff(n_eff, CANONICAL_KEYNES_N0);

        Self {
            bounds: [lower, upper],
            imbens_manski_bounds: [im_lower, im_upper],
            identification_width: width,
            n_eff,
            n_eff_variance: n_obs as f64,
            design_effect_deff: (1.0 / p_obs.max(1e-12)).max(1.0),
            keynes_weight: v_keynes,
            epistemic_margin_score: Self::canonical_score(v_keynes, total_uncertainty_ratio, 1.0),
            ..Self::default()
        }
    }

    /// **Chapter 3 Operator — Two-Arm Manski & Imbens–Manski (2004) Partial Identification Bounds on the
    /// Average Treatment Effect $\mathrm{ATE} = \mathbb{E}[Y(1)] - \mathbb{E}[Y(0)]$.**
    ///
    /// Computes worst-case non-parametric bounds $[L_{\mathrm{ATE}}, U_{\mathrm{ATE}}] = [L_M(1) - U_M(0),\, U_M(1) - L_M(0)]$
    /// and solves the joint Imbens–Manski (2004, Eq. 4) confidence interval using the exact quadrature
    /// standard error $\mathrm{SE}_{\mathrm{ATE}} = \sqrt{\mathrm{SE}_1^2 + \mathrm{SE}_0^2}$ of the two independent arms
    /// (including binomial response-rate variance and Popoviciu single-observation fallback).
    pub fn from_ate_bounds(
        treated_obs: &[f64],
        missing_treated: usize,
        control_obs: &[f64],
        missing_control: usize,
        y_min: f64,
        y_max: f64,
        alpha: f64,
    ) -> Self {
        if ![y_min, y_max, alpha].iter().all(|x| x.is_finite())
            || treated_obs.iter().any(|x| !x.is_finite())
            || control_obs.iter().any(|x| !x.is_finite())
        {
            return Self::default();
        }
        let lo_supp = y_min.min(y_max);
        let hi_supp = y_max.max(y_min);
        let span = (hi_supp - lo_supp).max(1e-9);
        let arm1 =
            Self::from_single_arm_bounds(treated_obs, missing_treated, lo_supp, hi_supp, alpha);
        let arm0 =
            Self::from_single_arm_bounds(control_obs, missing_control, lo_supp, hi_supp, alpha);
        let lower_ate = arm1.bounds[0] - arm0.bounds[1];
        let upper_ate = arm1.bounds[1] - arm0.bounds[0];
        let width_ate = (upper_ate - lower_ate).max(0.0);

        let arm_se = |obs: &[f64], miss: usize| -> (f64, f64, f64) {
            let n = obs.len();
            if n == 0 {
                return (0.5 * span, 0.5 * span, 0.25 * span * span);
            }
            let n_tot = (n + miss).max(1);
            let p = (n as f64) / (n_tot as f64);
            let var_y = if n >= 2 {
                sample_variance(obs)
            } else {
                0.25 * span * span
            };
            let var_p = p * (1.0 - p) / (n_tot as f64);
            let mu = mean(obs);
            let se_lo = (p * p * (var_y / (n as f64)) + var_p * (mu - lo_supp).powi(2)).sqrt();
            let se_hi = (p * p * (var_y / (n as f64)) + var_p * (hi_supp - mu).powi(2)).sqrt();
            (se_lo, se_hi, var_y / (n as f64))
        };
        let (se1_lo, se1_hi, var1_obs) = arm_se(treated_obs, missing_treated);
        let (se0_lo, se0_hi, var0_obs) = arm_se(control_obs, missing_control);
        let se_ate_lo = (se1_lo * se1_lo + se0_hi * se0_hi).sqrt();
        let se_ate_hi = (se1_hi * se1_hi + se0_lo * se0_lo).sqrt();
        let se_ate = se_ate_lo.max(se_ate_hi);

        let c_alpha = Self::imbens_manski_critical_value(width_ate, se_ate, alpha);
        let im_lower = (lower_ate - c_alpha * se_ate_lo).max(-span);
        let im_upper = (upper_ate + c_alpha * se_ate_hi).min(span);
        let n_eff = arm1.n_eff + arm0.n_eff;
        let n_var = (treated_obs.len() + control_obs.len()) as f64;
        let z_two = inverse_standard_normal_cdf(1.0 - 0.5 * alpha.clamp(1e-9, 0.5));
        let eps_obs = (2.0 * z_two * (var1_obs + var0_obs).sqrt() / (2.0 * span)).clamp(0.0, 1.0);
        let rel_id_width = (width_ate / (2.0 * span)).clamp(0.0, 1.0);
        let total_uncertainty_ratio =
            (rel_id_width + (1.0 - rel_id_width) * eps_obs).clamp(0.0, 1.0);
        let v_keynes = Self::keynes_weight_from_n_eff(n_eff, CANONICAL_KEYNES_N0);

        Self {
            bounds: [lower_ate, upper_ate],
            imbens_manski_bounds: [im_lower, im_upper],
            identification_width: width_ate,
            n_eff,
            n_eff_variance: n_var,
            design_effect_deff: (n_var / n_eff.max(1e-12)).max(1.0),
            keynes_weight: v_keynes,
            epistemic_margin_score: Self::canonical_score(v_keynes, total_uncertainty_ratio, 1.0),
            ..Self::default()
        }
    }

    /// **Mayo–Manski Severe Boundary Test:** returns `true` iff even the conservative lower
    /// Imbens–Manski confidence boundary strictly exceeds `min_relevant_effect`.
    #[inline]
    pub fn passes_severe_boundary(&self, min_relevant_effect: f64) -> bool {
        self.imbens_manski_bounds[0] > min_relevant_effect
    }

    /// Fractional-trimmed mean: removes the top `trim_fraction` (`trim_high = true`) or the bottom
    /// `trim_fraction` (`trim_high = false`) of the sorted slice, weighting the boundary observation
    /// fractionally so that any `trim_fraction \in [0, 1)` removes exactly that mass (Lee 2009).
    fn trimmed_mean(sorted: &[f64], trim_fraction: f64, trim_high: bool) -> f64 {
        let n = sorted.len();
        if n == 0 {
            return 0.0;
        }
        let t = trim_fraction.clamp(0.0, 1.0 - 1e-12);
        let keep = (1.0 - t) * (n as f64);
        let mut total = 0.0;
        let mut count = 0.0;
        if trim_high {
            let hi = keep.floor() as usize;
            let frac = keep - (hi as f64);
            for &x in &sorted[..hi.min(n)] {
                total += x;
                count += 1.0;
            }
            if hi < n {
                total += frac * sorted[hi];
                count += frac;
            }
        } else {
            let start = (n as f64) - keep;
            let lo = start.floor() as usize;
            let frac = 1.0 - (start - (lo as f64));
            if lo < n {
                total += frac * sorted[lo];
                count += frac;
            }
            for &x in &sorted[(lo + 1).min(n)..] {
                total += x;
                count += 1.0;
            }
        }
        total / count.max(1e-12)
    }

    /// **Chapter 3 Operator — David S. Lee (2009) Sharp Trimming Bounds for Selective Attrition on $\mathrm{ATE}$.**
    pub fn from_lee_attrition_bounds(
        treated_obs: &[f64],
        n_treated_assigned: usize,
        control_obs: &[f64],
        n_control_assigned: usize,
    ) -> Self {
        if treated_obs.is_empty() || control_obs.is_empty() {
            return Self::default();
        }
        let n_t_ass = n_treated_assigned.max(treated_obs.len());
        let n_c_ass = n_control_assigned.max(control_obs.len());
        let r1 = (treated_obs.len() as f64) / (n_t_ass as f64);
        let r0 = (control_obs.len() as f64) / (n_c_ass as f64);
        let naive = mean(treated_obs) - mean(control_obs);

        let mut t_sorted = treated_obs.to_vec();
        t_sorted.sort_by(|a, b| a.total_cmp(b));
        let mut c_sorted = control_obs.to_vec();
        c_sorted.sort_by(|a, b| a.total_cmp(b));

        let (lo, hi) = if (r1 - r0).abs() <= 1e-12 {
            (naive, naive)
        } else if r1 > r0 {
            let p_trim = (r1 - r0) / r1;
            let mean_low = Self::trimmed_mean(&t_sorted, p_trim, true);
            let mean_high = Self::trimmed_mean(&t_sorted, p_trim, false);
            let c_mean = mean(&c_sorted);
            (mean_low - c_mean, mean_high - c_mean)
        } else {
            let p_trim = (r0 - r1) / r0;
            let c_low = Self::trimmed_mean(&c_sorted, p_trim, true);
            let c_high = Self::trimmed_mean(&c_sorted, p_trim, false);
            let t_mean = mean(&t_sorted);
            (t_mean - c_high, t_mean - c_low)
        };
        let n_eff = (treated_obs.len() + control_obs.len()) as f64;
        let v_keynes = Self::keynes_weight_from_n_eff(n_eff, CANONICAL_KEYNES_N0);
        let max_span = (t_sorted.last().copied().unwrap_or(1.0)
            - c_sorted.first().copied().unwrap_or(0.0))
        .abs()
        .max(1.0);
        let se_t = if treated_obs.len() >= 2 {
            sample_std(treated_obs) / (treated_obs.len() as f64).sqrt()
        } else {
            0.5 * max_span
        };
        let se_c = if control_obs.len() >= 2 {
            sample_std(control_obs) / (control_obs.len() as f64).sqrt()
        } else {
            0.5 * max_span
        };
        let se_lee = (se_t * se_t + se_c * se_c).sqrt();
        let im_lo = lo - 1.96 * se_lee;
        let im_hi = hi + 1.96 * se_lee;
        let sampling_noise_ratio =
            (((im_hi - im_lo) - (hi - lo).max(0.0)).max(0.0) / (2.0 * max_span)).clamp(0.0, 1.0);
        Self {
            bounds: [lo, hi],
            imbens_manski_bounds: [im_lo, im_hi],
            identification_width: (hi - lo).max(0.0),
            n_eff,
            n_eff_variance: n_eff,
            design_effect_deff: 1.0,
            keynes_weight: v_keynes,
            epistemic_margin_score: Self::canonical_score(
                v_keynes,
                sampling_noise_ratio,
                0.5 * (r1 + r0),
            ),
            ..Self::default()
        }
    }

    /// **Chapter 3 Operator — Worst-case censoring bounds on survival** (NOT Peterson 1976).
    /// With `failed_by_t` events, `censored_by_t` at-risk losses, and `survived_past_t` survivors,
    /// the worst-case completion of censored units gives $\mathbb{P}(\text{survive}) \in [\frac{s}{n}, \frac{s+c}{n}]$;
    /// the naive independent-censoring estimate is the Kaplan–Meier value $\frac{s}{s+f}$.
    /// Returns `(EpistemicMarginState, naive_km_survival)`.
    pub fn from_censoring_bounds(
        failed_by_t: usize,
        censored_by_t: usize,
        survived_past_t: usize,
    ) -> (Self, f64) {
        let total = failed_by_t + censored_by_t + survived_past_t;
        if total == 0 {
            return (Self::default(), 0.0);
        }
        let n = total as f64;
        let s_lower = (survived_past_t as f64) / n;
        let s_upper = ((survived_past_t + censored_by_t) as f64) / n;
        let uncensored = (failed_by_t + survived_past_t).max(1) as f64;
        let s_naive = (survived_past_t as f64) / uncensored;
        let width = (s_upper - s_lower).max(0.0);
        let se_surv = ((s_naive * (1.0 - s_naive)).max(1e-6) / uncensored).sqrt();
        let im_lo = (s_lower - 1.96 * se_surv).max(0.0);
        let im_hi = (s_upper + 1.96 * se_surv).min(1.0);
        let sampling_noise_ratio = ((im_hi - im_lo) - width).clamp(0.0, 1.0);
        let v_keynes = Self::keynes_weight_from_n_eff(uncensored, CANONICAL_KEYNES_N0);
        let state = Self {
            bounds: [s_lower, s_upper],
            imbens_manski_bounds: [im_lo, im_hi],
            identification_width: width,
            n_eff: uncensored,
            n_eff_variance: uncensored,
            design_effect_deff: 1.0,
            keynes_weight: v_keynes,
            epistemic_margin_score: Self::canonical_score(v_keynes, sampling_noise_ratio, 1.0),
            ..Self::default()
        };
        (state, s_naive)
    }

    /// **Chapter 3 Operator — Arthur V. Peterson (1976, *PNAS* 73(1): 11–13) sharp Fréchet bounds
    /// on the joint survival $S_T(t) = \mathbb{P}(\min(T_1, T_2) > t)$ under potentially dependent
    /// competing risks with observed marginal survivals $S_1(t), S_2(t)$:**
    /// $$\max\bigl(0,\, S_1 + S_2 - 1\bigr) \le S_T(t) \le \min\bigl(S_1, S_2\bigr).$$
    /// Returns `(EpistemicMarginState, naive_independent_survival = S_1 S_2)`.
    pub fn from_peterson_competing_risks_bounds(s1_t: f64, s2_t: f64) -> (Self, f64) {
        let s1 = s1_t.clamp(0.0, 1.0);
        let s2 = s2_t.clamp(0.0, 1.0);
        let lower = (s1 + s2 - 1.0).max(0.0);
        let upper = s1.min(s2);
        let width = (upper - lower).max(0.0);
        let s_indep = s1 * s2;
        let v_keynes = Self::keynes_weight_from_n_eff(200.0, CANONICAL_KEYNES_N0);
        let state = Self {
            bounds: [lower, upper],
            imbens_manski_bounds: [lower, upper],
            identification_width: width,
            n_eff: 200.0,
            n_eff_variance: 200.0,
            design_effect_deff: 1.0,
            keynes_weight: v_keynes,
            epistemic_margin_score: Self::canonical_score(v_keynes, 0.0, 1.0),
            ..Self::default()
        };
        (state, s_indep)
    }

    /// **Chapter 3 Operator — Cromwell–Ville Betting Martingale (*e*-Process) & Lakatos Problemshift Ratio.**
    pub fn from_e_process(
        xs: &[f64],
        null_mean: f64,
        sigma: f64,
        lambda_bet: f64,
        ad_hoc_adjustments_k: usize,
    ) -> Self {
        let sig = sigma.max(1e-9);
        let lam = lambda_bet.clamp(-2.0, 2.0);
        let mut wealth = 1.0_f64;
        for &x in xs {
            let z = (x - null_mean) / sig;
            let factor = (lam * z - 0.5 * lam * lam).exp().max(0.0);
            wealth = (wealth * factor).clamp(1e-30, 1e30);
        }
        let adjusted_e = lakatos_shift_ratio(wealth, ad_hoc_adjustments_k);
        let db = 10.0 * adjusted_e.max(1e-30).log10();
        let n = xs.len().max(1);
        let r = anytime_valid_radius(sig, n, 0.05);
        let m = if xs.is_empty() { null_mean } else { mean(xs) };
        let v_keynes = Self::keynes_weight_from_n_eff(n as f64, CANONICAL_KEYNES_N0);
        let rel_w = (r / (4.0 * sig)).clamp(0.0, 1.0);
        let lakatos_integrity = 1.0 / (1.0 + ad_hoc_adjustments_k as f64);
        Self {
            bounds: [m - r, m + r],
            imbens_manski_bounds: [m - r, m + r],
            identification_width: 2.0 * r,
            n_eff: n as f64,
            n_eff_variance: n as f64,
            design_effect_deff: 1.0,
            keynes_weight: v_keynes,
            epistemic_margin_score: Self::canonical_score(v_keynes, rel_w, lakatos_integrity),
            evidence_e_value: adjusted_e,
            decibans: db,
            ..Self::default()
        }
    }

    /// **Chapter 4 Operator — Campbell–Moulton Cluster Design Effect ($\mathrm{DEFF}$) & Keynes $V$.**
    ///
    /// - When `tail_quantile_p <= 0.0`, computes the one-way random-effects ANOVA intracluster correlation
    ///   $\hat{\rho}_Y$ directly on the continuous outcome $Y_{cj}$ and sets `bounds` to the cluster-adjusted
    ///   mean estimate $\bar{Y}$ (matching the continuous estimand $\mathbb{E}[Y]$).
    /// - When `tail_quantile_p > 0.0`, computes the ANOVA ICC on the binary exceedance indicator
    ///   $Z_{cj} = \mathbf{1}\{Y_{cj} > q_p\}$ and sets `bounds` to the exceedance probability $\hat{p}_q = \bar{Z}$
    ///   (matching the tail-probability estimand $\mathbb{P}(Y > q_p)$).
    pub fn from_archive_clusters(
        clusters: &[Vec<f64>],
        tail_quantile_p: f64,
        reference_n0: f64,
    ) -> Self {
        if !tail_quantile_p.is_finite()
            || !reference_n0.is_finite()
            || reference_n0 <= 0.0
            || clusters
                .iter()
                .flat_map(|c| c.iter())
                .any(|x| !x.is_finite())
        {
            return Self::default();
        }
        let non_empty: Vec<&Vec<f64>> = clusters.iter().filter(|c| !c.is_empty()).collect();
        if non_empty.is_empty() {
            return Self::default();
        }
        let c_count = non_empty.len();
        let mut all_vals = Vec::new();
        let mut sum_m = 0usize;
        let mut sum_m2 = 0usize;
        for c in &non_empty {
            let m = c.len();
            sum_m += m;
            sum_m2 += m * m;
            all_vals.extend_from_slice(c);
        }
        let n_f = sum_m.max(1) as f64;
        if c_count < 2 {
            // A1-07: With only 1 cluster, between-cluster correlation cannot be identified;
            // conservatively treat the entire cluster as 1 effective independent unit (DEFF = n_f).
            let m = mean(&all_vals);
            let v_k = Self::keynes_weight_from_n_eff(1.0, reference_n0);
            return Self {
                bounds: [m, m],
                imbens_manski_bounds: [m, m],
                n_eff: 1.0,
                n_eff_variance: 1.0,
                design_effect_deff: n_f.max(1.0),
                keynes_weight: v_k,
                epistemic_margin_score: Self::canonical_score(v_k, 0.0, 1.0),
                ..Self::default()
            };
        }
        let c_f = c_count as f64;
        let m_tilde = (sum_m2 as f64) / n_f;

        let (obs_estimand, ssb, ssw) = if tail_quantile_p <= 0.0 {
            let grand_mean = mean(&all_vals);
            let mut b = 0.0_f64;
            let mut w = 0.0_f64;
            for c in &non_empty {
                let m_c = c.len() as f64;
                let mu_c = mean(c);
                let diff = mu_c - grand_mean;
                b += m_c * diff * diff;
                for &x in *c {
                    let d = x - mu_c;
                    w += d * d;
                }
            }
            (grand_mean, b, w)
        } else {
            let thresh = quantile(&all_vals, tail_quantile_p.clamp(0.05, 0.99));
            let mut cluster_rates = Vec::with_capacity(c_count);
            let mut total_exceed = 0.0;
            let mut w = 0.0;
            for c in &non_empty {
                let k_c = c.iter().filter(|&&x| x > thresh).count() as f64;
                let m_c = c.len() as f64;
                let p_c = k_c / m_c;
                cluster_rates.push((m_c, p_c));
                total_exceed += k_c;
                w += m_c * p_c * (1.0 - p_c);
            }
            let p_bar = total_exceed / n_f;
            let mut b = 0.0;
            for &(m_c, p_c) in &cluster_rates {
                let diff = p_c - p_bar;
                b += m_c * diff * diff;
            }
            (p_bar, b, w)
        };

        let msb = ssb / ((c_f - 1.0).max(1.0));
        let msw = ssw / ((n_f - c_f).max(1.0));
        let n0 = (n_f - (sum_m2 as f64) / n_f) / ((c_f - 1.0).max(1.0));
        let denom = msb + (n0 - 1.0).max(0.0) * msw;
        let rho = if denom <= 1e-12 {
            0.0
        } else {
            ((msb - msw) / denom).clamp(0.0, 1.0)
        };
        let deff = (1.0 + (m_tilde - 1.0).max(0.0) * rho).max(1.0);
        let n_eff = Self::unified_effective_n(n_f, n_f, deff, 0.0);
        let v_keynes = Self::keynes_weight_from_n_eff(n_eff, reference_n0);

        Self {
            bounds: [obs_estimand, obs_estimand],
            imbens_manski_bounds: [obs_estimand, obs_estimand],
            identification_width: 0.0,
            n_eff,
            n_eff_variance: n_eff,
            design_effect_deff: deff,
            keynes_weight: v_keynes,
            epistemic_margin_score: Self::canonical_score(v_keynes, 0.0, 1.0),
            ..Self::default()
        }
    }

    /// **Chapter 4 Operator — Meng's Big Data Paradox Unified with $\mathrm{DEFF}_{\mathrm{total}}$.**
    pub fn from_meng_big_data(
        population_n: f64,
        sample_n: f64,
        rho_ry: f64,
        sigma_y: f64,
        deff_total: f64,
        reference_n0: f64,
    ) -> Self {
        if ![
            population_n,
            sample_n,
            rho_ry,
            sigma_y,
            deff_total,
            reference_n0,
        ]
        .iter()
        .all(|x| x.is_finite())
            || sample_n <= 0.0
            || population_n <= 0.0
            || deff_total <= 0.0
            || reference_n0 <= 0.0
        {
            return Self::default();
        }
        let s_n = sample_n.max(1.0);
        let p_n = population_n.max(s_n);
        let sig = sigma_y.max(0.0);
        let f = (s_n / p_n).clamp(1e-12, 1.0);
        let drop_factor = ((1.0 - f) / f).max(0.0).sqrt();
        let err = rho_ry * drop_factor * sig;
        let deff = deff_total.max(1.0);
        let n_eff = Self::unified_effective_n(s_n, p_n, deff, rho_ry);
        let n_var = s_n / deff;
        let v_keynes = Self::keynes_weight_from_n_eff(n_eff, reference_n0);
        let rel_w = (err.abs() / sig.max(1e-9)).clamp(0.0, 1.0);

        Self {
            bounds: [-err.abs(), err.abs()],
            imbens_manski_bounds: [-err.abs(), err.abs()],
            identification_width: 2.0 * err.abs(),
            n_eff,
            n_eff_variance: n_var,
            design_effect_deff: deff,
            meng_estimation_error: err,
            keynes_weight: v_keynes,
            epistemic_margin_score: Self::canonical_score(v_keynes, rel_w, 1.0),
            ..Self::default()
        }
    }

    /// **Chapter 4 Operator — Horvitz–Thompson & Kish Inverse-Propensity Weighting Floor.**
    pub fn from_kish_strata(
        strata_counts_and_propensities: &[(usize, f64)],
        reference_n0: f64,
    ) -> Self {
        if strata_counts_and_propensities.is_empty() {
            return Self::default();
        }
        let mut total_n = 0usize;
        let mut sum_w = 0.0_f64;
        let mut sum_w2 = 0.0_f64;
        for &(count, pi) in strata_counts_and_propensities {
            if count == 0 {
                continue;
            }
            let clean_pi = pi.clamp(1e-9, 1.0);
            let w = 1.0 / clean_pi;
            total_n += count;
            sum_w += (count as f64) * w;
            sum_w2 += (count as f64) * w * w;
        }
        let kish_n = if sum_w2 <= 1e-15 {
            0.0
        } else {
            (sum_w * sum_w) / sum_w2
        };
        let n_f = total_n.max(1) as f64;
        let deff_kish = (n_f / kish_n.max(1e-12)).max(1.0);
        let v_keynes = Self::keynes_weight_from_n_eff(kish_n, reference_n0);

        Self {
            n_eff: kish_n,
            n_eff_variance: kish_n,
            design_effect_deff: deff_kish,
            keynes_weight: v_keynes,
            epistemic_margin_score: Self::canonical_score(v_keynes, 0.0, 1.0),
            ..Self::default()
        }
    }

    /// **Chapter 4 Operator — Lincoln–Petersen & Chao (1987) Capture-Recapture Dark Figure $\hat{n}_{00}$.**
    ///
    /// Sets `bounds[0] = chao_lower_bound_total` (Chao's heterogeneity-robust lower bound $\hat{N}_{\mathrm{Chao}} = N_{\mathrm{obs}} + \frac{f_1^2}{2 f_2}$)
    /// and `bounds[1]` to Chao's (1987, *Biometrics* 43(4): 783–791, Eq. 12) log-normal 95% upper confidence limit
    /// $U_{\mathrm{Chao}, 0.95} = N_{\mathrm{obs}} + \hat{n}_{00,\mathrm{Chao}} \exp\!\bigl(1.96\sqrt{\ln(1 + \widehat{\mathrm{Var}}(\hat{N}_{\mathrm{Chao}}) / \hat{n}_{00,\mathrm{Chao}}^2)}\bigr)$.
    pub fn from_capture_recapture(both_n11: usize, only_a_n10: usize, only_b_n01: usize) -> Self {
        let obs = both_n11 + only_a_n10 + only_b_n01;
        let f2 = both_n11.max(1) as f64;
        let n00 = ((only_a_n10 * only_b_n01) as f64) / f2;
        let f1 = (only_a_n10 + only_b_n01) as f64;
        let chao_add = (f1 * f1) / (2.0 * f2);
        let chao_total = (obs as f64) + chao_add;
        let ratio = f1 / f2;
        let var_chao = f2 * (0.5 * ratio.powi(2) + ratio.powi(3) + 0.25 * ratio.powi(4));
        let chao_upper_95 = if chao_add <= 1e-12 {
            chao_total
        } else {
            let c_factor = (1.96 * (1.0 + var_chao / (chao_add * chao_add)).ln().sqrt()).exp();
            (obs as f64) + chao_add * c_factor
        };
        let width = (chao_upper_95 - chao_total).max(0.0);
        let v_keynes = Self::keynes_weight_from_n_eff(obs as f64, CANONICAL_KEYNES_N0);
        let rel_w = (width / chao_upper_95.max(1.0)).clamp(0.0, 1.0);

        Self {
            bounds: [chao_total, chao_upper_95],
            imbens_manski_bounds: [chao_total, chao_upper_95],
            identification_width: width,
            n_eff: obs as f64,
            n_eff_variance: obs as f64,
            design_effect_deff: 1.0,
            keynes_weight: v_keynes,
            epistemic_margin_score: Self::canonical_score(v_keynes, rel_w, 1.0),
            uncounted_tail_n00: n00,
            chao_lower_bound_total: chao_total,
            ..Self::default()
        }
    }

    /// **Chapter 4 Operator — Bitemporal Vintage Revision Audit (`rtdsm_routput_vintages.csv`).**
    pub fn from_vintages(vintages: &[(u32, u32, f64)]) -> Self {
        let mut by_vintage: BTreeMap<u32, BTreeMap<u32, f64>> = BTreeMap::new();
        for &(v_q, o_q, val) in vintages {
            if val > 0.0 {
                by_vintage.entry(v_q).or_default().insert(o_q, val);
            }
        }
        let Some((_, final_series)) = by_vintage.iter().next_back() else {
            return Self::default();
        };
        let mut diffs = Vec::new();
        let mut flips = 0usize;
        for (&v_q, series) in &by_vintage {
            if v_q < 2 {
                continue;
            }
            let target_q = v_q - 1;
            let prev_q = v_q - 2;
            if let (Some(&init_curr), Some(&init_prev), Some(&fin_curr), Some(&fin_prev)) = (
                series.get(&target_q),
                series.get(&prev_q),
                final_series.get(&target_q),
                final_series.get(&prev_q),
            ) {
                let g_init = 400.0 * ((init_curr / init_prev).ln());
                let g_fin = 400.0 * ((fin_curr / fin_prev).ln());
                diffs.push((g_fin - g_init).abs());
                if g_init * g_fin < 0.0 {
                    flips += 1;
                }
            }
        }
        if diffs.is_empty() {
            return Self::default();
        }
        let mean_rev = mean(&diffs);
        let max_rev = diffs.iter().copied().fold(0.0_f64, f64::max);
        let flip_rate = (flips as f64) / (diffs.len() as f64);
        let v_keynes = Self::keynes_weight_from_n_eff(diffs.len() as f64, CANONICAL_KEYNES_N0);
        let rel_noise = (mean_rev / max_rev.max(1.0)).clamp(0.0, 1.0);

        Self {
            bounds: [-max_rev, max_rev],
            imbens_manski_bounds: [-mean_rev, mean_rev],
            identification_width: 2.0 * max_rev,
            n_eff: diffs.len() as f64,
            n_eff_variance: diffs.len() as f64,
            design_effect_deff: 1.0,
            keynes_weight: v_keynes,
            epistemic_margin_score: Self::canonical_score(
                v_keynes,
                rel_noise,
                (1.0 - flip_rate).clamp(0.0, 1.0),
            ),
            vintage_num_quarters: diffs.len(),
            vintage_mean_revision_pp: mean_rev,
            vintage_max_revision_pp: max_rev,
            vintage_sign_flip_rate: flip_rate,
            ..Self::default()
        }
    }

    /// **Sequential Engine Operator (`engine.rs`) — Evaluates the complete `EpistemicMarginState` for shift $t$.**
    ///
    /// Computes cluster-adjusted Manski & Imbens–Manski bounds without double-counting Meng's bias on top of
    /// Manski's worst-case support bounds, and estimates $\hat{\rho}_{R,Y}$ and $\hat{\sigma}_Y$ empirically from
    /// the observed cluster retention rates and outcomes.
    pub fn for_shift_observation(
        observed_clusters: &[Vec<f64>],
        missing_count: usize,
        demand_support: (f64, f64),
    ) -> Self {
        if !demand_support.0.is_finite()
            || !demand_support.1.is_finite()
            || observed_clusters
                .iter()
                .flat_map(|c| c.iter())
                .any(|x| !x.is_finite())
        {
            return Self::default();
        }
        let all_obs: Vec<f64> = observed_clusters
            .iter()
            .flat_map(|c| c.iter().copied())
            .collect();
        if all_obs.is_empty() {
            return Self::default();
        }
        let raw_n = all_obs.len() as f64;
        let total_n = (all_obs.len() + missing_count).max(1);
        let p_obs = raw_n / (total_n as f64);
        let p_miss = 1.0 - p_obs;

        let cluster_state =
            Self::from_archive_clusters(observed_clusters, 0.0, CANONICAL_KEYNES_N0);
        // Estimate Kish propensity strata from observed cluster retention rates (nominal 10 units per cluster):
        let strata: Vec<(usize, f64)> = observed_clusters
            .iter()
            .map(|c| (c.len(), (c.len() as f64 / 10.0).clamp(0.10, 1.0)))
            .collect();
        let kish_state = Self::from_kish_strata(&strata, CANONICAL_KEYNES_N0);
        let non_empty_clusters = observed_clusters.iter().filter(|c| !c.is_empty()).count();
        let n_var_floor = non_empty_clusters.clamp(1, 4) as f64;
        // Multiplicative design-effect composition: DEFF_var = DEFF_cluster * DEFF_Kish
        // with declared floor n_var_floor = min(non_empty_clusters, 4)
        let deff_var = (cluster_state.design_effect_deff * kish_state.design_effect_deff).max(1.0);
        let n_var = (raw_n / deff_var).max(n_var_floor);

        let obs_mean = mean(&all_obs);
        let obs_var = if all_obs.len() >= 2 {
            sample_variance(&all_obs)
        } else {
            1.0
        };
        let sigma_y = obs_var.sqrt().max(0.5);

        // Empirical between-cluster correlation between cluster retention rate r_c and cluster mean y_c:
        let c_means: Vec<f64> = observed_clusters.iter().map(|c| mean(c)).collect();
        let c_rets: Vec<f64> = observed_clusters
            .iter()
            .map(|c| (c.len() as f64) / 10.0)
            .collect();
        let m_c = mean(&c_means);
        let m_r = mean(&c_rets);
        let mut cov_ry = 0.0_f64;
        let mut var_r = 0.0_f64;
        let mut var_y = 0.0_f64;
        for (&yc, &rc) in c_means.iter().zip(c_rets.iter()) {
            cov_ry += (rc - m_r) * (yc - m_c);
            var_r += (rc - m_r) * (rc - m_r);
            var_y += (yc - m_c) * (yc - m_c);
        }
        let rho_ry = if var_r * var_y <= 1e-12 {
            0.0
        } else {
            (cov_ry / (var_r * var_y).sqrt()).clamp(-0.95, 0.95)
        };

        let pop_n = (total_n as f64).max(raw_n);
        let meng_state =
            Self::from_meng_big_data(pop_n, raw_n, rho_ry, sigma_y, deff_var, CANONICAL_KEYNES_N0);
        let n_eff = meng_state.n_eff.min(n_var * p_obs);

        // Manski worst-case structural partial-identification bounds [lower, upper]
        // separated from finite-sample cluster-adjusted Imbens-Manski confidence bounds [lo, hi]:
        let lower = p_obs * obs_mean + p_miss * demand_support.0;
        let upper = p_obs * obs_mean + p_miss * demand_support.1;
        let width = (upper - lower).max(0.0);
        let var_p = (p_obs * p_miss) / (total_n as f64);
        let se_cluster = (p_obs * p_obs * (obs_var / n_var) + var_p * obs_var).sqrt();
        let z_crit = Self::imbens_manski_critical_value(width, se_cluster, 0.05);
        let lo = (lower - z_crit * se_cluster).max(demand_support.0);
        let hi = (upper + z_crit * se_cluster).min(demand_support.1);
        let span = (demand_support.1 - demand_support.0).max(1.0);
        let eps_cluster = (2.0 * 1.96 * (obs_var / n_var).sqrt() / span).clamp(0.0, 1.0);
        let rel_id_width = (width / span).clamp(0.0, 1.0);
        let total_uncertainty_ratio =
            (rel_id_width + (1.0 - rel_id_width) * eps_cluster).clamp(0.0, 1.0);
        let n_var_eff = n_var * p_obs;
        let v_keynes = Self::keynes_weight_from_n_eff(n_var_eff, 2.0);

        Self {
            bounds: [lower, upper],
            imbens_manski_bounds: [lo, hi],
            identification_width: width,
            n_eff,
            n_eff_variance: n_var,
            design_effect_deff: (raw_n / n_eff.max(1e-12)).max(1.0),
            meng_estimation_error: meng_state.meng_estimation_error,
            keynes_weight: v_keynes,
            epistemic_margin_score: Self::canonical_score(v_keynes, total_uncertainty_ratio, 1.0),
            evidence_e_value: 1.0,
            ..Self::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_operator_i_unifies_meng_moulton_kish_and_manski_lee() {
        let meng = EpistemicMarginState::from_meng_big_data(
            10_000_000.0,
            2_300_000.0,
            -0.05,
            0.50,
            1.0,
            2500.0,
        );
        assert!((meng.n_eff - 119.5).abs() < 0.2);

        let inst = EpistemicMarginState::from_inquiry_framing(
            100.0, 1.0, 200.0, 0.20, 0.30, 1.0, 18.0, 0.05, 12.0, 28.0, 21.0, 1.0, 0.0,
        );
        let affected = EpistemicMarginState::from_inquiry_framing(
            100.0, 1.0, 2000.0, 0.20, 0.30, 1.0, 140.0, 0.05, 12.0, 28.0, 21.0, 21.0, 28.0,
        );
        assert!((inst.framing_gap(&affected) - 0.45).abs() < 1e-12);
        assert!(inst.imbens_manski_bounds[0] < inst.bounds[0]);
        assert!(inst.imbens_manski_bounds[1] > inst.bounds[1]);
        // Power now uses the corrected inverse normal CDF: Phi^{-1}(0.95)=1.64485 (was a
        // broken tail ~1.8476 before the fix), so inst power ~0.355 and affected power ~0.972.
        assert!(inst.severe_power < 0.36 && affected.severe_power > 0.97);
        assert!(inst.is_type_three_error);
        assert_eq!(
            inst.framing_regime,
            CognitiveFramingRegime::StrategicPremiseRigging
        );
        assert_eq!(
            affected.framing_regime,
            CognitiveFramingRegime::CommunicativeInfluence
        );

        let ate = EpistemicMarginState::from_ate_bounds(
            &[6000.0, 6500.0, 6550.0],
            1,
            &[4400.0, 4600.0, 4650.0],
            1,
            0.0,
            15000.0,
            0.05,
        );
        assert!(ate.bounds[0] < 1800.0 && ate.bounds[1] > 1800.0);
        assert!(
            ate.imbens_manski_bounds[0] <= ate.bounds[0]
                && ate.imbens_manski_bounds[1] >= ate.bounds[1]
        );
        let (pet_exact, _) = EpistemicMarginState::from_peterson_competing_risks_bounds(1.0, 1.0);
        assert!((pet_exact.epistemic_margin_score - 0.80_f64.sqrt()).abs() < 1e-12);
        assert!((pet_exact.keynes_weight - 0.80).abs() < 1e-12);

        // Verify A1 & A2: single-observation n=1 has strictly positive Imbens-Manski SE and scores strictly below n=100
        let single_obs = EpistemicMarginState::from_single_arm_bounds(&[50.0], 0, 0.0, 100.0, 0.05);
        let large_obs =
            EpistemicMarginState::from_single_arm_bounds(&[50.0; 100], 0, 0.0, 100.0, 0.05);
        assert!(single_obs.imbens_manski_bounds[1] > single_obs.bounds[1]);
        assert!(single_obs.epistemic_margin_score < large_obs.epistemic_margin_score);

        // Verify B5: Chao lower bound is bounds[0] and strictly below the 95% upper confidence limit bounds[1]
        let chao = EpistemicMarginState::from_capture_recapture(320, 480, 500);
        assert!((chao.bounds[0] - chao.chao_lower_bound_total).abs() < 1e-12);
        assert!(chao.bounds[1] > chao.bounds[0]);

        let n_eff_zero = EpistemicMarginState::unified_effective_n(100.0, 200.0, 3.0, 0.0);
        let n_eff_eps = EpistemicMarginState::unified_effective_n(100.0, 200.0, 3.0, 1e-6);
        assert!((n_eff_zero - 100.0 / 3.0).abs() < 1e-12);
        assert!((n_eff_zero - n_eff_eps).abs() < 1e-8);

        // Verify B2: Honest full-support partial identification (w_id = 1.0 under missingness) is NOT penalized to zero
        let honest_full_support =
            EpistemicMarginState::from_single_arm_bounds(&[30.0; 25], 75, 5.0, 65.0, 0.05);
        assert!(honest_full_support.epistemic_margin_score > 0.10);
        assert!(EpistemicMarginState::canonical_score(0.80, 1.0, 1.0) > 0.40);

        // Verify A1-01 & A1-02: S_epi is strictly decreasing in missingness across single-arm, ATE, and shift operators
        let sample_100: Vec<f64> = (0..100).map(|i| 10.0 + (i as f64) * 0.4).collect();
        let mut prev_single = 1.0_f64;
        let mut prev_shift = 1.0_f64;
        let clusters = vec![
            vec![18.0, 20.0, 22.0, 21.0, 19.0],
            vec![30.0, 32.0, 34.0],
            vec![40.0, 42.0],
        ];
        let zero_miss_shift =
            EpistemicMarginState::for_shift_observation(&clusters, 0, (5.0, 65.0));
        assert_eq!(zero_miss_shift.meng_estimation_error, 0.0);
        for &miss in &[0usize, 10, 100, 900, 2000, 99_000] {
            let s_arm =
                EpistemicMarginState::from_single_arm_bounds(&sample_100, miss, 0.0, 100.0, 0.05);
            let s_shift = EpistemicMarginState::for_shift_observation(&clusters, miss, (5.0, 65.0));
            assert!(
                s_arm.epistemic_margin_score < prev_single,
                "single-arm S_epi must decrease in missingness at miss={miss}"
            );
            assert!(
                s_shift.epistemic_margin_score < prev_shift,
                "shift S_epi must decrease in missingness at miss={miss}"
            );
            prev_single = s_arm.epistemic_margin_score;
            prev_shift = s_shift.epistemic_margin_score;
        }

        // Verify A1-07: Single cluster yields conservative n_eff = 1.0 and deff = n_f
        let one_cluster =
            EpistemicMarginState::from_archive_clusters(&[vec![10.0; 100]], 0.0, 50.0);
        assert_eq!(one_cluster.n_eff, 1.0);
        assert_eq!(one_cluster.design_effect_deff, 100.0);

        // Verify A1-09: Non-finite inputs fail closed to 0.0 (never NaN)
        assert_eq!(
            EpistemicMarginState::canonical_score(f64::NAN, 0.5, 1.0),
            0.0
        );
        assert_eq!(
            EpistemicMarginState::keynes_weight_from_n_eff(10.0, -5.0),
            0.0
        );
        assert_eq!(
            EpistemicMarginState::unified_effective_n(-10.0, 100.0, 1.0, 0.0),
            0.0
        );
        let nan_arm = EpistemicMarginState::from_single_arm_bounds(
            &[10.0, f64::NAN, 20.0],
            0,
            0.0,
            100.0,
            0.05,
        );
        assert_eq!(nan_arm.epistemic_margin_score, 0.0);

        // Verify M2: for_shift_observation separates structural Manski bounds from Imbens-Manski CI bounds
        let shift_epi = EpistemicMarginState::for_shift_observation(
            &[vec![20.0, 22.0, 24.0], vec![30.0, 34.0]],
            5,
            (5.0, 65.0),
        );
        assert!(shift_epi.bounds[0] > shift_epi.imbens_manski_bounds[0]);
        assert!(shift_epi.bounds[1] < shift_epi.imbens_manski_bounds[1]);

        // Verify A3 / MIN-01: multiplicative DEFF_var = DEFF_cluster * DEFF_Kish is strictly more conservative than min(cluster, kish)
        let multi_clusters = vec![
            vec![16.0, 18.0, 20.0, 22.0, 24.0, 26.0, 17.0, 19.0, 21.0, 23.0],
            vec![19.0, 21.0, 23.0, 25.0, 27.0, 29.0, 20.0, 22.0, 24.0, 26.0],
            vec![24.0, 28.0, 32.0, 36.0],
            vec![26.0, 30.0, 34.0, 38.0],
        ];
        let cl_only = EpistemicMarginState::from_archive_clusters(&multi_clusters, 0.0, 25.0);
        let ks_only = EpistemicMarginState::from_kish_strata(
            &[(10, 1.0), (10, 1.0), (4, 0.4), (4, 0.4)],
            25.0,
        );
        let comb_shift =
            EpistemicMarginState::for_shift_observation(&multi_clusters, 12, (5.0, 65.0));
        assert!(
            comb_shift.n_eff_variance < cl_only.n_eff_variance.min(ks_only.n_eff_variance),
            "multiplicative DEFF composition must yield n_eff_variance ({}) < min(cluster={}, kish={})",
            comb_shift.n_eff_variance,
            cl_only.n_eff_variance,
            ks_only.n_eff_variance
        );

        // Verify G7: Duplicating identical multiverse specifications from K=6 to K=600 penalizes forking paths
        let mv6 = EpistemicMarginState::from_multiverse(&[1.5; 6], &[35.0; 6], 0.05);
        let mv600 = EpistemicMarginState::from_multiverse(&[1.5; 600], &[35.0; 600], 0.05);
        assert!(
            mv600.epistemic_margin_score < mv6.epistemic_margin_score,
            "k=600 ({}) must score strictly lower than k=6 ({})",
            mv600.epistemic_margin_score,
            mv6.epistemic_margin_score
        );

        // Verify M1 & B2 across constructors: structural identification width never zeroes out S_epi
        let (cens, _) = EpistemicMarginState::from_censoring_bounds(20, 80, 100);
        let (pet_wide, _) = EpistemicMarginState::from_peterson_competing_risks_bounds(0.60, 0.60);
        let lee_wide = EpistemicMarginState::from_lee_attrition_bounds(
            &[10.0, 20.0, 30.0, 40.0],
            4,
            &[10.0, 20.0],
            4,
        );
        assert!(cens.epistemic_margin_score > 0.50);
        assert!((pet_wide.epistemic_margin_score - 0.80_f64.sqrt()).abs() < 1e-12);
        assert!(lee_wide.epistemic_margin_score > 0.15);
    }
}
