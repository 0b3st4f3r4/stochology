//! Custody validation, integrity, bitemporal ordering, and storage errors.

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

pub(crate) fn map_storage<E: core::fmt::Display>(err: E) -> CustodyError {
    CustodyError::Storage(err.to_string())
}
