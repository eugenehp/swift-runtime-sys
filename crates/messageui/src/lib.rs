//! Apple MessageUI — in-app email and SMS compose from Rust.
//!
//! **Platform support:** iOS 3+.
//!
//! ```ignore
//! assert!(messageui::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"messageui_available"; "ios");
