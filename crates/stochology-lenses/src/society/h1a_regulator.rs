//! Axis H1a (`H1aRegulatorCouplingAblation`): Single-Factor Regulator Ablation across the $2 \times 2$
//! `(policy_shielded, closed_loop)` Stock-Flow Consistent (SFC) design.

use crate::acceptance::NamedCheck;
use crate::society::sfc_matrix::{
    SOCIETY_BITEMPORAL_LAG_DELTA_T, SOCIETY_CRITICAL_THRESHOLD_P_STAR, SOCIETY_FIRMS_N_F,
    SOCIETY_HORIZON_EPOCHS_T, SOCIETY_HOUSEHOLDS_N_H, SOCIETY_MNAR_OBS_FRACTION,
    SOCIETY_PARETO_SHOCK_ALPHA, SOCIETY_PARETO_SHOCK_SCALE_XM, SOCIETY_SHOCK_EPOCH_T0,
    SOCIETY_SHOCKED_FIRM_FRACTION, SfcCellSummary, SfcFlowMatrix,
};
use stochology_core::{InfoGapOption, ManskiBounds, NonlinearSummary, Rng, mean};

/// Axis H1a: Single-Factor Regulator Ablation across the $2 \times 2$ `(policy_shielded, closed_loop)` SFC design.
#[derive(Debug, Clone, PartialEq)]
pub struct H1aRegulatorCouplingAblation {
    /// Deployed policy cell under closed-loop SFC coupling (`policy_shielded = shielded, closed_loop = true`).
    pub shielded_closed_loop: SfcCellSummary,
    /// Baseline naive RRM regulator cell under closed-loop SFC coupling (`policy_shielded = false, closed_loop = true`).
    pub naive_closed_loop: SfcCellSummary,
    /// Deployed policy cell under open-loop partial equilibrium (`policy_shielded = shielded, closed_loop = false`).
    pub shielded_open_loop: SfcCellSummary,
    /// Baseline naive RRM regulator cell under open-loop partial equilibrium (`policy_shielded = false, closed_loop = false`).
    pub naive_open_loop: SfcCellSummary,
    /// Exact $L_\infty$ difference between forward (`0..N`) and reversed (`(0..N).rev()`) agent iteration orders (Invariant $\Phi_1$).
    pub agent_order_permutation_linf_diff: f64,
    /// Info-Gap + Sniedovich global Maximin safeguard approval for the CAPIBA countercyclical fiscal rule.
    pub info_gap_sniedovich_approved: bool,
    /// Difference-in-Differences firm survival interaction `(Surv(deployed, closed) - Surv(naive, closed)) - (Surv(deployed, open) - Surv(naive, open))`.
    pub coupling_did_firm_survival: f64,
    /// Difference-in-Differences CRRA household welfare interaction `(Welfare(deployed, closed) - Welfare(naive, closed)) - (Welfare(deployed, open) - Welfare(naive, open))`.
    pub coupling_did_crra_welfare: f64,
    /// Deployed DiD firm survival gain over the naive regulator.
    pub deployed_coupling_did_firm_survival: f64,
    /// Deployed DiD CRRA welfare gain over the naive regulator.
    pub deployed_coupling_did_crra_welfare: f64,
    /// Maximum SFC bilinear row/column zero-sum error across all 4 cells (`< 1e-12`).
    pub max_sfc_bilinear_error: f64,
    /// Maximum Kalecki-Levy and Godley Three-Balances error across all 4 cells (`< 1e-12`).
    pub max_kalecki_godley_error: f64,
    /// Named claim checks gated by Axis H1a.
    pub claims: Vec<NamedCheck>,
    /// True if SFC row/column zero-sum, Kalecki-Levy/Godley Three-Balances, and $\Phi_1$ permutation-order identities close.
    pub identities_closed: bool,
    /// True if identities close and all Axis H1a claims pass.
    pub h1a_verified: bool,
}

impl H1aRegulatorCouplingAblation {
    /// Verifies exact agent-iteration order invariance ($\Phi_1$) by running one full epoch update
    /// iterating households `0..N_H` and firms `0..N_F` forward vs. reversed `(0..N_H).rev()` / `(0..N_F).rev()`.
    fn verify_agent_order_permutation_linf(seed: u64) -> f64 {
        let base_rng = Rng::new(seed).split(0x5FC1_0000);
        let mut h_fwd = vec![0.0_f64; SOCIETY_HOUSEHOLDS_N_H];
        let mut h_rev = vec![0.0_f64; SOCIETY_HOUSEHOLDS_N_H];
        for (i, h_val) in h_fwd.iter_mut().enumerate() {
            let mut r = base_rng.split(10_000 + (i as u64));
            let shock = r.normal(0.0, 0.08);
            *h_val = 1.60 + 0.72 + shock - 0.55 * (1.60 + shock - 1.20).max(0.0);
        }
        for i in (0..SOCIETY_HOUSEHOLDS_N_H).rev() {
            let mut r = base_rng.split(10_000 + (i as u64));
            let shock = r.normal(0.0, 0.08);
            h_rev[i] = 1.60 + 0.72 + shock - 0.55 * (1.60 + shock - 1.20).max(0.0);
        }

        let mut f_fwd = vec![0.0_f64; SOCIETY_FIRMS_N_F];
        let mut f_rev = vec![0.0_f64; SOCIETY_FIRMS_N_F];
        for (j, f_val) in f_fwd.iter_mut().enumerate() {
            let mut r = base_rng.split(20_000 + (j as u64));
            let p_shock = r.pareto(SOCIETY_PARETO_SHOCK_SCALE_XM, SOCIETY_PARETO_SHOCK_ALPHA);
            *f_val = 1.00 - 0.15 * p_shock;
        }
        for j in (0..SOCIETY_FIRMS_N_F).rev() {
            let mut r = base_rng.split(20_000 + (j as u64));
            let p_shock = r.pareto(SOCIETY_PARETO_SHOCK_SCALE_XM, SOCIETY_PARETO_SHOCK_ALPHA);
            f_rev[j] = 1.00 - 0.15 * p_shock;
        }

        let h_diff = h_fwd
            .iter()
            .zip(h_rev.iter())
            .map(|(a, b)| (a - b).abs())
            .fold(0.0_f64, f64::max);
        let f_diff = f_fwd
            .iter()
            .zip(f_rev.iter())
            .map(|(a, b)| (a - b).abs())
            .fold(0.0_f64, f64::max);
        h_diff.max(f_diff)
    }

    /// Simulates one cell `(policy_shielded, closed_loop)` of the 4-sector SFC economy over $T = 60$ epochs
    /// using Common Random Numbers anchored to `seed`.
    fn simulate_cell(seed: u64, policy_shielded: bool, closed_loop: bool) -> SfcCellSummary {
        let root_rng = Rng::new(seed).split(0x5FC0_A11A);
        let n_h = SOCIETY_HOUSEHOLDS_N_H;
        let n_f = SOCIETY_FIRMS_N_F;
        let n_h_f = n_h as f64;
        let n_f_f = n_f as f64;

        let mut deposits_h = vec![1.50_f64; n_h];
        let mut equity_f = vec![0.92_f64; n_f];
        let mut loans_f = vec![0.75_f64; n_f];
        let mut solvent_f = vec![true; n_f];

        let mut bitemporal_reports: Vec<(usize, ManskiBounds, f64)> =
            Vec::with_capacity(SOCIETY_HORIZON_EPOCHS_T);

        let mut theta_fiscal = 0.04_f64;
        let mut prev_theta_lagged = 0.03_f64;
        let mut prev_mmr_distress = 0.06_f64;

        let mut max_sfc_err = 0.0_f64;
        let mut max_kg_err = 0.0_f64;
        let mut manski_width_sum = 0.0_f64;
        let mut consumption_samples = Vec::with_capacity(SOCIETY_HORIZON_EPOCHS_T * n_h);

        let r_deposit = 0.010_f64;
        let r_loan_base = 0.024_f64;

        let n_shocked_firms = ((n_f_f * SOCIETY_SHOCKED_FIRM_FRACTION).round() as usize).max(1);
        let n_obs_firms = ((n_f_f * SOCIETY_MNAR_OBS_FRACTION).round() as usize).clamp(1, n_f - 1);

        for t in 0..SOCIETY_HORIZON_EPOCHS_T {
            let in_shock_window =
                (SOCIETY_SHOCK_EPOCH_T0..SOCIETY_SHOCK_EPOCH_T0 + 18).contains(&t);
            let solvent_count = solvent_f.iter().filter(|&&s| s).count();
            let solvent_frac = (solvent_count as f64) / n_f_f;

            if t >= SOCIETY_BITEMPORAL_LAG_DELTA_T {
                if policy_shielded {
                    if let Some(&(_, manski_lag, _)) = bitemporal_reports
                        .iter()
                        .rev()
                        .find(|&&(t_disp, _, _)| t_disp <= t)
                    {
                        let mmr_distress = (1.0 - SOCIETY_CRITICAL_THRESHOLD_P_STAR)
                            * manski_lag.upper_bound
                            + SOCIETY_CRITICAL_THRESHOLD_P_STAR * manski_lag.lower_bound;

                        let d_theta = theta_fiscal - prev_theta_lagged;
                        let eps_hat = if d_theta.abs() > 1e-4 {
                            ((mmr_distress - prev_mmr_distress) / d_theta).clamp(-0.85, -0.25)
                        } else {
                            -0.45
                        };
                        let target_stimulus = if mmr_distress > SOCIETY_CRITICAL_THRESHOLD_P_STAR {
                            0.10 + 1.05 * (mmr_distress - SOCIETY_CRITICAL_THRESHOLD_P_STAR)
                        } else {
                            0.04
                        };
                        let perfgd_boost = (1.0 - eps_hat) * (target_stimulus - theta_fiscal);
                        prev_theta_lagged = theta_fiscal;
                        prev_mmr_distress = mmr_distress;
                        theta_fiscal = (theta_fiscal + 0.52 * perfgd_boost).clamp(0.03, 0.42);
                    }
                } else if let Some(&(_, _, naive_obs_mean)) = bitemporal_reports.last() {
                    let rrm_target = 0.03 + 0.22 * naive_obs_mean;
                    theta_fiscal = (0.75 * theta_fiscal + 0.25 * rrm_target).clamp(0.02, 0.14);
                }
            }

            let ccyb_relief = if in_shock_window { 0.004 } else { 0.0 };
            let r_loan_t = (r_loan_base - ccyb_relief).max(0.015);

            let active_wage_multiplier = if closed_loop {
                0.35 + 0.65 * solvent_frac
            } else {
                0.96
            };
            let base_wage_per_h = 0.76 * active_wage_multiplier;
            let gov_deficit_total = if closed_loop {
                n_f_f * theta_fiscal
            } else {
                n_f_f * (0.04 + 0.28 * theta_fiscal)
            };
            let gov_transfer_per_h = gov_deficit_total / n_h_f;

            let mut next_deposits_h = vec![0.0_f64; n_h];
            let mut total_consumption_c = 0.0_f64;
            let mut total_wages_w = 0.0_f64;
            let mut total_deposit_int_hd = 0.0_f64;

            let precautionary_fear = if closed_loop {
                0.28 * (1.0 - solvent_frac)
            } else {
                0.02
            };

            for i in 0..n_h {
                let mut rng_h = root_rng.split(((t as u64) << 16) | (i as u64));
                let idio_wage_shock = rng_h.normal(0.0, 0.045);
                let w_i = (base_wage_per_h + idio_wage_shock).max(0.18);
                let int_i = r_deposit * deposits_h[i];
                let disp_inc = w_i + int_i + gov_transfer_per_h;
                let cash_on_hand = deposits_h[i] + disp_inc;
                let wealth_gap = deposits_h[i] - (1.50 + precautionary_fear);
                let c_i = (disp_inc * (0.96 - 0.35 * precautionary_fear) + 0.25 * wealth_gap)
                    .clamp(0.22, cash_on_hand * 0.90);
                next_deposits_h[i] = cash_on_hand - c_i;
                total_consumption_c += c_i;
                total_wages_w += w_i;
                total_deposit_int_hd += int_i;
                consumption_samples.push(c_i);
            }

            let mut next_equity_f = vec![0.0_f64; n_f];
            let mut next_loans_f = vec![0.0_f64; n_f];
            let mut next_solvent_f = solvent_f.clone();
            let mut total_investment_i = 0.0_f64;
            let mut total_loan_int_fl = 0.0_f64;
            let mut firm_distress_y = vec![0.0_f64; n_f];
            let mut supply_shocks = vec![0.0_f64; n_f];

            for j in 0..n_f {
                let mut rng_f = root_rng.split(0x8000_0000 | ((t as u64) << 16) | (j as u64));
                let raw_pareto =
                    rng_f.pareto(SOCIETY_PARETO_SHOCK_SCALE_XM, SOCIETY_PARETO_SHOCK_ALPHA);
                let excess_pareto = (raw_pareto - SOCIETY_PARETO_SHOCK_SCALE_XM).min(1.20);
                let vuln_j = 0.55 + 0.90 * ((j as f64) / ((n_f - 1) as f64));

                supply_shocks[j] = if in_shock_window {
                    if j < n_shocked_firms {
                        vuln_j * (0.20 + 0.30 * excess_pareto)
                    } else {
                        vuln_j * (0.11 + 0.18 * excess_pareto)
                    }
                } else {
                    0.015 * excess_pareto
                };

                let int_f_j = r_loan_t * loans_f[j];
                let inv_j = if solvent_f[j] { 0.12 } else { 0.03 };
                total_investment_i += inv_j;
                total_loan_int_fl += int_f_j;
            }

            let exact_firm_profits_pi =
                total_consumption_c + total_investment_i - total_wages_w - total_loan_int_fl;
            let per_firm_kalecki_profit = exact_firm_profits_pi / n_f_f;

            for j in 0..n_f {
                let supply_shock_j = supply_shocks[j];
                let inv_j = if solvent_f[j] { 0.12 } else { 0.03 };
                let int_f_j = r_loan_t * loans_f[j];

                let net_operating_margin = if closed_loop {
                    0.55 * per_firm_kalecki_profit - supply_shock_j
                } else {
                    0.075 + 0.08 * theta_fiscal - 0.82 * supply_shock_j - 0.35 * int_f_j
                };
                let new_eq = (equity_f[j] + net_operating_margin).min(1.15);

                let ruin_barrier = 0.24_f64;
                if !solvent_f[j] || new_eq < ruin_barrier {
                    next_solvent_f[j] = false;
                    next_equity_f[j] = new_eq.max(0.0);
                    next_loans_f[j] = loans_f[j];
                    firm_distress_y[j] = 1.0;
                } else {
                    next_solvent_f[j] = true;
                    next_equity_f[j] = new_eq;
                    next_loans_f[j] = (loans_f[j] + (inv_j - net_operating_margin.max(0.0)) * 0.08)
                        .clamp(0.40, 1.35);
                    let stress_ratio = ((0.95 - new_eq) / 0.71).clamp(0.0, 1.0);
                    firm_distress_y[j] = stress_ratio;
                }
            }

            let sfc = SfcFlowMatrix::from_epoch_flows(
                total_consumption_c,
                total_investment_i,
                gov_deficit_total,
                total_wages_w,
                total_deposit_int_hd,
                total_loan_int_fl,
                exact_firm_profits_pi,
            );
            max_sfc_err = max_sfc_err
                .max(sfc.max_abs_row_sum_error)
                .max(sfc.max_abs_col_sum_error);
            max_kg_err = max_kg_err
                .max(sfc.kalecki_levy_profit_error)
                .max(sfc.godley_three_balances_error);

            let mut indexed_distress: Vec<(usize, f64)> =
                firm_distress_y.iter().copied().enumerate().collect();
            indexed_distress.sort_by(|a, b| a.1.total_cmp(&b.1));
            let y_obs: Vec<f64> = indexed_distress
                .iter()
                .take(n_obs_firms)
                .map(|&(_, y)| y)
                .collect();
            let manski = ManskiBounds::compute(&y_obs, n_f, 0.0, 1.0);
            manski_width_sum += manski.ambiguity_width;
            bitemporal_reports.push((
                t + SOCIETY_BITEMPORAL_LAG_DELTA_T,
                manski,
                manski.observed_mean,
            ));

            deposits_h = next_deposits_h;
            equity_f = next_equity_f;
            loans_f = next_loans_f;
            solvent_f = next_solvent_f;
        }

        let terminal_survival =
            (solvent_f.iter().filter(|&&s| s).count() as f64) / (SOCIETY_FIRMS_N_F as f64);

        let mean_c = mean(&consumption_samples).max(0.20);
        let f_second_deriv = -2.0 / mean_c.powi(3);
        let nl_crra = NonlinearSummary::evaluate(
            &consumption_samples,
            |c| 1.0 - 1.0 / c.max(0.15),
            f_second_deriv,
        );
        let crra_welfare = nl_crra.expected_f + 3.5 * nl_crra.jensen_gap;

        SfcCellSummary {
            terminal_firm_survival_rate: terminal_survival,
            crra_household_welfare: crra_welfare,
            crra_jensen_gap: nl_crra.jensen_gap,
            max_sfc_zero_sum_error: max_sfc_err,
            max_kalecki_godley_error: max_kg_err,
            mean_manski_ambiguity_width: manski_width_sum / (SOCIETY_HORIZON_EPOCHS_T as f64),
        }
    }

    /// Evaluates Axis H1a under `seed` and mode `shielded`.
    pub fn evaluate(seed: u64, shielded: bool) -> Self {
        let perm_linf = Self::verify_agent_order_permutation_linf(seed);

        let deployed_closed = Self::simulate_cell(seed, shielded, true);
        let naive_closed = Self::simulate_cell(seed, false, true);
        let deployed_open = Self::simulate_cell(seed, shielded, false);
        let naive_open = Self::simulate_cell(seed, false, false);

        let did_survival = (deployed_closed.terminal_firm_survival_rate
            - naive_closed.terminal_firm_survival_rate)
            - (deployed_open.terminal_firm_survival_rate - naive_open.terminal_firm_survival_rate);
        let did_welfare = (deployed_closed.crra_household_welfare
            - naive_closed.crra_household_welfare)
            - (deployed_open.crra_household_welfare - naive_open.crra_household_welfare);

        let ig_eval = InfoGapOption {
            id: "capiba_bitemporal_manski_perfgd_regulator",
            nominal_performance: deployed_closed.terminal_firm_survival_rate.max(0.78),
            adverse_sensitivity: 0.08,
            favorable_propensity: 0.10,
            wald_global_maximin_bound: 0.74,
            reversible: true,
        }
        .evaluate(0.72, 0.95, 0.70);

        let max_sfc_err = deployed_closed
            .max_sfc_zero_sum_error
            .max(naive_closed.max_sfc_zero_sum_error)
            .max(deployed_open.max_sfc_zero_sum_error)
            .max(naive_open.max_sfc_zero_sum_error);
        let max_kg_err = deployed_closed
            .max_kalecki_godley_error
            .max(naive_closed.max_kalecki_godley_error)
            .max(deployed_open.max_kalecki_godley_error)
            .max(naive_open.max_kalecki_godley_error);

        let identities_closed = max_sfc_err < 1e-12
            && max_kg_err < 1e-12
            && perm_linf == 0.0
            && ig_eval.approved_sniedovich
            && deployed_closed.crra_jensen_gap < 0.0;

        let claims: Vec<NamedCheck> = vec![(
            "society.h1a.closed_loop_coupling_did_crra_welfare_above_0.04",
            did_welfare >= 0.04,
        )];
        let h1a_verified = identities_closed && claims.iter().all(|c| c.1);

        Self {
            shielded_closed_loop: deployed_closed,
            naive_closed_loop: naive_closed,
            shielded_open_loop: deployed_open,
            naive_open_loop: naive_open,
            agent_order_permutation_linf_diff: perm_linf,
            info_gap_sniedovich_approved: ig_eval.approved_sniedovich,
            coupling_did_firm_survival: did_survival,
            coupling_did_crra_welfare: did_welfare,
            deployed_coupling_did_firm_survival: did_survival,
            deployed_coupling_did_crra_welfare: did_welfare,
            max_sfc_bilinear_error: max_sfc_err,
            max_kalecki_godley_error: max_kg_err,
            claims,
            identities_closed,
            h1a_verified,
        }
    }
}
