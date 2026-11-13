//! The diagnostic fold over a finished trajectory, `01` §4.4 and `42` §1.4.
//!
//! # What this function is
//!
//! `diagnose` folds the whole trajectory and reports **which plane failed**. It is
//! not a control law and it is not an input to any action: [`Diagnosis`] has private
//! fields, no public constructor, no `Debug`, no `Deref`, no `Into<State>`, and
//! neither [`crate::step`] nor [`crate::accept`] accepts it. Its only observation is
//! [`Diagnosis::observe`], which hands back plain numbers in a copy.
//!
//! # Missing propagates, it does not average away
//!
//! A reading that is absent poisons the fold of its axis and of the binding axis, and
//! an absent binding axis is reported as "none" rather than as the smallest surviving
//! axis. Averaging over the survivors would be the aggregate the triad forbids, at the
//! diagnostic level.
//!
//! # The absorbed trajectory
//!
//! `42` §1.1.2 defines the vital coordinate as a barrier, not as a number: after
//! absorption the effective capacity is zero for every remaining shift. The absorbed
//! step therefore carries no vital value at all, and inventing a large one — or a
//! zero — would be fabricating data. The trajectory records `absorbed`, the ruin step
//! and the terminal reserve; the reader branches on `absorbed` before touching the
//! vital coordinate, and the vital fold skips absorbed steps instead of reading a
//! number they do not have. An absorbed trajectory is complete, not truncated: the
//! loss is the remainder of the horizon (`42` §1.4, line 231).

use crate::{Axis, Score};

/// One step of a finished trajectory: the three readings, the absorption flag, the reserve.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct StepReading {
    /// The epistemic reading at this step.
    pub s_epi: Score,
    /// The vital reading at this step, absent when the step is absorbed.
    pub s_vital: Score,
    /// The governance reading at this step.
    pub s_gov: Score,
    /// Whether the absorbing barrier was crossed at this step.
    pub absorbed: bool,
    /// The reserve at the end of this step.
    pub reserve: i64,
}

impl StepReading {
    /// A reading from its five quantities.
    pub const fn new(
        s_epi: Score,
        s_vital: Score,
        s_gov: Score,
        absorbed: bool,
        reserve: i64,
    ) -> StepReading {
        StepReading {
            s_epi,
            s_vital,
            s_gov,
            absorbed,
            reserve,
        }
    }
}

/// A finished trajectory: a borrowed slice of readings, sealed as one object.
///
/// Borrowing rather than owning keeps the fold allocation-free, which is what lets the
/// same code run in the core and on the wire; the lifetime is the caller's and the
/// trajectory cannot outlive the slice it was sealed from.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Trajectory<'a> {
    steps: &'a [StepReading],
}

impl<'a> Trajectory<'a> {
    /// Seal a slice of readings as a trajectory. Cannot fail: a trajectory is a record.
    pub const fn seal(steps: &'a [StepReading]) -> Trajectory<'a> {
        Trajectory { steps }
    }

    /// The number of steps.
    pub const fn len(&self) -> usize {
        self.steps.len()
    }

    /// Whether the trajectory has no step.
    pub const fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }
}

/// The diagnostic fold. Opaque on purpose.
///
/// There is no public constructor and no field access. `Diagnosis` is deliberately
/// not `Copy`-observable through a field, not `Debug`, not `Serialize`, and not
/// convertible into [`crate::State`]: the separation between the diagnostic verdict
/// and the action is a property of the type, checked by the compiler, not a
/// convention of the caller.
///
/// It holds no method that returns a [`crate::Regime`] and no method that returns a
/// [`crate::ActionSet`], so no path from this object to an action exists at all.
#[derive(Clone, Copy)]
pub struct Diagnosis {
    binding_axis: Option<Axis>,
    absorbed: bool,
    ruin_step: Option<u32>,
    terminal_reserve: Score,
    s_epi: Score,
    s_vital: Score,
    s_gov: Score,
}

/// A copy of what a diagnosis reports, in plain numbers.
///
/// The only way out of a [`Diagnosis`]. Every field is a value, no reference, no
/// handle back into the trajectory, and nothing here is accepted by [`crate::step`].
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct DiagnosisView {
    /// The axis the minimum names, or `None` when an axis that must be read is absent.
    pub binding_axis: Option<Axis>,
    /// Whether the trajectory crossed the absorbing barrier.
    pub absorbed: bool,
    /// The step at which the barrier was crossed, if it was.
    pub ruin_step: Option<u32>,
    /// The reserve at the end of the trajectory, absent for an empty one.
    pub terminal_reserve: Score,
    /// The epistemic coordinate of the fold.
    pub s_epi: Score,
    /// The vital coordinate of the fold.
    pub s_vital: Score,
    /// The governance coordinate of the fold.
    pub s_gov: Score,
}

impl Diagnosis {
    /// The observable content of this diagnosis.
    pub fn observe(&self) -> DiagnosisView {
        DiagnosisView {
            binding_axis: self.binding_axis,
            absorbed: self.absorbed,
            ruin_step: self.ruin_step,
            terminal_reserve: self.terminal_reserve,
            s_epi: self.s_epi,
            s_vital: self.s_vital,
            s_gov: self.s_gov,
        }
    }
}

/// Fold one coordinate over the selected steps; absence poisons the fold.
fn fold_readings<'a, I>(readings: I) -> Score
where
    I: Iterator<Item = Score> + 'a,
{
    let mut best: Option<i64> = None;
    for score in readings {
        match score {
            Score::Missing => return Score::Missing,
            Score::Present(v) => {
                best = Some(match best {
                    Some(current) if current <= v => current,
                    _ => v,
                });
            }
        }
    }
    match best {
        Some(v) => Score::Present(v),
        None => Score::Missing,
    }
}

/// The binding axis: the minimum of the axes that must be read, ties to the earlier.
///
/// The order of the axes is the order of the fold — epistemic, vital, governance — and
/// a tie goes to the earlier one, which is a declared convention and not a derivation
/// from the book. After absorption the vital coordinate is not read at all, because the
/// barrier has no value to compare (`01` §4.4, L4).
fn binding_axis(absorbed: bool, s_epi: Score, s_vital: Score, s_gov: Score) -> Option<Axis> {
    let read: &[(Axis, Score)] = if absorbed {
        &[(Axis::Epistemic, s_epi), (Axis::Governance, s_gov)]
    } else {
        &[
            (Axis::Epistemic, s_epi),
            (Axis::Vital, s_vital),
            (Axis::Governance, s_gov),
        ]
    };
    let mut best: Option<(Axis, i64)> = None;
    for (axis, score) in read {
        match score {
            Score::Missing => return None,
            Score::Present(v) => match best {
                Some((_, current)) if current <= *v => {}
                _ => best = Some((*axis, *v)),
            },
        }
    }
    match best {
        Some((axis, _)) => Some(axis),
        None => None,
    }
}

/// The diagnostic fold: `Trajectory -> Diagnosis`, total, never fails.
///
/// ```
/// use stoch_core::{diagnose, Axis, Score, StepReading, Trajectory};
///
/// let steps = [
///     StepReading::new(
///         Score::Present(500_000),
///         Score::Present(600_000),
///         Score::Present(700_000),
///         false,
///         1_000,
///     ),
/// ];
/// let view = diagnose(&Trajectory::seal(&steps)).observe();
/// assert_eq!(view.binding_axis, Some(Axis::Epistemic));
/// assert_eq!(view.s_epi, Score::Present(500_000));
/// assert_eq!(view.terminal_reserve, Score::Present(1_000));
/// assert!(!view.absorbed);
/// ```
///
/// An absorbed run reports the ruin step and the terminal reserve, and reads no vital
/// value:
///
/// ```
/// use stoch_core::{diagnose, Axis, Score, StepReading, Trajectory};
///
/// let steps = [
///     StepReading::new(Score::Present(400_000), Score::Present(300_000), Score::Present(600_000), false, 900),
///     StepReading::new(Score::Present(400_000), Score::Missing, Score::Present(600_000), true, 0),
///     StepReading::new(Score::Present(400_000), Score::Missing, Score::Present(600_000), true, 0),
/// ];
/// let view = diagnose(&Trajectory::seal(&steps)).observe();
/// assert!(view.absorbed);
/// assert_eq!(view.ruin_step, Some(1));
/// // The two absorbed steps contribute no vital value at all — not a zero.
/// assert_eq!(view.s_vital, Score::Present(300_000));
/// // The vital axis is not read after absorption, so it cannot bind.
/// assert_eq!(view.binding_axis, Some(Axis::Epistemic));
/// assert_eq!(view.terminal_reserve, Score::Present(0));
/// ```
pub fn diagnose(trajectory: &Trajectory<'_>) -> Diagnosis {
    let steps = trajectory.steps;

    // The first absorbed step is the ruin step; the last reserve is the terminal one.
    let mut absorbed = false;
    let mut ruin_step: Option<u32> = None;
    for (index, step) in steps.iter().enumerate() {
        if step.absorbed && !absorbed {
            absorbed = true;
            // The record's step index is a `u32`; a trajectory longer than that
            // saturates rather than wrapping, because a wrapped index would name the
            // wrong step and a saturating one names the last representable step. The
            // position is converted, never incremented.
            ruin_step = Some(u32::try_from(index).unwrap_or(u32::MAX));
        }
    }

    let terminal_reserve = match steps.last() {
        Some(step) => Score::Present(step.reserve),
        None => Score::Missing,
    };

    let s_epi = fold_readings(steps.iter().map(|s| s.s_epi));
    let s_gov = fold_readings(steps.iter().map(|s| s.s_gov));
    // The vital coordinate of an absorbed step is a barrier, not a number: skipping
    // those steps keeps the fold free of a value that does not exist.
    let s_vital = fold_readings(steps.iter().filter(|s| !s.absorbed).map(|s| s.s_vital));

    let binding = binding_axis(absorbed, s_epi, s_vital, s_gov);

    Diagnosis {
        binding_axis: binding,
        absorbed,
        ruin_step,
        terminal_reserve,
        s_epi,
        s_vital,
        s_gov,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn one(epi: Score, vital: Score, gov: Score, absorbed: bool) -> [StepReading; 1] {
        [StepReading::new(epi, vital, gov, absorbed, 10)]
    }

    #[test]
    fn the_binding_axis_is_the_minimum() {
        let steps = one(
            Score::Present(500_000),
            Score::Present(600_000),
            Score::Present(700_000),
            false,
        );
        let view = diagnose(&Trajectory::seal(&steps)).observe();
        assert_eq!(view.binding_axis, Some(Axis::Epistemic));

        let steps = one(
            Score::Present(700_000),
            Score::Present(400_000),
            Score::Present(600_000),
            false,
        );
        assert_eq!(
            diagnose(&Trajectory::seal(&steps)).observe().binding_axis,
            Some(Axis::Vital)
        );

        let steps = one(
            Score::Present(600_000),
            Score::Present(700_000),
            Score::Present(300_000),
            false,
        );
        assert_eq!(
            diagnose(&Trajectory::seal(&steps)).observe().binding_axis,
            Some(Axis::Governance)
        );
    }

    #[test]
    fn a_tie_goes_to_the_earlier_axis() {
        let steps = one(
            Score::Present(400_000),
            Score::Present(400_000),
            Score::Present(900_000),
            false,
        );
        assert_eq!(
            diagnose(&Trajectory::seal(&steps)).observe().binding_axis,
            Some(Axis::Epistemic)
        );
    }

    #[test]
    fn an_absent_vital_reading_removes_the_binding_axis() {
        let steps = one(
            Score::Present(500_000),
            Score::Missing,
            Score::Present(600_000),
            false,
        );
        let view = diagnose(&Trajectory::seal(&steps)).observe();
        assert_eq!(view.binding_axis, None);
        assert_eq!(view.s_vital, Score::Missing);
    }

    #[test]
    fn an_absorbed_run_reads_no_vital_value_and_still_binds() {
        let steps = one(
            Score::Present(400_000),
            Score::Missing,
            Score::Present(600_000),
            true,
        );
        let view = diagnose(&Trajectory::seal(&steps)).observe();
        assert!(view.absorbed);
        assert_eq!(view.ruin_step, Some(0));
        assert_eq!(view.binding_axis, Some(Axis::Epistemic));
        assert_eq!(view.s_vital, Score::Missing);
    }

    #[test]
    fn an_absent_everything_binds_nothing() {
        let steps = one(Score::Missing, Score::Missing, Score::Missing, true);
        let view = diagnose(&Trajectory::seal(&steps)).observe();
        assert_eq!(view.binding_axis, None);
        assert_eq!(view.s_epi, Score::Missing);
        assert_eq!(view.s_gov, Score::Missing);
    }

    #[test]
    fn an_empty_trajectory_is_not_a_zero() {
        let view = diagnose(&Trajectory::seal(&[])).observe();
        assert_eq!(view.binding_axis, None);
        assert!(!view.absorbed);
        assert_eq!(view.ruin_step, None);
        assert_eq!(view.terminal_reserve, Score::Missing);
        assert_eq!(view.s_epi, Score::Missing);
    }

    #[test]
    fn the_first_absorbed_step_is_the_ruin_step() {
        let steps = [
            StepReading::new(
                Score::Present(1),
                Score::Present(1),
                Score::Present(1),
                false,
                30,
            ),
            StepReading::new(
                Score::Present(1),
                Score::Missing,
                Score::Present(1),
                true,
                20,
            ),
            StepReading::new(
                Score::Present(1),
                Score::Missing,
                Score::Present(1),
                true,
                0,
            ),
        ];
        let view = diagnose(&Trajectory::seal(&steps)).observe();
        assert_eq!(view.ruin_step, Some(1));
        assert_eq!(view.terminal_reserve, Score::Present(0));
    }

    #[test]
    fn missing_poisons_rather_than_averages() {
        let steps = [
            StepReading::new(
                Score::Present(900_000),
                Score::Present(900_000),
                Score::Present(900_000),
                false,
                10,
            ),
            StepReading::new(
                Score::Missing,
                Score::Present(900_000),
                Score::Present(900_000),
                false,
                10,
            ),
        ];
        let view = diagnose(&Trajectory::seal(&steps)).observe();
        assert_eq!(view.s_epi, Score::Missing);
        assert_eq!(view.binding_axis, None);
        assert_eq!(view.s_vital, Score::Present(900_000));
    }
}
