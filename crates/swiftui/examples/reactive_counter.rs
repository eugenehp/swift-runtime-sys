//! Reactive counter — clean API.

use swiftui::prelude::*;

fn main() {
    app("Counter", 400.0, 350.0, |cx| {
        let count = cx.state(0i32);
        let label_text = cx.state("Rust".to_string());

        vstack![
            text(&format!("Hello, {}!", label_text.get()))
                .bold()
                .size(28.0)
                .foreground(Color::BLUE),
            text(&format!("{}", count.get()))
                .size(64.0)
                .foreground(if count.get() >= 0 {
                    Color::WHITE
                } else {
                    Color::RED
                }),
            divider(),
            button("+1", count.bind(|n| n + 1)),
            button("-1", count.bind(|n| n - 1)),
            button("Reset", count.set_to(0)),
            spacer(),
            text("State lives in Rust. Rendering by SwiftUI.")
                .size(11.0)
                .color(Color::GRAY),
        ]
        .padding(24.0)
        .bg(Color::DARKER)
    });
}
