//! Apple AppIntents — Siri shortcuts and Spotlight integration from Rust.
//!
//! **Platform support:** macOS 13+, iOS 16+, tvOS 16+, visionOS 1+, watchOS 9+.
//!
//! ```ignore
//! assert!(appintents::is_available());
//! ```
//!
//! Note: Defining actual App Intents requires the Swift @AppIntent protocol
//! which needs compiler macro support. This crate provides availability
//! checking and will be extended as the bridge generator supports protocols.

apple_sys_helpers::apple_framework!(c"appintents_available");
