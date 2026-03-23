//! Apple Intents — Siri intents and shortcuts from Rust.
//!
//! **Platform support:** macOS 11+, iOS 10+, tvOS 14+, watchOS 3.2+.
//!
//! ```ignore
//! assert!(intents::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"intents_available"; "macos", "ios", "tvos", "watchos");
