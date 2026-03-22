//! Build and display SwiftUI views from Rust.
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

mod handle;
mod views;
mod app;
mod color;
mod view;
mod context;
pub mod dsl;

pub use handle::ViewHandle;
pub use views::SwiftUI;
pub use app::show_window;
// Color is re-exported via dsl::Color
pub use view::View;

/// Initialize SwiftUI with the helper dylib path.
/// Must be called before using any DSL functions.
pub fn init(helper_path: &str) {
    app::init_app();
    context::init(helper_path);
}
