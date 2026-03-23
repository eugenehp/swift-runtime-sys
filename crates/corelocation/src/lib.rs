//! Apple Core Location — GPS and location services from Rust.
//!
//! **Platform support:** macOS 10.6+, iOS 2+, tvOS 9+, visionOS 1+, watchOS 2+.
//!
//! Wraps Core Location for GPS, geofencing, beacon ranging, and heading updates.
//!
//! ```ignore
//! assert!(corelocation::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"corelocation_available");
