//! Apple EnergyKit — energy usage information from Rust.
//!
//! **Platform support:** iOS 26+.
//!
//! ```ignore
//! assert!(energykit::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"energykit_available"; "ios");
