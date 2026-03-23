//! Apple WorkoutKit — workout composition from Rust.
//!
//! **Platform support:** iOS 17+, watchOS 10+.
//!
//! Wraps WorkoutKit for building custom workout plans and intervals.
//!
//! ```ignore
//! assert!(workoutkit::is_available());
//! ```

apple_sys_helpers::apple_framework!(c"workoutkit_available"; "ios", "watchos");
