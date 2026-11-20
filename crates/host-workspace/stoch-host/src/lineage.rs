//! Contestatory peaceful lineage branching (`ForkLocalLineage`) backed by [`CustodyLedger`].
//!
//! Dissenting nodes may refuse (`r2_refuse`) or restore (`r2_unrefuse`) participation in a
//! specific collective commitment without stalling surviving honest threshold quorums and
//! without consuming linear `UCAN` capabilities in `SPENT_UCANS`.

use stoch_custody::CustodyLedger;

use crate::HostError;

/// Operations managing non-coercive peaceful lineage branching without Byzantine sanctions.
pub struct ForkLocalLineage;

impl ForkLocalLineage {
    /// Persists a peaceful local abstention (`r2-refuse`) against `target_id` without spending
    /// any `UCAN` authorization token in `SPENT_UCANS`.
    pub fn r2_refuse(ledger: &CustodyLedger, target_id: &[u8; 32]) -> Result<(), HostError> {
        ledger.refuse_r2_target(target_id)?;
        Ok(())
    }

    /// Hot-revokes a prior peaceful local abstention (`r2-unrefuse`) leaving `SPENT_UCANS` and
    /// sealed custody tapes intact.
    pub fn r2_unrefuse(ledger: &CustodyLedger, target_id: &[u8; 32]) -> Result<bool, HostError> {
        Ok(ledger.unrefuse_r2_target(target_id)?)
    }

    /// Checks whether the remaining honest signers after peaceful abstentions can still meet `threshold`.
    #[must_use]
    pub fn remaining_quorum_viable(
        total_peers: usize,
        abstained_peers: usize,
        threshold: usize,
    ) -> bool {
        total_peers.saturating_sub(abstained_peers) >= threshold
    }
}
