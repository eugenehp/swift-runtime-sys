//! Reference counting helpers.

use core::ffi::c_void;

/// A retained Swift heap object. Automatically releases on drop.
pub struct Retained(*mut c_void);

impl Retained {
    /// Take ownership of a raw Swift object pointer.
    ///
    /// # Safety
    /// The pointer must be a valid, retained Swift heap object.
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            Some(Self(ptr))
        }
    }

    /// Get the raw pointer without consuming.
    pub fn as_raw(&self) -> *mut c_void {
        self.0
    }

    /// Release ownership, returning the raw pointer.
    /// Caller is responsible for releasing.
    pub fn into_raw(self) -> *mut c_void {
        let ptr = self.0;
        core::mem::forget(self);
        ptr
    }
}

impl Clone for Retained {
    fn clone(&self) -> Self {
        unsafe {
            swift_runtime_sys::RuntimeRaw::swift_retain(self.0);
        }
        Self(self.0)
    }
}

impl Drop for Retained {
    fn drop(&mut self) {
        unsafe {
            swift_runtime_sys::RuntimeRaw::swift_release(self.0);
        }
    }
}

impl std::fmt::Debug for Retained {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Retained({:?})", self.0)
    }
}
