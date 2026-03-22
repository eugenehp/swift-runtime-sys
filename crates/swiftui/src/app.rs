//! App initialization helpers.

#[link(name = "AppKit", kind = "framework")]
unsafe extern "C" {
    fn NSApplicationLoad() -> bool;
}

/// Initialize AppKit (must be called before creating any SwiftUI views).
pub fn init_app() {
    unsafe {
        NSApplicationLoad();
    }
}

/// Convenience: init app, build a view, show in window.
pub fn show_window(
    helper_path: &str,
    title: &str,
    width: f32,
    height: f32,
    build: impl FnOnce(&crate::SwiftUI) -> crate::ViewHandle,
) {
    init_app();
    let ui = crate::SwiftUI::load(helper_path).expect("Failed to load SwiftUI helper");
    let view = build(&ui);
    ui.show_window(&view, title, width, height);
}
