//! Note editor and detail views.

use crate::model::Note;
use crate::views::components::*;
use swiftui::prelude::*;

/// Read-only note detail view.
pub fn note_detail(notes: &[Note], selected: usize) -> View {
    if selected >= notes.len() {
        return empty_state("📝", "Select a note");
    }

    let note = &notes[selected];

    vstack![
        // Title
        text(&note.title).bold().size(24.0).foreground(Color::WHITE),
        // Tag + pin
        hstack![tag_badge(&note.tag), pin_icon(note.pinned), spacer(),],
        divider(),
        // Body
        text(&note.body)
            .size(15.0)
            .foreground(Color::rgb(0.85, 0.85, 0.85)),
        spacer(),
    ]
    .padding(20.0)
}

/// Editor view (simplified — shows textfield for title and body).
pub fn editor(notes: &[Note], selected: usize) -> View {
    if selected >= notes.len() {
        return empty_state("✏️", "No note to edit");
    }

    let note = &notes[selected];

    vstack![
        text("Editing").style(StylePreset::Caption),
        textfield("Title", &note.title),
        divider(),
        textfield("Body", &note.body),
        spacer(),
        hstack![tag_badge(&note.tag), spacer(),],
    ]
    .padding(20.0)
}
