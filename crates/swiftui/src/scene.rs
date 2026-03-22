//! Scene-based app lifecycle — Swift owns `@main`, Rust provides content.
//!
//! ```ignore
//! use swiftui::prelude::*;
//!
//! SceneApp::new()
//!     .window("main", "My App", 800.0, 600.0, |cx| {
//!         vstack![text("Hello").style(StylePreset::Title)]
//!     })
//!     .settings("Preferences", |cx| {
//!         vstack![toggle("Dark mode", true)]
//!     })
//!     .menu_bar("Status", "star.fill", |cx| {
//!         vstack![text("Menu content")]
//!     })
//!     .launch();
//! ```

use crate::state::Cx;
use crate::View;
use core::ffi::{c_char, c_void};

unsafe extern "C" {
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
}

fn sym(name: &core::ffi::CStr) -> *const c_void {
    unsafe { dlsym((-2isize) as *mut c_void, name.as_ptr()) as *const c_void }
}

struct SceneData {
    store: crate::state::Store,
    build: Box<dyn Fn(&Cx) -> View>,
}

/// Scene-based app builder.
///
/// Unlike `app()` which uses `NSWindow`/`UIWindow` directly, `SceneApp`
/// uses Swift's native `App` protocol with `Scene` builders. This enables:
/// - `WindowGroup` (standard windows)
/// - `Settings` (macOS preferences window)
/// - `MenuBarExtra` (macOS status bar)
/// - Multiple named window groups
/// - Future: `ImmersiveSpace` (visionOS), `DocumentGroup`
pub struct SceneApp {
    scenes: Vec<SceneEntry>,
    helper_path: Option<String>,
}

struct SceneEntry {
    id: String,
    title: String,
    kind: SceneKind,
    build: Box<dyn Fn(&Cx) -> View + 'static>,
    width: f32,
    height: f32,
    image: String,
}

#[derive(Clone, Copy)]
enum SceneKind {
    WindowGroup,
    Settings,
    MenuBarExtra,
}

impl SceneApp {
    pub fn new() -> Self {
        Self {
            scenes: Vec::new(),
            helper_path: None,
        }
    }

    /// Set the Swift helper dylib path.
    pub fn helper(mut self, path: &str) -> Self {
        self.helper_path = Some(path.to_string());
        self
    }

    /// Add a window group scene.
    pub fn window(
        mut self,
        id: &str,
        title: &str,
        width: f32,
        height: f32,
        build: impl Fn(&Cx) -> View + 'static,
    ) -> Self {
        self.scenes.push(SceneEntry {
            id: id.to_string(),
            title: title.to_string(),
            kind: SceneKind::WindowGroup,
            build: Box::new(build),
            width,
            height,
            image: String::new(),
        });
        self
    }

    /// Add a settings scene (macOS only).
    pub fn settings(mut self, title: &str, build: impl Fn(&Cx) -> View + 'static) -> Self {
        self.scenes.push(SceneEntry {
            id: "settings".to_string(),
            title: title.to_string(),
            kind: SceneKind::Settings,
            build: Box::new(build),
            width: 400.0,
            height: 300.0,
            image: String::new(),
        });
        self
    }

    /// Add a menu bar extra (macOS status bar item).
    pub fn menu_bar(
        mut self,
        title: &str,
        system_image: &str,
        build: impl Fn(&Cx) -> View + 'static,
    ) -> Self {
        self.scenes.push(SceneEntry {
            id: "menubar".to_string(),
            title: title.to_string(),
            kind: SceneKind::MenuBarExtra,
            build: Box::new(build),
            width: 300.0,
            height: 400.0,
            image: system_image.to_string(),
        });
        self
    }

    /// Launch the app. This blocks until the app exits.
    pub fn launch(self) {
        crate::app::init_app();

        // Auto-discover and load the helper
        let default_path = crate::loader::helper_path().to_str().unwrap().to_string();
        let helper = self.helper_path.as_deref().unwrap_or(&default_path);
        if !crate::context::is_initialized() {
            crate::init(helper);
        }
        crate::loader::ensure_loaded();

        // Register each scene
        for _entry in &self.scenes {
            let store = crate::state::Store::new();
            let data = Box::new(SceneData {
                store,
                build: Box::new({
                    // We need to move the build fn into a new box that the trampoline can call
                    // This is tricky because entry.build is behind &self
                    // We'll use a double-indirection via leaked pointer
                    let _placeholder = (); // handled below
                    move |_cx: &Cx| -> View { crate::dsl::text("placeholder").into() }
                }),
            });
            let _data_ptr = Box::into_raw(data) as *mut c_void;
            // Registration handled below per-kind
        }

        // Register scenes with the Swift side
        for entry in self.scenes {
            let store = crate::state::Store::new();
            let data = Box::new(SceneData {
                store,
                build: entry.build,
            });
            let data_ptr = Box::into_raw(data) as *mut c_void;

            match entry.kind {
                SceneKind::WindowGroup => {
                    let f = sym(c"scene_register_window");
                    if !f.is_null() {
                        type F = unsafe extern "C" fn(
                            *const u8,
                            usize,
                            *const u8,
                            usize,
                            f32,
                            f32,
                            unsafe extern "C" fn(*mut c_void, *mut c_void) -> *mut c_void,
                            *mut c_void,
                        );
                        unsafe {
                            (std::mem::transmute::<_, F>(f))(
                                entry.id.as_ptr(),
                                entry.id.len(),
                                entry.title.as_ptr(),
                                entry.title.len(),
                                entry.width,
                                entry.height,
                                scene_trampoline,
                                data_ptr,
                            );
                        }
                    }
                }
                SceneKind::Settings => {
                    let f = sym(c"scene_register_settings");
                    if !f.is_null() {
                        type F = unsafe extern "C" fn(
                            *const u8,
                            usize,
                            unsafe extern "C" fn(*mut c_void, *mut c_void) -> *mut c_void,
                            *mut c_void,
                        );
                        unsafe {
                            (std::mem::transmute::<_, F>(f))(
                                entry.title.as_ptr(),
                                entry.title.len(),
                                scene_trampoline,
                                data_ptr,
                            );
                        }
                    }
                }
                SceneKind::MenuBarExtra => {
                    let f = sym(c"scene_register_menu_bar");
                    if !f.is_null() {
                        type F = unsafe extern "C" fn(
                            *const u8,
                            usize,
                            *const u8,
                            usize,
                            unsafe extern "C" fn(*mut c_void, *mut c_void) -> *mut c_void,
                            *mut c_void,
                        );
                        unsafe {
                            (std::mem::transmute::<_, F>(f))(
                                entry.title.as_ptr(),
                                entry.title.len(),
                                entry.image.as_ptr(),
                                entry.image.len(),
                                scene_trampoline,
                                data_ptr,
                            );
                        }
                    }
                }
            }
        }

        // Launch the Swift app
        let launch = sym(c"scene_launch");
        if !launch.is_null() {
            type F = unsafe extern "C" fn();
            unsafe { (std::mem::transmute::<_, F>(launch))() };
        } else {
            panic!("scene_launch not found — compile AppHost.swift into the helper");
        }
    }
}

unsafe extern "C" fn scene_trampoline(
    user_data: *mut c_void,
    model_handle: *mut c_void,
) -> *mut c_void {
    let data = &*(user_data as *const SceneData);
    data.store.set_trigger(model_handle);

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
