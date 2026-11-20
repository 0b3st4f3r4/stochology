//! Host governance, ephemeral FROST threshold signing, HyParView 2PC membership,
//! peaceful lineage branching, two-level OCap firewall, HSM FFI strict verification,
//! solar energetic survival watchdog, contention-free OpenMetrics, <=10 bits/s cues,
//! constitutional sunsets, and offline AOT `.cwasm` precompilation.
//!
//! Strictly separates online control inputs (`stoch_core::State`) from ex-post trajectory
//! audit envelopes ([`RetrospectiveAudit`]):
//!
//! ```compile_fail
//! use stoch_core::{diagnose, step, Trajectory};
//! use stoch_host::RetrospectiveAudit;
//!
//! let diag = diagnose(&Trajectory::seal(&[]));
//! let audit = RetrospectiveAudit::from_diagnosis(&diag);
//! let _ = step(&audit);
//! ```

#![deny(unsafe_code)]
#![deny(missing_docs)]
#![deny(clippy::float_arithmetic)]
#![forbid(clippy::cast_possible_truncation)]

pub mod disclosure;
pub mod emergency;
pub mod frost;
pub mod governance;
pub mod hsm;
pub mod hyparview;
pub mod lineage;
pub mod metrics;
pub mod ocap;
pub mod solar;

use stoch_core::{Axis, Diagnosis, DiagnosisView, Score};
use stoch_custody::CustodyError;

use crate::frost::CeremonyKey;

/// Errors returned by host governance, cryptographic verification, HSM FFI, or custody layers.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum HostError {
    /// Durable custody error propagated from `stoch-custody`.
    Custody(CustodyError),
    /// FROST ceremony nonce already consumed or superseded across attempts/rollbacks.
    NonceReplayRejected(CeremonyKey),
    /// FROST DKG or signing protocol error.
    FrostProtocol(String),
    /// Encoded signature failed length (64 B), canonical scalar, or `verify_strict` checks.
    MalformedSignature,
    /// HyParView two-phase rewiring conflict or invalid proposal identifier.
    HyParViewConflict(&'static str),
    /// Object-capability firewall rejected a sandbox intent, TTL, PoP, or sequence number.
    OcapDenied(&'static str),
    /// Isolated C-ABI HSM boundary error.
    HsmFfi(&'static str),
    /// Post-signature `verify_strict` detected corrupted output across the HSM seam.
    HsmFaultInjectionDetected,
    /// Invalid host configuration bounds.
    InvalidConfiguration(&'static str),
    /// Malformed packed Level-1 cue bits.
    InvalidCueEncoding(&'static str),
    /// Emergency lease sunset, external counter-signature, or contestation failure.
    LeaseViolation(&'static str),
}

impl From<CustodyError> for HostError {
    fn from(err: CustodyError) -> Self {
        Self::Custody(err)
    }
}

impl core::fmt::Display for HostError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Custody(err) => write!(f, "custody error: {err}"),
            Self::NonceReplayRejected(key) => {
                write!(f, "FROST nonce replay rejected at attempt {}", key.attempt)
            }
            Self::FrostProtocol(msg) => write!(f, "FROST protocol error: {msg}"),
            Self::MalformedSignature => write!(f, "malformed or non-strict signature rejected"),
            Self::HyParViewConflict(msg) => write!(f, "HyParView conflict: {msg}"),
            Self::OcapDenied(msg) => write!(f, "OCap firewall denied: {msg}"),
            Self::HsmFfi(msg) => write!(f, "HSM FFI error: {msg}"),
            Self::HsmFaultInjectionDetected => {
                write!(f, "HSM post-sign verify_strict caught fault glitch")
            }
            Self::InvalidConfiguration(msg) => write!(f, "invalid host config: {msg}"),
            Self::InvalidCueEncoding(msg) => write!(f, "invalid Level-1 cue byte: {msg}"),
            Self::LeaseViolation(msg) => write!(f, "lease or contestation failure: {msg}"),
        }
    }
}

impl std::error::Error for HostError {}

/// Strictly ex-post retrospective audit envelope wrapping `stoch_core::DiagnosisView` (BS-1).
///
/// Implements NO conversion into `stoch_core::State`, `Bounds`, or `Params`, enforcing at the
/// type level that retrospective co-diagnostic folds never drive live `State::seal` actions.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RetrospectiveAudit {
    view: DiagnosisView,
}

impl RetrospectiveAudit {
    /// Captures a read-only retrospective audit from a completed trajectory [`Diagnosis`].
    #[must_use]
    pub fn from_diagnosis(diagnosis: &Diagnosis) -> Self {
        Self {
            view: diagnosis.observe(),
        }
    }

    /// Weakest binding axis named by the completed trajectory fold, or `None` if poisoned.
    #[must_use]
    pub const fn binding_axis(&self) -> Option<Axis> {
        self.view.binding_axis
    }

    /// Whether the trajectory crossed its physical ruin barrier.
    #[must_use]
    pub const fn is_absorbed(&self) -> bool {
        self.view.absorbed
    }

    /// Ex-post reception-basin perimeter reading (`theta_basin`).
    #[must_use]
    pub const fn theta_basin(&self) -> Score {
        self.view.theta_basin
    }
}
