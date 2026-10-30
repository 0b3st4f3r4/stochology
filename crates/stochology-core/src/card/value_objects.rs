//! Typed Value Objects for quantitative epistemological declarations inside [`super::EpistemologicalCard`].
//!
//! Encapsulates numeric parsing, formatting, and algebraic invariant verification for:
//! - [`NonlinearSummaryMetrics`] (`PROHIBITION_2`: $f(\mathbb{E}[X])$, $\mathbb{E}[f(X)]$, $\Delta_J$, quantiles, $\mathrm{CRPS}$)
//! - [`MoultonClusterMetrics`] ($\Phi_2$: $\tilde{m}$, $\rho_I(p)$, $\mathrm{DEFF}_{\text{exc}}$, $n_{\mathrm{eff}}(p)$)
//! - [`RrmContractionMetrics`] (`PROHIBITION_3`: $\frac{\varepsilon \beta}{\gamma} < 1$)
//! - [`PerformativeDivergenceMetrics`] (`PROHIBITION_3`: $\theta_{\mathrm{PS}} \ne \theta_{\mathrm{PO}}$)
//! - [`EProcessMetrics`] (`PROHIBITION_4`: $E_t / K \ge 1/\alpha \ge 20$)
//! - [`ChouldechovaRatioMetrics`] ($\Phi_4$: $\mathrm{FPR}_A / \mathrm{FPR}_B \ne 1$)

/// Structured Value Object for Prohibition 2 (Ban on the Lone Mean) metrics.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NonlinearSummaryMetrics {
    pub f_at_mean: f64,
    pub expected_f: f64,
    pub jensen_gap_delta_j: f64,
    pub median: f64,
    pub q05: f64,
    pub q95: f64,
    pub crps: f64,
}

impl NonlinearSummaryMetrics {
    /// Parses Prohibition 2 metrics from a structured declaration string.
    pub fn parse_from_declaration(declaration: &str) -> Option<Self> {
        let nl = declaration.to_ascii_lowercase();
        for required in [
            "f(e[x])", "e[f(x)]", "jensen", "median", "q05", "q95", "crps",
        ] {
            if !nl.contains(required) {
                return None;
            }
        }
        Some(Self {
            f_at_mean: extract_number_after(&nl, "f(e[x])")?,
            expected_f: extract_number_after(&nl, "e[f(x)]")?,
            jensen_gap_delta_j: extract_number_after(&nl, "delta_j")
                .or_else(|| extract_number_after(&nl, "jensen"))?,
            median: extract_number_after(&nl, "median")?,
            q05: extract_number_after(&nl, "q05")?,
            q95: extract_number_after(&nl, "q95")?,
            crps: extract_number_after(&nl, "crps")?,
        })
    }

    /// Verifies the Jensen Gap identity ($\Delta_J = \mathbb{E}[f(X)] - f(\mathbb{E}[X]) \ne 0$),
    /// strict quantile ordering ($q_{05} < \text{median} < q_{95}$), and $0 < \mathrm{CRPS} < (q_{95} - q_{05})$.
    pub fn is_algebraically_coherent(&self) -> bool {
        let dj_tol = 0.5_f64.max(self.expected_f.abs() * 1e-3);
        self.jensen_gap_delta_j.abs() > 1e-6
            && ((self.expected_f - self.f_at_mean) - self.jensen_gap_delta_j).abs() <= dj_tol
            && self.q05 < self.median
            && self.median < self.q95
            && self.q05 <= self.f_at_mean
            && self.f_at_mean <= self.q95
            && self.q05 <= self.expected_f
            && self.expected_f <= self.q95
            && 0.0 < self.crps
            && self.crps < (self.q95 - self.q05)
    }
}

/// Structured Value Object for Invariant $\Phi_2$ (Moulton-Campbell unequal-cluster design effect).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MoultonClusterMetrics {
    pub weighted_cluster_mean_m_tilde: f64,
    pub threshold_exceedance_icc_rho_i: f64,
}

impl MoultonClusterMetrics {
    /// Parses $\tilde{m}$ and $\rho_I(p)$ from the card's unit-and-margin declarations.
    pub fn parse_from_declarations(m_tilde_decl: &str, icc_decl: &str) -> Option<Self> {
        let m_tilde_lower = m_tilde_decl.to_ascii_lowercase();
        let icc_lower = icc_decl.to_ascii_lowercase();
        if (!m_tilde_decl.contains("CV") && !m_tilde_lower.contains("moulton"))
            || m_tilde_lower.contains("ignored")
        {
            return None;
        }
        if !icc_decl.contains("rho_I") || icc_lower.contains("unadjusted") {
            return None;
        }
        let m_tilde = parse_m_tilde(&m_tilde_lower)?;
        let rho_i = extract_number_after(&icc_lower, "rho_i(0.90) =")
            .or_else(|| extract_number_after(&icc_lower, "rho_i"))?;
        Some(Self {
            weighted_cluster_mean_m_tilde: m_tilde,
            threshold_exceedance_icc_rho_i: rho_i,
        })
    }

    /// Computes $\mathrm{DEFF}_{\text{exc}}(p) = 1 + (\tilde{m} - 1)\rho_I(p)$.
    #[inline]
    pub fn design_effect_deff(&self) -> f64 {
        1.0 + (self.weighted_cluster_mean_m_tilde - 1.0).max(0.0)
            * self.threshold_exceedance_icc_rho_i
    }

    /// Verifies $n_{\mathrm{eff}}(p) = \frac{n}{\mathrm{DEFF}_{\text{exc}}(p)}$ within rounding tolerance `1.5`.
    pub fn is_coherent_with_sample_sizes(&self, nominal_n: usize, effective_n: usize) -> bool {
        if self.weighted_cluster_mean_m_tilde <= 1.0
            || self.threshold_exceedance_icc_rho_i <= 0.0
            || self.threshold_exceedance_icc_rho_i > 1.0
            || effective_n == 0
            || effective_n >= nominal_n
        {
            return false;
        }
        let expected_n_eff = (nominal_n as f64) / self.design_effect_deff();
        ((effective_n as f64) - expected_n_eff).abs() <= 1.5
    }
}

/// Structured Value Object for Prohibition 3 Repeated Risk Minimization contraction ($\frac{\varepsilon \beta}{\gamma} < 1$).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RrmContractionMetrics {
    pub epsilon: f64,
    pub beta: f64,
    pub gamma: f64,
    pub contraction_ratio: f64,
}

impl RrmContractionMetrics {
    /// Parses `(epsilon, beta, gamma, ratio)` from a contraction declaration ending in `< 1`.
    pub fn parse_from_declaration(cond: &str) -> Option<Self> {
        if !cond.contains("< 1") {
            return None;
        }
        let before_lt = cond.split("< 1").next().unwrap_or("");
        let nums: Vec<f64> = before_lt
            .split(|c: char| !(c.is_ascii_digit() || c == '.' || c == '+' || c == '-'))
            .filter(|s| !s.is_empty())
            .filter_map(|s| s.parse::<f64>().ok())
            .filter(|v| v.is_finite())
            .collect();
        if nums.len() >= 4 {
            Some(Self {
                epsilon: nums[0],
                beta: nums[1],
                gamma: nums[2],
                contraction_ratio: nums[3],
            })
        } else {
            None
        }
    }

    /// Checks whether $\frac{\varepsilon \beta}{\gamma} = r \in [0, 1)$ and matches `declared_eps`.
    pub fn satisfies_strict_contraction(&self, declared_eps: f64) -> bool {
        declared_eps.is_finite()
            && declared_eps >= 0.0
            && (self.epsilon - declared_eps).abs() <= 1e-4
            && self.gamma > 0.0
            && self.beta >= self.gamma
            && ((self.epsilon * self.beta / self.gamma) - self.contraction_ratio).abs() <= 1e-2
            && (0.0..1.0).contains(&self.contraction_ratio)
    }
}

/// Structured Value Object for Prohibition 3 Performative Stability vs. Stackelberg Optimality ($\theta_{\mathrm{PS}}$ vs. $\theta_{\mathrm{PO}}$).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PerformativeDivergenceMetrics {
    pub theta_ps: f64,
    pub theta_po: f64,
}

impl PerformativeDivergenceMetrics {
    /// Parses `theta_PS` and `theta_PO` from a stability-optimality divergence declaration.
    pub fn parse_from_declaration(div: &str) -> Option<Self> {
        let lower = div.to_ascii_lowercase();
        Some(Self {
            theta_ps: extract_number_after(&lower, "theta_ps")?,
            theta_po: extract_number_after(&lower, "theta_po")?,
        })
    }

    /// True if $|\theta_{\mathrm{PS}} - \theta_{\mathrm{PO}}| > 10^{-6}$.
    #[inline]
    pub fn has_strict_divergence(&self) -> bool {
        (self.theta_ps - self.theta_po).abs() > 1e-6
    }
}

/// Structured Value Object for Prohibition 4 anytime-valid e-process multiplicity ($E_t / K \ge 1/\alpha$).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EProcessMetrics {
    pub terminal_e_value: f64,
    pub rejection_threshold: f64,
}

impl EProcessMetrics {
    /// Parses `E_t` and `1/alpha` (defaulting to `20.0`) from an e-process declaration.
    pub fn parse_from_declaration(e_str: &str) -> Option<Self> {
        let lower = e_str.to_ascii_lowercase();
        let terminal_e_value = extract_number_after(&lower, "e_")?;
        let rejection_threshold = extract_number_after(&lower, "1/alpha")
            .or_else(|| extract_number_after(&lower, "threshold"))
            .unwrap_or(20.0);
        Some(Self {
            terminal_e_value,
            rejection_threshold,
        })
    }

    /// True if $K \ge 1$, $E_t > 0$, threshold $\ge 20$, and $E_t / K \ge \text{threshold}$.
    #[inline]
    pub fn rejects_under_multiplicity(&self, attempt_counter_k: usize) -> bool {
        self.rejects_under_multiplicity_at(attempt_counter_k, 20.0)
    }

    /// As [`Self::rejects_under_multiplicity`] with a profile's minimum $1/\alpha$ in place of $20$.
    #[inline]
    pub fn rejects_under_multiplicity_at(
        &self,
        attempt_counter_k: usize,
        min_inverse_alpha: f64,
    ) -> bool {
        attempt_counter_k >= 1
            && self.terminal_e_value > 0.0
            && self.rejection_threshold >= min_inverse_alpha
            && (self.terminal_e_value / (attempt_counter_k as f64)) >= self.rejection_threshold
    }
}

/// Structured Value Object for Invariant $\Phi_4$ Chouldechova trilemma audit ($\mathrm{FPR}_A / \mathrm{FPR}_B \ne 1$).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChouldechovaRatioMetrics {
    pub fpr_ratio_a_over_b: f64,
}

impl ChouldechovaRatioMetrics {
    /// Parses `FPR_A/FPR_B` from a `who_pays_for_error` declaration.
    pub fn parse_from_declaration(who_pays: &str) -> Option<Self> {
        let wp = who_pays.to_ascii_lowercase();
        if !wp.contains("chouldechova")
            || !wp.contains("fpr")
            || !wp.contains("ppv")
            || wp.contains("not audited")
            || wp.contains("equally fair")
        {
            return None;
        }
        Some(Self {
            fpr_ratio_a_over_b: extract_number_after(&wp, "fpr_a/fpr_b")?,
        })
    }

    /// True if $|\mathrm{FPR}_A/\mathrm{FPR}_B - 1| > 10^{-3}$.
    #[inline]
    pub fn is_non_unit_ratio(&self) -> bool {
        self.is_non_unit_ratio_beyond(1e-3)
    }

    /// True if $|\mathrm{FPR}_A/\mathrm{FPR}_B - 1| > \text{tolerance}$.
    #[inline]
    pub fn is_non_unit_ratio_beyond(&self, tolerance: f64) -> bool {
        (self.fpr_ratio_a_over_b - 1.0).abs() > tolerance
    }
}

/// Extracts a finite `f64` immediately following `marker_lower` (or after `=` within the same clause).
pub(crate) fn extract_number_after(text_lower: &str, marker_lower: &str) -> Option<f64> {
    let idx = text_lower.find(marker_lower)?;
    let rest = &text_lower[idx + marker_lower.len()..];
    let mut clause_end = rest.find([',', ';']).unwrap_or(rest.len());
    if let Some(arrow_idx) = rest.find("->") {
        clause_end = clause_end.min(arrow_idx);
    }
    let clause = &rest[..clause_end];
    let after_marker = if marker_lower.trim_end().ends_with('=') {
        clause
    } else if let Some(eq_idx) = clause.find('=') {
        &clause[eq_idx + 1..]
    } else {
        clause
    };
    let trimmed = after_marker.trim_start_matches([' ', '\t', ':']);
    let bytes = trimmed.as_bytes();
    let mut i = 0;
    if i < bytes.len() && (bytes[i] == b'+' || bytes[i] == b'-') {
        i += 1;
    }
    let mut has_digits = false;
    while i < bytes.len() && bytes[i].is_ascii_digit() {
        has_digits = true;
        i += 1;
    }
    if i < bytes.len() && bytes[i] == b'.' {
        i += 1;
        while i < bytes.len() && bytes[i].is_ascii_digit() {
            has_digits = true;
            i += 1;
        }
    }
    if !has_digits {
        return None;
    }
    if i < bytes.len() && (bytes[i] == b'e' || bytes[i] == b'E') {
        let mut exp_i = i + 1;
        if exp_i < bytes.len() && (bytes[exp_i] == b'+' || bytes[exp_i] == b'-') {
            exp_i += 1;
        }
        let exp_digits_start = exp_i;
        while exp_i < bytes.len() && bytes[exp_i].is_ascii_digit() {
            exp_i += 1;
        }
        if exp_i > exp_digits_start {
            i = exp_i;
        }
    }
    let val = trimmed[..i].parse::<f64>().ok()?;
    val.is_finite().then_some(val)
}

pub(crate) fn parse_m_tilde(m_tilde_lower: &str) -> Option<f64> {
    let prefix = if let Some(idx) = m_tilde_lower.find("(moulton") {
        &m_tilde_lower[..idx]
    } else {
        m_tilde_lower
    };
    if let Some(last_eq) = prefix.rfind('=') {
        if let Some(v) = extract_number_after(&prefix[last_eq..], "=") {
            return Some(v);
        }
    }
    extract_number_after(m_tilde_lower, "m_tilde")
}

pub(crate) fn parse_seed_from_env(seed_env: &str) -> Option<u64> {
    for part in seed_env.split(';') {
        let t = part.trim();
        if let Some(rest) = t.strip_prefix("seed=") {
            return rest.trim().parse::<u64>().ok();
        }
    }
    None
}
