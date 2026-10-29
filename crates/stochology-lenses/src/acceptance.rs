//! Multi-seed acceptance and sealed confirmation for simulated syntheses (`stochology_lenses::acceptance`).
//!
//! A claim verified at a single seed can be seed luck, and a claim set tuned on a seed block can pass that block by selection.
//! Gate `G8` therefore separates two seed sets:
//! - **Development seeds** [`ACCEPTANCE_SEEDS`] (`2026..=2045`) were used to decide which claims to keep, discard, or re-specify.
//!   Each gated claim must still pass on at least [`MIN_CLAIM_PASS_RATE`] of them.
//! - **Confirmation seeds** [`confirmation_seeds`] (`9_000_000..=9_000_199`) were evaluated once, after the claim set was frozen, and
//!   never used for a decision. Each gated claim must pass on at least [`MIN_CLAIM_PASS_RATE`] of them; claims that failed that one
//!   evaluation are recorded as refuted in `discarded_claims` and in the sealed registry.
//!
//! Claims whose name contains `.det.` do not depend on the seed: they must pass on every seed, and the gate checks that their
//! outcome is indeed the same on every seed. The registry ([`ACCEPTANCE_REGISTRY_PATH`]) seals the seed sets, the pass rate, every
//! claim name, the SHA-256 of every source file that defines the claims ([`CLAIM_DEFINING_SOURCES`]), and the confirmation pass
//! rates; its own SHA-256 is listed in `data/SHA256SUMS`.

use std::path::Path;

/// A named verification check `(name, passed)`.
pub type NamedCheck = (&'static str, bool);

/// Development seeds: the canonical `seed = 2026` followed by the next 19 integers.
pub const ACCEPTANCE_SEEDS: [u64; 20] = [
    2026, 2027, 2028, 2029, 2030, 2031, 2032, 2033, 2034, 2035, 2036, 2037, 2038, 2039, 2040, 2041,
    2042, 2043, 2044, 2045,
];

/// First and last confirmation seed for K=3..5 (200 seeds, disjoint from every seed used for development or review).
pub const CONFIRMATION_SEED_RANGE: (u64, u64) = (9_000_000, 9_000_199);

/// Virgin confirmation seed block for K=6 (`society`, 200 seeds `9_000_200..=9_000_399`, never used prior to freezing `society`).
pub const SOCIETY_CONFIRMATION_SEED_RANGE: (u64, u64) = (9_000_200, 9_000_399);

/// Minimum fraction of seeds on which each stochastic gated claim must pass (19 of 20 development seeds, 190 of 200 confirmation seeds).
pub const MIN_CLAIM_PASS_RATE: f64 = 0.95;

/// Path of the sealed acceptance registry, relative to the workspace root; its SHA-256 is listed in `data/SHA256SUMS`.
pub const ACCEPTANCE_REGISTRY_PATH: &str = "data/preregistration/acceptance_registry.txt";

/// Source files whose bytes define the gated claims (thresholds, sample sizes, generating constants, and the methods they call).
/// This is the transitive closure of the modules the five syntheses use (checked by a unit test); only `relational/` and
/// `core/src/card/` are outside it. Any edit to these files changes the registry and requires re-sealing under human review.
pub const CLAIM_DEFINING_SOURCES: [&str; 60] = [
    "crates/stochology-lenses/src/acceptance.rs",
    "crates/stochology-lenses/src/analog/empirical.rs",
    "crates/stochology-lenses/src/analog/mod.rs",
    "crates/stochology-lenses/src/analog/model_collapse.rs",
    "crates/stochology-lenses/src/analog/ode_aimc.rs",
    "crates/stochology-lenses/src/analog/solvers.rs",
    "crates/stochology-lenses/src/analog/wave_organoid.rs",
    "crates/stochology-lenses/src/critical/adorno_marcuse.rs",
    "crates/stochology-lenses/src/critical/collapse_overcoming.rs",
    "crates/stochology-lenses/src/critical/empirical.rs",
    "crates/stochology-lenses/src/critical/mod.rs",
    "crates/stochology-lenses/src/decision/ahp_ppi.rs",
    "crates/stochology-lenses/src/decision/corp_murphy.rs",
    "crates/stochology-lenses/src/decision/cost_loss.rs",
    "crates/stochology-lenses/src/decision/mod.rs",
    "crates/stochology-lenses/src/information.rs",
    "crates/stochology-lenses/src/invariance/icp.rs",
    "crates/stochology-lenses/src/invariance/mod.rs",
    "crates/stochology-lenses/src/invariance/nile_irm.rs",
    "crates/stochology-lenses/src/invariance/shapley_causal.rs",
    "crates/stochology-lenses/src/lib.rs",
    "crates/stochology-lenses/src/machine.rs",
    "crates/stochology-lenses/src/society/empirical.rs",
    "crates/stochology-lenses/src/society/h1a_regulator.rs",
    "crates/stochology-lenses/src/society/h1b_bester.rs",
    "crates/stochology-lenses/src/society/h2_vcg_commons.rs",
    "crates/stochology-lenses/src/society/mod.rs",
    "crates/stochology-lenses/src/society/sfc_matrix.rs",
    "crates/stochology-lenses/src/stocks/cybernetic_vsm_pate.rs",
    "crates/stochology-lenses/src/stocks/empirical.rs",
    "crates/stochology-lenses/src/stocks/mod.rs",
    "crates/stochology-lenses/src/stocks/powell_friston_pearl.rs",
    "crates/stochology-lenses/src/stocks/supply_chain_plant.rs",
    "crates/stochology-lenses/src/stocks/topology_axioms.rs",
    "crates/stochology-lenses/src/strategic/contagion_ahp_kalman.rs",
    "crates/stochology-lenses/src/strategic/empirical.rs",
    "crates/stochology-lenses/src/strategic/minsky_shapley_rag.rs",
    "crates/stochology-lenses/src/strategic/mod.rs",
    "crates/stochology-lenses/src/tail.rs",
    "crates/stochology-lenses/src/time.rs",
    "crates/stochology-lenses/src/triad/ctrw_pdhte.rs",
    "crates/stochology-lenses/src/triad/empirical.rs",
    "crates/stochology-lenses/src/triad/helmholtz_hmatrix.rs",
    "crates/stochology-lenses/src/triad/mod.rs",
    "crates/stochology-lenses/src/triad/weyl_sna_rlhf.rs",
    "crates/stochology-core/src/invariants.rs",
    "crates/stochology-core/src/layers/jensen.rs",
    "crates/stochology-core/src/layers/missingness.rs",
    "crates/stochology-core/src/layers/mod.rs",
    "crates/stochology-core/src/layers/ontology.rs",
    "crates/stochology-core/src/layers/performativity.rs",
    "crates/stochology-core/src/layers/provenance.rs",
    "crates/stochology-core/src/lib.rs",
    "crates/stochology-core/src/math/complex.rs",
    "crates/stochology-core/src/math/linalg.rs",
    "crates/stochology-core/src/math/mod.rs",
    "crates/stochology-core/src/math/stats.rs",
    "crates/stochology-core/src/rng.rs",
    "crates/stochology-core/src/sha256.rs",
    "crates/stochology-core/src/uncertainty.rs",
];

/// Composite audit contract for an individual axis or hypothesis within a simulated synthesis.
pub trait SynthesisAxisAudit {
    /// Named claim checks gated by this axis.
    fn claims(&self) -> &[NamedCheck];
    /// Claims removed from the gate after failing multi-seed acceptance.
    fn discarded_claims(&self) -> &[NamedCheck] {
        &[]
    }
    /// True if all algebraic/numerical identities of this axis close.
    fn identities_closed(&self) -> bool;
    /// True if identities close and all gated claims pass.
    fn verified(&self) -> bool {
        self.identities_closed() && self.claims().iter().all(|c| c.1)
    }
}

/// Template-Method contract for a multi-axis simulated synthesis evaluated across development and confirmation seeds.
pub trait SimulatedSynthesis: Sized + Send + Sync {
    /// Short registry label (`critical`, `stocks`, `triad`, `strategic`, `society`).
    const LABEL: &'static str;
    /// Confirmation seed range `(first_seed, last_seed)` for this synthesis.
    const CONFIRMATION_RANGE: (u64, u64) = CONFIRMATION_SEED_RANGE;

    /// Evaluates the synthesis at `seed` in shielded (`true`) or naive/defect (`false`) mode.
    fn evaluate_with_mode(seed: u64, shielded: bool) -> Self;

    /// Evaluates the canonical shielded synthesis at `seed`.
    fn evaluate_canonical(seed: u64) -> Self {
        Self::evaluate_with_mode(seed, true)
    }

    /// All gated claims across all axes, in declaration order.
    fn claim_checks(&self) -> Vec<NamedCheck>;

    /// Claims removed from the gate after failing multi-seed acceptance.
    fn discarded_claim_checks(&self) -> Vec<NamedCheck>;

    /// True if every axis's identity checks close.
    fn all_identities_closed(&self) -> bool;

    /// True if identities close and all axes pass.
    fn all_axes_verified(&self) -> bool;

    /// Per-claim pass rates over `seeds` in the given mode.
    fn multi_seed_acceptance(seeds: &[u64], shielded: bool) -> MultiSeedAcceptance {
        MultiSeedAcceptance::evaluate(seeds, |seed| {
            let syn = Self::evaluate_with_mode(seed, shielded);
            (syn.claim_checks(), syn.all_identities_closed())
        })
    }

    /// Evaluates development and confirmation acceptance for this synthesis.
    fn evaluate_synthesis_acceptance() -> SynthesisAcceptance {
        let confirmation: Vec<u64> =
            (Self::CONFIRMATION_RANGE.0..=Self::CONFIRMATION_RANGE.1).collect();
        let names = |claims: Vec<NamedCheck>| claims.into_iter().map(|c| c.0).collect::<Vec<_>>();
        let syn0 = Self::evaluate_canonical(ACCEPTANCE_SEEDS[0]);
        SynthesisAcceptance {
            label: Self::LABEL,
            gated: names(syn0.claim_checks()),
            discarded: names(syn0.discarded_claim_checks()),
            development: Self::multi_seed_acceptance(&ACCEPTANCE_SEEDS, true),
            confirmation: MultiSeedAcceptance::evaluate(&confirmation, |seed| {
                let syn = Self::evaluate_canonical(seed);
                let mut all = syn.claim_checks();
                all.extend(syn.discarded_claim_checks());
                (all, syn.all_identities_closed())
            }),
        }
    }

    /// Verifies that in unshielded (`false`) mode, every development seed closes all identities,
    /// fails overall verification, and fails at least one seed-dependent claim.
    fn verify_negative_defect_all_dev_seeds() -> bool {
        ACCEPTANCE_SEEDS.iter().all(|&seed| {
            let naive = Self::evaluate_with_mode(seed, false);
            naive.all_identities_closed()
                && !naive.all_axes_verified()
                && naive
                    .claim_checks()
                    .iter()
                    .any(|&(name, ok)| !ok && !is_deterministic(name))
        })
    }
}

macro_rules! impl_axis_audit {
    ($ty:ty, $verified_field:ident) => {
        impl SynthesisAxisAudit for $ty {
            fn claims(&self) -> &[NamedCheck] {
                &self.claims
            }
            fn identities_closed(&self) -> bool {
                self.identities_closed
            }
            fn verified(&self) -> bool {
                self.$verified_field
            }
        }
    };
    ($ty:ty, $verified_field:ident, with_discarded) => {
        impl SynthesisAxisAudit for $ty {
            fn claims(&self) -> &[NamedCheck] {
                &self.claims
            }
            fn discarded_claims(&self) -> &[NamedCheck] {
                &self.discarded_claims
            }
            fn identities_closed(&self) -> bool {
                self.identities_closed
            }
            fn verified(&self) -> bool {
                self.$verified_field
            }
        }
    };
}

impl_axis_audit!(
    crate::critical::AdornoNonIdenticalReificationAudit,
    axis1_verified,
    with_discarded
);
impl_axis_audit!(
    crate::critical::MarcuseRouvroyStochasticParrotAudit,
    axis2_verified
);
impl SynthesisAxisAudit for crate::critical::MaterialCollapseTriadAudit {
    fn claims(&self) -> &[NamedCheck] {
        &self.claims
    }
    fn identities_closed(&self) -> bool {
        true
    }
    fn verified(&self) -> bool {
        self.axis3_verified
    }
}
impl_axis_audit!(
    crate::critical::DialecticalOvercomingSynthesisAudit,
    axis4_verified
);

impl_axis_audit!(
    crate::stocks::TopologyAndFourStockAxiomsAudit,
    axis1_verified
);
impl_axis_audit!(
    crate::stocks::PowellFristonCausalEngineAudit,
    axis2_verified
);
impl_axis_audit!(
    crate::stocks::StrategicStockPlantResilienceAudit,
    axis3_verified,
    with_discarded
);
impl_axis_audit!(
    crate::stocks::CyberneticVsmAndPateGovernanceAudit,
    axis4_verified,
    with_discarded
);

impl_axis_audit!(crate::triad::ComplexWaveGaborHelmholtzAudit, axis1_verified);
impl_axis_audit!(
    crate::triad::ComplexRealIsomorphismHMatrixAudit,
    axis2_verified
);
impl_axis_audit!(crate::triad::FractionalCtrwAnalogEccAudit, axis3_verified);
impl_axis_audit!(
    crate::triad::MultifractalPdhteRagCollapseAudit,
    axis4_verified
);
impl_axis_audit!(
    crate::triad::WeylTorusHermiteMehlerDfmAudit,
    axis5_verified,
    with_discarded
);
impl_axis_audit!(
    crate::triad::SnaRlhfFhrrCohomologyTechniumAudit,
    axis6_verified
);

impl_axis_audit!(
    crate::strategic::MinskyPapertGeometrySocietyOfMindAudit,
    axis1_verified
);
impl_axis_audit!(
    crate::strategic::ShapleyCausalRubinMiDoeAudit,
    axis2_verified,
    with_discarded
);
impl_axis_audit!(
    crate::strategic::GenerativeRagLlmJudgeCompressionAudit,
    axis3_verified
);
impl_axis_audit!(
    crate::strategic::ComplexNetworkHubContagionAudit,
    axis4_verified
);
impl_axis_audit!(
    crate::strategic::MultiCriteriaAhpRegulatoryPkmAudit,
    axis5_verified
);
impl_axis_audit!(
    crate::strategic::RobustKalmanClosedLoopControlAudit,
    axis6_verified
);

impl_axis_audit!(crate::society::H1aRegulatorCouplingAblation, h1a_verified);
impl_axis_audit!(crate::society::H1bCreditScreeningAblation, h1b_verified);
impl_axis_audit!(
    crate::society::H2VcgGreenLaffontCommonsAblation,
    h2_verified
);

impl SimulatedSynthesis for crate::critical::CriticalStochasticRationalitySynthesis {
    const LABEL: &'static str = "critical";
    fn evaluate_with_mode(seed: u64, shielded: bool) -> Self {
        Self::evaluate_with_mode(seed, shielded)
    }
    fn claim_checks(&self) -> Vec<NamedCheck> {
        Self::claim_checks(self)
    }
    fn discarded_claim_checks(&self) -> Vec<NamedCheck> {
        Self::discarded_claim_checks(self)
    }
    fn all_identities_closed(&self) -> bool {
        self.all_identities_closed
    }
    fn all_axes_verified(&self) -> bool {
        self.all_four_axes_verified
    }
}

impl SimulatedSynthesis for crate::stocks::StocharyConjecturalCyberneticSynthesis {
    const LABEL: &'static str = "stocks";
    fn evaluate_with_mode(seed: u64, shielded: bool) -> Self {
        Self::evaluate_with_mode(seed, shielded)
    }
    fn claim_checks(&self) -> Vec<NamedCheck> {
        Self::claim_checks(self)
    }
    fn discarded_claim_checks(&self) -> Vec<NamedCheck> {
        Self::discarded_claim_checks(self)
    }
    fn all_identities_closed(&self) -> bool {
        self.all_identities_closed
    }
    fn all_axes_verified(&self) -> bool {
        self.all_four_axes_verified
    }
}

impl SimulatedSynthesis for crate::triad::ComplexFractalToroidalTechniumSynthesis {
    const LABEL: &'static str = "triad";
    fn evaluate_with_mode(seed: u64, shielded: bool) -> Self {
        Self::evaluate_with_mode(seed, shielded)
    }
    fn claim_checks(&self) -> Vec<NamedCheck> {
        Self::claim_checks(self)
    }
    fn discarded_claim_checks(&self) -> Vec<NamedCheck> {
        Self::discarded_claim_checks(self)
    }
    fn all_identities_closed(&self) -> bool {
        self.all_identities_closed
    }
    fn all_axes_verified(&self) -> bool {
        self.all_six_bridges_verified
    }
}

impl SimulatedSynthesis for crate::strategic::StrategicAiAccommodationSynthesis {
    const LABEL: &'static str = "strategic";
    fn evaluate_with_mode(seed: u64, shielded: bool) -> Self {
        Self::evaluate_with_mode(seed, shielded)
    }
    fn claim_checks(&self) -> Vec<NamedCheck> {
        Self::claim_checks(self)
    }
    fn discarded_claim_checks(&self) -> Vec<NamedCheck> {
        Self::discarded_claim_checks(self)
    }
    fn all_identities_closed(&self) -> bool {
        self.all_identities_closed
    }
    fn all_axes_verified(&self) -> bool {
        self.all_six_axes_verified
    }
}

impl SimulatedSynthesis for crate::society::CoupledPublicPrivateSocietySynthesis {
    const LABEL: &'static str = "society";
    const CONFIRMATION_RANGE: (u64, u64) = SOCIETY_CONFIRMATION_SEED_RANGE;
    fn evaluate_with_mode(seed: u64, shielded: bool) -> Self {
        Self::evaluate_with_mode(seed, shielded)
    }
    fn claim_checks(&self) -> Vec<NamedCheck> {
        Self::claim_checks(self)
    }
    fn discarded_claim_checks(&self) -> Vec<NamedCheck> {
        Self::discarded_claim_checks(self)
    }
    fn all_identities_closed(&self) -> bool {
        self.all_identities_closed
    }
    fn all_axes_verified(&self) -> bool {
        self.all_axes_verified
    }
}

/// The 200 confirmation seeds for K=3..5 (`9_000_000..=9_000_199`).
pub fn confirmation_seeds() -> Vec<u64> {
    (CONFIRMATION_SEED_RANGE.0..=CONFIRMATION_SEED_RANGE.1).collect()
}

/// The 200 virgin confirmation seeds for K=6 (`society`, `9_000_200..=9_000_399`).
pub fn society_confirmation_seeds() -> Vec<u64> {
    (SOCIETY_CONFIRMATION_SEED_RANGE.0..=SOCIETY_CONFIRMATION_SEED_RANGE.1).collect()
}

/// True if the claim is declared seed-independent (its name contains `.det.`).
pub fn is_deterministic(name: &str) -> bool {
    name.contains(".det.")
}

/// Per-claim pass rates of a synthesis across a seed set.
#[derive(Debug, Clone, PartialEq)]
pub struct MultiSeedAcceptance {
    /// Seeds evaluated.
    pub seeds: Vec<u64>,
    /// Pass rate of every claim, in declaration order.
    pub claim_pass_rates: Vec<(&'static str, f64)>,
    /// True if the identity checks closed on every seed.
    pub identities_closed_on_all_seeds: bool,
    /// True if every `.det.` claim had the same outcome on every seed.
    pub deterministic_claims_constant: bool,
    /// True if identities closed on every seed, `.det.` claims are constant and pass everywhere, and every other claim reaches
    /// [`MIN_CLAIM_PASS_RATE`].
    pub accepted: bool,
}

impl MultiSeedAcceptance {
    /// Evaluates `run(seed) -> (claims, identities_closed)` on every seed (in parallel) and aggregates per-claim pass rates.
    pub fn evaluate<F>(seeds: &[u64], run: F) -> Self
    where
        F: Fn(u64) -> (Vec<NamedCheck>, bool) + Sync,
    {
        let workers = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1)
            .clamp(1, 16);
        let chunk = seeds.len().div_ceil(workers).max(1);
        let run_ref = &run;
        let results: Vec<(Vec<NamedCheck>, bool)> = std::thread::scope(|scope| {
            let handles: Vec<_> = seeds
                .chunks(chunk)
                .map(|block| {
                    scope.spawn(move || block.iter().map(|&s| run_ref(s)).collect::<Vec<_>>())
                })
                .collect();
            handles
                .into_iter()
                .flat_map(|h| h.join().expect("acceptance worker panicked"))
                .collect()
        });

        let mut names: Vec<&'static str> = Vec::new();
        let mut passes: Vec<usize> = Vec::new();
        let mut identities_closed_on_all_seeds = true;
        for (claims, identities) in &results {
            identities_closed_on_all_seeds &= *identities;
            for &(name, ok) in claims {
                let idx = match names.iter().position(|&n| n == name) {
                    Some(i) => i,
                    None => {
                        names.push(name);
                        passes.push(0);
                        names.len() - 1
                    }
                };
                if ok {
                    passes[idx] += 1;
                }
            }
        }
        let total = seeds.len().max(1);
        let claim_pass_rates: Vec<(&'static str, f64)> = names
            .iter()
            .zip(&passes)
            .map(|(&n, &p)| (n, p as f64 / total as f64))
            .collect();
        let deterministic_claims_constant = names
            .iter()
            .zip(&passes)
            .filter(|(n, _)| is_deterministic(n))
            .all(|(_, &p)| p == 0 || p == total);
        let accepted = identities_closed_on_all_seeds
            && deterministic_claims_constant
            && claim_pass_rates.iter().all(|&(name, rate)| {
                if is_deterministic(name) {
                    rate == 1.0
                } else {
                    rate >= MIN_CLAIM_PASS_RATE
                }
            });
        Self {
            seeds: seeds.to_vec(),
            claim_pass_rates,
            identities_closed_on_all_seeds,
            deterministic_claims_constant,
            accepted,
        }
    }

    /// Claims that miss their requirement (`.det.` claims below 1.0, other claims below [`MIN_CLAIM_PASS_RATE`]).
    pub fn failing_claims(&self) -> Vec<(&'static str, f64)> {
        self.claim_pass_rates
            .iter()
            .copied()
            .filter(|&(name, rate)| {
                if is_deterministic(name) {
                    rate < 1.0
                } else {
                    rate < MIN_CLAIM_PASS_RATE
                }
            })
            .collect()
    }

    /// Number of claims that depend on the seed.
    pub fn stochastic_claim_count(&self) -> usize {
        self.claim_pass_rates
            .iter()
            .filter(|(n, _)| !is_deterministic(n))
            .count()
    }
}

/// Exact probability $P(X \ge k_{\min})$ for $X \sim \mathrm{Binomial}(n, p)$: the chance that a claim whose true pass rate is $p$
/// clears a `k_min`-of-`n` acceptance rule (the rule's operating characteristic).
pub fn binomial_accept_probability(p: f64, n: u64, k_min: u64) -> f64 {
    if k_min == 0 {
        return 1.0;
    }
    if p <= 0.0 {
        return 0.0;
    }
    if p >= 1.0 {
        return if k_min <= n { 1.0 } else { 0.0 };
    }
    let (ln_p, ln_q) = (p.ln(), (1.0 - p).ln());
    let mut ln_choose = 0.0_f64; // ln C(n, 0)
    let mut total = 0.0_f64;
    for k in 0..=n {
        if k > 0 {
            ln_choose += ((n - k + 1) as f64).ln() - (k as f64).ln();
        }
        if k >= k_min {
            total += (ln_choose + k as f64 * ln_p + (n - k) as f64 * ln_q).exp();
        }
    }
    total.min(1.0)
}

/// One-sided Clopper-Pearson lower confidence bound at level $1 - \alpha$ for a pass rate observed as `k` of `n`:
/// the smallest $p$ with $P(X \ge k \mid p) \ge \alpha$, found by bisection on the exact binomial tail.
pub fn clopper_pearson_lower(k: u64, n: u64, alpha: f64) -> f64 {
    if k == 0 {
        return 0.0;
    }
    let (mut lo, mut hi) = (0.0_f64, 1.0_f64);
    for _ in 0..200 {
        let mid = 0.5 * (lo + hi);
        if binomial_accept_probability(mid, n, k) < alpha {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    hi
}

/// Development and confirmation acceptance of one simulated synthesis.
#[derive(Debug, Clone, PartialEq)]
pub struct SynthesisAcceptance {
    /// Synthesis label used in the registry (`critical`, `stocks`, `triad`, `strategic`, `society`).
    pub label: &'static str,
    /// Names of the gated claims.
    pub gated: Vec<&'static str>,
    /// Names of the discarded claims.
    pub discarded: Vec<&'static str>,
    /// Gated claims on the development seeds.
    pub development: MultiSeedAcceptance,
    /// Gated and discarded claims on the confirmation seeds.
    pub confirmation: MultiSeedAcceptance,
}

impl SynthesisAcceptance {
    /// True if the gated claims pass on both seed sets.
    pub fn accepted(&self) -> bool {
        let confirmation_ok = self.confirmation.identities_closed_on_all_seeds
            && self.confirmation.deterministic_claims_constant
            && self
                .confirmation
                .failing_claims()
                .iter()
                .all(|(name, _)| !self.gated.contains(name));
        self.development.accepted && confirmation_ok
    }

    /// Gated claims that miss their requirement on the confirmation seeds.
    pub fn refuted_on_confirmation(&self) -> Vec<(&'static str, f64)> {
        self.confirmation
            .failing_claims()
            .into_iter()
            .filter(|(name, _)| self.gated.contains(name))
            .collect()
    }
}

/// Evaluates development and confirmation acceptance of the five simulated syntheses (grounded/shielded mode).
pub fn evaluate_all_syntheses() -> Vec<SynthesisAcceptance> {
    use crate::critical::CriticalStochasticRationalitySynthesis as Critical;
    use crate::society::CoupledPublicPrivateSocietySynthesis as Society;
    use crate::stocks::StocharyConjecturalCyberneticSynthesis as Stocks;
    use crate::strategic::StrategicAiAccommodationSynthesis as Strategic;
    use crate::triad::ComplexFractalToroidalTechniumSynthesis as Triad;
    vec![
        Critical::evaluate_synthesis_acceptance(),
        Stocks::evaluate_synthesis_acceptance(),
        Triad::evaluate_synthesis_acceptance(),
        Strategic::evaluate_synthesis_acceptance(),
        Society::evaluate_synthesis_acceptance(),
    ]
}

/// Canonical (non-comment) lines of the acceptance registry, regenerated from the code and the claim-defining sources under `root`.
pub fn canonical_registry_lines(
    root: &Path,
    syntheses: &[SynthesisAcceptance],
) -> Result<Vec<String>, String> {
    let seeds: Vec<String> = ACCEPTANCE_SEEDS.iter().map(|s| s.to_string()).collect();
    let mut lines = vec![
        "acceptance_registry v1".to_string(),
        format!("development_seeds {}", seeds.join(" ")),
        format!(
            "confirmation_seeds {}..={}",
            CONFIRMATION_SEED_RANGE.0, CONFIRMATION_SEED_RANGE.1
        ),
        format!(
            "confirmation_seeds_society {}..={}",
            SOCIETY_CONFIRMATION_SEED_RANGE.0, SOCIETY_CONFIRMATION_SEED_RANGE.1
        ),
        format!("min_claim_pass_rate {MIN_CLAIM_PASS_RATE:.2}"),
    ];
    for rel in CLAIM_DEFINING_SOURCES {
        let bytes = std::fs::read(root.join(rel))
            .map_err(|e| format!("cannot read source '{rel}': {e}"))?;
        lines.push(format!(
            "source {} {rel}",
            stochology_core::sha256_hex(&bytes)
        ));
    }
    for syn in syntheses {
        let rate = |name: &str| {
            syn.confirmation
                .claim_pass_rates
                .iter()
                .find(|(n, _)| *n == name)
                .map_or(f64::NAN, |&(_, r)| r)
        };
        for name in &syn.gated {
            lines.push(format!(
                "gated {} {name} confirmation {:.3}",
                syn.label,
                rate(name)
            ));
        }
        for name in &syn.discarded {
            lines.push(format!(
                "discarded {} {name} confirmation {:.3}",
                syn.label,
                rate(name)
            ));
        }
    }
    Ok(lines)
}

/// Checks a registry file's content against the expected canonical lines (comment lines starting with `#` and blank lines are
/// ignored for the comparison but still covered by the file's SHA-256). Returns the first mismatch, if any.
pub fn verify_registry_content(content: &str, expected: &[String]) -> Result<(), String> {
    let found: Vec<&str> = content
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .collect();
    for (i, exp) in expected.iter().enumerate() {
        match found.get(i) {
            Some(got) if got == exp => {}
            Some(got) => {
                return Err(format!(
                    "registry line {} is '{got}', code expects '{exp}'",
                    i + 1
                ));
            }
            None => return Err(format!("registry is missing '{exp}'")),
        }
    }
    if found.len() > expected.len() {
        return Err(format!(
            "registry has unexpected line '{}'",
            found[expected.len()]
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pass_rates_and_acceptance() {
        let acc = MultiSeedAcceptance::evaluate(&[1, 2, 3, 4], |seed| {
            (vec![("always", true), ("even_only", seed % 2 == 0)], true)
        });
        assert_eq!(
            acc.claim_pass_rates,
            vec![("always", 1.0), ("even_only", 0.5)]
        );
        assert!(!acc.accepted);
        assert_eq!(acc.failing_claims(), vec![("even_only", 0.5)]);
        let ok = MultiSeedAcceptance::evaluate(&[1, 2], |_| (vec![("always", true)], true));
        assert!(ok.accepted);
    }

    #[test]
    fn test_operating_characteristic_and_clopper_pearson() {
        let accept = |p: f64| binomial_accept_probability(p, 200, 190);
        // Reference values from the exact binomial sum (independently computed): 0.0081, 0.1661, 0.5831, 0.9599.
        assert!((accept(0.95) - 0.5831).abs() < 1e-3, "{}", accept(0.95));
        assert!((accept(0.97) - 0.9599).abs() < 1e-3, "{}", accept(0.97));
        assert!(accept(0.90) < 0.01 && accept(0.99) > 0.999);
        assert!(accept(0.93) < accept(0.95) && accept(0.95) < accept(0.97));
        // 200 of 200 passes: the 95% one-sided lower bound is 0.05^(1/200).
        let lower = clopper_pearson_lower(200, 200, 0.05);
        assert!((lower - 0.05_f64.powf(1.0 / 200.0)).abs() < 1e-9, "{lower}");
        assert!(clopper_pearson_lower(190, 200, 0.05) < 0.95);
    }

    #[test]
    fn test_registry_content_detects_tampering() {
        let expected = vec!["a".to_string(), "b".to_string()];
        assert!(verify_registry_content("# comment\na\n\nb\n", &expected).is_ok());
        assert!(verify_registry_content("a\nc\n", &expected).is_err());
        assert!(verify_registry_content("a\n", &expected).is_err());
        assert!(verify_registry_content("a\nb\nextra\n", &expected).is_err());
    }

    fn collect_rs_files(dir: &Path, workspace: &Path) -> Vec<String> {
        let mut out = Vec::new();
        for entry in std::fs::read_dir(dir).expect("read dir") {
            let entry = entry.expect("dir entry");
            let path = entry.path();
            if path.is_dir() {
                out.extend(collect_rs_files(&path, workspace));
            } else if path.extension().is_some_and(|e| e == "rs") {
                let rel = path
                    .strip_prefix(workspace)
                    .expect("workspace prefix")
                    .to_string_lossy()
                    .replace('\\', "/");
                out.push(rel);
            }
        }
        out.sort();
        out
    }

    #[test]
    fn test_claim_defining_sources_cover_the_module_closure() {
        let lenses_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let workspace = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("crates dir")
            .parent()
            .expect("workspace root");
        let modules = |text: &str| -> Vec<String> {
            let mut found = Vec::new();
            let mut rest = text;
            while let Some(i) = rest.find("crate::") {
                rest = &rest[i + "crate::".len()..];
                if let Some(inner) = rest.strip_prefix('{') {
                    let end = inner.find('}').unwrap_or(inner.len());
                    for item in inner[..end].split(',') {
                        let name: String = item
                            .trim()
                            .chars()
                            .take_while(|c| c.is_alphanumeric() || *c == '_')
                            .collect();
                        if !name.is_empty() {
                            found.push(name);
                        }
                    }
                } else {
                    let name: String = rest
                        .chars()
                        .take_while(|c| c.is_alphanumeric() || *c == '_')
                        .collect();
                    found.push(name);
                }
            }
            found
        };
        let mut queue = vec![
            "critical".to_string(),
            "stocks".to_string(),
            "triad".to_string(),
            "strategic".to_string(),
            "society".to_string(),
            "acceptance".to_string(),
        ];
        let mut closure: Vec<String> = Vec::new();
        let mut closure_files: Vec<String> = Vec::new();
        while let Some(module) = queue.pop() {
            if module.is_empty() || closure.contains(&module) {
                continue;
            }
            let file_path = lenses_dir.join(format!("{module}.rs"));
            let dir_path = lenses_dir.join(&module);
            let module_files = if file_path.exists() {
                vec![format!("crates/stochology-lenses/src/{module}.rs")]
            } else if dir_path.is_dir() {
                collect_rs_files(&dir_path, workspace)
            } else {
                continue;
            };
            let card_path = ["card", "::"].concat();
            for rel in &module_files {
                let text =
                    std::fs::read_to_string(workspace.join(rel)).expect("read lenses module");
                assert!(
                    !text.contains(&card_path),
                    "{rel} references core card module"
                );
                queue.extend(modules(&text));
            }
            closure.push(module);
            closure_files.extend(module_files);
        }
        for rel in &closure_files {
            assert!(
                CLAIM_DEFINING_SOURCES.contains(&rel.as_str()),
                "{rel} is used by a synthesis but not sealed"
            );
        }
        for rel in collect_rs_files(&workspace.join("crates/stochology-core/src"), workspace) {
            if !rel.starts_with("crates/stochology-core/src/card/") {
                assert!(
                    CLAIM_DEFINING_SOURCES.contains(&rel.as_str()),
                    "{rel} is not sealed"
                );
            }
        }
    }

    #[test]
    fn test_deterministic_claims_must_be_constant() {
        let acc = MultiSeedAcceptance::evaluate(&[1, 2, 3, 4], |seed| {
            (vec![("x.a1.det.flag", seed != 3)], true)
        });
        assert!(!acc.deterministic_claims_constant);
        assert!(!acc.accepted);
        assert_eq!(acc.stochastic_claim_count(), 0);
    }
}
