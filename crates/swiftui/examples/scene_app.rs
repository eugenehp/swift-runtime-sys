//! Scene-based app — uses SwiftUI's native App protocol.

use swiftui::prelude::*;

fn main() {
    SceneApp::new()
        .window("main", "Scene App 🦀", 500.0, 400.0, |cx| {
            let count = cx.state(0i32);

            vstack![
                text("Scene-Based App").style(StylePreset::Title),
                text("Uses SwiftUI App protocol natively").style(StylePreset::Subtitle),
                divider(),
                text_fmt!("Count: {count}").size(48.0),
                hstack![
                    button("+1", count.bind(|n| n + 1)),
                    button("-1", count.bind(|n| n - 1)),
                    button("Reset", count.set_to(0)),
                ],
                spacer(),
                text("WindowGroup scene from Rust").style(StylePreset::Caption),
            ]
            .padding(24.0)
            .bg(Color::DARKER)
        })
        .launch();
}
