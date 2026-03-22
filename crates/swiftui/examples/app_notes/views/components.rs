//! Reusable small components.

use crate::model::Tag;
use swiftui::prelude::*;

/// A tag badge — small colored pill.
pub fn tag_badge(tag: &Tag) -> View {
    text(tag.label())
        .size(10.0)
        .foreground(tag.color())
        .padding(4.0)
        .bg(rgba(tag.color().r, tag.color().g, tag.color().b, 0.15))
        .rounded(6.0)
}

/// A pin indicator — no more .into() needed.
pub fn pin_icon(pinned: bool) -> View {
    view_if(pinned, || text("📌").size(12.0), || text("").size(12.0))
}

/// Section header with title.
pub fn section_header(title: &str) -> View {
    text(title).size(11.0).foreground(GRAY).padding(4.0)
}

/// Empty state view.
pub fn empty_state(icon: &str, message: &str) -> View {
    vstack![
        spacer(),
        text(icon).size(48.0),
        text(message).style(Subtitle),
        spacer(),
    ]
}
