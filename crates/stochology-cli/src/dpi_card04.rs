//! Digital Public Infrastructure (`card_04_dpi_civil_defense_triage.yaml`) audit combining
//! Meng's (2018) Identity, David Lee's (2009) sharp trimming bounds, cost-loss threshold $p^*$,
//! Chouldechova/KMR demographic disparity, and Bridge $L_2 \to L_3$ `AlertFatigueAudit`.

use crate::csv::CsvTable;
use crate::loaders::load_lalonde_nsw_cps;
use std::path::Path;
use stochology_core::{ChouldechovaAudit, ManskiBounds, MengAudit};
use stochology_lenses::CostLossRule;
use stochology_siga::AlertFatigueAudit;

/// Complete Digital Public Infrastructure (`card_04_dpi_civil_defense_triage.yaml`) audit combining
/// Meng's (2018) Identity ($\rho_{R,Y}$, $n_{\mathrm{eff}}$ in massive administrative registries),
/// David Lee's (2009) sharp trimming bounds (`ManskiBounds::lee_trimming_bounds`),
/// cost-loss threshold $p^* = \frac{C}{\eta L}$, Chouldechova/KMR demographic disparity (`who_pays_for_error`, $\Phi_4$),
/// and Bridge $L_2 \to L_3$ `AlertFatigueAudit` (Turing decibans & $10\text{ bit/s}$ conscious ceiling).
#[derive(Debug, Clone, PartialEq)]
pub struct PublicInfrastructureCard04Audit {
    pub meng_audit: MengAudit,
    pub manski_bounds: ManskiBounds,
    pub lee_trimming_bounds: (f64, f64, f64),
    pub cost_loss_p_star: f64,
    pub chouldechova_fpr_ratio: f64,
    pub kmr_variance_gap: f64,
    pub alert_fatigue_epic_vs_stochological: (AlertFatigueAudit, AlertFatigueAudit),
    pub verified: bool,
}

impl PublicInfrastructureCard04Audit {
    pub fn evaluate_from_disk(root_dir: &Path) -> Self {
        let (nsw_t, nsw_c, cps_c) = load_lalonde_nsw_cps(root_dir);
        let mut pop_y = Vec::with_capacity(nsw_c.len() + cps_c.len());
        let mut rec = Vec::with_capacity(nsw_c.len() + cps_c.len());
        for &y in &cps_c {
            pop_y.push(y);
            rec.push(true);
        }
        for &y in &nsw_c {
            pop_y.push(y);
            rec.push(false);
        }
        let meng_audit = MengAudit::from_population(&rec, &pop_y);
        let manski_bounds = ManskiBounds::compute(&nsw_t, nsw_t.len() + 30, 0.0, 60_000.0);
        let lee_trimming_bounds =
            ManskiBounds::lee_trimming_bounds(&nsw_t, nsw_t.len() + 15, &nsw_c, nsw_c.len() + 30);

        let cost_loss = CostLossRule::new(420.0, 3_500.0, 1.0);
        let cost_loss_p_star = cost_loss.critical_threshold_p_star;

        // Compute vulnerability base rates across protected vs. reference cohorts in cps_observational.csv
        let table =
            CsvTable::open(root_dir, "data/cps_observational.csv").expect("read cps_observational");
        let mut g_a_events = 0usize;
        let mut g_a_total = 0usize;
        let mut g_b_events = 0usize;
        let mut g_b_total = 0usize;
        for row in table.rows() {
            if let (Some(black), Some(hisp), Some(re78)) =
                (row.u32_at(5), row.u32_at(6), row.f64_at(11))
            {
                let vuln = re78 < 1_500.0;
                if black == 1 || hisp == 1 {
                    g_a_total += 1;
                    if vuln {
                        g_a_events += 1;
                    }
                } else {
                    g_b_total += 1;
                    if vuln {
                        g_b_events += 1;
                    }
                }
            }
        }
        let p_a = (g_a_events as f64) / (g_a_total.max(1) as f64);
        let p_b = (g_b_events as f64) / (g_b_total.max(1) as f64);
        let (_, _, chouldechova_fpr_ratio) =
            ChouldechovaAudit::fpr_ratio_under_equal_ppv_and_fnr(p_a, p_b, 0.65, 0.25);
        let kmr =
            ChouldechovaAudit::from_confusion_matrices((180, 90, 630, 100), (70, 55, 825, 50));
        let (_, _, kmr_variance_gap) = kmr.kmr_continuous_impossibility_gap();

        let (epic, stoch_gate) = AlertFatigueAudit::canonical_epic_sepsis_vs_stochological_gate();
        let verified = meng_audit.rho_r_y.abs() > 0.10
            && meng_audit.n_eff_meng < 3_500.0
            && lee_trimming_bounds.0 <= lee_trimming_bounds.1
            && (cost_loss_p_star - 0.12).abs() < 1e-12
            && (chouldechova_fpr_ratio - 1.0).abs() > 0.15
            && kmr_variance_gap > 0.01
            && !epic.approved_bridge_l2_l3
            && stoch_gate.approved_bridge_l2_l3;

        Self {
            meng_audit,
            manski_bounds,
            lee_trimming_bounds,
            cost_loss_p_star,
            chouldechova_fpr_ratio,
            kmr_variance_gap,
            alert_fatigue_epic_vs_stochological: (epic, stoch_gate),
            verified,
        }
    }
}
