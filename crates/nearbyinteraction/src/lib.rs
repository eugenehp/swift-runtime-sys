//! Apple NearbyInteraction — UWB ranging from Rust.
//!
//! **Platform support:** macOS 12+, iOS 14+, visionOS 1+, watchOS 8+.
//!
//! Wraps NearbyInteraction for ultra-wideband (U1 chip) spatial awareness.
//!
//! ```ignore
//! assert!(nearbyinteraction::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"nearbyinteraction_available"; "macos", "ios", "xros", "watchos");
