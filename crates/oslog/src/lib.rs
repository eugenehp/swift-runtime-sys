//! Apple OSLog — unified logging from Rust.
//!
//! **Platform support:** macOS 10.15+, iOS 15+, tvOS 15+, visionOS 1+, watchOS 8+.
//!
//! Wraps OSLog for structured logging with the unified logging system.
//!
//! ```ignore
//! assert!(oslog::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"oslog_available");
