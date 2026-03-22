//! DX Improvements demo — showcases all ergonomic fixes.

use swiftui::prelude::*;
use swiftui::{hstack, txt, v, vstack};

fn main() {
    app("DX Improvements", 500.0, 600.0, |cx| {
        let count = cx.state(0i32);
        let items = cx.state(vec![
            "Apple".to_string(),
            "Banana".to_string(),
            "Cherry".to_string(),
        ]);
        let show_list = cx.state(true);

        let n = count.get();
        let list = items.get();

        vstack![
            // Fix 1: txt! macro — unified text with inline formatting
            txt!("Count: {}", n).bold().size(32.0),
            txt!("Items: {}", list.len()).style(Subtitle),
            divider(),
            // Fix 2: v!() macro — no more .into()
            if n >= 0 {
                v!(txt!("{n}").foreground(GREEN))
            } else {
                v!(txt!("{n}").foreground(RED))
            },
            // Or use view_if (also works with TextView, no .into()):
            view_if(
                n > 10,
                || txt!("Big number!").foreground(YELLOW),
                || txt!("Keep going").foreground(GRAY),
            ),
            divider(),
            // Fix 3: State<Vec> helpers — no more verbose .update(|list| { let mut new = ... })
            hstack![
                button("Add item", {
                    let items = items.clone();
                    let n = items.len();
                    move || items.push(format!("Item {}", n + 1))
                }),
                button("Remove last", {
                    let items = items.clone();
                    move || {
                        let len = items.len();
                        if len > 0 {
                            items.remove(len - 1);
                        }
                    }
                }),
                button("Clear", {
                    let items = items.clone();
                    move || items.clear()
                }),
            ],
            // Fix 4: show_if — cleaner than when()
            show_if(show_list.get(), || {
                for_each(&list, |item| {
                    txt!("• {item}").on_tap(|| println!("Tapped!")) // Fix 5: gesture support!
                })
            }),
            divider(),
            // Fix 5: .on_tap() on any view
            color(BLUE).frame(100.0, 50.0).rounded(8.0).on_tap({
                let count = count.clone();
                move || count.update(|n| n + 10)
            }),
            txt!("Tap the blue box for +10").style(Caption),
            divider(),
            // Fix 6: .styles() for composing multiple presets
            txt!("Composed styles").styles(&[Title, CardDark]),
            spacer(),
            // Counter controls
            hstack![
                button("+1", count.increment()),
                button("-1", count.decrement()),
                button("Reset", count.set_to(0)),
                button("Toggle list", show_list.toggle()),
            ],
        ]
        .style(Page)
    });
}
