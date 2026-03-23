//! Apple DeviceDiscoveryExtension — streaming device discovery from Rust.
//!
//! **Platform support:** macOS 14+, iOS 16+, tvOS 16+.
//!
//! ```ignore
//! assert!(devicediscoveryextension::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"devicediscoveryextension_available"; "macos", "ios", "tvos");
