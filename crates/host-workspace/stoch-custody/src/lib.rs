//! Host-side bitemporal two-tape custody ledger backed by `redb` (2PC + immediate durability)
//! and per-read `BLAKE3` integrity verification.
//!
//! Preserves strict linear `SPENT_UCANS` anti-replay records and `REFUSED_R2_TARGETS` across
//! `rollback_r0`, keeps `CONTROL_TAPE`, `CONTINGENCY_TAPE`, and `REOPEN_LOG` immutable, and
//! guards counter-inductive frame reopenings behind positive custodian escrow.

#![deny(unsafe_code)]
#![deny(missing_docs)]
#![deny(clippy::float_arithmetic)]
#![forbid(clippy::cast_possible_truncation)]

use std::path::Path;

use redb::{Database, Durability, ReadableTable, TableDefinition};

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

const META_HEADS_KEY: &str = "heads_v1";

const DOMAIN_CONTROL: &[u8] = b"stoch-custody/control/v1";
const DOMAIN_CONTINGENCY: &[u8] = b"stoch-custody/contingency/v1";
const DOMAIN_REOPEN: &[u8] = b"stoch-custody/reopen/v1";
const DOMAIN_UCAN: &[u8] = b"stoch-custody/ucan-spent/v1";
const DOMAIN_R2_REFUSAL: &[u8] = b"stoch-custody/r2-refusal/v1";
const DOMAIN_DELEG_SEQ: &[u8] = b"stoch-custody/deleg-seq/v1";
const DOMAIN_SNAPSHOT: &[u8] = b"stoch-custody/r0-snapshot/v1";
const DOMAIN_META: &[u8] = b"stoch-custody/meta-heads/v1";

/// Custody validation, integrity, bitemporal ordering, or backend storage error.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum CustodyError {
    /// Rejects `valid_ts > tx_ts` where claimed world time exceeds archival availability time.
    InvalidBitemporalOrder {
        /// Event validity timestamp.
        valid_ts: u64,
        /// Archive transaction availability timestamp.
        tx_ts: u64,
    },
    /// Detects byte tampering, bit-rot, malformed framing, or broken hash chaining on read/verify.
    IntegrityCorruption(&'static str),
    /// Requested sequence number or snapshot identifier does not exist.
    MissingRecord(&'static str),
    /// Linear capability identifier was already spent and cannot be replayed.
    ReplayRejected([u8; 32]),
    /// Counter-inductive reopening requires strictly positive custodian escrow.
    ZeroEscrow,
    /// Sequence counter or payload length exceeded representable limits.
    Overflow,
    /// Underlying `redb` database or transaction failure.
    Storage(String),
}

impl core::fmt::Display for CustodyError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidBitemporalOrder { valid_ts, tx_ts } => {
                write!(f, "invalid bitemporal order: valid_ts ({valid_ts}) > tx_ts ({tx_ts})")
            }
            Self::IntegrityCorruption(msg) => write!(f, "custody integrity corruption: {msg}"),
            Self::MissingRecord(msg) => write!(f, "missing custody record: {msg}"),
            Self::ReplayRejected(_) => write!(f, "spent linear authorization rejected on replay"),
            Self::ZeroEscrow => write!(f, "counter-inductive reopening requires escrow > 0"),
            Self::Overflow => write!(f, "custody counter or payload length overflow"),
            Self::Storage(msg) => write!(f, "custody storage error: {msg}"),
        }
    }
}

impl std::error::Error for CustodyError {}

fn map_storage<E: core::fmt::Display>(err: E) -> CustodyError {
    CustodyError::Storage(err.to_string())
}

/// Decoded and `BLAKE3`-verified entry on either [`CONTROL_TAPE`] or [`CONTINGENCY_TAPE`].
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TapeRecord {
    /// Monotonic 1-based physical record index.
    pub seq: u64,
    /// Valid world event timestamp (`valid_ts <= tx_ts`).
    pub valid_ts: u64,
    /// Archive commitment timestamp.
    pub tx_ts: u64,
    /// Predecessor sequence number on this lineage (`0` if root).
    pub prev_seq: u64,
    /// Predecessor record seal digest (`[0; 32]` if `prev_seq == 0`).
    pub prev_hash: [u8; 32],
    /// Raw opaque sealed payload bytes.
    pub payload: Vec<u8>,
    /// Domain-separated `BLAKE3` digest sealing the record fields and payload.
    pub seal_hash: [u8; 32],
}

/// Decoded and `BLAKE3`-verified entry from [`REOPEN_LOG`].
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CounterInductiveReopen {
    /// Monotonic 1-based reopening entry sequence number.
    pub seq: u64,
    /// Valid world timestamp (`valid_ts <= tx_ts`).
    pub valid_ts: u64,
    /// Archive commitment timestamp.
    pub tx_ts: u64,
    /// Custodian escrow bond staked for frame expansion (strictly positive).
    pub escrow_deposit: u64,
    /// Anchored sequence number on [`CONTROL_TAPE`] referenced by this reopening.
    pub prior_control_seq: u64,
    /// Verified `BLAKE3` seal digest of the anchored [`CONTROL_TAPE`] entry.
    pub prior_control_hash: [u8; 32],
    /// Predecessor reopening sequence (`0` if first).
    pub prev_seq: u64,
    /// Predecessor reopening seal digest (`[0; 32]` if `prev_seq == 0`).
    pub prev_hash: [u8; 32],
    /// Documented rationale or protocol expansion note.
    pub rationale: Vec<u8>,
    /// Domain-separated `BLAKE3` digest sealing all fields of this reopening record.
    pub seal_hash: [u8; 32],
}

/// Current active operational heads alongside immutable total seal counts.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CustodyHeads {
    /// Active head sequence on `CONTROL_TAPE` (rewound by `rollback_r0`).
    pub control_active_seq: u64,
    /// Total immutable records written to `CONTROL_TAPE`.
    pub control_total_sealed: u64,
    /// Active head sequence on `CONTINGENCY_TAPE` (rewound by `rollback_r0`).
    pub contingency_active_seq: u64,
    /// Total immutable records written to `CONTINGENCY_TAPE`.
    pub contingency_total_sealed: u64,
    /// Total immutable records written to `REOPEN_LOG`.
    pub reopen_total: u64,
}

#[derive(Clone, Copy)]
struct SnapshotRecord {
    snapshot_id: u64,
    control_head_seq: u64,
    control_head_hash: [u8; 32],
    contingency_head_seq: u64,
    contingency_head_hash: [u8; 32],
}

/// Host custody ledger wrapping `redb` (2PC + `Durability::Immediate`) and per-read `BLAKE3`.
pub struct CustodyLedger {
    db: Database,
}

impl CustodyLedger {
    /// Opens or initializes a custody database at `path`, creating all required tables under
    /// immediate-durability two-phase commit and verifying existing metadata integrity.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, CustodyError> {
        let db = Database::create(path.as_ref()).map_err(map_storage)?;
        {
            let mut tx = db.begin_write().map_err(map_storage)?;
            tx.set_durability(Durability::Immediate);
            tx.set_two_phase_commit(true);
            {
                let _ = tx.open_table(CONTROL_TAPE).map_err(map_storage)?;
                let _ = tx.open_table(CONTINGENCY_TAPE).map_err(map_storage)?;
                let _ = tx.open_table(REOPEN_LOG).map_err(map_storage)?;
                let _ = tx.open_table(SPENT_UCANS).map_err(map_storage)?;
                let _ = tx.open_table(REFUSED_R2_TARGETS).map_err(map_storage)?;
                let _ = tx.open_table(R0_SNAPSHOTS).map_err(map_storage)?;
                let _ = tx.open_table(DELEGATION_SEQS).map_err(map_storage)?;
                let mut meta = tx.open_table(META).map_err(map_storage)?;
                let existing = meta
                    .get(META_HEADS_KEY)
                    .map_err(map_storage)?
                    .map(|g| decode_heads(g.value()));
                match existing {
                    Some(res) => {
                        let _ = res?;
                    }
                    None => {
                        let init = CustodyHeads {
                            control_active_seq: 0,
                            control_total_sealed: 0,
                            contingency_active_seq: 0,
                            contingency_total_sealed: 0,
                            reopen_total: 0,
                        };
                        let bytes = encode_heads(&init);
                        meta.insert(META_HEADS_KEY, bytes.as_slice())
                            .map_err(map_storage)?;
                    }
                }
            }
            tx.commit().map_err(map_storage)?;
        }
        Ok(Self { db })
    }

    fn begin_durable_write(&self) -> Result<redb::WriteTransaction, CustodyError> {
        let mut tx = self.db.begin_write().map_err(map_storage)?;
        tx.set_durability(Durability::Immediate);
        tx.set_two_phase_commit(true);
        Ok(tx)
    }

    /// Reads and `BLAKE3`-verifies the current operational heads and seal totals.
    pub fn heads(&self) -> Result<CustodyHeads, CustodyError> {
        let tx = self.db.begin_read().map_err(map_storage)?;
        let meta = tx.open_table(META).map_err(map_storage)?;
        let guard = meta
            .get(META_HEADS_KEY)
            .map_err(map_storage)?
            .ok_or(CustodyError::MissingRecord("custody heads metadata absent"))?;
        decode_heads(guard.value())
    }

    /// Appends a bitemporal record (`valid_ts <= tx_ts`) to [`CONTROL_TAPE`] with 2PC durability.
    pub fn append_control(
        &self,
        valid_ts: u64,
        tx_ts: u64,
        payload: &[u8],
    ) -> Result<TapeRecord, CustodyError> {
        self.append_tape(true, valid_ts, tx_ts, payload)
    }

    /// Appends a bitemporal record (`valid_ts <= tx_ts`) to [`CONTINGENCY_TAPE`] with 2PC durability.
    pub fn append_contingency(
        &self,
        valid_ts: u64,
        tx_ts: u64,
        payload: &[u8],
    ) -> Result<TapeRecord, CustodyError> {
        self.append_tape(false, valid_ts, tx_ts, payload)
    }

    fn append_tape(
        &self,
        is_control: bool,
        valid_ts: u64,
        tx_ts: u64,
        payload: &[u8],
    ) -> Result<TapeRecord, CustodyError> {
        ensure_bitemporal(valid_ts, tx_ts)?;
        let domain = if is_control {
            DOMAIN_CONTROL
        } else {
            DOMAIN_CONTINGENCY
        };
        let tx = self.begin_durable_write()?;
        let record = {
            let mut meta = tx.open_table(META).map_err(map_storage)?;
            let mut heads = {
                let raw = meta
                    .get(META_HEADS_KEY)
                    .map_err(map_storage)?
                    .ok_or(CustodyError::MissingRecord("custody heads missing"))?;
                decode_heads(raw.value())?
            };

            let mut tape = tx
                .open_table(if is_control {
                    CONTROL_TAPE
                } else {
                    CONTINGENCY_TAPE
                })
                .map_err(map_storage)?;

            let prev_seq = if is_control {
                heads.control_active_seq
            } else {
                heads.contingency_active_seq
            };
            let prev_hash = if prev_seq == 0 {
                [0_u8; 32]
            } else {
                let raw_prev = tape
                    .get(prev_seq)
                    .map_err(map_storage)?
                    .ok_or(CustodyError::IntegrityCorruption(
                        "active head sequence missing from tape",
                    ))?;
                decode_tape_record(domain, prev_seq, raw_prev.value())?.seal_hash
            };

            let total_ref = if is_control {
                &mut heads.control_total_sealed
            } else {
                &mut heads.contingency_total_sealed
            };
            let new_seq = total_ref.checked_add(1).ok_or(CustodyError::Overflow)?;
            *total_ref = new_seq;
            if is_control {
                heads.control_active_seq = new_seq;
            } else {
                heads.contingency_active_seq = new_seq;
            }

            let record = build_tape_record(domain, new_seq, valid_ts, tx_ts, prev_seq, prev_hash, payload)?;
            let encoded = encode_tape_record(&record);
            tape.insert(new_seq, encoded.as_slice())
                .map_err(map_storage)?;

            let heads_bytes = encode_heads(&heads);
            meta.insert(META_HEADS_KEY, heads_bytes.as_slice())
                .map_err(map_storage)?;
            record
        };
        tx.commit().map_err(map_storage)?;
        Ok(record)
    }

    /// Reads and `BLAKE3`-verifies control record `seq` (rejects tampering via `IntegrityCorruption`).
    pub fn read_control(&self, seq: u64) -> Result<TapeRecord, CustodyError> {
        self.read_tape(true, seq)
    }

    /// Reads and `BLAKE3`-verifies contingency record `seq` (rejects tampering via `IntegrityCorruption`).
    pub fn read_contingency(&self, seq: u64) -> Result<TapeRecord, CustodyError> {
        self.read_tape(false, seq)
    }

    fn read_tape(&self, is_control: bool, seq: u64) -> Result<TapeRecord, CustodyError> {
        let domain = if is_control {
            DOMAIN_CONTROL
        } else {
            DOMAIN_CONTINGENCY
        };
        let tx = self.db.begin_read().map_err(map_storage)?;
        let tape = tx
            .open_table(if is_control {
                CONTROL_TAPE
            } else {
                CONTINGENCY_TAPE
            })
            .map_err(map_storage)?;
        let guard = tape
            .get(seq)
            .map_err(map_storage)?
            .ok_or(CustodyError::MissingRecord("tape sequence absent"))?;
        decode_tape_record(domain, seq, guard.value())
    }

    /// Atomically consumes a linear `UCAN` identifier if unseen, returning `ReplayRejected` if spent.
    pub fn consume_ucan(&self, ucan_id: &[u8; 32]) -> Result<(), CustodyError> {
        let tx = self.begin_durable_write()?;
        {
            let mut table = tx.open_table(SPENT_UCANS).map_err(map_storage)?;
            let existing = table.get(ucan_id).map_err(map_storage)?.map(|g| *g.value());
            if let Some(tag) = existing {
                verify_tag(DOMAIN_UCAN, ucan_id, &tag)?;
                return Err(CustodyError::ReplayRejected(*ucan_id));
            }
            let tag = compute_key_tag(DOMAIN_UCAN, ucan_id);
            table.insert(ucan_id, &tag).map_err(map_storage)?;
        }
        tx.commit().map_err(map_storage)?;
        Ok(())
    }

    /// Checks and `BLAKE3`-verifies whether `ucan_id` is marked spent.
    pub fn is_ucan_spent(&self, ucan_id: &[u8; 32]) -> Result<bool, CustodyError> {
        let tx = self.db.begin_read().map_err(map_storage)?;
        let table = tx.open_table(SPENT_UCANS).map_err(map_storage)?;
        match table.get(ucan_id).map_err(map_storage)? {
            Some(tag) => {
                verify_tag(DOMAIN_UCAN, ucan_id, tag.value())?;
                Ok(true)
            }
            None => Ok(false),
        }
    }

    /// Records a persistent peaceful lineage refusal target (`REFUSED_R2_TARGETS`).
    pub fn refuse_r2_target(&self, target_id: &[u8; 32]) -> Result<(), CustodyError> {
        let tx = self.begin_durable_write()?;
        {
            let mut table = tx.open_table(REFUSED_R2_TARGETS).map_err(map_storage)?;
            let existing = table
                .get(target_id)
                .map_err(map_storage)?
                .map(|g| *g.value());
            if let Some(tag) = existing {
                verify_tag(DOMAIN_R2_REFUSAL, target_id, &tag)?;
                return Ok(());
            }
            let tag = compute_key_tag(DOMAIN_R2_REFUSAL, target_id);
            table.insert(target_id, &tag).map_err(map_storage)?;
        }
        tx.commit().map_err(map_storage)?;
        Ok(())
    }

    /// Checks and `BLAKE3`-verifies whether `target_id` is recorded in `REFUSED_R2_TARGETS`.
    pub fn is_r2_refused(&self, target_id: &[u8; 32]) -> Result<bool, CustodyError> {
        let tx = self.db.begin_read().map_err(map_storage)?;
        let table = tx.open_table(REFUSED_R2_TARGETS).map_err(map_storage)?;
        match table.get(target_id).map_err(map_storage)? {
            Some(tag) => {
                verify_tag(DOMAIN_R2_REFUSAL, target_id, tag.value())?;
                Ok(true)
            }
            None => Ok(false),
        }
    }

    /// Hot-revokes a prior peaceful lineage refusal (`r2-unrefuse`), leaving `SPENT_UCANS`
    /// and all sealed tape records intact.
    ///
    /// Returns `Ok(true)` if an existing verified refusal entry was removed, `Ok(false)` if absent.
    pub fn unrefuse_r2_target(&self, target_id: &[u8; 32]) -> Result<bool, CustodyError> {
        let tx = self.begin_durable_write()?;
        let removed = {
            let mut table = tx.open_table(REFUSED_R2_TARGETS).map_err(map_storage)?;
            let existing = table
                .get(target_id)
                .map_err(map_storage)?
                .map(|g| *g.value());
            if let Some(tag) = existing {
                verify_tag(DOMAIN_R2_REFUSAL, target_id, &tag)?;
                let _ = table.remove(target_id).map_err(map_storage)?;
                true
            } else {
                false
            }
        };
        tx.commit().map_err(map_storage)?;
        Ok(removed)
    }

    /// Reads and `BLAKE3`-verifies the persisted monotonic delegation sequence for `sovereign_id`
    /// (`0` if no transport delegation has yet been persisted).
    pub fn delegation_seq(&self, sovereign_id: &[u8; 32]) -> Result<u64, CustodyError> {
        let tx = self.db.begin_read().map_err(map_storage)?;
        let table = tx.open_table(DELEGATION_SEQS).map_err(map_storage)?;
        match table.get(sovereign_id).map_err(map_storage)? {
            Some(raw) => verify_deleg_seq(sovereign_id, raw.value()),
            None => Ok(0),
        }
    }

    /// Persists strictly monotonic `next_seq > current` for `sovereign_id` under 2PC durability.
    ///
    /// Rejects non-increasing or replayed sequences with `CustodyError::ReplayRejected`.
    pub fn advance_delegation_seq(
        &self,
        sovereign_id: &[u8; 32],
        next_seq: u64,
    ) -> Result<(), CustodyError> {
        let tx = self.begin_durable_write()?;
        {
            let mut table = tx.open_table(DELEGATION_SEQS).map_err(map_storage)?;
            let existing = table
                .get(sovereign_id)
                .map_err(map_storage)?
                .map(|g| *g.value());
            let current = match existing {
                Some(raw) => verify_deleg_seq(sovereign_id, &raw)?,
                None => 0,
            };
            if next_seq <= current {
                return Err(CustodyError::ReplayRejected(*sovereign_id));
            }
            let encoded = encode_deleg_seq(sovereign_id, next_seq);
            table.insert(sovereign_id, &encoded).map_err(map_storage)?;
        }
        tx.commit().map_err(map_storage)?;
        Ok(())
    }

    /// Persists an authenticated `R0` recovery checkpoint capturing active control/contingency heads.
    pub fn create_r0_snapshot(&self, snapshot_id: u64) -> Result<CustodyHeads, CustodyError> {
        let tx = self.begin_durable_write()?;
        let heads = {
            let meta = tx.open_table(META).map_err(map_storage)?;
            let heads = {
                let raw = meta
                    .get(META_HEADS_KEY)
                    .map_err(map_storage)?
                    .ok_or(CustodyError::MissingRecord("custody heads missing"))?;
                decode_heads(raw.value())?
            };
            let control_tape = tx.open_table(CONTROL_TAPE).map_err(map_storage)?;
            let contingency_tape = tx.open_table(CONTINGENCY_TAPE).map_err(map_storage)?;

            let control_hash = if heads.control_active_seq == 0 {
                [0_u8; 32]
            } else {
                let raw = control_tape
                    .get(heads.control_active_seq)
                    .map_err(map_storage)?
                    .ok_or(CustodyError::IntegrityCorruption("active control seal absent"))?;
                decode_tape_record(DOMAIN_CONTROL, heads.control_active_seq, raw.value())?.seal_hash
            };
            let contingency_hash = if heads.contingency_active_seq == 0 {
                [0_u8; 32]
            } else {
                let raw = contingency_tape
                    .get(heads.contingency_active_seq)
                    .map_err(map_storage)?
                    .ok_or(CustodyError::IntegrityCorruption("active contingency seal absent"))?;
                decode_tape_record(DOMAIN_CONTINGENCY, heads.contingency_active_seq, raw.value())?
                    .seal_hash
            };

            let snap = SnapshotRecord {
                snapshot_id,
                control_head_seq: heads.control_active_seq,
                control_head_hash: control_hash,
                contingency_head_seq: heads.contingency_active_seq,
                contingency_head_hash: contingency_hash,
            };
            let encoded = encode_snapshot(&snap);
            let mut snapshots = tx.open_table(R0_SNAPSHOTS).map_err(map_storage)?;
            snapshots
                .insert(snapshot_id, encoded.as_slice())
                .map_err(map_storage)?;
            heads
        };
        tx.commit().map_err(map_storage)?;
        Ok(heads)
    }

    /// Rewinds active operational heads to `snapshot_id` without clearing `SPENT_UCANS`,
    /// `REFUSED_R2_TARGETS`, `REOPEN_LOG`, or historical sealed tape records.
    pub fn rollback_r0(&self, snapshot_id: u64) -> Result<CustodyHeads, CustodyError> {
        let tx = self.begin_durable_write()?;
        let new_heads = {
            let mut meta = tx.open_table(META).map_err(map_storage)?;
            let mut heads = {
                let raw = meta
                    .get(META_HEADS_KEY)
                    .map_err(map_storage)?
                    .ok_or(CustodyError::MissingRecord("custody heads missing"))?;
                decode_heads(raw.value())?
            };
            let snapshots = tx.open_table(R0_SNAPSHOTS).map_err(map_storage)?;
            let raw_snap = snapshots
                .get(snapshot_id)
                .map_err(map_storage)?
                .ok_or(CustodyError::MissingRecord("r0 snapshot_id absent"))?;
            let snap = decode_snapshot(snapshot_id, raw_snap.value())?;

            let control_tape = tx.open_table(CONTROL_TAPE).map_err(map_storage)?;
            let contingency_tape = tx.open_table(CONTINGENCY_TAPE).map_err(map_storage)?;

            if snap.control_head_seq > 0 {
                let raw = control_tape
                    .get(snap.control_head_seq)
                    .map_err(map_storage)?
                    .ok_or(CustodyError::IntegrityCorruption("snapshot control seal absent"))?;
                let rec = decode_tape_record(DOMAIN_CONTROL, snap.control_head_seq, raw.value())?;
                if rec.seal_hash != snap.control_head_hash {
                    return Err(CustodyError::IntegrityCorruption(
                        "snapshot control hash mismatch",
                    ));
                }
            }
            if snap.contingency_head_seq > 0 {
                let raw = contingency_tape
                    .get(snap.contingency_head_seq)
                    .map_err(map_storage)?
                    .ok_or(CustodyError::IntegrityCorruption(
                        "snapshot contingency seal absent",
                    ))?;
                let rec = decode_tape_record(
                    DOMAIN_CONTINGENCY,
                    snap.contingency_head_seq,
                    raw.value(),
                )?;
                if rec.seal_hash != snap.contingency_head_hash {
                    return Err(CustodyError::IntegrityCorruption(
                        "snapshot contingency hash mismatch",
                    ));
                }
            }

            heads.control_active_seq = snap.control_head_seq;
            heads.contingency_active_seq = snap.contingency_head_seq;
            let encoded = encode_heads(&heads);
            meta.insert(META_HEADS_KEY, encoded.as_slice())
                .map_err(map_storage)?;
            heads
        };
        tx.commit().map_err(map_storage)?;
        Ok(new_heads)
    }

    /// Appends an attested counter-inductive reopening strictly downstream of sealed history,
    /// requiring `escrow_deposit > 0`, `valid_ts <= tx_ts`, and an intact prior control seal.
    pub fn append_counter_inductive_reopen(
        &self,
        valid_ts: u64,
        tx_ts: u64,
        escrow_deposit: u64,
        prior_control_seq: u64,
        rationale: &[u8],
    ) -> Result<CounterInductiveReopen, CustodyError> {
        ensure_bitemporal(valid_ts, tx_ts)?;
        if escrow_deposit == 0 {
            return Err(CustodyError::ZeroEscrow);
        }
        let tx = self.begin_durable_write()?;
        let rec = {
            let mut meta = tx.open_table(META).map_err(map_storage)?;
            let mut heads = {
                let raw = meta
                    .get(META_HEADS_KEY)
                    .map_err(map_storage)?
                    .ok_or(CustodyError::MissingRecord("custody heads missing"))?;
                decode_heads(raw.value())?
            };

            let control_tape = tx.open_table(CONTROL_TAPE).map_err(map_storage)?;
            if prior_control_seq == 0 {
                return Err(CustodyError::MissingRecord(
                    "reopen must cite an existing control seal >= 1",
                ));
            }
            let prior_raw = control_tape
                .get(prior_control_seq)
                .map_err(map_storage)?
                .ok_or(CustodyError::MissingRecord("prior control seal absent"))?;
            let prior_control =
                decode_tape_record(DOMAIN_CONTROL, prior_control_seq, prior_raw.value())?;

            let mut reopen_log = tx.open_table(REOPEN_LOG).map_err(map_storage)?;
            let prev_seq = heads.reopen_total;
            let prev_hash = if prev_seq == 0 {
                [0_u8; 32]
            } else {
                let raw_prev = reopen_log
                    .get(prev_seq)
                    .map_err(map_storage)?
                    .ok_or(CustodyError::IntegrityCorruption("prior reopen entry absent"))?;
                decode_reopen_record(prev_seq, raw_prev.value())?.seal_hash
            };

            let new_seq = heads.reopen_total.checked_add(1).ok_or(CustodyError::Overflow)?;
            heads.reopen_total = new_seq;

            let rec = build_reopen_record(
                new_seq,
                valid_ts,
                tx_ts,
                escrow_deposit,
                prior_control_seq,
                prior_control.seal_hash,
                prev_seq,
                prev_hash,
                rationale,
            )?;
            let bytes = encode_reopen_record(&rec);
            reopen_log
                .insert(new_seq, bytes.as_slice())
                .map_err(map_storage)?;
            let heads_bytes = encode_heads(&heads);
            meta.insert(META_HEADS_KEY, heads_bytes.as_slice())
                .map_err(map_storage)?;
            rec
        };
        tx.commit().map_err(map_storage)?;
        Ok(rec)
    }

    /// Reads and `BLAKE3`-verifies reopening entry `seq`.
    pub fn read_reopen(&self, seq: u64) -> Result<CounterInductiveReopen, CustodyError> {
        let tx = self.db.begin_read().map_err(map_storage)?;
        let table = tx.open_table(REOPEN_LOG).map_err(map_storage)?;
        let guard = table
            .get(seq)
            .map_err(map_storage)?
            .ok_or(CustodyError::MissingRecord("reopen entry absent"))?;
        decode_reopen_record(seq, guard.value())
    }

    /// Walks and cryptographically verifies every record across `CONTROL_TAPE`,
    /// `CONTINGENCY_TAPE`, `REOPEN_LOG`, `SPENT_UCANS`, `REFUSED_R2_TARGETS`, and `R0_SNAPSHOTS`.
    pub fn verify_full_chain(&self) -> Result<CustodyHeads, CustodyError> {
        let tx = self.db.begin_read().map_err(map_storage)?;
        let meta = tx.open_table(META).map_err(map_storage)?;
        let raw_heads = meta
            .get(META_HEADS_KEY)
            .map_err(map_storage)?
            .ok_or(CustodyError::MissingRecord("custody heads missing"))?;
        let heads = decode_heads(raw_heads.value())?;

        let control_tape = tx.open_table(CONTROL_TAPE).map_err(map_storage)?;
        let mut control_hashes = Vec::with_capacity(
            usize::try_from(heads.control_total_sealed).unwrap_or(0),
        );
        for seq in 1..=heads.control_total_sealed {
            let guard = control_tape
                .get(seq)
                .map_err(map_storage)?
                .ok_or(CustodyError::IntegrityCorruption("missing control seal in chain"))?;
            let rec = decode_tape_record(DOMAIN_CONTROL, seq, guard.value())?;
            verify_prev_link(rec.prev_seq, rec.prev_hash, seq, &control_hashes)?;
            control_hashes.push(rec.seal_hash);
        }

        let contingency_tape = tx.open_table(CONTINGENCY_TAPE).map_err(map_storage)?;
        let mut contingency_hashes = Vec::with_capacity(
            usize::try_from(heads.contingency_total_sealed).unwrap_or(0),
        );
        for seq in 1..=heads.contingency_total_sealed {
            let guard = contingency_tape
                .get(seq)
                .map_err(map_storage)?
                .ok_or(CustodyError::IntegrityCorruption("missing contingency seal in chain"))?;
            let rec = decode_tape_record(DOMAIN_CONTINGENCY, seq, guard.value())?;
            verify_prev_link(rec.prev_seq, rec.prev_hash, seq, &contingency_hashes)?;
            contingency_hashes.push(rec.seal_hash);
        }

        let reopen_log = tx.open_table(REOPEN_LOG).map_err(map_storage)?;
        let mut prev_reopen_hash = [0_u8; 32];
        for seq in 1..=heads.reopen_total {
            let guard = reopen_log
                .get(seq)
                .map_err(map_storage)?
                .ok_or(CustodyError::IntegrityCorruption("missing reopen entry in chain"))?;
            let rec = decode_reopen_record(seq, guard.value())?;
            if rec.prev_seq != seq - 1 || rec.prev_hash != prev_reopen_hash {
                return Err(CustodyError::IntegrityCorruption(
                    "reopen chain backlink mismatch",
                ));
            }
            let idx = usize::try_from(rec.prior_control_seq - 1).map_err(|_| CustodyError::Overflow)?;
            let expected_ctrl = control_hashes.get(idx).ok_or(CustodyError::IntegrityCorruption(
                "reopen prior_control_seq out of range",
            ))?;
            if *expected_ctrl != rec.prior_control_hash {
                return Err(CustodyError::IntegrityCorruption(
                    "reopen prior_control_hash mismatch",
                ));
            }
            prev_reopen_hash = rec.seal_hash;
        }

        let spent_ucans = tx.open_table(SPENT_UCANS).map_err(map_storage)?;
        for entry in spent_ucans.iter().map_err(map_storage)? {
            let (k, v) = entry.map_err(map_storage)?;
            verify_tag(DOMAIN_UCAN, k.value(), v.value())?;
        }

        let refused_r2 = tx.open_table(REFUSED_R2_TARGETS).map_err(map_storage)?;
        for entry in refused_r2.iter().map_err(map_storage)? {
            let (k, v) = entry.map_err(map_storage)?;
            verify_tag(DOMAIN_R2_REFUSAL, k.value(), v.value())?;
        }

        let snapshots = tx.open_table(R0_SNAPSHOTS).map_err(map_storage)?;
        for entry in snapshots.iter().map_err(map_storage)? {
            let (k, v) = entry.map_err(map_storage)?;
            let _ = decode_snapshot(k.value(), v.value())?;
        }

        let deleg_seqs = tx.open_table(DELEGATION_SEQS).map_err(map_storage)?;
        for entry in deleg_seqs.iter().map_err(map_storage)? {
            let (k, v) = entry.map_err(map_storage)?;
            let _ = verify_deleg_seq(k.value(), v.value())?;
        }

        Ok(heads)
    }
}

fn ensure_bitemporal(valid_ts: u64, tx_ts: u64) -> Result<(), CustodyError> {
    if valid_ts > tx_ts {
        Err(CustodyError::InvalidBitemporalOrder { valid_ts, tx_ts })
    } else {
        Ok(())
    }
}

fn verify_prev_link(
    prev_seq: u64,
    prev_hash: [u8; 32],
    current_seq: u64,
    hashes: &[[u8; 32]],
) -> Result<(), CustodyError> {
    if prev_seq == 0 {
        if prev_hash != [0_u8; 32] {
            return Err(CustodyError::IntegrityCorruption("non-zero genesis prev_hash"));
        }
        return Ok(());
    }
    if prev_seq >= current_seq {
        return Err(CustodyError::IntegrityCorruption("non-monotonic prev_seq backlink"));
    }
    let idx = usize::try_from(prev_seq - 1).map_err(|_| CustodyError::Overflow)?;
    let expected = hashes.get(idx).ok_or(CustodyError::IntegrityCorruption(
        "prev_seq target missing from tape",
    ))?;
    if *expected != prev_hash {
        return Err(CustodyError::IntegrityCorruption("tape prev_hash mismatch"));
    }
    Ok(())
}

fn compute_key_tag(domain: &[u8], key: &[u8; 32]) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new();
    hasher.update(domain);
    hasher.update(key);
    *hasher.finalize().as_bytes()
}

fn verify_tag(domain: &[u8], key: &[u8; 32], stored: &[u8; 32]) -> Result<(), CustodyError> {
    let expected = blake3::Hash::from(compute_key_tag(domain, key));
    if expected == blake3::Hash::from(*stored) {
        Ok(())
    } else {
        Err(CustodyError::IntegrityCorruption("key authentication tag mismatch"))
    }
}

fn encode_deleg_seq(sovereign_id: &[u8; 32], seq: u64) -> [u8; 40] {
    let mut hasher = blake3::Hasher::new();
    hasher.update(DOMAIN_DELEG_SEQ);
    hasher.update(sovereign_id);
    hasher.update(&seq.to_le_bytes());
    let tag = *hasher.finalize().as_bytes();

    let mut out = [0_u8; 40];
    out[0..8].copy_from_slice(&seq.to_le_bytes());
    out[8..40].copy_from_slice(&tag);
    out
}

fn verify_deleg_seq(sovereign_id: &[u8; 32], stored: &[u8; 40]) -> Result<u64, CustodyError> {
    let seq = read_u64(&stored[0..8])?;
    let expected = encode_deleg_seq(sovereign_id, seq);
    let stored_tag: [u8; 32] = stored[8..40]
        .try_into()
        .map_err(|_| CustodyError::IntegrityCorruption("delegation tag width"))?;
    let expected_tag: [u8; 32] = expected[8..40]
        .try_into()
        .map_err(|_| CustodyError::IntegrityCorruption("delegation tag width"))?;
    if blake3::Hash::from(stored_tag) == blake3::Hash::from(expected_tag) {
        Ok(seq)
    } else {
        Err(CustodyError::IntegrityCorruption("delegation_seq BLAKE3 mismatch"))
    }
}

fn compute_tape_hash(
    domain: &[u8],
    seq: u64,
    valid_ts: u64,
    tx_ts: u64,
    prev_seq: u64,
    prev_hash: &[u8; 32],
    payload: &[u8],
) -> Result<[u8; 32], CustodyError> {
    let payload_len = u32::try_from(payload.len()).map_err(|_| CustodyError::Overflow)?;
    let mut hasher = blake3::Hasher::new();
    hasher.update(domain);
    hasher.update(&seq.to_le_bytes());
    hasher.update(&valid_ts.to_le_bytes());
    hasher.update(&tx_ts.to_le_bytes());
    hasher.update(&prev_seq.to_le_bytes());
    hasher.update(prev_hash);
    hasher.update(&payload_len.to_le_bytes());
    hasher.update(payload);
    Ok(*hasher.finalize().as_bytes())
}

fn build_tape_record(
    domain: &[u8],
    seq: u64,
    valid_ts: u64,
    tx_ts: u64,
    prev_seq: u64,
    prev_hash: [u8; 32],
    payload: &[u8],
) -> Result<TapeRecord, CustodyError> {
    let seal_hash = compute_tape_hash(domain, seq, valid_ts, tx_ts, prev_seq, &prev_hash, payload)?;
    Ok(TapeRecord {
        seq,
        valid_ts,
        tx_ts,
        prev_seq,
        prev_hash,
        payload: payload.to_vec(),
        seal_hash,
    })
}

fn encode_tape_record(rec: &TapeRecord) -> Vec<u8> {
    let payload_len = u32::try_from(rec.payload.len()).unwrap_or(u32::MAX);
    let mut out = Vec::with_capacity(68 + rec.payload.len() + 32);
    out.extend_from_slice(&rec.seq.to_le_bytes());
    out.extend_from_slice(&rec.valid_ts.to_le_bytes());
    out.extend_from_slice(&rec.tx_ts.to_le_bytes());
    out.extend_from_slice(&rec.prev_seq.to_le_bytes());
    out.extend_from_slice(&rec.prev_hash);
    out.extend_from_slice(&payload_len.to_le_bytes());
    out.extend_from_slice(&rec.payload);
    out.extend_from_slice(&rec.seal_hash);
    out
}

fn decode_tape_record(
    domain: &[u8],
    expected_seq: u64,
    bytes: &[u8],
) -> Result<TapeRecord, CustodyError> {
    if bytes.len() < 100 {
        return Err(CustodyError::IntegrityCorruption("truncated tape record"));
    }
    let seq = read_u64(&bytes[0..8])?;
    let valid_ts = read_u64(&bytes[8..16])?;
    let tx_ts = read_u64(&bytes[16..24])?;
    let prev_seq = read_u64(&bytes[24..32])?;
    let prev_hash: [u8; 32] = bytes[32..64]
        .try_into()
        .map_err(|_| CustodyError::IntegrityCorruption("invalid prev_hash width"))?;
    let payload_len = usize::try_from(read_u32(&bytes[64..68])?)
        .map_err(|_| CustodyError::Overflow)?;
    if bytes.len() != 68 + payload_len + 32 {
        return Err(CustodyError::IntegrityCorruption("tape record length mismatch"));
    }
    if seq != expected_seq {
        return Err(CustodyError::IntegrityCorruption("tape sequence key mismatch"));
    }
    ensure_bitemporal(valid_ts, tx_ts)
        .map_err(|_| CustodyError::IntegrityCorruption("bitemporal invariant broken on disk"))?;

    let payload = &bytes[68..68 + payload_len];
    let stored_hash: [u8; 32] = bytes[68 + payload_len..100 + payload_len]
        .try_into()
        .map_err(|_| CustodyError::IntegrityCorruption("invalid seal_hash width"))?;
    let expected_hash =
        compute_tape_hash(domain, seq, valid_ts, tx_ts, prev_seq, &prev_hash, payload)?;
    if blake3::Hash::from(stored_hash) != blake3::Hash::from(expected_hash) {
        return Err(CustodyError::IntegrityCorruption("tape BLAKE3 seal mismatch"));
    }
    Ok(TapeRecord {
        seq,
        valid_ts,
        tx_ts,
        prev_seq,
        prev_hash,
        payload: payload.to_vec(),
        seal_hash: stored_hash,
    })
}

#[allow(clippy::too_many_arguments)]
fn compute_reopen_hash(
    seq: u64,
    valid_ts: u64,
    tx_ts: u64,
    escrow_deposit: u64,
    prior_control_seq: u64,
    prior_control_hash: &[u8; 32],
    prev_seq: u64,
    prev_hash: &[u8; 32],
    rationale: &[u8],
) -> Result<[u8; 32], CustodyError> {
    let r_len = u32::try_from(rationale.len()).map_err(|_| CustodyError::Overflow)?;
    let mut hasher = blake3::Hasher::new();
    hasher.update(DOMAIN_REOPEN);
    hasher.update(&seq.to_le_bytes());
    hasher.update(&valid_ts.to_le_bytes());
    hasher.update(&tx_ts.to_le_bytes());
    hasher.update(&escrow_deposit.to_le_bytes());
    hasher.update(&prior_control_seq.to_le_bytes());
    hasher.update(prior_control_hash);
    hasher.update(&prev_seq.to_le_bytes());
    hasher.update(prev_hash);
    hasher.update(&r_len.to_le_bytes());
    hasher.update(rationale);
    Ok(*hasher.finalize().as_bytes())
}

#[allow(clippy::too_many_arguments)]
fn build_reopen_record(
    seq: u64,
    valid_ts: u64,
    tx_ts: u64,
    escrow_deposit: u64,
    prior_control_seq: u64,
    prior_control_hash: [u8; 32],
    prev_seq: u64,
    prev_hash: [u8; 32],
    rationale: &[u8],
) -> Result<CounterInductiveReopen, CustodyError> {
    let seal_hash = compute_reopen_hash(
        seq,
        valid_ts,
        tx_ts,
        escrow_deposit,
        prior_control_seq,
        &prior_control_hash,
        prev_seq,
        &prev_hash,
        rationale,
    )?;
    Ok(CounterInductiveReopen {
        seq,
        valid_ts,
        tx_ts,
        escrow_deposit,
        prior_control_seq,
        prior_control_hash,
        prev_seq,
        prev_hash,
        rationale: rationale.to_vec(),
        seal_hash,
    })
}

fn encode_reopen_record(rec: &CounterInductiveReopen) -> Vec<u8> {
    let r_len = u32::try_from(rec.rationale.len()).unwrap_or(u32::MAX);
    let mut out = Vec::with_capacity(116 + rec.rationale.len() + 32);
    out.extend_from_slice(&rec.seq.to_le_bytes());
    out.extend_from_slice(&rec.valid_ts.to_le_bytes());
    out.extend_from_slice(&rec.tx_ts.to_le_bytes());
    out.extend_from_slice(&rec.escrow_deposit.to_le_bytes());
    out.extend_from_slice(&rec.prior_control_seq.to_le_bytes());
    out.extend_from_slice(&rec.prior_control_hash);
    out.extend_from_slice(&rec.prev_seq.to_le_bytes());
    out.extend_from_slice(&rec.prev_hash);
    out.extend_from_slice(&r_len.to_le_bytes());
    out.extend_from_slice(&rec.rationale);
    out.extend_from_slice(&rec.seal_hash);
    out
}

fn decode_reopen_record(
    expected_seq: u64,
    bytes: &[u8],
) -> Result<CounterInductiveReopen, CustodyError> {
    if bytes.len() < 148 {
        return Err(CustodyError::IntegrityCorruption("truncated reopen record"));
    }
    let seq = read_u64(&bytes[0..8])?;
    let valid_ts = read_u64(&bytes[8..16])?;
    let tx_ts = read_u64(&bytes[16..24])?;
    let escrow_deposit = read_u64(&bytes[24..32])?;
    let prior_control_seq = read_u64(&bytes[32..40])?;
    let prior_control_hash: [u8; 32] = bytes[40..72]
        .try_into()
        .map_err(|_| CustodyError::IntegrityCorruption("invalid prior_control_hash width"))?;
    let prev_seq = read_u64(&bytes[72..80])?;
    let prev_hash: [u8; 32] = bytes[80..112]
        .try_into()
        .map_err(|_| CustodyError::IntegrityCorruption("invalid reopen prev_hash width"))?;
    let r_len = usize::try_from(read_u32(&bytes[112..116])?)
        .map_err(|_| CustodyError::Overflow)?;
    if bytes.len() != 116 + r_len + 32 {
        return Err(CustodyError::IntegrityCorruption("reopen record length mismatch"));
    }
    if seq != expected_seq || escrow_deposit == 0 || prior_control_seq == 0 {
        return Err(CustodyError::IntegrityCorruption("invalid reopen invariants on disk"));
    }
    ensure_bitemporal(valid_ts, tx_ts)
        .map_err(|_| CustodyError::IntegrityCorruption("reopen bitemporal invariant broken"))?;
    let rationale = &bytes[116..116 + r_len];
    let stored_hash: [u8; 32] = bytes[116 + r_len..148 + r_len]
        .try_into()
        .map_err(|_| CustodyError::IntegrityCorruption("invalid reopen seal_hash width"))?;
    let expected_hash = compute_reopen_hash(
        seq,
        valid_ts,
        tx_ts,
        escrow_deposit,
        prior_control_seq,
        &prior_control_hash,
        prev_seq,
        &prev_hash,
        rationale,
    )?;
    if blake3::Hash::from(stored_hash) != blake3::Hash::from(expected_hash) {
        return Err(CustodyError::IntegrityCorruption("reopen BLAKE3 seal mismatch"));
    }
    Ok(CounterInductiveReopen {
        seq,
        valid_ts,
        tx_ts,
        escrow_deposit,
        prior_control_seq,
        prior_control_hash,
        prev_seq,
        prev_hash,
        rationale: rationale.to_vec(),
        seal_hash: stored_hash,
    })
}

fn encode_heads(heads: &CustodyHeads) -> [u8; 72] {
    let mut hasher = blake3::Hasher::new();
    hasher.update(DOMAIN_META);
    hasher.update(&heads.control_active_seq.to_le_bytes());
    hasher.update(&heads.control_total_sealed.to_le_bytes());
    hasher.update(&heads.contingency_active_seq.to_le_bytes());
    hasher.update(&heads.contingency_total_sealed.to_le_bytes());
    hasher.update(&heads.reopen_total.to_le_bytes());
    let tag = *hasher.finalize().as_bytes();

    let mut out = [0_u8; 72];
    out[0..8].copy_from_slice(&heads.control_active_seq.to_le_bytes());
    out[8..16].copy_from_slice(&heads.control_total_sealed.to_le_bytes());
    out[16..24].copy_from_slice(&heads.contingency_active_seq.to_le_bytes());
    out[24..32].copy_from_slice(&heads.contingency_total_sealed.to_le_bytes());
    out[32..40].copy_from_slice(&heads.reopen_total.to_le_bytes());
    out[40..72].copy_from_slice(&tag);
    out
}

fn decode_heads(bytes: &[u8]) -> Result<CustodyHeads, CustodyError> {
    if bytes.len() != 72 {
        return Err(CustodyError::IntegrityCorruption("invalid metadata heads length"));
    }
    let heads = CustodyHeads {
        control_active_seq: read_u64(&bytes[0..8])?,
        control_total_sealed: read_u64(&bytes[8..16])?,
        contingency_active_seq: read_u64(&bytes[16..24])?,
        contingency_total_sealed: read_u64(&bytes[24..32])?,
        reopen_total: read_u64(&bytes[32..40])?,
    };
    let stored: [u8; 32] = bytes[40..72]
        .try_into()
        .map_err(|_| CustodyError::IntegrityCorruption("invalid heads digest"))?;
    let expected = encode_heads(&heads);
    if blake3::Hash::from(stored)
        != blake3::Hash::from(<[u8; 32]>::try_from(&expected[40..72]).unwrap_or([0; 32]))
    {
        return Err(CustodyError::IntegrityCorruption("metadata heads BLAKE3 mismatch"));
    }
    Ok(heads)
}

fn encode_snapshot(snap: &SnapshotRecord) -> [u8; 120] {
    let mut hasher = blake3::Hasher::new();
    hasher.update(DOMAIN_SNAPSHOT);
    hasher.update(&snap.snapshot_id.to_le_bytes());
    hasher.update(&snap.control_head_seq.to_le_bytes());
    hasher.update(&snap.control_head_hash);
    hasher.update(&snap.contingency_head_seq.to_le_bytes());
    hasher.update(&snap.contingency_head_hash);
    let tag = *hasher.finalize().as_bytes();

    let mut out = [0_u8; 120];
    out[0..8].copy_from_slice(&snap.snapshot_id.to_le_bytes());
    out[8..16].copy_from_slice(&snap.control_head_seq.to_le_bytes());
    out[16..48].copy_from_slice(&snap.control_head_hash);
    out[48..56].copy_from_slice(&snap.contingency_head_seq.to_le_bytes());
    out[56..88].copy_from_slice(&snap.contingency_head_hash);
    out[88..120].copy_from_slice(&tag);
    out
}

fn decode_snapshot(expected_id: u64, bytes: &[u8]) -> Result<SnapshotRecord, CustodyError> {
    if bytes.len() != 120 {
        return Err(CustodyError::IntegrityCorruption("invalid snapshot length"));
    }
    let snapshot_id = read_u64(&bytes[0..8])?;
    if snapshot_id != expected_id {
        return Err(CustodyError::IntegrityCorruption("snapshot key mismatch"));
    }
    let snap = SnapshotRecord {
        snapshot_id,
        control_head_seq: read_u64(&bytes[8..16])?,
        control_head_hash: bytes[16..48]
            .try_into()
            .map_err(|_| CustodyError::IntegrityCorruption("snapshot hash width"))?,
        contingency_head_seq: read_u64(&bytes[48..56])?,
        contingency_head_hash: bytes[56..88]
            .try_into()
            .map_err(|_| CustodyError::IntegrityCorruption("snapshot hash width"))?,
    };
    let stored: [u8; 32] = bytes[88..120]
        .try_into()
        .map_err(|_| CustodyError::IntegrityCorruption("snapshot tag width"))?;
    let expected = encode_snapshot(&snap);
    if blake3::Hash::from(stored)
        != blake3::Hash::from(<[u8; 32]>::try_from(&expected[88..120]).unwrap_or([0; 32]))
    {
        return Err(CustodyError::IntegrityCorruption("snapshot BLAKE3 mismatch"));
    }
    Ok(snap)
}

fn read_u64(slice: &[u8]) -> Result<u64, CustodyError> {
    let arr: [u8; 8] = slice
        .try_into()
        .map_err(|_| CustodyError::IntegrityCorruption("malformed u64 field"))?;
    Ok(u64::from_le_bytes(arr))
}

fn read_u32(slice: &[u8]) -> Result<u32, CustodyError> {
    let arr: [u8; 4] = slice
        .try_into()
        .map_err(|_| CustodyError::IntegrityCorruption("malformed u32 field"))?;
    Ok(u32::from_le_bytes(arr))
}
