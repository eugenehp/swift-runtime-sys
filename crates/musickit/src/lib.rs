//! Apple MusicKit — Apple Music integration from Rust.
//!
//! **Platform support:** macOS 12+, iOS 15+, tvOS 15+, watchOS 8+.
//!
//! Wraps MusicKit for Apple Music catalog search, playback, and library access.
//!
//! ```ignore
//! assert!(musickit::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"musickit_available"; "macos", "ios", "tvos", "watchos");
