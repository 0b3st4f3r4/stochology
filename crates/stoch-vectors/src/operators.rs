//! The Three Canonical Operators of Part IV, as a declared integer layer.
//!
//! Part IV runs the case through the sequential loop and the three filters; the
//! operators below are the three transformations that *produce* the margins the
//! discrete core then consumes. They are a layer above the step, not part of it:
//! nothing here is an argument of `step`, nothing here reaches the acceptance
//! path, and nothing here folds the three coordinates into one number. The
//! Leontief minimum stays what the core does with a finished trajectory.
//!
//! Every function is integer-only, takes `1_000_000` as its unit, refuses values
//! that would leave the declared range, and states its truncation where a
//! division occurs. No constant here is read from the manuscript: a parameter is
//! a parameter, and where the book prints a threshold the caller passes it.
//!
//! ```
//! use stoch_vectors::operators::UNIT;
//! use stoch_vectors::operators::epistemic::{compose_design_effects, effective_sample};
//!
//! // The composition of two design effects is additive, and the product is only
//! // an upper bound on it.
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
//! // The governance coordinate fixes the sign; on the pivot the differential is
//! // zero, and the tie is a named arm rather than the permitted side.
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

use core::i64::MAX as I64_MAX;
use core::i64::MIN as I64_MIN;

/// The fixed-point unit shared with the discrete core.
pub const UNIT: i64 = stoch_core::SCALE;

/// Why an operator refused its arguments.
///
/// A refusal is not a verdict and not a regime: it is the layer saying that the
/// numbers it was handed do not lie in the domain it declared.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum OperatorError {
    /// An intermediate product left the declared range.
    Overflow,
    /// A value below the unit where the unit is the floor (a contraction < 1).
    BelowUnit,
    /// A value outside the interval the operator declared for it.
    OutOfRange(&'static str),
    /// An empty list where the operator needs at least one element.
    Empty(&'static str),
    /// A denominator that cannot carry the quotient.
    NonPositive(&'static str),
}

impl OperatorError {
    /// A stable code, negative so it never collides with a count.
    pub const fn code(self) -> i32 {
        match self {
            OperatorError::Overflow => -1,
            OperatorError::BelowUnit => -2,
            OperatorError::OutOfRange(_) => -3,
            OperatorError::Empty(_) => -4,
            OperatorError::NonPositive(_) => -5,
        }
    }
}

fn wide(value: i64) -> i128 {
    value as i128
}

fn narrow(value: i128) -> Result<i64, OperatorError> {
    if value > wide(I64_MAX) || value < wide(I64_MIN) {
        Err(OperatorError::Overflow)
    } else {
        Ok(value as i64)
    }
}

/// The integer square root: the largest `r` with `r * r <= value`.
///
/// Declared truncation, no approximation beyond it: `r * r <= value < (r + 1) * (r + 1)`.
/// This is how a square root enters an integer layer — never as a decimal.
pub fn integer_root(value: i64) -> Result<i64, OperatorError> {
    if value < 0 {
        return Err(OperatorError::OutOfRange("integer_root of a negative value"));
    }
    if value < 2 {
        return Ok(value);
    }
    let mut low: i64 = 1;
    let mut high: i64 = value;
    while low <= high {
        let middle = low + (high - low) / 2;
        if wide(middle) * wide(middle) <= wide(value) {
            low = middle + 1;
        } else {
            high = middle - 1;
        }
    }
    Ok(high)
}

/// Operator I — the epistemic margin (`43`–`45`).
pub mod epistemic {
    use super::{narrow, wide, OperatorError, UNIT};

    /// The effective sample after a design effect contracts it.
    ///
    /// `n_eff = n / deff`, computed as `n * UNIT / deff` with the division
    /// truncating toward zero (declared). A design effect below the unit is
    /// refused: an expansion is not a contraction and this operator only
    /// contracts.
    pub fn effective_sample(size: i64, design_effect: i64) -> Result<i64, OperatorError> {
        if size < 0 {
            return Err(OperatorError::OutOfRange("effective_sample: negative size"));
        }
        if design_effect < UNIT {
            return Err(OperatorError::BelowUnit);
        }
        narrow(wide(size) * wide(UNIT) / wide(design_effect))
    }

    /// The composition of several design effects.
    ///
    /// The composition is additive: `1 + Σ (d_k − 1)`. The product `Π d_k` is
    /// only an upper bound on it, and [`design_effect_product_bound`] is the
    /// function that returns that bound — never the composition.
    pub fn compose_design_effects(effects: &[i64]) -> Result<i64, OperatorError> {
        if effects.is_empty() {
            return Err(OperatorError::Empty("design effects"));
        }
        let mut composed = UNIT;
        for effect in effects {
            if *effect < UNIT {
                return Err(OperatorError::BelowUnit);
            }
            composed = composed
                .checked_add(effect - UNIT)
                .ok_or(OperatorError::Overflow)?;
        }
        Ok(composed)
    }

    /// The upper bound: the effects multiplied, never the composition itself.
    ///
    /// Scaled by the unit at every step and **truncating at every step** (declared):
    /// the result is an upper bound on the additive composition and at most the
    /// exact product, so it is a majorant of the composition and a lower
    /// approximation of the product — it is not the product. The chain multiplies
    /// with [`i128::checked_mul`] and refuses an intermediate that leaves the wide
    /// range with [`OperatorError::Overflow`].
    ///
    /// The bound can equal the composition while more than one effect exceeds the
    /// unit: the excess of the product over the additive term is then smaller than
    /// the unit that the divisions truncate away. `[UNIT + 1, UNIT + 1]` is such a
    /// list — the excess is `1 / UNIT` — and the two coincide there.
    pub fn design_effect_product_bound(effects: &[i64]) -> Result<i64, OperatorError> {
        if effects.is_empty() {
            return Err(OperatorError::Empty("design effects"));
        }
        let mut product = wide(UNIT);
        for effect in effects {
            if *effect < UNIT {
                return Err(OperatorError::BelowUnit);
            }
            product = product
                .checked_mul(wide(*effect))
                .ok_or(OperatorError::Overflow)?
                / wide(UNIT);
            if product > wide(i64::MAX) {
                return Err(OperatorError::Overflow);
            }
        }
        narrow(product)
    }

    /// The declared ceiling on `sigma`: the same absolute bound the core declares.
    ///
    /// The cap is not cosmetic. `meng_bias_squared` squares `sigma`, and an
    /// unbounded `sigma` would let the wide product leave `i128` and wrap a square
    /// into a negative number. With `sigma <= SIGMA_CAP` and `|rho| <= UNIT` the
    /// largest numerator is `1e6 * 1e6 * 1e9 * 1e9 * 1e6 = 1e36`, inside `i128`.
    ///
    /// The value is the core's declared absolute ceiling (`MAX_ABS`), written as a
    /// literal so that the operator layer reads exactly one symbol from the core —
    /// the unit — and no path from here into the core's types exists to trace.
    pub const SIGMA_CAP: i64 = 1_000_000_000;

    /// The declared non-response parameter of Meng's identity.
    ///
    /// `rho` is the data-defect correlation scaled by the unit and lying in
    /// `[-UNIT, UNIT]`; `sigma` is the spread of the variable in the same units,
    /// capped at [`SIGMA_CAP`] (the core's declared absolute ceiling) and refused
    /// above it; `response_fraction` is the respondent share, in `(0, UNIT]`. The
    /// three are declared by the caller — the operator invents none of them.
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub struct Nonresponse {
        rho: i64,
        sigma: i64,
        response_fraction: i64,
    }

    impl Nonresponse {
        /// Declare the non-response parameter.
        pub fn new(
            rho: i64,
            sigma: i64,
            response_fraction: i64,
        ) -> Result<Nonresponse, OperatorError> {
            if rho < -UNIT || rho > UNIT {
                return Err(OperatorError::OutOfRange("rho outside [-1, 1]"));
            }
            if sigma < 0 {
                return Err(OperatorError::OutOfRange("negative sigma"));
            }
            if sigma > SIGMA_CAP {
                return Err(OperatorError::OutOfRange("sigma above the declared cap"));
            }
            if response_fraction <= 0 || response_fraction > UNIT {
                return Err(OperatorError::OutOfRange(
                    "response fraction outside (0, 1]",
                ));
            }
            Ok(Nonresponse {
                rho,
                sigma,
                response_fraction,
            })
        }

        /// The data-defect correlation, scaled by the unit.
        pub const fn rho(self) -> i64 {
            self.rho
        }

        /// The spread of the variable, in the operator's units.
        pub const fn sigma(self) -> i64 {
            self.sigma
        }

        /// The respondent share, scaled by the unit.
        pub const fn response_fraction(self) -> i64 {
            self.response_fraction
        }

        /// Whether non-response is ignorable, which is exactly `rho == 0`.
        pub const fn ignorable(self) -> bool {
            self.rho == 0
        }
    }

    /// The data-defect index `D = rho^2`, scaled by the unit and **truncating**.
    ///
    /// `rho * rho / UNIT`, divided toward zero, so `rho = 1001` returns `1` against
    /// the exact `1.002001`; the truncation is declared here, as it is in
    /// [`effective_sample`], [`design_effect_product_bound`],
    /// [`super::vital::volatility_drag`], [`meng_bias_squared`] and
    /// [`super::governance::Integrity::ratio`].
    pub fn data_defect_index(parameter: Nonresponse) -> Result<i64, OperatorError> {
        let rho = wide(parameter.rho);
        narrow(rho * rho / wide(UNIT))
    }

    /// The squared bias of Meng's identity, in squared units of `sigma`.
    ///
    /// With `f` the response fraction and `rho` the defect correlation,
    /// `bias = rho * sigma * sqrt((1 - f) / f)`, so
    ///
    /// ```text
    /// bias^2 = rho^2 * sigma^2 * (1 - f) / (UNIT^2 * f)
    /// ```
    ///
    /// computed in a wide integer with a checked chain and truncated once by the
    /// division (declared). The identity is exact in this squared domain; the
    /// square root below is the only other place where truncation is declared.
    ///
    /// Every factor is multiplied with [`i128::checked_mul`], and a quotient below
    /// zero is refused with [`OperatorError::OutOfRange`]: a square is not negative,
    /// so a negative result could only be a wrapped value, and no wrap is allowed to
    /// leave this function as an answer. `Nonresponse::new` caps `sigma` so that the
    /// chain stays inside the wide range.
    pub fn meng_bias_squared(parameter: Nonresponse) -> Result<i64, OperatorError> {
        let unit = wide(UNIT);
        let fraction = wide(parameter.response_fraction);
        let mut numerator = wide(1);
        for factor in [
            wide(parameter.rho),
            wide(parameter.rho),
            wide(parameter.sigma),
            wide(parameter.sigma),
            unit - fraction,
        ] {
            numerator = numerator.checked_mul(factor).ok_or(OperatorError::Overflow)?;
        }
        let mut denominator = wide(1);
        for factor in [unit, unit, fraction] {
            denominator = denominator.checked_mul(factor).ok_or(OperatorError::Overflow)?;
        }
        if numerator < 0 || denominator <= 0 {
            return Err(OperatorError::OutOfRange("a square cannot be negative"));
        }
        let quotient = numerator / denominator;
        if quotient < 0 {
            return Err(OperatorError::OutOfRange("a square cannot be negative"));
        }
        narrow(quotient)
    }

    /// The bias magnitude of Meng's identity, as an integer square root.
    ///
    /// `bias * bias <= meng_bias_squared < (bias + 1) * (bias + 1)` — the declared
    /// truncation, and the only one. Zero exactly when `rho` is zero.
    pub fn meng_bias(parameter: Nonresponse) -> Result<i64, OperatorError> {
        super::integer_root(meng_bias_squared(parameter)?)
    }

    /// The bias with the sign of `rho`, so the correction knows its direction.
    pub fn signed_bias(parameter: Nonresponse) -> Result<i64, OperatorError> {
        let magnitude = meng_bias(parameter)?;
        Ok(if parameter.rho < 0 { -magnitude } else { magnitude })
    }

    /// The respondent level corrected for non-response: `level - signed_bias`.
    ///
    /// When non-response is ignorable the correction is the identity, which is
    /// the clause Meng's identity makes explicit: a defect correlation of zero is
    /// the only case in which no correction is owed.
    pub fn corrected_level(
        respondent_level: i64,
        parameter: Nonresponse,
    ) -> Result<i64, OperatorError> {
        respondent_level
            .checked_sub(signed_bias(parameter)?)
            .ok_or(OperatorError::Overflow)
    }
}

/// Operator II — the vital margin (`46`–`47`).
pub mod vital {
    use super::{narrow, wide, OperatorError, UNIT};

    /// The reserve dynamics of one trajectory: what comes in, what goes out, and
    /// how heavy the tail is (its scale, in the operator's units).
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub struct ReserveDynamics {
        inflow: i64,
        outflow: i64,
        tail_scale: i64,
    }

    impl ReserveDynamics {
        /// Declare the dynamics. All three are non-negative: a negative inflow is
        /// a different model, not a smaller one.
        pub fn new(inflow: i64, outflow: i64, tail_scale: i64) -> Result<Self, OperatorError> {
            if inflow < 0 || outflow < 0 || tail_scale < 0 {
                return Err(OperatorError::OutOfRange("negative reserve dynamics"));
            }
            Ok(ReserveDynamics {
                inflow,
                outflow,
                tail_scale,
            })
        }

        /// What the reserve gains per period.
        pub const fn inflow(self) -> i64 {
            self.inflow
        }

        /// What the reserve loses per period.
        pub const fn outflow(self) -> i64 {
            self.outflow
        }

        /// The scale of the tail the reserve is exposed to.
        pub const fn tail_scale(self) -> i64 {
            self.tail_scale
        }

        /// The net drift per period, which may be negative.
        pub fn net(self) -> Result<i64, OperatorError> {
            self.inflow
                .checked_sub(self.outflow)
                .ok_or(OperatorError::Overflow)
        }
    }

    /// The volatility drag of a multiplicative reserve: `sigma^2 / 2`.
    ///
    /// Truncating division by `2 * UNIT`, declared. It is the second-order term
    /// that separates the time growth of one path from the growth of the
    /// cross-section; it is computed, never quoted.
    pub fn volatility_drag(tail_scale: i64) -> Result<i64, OperatorError> {
        if tail_scale < 0 {
            return Err(OperatorError::OutOfRange("negative tail scale"));
        }
        narrow(wide(tail_scale) * wide(tail_scale) / (2 * wide(UNIT)))
    }

    /// The growth of one long path: the drift less the drag.
    pub fn time_growth(drift: i64, tail_scale: i64) -> Result<i64, OperatorError> {
        drift
            .checked_sub(volatility_drag(tail_scale)?)
            .ok_or(OperatorError::Overflow)
    }

    /// The growth of the cross-section: the drift itself.
    ///
    /// Under a heavy tail the two differ, and that difference is what makes the
    /// reserve dynamics non-ergodic: the expectation climbs while a single path
    /// declines.
    pub const fn ensemble_growth(drift: i64) -> i64 {
        drift
    }

    /// The gap between the cross-section and the path, which is the drag.
    pub fn non_ergodic_gap(drift: i64, tail_scale: i64) -> Result<i64, OperatorError> {
        narrow(wide(ensemble_growth(drift)) - wide(time_growth(drift, tail_scale)?))
    }

    /// The reserve after `periods` of the declared net drift.
    pub fn reserve_at(
        dynamics: ReserveDynamics,
        initial: i64,
        periods: u32,
    ) -> Result<i64, OperatorError> {
        narrow(wide(initial) + wide(i64::from(periods)) * wide(dynamics.net()?))
    }

    /// The reserve after a tail draw of `sigmas` tail scales.
    pub fn reserve_after_tail(
        dynamics: ReserveDynamics,
        reserve: i64,
        sigmas: i64,
    ) -> Result<i64, OperatorError> {
        if sigmas < 0 {
            return Err(OperatorError::OutOfRange("negative tail draw"));
        }
        narrow(wide(reserve) - wide(sigmas) * wide(dynamics.tail_scale))
    }

    /// Whether one tail draw at `sigmas` scales takes the reserve below zero.
    pub fn ruined_by_tail(
        dynamics: ReserveDynamics,
        reserve: i64,
        sigmas: i64,
    ) -> Result<bool, OperatorError> {
        Ok(reserve_after_tail(dynamics, reserve, sigmas)? < 0)
    }

    /// The first period in which the reserve reaches `target`, or `None` when it
    /// never does inside `horizon`.
    ///
    /// The crossing is found by a monotone search over `reserve_at`, which is
    /// monotone in the period because the net drift is fixed: the answer is the
    /// least `t` with `reserve_at(t) >= target`, and the search is a bisection,
    /// not an iteration of the action step and not a fixed point of it. Existence
    /// is decided first (`reserve_at(horizon) >= target`), so a `None` means the
    /// crossing does not exist inside the declared horizon rather than that the
    /// search failed.
    pub fn recovery_time(
        dynamics: ReserveDynamics,
        initial: i64,
        target: i64,
        horizon: u32,
    ) -> Result<Option<u32>, OperatorError> {
        if initial >= target {
            return Ok(Some(0));
        }
        if dynamics.net()? <= 0 {
            return Ok(None);
        }
        if reserve_at(dynamics, initial, horizon)? < target {
            return Ok(None);
        }
        let mut low: u32 = 1;
        let mut high: u32 = horizon;
        while low < high {
            let middle = low + (high - low) / 2;
            if reserve_at(dynamics, initial, middle)? >= target {
                high = middle;
            } else {
                low = middle + 1;
            }
        }
        Ok(Some(low))
    }

    /// `TTS_survival >= TTR_recovery`, as a predicate outside the step.
    ///
    /// Both arguments are durations in the same declared unit. The comparison is
    /// a statement about a trajectory, not an input to an action.
    pub const fn survival_condition(tts_survival: i64, ttr_recovery: i64) -> bool {
        tts_survival >= ttr_recovery
    }

    /// The three named outcomes of the survival question.
    ///
    /// A crossing either exists inside the horizon or it does not; when it
    /// exists, it either arrives while the reserve still lasts or after ruin.
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub enum SurvivalOutcome {
        /// The crossing exists and `TTS_survival >= TTR_recovery`.
        RecoversInTime,
        /// The crossing exists but arrives after the reserve is exhausted.
        CrossesAfterRuin,
        /// No crossing exists inside the declared horizon.
        NoCrossingWithinHorizon,
    }

    impl SurvivalOutcome {
        /// The wire tag, `0 .. 2`.
        pub const fn tag(self) -> u8 {
            match self {
                SurvivalOutcome::RecoversInTime => 0,
                SurvivalOutcome::CrossesAfterRuin => 1,
                SurvivalOutcome::NoCrossingWithinHorizon => 2,
            }
        }
    }

    /// Classify the survival question from a duration and a crossing.
    pub fn survival_outcome(tts_survival: i64, ttr_recovery: Option<i64>) -> SurvivalOutcome {
        match ttr_recovery {
            None => SurvivalOutcome::NoCrossingWithinHorizon,
            Some(ttr) if survival_condition(tts_survival, ttr) => {
                SurvivalOutcome::RecoversInTime
            }
            Some(_) => SurvivalOutcome::CrossesAfterRuin,
        }
    }
}

/// Operator III — the governance margin (`48`–`49`).
pub mod governance {
    use super::{narrow, wide, OperatorError, UNIT};

    /// The three temporal readings of the governance coordinate.
    ///
    /// These are three readings of *one* coordinate — before, at the moment, and
    /// after — not three coordinates. Their minimum is a Leontief fold over time,
    /// which is why it cannot stand in for a fold over the three axes.
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub struct Bitemporal {
        ex_ante: i64,
        kairos: i64,
        ex_post: i64,
    }

    impl Bitemporal {
        /// Declare the three readings, each in `[0, UNIT]`.
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

        /// `min(g_ex_ante, g_kairos, g_ex_post)`.
        pub fn responsibility(self) -> i64 {
            self.ex_ante.min(self.kairos).min(self.ex_post)
        }

        /// The reading before the act.
        pub const fn ex_ante(self) -> i64 {
            self.ex_ante
        }

        /// The reading at the moment of the act.
        pub const fn kairos(self) -> i64 {
            self.kairos
        }

        /// The reading after the act.
        pub const fn ex_post(self) -> i64 {
            self.ex_post
        }
    }

    /// The performative-integrity gate: what was committed against what was done.
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub struct Integrity {
        commitments: i64,
        discharges: i64,
    }

    impl Integrity {
        /// Declare the gate. Commitments must be positive; discharges non-negative.
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

        /// The discharge ratio, scaled by the unit, truncating.
        ///
        /// A ratio above one is possible and is not clamped: over-discharge is a
        /// fact of the record, and clamping it would hide the very asymmetry the
        /// gate exists to show.
        pub fn ratio(self) -> Result<i64, OperatorError> {
            narrow(wide(self.discharges) * wide(UNIT) / wide(self.commitments))
        }

        /// Whether the ratio reaches the floor the caller declares.
        pub fn gate(self, floor: i64) -> Result<bool, OperatorError> {
            Ok(self.ratio()? >= floor)
        }
    }

    /// The sign of the differential, which the governance coordinate fixes.
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub enum DifferentialSign {
        /// The differential runs in the permitted direction.
        Positive,
        /// The differential runs against the common reserve.
        Negative,
        /// The coordinate sits exactly on the pivot: the differential is zero, and
        /// zero is neither permitted nor against. The arm exists so that the tie is
        /// named rather than silently counted as permission.
        Neutral,
    }

    impl DifferentialSign {
        /// `+1`, `-1`, or `0` at the tie.
        pub const fn factor(self) -> i64 {
            match self {
                DifferentialSign::Positive => 1,
                DifferentialSign::Negative => -1,
                DifferentialSign::Neutral => 0,
            }
        }

        /// The stable tag: `1`, `-1`, or `0` at the tie.
        pub const fn tag(self) -> i8 {
            self.factor() as i8
        }
    }

    /// The sign the governance coordinate fixes, given the pivot the caller declares.
    ///
    /// The sign is read from the coordinate's position relative to the pivot and
    /// from nothing else: it does not move with the magnitude, and no other
    /// coordinate participates. This is the structural difference between a
    /// coordinate that sets a sign and a reading that enters a fold.
    ///
    /// The three arms follow from that position: `governance > pivot` is
    /// [`DifferentialSign::Positive`], `governance < pivot` is
    /// [`DifferentialSign::Negative`], and `governance == pivot` is
    /// [`DifferentialSign::Neutral`] with factor `0` — the differential is zero
    /// there, so it is neither permitted nor against. The caller reads the tie from
    /// the named arm; it is never folded into the positive side.
    pub fn differential_sign(
        governance: i64,
        pivot: i64,
    ) -> Result<DifferentialSign, OperatorError> {
        if !(0..=UNIT).contains(&governance) {
            return Err(OperatorError::OutOfRange("governance outside [0, 1]"));
        }
        if pivot <= 0 || pivot >= UNIT {
            return Err(OperatorError::OutOfRange("pivot outside (0, 1)"));
        }
        Ok(match governance.cmp(&pivot) {
            core::cmp::Ordering::Greater => DifferentialSign::Positive,
            core::cmp::Ordering::Less => DifferentialSign::Negative,
            core::cmp::Ordering::Equal => DifferentialSign::Neutral,
        })
    }

    /// The differential with its sign set by the governance coordinate.
    ///
    /// `magnitude >= 0`; the returned value is `+magnitude`, `-magnitude`, or `0`
    /// at the tie, according to [`differential_sign`]. The magnitude never chooses
    /// the sign, and the coordinate never chooses the magnitude.
    pub fn signed_differential(
        magnitude: i64,
        governance: i64,
        pivot: i64,
    ) -> Result<i64, OperatorError> {
        if magnitude < 0 {
            return Err(OperatorError::OutOfRange("negative magnitude"));
        }
        let sign = differential_sign(governance, pivot)?;
        magnitude.checked_mul(sign.factor()).ok_or(OperatorError::Overflow)
    }
}

#[cfg(test)]
mod tests {
    use super::epistemic::{
        compose_design_effects, data_defect_index, design_effect_product_bound, meng_bias_squared,
        Nonresponse, SIGMA_CAP,
    };
    use super::governance::{differential_sign, signed_differential, DifferentialSign};
    use super::{OperatorError, UNIT};

    fn cap_parameter() -> Nonresponse {
        Nonresponse::new(UNIT, SIGMA_CAP, UNIT / 2).unwrap()
    }

    #[test]
    fn sigma_at_the_cap_is_accepted_and_above_it_is_refused() {
        let at_cap = cap_parameter();
        assert_eq!(at_cap.sigma(), SIGMA_CAP);
        let squared = meng_bias_squared(at_cap).unwrap();
        assert!(squared >= 0, "a square is not negative");
        // The largest admissible numerator stays inside the wide range.
        assert!(squared > 0);

        for sigma in [SIGMA_CAP + 1, SIGMA_CAP * 2, 26_000_000_000] {
            let refusal = Nonresponse::new(UNIT, sigma, UNIT / 2).unwrap_err();
            assert_eq!(refusal.code(), -3, "sigma {sigma} must be refused at the gate");
        }
    }

    #[test]
    fn the_squared_bias_never_returns_a_wrapped_value() {
        // The reviewer's reproduction: 26e9 used to wrap into a negative square.
        assert_eq!(
            Nonresponse::new(UNIT, 26_000_000_000, UNIT / 2)
                .unwrap_err()
                .code(),
            -3
        );
        // A corner whose quotient leaves the narrow range is refused, never wrapped.
        let extreme = Nonresponse::new(UNIT, SIGMA_CAP, 1).unwrap();
        assert_eq!(meng_bias_squared(extreme).unwrap_err(), OperatorError::Overflow);
        // A representative corner stays finite and non-negative.
        assert_eq!(meng_bias_squared(cap_parameter()).unwrap(), 1_000_000_000_000_000_000);
        assert!(meng_bias_squared(cap_parameter()).unwrap() >= 0);
    }

    #[test]
    fn the_bound_truncates_every_step_and_is_not_the_product() {
        // Two effects above the unit still coincide with the composition: the
        // excess of the product over the linear term is 1 / UNIT and the division
        // truncates it away. This is the list that falsified the old sentence.
        let tied = [UNIT + 1, UNIT + 1];
        assert_eq!(compose_design_effects(&tied).unwrap(), UNIT + 2);
        assert_eq!(design_effect_product_bound(&tied).unwrap(), UNIT + 2);
        // Strictly above the composition where the excess survives truncation.
        assert!(design_effect_product_bound(&[2 * UNIT, 3 * UNIT]).unwrap() > 4 * UNIT);
        // And the chain refuses rather than wrapping.
        assert_eq!(
            design_effect_product_bound(&[UNIT, i64::MAX / 2, i64::MAX / 2])
                .unwrap_err(),
            OperatorError::Overflow
        );
    }

    #[test]
    fn the_defect_index_truncates_and_says_so() {
        let parameter = Nonresponse::new(1001, 1, UNIT).unwrap();
        // 1001 * 1001 / 1_000_000 = 1.002001 truncated to 1.
        assert_eq!(data_defect_index(parameter).unwrap(), 1);
        let exact = Nonresponse::new(1000, 1, UNIT).unwrap();
        assert_eq!(data_defect_index(exact).unwrap(), 1);
    }

    #[test]
    fn the_pivot_is_a_named_tie_not_the_permitted_side() {
        assert_eq!(
            differential_sign(UNIT / 2, UNIT / 2).unwrap(),
            DifferentialSign::Neutral
        );
        assert_eq!(differential_sign(UNIT / 2, UNIT / 2).unwrap().factor(), 0);
        assert_eq!(signed_differential(7, UNIT / 2, UNIT / 2).unwrap(), 0);
        assert_eq!(
            differential_sign(UNIT / 2 + 1, UNIT / 2).unwrap(),
            DifferentialSign::Positive
        );
        assert_eq!(
            differential_sign(UNIT / 2 - 1, UNIT / 2).unwrap(),
            DifferentialSign::Negative
        );
    }
}
