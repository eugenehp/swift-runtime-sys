//! Apple DeviceCheck — device attestation from Rust.
//!
//! **Platform support:** macOS 10.15+, iOS 11+, tvOS 11+, visionOS 1+, watchOS 9+.
//!
//! Wraps DeviceCheck and App Attest for device-level fraud prevention.
//!
//! ```ignore
//! assert!(devicecheck::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"devicecheck_available");
