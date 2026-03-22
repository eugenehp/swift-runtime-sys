//! Notes App — multi-file, deeply nested, composable views.

mod model;
mod views;

use model::*;
use swiftui::prelude::*;
use views::*;

fn main() {
    app("Notes", 700.0, 500.0, |cx| {
        let notes = cx.state(Note::samples());
        let selected = cx.state(0usize);
        let editing = cx.state(false);

        let sel = selected.get();
        let note_list = notes.get();
        let is_editing = editing.get();

        hstack![
            sidebar(&note_list, &selected, &notes).frame(220.0, -1.0),
            divider(),
            vstack![
                toolbar(&note_list, sel, is_editing, &editing, &notes, &selected),
                divider(),
                // view_if — no .into() needed, auto-converts both branches
                view_if(
                    is_editing,
                    || editor(&note_list, sel),
                    || note_detail(&note_list, sel),
                ),
            ],
        ]
        .bg(DARKER)
    });
}
