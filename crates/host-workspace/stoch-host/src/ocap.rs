//! Two-level object-capability firewall separating sandboxed domain intentions (`Level 1`)
//! from short-lived host transport key delegations (`Level 2`, exact 113-byte payload).

use core::sync::atomic::Ordering;
use ed25519_dalek::{Signature, VerifyingKey};
use stoch_custody::CustodyLedger;

use crate::metrics::LockFreeMetrics;
use crate::HostError;

/// Domain prefix for Level-2 transport key delegation payloads (`25` bytes).
pub const DOMAIN_TRANSPORT_DELEG: &[u8; 25] = b"STOCH_TRANSPORT_DELEG_V01";
/// Domain prefix for mutual Proof-of-Possession signed by the ephemeral transport key.
pub const DOMAIN_TRANSPORT_POP: &[u8; 23] = b"STOCH_TRANSPORT_POP_V01";
/// Exact canonical byte length of a serialized transport delegation payload (`25 + 32 + 32 + 8 + 8 + 8`).
pub const TRANSPORT_DELEG_PAYLOAD_LEN: usize = 113;
/// Maximum allowed TTL (`3600` seconds) for ephemeral Level-2 transport keys.
pub const MAX_TRANSPORT_DELEG_TTL_SECS: u64 = 3600;

const _: () = assert!(DOMAIN_TRANSPORT_DELEG.len() == 25);
const _: () = assert!(TRANSPORT_DELEG_PAYLOAD_LEN == 113);

/// Level 1 closed capability class permitted out of the isolated Wasm zone.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum SandboxIntent {
    /// Physical reserve / vital survival status update.
    Vital,
    /// Cross-witness neighbor observation receipt.
    Witness,
    /// Self-verifying objective Byzantine fraud alert.
    Fraud,
    /// Local neighbor conductance/edge adjustment intent.
    Edge,
    /// Peaceful R2 lineage attestation / abstention notice.
    R2Attestation,
}

/// Level 1 policy mask governing which [`SandboxIntent`] classes a sandbox instance may emit.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct SandboxCapabilitySet {
    /// Permits `SandboxIntent::Vital`.
    pub allow_vital: bool,
    /// Permits `SandboxIntent::Witness`.
    pub allow_witness: bool,
    /// Permits `SandboxIntent::Fraud`.
    pub allow_fraud: bool,
    /// Permits `SandboxIntent::Edge`.
    pub allow_edge: bool,
    /// Permits `SandboxIntent::R2Attestation`.
    pub allow_r2_attestation: bool,
}

impl SandboxCapabilitySet {
    /// Authorizes every nominal Level-1 domain intent (never raw HSM or network sockets).
    #[must_use]
    pub const fn full_sandbox_intents() -> Self {
        Self {
            allow_vital: true,
            allow_witness: true,
            allow_fraud: true,
            allow_edge: true,
            allow_r2_attestation: true,
        }
    }

    /// Checks whether `intent` is permitted by this capability mask.
    #[must_use]
    pub const fn permits(self, intent: SandboxIntent) -> bool {
        match intent {
            SandboxIntent::Vital => self.allow_vital,
            SandboxIntent::Witness => self.allow_witness,
            SandboxIntent::Fraud => self.allow_fraud,
            SandboxIntent::Edge => self.allow_edge,
            SandboxIntent::R2Attestation => self.allow_r2_attestation,
        }
    }
}

/// Enforces Level-1 capability containment and increments lock-free violation telemetry on denial.
pub struct OcapFirewall;

impl OcapFirewall {
    /// Validates `intent` against `mask`, recording lock-free violation counts when denied.
    pub fn authorize_intent(
        mask: SandboxCapabilitySet,
        intent: SandboxIntent,
        metrics: &LockFreeMetrics,
    ) -> Result<(), HostError> {
        if mask.permits(intent) {
            Ok(())
        } else {
            metrics.ocap_violations_total.fetch_add(1, Ordering::Relaxed);
            Err(HostError::OcapDenied("sandbox intent denied by Level-1 OCap mask"))
        }
    }
}

/// Level-2 short-lived transport key delegation certificate with mutual Proof-of-Possession.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct TransportDelegationCert {
    /// Long-term sovereign public key authenticating this delegation (`verify_strict`).
    pub sovereign_pk: [u8; 32],
    /// Ephemeral session transport public key bound to `sovereign_pk`.
    pub transport_pk: [u8; 32],
    /// Monotonic sequence persisted in `CustodyLedger::DELEGATION_SEQS`.
    pub delegation_seq: u64,
    /// UNIX issuance timestamp in seconds.
    pub issued_at: u64,
    /// Validity window in seconds (`1 <= ttl_secs <= 3600`).
    pub ttl_secs: u64,
    /// Signature over the 113-byte canonical delegation payload by `sovereign_pk`.
    pub sovereign_sig: [u8; 64],
    /// Mutual Proof-of-Possession signature over `DOMAIN_TRANSPORT_POP || payload_113` by `transport_pk`.
    pub transport_pop_sig: [u8; 64],
}

impl TransportDelegationCert {
    /// Encodes the exact 113-byte canonical delegation payload signed by `sovereign_pk`.
    #[must_use]
    pub fn canonical_payload(&self) -> [u8; TRANSPORT_DELEG_PAYLOAD_LEN] {
        let mut buf = [0_u8; TRANSPORT_DELEG_PAYLOAD_LEN];
        buf[0..25].copy_from_slice(DOMAIN_TRANSPORT_DELEG);
        buf[25..57].copy_from_slice(&self.sovereign_pk);
        buf[57..89].copy_from_slice(&self.transport_pk);
        buf[89..97].copy_from_slice(&self.delegation_seq.to_le_bytes());
        buf[97..105].copy_from_slice(&self.issued_at.to_le_bytes());
        buf[105..113].copy_from_slice(&self.ttl_secs.to_le_bytes());
        buf
    }

    /// Computes the exact byte slice signed by `transport_pk` to prove possession.
    #[must_use]
    pub fn pop_message(&self) -> [u8; 23 + TRANSPORT_DELEG_PAYLOAD_LEN] {
        let payload = self.canonical_payload();
        let mut msg = [0_u8; 23 + TRANSPORT_DELEG_PAYLOAD_LEN];
        msg[0..23].copy_from_slice(DOMAIN_TRANSPORT_POP);
        msg[23..136].copy_from_slice(&payload);
        msg
    }

    /// Verifies TTL limits (`1..=3600`), expiry against `now_secs`, both strict Ed25519 signatures
    /// (`verify_strict`), and atomically advances `delegation_seq` in `ledger` to prevent replay.
    pub fn verify_and_activate(
        &self,
        now_secs: u64,
        ledger: &CustodyLedger,
        metrics: &LockFreeMetrics,
    ) -> Result<(), HostError> {
        if self.ttl_secs == 0 || self.ttl_secs > MAX_TRANSPORT_DELEG_TTL_SECS {
            metrics.ocap_violations_total.fetch_add(1, Ordering::Relaxed);
            return Err(HostError::OcapDenied("delegation TTL outside 1..=3600 seconds"));
        }
        let expires_at = self
            .issued_at
            .checked_add(self.ttl_secs)
            .ok_or(HostError::OcapDenied("delegation expiry timestamp overflow"))?;
        if now_secs < self.issued_at || now_secs >= expires_at {
            metrics.ocap_violations_total.fetch_add(1, Ordering::Relaxed);
            return Err(HostError::OcapDenied("delegation expired or not yet active"));
        }

        let sov_vk = VerifyingKey::from_bytes(&self.sovereign_pk)
            .map_err(|_| HostError::OcapDenied("invalid sovereign public key"))?;
        let trn_vk = VerifyingKey::from_bytes(&self.transport_pk)
            .map_err(|_| HostError::OcapDenied("invalid transport public key"))?;

        let payload = self.canonical_payload();
        let sov_sig = Signature::from_bytes(&self.sovereign_sig);
        if sov_vk.verify_strict(&payload, &sov_sig).is_err() {
            metrics.ocap_violations_total.fetch_add(1, Ordering::Relaxed);
            return Err(HostError::OcapDenied("invalid sovereign delegation signature"));
        }

        let pop_msg = self.pop_message();
        let pop_sig = Signature::from_bytes(&self.transport_pop_sig);
        if trn_vk.verify_strict(&pop_msg, &pop_sig).is_err() {
            metrics.ocap_violations_total.fetch_add(1, Ordering::Relaxed);
            return Err(HostError::OcapDenied("invalid mutual transport Proof-of-Possession"));
        }

        if let Err(err) = ledger.advance_delegation_seq(&self.sovereign_pk, self.delegation_seq) {
            metrics.ocap_violations_total.fetch_add(1, Ordering::Relaxed);
            return Err(err.into());
        }
        Ok(())
    }
}
