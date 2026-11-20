//! Fixed-point epistemic, spectral, and delay-stability pre-operators (`#![no_std]`).
//!
//! This crate preserves `stoch-core` untouched while providing pure integer-only
//! pre-operators at scale [`UNIT`] (`1_000_000` = `1.0`): exact integer square
//! roots, Codd 4-state relational values, Stevens measurement scales, Meng + `DEFF`
//! interval widening before sealing, Lyapunov predictability horizons, the
//! Wirtinger--RCC time-varying delay bound, and Anderson--Morley 1-hop graph
//! utilization.
//!
//! No binary floating-point types or `unsafe` regions exist in this crate.

#![no_std]
#![deny(unsafe_code)]
#![deny(missing_docs)]
#![deny(clippy::float_arithmetic)]
#![forbid(clippy::cast_possible_truncation)]

#[cfg(test)]
extern crate std;

/// Fixed-point unit corresponding to `1.0` (millionths).
pub const UNIT: i64 = 1_000_000;

/// Domain or arithmetic overflow errors returned by `stoch-math`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MathError {
    /// Input parameter lies outside the declared mathematical domain.
    InvalidDomain(&'static str),
    /// Fixed-point multiplication, addition, or narrowing exceeded integer bounds.
    Overflow,
}

fn narrow(value: i128) -> Result<i64, MathError> {
    i64::try_from(value).map_err(|_| MathError::Overflow)
}

/// Exact integer floor square root: greatest `r >= 0` with `r * r <= value`.
///
/// ```
/// use stoch_math::{integer_root, MathError};
///
/// assert_eq!(integer_root(0), Ok(0));
/// assert_eq!(integer_root(2_000_000_000_000), Ok(1_414_213));
/// assert!(matches!(integer_root(-1), Err(MathError::InvalidDomain(_))));
/// ```
pub fn integer_root(value: i128) -> Result<i128, MathError> {
    if value < 0 {
        return Err(MathError::InvalidDomain("integer_root of negative value"));
    }
    if value < 2 {
        return Ok(value);
    }
    let mut low: i128 = 1;
    let mut high: i128 = if value > i128::from(i64::MAX) {
        (1_i128 << 64) - 1
    } else {
        value
    };
    while low <= high {
        let mid = low + (high - low) / 2;
        if let Some(sq) = mid.checked_mul(mid) {
            if sq <= value {
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        } else {
            high = mid - 1;
        }
    }
    Ok(high)
}

fn integer_root_ceil(value: i128) -> Result<i128, MathError> {
    let r = integer_root(value)?;
    let sq = r.checked_mul(r).ok_or(MathError::Overflow)?;
    if sq < value {
        r.checked_add(1).ok_or(MathError::Overflow)
    } else {
        Ok(r)
    }
}

/// Four-state Codd relational semantics separating observed values from missingness
/// (never coercing applicable unknown, inapplicable, or unaskable entries to zero).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CoddValue {
    /// Measured domain value (including a genuinely measured `0`).
    Known(i64),
    /// A-mark: attribute applies to the unit, yet no reading was captured.
    ApplicableUnknown,
    /// I-mark: attribute is declared in the table, yet inapplicable to this unit.
    Inapplicable,
    /// Pre-schema exclusion: unit sits outside every declared frame (`pi_i == 0`).
    NotAskable,
}

impl CoddValue {
    /// Returns the measured numeric value only when [`CoddValue::Known`].
    pub const fn known(self) -> Option<i64> {
        match self {
            CoddValue::Known(value) => Some(value),
            CoddValue::ApplicableUnknown | CoddValue::Inapplicable | CoddValue::NotAskable => None,
        }
    }

    /// Returns `true` only for [`CoddValue::Known`].
    pub const fn is_known(self) -> bool {
        matches!(self, CoddValue::Known(_))
    }
}

/// Validated closed fixed-point interval `[lo, hi]`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Interval {
    /// Lower endpoint in fixed-point units.
    pub lo: i64,
    /// Upper endpoint in fixed-point units.
    pub hi: i64,
}

impl Interval {
    /// Constructs a closed interval `[lo, hi]` after validating `lo <= hi`.
    pub const fn new(lo: i64, hi: i64) -> Result<Self, MathError> {
        if lo > hi {
            Err(MathError::InvalidDomain("inverted interval"))
        } else {
            Ok(Self { lo, hi })
        }
    }
}

/// Stevens measurement scale level gating arithmetic operations against ordinal averaging.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ScaleKind {
    /// Nominal classification: equality and distinctness only.
    Nominal,
    /// Ordinal rank: monotone order only, never arithmetic means.
    Ordinal,
    /// Interval scale: meaningful differences with arbitrary zero.
    IntervalScale,
    /// Ratio scale: true zero and meaningful ratios.
    Ratio,
}

impl ScaleKind {
    /// Returns `true` iff arithmetic means are mathematically permissible on this scale.
    pub const fn permits_arithmetic_mean(self) -> bool {
        matches!(self, ScaleKind::IntervalScale | ScaleKind::Ratio)
    }
}

/// Widens a raw interval `[lo, hi]` conservatively by worst-case Meng selection
/// bias `meng_bias_abs`, total cluster/weighting design effect `deff_total >= UNIT`,
/// and testimonial/witness loss `core_loss_margin` prior to sealing core bounds.
///
/// Rounds uncertainty expansion upward (never inward) and refuses negative biases,
/// `deff_total < UNIT`, or inverted intervals.
pub fn widen_before_seal(
    raw: Interval,
    meng_bias_abs: i64,
    deff_total: i64,
    core_loss_margin: i64,
) -> Result<Interval, MathError> {
    if raw.lo > raw.hi || meng_bias_abs < 0 || deff_total < UNIT || core_loss_margin < 0 {
        return Err(MathError::InvalidDomain("invalid epistemic discount parameters"));
    }
    let unit = i128::from(UNIT);
    let width = i128::from(raw.hi) - i128::from(raw.lo);
    let sqrt_deff = integer_root_ceil(i128::from(deff_total) * unit)?;
    let scale_excess = sqrt_deff - unit;
    let deff_half_expand = if width > 0 && scale_excess > 0 {
        let num = width.checked_mul(scale_excess).ok_or(MathError::Overflow)?;
        let den = 2 * unit;
        (num.checked_add(den - 1).ok_or(MathError::Overflow)?) / den
    } else {
        0
    };
    let extra = narrow(
        i128::from(meng_bias_abs) + i128::from(core_loss_margin) + deff_half_expand,
    )?;
    let lo = raw.lo.checked_sub(extra).ok_or(MathError::Overflow)?;
    let hi = raw.hi.checked_add(extra).ok_or(MathError::Overflow)?;
    Interval::new(lo, hi)
}

/// Discrete Lyapunov predictability horizon `floor(ln_ratio_scaled * UNIT / lambda_1_scaled)`
/// in fixed-point time units (`UNIT = 1_000_000`).
pub fn lyapunov_horizon_scaled(
    ln_ratio_scaled: i64,
    lambda_1_scaled: i64,
) -> Result<i64, MathError> {
    if ln_ratio_scaled < 0 || lambda_1_scaled <= 0 {
        return Err(MathError::InvalidDomain(
            "non-positive Lyapunov exponent or negative log ratio",
        ));
    }
    narrow((i128::from(ln_ratio_scaled) * i128::from(UNIT)) / i128::from(lambda_1_scaled))
}

/// Closed-form Wirtinger--RCC maximum admissible time-varying delay bound
/// in fixed-point units (`UNIT = 1_000_000`).
///
/// All scalar arguments are scaled by [`UNIT`]:
/// - `mu_plus` in `0 .. UNIT`: upper delay-derivative rate bound.
/// - `mu_minus >= 0`: reverse delay-rate magnitude used when `doppler == true`.
/// - `rho` in `1 ..= UNIT`: reciprocally convex slack contraction ratio.
/// - `gamma_cap > 0`: loop gain / coupling sensitivity.
/// - `lambda_max > 0`: upper spectral bound on the network Laplacian.
pub fn thm3_max_delay(
    mu_plus: i64,
    mu_minus: i64,
    rho: i64,
    gamma_cap: i64,
    lambda_max: i64,
    doppler: bool,
) -> Result<i64, MathError> {
    if !(0..UNIT).contains(&mu_plus)
        || mu_minus < 0
        || !(1..=UNIT).contains(&rho)
        || gamma_cap <= 0
        || lambda_max <= 0
    {
        return Err(MathError::InvalidDomain("parameter out of Wirtinger-RCC domain"));
    }
    let unit = i128::from(UNIT);
    let mp = i128::from(mu_plus);
    let mm = i128::from(mu_minus);
    let r = i128::from(rho);

    let (eta_bar, rho_eff) = if doppler {
        let eta = unit.checked_add(mm).ok_or(MathError::Overflow)?;
        let r_eff = ((unit - mp) * r) / eta;
        (eta, r_eff)
    } else {
        (unit, r)
    };

    let inner_term = (2 * (unit - mp) * rho_eff * unit) / (unit + rho_eff);
    let inner_sq = (mp * mp) / 4 + inner_term;
    let inner_root = integer_root(inner_sq)?;
    let radicand_u = unit - (mp + 1) / 2 + inner_root;
    let num_u = integer_root(radicand_u * unit)?;

    let denom_gl = i128::from(gamma_cap)
        .checked_mul(i128::from(lambda_max))
        .ok_or(MathError::Overflow)?;
    let denom = eta_bar.checked_mul(denom_gl).ok_or(MathError::Overflow)?;
    let num = num_u * unit * unit * unit;
    narrow(num / denom)
}

/// Local 1-hop Anderson & Morley (1985) spectral utilization ratio
/// `(deg_i + deg_j) * UNIT / budget_ij`.
///
/// `deg_i`, `deg_j`, and `budget_ij` must share the same degree unit
/// (either all unscaled edge counts or all pre-scaled by [`UNIT`]).
pub fn anderson_morley_utilization(
    deg_i: i64,
    deg_j: i64,
    budget_ij: i64,
) -> Result<i64, MathError> {
    if deg_i < 0 || deg_j < 0 || budget_ij <= 0 {
        return Err(MathError::InvalidDomain("invalid degree or spectral budget"));
    }
    let deg_sum = i128::from(deg_i) + i128::from(deg_j);
    let num = deg_sum
        .checked_mul(i128::from(UNIT))
        .ok_or(MathError::Overflow)?;
    narrow(num / i128::from(budget_ij))
}

fn div_ceil_pos_den(num: i128, den: i128) -> Result<i128, MathError> {
    if den <= 0 {
        return Err(MathError::InvalidDomain("divisor must be positive"));
    }
    if num >= 0 {
        let adjusted = num.checked_add(den - 1).ok_or(MathError::Overflow)?;
        Ok(adjusted / den)
    } else {
        Ok(num / den)
    }
}

/// L09: Kish (1965) unequal-weight design effect scaled by [`UNIT`],
/// rounded upward (`>= UNIT`) so effective sample sizes never overstate precision.
///
/// Computes `ceil_unit(n * sum(w_i^2) / (sum(w_i))^2)`. Refuses empty slices,
/// non-positive weights, or products exceeding `i128`/`i64` bounds.
pub fn kish_design_effect(weights_scaled: &[i64]) -> Result<i64, MathError> {
    if weights_scaled.is_empty() {
        return Err(MathError::InvalidDomain("weights slice cannot be empty"));
    }
    let mut sum_w: i128 = 0;
    let mut sum_w2: i128 = 0;
    for w in weights_scaled {
        if *w <= 0 {
            return Err(MathError::InvalidDomain("weights must be strictly positive"));
        }
        let wi = i128::from(*w);
        sum_w = sum_w.checked_add(wi).ok_or(MathError::Overflow)?;
        let wi2 = wi.checked_mul(wi).ok_or(MathError::Overflow)?;
        sum_w2 = sum_w2.checked_add(wi2).ok_or(MathError::Overflow)?;
    }
    let n = i128::try_from(weights_scaled.len()).map_err(|_| MathError::Overflow)?;
    let unit = i128::from(UNIT);
    let den = sum_w.checked_mul(sum_w).ok_or(MathError::Overflow)?;
    let prod = n.checked_mul(sum_w2).ok_or(MathError::Overflow)?;
    let deff = if let Some(num) = prod.checked_mul(unit) {
        div_ceil_pos_den(num, den)?
    } else {
        let q = prod / den;
        let r = prod % den;
        let rem_scaled = div_ceil_pos_den(
            r.checked_mul(unit).ok_or(MathError::Overflow)?,
            den,
        )?;
        q.checked_mul(unit)
            .and_then(|v| v.checked_add(rem_scaled))
            .ok_or(MathError::Overflow)?
    };
    narrow(deff.max(unit))
}

/// L09: Moulton (1986) unequal-cluster design effect scaled by [`UNIT`],
/// using contractive cluster-size factor `m_tilde = sum(m_g^2) / sum(m_g)`
/// (`DEFF_cluster = 1 + (m_tilde - 1) * rho_cl`), rounded conservatively upward.
///
/// Validates `m_g >= 1`, `|rho_cl_scaled| <= UNIT`, and the cluster-matrix
/// positive-semidefinite constraint `1 + (m_max - 1) * rho_cl > 0`.
pub fn moulton_cluster_deff(
    cluster_sizes: &[u64],
    rho_cl_scaled: i64,
) -> Result<i64, MathError> {
    if cluster_sizes.is_empty() {
        return Err(MathError::InvalidDomain("cluster_sizes cannot be empty"));
    }
    if !(-UNIT..=UNIT).contains(&rho_cl_scaled) {
        return Err(MathError::InvalidDomain("rho_cl outside [-UNIT, UNIT]"));
    }
    let mut n: i128 = 0;
    let mut sum_m2: i128 = 0;
    let mut m_max: i128 = 0;
    for &m in cluster_sizes {
        if m == 0 {
            return Err(MathError::InvalidDomain("cluster size must be at least 1"));
        }
        let mi = i128::from(m);
        n = n.checked_add(mi).ok_or(MathError::Overflow)?;
        let mi2 = mi.checked_mul(mi).ok_or(MathError::Overflow)?;
        sum_m2 = sum_m2.checked_add(mi2).ok_or(MathError::Overflow)?;
        if mi > m_max {
            m_max = mi;
        }
    }
    let unit = i128::from(UNIT);
    let rho = i128::from(rho_cl_scaled);
    let psd_margin = (m_max - 1)
        .checked_mul(rho)
        .and_then(|v| unit.checked_add(v))
        .ok_or(MathError::Overflow)?;
    if psd_margin <= 0 {
        return Err(MathError::InvalidDomain(
            "rho_cl violates cluster positive-semidefinite bound",
        ));
    }
    let excess_num = (sum_m2 - n)
        .checked_mul(rho)
        .ok_or(MathError::Overflow)?;
    let excess = div_ceil_pos_den(excess_num, n)?;
    let deff = unit.checked_add(excess).ok_or(MathError::Overflow)?;
    if deff <= 0 {
        return Err(MathError::InvalidDomain("cluster design effect must stay positive"));
    }
    narrow(deff)
}

/// L09: Combines cluster and weighting design effects as `DEFF_cluster + DEFF_Kish - UNIT`
/// under cluster-weight independence (`DEFF_Kish >= UNIT`, `DEFF_cluster > 0`).
pub fn combine_deff_additive(
    deff_cluster_scaled: i64,
    deff_kish_scaled: i64,
) -> Result<i64, MathError> {
    if deff_cluster_scaled <= 0 || deff_kish_scaled < UNIT {
        return Err(MathError::InvalidDomain("invalid component design effect"));
    }
    let total = i128::from(deff_cluster_scaled) + i128::from(deff_kish_scaled) - i128::from(UNIT);
    if total <= 0 {
        return Err(MathError::InvalidDomain("combined design effect must stay positive"));
    }
    narrow(total)
}

/// L24: Chao1 (Chao 1984; Chao & Colwell 2017) unseen-class lower bound scaled by [`UNIT`],
/// truncated toward zero so integer rounding never overstates the floor.
///
/// Computes `f1^2 / (2 * f2)` when `f2 > 0`, and bias-corrected `f1 * (f1 - 1) / 2`
/// when `f2 == 0` (keeping finite bounds when no doubletons are observed).
pub fn chao1_lower_bound(f1: u64, f2: u64) -> Result<i64, MathError> {
    if f1 == 0 {
        return Ok(0);
    }
    let f1_w = i128::from(f1);
    let unit = i128::from(UNIT);
    let (num, den) = if f2 > 0 {
        let num = f1_w
            .checked_mul(f1_w)
            .and_then(|v| v.checked_mul(unit))
            .ok_or(MathError::Overflow)?;
        let den = i128::from(f2).checked_mul(2).ok_or(MathError::Overflow)?;
        (num, den)
    } else {
        let num = f1_w
            .checked_mul(f1_w - 1)
            .and_then(|v| v.checked_mul(unit))
            .ok_or(MathError::Overflow)?;
        (num, 2)
    };
    narrow(num / den)
}

/// L24: Chao2 (Chao 1989; Chao & Colwell 2017) two-list incidence unseen-unit lower bound
/// scaled by [`UNIT`], truncated toward zero so integer rounding never exceeds the floor.
///
/// With single-list counts `f1 = n10 + n01` and overlap `m = n11`, computes
/// `f1^2 / (4 * m)` when `m > 0`, and bias-corrected `f1 * (f1 - 1) / 4` when `m == 0`.
pub fn chao2_two_list_lower_bound(n10: u64, n01: u64, m: u64) -> Result<i64, MathError> {
    let f1 = i128::from(n10) + i128::from(n01);
    if f1 == 0 {
        return Ok(0);
    }
    let unit = i128::from(UNIT);
    let (num, den) = if m > 0 {
        let num = f1
            .checked_mul(f1)
            .and_then(|v| v.checked_mul(unit))
            .ok_or(MathError::Overflow)?;
        let den = i128::from(m).checked_mul(4).ok_or(MathError::Overflow)?;
        (num, den)
    } else {
        let num = f1
            .checked_mul(f1 - 1)
            .and_then(|v| v.checked_mul(unit))
            .ok_or(MathError::Overflow)?;
        (num, 4)
    };
    narrow(num / den)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integer_root_handles_zero_squares_non_squares_and_max() {
        assert_eq!(integer_root(0), Ok(0));
        assert_eq!(integer_root(1), Ok(1));
        assert_eq!(integer_root(4), Ok(2));
        assert_eq!(integer_root(15), Ok(3));
        assert_eq!(integer_root(16), Ok(4));
        assert_eq!(integer_root(2 * i128::from(UNIT) * i128::from(UNIT)), Ok(1_414_213));

        let r = integer_root(i128::MAX).expect("i128::MAX root succeeds");
        assert!(r > 0);
        assert!(r.checked_mul(r).is_some());
        assert!((r + 1).checked_mul(r + 1).is_none());

        assert!(matches!(integer_root(-1), Err(MathError::InvalidDomain(_))));
    }

    #[test]
    fn codd_and_scale_refuse_silent_coercion() {
        assert_eq!(CoddValue::Known(0).known(), Some(0));
        assert!(CoddValue::Known(0).is_known());
        assert_eq!(CoddValue::ApplicableUnknown.known(), None);
        assert_eq!(CoddValue::Inapplicable.known(), None);
        assert_eq!(CoddValue::NotAskable.known(), None);
        assert!(!CoddValue::ApplicableUnknown.is_known());
        assert_ne!(CoddValue::Known(0), CoddValue::ApplicableUnknown);

        assert!(!ScaleKind::Nominal.permits_arithmetic_mean());
        assert!(!ScaleKind::Ordinal.permits_arithmetic_mean());
        assert!(ScaleKind::IntervalScale.permits_arithmetic_mean());
        assert!(ScaleKind::Ratio.permits_arithmetic_mean());
    }

    #[test]
    fn interval_and_widening_preserve_center_and_bounds() {
        assert!(matches!(Interval::new(10, 9), Err(MathError::InvalidDomain(_))));
        let base = Interval::new(100_000, 300_000).expect("valid");
        let unchanged = widen_before_seal(base, 0, UNIT, 0).expect("identity");
        assert_eq!(unchanged, base);

        let widened = widen_before_seal(base, 25_000, 4 * UNIT, 10_000).expect("widened");
        // sqrt(4) = 2 -> (scale_excess = 1.0) -> deff_half_expand = width/2 = 100_000;
        // total extra = 25_000 + 10_000 + 100_000 = 135_000.
        assert_eq!(widened.lo, -35_000);
        assert_eq!(widened.hi, 435_000);
        assert_eq!(i128::from(widened.lo) + i128::from(widened.hi), i128::from(base.lo) + i128::from(base.hi));

        // Direct inverted struct literal is still refused defensively.
        let bad = Interval { lo: 50, hi: 10 };
        assert!(matches!(widen_before_seal(bad, 0, UNIT, 0), Err(MathError::InvalidDomain(_))));

        // Overflow is reported rather than saturating or wrapping.
        let near_edge = Interval::new(-i64::MAX, i64::MAX).expect("valid");
        assert_eq!(widen_before_seal(near_edge, 1, UNIT, 0), Err(MathError::Overflow));
    }

    #[test]
    fn lyapunov_and_thm3_delay_bounds() {
        assert_eq!(lyapunov_horizon_scaled(2_302_585, 500_000), Ok(4_605_170));
        assert!(matches!(lyapunov_horizon_scaled(-1, 500_000), Err(MathError::InvalidDomain(_))));
        assert!(matches!(lyapunov_horizon_scaled(1_000_000, 0), Err(MathError::InvalidDomain(_))));

        // Baseline: mu_plus = 0, rho = UNIT, gamma = UNIT, lambda_max = UNIT, no Doppler => sqrt(2) * UNIT.
        let baseline = thm3_max_delay(0, 0, UNIT, UNIT, UNIT, false).expect("baseline");
        assert_eq!(baseline, 1_414_213);

        // Doppler dispersion strictly reduces or equals non-Doppler bound.
        let no_dop = thm3_max_delay(200_000, 100_000, 500_000, UNIT, UNIT, false).expect("no_dop");
        let with_dop = thm3_max_delay(200_000, 100_000, 500_000, UNIT, UNIT, true).expect("with_dop");
        assert!(with_dop <= no_dop);

        // Domain rejections.
        assert!(matches!(thm3_max_delay(UNIT, 0, UNIT, UNIT, UNIT, false), Err(MathError::InvalidDomain(_))));
        assert!(matches!(thm3_max_delay(0, -1, UNIT, UNIT, UNIT, false), Err(MathError::InvalidDomain(_))));
        assert!(matches!(thm3_max_delay(0, 0, 0, UNIT, UNIT, false), Err(MathError::InvalidDomain(_))));
        assert!(matches!(thm3_max_delay(0, 0, UNIT, 0, UNIT, false), Err(MathError::InvalidDomain(_))));
        assert!(matches!(thm3_max_delay(0, 0, UNIT, UNIT, 0, false), Err(MathError::InvalidDomain(_))));
    }

    #[test]
    fn anderson_morley_utilization_checks_scale_and_overflow() {
        assert_eq!(anderson_morley_utilization(3, 5, 10), Ok(800_000));
        assert_eq!(anderson_morley_utilization(3 * UNIT, 5 * UNIT, 10 * UNIT), Ok(800_000));
        assert!(matches!(anderson_morley_utilization(-1, 5, 10), Err(MathError::InvalidDomain(_))));
        assert!(matches!(anderson_morley_utilization(3, 5, 0), Err(MathError::InvalidDomain(_))));
        assert_eq!(anderson_morley_utilization(i64::MAX, 1, 1), Err(MathError::Overflow));
    }

    #[test]
    fn kish_moulton_and_additive_deff_match_l09_identities() {
        // Equal weights => DEFF_Kish == UNIT.
        assert_eq!(kish_design_effect(&[UNIT, UNIT, UNIT, UNIT]), Ok(UNIT));
        // Unequal weights [1, 3] => n=2, sum=4, sum_sq=10 => 2*10/16 = 1.25 => 1_250_000.
        assert_eq!(kish_design_effect(&[UNIT, 3 * UNIT]), Ok(1_250_000));
        assert!(matches!(kish_design_effect(&[]), Err(MathError::InvalidDomain(_))));
        assert!(matches!(kish_design_effect(&[UNIT, 0]), Err(MathError::InvalidDomain(_))));

        // Equal clusters [10, 10], rho = 0.1 => 1 + 9*0.1 = 1.9.
        assert_eq!(moulton_cluster_deff(&[10, 10], 100_000), Ok(1_900_000));
        // Unequal clusters [2, 6]: n=8, sum_m2=40 => m_tilde = 5; rho = 0.25 => 1 + 4*0.25 = 2.0.
        assert_eq!(moulton_cluster_deff(&[2, 6], 250_000), Ok(2_000_000));
        // Uniform unit clusters [1, 1, 1] => 1.0 even for positive rho.
        assert_eq!(moulton_cluster_deff(&[1, 1, 1], 500_000), Ok(UNIT));
        // PSD bound rejection: 1 + (11 - 1) * (-0.1) = 0 <= 0.
        assert!(matches!(
            moulton_cluster_deff(&[2, 11], -100_000),
            Err(MathError::InvalidDomain(_))
        ));
        assert!(matches!(moulton_cluster_deff(&[2, 0], 100_000), Err(MathError::InvalidDomain(_))));

        // Total additive combination strictly below product when both > UNIT.
        let deff_cl = moulton_cluster_deff(&[2, 6], 250_000).expect("valid");
        let deff_k = kish_design_effect(&[UNIT, 3 * UNIT]).expect("valid");
        let total = combine_deff_additive(deff_cl, deff_k).expect("valid");
        assert_eq!(total, 2_250_000);
        let prod_scaled = (i128::from(deff_cl) * i128::from(deff_k)) / i128::from(UNIT);
        assert!(i128::from(total) < prod_scaled);
    }

    #[test]
    fn chao1_and_chao2_cover_regular_and_zero_doubleton_regimes() {
        assert_eq!(chao1_lower_bound(0, 5), Ok(0));
        assert_eq!(chao1_lower_bound(1, 0), Ok(0));
        // f1=4, f2=2 => 16 / 4 = 4.0
        assert_eq!(chao1_lower_bound(4, 2), Ok(4 * UNIT));
        // f1=5, f2=0 => 5*4 / 2 = 10.0
        assert_eq!(chao1_lower_bound(5, 0), Ok(10 * UNIT));

        assert_eq!(chao2_two_list_lower_bound(0, 0, 5), Ok(0));
        assert_eq!(chao2_two_list_lower_bound(1, 0, 0), Ok(0));
        // n10=3, n01=5, m=2 => f1=8, 64 / 8 = 8.0
        assert_eq!(chao2_two_list_lower_bound(3, 5, 2), Ok(8 * UNIT));
        // n10=3, n01=2, m=0 => f1=5, 5*4 / 4 = 5.0
        assert_eq!(chao2_two_list_lower_bound(3, 2, 0), Ok(5 * UNIT));
        assert_eq!(chao1_lower_bound(u64::MAX, 1), Err(MathError::Overflow));
    }
}
