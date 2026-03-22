//! Notes App — multi-file, deeply nested, composable views.
//!
//! Structure:
//!   main.rs        — app entry, navigation
//!   views/mod.rs   — re-exports
//!   views/sidebar.rs    — note list
//!   views/editor.rs     — note editor
//!   views/toolbar.rs    — action bar
//!   views/components.rs — reusable small components
//!   model.rs       — data model

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
            // Sidebar — note list
            sidebar(&note_list, sel, &selected, &notes).frame(220.0, -1.0),
            divider(),
            // Main area
            vstack![
                toolbar(&note_list, sel, is_editing, &editing, &notes, &selected,),
                divider(),
                if is_editing {
                    editor(&note_list, sel)
                } else {
                    note_detail(&note_list, sel)
                },
            ],
        ]
        .bg(Color::DARKER)
    });
}
