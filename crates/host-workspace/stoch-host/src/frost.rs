//! Ephemeral threshold governance using 3-round Feldman VSS DKG and 2-round
//! FROST Schnorr threshold signatures (`frost-ed25519` / RFC 9591).
//!
//! Enforces strict linear consumption of signing nonces bound to [`CeremonyKey`] inside
//! [`CustodyLedger`] prior to releasing round-two shares, separates self-verifying fraud
//! proofs from silent network timeouts, and rejects non-canonical / truncated / bit-flipped
//! 64-byte group signatures via `verify_strict`.

use std::collections::{BTreeMap, BTreeSet};

use ed25519_dalek::{Signature as DalekSignature, VerifyingKey as DalekVerifyingKey};
use frost_ed25519::keys::dkg::{part1, part2, part3, round1 as dkg_round1, round2 as dkg_round2};
use frost_ed25519::keys::{KeyPackage, PublicKeyPackage};
use frost_ed25519::round1::{self, SigningCommitments, SigningNonces};
use frost_ed25519::round2::{self, SignatureShare};
use frost_ed25519::rand_core::{CryptoRng, RngCore};
use frost_ed25519::{aggregate, Identifier, Signature, SigningPackage};
use stoch_custody::CustodyLedger;
use zeroize::Zeroize;

use crate::HostError;

const DOMAIN_CEREMONY_NONCE: &[u8] = b"stoch-host/frost-nonce-consumption/v1";

/// Canonical tuple binding an ephemeral FROST ceremony nonce strictly to one attempt.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct CeremonyKey {
    /// Assembly or group session identifier.
    pub assembly_id: [u8; 32],
    /// Linear capability authorization identifier for this action.
    pub ucan_cid: [u8; 32],
    /// Strictly increasing retry attempt index within `(assembly_id, ucan_cid)`.
    pub attempt: u64,
}

impl CeremonyKey {
    /// Computes the collision-resistant 32-byte custody identifier stored in `SPENT_UCANS`
    /// prior to releasing any round-two signature share for `signer`.
    #[must_use]
    pub fn nonce_consumption_id(&self, signer: Identifier) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(DOMAIN_CEREMONY_NONCE);
        hasher.update(&self.assembly_id);
        hasher.update(&self.ucan_cid);
        hasher.update(&self.attempt.to_le_bytes());
        hasher.update(&signer.serialize());
        *hasher.finalize().as_bytes()
    }
}

/// Strictly single-use signing-nonce store preventing two-signature key extraction under
/// retries or power-loss flash rollbacks (`R0`).
#[derive(Default)]
pub struct SingleUseNonceStore {
    nonces: BTreeMap<CeremonyKey, SigningNonces>,
    highest_attempt: BTreeMap<([u8; 32], [u8; 32]), u64>,
}

impl SingleUseNonceStore {
    /// Creates an empty nonce store.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Generates and stores round-one commitments/nonces for a strictly increasing attempt.
    ///
    /// Rejects non-monotonic attempts and rejects ceremonies whose consumption marker already
    /// exists on `ledger` (e.g., after an `R0` rollback).
    pub fn commit_round1<R: RngCore + CryptoRng>(
        &mut self,
        ceremony: CeremonyKey,
        key_package: &KeyPackage,
        ledger: &CustodyLedger,
        rng: &mut R,
    ) -> Result<SigningCommitments, HostError> {
        let pair = (ceremony.assembly_id, ceremony.ucan_cid);
        if let Some(&prev) = self.highest_attempt.get(&pair) {
            if ceremony.attempt <= prev {
                return Err(HostError::NonceReplayRejected(ceremony));
            }
        }
        let cid = ceremony.nonce_consumption_id(*key_package.identifier());
        if ledger.is_ucan_spent(&cid)? {
            return Err(HostError::NonceReplayRejected(ceremony));
        }
        let (nonces, commitments) = round1::commit(key_package.signing_share(), rng);
        self.highest_attempt.insert(pair, ceremony.attempt);
        self.nonces.insert(ceremony, nonces);
        Ok(commitments)
    }

    /// Atomically removes the nonce from RAM, commits its durable consumption token to
    /// `ledger` (`Durability::Immediate` + 2PC) BEFORE computing the round-two share,
    /// produces the signature share, and zeroizes the consumed nonce buffer.
    pub fn consume_and_sign_round2(
        &mut self,
        ceremony: CeremonyKey,
        signing_package: &SigningPackage,
        key_package: &KeyPackage,
        ledger: &CustodyLedger,
    ) -> Result<SignatureShare, HostError> {
        let mut nonces = self
            .nonces
            .remove(&ceremony)
            .ok_or(HostError::NonceReplayRejected(ceremony))?;
        let cid = ceremony.nonce_consumption_id(*key_package.identifier());
        // Durable commit MUST succeed before emitting any round-two signature share.
        if let Err(err) = ledger.consume_ucan(&cid) {
            nonces.zeroize();
            return Err(err.into());
        }
        let share_res = round2::sign(signing_package, &nonces, key_package);
        nonces.zeroize();
        share_res.map_err(|e| HostError::FrostProtocol(e.to_string()))
    }
}

/// Self-verifying objective proof of Byzantine fraud in DKG or FROST round two.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum SignedFraudProof {
    /// Invalid Feldman VSS commitment, proof of knowledge, or secret share during DKG.
    DkgCheater {
        /// Culprit identifier when determinable by share/PoK verification.
        culprit: Option<Identifier>,
        /// Protocol failure explanation.
        reason: String,
    },
    /// Invalid or tampered FROST round-two signature share emitted by `culprit`.
    FrostRound2Cheater {
        /// Offending signer identifier.
        culprit: Identifier,
        /// Ceremony tuple in which the invalid share was produced.
        ceremony: CeremonyKey,
        /// Verification failure explanation.
        reason: String,
    },
}

/// Result of aggregating round-two shares with strict fraud vs. silent-timeout separation.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum SigningOutcome {
    /// Threshold quorum succeeded and produced a verified group signature.
    Aggregated(Signature),
    /// Independent verification caught an invalid share (`SignedFraudProof`), excluding silent drops.
    CheaterDetected(SignedFraudProof),
    /// Signing lacked threshold due solely to silent unresponsive peers (never treated as fraud).
    PeerTimeout {
        /// Peers that failed to deliver round-two shares within the ceremony deadline.
        timed_out: Vec<Identifier>,
    },
}

/// Complete participant output from a 3-round Feldman VSS DKG execution.
pub struct DkgOutcome {
    /// Per-participant long-lived secret/public key share packages.
    pub key_packages: BTreeMap<Identifier, KeyPackage>,
    /// Shared group verifying package containing every participant share commitment.
    pub public_key_package: PublicKeyPackage,
}

/// Executes a genuine 3-round Feldman VSS distributed key generation across `1..=max_signers`.
pub fn run_dkg_3_rounds<R: RngCore + CryptoRng>(
    min_signers: u16,
    max_signers: u16,
    rng: &mut R,
) -> Result<DkgOutcome, HostError> {
    if min_signers < 2 || max_signers < min_signers {
        return Err(HostError::FrostProtocol(
            "FROST DKG requires 2 <= min_signers <= max_signers".to_string(),
        ));
    }

    let mut ids = Vec::with_capacity(usize::from(max_signers));
    for raw_id in 1..=max_signers {
        let id = Identifier::try_from(raw_id)
            .map_err(|e| HostError::FrostProtocol(e.to_string()))?;
        ids.push(id);
    }

    // Round 1: each participant runs Feldman commitment + PoK generation.
    let mut r1_secrets: BTreeMap<Identifier, dkg_round1::SecretPackage> = BTreeMap::new();
    let mut r1_packages: BTreeMap<Identifier, dkg_round1::Package> = BTreeMap::new();
    for &id in &ids {
        let (sec, pkg) = part1(id, max_signers, min_signers, &mut *rng)
            .map_err(|e| HostError::FrostProtocol(e.to_string()))?;
        r1_secrets.insert(id, sec);
        r1_packages.insert(id, pkg);
    }

    // Round 2: each participant receives peers' Round-1 packages and emits encrypted/authenticated shares.
    let mut r2_secrets: BTreeMap<Identifier, dkg_round2::SecretPackage> = BTreeMap::new();
    let mut r2_inboxes: BTreeMap<Identifier, BTreeMap<Identifier, dkg_round2::Package>> =
        BTreeMap::new();
    for &id in &ids {
        r2_inboxes.insert(id, BTreeMap::new());
    }
    for &sender in &ids {
        let sec1 = r1_secrets
            .remove(&sender)
            .ok_or_else(|| HostError::FrostProtocol("missing r1 secret".to_string()))?;
        let others_r1: BTreeMap<Identifier, dkg_round1::Package> = r1_packages
            .iter()
            .filter(|(&k, _)| k != sender)
            .map(|(&k, v)| (k, v.clone()))
            .collect();
        let (sec2, pkgs_for_peers) = part2(sec1, &others_r1)
            .map_err(|e| HostError::FrostProtocol(e.to_string()))?;
        r2_secrets.insert(sender, sec2);
        for (recipient, pkg) in pkgs_for_peers {
            if let Some(inbox) = r2_inboxes.get_mut(&recipient) {
                inbox.insert(sender, pkg);
            }
        }
    }

    // Round 3: each participant verifies Feldman commitments & derives KeyPackage + PublicKeyPackage.
    let mut key_packages = BTreeMap::new();
    let mut group_public: Option<PublicKeyPackage> = None;
    for &id in &ids {
        let sec2 = r2_secrets
            .get(&id)
            .ok_or_else(|| HostError::FrostProtocol("missing r2 secret".to_string()))?;
        let others_r1: BTreeMap<Identifier, dkg_round1::Package> = r1_packages
            .iter()
            .filter(|(&k, _)| k != id)
            .map(|(&k, v)| (k, v.clone()))
            .collect();
        let inbox = r2_inboxes
            .get(&id)
            .ok_or_else(|| HostError::FrostProtocol("missing r2 inbox".to_string()))?;
        let (key_pkg, pub_pkg) = part3(sec2, &others_r1, inbox)
            .map_err(|e| HostError::FrostProtocol(e.to_string()))?;
        key_packages.insert(id, key_pkg);
        if group_public.is_none() {
            group_public = Some(pub_pkg);
        }
    }

    let public_key_package = group_public
        .ok_or_else(|| HostError::FrostProtocol("empty DKG output".to_string()))?;
    Ok(DkgOutcome {
        key_packages,
        public_key_package,
    })
}

/// Independently verifies and aggregates round-two signature shares for `ceremony`.
///
/// Missing committed signers yield [`SigningOutcome::PeerTimeout`] without emitting fraud;
/// invalid cryptographic shares return [`SigningOutcome::CheaterDetected`].
pub fn aggregate_with_independent_verification(
    ceremony: CeremonyKey,
    signing_package: &SigningPackage,
    signature_shares: &BTreeMap<Identifier, SignatureShare>,
    public_key_package: &PublicKeyPackage,
) -> Result<SigningOutcome, HostError> {
    let expected: BTreeSet<Identifier> = signing_package
        .signing_commitments()
        .keys()
        .copied()
        .collect();
    let timed_out: Vec<Identifier> = expected
        .iter()
        .filter(|id| !signature_shares.contains_key(*id))
        .copied()
        .collect();
    if !timed_out.is_empty() {
        return Ok(SigningOutcome::PeerTimeout { timed_out });
    }

    match aggregate(signing_package, signature_shares, public_key_package) {
        Ok(sig) => Ok(SigningOutcome::Aggregated(sig)),
        Err(err) => {
            if let Some(culprit) = err.culprit() {
                Ok(SigningOutcome::CheaterDetected(
                    SignedFraudProof::FrostRound2Cheater {
                        culprit,
                        ceremony,
                        reason: err.to_string(),
                    },
                ))
            } else {
                Err(HostError::FrostProtocol(err.to_string()))
            }
        }
    }
}

/// Verifies an encoded group Ed25519 signature strictly (`len == 64`, canonical scalar,
/// small-order point rejection, and FROST group key equation).
pub fn verify_group_signature(
    public_key_package: &PublicKeyPackage,
    message: &[u8],
    sig_bytes: &[u8],
) -> Result<(), HostError> {
    let arr: [u8; 64] = sig_bytes
        .try_into()
        .map_err(|_| HostError::MalformedSignature)?;

    let frost_sig = Signature::deserialize(&arr)
        .map_err(|_| HostError::MalformedSignature)?;
    public_key_package
        .verifying_key()
        .verify(message, &frost_sig)
        .map_err(|_| HostError::MalformedSignature)?;

    let vk_bytes_vec = public_key_package
        .verifying_key()
        .serialize()
        .map_err(|e| HostError::FrostProtocol(e.to_string()))?;
    let vk_bytes: [u8; 32] = vk_bytes_vec
        .try_into()
        .map_err(|_| HostError::MalformedSignature)?;
    let dalek_vk = DalekVerifyingKey::from_bytes(&vk_bytes)
        .map_err(|_| HostError::MalformedSignature)?;
    let dalek_sig = DalekSignature::from_bytes(&arr);
    dalek_vk
        .verify_strict(message, &dalek_sig)
        .map_err(|_| HostError::MalformedSignature)
}
