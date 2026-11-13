//! The one module that touches a caller's memory.
//!
//! Everything in this file is the linear boundary of the archived design document
//! (`02-arquitetura.md` §2): the `#[repr(C)]` records, the pointer gates, the five exports and their pointer
//! forms. Nothing outside this file may reach a raw pointer, which is what the
//! crate-level `deny` and the single `allow` below make checkable rather than
//! conventional: a second boundary would need a second `allow`, and a second `allow`
//! is what the boundary test refuses.
//!
//! The port adds nothing to the core. It reads bytes, rebuilds the core's sealed
//! objects, and writes bytes back; every predicate is the core's own.

#![allow(unsafe_code)]

use core::cell::UnsafeCell;
use core::mem::{align_of, size_of};

use stoch_core::{
    accept as accept_core, diagnose as diagnose_core, step as step_core, Assessment, AxisReading,
    AxialReadings, Bounds, Budget, Capacity, ContingencyTape, ControlTape, Declared, Guards,
    Instrument, MissingSlot, Params, Proposition, Regime, Score, Sentence, SevenReadings, Site,
    State, StepReading, Thresholds, Trajectory, Transcript, Verdict, SCALE,
};

/// The handshake version of this boundary, `02` §2.1.
///
/// The adapter compares it against the version it was built for and refuses to run
/// against another. It is `i64` because the export is `() -> i64`: a version that
/// could alias a regime tag would be read as an action by a caller that forgot the
/// handshake.
pub const ABI_VERSION: i64 = 1;

/// The caller-contract code, `02` §2.3: `-14`, `E_CALLER_CONTRACT`.
///
/// It is outside the core's closed range `-1 ..= -12` on purpose. A misaligned
/// pointer is not an inverted extent, and naming it with a state code would let a
/// caller believe the sentence had been judged.
pub const E_CALLER_CONTRACT: i32 = -14;

/// The arena stride of the caller, `02` §8.1: 320 bytes per step.
///
/// One stride holds a [`StateRec`] (256 bytes) and its [`RegimeOut`] (64 bytes), so a
/// caller that lays out `off_in = base + slot * ARENA_STEP_BYTES` and
/// `off_out = off_in + 256` never overlaps two steps and never writes over the input
/// it is still reading.
pub const ARENA_STEP_BYTES: usize = 320;

/// The greatest trajectory length the port can fold, in steps.
///
/// The fold needs the readings as one slice, and this crate has no allocator. The
/// buffer is static and therefore bounded; a longer trajectory is refused with
/// [`E_CALLER_CONTRACT`] rather than truncated, because a truncated fold is a
/// different fold and the diagnosis would name the wrong step. See README
/// divergence D5.
pub const MAX_STEPS: usize = 1024;

/// One sealed state on the wire, `02` §2.2: 256 bytes, alignment 8.
///
/// The layout is fixed by the document, field by field, and asserted below with
/// `offset_of!` and `size_of!`. The reserved tail is not padding the compiler may
/// reuse: it is the room the document leaves for the fields a later version adds
/// without changing the stride.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct StateRec {
    /// `0`: instrument of observation: `0` sealed, `1` incomplete.
    pub instrument: i32,
    /// `1`: the absent slot, `1 ..= 7` when incomplete and `0` otherwise.
    pub missing_slot: i32,
    /// `2`: lower edge `l`.
    pub l: i64,
    /// `3`: upper edge `u`.
    pub u: i64,
    /// `4`: capacity `c`.
    pub c: i64,
    /// `5`: buffered amount `b`.
    pub b: i64,
    /// `6`: reserve `r`.
    pub r: i64,
    /// `7`: time to serve.
    pub tts: i64,
    /// `8`: effective time to restore.
    pub ttr_eff: i64,
    /// `9`: effective sample size.
    pub n_eff: i64,
    /// `10`: design effect, in integer millionths.
    pub defect: i64,
    /// `11`: `1` when the proposed action can be undone, `0` otherwise.
    pub reversible: i32,
    /// `12`: `1` when the governance channel is corrupt, `0` otherwise.
    pub gov_corrupt: i32,
    /// `13`: `1` when the vital floor is breached, `0` otherwise.
    pub vital_floor: i32,
    /// `14`: `1` when the governance floor is breached, `0` otherwise.
    pub gov_floor: i32,
    /// `15`: `0` when the control tape is not attested, `1` when it is.
    pub tape: i32,
    /// `16`: declared site, `0` for none, `1 ..= 7` for `T1 ..= T7`.
    pub site: i32,
    /// `17`: the acceptance threshold of stage 4.
    pub accept_threshold: i64,
    /// `18`: the width of the frontier band of stage 5.
    pub band_width: i64,
    /// `19`: the seven guard readings in site order, `-1` for absent.
    pub guard: [i64; 7],
    /// `20`: the derivation allowance.
    pub budget: i64,
    /// `21`: what the derivation consumed.
    pub elapsed: i64,
    /// `22`: declared collapse proposition, `0` for none, `1 ..= 3` otherwise.
    pub proposition: i32,
    /// `23`: declared step index, `-1` for none.
    pub step_index: i32,
    /// `24`: reserved, zero.
    pub reserved1: [u8; 56],
}

/// One regime on the wire, `02` §2.2 and §2.4: 64 bytes, alignment 8.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RegimeOut {
    /// `0`: the probe fraction in millionths; `0` for every other regime.
    pub payload_0: i64,
    /// `1`: the breached floor, the absent slot or the halt cause, by tag.
    pub payload_1: i32,
    /// `2`: reserved, zero.
    pub payload_2: i32,
    /// `3`: reserved, zero.
    pub reserved: [u8; 48],
}

/// One verdict on the wire, `02` §2.2: 64 bytes, alignment 8.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VerdictOut {
    /// `0`: the collapse proposition, `0` when the verdict names none.
    pub proposition: i32,
    /// `4`: the guard site, `0` when the verdict names none.
    pub site: i32,
    /// `8`: the margin the band names, `0` for every other verdict.
    pub margin: i64,
    /// `16`: the declared step index, `-1` when none was declared.
    pub at_step: i64,
    /// `24`: reserved, zero.
    pub reserved: [u8; 40],
}

/// One step of a finished trajectory on the wire, `02` §8.1: 32 bytes, alignment 8.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct StepTriple {
    /// `0`: the epistemic reading, `-1` for absent.
    pub s_epi: i64,
    /// `8`: the vital reading, `-1` for absent.
    pub s_vital: i64,
    /// `16`: the governance reading, `-1` for absent.
    pub s_gov: i64,
    /// `24`: `1` when the absorbing barrier was crossed at this step.
    pub absorbed: i32,
    /// `28`: the reserve at the end of this step.
    pub reserve: i32,
}

/// The diagnostic fold on the wire, `02` §2.2: 48 bytes, alignment 8.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DiagnosisOut {
    /// `0`: the binding axis, `0` for none, `1 ..= 3` by axis tag.
    pub binding_axis: i32,
    /// `4`: `1` when the trajectory was absorbed.
    pub absorbed: i32,
    /// `8`: the epistemic coordinate of the fold, `-1` for absent.
    pub s_epi: i64,
    /// `16`: the vital coordinate, `-1` when absent or not read.
    pub s_vital: i64,
    /// `24`: the governance coordinate of the fold, `-1` for absent.
    pub s_gov: i64,
    /// `32`: the step at which the barrier was crossed, `-1` when it was not.
    pub ruin_step: i64,
    /// `40`: the reserve at the end of the trajectory, `-1` for an empty one.
    pub terminal_reserve: i64,
}

// ---------------------------------------------------------------------------
// The layout contract, asserted at compile time against `02` §2.2 and §8.1.
// ---------------------------------------------------------------------------

const _: () = {
    assert!(size_of::<StateRec>() == 256);
    assert!(align_of::<StateRec>() == 8);
    assert!(core::mem::offset_of!(StateRec, instrument) == 0);
    assert!(core::mem::offset_of!(StateRec, missing_slot) == 4);
    assert!(core::mem::offset_of!(StateRec, l) == 8);
    assert!(core::mem::offset_of!(StateRec, u) == 16);
    assert!(core::mem::offset_of!(StateRec, c) == 24);
    assert!(core::mem::offset_of!(StateRec, b) == 32);
    assert!(core::mem::offset_of!(StateRec, r) == 40);
    assert!(core::mem::offset_of!(StateRec, tts) == 48);
    assert!(core::mem::offset_of!(StateRec, ttr_eff) == 56);
    assert!(core::mem::offset_of!(StateRec, n_eff) == 64);
    assert!(core::mem::offset_of!(StateRec, defect) == 72);
    assert!(core::mem::offset_of!(StateRec, reversible) == 80);
    assert!(core::mem::offset_of!(StateRec, gov_corrupt) == 84);
    assert!(core::mem::offset_of!(StateRec, vital_floor) == 88);
    assert!(core::mem::offset_of!(StateRec, gov_floor) == 92);
    assert!(core::mem::offset_of!(StateRec, tape) == 96);
    assert!(core::mem::offset_of!(StateRec, site) == 100);
    assert!(core::mem::offset_of!(StateRec, accept_threshold) == 104);
    assert!(core::mem::offset_of!(StateRec, band_width) == 112);
    assert!(core::mem::offset_of!(StateRec, guard) == 120);
    assert!(core::mem::offset_of!(StateRec, budget) == 176);
    assert!(core::mem::offset_of!(StateRec, elapsed) == 184);
    assert!(core::mem::offset_of!(StateRec, proposition) == 192);
    assert!(core::mem::offset_of!(StateRec, step_index) == 196);
    assert!(core::mem::offset_of!(StateRec, reserved1) == 200);
};

const _: () = {
    assert!(size_of::<RegimeOut>() == 64);
    assert!(align_of::<RegimeOut>() == 8);
    assert!(core::mem::offset_of!(RegimeOut, payload_0) == 0);
    assert!(core::mem::offset_of!(RegimeOut, payload_1) == 8);
    assert!(core::mem::offset_of!(RegimeOut, payload_2) == 12);
    assert!(core::mem::offset_of!(RegimeOut, reserved) == 16);

    assert!(size_of::<VerdictOut>() == 64);
    assert!(align_of::<VerdictOut>() == 8);
    assert!(core::mem::offset_of!(VerdictOut, proposition) == 0);
    assert!(core::mem::offset_of!(VerdictOut, site) == 4);
    assert!(core::mem::offset_of!(VerdictOut, margin) == 8);
    assert!(core::mem::offset_of!(VerdictOut, at_step) == 16);
    assert!(core::mem::offset_of!(VerdictOut, reserved) == 24);

    assert!(size_of::<StepTriple>() == 32);
    assert!(align_of::<StepTriple>() == 8);
    assert!(core::mem::offset_of!(StepTriple, s_epi) == 0);
    assert!(core::mem::offset_of!(StepTriple, s_vital) == 8);
    assert!(core::mem::offset_of!(StepTriple, s_gov) == 16);
    assert!(core::mem::offset_of!(StepTriple, absorbed) == 24);
    assert!(core::mem::offset_of!(StepTriple, reserve) == 28);

    assert!(size_of::<DiagnosisOut>() == 48);
    assert!(align_of::<DiagnosisOut>() == 8);
    assert!(core::mem::offset_of!(DiagnosisOut, binding_axis) == 0);
    assert!(core::mem::offset_of!(DiagnosisOut, absorbed) == 4);
    assert!(core::mem::offset_of!(DiagnosisOut, s_epi) == 8);
    assert!(core::mem::offset_of!(DiagnosisOut, s_vital) == 16);
    assert!(core::mem::offset_of!(DiagnosisOut, s_gov) == 24);
    assert!(core::mem::offset_of!(DiagnosisOut, ruin_step) == 32);
    assert!(core::mem::offset_of!(DiagnosisOut, terminal_reserve) == 40);
};

const _: () = {
    // One stride holds one input and one output record; a caller laying out the
    // arena of §8.1 depends on both halves fitting and on the output not starting
    // inside the input.
    assert!(ARENA_STEP_BYTES >= size_of::<StateRec>() + size_of::<RegimeOut>());
    assert!(size_of::<StateRec>() + size_of::<RegimeOut>() == ARENA_STEP_BYTES);
};

// ---------------------------------------------------------------------------
// Pointers: refused before they are read.
// ---------------------------------------------------------------------------

/// Whether `len` bytes at `ptr` lie wholly inside this module's memory.
///
/// On wasm32 the memory size is an instruction away, so a pointer that a truncated
/// `i32` or an overflowed arena offset produced is refused instead of trapping. On
/// the host the test harness owns the pointer and the range check is the allocator's;
/// the function returns `true` so that the same code path is exercised natively.
#[inline]
fn in_memory(ptr: usize, len: usize) -> bool {
    #[cfg(target_arch = "wasm32")]
    {
        let bytes = core::arch::wasm32::memory_size(0).saturating_mul(65536);
        match ptr.checked_add(len) {
            Some(end) => end <= bytes,
            None => false,
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (ptr, len);
        true
    }
}

/// The byte range `[start, end)` this module's own static store occupies.
///
/// The range is derived from the store itself — `addr_of!` over the buffer the
/// fold borrows — and never from a number measured by hand. A hand-measured floor
/// is a floor for one build: the store moves when the crate changes, and a caller
/// that trusted the number would write its arena over the module's memory without
/// noticing.
#[inline]
pub fn store_span() -> (usize, usize) {
    let start = core::ptr::addr_of!(TRAJECTORY.0) as usize;
    (start, start + size_of::<[StepReading; MAX_STEPS]>())
}

/// Whether `[ptr, ptr + len)` intersects this module's static store.
///
/// The empty interval at a boundary does not intersect: a zero-length input is not
/// a claim on any byte. A range whose end cannot be represented counts as an
/// intersection, because its extent is unknown and the store is the only thing it
/// can be assumed to reach.
#[inline]
pub fn store_overlaps(ptr: usize, len: usize) -> bool {
    if len == 0 {
        return false;
    }
    let (start, end) = store_span();
    match ptr.checked_add(len) {
        Some(tail) => ptr < end && start < tail,
        None => true,
    }
}

/// Whether a record pointer may be read or written: non-null, aligned, in memory,
/// and not inside the module's own static store.
///
/// The last clause is the difference between a refusal and silent corruption: an
/// arena placed at the store's first byte is aligned, non-null and in memory, and
/// a `step` that accepted it would hand the caller the module's readings as its own
/// records.
#[inline]
fn admissible(ptr: usize, len: usize, align: usize) -> bool {
    ptr != 0 && ptr % align == 0 && in_memory(ptr, len) && !store_overlaps(ptr, len)
}

// ---------------------------------------------------------------------------
// The field gate.
// ---------------------------------------------------------------------------

/// Read a `0 | 1` flag, refusing anything else.
#[inline]
fn flag(value: i32) -> Result<bool, i32> {
    match value {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(E_CALLER_CONTRACT),
    }
}

/// Read one guard reading: `-1` is absent, otherwise a margin on the declared scale.
#[inline]
fn margin(value: i64) -> Result<Score, i32> {
    if value == -1 {
        Ok(Score::Missing)
    } else if (0..=SCALE).contains(&value) {
        Ok(Score::Present(value))
    } else {
        // A margin off the scale is not a large margin; it is a number on a scale
        // nobody declared, and the core names that refusal `ScaleOverflow`.
        Err(stoch_core::Error::ScaleOverflow.code())
    }
}

/// Read a wire tag for a site: `0` is none, `1 ..= 7` is `T1 ..= T7`.
#[inline]
fn site(value: i32) -> Result<Option<Site>, i32> {
    match value {
        0 => Ok(None),
        1..=7 => Ok(Site::from_code(value as u8)),
        _ => Err(E_CALLER_CONTRACT),
    }
}

/// Read a wire tag for a collapse proposition: `0` is none, `1 ..= 3` otherwise.
#[inline]
fn proposition(value: i32) -> Result<Option<Proposition>, i32> {
    match value {
        0 => Ok(None),
        1..=3 => Ok(Proposition::from_code(i64::from(value))),
        _ => Err(E_CALLER_CONTRACT),
    }
}

/// Read a declared step index: `-1` is none, `0 ..= u32::MAX` is a step.
#[inline]
fn step_index(value: i32) -> Result<Option<u32>, i32> {
    if value == -1 {
        Ok(None)
    } else if value >= 0 {
        Ok(Some(value as u32))
    } else {
        Err(E_CALLER_CONTRACT)
    }
}

/// Rebuild a sealed [`State`] from the record, or name why the record is not one.
///
/// The reconstruction is total: every field is read, every tag is checked against
/// its closed set, and [`State::seal`] then applies the core's own seven checks in
/// the core's own order — this function does not duplicate them.
fn state_from(rec: &StateRec) -> Result<State, i32> {
    let tape = match rec.tape {
        0 => ControlTape::Unattested,
        1 => ControlTape::Attested,
        _ => return Err(E_CALLER_CONTRACT),
    };

    let instrument = match rec.instrument {
        0 => {
            if rec.missing_slot != 0 {
                return Err(E_CALLER_CONTRACT);
            }
            Instrument::Sealed(SevenReadings::new(rec.guard))
        }
        1 => {
            if !(1..=7).contains(&rec.missing_slot) {
                return Err(E_CALLER_CONTRACT);
            }
            match MissingSlot::new(rec.missing_slot as u8) {
                Some(slot) => Instrument::Incomplete(slot),
                None => return Err(E_CALLER_CONTRACT),
            }
        }
        _ => return Err(E_CALLER_CONTRACT),
    };

    let params = Params::new(
        rec.tts,
        rec.ttr_eff,
        flag(rec.reversible)?,
        flag(rec.vital_floor)?,
        flag(rec.gov_floor)?,
        flag(rec.gov_corrupt)?,
        Thresholds::new(rec.accept_threshold, rec.band_width),
    );

    State::seal(
        tape,
        Bounds::new(rec.l, rec.u, rec.n_eff, rec.defect),
        Capacity::new(rec.c, rec.b, rec.r),
        Guards::new(instrument),
        params,
    )
    .map_err(|error| error.code())
}

/// The weakest reading and the count of readings over a run of sites.
///
/// `-1` is absent and is not counted; a present margin is. The axis is therefore
/// read from the same seven numbers the step reads, which is why the record needs no
/// per-axis column: the axis reading *is* the weakest of its sites (`01` §3.3).
fn axis_reading(margins: &[i64; 7], first: usize, last: usize) -> Result<AxisReading, i32> {
    let mut count = 0usize;
    let mut weakest: Option<i64> = None;
    let mut index = first;
    while index <= last {
        match margin(margins[index])? {
            Score::Present(value) => {
                count += 1;
                weakest = Some(match weakest {
                    Some(current) => {
                        if value < current {
                            value
                        } else {
                            current
                        }
                    }
                    None => value,
                });
            }
            Score::Missing => {}
        }
        index += 1;
    }
    let weakest = match weakest {
        Some(value) => Score::Present(value),
        None => Score::Missing,
    };
    Ok(AxisReading::new(count, weakest))
}

// ---------------------------------------------------------------------------
// The five exports.
// ---------------------------------------------------------------------------

/// The handshake version, `02` §2.1.
///
/// Returns [`ABI_VERSION`] and never fails: it is the export an adapter calls before
/// it trusts any other.
#[no_mangle]
pub extern "C" fn abi_version() -> i64 {
    ABI_VERSION
}

/// Write the 32-byte digest of the fixture at `out`, `02` §2.1.
///
/// The digest is the core's [`stoch_core::vectors_hash`], which is the SHA-256 of the
/// `vectors.kv` this crate was built against — the file itself, embedded at build
/// time, not a literal copied from it. The first eight bytes are the handshake token
/// the adapter compares; the whole 32 are written because the export returns `i32`
/// and 64 bits do not fit in it (§2.1 fixes the signature; see README D3).
///
/// Returns `0` on success, [`E_CALLER_CONTRACT`] for a null or out-of-memory pointer.
#[no_mangle]
pub extern "C" fn vectors_hash(out: i32) -> i32 {
    // SAFETY: the pointer is validated by `vectors_hash_at`, which checks null,
    // alignment and range before the single store.
    unsafe { vectors_hash_at(out as usize as *mut [u8; 32]) }
}

/// Take one step and write its regime, `02` §2.1 and §2.4.
///
/// Returns the regime tag `0 ..= 5` on success, or a negative code: the core's
/// `-1 ..= -12` for a refusal of the state, [`E_CALLER_CONTRACT`] for a fault of the
/// record. On any negative return `out` is untouched.
#[no_mangle]
pub extern "C" fn step(rec: i32, out: i32) -> i32 {
    // SAFETY: both pointers are validated inside `step_at`.
    unsafe {
        step_at(
            rec as usize as *const StateRec,
            out as usize as *mut RegimeOut,
        )
    }
}

/// Judge one transcript and write its verdict, `02` §2.1 and §2.2.
///
/// The axes are derived from the seven guard readings of the record (each axis's
/// weakest present reading, and the count of present ones), which is the definition
/// of the fold; the record carries no per-axis margin, so none is invented.
/// The contingency record is [`ContingencyTape::new`]`(0)`: `accept` never reads it,
/// and §2.2 gives it no field.
///
/// Returns the verdict tag `0 ..= 3`, or a negative code. On any negative return
/// `out` is untouched.
#[no_mangle]
pub extern "C" fn accept(rec: i32, out: i32) -> i32 {
    // SAFETY: both pointers are validated inside `accept_at`.
    unsafe {
        accept_at(
            rec as usize as *const StateRec,
            out as usize as *mut VerdictOut,
        )
    }
}

/// Fold a finished trajectory and write its diagnosis, `02` §2.1 and §8.1.
///
/// The client's separation is a property of the boundary, not a convention: the
/// output is a [`DiagnosisOut`], which is not a [`StateRec`], carries no regime, no
/// action set and no state field, and therefore cannot be handed back to [`step`].
/// The type that does cross is a plain record of numbers, so no path from a diagnosis
/// to an action exists in either direction of the wire.
///
/// Returns `0` on success, [`E_CALLER_CONTRACT`] for a null, misaligned or
/// out-of-memory pointer, for a `len` outside `0 ..= `[`MAX_STEPS`], for a flag
/// outside `0 | 1`, or for an absorbed step that carries a vital reading (a value the
/// barrier cannot have); returns [`stoch_core::Error::ScaleOverflow`]`::code()` for a
/// reading off the declared scale. On any negative return `out` is untouched.
#[no_mangle]
pub extern "C" fn diagnose(triples: i32, len: i32, out: i32) -> i32 {
    // SAFETY: all three arguments are validated inside `diagnose_at`.
    unsafe {
        diagnose_at(
            triples as usize as *const StepTriple,
            len,
            out as usize as *mut DiagnosisOut,
        )
    }
}

// ---------------------------------------------------------------------------
// The port proper. These functions take real pointers so that the host tests
// exercise the same code the wasm exports run.
// ---------------------------------------------------------------------------

/// The pointer form of [`vectors_hash`].
///
/// # Safety
///
/// `out` must be null or point to 32 writable bytes inside this module's memory.
pub unsafe fn vectors_hash_at(out: *mut [u8; 32]) -> i32 {
    let address = out as usize;
    if !admissible(address, size_of::<[u8; 32]>(), align_of::<[u8; 32]>()) {
        return E_CALLER_CONTRACT;
    }
    let digest = stoch_core::vectors_hash();
    // SAFETY: the caller contract above places 32 writable bytes at `out`, and the
    // pointer was checked non-null, aligned and in range.
    unsafe { core::ptr::write(out, digest) };
    0
}

/// The pointer form of [`step`].
///
/// # Safety
///
/// `rec` must point to a readable [`StateRec`] and `out` to a writable
/// [`RegimeOut`], both inside this module's memory.
pub unsafe fn step_at(rec: *const StateRec, out: *mut RegimeOut) -> i32 {
    if !admissible(
        rec as usize,
        size_of::<StateRec>(),
        align_of::<StateRec>(),
    ) || !admissible(out as usize, size_of::<RegimeOut>(), align_of::<RegimeOut>())
    {
        return E_CALLER_CONTRACT;
    }

    // SAFETY: `rec` is aligned and in range for a whole `StateRec`, and the struct is
    // `Copy` and carries no invalid bit pattern, so the read is a copy of caller
    // bytes and creates no reference into caller memory.
    let record = unsafe { core::ptr::read(rec) };

    let regime = match state_from(&record) {
        Ok(state) => match step_core(&state) {
            Ok(regime) => regime,
            Err(error) => return error.code(),
        },
        Err(code) => return code,
    };

    let (payload_0, payload_1) = match regime {
        Regime::ReversibleFractionalProbe { fraction } => (fraction, 0),
        Regime::InwardAction { breached } => (0, i32::from(breached.code())),
        Regime::Quarantine { missing } => (0, i32::from(missing.get())),
        Regime::AlgedonicHalt { cause } => (0, i32::from(cause.code())),
        Regime::BufferedAction | Regime::NoAction => (0, 0),
    };

    let written = RegimeOut {
        payload_0,
        payload_1,
        payload_2: 0,
        reserved: [0u8; 48],
    };
    // SAFETY: the output pointer was checked aligned and in range for 64 bytes, and
    // the record is assembled in full before this store.
    unsafe { core::ptr::write(out, written) };

    i32::from(regime.tag())
}

/// The pointer form of [`accept`].
///
/// # Safety
///
/// `rec` must point to a readable [`StateRec`] and `out` to a writable
/// [`VerdictOut`], both inside this module's memory.
pub unsafe fn accept_at(rec: *const StateRec, out: *mut VerdictOut) -> i32 {
    if !admissible(
        rec as usize,
        size_of::<StateRec>(),
        align_of::<StateRec>(),
    ) || !admissible(out as usize, size_of::<VerdictOut>(), align_of::<VerdictOut>())
    {
        return E_CALLER_CONTRACT;
    }

    // SAFETY: as in `step_at`; the pointer is aligned and in range for a `StateRec`.
    let record = unsafe { core::ptr::read(rec) };

    let verdict = match verdict_from(&record) {
        Ok(verdict) => verdict,
        Err(code) => return code,
    };

    let (proposition_tag, site_tag, margin, at_step) = match verdict {
        Verdict::Interior => (0, 0, 0, -1),
        Verdict::Boundary { site, margin } => (0, i32::from(site.code()), margin, -1),
        Verdict::Collapse {
            proposition,
            site,
            step,
        } => (
            proposition.code() as i32,
            i32::from(site.code()),
            0,
            i64::from(step),
        ),
        Verdict::Unparsable { .. } => (0, 0, 0, -1),
    };

    let written = VerdictOut {
        proposition: proposition_tag,
        site: site_tag,
        margin,
        at_step,
        reserved: [0u8; 40],
    };
    // SAFETY: the output pointer was checked aligned and in range for 64 bytes, and
    // the record is assembled in full before this store.
    unsafe { core::ptr::write(out, written) };

    i32::from(verdict.tag())
}

/// Rebuild the transcript from the record and judge it.
fn verdict_from(rec: &StateRec) -> Result<Verdict, i32> {
    let tape = match rec.tape {
        0 => ControlTape::Unattested,
        1 => ControlTape::Attested,
        _ => return Err(E_CALLER_CONTRACT),
    };

    // An incomplete instrument has no reading at the absent slot. A record that
    // carries one is declaring a measurement it also declares it did not take.
    if rec.instrument == 1 {
        if !(1..=7).contains(&rec.missing_slot) {
            return Err(E_CALLER_CONTRACT);
        }
        if rec.guard[(rec.missing_slot - 1) as usize] != -1 {
            return Err(E_CALLER_CONTRACT);
        }
    } else if rec.instrument != 0 || rec.missing_slot != 0 {
        return Err(E_CALLER_CONTRACT);
    }

    let axes = AxialReadings::new(
        axis_reading(&rec.guard, 0, 2)?,
        axis_reading(&rec.guard, 3, 4)?,
        axis_reading(&rec.guard, 5, 6)?,
    );

    // `Assessment::new` declares the pair; the door that refuses it is
    // `Transcript::seal`, which bounds both thresholds and is called below.
    let assessment = Assessment::new(rec.accept_threshold, rec.band_width);

    let sentence = Sentence {
        tape,
        contingency: ContingencyTape::new(0),
        bounds: Bounds::new(rec.l, rec.u, rec.n_eff, rec.defect),
        capacity: Capacity::new(rec.c, rec.b, rec.r),
        budget: Budget::new(rec.budget, rec.elapsed),
        axes,
        declared: Declared::new(
            proposition(rec.proposition)?,
            site(rec.site)?,
            step_index(rec.step_index)?,
        ),
        assessment,
    };

    let transcript = Transcript::seal(sentence).map_err(|error| error.code())?;
    accept_core(&transcript).map_err(|error| error.code())
}

/// The trajectory buffer the fold borrows.
///
/// A `no_std` crate without an allocator cannot build a `&[StepReading]` of unknown
/// length; it can only borrow one. The buffer is static because the fold is over a
/// slice, the slice must live somewhere, and the caller's memory is not ours to
/// write. `UnsafeCell` rather than `static mut` keeps the aliasing rule explicit:
/// the only access is the single `&mut` taken in `diagnose_at`, and wasm is
/// single-threaded, so no two calls overlap.
struct TrajectoryBuffer(UnsafeCell<[StepReading; MAX_STEPS]>);

// SAFETY: a wasm module has one thread and no interrupt that reaches this crate; the
// only access is the exclusive borrow taken for the duration of one call to
// `diagnose_at`, which does not call out to the caller.
unsafe impl Sync for TrajectoryBuffer {}

/// The one reading every slot starts from, so the buffer needs no allocator.
///
/// Every slot is overwritten before the fold reads it, so the initial value is
/// unobservable; it is chosen as the all-zero bit pattern (`Score::Present(0)`) so
/// that the buffer lands in zero-initialised memory and costs no data segment bytes
/// in the shipped module.
const EMPTY_READING: StepReading = StepReading::new(
    Score::Present(0),
    Score::Present(0),
    Score::Present(0),
    false,
    0,
);

static TRAJECTORY: TrajectoryBuffer = TrajectoryBuffer(UnsafeCell::new([EMPTY_READING; MAX_STEPS]));

/// The pointer form of [`diagnose`].
///
/// # Safety
///
/// `triples` must point to `len` readable [`StepTriple`] records and `out` to a
/// writable [`DiagnosisOut`], both inside this module's memory; `len` must be
/// non-negative.
pub unsafe fn diagnose_at(triples: *const StepTriple, len: i32, out: *mut DiagnosisOut) -> i32 {
    if !admissible(out as usize, size_of::<DiagnosisOut>(), align_of::<DiagnosisOut>()) {
        return E_CALLER_CONTRACT;
    }
    if len < 0 {
        return E_CALLER_CONTRACT;
    }
    let len = len as usize;
    if len > MAX_STEPS {
        return E_CALLER_CONTRACT;
    }
    if len > 0
        && !admissible(
            triples as usize,
            len * size_of::<StepTriple>(),
            align_of::<StepTriple>(),
        )
    {
        return E_CALLER_CONTRACT;
    }

    // SAFETY: the buffer is a `static` of `MAX_STEPS` slots, `len <= MAX_STEPS`, and
    // this is the only borrow taken in the call; `diagnose_core` never re-enters.
    let buffer: &mut [StepReading] = unsafe {
        let base = TRAJECTORY.0.get() as *mut StepReading;
        core::slice::from_raw_parts_mut(base, len)
    };

    let mut index = 0usize;
    while index < len {
        // SAFETY: the input range was checked to hold `len` triples; each read is a
        // copy of an aligned, in-range record.
        let triple = unsafe { core::ptr::read(triples.add(index)) };
        let absorbed = match triple.absorbed {
            0 => false,
            1 => true,
            _ => return E_CALLER_CONTRACT,
        };
        let s_epi = match margin(triple.s_epi) {
            Ok(score) => score,
            Err(code) => return code,
        };
        let s_vital = match margin(triple.s_vital) {
            Ok(score) => score,
            Err(code) => return code,
        };
        let s_gov = match margin(triple.s_gov) {
            Ok(score) => score,
            Err(code) => return code,
        };
        // After absorption the vital coordinate is a barrier, not a number. A record
        // that carries one is inventing a measurement the barrier cannot have, and
        // the core would silently drop it; the port refuses it instead.
        if absorbed && s_vital.is_present() {
            return E_CALLER_CONTRACT;
        }
        buffer[index] = StepReading::new(
            s_epi,
            s_vital,
            s_gov,
            absorbed,
            i64::from(triple.reserve),
        );
        index += 1;
    }

    let trajectory = Trajectory::seal(buffer);
    let view = diagnose_core(&trajectory).observe();

    let written = DiagnosisOut {
        binding_axis: match view.binding_axis {
            Some(axis) => i32::from(axis.code()),
            None => 0,
        },
        absorbed: if view.absorbed { 1 } else { 0 },
        s_epi: score_to_wire(view.s_epi),
        s_vital: score_to_wire(view.s_vital),
        s_gov: score_to_wire(view.s_gov),
        ruin_step: match view.ruin_step {
            Some(step) => i64::from(step),
            None => -1,
        },
        terminal_reserve: score_to_wire(view.terminal_reserve),
    };
    // SAFETY: the output pointer was checked aligned and in range for 48 bytes, and
    // the record is assembled in full before this store.
    unsafe { core::ptr::write(out, written) };

    0
}

/// `Score` on the wire: a present value, or `-1` for an absent one.
#[inline]
fn score_to_wire(score: Score) -> i64 {
    match score {
        Score::Present(value) => value,
        Score::Missing => -1,
    }
}

// ---------------------------------------------------------------------------
// Unit tests for the gates that can be exercised without a caller's arena.
// ---------------------------------------------------------------------------

#[cfg(test)]
mod port_tests {
    use super::*;

    #[test]
    fn the_version_is_one_and_never_zero() {
        assert_eq!(abi_version(), 1);
        assert_eq!(ABI_VERSION, 1);
    }

    #[test]
    fn a_null_or_misaligned_pointer_is_a_caller_contract_fault() {
        // SAFETY: null pointers are exactly what this test asserts are refused.
        unsafe {
            assert_eq!(vectors_hash_at(core::ptr::null_mut()), E_CALLER_CONTRACT);
            assert_eq!(step_at(core::ptr::null(), core::ptr::null_mut()), E_CALLER_CONTRACT);
            assert_eq!(accept_at(core::ptr::null(), core::ptr::null_mut()), E_CALLER_CONTRACT);
            assert_eq!(diagnose_at(core::ptr::null(), 0, core::ptr::null_mut()), E_CALLER_CONTRACT);
        }
    }

    #[test]
    fn a_margin_off_the_scale_is_the_core_code_and_not_a_caller_fault() {
        assert_eq!(margin(SCALE + 1), Err(stoch_core::Error::ScaleOverflow.code()));
        assert_eq!(margin(-2), Err(stoch_core::Error::ScaleOverflow.code()));
        assert_eq!(margin(-1), Ok(Score::Missing));
        assert_eq!(margin(0), Ok(Score::Present(0)));
        assert_eq!(margin(SCALE), Ok(Score::Present(SCALE)));
    }

    #[test]
    fn the_field_gate_refuses_tags_it_does_not_define() {
        assert_eq!(flag(2), Err(E_CALLER_CONTRACT));
        assert_eq!(flag(-1), Err(E_CALLER_CONTRACT));
        assert_eq!(site(8), Err(E_CALLER_CONTRACT));
        assert_eq!(proposition(4), Err(E_CALLER_CONTRACT));
        assert_eq!(step_index(-2), Err(E_CALLER_CONTRACT));
        assert_eq!(site(0), Ok(None));
        assert_eq!(step_index(-1), Ok(None));
    }
}
