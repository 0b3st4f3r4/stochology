//! Integration tests for `stoch-host` (`P2-B1`): FROST RFC 9591 DKG + single-use nonces,
//! 512-bitflip & truncation checks, HyParView 2PC, ForkLocalLineage, OCap 113B PoP, HSM FFI
//! strict verification + concurrent TSan check, SolarWatchdogEngine, LockFreeMetrics,
//! Level1Cue <= 10 bits/s, EmergencyLease/Contestation/Allostasis, and Regime III-B independence.

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;

use ed25519_dalek::{Signer, SigningKey};
use frost_ed25519::round2::SignatureShare;
use frost_ed25519::{Identifier, SigningPackage};
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha20Rng;
use stoch_core::{
    diagnose, Bounds, Capacity, ControlTape, Guards, HaltCause, Instrument, Params, Regime, Score,
    SevenReadings, State, StepReading, Thresholds, Trajectory, MAX_ABS, SCALE,
};
use stoch_custody::CustodyLedger;
use stoch_host::disclosure::{
    BindingAxisTag, CoddStatusTag, Level1Cue, Level2AuditDrilldown, RegimeTag,
};
use stoch_host::emergency::LocalEmergencyController;
use stoch_host::frost::{
    aggregate_with_independent_verification, run_dkg_3_rounds, verify_group_signature,
    CeremonyKey, SignedFraudProof, SigningOutcome, SingleUseNonceStore,
};
use stoch_host::governance::{AllostaticCareLedger, ContestationRecord, EmergencyLease};
use stoch_host::hsm::HsmSigner;
use stoch_host::hyparview::HyParViewOverlay;
use stoch_host::lineage::ForkLocalLineage;
use stoch_host::metrics::LockFreeMetrics;
use stoch_host::ocap::{
    OcapFirewall, SandboxCapabilitySet, SandboxIntent, TransportDelegationCert,
    TRANSPORT_DELEG_PAYLOAD_LEN,
};
use stoch_host::solar::{SolarReading, SolarState, SolarWatchdogConfig, SolarWatchdogEngine};
use stoch_host::{HostError, RetrospectiveAudit};
use stoch_math::{CoddValue, UNIT};

static TEST_SEQ: AtomicU64 = AtomicU64::new(0);

struct TempDb {
    dir: PathBuf,
    path: PathBuf,
}

impl TempDb {
    fn new(tag: &str) -> Self {
        let id = TEST_SEQ.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!(
            "stoch-host-{tag}-{}-{id}",
            std::process::id()
        ));
        fs::create_dir_all(&dir).expect("create test dir");
        let path = dir.join("custody.redb");
        Self { dir, path }
    }
}

impl Drop for TempDb {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.dir);
    }
}

fn id_from(u: u16) -> Identifier {
    Identifier::try_from(u).expect("valid non-zero id")
}

#[test]
fn frost_3_of_4_dkg_sign_cheater_timeout_rollback_and_512_bitflips() {
    let tmp = TempDb::new("frost");
    let ledger = CustodyLedger::open(&tmp.path).expect("open ledger");
    let _seed = ledger.append_control(10, 10, b"dkg-epoch-0").expect("seed");
    ledger.create_r0_snapshot(1).expect("snap 1");

    let mut rng = ChaCha20Rng::seed_from_u64(0x570C_4F95_9101_0001);
    let dkg = run_dkg_3_rounds(3, 4, &mut rng).expect("3-of-4 DKG succeeds");
    assert_eq!(dkg.key_packages.len(), 4);

    let mut nonces_1 = SingleUseNonceStore::new();
    let mut nonces_2 = SingleUseNonceStore::new();
    let mut nonces_3 = SingleUseNonceStore::new();
    let mut nonces_4 = SingleUseNonceStore::new();

    let ceremony_1 = CeremonyKey {
        assembly_id: [0x11; 32],
        ucan_cid: [0x22; 32],
        attempt: 1,
    };
    let msg = b"r2-governance-commit-state";

    // Round 1 commitments from {1, 2, 3}.
    let c1 = nonces_1
        .commit_round1(ceremony_1, &dkg.key_packages[&id_from(1)], &ledger, &mut rng)
        .expect("c1");
    let c2 = nonces_2
        .commit_round1(ceremony_1, &dkg.key_packages[&id_from(2)], &ledger, &mut rng)
        .expect("c2");
    let c3 = nonces_3
        .commit_round1(ceremony_1, &dkg.key_packages[&id_from(3)], &ledger, &mut rng)
        .expect("c3");
    let commitments = BTreeMap::from([(id_from(1), c1), (id_from(2), c2), (id_from(3), c3)]);
    let pkg1 = SigningPackage::new(commitments, msg);

    // Silent timeout when signer 3 drops before round-2 emission: NEVER emits fraud proof.
    let s1 = nonces_1
        .consume_and_sign_round2(ceremony_1, &pkg1, &dkg.key_packages[&id_from(1)], &ledger)
        .expect("s1");
    let s2 = nonces_2
        .consume_and_sign_round2(ceremony_1, &pkg1, &dkg.key_packages[&id_from(2)], &ledger)
        .expect("s2");
    let partial_shares = BTreeMap::from([(id_from(1), s1), (id_from(2), s2)]);
    let timeout_res = aggregate_with_independent_verification(
        ceremony_1,
        &pkg1,
        &partial_shares,
        &dkg.public_key_package,
    )
    .expect("timeout classification");
    assert_eq!(
        timeout_res,
        SigningOutcome::PeerTimeout {
            timed_out: vec![id_from(3)]
        }
    );

    // Anti-replay invariant: consuming the same ceremony nonce after rollback_r0 fails cleanly.
    let _ = ledger.rollback_r0(1).expect("rollback r0");
    assert!(matches!(
        nonces_1.commit_round1(ceremony_1, &dkg.key_packages[&id_from(1)], &ledger, &mut rng),
        Err(HostError::NonceReplayRejected(_))
    ));
    assert!(matches!(
        nonces_1.consume_and_sign_round2(
            ceremony_1,
            &pkg1,
            &dkg.key_packages[&id_from(1)],
            &ledger
        ),
        Err(HostError::NonceReplayRejected(_))
    ));

    // Attempt 2: signer 3 submits a tampered round-2 share -> self-verifying FrostRound2Cheater!
    let ceremony_2 = CeremonyKey { attempt: 2, ..ceremony_1 };
    let c1_a2 = nonces_1
        .commit_round1(ceremony_2, &dkg.key_packages[&id_from(1)], &ledger, &mut rng)
        .expect("c1_a2");
    let c2_a2 = nonces_2
        .commit_round1(ceremony_2, &dkg.key_packages[&id_from(2)], &ledger, &mut rng)
        .expect("c2_a2");
    let c3_a2 = nonces_3
        .commit_round1(ceremony_2, &dkg.key_packages[&id_from(3)], &ledger, &mut rng)
        .expect("c3_a2");
    let pkg2 = SigningPackage::new(
        BTreeMap::from([(id_from(1), c1_a2), (id_from(2), c2_a2), (id_from(3), c3_a2)]),
        msg,
    );
    let s1_a2 = nonces_1
        .consume_and_sign_round2(ceremony_2, &pkg2, &dkg.key_packages[&id_from(1)], &ledger)
        .expect("s1_a2");
    let s2_a2 = nonces_2
        .consume_and_sign_round2(ceremony_2, &pkg2, &dkg.key_packages[&id_from(2)], &ledger)
        .expect("s2_a2");
    let s3_real = nonces_3
        .consume_and_sign_round2(ceremony_2, &pkg2, &dkg.key_packages[&id_from(3)], &ledger)
        .expect("s3_real");
    let mut bad_bytes = s3_real.serialize();
    bad_bytes[0] ^= 0x01;
    let s3_bad = SignatureShare::deserialize(&bad_bytes).expect("deserializable tampered scalar");

    let cheater_res = aggregate_with_independent_verification(
        ceremony_2,
        &pkg2,
        &BTreeMap::from([(id_from(1), s1_a2), (id_from(2), s2_a2), (id_from(3), s3_bad)]),
        &dkg.public_key_package,
    )
    .expect("cheater outcome");
    assert!(matches!(
        cheater_res,
        SigningOutcome::CheaterDetected(SignedFraudProof::FrostRound2Cheater { culprit, .. })
            if culprit == id_from(3)
    ));

    // Attempt 3: replace culprit 3 with honest signer 4 -> valid aggregated Ed25519 signature.
    let ceremony_3 = CeremonyKey { attempt: 3, ..ceremony_1 };
    let c1_a3 = nonces_1
        .commit_round1(ceremony_3, &dkg.key_packages[&id_from(1)], &ledger, &mut rng)
        .expect("c1_a3");
    let c2_a3 = nonces_2
        .commit_round1(ceremony_3, &dkg.key_packages[&id_from(2)], &ledger, &mut rng)
        .expect("c2_a3");
    let c4_a3 = nonces_4
        .commit_round1(ceremony_3, &dkg.key_packages[&id_from(4)], &ledger, &mut rng)
        .expect("c4_a3");
    let pkg3 = SigningPackage::new(
        BTreeMap::from([(id_from(1), c1_a3), (id_from(2), c2_a3), (id_from(4), c4_a3)]),
        msg,
    );
    let s1_a3 = nonces_1
        .consume_and_sign_round2(ceremony_3, &pkg3, &dkg.key_packages[&id_from(1)], &ledger)
        .expect("s1_a3");
    let s2_a3 = nonces_2
        .consume_and_sign_round2(ceremony_3, &pkg3, &dkg.key_packages[&id_from(2)], &ledger)
        .expect("s2_a3");
    let s4_a3 = nonces_4
        .consume_and_sign_round2(ceremony_3, &pkg3, &dkg.key_packages[&id_from(4)], &ledger)
        .expect("s4_a3");

    let sig = match aggregate_with_independent_verification(
        ceremony_3,
        &pkg3,
        &BTreeMap::from([(id_from(1), s1_a3), (id_from(2), s2_a3), (id_from(4), s4_a3)]),
        &dkg.public_key_package,
    )
    .expect("aggregate")
    {
        SigningOutcome::Aggregated(sig) => sig,
        other => panic!("expected aggregated signature, got {other:?}"),
    };

    let sig_bytes = sig.serialize().expect("serialize sig");
    assert_eq!(sig_bytes.len(), 64);
    verify_group_signature(&dkg.public_key_package, msg, &sig_bytes).expect("valid group sig");

    // Exhaustive single-bit flip check across all 512 bits + truncation + trailing bytes.
    for bit_idx in 0..512usize {
        let mut flipped = sig_bytes.clone();
        flipped[bit_idx / 8] ^= 1_u8 << (bit_idx % 8);
        assert!(
            matches!(
                verify_group_signature(&dkg.public_key_package, msg, &flipped),
                Err(HostError::MalformedSignature)
            ),
            "bit {bit_idx} mutation unexpectedly accepted"
        );
    }
    assert!(matches!(
        verify_group_signature(&dkg.public_key_package, msg, &sig_bytes[..63]),
        Err(HostError::MalformedSignature)
    ));
    let mut extended = sig_bytes;
    extended.push(0x00);
    assert!(matches!(
        verify_group_signature(&dkg.public_key_package, msg, &extended),
        Err(HostError::MalformedSignature)
    ));
}

#[test]
fn hyparview_2pc_and_peaceful_lineage_preserve_invariants() {
    let tmp = TempDb::new("hyparview_lineage");
    let ledger = CustodyLedger::open(&tmp.path).expect("open");

    let mut overlay = HyParViewOverlay::new(2, 4).expect("overlay");
    let p1 = [1; 32];
    let p2 = [2; 32];
    let p3 = [3; 32];
    assert!(overlay.seed_peer(p1));
    assert!(overlay.seed_peer(p2));

    overlay.propose_phase_a(1, p3, Some(p2)).expect("phase a");
    overlay.acquire_phase_b1_locks(1).expect("locks");
    overlay.vote_phase_b1(1, p1, true).expect("vote 1");
    overlay.vote_phase_b1(1, p2, true).expect("vote 2");
    assert!(overlay.commit_phase_b2(1, 2).expect("commit"));
    assert_eq!(overlay.pending_stubs_count(), 0);
    assert_eq!(overlay.active_locks_count(), 0);
    assert!(overlay.is_active(&p3));
    assert!(!overlay.is_active(&p2));

    // Aborted/timed-out rewiring leaves zero stubs and zero locks.
    overlay.propose_phase_a(2, p2, Some(p1)).expect("phase a 2");
    overlay.acquire_phase_b1_locks(2).expect("locks 2");
    overlay.abort_or_timeout(2).expect("abort");
    assert_eq!(overlay.pending_stubs_count(), 0);
    assert_eq!(overlay.active_locks_count(), 0);
    assert!(overlay.is_active(&p1));

    // Peaceful lineage fork does not spend any UCAN and can be hot-revoked.
    let target = [0xCD; 32];
    let ucan = [0xEF; 32];
    ForkLocalLineage::r2_refuse(&ledger, &target).expect("r2_refuse");
    assert!(ledger.is_r2_refused(&target).expect("refused"));
    assert!(!ledger.is_ucan_spent(&ucan).expect("ucan untouched"));
    assert!(ForkLocalLineage::remaining_quorum_viable(4, 1, 3));
    assert!(ForkLocalLineage::r2_unrefuse(&ledger, &target).expect("r2_unrefuse"));
    assert!(!ledger.is_r2_refused(&target).expect("unrefused"));
}

#[test]
fn ocap_two_levels_and_113_byte_delegation_cert_enforce_pop_and_monotonic_seq() {
    let tmp = TempDb::new("ocap");
    let ledger = CustodyLedger::open(&tmp.path).expect("open");
    let metrics = LockFreeMetrics::new();

    // Level 1 intent containment.
    let mut mask = SandboxCapabilitySet::full_sandbox_intents();
    mask.allow_edge = false;
    OcapFirewall::authorize_intent(mask, SandboxIntent::Vital, &metrics).expect("vital ok");
    assert!(matches!(
        OcapFirewall::authorize_intent(mask, SandboxIntent::Edge, &metrics),
        Err(HostError::OcapDenied(_))
    ));
    assert_eq!(metrics.ocap_violations_total.load(Ordering::Relaxed), 1);

    // Level 2 113-byte cert with sovereign + transport PoP + monotonic delegation_seq.
    let sovereign_sk = SigningKey::from_bytes(&[0x33; 32]);
    let transport_sk = SigningKey::from_bytes(&[0x44; 32]);
    let mut cert = TransportDelegationCert {
        sovereign_pk: sovereign_sk.verifying_key().to_bytes(),
        transport_pk: transport_sk.verifying_key().to_bytes(),
        delegation_seq: 1,
        issued_at: 1_000,
        ttl_secs: 600,
        sovereign_sig: [0; 64],
        transport_pop_sig: [0; 64],
    };
    let payload = cert.canonical_payload();
    assert_eq!(payload.len(), TRANSPORT_DELEG_PAYLOAD_LEN);
    assert_eq!(payload.len(), 113);
    cert.sovereign_sig = sovereign_sk.sign(&payload).to_bytes();
    cert.transport_pop_sig = transport_sk.sign(&cert.pop_message()).to_bytes();

    cert.verify_and_activate(1_200, &ledger, &metrics).expect("valid activation");
    // Replaying same seq=1 must fail.
    assert!(matches!(
        cert.verify_and_activate(1_200, &ledger, &metrics),
        Err(HostError::Custody(_))
    ));
    // Expired certificate fails.
    let mut expired = cert;
    expired.delegation_seq = 2;
    let p2 = expired.canonical_payload();
    expired.sovereign_sig = sovereign_sk.sign(&p2).to_bytes();
    expired.transport_pop_sig = transport_sk.sign(&expired.pop_message()).to_bytes();
    assert!(matches!(
        expired.verify_and_activate(1_600, &ledger, &metrics),
        Err(HostError::OcapDenied(_))
    ));
    // TTL > 3600 fails.
    let mut over_ttl = cert;
    over_ttl.ttl_secs = 3_601;
    assert!(matches!(
        over_ttl.verify_and_activate(1_200, &ledger, &metrics),
        Err(HostError::OcapDenied(_))
    ));
}

#[test]
fn hsm_ffi_sign_verify_strict_catches_fault_and_is_thread_safe() {
    let metrics = Arc::new(LockFreeMetrics::new());
    let signer = Arc::new(HsmSigner::from_seed([0x55; 32]).expect("init hsm slot"));

    let sig_bytes = signer
        .sign_and_verify_strict(b"canonical-payload", &metrics)
        .expect("nominal FFI sign passes verify_strict");
    assert_eq!(sig_bytes.len(), 64);

    // Concurrent reads/signing under ThreadSanitizer.
    let mut handles = Vec::new();
    for t in 0..4_u8 {
        let s = Arc::clone(&signer);
        let m = Arc::clone(&metrics);
        handles.push(thread::spawn(move || {
            for round in 0..16_u8 {
                let msg = [t, round, 0xAA, 0x55];
                let out = s
                    .sign_and_verify_strict(&msg, &m)
                    .expect("concurrent sign ok");
                assert_eq!(out.len(), 64);
            }
        }));
    }
    for h in handles {
        h.join().expect("thread finished");
    }

    // Arm simulated physical fault glitch inside C-ABI module -> verify_strict rejects & counts.
    signer.arm_simulated_fault_glitch().expect("arm fault");
    assert_eq!(
        signer.sign_and_verify_strict(b"glitched-msg", &metrics),
        Err(HostError::HsmFaultInjectionDetected)
    );
    assert_eq!(metrics.hsm_fault_injections_total.load(Ordering::Relaxed), 1);
}

#[test]
fn solar_watchdog_maps_lvd_blackout_to_codd_and_checkpoints_without_fraud() {
    let tmp = TempDb::new("solar");
    let ledger = CustodyLedger::open(&tmp.path).expect("open");
    let metrics = LockFreeMetrics::new();
    let mut engine = SolarWatchdogEngine::new(SolarWatchdogConfig::default_calibrated()).expect("cfg");

    // Nominal step sets EMA cleanly inside Nominal.
    let s1 = engine
        .step_reading(
            SolarReading {
                voltage_scaled: 900_000,
                soc_scaled: 850_000,
                bus_powered: true,
            },
            10,
            10,
            &ledger,
            &metrics,
        )
        .expect("nominal");
    assert_eq!(s1.state, SolarState::Nominal);
    assert_eq!(s1.voltage_codd, CoddValue::Known(900_000));
    assert_eq!(s1.energy_kappa_scale_millionths, UNIT);

    // Low-voltage disconnect / unpowered bus: MUST report ApplicableUnknown, never Known(0).
    let mut pre_lvd_triggered = false;
    for ts in 11..=30 {
        let out = engine
            .step_reading(
                SolarReading {
                    voltage_scaled: 0,
                    soc_scaled: 800_000,
                    bus_powered: false,
                },
                ts,
                ts,
                &ledger,
                &metrics,
            )
            .expect("blackout step");
        assert_eq!(out.voltage_codd, CoddValue::ApplicableUnknown);
        if out.checkpoint_written {
            pre_lvd_triggered = true;
            assert_eq!(out.state, SolarState::PreLvdShutdown);
            assert_eq!(out.energy_kappa_scale_millionths, 0);
        }
    }
    assert!(pre_lvd_triggered);
    assert_eq!(metrics.redb_checkpoints_total.load(Ordering::Relaxed), 1);
    assert_eq!(ledger.heads().expect("heads").contingency_total_sealed, 1);

    // Level-1 cue packs into 8 bits (<= 10 bits/s) and Level-2 audit drilldown captures state.
    let cue = Level1Cue {
        regime: RegimeTag::AlgedonicHalt,
        binding_axis: BindingAxisTag::Vital,
        algedonic: true,
        codd_status: CoddStatusTag::from_codd(CoddValue::ApplicableUnknown),
    };
    let packed = cue.pack();
    assert_eq!(Level1Cue::unpack(packed).expect("unpack"), cue);

    let step_reading = StepReading::new(
        Score::Present(SCALE),
        Score::Present(200_000),
        Score::Present(SCALE),
        Score::Present(SCALE),
        false,
        50,
    );
    let steps = [step_reading];
    let diag = diagnose(&Trajectory::seal(&steps));
    let _drilldown = Level2AuditDrilldown {
        custody_heads: ledger.heads().expect("heads"),
        retrospective_audit: RetrospectiveAudit::from_diagnosis(&diag),
        metrics: metrics.snapshot(),
    };
    let om = metrics.render_openmetrics();
    assert!(om.contains("stoch_redb_checkpoints_total 1"));
    assert!(om.ends_with("# EOF\n"));
}

#[test]
fn leases_contestation_allostatic_care_and_correlated_brownout_regime_iiib() {
    let custodian_sk = SigningKey::from_bytes(&[0x66; 32]);
    let forum_sk = SigningKey::from_bytes(&[0x77; 32]);
    let mut lease = EmergencyLease::new(
        [0x01; 32],
        10,
        50,
        1,
        custodian_sk.verifying_key().to_bytes(),
        forum_sk.verifying_key().to_bytes(),
    )
    .expect("lease");
    assert!(lease.is_active(49));
    assert!(!lease.is_active(50));

    // Self-countersigning by custodian must fail.
    let msg = lease.renewal_message(90);
    let self_sig = custodian_sk.sign(&msg).to_bytes();
    assert!(matches!(
        lease.renew(90, &self_sig),
        Err(HostError::LeaseViolation(_))
    ));
    // External accountability forum counter-signature succeeds once (`max_renewals = 1`).
    let forum_sig = forum_sk.sign(&msg).to_bytes();
    lease.renew(90, &forum_sig).expect("external renew");
    assert!(lease.is_active(80));
    let msg2 = lease.renewal_message(120);
    let forum_sig2 = forum_sk.sign(&msg2).to_bytes();
    assert!(matches!(
        lease.renew(120, &forum_sig2),
        Err(HostError::LeaseViolation(_))
    ));

    // Frontline ContestationRecord (*Philoctetes*) requires delta_status_zero=true.
    let c_payload = ContestationRecord::signing_payload(42, &[0x99; 32], b"telemetry-undercount");
    let c_sig = custodian_sk.sign(&c_payload).to_bytes();
    let rec = ContestationRecord {
        issuer_pk: custodian_sk.verifying_key().to_bytes(),
        epoch: 42,
        target_seal_hash: [0x99; 32],
        delta_status_zero: true,
        statement: b"telemetry-undercount".to_vec(),
        signature: c_sig,
    };
    rec.verify().expect("valid contestation");

    // Allostatic caregiver wear drives Params.vital_floor_breached -> LocalEmergencyController.
    let mut care = AllostaticCareLedger::new(100);
    care.record_shift_wear(40);
    assert!(!care.vital_floor_breached(30));
    care.record_shift_wear(35);
    assert!(care.vital_floor_breached(30));

    // Even under >1/3 (750_000 = 75%) correlated brownout freezing 2/3 FROST quorums,
    // LocalEmergencyController fires Regime III-B AlgedonicHalt immediately.
    let metrics = LockFreeMetrics::new();
    let sealed = State::seal(
        ControlTape::Attested,
        Bounds::new(100, 300, 1000, 0),
        Capacity::new(200, 0, MAX_ABS),
        Guards::new(Instrument::Sealed(SevenReadings::new([SCALE; 7]))),
        Params::new(
            10, // tts < ttr_eff triggers SurvivalLatency Regime III-B halt
            50,
            true,
            care.vital_floor_breached(30),
            false,
            false,
            Thresholds::new(1, 1),
        ),
    )
    .expect("sealed state");

    let regime = LocalEmergencyController::evaluate_local_survival(&sealed, 750_000, &metrics)
        .expect("local emergency controller never stalls");
    assert_eq!(
        regime,
        Regime::AlgedonicHalt {
            cause: HaltCause::SurvivalLatency
        }
    );
    assert_eq!(metrics.local_halt_triggers_total.load(Ordering::Relaxed), 1);
    assert_eq!(
        metrics.fleet_brownout_ratio_millionths.load(Ordering::Relaxed),
        750_000
    );
}
