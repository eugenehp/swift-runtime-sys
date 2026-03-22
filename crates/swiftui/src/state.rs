//! Reactive state for SwiftUI views.
//!
//! ```ignore
//! use swiftui::prelude::*;
//!
//! app("Counter", 400.0, 300.0, |cx| {
//!     let count = cx.state(0i32);
//!
//!     vstack![
//!         text(&format!("Count: {}", count.get())).bold().size(48.0),
//!         button("+1", count.bind(|n| n + 1)),
//!         button("Reset", count.set_to(0)),
//!     ]
//! });
//! ```

use core::ffi::c_void;
use std::any::Any;
use std::sync::{Arc, Mutex};

// ═══════════════════════════════════════════════════════════════════════════
// Internal store
// ═══════════════════════════════════════════════════════════════════════════

struct StoreInner {
    values: Vec<Box<dyn Any + Send>>,
    trigger: Option<*mut c_void>,
}

// SAFETY: trigger pointer is only used via DispatchQueue.main in Swift
unsafe impl Send for StoreInner {}

#[derive(Clone)]
pub struct Store(Arc<Mutex<StoreInner>>);

impl Store {
    pub(crate) fn new() -> Self {
        Self(Arc::new(Mutex::new(StoreInner {
            values: Vec::new(),
            trigger: None,
        })))
    }

    fn create<T: Any + Send + Clone + 'static>(&self, initial: T) -> usize {
        let mut inner = self.0.lock().unwrap();
        let idx = inner.values.len();
        inner.values.push(Box::new(initial));
        idx
    }

    fn get<T: Any + Send + Clone + 'static>(&self, idx: usize) -> T {
        let inner = self.0.lock().unwrap();
        inner.values[idx].downcast_ref::<T>().unwrap().clone()
    }

    fn set_raw(&self, idx: usize, val: Box<dyn Any + Send>) {
        let mut inner = self.0.lock().unwrap();
        inner.values[idx] = val;
        let trigger = inner.trigger;
        drop(inner);
        if let Some(h) = trigger {
            trigger_swift(h);
        }
    }

    pub fn set_trigger(&self, handle: *mut c_void) {
        self.0.lock().unwrap().trigger = Some(handle);
    }
}

fn trigger_swift(handle: *mut c_void) {
    unsafe {
        use core::ffi::c_char;
        unsafe extern "C" {
            fn dlsym(h: *mut c_void, s: *const c_char) -> *mut c_void;
        }
        let f = dlsym(
            (-2isize) as *mut c_void,
            c"swiftui_trigger_rebuild".as_ptr(),
        );
        if !f.is_null() {
            type F = unsafe extern "C" fn(*mut c_void);
            (std::mem::transmute::<_, F>(f))(handle);
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// State<T> — a handle to a reactive value
// ═══════════════════════════════════════════════════════════════════════════

/// A handle to a reactive state value. Cheap to clone (just index + Arc).
///
/// ```ignore
/// let count = cx.state(0i32);
/// count.get()               // read
/// count.set(42)             // write + rebuild
/// count.update(|n| n + 1)   // mutate + rebuild
/// count.bind(|n| n + 1)     // closure for button callbacks
/// count.set_to(0)           // closure that sets to a fixed value
/// ```
pub struct State<T: Any + Send + Clone + 'static> {
    idx: usize,
    pub store: Store,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Any + Send + Clone + 'static> Clone for State<T> {
    fn clone(&self) -> Self {
        Self {
            idx: self.idx,
            store: self.store.clone(),
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T: Any + Send + Clone + 'static> State<T> {
    /// Get the current value.
    pub fn get(&self) -> T {
        self.store.get::<T>(self.idx)
    }

    /// Set a new value. Triggers UI rebuild.
    pub fn set(&self, value: T) {
        self.store.set_raw(self.idx, Box::new(value));
    }

    /// Update the value with a function. Triggers UI rebuild.
    pub fn update(&self, f: impl FnOnce(&T) -> T) {
        let old = self.get();
        let new = f(&old);
        self.set(new);
    }

    /// Create a callback closure that updates the value.
    /// Use with `button(label, count.bind(|n| n + 1))`.
    pub fn bind(&self, f: impl Fn(&T) -> T + 'static) -> impl Fn() + 'static {
        let s = self.clone();
        move || s.update(&f)
    }

    /// Create a callback closure that sets to a fixed value.
    /// Use with `button(label, count.set_to(0))`.
    pub fn set_to(&self, value: T) -> impl Fn() + 'static
    where
        T: Clone,
    {
        let s = self.clone();
        move || s.set(value.clone())
    }

    /// Derive a value from the current state (read-only transform).
    /// ```ignore
    /// let count = cx.state(5i32);
    /// let label = count.map(|n| format!("Count: {n}"));
    /// text(&label) // "Count: 5"
    /// ```
    pub fn map<U>(&self, f: impl FnOnce(&T) -> U) -> U {
        let val = self.get();
        f(&val)
    }

    /// Create a closure that runs an arbitrary action with access to current value.
    /// Avoids manual `.clone()` boilerplate:
    /// ```ignore
    /// // Before:
    /// let ns = notes.clone();
    /// let ss = selected.clone();
    /// button("Add", move || { ns.update(|n| ...); ss.set(0); })
    ///
    /// // After:
    /// button("Add", action(&[&notes, &selected], || { ... }))
    /// ```
    pub fn with<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&State<T>) -> R,
    {
        f(self)
    }
}

impl State<bool> {
    /// Toggle a boolean state. Use with `button("Toggle", flag.toggle())`.
    pub fn toggle(&self) -> impl Fn() + 'static {
        self.bind(|b| !b)
    }
}

impl State<i32> {
    /// Increment by 1.
    pub fn increment(&self) -> impl Fn() + 'static {
        self.bind(|n| n + 1)
    }
    /// Decrement by 1.
    pub fn decrement(&self) -> impl Fn() + 'static {
        self.bind(|n| n - 1)
    }
}

impl State<String> {
    /// Get as &str via map.
    pub fn as_str(&self) -> String {
        self.get()
    }
}

/// Create a closure that captures multiple states. Avoids manual cloning:
/// ```ignore
/// // Before:
/// let ns = notes.clone();
/// let ss = selected.clone();
/// button("Act", move || { ns.update(...); ss.set(0); })
///
/// // After:
/// let ns = notes.clone();
/// let ss = selected.clone();
/// button("Act", action(move || { ns.update(...); ss.set(0); }))
/// ```
pub fn action(f: impl Fn() + 'static) -> impl Fn() + 'static {
    f
}

// ═══════════════════════════════════════════════════════════════════════════
// Cx — build context passed to the reactive build function
// ═══════════════════════════════════════════════════════════════════════════

/// Build context — provides state creation and access.
pub struct Cx {
    pub store: Store,
    pub next_idx: std::cell::Cell<usize>,
}

impl Cx {
    /// Create or access a state value. On first build, creates with `initial`.
    /// On subsequent builds, returns the existing value (ignores `initial`).
    pub fn state<T: Any + Send + Clone + 'static>(&self, initial: T) -> State<T> {
        let idx = self.next_idx.get();
        self.next_idx.set(idx + 1);

        // Create on first call, skip on rebuild
        {
            let inner = self.store.0.lock().unwrap();
            if idx >= inner.values.len() {
                drop(inner);
                self.store.create(initial);
            }
        }

        State {
            idx,
            store: self.store.clone(),
            _phantom: std::marker::PhantomData,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// button() that works with state closures
// ═══════════════════════════════════════════════════════════════════════════

/// Create a button that triggers a state update on click.
///
/// ```ignore
/// button("+1", count.bind(|n| n + 1))
/// button("Reset", count.set_to(0))
/// button("Custom", || println!("clicked"))
/// ```
pub fn button(label: &str, action: impl Fn() + 'static) -> crate::View {
    let boxed: Box<Box<dyn Fn()>> = Box::new(Box::new(action));
    let ptr = Box::into_raw(boxed) as *mut c_void;

    crate::dsl::with_ui(|ui| crate::View::new(ui.button_raw(label, trampoline, ptr)))
}

unsafe extern "C" fn trampoline(ptr: *mut c_void) {
    let action = &*(ptr as *const Box<dyn Fn()>);
    action();
}

// ═══════════════════════════════════════════════════════════════════════════
// app() — the main entry point
// ═══════════════════════════════════════════════════════════════════════════

/// Launch a reactive SwiftUI app.
///
/// ```ignore
/// use swiftui::prelude::*;
///
/// app("My App", 400.0, 300.0, |cx| {
///     let count = cx.state(0i32);
///     vstack![
///         text(&format!("{}", count.get())).size(48.0),
///         button("+1", count.bind(|n| n + 1)),
///     ]
/// });
/// ```
pub fn app(title: &str, width: f32, height: f32, build: impl Fn(&Cx) -> crate::View + 'static) {
    crate::app::init_app();
    if !crate::context::is_initialized() {
        // Try common paths
        let paths = [
            "swift_helper/libSwiftUIHelper.dylib",
            "../../swift_helper/libSwiftUIHelper.dylib",
        ];
        for p in paths {
            if std::path::Path::new(p).exists() {
                crate::init(p);
                break;
            }
        }
    }

    let store = Store::new();
    let data = Box::new(AppData {
        store: store.clone(),
        build: Box::new(build),
    });
    let data_ptr = Box::into_raw(data) as *mut c_void;

    unsafe {
        use core::ffi::c_char;
        unsafe extern "C" {
            fn dlsym(h: *mut c_void, s: *const c_char) -> *mut c_void;
            fn dlopen(p: *const c_char, m: i32) -> *mut c_void;
        }
        let paths = [
            c"swift_helper/libSwiftUIHelper.dylib".as_ptr(),
            c"../../swift_helper/libSwiftUIHelper.dylib".as_ptr(),
        ];
        for p in paths {
            dlopen(p, 2);
        }

        let f = dlsym(
            (-2isize) as *mut c_void,
            c"swiftui_reactive_window".as_ptr(),
        );
        assert!(
            !f.is_null(),
            "swiftui_reactive_window not found — build the helper"
        );

        type WinFn = unsafe extern "C" fn(
            *const u8,
            usize,
            f32,
            f32,
            unsafe extern "C" fn(*mut c_void, *mut c_void) -> *mut c_void,
            *mut c_void,
        );
        let win: WinFn = std::mem::transmute(f);
        win(
            title.as_ptr(),
            title.len(),
            width,
            height,
            app_trampoline,
            data_ptr,
        );
    }
}

struct AppData {
    pub store: Store,
    build: Box<dyn Fn(&Cx) -> crate::View>,
}

unsafe extern "C" fn app_trampoline(user_data: *mut c_void, model: *mut c_void) -> *mut c_void {
    let data = &*(user_data as *const AppData);
    data.store.set_trigger(model);

    let cx = Cx {
        store: data.store.clone(),
        next_idx: std::cell::Cell::new(0),
    };

    let view = (data.build)(&cx);
    let handle = view.into_handle();
    let raw = handle.as_raw();
    std::mem::forget(handle);
    raw
}
