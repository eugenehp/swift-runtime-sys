//! Combine publisher/subscriber bridge from Rust.
//!
//! **Platform support:** macOS 10.15+, iOS 13+, tvOS 13+, visionOS 1+, watchOS 6+.
//!
//! Provides `Subject` (PassthroughSubject) and `CurrentValue` (CurrentValueSubject)
//! for reactive event streams between Rust and Swift.
//!
//! ```ignore
//! let subject = Subject::new();
//! let _sub = subject.subscribe(|v| println!("Got: {v}"));
//! subject.send(42);
//!
//! let current = CurrentValue::new(0);
//! current.set(10);
//! assert_eq!(current.get(), 10);
//! ```
//!
//! Subscriptions auto-cancel on drop. Subjects auto-release on drop.

use core::ffi::c_void;

/// A PassthroughSubject<Int> — sends values to subscribers.
pub struct Subject {
    ptr: *mut c_void,
}

impl Subject {
    pub fn new() -> Self {
        type F = unsafe extern "C" fn() -> *mut c_void;
        let f: F = unsafe { std::mem::transmute(apple_sys_helpers::sym(c"combine_subject_create")) };
        Self {
            ptr: unsafe { f() },
        }
    }

    pub fn send(&self, value: isize) {
        type F = unsafe extern "C" fn(*mut c_void, isize);
        let f: F = unsafe { std::mem::transmute(apple_sys_helpers::sym(c"combine_subject_send")) };
        unsafe { f(self.ptr, value) };
    }

    pub fn subscribe(&self, callback: impl Fn(isize) + 'static) -> Subscription {
        let boxed: Box<Box<dyn Fn(isize)>> = Box::new(Box::new(callback));
        let ud = Box::into_raw(boxed) as *mut c_void;
        unsafe extern "C" fn tramp(val: isize, ud: *mut c_void) {
            let f = &*(ud as *const Box<dyn Fn(isize)>);
            f(val);
        }
        type F = unsafe extern "C" fn(
            *mut c_void,
            unsafe extern "C" fn(isize, *mut c_void),
            *mut c_void,
        ) -> *mut c_void;
        let f: F = unsafe { std::mem::transmute(apple_sys_helpers::sym(c"combine_subject_subscribe")) };
        Subscription {
            ptr: unsafe { f(self.ptr, tramp, ud) },
        }
    }
}

impl Drop for Subject {
    fn drop(&mut self) {
        type F = unsafe extern "C" fn(*mut c_void);
        let f: F = unsafe { std::mem::transmute(apple_sys_helpers::sym(c"combine_release")) };
        unsafe { f(self.ptr) };
    }
}

/// A CurrentValueSubject<Int> — holds a value, notifies on change.
pub struct CurrentValue {
    ptr: *mut c_void,
}

impl CurrentValue {
    pub fn new(initial: isize) -> Self {
        type F = unsafe extern "C" fn(isize) -> *mut c_void;
        let f: F = unsafe { std::mem::transmute(apple_sys_helpers::sym(c"combine_current_value_create")) };
        Self {
            ptr: unsafe { f(initial) },
        }
    }

    pub fn get(&self) -> isize {
        type F = unsafe extern "C" fn(*mut c_void) -> isize;
        let f: F = unsafe { std::mem::transmute(apple_sys_helpers::sym(c"combine_current_value_get")) };
        unsafe { f(self.ptr) }
    }

    pub fn set(&self, value: isize) {
        type F = unsafe extern "C" fn(*mut c_void, isize);
        let f: F = unsafe { std::mem::transmute(apple_sys_helpers::sym(c"combine_current_value_set")) };
        unsafe { f(self.ptr, value) };
    }
}

impl Drop for CurrentValue {
    fn drop(&mut self) {
        type F = unsafe extern "C" fn(*mut c_void);
        let f: F = unsafe { std::mem::transmute(apple_sys_helpers::sym(c"combine_release")) };
        unsafe { f(self.ptr) };
    }
}

/// A subscription — cancels on drop.
pub struct Subscription {
    ptr: *mut c_void,
}

impl Drop for Subscription {
    fn drop(&mut self) {
        type F = unsafe extern "C" fn(*mut c_void);
        let f: F = unsafe { std::mem::transmute(apple_sys_helpers::sym(c"combine_cancel")) };
        unsafe { f(self.ptr) };
    }
}
