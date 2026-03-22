//! Full SwiftUI demo — showcases all views and modifiers.

use swiftui::dsl::*;
use swiftui::{hstack, vstack, FontWeight};

fn main() {
    swiftui::init("swift_helper/libSwiftUIHelper.dylib");

    fn on_click() {
        println!("🦀 Clicked!");
    }

    window(
        "SwiftUI from Rust — Full Demo",
        520.0,
        700.0,
        vstack![
            // Title with shadow
            text("SwiftUI from Rust")
                .bold()
                .size(32.0)
                .foreground(Color::BLUE)
                .shadow(Color::BLUE, 4.0, 0.0, 2.0),
            text("Every modifier, every view")
                .italic()
                .size(13.0)
                .color(Color::GRAY),
            divider(),
            // Label with icon
            label("Settings", "gear"),
            label("Profile", "person.circle"),
            label("Notifications", "bell.badge"),
            divider(),
            // Controls row
            hstack![button("Click me", on_click), slider(0.5, 0.0, 1.0),].padding(8.0),
            // Card with overlay
            vstack![
                toggle("Dark mode", true),
                toggle("Sounds", false),
                progress(0.65, 1.0),
                textfield("Email", "user@example.com"),
            ]
            .padding(16.0)
            .bg(Color::DARK)
            .rounded(12.0)
            .shadow(rgb(0.0, 0.0, 0.0), 8.0, 0.0, 4.0),
            // Color circles with clip + scale
            hstack![
                color_circle(Color::RED, "Red"),
                color_circle(Color::GREEN, "Green"),
                color_circle(Color::BLUE, "Blue"),
                color_circle(Color::YELLOW, "Gold"),
                color_circle(Color::PURPLE, "Purple"),
            ],
            // Link
            link("Open Rust website", "https://www.rust-lang.org"),
            spacer(),
            // Footer with rotation effect
            text("swift-runtime-sys v0.0.3")
                .size(11.0)
                .color(rgb(0.4, 0.4, 0.4)),
        ]
        .padding(20.0)
        .bg(Color::DARKER)
        .scroll(),
    );
}

fn color_circle(c: Color, name: &str) -> swiftui::View {
    vstack![
        color(c)
            .frame(40.0, 40.0)
            .clip_circle()
            .shadow(c, 4.0, 0.0, 2.0),
        text(name).size(10.0).color(Color::GRAY),
    ]
}
