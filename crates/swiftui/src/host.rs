//! Host container configuration.
//!
//! Configure the app window before launching.
//!
//! ```ignore
//! use swiftui::prelude::*;
//!
//! App::new("My App", 800.0, 600.0)
//!     .borderless()
//!     .min_size(400.0, 300.0)
//!     .on_appear(|| println!("Window appeared"))
//!     .menu_bar_extra("Status", "star.fill", |cx| text("Menu"))
//!     .run(|cx| {
//!         vstack![text("Hello").style(StylePreset::Title)]
//!     });
//! ```

use crate::state::Cx;
use crate::View;
use core::ffi::{c_char, c_void};

unsafe extern "C" {
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
}

fn call(name: &core::ffi::CStr) -> *const c_void {
    unsafe { dlsym((-2isize) as *mut c_void, name.as_ptr()) as *const c_void }
}

/// Window style.
#[derive(Clone, Copy, Debug)]
pub enum WindowStyle {
    /// Standard titled window (default).
    Default = 0,
    /// No title bar, no chrome.
    Borderless = 1,
    /// Full screen.
    Fullscreen = 2,
    /// Floating above other windows.
    Floating = 3,
    /// Transparent background, no chrome.
    Transparent = 4,
}

/// Background material (macOS vibrancy).
#[derive(Clone, Copy, Debug)]
pub enum BackgroundMaterial {
    None = 0,
    Thin = 1,
    Regular = 2,
    Thick = 3,
    Ultra = 4,
}

/// App builder — configure and launch a SwiftUI app.
pub struct App {
    title: String,
    width: f32,
    height: f32,
    window_style: WindowStyle,
    resizable: bool,
    min_size: Option<(f32, f32)>,
    max_size: Option<(f32, f32)>,
    on_appear: Option<Box<dyn Fn()>>,
    on_disappear: Option<Box<dyn Fn()>>,
    #[cfg(target_os = "macos")]
    titlebar_hidden: bool,
    #[cfg(target_os = "macos")]
    hide_dock: bool,
    #[cfg(target_os = "macos")]
    background_material: BackgroundMaterial,
    helper_path: Option<String>,
}

impl App {
    pub fn new(title: &str, width: f32, height: f32) -> Self {
        Self {
            title: title.to_string(),
            width,
            height,
            window_style: WindowStyle::Default,
            resizable: true,
            min_size: None,
            max_size: None,
            on_appear: None,
            on_disappear: None,
            #[cfg(target_os = "macos")]
            titlebar_hidden: false,
            #[cfg(target_os = "macos")]
            hide_dock: false,
            #[cfg(target_os = "macos")]
            background_material: BackgroundMaterial::None,
            helper_path: None,
        }
    }

    /// Set the path to the Swift helper dylib.
    pub fn helper(mut self, path: &str) -> Self {
        self.helper_path = Some(path.to_string());
        self
    }

    /// Set the window style.
    pub fn window_style(mut self, style: WindowStyle) -> Self {
        self.window_style = style;
        self
    }

    /// Borderless window (no title bar).
    pub fn borderless(self) -> Self {
        self.window_style(WindowStyle::Borderless)
    }

    /// Full screen window.
    pub fn fullscreen(self) -> Self {
        self.window_style(WindowStyle::Fullscreen)
    }

    /// Floating window (always on top).
    pub fn floating(self) -> Self {
        self.window_style(WindowStyle::Floating)
    }

    /// Transparent window background.
    pub fn transparent(self) -> Self {
        self.window_style(WindowStyle::Transparent)
    }

    /// Set whether the window is resizable.
    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

    /// Fixed-size window.
    pub fn fixed_size(self) -> Self {
        self.resizable(false)
    }

    /// Set minimum window size.
    pub fn min_size(mut self, w: f32, h: f32) -> Self {
        self.min_size = Some((w, h));
        self
    }

    /// Set maximum window size.
    pub fn max_size(mut self, w: f32, h: f32) -> Self {
        self.max_size = Some((w, h));
        self
    }

    /// Callback when the window appears.
    pub fn on_appear(mut self, f: impl Fn() + 'static) -> Self {
        self.on_appear = Some(Box::new(f));
        self
    }

    /// Callback when the window disappears.
    pub fn on_disappear(mut self, f: impl Fn() + 'static) -> Self {
        self.on_disappear = Some(Box::new(f));
        self
    }

    /// Hide the title bar (macOS only).
    #[cfg(target_os = "macos")]
    pub fn titlebar_hidden(mut self) -> Self {
        self.titlebar_hidden = true;
        self
    }

    /// Hide the dock icon (macOS only — makes it a background/accessory app).
    #[cfg(target_os = "macos")]
    pub fn hide_dock(mut self) -> Self {
        self.hide_dock = true;
        self
    }

    /// Set background material/vibrancy (macOS only).
    #[cfg(target_os = "macos")]
    pub fn material(mut self, mat: BackgroundMaterial) -> Self {
        self.background_material = mat;
        self
    }

    /// Launch the app with a reactive build function.
    pub fn run(self, build: impl Fn(&Cx) -> View + 'static) {
        crate::app::init_app();

        // Auto-discover and load the helper
        let default_path = crate::loader::helper_path().to_str().unwrap().to_string();
        let helper = self.helper_path.as_deref().unwrap_or(&default_path);
        if !crate::context::is_initialized() {
            crate::init(helper);
        }
        crate::loader::ensure_loaded();

        // Apply host configuration via C calls
        self.apply_config();

        // Launch via the reactive window path
        crate::state::app(&self.title, self.width, self.height, build);
    }

    fn apply_config(&self) {
        unsafe {
            // Window style
            let f = call(c"host_set_window_style");
            if !f.is_null() {
                type F = unsafe extern "C" fn(i32);
                (std::mem::transmute::<_, F>(f))(self.window_style as i32);
            }

            // Resizable
            let f = call(c"host_set_resizable");
            if !f.is_null() {
                type F = unsafe extern "C" fn(bool);
                (std::mem::transmute::<_, F>(f))(self.resizable);
            }

            // Min size
            if let Some((w, h)) = self.min_size {
                let f = call(c"host_set_min_size");
                if !f.is_null() {
                    type F = unsafe extern "C" fn(f32, f32);
                    (std::mem::transmute::<_, F>(f))(w, h);
                }
            }

            // Max size
            if let Some((w, h)) = self.max_size {
                let f = call(c"host_set_max_size");
                if !f.is_null() {
                    type F = unsafe extern "C" fn(f32, f32);
                    (std::mem::transmute::<_, F>(f))(w, h);
                }
            }

            // onAppear
            if let Some(ref cb) = self.on_appear {
                let f = call(c"host_set_on_appear");
                if !f.is_null() {
                    type F = unsafe extern "C" fn(unsafe extern "C" fn(*mut c_void), *mut c_void);
                    let boxed: Box<Box<dyn Fn()>> = Box::new(Box::new({
                        let cb_ptr = cb as *const Box<dyn Fn()>;
                        move || (*cb_ptr)()
                    }));
                    let ptr = Box::into_raw(boxed) as *mut c_void;
                    unsafe extern "C" fn tramp(p: *mut c_void) {
                        let f = &*(p as *const Box<dyn Fn()>);
                        f();
                    }
                    (std::mem::transmute::<_, F>(f))(tramp, ptr);
                }
            }

            // macOS-specific
            #[cfg(target_os = "macos")]
            {
                if self.titlebar_hidden {
                    let f = call(c"host_set_titlebar_hidden");
                    if !f.is_null() {
                        type F = unsafe extern "C" fn(bool);
                        (std::mem::transmute::<_, F>(f))(true);
                    }
                }
                if self.hide_dock {
                    let f = call(c"host_hide_dock_icon");
                    if !f.is_null() {
                        type F = unsafe extern "C" fn(bool);
                        (std::mem::transmute::<_, F>(f))(true);
                    }
                }
                if self.background_material as i32 != 0 {
                    let f = call(c"host_set_background_material");
                    if !f.is_null() {
                        type F = unsafe extern "C" fn(i32);
                        (std::mem::transmute::<_, F>(f))(self.background_material as i32);
                    }
                }
            }
        }
    }
}
