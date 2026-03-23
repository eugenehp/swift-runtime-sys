//! Apple NetworkExtension — VPN and content filtering from Rust.
//!
//! **Platform support:** macOS 10.11+, iOS 8+, tvOS 17+, visionOS 1+.
//!
//! Wraps NetworkExtension for VPN, DNS proxy, and content filter providers.
//!
//! ```ignore
//! assert!(networkextension::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"networkextension_available"; "macos", "ios", "tvos", "xros");
