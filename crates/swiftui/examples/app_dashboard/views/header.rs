use swiftui::prelude::*;
use swiftui::{hstack, txt, vstack};

pub fn page_header(title: &str, subtitle: &str, refresh: &State<i32>) -> View {
    let n = refresh.get();
    hstack![
        vstack![
            txt!("{title}").style(Title),
            txt!("{subtitle}").style(Subtitle)
        ],
        spacer(),
        vstack![
            button("Refresh", refresh.increment()),
            txt!("Refreshed {n}x").size(10.0).foreground(GRAY),
        ],
    ]
}
