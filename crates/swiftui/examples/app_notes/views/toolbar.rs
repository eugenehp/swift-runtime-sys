use crate::model::*;
use swiftui::hstack;
use swiftui::prelude::*;

pub fn toolbar(
    notes: &State<Vec<Note>>,
    selected: usize,
    editing: &State<bool>,
    selected_state: &State<usize>,
) -> View {
    let has_note = selected < notes.len();

    hstack![
        show_if(has_note, || {
            button(
                if editing.get() { "Done" } else { "Edit" },
                editing.toggle(),
            )
        }),
        spacer(),
        show_if(has_note, || {
            let notes = notes.clone();
            let pinned = notes.get().get(selected).map(|n| n.pinned).unwrap_or(false);
            button(if pinned { "Unpin" } else { "Pin" }, {
                let notes = notes.clone();
                action(move || {
                    notes.update_at(selected, |n| Note {
                        pinned: !n.pinned,
                        ..n.clone()
                    })
                })
            })
        }),
        show_if(has_note, || {
            let notes = notes.clone();
            let sel = selected_state.clone();
            button(
                "Delete",
                action(move || {
                    notes.remove(selected);
                    if selected >= notes.len() && notes.len() > 0 {
                        sel.set(notes.len() - 1);
                    }
                }),
            )
        }),
    ]
    .padding(8.0)
    .bg(rgb(0.12, 0.12, 0.15))
}
