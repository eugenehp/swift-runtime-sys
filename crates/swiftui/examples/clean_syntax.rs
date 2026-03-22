//! Clean syntax demo — short enum names via prelude.
//!
//! Before: .style(StylePreset::Elevated).foreground(Color::BLUE).font(18.0, FontWeight::Bold)
//! After:  .style(Elevated).foreground(BLUE).font(18.0, Bold)

use swiftui::prelude::*;

fn main() {
    App::new("Clean Syntax", 500.0, 500.0)
        .min_size(400.0, 300.0)
        .run(|cx| {
            let count = cx.state(0i32);

            vstack![
                // Short color names
                text("Short Colors").style(Title),
                hstack![
                    color(RED).frame(30.0, 30.0).clip_circle(),
                    color(GREEN).frame(30.0, 30.0).clip_circle(),
                    color(BLUE).frame(30.0, 30.0).clip_circle(),
                    color(YELLOW).frame(30.0, 30.0).clip_circle(),
                    color(PURPLE).frame(30.0, 30.0).clip_circle(),
                ],
                divider(),
                // Short style names
                text("Style Presets").style(Heading),
                text("Title style").style(Title),
                text("Subtitle style").style(Subtitle),
                text("Caption style").style(Caption),
                text("Body style").style(Body),
                divider(),
                // Short font weight names
                text("Font Weights").style(Heading),
                text("Regular").font(16.0, Regular),
                text("Medium").font(16.0, Medium),
                text("Semibold").font(16.0, Semibold),
                text("Bold").font(16.0, Bold),
                text("Heavy").font(16.0, Heavy),
                divider(),
                // Card styles
                text("Cards").style(Heading),
                vstack![
                    text_fmt!("Count: {count}").size(24.0).foreground(BLUE),
                    button("+1", count.bind(|n| n + 1)),
                ]
                .style(Elevated),
                text("Pill badge").style(Pill),
                spacer(),
            ]
            .style(Page)
        });
}
