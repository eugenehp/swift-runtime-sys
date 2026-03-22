//! Page header with title, subtitle, and refresh button.

use swiftui::prelude::*;

pub fn page_header(title: &str, subtitle: &str, refresh_count: &State<i32>) -> View {
    let n = refresh_count.get();

    hstack![
        vstack![
            text(title).style(StylePreset::Title),
            text(subtitle).style(StylePreset::Subtitle),
        ],
        spacer(),
        vstack![
            button("Refresh", refresh_count.bind(|n| n + 1)),
            text(&format!("Refreshed {n}x"))
                .size(10.0)
                .foreground(Color::GRAY),
        ],
    ]
}
