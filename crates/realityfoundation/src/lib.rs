//! Apple RealityFoundation — RealityKit foundation types from Rust.
//!
//! **Platform support:** macOS 15+, iOS 18+, visionOS 2+.
//!
//! ```ignore
//! assert!(realityfoundation::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"realityfoundation_available"; "macos", "ios", "xros");
