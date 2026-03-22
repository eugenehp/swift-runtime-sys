//! Opaque view handle — wraps a retained AnyView pointer from the Swift helper.

use core::ffi::c_void;

/// An opaque handle to a SwiftUI view.
///
/// Holds a retained reference to a Swift `AnyView` object.
/// Automatically releases on drop.
pub struct ViewHandle {
    ptr: *mut c_void,
    release_fn: Option<unsafe extern "C" fn(*mut c_void)>,
}

impl ViewHandle {
    pub(crate) fn new(ptr: *mut c_void, release_fn: unsafe extern "C" fn(*mut c_void)) -> Self {
        Self { ptr, release_fn: Some(release_fn) }
    }

    /// Get the raw pointer (for passing to helper functions).
    pub fn as_raw(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for ViewHandle {
    fn drop(&mut self) {
        if let Some(release) = self.release_fn {
            unsafe { release(self.ptr) };
        }
    }
}

// ViewHandle is not Clone — each handle owns a retain count.
// Use the SwiftUI API to create new views instead.

unsafe impl Send for ViewHandle {}
