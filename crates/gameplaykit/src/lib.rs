//! Apple GameplayKit — game logic from Rust.
//!
//! **Platform support:** macOS 10.11+, iOS 9+, tvOS 9+, visionOS 1+.
//!
//! Wraps GameplayKit for pathfinding, AI state machines, and random sources.
//!
//! ```ignore
//! assert!(gameplaykit::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"gameplaykit_available"; "macos", "ios", "tvos", "xros");
