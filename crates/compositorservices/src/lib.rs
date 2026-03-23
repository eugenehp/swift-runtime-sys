//! Apple CompositorServices — visionOS rendering from Rust.
//!
//! **Platform support:** visionOS 1+.
//!
//! Wraps CompositorServices for low-level visionOS rendering with Metal.
//!
//! ```ignore
//! assert!(compositorservices::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"compositorservices_available"; "xros");
