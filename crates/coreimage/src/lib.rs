//! Apple Core Image — image processing and filters from Rust.
//!
//! **Platform support:** macOS 10.4+, iOS 5+, tvOS 9+, visionOS 1+.
//!
//! Wraps Core Image for image filters, face detection, and GPU-accelerated processing.
//!
//! ```ignore
//! assert!(coreimage::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"coreimage_available"; "macos", "ios", "tvos", "xros");
