//! Apple MetalKit — Metal utilities from Rust.
//!
//! **Platform support:** macOS 10.11+, iOS 9+, tvOS 9+, visionOS 1+.
//!
//! Wraps MetalKit for MTKView, texture loading, and model I/O integration.
//!
//! ```ignore
//! assert!(metalkit::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"metalkit_available"; "macos", "ios", "tvos", "xros");
