//! Toolbar — action buttons. Cleaner with .toggle() and action().

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
        // Edit/Done — uses .toggle() instead of manual clone + update
        show_if(has_note, || {
            button(if is_editing { "Done" } else { "Edit" }, editing.toggle())
        }),
        spacer(),
        // Pin toggle — one clone, one closure
        show_if(has_note, || {
            let ns = notes_state.clone();
            let pinned = notes.get(selected).map(|n| n.pinned).unwrap_or(false);
            button(
                if pinned { "Unpin" } else { "Pin" },
                action(move || {
                    ns.update(|notes| {
                        let mut new = notes.clone();
                        if selected < new.len() {
                            new[selected].pinned = !new[selected].pinned;
                        }
                        new
                    });
                }),
            )
        }),
        // Delete
        show_if(has_note, || {
            let ns = notes_state.clone();
            let ss = selected_state.clone();
            button(
                "Delete",
                action(move || {
                    ns.update(|notes| {
                        let mut new = notes.clone();
                        if selected < new.len() {
                            new.remove(selected);
                        }
                        new
                    });
                    let len = ns.get().len();
                    if selected >= len && len > 0 {
                        ss.set(len - 1);
                    }
                }),
            )
        }),
    ]
    .padding(8.0)
    .bg(rgb(0.12, 0.12, 0.15))
}
