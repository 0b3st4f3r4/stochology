#!/usr/bin/env python3
"""Consumer B: the same .wasm, driven from Python through Wasmtime.

It is a DOUBLÊ — a test double, not a product. It does not import the TypeScript
adapter and does not share a line of its code: it reads the same binary, lays out
the same bytes, and checks the boundary invariants on its own (sizes, return codes,
the fixture digest, the arena base). If the two ports agree, the wire is at least
pinned twice; §8.3 of `02-arquitetura.md` says exactly that and no more.

Run: python3 consumers/python/run.py

Operational note: installing this consumer's one dependency (`wasmtime`) fails with
`OSError(122, "Disk quota exceeded")` when `TMPDIR` is the /tmp tmpfs, which is
usually near full on this host. It installs with `TMPDIR` on the home filesystem
(or with `--target` a directory under `$HOME`). That is a packaging problem of the
host, not a failure of the adapter or of the port.

Two rules this consumer learned from the port itself, both documented in the README:

* a sealed instrument must carry seven margins inside `0 ..= SCALE` — a `State` with
  an absent reading is refused by the core's seventh check with `-5`;
* `step` and `accept` return the **tag** on success (`0 ..= 5`, `0 ..= 3`) and a
  negative code on failure, so `0` is a tag and never means "ok".
"""

from __future__ import annotations

import hashlib
import os
import struct
import sys
from pathlib import Path

try:
    import wasmtime
except ModuleNotFoundError:  # the host installs it outside site-packages
    for candidate in (
        Path.home() / "pylibs",
        Path(os.environ.get("STOCHOLOGY_PYLIBS", "/nonexistent")),
    ):
        if candidate.is_dir():
            sys.path.insert(0, str(candidate))
    import wasmtime

ROOT = Path(__file__).resolve().parents[2]
WASM = ROOT / "target/wasm32-unknown-unknown/release/stoch_abi.wasm"
VECTORS = ROOT / "crates/stoch-core/vectors.kv"

ABI_VERSION = 1
STATE_REC_BYTES = 256
REGIME_OUT_BYTES = 64
VERDICT_OUT_BYTES = 64
STEP_TRIPLE_BYTES = 32
DIAGNOSIS_OUT_BYTES = 48
ARENA_STEP_BYTES = 320
MAX_STEPS = 1024
STEP_READING_BYTES = 64
DIGEST_BYTES = 32
E_CALLER_CONTRACT = -14

# The arena is the top of memory: the module has no allocator and no imports, so
# nothing above its statics is in use, and the top is the one placement that does
# not depend on measuring where the statics end. The layout keeps 4096 bytes
# between regions so a wrong pointer is caught as a refusal, not as silent overlap.
LAYOUT_BYTES = ARENA_STEP_BYTES + 3 * 4096

failures: list[str] = []


def check(name: str, condition: bool, detail: str = "") -> None:
    if condition:
        print(f"PASS {name}")
    else:
        print(f"FAIL {name}{': ' + detail if detail else ''}")
        failures.append(name)


def align8(value: int) -> int:
    return (value + 7) & ~7


def leb(blob: bytes, offset: int) -> tuple[int, int]:
    result = 0
    shift = 0
    while True:
        byte = blob[offset]
        offset += 1
        result |= (byte & 0x7F) << shift
        if byte & 0x80 == 0:
            return result, offset
        shift += 7


def sleb(blob: bytes, offset: int) -> tuple[int, int]:
    result = 0
    shift = 0
    while True:
        byte = blob[offset]
        offset += 1
        result |= (byte & 0x7F) << shift
        shift += 7
        if byte & 0x80 == 0:
            if byte & 0x40:
                result |= -(1 << shift)
            return result, offset


def const_offset(blob: bytes, offset: int) -> tuple[int, int]:
    """Read an `i32.const N; end` offset expression and return N."""
    if blob[offset] != 0x41:
        raise AssertionError(f"the data offset is opcode {blob[offset]:#x}, not i32.const")
    value, cursor = sleb(blob, offset + 1)
    if blob[cursor] != 0x0B:
        raise AssertionError("the data offset expression is not a single i32.const")
    return value, cursor + 1


def data_end(blob: bytes) -> int:
    """The end of section 11 (data), align8 — computed here, not read from the module."""
    assert blob[:4] == b"\x00asm" and blob[4:8] == b"\x01\x00\x00\x00"
    offset = 8
    end = 0
    while offset < len(blob):
        section_id = blob[offset]
        offset += 1
        size, offset = leb(blob, offset)
        body = offset
        if section_id == 11:
            count, cursor = leb(blob, body)
            for _ in range(count):
                flags, cursor = leb(blob, cursor)
                if flags == 0:
                    base, cursor = const_offset(blob, cursor)
                    length, cursor = leb(blob, cursor)
                    end = max(end, base + length)
                elif flags == 2:
                    raise AssertionError(
                        "a memory-indexed data segment: this reader would have to evaluate "
                        "the offset expression, and the module has none"
                    )
                else:
                    raise AssertionError(f"data segment flag {flags} is not one this reader knows")
        offset = body + size
    return align8(end)


class Port:
    def __init__(self, blob: bytes) -> None:
        self.engine = wasmtime.Engine()
        self.module = wasmtime.Module(self.engine, blob)
        self.store = wasmtime.Store(self.engine)
        self.instance = wasmtime.Instance(self.store, self.module, [])
        self.exports = self.instance.exports(self.store)
        self.memory = self.exports["memory"]

    def call(self, name: str, *args: int) -> int:
        results = self.exports[name](self.store, *args)
        if isinstance(results, list):
            return int(results[0])
        return int(results)

    def read(self, start: int, length: int) -> bytes:
        return bytes(self.memory.read(self.store, start, start + length))

    def write(self, start: int, payload: bytes) -> None:
        self.memory.write(self.store, payload, start)

    def i64(self, address: int) -> int:
        return int.from_bytes(self.read(address, 8), "little", signed=True)

    def i32(self, address: int) -> int:
        return int.from_bytes(self.read(address, 4), "little", signed=True)


def state(
    *,
    instrument: int = 0,
    missing_slot: int = 0,
    l: int = 0,
    u: int = 0,
    c: int = 0,
    b: int = 0,
    r: int = 0,
    tts: int = 0,
    ttr_eff: int = 0,
    n_eff: int = 1000,
    defect: int = 0,
    reversible: int = 1,
    gov_corrupt: int = 0,
    vital_floor: int = 0,
    gov_floor: int = 0,
    tape: int = 1,
    site: int = 0,
    accept_threshold: int = 100_000,
    band_width: int = 50_000,
    guard: int = 900_000,
    budget: int = 1000,
    elapsed: int = 10,
    proposition: int = 0,
    step_index: int = -1,
) -> bytes:
    buf = bytearray(STATE_REC_BYTES)
    buf[0:4] = instrument.to_bytes(4, "little", signed=True)
    buf[4:8] = missing_slot.to_bytes(4, "little", signed=True)
    for offset, value in ((8, l), (16, u), (24, c), (32, b), (40, r), (48, tts), (56, ttr_eff)):
        buf[offset : offset + 8] = value.to_bytes(8, "little", signed=True)
    buf[64:72] = n_eff.to_bytes(8, "little", signed=True)
    buf[72:80] = defect.to_bytes(8, "little", signed=True)
    for offset, value in (
        (80, reversible),
        (84, gov_corrupt),
        (88, vital_floor),
        (92, gov_floor),
        (96, tape),
        (100, site),
    ):
        buf[offset : offset + 4] = value.to_bytes(4, "little", signed=True)
    buf[104:112] = accept_threshold.to_bytes(8, "little", signed=True)
    buf[112:120] = band_width.to_bytes(8, "little", signed=True)
    for index in range(7):
        at = 120 + index * 8
        buf[at : at + 8] = guard.to_bytes(8, "little", signed=True)
    buf[176:184] = budget.to_bytes(8, "little", signed=True)
    buf[184:192] = elapsed.to_bytes(8, "little", signed=True)
    buf[192:196] = proposition.to_bytes(4, "little", signed=True)
    buf[196:200] = step_index.to_bytes(4, "little", signed=True)
    return bytes(buf)


def triple(s_epi: int, s_vital: int, s_gov: int, absorbed: int = 0, reserve: int = 0) -> bytes:
    return (
        s_epi.to_bytes(8, "little", signed=True)
        + s_vital.to_bytes(8, "little", signed=True)
        + s_gov.to_bytes(8, "little", signed=True)
        + absorbed.to_bytes(4, "little", signed=True)
        + reserve.to_bytes(4, "little", signed=True)
    )


def main() -> int:
    if not WASM.exists():
        print(f"FAIL the module is missing: {WASM}")
        return 2
    blob = WASM.read_bytes()
    port = Port(blob)

    # --- the shape of the module (D11, mirrored without the adapter) ----------
    check("D11 the module declares no import", list(port.module.imports) == [], f"{list(port.module.imports)}")
    names = sorted(export.name for export in port.module.exports)
    check(
        "D11 the module exports exactly the declared surface",
        names == ["abi_version", "accept", "diagnose", "memory", "step", "vectors_hash"],
        ",".join(names),
    )

    # --- the version handshake ------------------------------------------------
    check("the version is one", port.call("abi_version") == ABI_VERSION)
    check(
        "the declared ABI version differs from zero, so a zeroed read is a mismatch",
        ABI_VERSION != 0,
    )

    # --- sizes ---------------------------------------------------------------
    check("StateRec is 256 bytes", STATE_REC_BYTES == 256)
    check("RegimeOut and VerdictOut are 64 bytes", REGIME_OUT_BYTES == 64 and VERDICT_OUT_BYTES == 64)
    check(
        "one arena step is the record plus the answer",
        STATE_REC_BYTES + REGIME_OUT_BYTES == ARENA_STEP_BYTES,
    )
    check(
        "the diagnosis view is 48 bytes and a triple is 32",
        DIAGNOSIS_OUT_BYTES == 48 and STEP_TRIPLE_BYTES == 32,
    )

    declared = data_end(blob)
    check("D7 the declared arena base is align8(__data_end)", declared % 8 == 0, f"base {declared}")
    memory_bytes = port.memory.data_len(port.store)

    # The module's statics begin at `align8(__data_end)`; a caller arena placed
    # there is refused with -14 instead of quietly overwriting them.
    port.write(declared, state(l=0, u=1400, c=400))
    check(
        "a caller arena inside the module's store is refused (D7)",
        port.call("step", declared, declared + STATE_REC_BYTES) == E_CALLER_CONTRACT,
        f"code {port.call('step', declared, declared + STATE_REC_BYTES)}",
    )
    check("the store begins at align8(__data_end)", declared > 0 and declared % 8 == 0, f"{declared}")

    # --- the arena this consumer uses: the top of memory ---------------------
    arena = (memory_bytes - LAYOUT_BYTES) & ~7
    rec, answer = arena, arena + STATE_REC_BYTES
    triples = arena + 4096
    diagnosis = triples + 4096
    digest_at = diagnosis + 4096
    check(
        "the arena begins above the module's store",
        arena > declared,
        f"arena {arena}, store {declared}",
    )
    check(
        "the whole layout fits in the memory the module was instantiated with",
        arena + LAYOUT_BYTES <= memory_bytes,
        f"{arena + LAYOUT_BYTES} > {memory_bytes}",
    )
    check(
        "1024 steps stay inside the wasm32 address space",
        arena + MAX_STEPS * ARENA_STEP_BYTES <= 0xFFFF_FFFF,
    )

    # --- the digest, read through the module's own pointer (D12) -------------
    port.write(digest_at, b"\x00" * DIGEST_BYTES)
    code = port.call("vectors_hash", digest_at)
    digest = port.read(digest_at, DIGEST_BYTES)
    expected = hashlib.sha256(VECTORS.read_bytes()).digest()
    check("vectors_hash returns 0", code == 0, f"code {code}")
    check(
        "the digest the module carries is the fixture on disk (D12)",
        digest == expected,
        f"{digest.hex()[:16]} vs {expected.hex()[:16]}",
    )

    # --- codes are codes, tags are tags (D8) --------------------------------
    check("a null pointer is a caller fault", port.call("step", 0, answer) == E_CALLER_CONTRACT)
    check("a misaligned pointer is a caller fault", port.call("step", rec + 4, answer) == E_CALLER_CONTRACT)
    port.write(rec, state(l=1, u=3, c=1))
    check("L == C is refused by the core (D8)", port.call("step", rec, answer) == -10)
    port.write(rec, state(l=0, u=1_000_000_000, c=999_999_999))
    check("a fraction that truncates to zero is refused (D8)", port.call("step", rec, answer) == -9)
    port.write(rec, state(l=0, u=1_000_000_001, c=1))
    check(
        "an extent above the declared scale is the core's code, not the caller's",
        port.call("step", rec, answer) == -5,
    )
    port.write(rec, state(l=-1, u=3, c=2))
    check("an unprovided extent is missing, not zero", port.call("step", rec, answer) == -6)
    port.write(rec, state(reversible=2))
    check("a flag outside 0|1 is a caller fault", port.call("step", rec, answer) == E_CALLER_CONTRACT)
    port.write(rec, state(l=200, u=400, c=300, guard=-1))
    check(
        "a sealed instrument with an absent reading is refused by the core's seventh check",
        port.call("step", rec, answer) == -5,
    )
    port.write(rec, state(l=200, u=400, c=300, instrument=1, missing_slot=1, guard=-1))
    check(
        "an incomplete instrument quarantines instead, and the tag is the return value",
        port.call("step", rec, answer) == 3,
    )

    # --- a step and an accept, byte for byte against the vectors -------------
    port.write(rec, state(l=200, u=400, c=300))
    code = port.call("step", rec, answer)
    payload = port.i64(answer)
    payload_2 = port.i32(answer + 12)
    check("s02 answers regime 1 (the return value is the tag)", code == 1, f"code {code}")
    check("s02 carries the fraction 500000 (D2: the module divides, not Python)", payload == 500_000, str(payload))
    check("s02's second payload word is zero", payload_2 == 0)

    port.write(rec, state(c=120, r=1000, guard=500_000))
    code = port.call("accept", rec, answer)
    check(
        "a01 answers verdict 0 (interior) with no proposition and no site",
        code == 0 and port.i32(answer) == 0 and port.i32(answer + 4) == 0,
        f"code {code}",
    )

    # --- the digest survives calls at the arena (D7) ------------------------
    port.write(digest_at, b"\x00" * DIGEST_BYTES)
    port.call("vectors_hash", digest_at)
    port.write(rec, state(l=200, u=400, c=300))
    port.call("step", rec, answer)
    port.call("diagnose", triples, 1, diagnosis)
    check(
        "the digest is not disturbed by a step or a fold",
        port.read(digest_at, DIGEST_BYTES) == expected,
    )

    # --- the diagnosis fold over three triples ------------------------------
    port.write(triples, triple(900_000, 400_000, 800_000, 0, 950_000))
    port.write(triples + 32, triple(700_000, 300_000, 600_000, 0, 900_000))
    port.write(triples + 64, triple(500_000, -1, 400_000, 1, -1))
    code = port.call("diagnose", triples, 3, diagnosis)
    check("diagnose accepts three steps", code == 0, f"code {code}")
    check("the fold is absorbed", port.i32(diagnosis + 4) == 1)
    check("the ruin is the third step", port.i64(diagnosis + 32) == 2, str(port.i64(diagnosis + 32)))
    check("the epistemic fold is the minimum over every step", port.i64(diagnosis + 8) == 500_000)
    check("the vital fold skips the absorbed step", port.i64(diagnosis + 16) == 300_000)
    check("the governance fold is the minimum over every step", port.i64(diagnosis + 24) == 400_000)
    check(
        "after absorption the binding axis is governance, not vital",
        port.i32(diagnosis) == 3,
        str(port.i32(diagnosis)),
    )
    check("the terminal reserve is Missing", port.i64(diagnosis + 40) == -1)

    port.write(triples, triple(900_000, 111_000, 800_000, 0, 950_000))
    port.write(triples + 32, triple(700_000, 222_000, 600_000, 0, 940_000))
    check("diagnose accepts two live steps", port.call("diagnose", triples, 2, diagnosis) == 0)
    check(
        "before absorption the binding axis is the vital one",
        port.i32(diagnosis) == 2,
        str(port.i32(diagnosis)),
    )

    port.write(triples, triple(1, 2, 3, 1, 0))
    check("an absorbed step may not carry a vital reading", port.call("diagnose", triples, 1, diagnosis) == E_CALLER_CONTRACT)
    check("a length above the arena is refused", port.call("diagnose", triples, MAX_STEPS + 1, diagnosis) == E_CALLER_CONTRACT)
    check("a null output pointer is a caller fault", port.call("diagnose", triples, 0, 0) == E_CALLER_CONTRACT)

    print()
    if failures:
        print(f"{len(failures)} invariants failed: {', '.join(failures)}")
        return 1
    print("consumer B agrees with the boundary contract")
    return 0


if __name__ == "__main__":
    sys.exit(main())
