//! The per-step canonical operator, `01` §2.2.
//!
//! `step` is the only emitter of a decision in this crate, and it takes nothing but
//! a [`State`]. It runs three questions in a fixed sequence, and the sequence is
//! flow of control, never an ordering by discriminant (`01` §1.3):
//!
//! * **Latency pre-emption** — does the vessel survive its own correction? (`TTS < TTR_eff`
//!   fires [`HaltCause::SurvivalLatency`] immediately, even if an instrument slot is
//!   missing — defeating compliance denial-of-service — and co-reports concurrent
//!   governance corruption as [`HaltCause::SurvivalAndGovernance`].)
//! * **Q1** — given surviving latency headroom, is the instrument known? The
//!   incompleteness is a variant of [`Instrument`], not a boolean, so the quarantine comes
//!   out of the type: there is no state in which `Sealed` carries an absent reading.
//! * **Q2** — given a surviving clock and sealed instrument, are governance channels and
//!   vital/governance floors intact? The channel check precedes the floors because an inward
//!   action presumes the alarm arrives (`01` §2.1).
//! * **Q3** — is the identification region decisive? The extent against the capacity:
//!   `L > C` or `L == C < U` licenses [`Regime::BufferedAction`], `L < C < U` licenses
//!   straddle handling (reversible probing vs. irreversible probe refusal or binary
//!   lethal-omission impasse), and `U <= C` (`U < C`, `L < C == U`, or `L == U == C`)
//!   licenses [`Regime::NoAction`] (`36` §2; `41` §5; `44` §3).
//!
//! Two boundaries are decided and not defaulted: `TTS == TTR` **passes** Q2 (the
//! inequality is `>=`), while direct calls to [`probe_fraction`] outside strict
//! `L < C < U` defensively return [`Error::BoundaryUnlicensed`] (`-10`).

use crate::{
    Bounds, Constraint, Error, HaltCause, Instrument, MissingSlot, Regime, SCALE, State,
};

/// The fraction of the reserve a reversible probe commits, `01` §5.
///
/// `a*_MMR = (U - C) / (U - L)`, in millionths, truncated toward zero. With
/// `L < C < U` both the numerator and the denominator are positive, so truncation
/// toward zero coincides with `floor`; the coincidence is stated here so that it is
/// not presumed anywhere else. The result is in `1 ..= SCALE - 1`, because
/// `U - C < U - L` when `C > L`.
///
/// A fraction that truncates to zero is [`Error::ProbeUnderflow`], not a zero-sized
/// probe: a silent zero looks like a decision, and total inaction assumes the worst
/// case as certain (`41` §5, line 309).
///
/// The two preconditions are the licence of the calling branch. They are re-checked
/// defensively so that the function is total on its own, but no sequence of states
/// reachable through [`crate::State::seal`] can violate them: the extent is checked,
/// and the branch that calls this function is `L < C < U`.
///
/// ```
/// use stoch_core::{probe_fraction, Bounds, Error};
///
/// let bounds = Bounds::new(120, 400, 1000, 0);
/// // (400 - 200) / (400 - 120) = 0.7142857..., truncated to 714_285.
/// assert_eq!(probe_fraction(bounds, 200), Ok(714_285));
/// // One millionth of a unit of room truncates to zero.
/// assert_eq!(
///     probe_fraction(Bounds::new(0, 1_000_000_000, 1, 0), 999_999_999),
///     Err(Error::ProbeUnderflow)
/// );
/// ```
pub fn probe_fraction(bounds: Bounds, c: i64) -> Result<i64, Error> {
    let num = bounds.u.checked_sub(c).ok_or(Error::ScaleOverflow)?;
    let den = bounds.u.checked_sub(bounds.l).ok_or(Error::ScaleOverflow)?;
    // Defensive: `L < C < U` is the caller's licence (01 §5).
    if den <= 0 {
        return Err(Error::BoundsInverted);
    }
    if num <= 0 {
        return Err(Error::BoundaryUnlicensed);
    }
    // `C == L` would give `q == SCALE`: probing the whole reserve is not a probe, and
    // the range documented above (`1 ..= SCALE - 1`) would be a lie about this branch.
    // The caller's licence excludes it; the function refuses it anyway.
    if num >= den {
        return Err(Error::BoundaryUnlicensed);
    }
    let scaled = num.checked_mul(SCALE).ok_or(Error::ScaleOverflow)?;
    let q = scaled.checked_div(den).ok_or(Error::ScaleOverflow)?;
    if q == 0 {
        return Err(Error::ProbeUnderflow);
    }
    Ok(q)
}

/// The canonical operator: `State -> Regime`, total over the sealed state space.
///
/// Survival latency (`tts < ttr_eff`) is tested first so missing instrumentation
/// cannot trap an expiring vessel inside a compliance-DoS quarantine (`41` §5), and
/// concurrent governance corruption is reported jointly rather than hidden. Once
/// survival headroom holds, incomplete instrumentation quarantines, followed by channel
/// integrity, step floors, and identification geometry (`Q3`).
///
/// ```
/// use stoch_core::{step, Bounds, Capacity, Constraint, ControlTape, Guards, HaltCause,
///                  Instrument, MissingSlot, Params, Regime, SevenReadings, State,
///                  Thresholds};
///
/// let sealed = Guards::new(Instrument::Sealed(SevenReadings::new([500_000; 7])));
/// let params = Params::new(200, 100, true, false, false, false, Thresholds::new(100_000, 50_000));
///
/// // L > C: a full intervention inside the buffer.
/// let state = State::seal(
///     ControlTape::Attested,
///     Bounds::new(400, 800, 1000, 0),
///     Capacity::new(200, 0, 1000),
///     sealed,
///     params,
/// ).unwrap();
/// assert_eq!(step(&state), Ok(Regime::BufferedAction));
///
/// // With survival headroom intact (tts >= ttr_eff), an incomplete instrument is quarantined.
/// let quarantined = State::seal(
///     ControlTape::Attested,
///     Bounds::new(400, 800, 1000, 0),
///     Capacity::new(200, 0, 1000),
///     Guards::new(Instrument::Incomplete(MissingSlot::new(3).unwrap())),
///     params,
/// ).unwrap();
/// assert_eq!(
///     step(&quarantined),
///     Ok(Regime::Quarantine { missing: MissingSlot::new(3).unwrap() })
/// );
///
/// // When survival time falls below recovery, an algedonic halt pre-empts missing-slot quarantine.
/// let urgent = State::seal(
///     ControlTape::Attested,
///     Bounds::new(400, 800, 1000, 0),
///     Capacity::new(200, 0, 1000),
///     Guards::new(Instrument::Incomplete(MissingSlot::new(3).unwrap())),
///     Params::new(50, 100, true, false, false, false, Thresholds::new(100_000, 50_000)),
/// ).unwrap();
/// assert_eq!(
///     step(&urgent),
///     Ok(Regime::AlgedonicHalt { cause: HaltCause::SurvivalLatency })
/// );
/// ```
pub fn step(s: &State) -> Result<Regime, Error> {
    // Question 1 (`41` §5): missing, uncalibrated, or negative survival/recovery clocks
    // (`T5`) never proceed unchecked—quarantine on `T5`, or escalate when `gov_corrupt` coincides.
    if s.params.clocks_missing || s.params.tts < 0 || s.params.ttr_eff < 0 {
        return Ok(if s.params.gov_corrupt {
            Regime::AlgedonicHalt {
                cause: HaltCause::GovernanceChannel,
            }
        } else {
            Regime::Quarantine {
                missing: MissingSlot::T5,
            }
        });
    }

    // Calibrated survival clock preemption (`41` §5): an expiring vessel must never be
    // trapped in missing-slot quarantine, and concurrent channel corruption surfaces together.
    if s.params.tts < s.params.ttr_eff {
        return Ok(Regime::AlgedonicHalt {
            cause: if s.params.gov_corrupt {
                HaltCause::SurvivalAndGovernance
            } else {
                HaltCause::SurvivalLatency
            },
        });
    }

    // Question 2 (`41` §5): instrument completeness & open `T1` reopening ahead of sealed
    // channel corruption so unsealed slots or `T1` reopenings stay quarantined on `III-A`.
    match &s.guards.instrument {
        Instrument::Incomplete(slot) => return Ok(Regime::Quarantine { missing: *slot }),
        Instrument::Sealed(_) => {}
    }

    if s.params.reopen_sealed {
        return Ok(Regime::Quarantine {
            missing: MissingSlot::T1,
        });
    }

    // Channel integrity and step floors once all seven guard slots are sealed (`41` §5).
    if s.params.gov_corrupt {
        return Ok(Regime::AlgedonicHalt {
            cause: HaltCause::GovernanceChannel,
        });
    }
    if s.params.vital_floor_breached {
        return Ok(Regime::InwardAction {
            breached: Constraint::VitalFloor,
        });
    }
    if s.params.gov_floor_breached {
        return Ok(Regime::InwardAction {
            breached: Constraint::GovernanceFloor,
        });
    }

    // Q3 — is the identification region decisive? (`36`, `41` §5, `44`).
    use core::cmp::Ordering::{Equal, Greater, Less};
    match (s.bounds.l.cmp(&s.capacity.c), s.bounds.u.cmp(&s.capacity.c)) {
        (Greater, _) | (Equal, Greater) => Ok(Regime::BufferedAction),
        (Less, Greater) => {
            if s.params.reversible {
                Ok(Regime::ReversibleFractionalProbe {
                    fraction: probe_fraction(s.bounds, s.capacity.c)?,
                })
            } else if s.params.lethal_omission {
                // Strictly binary irreversible choice where inaction is itself ruin (`41` §5).
                Ok(Regime::AlgedonicHalt {
                    cause: HaltCause::LethalOmissionImpasse,
                })
            } else {
                // Unilateral irreversible straddle defaults safely to probe refusal (`01` §2.1, L2).
                Ok(Regime::AlgedonicHalt {
                    cause: HaltCause::IrreversibleProbeRefused,
                })
            }
        }
        // `U <= C` (`U < C`, `L < C == U`, or `L == U == C`): `a*_MMR = 0`, so abstaining is
        // licensed whenever inaction is non-lethal (`36`, `41` §5 lines 337–340, `44`).
        (Less, Less) | (Less, Equal) | (Equal, Equal) => Ok(Regime::NoAction),
        // Unreachable for sealed `Bounds` (`L <= U`), kept total for raw internal states.
        (Equal, Less) => Err(Error::BoundsInverted),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        Capacity, ControlTape, Guards, MissingSlot, Params, SevenReadings, State, Thresholds,
    };

    fn sealed_state(l: i64, u: i64, c: i64, tts: i64, ttr: i64, reversible: bool) -> State {
        State::seal(
            ControlTape::Attested,
            Bounds::new(l, u, 1000, 0),
            Capacity::new(c, 0, 1000),
            Guards::new(Instrument::Sealed(SevenReadings::new([SCALE; 7]))),
            Params::new(
                tts,
                ttr,
                reversible,
                false,
                false,
                false,
                Thresholds::new(100_000, 50_000),
            ),
        )
        .unwrap()
    }

    #[test]
    fn probe_fraction_truncates_toward_zero() {
        assert_eq!(probe_fraction(Bounds::new(120, 400, 1, 0), 200), Ok(714_285));
        assert_eq!(
            probe_fraction(Bounds::new(0, 3_000_000, 1, 0), 1_000_000),
            Ok(666_666)
        );
        // The bound of §5: q <= SCALE - 1, because C > L.
        let almost = probe_fraction(Bounds::new(0, 1_000_000, 1, 0), 1);
        assert_eq!(almost, Ok(999_999));
    }

    #[test]
    fn probe_fraction_underflows_instead_of_zeroing() {
        assert_eq!(
            probe_fraction(Bounds::new(0, 1_000_000_000, 1, 0), 999_999_999),
            Err(Error::ProbeUnderflow)
        );
    }

    #[test]
    fn probe_fraction_defends_its_preconditions() {
        assert_eq!(
            probe_fraction(Bounds::new(400, 400, 1, 0), 400),
            Err(Error::BoundsInverted)
        );
        assert_eq!(
            probe_fraction(Bounds::new(100, 400, 1, 0), 400),
            Err(Error::BoundaryUnlicensed)
        );
    }

    #[test]
    fn survival_latency_preempts_quarantine_and_co_reports_channel() {
        let incomplete_urgent = State::seal(
            ControlTape::Attested,
            Bounds::new(400, 800, 1000, 0),
            Capacity::new(200, 0, 1000),
            Guards::new(Instrument::Incomplete(MissingSlot::new(3).unwrap())),
            Params::new(
                5,
                10,
                true,
                true,
                true,
                false,
                Thresholds::new(100_000, 50_000),
            ),
        )
        .unwrap();
        assert_eq!(
            step(&incomplete_urgent),
            Ok(Regime::AlgedonicHalt {
                cause: HaltCause::SurvivalLatency
            })
        );

        let incomplete_joint = State::seal(
            ControlTape::Attested,
            Bounds::new(400, 800, 1000, 0),
            Capacity::new(200, 0, 1000),
            Guards::new(Instrument::Incomplete(MissingSlot::new(3).unwrap())),
            Params::new(
                5,
                10,
                true,
                true,
                true,
                true,
                Thresholds::new(100_000, 50_000),
            ),
        )
        .unwrap();
        assert_eq!(
            step(&incomplete_joint),
            Ok(Regime::AlgedonicHalt {
                cause: HaltCause::SurvivalAndGovernance
            })
        );

        let incomplete_survivable = State::seal(
            ControlTape::Attested,
            Bounds::new(400, 800, 1000, 0),
            Capacity::new(200, 0, 1000),
            Guards::new(Instrument::Incomplete(MissingSlot::new(3).unwrap())),
            Params::new(
                10,
                10,
                true,
                true,
                true,
                true,
                Thresholds::new(100_000, 50_000),
            ),
        )
        .unwrap();
        assert_eq!(
            step(&incomplete_survivable),
            Ok(Regime::Quarantine {
                missing: MissingSlot::new(3).unwrap()
            })
        );
    }

    #[test]
    fn lethal_omission_distinguishes_binary_impasse_from_safe_probe_refusal() {
        let base = State::seal(
            ControlTape::Attested,
            Bounds::new(120, 400, 1000, 0),
            Capacity::new(200, 0, 1000),
            Guards::new(Instrument::Sealed(SevenReadings::new([SCALE; 7]))),
            Params::new(10, 10, false, false, false, false, Thresholds::new(100_000, 50_000))
                .with_lethal_omission(true),
        )
        .unwrap();
        assert_eq!(
            step(&base),
            Ok(Regime::AlgedonicHalt {
                cause: HaltCause::LethalOmissionImpasse
            })
        );
    }

    #[test]
    fn q2_precedes_q3() {
        let state = sealed_state(400, 800, 200, 5, 10, true);
        assert_eq!(
            step(&state),
            Ok(Regime::AlgedonicHalt {
                cause: HaltCause::SurvivalLatency
            })
        );
    }

    #[test]
    fn latency_equality_passes_q2() {
        let state = sealed_state(400, 800, 200, 10, 10, true);
        assert_eq!(step(&state), Ok(Regime::BufferedAction));
    }

    #[test]
    fn boundary_equalities_follow_chapter_41_licence() {
        assert_eq!(
            step(&sealed_state(200, 400, 200, 10, 10, true)),
            Ok(Regime::BufferedAction)
        );
        assert_eq!(
            step(&sealed_state(100, 200, 200, 10, 10, true)),
            Ok(Regime::NoAction)
        );
        assert_eq!(
            step(&sealed_state(200, 200, 200, 10, 10, true)),
            Ok(Regime::NoAction)
        );
    }

    #[test]
    fn the_region_below_the_cost_is_no_action() {
        let regime = step(&sealed_state(100, 150, 200, 10, 10, true)).unwrap();
        assert_eq!(regime.tag(), 5);
        assert_eq!(regime.permitted(), crate::ActionSet::None);
    }
}

/// The exhaustive state grid of `02` §2.5.3.
///
/// Two grids of 96 states each, and the difference between them is the point.
///
/// **Grid A** — `2^5 * 3 = 96`: the five binary parameters against the three
/// instrument conditions. Its question is whether every combination of the guards is
/// answered at all, that is, whether the operator is total on the boolean cube.
///
/// **Grid B** — `16 * 6 = 96`: the four binary parameters against the six geometries
/// of the extent, with reversibility carried by the geometry. Its question is whether
/// the four biconditionals of `02` §2.5.3 hold, and it is the grid where the *naive*
/// forms fail: a bare "halt iff irreversible" or a bare "no action iff `U < C`"
/// misclassifies states that an earlier question already decided. The guarded forms
/// hold on all 96, which is what makes the guards load-bearing rather than decorative.
#[cfg(test)]
mod grid_tests {
    use super::*;
    use crate::{Capacity, ControlTape, Guards, MissingSlot, Params, SevenReadings, Thresholds};

    /// Six geometries of the extent, `02` §2.5.3.
    struct Geometry {
        name: &'static str,
        l: i64,
        u: i64,
        c: i64,
        reversible: bool,
    }

    const GEOMETRIES: [Geometry; 6] = [
        Geometry {
            name: "g1: l > c, buffered",
            l: 400,
            u: 800,
            c: 200,
            reversible: true,
        },
        Geometry {
            name: "g2: l < c < u, reversible",
            l: 120,
            u: 400,
            c: 200,
            reversible: true,
        },
        Geometry {
            name: "g3: l < c < u, irreversible",
            l: 120,
            u: 400,
            c: 200,
            reversible: false,
        },
        Geometry {
            name: "g4: u < c",
            l: 100,
            u: 150,
            c: 200,
            reversible: true,
        },
        Geometry {
            name: "g5: l == c",
            l: 200,
            u: 400,
            c: 200,
            reversible: true,
        },
        Geometry {
            name: "g6: l > u, unsealable",
            l: 400,
            u: 200,
            c: 100,
            reversible: true,
        },
    ];

    /// A state assembled field by field, past the seal.
    ///
    /// This is only reachable inside the crate, and it is what lets the grid include
    /// the geometries the seal refuses: the operator is total on inputs the door
    /// rejects, and a grid built on the door would silently test only the licit
    /// half. Every geometry the door would refuse is therefore named as such and its
    /// expected answer is the operator's own, not a verdict about admissibility.
    fn raw(
        geometry: &Geometry,
        tts: i64,
        ttr: i64,
        vital_floor: bool,
        gov_floor: bool,
        gov_corrupt: bool,
        instrument: Instrument,
    ) -> State {
        State {
            bounds: Bounds::new(geometry.l, geometry.u, 1000, 0),
            capacity: Capacity::new(geometry.c, 0, 0),
            guards: Guards::new(instrument),
            params: Params::new(
                tts,
                ttr,
                geometry.reversible,
                vital_floor,
                gov_floor,
                gov_corrupt,
                Thresholds::new(100_000, 50_000),
            ),
        }
    }

    fn sealed_instrument() -> Instrument {
        Instrument::Sealed(SevenReadings::new([500_000; 7]))
    }

    #[test]
    fn grid_a_covers_the_boolean_cube_against_three_instrument_conditions() {
        let instruments = [
            sealed_instrument(),
            Instrument::Incomplete(MissingSlot::new(1).unwrap()),
            Instrument::Incomplete(MissingSlot::new(7).unwrap()),
        ];
        let geometry = &GEOMETRIES[0];
        let mut histogram = [0usize; 6];
        let mut states = 0usize;

        for bits in 0u8..32 {
            let latency = bits & 1 != 0;
            let vital_floor = bits & 2 != 0;
            let gov_floor = bits & 4 != 0;
            let gov_corrupt = bits & 8 != 0;
            // Bit 4 is reversibility: it cannot matter in `l > c`, and the grid says
            // so by producing the same answer for both readings.
            let reversible_bit = bits & 16 != 0;

            for instrument in instruments {
                let state = raw(
                    geometry,
                    if latency { 5 } else { 10 },
                    10,
                    vital_floor,
                    gov_floor,
                    gov_corrupt,
                    instrument,
                );
                let mut state = state;
                state.params.reversible = reversible_bit;
                let regime = step(&state).expect("every state in grid A answers");
                histogram[usize::from(regime.tag())] += 1;
                states += 1;
            }
        }

        assert_eq!(states, 96, "grid A is 2^5 * 3");
        // Under survival latency (tts < ttr_eff), halts fire across all three instrument
        // conditions (16 * 3 = 48 states); the 32 non-latency incomplete states quarantine.
        assert_eq!(histogram[3], 32, "non-latency incomplete instruments quarantine");
        assert_eq!(histogram[0] + histogram[2] + histogram[4], 64);
        assert_eq!(histogram[1], 0, "no probe in this geometry");
        assert_eq!(histogram[5], 0, "no no-action in this geometry");
    }

    #[test]
    fn grid_b_holds_the_guarded_biconditionals_on_all_ninety_six() {
        let mut states = 0usize;
        let mut answered = 0usize;
        let mut refused = 0usize;
        let mut histogram = [0usize; 6];

        let mut guarded_2 = 0usize;
        let mut guarded_3 = 0usize;
        let mut guarded_4 = 0usize;
        let mut guarded_5 = 0usize;
        let mut naive_2 = 0usize;
        let mut naive_3 = 0usize;
        let mut naive_5 = 0usize;

        for geometry in &GEOMETRIES {
            for bits in 0u8..16 {
                let latency = bits & 1 != 0;
                let vital_floor = bits & 2 != 0;
                let gov_floor = bits & 4 != 0;
                let gov_corrupt = bits & 8 != 0;

                let state = raw(
                    geometry,
                    if latency { 5 } else { 10 },
                    10,
                    vital_floor,
                    gov_floor,
                    gov_corrupt,
                    sealed_instrument(),
                );
                states += 1;

                let regime = step(&state);
                let tag = match regime {
                    Ok(regime) => {
                        answered += 1;
                        histogram[usize::from(regime.tag())] += 1;
                        Some(regime.tag())
                    }
                    Err(error) => {
                        assert_eq!(
                            error,
                            Error::BoundaryUnlicensed,
                            "{} only refuses the unlicensed region",
                            geometry.name
                        );
                        refused += 1;
                        None
                    }
                };

                let floor = vital_floor || gov_floor;
                let straddle = geometry.l < geometry.c && geometry.c < geometry.u;
                let latency_clean = !latency;
                let channel_clean = !gov_corrupt;
                let floor_clear = !floor;

                let is_2 = tag == Some(2);
                let is_4 = tag == Some(4);
                let is_5 = tag == Some(5);

                // The guarded forms, `02` §2.5.3.
                if is_2 != (floor && latency_clean && channel_clean) {
                    guarded_2 += 1;
                }
                if is_4
                    != (latency
                        || gov_corrupt
                        || (straddle
                            && !geometry.reversible
                            && latency_clean
                            && channel_clean
                            && floor_clear))
                {
                    guarded_3 += 1;
                }
                if is_5 != (geometry.u <= geometry.c && latency_clean && channel_clean && floor_clear)
                {
                    guarded_4 += 1;
                }
                // The instrument biconditional: a sealed instrument iff no quarantine.
                if (tag == Some(3)) != !sealed_instrument().is_sealed() {
                    guarded_5 += 1;
                }

                // The naive forms, for comparison: each omits a guard an earlier
                // question already applied.
                if is_2 != (floor && latency_clean && channel_clean) {
                    naive_2 += 1;
                }
                if is_4 != (latency || gov_corrupt || (straddle && !geometry.reversible)) {
                    naive_3 += 1;
                }
                if is_5 != (geometry.u <= geometry.c) {
                    naive_5 += 1;
                }
            }
        }

        assert_eq!(states, 96, "grid B is 16 * 6");
        assert_eq!(answered + refused, 96);
        // Under Chs. 36, 41 §5, and 44, `l == c < u` triggers BufferedAction, so every
        // state in Grid B answers (`refused == 0`).
        assert_eq!(refused, 0, "every grid B state answers under chapter 41 boundary licences");

        // The guarded biconditionals hold everywhere.
        assert_eq!(guarded_2, 0, "tag 2 iff floor, no latency, no corruption");
        assert_eq!(guarded_3, 0, "tag 4 iff latency, corruption, or refused probe");
        assert_eq!(guarded_4, 0, "tag 5 iff u < c and the guards are clean");
        assert_eq!(guarded_5, 0, "tag 3 iff the instrument is incomplete");

        // The naive forms do not, and that is the reason the guards exist.
        assert_eq!(naive_2, 0, "this one is already guarded");
        assert!(naive_3 > 0, "the bare probe rule misclassifies");
        assert!(naive_5 > 0, "the bare u < c rule misclassifies");

        // All six regimes are reachable inside the grid except the quarantine, which
        // needs an incomplete instrument and is covered by grid A.
        assert_eq!(histogram[0] > 0, true);
        assert_eq!(histogram[1] > 0, true, "a probe is reachable");
        assert_eq!(histogram[2] > 0, true, "an inward action is reachable");
        assert_eq!(histogram[4] > 0, true, "a halt is reachable");
        assert_eq!(histogram[5] > 0, true, "no action is reachable");
    }

    /// The ten reach rows of `02` §2.5.3, one per named cell of the operator.
    #[test]
    fn the_ten_reach_rows_answer_as_the_table_says() {
        let geometry = |l: i64, u: i64, c: i64, reversible: bool| Geometry {
            name: "reach",
            l,
            u,
            c,
            reversible,
        };
        let plain = |g: &Geometry, tts: i64, ttr: i64| {
            raw(g, tts, ttr, false, false, false, sealed_instrument())
        };
        let incomplete = Geometry {
            name: "reach",
            l: 200,
            u: 400,
            c: 120,
            reversible: true,
        };

        // 1 — an incomplete instrument quarantines at the absent slot.
        let regime = step(&raw(
            &incomplete,
            10,
            10,
            false,
            false,
            false,
            Instrument::Incomplete(MissingSlot::new(3).unwrap()),
        ))
        .unwrap();
        assert_eq!(regime.tag(), 3);
        assert_eq!(regime.missing().unwrap().get(), 3);

        // 2 — survival latency halts, cause 0.
        let g = geometry(200, 400, 120, true);
        let regime = step(&plain(&g, 100, 200)).unwrap();
        assert_eq!(regime.tag(), 4);
        assert_eq!(regime.cause(), Some(HaltCause::SurvivalLatency));

        // 3 — the vital floor turns the step inward, constraint 0.
        let regime = step(&raw(&g, 10, 10, true, false, false, sealed_instrument())).unwrap();
        assert_eq!(regime.tag(), 2);
        assert_eq!(regime.constraint(), Some(Constraint::VitalFloor));

        // 4 — the governance floor turns the step inward, constraint 1.
        let regime = step(&raw(&g, 10, 10, false, true, false, sealed_instrument())).unwrap();
        assert_eq!(regime.tag(), 2);
        assert_eq!(regime.constraint(), Some(Constraint::GovernanceFloor));

        // 5 — a corrupt channel halts, cause 1.
        let regime = step(&raw(&g, 10, 10, false, false, true, sealed_instrument())).unwrap();
        assert_eq!(regime.tag(), 4);
        assert_eq!(regime.cause(), Some(HaltCause::GovernanceChannel));

        // 6 — the buffered region.
        let g1 = geometry(400, 800, 200, true);
        assert_eq!(step(&plain(&g1, 10, 10)).unwrap().tag(), 0);

        // 7 — the reversible straddle probes at the truncated fraction.
        let g2 = geometry(120, 400, 200, true);
        let regime = step(&plain(&g2, 10, 10)).unwrap();
        assert_eq!(regime.tag(), 1);
        assert_eq!(regime.fraction(), Some(714_285));

        // 8 — the irreversible straddle halts, cause 2.
        let g3 = geometry(120, 400, 200, false);
        let regime = step(&plain(&g3, 10, 10)).unwrap();
        assert_eq!(regime.tag(), 4);
        assert_eq!(regime.cause(), Some(HaltCause::IrreversibleProbeRefused));

        // 9 — the region below the cost is no action.
        let g4 = geometry(50, 100, 200, true);
        assert_eq!(step(&plain(&g4, 10, 10)).unwrap().tag(), 5);

        // 10 — licensed equality boundaries (`36`, `41` §5, `44`): L == C < U is BufferedAction, U <= C is NoAction.
        let g5 = geometry(200, 400, 200, true);
        assert_eq!(step(&plain(&g5, 10, 10)), Ok(Regime::BufferedAction));
        let g5b = geometry(200, 400, 400, true);
        assert_eq!(step(&plain(&g5b, 10, 10)), Ok(Regime::NoAction));
        let g5c = geometry(200, 200, 200, true);
        assert_eq!(step(&plain(&g5c, 10, 10)), Ok(Regime::NoAction));
    }

    /// The three overlap rows: the guards are ordered, and the order is the answer.
    #[test]
    fn the_overlap_rows_answer_by_precedence() {
        let g = Geometry {
            name: "overlap",
            l: 200,
            u: 400,
            c: 120,
            reversible: true,
        };

        // Floor and latency: latency wins, cause 0.
        let regime = step(&raw(&g, 5, 10, true, false, false, sealed_instrument())).unwrap();
        assert_eq!(regime.tag(), 4);
        assert_eq!(regime.cause(), Some(HaltCause::SurvivalLatency));

        // Floor and corruption: the channel wins, cause 1.
        let regime = step(&raw(&g, 10, 10, true, false, true, sealed_instrument())).unwrap();
        assert_eq!(regime.tag(), 4);
        assert_eq!(regime.cause(), Some(HaltCause::GovernanceChannel));

        // All four at once: survival and governance channel report together.
        let regime = step(&raw(&g, 5, 10, true, true, true, sealed_instrument())).unwrap();
        assert_eq!(regime.tag(), 4);
        assert_eq!(regime.cause(), Some(HaltCause::SurvivalAndGovernance));
    }

    #[test]
    fn missing_clocks_and_reopen_sealed_quarantine_or_escalate_under_corruption() {
        let g = Geometry {
            name: "buffered",
            l: 200,
            u: 400,
            c: 120,
            reversible: true,
        };

        // Missing causal/rate clocks trigger Quarantine(T5) even if raw tts < ttr_eff.
        let mut s = State::seal(
            ControlTape::Attested,
            Bounds::new(g.l, g.u, 1000, 0),
            Capacity::new(g.c, 0, 1000),
            Guards::new(sealed_instrument()),
            Params::new(5, 10, true, false, false, false, Thresholds::new(0, 0))
                .with_clocks_missing(true),
        )
        .unwrap();
        assert_eq!(
            step(&s),
            Ok(Regime::Quarantine {
                missing: MissingSlot::T5
            })
        );

        // Negative clocks quarantine at T5 (not buffered even when tts >= ttr_eff).
        s = State::seal(
            ControlTape::Attested,
            Bounds::new(g.l, g.u, 1000, 0),
            Capacity::new(g.c, 0, 1000),
            Guards::new(sealed_instrument()),
            Params::new(10, -5, true, false, false, false, Thresholds::new(0, 0)),
        )
        .unwrap();
        assert_eq!(
            step(&s),
            Ok(Regime::Quarantine {
                missing: MissingSlot::T5
            })
        );

        // Concurrent corruption with missing/negative clocks escalates to GovernanceChannel halt.
        s = State::seal(
            ControlTape::Attested,
            Bounds::new(g.l, g.u, 1000, 0),
            Capacity::new(g.c, 0, 1000),
            Guards::new(sealed_instrument()),
            Params::new(100, 100, true, false, false, true, Thresholds::new(0, 0))
                .with_clocks_missing(true),
        )
        .unwrap();
        assert_eq!(
            step(&s),
            Ok(Regime::AlgedonicHalt {
                cause: HaltCause::GovernanceChannel
            })
        );

        // Counter-inductive reopening with intact clocks and sealed instrument quarantines at T1,
        // even ahead of sealed governance-channel corruption (`41` §5).
        s = State::seal(
            ControlTape::Attested,
            Bounds::new(g.l, g.u, 1000, 0),
            Capacity::new(g.c, 0, 1000),
            Guards::new(sealed_instrument()),
            Params::new(100, 100, true, false, false, true, Thresholds::new(0, 0))
                .with_reopen_sealed(true),
        )
        .unwrap();
        assert_eq!(
            step(&s),
            Ok(Regime::Quarantine {
                missing: MissingSlot::T1
            })
        );

        // Calibrated non-negative survival deficit (tts < ttr_eff) preempts reopen_sealed & Incomplete.
        s = State::seal(
            ControlTape::Attested,
            Bounds::new(g.l, g.u, 1000, 0),
            Capacity::new(g.c, 0, 1000),
            Guards::new(Instrument::Incomplete(MissingSlot::new(3).unwrap())),
            Params::new(5, 10, true, false, false, false, Thresholds::new(0, 0))
                .with_reopen_sealed(true),
        )
        .unwrap();
        assert_eq!(
            step(&s),
            Ok(Regime::AlgedonicHalt {
                cause: HaltCause::SurvivalLatency
            })
        );
    }
}
