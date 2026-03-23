//! Apple Core Bluetooth — BLE from Rust.
//!
//! **Platform support:** macOS 10.10+, iOS 5+, tvOS 9+, visionOS 1+, watchOS 2+.
//!
//! Wraps Core Bluetooth for BLE central/peripheral, service discovery, and characteristics.
//!
//! ```ignore
//! assert!(corebluetooth::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"corebluetooth_available");
