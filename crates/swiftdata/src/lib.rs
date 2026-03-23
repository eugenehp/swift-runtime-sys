//! Apple SwiftData — modern persistence framework from Rust.
//!
//! **Platform support:** macOS 14+, iOS 17+, tvOS 17+, visionOS 1+, watchOS 10+.
//!
//! Wraps SwiftData for declarative data modeling and persistence.
//!
//! Note: The existing `swift-data` crate provides UserDefaults. This crate wraps the SwiftData framework.
//!
//! ```ignore
//! assert!(swiftdata::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"swiftdata_available");
