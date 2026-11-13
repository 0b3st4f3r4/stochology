# stoch-vectors — the normative harness

A harness over [`stoch-core`](../stoch-core). It has one dependency, the core it
exercises, and no opinion of its own: every observation here is a call into
`step`, `accept`, `seal` or `diagnose` followed by a rendering of what came back.
`step`, `accept` and `diagnose` are not reimplemented.

```
TMPDIR=/home/silvano-neto/.cache/rs-tmp cargo test --offline
TMPDIR=/home/silvano-neto/.cache/rs-tmp cargo run --offline -p stoch-vectors -- check
```

## Subcommands

| Command | What it does | Exit |
| :--- | :--- | :--- |
| `check [--vectors PATH]` | Runs every row of the fixture and compares the canonical line with the declaration in the row. | 1 on divergence, per-row `MISMATCH` with `declared:`/`returned:` |
| `gen` | Prints the generated coverage corpus (48 rows, `g01`..`g48`). | 0 |
| `census` | Runs the generated corpus and reports what §6.3 requires and it does not reach. | 1 if an item is uncovered |
| `topology` | Reads Table B.1 and the preface, reconciles the 35 triples with `binom(7,3)` and the Vandermonde partition. | 1 on divergence |
| `concordance` | Reads §V-9 from `AGENTS.md` and sweeps `book-src/` and `crates/**/*.rs`. | 1 on violation |
| `operators` | Runs one declared example of each canonical operator and prints the margins and durations. | 0 |

## The five instruments

- **`fixture`** — the 52 normative vectors of `crates/stoch-core/vectors.kv`, with
  the same decoding as the core's own conformance test. The canonical line is one
  spelling per answer (`step regime=… cause=… constraint=… fraction=… missing=…`,
  `accept verdict=…`, `diagnose binding=… absorbed=… ruin=… reserve=…`, `seal sealed`,
  or `… err=<code>`), so `declared` and `returned` sit on the same line and a
  divergence is readable without a diff.
- **`generator`** — a second, disjoint corpus that reaches every item of the
  minimum coverage list `01` §6.3: six regimes, four verdicts, seven collapse
  sites, seven quarantine slots, four bindings, and the named edges — `L == C`,
  `U == C`, `U < C`, `TTS == TTR`, truncating division, probe underflow,
  irreversible probe refusal, a missing score, absorption, instrument-before-latency,
  latency-before-geometry, an unnamed collapse, a site/proposition mismatch.
  A census over the *rendered* rows checks the reach; a flag per item checks that
  the edge itself is on the row, not merely that a histogram bucket is full.
- **`contract`** — §V-9 read from `AGENTS.md` at run time (a missing file is a
  failure that names the path), each canonical string required to occur in
  `book-src/`, each contradicting variant forbidden everywhere in `book-src/` and
  in `crates/**/*.rs`, after collapsing whitespace runs.
- **`topology`** — `binom(7,3) = 35`, the Vandermonde partition, and Table B.1
  parsed from the appendix and the seven anchors from the preface. Explored, not
  gated: nothing here can return a verdict, and no type of this crate is
  serializable or reachable from `accept`. The book's `12 + 9 + 9 + 4 + 1` is
  recovered as a count of the five exposure labels; the raw axis classes are
  `V = 0: 10`, `G = 0: 10`, `E = 0: 4`, and the pure epistemic triple — which is
  in both zero-classes — is drained by its own label, which is why the two nines
  and not two tens.
- **`schema`** — the record, its grammar, its parser and its renderer.

## The fixture's grammar

The normative fixture is `crates/stoch-core/vectors.kv`: 52 records, one per line,
each a sequence of `key=value` pairs separated by exactly one space, under three
comment lines that name the file, say what it is, and point at the grammar here.
A line whose first non-blank character is `#` is a comment and a blank line is
ignored. There is no header line, no comma, no quote, no tab and no carriage
return; a record has no leading or trailing space.

The parser (`src/schema.rs`) looks a key up by name, never by position. The
canonical order of `HEADER` is how the file is written so a human can read it in
columns, and the schema's own tests shuffle a record's keys to prove nothing
depends on that order. An unknown key, a repeated key, a field without `=`, a
non-integer value and a line ending in `\r` are errors, each naming its line and,
where there is one, its key.

**Absence is a value, not a defect.** A key the record does not name reads as
`Row::ABSENT`, which is `-1` — the marker the core already uses for a missing
reading. The fixture never writes `-1`: a record that fills a cell with it is not
spelling absence, and the renderer omits absent keys rather than writing them out.
This is why a corrupt cell cannot hide: every value the record declares is on the
line, and everything else is declared absent.

**Required keys per entry point.** A required key is one the harness dereferences
for that `fn` *and* one every record of that `fn` in the corpus states. Where
absence is itself the declared value, the key is not required:

| `fn` | entry point | required keys | read at |
| :--- | :--- | :--- | :--- |
| `0` | `step` | `id fn tape instrument missing_slot tts ttr l u c r reversible` | `src/fixture.rs:288-306` |
| `1` | `accept` | `id fn l u c epi vital gov site` | `src/fixture.rs:239-245`, `260-261` |
| `2` | `diagnose` | `id fn absorbed expect_binding_axis expect_error` | `src/fixture.rs:98`, `146-149`, `272` |
| `3` | `seal` | `id fn tape instrument missing_slot tts ttr u c r reversible` | `src/fixture.rs:288-306` |

Every key in that table is read by the harness on the entry point it is required
of; the citation is the line that reads it. Two absences are deliberate and so are
not requirements:

- `l` on `seal`. `Bounds::new(row.at(C_L), …)` at `src/fixture.rs:300` reads the
  extent, and one seal record declares the extent *unprovided* by not naming `l`.
  Demanding it would turn that declaration into a parse error.
- `r` on `accept` and `diagnose`, and `m_epi m_vital m_gov` on `step`. These are
  read through `.max(0)` (`src/fixture.rs:261`, `278`, `294`), so absence reads
  `0` — the value the record meant — rather than a value the harness cannot
  interpret. On `step` the margin is discarded anyway: `seal_state` supplies the
  flat `500_000` whenever the entry point does not need the declared one.

Conditional keys follow from what the record declares. A record whose
`expect_error` is not `0` names only that code. A `step` record expecting success
names `expect_variant`; an `accept` record names `expect_verdict` and then, by
verdict, the margin of its own axis (`1`), `proposition` and `step_index` (`2`) or
`expect_reason` (`3`). `diagnose` reads `expect_binding_axis` and `expect_error`
unconditionally: it has no other expectation column to fall back on.

## Separation (Chapter 41 §4)

The core proves the type-level separation; `tests/separation.rs` proves the
behaviour:

- **prefix** — two states differing only in the reserve (the record of where the
  trajectory has been) return the same regime, and the states compare unequal; a
  difference written into a field the operator reads moves the regime; a state
  whose tape was never attested cannot be sealed at all, so no undeclared prefix
  can reach the operator.
- **contingency** — four verdict shapes, two contingency records; the verdicts do
  not move, and the four shapes are the four verdict tags, so the equality is not
  the equality of four errors.
- **seal** — reporting the fold twice is the same report, a revision is a second
  transcript and cannot revise the first, and the two `compile_fail` doctests in
  `src/lib.rs` show that neither a `Verdict` nor a `Diagnosis` can be handed back
  to the operator that produced it.

## Corrupting one cell

Copy the fixture, change one expectation, and the harness names the row:

```
$ cp crates/stoch-core/vectors.kv /tmp/vectors.kv   # then set s02's expect_fraction to 495000
$ cargo run -q -p stoch-vectors -- check --vectors /tmp/vectors.kv
52 rows: step 22, accept 20, diagnose 6, seal 4
MISMATCH s02
  declared: step regime=1 cause=- constraint=- fraction=495000 missing=-
  returned: step regime=1 cause=- constraint=- fraction=500000 missing=-
1 of 50 rows diverge
$ echo $?
1
```

The same happens for a corrupted verdict (`accept verdict=0` declared where the
operator returns `accept verdict=2 proposition=1 site=1 step=7`) and for a
corrupted input (`s02`'s capacity raised by one gives `fraction=495000`), which is
how the tests show the harness reads the inputs and not only the declarations.

## The Three Canonical Operators (`src/operators.rs`)

Part IV's three transformations, as a declared integer layer *above* the step.
They take the quantities the discrete core already reduces — `[L, U]`, `C`, `TTS`,
`TTR`, the three margins — and produce margins and durations. They are not
arguments of `step`, they never enter the acceptance path, and they never fold the
three coordinates into one number; the Leontief minimum stays what the core does
with a finished trajectory. Two `compile_fail` doctests in the module prove the
first two clauses at compile time, and `tests/operators.rs` proves the last two by
scanning the module's own code.

- **I — epistemic** (`43`–`45`). `effective_sample` contracts by a design effect;
  `compose_design_effects` composes several of them additively, `1 + Σ (d_k − 1)`;
  `design_effect_product_bound` returns the effects multiplied, exposed *as a
  bound* and not as the product: it truncates at every step, so it is an upper
  bound on the additive composition and a lower approximation of the exact
  product, never the product itself. It can still coincide with the composition
  while more than one effect exceeds the unit, because the excess of the product
  over the linear term is then smaller than the unit each division truncates away;
  `[UNIT + 1, UNIT + 1]` is such a list — the excess is `1 / UNIT`.
  `Nonresponse` is the declared non-response parameter (`rho`, `sigma`, response
  fraction), with `sigma` capped at the declared `SIGMA_CAP` (the core's absolute
  bound) so the squared identity cannot leave the wide range; `meng_bias_squared`
  evaluates the identity in its squared domain with a checked multiplication
  chain, refuses a negative quotient, and truncates once; `meng_bias` takes the
  integer square root with the truncation declared
  (`bias² ≤ squared < (bias+1)²`), `signed_bias` carries the sign of `rho`, and
  `corrected_level` subtracts it — the correction is the identity exactly when
  `rho = 0`, which is what "ignorable" means here.
- **II — vital** (`46`–`47`). `volatility_drag` is `sigma² / 2`, the second-order
  term that separates the growth of one path (`time_growth`) from the growth of the
  cross-section (`ensemble_growth`); their difference is the non-ergodic gap, and
  the gap is the drag. A tail draw of `k` scales can ruin the path while the
  cross-section climbs. `recovery_time` finds the *least* crossing by bisection over
  the monotone reserve map — outside the step and outside any fixed point of it —
  and decides existence first, so `None` means "no crossing inside the horizon".
  `survival_condition` is the predicate `TTS_survival >= TTR_recovery`, and
  `SurvivalOutcome` names the three answers: `RecoversInTime` (tag 0),
  `CrossesAfterRuin` (tag 1), `NoCrossingWithinHorizon` (tag 2).
- **III — governance** (`48`–`49`). `Bitemporal` holds three readings of *one*
  coordinate (before, at the moment, after) and `responsibility` is their minimum —
  a fold over time, not over the three axes. `Integrity` is the performative gate:
  `ratio` is discharges against commitments, truncating, and not clamped above one,
  because over-discharge is a fact the gate exists to show. `differential_sign`
  reads the sign from the coordinate's position relative to a **caller-declared**
  pivot, and `signed_differential` applies it: the magnitude never chooses the sign
  and the coordinate never chooses the magnitude. The pivot is a convention of the
  caller, not a value this layer invents, and it is *not* folded into the permitted
  side: on `governance == pivot` the differential is zero and the answer is the
  named arm `DifferentialSign::Neutral` (`factor` 0, `tag` 0), so a tie is never
  read as permission. The three arms, published as the CLI prints them:

  ```
  sign(below)=Negative factor=-1 sign(tie)=Neutral factor=0 sign(above)=Positive factor=1
  ```

No `log` and no `exp` appear: the drag is the exact second-order term, the square
root is an integer bisection with its truncation stated, and the survival question
is a comparison. Zero floats in the source.

### Pending specification decisions (not resolved here)

Two questions belong to the specification, not to this layer, and are recorded
rather than answered:

1. **Per-axis margins in the fixture.** This was open and is now closed: the 33
   records of `vectors.kv` (`01` §6.4) carry `m_epi`, `m_vital` and `m_gov`, so a
   row can write a non-degenerate Leontief minimum and the fold is the minimum of
   the three. Vectors `a13`–`a18` exercise it, including the boundary/skeleton
   coherence of B.1 line 163 in both directions.
2. **`StepReading` in the ABI, or declared by the caller?** `diagnose` takes
   readings of a step; whether that type belongs in a future wire format or is
   declared by whoever calls the fold is undecided. This crate keeps it a caller
   type and declares its fields inline, so either decision can be taken without
   rewriting the operators.
