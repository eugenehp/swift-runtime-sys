//! Apple JournalingSuggestions — journaling suggestion picker from Rust.
//!
//! **Platform support:** iOS 17.2+.
//!
//! ```ignore
//! assert!(journalingsuggestions::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"journalingsuggestions_available"; "ios");
