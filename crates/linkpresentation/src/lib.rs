//! Apple LinkPresentation — URL previews from Rust.
//!
//! **Platform support:** macOS 10.15+, iOS 13+, visionOS 1+.
//!
//! Wraps LinkPresentation for fetching rich URL metadata and preview views.
//!
//! ```ignore
//! assert!(linkpresentation::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"linkpresentation_available"; "macos", "ios", "xros");
