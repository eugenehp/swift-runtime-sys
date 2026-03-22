//! Build and display SwiftUI views from Rust.
//!
//! This crate provides an ergonomic API for constructing SwiftUI view trees
//! and displaying them in macOS windows, driven entirely from Rust.
//!
//! Requires a small Swift helper dylib (`libSwiftUIHelper.dylib`) for calling
//! convention bridging. See `swift_helper/SwiftUIHelper.swift`.
//!
//! # Example
//! ```ignore
//! use swiftui::*;
//!
//! let ui = SwiftUI::load("swift_helper/libSwiftUIHelper.dylib").unwrap();
//!
//! let view = ui.vstack(&[
//!     ui.text("Hello from Rust! 🦀").bold().font_size(24.0),
//!     ui.spacer(),
//!     ui.button("Click me", || println!("clicked!")),
//! ]);
//!
//! ui.show_window(view, "My App", 400.0, 300.0);
//! ```

mod handle;
mod views;
mod app;

pub use handle::ViewHandle;
pub use views::SwiftUI;
pub use app::show_window;
