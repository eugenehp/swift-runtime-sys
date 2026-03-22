use swiftui::prelude::*;
use swiftui::{hstack, txt, vstack};

fn main() {
    app("TODO", 450.0, 500.0, |cx| {
        let items = cx.state(vec![
            ("Learn Rust", true),
            ("Build SwiftUI bridge", true),
            ("Add reactive state", true),
            ("Write docs", false),
            ("Ship it", false),
        ]);

        let list = items.get();
        let done = list.iter().filter(|(_, d)| *d).count();
        let total = list.len();

        vstack![
            txt!("📋 TODO").style(Title),
            txt!("{done}/{total} completed").style(Subtitle),
            progress(done as f32, total as f32),
            divider(),
            for_each(&list, |(title, done)| {
                let icon = if *done { "✅" } else { "⬜" };
                let c = if *done { GREEN } else { WHITE };
                txt!("{icon} {title}").foreground(c)
            }),
            divider(),
            hstack![
                button("Complete next", {
                    let items = items.clone();
                    move || {
                        let list = items.get();
                        if let Some(i) = list.iter().position(|(_, d)| !d) {
                            items.update_at(i, |(s, _)| (s.clone(), true));
                        }
                    }
                }),
                button("Reset all", {
                    let items = items.clone();
                    move || {
                        let n = items.len();
                        for i in 0..n {
                            items.update_at(i, |(s, _)| (s.clone(), false));
                        }
                    }
                }),
            ],
            spacer(),
        ]
        .padding(20.0)
        .bg(DARKER)
    });
}
