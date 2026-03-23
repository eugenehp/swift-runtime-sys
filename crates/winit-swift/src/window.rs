//! Window creation and management.

use crate::{fns, Handle, WindowId, Theme};

bitflags::bitflags! {
    /// Window creation flags.
    #[derive(Debug, Clone, Copy)]
    pub struct WindowFlags: u64 {
        const TRANSPARENT       = 0b0001;
        const TITLEBAR_HIDDEN   = 0b0010;
        const FULLSIZE_CONTENT  = 0b0100;
        const HDR               = 0b1000;
    }
}

/// Window creation attributes.
#[derive(Debug, Clone)]
pub struct WindowAttributes {
    pub title: String,
    pub width: f64,
    pub height: f64,
    pub flags: WindowFlags,
}

impl WindowAttributes {
    pub fn new(title: &str, width: f64, height: f64) -> Self {
        Self { title: title.into(), width, height, flags: WindowFlags::empty() }
    }
    pub fn transparent(mut self) -> Self { self.flags |= WindowFlags::TRANSPARENT; self }
    pub fn titlebar_hidden(mut self) -> Self { self.flags |= WindowFlags::TITLEBAR_HIDDEN; self }
    pub fn fullsize_content(mut self) -> Self { self.flags |= WindowFlags::FULLSIZE_CONTENT; self }
    pub fn hdr(mut self) -> Self { self.flags |= WindowFlags::HDR; self }
}

/// Window level (z-order).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowLevel {
    Normal = 0,
    Floating = 1,
    ModalPanel = 2,
}

/// A platform window with a Metal-backed surface.
pub struct Window {
    id: WindowId,
}

impl Window {
    pub(crate) fn new(attrs: WindowAttributes) -> Self {
        let id = unsafe {
            (fns().create_window)(attrs.title.as_ptr(), attrs.title.len(), attrs.width, attrs.height, attrs.flags.bits())
        };
        Window { id: WindowId(id) }
    }

    // ── Identity ──

    pub fn id(&self) -> WindowId { self.id }

    // ── Getters ──

    pub fn metal_layer(&self) -> Option<MetalLayer> {
        let h = unsafe { (fns().window_metal_layer)(self.id.0) };
        if h.is_null() { None } else { Some(MetalLayer { handle: h }) }
    }

    pub fn title(&self) -> String {
        let mut buf = vec![0u8; 512];
        let len = unsafe { (fns().window_title)(self.id.0, buf.as_mut_ptr(), buf.len()) };
        String::from_utf8_lossy(&buf[..len]).to_string()
    }

    pub fn surface_size(&self) -> (u32, u32) {
        let (mut w, mut h) = (0u32, 0u32);
        unsafe { (fns().window_size)(self.id.0, &mut w, &mut h) };
        (w, h)
    }

    pub fn scale_factor(&self) -> f64 {
        unsafe { (fns().window_scale_factor)(self.id.0) }
    }

    pub fn outer_position(&self) -> (i32, i32) {
        let (mut x, mut y) = (0i32, 0i32);
        unsafe { (fns().window_outer_position)(self.id.0, &mut x, &mut y) };
        (x, y)
    }

    pub fn outer_size(&self) -> (u32, u32) {
        let (mut w, mut h) = (0u32, 0u32);
        unsafe { (fns().window_outer_size)(self.id.0, &mut w, &mut h) };
        (w, h)
    }

    pub fn safe_area(&self) -> (u32, u32, u32, u32) {
        let (mut t, mut l, mut b, mut r) = (0u32, 0u32, 0u32, 0u32);
        unsafe { (fns().window_safe_area)(self.id.0, &mut t, &mut l, &mut b, &mut r) };
        (t, l, b, r)
    }

    pub fn theme(&self) -> Theme {
        if unsafe { (fns().window_theme)(self.id.0) } == 1 { Theme::Dark } else { Theme::Light }
    }

    pub fn has_focus(&self) -> bool { unsafe { (fns().window_has_focus)(self.id.0) } }
    pub fn is_visible(&self) -> bool { unsafe { (fns().window_is_visible)(self.id.0) } }
    pub fn is_minimized(&self) -> bool { unsafe { (fns().window_is_minimized)(self.id.0) } }
    pub fn is_maximized(&self) -> bool { unsafe { (fns().window_is_maximized)(self.id.0) } }
    pub fn is_fullscreen(&self) -> bool { unsafe { (fns().window_is_fullscreen)(self.id.0) } }
    pub fn is_resizable(&self) -> bool { unsafe { (fns().window_is_resizable)(self.id.0) } }
    pub fn is_decorated(&self) -> bool { unsafe { (fns().window_is_decorated)(self.id.0) } }

    pub fn raw_view_handle(&self) -> *mut std::ffi::c_void {
        unsafe { (fns().window_raw_handle)(self.id.0) }
    }

    // ── Setters ──

    pub fn set_title(&self, title: &str) {
        unsafe { (fns().window_set_title)(self.id.0, title.as_ptr(), title.len()) };
    }

    pub fn set_visible(&self, v: bool) { unsafe { (fns().window_set_visible)(self.id.0, v) }; }
    pub fn set_fullscreen(&self, v: bool) { unsafe { (fns().window_set_fullscreen)(self.id.0, v) }; }
    pub fn set_minimized(&self, v: bool) { unsafe { (fns().window_set_minimized)(self.id.0, v) }; }
    pub fn set_maximized(&self, v: bool) { unsafe { (fns().window_set_maximized)(self.id.0, v) }; }
    pub fn set_resizable(&self, v: bool) { unsafe { (fns().window_set_resizable)(self.id.0, v) }; }
    pub fn set_decorations(&self, v: bool) { unsafe { (fns().window_set_decorations)(self.id.0, v) }; }
    pub fn set_transparent(&self, v: bool) { unsafe { (fns().window_set_transparent)(self.id.0, v) }; }
    pub fn set_blur(&self, v: bool) { unsafe { (fns().window_set_blur)(self.id.0, v) }; }
    pub fn set_content_protected(&self, v: bool) { unsafe { (fns().window_set_content_protected)(self.id.0, v) }; }

    pub fn set_outer_position(&self, x: i32, y: i32) {
        unsafe { (fns().window_set_outer_position)(self.id.0, x, y) };
    }

    pub fn set_min_size(&self, w: f64, h: f64) { unsafe { (fns().window_set_min_size)(self.id.0, w, h) }; }
    pub fn set_max_size(&self, w: f64, h: f64) { unsafe { (fns().window_set_max_size)(self.id.0, w, h) }; }

    pub fn set_window_level(&self, level: WindowLevel) {
        unsafe { (fns().window_set_window_level)(self.id.0, level as i32) };
    }

    /// Set theme: `None` = follow system, `Some(Theme::Dark)` / `Some(Theme::Light)`.
    pub fn set_theme(&self, theme: Option<Theme>) {
        let v = match theme {
            None => -1i8,
            Some(Theme::Light) => 0,
            Some(Theme::Dark) => 1,
        };
        unsafe { (fns().window_set_theme)(self.id.0, v) };
    }

    pub fn set_cursor_visible(&self, v: bool) { unsafe { (fns().window_set_cursor_visible)(self.id.0, v) }; }

    pub fn set_cursor_position(&self, x: f64, y: f64) {
        unsafe { (fns().window_set_cursor_position)(self.id.0, x, y) };
    }

    pub fn request_redraw(&self) { unsafe { (fns().window_request_redraw)(self.id.0) }; }

    pub fn request_user_attention(&self, critical: bool) {
        unsafe { (fns().window_request_attention)(self.id.0, critical) };
    }

    pub fn focus(&self) { unsafe { (fns().window_focus)(self.id.0) }; }

    pub fn drag_window(&self) { unsafe { (fns().window_drag)(self.id.0) }; }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe { (fns().destroy_window)(self.id.0) };
    }
}

impl raw_window_handle::HasWindowHandle for Window {
    fn window_handle(&self) -> Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError> {
        let ptr = std::ptr::NonNull::new(self.raw_view_handle())
            .ok_or(raw_window_handle::HandleError::Unavailable)?;
        let raw = raw_window_handle::RawWindowHandle::AppKit(raw_window_handle::AppKitWindowHandle::new(ptr));
        Ok(unsafe { raw_window_handle::WindowHandle::borrow_raw(raw) })
    }
}

impl raw_window_handle::HasDisplayHandle for Window {
    fn display_handle(&self) -> Result<raw_window_handle::DisplayHandle<'_>, raw_window_handle::HandleError> {
        let raw = raw_window_handle::RawDisplayHandle::AppKit(raw_window_handle::AppKitDisplayHandle::new());
        Ok(unsafe { raw_window_handle::DisplayHandle::borrow_raw(raw) })
    }
}

// ── Metal Layer ─────────────────────────────────────────────────────────────

/// A CAMetalLayer attached to a window.
pub struct MetalLayer {
    pub(crate) handle: Handle,
}

impl MetalLayer {
    pub fn set_vsync(&self, enabled: bool) { unsafe { (fns().metal_layer_set_vsync)(self.handle, enabled) }; }
    pub fn set_pixel_format(&self, format: u64) { unsafe { (fns().metal_layer_set_pixel_format)(self.handle, format) }; }
    pub fn set_max_drawables(&self, count: usize) { unsafe { (fns().metal_layer_set_drawable_count)(self.handle, count) }; }
    pub fn set_hdr(&self, enabled: bool) { unsafe { (fns().metal_layer_set_hdr)(self.handle, enabled) }; }

    pub fn next_drawable(&self) -> Option<MetalDrawable> {
        let h = unsafe { (fns().metal_next_drawable)(self.handle) };
        if h.is_null() { None } else { Some(MetalDrawable { handle: h }) }
    }

    pub fn raw(&self) -> Handle { self.handle }
}

/// A drawable surface from a CAMetalLayer.
pub struct MetalDrawable {
    pub(crate) handle: Handle,
}

impl MetalDrawable {
    pub fn texture(&self) -> Handle { unsafe { (fns().metal_drawable_texture)(self.handle) } }
    pub fn raw(&self) -> Handle { self.handle }
}

impl Drop for MetalDrawable {
    fn drop(&mut self) {
        unsafe { (fns().metal_release)(self.handle) };
    }
}
