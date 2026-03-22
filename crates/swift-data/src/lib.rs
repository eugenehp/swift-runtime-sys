//! Persistent key-value store backed by UserDefaults.
//!
//! ```ignore
//! use swift_data::*;
//!
//! let db = Store::new();           // standard UserDefaults
//! let db = Store::suite("group.myapp"); // app group suite
//!
//! db.set("users", "name", "Alice");
//! db.set_int("users", "age", 30);
//! db.set_bool("settings", "dark_mode", true);
//!
//! let name = db.get("users", "name");     // Some("Alice")
//! let age = db.get_int("users", "age");   // 30
//! let dark = db.get_bool("settings", "dark_mode"); // true
//!
//! db.delete("users", "name");
//! ```

use core::ffi::{c_char, c_void};

unsafe extern "C" {
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
}

fn sym(name: &core::ffi::CStr) -> *const c_void {
    unsafe { dlsym((-2isize) as *mut c_void, name.as_ptr()) as *const c_void }
}

/// A persistent key-value store.
pub struct Store {
    ptr: *mut c_void,
}

impl Store {
    /// Create using standard UserDefaults.
    pub fn new() -> Self {
        type F = unsafe extern "C" fn(*const u8, usize) -> *mut c_void;
        let f: F = unsafe { std::mem::transmute(sym(c"kv_store_create")) };
        Self {
            ptr: unsafe { f(std::ptr::null(), 0) },
        }
    }

    /// Create with a named suite (for app groups).
    pub fn suite(name: &str) -> Self {
        type F = unsafe extern "C" fn(*const u8, usize) -> *mut c_void;
        let f: F = unsafe { std::mem::transmute(sym(c"kv_store_create")) };
        Self {
            ptr: unsafe { f(name.as_ptr(), name.len()) },
        }
    }

    /// Set a string value.
    pub fn set(&self, table: &str, key: &str, value: &str) {
        type F =
            unsafe extern "C" fn(*mut c_void, *const u8, usize, *const u8, usize, *const u8, usize);
        let f: F = unsafe { std::mem::transmute(sym(c"kv_store_set_string")) };
        unsafe {
            f(
                self.ptr,
                table.as_ptr(),
                table.len(),
                key.as_ptr(),
                key.len(),
                value.as_ptr(),
                value.len(),
            )
        };
    }

    /// Get a string value.
    pub fn get(&self, table: &str, key: &str) -> Option<String> {
        type F = unsafe extern "C" fn(
            *mut c_void,
            *const u8,
            usize,
            *const u8,
            usize,
            *mut *mut c_void,
            *mut usize,
        ) -> bool;
        let f: F = unsafe { std::mem::transmute(sym(c"kv_store_get_string")) };
        let mut ptr: *mut c_void = std::ptr::null_mut();
        let mut len: usize = 0;
        let ok = unsafe {
            f(
                self.ptr,
                table.as_ptr(),
                table.len(),
                key.as_ptr(),
                key.len(),
                &mut ptr,
                &mut len,
            )
        };
        if ok && !ptr.is_null() && len > 0 {
            let s = unsafe {
                String::from_utf8_lossy(std::slice::from_raw_parts(ptr as *const u8, len))
                    .into_owned()
            };
            unsafe { libc::free(ptr) };
            Some(s)
        } else {
            None
        }
    }

    /// Set an integer value.
    pub fn set_int(&self, table: &str, key: &str, value: isize) {
        type F = unsafe extern "C" fn(*mut c_void, *const u8, usize, *const u8, usize, isize);
        let f: F = unsafe { std::mem::transmute(sym(c"kv_store_set_int")) };
        unsafe {
            f(
                self.ptr,
                table.as_ptr(),
                table.len(),
                key.as_ptr(),
                key.len(),
                value,
            )
        };
    }

    /// Get an integer value (returns 0 if not found).
    pub fn get_int(&self, table: &str, key: &str) -> isize {
        type F = unsafe extern "C" fn(*mut c_void, *const u8, usize, *const u8, usize) -> isize;
        let f: F = unsafe { std::mem::transmute(sym(c"kv_store_get_int")) };
        unsafe {
            f(
                self.ptr,
                table.as_ptr(),
                table.len(),
                key.as_ptr(),
                key.len(),
            )
        }
    }

    /// Set a boolean value.
    pub fn set_bool(&self, table: &str, key: &str, value: bool) {
        type F = unsafe extern "C" fn(*mut c_void, *const u8, usize, *const u8, usize, bool);
        let f: F = unsafe { std::mem::transmute(sym(c"kv_store_set_bool")) };
        unsafe {
            f(
                self.ptr,
                table.as_ptr(),
                table.len(),
                key.as_ptr(),
                key.len(),
                value,
            )
        };
    }

    /// Get a boolean value.
    pub fn get_bool(&self, table: &str, key: &str) -> bool {
        type F = unsafe extern "C" fn(*mut c_void, *const u8, usize, *const u8, usize) -> bool;
        let f: F = unsafe { std::mem::transmute(sym(c"kv_store_get_bool")) };
        unsafe {
            f(
                self.ptr,
                table.as_ptr(),
                table.len(),
                key.as_ptr(),
                key.len(),
            )
        }
    }

    /// Delete a key.
    pub fn delete(&self, table: &str, key: &str) {
        type F = unsafe extern "C" fn(*mut c_void, *const u8, usize, *const u8, usize);
        let f: F = unsafe { std::mem::transmute(sym(c"kv_store_delete")) };
        unsafe {
            f(
                self.ptr,
                table.as_ptr(),
                table.len(),
                key.as_ptr(),
                key.len(),
            )
        };
    }
}

impl Drop for Store {
    fn drop(&mut self) {
        type F = unsafe extern "C" fn(*mut c_void);
        let f: F = unsafe { std::mem::transmute(sym(c"kv_store_release")) };
        unsafe { f(self.ptr) };
    }
}
