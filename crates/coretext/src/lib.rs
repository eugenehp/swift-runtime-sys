//! Apple Core Text — text layout and font handling from Rust.
//!
//! **Platform support:** macOS 10.5+, iOS 3.2+, tvOS 9+, visionOS 1+, watchOS 2+.
//!
//! Wraps Core Text for advanced text layout, font enumeration, and glyph rendering.
//!
//! ```ignore
//! assert!(coretext::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"coretext_available");
