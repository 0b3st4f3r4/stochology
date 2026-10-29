//! `stochology-siga` — The 4 Eliminatory Filters and the 8-Step SIGA Operational Roadmap (§3).

pub mod funnel;
pub mod roadmap;

pub use funnel::{
    evaluate_four_filter_funnel, fit_shelf_beta_prior, ln_gamma_lanczos,
    ppi_effective_sample_size_multiplier, prospective_clustered_mde,
    prospective_ppi_gold_sample_size, regularized_incomplete_beta, sbc_rank_chi2, FunnelCandidate,
    FunnelResult, LongContextDpiDilutionAudit, SimpsonCausalParadoxAudit,
};
pub use roadmap::{
    run_full_siga_roadmap, run_full_siga_roadmap_empirical, simulate_sbc_beta_binomial_ranks,
    AlertFatigueAudit, EvidenceProcess, MultimodalCorrelatedFusionAudit,
    NaturalFrequencyMemorandum, PreMortemCounterfactualRootCauseAudit, SaviConfidenceSequence,
    SigaExecutionReport, VarEsEBacktest,
};
