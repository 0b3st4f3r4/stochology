//! The Three Canonical Operators of Part IV, re-exported from [`stoch_math`].
//!
//! Part IV runs the case through the sequential loop and the three filters; the
//! operators below live `#![no_std]` in `stoch-math` and produce margins and
//! durations strictly above the step: nothing here is an argument of `step`,
//! nothing reaches the acceptance path, and nothing folds the three coordinates
//! into one number.
//!
//! ```
//! use stoch_vectors::operators::UNIT;
//! use stoch_vectors::operators::epistemic::{compose_design_effects, effective_sample};
//!
//! let additive = compose_design_effects(&[2 * UNIT, 3 * UNIT]).unwrap();
//! let bound = stoch_vectors::operators::epistemic::design_effect_product_bound(
//!     &[2 * UNIT, 3 * UNIT],
//! )
//! .unwrap();
//! assert_eq!(additive, 4 * UNIT);
//! assert_eq!(bound, 6 * UNIT);
//! assert!(additive < bound);
//! assert_eq!(effective_sample(1000, additive).unwrap(), 250);
//!
//! use stoch_vectors::operators::governance::{differential_sign, DifferentialSign};
//! assert_eq!(differential_sign(UNIT, UNIT / 2).unwrap(), DifferentialSign::Positive);
//! assert_eq!(differential_sign(UNIT / 2, UNIT / 2).unwrap(), DifferentialSign::Neutral);
//! assert_eq!(differential_sign(UNIT / 2, UNIT / 2).unwrap().factor(), 0);
//! ```
//!
//! Isolation, proven at compile time: an operator answer cannot stand where a
//! step takes a state.
//!
//! ```compile_fail
//! use stoch_core::step;
//! use stoch_vectors::operators::vital::SurvivalOutcome;
//!
//! let _ = step(&SurvivalOutcome::RecoversInTime);
//! ```
//!
//! Nor where the acceptance path takes a transcript.
//!
//! ```compile_fail
//! use stoch_core::accept;
//! use stoch_vectors::operators::governance::Integrity;
//!
//! let integrity = Integrity::new(10, 10).unwrap();
//! let _ = accept(&integrity);
//! ```

pub use stoch_math::{epistemic, governance, vital, OperatorError};

/// The fixed-point unit shared with the discrete core.
pub const UNIT: i64 = stoch_core::SCALE;

const _: () = assert!(UNIT == stoch_math::UNIT);

/// The integer floor square root: the largest `r >= 0` with `r * r <= value`.
pub fn integer_root(value: i64) -> Result<i64, OperatorError> {
    stoch_math::integer_root_i64(value)
}
