//! `redb` table definitions and BLAKE3 domain separation prefixes for `stoch-custody`.

use redb::TableDefinition;

/// Immutable control-intent tape keyed by monotonic 1-based sequence number.
pub const CONTROL_TAPE: TableDefinition<'static, u64, &'static [u8]> =
    TableDefinition::new("control_tape");
/// Immutable world/contingency observation tape keyed by monotonic 1-based sequence number.
pub const CONTINGENCY_TAPE: TableDefinition<'static, u64, &'static [u8]> =
    TableDefinition::new("contingency_tape");
/// Append-only attested counter-inductive reopening log keyed by 1-based sequence number.
pub const REOPEN_LOG: TableDefinition<'static, u64, &'static [u8]> =
    TableDefinition::new("reopen_log");
/// Monotone grow-only set of consumed linear authorization identifiers (`UCAN` digests).
pub const SPENT_UCANS: TableDefinition<'static, &'static [u8; 32], &'static [u8; 32]> =
    TableDefinition::new("spent_ucans");
/// Monotone grow-only set of peaceful lineage refusal targets preserved across rollbacks.
pub const REFUSED_R2_TARGETS: TableDefinition<'static, &'static [u8; 32], &'static [u8; 32]> =
    TableDefinition::new("refused_r2_targets");
/// Named recovery checkpoints for bounded `R0` state rewinds.
pub const R0_SNAPSHOTS: TableDefinition<'static, u64, &'static [u8]> =
    TableDefinition::new("r0_snapshots");
/// Monotonic transport delegation sequence numbers per sovereign identity (`seq` LE + `BLAKE3`).
pub const DELEGATION_SEQS: TableDefinition<'static, &'static [u8; 32], &'static [u8; 40]> =
    TableDefinition::new("delegation_seqs");
/// Authenticated metadata pointers tracking active heads and total immutable seal counts.
pub const META: TableDefinition<'static, &'static str, &'static [u8]> =
    TableDefinition::new("custody_meta");

pub(crate) const META_HEADS_KEY: &str = "heads_v1";

pub(crate) const DOMAIN_CONTROL: &[u8] = b"stoch-custody/control/v1";
pub(crate) const DOMAIN_CONTINGENCY: &[u8] = b"stoch-custody/contingency/v1";
pub(crate) const DOMAIN_REOPEN: &[u8] = b"stoch-custody/reopen/v1";
pub(crate) const DOMAIN_UCAN: &[u8] = b"stoch-custody/ucan-spent/v1";
pub(crate) const DOMAIN_R2_REFUSAL: &[u8] = b"stoch-custody/r2-refusal/v1";
pub(crate) const DOMAIN_DELEG_SEQ: &[u8] = b"stoch-custody/deleg-seq/v1";
pub(crate) const DOMAIN_SNAPSHOT: &[u8] = b"stoch-custody/r0-snapshot/v1";
pub(crate) const DOMAIN_META: &[u8] = b"stoch-custody/meta-heads/v1";
