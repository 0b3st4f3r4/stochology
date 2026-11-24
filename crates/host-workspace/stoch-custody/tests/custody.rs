//! Integration tests for `stoch-custody`: 2PC persistence, keyed HMAC-SHA256 verification,
//! anti-replay under `rollback_r0`, and escrowed `CounterInductiveReopen`.

use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use redb::{Database, Durability, ReadableTable};
use stoch_custody::{CustodyError, CustodyLedger, CONTROL_TAPE};

static SEQ: AtomicU64 = AtomicU64::new(0);

struct TempDb {
    dir: PathBuf,
    db_path: PathBuf,
}

impl TempDb {
    fn new(label: &str) -> Self {
        let id = SEQ.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!(
            "stoch-custody-{label}-{}-{id}",
            std::process::id()
        ));
        fs::create_dir_all(&dir).expect("create test temp dir");
        let db_path = dir.join("custody.redb");
        Self { dir, db_path }
    }
}

impl Drop for TempDb {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.dir);
    }
}

#[test]
fn bitemporal_order_crash_reopen_and_chain_verification() {
    let tmp = TempDb::new("bitemporal");
    {
        let ledger = CustodyLedger::open_with_key(&tmp.db_path, [0x5A_u8; 32]).expect("open ledger");
        assert!(matches!(
            ledger.append_control(200, 100, b"backdated"),
            Err(CustodyError::InvalidBitemporalOrder { valid_ts: 200, tx_ts: 100 })
        ));
        let c1 = ledger.append_control(100, 110, b"telos-step-1").expect("c1");
        let c2 = ledger.append_control(120, 125, b"telos-step-2").expect("c2");
        assert_eq!(c2.prev_seq, c1.seq);
        assert_eq!(c2.prev_hash, c1.seal_hash);

        let w1 = ledger
            .append_contingency(120, 130, b"world-observation-1")
            .expect("w1");
        assert_eq!(w1.seq, 1);
    }

    // Reopen fresh instance simulating crash recovery / process restart.
    let reopened = CustodyLedger::open_with_key(&tmp.db_path, [0x5A_u8; 32]).expect("reopen");
    let heads = reopened.verify_full_chain().expect("full chain verified");
    assert_eq!(heads.control_total_sealed, 2);
    assert_eq!(heads.contingency_total_sealed, 1);
    let rec2 = reopened.read_control(2).expect("read control 2");
    assert_eq!(rec2.payload, b"telos-step-2");
}

#[test]
fn bit_rot_in_storage_is_caught_on_every_read_and_full_walk() {
    let tmp = TempDb::new("bitrot");
    {
        let ledger = CustodyLedger::open_with_key(&tmp.db_path, [0x5A_u8; 32]).expect("open");
        ledger.append_control(10, 10, b"critical-seal").expect("append");
    }

    // Flip one byte inside stored CONTROL_TAPE record 1 directly via raw redb write.
    {
        let raw_db = Database::open(&tmp.db_path).expect("raw redb open");
        let mut wtx = raw_db.begin_write().expect("begin write");
        wtx.set_durability(Durability::Immediate);
        {
            let mut tbl = wtx.open_table(CONTROL_TAPE).expect("open control");
            let mut corrupted = tbl
                .get(1)
                .expect("get")
                .map(|g| g.value().to_vec())
                .expect("record 1 exists");
            let last_payload_byte = corrupted.len() - 33;
            corrupted[last_payload_byte] ^= 0x01;
            tbl.insert(1, corrupted.as_slice()).expect("overwrite corrupted");
        }
        wtx.commit().expect("commit corrupted payload");
    }

    let ledger = CustodyLedger::open_with_key(&tmp.db_path, [0x5A_u8; 32]).expect("reopen ledger");
    assert!(matches!(
        ledger.read_control(1),
        Err(CustodyError::IntegrityCorruption(_))
    ));
    assert!(matches!(
        ledger.verify_full_chain(),
        Err(CustodyError::IntegrityCorruption(_))
    ));
}

#[test]
fn rollback_r0_preserves_spent_ucans_refused_r2_and_immutable_seals() {
    let tmp = TempDb::new("rollback");
    let ledger = CustodyLedger::open_with_key(&tmp.db_path, [0x5A_u8; 32]).expect("open");

    let c1 = ledger.append_control(10, 10, b"pre-snapshot-control").expect("c1");
    let _w1 = ledger
        .append_contingency(10, 12, b"pre-snapshot-world")
        .expect("w1");
    ledger.create_r0_snapshot(1).expect("snapshot 1");

    // Post-snapshot events: spend a linear UCAN, record peaceful refusal, append control 2.
    let ucan = [0xAA_u8; 32];
    let peer = [0xBB_u8; 32];
    ledger.consume_ucan(&ucan).expect("first consume");
    assert!(matches!(ledger.consume_ucan(&ucan), Err(CustodyError::ReplayRejected(_))));
    ledger.refuse_r2_target(&peer).expect("record refusal");
    let _c2 = ledger.append_control(20, 22, b"post-snapshot-control").expect("c2");

    // Rewind operational heads to snapshot 1 (`R0`).
    let rewound = ledger.rollback_r0(1).expect("rollback r0");
    assert_eq!(rewound.control_active_seq, 1);
    assert_eq!(rewound.control_total_sealed, 2);

    // Anti-replay MUST remain active across R0 rollback, and R2 refusal must persist.
    assert!(ledger.is_ucan_spent(&ucan).expect("spent query"));
    assert!(matches!(ledger.consume_ucan(&ucan), Err(CustodyError::ReplayRejected(_))));
    assert!(ledger.is_r2_refused(&peer).expect("r2 refused query"));

    // Subsequent control append forks cleanly off snapshot 1 (`c1`), preserving `c2` intact.
    let c3 = ledger
        .append_control(25, 30, b"post-rollback-branch")
        .expect("c3");
    assert_eq!(c3.seq, 3);
    assert_eq!(c3.prev_seq, c1.seq);
    assert_eq!(c3.prev_hash, c1.seal_hash);

    let verified = ledger.verify_full_chain().expect("full DAG intact");
    assert_eq!(verified.control_total_sealed, 3);
    assert_eq!(verified.control_active_seq, 3);
}

#[test]
fn counter_inductive_reopen_requires_positive_escrow_and_intact_prior_seal() {
    let tmp = TempDb::new("reopen");
    {
        let ledger = CustodyLedger::open_with_key(&tmp.db_path, [0x5A_u8; 32]).expect("open");
        let c1 = ledger.append_control(10, 15, b"initial-frame").expect("c1");

        assert_eq!(
            ledger.append_counter_inductive_reopen(20, 25, 0, c1.seq, b"free-reopen"),
            Err(CustodyError::ZeroEscrow)
        );
        assert!(matches!(
            ledger.append_counter_inductive_reopen(30, 25, 100, c1.seq, b"bad-time"),
            Err(CustodyError::InvalidBitemporalOrder { valid_ts: 30, tx_ts: 25 })
        ));

        let reopen = ledger
            .append_counter_inductive_reopen(20, 25, 500, c1.seq, b"add unobserved sensor class")
            .expect("valid reopen");
        assert_eq!(reopen.seq, 1);
        assert_eq!(reopen.escrow_deposit, 500);
        assert_eq!(reopen.prior_control_hash, c1.seal_hash);
    }

    let reopened = CustodyLedger::open_with_key(&tmp.db_path, [0x5A_u8; 32]).expect("reopen db");
    let heads = reopened.verify_full_chain().expect("verify full chain");
    assert_eq!(heads.reopen_total, 1);
    let entry = reopened.read_reopen(1).expect("read reopen 1");
    assert_eq!(entry.rationale, b"add unobserved sensor class");
}

#[test]
fn hot_unrefuse_and_monotonic_delegation_seq_survive_reopen_and_rollback() {
    let tmp = TempDb::new("deleg_unrefuse");
    let target = [0x42_u8; 32];
    let sovereign = [0x11_u8; 32];
    let ucan = [0x77_u8; 32];
    {
        let ledger = CustodyLedger::open_with_key(&tmp.db_path, [0x5A_u8; 32]).expect("open");
        let _ = ledger.append_control(10, 10, b"seed").expect("seed");
        ledger.create_r0_snapshot(1).expect("snap");
        ledger.consume_ucan(&ucan).expect("spend ucan");

        ledger.refuse_r2_target(&target).expect("refuse target");
        assert!(ledger.is_r2_refused(&target).expect("query refused"));
        assert!(ledger.unrefuse_r2_target(&target).expect("hot unrefuse"));
        assert!(!ledger.unrefuse_r2_target(&target).expect("idempotent second unrefuse"));
        assert!(!ledger.is_r2_refused(&target).expect("target cleared"));

        assert_eq!(ledger.delegation_seq(&sovereign).expect("seq 0"), 0);
        ledger.advance_delegation_seq(&sovereign, 1).expect("seq 1");
        assert!(matches!(
            ledger.advance_delegation_seq(&sovereign, 1),
            Err(CustodyError::ReplayRejected(_))
        ));
        ledger.advance_delegation_seq(&sovereign, 4).expect("seq 4");

        // R0 rollback preserves delegation_seq and SPENT_UCANS.
        let _ = ledger.rollback_r0(1).expect("rollback");
        assert_eq!(ledger.delegation_seq(&sovereign).expect("seq kept"), 4);
        assert!(ledger.is_ucan_spent(&ucan).expect("ucan kept"));
    }

    let reopened = CustodyLedger::open_with_key(&tmp.db_path, [0x5A_u8; 32]).expect("reopen");
    let _ = reopened.verify_full_chain().expect("full chain verified");
    assert_eq!(reopened.delegation_seq(&sovereign).expect("seq persisted"), 4);
    assert!(matches!(
        reopened.advance_delegation_seq(&sovereign, 3),
        Err(CustodyError::ReplayRejected(_))
    ));
}

#[test]
fn keyed_hmac_rejects_opening_or_reading_under_wrong_mac_key() {
    let tmp = TempDb::new("wrong_key");
    let key_a = [0xAA_u8; 32];
    let key_b = [0xBB_u8; 32];
    {
        let ledger = CustodyLedger::open_with_key(&tmp.db_path, key_a).expect("open with key A");
        let _ = ledger.append_control(10, 10, b"authenticated-seal").expect("append");
        assert_eq!(ledger.verify_full_chain().expect("chain ok").control_total_sealed, 1);
    }
    assert!(matches!(
        CustodyLedger::open_with_key(&tmp.db_path, key_b),
        Err(CustodyError::IntegrityCorruption(_))
    ));
    let reopened = CustodyLedger::open_with_key(&tmp.db_path, key_a).expect("reopen with key A");
    assert_eq!(reopened.read_control(1).expect("read").payload, b"authenticated-seal");
}
