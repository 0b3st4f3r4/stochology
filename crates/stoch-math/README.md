# stoch-math

Fixed-point Part IV/V pre-operators, relational null/conflict semantics, epistemic
bounds, and delay-stability primitives for Stochology (`#![no_std]`, zero external
dependencies, zero binary floats, `#![deny(unsafe_code)]`).

This crate holds pure integer arithmetic scaled at `UNIT = 1_000_000` (millionths),
strictly subordinate to the chapters in `book-src/`. Where prose and code diverge,
the chapter governs and this crate is repaired.

## Modules

- Root (`src/lib.rs`) — exact integer square roots (`integer_root`, `integer_root_i64`),
  four-state Codd relational values (`CoddValue`), validated closed intervals (`Interval`),
  Stevens measurement scales (`ScaleKind`), conservative interval widening
  (`widen_before_seal`), Lyapunov predictability horizon, Wirtinger–RCC delay bound
  (`thm3_max_delay`), Anderson–Morley 1-hop utilization, Kish and Moulton design effects,
  Chao1/Chao2 unseen-unit lower bounds, and `OperatorError`.
- `epistemic` — Part IV Operator I (`effective_sample`, additive design-effect
  composition, product majorant bound, and Meng's non-response bias identity).
- `vital` — Part IV Operator II (`ReserveDynamics`, `volatility_drag`, path vs.
  ensemble growth, monotone `recovery_time` bisection, and `SurvivalOutcome`).
- `governance` — Part IV Operator III (`Bitemporal` responsibility fold, `Integrity`
  discharge gate, and caller-pivoted `DifferentialSign`).
- `paraconsistent` — pre-decisional Filter I conflict/gap containers (`FdeState`
  Belnap–Dunn four-valued lattice, LFI `LocalConsistency`, `Filter1Claim`, and `LPA2v`
  `AnnotatedEvidence`), strictly barred from replacing live `stoch-core` verdicts.

## Tests

```sh
TMPDIR="${TMPDIR:-$HOME/.cache/rs-tmp}" cargo test --offline -p stoch-math
```
