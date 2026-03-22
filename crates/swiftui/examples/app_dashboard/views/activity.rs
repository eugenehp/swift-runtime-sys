//! Activity feed — timestamped events.

use swiftui::prelude::*;

fn activity_row(icon: &str, message: &str, time: &str) -> View {
    hstack![
        text(icon).size(16.0),
        vstack![
            text(message).size(13.0).foreground(Color::WHITE),
            text(time).size(10.0).foreground(Color::GRAY),
        ],
        spacer(),
    ]
    .padding(6.0)
}

/// Recent activity feed.
pub fn recent_activity() -> View {
    vstack![
        text("Recent Activity").style(StylePreset::Heading),
        activity_row("🔨", "Built SwiftUI bridge", "2 min ago"),
        activity_row("✅", "Tests passing: 112", "5 min ago"),
        activity_row("📦", "Added scene support", "12 min ago"),
        activity_row("🎨", "New style presets", "1 hr ago"),
        activity_row("📱", "iOS support added", "2 hr ago"),
        activity_row("🚀", "Reactive state v2", "3 hr ago"),
        spacer(),
    ]
}

/// Full activity feed page.
pub fn activity_feed(cx: &Cx) -> View {
    let show_all = cx.state(false);

    vstack![
        text("Activity").style(StylePreset::Title),
        divider(),
        recent_activity(),
        when(!show_all.get(), || {
            button("Show all", show_all.set_to(true))
        }),
        when(show_all.get(), || {
            vstack![
                activity_row("🔧", "Initial commit", "1 day ago"),
                activity_row("📋", "PLAN.md created", "1 day ago"),
                activity_row("🏗️", "Workspace restructured", "1 day ago"),
                button("Show less", show_all.set_to(false)),
            ]
        }),
        spacer(),
    ]
    .padding(16.0)
}
