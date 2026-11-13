//! The caller's arena must not overlap the module's own static store.
//!
//! The buffer the fold borrows lives in the module's static memory, and a caller
//! following the address the document used to publish as a floor would place its
//! arena on that buffer's first byte. The call would then succeed and the caller
//! would read the module's readings as its own records — a silent corruption, which
//! for a reference monitor is worse than any refusal.
//!
//! What is tested here is the refusal and its boundary: an interval that intersects
//! the store's own range, as the module computes that range, is refused with the
//! caller-contract code; the same call with an arena at least `65536` bytes above the
//! floor, on the heap, answers normally.

use stoch_abi::{
    accept_at, diagnose_at, step_at, store_overlaps, store_span, vectors_hash_at, DiagnosisOut,
    RegimeOut, StateRec, StepTriple, VerdictOut, E_CALLER_CONTRACT, MAX_STEPS,
};

/// The bytes the store occupies, as the module computes them.
const STORE_BYTES: usize = 65_536;

// ---------------------------------------------------------------------------
// Records.
// ---------------------------------------------------------------------------

/// A sealed state that takes the buffered branch: `l > c`, no floor, no latency.
fn buffered_record() -> StateRec {
    StateRec {
        instrument: 0,
        missing_slot: 0,
        l: 200,
        u: 400,
        c: 120,
        b: 0,
        r: 1000,
        tts: 100,
        ttr_eff: 100,
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
        guard: [900_000; 7],
        budget: 1000,
        elapsed: 10,
        proposition: 0,
        step_index: -1,
        reserved1: [0u8; 56],
    }
}

fn regime_out() -> RegimeOut {
    RegimeOut {
        payload_0: -7,
        payload_1: -7,
        payload_2: -7,
        reserved: [0x7F; 48],
    }
}

fn verdict_out() -> VerdictOut {
    VerdictOut {
        proposition: -7,
        site: -7,
        margin: -7,
        at_step: -7,
        reserved: [0x7F; 40],
    }
}

fn diagnosis_out() -> DiagnosisOut {
    DiagnosisOut {
        binding_axis: -7,
        absorbed: -7,
        s_epi: -7,
        s_vital: -7,
        s_gov: -7,
        ruin_step: -7,
        terminal_reserve: -7,
    }
}

fn triple() -> StepTriple {
    StepTriple {
        s_epi: 900_000,
        s_vital: 700_000,
        s_gov: 800_000,
        absorbed: 0,
        reserve: 1000,
    }
}

/// An arena on the heap, at least `65536` bytes above the store's first byte.
///
/// `Vec<u64>` rather than `Vec<u8>` so the block is 8-aligned and can hold a
/// `StateRec` without a second copy.
struct Arena {
    words: Vec<u64>,
}

impl Arena {
    fn new() -> Self {
        // 320 bytes of arena plus a little slack, in 8-byte words.
        let words = vec![0u64; 48];
        Self { words }
    }

    fn start(&self) -> usize {
        self.words.as_ptr() as usize
    }
}

// ---------------------------------------------------------------------------
// The boundary, as the module computes it.
// ---------------------------------------------------------------------------

#[test]
fn the_store_span_is_the_fold_buffer_and_not_a_measured_floor() {
    let (start, end) = store_span();
    assert!(start != 0, "a static has an address");
    assert!(end > start, "the span is not empty");
    assert_eq!(end - start, STORE_BYTES, "MAX_STEPS slots of 64 bytes");
    assert_eq!(STORE_BYTES, MAX_STEPS * 64);
}

#[test]
fn the_overlap_predicate_holds_the_boundary_exactly() {
    let (start, end) = store_span();

    assert!(store_overlaps(start, 1), "the first byte is the store's");
    assert!(store_overlaps(start, 320), "the caller's arena at the floor");
    assert!(store_overlaps(start - 8, 16), "a range that straddles the floor");
    assert!(store_overlaps(end - 1, 1), "the last byte is the store's");
    assert!(store_overlaps(end - 8, 8), "the tail reaches the last byte");

    assert!(!store_overlaps(end, 1), "the byte after the store is not its own");
    assert!(!store_overlaps(end, 320), "an arena just above the store");
    assert!(!store_overlaps(start + STORE_BYTES, 320), "65536 bytes above");
    assert!(!store_overlaps(start.saturating_sub(4096), 4096), "just below");
    assert!(!store_overlaps(start + 4096, 0), "an empty interval claims no byte");
}

// ---------------------------------------------------------------------------
// The four entry points refuse a pointer into the store.
// ---------------------------------------------------------------------------

/// The process-wide lock over the module's static store.
///
/// The store is also the fold buffer `diagnose` writes through, so two calls that
/// reach it at the same time in one test binary answer from each other's readings:
/// an unlocked control once saw the vital plane's fold replaced by the epistemic
/// one. Every test that writes the store *or* calls `diagnose` holds this lock.
fn store_lock() -> std::sync::MutexGuard<'static, ()> {
    static STORE_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    STORE_LOCK.lock().unwrap_or_else(|poison| poison.into_inner())
}

/// Snapshot, place a record at `start`, run `call`, then restore the store.
///
/// The store is the module's own memory, so a test that writes there must put it
/// back byte for byte: the tests of one binary share the process, and a
/// half-written buffer would make an unrelated fold answer from a corrupted slot.
/// The lock keeps two snapshot/restore pairs from interleaving, which would leave
/// one test's record in place after the other had restored.
fn at_the_floor<R>(place: impl FnOnce(usize), call: impl FnOnce(usize) -> R) -> R {
    let _guard = store_lock();
    let (start, _) = store_span();
    // The buffer is zero-initialised; the snapshot keeps the restore exact even if
    // that ever stops being true.
    let mut snapshot = vec![0u8; STORE_BYTES];
    // SAFETY: `start` is the first byte of a live static of `STORE_BYTES` bytes.
    unsafe {
        core::ptr::copy_nonoverlapping(start as *const u8, snapshot.as_mut_ptr(), STORE_BYTES);
    }
    place(start);
    let answer = call(start);
    // SAFETY: the same live range, restored from the bytes just copied out of it.
    unsafe {
        core::ptr::copy_nonoverlapping(snapshot.as_ptr(), start as *mut u8, STORE_BYTES);
    }
    answer
}

#[test]
fn step_refuses_a_record_that_overlaps_the_store() {
    let answer = at_the_floor(
        |start| {
            // SAFETY: the store's first byte is 8-aligned and 256 bytes of it are a
            // `StateRec`'s worth of memory; the write is the caller's own mistake.
            unsafe { core::ptr::write(start as *mut StateRec, buffered_record()) };
        },
        |start| {
            let mut out = regime_out();
            // SAFETY: `start` is 8-aligned and holds a `StateRec`; `out` is a live
            // local.
            unsafe { step_at(start as *const StateRec, &mut out) }
        },
    );
    assert_eq!(answer, E_CALLER_CONTRACT);
}

#[test]
fn step_refuses_an_output_record_that_overlaps_the_store() {
    let answer = at_the_floor(
        |_| {},
        |start| {
            let rec = buffered_record();
            // SAFETY: the input is a live local; the output pointer is the caller's
            // mistake, and the gate must refuse it before writing.
            unsafe { step_at(&rec, start as *mut RegimeOut) }
        },
    );
    assert_eq!(answer, E_CALLER_CONTRACT);
}

#[test]
fn accept_refuses_a_record_that_overlaps_the_store() {
    let answer = at_the_floor(
        |start| {
            // SAFETY: as in `step_refuses_a_record_that_overlaps_the_store`.
            unsafe { core::ptr::write(start as *mut StateRec, buffered_record()) };
        },
        |start| {
            let mut out = verdict_out();
            // SAFETY: `start` is 8-aligned and holds a `StateRec`; `out` is a live
            // local.
            unsafe { accept_at(start as *const StateRec, &mut out) }
        },
    );
    assert_eq!(answer, E_CALLER_CONTRACT);
}

#[test]
fn diagnose_refuses_a_trajectory_that_overlaps_the_store() {
    let answer = at_the_floor(
        |start| {
            // SAFETY: the store's first byte is 8-aligned and one `StepTriple` fits.
            unsafe { core::ptr::write(start as *mut StepTriple, triple()) };
        },
        |start| {
            let mut out = diagnosis_out();
            // SAFETY: `start` holds one triple and `len` is 1; `out` is a live local.
            unsafe { diagnose_at(start as *const StepTriple, 1, &mut out) }
        },
    );
    assert_eq!(answer, E_CALLER_CONTRACT);
}

#[test]
fn diagnose_refuses_an_output_record_that_overlaps_the_store() {
    let answer = at_the_floor(
        |_| {},
        |start| {
            let steps = [triple()];
            // SAFETY: the input slice is a live local; the output pointer is the
            // caller's mistake.
            unsafe { diagnose_at(steps.as_ptr(), 1, start as *mut DiagnosisOut) }
        },
    );
    assert_eq!(answer, E_CALLER_CONTRACT);
}

#[test]
fn vectors_hash_refuses_a_destination_that_overlaps_the_store() {
    let answer = at_the_floor(
        |_| {},
        |start| {
            // SAFETY: the destination is the caller's mistake; 32 bytes are written
            // only after the gate admits the pointer.
            unsafe { vectors_hash_at(start as *mut [u8; 32]) }
        },
    );
    assert_eq!(answer, E_CALLER_CONTRACT);
}

// ---------------------------------------------------------------------------
// The control: an arena clear of the store answers normally.
// ---------------------------------------------------------------------------

#[test]
fn the_same_calls_answer_normally_an_arena_of_65536_bytes_above_the_floor() {
    // The diagnosis branch below goes through the module's static fold buffer.
    let _guard = store_lock();
    let (start, end) = store_span();
    let arena = Arena::new();
    let at = arena.start();
    assert!(at >= end, "the heap arena lies above the store");
    assert!(
        at - start >= STORE_BYTES,
        "the arena is at least 65536 bytes above the floor: {} vs {}",
        at - start,
        STORE_BYTES
    );
    assert!(!store_overlaps(at, 320), "and the gate sees it as clear");

    // The record goes in the arena, the output in a local.
    // SAFETY: the arena is 8-aligned, 320 bytes long, and clear of the store.
    unsafe { core::ptr::write(at as *mut StateRec, buffered_record()) };
    let mut out = regime_out();
    // SAFETY: both pointers are aligned, live, and outside the store.
    let tag = unsafe { step_at(at as *const StateRec, &mut out) };
    assert_eq!(tag, 0, "the buffered branch");
    assert_eq!(out.payload_0, 0, "no payload");
    assert_eq!(out.payload_1, 0, "no payload");

    // accept over the same record: the readings fold to a single floor, which is an
    // interior verdict (0), and nothing is refused as a contract fault.
    let mut verdict = verdict_out();
    // SAFETY: as above.
    let status = unsafe { accept_at(at as *const StateRec, &mut verdict) };
    assert_eq!(status, 0, "interior");
    assert_eq!(verdict.margin, 0, "an interior verdict names no site and no margin");

    // diagnose over one triple placed in the arena.
    // SAFETY: 32 bytes of the 320-byte arena, 8-aligned.
    unsafe { core::ptr::write(at as *mut StepTriple, triple()) };
    let mut diagnosis = diagnosis_out();
    // SAFETY: one triple at `at`, output is a live local, both clear of the store.
    let status = unsafe { diagnose_at(at as *const StepTriple, 1, &mut diagnosis) };
    assert_eq!(status, 0, "the fold ran");
    assert_eq!(diagnosis.binding_axis, 2, "binding axis 2 is the vital plane");
    assert_eq!(diagnosis.absorbed, 0);
    assert_eq!(diagnosis.ruin_step, -1, "no ruin on this trajectory");

    // vectors_hash into the arena.
    // SAFETY: 32 bytes of an aligned, clear arena.
    let status = unsafe { vectors_hash_at(at as *mut [u8; 32]) };
    assert_eq!(status, 0);
    // SAFETY: the call just wrote 32 bytes at `at`.
    let digest = unsafe { core::slice::from_raw_parts(at as *const u8, 4) };
    assert_eq!(digest, &[0xef, 0xe2, 0xba, 0xd3], "the fixture's digest prefix");
}


