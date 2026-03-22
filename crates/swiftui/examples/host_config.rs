//! Host configuration examples — different window styles.

use swiftui::prelude::*;

fn main() {
    // Pick which demo to run based on CLI arg
    let demo = std::env::args().nth(1).unwrap_or("default".into());

    match demo.as_str() {
        "borderless" => borderless_demo(),
        "floating" => floating_demo(),
        "transparent" => transparent_demo(),
        "fixed" => fixed_demo(),
        "callbacks" => callbacks_demo(),
        _ => default_demo(),
    }
}

fn default_demo() {
    App::new("Default Window", 500.0, 400.0)
        .min_size(300.0, 200.0)
        .on_appear(|| println!("Window appeared!"))
        .run(|cx| {
            let count = cx.state(0i32);
            vstack![
                text("Default Window Style").style(StylePreset::Title),
                text("Standard titled, resizable, closable").style(StylePreset::Subtitle),
                divider(),
                text_fmt!("Count: {count}").size(32.0),
                button("+1", count.bind(|n| n + 1)),
                spacer(),
            ]
            .padding(20.0)
            .bg(Color::DARKER)
        });
}

fn borderless_demo() {
    App::new("", 600.0, 400.0).borderless().run(|_cx| {
        vstack![
            spacer(),
            text("Borderless Window").style(StylePreset::Title),
            text("No title bar, no chrome").style(StylePreset::Subtitle),
            spacer(),
        ]
        .frame_max()
        .bg(Color::DARK)
    });
}

fn floating_demo() {
    App::new("Floating", 300.0, 200.0)
        .floating()
        .fixed_size()
        .run(|_cx| {
            vstack![
                text("Floating Window").style(StylePreset::Heading),
                text("Always on top").style(StylePreset::Caption),
            ]
            .padding(20.0)
            .bg(Color::DARK)
        });
}

fn transparent_demo() {
    App::new("", 400.0, 300.0).transparent().run(|_cx| {
        vstack![
            text("Transparent Background").style(StylePreset::Title),
            text("See-through window").style(StylePreset::Subtitle),
        ]
        .padding(40.0)
        .bg(rgba(0.0, 0.0, 0.0, 0.7))
        .rounded(20.0)
    });
}

fn fixed_demo() {
    App::new("Fixed Size", 400.0, 300.0)
        .fixed_size()
        .run(|_cx| {
            vstack![
                text("Fixed Size").style(StylePreset::Title),
                text("Cannot resize this window").style(StylePreset::Subtitle),
            ]
            .padding(20.0)
            .bg(Color::DARKER)
        });
}

fn callbacks_demo() {
    App::new("Lifecycle", 400.0, 300.0)
        .on_appear(|| println!("👋 Window appeared!"))
        .on_disappear(|| println!("🚪 Window disappeared!"))
        .run(|_cx| {
            vstack![
                text("Lifecycle Callbacks").style(StylePreset::Title),
                text("Check the terminal for appear/disappear events").style(StylePreset::Subtitle),
            ]
            .padding(20.0)
            .bg(Color::DARKER)
        });
}
