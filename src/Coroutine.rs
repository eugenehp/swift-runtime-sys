#![allow(non_camel_case_types, non_snake_case, dead_code)]

//! Swift runtime coroutine support.

use core::ffi::c_void;

unsafe extern "C" {
    /// Allocate a coroutine frame.
    pub fn swift_coroFrameAlloc(size: usize, align_mask: usize) -> *mut c_void;
}
