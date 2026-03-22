use crate::model::*;
use crate::views::components::*;
use swiftui::prelude::*;
use swiftui::vstack;

pub fn sidebar(notes: &State<Vec<Note>>, selected: &State<usize>) -> View {
    let list = notes.get();
    let pinned: Vec<_> = list.iter().enumerate().filter(|(_, n)| n.pinned).collect();
    let unpinned: Vec<_> = list.iter().enumerate().filter(|(_, n)| !n.pinned).collect();

    vstack![
        textfield("Search notes...", "").padding(8.0),
        show_if(!pinned.is_empty(), || {
            vstack![
                section_header("📌 Pinned"),
                for_each(&pinned, |(i, note)| {
                    button(&note.title, selected.set_to(*i)).padding(2.0)
                }),
            ]
        }),
        section_header("All Notes"),
        for_each(&unpinned, |(i, note)| {
            button(&note.title, selected.set_to(*i)).padding(2.0)
        }),
        spacer(),
        button("+ New Note", {
            let notes = notes.clone();
            let selected = selected.clone();
            move || {
                let n = notes.len();
                notes.push(Note::new(&format!("Note {}", n + 1)));
                selected.set(n);
            }
        })
        .padding(8.0),
    ]
    .padding(4.0)
    .bg(rgb(0.08, 0.08, 0.1))
}
