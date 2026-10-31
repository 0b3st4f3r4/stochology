//! `stochology-cli` — Empirical dataset loaders (`data/*.csv`), SHA-256 manifest verifier,
//! and progressive Chapter 1–9 laboratories for Stochology.

pub mod chapters;
pub mod csv;
pub mod loaders;

pub use chapters::{
    run_all_or_chapter, run_chapter_1, run_chapter_2, run_chapter_3, run_chapter_4, run_chapter_5,
    run_chapter_6, run_chapter_7, run_chapter_8, run_chapter_9,
};
pub use csv::{CsvRow, CsvTable};
pub use loaders::{
    load_atlantic_storms_environments, load_atlantic_storms_forecasts, load_bank_call_clusters,
    load_cps_cohort_clusters, load_danish_fire, load_djia_returns, load_lalonde_nsw_cps,
    load_nyc_flight_delays, load_rtdsm_routput_vintages, load_toronto_arrests_prevalences,
    load_vic_elec_demand, resolve_workspace_root, verify_sha256sums_manifest,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_nine_chapter_laboratories_and_sealed_datasets_verify() {
        let root = resolve_workspace_root();
        run_all_or_chapter(&root, None).expect("all 9 chapters and SHA256SUMS must verify");
    }
}
