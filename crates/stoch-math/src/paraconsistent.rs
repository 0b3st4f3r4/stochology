//! Pre-decisional Filter I paraconsistent conflict and gap containers (`#![no_std]`).
//!
//! Models Belnap--Dunn four-valued epistemic status ([`FdeState`]), Logics of Formal
//! Inconsistency local-consistency markers ([`LocalConsistency`]), pre-decisional archive
//! reconciliation ([`Filter1Claim`]), and two-coordinate annotated evidence
//! ([`AnnotatedEvidence`]). These structures live strictly ahead of sealing and never
//! alter or replace the live linear acceptance predicate.
//!
//! ```
//! use stoch_math::paraconsistent::{AnnotatedEvidence, FdeState, Filter1Claim, LocalConsistency};
//! use stoch_math::{CoddValue, UNIT};
//!
//! let official = Filter1Claim::new(CoddValue::Known(700_000), FdeState::True, LocalConsistency::Consistent);
//! let community = Filter1Claim::new(CoddValue::Known(450_000), FdeState::False, LocalConsistency::Consistent);
//! let merged = official.merge_claim(community);
//! assert_eq!(merged.fde, FdeState::Both);
//! assert_eq!(merged.consistency, LocalConsistency::Contested);
//! assert!(!merged.can_seal());
//!
//! let ev = AnnotatedEvidence::new(800_000, 600_000).unwrap();
//! assert_eq!(ev.certainty_degree(), 200_000);
//! assert_eq!(ev.contradiction_degree(), 400_000);
//! ```

use crate::{CoddValue, MathError, UNIT};

/// Belnap--Dunn four-valued epistemic state (`FDE`) separating information gaps from gluts.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FdeState {
    /// Neither asserted nor denied (information gap / paracomplete).
    Neither,
    /// Asserted without contradiction.
    True,
    /// Denied without contradiction.
    False,
    /// Simultaneously asserted and denied by distinct sources (information glut).
    Both,
}

impl FdeState {
    /// Information-order join (`<=_k`), forming an associative, commutative, idempotent semilattice
    /// with bottom [`FdeState::Neither`] and top [`FdeState::Both`].
    #[must_use]
    pub const fn info_join(self, other: Self) -> Self {
        match (self, other) {
            (Self::Neither, x) | (x, Self::Neither) => x,
            (Self::True, Self::True) => Self::True,
            (Self::False, Self::False) => Self::False,
            _ => Self::Both,
        }
    }

    /// Truth-order meet (`<=_t`): `False < {Neither, Both} < True` (`Neither /\ Both = False`).
    #[must_use]
    pub const fn truth_meet(self, other: Self) -> Self {
        match (self, other) {
            (Self::False, _) | (_, Self::False) => Self::False,
            (Self::True, x) | (x, Self::True) => x,
            (Self::Neither, Self::Neither) => Self::Neither,
            (Self::Both, Self::Both) => Self::Both,
            (Self::Neither, Self::Both) | (Self::Both, Self::Neither) => Self::False,
        }
    }

    /// Truth-order join (`<=_t`): `False < {Neither, Both} < True` (`Neither \/ Both = True`).
    #[must_use]
    pub const fn truth_join(self, other: Self) -> Self {
        match (self, other) {
            (Self::True, _) | (_, Self::True) => Self::True,
            (Self::False, x) | (x, Self::False) => x,
            (Self::Neither, Self::Neither) => Self::Neither,
            (Self::Both, Self::Both) => Self::Both,
            (Self::Neither, Self::Both) | (Self::Both, Self::Neither) => Self::True,
        }
    }

    /// De Morgan negation preserving fixed points [`FdeState::Neither`] and [`FdeState::Both`].
    #[must_use]
    pub const fn negate(self) -> Self {
        match self {
            Self::Neither => Self::Neither,
            Self::True => Self::False,
            Self::False => Self::True,
            Self::Both => Self::Both,
        }
    }

    /// Returns `true` iff this entry is an unpopulated gap ([`FdeState::Neither`]).
    #[must_use]
    pub const fn is_gap(self) -> bool {
        matches!(self, Self::Neither)
    }

    /// Returns `true` iff this entry holds conflicting testimony ([`FdeState::Both`]).
    #[must_use]
    pub const fn is_glut(self) -> bool {
        matches!(self, Self::Both)
    }
}

/// LFI primitive consistency marker (`circ A` vs. open contestation).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LocalConsistency {
    /// Claimed observation satisfies local consistency (`circ A`).
    Consistent,
    /// Claimed observation sits under active inter-source contestation.
    Contested,
}

/// Pre-decisional Filter I record pairing Codd missingness with Belnap--Dunn conflict tracking.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Filter1Claim {
    /// Four-state Codd relational observation status.
    pub value: CoddValue,
    /// Four-valued Belnap--Dunn epistemic state.
    pub fde: FdeState,
    /// Local consistency attestation (`circ A`).
    pub consistency: LocalConsistency,
}

impl Filter1Claim {
    /// Constructs a pre-decisional [`Filter1Claim`].
    #[must_use]
    pub const fn new(
        value: CoddValue,
        fde: FdeState,
        consistency: LocalConsistency,
    ) -> Self {
        Self {
            value,
            fde,
            consistency,
        }
    }

    /// Returns `true` strictly when the value is known, neither a gap nor a glut, and locally consistent.
    #[must_use]
    pub const fn can_seal(self) -> bool {
        self.value.is_known()
            && !self.fde.is_gap()
            && !self.fde.is_glut()
            && matches!(self.consistency, LocalConsistency::Consistent)
    }

    /// Merges two legitimate archive claims conservatively inside Filter I quarantine.
    ///
    /// Forms a commutative, associative, idempotent join semilattice: preserves
    /// canonical `NotAskable` exclusions as top annihilator without coercing to zero or `Both`,
    /// elevates any discrepant known readings or contradictory Codd statuses to
    /// [`CoddValue::ApplicableUnknown`], [`FdeState::Both`], and [`LocalConsistency::Contested`],
    /// and preserves identical uncontested observations.
    #[must_use]
    pub const fn merge_claim(self, other: Self) -> Self {
        if matches!(self.value, CoddValue::NotAskable)
            || matches!(other.value, CoddValue::NotAskable)
        {
            return Self {
                value: CoddValue::NotAskable,
                fde: FdeState::Neither,
                consistency: LocalConsistency::Consistent,
            };
        }

        let mut fde = self.fde.info_join(other.fde);
        let mut consistency = match (self.consistency, other.consistency) {
            (LocalConsistency::Consistent, LocalConsistency::Consistent) => {
                LocalConsistency::Consistent
            }
            _ => LocalConsistency::Contested,
        };

        let value = match (self.value, other.value) {
            (CoddValue::Inapplicable, CoddValue::Inapplicable) => CoddValue::Inapplicable,
            (CoddValue::Known(a), CoddValue::Known(b)) if a == b => CoddValue::Known(a),
            (CoddValue::ApplicableUnknown, CoddValue::ApplicableUnknown) => {
                CoddValue::ApplicableUnknown
            }
            _ => {
                fde = FdeState::Both;
                consistency = LocalConsistency::Contested;
                CoddValue::ApplicableUnknown
            }
        };

        if fde.is_glut() {
            consistency = LocalConsistency::Contested;
        }

        Self {
            value,
            fde,
            consistency,
        }
    }
}

/// Paraconsistent annotated evidence (`LPA2v`, Chs. `33` & `45`) in fixed-point millionths.
///
/// Tracks favorable evidence `mu in [0, UNIT]` separately from unfavorable evidence
/// `lambda in [0, UNIT]` so inter-source contradiction is never collapsed into a single average.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct AnnotatedEvidence {
    mu: i64,
    lambda: i64,
}

impl AnnotatedEvidence {
    /// Validates favorable (`mu`) and unfavorable (`lambda`) annotations inside `[0, UNIT]`.
    pub const fn new(mu: i64, lambda: i64) -> Result<Self, MathError> {
        if mu < 0 || mu > UNIT || lambda < 0 || lambda > UNIT {
            return Err(MathError::InvalidDomain(
                "LPA2v evidence coordinates must lie in [0, UNIT]",
            ));
        }
        Ok(Self { mu, lambda })
    }

    /// Favorable evidence degree `mu in [0, UNIT]`.
    #[must_use]
    pub const fn mu(self) -> i64 {
        self.mu
    }

    /// Unfavorable evidence degree `lambda in [0, UNIT]`.
    #[must_use]
    pub const fn lambda(self) -> i64 {
        self.lambda
    }

    /// Certainty coordinate `G_c = mu - lambda` in `[-UNIT, UNIT]`.
    #[must_use]
    pub const fn certainty_degree(self) -> i64 {
        self.mu - self.lambda
    }

    /// Contradiction coordinate `G_ct = mu + lambda - UNIT` in `[-UNIT, UNIT]`.
    #[must_use]
    pub const fn contradiction_degree(self) -> i64 {
        self.mu + self.lambda - UNIT
    }

    /// Returns `true` iff contradiction exceeds zero (`mu + lambda > UNIT`).
    #[must_use]
    pub const fn is_overdetermined(self) -> bool {
        self.contradiction_degree() > 0
    }

    /// Returns `true` iff total evidence stays below saturation (`mu + lambda < UNIT`).
    #[must_use]
    pub const fn is_underdetermined(self) -> bool {
        self.contradiction_degree() < 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fde_info_join_forms_lattice_and_preserves_not_askable() {
        assert_eq!(FdeState::True.info_join(FdeState::False), FdeState::Both);
        assert_eq!(FdeState::Neither.info_join(FdeState::True), FdeState::True);
        assert_eq!(FdeState::Both.negate(), FdeState::Both);
        assert_eq!(FdeState::Neither.negate(), FdeState::Neither);
        assert_eq!(FdeState::True.truth_meet(FdeState::False), FdeState::False);
        assert_eq!(FdeState::Neither.truth_join(FdeState::Both), FdeState::True);

        let excluded = Filter1Claim::new(
            CoddValue::NotAskable,
            FdeState::Neither,
            LocalConsistency::Consistent,
        );
        let measured = Filter1Claim::new(
            CoddValue::Known(500_000),
            FdeState::True,
            LocalConsistency::Consistent,
        );
        let merged_excluded = excluded.merge_claim(measured);
        assert_eq!(merged_excluded.value, CoddValue::NotAskable);
        assert!(!merged_excluded.can_seal());
    }

    #[test]
    fn filter1_claim_blocks_gaps_gluts_and_discrepant_knowns() {
        let a = Filter1Claim::new(
            CoddValue::Known(700_000),
            FdeState::True,
            LocalConsistency::Consistent,
        );
        let a_same = Filter1Claim::new(
            CoddValue::Known(700_000),
            FdeState::True,
            LocalConsistency::Consistent,
        );
        assert!(a.merge_claim(a_same).can_seal());

        let b_diff = Filter1Claim::new(
            CoddValue::Known(500_000),
            FdeState::True,
            LocalConsistency::Consistent,
        );
        let conflict = a.merge_claim(b_diff);
        assert_eq!(conflict.fde, FdeState::Both);
        assert_eq!(conflict.consistency, LocalConsistency::Contested);
        assert!(!conflict.can_seal());

        let gap = Filter1Claim::new(
            CoddValue::ApplicableUnknown,
            FdeState::Neither,
            LocalConsistency::Consistent,
        );
        assert!(!gap.can_seal());

        let sample = [
            Filter1Claim::new(
                CoddValue::NotAskable,
                FdeState::Neither,
                LocalConsistency::Consistent,
            ),
            a,
            b_diff,
            gap,
            Filter1Claim::new(
                CoddValue::Inapplicable,
                FdeState::False,
                LocalConsistency::Contested,
            ),
        ];
        for &x in &sample {
            assert_eq!(x.merge_claim(x).value, x.value);
            for &y in &sample {
                assert_eq!(x.merge_claim(y), y.merge_claim(x));
                for &z in &sample {
                    assert_eq!(
                        x.merge_claim(y).merge_claim(z),
                        x.merge_claim(y.merge_claim(z))
                    );
                }
            }
        }
    }

    #[test]
    fn lpa2v_coordinates_stay_bounded_and_separate_certainty_from_contradiction() {
        let conflict = AnnotatedEvidence::new(900_000, 800_000).expect("valid");
        assert_eq!(conflict.certainty_degree(), 100_000);
        assert_eq!(conflict.contradiction_degree(), 700_000);
        assert!(conflict.is_overdetermined());
        assert!(!conflict.is_underdetermined());

        let gap = AnnotatedEvidence::new(200_000, 100_000).expect("valid");
        assert_eq!(gap.certainty_degree(), 100_000);
        assert_eq!(gap.contradiction_degree(), -700_000);
        assert!(gap.is_underdetermined());

        assert!(matches!(
            AnnotatedEvidence::new(UNIT + 1, 0),
            Err(MathError::InvalidDomain(_))
        ));
        assert!(matches!(
            AnnotatedEvidence::new(0, -1),
            Err(MathError::InvalidDomain(_))
        ));
    }
}
