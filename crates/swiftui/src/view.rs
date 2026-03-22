//! The `View` trait — chainable modifiers on any SwiftUI view.

use crate::color::Color;
use crate::context::with_ui;
pub use crate::handle::ViewHandle;

/// A SwiftUI view with chainable modifiers.
///
/// Every view constructor returns a `View`, and modifiers return a new `View`.
///
/// ```ignore
/// text("Hello").bold().size(24).padding(16).bg(Color::DARK).rounded(8)
/// ```
pub struct View {
    pub(crate) handle: ViewHandle,
}

impl View {
    pub(crate) fn new(handle: ViewHandle) -> Self {
        Self { handle }
    }

    /// Get the raw handle (for stack assembly).
    pub fn handle(&self) -> &ViewHandle {
        &self.handle
    }

    /// Consume and return the handle.
    pub fn into_handle(self) -> ViewHandle {
        self.handle
    }

    // ── Layout modifiers ──

    pub fn padding(self, amount: f32) -> Self {
        with_ui(|ui| Self::new(ui.padding(&self.handle, amount)))
    }

    pub fn frame(self, w: f32, h: f32) -> Self {
        with_ui(|ui| Self::new(ui.frame(&self.handle, w, h)))
    }

    pub fn frame_max(self) -> Self {
        self.frame(-1.0, -1.0)
    }

    /// Convenience: apply padding, bg, and corner radius in one call.
    pub fn card(self, padding: f32, bg: crate::color::Color, radius: f32) -> Self {
        self.padding(padding).bg(bg).rounded(radius)
    }

    // ── Appearance modifiers ──

    pub fn bg(self, color: Color) -> Self {
        with_ui(|ui| Self::new(ui.background(&self.handle, color.r, color.g, color.b, color.a)))
    }

    pub fn rounded(self, radius: f32) -> Self {
        with_ui(|ui| Self::new(ui.corner_radius(&self.handle, radius)))
    }

    pub fn opacity(self, value: f32) -> Self {
        with_ui(|ui| Self::new(ui.opacity(&self.handle, value)))
    }

    pub fn border(self, color: Color, width: f32) -> Self {
        with_ui(|ui| Self::new(ui.border(&self.handle, color.r, color.g, color.b, width)))
    }

    pub fn foreground(self, color: Color) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe {
                    (ui.fns.foreground_color)(
                        self.handle.as_raw(),
                        color.r,
                        color.g,
                        color.b,
                        color.a,
                    )
                },
                ui.fns.release,
            ))
        })
    }

    pub fn shadow(self, color: Color, radius: f32, x: f32, y: f32) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe {
                    (ui.fns.shadow)(
                        self.handle.as_raw(),
                        color.r,
                        color.g,
                        color.b,
                        radius,
                        x,
                        y,
                    )
                },
                ui.fns.release,
            ))
        })
    }

    pub fn offset(self, x: f32, y: f32) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.offset)(self.handle.as_raw(), x, y) },
                ui.fns.release,
            ))
        })
    }

    pub fn scale(self, factor: f32) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.scale)(self.handle.as_raw(), factor) },
                ui.fns.release,
            ))
        })
    }

    pub fn rotation(self, degrees: f32) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.rotation)(self.handle.as_raw(), degrees) },
                ui.fns.release,
            ))
        })
    }

    pub fn hidden(self) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.hidden)(self.handle.as_raw()) },
                ui.fns.release,
            ))
        })
    }

    pub fn disabled(self, disabled: bool) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.disabled)(self.handle.as_raw(), disabled) },
                ui.fns.release,
            ))
        })
    }

    pub fn overlay(self, overlay: View) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.overlay)(self.handle.as_raw(), overlay.handle.as_raw()) },
                ui.fns.release,
            ))
        })
    }

    pub fn clip_circle(self) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.clip_circle)(self.handle.as_raw()) },
                ui.fns.release,
            ))
        })
    }

    pub fn font(self, size: f32, weight: FontWeight) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.font_system)(self.handle.as_raw(), size, weight as i32) },
                ui.fns.release,
            ))
        })
    }

    // ── Gestures ──

    /// Tap gesture with a closure.
    pub fn on_tap(self, action: impl Fn() + 'static) -> Self {
        let boxed: Box<Box<dyn Fn()>> = Box::new(Box::new(action));
        let ptr = Box::into_raw(boxed) as *mut core::ffi::c_void;
        unsafe extern "C" fn tramp(p: *mut core::ffi::c_void) {
            let f = &*(p as *const Box<dyn Fn()>);
            f();
        }
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.on_tap)(self.handle.as_raw(), tramp, ptr) },
                ui.fns.release,
            ))
        })
    }

    /// Long press gesture with a closure.
    pub fn on_long_press(self, action: impl Fn() + 'static) -> Self {
        let boxed: Box<Box<dyn Fn()>> = Box::new(Box::new(action));
        let ptr = Box::into_raw(boxed) as *mut core::ffi::c_void;
        unsafe extern "C" fn tramp(p: *mut core::ffi::c_void) {
            let f = &*(p as *const Box<dyn Fn()>);
            f();
        }
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.on_long_press)(self.handle.as_raw(), tramp, ptr) },
                ui.fns.release,
            ))
        })
    }

    // ── Visual effects ──

    pub fn blur(self, radius: f32) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.blur)(self.handle.as_raw(), radius) },
                ui.fns.release,
            ))
        })
    }

    pub fn brightness(self, amount: f32) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.brightness)(self.handle.as_raw(), amount) },
                ui.fns.release,
            ))
        })
    }

    pub fn saturation(self, amount: f32) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.saturation)(self.handle.as_raw(), amount) },
                ui.fns.release,
            ))
        })
    }

    pub fn grayscale(self, amount: f32) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.grayscale)(self.handle.as_raw(), amount) },
                ui.fns.release,
            ))
        })
    }

    // ── Text layout ──

    pub fn line_limit(self, limit: i32) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.line_limit)(self.handle.as_raw(), limit) },
                ui.fns.release,
            ))
        })
    }

    pub fn fixed_size_mod(self) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.fixed_size_mod)(self.handle.as_raw()) },
                ui.fns.release,
            ))
        })
    }

    // ── Layout ──

    pub fn aspect_ratio(self, ratio: f32, fit: bool) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe {
                    (ui.fns.aspect_ratio)(self.handle.as_raw(), ratio, if fit { 0 } else { 1 })
                },
                ui.fns.release,
            ))
        })
    }

    pub fn clipped(self) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.clipped)(self.handle.as_raw()) },
                ui.fns.release,
            ))
        })
    }

    // ── Misc ──

    pub fn tint(self, c: Color) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.tint)(self.handle.as_raw(), c.r, c.g, c.b) },
                ui.fns.release,
            ))
        })
    }

    pub fn badge(self, count: i32) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.badge)(self.handle.as_raw(), count) },
                ui.fns.release,
            ))
        })
    }

    pub fn help(self, text: &str) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.help_text)(self.handle.as_raw(), text.as_ptr(), text.len()) },
                ui.fns.release,
            ))
        })
    }

    // ── Animation ──

    pub fn animated(self) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.animation)(self.handle.as_raw(), 0) },
                ui.fns.release,
            ))
        })
    }

    pub fn spring(self) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.animation)(self.handle.as_raw(), 4) },
                ui.fns.release,
            ))
        })
    }

    pub fn bouncy(self) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe { (ui.fns.animation)(self.handle.as_raw(), 5) },
                ui.fns.release,
            ))
        })
    }

    // ── Presentation ──

    pub fn sheet(self, content: View, is_presented: bool) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe {
                    (ui.fns.sheet)(self.handle.as_raw(), content.handle.as_raw(), is_presented)
                },
                ui.fns.release,
            ))
        })
    }

    pub fn alert(self, title: &str, message: &str, is_presented: bool) -> Self {
        with_ui(|ui| {
            Self::new(ViewHandle::new(
                unsafe {
                    (ui.fns.alert)(
                        self.handle.as_raw(),
                        title.as_ptr(),
                        title.len(),
                        message.as_ptr(),
                        message.len(),
                        is_presented,
                    )
                },
                ui.fns.release,
            ))
        })
    }

    // ── Container modifiers ──

    pub fn scroll(self) -> Self {
        with_ui(|ui| Self::new(ui.scroll(&self.handle)))
    }
}

/// Font weight for the `.font()` modifier.
#[repr(i32)]
#[derive(Clone, Copy, Debug)]
pub enum FontWeight {
    Regular = 0,
    Bold = 1,
    Semibold = 2,
    Heavy = 3,
    Light = 4,
    Thin = 5,
    Medium = 6,
}
