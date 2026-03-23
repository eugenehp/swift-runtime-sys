//! Apple VisualIntelligence — visual lookup and intelligence from Rust.
//!
//! **Platform support:** iOS 26+.
//!
//! ```ignore
//! assert!(visualintelligence::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"visualintelligence_available"; "ios");
