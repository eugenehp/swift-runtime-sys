//! Reactive counter — state updates trigger automatic UI rebuilds.
//!
//! Build helper: swift_helper/build.sh

use swiftui::dsl::*;
use swiftui::state::*;
use swiftui::vstack;

fn main() {
    let store = Store::new();
    let count = store.create(0i32);
    let name = store.create(String::from("Rust"));

    reactive_window("Reactive Counter", 400.0, 350.0, &store, move |ctx| {
        let n: i32 = ctx.get(&count);
        let who: String = ctx.get(&name);

        let count_for_inc = count;
        let store_for_inc = ctx.store.clone();

        let count_for_dec = count;
        let store_for_dec = ctx.store.clone();

        let count_for_reset = count;
        let store_for_reset = ctx.store.clone();

        vstack![
            text(&format!("Hello, {who}!"))
                .bold()
                .size(28.0)
                .foreground(Color::BLUE),
            text(&format!("Count: {n}"))
                .size(48.0)
                .foreground(if n >= 0 { Color::WHITE } else { Color::RED }),
            spacer(),
            state_button("+1", move || {
                store_for_inc.update(&count_for_inc, |n| n + 1);
            }),
            state_button("-1", move || {
                store_for_dec.update(&count_for_dec, |n| n - 1);
            }),
            state_button("Reset", move || {
                store_for_reset.set(&count_for_reset, 0);
            }),
            spacer(),
            text("Click buttons — UI updates reactively!")
                .size(12.0)
                .color(Color::GRAY),
        ]
        .padding(24.0)
        .bg(Color::DARKER)
    });
}
