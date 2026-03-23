//! Apple DockKit — motorized stand control from Rust.
//!
//! **Platform support:** iOS 17+.
//!
//! Wraps DockKit for controlling motorized camera stands and tracking.
//!
//! ```ignore
//! assert!(dockkit::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"dockkit_available"; "ios");
