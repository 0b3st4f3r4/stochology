//! Independent Wasmtime runtime parity witness over `stoch_abi.wasm`.
//!
//! Exercises the compiled WebAssembly module through Wasmtime's linear memory
//! and typed exports without sharing code with the TypeScript adapter, verifying
//! module shape, store overlap refusal, `vectors_hash`, boundary codes/tags,
//! `ESC-02`/`ESC-03` wire semantics, and the diagnostic fold invariants.

#![deny(unsafe_code)]
#![deny(clippy::float_arithmetic)]
#![forbid(clippy::cast_possible_truncation)]

use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use wasmtime::{Engine, Instance, Memory, Module, Store};

const ABI_VERSION: i64 = 1;
const STATE_REC_BYTES: usize = 256;
const REGIME_OUT_BYTES: usize = 64;
const VERDICT_OUT_BYTES: usize = 64;
const STEP_TRIPLE_BYTES: usize = 40;
const DIAGNOSIS_OUT_BYTES: usize = 56;
const ARENA_STEP_BYTES: usize = STATE_REC_BYTES + REGIME_OUT_BYTES;
const MAX_STEPS: usize = 1024;
const DIGEST_BYTES: usize = 32;
const E_CALLER_CONTRACT: i32 = -14;
const LAYOUT_BYTES: usize = ARENA_STEP_BYTES + 3 * 4096;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("..")
        .canonicalize()
        .expect("stoch-aot lives at crates/host-workspace/stoch-aot")
}

fn load_wasm_bytes() -> Vec<u8> {
    let root = repo_root();
    let wasm_path = env::var_os("STOCH_WASM_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|| root.join("target/wasm32-unknown-unknown/release/stoch_abi.wasm"));
    if !wasm_path.is_file() {
        let mut cmd = Command::new("cargo");
        cmd.args([
            "build",
            "--release",
            "--offline",
            "--locked",
            "-p",
            "stoch-abi",
            "--target",
            "wasm32-unknown-unknown",
        ])
        .current_dir(&root);
        let status = cmd.status().expect("run cargo build for stoch-abi");
        assert!(status.success(), "building stoch-abi wasm failed");
    }
    fs::read(&wasm_path).unwrap_or_else(|e| panic!("reading {}: {e}", wasm_path.display()))
}

const fn align8(value: usize) -> usize {
    (value + 7) & !7
}

fn uleb(blob: &[u8], cursor: &mut usize) -> usize {
    let mut result = 0usize;
    let mut shift = 0u32;
    loop {
        let byte = blob[*cursor];
        *cursor += 1;
        result |= usize::from(byte & 0x7F) << shift;
        if (byte & 0x80) == 0 {
            return result;
        }
        shift += 7;
    }
}

fn sleb(blob: &[u8], cursor: &mut usize) -> isize {
    let mut result = 0isize;
    let mut shift = 0u32;
    loop {
        let byte = blob[*cursor];
        *cursor += 1;
        result |= isize::from(byte & 0x7F) << shift;
        shift += 7;
        if (byte & 0x80) == 0 {
            if (byte & 0x40) != 0 && shift < usize::BITS {
                result |= -(1isize << shift);
            }
            return result;
        }
    }
}

fn data_end(blob: &[u8]) -> usize {
    assert_eq!(&blob[0..4], b"\0asm");
    assert_eq!(&blob[4..8], &[1, 0, 0, 0]);
    let mut offset = 8usize;
    let mut end = 0usize;
    while offset < blob.len() {
        let section_id = blob[offset];
        offset += 1;
        let size = uleb(blob, &mut offset);
        let body = offset;
        if section_id == 11 {
            let mut cursor = body;
            let count = uleb(blob, &mut cursor);
            for _ in 0..count {
                let flags = uleb(blob, &mut cursor);
                assert_eq!(flags, 0, "unexpected passive/memory-indexed data segment");
                assert_eq!(blob[cursor], 0x41, "expected i32.const offset opcode");
                cursor += 1;
                let base = usize::try_from(sleb(blob, &mut cursor)).expect("non-negative data base");
                assert_eq!(blob[cursor], 0x0B, "expected end opcode");
                cursor += 1;
                let length = uleb(blob, &mut cursor);
                end = end.max(base + length);
                cursor += length;
            }
        }
        offset = body + size;
    }
    align8(end)
}

#[derive(Clone, Copy)]
struct StateArgs {
    instrument: i32,
    missing_slot: i32,
    l: i64,
    u: i64,
    c: i64,
    b: i64,
    r: i64,
    tts: i64,
    ttr_eff: i64,
    n_eff: i64,
    defect: i64,
    reversible: i32,
    gov_corrupt: i32,
    vital_floor: i32,
    gov_floor: i32,
    tape: i32,
    site: i32,
    accept_threshold: i64,
    band_width: i64,
    guard: i64,
    budget: i64,
    elapsed: i64,
    proposition: i32,
    step_index: i32,
    lethal_omission: i32,
    reopen_sealed: i32,
}

impl Default for StateArgs {
    fn default() -> Self {
        Self {
            instrument: 0,
            missing_slot: 0,
            l: 0,
            u: 0,
            c: 0,
            b: 0,
            r: 0,
            tts: 0,
            ttr_eff: 0,
            n_eff: 1000,
            defect: 0,
            reversible: 1,
            gov_corrupt: 0,
            vital_floor: 0,
            gov_floor: 0,
            tape: 1,
            site: 0,
            accept_threshold: 100_000,
            band_width: 50_000,
            guard: 900_000,
            budget: 1000,
            elapsed: 10,
            proposition: 0,
            step_index: -1,
            lethal_omission: 0,
            reopen_sealed: 0,
        }
    }
}

fn encode_state(args: StateArgs) -> [u8; STATE_REC_BYTES] {
    let mut buf = [0u8; STATE_REC_BYTES];
    buf[0..4].copy_from_slice(&args.instrument.to_le_bytes());
    buf[4..8].copy_from_slice(&args.missing_slot.to_le_bytes());
    for (offset, val) in [
        (8, args.l),
        (16, args.u),
        (24, args.c),
        (32, args.b),
        (40, args.r),
        (48, args.tts),
        (56, args.ttr_eff),
        (64, args.n_eff),
        (72, args.defect),
    ] {
        buf[offset..offset + 8].copy_from_slice(&val.to_le_bytes());
    }
    for (offset, val) in [
        (80, args.reversible),
        (84, args.gov_corrupt),
        (88, args.vital_floor),
        (92, args.gov_floor),
        (96, args.tape),
        (100, args.site),
    ] {
        buf[offset..offset + 4].copy_from_slice(&val.to_le_bytes());
    }
    buf[104..112].copy_from_slice(&args.accept_threshold.to_le_bytes());
    buf[112..120].copy_from_slice(&args.band_width.to_le_bytes());
    for index in 0..7 {
        let at = 120 + index * 8;
        buf[at..at + 8].copy_from_slice(&args.guard.to_le_bytes());
    }
    buf[176..184].copy_from_slice(&args.budget.to_le_bytes());
    buf[184..192].copy_from_slice(&args.elapsed.to_le_bytes());
    buf[192..196].copy_from_slice(&args.proposition.to_le_bytes());
    buf[196..200].copy_from_slice(&args.step_index.to_le_bytes());
    buf[200..204].copy_from_slice(&args.lethal_omission.to_le_bytes());
    buf[204..208].copy_from_slice(&args.reopen_sealed.to_le_bytes());
    buf
}

fn encode_triple(
    s_epi: i64,
    s_vital: i64,
    s_gov: i64,
    absorbed: i32,
    reserve: i32,
    theta_basin: i64,
) -> [u8; STEP_TRIPLE_BYTES] {
    let mut buf = [0u8; STEP_TRIPLE_BYTES];
    buf[0..8].copy_from_slice(&s_epi.to_le_bytes());
    buf[8..16].copy_from_slice(&s_vital.to_le_bytes());
    buf[16..24].copy_from_slice(&s_gov.to_le_bytes());
    buf[24..32].copy_from_slice(&theta_basin.to_le_bytes());
    buf[32..36].copy_from_slice(&absorbed.to_le_bytes());
    buf[36..40].copy_from_slice(&reserve.to_le_bytes());
    buf
}

struct Port {
    store: Store<()>,
    instance: Instance,
    memory: Memory,
}

impl Port {
    fn new(bytes: &[u8]) -> Self {
        let engine = Engine::default();
        let module = Module::new(&engine, bytes).expect("compile wasm module");
        assert_eq!(module.imports().len(), 0, "module must declare zero imports");
        let export_names: BTreeSet<String> = module
            .exports()
            .map(|e| e.name().to_string())
            .collect();
        let expected: BTreeSet<String> = [
            "abi_version",
            "accept",
            "diagnose",
            "memory",
            "step",
            "vectors_hash",
        ]
        .into_iter()
        .map(str::to_string)
        .collect();
        assert_eq!(export_names, expected, "unexpected wasm export surface");

        let mut store = Store::new(&engine, ());
        let instance = Instance::new(&mut store, &module, &[]).expect("instantiate module");
        let memory = instance
            .get_memory(&mut store, "memory")
            .expect("memory export");
        Self {
            store,
            instance,
            memory,
        }
    }

    fn abi_version(&mut self) -> i64 {
        let f = self
            .instance
            .get_typed_func::<(), i64>(&mut self.store, "abi_version")
            .expect("abi_version export");
        f.call(&mut self.store, ()).expect("call abi_version")
    }

    fn step(&mut self, rec_ptr: i32, out_ptr: i32) -> i32 {
        let f = self
            .instance
            .get_typed_func::<(i32, i32), i32>(&mut self.store, "step")
            .expect("step export");
        f.call(&mut self.store, (rec_ptr, out_ptr)).expect("call step")
    }

    fn accept(&mut self, rec_ptr: i32, out_ptr: i32) -> i32 {
        let f = self
            .instance
            .get_typed_func::<(i32, i32), i32>(&mut self.store, "accept")
            .expect("accept export");
        f.call(&mut self.store, (rec_ptr, out_ptr))
            .expect("call accept")
    }

    fn diagnose(&mut self, triples_ptr: i32, len: i32, out_ptr: i32) -> i32 {
        let f = self
            .instance
            .get_typed_func::<(i32, i32, i32), i32>(&mut self.store, "diagnose")
            .expect("diagnose export");
        f.call(&mut self.store, (triples_ptr, len, out_ptr))
            .expect("call diagnose")
    }

    fn vectors_hash(&mut self, out_ptr: i32) -> i32 {
        let f = self
            .instance
            .get_typed_func::<i32, i32>(&mut self.store, "vectors_hash")
            .expect("vectors_hash export");
        f.call(&mut self.store, out_ptr)
            .expect("call vectors_hash")
    }

    fn write(&mut self, addr: usize, payload: &[u8]) {
        self.memory
            .write(&mut self.store, addr, payload)
            .expect("memory write");
    }

    fn read_bytes(&self, addr: usize, len: usize) -> Vec<u8> {
        let mut buf = vec![0u8; len];
        self.memory
            .read(&self.store, addr, &mut buf)
            .expect("memory read");
        buf
    }

    fn read_i32(&self, addr: usize) -> i32 {
        let bytes = self.read_bytes(addr, 4);
        i32::from_le_bytes(bytes.try_into().expect("4 bytes"))
    }

    fn read_i64(&self, addr: usize) -> i64 {
        let bytes = self.read_bytes(addr, 8);
        i64::from_le_bytes(bytes.try_into().expect("8 bytes"))
    }
}

#[test]
fn wasmtime_parity_matches_core_and_abi_contract() {
    assert_eq!(STATE_REC_BYTES + REGIME_OUT_BYTES, ARENA_STEP_BYTES);
    assert_eq!(VERDICT_OUT_BYTES, 64);
    assert_eq!(DIAGNOSIS_OUT_BYTES, 56);

    let blob = load_wasm_bytes();
    let declared = data_end(&blob);
    assert!(declared > 0 && declared % 8 == 0);

    let mut port = Port::new(&blob);
    assert_eq!(port.abi_version(), ABI_VERSION);

    let memory_bytes = port.memory.data_size(&port.store);
    let declared_i32 = i32::try_from(declared).expect("declared fits i32");
    let rec_len_i32 = i32::try_from(STATE_REC_BYTES).expect("rec size fits i32");

    // Store overlap at align8(__data_end) must be refused with E_CALLER_CONTRACT.
    port.write(
        declared,
        &encode_state(StateArgs {
            l: 0,
            u: 1400,
            c: 400,
            ..Default::default()
        }),
    );
    assert_eq!(
        port.step(declared_i32, declared_i32 + rec_len_i32),
        E_CALLER_CONTRACT
    );

    // Caller arena sits safely at top of instantiated linear memory.
    let arena = (memory_bytes - LAYOUT_BYTES) & !7;
    assert!(arena > declared);
    assert!(arena + LAYOUT_BYTES <= memory_bytes);

    let rec = arena;
    let answer = arena + STATE_REC_BYTES;
    let triples = arena + 4096;
    let diagnosis = triples + 4096;
    let digest_at = diagnosis + 4096;

    let rec_i32 = i32::try_from(rec).expect("rec fits i32");
    let answer_i32 = i32::try_from(answer).expect("answer fits i32");
    let triples_i32 = i32::try_from(triples).expect("triples fits i32");
    let diagnosis_i32 = i32::try_from(diagnosis).expect("diagnosis fits i32");
    let digest_i32 = i32::try_from(digest_at).expect("digest fits i32");

    // Digest parity against stoch-core::vectors_hash().
    port.write(digest_at, &[0u8; DIGEST_BYTES]);
    assert_eq!(port.vectors_hash(digest_i32), 0);
    let expected_digest = stoch_core::vectors_hash();
    assert_eq!(port.read_bytes(digest_at, DIGEST_BYTES), expected_digest);

    // Pointer, scale, and tag vs error code invariants.
    assert_eq!(port.step(0, answer_i32), E_CALLER_CONTRACT);
    assert_eq!(port.step(rec_i32 + 4, answer_i32), E_CALLER_CONTRACT);

    port.write(rec, &encode_state(StateArgs { l: 1, u: 3, c: 1, ..Default::default() }));
    assert_eq!(port.step(rec_i32, answer_i32), 0);

    port.write(rec, &encode_state(StateArgs { l: 1, u: 3, c: 3, ..Default::default() }));
    assert_eq!(port.step(rec_i32, answer_i32), 5);

    port.write(rec, &encode_state(StateArgs { l: 3, u: 3, c: 3, ..Default::default() }));
    assert_eq!(port.step(rec_i32, answer_i32), 5);

    port.write(
        rec,
        &encode_state(StateArgs {
            l: 0,
            u: 1_000_000_000,
            c: 999_999_999,
            ..Default::default()
        }),
    );
    assert_eq!(port.step(rec_i32, answer_i32), -9);

    port.write(
        rec,
        &encode_state(StateArgs {
            l: 0,
            u: 1_000_000_001,
            c: 1,
            ..Default::default()
        }),
    );
    assert_eq!(port.step(rec_i32, answer_i32), -5);

    port.write(rec, &encode_state(StateArgs { l: -1, u: 3, c: 2, ..Default::default() }));
    assert_eq!(port.step(rec_i32, answer_i32), -6);

    port.write(rec, &encode_state(StateArgs { reversible: 2, ..Default::default() }));
    assert_eq!(port.step(rec_i32, answer_i32), E_CALLER_CONTRACT);

    port.write(
        rec,
        &encode_state(StateArgs {
            l: 200,
            u: 400,
            c: 300,
            guard: -1,
            ..Default::default()
        }),
    );
    assert_eq!(port.step(rec_i32, answer_i32), -5);

    port.write(
        rec,
        &encode_state(StateArgs {
            l: 200,
            u: 400,
            c: 300,
            instrument: 1,
            missing_slot: 1,
            guard: -1,
            ..Default::default()
        }),
    );
    assert_eq!(port.step(rec_i32, answer_i32), 3);

    port.write(
        rec,
        &encode_state(StateArgs {
            l: 200,
            u: 400,
            c: 300,
            instrument: 1,
            missing_slot: 1,
            tts: 5,
            ttr_eff: 10,
            gov_corrupt: 1,
            guard: -1,
            ..Default::default()
        }),
    );
    assert_eq!(port.step(rec_i32, answer_i32), 4);
    assert_eq!(port.read_i32(answer + 8), 3);

    port.write(
        rec,
        &encode_state(StateArgs {
            l: 200,
            u: 400,
            c: 300,
            reversible: 0,
            lethal_omission: 1,
            ..Default::default()
        }),
    );
    assert_eq!(port.step(rec_i32, answer_i32), 4);
    assert_eq!(port.read_i32(answer + 8), 4);

    port.write(
        rec,
        &encode_state(StateArgs {
            c: 120,
            r: 1000,
            guard: 500_000,
            reopen_sealed: 1,
            ..Default::default()
        }),
    );
    assert_eq!(port.accept(rec_i32, answer_i32), 3);
    assert_eq!(port.read_i32(answer + 24), 5);

    // Byte-for-byte s02 and a01 checks.
    port.write(rec, &encode_state(StateArgs { l: 200, u: 400, c: 300, ..Default::default() }));
    assert_eq!(port.step(rec_i32, answer_i32), 1);
    assert_eq!(port.read_i64(answer), 500_000);
    assert_eq!(port.read_i32(answer + 12), 0);

    port.write(
        rec,
        &encode_state(StateArgs {
            c: 120,
            r: 1000,
            guard: 500_000,
            ..Default::default()
        }),
    );
    assert_eq!(port.accept(rec_i32, answer_i32), 0);
    assert_eq!(port.read_i32(answer), 0);
    assert_eq!(port.read_i32(answer + 4), 0);

    // Digest survives calls at the top-of-memory arena.
    assert_eq!(port.read_bytes(digest_at, DIGEST_BYTES), expected_digest);

    // Diagnosis fold with absorption + co-diagnostic theta_basin.
    port.write(triples, &encode_triple(900_000, 400_000, 800_000, 0, 950_000, 250_000));
    port.write(
        triples + STEP_TRIPLE_BYTES,
        &encode_triple(700_000, 300_000, 600_000, 0, 900_000, 120_000),
    );
    port.write(
        triples + 2 * STEP_TRIPLE_BYTES,
        &encode_triple(500_000, -1, 400_000, 1, -1, 180_000),
    );
    assert_eq!(port.diagnose(triples_i32, 3, diagnosis_i32), 0);
    assert_eq!(port.read_i32(diagnosis), 3);
    assert_eq!(port.read_i32(diagnosis + 4), 1);
    assert_eq!(port.read_i64(diagnosis + 8), 500_000);
    assert_eq!(port.read_i64(diagnosis + 16), 300_000);
    assert_eq!(port.read_i64(diagnosis + 24), 400_000);
    assert_eq!(port.read_i64(diagnosis + 32), 2);
    assert_eq!(port.read_i64(diagnosis + 40), -1);
    assert_eq!(port.read_i64(diagnosis + 48), 120_000);

    // Two live steps: theta_basin never alters binding_axis.
    port.write(triples, &encode_triple(900_000, 111_000, 800_000, 0, 950_000, 50_000));
    port.write(
        triples + STEP_TRIPLE_BYTES,
        &encode_triple(700_000, 222_000, 600_000, 0, 940_000, 90_000),
    );
    assert_eq!(port.diagnose(triples_i32, 2, diagnosis_i32), 0);
    assert_eq!(port.read_i32(diagnosis), 2);
    assert_eq!(port.read_i64(diagnosis + 48), 50_000);

    // Contract refusals on diagnose.
    port.write(triples, &encode_triple(1, 2, 3, 1, 0, -1));
    assert_eq!(port.diagnose(triples_i32, 1, diagnosis_i32), E_CALLER_CONTRACT);
    let too_many = i32::try_from(MAX_STEPS + 1).expect("MAX_STEPS + 1 fits i32");
    assert_eq!(port.diagnose(triples_i32, too_many, diagnosis_i32), E_CALLER_CONTRACT);
    assert_eq!(port.diagnose(triples_i32, 0, 0), E_CALLER_CONTRACT);
}
