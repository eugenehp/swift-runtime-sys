//! Apple Core Data — persistent object graph from Rust.
//!
//! **Platform support:** macOS 10.4+, iOS 3+, tvOS 9+, visionOS 1+, watchOS 2+.
//!
//! Wraps Core Data for managed object models, persistent stores, and fetch requests.
//!
//! ```ignore
//! assert!(coredata::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"coredata_available");
