//! Reusable small components.

use crate::model::{Note, Tag};
use swiftui::prelude::*;

/// A tag badge — small colored pill.
pub fn tag_badge(tag: &Tag) -> View {
    text(tag.label())
        .size(10.0)
        .foreground(tag.color())
        .padding(4.0)
        .bg(Color::rgba(
            tag.color().r,
            tag.color().g,
            tag.color().b,
            0.15,
        ))
        .rounded(6.0)
}

/// A pin indicator.
pub fn pin_icon(pinned: bool) -> View {
    if pinned {
        text("📌").size(12.0).into()
    } else {
        text("").size(12.0).into()
    }
}

/// Section header with title.
pub fn section_header(title: &str) -> View {
    text(title).size(11.0).foreground(Color::GRAY).padding(4.0)
}

/// A note row for the sidebar.
pub fn note_row(note: &Note, is_selected: bool) -> View {
    let bg = if is_selected {
        Color::rgb(0.2, 0.3, 0.5)
    } else {
        Color::CLEAR
    };

    hstack![
        pin_icon(note.pinned),
        vstack![
            text(&note.title).size(14.0).foreground(if is_selected {
                Color::WHITE
            } else {
                Color::rgb(0.9, 0.9, 0.9)
            }),
            hstack![
                tag_badge(&note.tag),
                text(&truncate(&note.body, 30))
                    .size(11.0)
                    .foreground(Color::GRAY),
            ],
        ],
    ]
    .padding(8.0)
    .bg(bg)
    .rounded(6.0)
}

/// Empty state view.
pub fn empty_state(icon: &str, message: &str) -> View {
    vstack![
        spacer(),
        text(icon).size(48.0),
        text(message).style(StylePreset::Subtitle),
        spacer(),
    ]
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}…", &s[..max])
    }
}
