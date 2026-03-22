//! The `View` trait — chainable modifiers on any SwiftUI view.

use crate::color::Color;
use crate::context::with_ui;
use crate::handle::ViewHandle;

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

    // ── Container modifiers ──

    pub fn scroll(self) -> Self {
        with_ui(|ui| Self::new(ui.scroll(&self.handle)))
    }
}
