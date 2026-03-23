//! Apple RoomPlan — 3D room scanning with LiDAR from Rust.
//!
//! **Platform support:** iOS 16+.
//!
//! ```ignore
//! assert!(roomplan::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"roomplan_available"; "ios");
