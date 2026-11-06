//! `stochology-cli` — Empirical dataset loaders (`data/*.csv`), SHA-256 manifest verifier,
//! and progressive Chapter 1–9 laboratories for Stochology.

pub mod chapters;
pub mod csv;
pub mod loaders;

pub use chapters::{
    audit_report, export_lab_trajectories_json, run_all_or_chapter, run_all_or_chapter_with_brake,
    run_chapter_1, run_chapter_2, run_chapter_3, run_chapter_4, run_chapter_5, run_chapter_6,
    run_chapter_7, run_chapter_8, run_chapter_8_with_brake, run_chapter_9,
};
pub use csv::{CsvRow, CsvTable};
pub use loaders::{
    UPSTREAM_DATASET_CONTRACTS, count_atlantic_storms_transitions,
    count_flight_csv_rows_and_missing, load_atlantic_storms_environments,
    load_atlantic_storms_forecasts, load_bank_call_clusters, load_cps_cohort_clusters,
    load_danish_fire, load_djia_returns, load_lalonde_nsw_cps, load_nyc_flight_delays,
    load_oot_ood_flight_delays, load_rtdsm_routput_vintages, load_toronto_arrests_full_summary,
    load_toronto_arrests_matched_and_classifier, load_toronto_arrests_prevalences,
    load_vic_elec_demand, resolve_workspace_root, verify_external_csv_bytes_and_schema,
    verify_external_upstream_schema_contract, verify_live_upstream_url_if_enabled,
    verify_sha256sums_manifest,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_nine_chapter_laboratories_and_sealed_datasets_verify() {
        let root = resolve_workspace_root();
        assert_eq!(verify_external_upstream_schema_contract(&root), Ok(14));
        let danish_bytes =
            std::fs::read(root.join("data/danish_fire.csv")).expect("read danish_fire.csv");
        assert_eq!(
            verify_external_csv_bytes_and_schema("data/danish_fire.csv", &danish_bytes),
            Ok(2_167)
        );
        let mut mutated = danish_bytes.clone();
        mutated[0] ^= 0x01;
        assert!(verify_external_csv_bytes_and_schema("data/danish_fire.csv", &mutated).is_err());
        assert!(verify_external_csv_bytes_and_schema("data/unknown.csv", &danish_bytes).is_err());
        assert_eq!(
            verify_live_upstream_url_if_enabled("data/danish_fire.csv"),
            Ok(false)
        );
        run_all_or_chapter(&root, None).expect("all 9 chapters and SHA256SUMS must verify");
        run_all_or_chapter(&root, Some(1)).expect("single chapter dispatch must succeed");
        assert!(run_all_or_chapter(&root, Some(99)).is_err());
        let json = export_lab_trajectories_json();
        assert!(json.contains("\"shockCovRate\": 0.7500"));
    }
}
