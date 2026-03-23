//! Apple CloudKit — iCloud database from Rust.
//!
//! **Platform support:** macOS 10.10+, iOS 8+, tvOS 9+, visionOS 1+, watchOS 3+.
//!
//! Wraps CloudKit for iCloud public/private database, records, and subscriptions.
//!
//! ```ignore
//! assert!(cloudkit::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"cloudkit_available");
