//! Apple LockedCameraCapture — locked screen camera capture from Rust.
//!
//! **Platform support:** iOS 18+.
//!
//! ```ignore
//! assert!(lockedcameracapture::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"lockedcameracapture_available"; "ios");
