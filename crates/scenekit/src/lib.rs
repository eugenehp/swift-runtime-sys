//! Apple SceneKit — 3D rendering from Rust.
//!
//! **Platform support:** macOS 10.8+, iOS 8+, tvOS 9+, visionOS 1+, watchOS 3+.
//!
//! Wraps SceneKit for 3D scene graphs, physics, and rendering.
//!
//! ```ignore
//! assert!(scenekit::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"scenekit_available");
