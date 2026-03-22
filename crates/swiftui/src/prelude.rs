//! Everything you need for a reactive SwiftUI app.
//!
//! ```ignore
//! use swiftui::prelude::*;
//! ```

pub use crate::color::{hex, rgb, rgba, Color};
pub use crate::conditional::{empty, for_each, for_each_enumerated, for_each_h, when, when_else};
pub use crate::dsl::{
    color, divider, image, label, link, progress, slider, spacer, text, textfield, toggle,
    IntoView, TextView,
};
pub use crate::host::{App, BackgroundMaterial, WindowStyle};
pub use crate::nav::{back_button, nav_button, navigator};
pub use crate::state::{app, button, Cx, State};
pub use crate::style::{StylePreset, Styled};
pub use crate::view::{FontWeight, View};
pub use crate::{hstack, vstack, zstack};

// Re-export the proc macros
pub use swiftui_macros::text_fmt;
