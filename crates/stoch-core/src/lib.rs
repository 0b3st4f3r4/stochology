//! Deterministic formal core of the Stochology framework.
//!
//! This crate is the executable form of three objects only:
//!
//! * [`step`] — the per-step canonical operator, `State -> Regime`, total over the
//!   finite state grid (2^5 x 3 = 96 combinations of the five booleans of the state
//!   and the three instrument conditions);
//! * [`accept`] — the ordered five-stage acceptance predicate over a finished
//!   trajectory, `&Transcript -> Verdict`, in exactly the order fixed by
//!   Appendix B.1: syntactic skeleton, axial coverage, finiteness, Leontief fold,
//!   declared frontier band;
//! * [`diagnose`] — the diagnostic fold over a finished trajectory,
//!   `&Trajectory -> Diagnosis`, which reports *which plane failed* and is never
//!   an input to any action.
//!
//! # Verdict names
//!
//! The four verdicts are the ones fixed by Appendix B.1 and `01` §3 —
//! [`Verdict::Interior`], [`Verdict::Boundary`], [`Verdict::Collapse`],
//! [`Verdict::Unparsable`] — because the wire tags `0..3` of `02` §2.3 and of
//! the normative fixture are fixed to that order. They are not renamed: a rename would
//! break tag conformance against the normative fixture, which is the acceptance
//! criterion of this phase.
//!
//! # Separation invariant
//!
//! A [`Diagnosis`] is opaque: it has no public constructor, no field access, no
//! `Deref`, no `Debug`, and its only observation is [`Diagnosis::observe`], which
//! returns a copy of plain numbers. It is not accepted by [`step`] nor by
//! [`accept`], so a diagnostic verdict cannot enter a decision. The guarantee is
//! stated as a set of `compile_fail` doctests on this page and on the types
//! involved.
//!
//! # Arithmetic
//!
//! All quantities are integers. Fixed point scale is [`SCALE`]
//! (1_000_000 = 100 %), every absolute value is bounded by [`MAX_ABS`], and every
//! operation that can escape the bound goes through a `checked_*` primitive or is
//! proven not to by construction. Binary floating point is not used anywhere in
//! the source; a test scans this tree and fails if the type names appear.
//!
//! # What this crate is not
//!
//! It is not the ABI. There is no `extern "C"`, no `cdylib`, no `unsafe` block, no
//! `wasm-bindgen` and no JSON: those belong to the wasm phase, which the plan gated
//! on real consumers and left unbuilt. That plan is archived outside the repository;
//! the document of record for what shipped is this crate and its README. `unsafe` is
//! denied crate-wide, and a test asserts that the count of `unsafe` regions is zero.
//!
//! # Example
//!
//! ```
//! use stoch_core::{probe_fraction, Bounds, Error};
//!
//! // A reversible probe inside the extent: (400 - 200) / (400 - 120) = 40/56.
//! assert_eq!(probe_fraction(Bounds::new(120, 400, 1000, 0), 200), Ok(714_285));
//! // The same probe with no room left truncates to zero: underflow, not a zero shot.
//! assert_eq!(
//!     probe_fraction(Bounds::new(0, 1_000_000_000, 1, 0), 999_999_999),
//!     Err(Error::ProbeUnderflow)
//! );
//! ```
//!
//! A diagnosis cannot be observed as a step parameter:
//!
//! ```compile_fail
//! use stoch_core::{diagnose, step, Score, StepReading, Trajectory};
//!
//! let readings = [StepReading {
//!     s_epi: Score::Present(1),
//!     s_vital: Score::Present(1),
//!     s_gov: Score::Present(1),
//!     absorbed: false,
//!     reserve: 10,
//! }];
//! let trajectory = Trajectory::seal(&readings);
//! let diagnosis = diagnose(&trajectory);
//! // `step` takes a `&State`; a diagnosis is not one, and no conversion exists.
//! let _ = step(&diagnosis);
//! ```
//!
//! And it has no public constructor and no field access:
//!
//! ```compile_fail
//! use stoch_core::Diagnosis;
//!
//! let diagnosis = Diagnosis {
//!     absorbed: false,
//!     ruin_step: None,
//!     binding_axis: None,
//!     s_epi: stoch_core::Score::Missing,
//!     s_vital: stoch_core::Score::Missing,
//!     s_gov: stoch_core::Score::Missing,
//!     terminal_reserve: 0,
//! };
//! ```
//!
//! ```compile_fail
//! use stoch_core::{diagnose, Score, StepReading, Trajectory};
//!
//! let readings = [StepReading {
//!     s_epi: Score::Present(1),
//!     s_vital: Score::Present(1),
//!     s_gov: Score::Present(1),
//!     absorbed: false,
//!     reserve: 10,
//! }];
//! let trajectory = Trajectory::seal(&readings);
//! let diagnosis = diagnose(&trajectory);
//! // The fields of a diagnosis are private.
//! let _ = diagnosis.absorbed;
//! ```

#![no_std]
#![deny(unsafe_code)]
#![deny(missing_docs)]
#![deny(clippy::float_arithmetic)]
#![forbid(clippy::cast_possible_truncation)]

#[cfg(test)]
extern crate std;

mod accept;
mod diagnose;
mod sha256;
mod step;

pub use accept::{
    accept, Assessment, AxialReadings, AxisReading, Budget, Declared, Sentence, Transcript,
    UnparsableReason, Verdict,
};
pub use diagnose::{diagnose, Diagnosis, DiagnosisView, StepReading, Trajectory};
pub use step::{probe_fraction, step};

/// Fixed point scale: 1_000_000 units per whole quantity (100 %).
///
/// `01` §1.1. Every ratio in this crate is an integer number of millionths.
pub const SCALE: i64 = 1_000_000;

/// Absolute bound on every quantity that crosses this boundary.
///
/// `01` §1.1 and `02` §2.1: `|x| <= 1_000_000_000`. A quantity outside the bound
/// is not clamped; the operation that would produce it returns
/// [`Error::ScaleOverflow`].
pub const MAX_ABS: i64 = 1_000_000_000;

/// Sentinel used by the normative fixture for "extent not provided".
///
/// It is a *source* encoding, valid only in [`Bounds`] as read by
/// [`State::seal`] and [`Transcript::seal`]; it never appears in a sealed object.
pub const EXTENT_NOT_PROVIDED: i64 = -1;

/// The closed error code set of the core, `01` §7.
///
/// Codes are the negative integers `-1 .. -12`, in the order of the specification.
/// `-13` is reserved and unused; `-14` (`E_CALLER_CONTRACT`) belongs to the ABI
/// layer and is not part of this crate.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Error {
    /// `-1`: an aggregate was asked for where the minimum is defined.
    AggregateForbidden,
    /// `-2`: `l > u` — inverted extent.
    BoundsInverted,
    /// `-3`: `c <= 0` — a capacity that cannot fund an action.
    CapacityNonPositive,
    /// `-4`: `r < 0` — a negative reserve.
    ReserveNegative,
    /// `-5`: a fixed point quantity escaped [`MAX_ABS`].
    ScaleOverflow,
    /// `-6`: an extent was required and the sentinel [`EXTENT_NOT_PROVIDED`] was read.
    ExtentMissing,
    /// `-7`: the control tape was not attested.
    TapeUnsealed,
    /// `-8`: an axis of the triad was left unguarded where a verdict needs it.
    AxisUnguarded,
    /// `-9`: the probe fraction truncated to zero.
    ProbeUnderflow,
    /// `-10`: a boundary verdict was claimed without a licence, or a degenerate
    /// extent was presented as a decision point.
    BoundaryUnlicensed,
    /// `-11`: the declared site and the declared proposition belong to different axes.
    SitePropositionMismatch,
    /// `-12`: the ABI record does not match the contract (reserved for the wire layer).
    AbiMismatch,
}

impl Error {
    /// The closed numeric code of this error, `-1 .. -12`.
    pub const fn code(self) -> i32 {
        match self {
            Error::AggregateForbidden => -1,
            Error::BoundsInverted => -2,
            Error::CapacityNonPositive => -3,
            Error::ReserveNegative => -4,
            Error::ScaleOverflow => -5,
            Error::ExtentMissing => -6,
            Error::TapeUnsealed => -7,
            Error::AxisUnguarded => -8,
            Error::ProbeUnderflow => -9,
            Error::BoundaryUnlicensed => -10,
            Error::SitePropositionMismatch => -11,
            Error::AbiMismatch => -12,
        }
    }
}

/// A reading of one axis: either a present integer quantity or an explicit absence.
///
/// Absence is not zero. Zero is a measurement; `Missing` is the record that no
/// measurement exists, and it poisons the fold rather than contributing to it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Score {
    /// A present quantity, inside the sealed range `0 ..= SCALE`.
    ///
    /// The range is not a comment: both doors impose it. [`State::seal`] refuses a
    /// guard reading outside it and [`crate::Transcript::seal`] refuses an axis's
    /// weakest reading outside it, both with [`Error::ScaleOverflow`]. A quantity
    /// above [`SCALE`] on a margin is not a large margin, it is a margin on a scale
    /// nobody declared.
    Present(i64),
    /// No measurement exists on this axis for this step.
    Missing,
}

impl Score {
    /// Returns the quantity, if present.
    pub const fn value(self) -> Option<i64> {
        match self {
            Score::Present(v) => Some(v),
            Score::Missing => None,
        }
    }

    /// Returns `true` when a measurement exists.
    pub const fn is_present(self) -> bool {
        matches!(self, Score::Present(_))
    }
}

/// The three axes of the Nuclear Triad, in the fixed order of the minimum.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Axis {
    /// The epistemic margin.
    Epistemic,
    /// The vital margin.
    Vital,
    /// The institutional governance margin.
    Governance,
}

impl Axis {
    /// Position of the axis in the fold, `1 .. 3` (0 is the wire sentinel for "none").
    pub const fn code(self) -> u8 {
        match self {
            Axis::Epistemic => 1,
            Axis::Vital => 2,
            Axis::Governance => 3,
        }
    }
}

/// The seven guard sites of Appendix E, `T1 .. T7`.
///
/// The cardinality is not free: it is the number of transits of the mediation
/// circuit, `3 * 2 + 1 = 7`. A site belongs to exactly one axis.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Site {
    /// Site `T1`, epistemic.
    T1,
    /// Site `T2`, epistemic.
    T2,
    /// Site `T3`, epistemic.
    T3,
    /// Site `T4`, vital.
    T4,
    /// Site `T5`, vital.
    T5,
    /// Site `T6`, governance.
    T6,
    /// Site `T7`, governance.
    T7,
}

impl Site {
    /// The site with the given wire tag, `1 .. 7`.
    pub const fn from_code(code: u8) -> Option<Site> {
        match code {
            1 => Some(Site::T1),
            2 => Some(Site::T2),
            3 => Some(Site::T3),
            4 => Some(Site::T4),
            5 => Some(Site::T5),
            6 => Some(Site::T6),
            7 => Some(Site::T7),
            _ => None,
        }
    }

    /// The wire tag of this site, `1 .. 7`.
    pub const fn code(self) -> u8 {
        match self {
            Site::T1 => 1,
            Site::T2 => 2,
            Site::T3 => 3,
            Site::T4 => 4,
            Site::T5 => 5,
            Site::T6 => 6,
            Site::T7 => 7,
        }
    }

    /// The axis this site guards.
    pub const fn axis(self) -> Axis {
        match self {
            Site::T1 | Site::T2 | Site::T3 => Axis::Epistemic,
            Site::T4 | Site::T5 => Axis::Vital,
            Site::T6 | Site::T7 => Axis::Governance,
        }
    }
}

/// The three collapse propositions of the preface, one per axis.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Proposition {
    /// Collapse proposition 1: no epistemic margin implies blind undersizing.
    P1Epistemic,
    /// Collapse proposition 2: no vital margin implies impotent lucidity before ruin.
    P2Vital,
    /// Collapse proposition 3: no institutional governance implies cynical capture.
    P3Governance,
}

impl Proposition {
    /// The proposition with the given wire tag, `1 .. 3`.
    pub const fn from_code(code: i64) -> Option<Proposition> {
        match code {
            1 => Some(Proposition::P1Epistemic),
            2 => Some(Proposition::P2Vital),
            3 => Some(Proposition::P3Governance),
            _ => None,
        }
    }

    /// The wire tag of this proposition, `1 .. 3`.
    pub const fn code(self) -> i64 {
        match self {
            Proposition::P1Epistemic => 1,
            Proposition::P2Vital => 2,
            Proposition::P3Governance => 3,
        }
    }

    /// The axis this proposition names.
    pub const fn axis(self) -> Axis {
        match self {
            Proposition::P1Epistemic => Axis::Epistemic,
            Proposition::P2Vital => Axis::Vital,
            Proposition::P3Governance => Axis::Governance,
        }
    }
}

/// Which floor was breached by an [`Regime::InwardAction`].
///
/// Two variants, `0` vital and `1` governance: the epistemic axis has no floor,
/// because a floor on it would be the point certainty the framework forbids. A
/// quarantine is not the variant that carries this payload: it carries the missing
/// slot instead, and a halt carries its cause.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Constraint {
    /// The vital floor was breached.
    VitalFloor,
    /// The governance floor was breached.
    GovernanceFloor,
}

impl Constraint {
    /// The wire tag of this constraint, `0` or `1`.
    pub const fn code(self) -> u8 {
        match self {
            Constraint::VitalFloor => 0,
            Constraint::GovernanceFloor => 1,
        }
    }
}

/// Why an algedonic halt fired.
///
/// Three variants, `0 .. 2`, in the precedence order in which they are tested:
/// survival latency, then governance channel, then irreversible probe.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum HaltCause {
    /// `0`: the survival time-to-serve is shorter than the recovery time-to-restore.
    SurvivalLatency,
    /// `1`: the governance channel is corrupt.
    GovernanceChannel,
    /// `2`: a probe was proposed that cannot be undone.
    IrreversibleProbeRefused,
}

impl HaltCause {
    /// The wire tag of this cause, `0 .. 2`.
    pub const fn code(self) -> u8 {
        match self {
            HaltCause::SurvivalLatency => 0,
            HaltCause::GovernanceChannel => 1,
            HaltCause::IrreversibleProbeRefused => 2,
        }
    }
}

/// One of the seven sites whose reading is absent.
///
/// Only `1 .. 7` exist; the type makes the eighth site unrepresentable rather than
/// merely rejected.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct MissingSlot(u8);

impl MissingSlot {
    /// The slot with the given tag, or `None` if it is not one of `1 .. 7`.
    pub const fn new(slot: u8) -> Option<MissingSlot> {
        if slot >= 1 && slot <= 7 {
            Some(MissingSlot(slot))
        } else {
            None
        }
    }

    /// The wire tag of the absent site, `1 .. 7`.
    pub const fn get(self) -> u8 {
        self.0
    }

    /// The site this slot denotes.
    pub const fn site(self) -> Site {
        match self.0 {
            1 => Site::T1,
            2 => Site::T2,
            3 => Site::T3,
            4 => Site::T4,
            5 => Site::T5,
            6 => Site::T6,
            _ => Site::T7,
        }
    }
}

/// The attestation of the control tape.
///
/// A tape that was not attested cannot be read as a permission: [`State::seal`]
/// refuses it with [`Error::TapeUnsealed`].
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ControlTape {
    /// The tape was attested and may be read.
    Attested,
    /// The tape was not attested.
    Unattested,
}

/// The contingency record against which a verdict is checked.
///
/// `accept` never reads this value. It exists in the transcript precisely so that
/// the non-commutation test can substitute it and observe that the verdict does not
/// move (Appendix B.1).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ContingencyTape {
    seal: u64,
}

impl ContingencyTape {
    /// A contingency record with the given seal.
    pub const fn new(seal: u64) -> ContingencyTape {
        ContingencyTape { seal }
    }

    /// The seal of this record.
    pub const fn seal(self) -> u64 {
        self.seal
    }
}

/// The seven readings of a sealed instrument, one per guard site.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct SevenReadings {
    /// The seven margins, indexed by site tag minus one.
    pub margins: [i64; 7],
}

impl SevenReadings {
    /// Seven readings from an array in site order.
    pub const fn new(margins: [i64; 7]) -> SevenReadings {
        SevenReadings { margins }
    }

    /// The reading at the given site.
    pub const fn at(self, site: Site) -> i64 {
        self.margins[(site.code() - 1) as usize]
    }
}

/// The instrument of observation: sealed with seven readings, or incomplete.
///
/// This is the only field of [`Guards`], and it is what the three instrument
/// conditions of the state grid are read from.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Instrument {
    /// Sealed: all seven sites were read.
    Sealed(SevenReadings),
    /// Incomplete: the named site has no reading.
    Incomplete(MissingSlot),
}

impl Instrument {
    /// `true` when the instrument is sealed.
    pub const fn is_sealed(self) -> bool {
        matches!(self, Instrument::Sealed(_))
    }
}

/// The guard state of the system.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Guards {
    /// The instrument of observation.
    pub instrument: Instrument,
}

impl Guards {
    /// Guards with the given instrument.
    pub const fn new(instrument: Instrument) -> Guards {
        Guards { instrument }
    }
}

/// The extent of the admissible region, in fixed point units.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Bounds {
    /// Lower edge `l`.
    pub l: i64,
    /// Upper edge `u`.
    pub u: i64,
    /// Effective sample size.
    pub n_eff: i64,
    /// Design effect, as an integer number of millionths.
    pub defect: i64,
}

impl Bounds {
    /// An extent from its four quantities.
    pub const fn new(l: i64, u: i64, n_eff: i64, defect: i64) -> Bounds {
        Bounds { l, u, n_eff, defect }
    }
}

/// The capacity to act, and the reserve it is drawn from.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Capacity {
    /// Capacity `c` available to this step.
    pub c: i64,
    /// The buffered amount `b`.
    pub b: i64,
    /// The reserve `r` held back.
    pub r: i64,
}

impl Capacity {
    /// A capacity from its three quantities.
    pub const fn new(c: i64, b: i64, r: i64) -> Capacity {
        Capacity { c, b, r }
    }
}

/// The two thresholds of an assessment, `01` §1.1 and Appendix B.1 §10.
///
/// They are declared, never inferred: the fold is compared against the threshold
/// the sentence declares, not against a constant of the crate.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Thresholds {
    /// The acceptance threshold: the fold below it collapses.
    pub accept_threshold: i64,
    /// The width of the frontier band above the threshold.
    pub band_width: i64,
}

impl Thresholds {
    /// The pair of thresholds.
    pub const fn new(accept_threshold: i64, band_width: i64) -> Thresholds {
        Thresholds {
            accept_threshold,
            band_width,
        }
    }
}

/// The declared parameters of a step, `01` §1.1.
///
/// The step reads all of them and writes none. The thresholds are declared here as
/// well as on the [`Assessment`] because `01` §1.1 puts them in `params`; `step`
/// does not read them, and `accept` reads the assessment's copy, because the
/// predicate never sees a state.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Params {
    /// Time to serve, in whole time units.
    pub tts: i64,
    /// Effective time to restore, in whole time units.
    pub ttr_eff: i64,
    /// Whether the proposed action can be undone.
    pub reversible: bool,
    /// Whether the vital floor is breached.
    pub vital_floor_breached: bool,
    /// Whether the governance floor is breached.
    pub gov_floor_breached: bool,
    /// Whether the governance channel is corrupt.
    pub gov_corrupt: bool,
    /// The thresholds, declared but unread by [`step`].
    pub thresholds: Thresholds,
}

impl Params {
    /// The seven declared parameters of a step.
    pub const fn new(
        tts: i64,
        ttr_eff: i64,
        reversible: bool,
        vital_floor_breached: bool,
        gov_floor_breached: bool,
        gov_corrupt: bool,
        thresholds: Thresholds,
    ) -> Params {
        Params {
            tts,
            ttr_eff,
            reversible,
            vital_floor_breached,
            gov_floor_breached,
            gov_corrupt,
            thresholds,
        }
    }
}

/// The sealed state of the system at one step.
///
/// The fields are private and [`State::seal`] is the only constructor: there is no
/// struct literal that produces a state, no `Default`, no `From`. Inside the crate
/// the step reads the fields directly (`s.bounds.l`, not `s.bounds().l()`); outward
/// there is no accessor at all, so a caller cannot reconstruct a state from a
/// verdict and cannot assemble one that skipped the seal.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct State {
    bounds: Bounds,
    capacity: Capacity,
    guards: Guards,
    params: Params,
}

impl State {
    /// Seal a state from an attested tape.
    ///
    /// The checks, in this order — each one names the code `01` §7 gives it:
    ///
    /// 1. the control tape is attested, else [`Error::TapeUnsealed`];
    /// 2. `l` is the sentinel [`EXTENT_NOT_PROVIDED`], else [`Error::ExtentMissing`];
    /// 3. every quantity is within [`MAX_ABS`], else [`Error::ScaleOverflow`];
    /// 4. `l <= u`, else [`Error::BoundsInverted`];
    /// 5. `c > 0`, else [`Error::CapacityNonPositive`];
    /// 6. `r >= 0`, else [`Error::ReserveNegative`];
    /// 7. every sealed margin is inside `0 ..= SCALE`, else [`Error::ScaleOverflow`].
    ///
    /// The seventh check is the one a [`Score::Present`] promises: an instrument
    /// reading is a margin on the declared scale, and the seal is where that claim
    /// stops being a comment. The tape is checked and then dropped: a sealed state
    /// records only the four objects of `01` §1.1.
    ///
    /// ```
    /// use stoch_core::{Bounds, Capacity, ControlTape, Guards, Instrument, MissingSlot,
    ///                  Params, SevenReadings, State, Thresholds, Error};
    ///
    /// let guard = Guards::new(Instrument::Incomplete(MissingSlot::new(3).unwrap()));
    /// let params = Params::new(100, 200, true, false, false, false, Thresholds::new(1, 1));
    /// let state = State::seal(
    ///     ControlTape::Attested,
    ///     Bounds::new(120, 400, 1000, 0),
    ///     Capacity::new(200, 0, 1000),
    ///     guard,
    ///     params,
    /// );
    /// assert!(state.is_ok());
    /// // An inverted extent is refused before any step is taken.
    /// let inverted = State::seal(
    ///     ControlTape::Attested,
    ///     Bounds::new(400, 200, 1000, 0),
    ///     Capacity::new(100, 0, 1000),
    ///     guard,
    ///     params,
    /// );
    /// assert_eq!(inverted.err(), Some(Error::BoundsInverted));
    /// ```
    pub fn seal(
        tape: ControlTape,
        bounds: Bounds,
        capacity: Capacity,
        guards: Guards,
        params: Params,
    ) -> Result<State, Error> {
        if tape != ControlTape::Attested {
            return Err(Error::TapeUnsealed);
        }
        if bounds.l == EXTENT_NOT_PROVIDED {
            return Err(Error::ExtentMissing);
        }
        for q in [
            bounds.l,
            bounds.u,
            bounds.n_eff,
            bounds.defect,
            capacity.c,
            capacity.b,
            capacity.r,
            params.tts,
            params.ttr_eff,
        ] {
            if q > MAX_ABS || q < -MAX_ABS {
                return Err(Error::ScaleOverflow);
            }
        }
        if bounds.l > bounds.u {
            return Err(Error::BoundsInverted);
        }
        if capacity.c <= 0 {
            return Err(Error::CapacityNonPositive);
        }
        if capacity.r < 0 {
            return Err(Error::ReserveNegative);
        }
        // Seventh check: a sealed instrument's readings are margins on the declared
        // scale (`Score::Present`), so a reading outside `0 ..= SCALE` is refused here
        // rather than compared as if it were a number of the same kind. An incomplete
        // instrument carries no reading and has nothing to check.
        if let Instrument::Sealed(readings) = guards.instrument {
            for margin in readings.margins {
                if !(0..=SCALE).contains(&margin) {
                    return Err(Error::ScaleOverflow);
                }
            }
        }
        Ok(State {
            bounds,
            capacity,
            guards,
            params,
        })
    }
}

/// The action set a regime permits, `01` §2.1.
///
/// Three variants and no fourth: an external action, an inward action, or nothing.
/// `Quarantine`, `InwardAction` and `AlgedonicHalt` all permit the same set
/// (`InwardOnly`), which is the proof that their distinction is diagnostic and not
/// semantic (`01` §9, L9). [`ActionSet::None`] is produced by exactly one variant,
/// [`Regime::NoAction`]; a value in this enum with no variant that produces it
/// would be a design error.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ActionSet {
    /// An external action is permitted.
    External,
    /// Only an inward action is permitted.
    InwardOnly,
    /// No action is permitted.
    None,
}

/// The canonical operator's output: the regime of action for this step.
///
/// Exactly six variants, tags `0 .. 5`, and [`Regime::NoAction`] is constructed in
/// exactly one site in the whole crate (`step`), which a test enforces by counting
/// the token in the source tree.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Regime {
    /// `0`: an action inside the buffered capacity, `L > C`.
    ///
    /// Regime I of `41` §5, line 304. The variant is a unit variant on purpose: the
    /// operator returns a regime, not a size, and the size of a full intervention is
    /// a quantity the book does not define (`01` §5, last paragraph).
    BufferedAction,
    /// `1`: a reversible fractional probe, `L < C < U`.
    ReversibleFractionalProbe {
        /// The probe fraction, in millionths of the capacity.
        fraction: i64,
    },
    /// `2`: an inward action that gives margin back.
    InwardAction {
        /// Which floor forced the inward action.
        breached: Constraint,
    },
    /// `3`: a quarantine while a site's reading is absent.
    Quarantine {
        /// The site whose reading is absent.
        missing: MissingSlot,
    },
    /// `4`: an algedonic halt.
    AlgedonicHalt {
        /// Why the halt fired.
        cause: HaltCause,
    },
    /// `5`: no action.
    NoAction,
}

impl Regime {
    /// The wire tag of this regime, `0 .. 5`.
    ///
    /// This is the single *reader* of [`Regime::NoAction`] outside its construction
    /// site; it is not a construction of it.
    pub const fn tag(self) -> u8 {
        match self {
            Regime::BufferedAction => 0,
            Regime::ReversibleFractionalProbe { .. } => 1,
            Regime::InwardAction { .. } => 2,
            Regime::Quarantine { .. } => 3,
            Regime::AlgedonicHalt { .. } => 4,
            Regime::NoAction => 5,
        }
    }

    /// The action set this regime permits, `01` §2.1.
    pub const fn permitted(self) -> ActionSet {
        match self {
            Regime::BufferedAction => ActionSet::External,
            Regime::ReversibleFractionalProbe { .. } => ActionSet::External,
            Regime::InwardAction { .. } => ActionSet::InwardOnly,
            Regime::Quarantine { .. } => ActionSet::InwardOnly,
            Regime::AlgedonicHalt { .. } => ActionSet::InwardOnly,
            Regime::NoAction => ActionSet::None,
        }
    }

    /// The constraint that forced an inward action, if this is one.
    pub const fn constraint(self) -> Option<Constraint> {
        match self {
            Regime::InwardAction { breached } => Some(breached),
            _ => None,
        }
    }

    /// The halt cause, if this is a halt.
    pub const fn cause(self) -> Option<HaltCause> {
        match self {
            Regime::AlgedonicHalt { cause } => Some(cause),
            _ => None,
        }
    }

    /// The absent site, if this is a quarantine.
    pub const fn missing(self) -> Option<MissingSlot> {
        match self {
            Regime::Quarantine { missing } => Some(missing),
            _ => None,
        }
    }

    /// The probe fraction in millionths, if this is a probe.
    pub const fn fraction(self) -> Option<i64> {
        match self {
            Regime::ReversibleFractionalProbe { fraction } => Some(fraction),
            _ => None,
        }
    }
}

/// Number of records in the normative fixture `vectors.kv`.
///
/// The file has 52 records, each one line, under a four-line comment header,
/// `01` §6.4. A record is a line that is neither blank nor a comment; the count is
/// of records, not of lines, because the fixture's spelling is not its content.
pub const VECTORS_DATA_ROWS: usize = 52;

/// The fixture's path, relative to the crate root, `01` §6.4.
pub const VECTORS_FILE: &str = "vectors.kv";

/// The normative fixture, embedded byte for byte.
///
/// `include_bytes!` and not a runtime read: the digest below is a compile-time
/// constant, so a fixture edited without editing the expectation fails the build.
pub const VECTORS_BYTES: &[u8] = include_bytes!("../vectors.kv");

/// The SHA-256 digest of the normative fixture, as 32 raw bytes.
pub const VECTORS_SHA256: [u8; 32] = sha256::digest(VECTORS_BYTES);

/// The expected digest, as lowercase hexadecimal, `01` §6.4.
pub const VECTORS_SHA256_HEX: &str =
    "efe2bad329e5727899e4c89b7989d4c9e92cc01c961ce896f71e7f1aed6bd3d0";

/// The SHA-256 of the embedded normative fixture, as 32 raw bytes.
///
/// This is the integrity anchor of `01` §6.4: the digest covers the whole file,
/// raw bytes, comment lines and final LF included, with no normalisation, no
/// reordering and no added separator. Any edit to any line invalidates it and
/// requires regeneration. A test recomputes the digest over the embedded bytes and
/// compares it against [`VECTORS_SHA256_HEX`].
pub const fn vectors_hash() -> [u8; 32] {
    VECTORS_SHA256
}

/// The same digest as [`vectors_hash`], in lowercase hexadecimal.
///
/// Sixty-four bytes, no allocation, no formatter.
pub fn vectors_hash_hex() -> [u8; 64] {
    sha256::hex(VECTORS_SHA256)
}

/// Panic handler for the wasm target only.
///
/// On the host the standard library provides the handler; on wasm32-unknown-unknown
/// nothing does, and `no_std` without a handler does not link. The body is a loop:
/// a panic in this crate is a bug in the trusted core, not a recoverable condition,
/// and there is no formatted output on the wasm surface.
#[cfg(all(target_arch = "wasm32", not(test)))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}

#[cfg(test)]
mod lib_tests {
    use super::*;

    #[test]
    fn scale_and_bound_are_the_normative_values() {
        assert_eq!(SCALE, 1_000_000);
        assert_eq!(MAX_ABS, 1_000_000_000);
        assert_eq!(SCALE * 1000, MAX_ABS);
    }

    #[test]
    fn error_codes_are_the_closed_set() {
        let codes = [
            Error::AggregateForbidden.code(),
            Error::BoundsInverted.code(),
            Error::CapacityNonPositive.code(),
            Error::ReserveNegative.code(),
            Error::ScaleOverflow.code(),
            Error::ExtentMissing.code(),
            Error::TapeUnsealed.code(),
            Error::AxisUnguarded.code(),
            Error::ProbeUnderflow.code(),
            Error::BoundaryUnlicensed.code(),
            Error::SitePropositionMismatch.code(),
            Error::AbiMismatch.code(),
        ];
        assert_eq!(codes, [-1, -2, -3, -4, -5, -6, -7, -8, -9, -10, -11, -12]);
        assert!(!codes.contains(&-13));
    }

    #[test]
    fn regime_tags_are_zero_to_five_and_actions_are_total() {
        let regimes = [
            (Regime::BufferedAction, 0, ActionSet::External),
            (
                Regime::ReversibleFractionalProbe { fraction: 1 },
                1,
                ActionSet::External,
            ),
            (
                Regime::InwardAction {
                    breached: Constraint::VitalFloor,
                },
                2,
                ActionSet::InwardOnly,
            ),
            (
                Regime::Quarantine {
                    missing: MissingSlot::new(1).unwrap(),
                },
                3,
                ActionSet::InwardOnly,
            ),
            (
                Regime::AlgedonicHalt {
                    cause: HaltCause::SurvivalLatency,
                },
                4,
                ActionSet::InwardOnly,
            ),
        ];
        let mut seen = [false; 6];
        let mut none_producers = 0;
        for (regime, tag, actions) in regimes {
            assert_eq!(regime.tag(), tag);
            assert_eq!(regime.permitted(), actions);
            seen[usize::from(tag)] = true;
            if actions == ActionSet::None {
                none_producers += 1;
            }
        }
        // The sixth variant is not written here: it has exactly one construction
        // site, and this table must not become a second one. It is obtained from the
        // operator and read through its accessors.
        let no_action = crate::step(&crate::State::seal(
            ControlTape::Attested,
            Bounds::new(100, 150, 1000, 0),
            Capacity::new(200, 0, 0),
            Guards::new(Instrument::Sealed(SevenReadings::new([1; 7]))),
            Params::new(10, 10, true, false, false, false, Thresholds::new(1, 1)),
        )
        .unwrap())
        .unwrap();
        assert_eq!(no_action.tag(), 5);
        assert_eq!(no_action.permitted(), ActionSet::None);
        seen[5] = true;
        none_producers += 1;
        assert_eq!(seen, [true; 6]);
        // `ActionSet::None` has exactly one producing variant (01 §2.2, restriction 3).
        assert_eq!(none_producers, 1);
    }

    #[test]
    fn sites_partition_into_three_axes() {
        let mut counts = [0usize; 3];
        for code in 1..=7u8 {
            let site = Site::from_code(code).expect("seven sites");
            assert_eq!(site.code(), code);
            counts[usize::from(site.axis().code()) - 1] += 1;
        }
        assert_eq!(counts, [3, 2, 2]);
        assert_eq!(Site::from_code(0), None);
        assert_eq!(Site::from_code(8), None);
    }

    #[test]
    fn proposition_and_site_axis_agree_on_the_pairs_the_fixture_declares() {
        for (site, prop) in [
            (Site::T1, Proposition::P1Epistemic),
            (Site::T2, Proposition::P1Epistemic),
            (Site::T3, Proposition::P1Epistemic),
            (Site::T4, Proposition::P2Vital),
            (Site::T5, Proposition::P2Vital),
            (Site::T6, Proposition::P3Governance),
            (Site::T7, Proposition::P3Governance),
        ] {
            assert_eq!(site.axis(), prop.axis(), "site {} / prop {}", site.code(), prop.code());
        }
    }

    #[test]
    fn missing_slot_has_seven_values_and_no_eighth() {
        assert_eq!(MissingSlot::new(0), None);
        assert_eq!(MissingSlot::new(1).map(MissingSlot::get), Some(1));
        assert_eq!(MissingSlot::new(7).map(MissingSlot::get), Some(7));
        assert_eq!(MissingSlot::new(8), None);
        for slot in 1..=7u8 {
            let m = MissingSlot::new(slot).unwrap();
            assert_eq!(m.site().code(), slot);
        }
    }

    #[test]
    fn score_absence_is_not_zero() {
        assert_eq!(Score::Present(0).value(), Some(0));
        assert!(Score::Present(0).is_present());
        assert_eq!(Score::Missing.value(), None);
        assert!(!Score::Missing.is_present());
    }

    #[test]
    fn digest_hex_matches_the_embedded_digest() {
        let hex = vectors_hash_hex();
        assert_eq!(core::str::from_utf8(&hex).unwrap(), VECTORS_SHA256_HEX);
        assert_eq!(vectors_hash().len(), 32);
        // 52 records under a comment header: the count is of records, not of lines.
        let text = core::str::from_utf8(VECTORS_BYTES).expect("the fixture is UTF-8");
        let records = text
            .lines()
            .filter(|line| {
                let line = line.trim();
                !line.is_empty() && !line.starts_with('#')
            })
            .count();
        assert_eq!(records, VECTORS_DATA_ROWS);
        assert_eq!(VECTORS_FILE, "vectors.kv");
    }

    #[test]
    fn the_state_door_refuses_a_margin_off_the_scale() {
        let seal = |margin: i64| {
            State::seal(
                ControlTape::Attested,
                Bounds::new(400, 800, 1000, 0),
                Capacity::new(200, 0, 1000),
                Guards::new(Instrument::Sealed(SevenReadings::new([margin; 7]))),
                Params::new(10, 10, true, false, false, false, Thresholds::new(100_000, 50_000)),
            )
        };

        // The value the review found sealing: nine trillion, twelve orders of
        // magnitude above the scale, which folded into a comfortable `Interior`.
        assert_eq!(seal(9_000_000_000_000).err(), Some(Error::ScaleOverflow));
        assert_eq!(seal(SCALE + 1).err(), Some(Error::ScaleOverflow));
        assert_eq!(seal(-1).err(), Some(Error::ScaleOverflow));
        // The scale itself, and zero, are margins and seal.
        assert!(seal(0).is_ok());
        assert!(seal(SCALE).is_ok());
        assert!(seal(500_000).is_ok());

        // One bad reading among seven is enough: the instrument is sealed as a whole.
        let mut readings = SevenReadings::new([500_000; 7]);
        readings.margins[6] = SCALE + 1;
        assert_eq!(
            State::seal(
                ControlTape::Attested,
                Bounds::new(400, 800, 1000, 0),
                Capacity::new(200, 0, 1000),
                Guards::new(Instrument::Sealed(readings)),
                Params::new(10, 10, true, false, false, false, Thresholds::new(100_000, 50_000)),
            )
            .err(),
            Some(Error::ScaleOverflow)
        );

        // An incomplete instrument carries no reading, so there is nothing to bound.
        assert!(State::seal(
            ControlTape::Attested,
            Bounds::new(400, 800, 1000, 0),
            Capacity::new(200, 0, 1000),
            Guards::new(Instrument::Incomplete(MissingSlot::new(3).unwrap())),
            Params::new(10, 10, true, false, false, false, Thresholds::new(100_000, 50_000)),
        )
        .is_ok());
    }
}
