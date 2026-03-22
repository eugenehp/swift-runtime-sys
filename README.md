# swift-runtime-sys

Build native Apple apps from Rust. SwiftUI, RealityKit, Combine, persistence — all with a declarative DSL, reactive state, and 100% pixel parity.

```rust
use swiftui::prelude::*;

fn main() {
    app("My App", 400.0, 300.0, |cx| {
        let count = cx.state(0i32);
        vstack![
            text_fmt!("Count: {count}").bold().size(48.0),
            button("+1", count.increment()),
            button("Reset", count.set_to(0)),
        ].style(Page)
    });
}
```

```bash
git clone https://github.com/eugenehp/swift-runtime-sys
cd swift-runtime-sys
cargo run -p swiftui --example showcase
```

Zero setup — `build.rs` auto-compiles the Swift helper and auto-detects your SDK.

## Crates

| Crate | Description |
|-------|-------------|
| [`swift-runtime-sys`](crates/swift-runtime-sys) | Raw FFI to the Swift runtime (490+ symbols, arm64 asm thunks) |
| [`swift-runtime`](crates/swift-runtime) | Safe Rust wrappers (Metadata, types, Retained) |
| [`swiftui-sys`](crates/swiftui-sys) | Raw SwiftUI FFI (dlsym function pointers) |
| [`swiftui-macros`](crates/swiftui-macros) | Proc macros (`text_fmt!`, `#[derive(View)]`) |
| [`swiftui`](crates/swiftui) | SwiftUI DSL — 142/142 API coverage |
| [`realitykit-sys`](crates/realitykit-sys) | Raw RealityKit FFI |
| [`realitykit`](crates/realitykit) | RealityKit 3D scene builder |
| [`combine-rs`](crates/combine) | Combine publisher/subscriber bridge |
| [`swift-data`](crates/swift-data) | Persistent key-value storage |
| [`swift-bridge-gen`](crates/swift-bridge-gen) | Auto-generate bindings for any Apple framework |

## SwiftUI (142/142 APIs)

### Views

```rust
text("Hello")                 image("star.fill")        label("Settings", "gear")
button("Click", || {})        toggle("On", true)        slider(0.5, 0.0, 1.0)
textfield("Name", "")         secure_field("Pass", "")  text_editor("Long text")
stepper("Qty", 1, 0, 10)      progress(0.7, 1.0)        link("Rust", "https://...")
color(RED)                    spacer()                  divider()
async_image("https://...")    photos_picker("Photo", |d| {})
map(37.7, -122.4, 0.1, 0.1)  video_player("https://...")
share_link("Share", "url")   empty_view()              content_unavailable(...)
group_box("Title", content)  disclosure_group(...)      labeled_content(...)
date_picker("Date")          color_picker("Color")
```

### Stacks & Layout

```rust
vstack![a, b, c]     hstack![a, b]     zstack![bg, fg]
grid(3, children)    hgrid(2, children)
form(children)       section("Title", children)
list![items]         tabview(vec![Tab::new("Home", "house", view)])
navigation_split_view(sidebar, detail)
```

### Modifiers

```rust
.padding(16.0)    .frame(200.0, 100.0)    .bg(DARK)         .foreground(BLUE)
.rounded(12.0)    .opacity(0.5)           .shadow(...)       .border(GRAY, 1.0)
.offset(10.0, 5.0) .scale(1.5)           .rotation(45.0)    .blur(3.0)
.brightness(0.2)  .saturation(0.5)        .grayscale(1.0)    .color_invert()
.clip_circle()    .clipped()              .mask(view)        .blend_mode(1)
.hidden()         .disabled(true)         .overlay(view)     .overlay_aligned(v, 2)
.font(18.0, Bold) .bold_mod()             .italic_mod()      .line_limit(2)
.truncation_mode(0) .minimum_scale_factor(0.5)               .fixed_size_mod()
.aspect_ratio(1.0, true)                  .tint(BLUE)        .badge(5)
.help("tooltip")  .keyboard_shortcut("r") .focusable()       .drawing_group()
.allows_hit_testing(true)                 .content_shape(0)
.navigation_title("Title")               .navigation_stack() .toolbar(content)
.context_menu(m)  .popover(v, shown)      .sheet(v, shown)   .alert(t, m, shown)
.confirmation_dialog(t, shown, acts)      .searchable(|q| {}) .refreshable(|| {})
.swipe_delete(|| {})                      .swipe_actions(v, leading)
.on_tap(|| {})    .on_long_press(|| {})   .on_drag(|x,y| {}) .on_magnify(|s| {})
.on_rotate(|d| {})                        .on_appear(|| {})   .on_disappear(|| {})
.task(|| {})      .scroll()               .scroll_id("id")
.preferred_color_scheme(true)             .ignores_safe_area()
.safe_area_inset_bottom(v)                .list_row_background(v)
.list_row_separator(false)                .container_relative_frame(0)
.matched_geometry("id")                   .symbol_bounce()    .symbol_pulse()
.animated()       .spring()               .bouncy()           .ease_in(0.3)
.ease_out(0.3)    .ease_in_out(0.3)       .linear(0.5)        .bezier(...)
.spring_params(0.5, 0.3)                  .transition_opacity() .transition_slide()
.phase_animate(3) .phase_animate_scale(&[...])                .keyframe(&[...], t)
.accessibility_label("x") .accessibility_hint("x")            .accessibility_hidden(t)
.style(Elevated)  .styles(&[Title, CardDark])
```

### Reactive State

```rust
app("App", 400.0, 300.0, |cx| {
    let count = cx.state(0i32);
    let name = cx.state("World".into());
    let items = cx.state(vec!["a".into(), "b".into()]);

    vstack![
        text_fmt!("Hello {name}! Count: {count}").style(Title),
        button("+1", count.increment()),
        button("-1", count.decrement()),
        button("Reset", count.set_to(0)),
        button("×2", count.bind(|n| n * 2)),
        bound_textfield("Name", &name),          // two-way binding
        bound_toggle("Agree", &agreed),
        bound_slider(&volume, 0.0, 1.0),
        bound_picker("Sort", &["Name","Date"], &sort),
        bound_color_picker("Color", &r, &g, &b),
        bound_date_picker("Date", &timestamp),
    ]
});

// Animated state changes
animate(|| count.set(0));
animate_spring(|| offset.set(100.0));
with_animation(AnimCurve::EaseInOut, 0.3, || { ... });

// Vec state helpers
items.push("new".into());
items.remove(0);
items.update_at(1, |s| s.to_uppercase());
items.clear();

// Persistence
app_storage_set("theme", "dark");
app_storage_get("theme"); // Some("dark")

// Focus management
let focus = FocusManager::new();
focusable_textfield("Email", &email, "email", &focus);
focus.focus("email");

// Timer
let _timer = RustTimer::start(1.0, || println!("tick"));
```

### Navigation

```rust
// State-based
navigator(&screen, |s| match s { ... })
nav_button("Go", &screen, Screen::Detail(1))
back_button(&screen, Screen::Home)

// SwiftUI NavigationStack
view.navigation_title("Title").navigation_stack()
navigation_link("Detail", detail_view)
```

### App Configuration

```rust
app("Simple", 400.0, 300.0, |cx| { ... });           // simple
App::new("Configured", 800.0, 600.0)                   // configured
    .borderless().min_size(400.0, 300.0).run(|cx| { ... });
SceneApp::new()                                         // scene-based
    .window("main", "App", 800.0, 600.0, |cx| { ... })
    .settings("Prefs", |cx| { ... })
    .launch();
```

## RealityKit

```rust
let rk = RealityKit::new()?;
let sphere = rk.sphere(0.5).at(0.0, 0.5, 0.0);
let floor = rk.plane(10.0, 10.0);
let light = rk.point_light().at(2.0, 3.0, 2.0);
rk.anchor(0.0, 0.0, -3.0).add(&floor).add(&sphere).add(&light);
```

## Combine

```rust
let subject = combine_rs::Subject::new();
let _sub = subject.subscribe(|v| println!("Got: {v}"));
subject.send(42);

let current = combine_rs::CurrentValue::new(0);
current.set(10);
assert_eq!(current.get(), 10);
```

## Persistence

```rust
let db = swift_data::Store::new();
db.set("users", "name", "Alice");
db.set_int("stats", "launches", 5);
assert_eq!(db.get("users", "name"), Some("Alice".into()));
assert_eq!(db.get_int("stats", "launches"), 5);
```

## Bridge Generator

```bash
xcrun swift-api-digester -dump-sdk -module Foundation \
  -target arm64-apple-macosx26.0 -sdk $(xcrun -sdk macosx --show-sdk-path) \
  -o api.json
cargo run -p swift-bridge-gen -- api.json --types URL,UUID,Date
```

## Platform Support

| | macOS | iOS | visionOS |
|--|-------|-----|----------|
| SwiftUI | ✅ | ✅ | future |
| RealityKit | ✅ | ✅ | future |
| Combine | ✅ | ✅ | ✅ |
| Persistence | ✅ | ✅ | ✅ |

Auto-detects SDK version. Pin with `features = ["macos-26"]` or `SWIFTUI_MACOS_VERSION=15.0`.

## Tests

```bash
cargo test --workspace -- --test-threads=1  # 91 tests, 100% pixel parity
```

## License

Apache-2.0
