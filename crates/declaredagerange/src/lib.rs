//! Apple DeclaredAgeRange — age range declaration from Rust.
//!
//! **Platform support:** macOS 26+, iOS 26+.
//!
//! ```ignore
//! assert!(declaredagerange::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"declaredagerange_available"; "macos", "ios");
