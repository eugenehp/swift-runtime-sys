//! Navigation sidebar.

use swiftui::prelude::*;

fn sidebar_item(icon: &str, title: &str, tab_idx: i32, current: i32, tab: &State<i32>) -> View {
    let is_selected = current == tab_idx;
    let bg = if is_selected {
        Color::rgb(0.2, 0.3, 0.5)
    } else {
        Color::CLEAR
    };

    button(&format!("{icon}  {title}"), tab.set_to(tab_idx))
        .padding(8.0)
        .bg(bg)
        .rounded(6.0)
}

pub fn sidebar(cx: &Cx, tab: &State<i32>) -> View {
    let current = tab.get();

    vstack![
        // Logo
        text("🦀 Dashboard")
            .bold()
            .size(16.0)
            .foreground(Color::BLUE)
            .padding(12.0),
        divider(),
        // Nav items
        sidebar_item("📊", "Overview", 0, current, tab),
        sidebar_item("📁", "Projects", 1, current, tab),
        sidebar_item("📋", "Activity", 2, current, tab),
        sidebar_item("⚙️", "Settings", 3, current, tab),
        spacer(),
        // Version
        text("v0.0.3").style(StylePreset::Caption).padding(8.0),
    ]
    .padding(4.0)
    .bg(Color::rgb(0.06, 0.06, 0.09))
}
