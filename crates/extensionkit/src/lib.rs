//! Apple ExtensionKit — app extensions from Rust.
//!
//! **Platform support:** macOS 13+, iOS 16+, tvOS 16+, visionOS 1+, watchOS 9+.
//!
//! Wraps ExtensionKit and ExtensionFoundation for building and hosting app extensions.
//!
//! ```ignore
//! assert!(extensionkit::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"extensionkit_available");
