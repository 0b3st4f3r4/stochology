//! Decentralized peer membership overlay (`HyParView`) with two-phase rewiring commit
//! ensuring zero orphan stubs and zero leaked locks across all abort/timeout/commit outcomes.

use std::collections::{BTreeMap, BTreeSet};

use crate::HostError;

/// Fixed 32-byte peer identifier in the overlay graph.
pub type PeerId = [u8; 32];

#[derive(Clone, PartialEq, Eq, Debug)]
struct PendingRewire {
    candidate: PeerId,
    displace: Option<PeerId>,
    locked_peers: BTreeSet<PeerId>,
    approvals: BTreeSet<PeerId>,
    rejections: BTreeSet<PeerId>,
    phase_b1_locked: bool,
}

/// Bounded `HyParView` membership overlay supporting 2PC neighbor rewiring (`Phase A -> B1 -> B2`).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct HyParViewOverlay {
    max_active: usize,
    max_passive: usize,
    active: BTreeSet<PeerId>,
    passive: BTreeSet<PeerId>,
    pending: BTreeMap<u64, PendingRewire>,
    active_locks: BTreeMap<PeerId, u64>,
}

impl HyParViewOverlay {
    /// Initializes an overlay with strict positive active and passive capacity ceilings.
    pub fn new(max_active: usize, max_passive: usize) -> Result<Self, HostError> {
        if max_active == 0 || max_passive == 0 {
            return Err(HostError::InvalidConfiguration(
                "HyParView requires max_active > 0 and max_passive > 0",
            ));
        }
        Ok(Self {
            max_active,
            max_passive,
            active: BTreeSet::new(),
            passive: BTreeSet::new(),
            pending: BTreeMap::new(),
            active_locks: BTreeMap::new(),
        })
    }

    /// Inserts an initial peer into the active view (or spills to passive if active is full).
    pub fn seed_peer(&mut self, peer: PeerId) -> bool {
        if self.active.contains(&peer) {
            return true;
        }
        if self.active.len() < self.max_active {
            self.passive.remove(&peer);
            self.active.insert(peer)
        } else if self.passive.len() < self.max_passive {
            self.passive.insert(peer)
        } else {
            false
        }
    }

    /// Current count of peers in the active view.
    #[must_use]
    pub fn active_len(&self) -> usize {
        self.active.len()
    }

    /// Current count of peers in the passive view.
    #[must_use]
    pub fn passive_len(&self) -> usize {
        self.passive.len()
    }

    /// Number of uncommitted pending proposal stubs currently open.
    #[must_use]
    pub fn pending_stubs_count(&self) -> usize {
        self.pending.len()
    }

    /// Number of peer locks currently held by open two-phase rewiring proposals.
    #[must_use]
    pub fn active_locks_count(&self) -> usize {
        self.active_locks.len()
    }

    /// Returns `true` if `peer` is currently in the active view.
    #[must_use]
    pub fn is_active(&self, peer: &PeerId) -> bool {
        self.active.contains(peer)
    }

    /// Phase A: registers a bounded rewiring proposal without acquiring locks yet.
    pub fn propose_phase_a(
        &mut self,
        proposal_id: u64,
        candidate: PeerId,
        displace: Option<PeerId>,
    ) -> Result<(), HostError> {
        if self.pending.contains_key(&proposal_id) {
            return Err(HostError::HyParViewConflict("duplicate proposal_id"));
        }
        if let Some(d) = displace {
            if d == candidate || !self.active.contains(&d) {
                return Err(HostError::HyParViewConflict(
                    "displaced peer must be an active peer distinct from candidate",
                ));
            }
        } else if self.active.len() >= self.max_active && !self.active.contains(&candidate) {
            return Err(HostError::HyParViewConflict(
                "active view full requires displace target",
            ));
        }

        self.pending.insert(
            proposal_id,
            PendingRewire {
                candidate,
                displace,
                locked_peers: BTreeSet::new(),
                approvals: BTreeSet::new(),
                rejections: BTreeSet::new(),
                phase_b1_locked: false,
            },
        );
        Ok(())
    }

    /// Phase B1 lock acquisition: atomically locks involved peers or fails cleanly with zero leaks.
    pub fn acquire_phase_b1_locks(&mut self, proposal_id: u64) -> Result<(), HostError> {
        let entry = self
            .pending
            .get(&proposal_id)
            .ok_or(HostError::HyParViewConflict("unknown proposal_id"))?;
        let candidate = entry.candidate;
        let displace = entry.displace;

        if self.active_locks.contains_key(&candidate) {
            return Err(HostError::HyParViewConflict("candidate peer already locked"));
        }
        if let Some(d) = displace {
            if self.active_locks.contains_key(&d) {
                return Err(HostError::HyParViewConflict("displace peer already locked"));
            }
        }

        let mut locked = BTreeSet::new();
        self.active_locks.insert(candidate, proposal_id);
        locked.insert(candidate);
        if let Some(d) = displace {
            self.active_locks.insert(d, proposal_id);
            locked.insert(d);
        }

        if let Some(mut_entry) = self.pending.get_mut(&proposal_id) {
            mut_entry.locked_peers = locked;
            mut_entry.phase_b1_locked = true;
        }
        Ok(())
    }

    /// Phase B1 voting: records an approval or rejection vote from `voter`.
    pub fn vote_phase_b1(
        &mut self,
        proposal_id: u64,
        voter: PeerId,
        approve: bool,
    ) -> Result<(), HostError> {
        let entry = self
            .pending
            .get_mut(&proposal_id)
            .ok_or(HostError::HyParViewConflict("unknown proposal_id"))?;
        if approve {
            entry.rejections.remove(&voter);
            entry.approvals.insert(voter);
        } else {
            entry.approvals.remove(&voter);
            entry.rejections.insert(voter);
        }
        Ok(())
    }

    /// Phase B2 commit: applies the swap only if `quorum` approvals hold with zero rejections,
    /// always releasing all pending stubs and peer locks even if commit fails validation.
    pub fn commit_phase_b2(&mut self, proposal_id: u64, quorum: usize) -> Result<bool, HostError> {
        let entry = self
            .pending
            .remove(&proposal_id)
            .ok_or(HostError::HyParViewConflict("unknown proposal_id"))?;
        self.release_locks(&entry);

        if quorum == 0
            || !entry.phase_b1_locked
            || !entry.rejections.is_empty()
            || entry.approvals.len() < quorum
        {
            return Ok(false);
        }

        if let Some(d) = entry.displace {
            if !self.active.remove(&d) {
                return Ok(false);
            }
            if self.passive.len() < self.max_passive {
                self.passive.insert(d);
            }
        } else if self.active.len() >= self.max_active && !self.active.contains(&entry.candidate) {
            return Ok(false);
        }

        self.passive.remove(&entry.candidate);
        self.active.insert(entry.candidate);
        Ok(true)
    }

    /// Phase B2 abort / Phase B1 timeout cleanup: drops stub and frees every lock without modifying views.
    pub fn abort_or_timeout(&mut self, proposal_id: u64) -> Result<(), HostError> {
        let entry = self
            .pending
            .remove(&proposal_id)
            .ok_or(HostError::HyParViewConflict("unknown proposal_id"))?;
        self.release_locks(&entry);
        Ok(())
    }

    fn release_locks(&mut self, entry: &PendingRewire) {
        for peer in &entry.locked_peers {
            self.active_locks.remove(peer);
        }
    }
}
