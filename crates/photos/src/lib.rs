//! Apple Photos — photo library access from Rust.
//!
//! **Platform support:** macOS 10.13+, iOS 8+, tvOS 10+, visionOS 1+.
//!
//! Wraps Photos for fetching, caching, and editing photo assets and albums.
//!
//! ```ignore
//! assert!(photos::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"photos_available"; "macos", "ios", "tvos", "xros");
