//! Apple SpriteKit — 2D game engine from Rust.
//!
//! **Platform support:** macOS 10.9+, iOS 7+, tvOS 9+, visionOS 1+, watchOS 3+.
//!
//! Wraps SpriteKit for 2D sprites, physics, and particle systems.
//!
//! ```ignore
//! assert!(spritekit::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"spritekit_available");
