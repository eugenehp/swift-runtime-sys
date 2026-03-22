use swiftui::prelude::*;
use swiftui::{txt, vstack};

fn sidebar_item(icon: &str, title: &str, idx: i32, current: i32, tab: &State<i32>) -> View {
    let bg = if current == idx {
        rgb(0.2, 0.3, 0.5)
    } else {
        CLEAR
    };
    button(&format!("{icon}  {title}"), tab.set_to(idx))
        .padding(8.0)
        .bg(bg)
        .rounded(6.0)
}

pub fn sidebar(tab: &State<i32>) -> View {
    let t = tab.get();
    vstack![
        txt!("🦀 Dashboard")
            .bold()
            .size(16.0)
            .foreground(BLUE)
            .padding(12.0),
        divider(),
        sidebar_item("📊", "Overview", 0, t, tab),
        sidebar_item("📁", "Projects", 1, t, tab),
        sidebar_item("📋", "Activity", 2, t, tab),
        sidebar_item("⚙️", "Settings", 3, t, tab),
        spacer(),
        txt!("v0.0.3").style(Caption).padding(8.0),
    ]
    .padding(4.0)
    .bg(rgb(0.06, 0.06, 0.09))
}
