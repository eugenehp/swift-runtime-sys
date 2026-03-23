//! Apple PhotosUI — photo picker from Rust.
//!
//! **Platform support:** macOS 13+, iOS 14+, visionOS 1+, watchOS 9+.
//!
//! Wraps PhotosUI for PHPickerViewController and editing extensions.
//!
//! ```ignore
//! assert!(photosui::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"photosui_available"; "macos", "ios", "xros", "watchos");
