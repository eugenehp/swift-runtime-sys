//! Apple AccessoryTransportExtension — accessory transport from Rust.
//!
//! **Platform support:** iOS 18+.
//!
//! ```ignore
//! assert!(accessorytransportextension::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"accessorytransportextension_available"; "ios");
