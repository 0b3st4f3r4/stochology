# Test adapters

**These two consumers are test doubles built to rehearse the ABI; the trigger of
PLAN §4.2 remains unmet.**

> **Design documents cited.** `02-arquitetura.md`, `03-cadeia-de-suprimentos.md` and
> `PLAN.md` are internal project documents and are not part of this repository. The
> references stand as a record of the design these doubles rehearse; nothing here needs
> them to be opened in order to run.

This directory holds the adapter of one of the two consumers. The adapter is not a
product and not a support promise: it exists to exercise the wasm boundary of
`crates/stoch-abi` from a language runtime, and to give the second consumer (Python,
straight on Wasmtime, in `consumers/python/`) something to disagree with.

## Contents

| Path | Role |
| :--- | :--- |
| `ts/adapter.ts` | TypeScript adapter: version handshake in `bigint`, arena in the caller's memory, a single declared `bigint → number` conversion point, decoding of both output records. |
| `ts/adapter.test.ts` | The fourteen divergence points of `02-arquitetura.md` §8.2 (D1–D14), one test per point. |
| `ts/mutation.mjs` | Mutation study: each of the fourteen points has a mutation that only its own test catches. |
| `ts/README.md` | Table of the fourteen points, known limits, and how to run. |

## How to run

```
node --test adapters/ts/adapter.test.ts
node adapters/ts/mutation.mjs
```

Both are needed: the first shows the tests pass against the real port, the second that
each test fails when its point is violated — a test that does not fail when the point
changes proves nothing about the point.

## Declared limit

The rehearsal proves that two ports agree, never that the vectors cover the domain
(archived design document, `02-arquitetura.md` §8.3). Coverage is the subject of
`01-funcionalidades.md` §6.3, also archived, and of the harness of `crates/stoch-vectors`,
not of this directory.
