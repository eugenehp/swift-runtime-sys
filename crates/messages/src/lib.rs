//! Apple Messages — iMessage app extensions from Rust.
//!
//! **Platform support:** iOS 10+.
//!
//! ```ignore
//! assert!(messages::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"messages_available"; "ios");
