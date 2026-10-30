//! `stochology-cli` — Real empirical dataset loaders (`data/*.csv`), Strategy-based card ground-truth
//! auditor (`CardDatasetStrategy`), CAPIBA defect registry (`ALL_DEFECTS`), and un-clamped cluster-permutation severe test (`seed = 2026`).

pub mod card_audit;
pub mod competencies;
pub mod csv;
pub mod defects;
pub mod dpi_card04;
pub mod error;
pub mod loaders;

pub use card_audit::{
    CardDatasetStrategy, ColumnMappedCsvStrategy, EmpiricalDatasetAudit, ExtractedCardDataset,
    compute_empirical_dataset_audit, compute_empirical_dataset_audit_with_profile,
    compute_unclamped_cluster_permutation_null, compute_ville_betting_e_values,
    evaluate_empirical_card_severe_test, evaluate_empirical_card_severe_test_with_profile,
    verify_card_against_empirical_data, verify_card_against_empirical_data_with_profile,
};
pub use competencies::{
    BookChapter, BookIndex, CompetencyDag, CompetencyDepth, CompetencyNode, Pathway,
    PracticumThread, replace_marked_block,
};
pub use csv::{CsvRow, CsvTable};
pub use defects::{ALL_DEFECTS, CapibaPillar, DefectDescriptor, all_defect_names};
pub use dpi_card04::PublicInfrastructureCard04Audit;
pub use error::CliError;
pub use loaders::{
    CANONICAL_CARD_PATHS, load_atlantic_storms_decade_environments, load_atlantic_storms_forecasts,
    load_bank_call_clusters, load_cards_from_disk, load_cps_cohort_clusters, load_danish_fire,
    load_djia_returns, load_flight_stream, load_lalonde_nsw_cps, load_nyc_flights_2013_2023,
    load_nyc_flights_cascade_2013_2023, load_oot_ood_flight_reserves, load_sha256_manifest_at,
    load_sha256sums_manifest, load_toronto_arrests_prevalences, load_vic_elec_demand,
    resolve_workspace_root,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn csv_loaders_read_real_datasets_in_data() {
        let root_buf = resolve_workspace_root();
        let root = root_buf.as_path();
        let losses = load_danish_fire(root);
        assert!(losses.len() > 2_000);
        let bank = load_bank_call_clusters(root);
        assert!(bank.len() > 150);
        let (nsw_t, nsw_c, cps_c) = load_lalonde_nsw_cps(root);
        assert!(nsw_t.len() + nsw_c.len() > 400 && cps_c.len() > 15_000);
        let vic = load_vic_elec_demand(root);
        assert_eq!(vic.len(), 52_608);
        let (x13_h1, y13_h1, x23_h1, y23_h1, x_hou, y_hou) = load_oot_ood_flight_reserves(root);
        assert!(x13_h1.len() > 1_400 && x13_h1.len() == y13_h1.len());
        assert!(x23_h1.len() > 1_400 && x23_h1.len() == y23_h1.len());
        assert!(x_hou.len() > 1_400 && x_hou.len() == y_hou.len());

        let djia = load_djia_returns(root);
        let (storm_p, storm_y) = load_atlantic_storms_forecasts(root);
        let emp_stoch = stochology_lenses::EmpiricalStocharyCyberneticAudit::evaluate_real_datasets(
            &djia,
            &bank,
            &losses,
            &vic,
            (&storm_p, &storm_y),
        );
        assert!(emp_stoch.empirical_stochary_verified);

        let dpi_card04 = PublicInfrastructureCard04Audit::evaluate_from_disk(root);
        assert!(dpi_card04.verified);
        assert_eq!(ALL_DEFECTS.len(), 21);
    }
}
