//! Operator III — the governance margin (`48`–`49`).
//!
//! Temporal responsibility folding over one coordinate, performative commitment
//! vs. discharge gating, and caller-pivoted differential signs.

use core::cmp::Ordering;

use crate::{narrow_op, OperatorError, UNIT};

/// Three temporal readings of the governance coordinate (before, at the moment,
/// and after). Their minimum is a temporal responsibility fold over one axis.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Bitemporal {
    ex_ante: i64,
    kairos: i64,
    ex_post: i64,
}

impl Bitemporal {
    /// Declares three bounded temporal readings in `[0, UNIT]`.
    pub fn new(ex_ante: i64, kairos: i64, ex_post: i64) -> Result<Self, OperatorError> {
        for value in [ex_ante, kairos, ex_post] {
            if !(0..=UNIT).contains(&value) {
                return Err(OperatorError::OutOfRange("temporal reading outside [0, 1]"));
            }
        }
        Ok(Bitemporal {
            ex_ante,
            kairos,
            ex_post,
        })
    }

    /// Returns `min(ex_ante, kairos, ex_post)`.
    pub fn responsibility(self) -> i64 {
        self.ex_ante.min(self.kairos).min(self.ex_post)
    }

    /// Reading recorded before the act.
    pub const fn ex_ante(self) -> i64 {
        self.ex_ante
    }

    /// Reading recorded at the moment of action.
    pub const fn kairos(self) -> i64 {
        self.kairos
    }

    /// Reading recorded after the act.
    pub const fn ex_post(self) -> i64 {
        self.ex_post
    }
}

/// Performative-integrity gate comparing committed obligations with discharges.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Integrity {
    commitments: i64,
    discharges: i64,
}

impl Integrity {
    /// Declares positive `commitments` and non-negative `discharges`.
    pub fn new(commitments: i64, discharges: i64) -> Result<Self, OperatorError> {
        if commitments <= 0 {
            return Err(OperatorError::NonPositive("commitments"));
        }
        if discharges < 0 {
            return Err(OperatorError::OutOfRange("negative discharges"));
        }
        Ok(Integrity {
            commitments,
            discharges,
        })
    }

    /// Truncating discharge ratio `discharges * UNIT / commitments` (unclamped above `UNIT`).
    pub fn ratio(self) -> Result<i64, OperatorError> {
        let num = i128::from(self.discharges)
            .checked_mul(i128::from(UNIT))
            .ok_or(OperatorError::Overflow)?;
        narrow_op(num / i128::from(self.commitments))
    }

    /// Returns `true` iff [`Integrity::ratio`] meets or exceeds `floor`.
    pub fn gate(self, floor: i64) -> Result<bool, OperatorError> {
        Ok(self.ratio()? >= floor)
    }
}

/// Sign of the differential determined solely by the governance coordinate.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DifferentialSign {
    /// Differential points in the permitted direction.
    Positive,
    /// Differential opposes the common reserve.
    Negative,
    /// Exact tie at the declared pivot (`0`, neither positive nor negative).
    Neutral,
}

impl DifferentialSign {
    /// Sign factor (`1`, `-1`, or `0` on tie).
    pub const fn factor(self) -> i64 {
        match self {
            DifferentialSign::Positive => 1,
            DifferentialSign::Negative => -1,
            DifferentialSign::Neutral => 0,
        }
    }

    /// Compact `i8` tag (`1`, `-1`, or `0` on tie).
    pub const fn tag(self) -> i8 {
        match self {
            DifferentialSign::Positive => 1,
            DifferentialSign::Negative => -1,
            DifferentialSign::Neutral => 0,
        }
    }
}

/// Determines the differential sign from `governance` relative to a caller-declared `pivot`.
pub fn differential_sign(governance: i64, pivot: i64) -> Result<DifferentialSign, OperatorError> {
    if !(0..=UNIT).contains(&governance) {
        return Err(OperatorError::OutOfRange("governance outside [0, 1]"));
    }
    if pivot <= 0 || pivot >= UNIT {
        return Err(OperatorError::OutOfRange("pivot outside (0, 1)"));
    }
    Ok(match governance.cmp(&pivot) {
        Ordering::Greater => DifferentialSign::Positive,
        Ordering::Less => DifferentialSign::Negative,
        Ordering::Equal => DifferentialSign::Neutral,
    })
}

/// Applies the governance-determined sign to a non-negative `magnitude`.
pub fn signed_differential(
    magnitude: i64,
    governance: i64,
    pivot: i64,
) -> Result<i64, OperatorError> {
    if magnitude < 0 {
        return Err(OperatorError::OutOfRange("negative magnitude"));
    }
    let sign = differential_sign(governance, pivot)?;
    magnitude
        .checked_mul(sign.factor())
        .ok_or(OperatorError::Overflow)
}
