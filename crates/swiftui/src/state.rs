//! Reactive state management for SwiftUI views.
//!
//! ```ignore
//! use swiftui::state::*;
//!
//! let store = Store::new();
//! let count = store.create(0i32);
//! let name = store.create(String::from("World"));
//!
//! reactive_window("Counter", 300.0, 200.0, &store, move |ctx| {
//!     let n = ctx.get(&count);
//!     let who = ctx.get(&name);
//!     vstack![
//!         text(&format!("Hello, {who}! Count: {n}")).bold().size(24.0),
//!         button("Increment", move || ctx.update(&count, |n| n + 1)),
//!         button("Reset", move || ctx.set(&count, 0)),
//!     ]
//! });
//! ```

use core::ffi::c_void;
use std::any::Any;
use std::sync::{Arc, Mutex};

/// An opaque key to a state value.
#[derive(Clone, Copy)]
pub struct StateKey {
    index: usize,
}

/// Thread-safe state store holding all reactive values.
pub struct Store {
    values: Arc<Mutex<Vec<Box<dyn Any + Send>>>>,
    trigger: Mutex<Option<*mut c_void>>, // model handle for swiftui_trigger_rebuild
}

// SAFETY: the trigger pointer is only used from the main thread via DispatchQueue.main
unsafe impl Send for Store {}
unsafe impl Sync for Store {}

impl Store {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            values: Arc::new(Mutex::new(Vec::new())),
            trigger: Mutex::new(None),
        })
    }

    /// Create a new state value, returns a key to access it.
    pub fn create<T: Any + Send + Clone + 'static>(self: &Arc<Self>, initial: T) -> StateKey {
        let mut vals = self.values.lock().unwrap();
        let index = vals.len();
        vals.push(Box::new(initial));
        StateKey { index }
    }

    /// Get a clone of the current value.
    pub fn get<T: Any + Send + Clone + 'static>(&self, key: &StateKey) -> T {
        let vals = self.values.lock().unwrap();
        vals[key.index].downcast_ref::<T>().unwrap().clone()
    }

    /// Set a new value and trigger UI rebuild.
    pub fn set<T: Any + Send + Clone + 'static>(&self, key: &StateKey, value: T) {
        {
            let mut vals = self.values.lock().unwrap();
            vals[key.index] = Box::new(value);
        }
        self.trigger_rebuild();
    }

    /// Update a value with a function and trigger UI rebuild.
    pub fn update<T: Any + Send + Clone + 'static>(&self, key: &StateKey, f: impl FnOnce(&T) -> T) {
        {
            let mut vals = self.values.lock().unwrap();
            let old = vals[key.index].downcast_ref::<T>().unwrap();
            let new = f(old);
            vals[key.index] = Box::new(new);
        }
        self.trigger_rebuild();
    }

    /// Store the Swift model handle (called by the build trampoline).
    pub(crate) fn set_trigger(&self, handle: *mut c_void) {
        *self.trigger.lock().unwrap() = Some(handle);
    }

    /// Trigger a SwiftUI rebuild via the Swift model.
    fn trigger_rebuild(&self) {
        let handle = *self.trigger.lock().unwrap();
        if let Some(h) = handle {
            // Resolve swiftui_trigger_rebuild via dlsym
            unsafe {
                use core::ffi::c_char;
                unsafe extern "C" {
                    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
                }
                let trigger = dlsym(
                    (-2isize) as *mut c_void,
                    c"swiftui_trigger_rebuild".as_ptr(),
                );
                if !trigger.is_null() {
                    type TriggerFn = unsafe extern "C" fn(*mut c_void);
                    let f: TriggerFn = std::mem::transmute(trigger);
                    f(h);
                }
            }
        }
    }
}

/// Context passed to the reactive build function.
/// Provides read access to state and the ability to create update closures.
pub struct BuildContext {
    pub store: Arc<Store>,
}

impl BuildContext {
    /// Get the current value of a state key.
    pub fn get<T: Any + Send + Clone + 'static>(&self, key: &StateKey) -> T {
        self.store.get(key)
    }
}

/// Open a reactive window. The `build` function is called on every state change.
///
/// ```ignore
/// let store = Store::new();
/// let count = store.create(0);
///
/// reactive_window("App", 400.0, 300.0, &store, move |ctx| {
///     let n = ctx.get(&count);
///     vstack![
///         text(&format!("Count: {n}")).bold().size(24.0),
///         button("+1", {
///             let store = ctx.store.clone();
///             let count = count;
///             move || store.update(&count, |n| n + 1)
///         }),
///     ]
/// });
/// ```
pub fn reactive_window(
    title: &str,
    width: f32,
    height: f32,
    store: &Arc<Store>,
    build: impl Fn(&BuildContext) -> crate::View + 'static,
) {
    crate::app::init_app();
    if !crate::context::is_initialized() {
        crate::init("swift_helper/libSwiftUIHelper.dylib");
    }

    // Box the build closure and store into a struct the trampoline can access
    let data = Box::new(TrampolineData {
        store: store.clone(),
        build: Box::new(build),
    });
    let data_ptr = Box::into_raw(data) as *mut c_void;

    // Resolve the reactive window function
    unsafe {
        use core::ffi::c_char;
        unsafe extern "C" {
            fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
            fn dlopen(path: *const c_char, mode: i32) -> *mut c_void;
        }

        // Ensure helper is loaded
        let paths = [
            c"swift_helper/libSwiftUIHelper.dylib".as_ptr(),
            c"../../swift_helper/libSwiftUIHelper.dylib".as_ptr(),
        ];
        for p in paths {
            dlopen(p, 2);
        }

        let reactive_win = dlsym(
            (-2isize) as *mut c_void,
            c"swiftui_reactive_window".as_ptr(),
        );
        assert!(!reactive_win.is_null(), "swiftui_reactive_window not found");

        type ReactiveWindowFn = unsafe extern "C" fn(
            *const u8,
            usize,
            f32,
            f32,
            unsafe extern "C" fn(*mut c_void, *mut c_void) -> *mut c_void,
            *mut c_void,
        );
        let f: ReactiveWindowFn = std::mem::transmute(reactive_win);
        f(
            title.as_ptr(),
            title.len(),
            width,
            height,
            build_trampoline,
            data_ptr,
        );
    }
}

struct TrampolineData {
    store: Arc<Store>,
    build: Box<dyn Fn(&BuildContext) -> crate::View>,
}

/// Called by Swift on every rebuild. Returns a ViewHandle.
unsafe extern "C" fn build_trampoline(
    user_data: *mut c_void,
    model_handle: *mut c_void,
) -> *mut c_void {
    let data = &*(user_data as *const TrampolineData);

    // Store the model handle so state changes can trigger rebuilds
    data.store.set_trigger(model_handle);

    let ctx = BuildContext {
        store: data.store.clone(),
    };

    let view = (data.build)(&ctx);

    // The view handle needs to be "leaked" to Swift — don't drop it
    let handle = view.into_handle();
    let raw = handle.as_raw();
    std::mem::forget(handle);
    raw
}

/// Button helper that captures a closure for state updates.
pub fn state_button(label: &str, action: impl Fn() + 'static) -> crate::View {
    // Store the closure in a leaked box, pass pointer as userdata
    let boxed: Box<Box<dyn Fn()>> = Box::new(Box::new(action));
    let ptr = Box::into_raw(boxed) as *mut c_void;

    crate::dsl::with_ui(|ui| crate::View::new(ui.button_raw(label, invoke_state_action, ptr)))
}

unsafe extern "C" fn invoke_state_action(ptr: *mut c_void) {
    let action = &*(ptr as *const Box<dyn Fn()>);
    action();
}
