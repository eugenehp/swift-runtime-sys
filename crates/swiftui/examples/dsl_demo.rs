//! SwiftUI DSL demo — clean, ergonomic Rust syntax.
//!
//! Build helper:
//!   xcrun swiftc -emit-library swift_helper/SwiftUIHelper.swift \
//!     -o swift_helper/libSwiftUIHelper.dylib \
//!     -target arm64-apple-macosx15.0 -sdk $(xcrun -sdk macosx --show-sdk-path)
//!
//! Run:
//!   cargo run -p swiftui --example dsl_demo

use swiftui::dsl::*;
use swiftui::{vstack, hstack};

fn main() {
    swiftui::init("swift_helper/libSwiftUIHelper.dylib");

    fn on_click() { println!("🦀 Button clicked from Rust!"); }

    window("Rust DSL → SwiftUI", 480.0, 520.0,

        vstack![
            // Header
            text("🦀 SwiftUI DSL").bold().size(28.0).color(Color::BLUE),
            text("Built with Rust macros").italic().size(14.0).color(Color::GRAY),

            divider(),

            // Buttons row
            hstack![
                button("Say Hello", on_click),
                spacer(),
                image("star.fill"),
                text("Starred"),
            ].padding(8.0),

            // Card
            vstack![
                text("Settings").bold().size(18.0),
                toggle("Dark mode", true),
                toggle("Notifications", false),
                progress(0.7, 1.0),
                textfield("Search...", ""),
            ].padding(16.0).bg(Color::DARK).rounded(12.0),

            // Colors
            hstack![
                swatch(Color::RED, "Red"),
                swatch(Color::GREEN, "Green"),
                swatch(Color::BLUE, "Blue"),
                swatch(Color::YELLOW, "Gold"),
                swatch(Color::PURPLE, "Purple"),
            ],

            spacer(),

            // Footer
            text("swift-runtime-sys v0.0.3")
                .size(11.0)
                .color(rgb(0.4, 0.4, 0.4)),
        ]
        .padding(20.0)
        .bg(Color::DARKER)
        .scroll()

    );
}

fn swatch(c: Color, name: &str) -> swiftui::View {
    vstack![
        color(c).frame(36.0, 36.0).rounded(6.0),
        text(name).size(10.0).color(Color::GRAY),
    ]
}
