use swiftui::prelude::*;
use swiftui::{hstack, txt, vstack};

pub fn stat_card(title: &str, value: &str, trend: &str, accent: Color) -> View {
    let tc = if trend.starts_with('+') { GREEN } else { RED };
    vstack![
        txt!("{title}").size(11.0).foreground(GRAY),
        txt!("{value}").bold().size(22.0).foreground(accent),
        txt!("{trend}").size(12.0).foreground(tc),
    ]
    .padding(12.0)
    .bg(DARK)
    .rounded(10.0)
}

fn project_row(name: &str, prog: f32, status: &str, accent: Color) -> View {
    hstack![
        color(accent).frame(8.0, 8.0).clip_circle(),
        vstack![
            txt!("{name}").size(14.0),
            hstack![
                progress(prog, 1.0),
                txt!("{}%", (prog * 100.0) as i32)
                    .size(11.0)
                    .foreground(GRAY),
            ],
        ],
        spacer(),
        txt!("{status}")
            .size(11.0)
            .foreground(accent)
            .padding(4.0)
            .bg(rgba(accent.r, accent.g, accent.b, 0.15))
            .rounded(4.0),
    ]
    .padding(8.0)
}

pub fn project_list() -> View {
    vstack![
        txt!("Projects").style(Heading),
        project_row("SwiftUI Bridge", 0.95, "Active", GREEN),
        project_row("Runtime Bindings", 1.0, "Done", BLUE),
        project_row("Bridge Generator", 0.6, "Active", GREEN),
        project_row("iOS Support", 0.3, "In Progress", YELLOW),
        project_row("Documentation", 0.8, "Active", GREEN),
    ]
}
