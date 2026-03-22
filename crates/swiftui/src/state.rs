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
    // Auto-discover and load the helper
    let helper = crate::loader::helper_path();
    if !crate::context::is_initialized() {
        crate::init(helper.to_str().unwrap());
    }
    crate::loader::ensure_loaded();

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

// ═══════════════════════════════════════════════════════════════════════════
// State<Vec<T>> convenience methods
// ═══════════════════════════════════════════════════════════════════════════

impl<T: Any + Send + Clone + 'static> State<Vec<T>> {
    /// Push an item to the end.
    pub fn push(&self, item: T) {
        self.update(|list| {
            let mut new = list.clone();
            new.push(item.clone());
            new
        });
    }

    /// Remove item at index.
    pub fn remove(&self, index: usize) {
        self.update(|list| {
            let mut new = list.clone();
            if index < new.len() {
                new.remove(index);
            }
            new
        });
    }

    /// Update item at index.
    pub fn update_at(&self, index: usize, f: impl FnOnce(&T) -> T) {
        self.update(|list| {
            let mut new = list.clone();
            if index < new.len() {
                new[index] = f(&new[index]);
            }
            new
        });
    }

    /// Get the length.
    pub fn len(&self) -> usize {
        self.get().len()
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.get().is_empty()
    }

    /// Clear all items.
    pub fn clear(&self) {
        self.set(Vec::new());
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Bound controls — two-way binding between Rust state and SwiftUI
// ═══════════════════════════════════════════════════════════════════════════

/// A text field that writes back to a State<String>.
pub fn bound_textfield(placeholder: &str, state: &State<String>) -> crate::View {
    let s = state.clone();
    let boxed: Box<Box<dyn Fn(*const u8, usize)>> = Box::new(Box::new(move |ptr, len| {
        let new =
            unsafe { String::from_utf8_lossy(std::slice::from_raw_parts(ptr, len)).into_owned() };
        s.set(new);
    }));
    let ud = Box::into_raw(boxed) as *mut c_void;

    unsafe extern "C" fn tramp(ptr: *const u8, len: usize, ud: *mut c_void) {
        let f = &*(ud as *const Box<dyn Fn(*const u8, usize)>);
        f(ptr, len);
    }

    let val = state.get();
    crate::dsl::with_ui(|ui| {
        crate::View::new(crate::handle::ViewHandle::new(
            unsafe {
                (ui.fns.bound_textfield)(
                    placeholder.as_ptr(),
                    placeholder.len(),
                    val.as_ptr(),
                    val.len(),
                    tramp,
                    ud,
                )
            },
            ui.fns.release,
        ))
    })
}

/// A toggle that writes back to a State<bool>.
pub fn bound_toggle(label: &str, state: &State<bool>) -> crate::View {
    let s = state.clone();
    let boxed: Box<Box<dyn Fn(bool)>> = Box::new(Box::new(move |val| {
        s.set(val);
    }));
    let ud = Box::into_raw(boxed) as *mut c_void;

    unsafe extern "C" fn tramp(val: bool, ud: *mut c_void) {
        let f = &*(ud as *const Box<dyn Fn(bool)>);
        f(val);
    }

    crate::dsl::with_ui(|ui| {
        crate::View::new(crate::handle::ViewHandle::new(
            unsafe { (ui.fns.bound_toggle)(label.as_ptr(), label.len(), state.get(), tramp, ud) },
            ui.fns.release,
        ))
    })
}

/// A slider that writes back to a State<f32>.
pub fn bound_slider(state: &State<f32>, min: f32, max: f32) -> crate::View {
    let s = state.clone();
    let boxed: Box<Box<dyn Fn(f32)>> = Box::new(Box::new(move |val| {
        s.set(val);
    }));
    let ud = Box::into_raw(boxed) as *mut c_void;

    unsafe extern "C" fn tramp(val: f32, ud: *mut c_void) {
        let f = &*(ud as *const Box<dyn Fn(f32)>);
        f(val);
    }

    crate::dsl::with_ui(|ui| {
        crate::View::new(crate::handle::ViewHandle::new(
            unsafe { (ui.fns.bound_slider)(state.get(), min, max, tramp, ud) },
            ui.fns.release,
        ))
    })
}

// ═══════════════════════════════════════════════════════════════════════════
// TabView / Picker — need callbacks for state binding
// ═══════════════════════════════════════════════════════════════════════════

/// Tab entry for tabview.
pub struct Tab {
    pub view: crate::View,
    pub label: String,
    pub icon: String,
}

impl Tab {
    pub fn new(label: &str, icon: &str, view: crate::View) -> Self {
        Self {
            view,
            label: label.into(),
            icon: icon.into(),
        }
    }
}

/// Create a TabView.
pub fn tabview(tabs: Vec<Tab>) -> crate::View {
    crate::dsl::with_ui(|ui| {
        let handles: Vec<_> = tabs.iter().map(|t| t.view.handle().as_raw()).collect();
        let label_ptrs: Vec<_> = tabs.iter().map(|t| t.label.as_ptr()).collect();
        let label_lens: Vec<_> = tabs.iter().map(|t| t.label.len()).collect();
        let icon_ptrs: Vec<_> = tabs.iter().map(|t| t.icon.as_ptr()).collect();
        let icon_lens: Vec<_> = tabs.iter().map(|t| t.icon.len()).collect();

        crate::View::new(crate::handle::ViewHandle::new(
            unsafe {
                (ui.fns.tabview)(
                    handles.as_ptr(),
                    label_ptrs.as_ptr(),
                    label_lens.as_ptr(),
                    icon_ptrs.as_ptr(),
                    icon_lens.as_ptr(),
                    tabs.len(),
                )
            },
            ui.fns.release,
        ))
    })
}

/// Create a bound Picker that writes back to State<i32>.
pub fn bound_picker(label: &str, options: &[&str], state: &State<i32>) -> crate::View {
    let s = state.clone();
    let boxed: Box<Box<dyn Fn(i32)>> = Box::new(Box::new(move |val| {
        s.set(val);
    }));
    let ud = Box::into_raw(boxed) as *mut c_void;
    unsafe extern "C" fn tramp(val: i32, ud: *mut c_void) {
        let f = &*(ud as *const Box<dyn Fn(i32)>);
        f(val);
    }

    let opt_ptrs: Vec<_> = options.iter().map(|s| s.as_ptr()).collect();
    let opt_lens: Vec<_> = options.iter().map(|s| s.len()).collect();

    crate::dsl::with_ui(|ui| {
        crate::View::new(crate::handle::ViewHandle::new(
            unsafe {
                (ui.fns.bound_picker)(
                    label.as_ptr(),
                    label.len(),
                    opt_ptrs.as_ptr(),
                    opt_lens.as_ptr(),
                    options.len(),
                    state.get(),
                    tramp,
                    ud,
                )
            },
            ui.fns.release,
        ))
    })
}

// ═══════════════════════════════════════════════════════════════════════════
// withAnimation — wraps a closure in a SwiftUI animation block
// ═══════════════════════════════════════════════════════════════════════════

/// Animation curve for `with_animation`.
#[derive(Clone, Copy)]
pub enum AnimCurve {
    Default = 0,
    EaseIn = 1,
    EaseOut = 2,
    EaseInOut = 3,
    Linear = 4,
    Spring = 5,
    Bouncy = 6,
}

/// Run a closure inside a SwiftUI animation block.
/// State changes made inside `f` will be animated.
///
/// ```ignore
/// with_animation(AnimCurve::Spring, 0.3, || {
///     count.set(count.get() + 1);
///     offset.set(100.0);
/// });
/// ```
pub fn with_animation(curve: AnimCurve, duration: f32, f: impl FnOnce()) {
    let boxed: Box<Box<dyn FnOnce()>> = Box::new(Box::new(f));
    let ptr = Box::into_raw(boxed) as *mut core::ffi::c_void;
    unsafe extern "C" fn tramp(p: *mut core::ffi::c_void) {
        let f = Box::from_raw(p as *mut Box<dyn FnOnce()>);
        f();
    }
    crate::dsl::with_ui(|ui| {
        unsafe { (ui.fns.with_animation)(curve as i32, duration, tramp, ptr) };
    });
}

/// Convenience: animate with default curve.
pub fn animate(f: impl FnOnce()) {
    with_animation(AnimCurve::Default, 0.3, f);
}

/// Convenience: animate with spring.
pub fn animate_spring(f: impl FnOnce()) {
    with_animation(AnimCurve::Spring, 0.5, f);
}

// ═══════════════════════════════════════════════════════════════════════════
// AppStorage — UserDefaults persistence
// ═══════════════════════════════════════════════════════════════════════════

/// Read a string from UserDefaults.
pub fn app_storage_get(key: &str) -> Option<String> {
    crate::dsl::with_ui(|ui| {
        let mut ptr: *mut core::ffi::c_void = core::ptr::null_mut();
        let mut len: usize = 0;
        let ok =
            unsafe { (ui.fns.app_storage_get_string)(key.as_ptr(), key.len(), &mut ptr, &mut len) };
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
    })
}

/// Write a string to UserDefaults.
pub fn app_storage_set(key: &str, value: &str) {
    crate::dsl::with_ui(|ui| {
        unsafe {
            (ui.fns.app_storage_set_string)(key.as_ptr(), key.len(), value.as_ptr(), value.len())
        };
    });
}

/// Read an int from UserDefaults.
pub fn app_storage_get_int(key: &str) -> isize {
    crate::dsl::with_ui(|ui| unsafe { (ui.fns.app_storage_get_int)(key.as_ptr(), key.len()) })
}

/// Write an int to UserDefaults.
pub fn app_storage_set_int(key: &str, value: isize) {
    crate::dsl::with_ui(|ui| {
        unsafe { (ui.fns.app_storage_set_int)(key.as_ptr(), key.len(), value) };
    });
}

/// Read a bool from UserDefaults.
pub fn app_storage_get_bool(key: &str) -> bool {
    crate::dsl::with_ui(|ui| unsafe { (ui.fns.app_storage_get_bool)(key.as_ptr(), key.len()) })
}

/// Write a bool to UserDefaults.
pub fn app_storage_set_bool(key: &str, value: bool) {
    crate::dsl::with_ui(|ui| {
        unsafe { (ui.fns.app_storage_set_bool)(key.as_ptr(), key.len(), value) };
    });
}
