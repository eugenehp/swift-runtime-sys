//! Toolbar — action buttons.

use crate::model::Note;
use swiftui::prelude::*;

pub fn toolbar(
    notes: &[Note],
    selected: usize,
    is_editing: bool,
    editing: &State<bool>,
    notes_state: &State<Vec<Note>>,
    selected_state: &State<usize>,
) -> View {
    let has_note = selected < notes.len();

    hstack![
        // Edit/Done toggle
        when(has_note, || {
            let ed = editing.clone();
            let label = if is_editing { "Done" } else { "Edit" };
            button(label, move || ed.update(|e| !e))
        }),
        spacer(),
        // Pin toggle
        when(has_note, || {
            let ns = notes_state.clone();
            let sel = selected;
            let pinned = notes.get(selected).map(|n| n.pinned).unwrap_or(false);
            let label = if pinned { "Unpin" } else { "Pin" };
            button(label, move || {
                ns.update(|notes| {
                    let mut new = notes.clone();
                    if sel < new.len() {
                        new[sel].pinned = !new[sel].pinned;
                    }
                    new
                });
            })
        }),
        // Delete
        when(has_note, || {
            let ns = notes_state.clone();
            let ss = selected_state.clone();
            let sel = selected;
            button("Delete", move || {
                ns.update(|notes| {
                    let mut new = notes.clone();
                    if sel < new.len() {
                        new.remove(sel);
                    }
                    new
                });
                let len = ns.get().len();
                if sel >= len && len > 0 {
                    ss.set(len - 1);
                }
            })
        }),
    ]
    .padding(8.0)
    .bg(Color::rgb(0.12, 0.12, 0.15))
}
