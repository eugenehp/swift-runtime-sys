//! Ergonomic SwiftUI from Rust using the `swiftui` crate.
//!
//! Build helper first:
//!   xcrun swiftc -emit-library swift_helper/SwiftUIHelper.swift \
//!     -o swift_helper/libSwiftUIHelper.dylib \
//!     -target arm64-apple-macosx15.0 -sdk $(xcrun -sdk macosx --show-sdk-path)

fn main() {
    swiftui::show_window(
        "swift_helper/libSwiftUIHelper.dylib",
        "Rust + SwiftUI 🦀",
        500.0,
        500.0,
        |ui| {
            fn on_hello() { println!("Hello clicked!"); }
            fn on_world() { println!("World clicked!"); }

            let title = ui.bold_text("🦀 Rust + SwiftUI", 28.0);
            let subtitle = ui.styled_text("Ergonomic API demo", 14.0, 2, 0.5, 0.5, 0.5, 1.0);

            let buttons = ui.hstack(&[
                ui.button("Hello", on_hello),
                ui.button("World", on_world),
            ]);

            let info = ui.hstack(&[
                ui.system_image("swift"),
                ui.text("Powered by swift-runtime-sys"),
                ui.spacer(),
                ui.text("v0.0.3"),
            ]);

            let card = ui.vstack(&[
                ui.toggle("Dark mode", true),
                ui.progress(0.75, 1.0),
                ui.textfield("Search...", ""),
            ]);
            let card = ui.padding(&card, 16.0);
            let card = ui.background(&card, 0.12, 0.12, 0.15, 1.0);
            let card = ui.corner_radius(&card, 10.0);

            let colors = ui.hstack(&[
                color_swatch(ui, 1.0, 0.3, 0.3, "Red"),
                color_swatch(ui, 0.3, 0.8, 0.3, "Green"),
                color_swatch(ui, 0.3, 0.5, 1.0, "Blue"),
                color_swatch(ui, 1.0, 0.8, 0.2, "Gold"),
            ]);

            let content = ui.vstack(&[
                title, subtitle,
                ui.divider(),
                buttons,
                card,
                info,
                colors,
                ui.spacer(),
            ]);
            let content = ui.padding(&content, 20.0);
            let content = ui.background(&content, 0.05, 0.05, 0.08, 1.0);
            ui.scroll(&content)
        },
    );
}

fn color_swatch(ui: &swiftui::SwiftUI, r: f32, g: f32, b: f32, name: &str) -> swiftui::ViewHandle {
    let c = ui.color(r, g, b, 1.0);
    let c = ui.frame(&c, 36.0, 36.0);
    let c = ui.corner_radius(&c, 6.0);
    let label = ui.styled_text(name, 10.0, 0, 0.6, 0.6, 0.6, 1.0);
    ui.vstack(&[c, label])
}
