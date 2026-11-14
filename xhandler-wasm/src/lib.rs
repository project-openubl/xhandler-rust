//! WebAssembly entry point of xhandler, shared by the Go, Python and Java bindings.
//!
//! Every operation goes through a single JSON function, so a host only needs
//! three exports:
//!
//! - `xh_alloc(len) -> ptr` reserves `len` bytes for the request.
//! - `xh_invoke(ptr, len) -> u64` runs `{"op": "...", "input": ...}` and returns
//!   the response as `(ptr << 32) | len`: `{"ok": ...}` or
//!   `{"error": {"kind": "...", "message": "..."}}`.
//! - `xh_free(ptr, len)` releases the request and the response.

mod builder;
mod bytes;
mod ops;
mod sender;
mod signer;

pub use builder::DefaultsInput;
pub use ops::invoke;

/// Reserves `len` bytes in the module memory.
#[no_mangle]
pub extern "C" fn xh_alloc(len: usize) -> *mut u8 {
    Box::into_raw(vec![0u8; len].into_boxed_slice()).cast()
}

/// Releases memory returned by [`xh_alloc`] or [`xh_invoke`]. It is zeroed
/// first: requests may carry passwords and private keys, and instances are
/// reused.
///
/// # Safety
/// `ptr` and `len` must come from one of those functions and be freed once.
#[no_mangle]
pub unsafe extern "C" fn xh_free(ptr: *mut u8, len: usize) {
    let mut memory = Box::from_raw(std::ptr::slice_from_raw_parts_mut(ptr, len));
    memory.fill(0);
}

/// Runs the JSON request at `ptr` and returns the packed `(ptr << 32) | len`
/// of the JSON response, which the caller must release with [`xh_free`].
///
/// # Safety
/// `ptr` and `len` must describe memory returned by [`xh_alloc`].
#[no_mangle]
pub unsafe extern "C" fn xh_invoke(ptr: *const u8, len: usize) -> u64 {
    let request = std::slice::from_raw_parts(ptr, len);
    let response = Box::into_raw(invoke(request).into_boxed_slice());
    ((response.cast::<u8>() as u64) << 32) | response.len() as u64
}
