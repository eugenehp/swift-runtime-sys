//! Full coverage demo — binding, list, sheet, animation, visual effects.

use swiftui::prelude::*;
use swiftui::{hstack, list, txt, vstack};

fn main() {
    app("Full Coverage", 550.0, 700.0, |cx| {
        let name = cx.state("".to_string());
        let agreed = cx.state(false);
        let volume = cx.state(0.5f32);
        let show_sheet = cx.state(false);
        let show_alert = cx.state(false);
        let items = cx.state(vec![
            "First item".to_string(),
            "Second item".to_string(),
            "Third item".to_string(),
        ]);

        vstack![
            txt!("Full Coverage Demo").style(Title),
            divider(),
            // ── Two-way binding ──
            group_box(
                "Two-Way Binding",
                vstack![
                    bound_textfield("Your name", &name),
                    {
                        let n = name.get();
                        let display = if n.is_empty() {
                            "stranger".to_string()
                        } else {
                            n
                        };
                        txt!("Hello, {display}!").foreground(BLUE)
                    },
                    bound_toggle("I agree to terms", &agreed),
                    show_if(agreed.get(), || txt!("✅ Agreed!").foreground(GREEN)),
                    bound_slider(&volume, 0.0, 1.0),
                    txt!("Volume: {:.0}%", volume.get() * 100.0).style(Caption),
                ]
            ),
            divider(),
            // ── List ──
            group_box(
                "SwiftUI List",
                vstack![
                    list![for_each(&items.get(), |item| {
                        txt!("• {item}").on_tap(|| println!("Tapped item"))
                    }),],
                    hstack![
                        button("Add", {
                            let items = items.clone();
                            move || items.push(format!("Item {}", items.len() + 1))
                        }),
                        button("Remove", {
                            let items = items.clone();
                            move || {
                                if !items.is_empty() {
                                    items.remove(items.len() - 1);
                                }
                            }
                        }),
                    ],
                ]
            ),
            divider(),
            // ── Sheet & Alert ──
            hstack![
                button("Show Sheet", show_sheet.set_to(true)),
                button("Show Alert", show_alert.set_to(true)),
            ],
            // ── Visual effects ──
            group_box(
                "Effects",
                hstack![
                    txt!("Blur").padding(8.0).bg(BLUE).rounded(6.0).blur(2.0),
                    txt!("Bright")
                        .padding(8.0)
                        .bg(PURPLE)
                        .rounded(6.0)
                        .brightness(0.3),
                    txt!("Gray")
                        .padding(8.0)
                        .bg(RED)
                        .rounded(6.0)
                        .grayscale(1.0),
                    txt!("Bouncy").padding(8.0).bg(GREEN).rounded(6.0).bouncy(),
                ]
            ),
            // ── Extra views ──
            group_box(
                "More Controls",
                vstack![secure_field("Password", ""), stepper("Quantity", 1, 0, 10),]
            ),
            spacer(),
        ]
        .style(Page)
        .sheet(
            vstack![
                txt!("Sheet Content").style(Title),
                txt!("This is a modal sheet").style(Subtitle),
                button("Dismiss", show_sheet.set_to(false)),
            ]
            .padding(24.0),
            show_sheet.get(),
        )
        .alert("Hello!", "This is an alert from Rust.", show_alert.get())
    });
}
