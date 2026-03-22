//! Notes App — multi-file, composable, latest DSL.

mod model;
mod views;

use model::*;
use swiftui::prelude::*;
use swiftui::{hstack, txt, vstack};

fn main() {
    app("Notes", 700.0, 500.0, |cx| {
        let notes = cx.state(Note::samples());
        let selected = cx.state(0usize);
        let editing = cx.state(false);

        hstack![
            views::sidebar(&notes, &selected).frame(220.0, -1.0),
            divider(),
            vstack![
                views::toolbar(&notes, selected.get(), &editing, &selected),
                divider(),
                view_if(
                    editing.get(),
                    || views::editor(&notes.get(), selected.get()),
                    || views::detail(&notes.get(), selected.get()),
                ),
            ],
        ]
        .bg(DARKER)
    });
}
