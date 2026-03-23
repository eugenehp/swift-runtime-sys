//! Apple TipKit — in-app tips and hints from Rust.
//!
//! **Platform support:** macOS 14+, iOS 17+, tvOS 17+, visionOS 1+, watchOS 10+.
//!
//! Wraps TipKit for displaying contextual tips, feature discovery, and onboarding hints.
//!
//! ```ignore
//! assert!(tipkit::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"tipkit_available");
