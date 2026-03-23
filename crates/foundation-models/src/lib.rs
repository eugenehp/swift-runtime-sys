//! Apple Intelligence on-device LLM from Rust.
//!
//! **Platform support:** macOS 26+, iOS 26+ (not available on tvOS, watchOS, or visionOS).
//!
//! Uses Apple's FoundationModels framework (macOS 26+) for private,
//! on-device language model inference. No API keys, no network required.
//!
//! ```ignore
//! use foundation_models::*;
//!
//! if is_available() {
//!     let session = Session::new(Some("You are a helpful assistant."));
//!     let response = session.respond("What is Rust?");
//!     println!("{}", response.unwrap());
//!
//!     // Streaming
//!     session.stream("Tell me a joke.", |token| {
//!         print!("{token}");
//!     });
//! }
//! ```

// Note: FoundationModels is available on macOS 26+, iOS 26+.
// On unsupported platforms, `is_available()` returns false.

use core::ffi::c_void;

/// Check if Apple Intelligence / on-device LLM is available.
pub fn is_available() -> bool {
    let f = apple_sys_helpers::sym(c"fm_available");
    if f.is_null() {
        return false;
    }
    type F = unsafe extern "C" fn() -> bool;
    unsafe { (std::mem::transmute::<_, F>(f))() }
}

/// A language model session.
pub struct Session {
    ptr: *mut c_void,
}

impl Session {
    /// Create a new session, optionally with system instructions.
    pub fn new(instructions: Option<&str>) -> Self {
        type F = unsafe extern "C" fn(*const u8, usize) -> *mut c_void;
        let f: F = unsafe { std::mem::transmute(apple_sys_helpers::sym(c"fm_session_create")) };
        let ptr = match instructions {
            Some(inst) => unsafe { f(inst.as_ptr(), inst.len()) },
            None => unsafe { f(std::ptr::null(), 0) },
        };
        Self {
            ptr: if ptr.is_null() {
                panic!("Failed to create LLM session — is Apple Intelligence available?")
            } else {
                ptr
            },
        }
    }

    /// Send a prompt and get a complete response (blocking).
    pub fn respond(&self, prompt: &str) -> Option<String> {
        type F = unsafe extern "C" fn(
            *mut c_void,
            *const u8,
            usize,
            *mut *mut c_void,
            *mut usize,
        ) -> bool;
        let f: F = unsafe { std::mem::transmute(apple_sys_helpers::sym(c"fm_respond")) };
        let mut ptr: *mut c_void = std::ptr::null_mut();
        let mut len: usize = 0;
        let ok = unsafe { f(self.ptr, prompt.as_ptr(), prompt.len(), &mut ptr, &mut len) };
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

    /// Stream a response token-by-token. Calls `on_token` for each partial,
    /// then `on_done` when complete.
    pub fn stream(&self, prompt: &str, on_token: impl Fn(&str) + 'static) {
        let token_box: Box<Box<dyn Fn(*const u8, usize)>> = Box::new(Box::new(move |ptr, len| {
            let s = unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(ptr, len)) };
            on_token(s);
        }));
        let token_ud = Box::into_raw(token_box) as *mut c_void;

        let done_box: Box<Box<dyn Fn()>> = Box::new(Box::new(|| {}));
        let _done_ud = Box::into_raw(done_box) as *mut c_void;

        unsafe extern "C" fn token_tramp(ptr: *const u8, len: usize, ud: *mut c_void) {
            let f = &*(ud as *const Box<dyn Fn(*const u8, usize)>);
            f(ptr, len);
        }
        unsafe extern "C" fn done_tramp(ud: *mut c_void) {
            let _ = &*(ud as *const Box<dyn Fn()>);
        }

        type F = unsafe extern "C" fn(
            *mut c_void,
            *const u8,
            usize,
            unsafe extern "C" fn(*const u8, usize, *mut c_void),
            unsafe extern "C" fn(*mut c_void),
            *mut c_void,
        );
        let f: F = unsafe { std::mem::transmute(apple_sys_helpers::sym(c"fm_stream_respond")) };
        unsafe {
            f(
                self.ptr,
                prompt.as_ptr(),
                prompt.len(),
                token_tramp,
                done_tramp,
                token_ud,
            )
        };
    }

    /// Check if the session is currently generating a response.
    pub fn is_responding(&self) -> bool {
        type F = unsafe extern "C" fn(*mut c_void) -> bool;
        let f: F = unsafe { std::mem::transmute(apple_sys_helpers::sym(c"fm_is_responding")) };
        unsafe { f(self.ptr) }
    }

    /// Send a prompt and get a response asynchronously.
    /// Requires the `async` feature: `foundation-models = { features = ["async"] }`
    #[cfg(feature = "async")]
    pub async fn respond_async(&self, prompt: &str) -> Option<String> {
        let prompt = prompt.to_string();
        let ptr = self.ptr as usize; // coerce to usize for Send
        let result = tokio::task::spawn_blocking(move || unsafe {
            let ptr = ptr as *mut c_void;
            type F = unsafe extern "C" fn(
                *mut c_void,
                *const u8,
                usize,
                *mut *mut c_void,
                *mut usize,
            ) -> bool;
            let f: F = std::mem::transmute(apple_sys_helpers::sym(c"fm_respond"));
            let mut out_ptr: *mut c_void = std::ptr::null_mut();
            let mut out_len: usize = 0;
            let ok = f(
                ptr,
                prompt.as_ptr(),
                prompt.len(),
                &mut out_ptr,
                &mut out_len,
            );
            if ok && !out_ptr.is_null() && out_len > 0 {
                let s = String::from_utf8_lossy(std::slice::from_raw_parts(
                    out_ptr as *const u8,
                    out_len,
                ))
                .into_owned();
                libc::free(out_ptr);
                Some(s)
            } else {
                None
            }
        })
        .await;
        result.ok().flatten()
    }

    /// Stream a response asynchronously, yielding each token via a channel.
    /// Returns a receiver that produces token strings.
    pub fn stream_channel(&self, prompt: &str) -> std::sync::mpsc::Receiver<String> {
        let (tx, rx) = std::sync::mpsc::channel();
        let token_box: Box<Box<dyn Fn(*const u8, usize)>> = Box::new(Box::new(move |ptr, len| {
            let s = unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(ptr, len)) };
            let _ = tx.send(s.to_string());
        }));
        let token_ud = Box::into_raw(token_box) as *mut c_void;

        unsafe extern "C" fn token_tramp(ptr: *const u8, len: usize, ud: *mut c_void) {
            let f = &*(ud as *const Box<dyn Fn(*const u8, usize)>);
            f(ptr, len);
        }
        unsafe extern "C" fn done_tramp(_ud: *mut c_void) {}

        type F = unsafe extern "C" fn(
            *mut c_void,
            *const u8,
            usize,
            unsafe extern "C" fn(*const u8, usize, *mut c_void),
            unsafe extern "C" fn(*mut c_void),
            *mut c_void,
        );
        let f: F = unsafe { std::mem::transmute(apple_sys_helpers::sym(c"fm_stream_respond")) };
        unsafe {
            f(
                self.ptr,
                prompt.as_ptr(),
                prompt.len(),
                token_tramp,
                done_tramp,
                token_ud,
            )
        };

        rx
    }
}

// Safety: Session pointer is only accessed from the main thread via Swift dispatch
unsafe impl Send for Session {}
unsafe impl Sync for Session {}

impl Drop for Session {
    fn drop(&mut self) {
        let f = apple_sys_helpers::sym(c"fm_session_release");
        if !f.is_null() {
            type F = unsafe extern "C" fn(*mut c_void);
            unsafe { (std::mem::transmute::<_, F>(f))(self.ptr) };
        }
    }
}
