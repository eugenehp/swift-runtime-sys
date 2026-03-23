//! Apple QuickLookUI — QuickLook preview panel from Rust.
//!
//! **Platform support:** macOS 10.5+.
//!
//! ```ignore
//! assert!(quicklookui::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"quicklookui_available"; "macos");
