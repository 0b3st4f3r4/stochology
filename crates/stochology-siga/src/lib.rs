//! `stochology-siga` — The 4 Eliminatory Filters and the 8-Step SIGA Operational Roadmap (§3).

pub mod funnel;
pub mod roadmap;

pub use funnel::{
    FunnelCandidate, FunnelResult, LongContextDpiDilutionAudit, SimpsonCausalParadoxAudit,
    evaluate_four_filter_funnel, fit_shelf_beta_prior, ln_gamma_lanczos,
    ppi_effective_sample_size_multiplier, prospective_clustered_mde,
    prospective_ppi_gold_sample_size, regularized_incomplete_beta, sbc_rank_chi2,
};
pub use roadmap::{
    AlertFatigueAudit, EBhMultiplicityAudit, EvidenceProcess, FilterOneAndTwoTensionsAudit,
    ImbensManskiConfidenceInterval, KeynesEvidentialWeightAudit, MayoManskiSevereBoundaryAudit,
    MultimodalCorrelatedFusionAudit, NaturalFrequencyMemorandum, PeekingMultiplicitySimulation,
    PreMortemCounterfactualRootCauseAudit, ProcessTracingTestKind, SaviConfidenceSequence,
    SigaExecutionReport, VarEsEBacktest, run_full_siga_roadmap, run_full_siga_roadmap_empirical,
    run_full_siga_roadmap_empirical_with_profile, simulate_sbc_beta_binomial_ranks,
};
