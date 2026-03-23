//! # winit-swift — drop-in winit replacement with full Apple platform support
//!
//! Provides windowing, Metal rendering, haptics, accessibility, HDR, and
//! visionOS/RealityKit integration — all through a single Swift bridge dylib.
//!
//! ## Quick start
//!
//! ```ignore
//! use winit_swift::*;
//!
//! fn main() {
//!     let mut app = App::new();
//!     let window = app.create_window(WindowAttributes::new("Hello", 800.0, 600.0));
//!     app.run(|event, control| match event {
//!         Event::RedrawRequested(id) => { /* draw */ },
//!         Event::CloseRequested(id) => control.exit(),
//!         _ => {}
//!     });
//! }
//! ```
//!
//! ## Metal rendering
//!
//! ```ignore
//! let gpu = app.metal_device().unwrap();
//! let layer = window.metal_layer().unwrap();
//! let queue = gpu.command_queue();
//! // ... render with Metal ...
//! ```
//!
//! ## Build the Swift bridge
//!
//! ```bash
//! cd crates/winit-swift/swift
//! swiftc -emit-library -o libWinitSwift.dylib WinitSwiftBridge.swift \
//!   -framework Foundation -framework QuartzCore \
//!   -framework Metal -framework CoreGraphics \
//!   -framework AppKit -framework CoreHaptics
//! ```

#![allow(non_snake_case)]

mod bridge;
pub mod event;
pub mod window;
pub mod metal;
pub mod haptics;
pub mod monitor;
pub mod accessibility;
pub mod compat;
pub mod prelude;
pub mod visionos;
pub mod sil;

pub use event::*;
pub use window::*;
pub use metal::*;
pub use haptics::*;
pub use monitor::*;
pub use accessibility::*;

use core::ffi::c_void;
use std::sync::OnceLock;

use bridge::Bridge;

type Handle = *mut c_void;

// ── Bridge Loader ───────────────────────────────────────────────────────────

static BRIDGE: OnceLock<Bridge> = OnceLock::new();

fn fns() -> &'static Bridge {
    BRIDGE.get().expect(
        "winit-swift bridge not loaded. Call winit_swift::load(\"path/to/libWinitSwift.dylib\") \
         or winit_swift::auto_load() first."
    )
}

/// Load the Swift bridge dylib. Must be called before any other function.
pub fn load(path: &str) {
    BRIDGE.get_or_init(|| bridge::load_bridge(path));
}

/// Auto-find and load the Swift bridge from common paths.
pub fn auto_load() {
    let candidates = [
        "libWinitSwift.dylib",
        "crates/winit-swift/swift/libWinitSwift.dylib",
        "../../crates/winit-swift/swift/libWinitSwift.dylib",
        "swift/libWinitSwift.dylib",
    ];
    for c in candidates {
        if std::path::Path::new(c).exists() {
            load(c);
            return;
        }
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let p = dir.join("libWinitSwift.dylib");
            if p.exists() {
                load(p.to_str().unwrap());
                return;
            }
        }
    }
    panic!(
        "libWinitSwift.dylib not found. Build it:\n  \
         cd crates/winit-swift/swift && swiftc -emit-library -o libWinitSwift.dylib \
         WinitSwiftBridge.swift -framework Foundation -framework QuartzCore \
         -framework Metal -framework CoreGraphics -framework AppKit -framework CoreHaptics"
    );
}

// ── App (Event Loop) ────────────────────────────────────────────────────────

/// The main application / event loop.
pub struct App {
    _initialized: bool,
}

/// Controls event loop behavior.
pub struct ControlFlow {
    should_exit: bool,
}

impl ControlFlow {
    /// Request the event loop to exit after this callback returns.
    pub fn exit(&mut self) {
        self.should_exit = true;
    }
}

impl App {
    /// Create a new application. Initializes the Swift bridge and platform runtime.
    pub fn new() -> Self {
        auto_load();
        let b = fns();

        // Set up the event callback
        unsafe {
            (b.init)(event_dispatch_trampoline);
        }

        App { _initialized: true }
    }

    /// Create a new application loading the bridge from a specific path.
    pub fn with_bridge(path: &str) -> Self {
        load(path);
        let b = fns();
        unsafe {
            (b.init)(event_dispatch_trampoline);
        }
        App { _initialized: true }
    }

    /// Create a window with the given attributes.
    pub fn create_window(&self, attrs: WindowAttributes) -> Window {
        Window::new(attrs)
    }

    /// Get the Metal device (GPU).
    pub fn metal_device(&self) -> Option<MetalDevice> {
        let h = unsafe { (fns().metal_device)() };
        if h.is_null() { None } else { Some(MetalDevice { handle: h }) }
    }

    /// Run the event loop, calling the handler for each event.
    ///
    /// This function blocks until the application exits.
    pub fn run<F: FnMut(Event, &mut ControlFlow) + 'static>(self, mut handler: F) {
        // Store the handler in thread-local storage
        EVENT_HANDLER.with(|cell| {
            cell.replace(Some(Box::new(move |event| {
                let mut cf = ControlFlow { should_exit: false };
                handler(event, &mut cf);
                if cf.should_exit {
                    unsafe { (fns().stop_event_loop)() };
                }
            })));
        });

        unsafe { (fns().run_event_loop)() };
    }

    /// Poll for events without blocking.
    pub fn poll<F: FnMut(Event, &mut ControlFlow) + 'static>(&self, mut handler: F) {
        EVENT_HANDLER.with(|cell| {
            cell.replace(Some(Box::new(move |event| {
                let mut cf = ControlFlow { should_exit: false };
                handler(event, &mut cf);
                if cf.should_exit {
                    unsafe { (fns().stop_event_loop)() };
                }
            })));
        });

        unsafe { (fns().poll_events)() };
    }

    /// Poll for events with a timeout.
    pub fn poll_timeout<F: FnMut(Event, &mut ControlFlow) + 'static>(&self, timeout_secs: f64, mut handler: F) {
        EVENT_HANDLER.with(|cell| {
            cell.replace(Some(Box::new(move |event| {
                let mut cf = ControlFlow { should_exit: false };
                handler(event, &mut cf);
                if cf.should_exit {
                    unsafe { (fns().stop_event_loop)() };
                }
            })));
        });

        unsafe { (fns().poll_events_timeout)(timeout_secs) };
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

// ── Event dispatch ──────────────────────────────────────────────────────────

type EventHandler = Box<dyn FnMut(Event)>;

thread_local! {
    static EVENT_HANDLER: std::cell::RefCell<Option<EventHandler>> = std::cell::RefCell::new(None);
}

extern "C" fn event_dispatch_trampoline(
    event_type: u32,
    window_id: u64,
    a: i64, b: i64,
    x: f64, y: f64,
) {
    let event = Event::from_raw(event_type, window_id, a, b, x, y);
    EVENT_HANDLER.with(|cell| {
        if let Some(handler) = cell.borrow_mut().as_mut() {
            handler(event);
        }
    });
}

// ── Compat event dispatch ───────────────────────────────────────────────────

extern "C" fn event_dispatch_trampoline_compat(
    event_type: u32,
    window_id: u64,
    a: i64, b: i64,
    x: f64, y: f64,
) {
    let event = Event::from_raw(event_type, window_id, a, b, x, y);
    compat::event_loop::dispatch_compat(event);
}

// ── raw-window-handle ───────────────────────────────────────────────────────

impl raw_window_handle::HasDisplayHandle for App {
    fn display_handle(&self) -> Result<raw_window_handle::DisplayHandle<'_>, raw_window_handle::HandleError> {
        let raw = raw_window_handle::RawDisplayHandle::AppKit(
            raw_window_handle::AppKitDisplayHandle::new()
        );
        Ok(unsafe { raw_window_handle::DisplayHandle::borrow_raw(raw) })
    }
}
