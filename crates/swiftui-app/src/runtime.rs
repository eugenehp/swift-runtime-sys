//! Runtime — load the generated Swift app and launch it.

use core::ffi::{c_char, c_void};
use std::ffi::CString;

unsafe extern "C" {
    fn dlopen(path: *const c_char, mode: i32) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
}

/// Events dispatched to your app handler.
#[derive(Debug)]
pub enum Event {
    /// App initialized, surfaces ready.
    Init,
    /// Metal frame — render into this texture.
    /// `window` is the window ID, `texture` is an `MTLTexture*`.
    Frame {
        window: &'static str,
        texture: *mut c_void,
    },
    /// App is terminating.
    Terminate,
}

/// Launch the SwiftUI app.
///
/// The `handler` receives events:
/// - `Event::Init` — app launched, surfaces ready
/// - `Event::Frame { window, texture }` — render a frame to the Metal texture
/// - `Event::Terminate` — app shutting down
///
/// This function **never returns**.
///
/// # Example
///
/// ```ignore
/// fn main() {
///     swiftui_app::launch(|event| match event {
///         swiftui_app::Event::Init => println!("Ready!"),
///         swiftui_app::Event::Frame { texture, .. } => {
///             // render with Metal to `texture`
///         },
///         _ => {}
///     });
/// }
/// ```
pub fn launch<F: FnMut(Event) + 'static>(handler: F) -> ! {
    unsafe { launch_inner(handler) }
}

// Thread-local handler storage
type Handler = Box<dyn FnMut(Event)>;
thread_local! {
    static HANDLER: std::cell::RefCell<Option<Handler>> = std::cell::RefCell::new(None);
}

// C callbacks that bridge Swift → Rust
extern "C" fn on_init_callback() {
    HANDLER.with(|h| {
        if let Some(handler) = h.borrow_mut().as_mut() {
            handler(Event::Init);
        }
    });
}

extern "C" fn on_frame_main_callback(texture: *mut c_void) {
    HANDLER.with(|h| {
        if let Some(handler) = h.borrow_mut().as_mut() {
            handler(Event::Frame {
                window: "main",
                texture,
            });
        }
    });
}

unsafe fn launch_inner<F: FnMut(Event) + 'static>(handler: F) -> ! {
    // Store handler
    HANDLER.with(|h| {
        *h.borrow_mut() = Some(Box::new(handler));
    });

    // Load the generated dylib
    let dylib_path = option_env!("SWIFTUI_APP_DYLIB")
        .unwrap_or("libGeneratedApp.dylib");

    let candidates = [
        dylib_path.to_string(),
        format!("{}/libGeneratedApp.dylib", env!("OUT_DIR", ".")),
        "libGeneratedApp.dylib".into(),
    ];

    let mut handle: *mut c_void = std::ptr::null_mut();
    for path in &candidates {
        let cpath = CString::new(path.as_str()).unwrap();
        let h = dlopen(cpath.as_ptr(), 1);
        if !h.is_null() {
            handle = h;
            break;
        }
    }
    assert!(
        !handle.is_null(),
        "Failed to load GeneratedApp dylib. Did build.rs run?"
    );

    // Register callbacks
    let set_init = dlsym(handle, c"swiftui_app_set_on_init".as_ptr());
    if !set_init.is_null() {
        type SetFn = unsafe extern "C" fn(extern "C" fn());
        let f: SetFn = core::mem::transmute(set_init);
        f(on_init_callback);
    }

    // Register frame callback for "main" window
    let set_frame = dlsym(handle, c"swiftui_app_set_on_frame_main".as_ptr());
    if !set_frame.is_null() {
        type SetFn = unsafe extern "C" fn(extern "C" fn(*mut c_void));
        let f: SetFn = core::mem::transmute(set_frame);
        f(on_frame_main_callback);
    }

    // Launch
    let launch = dlsym(handle, c"swiftui_app_launch".as_ptr());
    assert!(!launch.is_null(), "swiftui_app_launch not found in dylib");
    type LaunchFn = unsafe extern "C" fn();
    let f: LaunchFn = core::mem::transmute(launch);
    f();

    std::process::exit(0);
}
