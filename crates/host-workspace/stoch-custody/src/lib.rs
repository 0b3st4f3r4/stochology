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

mod codec;
mod error;
mod tables;

use std::path::Path;

use redb::{Database, Durability, ReadableTable};

pub use crate::codec::{CounterInductiveReopen, CustodyHeads, TapeRecord};
pub use crate::error::CustodyError;
pub use crate::tables::{
    CONTINGENCY_TAPE, CONTROL_TAPE, DELEGATION_SEQS, META, R0_SNAPSHOTS, REFUSED_R2_TARGETS,
    REOPEN_LOG, SPENT_UCANS,
};

use crate::codec::{
    build_reopen_record, build_tape_record, compute_key_tag, decode_heads, decode_reopen_record,
    decode_snapshot, decode_tape_record, encode_deleg_seq, encode_heads, encode_reopen_record,
    encode_snapshot, encode_tape_record, ensure_bitemporal, verify_deleg_seq, verify_prev_link,
    verify_tag, SnapshotRecord,
};
use crate::error::map_storage;
use crate::tables::{
    DOMAIN_CONTINGENCY, DOMAIN_CONTROL, DOMAIN_R2_REFUSAL, DOMAIN_UCAN, META_HEADS_KEY,
};

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

            let record =
                build_tape_record(domain, new_seq, valid_ts, tx_ts, prev_seq, prev_hash, payload)?;
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
            let idx = usize::try_from(rec.prior_control_seq - 1)
                .map_err(|_| CustodyError::Overflow)?;
            let expected_ctrl = control_hashes.get(idx).ok_or(
                CustodyError::IntegrityCorruption("reopen prior_control_seq out of range"),
            )?;
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
