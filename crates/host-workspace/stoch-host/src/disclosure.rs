//! Two-level disclosure interface keeping synchronous operator cues strictly within 10 bits/s
//! while relegating forensic ledger drilldowns to off-shift inspection.

use stoch_core::{Axis, Regime};
use stoch_custody::CustodyHeads;
use stoch_math::CoddValue;

use crate::metrics::MetricsSnapshot;
use crate::{HostError, RetrospectiveAudit};

/// Compact 3-bit regime discriminant matching `stoch_core::Regime::tag()`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum RegimeTag {
    /// Regime I buffered action (`0`).
    BufferedAction = 0,
    /// Regime II fractional probe (`1`).
    ReversibleFractionalProbe = 1,
    /// Inward margin-recovery action (`2`).
    InwardAction = 2,
    /// Quarantine on missing reading (`3`).
    Quarantine = 3,
    /// Regime III-B algedonic halt (`4`).
    AlgedonicHalt = 4,
    /// No action (`5`).
    NoAction = 5,
}

impl RegimeTag {
    /// Maps a live [`stoch_core::Regime`] to its 3-bit display tag.
    #[must_use]
    pub const fn from_regime(regime: &Regime) -> Self {
        match regime {
            Regime::BufferedAction => Self::BufferedAction,
            Regime::ReversibleFractionalProbe { .. } => Self::ReversibleFractionalProbe,
            Regime::InwardAction { .. } => Self::InwardAction,
            Regime::Quarantine { .. } => Self::Quarantine,
            Regime::AlgedonicHalt { .. } => Self::AlgedonicHalt,
            Regime::NoAction => Self::NoAction,
        }
    }

    fn from_bits(bits: u8) -> Result<Self, HostError> {
        match bits {
            0 => Ok(Self::BufferedAction),
            1 => Ok(Self::ReversibleFractionalProbe),
            2 => Ok(Self::InwardAction),
            3 => Ok(Self::Quarantine),
            4 => Ok(Self::AlgedonicHalt),
            5 => Ok(Self::NoAction),
            _ => Err(HostError::InvalidCueEncoding("invalid 3-bit regime tag")),
        }
    }
}

/// Compact 2-bit axis discriminant (`0..=3`).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum BindingAxisTag {
    /// No axis currently named (`0`).
    None = 0,
    /// Epistemic margin (`1`).
    Epistemic = 1,
    /// Vital margin (`2`).
    Vital = 2,
    /// Institutional governance margin (`3`).
    Governance = 3,
}

impl BindingAxisTag {
    /// Encodes an optional `stoch_core::Axis`.
    #[must_use]
    pub const fn from_option(axis: Option<Axis>) -> Self {
        match axis {
            None => Self::None,
            Some(Axis::Epistemic) => Self::Epistemic,
            Some(Axis::Vital) => Self::Vital,
            Some(Axis::Governance) => Self::Governance,
        }
    }

    fn from_bits(bits: u8) -> Result<Self, HostError> {
        match bits {
            0 => Ok(Self::None),
            1 => Ok(Self::Epistemic),
            2 => Ok(Self::Vital),
            3 => Ok(Self::Governance),
            _ => Err(HostError::InvalidCueEncoding("invalid 2-bit binding axis")),
        }
    }
}

/// Compact 2-bit Codd relational intake discriminant (`0..=3`).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum CoddStatusTag {
    /// Known scalar observation (`0`).
    Known = 0,
    /// Applicable yet unobserved / unpowered `A-mark` (`1`).
    ApplicableUnknown = 1,
    /// Inapplicable `I-mark` (`2`).
    Inapplicable = 2,
    /// Pre-schema excluded (`3`).
    NotAskable = 3,
}

impl CoddStatusTag {
    /// Classifies a [`CoddValue`] into its 2-bit tag.
    #[must_use]
    pub const fn from_codd(val: CoddValue) -> Self {
        match val {
            CoddValue::Known(_) => Self::Known,
            CoddValue::ApplicableUnknown => Self::ApplicableUnknown,
            CoddValue::Inapplicable => Self::Inapplicable,
            CoddValue::NotAskable => Self::NotAskable,
        }
    }

    fn from_bits(bits: u8) -> Result<Self, HostError> {
        match bits {
            0 => Ok(Self::Known),
            1 => Ok(Self::ApplicableUnknown),
            2 => Ok(Self::Inapplicable),
            3 => Ok(Self::NotAskable),
            _ => Err(HostError::InvalidCueEncoding("invalid 2-bit Codd status")),
        }
    }
}

/// Level 1 synchronous operator cue packed into 1 byte (`8 bits <= 10 bits/s` ceiling).
///
/// Bit layout (`LSB -> MSB`):
/// - bits `0..=2`: `regime` (3 bits)
/// - bits `3..=4`: `binding_axis` (2 bits)
/// - bit  `5`:     `algedonic` flag (1 bit)
/// - bits `6..=7`: `codd_status` (2 bits)
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Level1Cue {
    /// Step regime tag.
    pub regime: RegimeTag,
    /// Binding / stressed axis indicator.
    pub binding_axis: BindingAxisTag,
    /// High-priority algedonic bypass indicator.
    pub algedonic: bool,
    /// Intake telemetry null-semantics indicator.
    pub codd_status: CoddStatusTag,
}

impl Level1Cue {
    /// Packs the cue into exactly 8 bits (`1` byte), well within the 10-bit ceiling.
    #[must_use]
    pub const fn pack(self) -> u8 {
        (self.regime as u8 & 0b111)
            | (((self.binding_axis as u8) & 0b11) << 3)
            | (if self.algedonic { 1 << 5 } else { 0 })
            | (((self.codd_status as u8) & 0b11) << 6)
    }

    /// Decodes an 8-bit packed cue back into validated enums.
    pub fn unpack(byte: u8) -> Result<Self, HostError> {
        let regime = RegimeTag::from_bits(byte & 0b111)?;
        let binding_axis = BindingAxisTag::from_bits((byte >> 3) & 0b11)?;
        let algedonic = ((byte >> 5) & 0b1) == 1;
        let codd_status = CoddStatusTag::from_bits((byte >> 6) & 0b11)?;
        Ok(Self {
            regime,
            binding_axis,
            algedonic,
            codd_status,
        })
    }
}

/// Level 2 off-shift diagnostic drilldown strictly separated from synchronous live triage.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Level2AuditDrilldown {
    /// Authenticated custody tape head summary.
    pub custody_heads: CustodyHeads,
    /// Retrospective ex-post triad/basin audit view.
    pub retrospective_audit: RetrospectiveAudit,
    /// Lock-free host metrics snapshot.
    pub metrics: MetricsSnapshot,
}
