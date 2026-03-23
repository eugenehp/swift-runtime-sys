//! Apple StoreKit — in-app purchases and subscriptions from Rust.
//!
//! **Platform support:** macOS 10.7+, iOS 3+, tvOS 9+, visionOS 1+, watchOS 6+.
//!
//! Wraps StoreKit 2 for products, transactions, subscriptions, and App Store receipt validation.
//!
//! ```ignore
//! assert!(storekit::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"storekit_available");
