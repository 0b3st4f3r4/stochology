//! Isolated Hardware Security Module (`PKCS#11`-style) strict-verification boundary.
//!
//! Defines the safe [`HsmBackend`] trait and [`StrictHsmSigner`] wrapper enforcing
//! [`VerifyingKey::verify_strict`] immediately after every signature to catch hardware
//! fault injection before releasing bytes to the protocol layer.
//!
//! The in-memory simulated C-ABI FFI mock (`OpaqueHsmSlot` and `HsmSigner`) is strictly
//! gated behind the `simulated-hsm-ffi` feature so default production builds contain zero
//! `unsafe` regions and zero `#[no_mangle]` exports.

use core::sync::atomic::Ordering;

use ed25519_dalek::{Signature, VerifyingKey};

use crate::metrics::LockFreeMetrics;
use crate::HostError;

/// Abstract signing backend whose private scalar material remains non-extractable.
pub trait HsmBackend {
    /// Returns the 32-byte public verifying key held by this slot.
    fn verifying_key(&self) -> VerifyingKey;

    /// Signs `msg` inside the isolated hardware/FFI domain and returns a raw 64-byte signature.
    fn sign_raw(&self, msg: &[u8]) -> Result<[u8; 64], HostError>;
}

/// Safe host custody wrapper unconditionally running [`VerifyingKey::verify_strict`] after every signature.
pub struct StrictHsmSigner<B: HsmBackend> {
    backend: B,
    verifying_key: VerifyingKey,
}

impl<B: HsmBackend> StrictHsmSigner<B> {
    /// Wraps an isolated [`HsmBackend`] and caches its public verifying key.
    #[must_use]
    pub fn new(backend: B) -> Self {
        let verifying_key = backend.verifying_key();
        Self {
            backend,
            verifying_key,
        }
    }

    /// Returns the public verifying key of this HSM slot.
    #[must_use]
    pub fn verifying_key(&self) -> VerifyingKey {
        self.verifying_key
    }

    /// Borrows the underlying backend handle.
    #[must_use]
    pub fn backend(&self) -> &B {
        &self.backend
    }

    /// Signs `msg` via `backend` and immediately enforces [`VerifyingKey::verify_strict`].
    ///
    /// On verification failure, increments `LockFreeMetrics::hsm_fault_injections_total` and
    /// refuses to release the corrupted signature.
    pub fn sign_and_verify_strict(
        &self,
        msg: &[u8],
        metrics: &LockFreeMetrics,
    ) -> Result<[u8; 64], HostError> {
        let sig_bytes = self.backend.sign_raw(msg)?;
        let sig = Signature::from_bytes(&sig_bytes);
        if self.verifying_key.verify_strict(msg, &sig).is_err() {
            metrics
                .hsm_fault_injections_total
                .fetch_add(1, Ordering::Relaxed);
            return Err(HostError::HsmFaultInjectionDetected);
        }
        Ok(sig_bytes)
    }
}

#[cfg(feature = "simulated-hsm-ffi")]
#[allow(unsafe_code)]
mod ffi {
    use core::ffi::c_int;
    use core::sync::atomic::{AtomicBool, Ordering};
    use ed25519_dalek::{Signer, SigningKey};

    /// Opaque C-ABI handle holding a non-extractable simulated Ed25519 signing key.
    pub struct OpaqueHsmSlot {
        signing_key: SigningKey,
        inject_next_fault: AtomicBool,
    }

    pub const HSM_OK: c_int = 0;
    pub const HSM_ERR_ARG: c_int = -1;

    /// Allocates an opaque non-extractable Ed25519 key slot from 32-byte seed material.
    ///
    /// # Safety
    /// Caller must ensure `seed_ptr` is null or points to at least 32 valid initialized bytes.
    #[no_mangle]
    pub unsafe extern "C" fn stoch_hsm_create_slot(seed_ptr: *const u8) -> *mut OpaqueHsmSlot {
        if seed_ptr.is_null() {
            return core::ptr::null_mut();
        }
        // SAFETY: caller guarantees `seed_ptr` points to 32 readable bytes.
        let seed_slice = unsafe { core::slice::from_raw_parts(seed_ptr, 32) };
        let Ok(seed_arr) = <[u8; 32]>::try_from(seed_slice) else {
            return core::ptr::null_mut();
        };
        let slot = Box::new(OpaqueHsmSlot {
            signing_key: SigningKey::from_bytes(&seed_arr),
            inject_next_fault: AtomicBool::new(false),
        });
        Box::into_raw(slot)
    }

    /// Destroys a previously created [`OpaqueHsmSlot`].
    #[no_mangle]
    pub unsafe extern "C" fn stoch_hsm_destroy_slot(slot: *mut OpaqueHsmSlot) {
        if !slot.is_null() {
            // SAFETY: `slot` originated from `Box::into_raw` and is freed once on Drop.
            unsafe {
                drop(Box::from_raw(slot));
            }
        }
    }

    /// Reads the 32-byte public verifying key without exposing private scalar bytes.
    #[no_mangle]
    pub unsafe extern "C" fn stoch_hsm_public_key(
        slot: *const OpaqueHsmSlot,
        out_pk: *mut u8,
    ) -> c_int {
        if slot.is_null() || out_pk.is_null() {
            return HSM_ERR_ARG;
        }
        // SAFETY: both pointers validated non-null for caller lifetime; slot carries 32B vk.
        let slot_ref = unsafe { &*slot };
        let vk_bytes = slot_ref.signing_key.verifying_key().to_bytes();
        // SAFETY: caller guarantees `out_pk` points to 32 writable bytes.
        let out_slice = unsafe { core::slice::from_raw_parts_mut(out_pk, 32) };
        out_slice.copy_from_slice(&vk_bytes);
        HSM_OK
    }

    /// Arms a one-shot simulated physical fault glitch inside the HSM module for self-check tests.
    #[no_mangle]
    pub unsafe extern "C" fn stoch_hsm_arm_fault_glitch(slot: *const OpaqueHsmSlot) -> c_int {
        if slot.is_null() {
            return HSM_ERR_ARG;
        }
        // SAFETY: pointer checked non-null; field is lock-free `AtomicBool`.
        let slot_ref = unsafe { &*slot };
        slot_ref.inject_next_fault.store(true, Ordering::Release);
        HSM_OK
    }

    /// Signs `msg_ptr[..msg_len]` across the C ABI boundary (`out_sig` must hold 64 bytes).
    #[no_mangle]
    pub unsafe extern "C" fn stoch_hsm_sign_ed25519(
        slot: *const OpaqueHsmSlot,
        msg_ptr: *const u8,
        msg_len: usize,
        out_sig: *mut u8,
    ) -> c_int {
        if slot.is_null() || out_sig.is_null() || (msg_len > 0 && msg_ptr.is_null()) {
            return HSM_ERR_ARG;
        }
        // SAFETY: validated non-null pointers and slice extents for the FFI call duration.
        let slot_ref = unsafe { &*slot };
        let msg = if msg_len == 0 {
            &[]
        } else {
            // SAFETY: caller guarantees `msg_ptr` spans `msg_len` initialized bytes.
            unsafe { core::slice::from_raw_parts(msg_ptr, msg_len) }
        };
        let mut sig_bytes = slot_ref.signing_key.sign(msg).to_bytes();
        if slot_ref.inject_next_fault.swap(false, Ordering::AcqRel) {
            // Simulate hardware bit-flip glitch inside signing arithmetic.
            sig_bytes[17] ^= 0x80;
        }
        // SAFETY: caller guarantees `out_sig` points to 64 writable bytes.
        let out_slice = unsafe { core::slice::from_raw_parts_mut(out_sig, 64) };
        out_slice.copy_from_slice(&sig_bytes);
        HSM_OK
    }
}

#[cfg(feature = "simulated-hsm-ffi")]
mod simulated_ffi {
    use core::ptr::NonNull;

    use ed25519_dalek::VerifyingKey;

    use crate::hsm::{ffi, HsmBackend, StrictHsmSigner};
    use crate::metrics::LockFreeMetrics;
    use crate::HostError;

    /// Safe Rust custody handle over the isolated C-ABI simulated HSM slot (`verify_strict` mandatory).
    pub struct HsmSigner {
        inner: StrictHsmSigner<SimulatedFfiBackend>,
    }

    pub(super) struct SimulatedFfiBackend {
        handle: NonNull<ffi::OpaqueHsmSlot>,
        verifying_key: VerifyingKey,
    }

    // SAFETY: `OpaqueHsmSlot` holds immutable `SigningKey` bytes and lock-free `AtomicBool`.
    #[allow(unsafe_code)]
    unsafe impl Send for SimulatedFfiBackend {}
    // SAFETY: `OpaqueHsmSlot` uses exclusively immutable reads plus atomic `swap`/`store`.
    #[allow(unsafe_code)]
    unsafe impl Sync for SimulatedFfiBackend {}

    impl Drop for SimulatedFfiBackend {
        fn drop(&mut self) {
            #[allow(unsafe_code)]
            unsafe {
                ffi::stoch_hsm_destroy_slot(self.handle.as_ptr());
            }
        }
    }

    impl HsmBackend for SimulatedFfiBackend {
        fn verifying_key(&self) -> VerifyingKey {
            self.verifying_key
        }

        fn sign_raw(&self, msg: &[u8]) -> Result<[u8; 64], HostError> {
            let mut sig_bytes = [0_u8; 64];
            #[allow(unsafe_code)]
            let rc = unsafe {
                ffi::stoch_hsm_sign_ed25519(
                    self.handle.as_ptr(),
                    msg.as_ptr(),
                    msg.len(),
                    sig_bytes.as_mut_ptr(),
                )
            };
            if rc != ffi::HSM_OK {
                return Err(HostError::HsmFfi("HSM C-ABI sign call returned error"));
            }
            Ok(sig_bytes)
        }
    }

    impl HsmSigner {
        /// Initializes an opaque HSM slot from 32-byte seed bytes (non-extractable afterwards).
        pub fn from_seed(seed: [u8; 32]) -> Result<Self, HostError> {
            #[allow(unsafe_code)]
            let raw = unsafe { ffi::stoch_hsm_create_slot(seed.as_ptr()) };
            let handle = NonNull::new(raw)
                .ok_or(HostError::HsmFfi("HSM slot allocation returned null"))?;

            let mut pk_bytes = [0_u8; 32];
            #[allow(unsafe_code)]
            let rc = unsafe { ffi::stoch_hsm_public_key(handle.as_ptr(), pk_bytes.as_mut_ptr()) };
            if rc != ffi::HSM_OK {
                #[allow(unsafe_code)]
                unsafe {
                    ffi::stoch_hsm_destroy_slot(handle.as_ptr());
                }
                return Err(HostError::HsmFfi("failed reading HSM public key"));
            }
            let verifying_key = VerifyingKey::from_bytes(&pk_bytes)
                .map_err(|_| HostError::HsmFfi("invalid Ed25519 public key from HSM"))?;
            Ok(Self {
                inner: StrictHsmSigner::new(SimulatedFfiBackend {
                    handle,
                    verifying_key,
                }),
            })
        }

        /// Returns the 32-byte public verifying key of this hardware slot.
        #[must_use]
        pub fn verifying_key(&self) -> VerifyingKey {
            self.inner.verifying_key()
        }

        /// Injects a single-call simulated hardware glitch inside the C-ABI slot for fault-detection tests.
        pub fn arm_simulated_fault_glitch(&self) -> Result<(), HostError> {
            #[allow(unsafe_code)]
            let rc = unsafe { ffi::stoch_hsm_arm_fault_glitch(self.inner.backend().handle.as_ptr()) };
            if rc == ffi::HSM_OK {
                Ok(())
            } else {
                Err(HostError::HsmFfi("failed arming simulated fault glitch"))
            }
        }

        /// Signs `msg` across the C-ABI boundary and immediately runs [`VerifyingKey::verify_strict`].
        pub fn sign_and_verify_strict(
            &self,
            msg: &[u8],
            metrics: &LockFreeMetrics,
        ) -> Result<[u8; 64], HostError> {
            self.inner.sign_and_verify_strict(msg, metrics)
        }
    }
}

#[cfg(feature = "simulated-hsm-ffi")]
pub use simulated_ffi::HsmSigner;
