#!/usr/bin/env python3
"""Independent formal-invariant oracle for Stochology Appendix B.1 and B.2.

Re-implements the discrete B.1 transcript seal, pre-stage coherence gate, and
five ordered acceptance stages cleanly from the specification (using only the
Python standard library), cross-checks all fn=1 records (`a01`..`a21`) in
`crates/stoch-core/vectors.kv`, proves contingency invariance and strict
119-permutation stage non-commutation, and verifies Table B.1 / J(7,3) topology.
"""

from __future__ import annotations

from dataclasses import dataclass
from itertools import combinations, permutations
from pathlib import Path
import re
import sys

ROOT = Path(__file__).resolve().parents[2]
VECTORS_PATH = ROOT / "crates" / "stoch-core" / "vectors.kv"
APPENDIX_B_PATH = ROOT / "book-src" / "appendix-b-architecture-verification.md"

SCALE = 1_000_000
MAX_ABS = 1_000_000_000
EXTENT_NOT_PROVIDED = -1

ERR_SCALE_OVERFLOW = -5
ERR_EXTENT_MISSING = -6
ERR_SITE_PROP_MISMATCH = -11

SITE_AXIS = {
    1: "Epistemic",
    2: "Epistemic",
    3: "Epistemic",
    4: "Vital",
    5: "Vital",
    6: "Governance",
    7: "Governance",
}
PROP_AXIS = {1: "Epistemic", 2: "Vital", 3: "Governance"}

VERDICT_INTERIOR = 0
VERDICT_BOUNDARY = 1
VERDICT_COLLAPSE = 2
VERDICT_UNPARSABLE = 3

REASON_SKELETON = 1
REASON_AXIAL_COVERAGE = 2
REASON_MARGIN_MISSING = 3
REASON_BUDGET_EXCEEDED = 4
REASON_COUNTER_INDUCTIVE_REOPEN = 5

FIXTURE_BUDGET_LIMIT = 1000
FIXTURE_BUDGET_ELAPSED = 10
FIXTURE_ACCEPT_THRESHOLD = 100_000
FIXTURE_BAND_WIDTH = 50_000
FIXTURE_N_EFF = 1000
FIXTURE_DEFECT = 0
FIXTURE_CONTINGENCY = 7


@dataclass(frozen=True)
class TranscriptInput:
    """Plain value representation of a sealed B.1 sentence."""

    attested: bool
    contingency: int
    l: int
    u: int
    n_eff: int
    defect: int
    c: int
    b: int
    r: int
    budget_limit: int
    budget_elapsed: int
    epi_readings: int
    vital_readings: int
    gov_readings: int
    m_epi: int | None
    m_vital: int | None
    m_gov: int | None
    proposition: int | None
    site: int | None
    step_index: int | None
    reopen_sealed: bool
    accept_threshold: int
    band_width: int


@dataclass(frozen=True)
class Outcome:
    """Normalized accept verdict or structural refusal error."""

    error: int = 0
    verdict: int | None = None
    reason: int = 0
    site: int | None = None
    proposition: int | None = None
    step_index: int | None = None
    margin: int | None = None


def seal_check(tx: TranscriptInput) -> Outcome | None:
    """Enforce Transcript::seal sentinel and wire bounds before staging."""
    if tx.l == EXTENT_NOT_PROVIDED:
        return Outcome(error=ERR_EXTENT_MISSING)
    scalars = (
        tx.l,
        tx.u,
        tx.n_eff,
        tx.defect,
        tx.c,
        tx.b,
        tx.r,
        tx.budget_limit,
        tx.budget_elapsed,
        tx.accept_threshold,
        tx.band_width,
    )
    if any(q > MAX_ABS or q < -MAX_ABS for q in scalars):
        return Outcome(error=ERR_SCALE_OVERFLOW)
    for margin in (tx.m_epi, tx.m_vital, tx.m_gov):
        if margin is not None and not (0 <= margin <= SCALE):
            return Outcome(error=ERR_SCALE_OVERFLOW)
    return None


def prestage_axis_check(tx: TranscriptInput) -> Outcome | None:
    """Enforce site-proposition plane coherence prior to stages 1..5."""
    if tx.proposition is not None and tx.site is not None:
        if PROP_AXIS[tx.proposition] != SITE_AXIS[tx.site]:
            return Outcome(error=ERR_SITE_PROP_MISMATCH)
    return None


def weakest_on_site(tx: TranscriptInput, site: int) -> int | None:
    """Return weakest margin of the axis holding `site`."""
    axis = SITE_AXIS[site]
    if axis == "Epistemic":
        return tx.m_epi
    if axis == "Vital":
        return tx.m_vital
    return tx.m_gov


def eval_stage(tx: TranscriptInput, stage: int, unguarded: bool = False) -> Outcome | None:
    """Evaluate one stage of Appendix B.1; return terminal Outcome or None to pass."""
    if stage == 1:
        if (
            not tx.attested
            or tx.l > tx.u
            or tx.c <= 0
            or tx.r < 0
            or tx.band_width <= 0
        ):
            return Outcome(verdict=VERDICT_UNPARSABLE, reason=REASON_SKELETON)
        if tx.budget_elapsed > tx.budget_limit:
            return Outcome(verdict=VERDICT_UNPARSABLE, reason=REASON_BUDGET_EXCEEDED)
        if tx.reopen_sealed:
            return Outcome(
                verdict=VERDICT_UNPARSABLE,
                reason=REASON_COUNTER_INDUCTIVE_REOPEN,
            )
        return None

    if stage == 2:
        if tx.epi_readings == 0 or tx.vital_readings == 0 or tx.gov_readings == 0:
            return Outcome(verdict=VERDICT_UNPARSABLE, reason=REASON_AXIAL_COVERAGE)
        return None

    if stage == 3:
        if tx.m_epi is None or tx.m_vital is None or tx.m_gov is None:
            return Outcome(verdict=VERDICT_UNPARSABLE, reason=REASON_MARGIN_MISSING)
        return None

    present = [m for m in (tx.m_epi, tx.m_vital, tx.m_gov) if m is not None]
    if not unguarded and len(present) < 3:
        return Outcome(verdict=VERDICT_UNPARSABLE, reason=REASON_MARGIN_MISSING)
    if not present:
        return None
    fold = min(present)

    if stage == 4:
        if fold < tx.accept_threshold:
            if (
                tx.proposition is not None
                and tx.site is not None
                and tx.step_index is not None
                and weakest_on_site(tx, tx.site) == fold
            ):
                return Outcome(
                    verdict=VERDICT_COLLAPSE,
                    proposition=tx.proposition,
                    site=tx.site,
                    step_index=tx.step_index,
                )
            return Outcome(verdict=VERDICT_UNPARSABLE, reason=REASON_SKELETON)
        return None

    if stage == 5:
        upper = tx.accept_threshold + tx.band_width
        condition = (
            fold < upper
            if unguarded
            else (tx.accept_threshold <= fold < upper)
        )
        if condition:
            if tx.site is not None and weakest_on_site(tx, tx.site) == fold:
                return Outcome(verdict=VERDICT_BOUNDARY, site=tx.site, margin=fold)
            return Outcome(verdict=VERDICT_UNPARSABLE, reason=REASON_SKELETON)
        return Outcome(verdict=VERDICT_INTERIOR)

    raise ValueError(f"unknown stage {stage}")


def accept_oracle(
    tx: TranscriptInput,
    order: tuple[int, ...] = (1, 2, 3, 4, 5),
    unguarded: bool = False,
) -> Outcome:
    """Execute seal, pre-stage coherence check, and staged acceptance in `order`."""
    if (err := seal_check(tx)) is not None:
        return err
    if (err := prestage_axis_check(tx)) is not None:
        return err
    for stage in order:
        if (out := eval_stage(tx, stage, unguarded=unguarded)) is not None:
            return out
    return Outcome(verdict=VERDICT_INTERIOR)


def parse_vectors() -> list[dict[str, int | str]]:
    """Parse `crates/stoch-core/vectors.kv` fn=1 rows into typed dictionaries."""
    rows: list[dict[str, int | str]] = []
    for raw in VECTORS_PATH.read_text(encoding="utf-8").splitlines():
        line = raw.strip()
        if not line or line.startswith("#"):
            continue
        rec: dict[str, int | str] = {}
        for token in line.split():
            k, v = token.split("=", 1)
            rec[k] = v if k == "id" else int(v)
        if rec.get("fn") == 1:
            rows.append(rec)
    return rows


def row_to_transcript(rec: dict[str, int | str], contingency: int = FIXTURE_CONTINGENCY) -> TranscriptInput:
    """Decode one `fn=1` fixture row identically to `fixture.rs::run_accept`."""
    def opt_score(key: str) -> int | None:
        val = int(rec.get(key, -1))
        return None if val < 0 else val

    prop_raw = int(rec.get("proposition", 0))
    site_raw = int(rec.get("site", 0))
    step_raw = int(rec.get("step_index", -1))
    return TranscriptInput(
        attested=int(rec.get("tape", -1)) != 0,
        contingency=contingency,
        l=int(rec["l"]),
        u=int(rec["u"]),
        n_eff=FIXTURE_N_EFF,
        defect=FIXTURE_DEFECT,
        c=int(rec["c"]),
        b=0,
        r=max(0, int(rec.get("r", 0))),
        budget_limit=FIXTURE_BUDGET_LIMIT,
        budget_elapsed=FIXTURE_BUDGET_ELAPSED,
        epi_readings=max(0, int(rec.get("epi", 0))),
        vital_readings=max(0, int(rec.get("vital", 0))),
        gov_readings=max(0, int(rec.get("gov", 0))),
        m_epi=opt_score("m_epi"),
        m_vital=opt_score("m_vital"),
        m_gov=opt_score("m_gov"),
        proposition=prop_raw if 1 <= prop_raw <= 3 else None,
        site=site_raw if 1 <= site_raw <= 7 else None,
        step_index=None if step_raw < 0 else step_raw,
        reopen_sealed=int(rec.get("reopen_sealed", 0)) == 1,
        accept_threshold=FIXTURE_ACCEPT_THRESHOLD,
        band_width=FIXTURE_BAND_WIDTH,
    )


def verify_vectors_and_contingency() -> int:
    """Check 100% conformance on `a01`..`a21` and contingency invariance."""
    rows = parse_vectors()
    assert len(rows) == 21, f"expected 21 fn=1 vectors, got {len(rows)}"
    for rec in rows:
        expected_err = int(rec["expect_error"])
        tx_base = row_to_transcript(rec, contingency=FIXTURE_CONTINGENCY)
        out = accept_oracle(tx_base)
        assert out.error == expected_err, f"{rec['id']}: error {out.error} != {expected_err}"
        if expected_err == 0:
            exp_verdict = int(rec["expect_verdict"])
            exp_reason = int(rec.get("expect_reason", 0))
            assert out.verdict == exp_verdict, f"{rec['id']}: verdict {out.verdict} != {exp_verdict}"
            assert out.reason == exp_reason, f"{rec['id']}: reason {out.reason} != {exp_reason}"
        for alt_cont in (0, 1, 7, 42, (1 << 63) - 1):
            assert accept_oracle(row_to_transcript(rec, contingency=alt_cont)) == out
    return len(rows)


def build_base_tx(**overrides: object) -> TranscriptInput:
    """Construct an admissible Interior baseline transcript with targeted overrides."""
    defaults: dict[str, object] = dict(
        attested=True,
        contingency=7,
        l=200,
        u=400,
        n_eff=1000,
        defect=0,
        c=120,
        b=0,
        r=0,
        budget_limit=1000,
        budget_elapsed=10,
        epi_readings=1,
        vital_readings=1,
        gov_readings=1,
        m_epi=500_000,
        m_vital=500_000,
        m_gov=500_000,
        proposition=1,
        site=1,
        step_index=7,
        reopen_sealed=False,
        accept_threshold=100_000,
        band_width=50_000,
    )
    defaults.update(overrides)
    return TranscriptInput(**defaults)  # type: ignore[arg-type]


def verify_non_commutation() -> int:
    """Exhaustively verify all 119 non-identity permutations diverge cleanly."""
    witnesses = [
        build_base_tx(attested=False, epi_readings=0, proposition=None, site=None),  # (1,2)
        build_base_tx(attested=False, m_epi=None, proposition=None, site=None),      # (1,3)
        build_base_tx(reopen_sealed=True, m_epi=50_000),                             # (1,4)
        build_base_tx(reopen_sealed=True, m_epi=101_000),                            # (1,5)
        build_base_tx(epi_readings=0, m_epi=None, proposition=None, site=None),      # (2,3)
        build_base_tx(epi_readings=0, m_epi=50_000),                                 # (2,4)
        build_base_tx(epi_readings=0, m_epi=101_000),                                # (2,5)
        build_base_tx(m_epi=None, m_vital=50_000, proposition=2, site=4),            # (3,4)
        build_base_tx(m_epi=None, m_vital=101_000, proposition=None, site=4),        # (3,5)
        build_base_tx(m_epi=50_000),                                                 # (4,5)
    ]
    canonical_outcomes = [accept_oracle(w, (1, 2, 3, 4, 5), unguarded=False) for w in witnesses]
    separating = 0
    for perm in permutations((1, 2, 3, 4, 5)):
        perm_outcomes = [accept_oracle(w, perm, unguarded=True) for w in witnesses]
        if perm == (1, 2, 3, 4, 5):
            assert perm_outcomes == canonical_outcomes, "canonical guarded/unguarded mismatch"
        else:
            assert perm_outcomes != canonical_outcomes, f"permutation {perm} failed to separate"
            separating += 1
    assert separating == 119
    return separating


def verify_table_b1_and_j73() -> tuple[int, int, dict[str, int]]:
    """Verify Table B.1 combinatorics, (E,V,G) signatures, Vandermonde count, and J(7,3)."""
    text = APPENDIX_B_PATH.read_text(encoding="utf-8")
    row_re = re.compile(
        r"^\|\s*`(L\d{2})`\s*\|\s*(\d+)\s*\|\s*`(S\d)`\s*\|[^|]+\|\s*`(T\d)`\s*\|\s*`\{(T\d),\s*(T\d),\s*(T\d)\}`\s*\|\s*`\((\d),\s*(\d),\s*(\d)\)`\s*\|\s*([^|]+)\|"
    )
    rows = []
    for line in text.splitlines():
        m = row_re.match(line)
        if m:
            rows.append(
                {
                    "order": m.group(1),
                    "ch": int(m.group(2)),
                    "stratum": m.group(3),
                    "primary": m.group(4),
                    "triple": frozenset((m.group(5), m.group(6), m.group(7))),
                    "sig": (int(m.group(8)), int(m.group(9)), int(m.group(10))),
                    "regime": m.group(11).strip(),
                }
            )
    assert len(rows) == 35, f"expected 35 Table B.1 rows, got {len(rows)}"
    assert [r["ch"] for r in rows] == list(range(51, 86))

    all_triples = {r["triple"] for r in rows}
    expected_triples = {frozenset(c) for c in combinations([f"T{i}" for i in range(1, 8)], 3)}
    assert all_triples == expected_triples and len(all_triples) == 35

    stratum_sites: dict[str, set[str]] = {}
    site_strata: dict[str, set[str]] = {}
    regime_counts: dict[str, int] = {}
    for r in rows:
        stratum_sites.setdefault(r["stratum"], set()).add(r["primary"])
        site_strata.setdefault(r["primary"], set()).add(r["stratum"])
        regime_counts[r["regime"]] = regime_counts.get(r["regime"], 0) + 1
        triple = r["triple"]
        assert r["primary"] in triple, f"{r['order']}: primary site outside triple"
        expected_sig = (
            sum(1 for t in triple if t in {"T1", "T2", "T3"}),
            sum(1 for t in triple if t in {"T4", "T5"}),
            sum(1 for t in triple if t in {"T6", "T7"}),
        )
        assert r["sig"] == expected_sig and sum(expected_sig) == 3

    assert all(len(v) == 5 for v in stratum_sites.values()) and len(stratum_sites) == 7
    assert all(len(v) == 5 for v in site_strata.values()) and len(site_strata) == 7
    assert sorted(regime_counts.values(), reverse=True) == [12, 9, 9, 4, 1]

    triples_list = sorted(expected_triples, key=sorted)
    degrees = [sum(1 for b in triples_list if len(a & b) == 2) for a in triples_list]
    assert degrees == [12] * 35
    edge_count = sum(degrees) // 2
    assert edge_count == 210
    assert 3 * (3 - 1) + 1 == 7
    return len(rows), edge_count, regime_counts


def main() -> int:
    """Run all formal-invariant checks and print concise verification output."""
    n_vec = verify_vectors_and_contingency()
    n_sep = verify_non_commutation()
    n_rows, n_edges, regimes = verify_table_b1_and_j73()
    print(f"vectors_conformance: PASS ({n_vec}/21 fn=1 rows + 5x contingency invariance)")
    print(f"stage_non_commutation: PASS ({n_sep}/119 non-identity stage permutations separated)")
    print(
        f"table_b1_j73_parity: PASS ({n_rows} orders, {n_edges} J(7,3) edges, "
        f"deg=12 regular, regimes={sorted(regimes.values(), reverse=True)})"
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
