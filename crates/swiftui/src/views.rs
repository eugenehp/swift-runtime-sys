//! SwiftUI view construction — wraps swiftui-sys with ergonomic API.

use crate::handle::ViewHandle;
use core::ffi::c_void;

/// The SwiftUI bridge — backed by swiftui-sys function pointers.
pub struct SwiftUI {
    pub(crate) fns: swiftui_sys::Fns,
}

impl SwiftUI {
    /// Load the Swift helper dylib.
    pub fn load(helper_path: &str) -> Result<Self, String> {
        Ok(Self {
            fns: swiftui_sys::load(helper_path)?,
        })
    }

    fn handle(&self, ptr: *mut c_void) -> ViewHandle {
        ViewHandle::new(ptr, self.fns.release)
    }

    // ── Views ──

    pub fn text(&self, s: &str) -> ViewHandle {
        self.handle(unsafe { (self.fns.text)(s.as_ptr(), s.len()) })
    }

    pub fn styled_text(
        &self,
        s: &str,
        size: f32,
        weight: i32,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    ) -> ViewHandle {
        self.handle(unsafe {
            (self.fns.styled_text)(s.as_ptr(), s.len(), size, weight, r, g, b, a)
        })
    }

    pub fn bold_text(&self, s: &str, size: f32) -> ViewHandle {
        self.styled_text(s, size, 1, 1.0, 1.0, 1.0, 1.0)
    }

    pub fn system_image(&self, name: &str) -> ViewHandle {
        self.handle(unsafe { (self.fns.system_image)(name.as_ptr(), name.len()) })
    }

    pub fn spacer(&self) -> ViewHandle {
        self.handle(unsafe { (self.fns.spacer)() })
    }

    pub fn divider(&self) -> ViewHandle {
        self.handle(unsafe { (self.fns.divider)() })
    }

    pub fn color(&self, r: f32, g: f32, b: f32, a: f32) -> ViewHandle {
        self.handle(unsafe { (self.fns.color)(r, g, b, a) })
    }

    pub fn progress(&self, value: f32, total: f32) -> ViewHandle {
        self.handle(unsafe { (self.fns.progress)(value, total) })
    }

    pub fn toggle(&self, label: &str, is_on: bool) -> ViewHandle {
        self.handle(unsafe { (self.fns.toggle)(label.as_ptr(), label.len(), is_on) })
    }

    pub fn textfield(&self, placeholder: &str, value: &str) -> ViewHandle {
        self.handle(unsafe {
            (self.fns.textfield)(
                placeholder.as_ptr(),
                placeholder.len(),
                value.as_ptr(),
                value.len(),
            )
        })
    }

    // ── Stacks ──

    pub fn vstack(&self, children: &[ViewHandle]) -> ViewHandle {
        let ptrs: Vec<*mut c_void> = children.iter().map(|h| h.as_raw()).collect();
        self.handle(unsafe { (self.fns.vstack)(ptrs.as_ptr(), ptrs.len()) })
    }

    pub fn hstack(&self, children: &[ViewHandle]) -> ViewHandle {
        let ptrs: Vec<*mut c_void> = children.iter().map(|h| h.as_raw()).collect();
        self.handle(unsafe { (self.fns.hstack)(ptrs.as_ptr(), ptrs.len()) })
    }

    pub fn zstack(&self, children: &[ViewHandle]) -> ViewHandle {
        let ptrs: Vec<*mut c_void> = children.iter().map(|h| h.as_raw()).collect();
        self.handle(unsafe { (self.fns.zstack)(ptrs.as_ptr(), ptrs.len()) })
    }

    pub fn scroll(&self, content: &ViewHandle) -> ViewHandle {
        self.handle(unsafe { (self.fns.scroll_view)(content.as_raw()) })
    }

    // ── Modifiers ──

    pub fn padding(&self, view: &ViewHandle, amount: f32) -> ViewHandle {
        self.handle(unsafe { (self.fns.padding)(view.as_raw(), amount) })
    }

    pub fn frame(&self, view: &ViewHandle, w: f32, h: f32) -> ViewHandle {
        self.handle(unsafe { (self.fns.frame)(view.as_raw(), w, h) })
    }

    pub fn background(&self, view: &ViewHandle, r: f32, g: f32, b: f32, a: f32) -> ViewHandle {
        self.handle(unsafe { (self.fns.background_color)(view.as_raw(), r, g, b, a) })
    }

    pub fn corner_radius(&self, view: &ViewHandle, radius: f32) -> ViewHandle {
        self.handle(unsafe { (self.fns.corner_radius)(view.as_raw(), radius) })
    }

    pub fn opacity(&self, view: &ViewHandle, value: f32) -> ViewHandle {
        self.handle(unsafe { (self.fns.opacity)(view.as_raw(), value) })
    }

    pub fn border(&self, view: &ViewHandle, r: f32, g: f32, b: f32, width: f32) -> ViewHandle {
        self.handle(unsafe { (self.fns.border)(view.as_raw(), r, g, b, width) })
    }

    // ── Button ──

    pub fn button_raw(
        &self,
        label: &str,
        callback: unsafe extern "C" fn(*mut c_void),
        userdata: *mut c_void,
    ) -> ViewHandle {
        self.handle(unsafe { (self.fns.button)(label.as_ptr(), label.len(), callback, userdata) })
    }

    pub fn button(&self, label: &str, callback: fn()) -> ViewHandle {
        unsafe extern "C" fn trampoline(ptr: *mut c_void) {
            let f: fn() = std::mem::transmute(ptr);
            f();
        }
        self.handle(unsafe {
            (self.fns.button)(
                label.as_ptr(),
                label.len(),
                trampoline,
                callback as *mut c_void,
            )
        })
    }

    // ── Window ──

    pub fn show_window(&self, view: &ViewHandle, title: &str, width: f32, height: f32) {
        unsafe { (self.fns.show_window)(view.as_raw(), title.as_ptr(), title.len(), width, height) }
    }
}
