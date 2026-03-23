//! Apple Core Spotlight — search indexing from Rust.
//!
//! **Platform support:** macOS 10.13+, iOS 9+, visionOS 1+.
//!
//! Wraps Core Spotlight for indexing app content for system search.
//!
//! ```ignore
//! assert!(corespotlight::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"corespotlight_available"; "macos", "ios", "xros");
