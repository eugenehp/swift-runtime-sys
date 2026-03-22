use swiftui::prelude::*;
use swiftui::{hstack, txt, vstack};

fn activity_row(icon: &str, message: &str, time: &str) -> View {
    hstack![
        txt!("{icon}").size(16.0),
        vstack![
            txt!("{message}").size(13.0),
            txt!("{time}").size(10.0).foreground(GRAY)
        ],
        spacer(),
    ]
    .padding(6.0)
}

pub fn recent_activity() -> View {
    vstack![
        txt!("Recent Activity").style(Heading),
        activity_row("🔨", "Built SwiftUI bridge", "2 min ago"),
        activity_row("✅", "Tests passing: 91", "5 min ago"),
        activity_row("📦", "Added scene support", "12 min ago"),
        activity_row("🎨", "New style presets", "1 hr ago"),
        activity_row("📱", "iOS support added", "2 hr ago"),
        spacer(),
    ]
}

pub fn activity_feed(cx: &Cx) -> View {
    let show_all = cx.state(false);
    vstack![
        txt!("Activity").style(Title),
        divider(),
        recent_activity(),
        view_if(
            show_all.get(),
            || vstack![
                activity_row("🔧", "Initial commit", "1 day ago"),
                activity_row("📋", "PLAN.md created", "1 day ago"),
                button("Show less", show_all.set_to(false)),
            ],
            || button("Show all", show_all.set_to(true)),
        ),
        spacer(),
    ]
    .padding(16.0)
}
