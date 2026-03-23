//! Apple MPS Graph — GPU machine learning graph operations from Rust.
//!
//! **Platform support:** macOS 11+, iOS 14+, tvOS 14+.
//!
//! ```ignore
//! assert!(metalperformanceshadersgraph::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"metalperformanceshadersgraph_available"; "macos", "ios", "tvos");
