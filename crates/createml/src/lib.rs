//! Apple Create ML — train machine learning models from Rust.
//!
//! **Platform support:** macOS 10.15+, iOS 15+.
//!
//! Wraps Create ML for training image classifiers, text classifiers, and more.
//!
//! ```ignore
//! assert!(createml::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"createml_available"; "macos", "ios");
