use crate::model::*;
use crate::views::components::*;
use swiftui::prelude::*;
use swiftui::{hstack, txt, vstack};

pub fn detail(notes: &[Note], selected: usize) -> View {
    if selected >= notes.len() {
        return empty_state("📝", "Select a note");
    }
    let note = &notes[selected];
    vstack![
        txt!("{}", note.title).bold().size(24.0),
        hstack![tag_badge(&note.tag), pin_icon(note.pinned), spacer()],
        divider(),
        txt!("{}", note.body)
            .size(15.0)
            .foreground(rgb(0.85, 0.85, 0.85)),
        spacer(),
    ]
    .padding(20.0)
}

pub fn editor(notes: &[Note], selected: usize) -> View {
    if selected >= notes.len() {
        return empty_state("✏️", "No note to edit");
    }
    let note = &notes[selected];
    vstack![
        txt!("Editing").style(Caption),
        textfield("Title", &note.title),
        divider(),
        textfield("Body", &note.body),
        spacer(),
        hstack![tag_badge(&note.tag), spacer()],
    ]
    .padding(20.0)
}
