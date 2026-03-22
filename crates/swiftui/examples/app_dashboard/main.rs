//! Dashboard App — cards, stats, charts, nested composition.
//!
//! Structure:
//!   main.rs          — app entry, layout
//!   views/mod.rs     — re-exports
//!   views/header.rs  — top bar with title + stats
//!   views/cards.rs   — stat cards, project cards
//!   views/activity.rs — activity feed
//!   views/sidebar.rs — navigation sidebar

mod views;

use swiftui::prelude::*;

fn main() {
    App::new("Dashboard", 800.0, 600.0)
        .min_size(600.0, 400.0)
        .on_appear(|| println!("Dashboard loaded"))
        .run(|cx| {
            let tab = cx.state(0i32);

            hstack![
                // Sidebar
                views::sidebar(cx, &tab).frame(180.0, -1.0),
                divider(),
                // Content
                match tab.get() {
                    0 => views::overview(cx),
                    1 => views::projects(cx),
                    2 => views::activity_feed(cx),
                    _ => views::settings_page(cx),
                },
            ]
            .bg(Color::DARKER)
        });
}
