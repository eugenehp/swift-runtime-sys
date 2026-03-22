//! Reactive TODO app — multiple state values, conditional rendering.

use swiftui::prelude::*;

fn main() {
    app("TODO", 450.0, 500.0, |cx| {
        let items = cx.state(vec![
            ("Learn Rust".to_string(), true),
            ("Build SwiftUI bridge".to_string(), true),
            ("Add reactive state".to_string(), true),
            ("Write documentation".to_string(), false),
            ("Ship it".to_string(), false),
        ]);
        let done_count = cx.state(3i32);

        let total = items.get().len() as i32;
        let done: i32 = done_count.get();

        vstack![
            // Header
            text("📋 TODO").bold().size(28.0),
            text(&format!("{done}/{total} completed"))
                .size(14.0)
                .color(Color::GRAY),
            progress(done as f32, total as f32),
            divider(),
            // Items
            todo_list(&items.get()),
            divider(),
            // Actions
            hstack![
                button("Complete next", {
                    let items = items.clone();
                    let done_count = done_count.clone();
                    move || {
                        items.update(|list| {
                            let mut new = list.clone();
                            if let Some(item) = new.iter_mut().find(|(_, d)| !d) {
                                item.1 = true;
                            }
                            new
                        });
                        done_count.update(|n| (n + 1).min(total));
                    }
                }),
                button("Reset all", {
                    let items = items.clone();
                    let done_count = done_count.clone();
                    move || {
                        items.update(|list| list.iter().map(|(s, _)| (s.clone(), false)).collect());
                        done_count.set(0);
                    }
                }),
            ],
            spacer(),
        ]
        .padding(20.0)
        .bg(Color::DARKER)
    });
}

fn todo_list(items: &[(String, bool)]) -> View {
    let views: Vec<View> = items
        .iter()
        .map(|(title, done)| {
            let icon = if *done { "✅" } else { "⬜" };
            let c = if *done { Color::GREEN } else { Color::WHITE };
            text(&format!("{icon} {title}"))
                .size(16.0)
                .foreground(c)
                .into()
        })
        .collect();
    swiftui::dsl::vstack(views)
}
