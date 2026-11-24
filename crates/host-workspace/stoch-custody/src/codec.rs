//! Canonical record structs and keyed HMAC-SHA256 serialization codecs for `stoch-custody`.

use crate::error::CustodyError;
use crate::hmac_sha256::{ct_eq, hmac_sha256};
use crate::tables::{DOMAIN_DELEG_SEQ, DOMAIN_META, DOMAIN_REOPEN, DOMAIN_SNAPSHOT};

/// Decoded and HMAC-SHA256-verified entry on either [`crate::CONTROL_TAPE`] or [`crate::CONTINGENCY_TAPE`].
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
    /// Domain-separated `RFC 2104 HMAC-SHA256` tag sealing the record fields and payload.
    pub seal_hash: [u8; 32],
}

/// Decoded and `RFC 2104 HMAC-SHA256`-verified entry from [`crate::REOPEN_LOG`].
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
    /// Anchored sequence number on [`crate::CONTROL_TAPE`] referenced by this reopening.
    pub prior_control_seq: u64,
    /// Verified `RFC 2104 HMAC-SHA256` seal tag of the anchored [`crate::CONTROL_TAPE`] entry.
    pub prior_control_hash: [u8; 32],
    /// Predecessor reopening sequence (`0` if first).
    pub prev_seq: u64,
    /// Predecessor reopening seal digest (`[0; 32]` if `prev_seq == 0`).
    pub prev_hash: [u8; 32],
    /// Documented rationale or protocol expansion note.
    pub rationale: Vec<u8>,
    /// Domain-separated `RFC 2104 HMAC-SHA256` tag sealing all fields of this reopening record.
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
pub(crate) struct SnapshotRecord {
    pub(crate) snapshot_id: u64,
    pub(crate) control_head_seq: u64,
    pub(crate) control_head_hash: [u8; 32],
    pub(crate) contingency_head_seq: u64,
    pub(crate) contingency_head_hash: [u8; 32],
}

pub(crate) fn ensure_bitemporal(valid_ts: u64, tx_ts: u64) -> Result<(), CustodyError> {
    if valid_ts > tx_ts {
        Err(CustodyError::InvalidBitemporalOrder { valid_ts, tx_ts })
    } else {
        Ok(())
    }
}

pub(crate) fn verify_prev_link(
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

pub(crate) fn compute_key_tag(
    mac_key: &[u8; 32],
    domain: &[u8],
    key: &[u8; 32],
) -> Result<[u8; 32], CustodyError> {
    hmac_sha256(mac_key, &[domain, key])
}

pub(crate) fn verify_tag(
    mac_key: &[u8; 32],
    domain: &[u8],
    key: &[u8; 32],
    stored: &[u8; 32],
) -> Result<(), CustodyError> {
    let expected = compute_key_tag(mac_key, domain, key)?;
    if ct_eq(&expected, stored) {
        Ok(())
    } else {
        Err(CustodyError::IntegrityCorruption("key authentication tag mismatch"))
    }
}

pub(crate) fn encode_deleg_seq(
    mac_key: &[u8; 32],
    sovereign_id: &[u8; 32],
    seq: u64,
) -> Result<[u8; 40], CustodyError> {
    let seq_bytes = seq.to_le_bytes();
    let tag = hmac_sha256(mac_key, &[DOMAIN_DELEG_SEQ, sovereign_id, &seq_bytes])?;

    let mut out = [0_u8; 40];
    out[0..8].copy_from_slice(&seq_bytes);
    out[8..40].copy_from_slice(&tag);
    Ok(out)
}

pub(crate) fn verify_deleg_seq(
    mac_key: &[u8; 32],
    sovereign_id: &[u8; 32],
    stored: &[u8; 40],
) -> Result<u64, CustodyError> {
    let seq = read_u64(&stored[0..8])?;
    let expected = encode_deleg_seq(mac_key, sovereign_id, seq)?;
    let stored_tag: [u8; 32] = stored[8..40]
        .try_into()
        .map_err(|_| CustodyError::IntegrityCorruption("delegation tag width"))?;
    let expected_tag: [u8; 32] = expected[8..40]
        .try_into()
        .map_err(|_| CustodyError::IntegrityCorruption("delegation tag width"))?;
    if ct_eq(&stored_tag, &expected_tag) {
        Ok(seq)
    } else {
        Err(CustodyError::IntegrityCorruption("delegation_seq HMAC mismatch"))
    }
}

#[allow(clippy::too_many_arguments)]
fn compute_tape_hash(
    mac_key: &[u8; 32],
    domain: &[u8],
    seq: u64,
    valid_ts: u64,
    tx_ts: u64,
    prev_seq: u64,
    prev_hash: &[u8; 32],
    payload: &[u8],
) -> Result<[u8; 32], CustodyError> {
    let payload_len = u32::try_from(payload.len()).map_err(|_| CustodyError::Overflow)?;
    hmac_sha256(
        mac_key,
        &[
            domain,
            &seq.to_le_bytes(),
            &valid_ts.to_le_bytes(),
            &tx_ts.to_le_bytes(),
            &prev_seq.to_le_bytes(),
            prev_hash,
            &payload_len.to_le_bytes(),
            payload,
        ],
    )
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn build_tape_record(
    mac_key: &[u8; 32],
    domain: &[u8],
    seq: u64,
    valid_ts: u64,
    tx_ts: u64,
    prev_seq: u64,
    prev_hash: [u8; 32],
    payload: &[u8],
) -> Result<TapeRecord, CustodyError> {
    let seal_hash =
        compute_tape_hash(mac_key, domain, seq, valid_ts, tx_ts, prev_seq, &prev_hash, payload)?;
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

pub(crate) fn encode_tape_record(rec: &TapeRecord) -> Vec<u8> {
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

pub(crate) fn decode_tape_record(
    mac_key: &[u8; 32],
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
        compute_tape_hash(mac_key, domain, seq, valid_ts, tx_ts, prev_seq, &prev_hash, payload)?;
    if !ct_eq(&stored_hash, &expected_hash) {
        return Err(CustodyError::IntegrityCorruption("tape HMAC seal mismatch"));
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
    mac_key: &[u8; 32],
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
    hmac_sha256(
        mac_key,
        &[
            DOMAIN_REOPEN,
            &seq.to_le_bytes(),
            &valid_ts.to_le_bytes(),
            &tx_ts.to_le_bytes(),
            &escrow_deposit.to_le_bytes(),
            &prior_control_seq.to_le_bytes(),
            prior_control_hash,
            &prev_seq.to_le_bytes(),
            prev_hash,
            &r_len.to_le_bytes(),
            rationale,
        ],
    )
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn build_reopen_record(
    mac_key: &[u8; 32],
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
        mac_key,
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

pub(crate) fn encode_reopen_record(rec: &CounterInductiveReopen) -> Vec<u8> {
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

pub(crate) fn decode_reopen_record(
    mac_key: &[u8; 32],
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
        mac_key,
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
    if !ct_eq(&stored_hash, &expected_hash) {
        return Err(CustodyError::IntegrityCorruption("reopen HMAC seal mismatch"));
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

pub(crate) fn encode_heads(
    mac_key: &[u8; 32],
    heads: &CustodyHeads,
) -> Result<[u8; 72], CustodyError> {
    let ca = heads.control_active_seq.to_le_bytes();
    let ct = heads.control_total_sealed.to_le_bytes();
    let wa = heads.contingency_active_seq.to_le_bytes();
    let wt = heads.contingency_total_sealed.to_le_bytes();
    let rt = heads.reopen_total.to_le_bytes();
    let tag = hmac_sha256(mac_key, &[DOMAIN_META, &ca, &ct, &wa, &wt, &rt])?;

    let mut out = [0_u8; 72];
    out[0..8].copy_from_slice(&ca);
    out[8..16].copy_from_slice(&ct);
    out[16..24].copy_from_slice(&wa);
    out[24..32].copy_from_slice(&wt);
    out[32..40].copy_from_slice(&rt);
    out[40..72].copy_from_slice(&tag);
    Ok(out)
}

pub(crate) fn decode_heads(mac_key: &[u8; 32], bytes: &[u8]) -> Result<CustodyHeads, CustodyError> {
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
    let expected = encode_heads(mac_key, &heads)?;
    let expected_tag: [u8; 32] = expected[40..72]
        .try_into()
        .map_err(|_| CustodyError::IntegrityCorruption("invalid heads digest"))?;
    if !ct_eq(&stored, &expected_tag) {
        return Err(CustodyError::IntegrityCorruption("metadata heads HMAC mismatch"));
    }
    Ok(heads)
}

pub(crate) fn encode_snapshot(
    mac_key: &[u8; 32],
    snap: &SnapshotRecord,
) -> Result<[u8; 120], CustodyError> {
    let sid = snap.snapshot_id.to_le_bytes();
    let cs = snap.control_head_seq.to_le_bytes();
    let ws = snap.contingency_head_seq.to_le_bytes();
    let tag = hmac_sha256(
        mac_key,
        &[
            DOMAIN_SNAPSHOT,
            &sid,
            &cs,
            &snap.control_head_hash,
            &ws,
            &snap.contingency_head_hash,
        ],
    )?;

    let mut out = [0_u8; 120];
    out[0..8].copy_from_slice(&sid);
    out[8..16].copy_from_slice(&cs);
    out[16..48].copy_from_slice(&snap.control_head_hash);
    out[48..56].copy_from_slice(&ws);
    out[56..88].copy_from_slice(&snap.contingency_head_hash);
    out[88..120].copy_from_slice(&tag);
    Ok(out)
}

pub(crate) fn decode_snapshot(
    mac_key: &[u8; 32],
    expected_id: u64,
    bytes: &[u8],
) -> Result<SnapshotRecord, CustodyError> {
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
    let expected = encode_snapshot(mac_key, &snap)?;
    let expected_tag: [u8; 32] = expected[88..120]
        .try_into()
        .map_err(|_| CustodyError::IntegrityCorruption("snapshot tag width"))?;
    if !ct_eq(&stored, &expected_tag) {
        return Err(CustodyError::IntegrityCorruption("snapshot HMAC mismatch"));
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
