# ABI cross-engine parity witnesses

**This consumer is a test double built to rehearse the ABI; the trigger of
PLAN §4.2 remains unmet.**

> **Design documents cited.** `02-arquitetura.md`, `03-cadeia-de-suprimentos.md` and
> `PLAN.md` are internal project documents and are not part of this repository. The
> references stand as a record of the design these doubles rehearse; nothing here needs
> them to be opened in order to run.

Two runtimes, two independent engines, one port:

| Path | Runtime | Path to the ABI |
| :--- | :--- | :--- |
| `consumers/node/run.mjs` | Node 24 / V8 (TypeScript by *type stripping*) | through the `adapters/ts/adapter.ts` adapter (0 external deps) |
| `crates/host-workspace/stoch-aot/tests/wasmtime_parity.rs` | Rust + Wasmtime | straight on the five exports, no TypeScript adapter |

The second exists so that the first is not the only witness: if both agree on the same
bytes, a disagreement between them would have to come from the port, and not from a
shared reading. Agreeing does **not** prove that the vectors cover the domain
(`02-arquitetura.md` §8.3).

```sh
node consumers/node/run.mjs        # 55 rows compared with vectors.kv, 2 declared inexpressible
(cd crates/host-workspace && cargo test --offline --locked -p stoch-aot --test wasmtime_parity)
```

## Open limits

`consumers/node/README.md` records the two wire-expressibility limits (`L2`, `L3`) for
rows `a10` and `a11`, and the top-of-memory arena placement rule (`L4`).
