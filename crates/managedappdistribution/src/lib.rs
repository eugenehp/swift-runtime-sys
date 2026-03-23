//! Apple ManagedAppDistribution — enterprise app distribution from Rust.
//!
//! **Platform support:** macOS 14+, iOS 17+.
//!
//! ```ignore
//! assert!(managedappdistribution::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"managedappdistribution_available"; "macos", "ios");
