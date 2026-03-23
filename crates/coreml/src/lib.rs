//! Apple Core ML — on-device machine learning from Rust.
//!
//! **Platform support:** macOS 10.13+, iOS 11+, tvOS 11+, visionOS 1+, watchOS 4+.
//!
//! Wraps Core ML for loading and running .mlmodel inference on-device.
//!
//! ```ignore
//! assert!(coreml::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"coreml_available");
