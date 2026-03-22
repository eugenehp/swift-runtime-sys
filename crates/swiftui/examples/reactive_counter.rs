use swiftui::prelude::*;
use swiftui::{txt, vstack};

fn main() {
    app("Counter", 400.0, 350.0, |cx| {
        let count = cx.state(0i32);

        vstack![
            txt!("Count: {}", count.get())
                .bold()
                .size(64.0)
                .foreground(if count.get() >= 0 { WHITE } else { RED }),
            divider(),
            button("+1", count.increment()),
            button("-1", count.decrement()),
            button("Reset", count.set_to(0)),
            spacer(),
            txt!("State lives in Rust. Rendering by SwiftUI.").style(Caption),
        ]
        .padding(24.0)
        .bg(DARKER)
    });
}
