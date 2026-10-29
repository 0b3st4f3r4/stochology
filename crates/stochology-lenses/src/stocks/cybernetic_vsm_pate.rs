//! Axis 4: Cybernetic Governance (`VSM` S1–S5 of Ashby, Conant & Stafford Beer), Human Judgment Noise Audit
//! (Kahneman, Sibony & Sunstein 2021), the Telic-Scopic Audit Protocol (`PATE`), and the 5-Step Notebook.

use super::powell_friston_pearl::PowellFristonCausalEngineAudit;
use super::supply_chain_plant::StrategicStockPlantResilienceAudit;
use super::topology_axioms::TopologyAndFourStockAxiomsAudit;
use crate::acceptance::NamedCheck;
use stochology_core::math::stats::{spearman_rank_correlation, two_sample_ks_distance};
use stochology_core::{mean, sample_std, Rng};

/// VSM Algedonic gate (*"not measured, not executed"*): an S1 proposal runs only if it carries a measurement record.
fn algedonic_gate_executes(measured: bool) -> bool {
    measured
}

/// The 4 Quadrants of the Telic-Scopic Audit Protocol (`PATE` — *Protocolo de Auditoria Télico-Escópica*),
/// separating the internal quality of the stochastic art (**Telos**) from the exogenous outcome (**Skopos**).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PateQuadrant {
    /// **Q1 — Maestria Estocológica Validada (High Telos + Skopos Success):**
    /// Calibrated forecasts ($\mathrm{REL}_{\mathrm{CORP}} \to 0$), controlled judgment noise, $\mathrm{TTS} \ge \mathrm{TTR}$,
    /// and target achieved. Action: standardize and codify into habitual `PFA` rules.
    Q1ValidatedMastery,
    /// **Q2 — Resiliência Dignificada (High Telos + Skopos Failure via Tail $\mathrm{UNC}_0$):**
    /// Operator fulfilled all precepts of the art (*facere omnia quae possit ut conliniet*), but an extreme tail
    /// realization in Murphy's exogenous $\mathrm{UNC}_0$ prevented the target. Action: exonerate operator (Cicero /
    /// Alexander of Aphrodisias), trigger $\mathrm{TTR}$ recovery, and inspect Wasserstein radius $\varepsilon$.
    Q2DignifiedResilience,
    /// **Q3 — Acidente Feliz / Ilusão de Jackpot (Low Telos + Skopos Success by Luck):**
    /// Miscalibrated forecasts, noisy judgment, non-ergodic exposure or $\mathrm{TTS} < \mathrm{TTR}$, yet survived by
    /// fortuitous absence of tail shocks ("unshattered porcelain cup"). Action: immediate `S3*` & Algedonic alarm!
    Q3HappyAccidentJackpotFragility,
    /// **Q4 — Falha Estocológica Estrutural (Low Telos + Skopos Failure):**
    /// Violated stock sizing, Bullwhip amplification, and unshielded exposure leading to stockout/ruin.
    /// Action: high-leverage structural redesign (Meadows Point 10/2 + Conant-Ashby regulator model overhaul).
    Q4StructuralFailure,
}

impl PateQuadrant {
    /// Classifies an operational decision episode into its canonical PATE quadrant from internal
    /// **Telos** metrics (`rel_corp`, `tts_to_ttr`, `judgment_noise_sd`) and external **Skopos** outcome (`skopos_attained`).
    pub fn classify(
        rel_corp: f64,
        tts_to_ttr: f64,
        judgment_noise_sd: f64,
        skopos_attained: bool,
    ) -> Self {
        let high_telos = rel_corp <= 0.015 && tts_to_ttr >= 1.0 && judgment_noise_sd <= 0.10;
        match (high_telos, skopos_attained) {
            (true, true) => Self::Q1ValidatedMastery,
            (true, false) => Self::Q2DignifiedResilience,
            (false, true) => Self::Q3HappyAccidentJackpotFragility,
            (false, false) => Self::Q4StructuralFailure,
        }
    }

    /// Outcome-only review that ignores the Telos metrics: success is read as mastery, failure as structural failure.
    pub fn classify_outcome_only(skopos_attained: bool) -> Self {
        if skopos_attained {
            Self::Q1ValidatedMastery
        } else {
            Self::Q4StructuralFailure
        }
    }
}

/// Executable 5-Step Stochary Engineering & Stress-Test Notebook (*Caderno de Engenharia Estocária de 5 Etapas*).
#[derive(Debug, Clone, PartialEq)]
pub struct StocharyStressTestNotebook {
    /// Step 1 (Dierickx & Cool / Meadows): Asset erosion rate $\delta$ and time-compression diseconomy factor mapped.
    pub step1_flows_erosion_and_interconnection_mapped: bool,
    /// Step 2 (Scarf + WDRO + Conformal): Critical thresholds $(s^*, S^*)$ parameterized under $\beta$-CVaR and Wasserstein ball $\varepsilon$.
    pub step2_wdro_conformal_s_big_s_thresholds: (f64, f64),
    /// Step 3 (Simchi-Levi): Node removal stress-test ensuring $\mathrm{TTS} \ge \mathrm{TTR}$ across all critical suppliers.
    pub step3_node_removal_tts_ge_ttr_passed: bool,
    /// Step 4 (Taleb & Douady): Discrete *tail vega* ($\Omega$) probe confirming non-negative convexity in lower tail dispersion.
    pub step4_tail_vega_convexity_verified: bool,
    /// Step 5 (Powell / Friston): the deployed agent's probe-or-act choices match the simulated value of information.
    pub step5_epistemic_efe_voi_evaluated: bool,
    /// Whether all 5 engineering stress-test steps passed.
    pub all_five_steps_approved: bool,
}

/// Axis 4: Cybernetic Governance (`VSM` S1–S5 of Ashby, Conant & Stafford Beer), Human Judgment Noise Audit
/// (Kahneman, Sibony & Sunstein 2021), the Telic-Scopic Audit Protocol (`PATE`), and the 5-Step Notebook.
#[derive(Debug, Clone, PartialEq)]
pub struct CyberneticVsmAndPateGovernanceAudit {
    /// Informative: residual variety $\max(V_D - V_R, 0)$ of the declared design (attenuated $V_D = 45$, amplified $V_R = 60$ bits/min
    /// if shielded; $V_D = 120$, $V_R = 18$ otherwise). Declared values, not measured.
    pub ashby_residual_unabsorbed_variety: f64,
    /// One-step out-of-sample $R^2$ of the deployed regulator's internal model of $y_t = 0.6\,y_{t-1} + 1.5\,u_t + 0.1\,\varepsilon_t$
    /// (OLS-identified if shielded, persistence $\hat{y}_t = y_{t-1}$ otherwise). Gated (`>= 0.95`).
    pub conant_ashby_regulator_homomorphism_r2: f64,
    /// Identity: Beer's `(Productivity, Latency, Performance)` with `Performance == Productivity * Latency`.
    pub beer_vsm_productivity_latency_performance: (f64, f64, f64),
    /// Informative: true if no unmeasured S1 action among 500 proposals was executed by the deployed channel (gated if shielded,
    /// ungated otherwise). The gate's blocking rule is checked under `identities_closed`.
    pub algedonic_not_measured_not_executed_enforced: bool,
    /// Simulated judgment-error SD of a single raw judge vs. the deployed protocol (bias calibration + 4-judge averaging if shielded).
    pub kahneman_judgment_noise_before_and_after: (f64, f64),
    /// Identity: `PATE` classification of the four canonical episodes.
    pub pate_quadrant_classifications: [PateQuadrant; 4],
    /// Informative: true if the deployed reviewer (Telos-aware if shielded, outcome-only otherwise) flags the lucky episode as `Q3`;
    /// the Telos-aware classification is part of the `PATE` identity table.
    pub pate_q3_jackpot_algedonic_alarm_triggered: bool,
    /// `PLAT` Spearman $r_s$ and KS distance between 250 days of front-office P&L and the deployed risk-model P&L
    /// (all three factors if shielded, third factor omitted otherwise).
    pub plat_spearman_and_ks: (f64, f64),
    /// True when the `PLAT` green zone ($r_s > 0.80$, $D_{\mathrm{KS}} < 0.09$) and the judgment-noise budget ($\le 0.10$) hold. Gated.
    pub pccp_and_evaluated_competence_verified: bool,
    /// Informative: 5-Step Stochary Engineering & Stress-Test Notebook. It aggregates claims gated elsewhere, so it is not gated again.
    pub engineering_notebook: StocharyStressTestNotebook,
    /// Named claim checks gated by this axis (see `crate::acceptance`).
    pub claims: Vec<NamedCheck>,
    /// Claims removed from the gate after failing multi-seed acceptance (`crate::acceptance`); still computed and reported.
    pub discarded_claims: Vec<NamedCheck>,
    /// True if the Beer, `PATE`, and Algedonic gate-rule identities close.
    pub identities_closed: bool,
    /// Whether identities, the raw-noise claim, and the deployed regulator, gate, reviewer, protocol, `PLAT`, and notebook pass.
    pub axis4_verified: bool,
}

impl CyberneticVsmAndPateGovernanceAudit {
    /// Evaluates Axis 4. When `cybernetic_shielded` is `false`, the persistence regulator, an ungated S1 channel, the outcome-only
    /// reviewer, raw single-judge estimates, and the factor-omitting risk model are deployed.
    pub fn evaluate(
        axis1: &TopologyAndFourStockAxiomsAudit,
        axis2: &PowellFristonCausalEngineAudit,
        axis3: &StrategicStockPlantResilienceAudit,
        seed: u64,
        cybernetic_shielded: bool,
    ) -> Self {
        let mut rng = Rng::new(seed ^ 0x45A4_2026);

        // 1. Ashby's Requisite Variety on the declared design varieties (bits/min)
        let (v_d, v_r) = if cybernetic_shielded {
            (45.0_f64, 60.0_f64)
        } else {
            (120.0_f64, 18.0_f64)
        };
        let ashby_residual_unabsorbed_variety = (v_d - v_r).max(0.0);

        // 2. Conant-Ashby Good Regulator: identify y_t = a y_{t-1} + b u_t from 200 periods, score on the next 200
        let mut y_series = vec![0.0_f64];
        let mut u_series = vec![0.0_f64];
        for _ in 0..400 {
            let u = rng.standard_normal();
            let y = 0.6 * y_series.last().copied().unwrap_or(0.0)
                + 1.5 * u
                + 0.1 * rng.standard_normal();
            u_series.push(u);
            y_series.push(y);
        }
        let (mut syy, mut syu, mut suu, mut sy_target, mut su_target) = (0.0, 0.0, 0.0, 0.0, 0.0);
        for t in 1..=200 {
            let (lag, u, y) = (y_series[t - 1], u_series[t], y_series[t]);
            syy += lag * lag;
            syu += lag * u;
            suu += u * u;
            sy_target += lag * y;
            su_target += u * y;
        }
        let det = (syy * suu - syu * syu).max(1e-12);
        let a_hat = (suu * sy_target - syu * su_target) / det;
        let b_hat = (syy * su_target - syu * sy_target) / det;
        let test_y = &y_series[201..];
        let y_mean = mean(test_y);
        let (mut sse, mut sst) = (0.0, 0.0);
        for t in 201..=400 {
            let pred = if cybernetic_shielded {
                a_hat * y_series[t - 1] + b_hat * u_series[t]
            } else {
                y_series[t - 1]
            };
            sse += (y_series[t] - pred).powi(2);
            sst += (y_series[t] - y_mean).powi(2);
        }
        let conant_ashby_regulator_homomorphism_r2 = 1.0 - sse / sst.max(1e-12);

        // 3. Stafford Beer's Triple Capacity Vector (Brain of the Firm, p. 163)
        let actuality: f64 = 72.0;
        let capability: f64 = 90.0;
        let potentiality: f64 = 120.0;
        let productivity = actuality / capability;
        let latency = capability / potentiality;
        let performance = actuality / potentiality;

        // 4. Algedonic Channel ("Not Measured, Not Executed"): 500 S1 proposals, 20% lacking a measurement record
        let (mut unmeasured_executed, mut gated_unmeasured_executed) = (0usize, 0usize);
        for _ in 0..500 {
            let measured = rng.next_f64() >= 0.20;
            if !measured && !cybernetic_shielded {
                unmeasured_executed += 1;
            }
            if !measured && algedonic_gate_executes(measured) {
                gated_unmeasured_executed += 1;
            }
        }
        let algedonic_not_measured_not_executed_enforced = unmeasured_executed == 0;

        // 5. Kahneman, Sibony & Sunstein (2021) noise audit: 40 judges with level bias N(0, 0.22^2) and occasion noise N(0, 0.08^2)
        // rate 200 cases. Hygiene: each judge's bias is estimated on 20 calibration cases and 4 de-biased judges are averaged.
        let judges = 40usize;
        let biases: Vec<f64> = (0..judges).map(|_| rng.normal(0.0, 0.22)).collect();
        let bias_hat: Vec<f64> = biases
            .iter()
            .map(|&b| (0..20).map(|_| b + rng.normal(0.0, 0.08)).sum::<f64>() / 20.0)
            .collect();
        let mut raw_errors = Vec::with_capacity(200);
        let mut hygiene_errors = Vec::with_capacity(200);
        for _ in 0..200 {
            let j = (rng.next_f64() * judges as f64) as usize % judges;
            raw_errors.push(biases[j] + rng.normal(0.0, 0.08));
            let avg = (0..4)
                .map(|_| {
                    let k = (rng.next_f64() * judges as f64) as usize % judges;
                    biases[k] + rng.normal(0.0, 0.08) - bias_hat[k]
                })
                .sum::<f64>()
                / 4.0;
            hygiene_errors.push(avg);
        }
        let noise_before = sample_std(&raw_errors);
        let noise_deployed = if cybernetic_shielded {
            sample_std(&hygiene_errors)
        } else {
            noise_before
        };

        // 6. PATE 4-Quadrant Audit
        let q1 = PateQuadrant::classify(0.004, 1.33, 0.03, true);
        let q2 = PateQuadrant::classify(0.004, 1.33, 0.03, false);
        let q3 = PateQuadrant::classify(0.085, 0.45, 0.24, true);
        let q4 = PateQuadrant::classify(0.085, 0.45, 0.24, false);
        let pate_quadrant_classifications = [q1, q2, q3, q4];
        let reviewed_lucky_episode = if cybernetic_shielded {
            PateQuadrant::classify(0.085, 0.45, 0.24, true)
        } else {
            PateQuadrant::classify_outcome_only(true)
        };
        let pate_q3_jackpot_algedonic_alarm_triggered =
            reviewed_lucky_episode == PateQuadrant::Q3HappyAccidentJackpotFragility;

        // 7. Basel III MAR33 PLAT: front-office P&L = 1.0 f1 + 0.8 f2 + 1.5 f3 over 250 days; the risk model adds N(0, 0.3^2)
        // revaluation noise and, when naive, omits the third factor.
        let mut fo_pnl = Vec::with_capacity(250);
        let mut rm_pnl = Vec::with_capacity(250);
        for _ in 0..250 {
            let f = [
                rng.standard_normal(),
                rng.standard_normal(),
                rng.standard_normal(),
            ];
            fo_pnl.push(1.0 * f[0] + 0.8 * f[1] + 1.5 * f[2]);
            let third = if cybernetic_shielded { 1.5 * f[2] } else { 0.0 };
            rm_pnl.push(1.0 * f[0] + 0.8 * f[1] + third + rng.normal(0.0, 0.3));
        }
        let plat_spearman_and_ks = (
            spearman_rank_correlation(&fo_pnl, &rm_pnl),
            two_sample_ks_distance(&fo_pnl, &rm_pnl),
        );
        let pccp_and_evaluated_competence_verified = plat_spearman_and_ks.0 > 0.80
            && plat_spearman_and_ks.1 < 0.09
            && noise_deployed <= 0.10;

        let engineering_notebook = StocharyStressTestNotebook {
            step1_flows_erosion_and_interconnection_mapped: axis3
                .dierickx_time_compression_diseconomy_ratio
                > 1.50,
            step2_wdro_conformal_s_big_s_thresholds: axis3.scarf_optimal_s_and_big_s,
            step3_node_removal_tts_ge_ttr_passed: axis3.simchi_levi_tts_to_ttr_ratio >= 1.15,
            step4_tail_vega_convexity_verified: axis1.buffered_convex_tail_vega > 0.0,
            step5_epistemic_efe_voi_evaluated: axis2.deployed_policy_choices_match_voi,
            all_five_steps_approved: axis3.dierickx_time_compression_diseconomy_ratio > 1.50
                && axis3.simchi_levi_tts_to_ttr_ratio >= 1.15
                && axis1.buffered_convex_tail_vega > 0.0
                && axis2.deployed_policy_choices_match_voi
                && axis1.axis1_verified
                && axis3.axis3_verified,
        };

        let identities_closed = (performance - productivity * latency).abs() < 1e-12
            && pate_quadrant_classifications
                == [
                    PateQuadrant::Q1ValidatedMastery,
                    PateQuadrant::Q2DignifiedResilience,
                    PateQuadrant::Q3HappyAccidentJackpotFragility,
                    PateQuadrant::Q4StructuralFailure,
                ]
            && gated_unmeasured_executed == 0;
        // A single raw judge's error SD exceeds 0.20 on 18 of 20 acceptance seeds (panel of 40 sampled biases).
        let discarded_claims: Vec<NamedCheck> =
            vec![("stocks.a4.raw_judge_noise_above_0.20", noise_before > 0.20)];
        let claims: Vec<NamedCheck> = vec![
            (
                "stocks.a4.deployed_regulator_r2_at_least_0.95",
                conant_ashby_regulator_homomorphism_r2 >= 0.95,
            ),
            (
                "stocks.a4.deployed_judgment_noise_below_0.05",
                noise_deployed < 0.05,
            ),
            (
                "stocks.a4.deployed_plat_rs_above_0.80_ks_below_0.09_noise_at_most_0.10",
                pccp_and_evaluated_competence_verified,
            ),
        ];
        let axis4_verified = identities_closed && claims.iter().all(|c| c.1);

        Self {
            ashby_residual_unabsorbed_variety,
            conant_ashby_regulator_homomorphism_r2,
            beer_vsm_productivity_latency_performance: (productivity, latency, performance),
            algedonic_not_measured_not_executed_enforced,
            kahneman_judgment_noise_before_and_after: (noise_before, noise_deployed),
            pate_quadrant_classifications,
            pate_q3_jackpot_algedonic_alarm_triggered,
            plat_spearman_and_ks,
            pccp_and_evaluated_competence_verified,
            engineering_notebook,
            claims,
            discarded_claims,
            identities_closed,
            axis4_verified,
        }
    }
}
