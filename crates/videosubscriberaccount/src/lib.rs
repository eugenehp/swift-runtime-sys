//! Apple VideoSubscriberAccount — TV provider authentication from Rust.
//!
//! **Platform support:** macOS 10.14+, iOS 10+, tvOS 10+.
//!
//! ```ignore
//! assert!(videosubscriberaccount::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"videosubscriberaccount_available"; "macos", "ios", "tvos");
