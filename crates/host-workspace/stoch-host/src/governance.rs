//! Constitutional emergency leases (Brake 3 non-self-extending sunsets), signed frontline
//! contestations (*Philoctetes* with zero status discounting), and discrete caregiver
//! allostatic load (`W_allostatic_t`) feeding online `Params.vital_floor_breached`.

use ed25519_dalek::{Signature, VerifyingKey};

use crate::HostError;

const DOMAIN_LEASE_RENEW: &[u8] = b"stoch-host/lease-renew/v1";
const DOMAIN_CONTESTATION: &[u8] = b"stoch-host/contestation/v1";

/// Constitutional time-bounded emergency authorization lease (`sunset` strictly finite).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct EmergencyLease {
    /// Unique lease identifier.
    pub lease_id: [u8; 32],
    /// Epoch when the lease enters force.
    pub issued_epoch: u64,
    /// Hard expiration epoch (`sunset_epoch > issued_epoch`).
    pub sunset_epoch: u64,
    /// Maximum externally counter-signed renewals allowed (`0` for non-renewable).
    pub max_renewals: u8,
    /// Renewals consumed so far (`renewals_used <= max_renewals`).
    pub renewals_used: u8,
    /// Sovereign/custodian public key prohibited from self-extending this lease.
    pub custodian_pk: [u8; 32],
    /// Independent external accountability forum public key required to co-sign renewals.
    pub external_forum_pk: [u8; 32],
}

impl EmergencyLease {
    /// Constructs a time-bounded emergency lease rejecting inverted deadlines or identical keys.
    pub fn new(
        lease_id: [u8; 32],
        issued_epoch: u64,
        sunset_epoch: u64,
        max_renewals: u8,
        custodian_pk: [u8; 32],
        external_forum_pk: [u8; 32],
    ) -> Result<Self, HostError> {
        if sunset_epoch <= issued_epoch {
            return Err(HostError::LeaseViolation("sunset_epoch must exceed issued_epoch"));
        }
        if custodian_pk == external_forum_pk {
            return Err(HostError::LeaseViolation(
                "emergency lease cannot self-countersign with identical custodian key",
            ));
        }
        Ok(Self {
            lease_id,
            issued_epoch,
            sunset_epoch,
            max_renewals,
            renewals_used: 0,
            custodian_pk,
            external_forum_pk,
        })
    }

    /// Returns `true` strictly while `issued_epoch <= now_epoch < sunset_epoch`.
    #[must_use]
    pub const fn is_active(&self, now_epoch: u64) -> bool {
        now_epoch >= self.issued_epoch && now_epoch < self.sunset_epoch
    }

    /// Canonical renewal message bound to `(lease_id, next_renewal_index, new_sunset_epoch)`.
    #[must_use]
    pub fn renewal_message(&self, new_sunset_epoch: u64) -> Vec<u8> {
        let next_idx = self.renewals_used.saturating_add(1);
        let mut msg = Vec::with_capacity(DOMAIN_LEASE_RENEW.len() + 41);
        msg.extend_from_slice(DOMAIN_LEASE_RENEW);
        msg.extend_from_slice(&self.lease_id);
        msg.push(next_idx);
        msg.extend_from_slice(&new_sunset_epoch.to_le_bytes());
        msg
    }

    /// Renews the emergency lease ONLY with a valid external accountability-forum signature
    /// (`verify_strict`), within `max_renewals`, and extending strictly forward (`> sunset_epoch`).
    pub fn renew(
        &mut self,
        new_sunset_epoch: u64,
        external_counter_sig: &[u8; 64],
    ) -> Result<(), HostError> {
        if self.renewals_used >= self.max_renewals {
            return Err(HostError::LeaseViolation("max_renewals exhausted"));
        }
        if new_sunset_epoch <= self.sunset_epoch {
            return Err(HostError::LeaseViolation("new sunset_epoch must strictly advance"));
        }
        let forum_vk = VerifyingKey::from_bytes(&self.external_forum_pk)
            .map_err(|_| HostError::LeaseViolation("invalid external forum verifying key"))?;
        let msg = self.renewal_message(new_sunset_epoch);
        let sig = Signature::from_bytes(external_counter_sig);
        forum_vk
            .verify_strict(&msg, &sig)
            .map_err(|_| HostError::LeaseViolation("invalid external forum counter-signature"))?;
        self.renewals_used += 1;
        self.sunset_epoch = new_sunset_epoch;
        Ok(())
    }
}

/// Signed frontline contestation record requiring zero status discounting (`delta_status_zero`).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ContestationRecord {
    /// Signing frontline observer/custodian public key.
    pub issuer_pk: [u8; 32],
    /// Epoch in which contestation is filed.
    pub epoch: u64,
    /// `BLAKE3` digest of the challenged sealed tape record or institutional directive.
    pub target_seal_hash: [u8; 32],
    /// Constitutional guarantee that testimonial weight carries zero status discounting (`delta = 0`).
    pub delta_status_zero: bool,
    /// Contestation text / technical grounds.
    pub statement: Vec<u8>,
    /// Strict Ed25519 signature over `(DOMAIN_CONTESTATION || epoch || target_seal_hash || statement)`.
    pub signature: [u8; 64],
}

impl ContestationRecord {
    /// Canonical byte payload authenticated by `signature`.
    #[must_use]
    pub fn signing_payload(epoch: u64, target_seal_hash: &[u8; 32], statement: &[u8]) -> Vec<u8> {
        let mut msg = Vec::with_capacity(DOMAIN_CONTESTATION.len() + 40 + statement.len());
        msg.extend_from_slice(DOMAIN_CONTESTATION);
        msg.extend_from_slice(&epoch.to_le_bytes());
        msg.extend_from_slice(target_seal_hash);
        msg.extend_from_slice(statement);
        msg
    }

    /// Verifies `delta_status_zero == true`, non-empty statement, and strict Ed25519 signature.
    pub fn verify(&self) -> Result<(), HostError> {
        if !self.delta_status_zero || self.statement.is_empty() {
            return Err(HostError::LeaseViolation(
                "contestation requires delta_status_zero=true and non-empty statement",
            ));
        }
        let vk = VerifyingKey::from_bytes(&self.issuer_pk)
            .map_err(|_| HostError::LeaseViolation("invalid contestation issuer public key"))?;
        let payload = Self::signing_payload(self.epoch, &self.target_seal_hash, &self.statement);
        let sig = Signature::from_bytes(&self.signature);
        vk.verify_strict(&payload, &sig)
            .map_err(|_| HostError::LeaseViolation("invalid contestation signature"))
    }
}

/// Discrete caregiver allostatic load carrier `W_allostatic,t` (`Ch. 42 §1.1.1`).
///
/// Monotonically accumulates emergency strain `psi_t >= 0` (`W_{t+1} = W_t + psi_t`) so online
/// `Params.vital_floor_breached` fires before human operators cross exhaustion even while
/// physical equipment reserves look intact.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct AllostaticCareLedger {
    r0_care: u64,
    w_allostatic: u64,
}

impl AllostaticCareLedger {
    /// Initializes an unstrained caregiver ledger with baseline capacity `r0_care`.
    #[must_use]
    pub const fn new(r0_care: u64) -> Self {
        Self {
            r0_care,
            w_allostatic: 0,
        }
    }

    /// Accumulates non-negative step strain `psi_t` monotonically (`W_{t+1} = W_t + psi_t`).
    pub fn record_shift_wear(&mut self, psi_t: u64) {
        self.w_allostatic = self.w_allostatic.saturating_add(psi_t);
    }

    /// Cumulative recorded wear `W_allostatic,t`.
    #[must_use]
    pub const fn w_allostatic(&self) -> u64 {
        self.w_allostatic
    }

    /// Remaining caregiver vital endurance `S_vital,t^care = max(0, R_0^care - W_allostatic,t)`.
    #[must_use]
    pub const fn s_vital_care(&self) -> u64 {
        self.r0_care.saturating_sub(self.w_allostatic)
    }

    /// Returns `true` when remaining care endurance falls below `floor_threshold`, signalling
    /// `stoch_core::Params.vital_floor_breached`.
    #[must_use]
    pub const fn vital_floor_breached(&self, floor_threshold: u64) -> bool {
        self.s_vital_care() < floor_threshold
    }
}
