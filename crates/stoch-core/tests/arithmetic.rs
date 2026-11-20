//! Integer arithmetic: the scale, the bound, the truncation, and the absence of
//! binary floating point.
//!
//! The crate fixes `SCALE = 1_000_000` and `MAX_ABS = 1_000_000_000`, and every
//! operation that can leave the bound goes through a `checked_*` primitive. The tests
//! here are the arithmetic half of that claim: the probe fraction is cross-checked
//! against an `i128` reference over a grid, the bound is exercised at its edges, and
//! the truncation is shown to be `floor` only under the precondition that licenses it.

use stoch_core::{probe_fraction, Bounds, Capacity, ControlTape, Error, Guards, Instrument, Params,
                 Score, SevenReadings, State, Thresholds, MAX_ABS, SCALE};

fn readings() -> SevenReadings {
    SevenReadings::new([SCALE; 7])
}

fn params() -> Params {
    Params::new(10, 10, true, false, false, false, Thresholds::new(1, 1))
}

fn seal(l: i64, u: i64, c: i64, r: i64) -> Result<State, Error> {
    State::seal(
        ControlTape::Attested,
        Bounds::new(l, u, 1000, 0),
        Capacity::new(c, 0, r),
        Guards::new(Instrument::Sealed(readings())),
        params(),
    )
}

#[test]
fn the_scale_and_the_bound_are_the_normative_values() {
    assert_eq!(SCALE, 1_000_000, "100 % in millionths");
    assert_eq!(MAX_ABS, 1_000_000_000, "the wire bound");
    // The scale is a power of ten below the bound, so a fraction times a quantity
    // inside the bound stays inside `i64` for the divisions the core performs:
    // `MAX_ABS * SCALE = 1e15`, four orders of magnitude under `i64::MAX`.
    assert_eq!(MAX_ABS * SCALE, 1_000_000_000_000_000);
    assert!(MAX_ABS * SCALE < i64::MAX / 1000);
}

#[test]
fn the_probe_fraction_is_the_truncated_ratio() {
    // The three rows of the fixture, `01` §5.
    assert_eq!(probe_fraction(Bounds::new(120, 400, 1000, 0), 200), Ok(714_285));
    assert_eq!(
        probe_fraction(Bounds::new(0, 3_000_000, 1000, 0), 1_000_000),
        Ok(666_666)
    );
    assert_eq!(
        probe_fraction(Bounds::new(0, 1_000_000_000, 1000, 0), 999_999_999),
        Err(Error::ProbeUnderflow)
    );
}

#[test]
fn the_truncation_is_floor_under_the_licence_and_only_then() {
    let mut checked = 0usize;
    for l in 0i64..6 {
        for u in (l + 1)..=(l + 6) {
            for c in (l + 1)..u {
                let expected = ((u - c) as i128 * SCALE as i128 / (u - l) as i128) as i64;
                let bounds = Bounds::new(l, u, 1000, 0);
                let actual = probe_fraction(bounds, c);
                if expected == 0 {
                    assert_eq!(actual, Err(Error::ProbeUnderflow), "l={} u={} c={}", l, u, c);
                } else {
                    assert_eq!(actual, Ok(expected), "l={} u={} c={}", l, u, c);
                    let fraction = actual.unwrap();
                    assert!(
                        (1..=SCALE - 1).contains(&fraction),
                        "the fraction stays strictly inside the unit interval: {}",
                        fraction
                    );
                }
                checked += 1;
            }
        }
    }
    assert!(checked > 30, "the grid is real: {} triples", checked);
}

#[test]
fn the_probe_refuses_its_own_preconditions_defensively() {
    // `u <= l` is not a straddle, whatever the caller believes.
    assert_eq!(
        probe_fraction(Bounds::new(400, 400, 1000, 0), 200),
        Err(Error::BoundsInverted)
    );
    assert_eq!(
        probe_fraction(Bounds::new(500, 400, 1000, 0), 200),
        Err(Error::BoundsInverted)
    );
    // `c <= l` or `c >= u` is outside the branch that calls this function.
    assert_eq!(
        probe_fraction(Bounds::new(120, 400, 1000, 0), 120),
        Err(Error::BoundaryUnlicensed)
    );
    assert_eq!(
        probe_fraction(Bounds::new(120, 400, 1000, 0), 400),
        Err(Error::BoundaryUnlicensed)
    );
    assert_eq!(
        probe_fraction(Bounds::new(120, 400, 1000, 0), 500),
        Err(Error::BoundaryUnlicensed)
    );
}

#[test]
fn the_bound_is_enforced_at_the_door_and_not_by_luck() {
    assert_eq!(seal(120, 400, 200, 0).is_ok(), true);
    assert_eq!(
        seal(120, MAX_ABS + 1, 200, 0),
        Err(Error::ScaleOverflow),
        "one unit past the bound is past the bound"
    );
    assert_eq!(seal(-MAX_ABS, MAX_ABS, 1, 0), Ok(seal(-MAX_ABS, MAX_ABS, 1, 0).unwrap()));
    assert_eq!(
        seal(-(MAX_ABS + 1), MAX_ABS, 1, 0),
        Err(Error::ScaleOverflow)
    );
    assert_eq!(seal(120, 400, MAX_ABS + 1, 0), Err(Error::ScaleOverflow));
    assert_eq!(seal(120, 400, 200, -(MAX_ABS + 1)), Err(Error::ScaleOverflow));
}

#[test]
fn an_extreme_but_legal_probe_does_not_overflow() {
    // The widest legal span: the numerator is at most `2 * MAX_ABS` and the product
    // with the scale is `2e15`, comfortably inside `i64`.
    let bounds = Bounds::new(-MAX_ABS, MAX_ABS, 1000, 0);
    assert_eq!(
        probe_fraction(bounds, -MAX_ABS + 1),
        Ok(999_999),
        "the widest span probes at essentially the whole reserve"
    );
    // The truncation floor of a billion-wide extent: a thousand units of room is the
    // first quantity that survives the division by the scale.
    assert_eq!(
        probe_fraction(Bounds::new(0, MAX_ABS, 1000, 0), MAX_ABS - 1_000),
        Ok(1),
        "1000 * 1e6 / 1e9 == 1 exactly"
    );
    assert_eq!(
        probe_fraction(Bounds::new(0, MAX_ABS, 1000, 0), MAX_ABS - 999),
        Err(Error::ProbeUnderflow),
        "999 * 1e6 / 1e9 truncates to zero"
    );
}

#[test]
fn absence_is_not_zero() {
    assert_ne!(Score::Missing, Score::Present(0));
    assert!(Score::Present(0).is_present());
    assert!(!Score::Missing.is_present());
    // The sentinel of a missing extent is not a coordinate either: `-1` is refused at
    // the door rather than read as a small negative extent.
    assert_eq!(
        State::seal(
            ControlTape::Attested,
            Bounds::new(-1, 400, 1000, 0),
            Capacity::new(200, 0, 0),
            Guards::new(Instrument::Sealed(readings())),
            params(),
        )
        .err(),
        Some(Error::ExtentMissing)
    );
}

#[test]
fn every_error_code_is_negative_and_distinct() {
    let errors = [
        Error::AggregateForbidden,
        Error::BoundsInverted,
        Error::CapacityNonPositive,
        Error::ReserveNegative,
        Error::ScaleOverflow,
        Error::ExtentMissing,
        Error::TapeUnsealed,
        Error::AxisUnguarded,
        Error::ProbeUnderflow,
        Error::BoundaryUnlicensed,
        Error::SitePropositionMismatch,
        Error::AbiMismatch,
    ];
    let mut codes: Vec<i32> = errors.iter().map(|e| e.code()).collect();
    let count = codes.len();
    codes.sort_unstable();
    codes.dedup();
    assert_eq!(codes.len(), count, "the codes are distinct");
    assert_eq!(codes.first(), Some(&-12));
    assert_eq!(codes.last(), Some(&-1));
    assert!(codes.iter().all(|c| *c < 0), "no zero code: zero is success");
    assert_eq!(count, 12, "twelve codes, -13 burned and -14 out of scope");
}
