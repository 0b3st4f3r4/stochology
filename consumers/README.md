# ABI test-double consumers

**These two consumers are test doubles built to rehearse the ABI; the trigger of
PLAN §4.2 remains unmet.**

> **Design documents cited.** `02-arquitetura.md`, `03-cadeia-de-suprimentos.md` and
> `PLAN.md` are internal project documents and are not part of this repository. The
> references stand as a record of the design these doubles rehearse; nothing here needs
> them to be opened in order to run.

Two consumers, two ecosystems, one port:

| Path | Runtime | Path to the ABI |
| :--- | :--- | :--- |
| `node/run.mjs` | Node 24 (TypeScript by *type stripping*) | through the `adapters/ts/adapter.ts` adapter |
| `python/run.py` | Python 3 + Wasmtime | straight on the five exports, no adapter |

The second exists so that the first is not the only witness: if both agree on the same
bytes, a disagreement between them would have to come from the port, and not from a
shared reading. Agreeing does **not** prove that the vectors cover the domain
(`02-arquitetura.md` §8.3).

```
node consumers/node/run.mjs        # 50 rows compared with the fixture, 2 inexpressible
python3 consumers/python/run.py    # boundary invariants, no adapter
```

## Operational note: `pip`, Wasmtime and the `/tmp` tmpfs

Installing Wasmtime on this machine with the default `TMPDIR` (`/tmp`, a tmpfs that
usually runs above 80% occupancy) fails during unpack:

```
OSError: [Errno 122] Disk quota exceeded
```

It works with `TMPDIR` set to a directory under `home`, or with `--target` pointing to a
directory under `$HOME` added to `PYTHONPATH`. This is a packaging quirk of the machine,
not of the adapter or of the port.

## Open limits

Each consumer records its own in its directory's README; the most important is in
`python/README.md` (L4): the arena base the ABI declares is the same address where the
static buffer of `diagnose` lives.
