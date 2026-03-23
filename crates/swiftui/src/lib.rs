//! Build and display SwiftUI views from Rust.
//!
//! **Platform support:** macOS 14+, iOS 17+, tvOS 17+, visionOS 1+.
//!
//! # Quick start
//!
//! ```ignore
//! use swiftui::dsl::*;
//!
//! swiftui::init("swift_helper/libSwiftUIHelper.dylib");
//!
//! window("My App", 400.0, 300.0,
//!     vstack![
//!         text("Hello from Rust!").bold().size(24),
//!         spacer(),
//!         button("Click me", || println!("clicked!")),
//!     ].padding(16).bg(Color::DARKER)
//! );
//! ```
//!
//! # Platform notes
//!
//! - **macOS**: Uses `NSApplicationLoad()` for initialization, supports
//!   titlebar customization, dock icon hiding, and vibrancy materials.
//! - **iOS**: UIKit manages the app lifecycle; use in an Xcode project
//!   with a Swift `@main` entry point that delegates to Rust.
//! - **tvOS**: Focus-based navigation; `button()` and `list()` respond
//!   to the Siri Remote. Touch gestures are not available.
//! - **visionOS**: Supports volumetric windows and immersive spaces via
//!   `WindowStyle::Volumetric` and `WindowStyle::ImmersiveSpace`.

mod app;
pub mod canvas;
mod color;
pub mod conditional;
mod context;
pub mod dsl;
mod handle;
pub mod host;
pub mod loader;
pub mod nav;
pub mod prelude;
pub mod scene;
pub mod state;
pub mod style;
mod view;
mod views;

pub use app::show_window;
pub use handle::ViewHandle;
pub use views::SwiftUI;
// Color is re-exported via dsl::Color
pub use view::{FontWeight, View};

/// Initialize SwiftUI with the helper dylib path.
/// Must be called before using any DSL functions.
pub fn init(helper_path: &str) {
    app::init_app();
    context::init(helper_path);
}
