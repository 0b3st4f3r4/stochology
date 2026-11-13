//! The acceptance predicate, Appendix B.1 lines 157-167.
//!
//! Five stages, in this order and no other. The order is the object under test: a
//! lower stage pre-empts every higher one, so a transcript that is ill-formed *and*
//! below the threshold is [`Verdict::Unparsable`], not [`Verdict::Collapse`].
//!
//! 1. **syntactic skeleton** — the tape is attested, the extent is not inverted, the
//!    capacity is positive, the reserve is not negative, and the budget was not
//!    exceeded (B.1, line 159);
//! 2. **axial coverage** — every axis carries at least one reading (line 160);
//! 3. **finiteness** — the weakest reading of every axis is present, because a
//!    non-finite margin is quarantined rather than compared (line 161);
//! 4. **Leontief fold** — `min(S_epi, S_vital, S_gov)` against the declared
//!    threshold (line 162). A collapse names its proposition, its site and its step,
//!    and names them truly: the site's own axis has to carry the fold. Equality
//!    attains it, so any site of a tied axis is coherent; a site on a healthy axis is
//!    an ill-formed sentence and answers `Unparsable`, exactly as in stage 5;
//! 5. **frontier band** — the band above the threshold names its site with its margin
//!    (line 163), and the margin it names is the fold: a declared site is coherent
//!    only when its own axis attains the fold. Equality attains it, so every site of a
//!    tied axis is coherent; a strict excess is not a boundary but an ill-formed
//!    declaration, and stage 5 answers it [`Verdict::Unparsable`].
//!
//! One check comes **before** all five and is not a stage: the declared site and the
//! declared proposition must share an axis (`01` §3.2). It is a structural invariant,
//! not a verdict, so a transcript that violates it returns
//! [`Error::SitePropositionMismatch`] instead of any verdict at all.
//!
//! # Non-commutation
//!
//! The predicate never reads the contingency record it carries. B.1 lines 165-167
//! state the property, and the test suite substitutes the record and observes that
//! the verdict does not move: the fold is over one step and over sealed margins, and
//! the contingency of that step cannot reach it.

use crate::{
    Axis, Bounds, Capacity, ContingencyTape, ControlTape, Error, Proposition, Score, Site,
    Thresholds, MAX_ABS, SCALE,
};

/// The two thresholds an assessment declares, `01` §10.
///
/// The core embeds no threshold: B.1 line 155 is literal that no threshold lives in
/// the grammar, and a published acceptance threshold is exactly the degeneration the
/// fourth reflexive brake prohibits.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Assessment {
    /// The acceptance threshold of stage 4.
    pub accept_threshold: i64,
    /// The width of the frontier band of stage 5.
    pub band_width: i64,
}

impl Assessment {
    /// A threshold pair, as declared by the caller.
    pub const fn new(accept_threshold: i64, band_width: i64) -> Assessment {
        Assessment {
            accept_threshold,
            band_width,
        }
    }

    /// The pair as a [`Thresholds`] value, for comparison with the step's declaration.
    pub const fn thresholds(self) -> Thresholds {
        Thresholds::new(self.accept_threshold, self.band_width)
    }
}

/// One axis as the transcript declares it: how many readings, and the weakest.
///
/// The weakest reading is what the fold takes, so a transcript that declares a count
/// without a reading, or a reading without a count, is not expressible as a stage-4
/// input: stage 2 and stage 3 read different fields of the same record.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct AxisReading {
    /// How many guard readings this axis carries.
    pub readings: usize,
    /// The weakest of them, or [`Score::Missing`] when no finite one exists.
    pub weakest: Score,
}

impl AxisReading {
    /// An axis with the given count and weakest reading.
    pub const fn new(readings: usize, weakest: Score) -> AxisReading {
        AxisReading { readings, weakest }
    }
}

/// The three axes of a transcript.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct AxialReadings {
    /// The epistemic axis.
    pub epi: AxisReading,
    /// The vital axis.
    pub vital: AxisReading,
    /// The governance axis.
    pub gov: AxisReading,
}

impl AxialReadings {
    /// The three axes, in the fixed order of the fold.
    pub const fn new(epi: AxisReading, vital: AxisReading, gov: AxisReading) -> AxialReadings {
        AxialReadings { epi, vital, gov }
    }
}

/// The derivation budget of a parse, B.1 line 159 and `01` §6.2.
///
/// `elapsed > limit` is the exceeded budget of stage 1: the derivation that would be
/// judged never terminated inside its allowance, so the transcript is a skeleton
/// fault rather than a comparison.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Budget {
    /// The allowance.
    pub limit: i64,
    /// What the derivation consumed.
    pub elapsed: i64,
}

impl Budget {
    /// A budget from its allowance and its consumption.
    pub const fn new(limit: i64, elapsed: i64) -> Budget {
        Budget { limit, elapsed }
    }
}

/// What a transcript declares: which proposition, which site, which step.
///
/// All three are optional because a verdict does not always name them: an interior
/// verdict names nothing, a boundary names its site, a collapse names all three. The
/// predicate requires each one exactly where B.1 requires it, and quarantines the
/// transcript where it is absent, rather than inventing a name.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Declared {
    /// The declared collapse proposition, if any.
    pub proposition: Option<Proposition>,
    /// The declared guard site, if any.
    pub site: Option<Site>,
    /// The declared step index, if any.
    pub step: Option<u32>,
}

impl Declared {
    /// A declaration, possibly partial.
    pub const fn new(
        proposition: Option<Proposition>,
        site: Option<Site>,
        step: Option<u32>,
    ) -> Declared {
        Declared {
            proposition,
            site,
            step,
        }
    }

    /// A declaration that names nothing.
    pub const fn none() -> Declared {
        Declared {
            proposition: None,
            site: None,
            step: None,
        }
    }
}

/// An unsealed transcript: the parsed record a caller presents for judgement.
///
/// This is the input record, not the judgement. [`Transcript::seal`] turns it into
/// the opaque object the predicate reads, and does so with the two checks that are
/// *structural* rather than verdict-bearing: the sentinel extent and the wire bound.
/// Every other skeleton fault is judged by [`accept`] as
/// [`UnparsableReason::Skeleton`], because B.1 line 159 makes it a verdict.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Sentence {
    /// The attestation of the control tape.
    pub tape: ControlTape,
    /// The contingency record; never read by the predicate.
    pub contingency: ContingencyTape,
    /// The extent the assessment is over.
    pub bounds: Bounds,
    /// The capacity and the reserve.
    pub capacity: Capacity,
    /// The derivation budget.
    pub budget: Budget,
    /// The three axes as declared.
    pub axes: AxialReadings,
    /// The declaration of proposition, site and step.
    pub declared: Declared,
    /// The two thresholds.
    pub assessment: Assessment,
}

impl Sentence {
    /// A sentence from its eight records.
    pub const fn new(
        tape: ControlTape,
        contingency: ContingencyTape,
        bounds: Bounds,
        capacity: Capacity,
        budget: Budget,
        axes: AxialReadings,
        declared: Declared,
        assessment: Assessment,
    ) -> Sentence {
        Sentence {
            tape,
            contingency,
            bounds,
            capacity,
            budget,
            axes,
            declared,
            assessment,
        }
    }
}

/// A sealed transcript: the object the predicate judges.
///
/// Fields are private and [`Transcript::seal`] is the only door, as with [`State`].
/// The record is *not* accepted by [`crate::step`] and is not convertible into one:
/// the two objects answer different questions and share nothing but their arithmetic.
///
/// [`State`]: crate::State
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Transcript {
    tape: ControlTape,
    contingency: ContingencyTape,
    bounds: Bounds,
    capacity: Capacity,
    budget: Budget,
    axes: AxialReadings,
    declared: Declared,
    assessment: Assessment,
}

impl Transcript {
    /// Seal a sentence, or refuse it with the two structural errors.
    ///
    /// Refusals: [`Error::ExtentMissing`] when the extent carries the sentinel
    /// [`crate::EXTENT_NOT_PROVIDED`], [`Error::ScaleOverflow`] when any quantity is
    /// outside [`crate::MAX_ABS`], and [`Error::ScaleOverflow`] again when an axis's
    /// weakest reading is outside `0 ..= SCALE`. Neither is a verdict: they are the
    /// record not being a record. A skeleton fault that *is* a verdict — an unattested
    /// tape, an inverted extent, a non-positive capacity, a negative reserve, an
    /// exceeded budget — is kept in the sealed transcript and judged in stage 1.
    ///
    /// The second range is what [`Score::Present`] promises ([`crate::Score`]): a
    /// margin is a quantity on the declared scale, so the door refuses `9e12` instead
    /// of folding it into a verdict that compares true against every threshold.
    pub fn seal(sentence: Sentence) -> Result<Transcript, Error> {
        if sentence.bounds.l == crate::EXTENT_NOT_PROVIDED {
            return Err(Error::ExtentMissing);
        }
        for q in [
            sentence.bounds.l,
            sentence.bounds.u,
            sentence.bounds.n_eff,
            sentence.bounds.defect,
            sentence.capacity.c,
            sentence.capacity.b,
            sentence.capacity.r,
            sentence.budget.limit,
            sentence.budget.elapsed,
            sentence.assessment.accept_threshold,
            sentence.assessment.band_width,
        ] {
            if q > MAX_ABS || q < -MAX_ABS {
                return Err(Error::ScaleOverflow);
            }
        }
        // The margins are a different kind of quantity from the skeleton: they are
        // readings on the fixed-point scale, not extents, and `Score::Present` says so.
        for axis in [sentence.axes.epi, sentence.axes.vital, sentence.axes.gov] {
            if let Score::Present(weakest) = axis.weakest {
                if !(0..=SCALE).contains(&weakest) {
                    return Err(Error::ScaleOverflow);
                }
            }
        }
        Ok(Transcript {
            tape: sentence.tape,
            contingency: sentence.contingency,
            bounds: sentence.bounds,
            capacity: sentence.capacity,
            budget: sentence.budget,
            axes: sentence.axes,
            declared: sentence.declared,
            assessment: sentence.assessment,
        })
    }

    /// The contingency record this transcript carries.
    ///
    /// Available for the non-commutation test and for the wire layer; the predicate
    /// does not call it.
    pub const fn contingency(&self) -> ContingencyTape {
        self.contingency
    }

    /// The assessment this transcript declares.
    pub const fn assessment(&self) -> Assessment {
        self.assessment
    }
}

/// Why a transcript is [`Verdict::Unparsable`], B.1 lines 159-161.
///
/// The reason is named, as B.1 requires: an ill-formed input has a verdict, and that
/// verdict is the quarantine with its reason, not the absence of a verdict.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum UnparsableReason {
    /// `1`: the syntactic skeleton does not parse, or the budget was exceeded.
    Skeleton,
    /// `2`: an axis carries no reading at all.
    AxialCoverage,
    /// `3`: an axis's weakest reading is not finite.
    MarginMissing,
    /// `4`: the derivation exceeded its budget.
    BudgetExceeded,
}

impl UnparsableReason {
    /// The wire code of this reason, `1 .. 4`.
    pub const fn code(self) -> u8 {
        match self {
            UnparsableReason::Skeleton => 1,
            UnparsableReason::AxialCoverage => 2,
            UnparsableReason::MarginMissing => 3,
            UnparsableReason::BudgetExceeded => 4,
        }
    }
}

/// The four verdicts, in the declaration order of Appendix B.1 line 134.
///
/// The discriminant is wire convention, not precedence: the precedence is the flow of
/// control of [`accept`] (`01` §1.3). No code in this crate orders verdicts by tag.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Verdict {
    /// `0`: the fold is clear of the band.
    Interior,
    /// `1`: the fold is inside the band, which names its site with its margin.
    Boundary {
        /// The site the band names.
        site: Site,
        /// The margin at that site.
        margin: i64,
    },
    /// `2`: the fold is below the threshold, and the collapse names its axis.
    Collapse {
        /// The collapse proposition.
        proposition: Proposition,
        /// The guard site that manifests it.
        site: Site,
        /// The step index.
        step: u32,
    },
    /// `3`: the transcript is quarantined, with the reason named.
    Unparsable {
        /// Why the transcript could not be judged.
        reason: UnparsableReason,
    },
}

impl Verdict {
    /// The wire tag of this verdict, `0 .. 3`.
    pub const fn tag(self) -> u8 {
        match self {
            Verdict::Interior => 0,
            Verdict::Boundary { .. } => 1,
            Verdict::Collapse { .. } => 2,
            Verdict::Unparsable { .. } => 3,
        }
    }

    /// The site this verdict names, if any.
    pub const fn site(self) -> Option<Site> {
        match self {
            Verdict::Boundary { site, .. } | Verdict::Collapse { site, .. } => Some(site),
            _ => None,
        }
    }

    /// The quarantine reason, if this verdict is a quarantine.
    pub const fn reason(self) -> Option<UnparsableReason> {
        match self {
            Verdict::Unparsable { reason } => Some(reason),
            _ => None,
        }
    }
}

/// The fold of one step: the minimum of three readings, poisoned by absence.
///
/// `min` over `Missing` would need an order between "absent" and "small", and B.1 line
/// 161 chooses the other way: an absent margin is quarantined, never compared. So the
/// fold is absent as soon as any contributor is absent.
fn fold3(a: Score, b: Score, c: Score) -> Score {
    let mut best: Option<i64> = None;
    for score in [a, b, c] {
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

/// The weakest reading of the axis a declared site sits on.
///
/// A verdict that names a site is naming a place in the record, and the reading it
/// answers for is the reading of that site's own axis. The fold and the frontier both
/// need this query, so it lives here once: duplicating it is how the stage 4 and
/// stage 5 rules drifted apart in the first place.
fn weakest_on(axes: &[AxisReading; 3], site: Site) -> Score {
    match site.axis() {
        Axis::Epistemic => axes[0].weakest,
        Axis::Vital => axes[1].weakest,
        Axis::Governance => axes[2].weakest,
    }
}

/// Whether the declared site's own axis attains the fold, B.1 line 163.
///
/// Equality attains it. When two or three axes tie at the minimum, a site on any tied
/// axis is coherent, because its axis reads exactly the fold; a strict excess is not,
/// because naming it would print a site beside a margin its axis does not carry. Both
/// the collapse and the boundary answer to this rule: a collapse that named a healthy
/// axis would be the same unsupported claim as a boundary whose margin came from
/// somewhere else.
fn site_attains_the_fold(axes: &[AxisReading; 3], site: Site, fold: i64) -> bool {
    let reading = weakest_on(axes, site);
    reading.is_present() && reading.value() == Some(fold)
}

/// The acceptance predicate: five stages in the fixed order of Appendix B.1.
///
/// ```
/// use stoch_core::{accept, Assessment, AxialReadings, AxisReading, Bounds, Budget,
///                  Capacity, ContingencyTape, ControlTape, Declared, Score, Sentence,
///                  Site, Transcript, Verdict};
///
/// let axes = AxialReadings::new(
///     AxisReading::new(2, Score::Present(500_000)),
///     AxisReading::new(2, Score::Present(600_000)),
///     AxisReading::new(1, Score::Present(700_000)),
/// );
/// let sentence = Sentence::new(
///     ControlTape::Attested,
///     ContingencyTape::new(1),
///     Bounds::new(120, 400, 1000, 0),
///     Capacity::new(200, 0, 1000),
///     Budget::new(1000, 10),
///     axes,
///     Declared::none(),
///     Assessment::new(100_000, 50_000),
/// );
/// let transcript = Transcript::seal(sentence).unwrap();
/// assert_eq!(accept(&transcript), Ok(Verdict::Interior));
///
/// // The band names its site: just above the threshold is a boundary, not an interior.
/// let banded = AxialReadings::new(
///     AxisReading::new(1, Score::Present(101_000)),
///     AxisReading::new(1, Score::Present(101_000)),
///     AxisReading::new(1, Score::Present(101_000)),
/// );
/// let sentence = Sentence::new(
///     ControlTape::Attested,
///     ContingencyTape::new(1),
///     Bounds::new(120, 400, 1000, 0),
///     Capacity::new(200, 0, 1000),
///     Budget::new(1000, 10),
///     banded,
///     Declared::new(None, Some(Site::T4), None),
///     Assessment::new(100_000, 50_000),
/// );
/// let transcript = Transcript::seal(sentence).unwrap();
/// assert_eq!(
///     accept(&transcript),
///     Ok(Verdict::Boundary { site: Site::T4, margin: 101_000 })
/// );

/// ```
pub fn accept(transcript: &Transcript) -> Result<Verdict, Error> {
    // Structural invariant, before any stage: site and proposition share an axis
    // (`01` §3.2). A collapse whose site belongs to one plane and whose proposition
    // names another is incoherent, and the core refuses it before emitting a verdict.
    if let (Some(proposition), Some(site)) = (transcript.declared.proposition, transcript.declared.site)
    {
        if proposition.axis() != site.axis() {
            return Err(Error::SitePropositionMismatch);
        }
    }

    // Stage 1 — syntactic skeleton, B.1 line 159. Order inside the stage follows the
    // record: tape, extent, capacity, reserve, budget.
    if transcript.tape != ControlTape::Attested {
        return Ok(Verdict::Unparsable {
            reason: UnparsableReason::Skeleton,
        });
    }
    if transcript.bounds.l > transcript.bounds.u {
        return Ok(Verdict::Unparsable {
            reason: UnparsableReason::Skeleton,
        });
    }
    if transcript.capacity.c <= 0 {
        return Ok(Verdict::Unparsable {
            reason: UnparsableReason::Skeleton,
        });
    }
    if transcript.capacity.r < 0 {
        return Ok(Verdict::Unparsable {
            reason: UnparsableReason::Skeleton,
        });
    }
    if transcript.budget.elapsed > transcript.budget.limit {
        return Ok(Verdict::Unparsable {
            reason: UnparsableReason::BudgetExceeded,
        });
    }

    // Stage 2 — axial coverage, line 160. An axis with no reading has nothing to fold.
    let axes = [
        transcript.axes.epi,
        transcript.axes.vital,
        transcript.axes.gov,
    ];
    for axis in axes {
        if axis.readings == 0 {
            return Ok(Verdict::Unparsable {
                reason: UnparsableReason::AxialCoverage,
            });
        }
    }

    // Stage 3 — finiteness, line 161. A non-finite margin is quarantined rather than
    // compared, because the alternative is a number that compares silently false
    // against a threshold and passes every guard that touches it.
    for axis in axes {
        if !axis.weakest.is_present() {
            return Ok(Verdict::Unparsable {
                reason: UnparsableReason::MarginMissing,
            });
        }
    }

    // Stage 4 — the Leontief fold, line 162.
    let fold = match fold3(axes[0].weakest, axes[1].weakest, axes[2].weakest) {
        Score::Present(v) => v,
        // Unreachable after stage 3; the match is total and the quarantine is the
        // safe reading rather than an `unwrap`.
        Score::Missing => {
            return Ok(Verdict::Unparsable {
                reason: UnparsableReason::MarginMissing,
            })
        }
    };

    if fold < transcript.assessment.accept_threshold {
        // The collapse names its proposition, its site and its step. A transcript that
        // fails the fold without naming them cannot be reported as a collapse: B.1
        // requires the names, and inventing one would be the fabrication the mandatory
        // quarantine exists to prevent.
        match (
            transcript.declared.proposition,
            transcript.declared.site,
            transcript.declared.step,
        ) {
            // Three names are necessary and not sufficient. B.1 line 163 asks the same
            // of a collapse that it asks of a boundary: the site must be the place the
            // fold is, and a site on a healthy axis would report a failure that the
            // record attributes to a plane which is reading above the fold. Naming a
            // healthy axis as the failed one is a claim the record does not support, so
            // the sentence is ill-formed (B.1 line 159) rather than a collapse.
            (Some(proposition), Some(site), Some(step))
                if site_attains_the_fold(&axes, site, fold) =>
            {
                Ok(Verdict::Collapse {
                    proposition,
                    site,
                    step,
                })
            }
            _ => Ok(Verdict::Unparsable {
                reason: UnparsableReason::Skeleton,
            }),
        }
    } else {
        // Stage 5 — the frontier band, line 163: `[threshold, threshold + width)`.
        //
        // Both ends are bounded by `MAX_ABS` at the door, so their sum is bounded by
        // `2 * MAX_ABS` and cannot leave an `i64`. The band is compared in `i128` so
        // that the arithmetic states that fact instead of carrying an error arm that
        // no input can reach.
        let upper = i128::from(transcript.assessment.accept_threshold)
            + i128::from(transcript.assessment.band_width);
        if i128::from(fold) < upper {
            match transcript.declared.site {
                Some(site) => {
                    // Line 163: the verdict names *that* site with *its* margin — the
                    // site whose axis carries the fold. The rule is the one the collapse
                    // above answers to as well, so both stages ask it in one place.
                    if site_attains_the_fold(&axes, site, fold) {
                        Ok(Verdict::Boundary { site, margin: fold })
                    } else {
                        // B.1 line 159: a control tape violating a declared invariant is
                        // unparsable with a named reason. The declaration says this site
                        // carries the fold; the readings say it does not. An ill-formed
                        // sentence has no verdict, it does not have a bad one.
                        Ok(Verdict::Unparsable {
                            reason: UnparsableReason::Skeleton,
                        })
                    }
                }
                _ => Ok(Verdict::Unparsable {
                    reason: UnparsableReason::Skeleton,
                }),
            }
        } else {
            Ok(Verdict::Interior)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Capacity, EXTENT_NOT_PROVIDED};

    fn sentence_with(axes: AxialReadings, declared: Declared) -> Sentence {
        Sentence::new(
            ControlTape::Attested,
            ContingencyTape::new(7),
            Bounds::new(120, 400, 1000, 0),
            Capacity::new(200, 0, 1000),
            Budget::new(1000, 10),
            axes,
            declared,
            Assessment::new(100_000, 50_000),
        )
    }

    fn three(m: i64) -> AxialReadings {
        AxialReadings::new(
            AxisReading::new(1, Score::Present(m)),
            AxisReading::new(1, Score::Present(m)),
            AxisReading::new(1, Score::Present(m)),
        )
    }

    #[test]
    fn verdict_tags_follow_the_declaration_order() {
        assert_eq!(Verdict::Interior.tag(), 0);
        assert_eq!(
            Verdict::Boundary {
                site: Site::T1,
                margin: 1
            }
            .tag(),
            1
        );
        assert_eq!(
            Verdict::Collapse {
                proposition: Proposition::P1Epistemic,
                site: Site::T1,
                step: 0
            }
            .tag(),
            2
        );
        assert_eq!(
            Verdict::Unparsable {
                reason: UnparsableReason::Skeleton
            }
            .tag(),
            3
        );
    }

    #[test]
    fn unparsable_reasons_are_coded_one_to_four() {
        assert_eq!(UnparsableReason::Skeleton.code(), 1);
        assert_eq!(UnparsableReason::AxialCoverage.code(), 2);
        assert_eq!(UnparsableReason::MarginMissing.code(), 3);
        assert_eq!(UnparsableReason::BudgetExceeded.code(), 4);
    }

    #[test]
    fn stage_one_preempts_every_later_stage() {
        // Inverted bounds and a margin below the threshold: skeleton, not collapse.
        let mut sentence = sentence_with(
            three(1),
            Declared::new(Some(Proposition::P1Epistemic), Some(Site::T1), Some(3)),
        );
        sentence.bounds = Bounds::new(400, 200, 1000, 0);
        let transcript = Transcript::seal(sentence).unwrap();
        assert_eq!(
            accept(&transcript),
            Ok(Verdict::Unparsable {
                reason: UnparsableReason::Skeleton
            })
        );
    }

    #[test]
    fn stage_two_preempts_finiteness_and_the_fold() {
        let axes = AxialReadings::new(
            AxisReading::new(0, Score::Missing),
            AxisReading::new(1, Score::Present(1)),
            AxisReading::new(1, Score::Present(1)),
        );
        let transcript =
            Transcript::seal(sentence_with(axes, Declared::none())).unwrap();
        assert_eq!(
            accept(&transcript),
            Ok(Verdict::Unparsable {
                reason: UnparsableReason::AxialCoverage
            })
        );
    }

    #[test]
    fn stage_three_preempts_the_fold() {
        let axes = AxialReadings::new(
            AxisReading::new(1, Score::Present(1)),
            AxisReading::new(1, Score::Missing),
            AxisReading::new(1, Score::Present(1)),
        );
        let transcript =
            Transcript::seal(sentence_with(axes, Declared::none())).unwrap();
        assert_eq!(
            accept(&transcript),
            Ok(Verdict::Unparsable {
                reason: UnparsableReason::MarginMissing
            })
        );
    }

    #[test]
    fn the_fold_is_a_minimum_and_absence_poisons_it() {
        assert_eq!(
            fold3(
                Score::Present(7),
                Score::Present(3),
                Score::Present(5)
            ),
            Score::Present(3)
        );
        assert_eq!(
            fold3(
                Score::Present(7),
                Score::Missing,
                Score::Present(5)
            ),
            Score::Missing
        );
    }

    #[test]
    fn the_band_is_half_open() {
        let boundary = Transcript::seal(sentence_with(
            three(100_000),
            Declared::new(None, Some(Site::T2), None),
        ))
        .unwrap();
        assert_eq!(
            accept(&boundary),
            Ok(Verdict::Boundary {
                site: Site::T2,
                margin: 100_000
            })
        );
        let interior = Transcript::seal(sentence_with(three(150_000), Declared::none())).unwrap();
        assert_eq!(accept(&interior), Ok(Verdict::Interior));
    }

    #[test]
    fn a_collapse_without_names_is_quarantined() {
        let transcript = Transcript::seal(sentence_with(three(1), Declared::none())).unwrap();
        assert_eq!(
            accept(&transcript),
            Ok(Verdict::Unparsable {
                reason: UnparsableReason::Skeleton
            })
        );
    }

    #[test]
    fn a_boundary_without_a_site_is_quarantined() {
        let transcript =
            Transcript::seal(sentence_with(three(120_000), Declared::none())).unwrap();
        assert_eq!(
            accept(&transcript),
            Ok(Verdict::Unparsable {
                reason: UnparsableReason::Skeleton
            })
        );
    }

    #[test]
    fn a_mismatched_axis_is_an_error_before_any_verdict() {
        let transcript = Transcript::seal(sentence_with(
            three(1),
            Declared::new(Some(Proposition::P1Epistemic), Some(Site::T4), Some(0)),
        ))
        .unwrap();
        assert_eq!(accept(&transcript), Err(Error::SitePropositionMismatch));
    }

    #[test]
    fn the_contingency_record_changes_nothing() {
        let base = sentence_with(
            three(101_000),
            Declared::new(None, Some(Site::T6), None),
        );
        let mut other = base;
        other.contingency = ContingencyTape::new(0);
        let a = Transcript::seal(base).unwrap();
        let b = Transcript::seal(other).unwrap();
        assert_eq!(accept(&a), accept(&b));
    }

    #[test]
    fn the_sentinel_extent_is_refused_at_the_door() {
        let mut sentence = sentence_with(three(500_000), Declared::none());
        sentence.bounds = Bounds::new(EXTENT_NOT_PROVIDED, 400, 1000, 0);
        assert_eq!(Transcript::seal(sentence).err(), Some(Error::ExtentMissing));
    }

    #[test]
    fn the_wire_bound_is_refused_at_the_door() {
        let mut sentence = sentence_with(three(500_000), Declared::none());
        sentence.capacity = Capacity::new(1_000_000_001, 0, 0);
        assert_eq!(Transcript::seal(sentence).err(), Some(Error::ScaleOverflow));
    }

    #[test]
    fn the_band_upper_edge_needs_no_error_arm() {
        // Both ends of the band are bounded at the door by MAX_ABS, so their sum is
        // bounded by twice the wire bound and cannot leave an i64. Stage 5 therefore
        // computes it in i128 and carries no error arm that no input can reach. A
        // margin at the top of the declared scale is far above any band: interior.
        let transcript =
            Transcript::seal(sentence_with(three(SCALE), Declared::none())).unwrap();
        assert_eq!(accept(&transcript), Ok(Verdict::Interior));

        // The band's ends are still bounded where they enter: a width past the wire
        // bound is refused at the door, not inside the predicate.
        let mut sentence = sentence_with(three(500_000), Declared::none());
        sentence.assessment = Assessment::new(MAX_ABS, MAX_ABS + 1);
        assert_eq!(Transcript::seal(sentence).err(), Some(Error::ScaleOverflow));
    }

    #[test]
    fn a_margin_off_the_scale_is_refused_at_the_door() {
        // F2: the range Score::Present documents is the range this door imposes.
        let mut sentence = sentence_with(three(500_000), Declared::none());
        sentence.axes.epi.weakest = Score::Present(9_000_000_000_000);
        assert_eq!(Transcript::seal(sentence).err(), Some(Error::ScaleOverflow));

        // One unit past the scale in either direction is refused; the scale itself and
        // zero are legal margins and still seal.
        for off in [SCALE + 1, -1] {
            let mut sentence = sentence_with(three(500_000), Declared::none());
            sentence.axes.vital.weakest = Score::Present(off);
            assert_eq!(
                Transcript::seal(sentence).err(),
                Some(Error::ScaleOverflow),
                "margin {} must be refused",
                off
            );
        }
        for edge in [0, SCALE] {
            let mut sentence = sentence_with(three(500_000), Declared::none());
            sentence.axes.gov.weakest = Score::Present(edge);
            assert!(
                Transcript::seal(sentence).is_ok(),
                "margin {} is on the scale and must seal",
                edge
            );
        }
        // An absent margin is not an out-of-range one: it is a value the door admits
        // and stage 3 quarantines.
        let mut sentence = sentence_with(three(500_000), Declared::none());
        sentence.axes.gov.weakest = Score::Missing;
        assert!(Transcript::seal(sentence).is_ok());
    }

    #[test]
    fn a_boundary_names_the_axis_that_carries_the_fold() {
        // Margins by axis, unequal: the fold is epistemic, and the declared site is on
        // the epistemic axis. That pair is coherent and the verdict names it.
        let axes = AxialReadings::new(
            AxisReading::new(1, Score::Present(101_000)),
            AxisReading::new(1, Score::Present(900_000)),
            AxisReading::new(1, Score::Present(900_000)),
        );
        let declared = Declared::new(Some(Proposition::P1Epistemic), Some(Site::T1), Some(0));
        let transcript = Transcript::seal(sentence_with(axes, declared)).unwrap();
        assert_eq!(
            accept(&transcript),
            Ok(Verdict::Boundary {
                site: Site::T1,
                margin: 101_000
            })
        );
    }

    #[test]
    fn an_incoherent_site_and_margin_pair_is_not_a_boundary() {
        // The reviewer's repro. The fold is epistemic (101_000); the declaration names
        // a vital site whose own axis reads 900_000. Naming T4 beside 101_000 would
        // print a pair the record does not support, so the sentence is ill-formed:
        // unparsable for a broken skeleton, never a boundary (B.1, 159 and 163).
        let axes = AxialReadings::new(
            AxisReading::new(1, Score::Present(101_000)),
            AxisReading::new(1, Score::Present(900_000)),
            AxisReading::new(1, Score::Present(900_000)),
        );
        let declared = Declared::new(Some(Proposition::P2Vital), Some(Site::T4), Some(0));
        let transcript = Transcript::seal(sentence_with(axes, declared)).unwrap();
        assert_eq!(
            accept(&transcript),
            Ok(Verdict::Unparsable {
                reason: UnparsableReason::Skeleton
            })
        );
        assert_ne!(accept(&transcript).map(|v| v.tag()), Ok(1));
    }

    #[test]
    fn a_tie_between_axes_licences_any_site_of_the_tied_axes() {
        // Equality attains the fold: an axis reading equal to the minimum is the
        // axis the fold came from, so every site of every tied axis is coherent.
        for site in [Site::T1, Site::T4, Site::T7] {
            let axes = AxialReadings::new(
                AxisReading::new(1, Score::Present(101_000)),
                AxisReading::new(1, Score::Present(101_000)),
                AxisReading::new(1, Score::Present(101_000)),
            );
            let proposition = match site.axis() {
                Axis::Epistemic => Proposition::P1Epistemic,
                Axis::Vital => Proposition::P2Vital,
                Axis::Governance => Proposition::P3Governance,
            };
            let declared = Declared::new(Some(proposition), Some(site), Some(0));
            let transcript = Transcript::seal(sentence_with(axes, declared)).unwrap();
            assert_eq!(
                accept(&transcript),
                Ok(Verdict::Boundary {
                    site,
                    margin: 101_000
                }),
                "{:?} attains the fold and must be nameable",
                site
            );
        }
    }

    fn readings(epi: i64, vital: i64, gov: i64) -> AxialReadings {
        AxialReadings::new(
            AxisReading::new(1, Score::Present(epi)),
            AxisReading::new(1, Score::Present(vital)),
            AxisReading::new(1, Score::Present(gov)),
        )
    }

    fn collapse_declared() -> Declared {
        Declared::new(Some(Proposition::P2Vital), Some(Site::T4), Some(7))
    }

    #[test]
    fn a_collapse_names_the_axis_that_carries_the_fold() {
        // The fold is epistemic (50000) and the declared site is epistemic: the pair is
        // coherent, the sentence names its proposition, its site and its step, and the
        // verdict is the collapse the fold demands.
        let declared = Declared::new(Some(Proposition::P1Epistemic), Some(Site::T1), Some(7));
        let transcript =
            Transcript::seal(sentence_with(readings(50_000, 900_000, 900_000), declared))
                .expect("seals");
        assert_eq!(
            accept(&transcript),
            Ok(Verdict::Collapse {
                proposition: Proposition::P1Epistemic,
                site: Site::T1,
                step: 7
            })
        );
    }

    #[test]
    fn a_collapse_over_a_healthy_axis_is_not_a_collapse() {
        // The asymmetry this test closes: stage 5 already refused to name a site whose
        // axis does not carry the fold, and stage 4 did not. Here the fold is epistemic
        // (50000) while the declared pair is vital on T4, whose axis reads 900000 — a
        // healthy plane. Naming it would report a failure of the vital axis that the
        // record attributes to a plane reading eighteen times the fold. B.1 line 159:
        // a sentence violating a declared invariant is unparsable; it has no verdict.
        let transcript =
            Transcript::seal(sentence_with(readings(50_000, 900_000, 900_000), collapse_declared()))
                .expect("seals");
        assert_eq!(
            accept(&transcript),
            Ok(Verdict::Unparsable {
                reason: UnparsableReason::Skeleton
            })
        );
        assert_ne!(accept(&transcript).map(|verdict| verdict.tag()), Ok(2));
    }

    #[test]
    fn a_collapse_on_a_tied_fold_licences_any_tied_site() {
        // Equality attains the fold, so when all three axes read the fold every site is
        // on an axis that carries it and every declared pair is coherent — which is
        // what the fixture's a03..a09 assert from the other direction.
        for (proposition, site) in [
            (Proposition::P1Epistemic, Site::T1),
            (Proposition::P2Vital, Site::T4),
            (Proposition::P3Governance, Site::T7),
        ] {
            let declared = Declared::new(Some(proposition), Some(site), Some(7));
            let transcript =
                Transcript::seal(sentence_with(readings(50_000, 50_000, 50_000), declared))
                    .expect("seals");
            assert_eq!(
                accept(&transcript),
                Ok(Verdict::Collapse {
                    proposition,
                    site,
                    step: 7
                }),
                "{:?} attains the fold and may be named",
                site
            );
        }

        // A two-way tie is the same rule with one axis above: both tied sites answer,
        // the third does not.
        for (proposition, site, expected) in [
            (Proposition::P1Epistemic, Site::T1, true),
            (Proposition::P2Vital, Site::T4, true),
            (Proposition::P3Governance, Site::T7, false),
        ] {
            let declared = Declared::new(Some(proposition), Some(site), Some(7));
            let transcript =
                Transcript::seal(sentence_with(readings(50_000, 50_000, 900_000), declared))
                    .expect("seals");
            let verdict = accept(&transcript).expect("a verdict");
            if expected {
                assert_eq!(
                    verdict,
                    Verdict::Collapse {
                        proposition,
                        site,
                        step: 7
                    },
                    "{:?} is tied at the fold",
                    site
                );
            } else {
                assert_eq!(
                    verdict,
                    Verdict::Unparsable {
                        reason: UnparsableReason::Skeleton
                    },
                    "{:?} reads above a two-way fold",
                    site
                );
            }
        }
    }
}
