//! Sidebar — note list with selection.

use crate::model::Note;
use crate::views::components::*;
use swiftui::prelude::*;

pub fn sidebar(
    notes: &[Note],
    selected: usize,
    selected_state: &State<usize>,
    notes_state: &State<Vec<Note>>,
) -> View {
    let pinned: Vec<_> = notes.iter().enumerate().filter(|(_, n)| n.pinned).collect();
    let unpinned: Vec<_> = notes
        .iter()
        .enumerate()
        .filter(|(_, n)| !n.pinned)
        .collect();

    vstack![
        // Search bar
        textfield("Search notes...", "").padding(8.0),
        // Pinned section
        when(!pinned.is_empty(), || {
            vstack![
                section_header("📌 Pinned"),
                for_each(&pinned, |(idx, note)| {
                    let i = *idx;
                    let ss = selected_state.clone();
                    button(&note.title, move || ss.set(i)).padding(2.0)
                }),
            ]
        }),
        // All notes section
        section_header("All Notes"),
        for_each(&unpinned, |(idx, note)| {
            let i = *idx;
            let ss = selected_state.clone();
            button(&note.title, move || ss.set(i)).padding(2.0)
        }),
        spacer(),
        // Add button
        button("+ New Note", {
            let ns = notes_state.clone();
            let ss = selected_state.clone();
            move || {
                ns.update(|notes| {
                    let mut new = notes.clone();
                    new.push(Note {
                        title: format!("Note {}", new.len() + 1),
                        body: "New note...".into(),
                        pinned: false,
                        tag: crate::model::Tag::Personal,
                    });
                    new
                });
                let len = ns.get().len();
                ss.set(len - 1);
            }
        })
        .padding(8.0),
    ]
    .padding(4.0)
    .bg(Color::rgb(0.08, 0.08, 0.1))
}
