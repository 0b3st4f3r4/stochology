//! Economic Cost-Loss Rule ($p^* = \frac{C}{\eta L}$), Expected Value of Perfect Information ($\mathrm{EVPI}$),
//! Relative Economic Value, and Strictly Proper Scoring Rules (`logarithmic_score`, `quantile_score`, `brier_score`).

/// Computes the critical probability threshold $p^* = \frac{C}{\eta L}$.
#[inline]
pub fn critical_cost_loss_threshold(cost_c: f64, loss_l: f64, efficacy_eta: f64) -> f64 {
    assert!(
        cost_c > 0.0 && loss_l > 0.0 && efficacy_eta > 0.0 && efficacy_eta <= 1.0,
        "requires C > 0, L > 0, and eta in (0, 1]"
    );
    cost_c / (efficacy_eta * loss_l)
}

/// Economic Cost-Loss Rule and Expected Value of Perfect Information ($\mathrm{EVPI}$).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CostLossRule {
    pub protection_cost_c: f64,
    pub event_loss_l: f64,
    pub efficacy_eta: f64,
    pub critical_threshold_p_star: f64,
}

impl CostLossRule {
    pub fn new(protection_cost_c: f64, event_loss_l: f64, efficacy_eta: f64) -> Self {
        let p_star = critical_cost_loss_threshold(protection_cost_c, event_loss_l, efficacy_eta);
        Self {
            protection_cost_c,
            event_loss_l,
            efficacy_eta,
            critical_threshold_p_star: p_star,
        }
    }

    /// Returns `"protect"` if $p > p^*$ else `"do_not_protect"`.
    #[inline]
    pub fn recommend_action(&self, probability_p: f64) -> &'static str {
        if probability_p > self.critical_threshold_p_star {
            "protect"
        } else {
            "do_not_protect"
        }
    }

    /// Computes the Expected Value of Perfect Information ($\mathrm{EVPI}$) per unit and across `volume_units`:
    /// - If $p \le p^*$: $\mathrm{EVPI}_{\text{unit}} = p (\eta L - C)$;
    /// - If $p > p^*$: $\mathrm{EVPI}_{\text{unit}} = (1 - p) C$.
    pub fn calculate_evpi(&self, prior_probability_p: f64, volume_units: f64) -> f64 {
        let effective_gain =
            (self.efficacy_eta * self.event_loss_l - self.protection_cost_c).max(0.0);
        let unit_evpi = if prior_probability_p <= self.critical_threshold_p_star {
            prior_probability_p * effective_gain
        } else {
            (1.0 - prior_probability_p) * self.protection_cost_c
        };
        unit_evpi * volume_units.max(0.0)
    }

    /// Computes Richardson's (2000) Relative Economic Value $V(p^*) \in (-\infty, 1]$ of a probabilistic
    /// forecast series `forecasts_p` against binary outcomes `outcomes_y`:
    /// $$V(p^*) = \frac{E_{\text{climatology}} - E_{\text{forecast}}}{E_{\text{climatology}} - E_{\text{perfect}}}$$
    /// where $V = 1$ is a perfect oracle and $V = 0$ is naive climatological base-rate action.
    pub fn relative_economic_value(&self, forecasts_p: &[f64], outcomes_y: &[f64]) -> f64 {
        assert_eq!(forecasts_p.len(), outcomes_y.len());
        assert!(!forecasts_p.is_empty());
        let n = forecasts_p.len() as f64;
        let pi_bar = outcomes_y.iter().sum::<f64>() / n;
        let residual_loss = (1.0 - self.efficacy_eta) * self.event_loss_l;

        let e_perfect = pi_bar * (self.protection_cost_c + residual_loss).min(self.event_loss_l);
        let e_clim = if pi_bar > self.critical_threshold_p_star {
            self.protection_cost_c + pi_bar * residual_loss
        } else {
            pi_bar * self.event_loss_l
        };

        let mut e_forecast = 0.0;
        for (&p, &y) in forecasts_p.iter().zip(outcomes_y.iter()) {
            if p > self.critical_threshold_p_star {
                e_forecast += self.protection_cost_c + y * residual_loss;
            } else {
                e_forecast += y * self.event_loss_l;
            }
        }
        e_forecast /= n;

        let denom = e_clim - e_perfect;
        if denom.abs() < 1e-12 {
            0.0
        } else {
            (e_clim - e_forecast) / denom
        }
    }
}

/// Strictly proper Logarithmic (Ignorance) Score with Cromwell floor $\varepsilon_{\text{floor}} \in (0, 0.5)$.
pub fn logarithmic_score(forecasts_p: &[f64], outcomes_y: &[f64], epsilon_floor: f64) -> f64 {
    assert_eq!(forecasts_p.len(), outcomes_y.len());
    assert!(!forecasts_p.is_empty());
    let n = forecasts_p.len() as f64;
    forecasts_p
        .iter()
        .zip(outcomes_y.iter())
        .map(|(&p, &y)| {
            let pc = stochology_core::apply_cromwell_floor(p, epsilon_floor);
            -(y * pc.ln() + (1.0 - y) * (1.0 - pc).ln())
        })
        .sum::<f64>()
        / n
}

/// Strictly proper Quantile Score (pinball loss) at level $\alpha \in (0, 1)$ (§1.3.5):
/// $$\mathrm{QS}_\alpha(q_\alpha, y) = (\mathbb{I}\{y \le q_\alpha\} - \alpha)(q_\alpha - y)$$
#[inline]
pub fn quantile_score(alpha: f64, forecast_q: f64, observation_y: f64) -> f64 {
    assert!(alpha > 0.0 && alpha < 1.0);
    let ind = if observation_y <= forecast_q {
        1.0
    } else {
        0.0
    };
    (ind - alpha) * (forecast_q - observation_y)
}

/// Mean Brier Score $\mathrm{BS} = \frac{1}{n}\sum_{i=1}^n (p_i - y_i)^2$.
pub fn brier_score(forecasts_p: &[f64], outcomes_y: &[f64]) -> f64 {
    assert_eq!(forecasts_p.len(), outcomes_y.len());
    assert!(!forecasts_p.is_empty());
    forecasts_p
        .iter()
        .zip(outcomes_y.iter())
        .map(|(&p, &y)| (p - y).powi(2))
        .sum::<f64>()
        / (forecasts_p.len() as f64)
}
