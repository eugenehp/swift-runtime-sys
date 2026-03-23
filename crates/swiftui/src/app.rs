//! App initialization helpers — cross-platform (macOS, iOS, tvOS, visionOS).

#[cfg(target_os = "macos")]
#[link(name = "AppKit", kind = "framework")]
unsafe extern "C" {
    fn NSApplicationLoad() -> bool;
}

#[cfg(target_os = "ios")]
#[link(name = "UIKit", kind = "framework")]
extern "C" {}

#[cfg(target_os = "tvos")]
#[link(name = "UIKit", kind = "framework")]
extern "C" {}

#[cfg(target_os = "xros")]
#[link(name = "UIKit", kind = "framework")]
extern "C" {}

/// Initialize the platform app runtime.
/// - macOS: calls `NSApplicationLoad()`
/// - iOS/tvOS/visionOS: no-op (UIKit manages the lifecycle)
pub fn init_app() {
    #[cfg(target_os = "macos")]
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
