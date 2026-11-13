# `adapters/ts` — the TypeScript duplex of the Stochology ABI

**These two consumers are test doubles built to rehearse the ABI; the trigger of
the archived plan (§4.2) remains unmet.**

This is the §8.2 adapter of the archived design document (`02-arquitetura.md`), written against
`crates/stoch-abi` and against nothing else. It embeds `stoch_abi.wasm`, performs the
version handshake, lays the caller's arena at the **top of memory**, and decodes the
five exports into named results.

Two files, no dependencies, no build step: Node 22+ strips the types and runs them.

```
node --test adapters/ts/adapter.test.ts      # 15 tests, one per divergence point plus the handshake
```

## What it does not do

- It does not re-implement `step`, `accept` or `diagnose`. Every decision about a
  regime, a verdict or a fold is the module's; the adapter moves bytes and names them.
- It does not convert `i64` to `number` anywhere on a 64-bit path. `toSafeNumber` is the
  single declared conversion, and it throws `E_BIGINT_PRECISION` past ±9007199254740991n.
- It does not derive the arena from the module's layout. The module has no allocator and
  no imports, so nothing above its statics is ever in use; the arena is therefore the top
  region of memory, `arenaBase(memoryBytes, steps) = memoryBytes - arenaBytes(steps)`
  with `arenaBytes = 464 + steps * 320`. The base is re-derived on **every** call, so a
  grown memory moves it, and `assertBase(base, floor)` refuses anything below the image
  the module was instantiated with — the statics live inside that image and growth only
  appends pages above it, so the base cannot land in the store. No `__data_end`, no
  `1048640`, no measured floor is used to place anything.
  `readDataEnd` is still parsed, but only as a reported diagnostic (`core.dataEnd`).
- It derives `ActionSet` on this side of the boundary, because by D13 that table does
  not cross: `actionSet(tag)` is `2` only for `NoAction` (tag 5).

## The 14 divergence points

| Point | What the adapter does | Test |
| :-- | :-- | :-- |
| D1 | `bigint` everywhere; one declared conversion that refuses 2^53 | `D1 — i64 crosses as bigint…` |
| D2 | a fractional probe is the module's truncating division, `714285` | `D2 — the fraction comes from the module…` |
| D3 | every `DataView` is little-endian; a big-endian read is refused as scale | `D3 — the record is little-endian…` |
| D4 | `-1` is `Missing` on the wire, and it is not `0` | `D4 — -1 is Missing…` |
| D5 | a tag outside `0..=5` throws `E_UNKNOWN_TAG` | `D5 — a tag outside 0..=5 is refused…` |
| D6 | the view is rebuilt after `memory.grow` | `D6 — after memory grows the view is rebuilt…` |
| D7 | the arena is the top of memory, above the module's store; a base below the floor is refused | `D7 — the arena sits at the top of memory…` |
| D8 | a negative return throws with the core's code; it is never a tag | `D8 — a negative return throws…` |
| D9 | Q1 preempts Q2: quarantine before the latency halt | `D9 — Q1 preempts Q2…` |
| D10 | the band is half-open: `threshold + width` is interior | `D10 — the band is half-open…` |
| D11 | zero imports, six exports, instantiation against `{}` | `D11 — the module declares no import…` |
| D12 | the digest is compared with the fixture on disk | `D12 — the digest the module carries…` |
| D13 | `ActionSet` derived here; only tag 5 is `None` | `D13 — ActionSet is derived on this side…` |
| D14 | an unknown payload throws `E_UNKNOWN_PAYLOAD` | `D14 — an unknown payload is refused…` |

`§8.3` of the same chapter bounds what this proves: two ports agreeing is not evidence
that the vectors cover the domain. That claim is not made here.

## Proof that each point's test actually fails

A test named after a divergence point proves nothing if it would also pass on a broken
adapter. `mutation.mjs` closes that gap: for each of the fourteen rows it applies one
targeted violation to a copy of the adapter, runs that row's test against the copy, and
requires the run to fail — after first running the same test against the unmutated
adapter and requiring it to pass, so a mistyped pattern cannot masquerade as a caught
violation.

```
node adapters/ts/mutation.mjs
```

Result on this port: **14 of 14 points are caught by their own test**, exit `0`.

Two of the fourteen mutations had to be retargeted before they proved anything, and the
reason is worth keeping:

- **D4** was first mutated on the *write* side (`-1n` coerced to `0n`). The test still
  passed: with every reading zero the fold is `0`, which falls under the threshold with
  no declaration, so the verdict is `Unparsable` — tag `3` either way. A mutation that
  leaves the observed tag unchanged is not a violation of the point. The mutation now
  coerces on the *read* side, where the test asserts `-1n` survives.
- **D12** was first mutated by moving the digest address (`base + DIGEST_AT + 1`): the
  module writes and the adapter reads at the same address, so the digest still matched.
  The mutation now moves only the module's write (`vectors_hash(at + 1)`), so the
  comparison is between one stale byte and thirty-one good ones.

## Why the arena is at the top (and not at a measured floor)

The module's statics begin at `align8(__data_end)`, and `diagnose` folds into a static
`[StepReading; 1024]` that starts there: 64 bytes per step. Placing the caller's arena at
that address — which is what a literal reading of the boundary's own arena base does —
puts records and fold on the same bytes. Two ways out exist: measure the store and start
above it, or stop measuring. The second is taken here, because it is the only placement
that does not depend on a number this build happens to have: nothing above the statics is
in use (no allocator, no imports), so the top of memory is free, and `memory.grow` —
which only appends — keeps the arena above the store by construction. The ABI now also
refuses a pointer that overlaps the store with `-14`, so a caller who tries the measured
floor is told, not silently corrupted.

Recorded as a lesson: the area above the store is free because the module never
allocates; the top of memory is the only placement that needs no measurement.
