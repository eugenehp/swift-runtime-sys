//! Everything you need for a reactive SwiftUI app.
//!
//! ```ignore
//! use swiftui::prelude::*;
//! ```

pub use crate::color::{
    hex, rgb, rgba, Color, BLACK, BLUE, CLEAR, DARK, DARKER, GRAY, GREEN, PURPLE, RED, WHITE,
    YELLOW,
};
pub use crate::conditional::{empty, for_each, for_each_enumerated, for_each_h, when, when_else};
pub use crate::dsl::{
    color, content_unavailable, disclosure_group, divider, form, grid, group_box, image, label,
    labeled_content, link, list, menu, navigation_split_view, progress, section, secure_field,
    share_link, show_if, slider, spacer, stepper, text, text_editor, textfield, toggle, view_if,
    IntoView, TextView,
};
pub use crate::host::{App, BackgroundMaterial, WindowStyle};
pub use crate::nav::{back_button, nav_button, navigator};
pub use crate::scene::SceneApp;
pub use crate::state::{
    action, app, bound_picker, bound_slider, bound_textfield, bound_toggle, button, tabview, Cx,
    State, Tab,
};
pub use crate::style::{MultiStyled, StylePreset, Styled};
pub use crate::view::{FontWeight, View};
pub use crate::{hstack, vstack, zstack};

// Re-export proc macros
pub use swiftui_macros::text_fmt;

// ── Short names — import once, use everywhere ──

// Style presets — use `Title` instead of `StylePreset::Title`
pub use crate::style::StylePreset::{
    Body, Caption, CardDark, CardLight, Elevated, Heading, Page, Pill, Subtitle, Title,
};

// Font weights — use `Bold` instead of `FontWeight::Bold`
pub use crate::view::FontWeight::{Bold, Heavy, Light, Medium, Regular, Semibold, Thin};

// Window styles — use `Borderless` instead of `WindowStyle::Borderless`
pub use crate::host::WindowStyle::{Borderless, Floating, Fullscreen, Transparent};
