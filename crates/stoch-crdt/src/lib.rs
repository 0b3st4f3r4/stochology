//! Causal-conservative join-semilattices, revocable rehabilitation,
//! coordinator-free 1-hop Byzantine proofs, and delay-tolerant bundle queues
//! (`#![no_std] + alloc`, zero external dependencies, zero floats).
//!
//! ```
//! use stoch_crdt::{ExilePhase, JoinSemilattice, LeontiefEpochState, SpentCapabilities};
//!
//! let mut a = LeontiefEpochState::new(1, 700_000, 600_000, 900_000);
//! let mut b = LeontiefEpochState::new(1, 500_000, 800_000, 750_000);
//! a.join(&b);
//! b.join(&LeontiefEpochState::new(1, 700_000, 600_000, 900_000));
//! assert_eq!(a, b);
//! assert_eq!(a.min_s_epi, 500_000);
//! assert_eq!(a.min_s_vital, 600_000);
//! assert!(!ExilePhase::Pharmacovigilance.has_witness_power());
//!
//! let mut caps = SpentCapabilities::new();
//! assert!(caps.consume([1; 32]));
//! assert!(!caps.consume([1; 32]));
//! ```

#![no_std]
#![deny(unsafe_code)]
#![deny(missing_docs)]
#![deny(clippy::float_arithmetic)]
#![forbid(clippy::cast_possible_truncation)]

extern crate alloc;

use alloc::collections::{BTreeMap, BTreeSet, VecDeque};
use alloc::vec::Vec;

/// Fixed-point unit corresponding to `1.0` in millionths.
pub const UNIT: i64 = 1_000_000;

/// Domain, capacity, or quorum validation failure in `stoch-crdt`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CrdtError {
    /// Parameter violates declared mathematical or algebraic bounds.
    InvalidDomain(&'static str),
    /// Delay-tolerant custody buffer reached capacity or shed non-urgent load.
    CapacityFull,
    /// Byzantine quorum, self-report exclusion, or equivocation check failed.
    QuorumViolation(&'static str),
}

/// Associative, commutative, and idempotent join-semilattice operation.
pub trait JoinSemilattice {
    /// Merges `other` into `self` preserving associativity, commutativity, and idempotence.
    fn join(&mut self, other: &Self);
}

/// Grow-only set of consumed linear authorization identifiers (`G-Set`).
///
/// Once an identifier is inserted via [`SpentCapabilities::consume`], merging replicas
/// across partitioned logs never resurrects spent capability authority.
#[derive(Clone, PartialEq, Eq, Default, Debug)]
pub struct SpentCapabilities {
    spent: BTreeSet<[u8; 32]>,
}

impl SpentCapabilities {
    /// Constructs an empty capability-consumption set.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            spent: BTreeSet::new(),
        }
    }

    /// Consumes a capability identifier strictly at most once.
    ///
    /// Returns `true` when freshly recorded and `false` on replay.
    pub fn consume(&mut self, capability_id: [u8; 32]) -> bool {
        self.spent.insert(capability_id)
    }

    /// Returns `true` if `capability_id` has already been consumed.
    #[must_use]
    pub fn is_spent(&self, capability_id: &[u8; 32]) -> bool {
        self.spent.contains(capability_id)
    }

    /// Number of consumed capability identifiers in the set.
    #[must_use]
    pub fn len(&self) -> usize {
        self.spent.len()
    }

    /// Returns `true` when no capabilities have been consumed.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.spent.is_empty()
    }
}

impl JoinSemilattice for SpentCapabilities {
    fn join(&mut self, other: &Self) {
        self.spent.extend(other.spent.iter().copied());
    }
}

/// Causal-conservative replicated epoch summary state (`RFDE-CRDT`).
///
/// Merging selects the higher causal epoch, resolves same-epoch ties by component-wise
/// conservative minimum across `(S_epi, S_vital, S_gov)`, and unions spent capabilities.
/// This structure summarizes replicated state diagnostics without issuing live step commands.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LeontiefEpochState {
    /// Monotonic causal epoch index.
    pub epoch: u64,
    /// Conservative floor observed along the epistemic axis.
    pub min_s_epi: i64,
    /// Conservative floor observed along the vital axis.
    pub min_s_vital: i64,
    /// Conservative floor observed along the governance axis.
    pub min_s_gov: i64,
    /// Monotonically growing set of spent linear capability identifiers.
    pub spent_capabilities: SpentCapabilities,
}

impl LeontiefEpochState {
    /// Creates an epoch state summary with an empty capability set.
    #[must_use]
    pub const fn new(epoch: u64, min_s_epi: i64, min_s_vital: i64, min_s_gov: i64) -> Self {
        Self {
            epoch,
            min_s_epi,
            min_s_vital,
            min_s_gov,
            spent_capabilities: SpentCapabilities::new(),
        }
    }

    /// Returns the non-compensatory three-axis bottle-neck floor (`min(epi, vital, gov)`).
    #[must_use]
    pub fn leontief_floor(&self) -> i64 {
        self.min_s_epi.min(self.min_s_vital).min(self.min_s_gov)
    }
}

impl JoinSemilattice for LeontiefEpochState {
    fn join(&mut self, other: &Self) {
        if other.epoch > self.epoch {
            self.epoch = other.epoch;
            self.min_s_epi = other.min_s_epi;
            self.min_s_vital = other.min_s_vital;
            self.min_s_gov = other.min_s_gov;
        } else if other.epoch == self.epoch {
            self.min_s_epi = self.min_s_epi.min(other.min_s_epi);
            self.min_s_vital = self.min_s_vital.min(other.min_s_vital);
            self.min_s_gov = self.min_s_gov.min(other.min_s_gov);
        }
        self.spent_capabilities.join(&other.spent_capabilities);
    }
}

/// Clock-free monotonic rehabilitation phase within an exile generation.
///
/// Ordering obeys `Quarantined < Canary < Pharmacovigilance < Active`.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ExilePhase {
    /// Isolated in quarantine with zero flow conductance and zero witness weight.
    Quarantined,
    /// Restricted probe re-entry with tight conductance cap and zero witness weight.
    Canary,
    /// Extended probationary surveillance with elevated cap and zero witness weight.
    Pharmacovigilance,
    /// Fully reintegrated with nominal conductance and normal witness participation.
    Active,
}

impl ExilePhase {
    /// Returns `true` only when fully [`ExilePhase::Active`].
    ///
    /// Both `Canary` and `Pharmacovigilance` carry zero probationary witness weight.
    #[must_use]
    pub const fn has_witness_power(self) -> bool {
        matches!(self, Self::Active)
    }

    /// Maximum permitted conductance relative to nominal flow, scaled by [`UNIT`].
    #[must_use]
    pub const fn conductance_cap_millionths(self) -> i64 {
        match self {
            Self::Active => UNIT,
            Self::Pharmacovigilance => 350_000,
            Self::Canary => 50_000,
            Self::Quarantined => 0,
        }
    }
}

/// Asymmetric quorum and exponential-backoff policy governing [`RehabLattice`].
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RehabPolicy {
    /// Minimum distinct witnesses required to trigger quarantine (`q_fraud < q_rehab`).
    pub q_fraud: usize,
    /// Minimum distinct witnesses required to advance a rehabilitation phase.
    pub q_rehab: usize,
    /// Baseline probation duration `T_base > 0` for generation-1 pharmacovigilance.
    pub t_base: u64,
}

impl RehabPolicy {
    /// Validates `0 < q_fraud < q_rehab` and `t_base > 0`.
    pub const fn new(q_fraud: usize, q_rehab: usize, t_base: u64) -> Result<Self, CrdtError> {
        if q_fraud == 0 || q_fraud >= q_rehab || t_base == 0 {
            return Err(CrdtError::InvalidDomain(
                "rehabilitation policy requires 0 < q_fraud < q_rehab and t_base > 0",
            ));
        }
        Ok(Self {
            q_fraud,
            q_rehab,
            t_base,
        })
    }

    /// Computes exponential probation horizon `T_vigil(g) = t_base * 2^(g - 1)` for `g >= 1`,
    /// or `0` when `generation == 0` (never exiled).
    pub fn vigil_duration(self, generation: u64) -> Result<u64, CrdtError> {
        if generation == 0 {
            return Ok(0);
        }
        let exp = u32::try_from(generation - 1)
            .map_err(|_| CrdtError::InvalidDomain("exile generation shift overflow"))?;
        let mult = 1_u64
            .checked_shl(exp)
            .ok_or(CrdtError::InvalidDomain("exponential backoff shift overflow"))?;
        self.t_base
            .checked_mul(mult)
            .ok_or(CrdtError::InvalidDomain("exponential backoff multiplication overflow"))
    }
}

/// Replicated, clock-free revocable rehabilitation lattice for a node.
///
/// Higher `(generation, phase)` dominates lexicographically, so a new quarantine at `g + 1`
/// dominates any concurrent promotion inside `g`, while attested fraud digests accumulate
/// permanently in `attested_fraud_logs`.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RehabLattice {
    generation: u64,
    phase: ExilePhase,
    attested_fraud_logs: BTreeSet<[u8; 32]>,
    fraud_votes: BTreeSet<(u64, [u8; 32])>,
    rehab_votes: BTreeSet<(u64, ExilePhase, [u8; 32])>,
}

impl Default for RehabLattice {
    fn default() -> Self {
        Self::new_active()
    }
}

impl RehabLattice {
    /// Constructs a healthy peer at generation `0` in [`ExilePhase::Active`].
    #[must_use]
    pub const fn new_active() -> Self {
        Self {
            generation: 0,
            phase: ExilePhase::Active,
            attested_fraud_logs: BTreeSet::new(),
            fraud_votes: BTreeSet::new(),
            rehab_votes: BTreeSet::new(),
        }
    }

    /// Constructs a lattice snapshot directly at `(generation, phase)` for property testing or replay.
    #[must_use]
    pub const fn at_state(generation: u64, phase: ExilePhase) -> Self {
        Self {
            generation,
            phase,
            attested_fraud_logs: BTreeSet::new(),
            fraud_votes: BTreeSet::new(),
            rehab_votes: BTreeSet::new(),
        }
    }

    /// Current exile generation (`0` if never quarantined, `>= 1` once exiled).
    #[must_use]
    pub const fn generation(&self) -> u64 {
        self.generation
    }

    /// Current phase within the active exile generation.
    #[must_use]
    pub const fn phase(&self) -> ExilePhase {
        self.phase
    }

    /// Count of permanent attested fraud digests retained across recoveries.
    #[must_use]
    pub fn attested_fraud_count(&self) -> usize {
        self.attested_fraud_logs.len()
    }

    /// Returns `nominal_weight` strictly when [`ExilePhase::Active`], otherwise `0`.
    #[must_use]
    pub const fn witness_weight(&self, nominal_weight: u64) -> u64 {
        if self.phase.has_witness_power() {
            nominal_weight
        } else {
            0
        }
    }

    /// Records a peer vote to quarantine for an attested fraud digest at the next generation.
    ///
    /// Transitions into `(generation + 1, Quarantined)` once `policy.q_fraud` distinct
    /// voters endorse the exile transition.
    pub fn vote_quarantine(
        &mut self,
        policy: &RehabPolicy,
        voter_id: [u8; 32],
        fraud_digest: [u8; 32],
    ) -> Result<bool, CrdtError> {
        self.attested_fraud_logs.insert(fraud_digest);
        let next_gen = self
            .generation
            .checked_add(1)
            .ok_or(CrdtError::InvalidDomain("exile generation overflow"))?;
        self.fraud_votes.insert((next_gen, voter_id));
        let count = self
            .fraud_votes
            .iter()
            .filter(|(g, _)| *g == next_gen)
            .count();
        if count >= policy.q_fraud {
            self.generation = next_gen;
            self.phase = ExilePhase::Quarantined;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Records a witness vote to advance `self.phase` one step forward within the current generation.
    ///
    /// Transitions occur strictly along `Quarantined -> Canary -> Pharmacovigilance -> Active`,
    /// require at least `policy.q_rehab` votes, and guard `Pharmacovigilance -> Active` with
    /// `vigil_ticks >= policy.vigil_duration(self.generation)`.
    pub fn vote_advance(
        &mut self,
        policy: &RehabPolicy,
        voter_id: [u8; 32],
        vigil_ticks: u64,
    ) -> Result<ExilePhase, CrdtError> {
        let target = match self.phase {
            ExilePhase::Quarantined => ExilePhase::Canary,
            ExilePhase::Canary => ExilePhase::Pharmacovigilance,
            ExilePhase::Pharmacovigilance => ExilePhase::Active,
            ExilePhase::Active => return Ok(ExilePhase::Active),
        };
        if target == ExilePhase::Active {
            let required = policy.vigil_duration(self.generation)?;
            if vigil_ticks < required {
                return Err(CrdtError::QuorumViolation(
                    "pharmacovigilance backoff window has not elapsed",
                ));
            }
        }
        self.rehab_votes.insert((self.generation, target, voter_id));
        let count = self
            .rehab_votes
            .iter()
            .filter(|(g, p, _)| *g == self.generation && *p == target)
            .count();
        if count >= policy.q_rehab {
            self.phase = target;
        }
        Ok(self.phase)
    }
}

impl JoinSemilattice for RehabLattice {
    fn join(&mut self, other: &Self) {
        if (other.generation, other.phase) > (self.generation, self.phase) {
            self.generation = other.generation;
            self.phase = other.phase;
        }
        self.attested_fraud_logs
            .extend(other.attested_fraud_logs.iter().copied());
        self.fraud_votes.extend(other.fraud_votes.iter().copied());
        self.rehab_votes.extend(other.rehab_votes.iter().copied());
    }
}

/// Signed neighbor health witness receipt excluding self-attestation.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct SignedWitness {
    /// Neighbor identifier authoring the witness statement.
    pub issuer: [u8; 32],
    /// Target peer being witnessed.
    pub subject: [u8; 32],
    /// Epoch for which conductance is attested.
    pub epoch: u64,
    /// Witnessed peer conductance or health score in millionths.
    pub conductance_millionths: i64,
    /// Content digest binding the attested state payload.
    pub digest: [u8; 32],
}

impl SignedWitness {
    /// Validates that a receipt is not a self-report (`issuer != subject`).
    pub fn ensure_non_self(&self) -> Result<(), CrdtError> {
        if self.issuer == self.subject {
            Err(CrdtError::QuorumViolation("self-report witness rejected"))
        } else {
            Ok(())
        }
    }
}

/// Computes the lower median conductance for `subject` at `epoch` from neighbor receipts.
///
/// Enforces three invariants:
/// 1. Self-reports (`issuer == subject`) are rejected;
/// 2. Only peers currently in [`ExilePhase::Active`] contribute non-zero witness weight;
/// 3. Conflicting claims by the same `issuer` for `(subject, epoch)` raise an equivocation error.
pub fn cross_witness_median(
    subject: &[u8; 32],
    epoch: u64,
    receipts: &[(&SignedWitness, ExilePhase)],
) -> Result<i64, CrdtError> {
    let mut per_issuer: BTreeMap<[u8; 32], (i64, [u8; 32])> = BTreeMap::new();
    for (receipt, issuer_phase) in receipts {
        if receipt.subject != *subject || receipt.epoch != epoch {
            continue;
        }
        receipt.ensure_non_self()?;
        if !issuer_phase.has_witness_power() {
            continue;
        }
        if let Some((prev_val, prev_digest)) = per_issuer.get(&receipt.issuer) {
            if *prev_val != receipt.conductance_millionths || *prev_digest != receipt.digest {
                return Err(CrdtError::QuorumViolation(
                    "equivocating witness receipts from same issuer in epoch",
                ));
            }
        } else {
            per_issuer.insert(receipt.issuer, (receipt.conductance_millionths, receipt.digest));
        }
    }
    if per_issuer.is_empty() {
        return Err(CrdtError::QuorumViolation(
            "no active cross-witness receipts available",
        ));
    }
    let mut values: Vec<i64> = per_issuer.values().map(|(v, _)| *v).collect();
    values.sort_unstable();
    Ok(values[(values.len() - 1) / 2])
}

/// Objective, delay-free proof that a peer under-reported its neighbor count or degree.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DegreeLieProof {
    /// Accused unit identifier.
    pub accused: [u8; 32],
    /// Epoch in which the degree report occurred.
    pub epoch: u64,
    /// Degree claimed by the accused peer.
    pub claimed_deg: u32,
    /// Distinct neighbors independently corroborated as adjacent to `accused`.
    pub witnessed_neighbors: Vec<[u8; 32]>,
    /// Active peer witnesses signing this proof.
    pub witnesses: Vec<[u8; 32]>,
}

impl DegreeLieProof {
    /// Verifies that `witnesses` satisfies quorum `q_fraud`, excludes the accused, and that
    /// the count of distinct corroborated neighbors strictly exceeds `claimed_deg`.
    pub fn verify(&self, q_fraud: usize) -> Result<(), CrdtError> {
        if q_fraud == 0 {
            return Err(CrdtError::InvalidDomain("q_fraud must be positive"));
        }
        let distinct_witnesses: BTreeSet<[u8; 32]> = self
            .witnesses
            .iter()
            .copied()
            .filter(|w| *w != self.accused)
            .collect();
        if distinct_witnesses.len() < q_fraud {
            return Err(CrdtError::QuorumViolation(
                "insufficient distinct active witnesses for DegreeLieProof",
            ));
        }
        let distinct_neighbors: BTreeSet<[u8; 32]> = self
            .witnessed_neighbors
            .iter()
            .copied()
            .filter(|n| *n != self.accused)
            .collect();
        if distinct_neighbors.len() <= usize::try_from(self.claimed_deg).unwrap_or(usize::MAX) {
            return Err(CrdtError::QuorumViolation(
                "corroborated neighbor count does not contradict claimed_deg",
            ));
        }
        Ok(())
    }
}

/// Bundle held in delay-tolerant custody across intermittent partitions.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StoreCarryBundle<T> {
    /// Payload held in custody.
    pub payload: T,
    /// Last epoch during which delivery remains valid.
    pub expiry_epoch: u64,
    /// Marks critical safety/algedonic bundles prioritized over routine telemetry.
    pub urgent: bool,
}

/// Bounded delay-tolerant store-carry-forward queue with Kingman congestion backpressure.
///
/// When buffered occupancy reaches `shed_threshold`, routine non-urgent telemetry is refused
/// (`Err(CrdtError::CapacityFull)`) while preserving headroom up to `capacity` for urgent
/// survival signals; stale expired bundles are evicted deterministically ahead of insertion.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StoreCarryQueue<T> {
    capacity: usize,
    shed_threshold: usize,
    items: VecDeque<StoreCarryBundle<T>>,
}

impl<T> StoreCarryQueue<T> {
    /// Creates a bounded DTN custody queue with Kingman backpressure shedding above `shed_threshold`.
    pub fn new(capacity: usize, shed_threshold: usize) -> Result<Self, CrdtError> {
        if capacity == 0 || shed_threshold == 0 || shed_threshold > capacity {
            return Err(CrdtError::InvalidDomain(
                "StoreCarryQueue requires 1 <= shed_threshold <= capacity",
            ));
        }
        Ok(Self {
            capacity,
            shed_threshold,
            items: VecDeque::with_capacity(capacity),
        })
    }

    /// Number of bundles currently buffered.
    #[must_use]
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Returns `true` when no bundles are stored.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Fixed-point utilization in millionths (`0..=UNIT`).
    pub fn utilization_millionths(&self) -> Result<i64, CrdtError> {
        let len_i = i64::try_from(self.items.len())
            .map_err(|_| CrdtError::InvalidDomain("queue length exceeds i64"))?;
        let cap_i = i64::try_from(self.capacity)
            .map_err(|_| CrdtError::InvalidDomain("queue capacity exceeds i64"))?;
        let scaled = i128::from(len_i) * i128::from(UNIT) / i128::from(cap_i);
        i64::try_from(scaled).map_err(|_| CrdtError::InvalidDomain("utilization overflow"))
    }

    /// Purges bundles strictly past `now_epoch` (`expiry_epoch < now_epoch`), returning purged count.
    pub fn purge_expired(&mut self, now_epoch: u64) -> usize {
        let before = self.items.len();
        self.items.retain(|b| b.expiry_epoch >= now_epoch);
        before - self.items.len()
    }

    /// Enqueues `bundle` at `now_epoch`, shedding expired entries first and applying Kingman
    /// backpressure (`len >= shed_threshold` sheds non-urgent payloads).
    pub fn enqueue(&mut self, bundle: StoreCarryBundle<T>, now_epoch: u64) -> Result<(), CrdtError> {
        self.purge_expired(now_epoch);
        if bundle.expiry_epoch < now_epoch {
            return Err(CrdtError::InvalidDomain("bundle already expired"));
        }
        if !bundle.urgent && self.items.len() >= self.shed_threshold {
            return Err(CrdtError::CapacityFull);
        }
        if self.items.len() >= self.capacity {
            if bundle.urgent {
                if let Some(idx) = self.items.iter().position(|b| !b.urgent) {
                    let _ = self.items.remove(idx);
                } else {
                    return Err(CrdtError::CapacityFull);
                }
            } else {
                return Err(CrdtError::CapacityFull);
            }
        }
        self.items.push_back(bundle);
        Ok(())
    }

    /// Pops the oldest non-expired bundle, preferring urgent signals when saturated at or above
    /// `shed_threshold`.
    pub fn pop_ready(&mut self, now_epoch: u64) -> Option<StoreCarryBundle<T>> {
        self.purge_expired(now_epoch);
        if self.items.len() >= self.shed_threshold {
            if let Some(idx) = self.items.iter().position(|b| b.urgent) {
                return self.items.remove(idx);
            }
        }
        self.items.pop_front()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(byte: u8) -> [u8; 32] {
        [byte; 32]
    }

    #[test]
    fn spent_capabilities_and_epoch_state_satisfy_aci() {
        let mut a = LeontiefEpochState::new(3, 700_000, 600_000, 800_000);
        assert!(a.spent_capabilities.consume(id(1)));
        assert!(!a.spent_capabilities.consume(id(1)));

        let mut b = LeontiefEpochState::new(3, 500_000, 900_000, 750_000);
        assert!(b.spent_capabilities.consume(id(2)));

        let mut ab = a.clone();
        ab.join(&b);
        let mut ba = b.clone();
        ba.join(&a);
        assert_eq!(ab, ba);
        assert_eq!(ab.min_s_epi, 500_000);
        assert_eq!(ab.min_s_vital, 600_000);
        assert_eq!(ab.min_s_gov, 750_000);
        assert!(ab.spent_capabilities.is_spent(&id(1)));
        assert!(ab.spent_capabilities.is_spent(&id(2)));
        assert_eq!(ab.leontief_floor(), 500_000);

        // Idempotence.
        let before = ab.clone();
        ab.join(&before);
        assert_eq!(ab, before);

        // Higher epoch supersedes older epoch vector while keeping spent capabilities.
        let mut c = LeontiefEpochState::new(4, 950_000, 950_000, 950_000);
        c.spent_capabilities.consume(id(3));
        ab.join(&c);
        assert_eq!(ab.epoch, 4);
        assert_eq!(ab.leontief_floor(), 950_000);
        assert!(ab.spent_capabilities.is_spent(&id(1)));
        assert!(ab.spent_capabilities.is_spent(&id(2)));
        assert!(ab.spent_capabilities.is_spent(&id(3)));
    }

    #[test]
    fn rehabilitation_lattice_enforces_probation_zero_weight_and_backoff() {
        let policy = RehabPolicy::new(2, 3, 100).expect("valid policy");
        assert!(matches!(RehabPolicy::new(3, 3, 100), Err(CrdtError::InvalidDomain(_))));
        assert!(matches!(RehabPolicy::new(1, 2, 0), Err(CrdtError::InvalidDomain(_))));

        let mut node = RehabLattice::new_active();
        assert_eq!(node.witness_weight(10), 10);

        // First fraud vote alone does not quarantine.
        assert!(!node.vote_quarantine(&policy, id(1), id(99)).expect("ok"));
        // Second distinct vote transitions into gen 1 Quarantined.
        assert!(node.vote_quarantine(&policy, id(2), id(99)).expect("ok"));
        assert_eq!(node.generation(), 1);
        assert_eq!(node.phase(), ExilePhase::Quarantined);
        assert_eq!(node.witness_weight(10), 0);
        assert_eq!(node.conductance_cap_or_default(), 0);

        // Advance to Canary then Pharmacovigilance with q_rehab = 3; witness weight stays zero.
        for v in [id(1), id(2), id(3)] {
            node.vote_advance(&policy, v, 0).expect("advance canary");
        }
        assert_eq!(node.phase(), ExilePhase::Canary);
        assert!(!node.phase().has_witness_power());
        assert_eq!(node.witness_weight(10), 0);
        assert_eq!(node.phase().conductance_cap_millionths(), 50_000);

        for v in [id(1), id(2), id(3)] {
            node.vote_advance(&policy, v, 0).expect("advance vigil");
        }
        assert_eq!(node.phase(), ExilePhase::Pharmacovigilance);
        assert!(!node.phase().has_witness_power());
        assert_eq!(node.witness_weight(10), 0);
        assert_eq!(node.phase().conductance_cap_millionths(), 350_000);

        // Cannot enter Active before vigil duration (t_base * 2^(1-1) = 100).
        assert!(matches!(
            node.vote_advance(&policy, id(1), 99),
            Err(CrdtError::QuorumViolation(_))
        ));
        for v in [id(1), id(2), id(3)] {
            node.vote_advance(&policy, v, 100).expect("advance active");
        }
        assert_eq!(node.phase(), ExilePhase::Active);
        assert_eq!(node.witness_weight(10), 10);
        // Fraud log stays preserved forever after rehabilitation.
        assert_eq!(node.attested_fraud_count(), 1);

        // Second exile doubles probation to 200.
        assert_eq!(policy.vigil_duration(2), Ok(200));
        assert_eq!(policy.vigil_duration(3), Ok(400));
    }

    impl RehabLattice {
        fn conductance_cap_or_default(&self) -> i64 {
            self.phase.conductance_cap_millionths()
        }
    }

    #[test]
    fn cross_witness_excludes_self_probation_and_equivocation() {
        let target = id(10);
        let w_self = SignedWitness {
            issuer: target,
            subject: target,
            epoch: 5,
            conductance_millionths: 999_999,
            digest: id(1),
        };
        assert!(matches!(w_self.ensure_non_self(), Err(CrdtError::QuorumViolation(_))));

        let w1 = SignedWitness {
            issuer: id(1),
            subject: target,
            epoch: 5,
            conductance_millionths: 400_000,
            digest: id(11),
        };
        let w2 = SignedWitness {
            issuer: id(2),
            subject: target,
            epoch: 5,
            conductance_millionths: 800_000,
            digest: id(12),
        };
        let w3_probation = SignedWitness {
            issuer: id(3),
            subject: target,
            epoch: 5,
            conductance_millionths: 50_000,
            digest: id(13),
        };

        // Probationary peers (Canary / Pharmacovigilance) contribute zero witness weight.
        let med = cross_witness_median(
            &target,
            5,
            &[
                (&w1, ExilePhase::Active),
                (&w2, ExilePhase::Active),
                (&w3_probation, ExilePhase::Pharmacovigilance),
                (&w3_probation, ExilePhase::Canary),
            ],
        )
        .expect("median");
        assert_eq!(med, 400_000);

        // Conflicting claim from same issuer in same epoch is flagged as equivocation.
        let w2_equiv = SignedWitness {
            issuer: id(2),
            subject: target,
            epoch: 5,
            conductance_millionths: 100_000,
            digest: id(99),
        };
        assert!(matches!(
            cross_witness_median(
                &target,
                5,
                &[(&w1, ExilePhase::Active), (&w2, ExilePhase::Active), (&w2_equiv, ExilePhase::Active)],
            ),
            Err(CrdtError::QuorumViolation(_))
        ));
    }

    #[test]
    fn degree_lie_proof_and_dtn_backpressure() {
        let proof = DegreeLieProof {
            accused: id(9),
            epoch: 12,
            claimed_deg: 1,
            witnessed_neighbors: alloc::vec![id(1), id(2)],
            witnesses: alloc::vec![id(3), id(4)],
        };
        assert_eq!(proof.verify(2), Ok(()));
        assert!(matches!(proof.verify(3), Err(CrdtError::QuorumViolation(_))));

        let mut dtn = StoreCarryQueue::new(3, 2).expect("valid");
        dtn.enqueue(StoreCarryBundle { payload: 1_u8, expiry_epoch: 10, urgent: false }, 1)
            .expect("slot 1");
        dtn.enqueue(StoreCarryBundle { payload: 2_u8, expiry_epoch: 10, urgent: false }, 1)
            .expect("slot 2");
        // Non-urgent telemetry is shed once utilization reaches shed_threshold (2).
        assert_eq!(
            dtn.enqueue(StoreCarryBundle { payload: 3_u8, expiry_epoch: 10, urgent: false }, 1),
            Err(CrdtError::CapacityFull)
        );
        // Urgent bundle still enters the reserved slot and pops first under saturation.
        dtn.enqueue(StoreCarryBundle { payload: 99_u8, expiry_epoch: 10, urgent: true }, 1)
            .expect("urgent bypass");
        assert_eq!(dtn.pop_ready(1).map(|b| b.payload), Some(99));
    }
}
