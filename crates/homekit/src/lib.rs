//! Apple HomeKit — smart home control from Rust.
//!
//! **Platform support:** iOS 8+, watchOS 2+, tvOS 10+, visionOS 1+.
//!
//! ```ignore
//! assert!(homekit::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"homekit_available"; "ios", "watchos", "tvos", "xros");
