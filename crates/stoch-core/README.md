# stoch-core

The deterministic core of the framework: one operator, one predicate, one fold.

This crate holds the three functions that Appendix B.1 and `01` fix — `step`, `accept`,
`diagnose` — as plain Rust with integer arithmetic, no dependencies, and no wire format.
The WebAssembly boundary, the byte-level transcript parser and the ABI error codes are
later phases; nothing here knows about bytes, hosts or clients.

```
State    --step-->    Regime        the canonical operator, per step
Sentence --seal-->    Transcript
Transcript --accept--> Verdict      the ordered predicate, over a finished trajectory
Trajectory --diagnose-> Diagnosis   the fold, which reports and never acts
```

The three are separated by type, not by convention: `step` cannot see a `Verdict`,
`accept` cannot see a `State`, and `diagnose` names neither a `Regime` nor an
`ActionSet`. `Diagnosis` is opaque — it has no public constructor, no public field, no
`Debug` and no `Deref`, and its only method returns a read-only `DiagnosisView`. The
`compile_fail` doctests on the crate root and on `Diagnosis` are the proof, and
`tests/separation.rs` scans the production source for the constructions that would
break it.

## Constants

| Name | Value | Meaning |
| :--- | :--- | :--- |
| `SCALE` | `1_000_000` | one unit of the fixed-point scale |
| `MAX_ABS` | `1_000_000_000` | the bound on every sealed quantity |
| `VECTORS_DATA_ROWS` | `52` | records of `vectors.kv`, comment lines excluded |
| `vectors_hash()` | `efe2bad3…ed6bd3d0` | SHA-256 of the fixture, whole file |

Every arithmetic operation is checked. `checked_add`, `checked_sub`, `checked_mul` and
`checked_div` appear on every path that can leave the integer range, and an overflow is
`E_SCALE_OVERFLOW`, never a wrap. There is no binary floating point anywhere in the
source: `tests/separation.rs` scans for the two type names and fails if either appears,
and `#![deny(clippy::float_arithmetic)]` refuses the operations even if a type slipped
in. The one division in the operator, `probe_fraction`, truncates toward zero, which is
the floor under the licence `L < C < U`; a quotient of zero is `E_PROBE_UNDERFLOW`, so a
probe that would read as "act now" is refused rather than rounded into existence.

## The predicate

Five stages, in the order of Appendix B.1, and the order is the content: a later stage
presupposes the answer to the earlier one, so the predicate does not commute.

1. **Skeleton.** The sentence must be well formed and inside its budget.
2. **Axial coverage.** Every axis the fold will read must carry a reading.
3. **Finiteness.** A reading must be a value; absence is not zero.
4. **The Leontief fold.** The verdict is the minimum of the three axes.
5. **The frontier band.** The minimum is compared with the declared threshold.

Two gates sit before the stages, because they are questions about the sentence rather
than about its verdict: the axis of the declared site must be the axis of the declared
proposition (`E_SITE_PROPOSITION_MISMATCH`), and a collapse must name the proposition,
the site and the step that carried it — an unnamed collapse is quarantined as
unparsable, never accepted as an anonymous failure. Naming is not enough: the named
site's own axis has to carry the fold, in the collapse as in the boundary. A site on a
healthy axis would report a failure of a plane reading above the fold, so that sentence
is unparsable too. Ties are coherent, because equality attains the fold.

The four verdicts are `Interior`, `Boundary`, `Collapse` and `Unparsable`, with tags
`0..3` in that order, which is the order the fixture's `expect_verdict` key uses.
A `Boundary` names the declared site *and* the fold, so the two must agree: the site's
own axis has to attain the fold (equality attains it, so any site of a tied axis is
coherent). A declared pair that does not agree is an ill-formed sentence and answers
`Unparsable`, because a boundary printed over a margin its axis does not carry would be
a claim the record cannot support.
`Unparsable` carries the stage that refused the sentence (`Skeleton`, `AxialCoverage`,
`MarginMissing`, `BudgetExceeded`, codes `1..4`).

## The operator

Six regimes, tags `0..5` in declaration order: buffered, reversible fractional probe,
inward action, quarantine, algedonic halt, no action. The operator asks three questions
in order, and the order is the doctrine: an incomplete instrument is a quarantine even
when the latency condition also holds; a survival latency outranks a governance
corruption, which outranks a floor breach. `Regime::NoAction` is constructed in exactly
one place — the `U < C` branch — and `ActionSet::None` is produced by exactly one
variant; both facts are asserted rather than described.

## Tests

```
TMPDIR="${TMPDIR:-$HOME/.cache/rs-tmp}" cargo test --offline
```

The `TMPDIR` prefix is a property of this machine, not of the crate: the default
temporary directory is a nearly full tmpfs and the linker dies with a bus error when it
runs out of space mid-link. Nothing in the test suite depends on it.

| Suite | What it holds |
| :--- | :--- |
| `src/*.rs` unit tests | the operator's branch table, the probe arithmetic, the fold |
| `src/step.rs` grid tests | the 96-state grid and the guarded biconditionals |
| `tests/conformance.rs` | all 52 fixture rows, and the coverage they certify |
| `tests/arithmetic.rs` | truncation, the bound at the door, the twelve error codes |
| `tests/integrity.rs` | fixture byte equality, the digest, the concordance scan |
| `tests/separation.rs` | the type separation, determinism, contingency independence |

The grid is `2^5 = 32` boolean assignments against the geometries of `02` §2.5.3, and it
is run twice: once as 96 states over three instrument conditions, once as 96 states over
six geometries. The second run is the one that matters, because it checks the guarded
biconditionals on every cell and shows that the naive forms of the same biconditionals
fail — a biconditional that no cell falsifies is not being tested.

## Deliberate deviations, recorded

- `Transcript::seal` takes a `Sentence`, not a byte slice. The byte parser is an adapter
  in `01` §8, and admitting bytes here would put a parser inside the audited core before
  its ABI exists.
- `E_CALLER_CONTRACT` (`-14`) is absent: it belongs to the boundary, and this crate has
  no boundary. `E_REGION_UNLICENSED` (`-13`) stays burned, so the twelve codes that
  remain keep their normative numbers.
- Thresholds live in the sealed record, in both the `Params` the operator reads and the
  `Assessment` the predicate reads, so neither function needs a configuration argument
  and a verdict cannot be moved by changing a global.
