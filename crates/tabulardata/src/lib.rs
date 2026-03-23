//! Apple TabularData — data tables and CSV from Rust.
//!
//! **Platform support:** macOS 12+, iOS 15+, tvOS 15+, visionOS 1+, watchOS 8+.
//!
//! Wraps TabularData for DataFrame, columns, and CSV/JSON import.
//!
//! ```ignore
//! assert!(tabulardata::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"tabulardata_available");
