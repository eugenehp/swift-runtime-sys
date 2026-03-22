mod views;

use swiftui::hstack;
use swiftui::prelude::*;

fn main() {
    App::new("Dashboard", 800.0, 600.0)
        .min_size(600.0, 400.0)
        .run(|cx| {
            let tab = cx.state(0i32);
            hstack![
                views::sidebar(&tab).frame(180.0, -1.0),
                divider(),
                match tab.get() {
                    0 => views::overview(cx),
                    1 => views::projects(cx),
                    2 => views::activity_feed(cx),
                    _ => views::settings_page(cx),
                },
            ]
            .bg(DARKER)
        });
}
