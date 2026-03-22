//! Showcase: every DX feature in one app.
//!
//! - Reactive state with cx.state() / .bind() / .set_to()
//! - text_fmt! macro for state interpolation
//! - Style presets
//! - Conditional views (when, when_else, for_each)
//! - Navigation (navigator, nav_button, back_button)

use swiftui::prelude::*;

#[derive(Clone, PartialEq)]
enum Screen {
    Home,
    Counter,
    TodoList,
    Colors,
}

fn main() {
    app("Showcase", 480.0, 600.0, |cx| {
        let screen = cx.state(Screen::Home);

        navigator(&screen, |s| match s {
            Screen::Home => home_screen(cx, &screen),
            Screen::Counter => counter_screen(cx, &screen),
            Screen::TodoList => todo_screen(cx, &screen),
            Screen::Colors => colors_screen(cx, &screen),
        })
        .style(StylePreset::Page)
    });
}

fn home_screen(cx: &Cx, screen: &State<Screen>) -> View {
    vstack![
        text("🦀 SwiftUI from Rust").style(StylePreset::Title),
        text("Full DX showcase").style(StylePreset::Subtitle),
        divider(),
        nav_button("Counter Demo", screen, Screen::Counter),
        nav_button("TODO List", screen, Screen::TodoList),
        nav_button("Color Palette", screen, Screen::Colors),
        spacer(),
        text("swiftui-macros + reactive state + navigation").style(StylePreset::Caption),
    ]
}

fn counter_screen(cx: &Cx, screen: &State<Screen>) -> View {
    let count = cx.state(0i32);
    let step = cx.state(1i32);

    let n = count.get();
    let s = step.get();

    vstack![
        back_button(screen, Screen::Home),
        divider(),
        text_fmt!("Count: {count}").size(48.0).foreground(when_else(
            n >= 0,
            || Color::WHITE,
            || Color::RED
        )),
        text_fmt!("Step: {step}").style(StylePreset::Subtitle),
        divider(),
        hstack![
            button(&format!("+{s}"), count.bind(move |n| n + s)),
            button(&format!("-{s}"), count.bind(move |n| n - s)),
            button("Reset", count.set_to(0)),
        ],
        hstack![
            button("Step=1", step.set_to(1)),
            button("Step=5", step.set_to(5)),
            button("Step=10", step.set_to(10)),
        ],
        spacer(),
    ]
}

fn todo_screen(cx: &Cx, screen: &State<Screen>) -> View {
    let items = cx.state(vec![
        ("Learn Rust", true),
        ("Build SwiftUI bridge", true),
        ("Add reactive state", true),
        ("Style presets", true),
        ("Navigation", true),
        ("Proc macros", false),
        ("Ship v1.0", false),
    ]);

    let list = items.get();
    let done = list.iter().filter(|(_, d)| *d).count();
    let total = list.len();

    vstack![
        back_button(screen, Screen::Home),
        divider(),
        text("📋 TODO").style(StylePreset::Title),
        text(&format!("{done}/{total} completed")).style(StylePreset::Subtitle),
        progress(done as f32, total as f32),
        divider(),
        for_each(&list, |(title, done)| {
            let icon = if *done { "✅" } else { "⬜" };
            let c = if *done { Color::GREEN } else { Color::WHITE };
            text(&format!("{icon} {title}")).foreground(c)
        }),
        divider(),
        hstack![
            button("Complete next", {
                let items = items.clone();
                move || {
                    items.update(|list| {
                        let mut new = list.clone();
                        if let Some(item) = new.iter_mut().find(|(_, d)| !d) {
                            item.1 = true;
                        }
                        new
                    });
                }
            }),
            button("Reset all", {
                let items = items.clone();
                move || {
                    items.update(|list| list.iter().map(|(s, _)| (s.clone(), false)).collect());
                }
            }),
        ],
        spacer(),
    ]
}

fn colors_screen(cx: &Cx, screen: &State<Screen>) -> View {
    let colors = [
        ("Red", Color::RED),
        ("Green", Color::GREEN),
        ("Blue", Color::BLUE),
        ("Yellow", Color::YELLOW),
        ("Purple", Color::PURPLE),
    ];

    vstack![
        back_button(screen, Screen::Home),
        divider(),
        text("🎨 Color Palette").style(StylePreset::Title),
        for_each_h(&colors, |(name, c)| {
            vstack![
                color(*c)
                    .frame(50.0, 50.0)
                    .clip_circle()
                    .shadow(*c, 4.0, 0.0, 2.0),
                text(name).style(StylePreset::Caption),
            ]
        }),
        divider(),
        // Style preset demos
        text("Style presets:").style(StylePreset::Heading),
        text("CardDark").style(StylePreset::CardDark),
        text("Pill style").style(StylePreset::Pill),
        text("Elevated").style(StylePreset::Elevated),
        spacer(),
    ]
}

// Helper: when_else for colors (returns Color not View)
fn when_else<T>(cond: bool, if_true: impl FnOnce() -> T, if_false: impl FnOnce() -> T) -> T {
    if cond {
        if_true()
    } else {
        if_false()
    }
}
