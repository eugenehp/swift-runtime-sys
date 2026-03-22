//! Stat cards and project cards — reusable composed components.

use swiftui::prelude::*;

/// A stat card showing a metric with trend.
pub fn stat_card(title: &str, value: &str, trend: &str, accent: Color) -> View {
    let trend_color = if trend.starts_with('+') {
        Color::GREEN
    } else {
        Color::RED
    };

    vstack![
        text(title).size(11.0).foreground(Color::GRAY),
        text(value).bold().size(22.0).foreground(accent),
        text(trend).size(12.0).foreground(trend_color),
    ]
    .padding(12.0)
    .bg(Color::DARK)
    .rounded(10.0)
}

/// A single project row.
fn project_row(name: &str, prog: f32, status: &str, accent: Color) -> View {
    hstack![
        color(accent).frame(8.0, 8.0).clip_circle(),
        vstack![
            text(name).size(14.0).foreground(Color::WHITE),
            hstack![
                progress(prog, 1.0),
                text(&format!("{}%", (prog * 100.0) as i32))
                    .size(11.0)
                    .foreground(Color::GRAY),
            ],
        ],
        spacer(),
        text(status)
            .size(11.0)
            .foreground(accent)
            .padding(4.0)
            .bg(Color::rgba(accent.r, accent.g, accent.b, 0.15))
            .rounded(4.0),
    ]
    .padding(8.0)
}

/// List of projects.
pub fn project_list() -> View {
    vstack![
        text("Projects").style(StylePreset::Heading),
        project_row("SwiftUI Bridge", 0.95, "Active", Color::GREEN),
        project_row("Runtime Bindings", 1.0, "Done", Color::BLUE),
        project_row("Bridge Generator", 0.6, "Active", Color::GREEN),
        project_row("iOS Support", 0.3, "In Progress", Color::YELLOW),
        project_row("Documentation", 0.8, "Active", Color::GREEN),
    ]
}
