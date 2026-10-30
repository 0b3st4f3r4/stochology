//! `stochology-core` — Core mathematical and epistemological primitives of the Stochology Protocol.

pub mod card;
pub mod invariants;
pub mod layers;
pub mod math;
pub mod rng;
pub mod sha256;
pub mod uncertainty;

pub use card::{
    BridgePerformativity, CANONICAL_CARD_SPECIFICATIONS, CardParseError, CardSpecification,
    CausalLadder, ChouldechovaRatioMetrics, ContractViolation, DataAndAuditContract,
    DecisionMakerQuestion, DecisionRuleAndScope, DominantEvent, EProcessMetrics,
    EpistemologicalCard, EpistemologicalHeader, FieldAuthorship, FindingsRegistry,
    InfoGapRobustness, MissingnessMechanism, MoultonClusterMetrics, MurphyCorpDecomposition,
    NoisePhysics, NonlinearSummaryMetrics, PerformativeDivergenceMetrics, RrmContractionMetrics,
    UncertaintyBudgetAndAuthorship, UnitAndMargin, VarianceBudget,
};
pub use invariants::{
    ChouldechovaAudit, ExceedanceDesignEffect, OperationalWeightingGap, prospective_clustered_mde,
    verify_resolution_phi5,
};
pub use layers::{
    ClaimProvenanceClass, DpiOperationalStage, EpistemicPkmProvenanceAudit,
    EpsilonDeclarationAudit, GoodhartTaxonomyAudit, GoodhartVariant, Layer, ManskiBounds,
    MengAudit, NonlinearSummary, PerformativeDiagnostic, PerformativePowerAudit,
    PerformativeSystem, RubinMultipleImputationAudit, UncertaintyClass, apply_cromwell_floor,
    empirical_crps, empirical_wasserstein_1, exact_wasserstein_1, laplace_succession,
    respects_cromwell,
};
pub use math::{
    Complex64, average_ranks, dot_product, empirical_wasserstein_1_grid, invert_matrix, l2_norm,
    mat_mul, mat_vec_mul, ols_slope, pearson_correlation, solve_linear_system,
    spearman_rank_correlation, two_sample_ks_distance, univariate_ols_fit,
};
pub use rng::{
    DEFAULT_SEED, MonteCarloSplitTreeAudit, OnlineMoments, Rng, SplitMix64, block_bootstrap_ci,
    mean, median, median_absolute_deviation, quantile, quantile_sorted, sample_std,
    sample_variance,
};
pub use sha256::{
    hmac_sha256_bytes, hmac_sha256_hex, is_valid_sha256_hex, sha256_bytes, sha256_hex,
};
pub use uncertainty::{
    InfoGapEvaluation, InfoGapOption, KennedyOHaganDiscrepancy, cholesky_decompose,
    cholesky_log_det, cholesky_solve, solve_spd_system,
};
