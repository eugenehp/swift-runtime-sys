//! Apple ServicesAccountLinking — service account linking from Rust.
//!
//! **Platform support:** iOS 26+.
//!
//! ```ignore
//! assert!(servicesaccountlinking::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"servicesaccountlinking_available"; "ios");
