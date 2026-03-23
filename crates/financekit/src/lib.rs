//! Apple FinanceKit — financial data from Rust.
//!
//! **Platform support:** macOS 15+, iOS 17.4+.
//!
//! Wraps FinanceKit for reading Apple Card and Apple Cash transaction history.
//!
//! ```ignore
//! assert!(financekit::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"financekit_available"; "macos", "ios");
