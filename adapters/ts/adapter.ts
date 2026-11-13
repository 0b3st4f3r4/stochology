// The TypeScript adapter of the archived design document (`02-arquitetura.md` §8.2).
//
// It is a *duplex*: it embeds the module, performs the version handshake, lays the
// caller's arena at the top of memory, and decodes the five exports
// into named results. It re-derives nothing the module already decides — except the
// six-row `ActionSet` table, which by `02` §8.2 (D13) does not cross the boundary and
// must be derived on both sides of it.
//
// Every 64-bit value crosses this file as a `bigint`. There is exactly one declared
// conversion, `toSafeNumber`, and it refuses anything a `number` cannot hold exactly.

export const ABI_VERSION: bigint = 1n;
export const STATE_REC_BYTES = 256;
export const REGIME_OUT_BYTES = 64;
export const VERDICT_OUT_BYTES = 64;
export const STEP_TRIPLE_BYTES = 32;
export const DIAGNOSIS_OUT_BYTES = 48;
export const ARENA_STEP_BYTES = 320;

/** The fixed header of one arena: state, the two output records, the fold, the digest. */
export const ARENA_HEADER_BYTES = 464;
const REGIME_OUT_AT = STATE_REC_BYTES;
const VERDICT_OUT_AT = REGIME_OUT_AT + REGIME_OUT_BYTES;
const DIAGNOSIS_AT = VERDICT_OUT_AT + VERDICT_OUT_BYTES;
const DIGEST_AT = DIAGNOSIS_AT + DIAGNOSIS_OUT_BYTES;
const TRIPLES_AT = ARENA_HEADER_BYTES;
export const MAX_STEPS = 1024;

/** Named refusals of the adapter. A code, not a stack trace, is what a caller tests. */
export type AbiCode =
  | "E_ABI_VERSION"
  | "E_IMPORTS_PRESENT"
  | "E_MISSING_EXPORT"
  | "E_ARENA_BASE"
  | "E_ARENA_OVERFLOW"
  | "E_BIGINT_REQUIRED"
  | "E_BIGINT_PRECISION"
  | "E_UNKNOWN_TAG"
  | "E_UNKNOWN_PAYLOAD"
  | "E_CORE";

export class AbiError extends Error {
  readonly abiCode: AbiCode;
  /** The core's numeric code when `abiCode` is `E_CORE`, otherwise `null`. */
  readonly coreCode: number | null;

  constructor(abiCode: AbiCode, message: string, coreCode: number | null = null) {
    super(`${abiCode}: ${message}`);
    this.name = "AbiError";
    this.abiCode = abiCode;
    this.coreCode = coreCode;
  }
}

// ---------------------------------------------------------------------------
// 64-bit strictness (D1). No `bigint` becomes a `number` except through the one
// declared conversion, and a `number` never becomes an `i64` at all.
// ---------------------------------------------------------------------------

/** The declared `bigint` → `number` conversion, and its whole domain. */
export function toSafeNumber(value: bigint): number {
  const limit = 9007199254740991n; // Number.MAX_SAFE_INTEGER
  if (value > limit || value < -limit) {
    throw new AbiError(
      "E_BIGINT_PRECISION",
      `${value} does not fit a number exactly; keep it a bigint`,
    );
  }
  return Number(value);
}

function i64(value: bigint | number, field: string): bigint {
  if (typeof value !== "bigint") {
    throw new AbiError(
      "E_BIGINT_REQUIRED",
      `${field} must be a bigint; a number cannot carry an i64 exactly`,
    );
  }
  return value;
}

// ---------------------------------------------------------------------------
// The arena (D7). The base is `align8(__data_end)`, read from the module's own
// globals section — the stack grows down from that address, so the bytes above it
// are the only region the module will never write behind the caller's back.
// ---------------------------------------------------------------------------

/**
 * `align8(__data_end)` of a module, from its data section.
 *
 * The module places its initialised data at the top of the memory image, with the
 * stack growing down from the same address; `__data_end` is therefore the end of the
 * highest data segment, and everything above it is untouched by the module. A base
 * computed from the stack-pointer global instead would start *inside* the digest the
 * module carries, and the first write would corrupt the fixture hash.
 */
export function readDataEnd(bytes: Uint8Array): number {
  let i = 8;
  let end = 0;
  const leb = (): number => {
    let result = 0;
    let shift = 0;
    for (;;) {
      const byte = bytes[i++];
      result |= (byte & 0x7f) << shift;
      shift += 7;
      if ((byte & 0x80) === 0) return result >>> 0;
    }
  };
  while (i < bytes.length) {
    const id = bytes[i++];
    const size = leb();
    const sectionEnd = i + size;
    if (id === 11) {
      const count = leb();
      for (let k = 0; k < count; k += 1) {
        const flag = leb();
        let offset = 0;
        if (flag === 0) {
          const opcode = bytes[i++];
          if (opcode !== 0x41) {
            throw new AbiError("E_ARENA_BASE", `data segment ${k} has no i32 offset`);
          }
          offset = leb();
          i += 1; // the `end` opcode of the offset expression
        } else if (flag === 2) {
          throw new AbiError("E_ARENA_BASE", `data segment ${k} is memory-indexed`);
        }
        const length = leb();
        const segmentEnd = offset + length;
        if (segmentEnd > end) end = segmentEnd;
        i += length;
      }
    }
    i = sectionEnd;
  }
  return (end + 7) & ~7;
}

/**
 * Refuse a base below `floor` (D7). The floor is the image the module was
 * instantiated with: the module's statics live inside it, and `memory.grow`
 * only appends pages above it, so a base at or above the floor cannot overlap
 * the store — and this needs no measurement of the module's layout.
 */
export function assertBase(base: number, floor: number): number {
  if (base < floor) {
    throw new AbiError("E_ARENA_BASE", `base ${base} is below the floor ${floor}`);
  }
  if (base % 8 !== 0) {
    throw new AbiError("E_ARENA_BASE", `base ${base} is not 8-aligned`);
  }
  return base;
}

/** The bytes one arena occupies: the fixed header plus one stride per record. */
export function arenaBytes(steps: number): number {
  if (!Number.isInteger(steps) || steps < 0 || steps > MAX_STEPS) {
    throw new AbiError("E_ARENA_OVERFLOW", `${steps} steps is outside 0..=${MAX_STEPS}`);
  }
  return ARENA_HEADER_BYTES + steps * ARENA_STEP_BYTES;
}

/**
 * The arena at the top of memory: `memoryBytes - arenaBytes(steps)`, rounded
 * down to 8. The module has no allocator and no imports, so nothing above its
 * statics is ever in use, and the top is the only placement that does not
 * depend on measuring where those statics end. Re-derived on every call, so a
 * grown memory moves the base with it.
 */
export function arenaBase(memoryBytes: number, steps: number): number {
  const bytes = arenaBytes(steps);
  const base = memoryBytes - bytes - ((memoryBytes - bytes) % 8);
  if (base <= 0) {
    throw new AbiError(
      "E_ARENA_OVERFLOW",
      `an arena of ${bytes} bytes does not fit in ${memoryBytes} bytes`,
    );
  }
  return base;
}

// ---------------------------------------------------------------------------
// The record I write and the records I read.
// ---------------------------------------------------------------------------

export interface StateRecord {
  instrument: number;
  missingSlot: number;
  l: bigint;
  u: bigint;
  c: bigint;
  b: bigint;
  r: bigint;
  tts: bigint;
  ttrEff: bigint;
  nEff: bigint;
  defect: bigint;
  reversible: 0 | 1;
  govCorrupt: 0 | 1;
  vitalFloor: 0 | 1;
  govFloor: 0 | 1;
  tape: 0 | 1;
  site: number;
  acceptThreshold: bigint;
  bandWidth: bigint;
  guard: bigint[];
  budget: bigint;
  elapsed: bigint;
  proposition: number;
  stepIndex: number;
}

export interface StepTriple {
  sEpi: bigint;
  sVital: bigint;
  sGov: bigint;
  absorbed: 0 | 1;
  reserve: number;
}

export interface Regime {
  tag: number;
  name: string;
  fraction: bigint | null;
  cause: number | null;
  constraint: number | null;
  missing: number | null;
}

export interface Verdict {
  tag: number;
  name: string;
  proposition: number;
  site: number;
  margin: bigint;
  atStep: bigint;
}

export interface DiagnosisOut {
  bindingAxis: number;
  absorbed: number;
  sEpi: bigint;
  sVital: bigint;
  sGov: bigint;
  ruinStep: bigint;
  terminalReserve: bigint;
}

const REGIME_NAMES = [
  "BufferedAction",
  "ReversibleFractionalProbe",
  "InwardAction",
  "Quarantine",
  "AlgedonicHalt",
  "NoAction",
] as const;

const VERDICT_NAMES = ["Interior", "Boundary", "Collapse", "Unparsable"] as const;

/** The five-site-to-axis derivation is the module's; here only the table of §8.2 D13. */
export function actionSet(tag: number): 0 | 1 | 2 {
  if (!Number.isInteger(tag) || tag < 0 || tag > 5) {
    throw new AbiError("E_UNKNOWN_TAG", `regime tag ${tag} is outside 0..=5`);
  }
  if (tag === 5) return 2; // ActionSet::None
  if (tag === 2 || tag === 3 || tag === 4) return 1; // InwardOnly
  return 0; // External
}

/** Decode a regime out of the tag and its payload, or refuse (D5, D14). */
export function decodeRegime(tag: number, payload0: bigint, payload1: number): Regime {
  if (!Number.isInteger(tag) || tag < 0 || tag > 5) {
    throw new AbiError("E_UNKNOWN_TAG", `regime tag ${tag} is outside 0..=5`);
  }
  const name = REGIME_NAMES[tag];
  if (tag === 1) {
    return {
      tag,
      name,
      fraction: payload0,
      cause: null,
      constraint: null,
      missing: null,
    };
  }
  if (tag === 2) {
    if (payload1 < 0 || payload1 > 1) {
      throw new AbiError("E_UNKNOWN_PAYLOAD", `constraint ${payload1} is outside 0..=1`);
    }
    return { tag, name, fraction: null, cause: null, constraint: payload1, missing: null };
  }
  if (tag === 3) {
    if (payload1 < 1 || payload1 > 7) {
      throw new AbiError("E_UNKNOWN_PAYLOAD", `missing slot ${payload1} is outside 1..=7`);
    }
    return { tag, name, fraction: null, cause: null, constraint: null, missing: payload1 };
  }
  if (tag === 4) {
    if (payload1 < 0 || payload1 > 2) {
      throw new AbiError("E_UNKNOWN_PAYLOAD", `halt cause ${payload1} is outside 0..=2`);
    }
    return { tag, name, fraction: null, cause: payload1, constraint: null, missing: null };
  }
  return { tag, name, fraction: null, cause: null, constraint: null, missing: null };
}

/** Decode a verdict out of the tag and its payload. */
export function decodeVerdict(tag: number, out: VerdictOutWire): Verdict {
  if (!Number.isInteger(tag) || tag < 0 || tag > 3) {
    throw new AbiError("E_UNKNOWN_TAG", `verdict tag ${tag} is outside 0..=3`);
  }
  return {
    tag,
    name: VERDICT_NAMES[tag],
    proposition: out.proposition,
    site: out.site,
    margin: out.margin,
    atStep: out.atStep,
  };
}

export interface VerdictOutWire {
  proposition: number;
  site: number;
  margin: bigint;
  atStep: bigint;
}

// ---------------------------------------------------------------------------
// The live module.
// ---------------------------------------------------------------------------

type Exports = {
  memory: WebAssembly.Memory;
  abi_version: () => bigint;
  vectors_hash: (out: number) => number;
  step: (rec: number, out: number) => number;
  accept: (rec: number, out: number) => number;
  diagnose: (triples: number, len: number, out: number) => number;
};

export interface Core {
  readonly version: bigint;
  /** The module's own `align8(__data_end)`, reported for diagnostics only. */
  readonly dataEnd: number;
  /** The base of the last arena placed; the arena moves with the memory. */
  readonly base: number;
  digest(): Uint8Array;
  digestHex(): string;
  step(state: StateRecord): Regime;
  accept(state: StateRecord): Verdict;
  diagnose(triples: StepTriple[]): DiagnosisOut;
  /** Grows the module's memory so `steps` strides fit, and returns the new page count. */
  reserve(steps: number): number;
  memoryBytes(): number;
  exportNames(): string[];
  close(): void;
}

const REQUIRED_EXPORTS = [
  "memory",
  "abi_version",
  "vectors_hash",
  "step",
  "accept",
  "diagnose",
] as const;

/** Open the module, handshake the version, and pin the arena base. */
export function open(
  bytes: Uint8Array,
  expectedVersion: bigint = ABI_VERSION,
): Core {
  const module = new WebAssembly.Module(bytes);
  const imports = WebAssembly.Module.imports(module);
  if (imports.length !== 0) {
    throw new AbiError(
      "E_IMPORTS_PRESENT",
      `the module declares ${imports.length} import(s): ${imports.map((i) => i.name).join(", ")}`,
    );
  }
  const instance = new WebAssembly.Instance(module, {});
  const exports = instance.exports as unknown as Partial<Exports>;
  for (const name of REQUIRED_EXPORTS) {
    if (exports[name] === undefined) {
      throw new AbiError("E_MISSING_EXPORT", `the module does not export ${name}`);
    }
  }
  const live = exports as Exports;

  const version = live.abi_version();
  if (typeof version !== "bigint") {
    throw new AbiError(
      "E_ABI_VERSION",
      `abi_version must be an i64 and therefore a bigint, got ${typeof version}`,
    );
  }
  if (version !== expectedVersion) {
    throw new AbiError(
      "E_ABI_VERSION",
      `module speaks ABI ${version}, this adapter speaks ${expectedVersion}`,
    );
  }

  const dataEnd = readDataEnd(bytes);
  let view = new DataView(live.memory.buffer);
  // The image the module was instantiated with. Its statics live inside it and
  // growth only appends pages, so the arena must start at or above this floor.
  const floor = live.memory.buffer.byteLength;
  let base = 0;

  const refresh = (): DataView => {
    // D6: a grown memory detaches the old ArrayBuffer; the view must be rebuilt.
    if (live.memory.buffer !== view.buffer) {
      view = new DataView(live.memory.buffer);
    }
    return view;
  };

  /**
   * Place the arena for `steps` records at the top of memory, growing the module
   * until the whole arena sits above `floor`. Returns the base, and moves
   * `core.base` with it.
   */
  const ensure = (steps: number): number => {
    const bytes = arenaBytes(steps);
    const need = floor + bytes;
    const memoryBytes = live.memory.buffer.byteLength;
    if (need > memoryBytes) {
      live.memory.grow(Math.ceil((need - memoryBytes) / 65536));
    }
    refresh();
    base = assertBase(arenaBase(live.memory.buffer.byteLength, steps), floor);
    return base;
  };

  const write = (state: StateRecord): number => {
    const offset = ensure(1);
    view.setInt32(offset + 0, state.instrument, true);
    view.setInt32(offset + 4, state.missingSlot, true);
    view.setBigInt64(offset + 8, i64(state.l, "l"), true);
    view.setBigInt64(offset + 16, i64(state.u, "u"), true);
    view.setBigInt64(offset + 24, i64(state.c, "c"), true);
    view.setBigInt64(offset + 32, i64(state.b, "b"), true);
    view.setBigInt64(offset + 40, i64(state.r, "r"), true);
    view.setBigInt64(offset + 48, i64(state.tts, "tts"), true);
    view.setBigInt64(offset + 56, i64(state.ttrEff, "ttr_eff"), true);
    view.setBigInt64(offset + 64, i64(state.nEff, "n_eff"), true);
    view.setBigInt64(offset + 72, i64(state.defect, "defect"), true);
    view.setInt32(offset + 80, state.reversible, true);
    view.setInt32(offset + 84, state.govCorrupt, true);
    view.setInt32(offset + 88, state.vitalFloor, true);
    view.setInt32(offset + 92, state.govFloor, true);
    view.setInt32(offset + 96, state.tape, true);
    view.setInt32(offset + 100, state.site, true);
    view.setBigInt64(offset + 104, i64(state.acceptThreshold, "accept_threshold"), true);
    view.setBigInt64(offset + 112, i64(state.bandWidth, "band_width"), true);
    if (state.guard.length !== 7) {
      throw new AbiError("E_CORE", `guard has ${state.guard.length} readings, not 7`);
    }
    for (let k = 0; k < 7; k += 1) {
      view.setBigInt64(offset + 120 + k * 8, i64(state.guard[k], `guard[${k}]`), true);
    }
    view.setBigInt64(offset + 176, i64(state.budget, "budget"), true);
    view.setBigInt64(offset + 184, i64(state.elapsed, "elapsed"), true);
    view.setInt32(offset + 192, state.proposition, true);
    view.setInt32(offset + 196, state.stepIndex, true);
    return offset;
  };

  ensure(1);

  const core = (code: number, what: string): never => {
    throw new AbiError("E_CORE", `${what} refused with core code ${code}`, code);
  };

  const api: Core = {
    version,
    dataEnd,
    get base() {
      return base;
    },
    digest(): Uint8Array {
      const at = ensure(1) + DIGEST_AT;
      const status = live.vectors_hash(at);
      if (status !== 0) core(status, "vectors_hash");
      return new Uint8Array(view.buffer, at, 32).slice();
    },
    digestHex(): string {
      return [...api.digest()].map((b) => b.toString(16).padStart(2, "0")).join("");
    },
    step(state: StateRecord): Regime {
      const at = write(state);
      const out = at + REGIME_OUT_AT;
      const tag = live.step(at, out);
      if (tag < 0) core(tag, "step");
      refresh();
      return decodeRegime(tag, view.getBigInt64(out, true), view.getInt32(out + 8, true));
    },
    accept(state: StateRecord): Verdict {
      const at = write(state);
      const out = at + VERDICT_OUT_AT;
      const tag = live.accept(at, out);
      if (tag < 0) core(tag, "accept");
      refresh();
      return decodeVerdict(tag, {
        proposition: view.getInt32(out, true),
        site: view.getInt32(out + 4, true),
        margin: view.getBigInt64(out + 8, true),
        atStep: view.getBigInt64(out + 16, true),
      });
    },
    diagnose(triples: StepTriple[]): DiagnosisOut {
      if (triples.length > MAX_STEPS) {
        throw new AbiError("E_ARENA_OVERFLOW", `${triples.length} steps exceeds ${MAX_STEPS}`);
      }
      const at = ensure(triples.length);
      const out = at + DIAGNOSIS_AT;
      const stride = at + TRIPLES_AT;
      triples.forEach((triple, index) => {
        const offset = stride + index * STEP_TRIPLE_BYTES;
        view.setBigInt64(offset + 0, i64(triple.sEpi, "s_epi"), true);
        view.setBigInt64(offset + 8, i64(triple.sVital, "s_vital"), true);
        view.setBigInt64(offset + 16, i64(triple.sGov, "s_gov"), true);
        view.setInt32(offset + 24, triple.absorbed, true);
        view.setInt32(offset + 28, triple.reserve, true);
      });
      const status = live.diagnose(stride, triples.length, out);
      if (status !== 0) core(status, "diagnose");
      refresh();
      return {
        bindingAxis: view.getInt32(out, true),
        absorbed: view.getInt32(out + 4, true),
        sEpi: view.getBigInt64(out + 8, true),
        sVital: view.getBigInt64(out + 16, true),
        sGov: view.getBigInt64(out + 24, true),
        ruinStep: view.getBigInt64(out + 32, true),
        terminalReserve: view.getBigInt64(out + 40, true),
      };
    },
    reserve(steps: number): number {
      ensure(steps);
      return live.memory.buffer.byteLength / 65536;
    },
    memoryBytes(): number {
      return live.memory.buffer.byteLength;
    },
    exportNames(): string[] {
      return WebAssembly.Module.exports(module).map((entry) => entry.name);
    },
    close(): void {
      base = 0;
    },
  };
  return api;
}
