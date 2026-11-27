//! Operator I — the epistemic margin (`43`–`45`).
//!
//! Provides fixed-point design-effect contraction, additive design-effect composition,
//! the multiplicative majorant bound, and Meng's non-response bias identity without
//! touching the live discrete step or acceptance paths.

use crate::{integer_root_i64, narrow_op, OperatorError, UNIT};

/// The declared ceiling on `sigma`: `1_000_000_000` (millionths).
///
/// With `sigma <= SIGMA_CAP` and `|rho| <= UNIT`, the numerator in
/// [`meng_bias_squared`] stays inside `i128` without overflow.
pub const SIGMA_CAP: i64 = 1_000_000_000;

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
    let num = i128::from(size)
        .checked_mul(i128::from(UNIT))
        .ok_or(OperatorError::Overflow)?;
    narrow_op(num / i128::from(design_effect))
}

/// The composition of several design effects.
///
/// Design effects compose additively as `UNIT + sum(d_k - UNIT)`. The product
/// `prod(d_k)` is only an upper bound on it, and [`design_effect_product_bound`]
/// returns that majorant — never the composition itself.
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
/// approximation of the product — it is not the product.
pub fn design_effect_product_bound(effects: &[i64]) -> Result<i64, OperatorError> {
    if effects.is_empty() {
        return Err(OperatorError::Empty("design effects"));
    }
    let mut product = i128::from(UNIT);
    for effect in effects {
        if *effect < UNIT {
            return Err(OperatorError::BelowUnit);
        }
        product = product
            .checked_mul(i128::from(*effect))
            .ok_or(OperatorError::Overflow)?
            / i128::from(UNIT);
        if product > i128::from(i64::MAX) {
            return Err(OperatorError::Overflow);
        }
    }
    narrow_op(product)
}

/// The declared non-response parameter of Meng's identity.
///
/// `rho` is the data-defect correlation scaled by [`UNIT`] and lying in
/// `[-UNIT, UNIT]`; `sigma` is the spread of the variable in the same units,
/// capped at [`SIGMA_CAP`] and refused above it; `response_fraction` is the
/// respondent share in `(0, UNIT]`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Nonresponse {
    rho: i64,
    sigma: i64,
    response_fraction: i64,
}

impl Nonresponse {
    /// Declares a validated non-response parameter tuple.
    pub fn new(
        rho: i64,
        sigma: i64,
        response_fraction: i64,
    ) -> Result<Nonresponse, OperatorError> {
        if !(-UNIT..=UNIT).contains(&rho) {
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

    /// The data-defect correlation, scaled by [`UNIT`].
    pub const fn rho(self) -> i64 {
        self.rho
    }

    /// The spread of the variable, in operator units.
    pub const fn sigma(self) -> i64 {
        self.sigma
    }

    /// The respondent share, scaled by [`UNIT`].
    pub const fn response_fraction(self) -> i64 {
        self.response_fraction
    }

    /// Returns `true` iff non-response is ignorable (`rho == 0`).
    pub const fn ignorable(self) -> bool {
        self.rho == 0
    }
}

/// The data-defect index `D = rho^2`, scaled by [`UNIT`] and **truncating**.
pub fn data_defect_index(parameter: Nonresponse) -> Result<i64, OperatorError> {
    let rho = i128::from(parameter.rho);
    narrow_op((rho * rho) / i128::from(UNIT))
}

/// The squared bias of Meng's identity, in squared units of `sigma`.
///
/// Computes `rho^2 * sigma^2 * (UNIT - f) / (UNIT^2 * f)` in `i128` using
/// checked multiplication and truncates once by integer division.
pub fn meng_bias_squared(parameter: Nonresponse) -> Result<i64, OperatorError> {
    let unit = i128::from(UNIT);
    let fraction = i128::from(parameter.response_fraction);
    let mut numerator = 1_i128;
    for factor in [
        i128::from(parameter.rho),
        i128::from(parameter.rho),
        i128::from(parameter.sigma),
        i128::from(parameter.sigma),
        unit - fraction,
    ] {
        numerator = numerator.checked_mul(factor).ok_or(OperatorError::Overflow)?;
    }
    let mut denominator = 1_i128;
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
    narrow_op(quotient)
}

/// The bias magnitude of Meng's identity, as an integer floor square root.
///
/// Satisfies `bias^2 <= meng_bias_squared < (bias + 1)^2`.
pub fn meng_bias(parameter: Nonresponse) -> Result<i64, OperatorError> {
    integer_root_i64(meng_bias_squared(parameter)?)
}

/// The bias carrying the sign of `rho`, giving correction direction.
pub fn signed_bias(parameter: Nonresponse) -> Result<i64, OperatorError> {
    let magnitude = meng_bias(parameter)?;
    Ok(if parameter.rho < 0 {
        -magnitude
    } else {
        magnitude
    })
}

/// The respondent level corrected for non-response: `respondent_level - signed_bias`.
///
/// Equals `respondent_level` whenever `rho == 0` (ignorable non-response).
pub fn corrected_level(
    respondent_level: i64,
    parameter: Nonresponse,
) -> Result<i64, OperatorError> {
    respondent_level
        .checked_sub(signed_bias(parameter)?)
        .ok_or(OperatorError::Overflow)
}

/// Exact integer cluster design effect under negative within-cluster correlation.
///
/// Computes `DEFF_cluster = UNIT + (cluster_size - 1) * rho_icc_scaled` for
/// `cluster_size >= 1` and `rho_icc_scaled` in `[-UNIT, 0]`, modeling
/// repulsive or determinantal sub-Poisson sampling (`Kulesza & Taskar 2012`,
/// `Ch. 59`). Refuses correlations at or below the Kish feasibility bound
/// `-1 / (cluster_size - 1)` where `DEFF_cluster <= 0`.
pub fn negative_icc_cluster_deff(
    cluster_size: i64,
    rho_icc_scaled: i64,
) -> Result<i64, OperatorError> {
    if cluster_size <= 0 {
        return Err(OperatorError::NonPositive("cluster_size"));
    }
    if rho_icc_scaled > 0 || rho_icc_scaled < -UNIT {
        return Err(OperatorError::OutOfRange(
            "negative_icc_cluster_deff: rho_icc outside [-UNIT, 0]",
        ));
    }
    let steps = i128::from(cluster_size)
        .checked_sub(1)
        .ok_or(OperatorError::Overflow)?;
    let delta = steps
        .checked_mul(i128::from(rho_icc_scaled))
        .ok_or(OperatorError::Overflow)?;
    let deff = i128::from(UNIT)
        .checked_add(delta)
        .ok_or(OperatorError::Overflow)?;
    if deff <= 0 {
        return Err(OperatorError::OutOfRange(
            "negative_icc_cluster_deff: rho_icc violates Kish lower bound",
        ));
    }
    narrow_op(deff)
}

/// The effective sample size under a sub-unit repulsive design effect.
///
/// Computes `n_eff = n * UNIT / design_effect` (truncating toward zero) when
/// `0 < design_effect <= UNIT`, complementing [`effective_sample`] for
/// negatively correlated or determinantal sampling without widening `effective_sample`.
pub fn repulsive_effective_sample(size: i64, design_effect: i64) -> Result<i64, OperatorError> {
    if size < 0 {
        return Err(OperatorError::OutOfRange(
            "repulsive_effective_sample: negative size",
        ));
    }
    if design_effect <= 0 {
        return Err(OperatorError::NonPositive(
            "repulsive_effective_sample: design_effect",
        ));
    }
    if design_effect > UNIT {
        return Err(OperatorError::OutOfRange(
            "repulsive_effective_sample: design_effect exceeds UNIT",
        ));
    }
    let num = i128::from(size)
        .checked_mul(i128::from(UNIT))
        .ok_or(OperatorError::Overflow)?;
    narrow_op(num / i128::from(design_effect))
}
