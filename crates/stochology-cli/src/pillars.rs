//! Real-data couplers of the six pillar-expansion instruments (`docs/research/pillar-expansion.md`):
//! one instrument per CAPIBA pillar, each run on a sealed dataset in `data/` so that a protocol input
//! moves from **Assumed** to **Measured**.

use std::path::Path;

use stochology_core::{EpsilonDeclarationAudit, PerformativePowerAudit, solve_linear_system};
use stochology_governance::{RealTimeVintageAudit, parse_quarter};
use stochology_lenses::{
    AnchorRegressionAudit, CorpCrpsDecomposition, CorpPavDecomposition, MulticalibrationAudit,
    MulticalibrationVerdict,
};
use stochology_siga::{EBhMultiplicityAudit, EvidenceProcess, PeekingMultiplicitySimulation};

use crate::loaders::{
    TorontoArrest, load_flight_anchor_design, load_flight_daily_exceedances_by_carrier,
    load_lalonde_nsw_cps, load_rtdsm_routput_vintages, load_toronto_arrests,
    load_vic_elec_daily_peaks,
};

/// Same-weekday lags of the 4-member analog ensemble (§3 of the pre-registration).
pub const VIC_ELEC_ANALOG_LAGS: [usize; 4] = [7, 14, 21, 28];
/// Relative bias and spread factor of the sharp-but-biased ensemble (`crps_miscalibration`, §3).
pub const SHARP_BIASED_DISTORTION: (f64, f64) = (-0.03, 0.25);

/// C — Calibrated: CORP decomposition of the CRPS of same-weekday analog ensembles for the daily
/// peak demand in `vic_elec.csv` (Measured), and of the sharp-but-biased distortion of those ensembles.
#[derive(Debug, Clone, PartialEq)]
pub struct VicElecCrpsAudit {
    pub analog: CorpCrpsDecomposition,
    pub sharp_biased: CorpCrpsDecomposition,
}

impl VicElecCrpsAudit {
    pub fn evaluate(root: &Path) -> Self {
        let peaks = load_vic_elec_daily_peaks(root);
        let (ensembles, outcomes) =
            CorpCrpsDecomposition::analog_ensembles(&peaks, &VIC_ELEC_ANALOG_LAGS);
        let (bias, spread) = SHARP_BIASED_DISTORTION;
        let distorted = CorpCrpsDecomposition::distort_ensembles(&ensembles, bias, spread);
        Self {
            analog: CorpCrpsDecomposition::compute(&ensembles, &outcomes),
            sharp_biased: CorpCrpsDecomposition::compute(&distorted, &outcomes),
        }
    }

    /// The defect a lone CRPS hides: the distorted forecast beats climatology yet is inadmissible.
    pub fn lone_crps_hides_miscalibration(&self) -> bool {
        self.sharp_biased.beats_climatology && !self.sharp_biased.admissible
    }

    pub fn identities_closed(&self) -> bool {
        [&self.analog, &self.sharp_biased]
            .iter()
            .all(|d| d.identity_error < 1e-6 && d.integral_error < 1e-6)
    }
}

/// Assumed: FDR level of the carrier hypotheses and of the `multiplicity` simulation (§4).
pub const EBH_ALPHA: f64 = 0.05;

/// A — Anytime-valid: e-BH across carriers flying from New York in both Aug–Dec 2013 and Aug–Dec 2023
/// (Measured, `nyc_flights_2013.csv` and `nyc_flights_2023.csv`). The independent unit is the day ($\Phi_2$):
/// for carrier $k$ and every calendar day $d$ it flew in both years, $X_{k,d} = r_{2023}(d) - r_{2013}(d)$,
/// the difference of the daily shares of arrivals more than 15 minutes late, in calendar order.
/// $H_{0,k}$: $\mathbb{E}[X_{k,d} \mid \text{earlier days}] = 0$ (no change in the daily late-arrival rate).
/// Pairing by calendar day removes the shared August–December seasonality from both years.
#[derive(Debug, Clone, PartialEq)]
pub struct FlightCarrierEbhAudit {
    /// `(carrier, paired days, 2013 rate, 2023 rate)` over the paired days, in carrier order.
    pub carriers: Vec<(String, usize, f64, f64)>,
    pub ebh: EBhMultiplicityAudit,
    /// Carriers the naive rule (running e-process ever crosses $1/\alpha$, $K = 1$ each) would reject.
    pub naive_peeking_rejected: Vec<String>,
    /// Simulated global null of the `multiplicity` proof (seed 2026).
    pub null_simulation: PeekingMultiplicitySimulation,
}

impl FlightCarrierEbhAudit {
    pub fn evaluate(root: &Path) -> Result<Self, String> {
        let d13 = load_flight_daily_exceedances_by_carrier(root, "data/nyc_flights_2013.csv");
        let d23 = load_flight_daily_exceedances_by_carrier(root, "data/nyc_flights_2023.csv");
        let mut carriers = Vec::new();
        let mut labels = Vec::new();
        let mut streams = Vec::new();
        for (carrier, days13) in &d13 {
            let Some(days23) = d23.get(carrier) else {
                continue;
            };
            let mut stream = Vec::new();
            let (mut late13, mut n13, mut late23, mut n23) = (0u32, 0u32, 0u32, 0u32);
            for (day, &(l13, t13)) in days13 {
                let Some(&(l23, t23)) = days23.get(day) else {
                    continue;
                };
                stream.push(f64::from(l23) / f64::from(t23) - f64::from(l13) / f64::from(t13));
                (late13, n13, late23, n23) = (late13 + l13, n13 + t13, late23 + l23, n23 + t23);
            }
            if stream.is_empty() {
                continue;
            }
            carriers.push((
                carrier.clone(),
                stream.len(),
                f64::from(late13) / f64::from(n13),
                f64::from(late23) / f64::from(n23),
            ));
            labels.push(carrier.clone());
            streams.push(stream);
        }
        let mut naive_peeking_rejected = Vec::new();
        for (label, stream) in labels.iter().zip(&streams) {
            if EvidenceProcess::from_grapa_betting(stream, EBH_ALPHA, 1)?.rejects_null_hypothesis {
                naive_peeking_rejected.push(label.clone());
            }
        }
        Ok(Self {
            carriers,
            naive_peeking_rejected,
            ebh: EBhMultiplicityAudit::from_grapa_streams(&labels, &streams, EBH_ALPHA)?,
            null_simulation: PeekingMultiplicitySimulation::simulate(
                2026, 20, 500, 200, EBH_ALPHA, 0.3,
            ),
        })
    }
}

/// Smoothness and strong convexity at which a declared $\varepsilon$ is re-audited in the
/// `understated_epsilon` proof (Assumed: the $\beta / \gamma = 1.5$ of `examples/credit/`).
pub const RRM_BETA_GAMMA: (f64, f64) = (1.5, 1.0);

/// P — Performative-aware: measured lower bound on $\varepsilon$ from the randomized NSW arms (`re78`,
/// `nsw_experimental.csv`, seed 2026). Reading the training assignment as a decision-maker's performative
/// action is an authorial extension; the distance is Measured.
#[derive(Debug, Clone, PartialEq)]
pub struct NswPerformativePowerAudit {
    pub power: PerformativePowerAudit,
    /// A declaration of a passive agent ($\varepsilon = 0$) for the same action.
    pub passive_declaration: EpsilonDeclarationAudit,
}

impl NswPerformativePowerAudit {
    pub fn evaluate(root: &Path) -> Self {
        let (treated, control, _) = load_lalonde_nsw_cps(root);
        let power = PerformativePowerAudit::measure(&treated, &control, 2026);
        let (beta, gamma) = RRM_BETA_GAMMA;
        let passive_declaration = power.audit_declaration(0.0, beta, gamma);
        Self {
            power,
            passive_declaration,
        }
    }
}

/// I — Invariant: anchor regression of arrival delay on departure hour and distance, anchors =
/// origin × month, fitted on Aug–Dec 2013 and scored on Aug–Dec 2023 (Measured, `nyc_flights_2013.csv`,
/// `nyc_flights_2023.csv`); the sealed reserves are not opened. The `anchor_extrapolation` proof uses the
/// simulated confounded model (seed 2026).
#[derive(Debug, Clone, PartialEq)]
pub struct FlightAnchorAudit {
    pub train_rows: usize,
    pub test_rows: usize,
    pub anchor: AnchorRegressionAudit,
    pub simulated_confounded: AnchorRegressionAudit,
}

impl FlightAnchorAudit {
    pub fn evaluate(root: &Path) -> Self {
        let train = load_flight_anchor_design(root, "data/nyc_flights_2013.csv");
        let test = load_flight_anchor_design(root, "data/nyc_flights_2023.csv");
        let sim_train = AnchorRegressionAudit::simulate_confounded(2026, 4_000, 10, 1.0);
        let sim_test = AnchorRegressionAudit::simulate_confounded(2027, 4_000, 10, 3.0);
        Self {
            train_rows: train.y.len(),
            test_rows: test.y.len(),
            anchor: AnchorRegressionAudit::evaluate(&train, &test),
            simulated_confounded: AnchorRegressionAudit::evaluate(&sim_train, &sim_test),
        }
    }
}

/// B — Bitemporal: real-time backtest of US real output growth on the Philadelphia Fed vintages
/// (Measured, `rtdsm_routput_vintages.csv`), decisions 1985Q1–2024Q4.
pub fn real_time_vintage_audit(root: &Path) -> RealTimeVintageAudit {
    let rows = load_rtdsm_routput_vintages(root);
    let first = parse_quarter("1985Q1").expect("valid quarter");
    let last = parse_quarter("2024Q4").expect("valid quarter");
    RealTimeVintageAudit::evaluate(&rows, first, last)
}

/// Logistic regression by Newton-IRLS (25 iterations, ridge $10^{-8}$ on the diagonal).
fn fit_logistic(x: &[Vec<f64>], y: &[f64]) -> Vec<f64> {
    let p = x[0].len();
    let mut b = vec![0.0; p];
    for _ in 0..25 {
        let mut hess = vec![vec![0.0; p]; p];
        let mut grad = vec![0.0; p];
        for (xi, &yi) in x.iter().zip(y) {
            let eta: f64 = xi.iter().zip(&b).map(|(a, w)| a * w).sum();
            let mu = 1.0 / (1.0 + (-eta).exp());
            let w = (mu * (1.0 - mu)).max(1e-12);
            for j in 0..p {
                grad[j] += xi[j] * (yi - mu);
                for k in 0..p {
                    hess[j][k] += w * xi[j] * xi[k];
                }
            }
        }
        for (j, row) in hess.iter_mut().enumerate() {
            row[j] += 1e-8;
        }
        let step =
            solve_linear_system(&hess, &grad).expect("logistic Hessian must be non-singular");
        let size: f64 = step.iter().map(|s| s * s).sum::<f64>().sqrt();
        b.iter_mut().zip(&step).for_each(|(w, s)| *w += s);
        if size < 1e-10 {
            break;
        }
    }
    b
}

/// Features of the colour-blind predictor: intercept, age / 10, male, employed, citizen, checks, year - 2000.
fn toronto_features(a: &TorontoArrest) -> Vec<f64> {
    vec![
        1.0,
        a.age / 10.0,
        f64::from(u8::from(a.male)),
        f64::from(u8::from(a.employed)),
        f64::from(u8::from(a.citizen)),
        a.checks,
        a.year - 2000.0,
    ]
}

/// A — Auditing: multicalibration of a colour-blind release predictor on `toronto_arrests.csv`
/// (Measured): logistic regression without `colour`, fitted and PAV-recalibrated on odd `rownames`,
/// audited on even `rownames` over colour × sex × employed × citizen and every marginal level.
/// The `subgroup_miscalibration` proof uses the simulated hidden-bias predictor (seed 2026).
#[derive(Debug, Clone, PartialEq)]
pub struct TorontoMulticalibrationAudit {
    pub development_rows: usize,
    pub audit_rows: usize,
    pub coefficients: Vec<f64>,
    pub audit: MulticalibrationAudit,
    pub simulated_hidden_bias: MulticalibrationAudit,
}

impl TorontoMulticalibrationAudit {
    pub fn evaluate(root: &Path) -> Self {
        let arrests = load_toronto_arrests(root);
        let (dev, eval): (Vec<&TorontoArrest>, Vec<&TorontoArrest>) =
            arrests.iter().partition(|a| a.row % 2 == 1);
        let outcome = |a: &TorontoArrest| f64::from(u8::from(a.released));
        let x_dev: Vec<Vec<f64>> = dev.iter().map(|a| toronto_features(a)).collect();
        let y_dev: Vec<f64> = dev.iter().map(|a| outcome(a)).collect();
        let b = fit_logistic(&x_dev, &y_dev);
        let predict = |a: &TorontoArrest| {
            let eta: f64 = toronto_features(a).iter().zip(&b).map(|(x, w)| x * w).sum();
            1.0 / (1.0 + (-eta).exp())
        };
        let p_dev: Vec<f64> = dev.iter().map(|a| predict(a)).collect();
        let corp = CorpPavDecomposition::compute(&p_dev, &y_dev);
        let mut map: Vec<(f64, f64)> = p_dev
            .iter()
            .copied()
            .zip(corp.recalibrated_probabilities_q.iter().copied())
            .collect();
        map.sort_by(|a, b| a.0.total_cmp(&b.0));
        let recalibrate = |raw: f64| {
            let k = map.partition_point(|&(p, _)| p <= raw);
            map[k.saturating_sub(1)].1
        };
        let p_eval: Vec<f64> = eval.iter().map(|a| recalibrate(predict(a))).collect();
        let y_eval: Vec<f64> = eval.iter().map(|a| outcome(a)).collect();

        type Trait = (&'static str, &'static str, fn(&TorontoArrest) -> bool);
        let traits: [Trait; 4] = [
            ("Black", "White", |a| a.black),
            ("Male", "Female", |a| a.male),
            ("employed", "not employed", |a| a.employed),
            ("citizen", "not citizen", |a| a.citizen),
        ];
        let mut groups: Vec<(String, Vec<bool>)> = Vec::new();
        for (yes, no, f) in &traits {
            groups.push((yes.to_string(), eval.iter().map(|a| f(a)).collect()));
            groups.push((no.to_string(), eval.iter().map(|a| !f(a)).collect()));
        }
        for mask in 0..16u32 {
            let label: Vec<&str> = traits
                .iter()
                .enumerate()
                .map(|(k, (yes, no, _))| if mask >> k & 1 == 1 { *yes } else { *no })
                .collect();
            let members = eval
                .iter()
                .map(|a| {
                    traits
                        .iter()
                        .enumerate()
                        .all(|(k, (_, _, f))| f(a) == (mask >> k & 1 == 1))
                })
                .collect();
            groups.push((label.join(" × "), members));
        }
        let (sp, sy, sg) = MulticalibrationAudit::simulate_hidden_subgroup_bias(2026, 20_000);
        Self {
            development_rows: dev.len(),
            audit_rows: eval.len(),
            coefficients: b,
            audit: MulticalibrationAudit::audit(&p_eval, &y_eval, &groups),
            simulated_hidden_bias: MulticalibrationAudit::audit(&sp, &sy, &sg),
        }
    }
}

/// The six pillar-expansion audits on real data, evaluated together for `gates` and `labs`.
#[derive(Debug, Clone, PartialEq)]
pub struct PillarExpansionAudit {
    pub calibrated: VicElecCrpsAudit,
    pub anytime: FlightCarrierEbhAudit,
    pub performative: NswPerformativePowerAudit,
    pub invariant: FlightAnchorAudit,
    pub bitemporal: RealTimeVintageAudit,
    pub auditing: TorontoMulticalibrationAudit,
}

impl PillarExpansionAudit {
    pub fn evaluate(root: &Path) -> Result<Self, String> {
        Ok(Self {
            calibrated: VicElecCrpsAudit::evaluate(root),
            anytime: FlightCarrierEbhAudit::evaluate(root)?,
            performative: NswPerformativePowerAudit::evaluate(root),
            invariant: FlightAnchorAudit::evaluate(root),
            bitemporal: real_time_vintage_audit(root),
            auditing: TorontoMulticalibrationAudit::evaluate(root),
        })
    }

    /// Positive checks: every identity closes, every real-data instrument returns its audited verdict,
    /// and every simulated defect is caught. Returns the failed check names (empty when approved).
    pub fn failed_checks(&self) -> Vec<&'static str> {
        let checks = [
            (
                "C: CRPS identity and integral close",
                self.calibrated.identities_closed(),
            ),
            (
                "C: analog ensemble admissible",
                self.calibrated.analog.admissible,
            ),
            (
                "C: lone CRPS hides the sharp-biased miscalibration",
                self.calibrated.lone_crps_hides_miscalibration(),
            ),
            (
                "A: e-BH rejections and discards partition the carriers",
                self.anytime.ebh.rejected.len() + self.anytime.ebh.discarded.len()
                    == self.anytime.ebh.hypotheses_k,
            ),
            (
                "A: naive peeking breaks FDR and e-BH keeps it (simulated)",
                self.anytime.null_simulation.naive_breaks_and_ebh_controls(),
            ),
            (
                "P: NSW arms show measurable performative power",
                self.performative.power.significant,
            ),
            (
                "P: a passive declaration is understated",
                self.performative.passive_declaration.understated,
            ),
            (
                "I: simulated confounding is not anchor-stable",
                !self.invariant.simulated_confounded.anchor_stable,
            ),
            (
                "B: real-time backtest passes the bitemporal clock",
                !self.bitemporal.real_time_clock_alarm,
            ),
            (
                "B: latest-vintage backtest trips the bitemporal clock",
                self.bitemporal.leaky_clock_alarm,
            ),
            (
                "A: simulated hidden subgroup bias is a violation",
                self.auditing.simulated_hidden_bias.verdict == MulticalibrationVerdict::Violated,
            ),
        ];
        checks
            .iter()
            .filter(|(_, ok)| !ok)
            .map(|(name, _)| *name)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resolve_workspace_root;

    #[test]
    fn pillar_expansion_audits_reproduce_the_measured_values() {
        let root = resolve_workspace_root();
        let audit = PillarExpansionAudit::evaluate(&root).expect("bounded payoffs");
        assert!(
            audit.failed_checks().is_empty(),
            "{:?}",
            audit.failed_checks()
        );

        // C — vic_elec.csv: analog CRPS 356.0 MW vs climatology 442.7 MW.
        let c = &audit.calibrated;
        assert_eq!(c.analog.cases, 1_068);
        assert!((c.analog.crps_mean - 355.96).abs() < 0.01);
        assert!((c.analog.unc_uncertainty - 442.72).abs() < 0.01);
        assert!(!c.sharp_biased.admissible && c.sharp_biased.beats_climatology);

        // A — nyc_flights, unit = paired calendar day: 11 carriers, e-BH rejects 9E and HA.
        let a = &audit.anytime;
        assert_eq!(a.ebh.hypotheses_k, 11);
        assert_eq!(a.ebh.rejected, ["9E", "HA"]);
        assert_eq!(a.ebh.discarded.len(), 9);
        assert_eq!(a.naive_peeking_rejected.len(), 5);
        assert_eq!(a.ebh.benjamini_yekutieli_rejected.len(), 1);

        // P — nsw_experimental.csv: W1 / sd_0 = 0.3272 (= |mean gap|), epsilon >= 0.113.
        let p = &audit.performative.power;
        assert!((p.w1_standardized - 0.32721).abs() < 1e-4);
        assert!((p.mean_difference_standardized - p.w1_standardized).abs() < 1e-9);
        assert!((p.epsilon_lower_bound - 0.1133).abs() < 1e-3);

        // I — nyc_flights 2013 -> 2023: anchor-stable at gamma = 5 (gap 0.048).
        let i = &audit.invariant.anchor;
        assert!(i.anchor_stable && (i.relative_slope_gap - 0.0476).abs() < 1e-3);

        // B — rtdsm_routput_vintages.csv: 158 decisions, mean revision +0.32 pp.
        let b = &audit.bitemporal;
        assert_eq!(b.decisions, 158);
        assert!((b.mean_revision - 0.3174).abs() < 1e-3);
        assert!(b.leakage_flattery() > 0.0);

        // A — toronto_arrests.csv: no certified violation, but cells too small to certify.
        let t = &audit.auditing.audit;
        assert_eq!(t.violations, 0);
        assert_eq!(t.verdict, MulticalibrationVerdict::Inconclusive);
    }
}
