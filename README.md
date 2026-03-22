# swift-runtime-sys

Build native SwiftUI and RealityKit apps from Rust. Reactive state, declarative DSL, 100% pixel parity with native Swift, zero configuration.

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

No manual setup — `build.rs` auto-compiles the Swift helper and auto-detects your SDK version.

## Workspace

```
crates/
├── swift-runtime-sys   Raw FFI to Swift runtime (490+ symbols, arm64 asm thunks)
├── swift-runtime       Safe wrappers (Metadata, types, Retained)
├── swiftui-sys         Raw SwiftUI FFI (function pointers via dlsym)
├── swiftui-macros      Proc macros (#[derive(View)], text_fmt!)
├── swiftui             Ergonomic SwiftUI DSL with reactive state
├── realitykit-sys      Raw RealityKit FFI
├── realitykit           Ergonomic RealityKit 3D scene builder
└── swift-bridge-gen    Code generator for any Apple framework
```

## SwiftUI

### Views

```rust
text("Hello")                          // Text
text("Hello").bold().size(24.0)        // Styled
image("star.fill")                     // SF Symbol
label("Settings", "gear")             // Icon + text
button("Click", || println!("!"))      // Button
button("+1", count.increment())        // Reactive button
toggle("Dark mode", true)             // Toggle
textfield("Search...", "")            // Text field
slider(0.5, 0.0, 1.0)                // Slider
progress(0.7, 1.0)                   // Progress bar
link("Rust", "https://rust-lang.org") // URL link
color(RED)                            // Color swatch
spacer()                              // Flexible space
divider()                             // Separator
```

### Stacks

```rust
vstack![view1, view2, view3]
hstack![view1, view2]
zstack![background, foreground]
```

### Modifiers

```rust
.padding(16.0)        .frame(200.0, 100.0)     .bg(DARK)
.foreground(BLUE)     .rounded(12.0)            .opacity(0.5)
.shadow(BLACK, 8.0, 0.0, 4.0)                   .border(GRAY, 1.0)
.offset(10.0, -5.0)  .scale(1.5)               .rotation(45.0)
.clip_circle()        .hidden()                  .disabled(true)
.overlay(badge)       .font(18.0, Bold)          .scroll()
.style(Elevated)      .frame_max()
```

### Style Presets

```rust
.style(Title)       // Bold 28pt white
.style(Subtitle)    // 14pt gray
.style(Caption)     // 11pt dim
.style(Heading)     // 22pt semibold
.style(CardDark)    // padding + dark bg + rounded
.style(Elevated)    // card + shadow
.style(Pill)        // small rounded pill
.style(Page)        // padding + scroll + dark bg
```

### Reactive State

```rust
app("Counter", 400.0, 300.0, |cx| {
    let count = cx.state(0i32);
    let name = cx.state("World".into());

    vstack![
        text_fmt!("Hello {name}!").style(Title),
        text_fmt!("Count: {count}").size(48.0),
        button("+1", count.increment()),
        button("-1", count.decrement()),
        button("Reset", count.set_to(0)),
        button("×2", count.bind(|n| n * 2)),
    ]
});
```

State changes automatically trigger UI rebuilds.

### Conditionals and Loops

```rust
view_if(is_premium, || text("Premium"), || text("Free"))
show_if(has_badge, || text("🏆"))
for_each(&items, |item| text(item))
for_each_enumerated(&items, |i, item| text(&format!("{i}. {item}")))
```

### Navigation

```rust
#[derive(Clone, PartialEq)]
enum Screen { Home, Settings, Detail(i32) }

app("Nav", 400.0, 600.0, |cx| {
    let screen = cx.state(Screen::Home);
    navigator(&screen, |s| match s {
        Screen::Home => vstack![
            text("Home").style(Title),
            nav_button("Settings", &screen, Screen::Settings),
        ],
        Screen::Settings => vstack![
            back_button(&screen, Screen::Home),
            toggle("Notifications", true),
        ],
        Screen::Detail(id) => vstack![
            back_button(&screen, Screen::Home),
            text(&format!("Item #{id}")),
        ],
    }).style(Page)
});
```

### App Configuration

```rust
// Simple
app("Title", 400.0, 300.0, |cx| { ... });

// Configured
App::new("Title", 800.0, 600.0)
    .borderless()                    // or .fullscreen(), .floating(), .transparent()
    .min_size(400.0, 300.0)
    .on_appear(|| println!("ready"))
    .run(|cx| { ... });

// Scene-based (SwiftUI App protocol)
SceneApp::new()
    .window("main", "My App", 800.0, 600.0, |cx| { ... })
    .settings("Preferences", |cx| { ... })
    .menu_bar("Status", "star.fill", |cx| { ... })
    .launch();
```

### Colors

```rust
RED  GREEN  BLUE  YELLOW  PURPLE  GRAY  WHITE  BLACK  DARK  DARKER  CLEAR
rgb(0.2, 0.4, 0.8)
rgba(1.0, 0.0, 0.0, 0.5)
hex(0x3366CC)
```

### Multi-File Composition

```rust
// views/mod.rs
mod sidebar;
mod cards;
pub use sidebar::*;
pub use cards::*;

// views/cards.rs
pub fn stat_card(title: &str, value: &str, color: Color) -> View {
    vstack![
        text(title).style(Caption),
        text(value).bold().size(22.0).foreground(color),
    ].style(CardDark)
}

// main.rs
mod views;
app("Dashboard", 800.0, 600.0, |cx| {
    hstack![
        views::sidebar(cx),
        views::stat_card("Users", "12,847", BLUE),
    ]
});
```

## RealityKit

```rust
use realitykit::prelude::*;

let rk = RealityKit::new()?;

let sphere = rk.sphere(0.5).at(0.0, 0.5, 0.0);
let floor = rk.plane(10.0, 10.0);
let label = rk.text("Hello 🦀", 0.1, 24.0).at(-1.0, 1.5, 0.0);
let light = rk.point_light().at(2.0, 3.0, 2.0);

rk.anchor(0.0, 0.0, -3.0)
    .add(&floor)
    .add(&sphere)
    .add(&label)
    .add(&light);
```

Primitives: `sphere`, `cube`, `box_`, `plane`, `cone`, `cylinder`, `text`
Materials: `Material::simple(r,g,b).roughness(0.3).metallic()`
Lights: `point_light`, `directional_light`

## Bridge Generator

Auto-generate bindings for any Apple framework:

```bash
# Dump API
xcrun swift-api-digester -dump-sdk -module Foundation \
  -target arm64-apple-macosx26.0 \
  -sdk $(xcrun -sdk macosx --show-sdk-path) \
  -o foundation_api.json

# Generate
cargo run -p swift-bridge-gen -- foundation_api.json --types URL,UUID,Date,Data
```

Produces `FoundationBridge.swift` + `foundation.rs` with typed constructors, getters, and RAII wrappers.

## Platform Support

|  | macOS | iOS Simulator | iOS Device | visionOS |
|--|-------|--------------|------------|----------|
| SwiftUI | ✅ | ✅ | ✅ | future |
| RealityKit | ✅ | ✅ | ✅ | future |
| Reactive state | ✅ | ✅ | ✅ | ✅ |
| Pixel parity tests | ✅ | ✅ | — | — |

### Deployment Targets

Auto-detected from your SDK. Pin with features:

```toml
[dependencies]
swiftui = { path = "crates/swiftui", features = ["macos-26"] }
```

Or environment variables:

```bash
SWIFTUI_MACOS_VERSION=15.0 cargo build
SWIFTUI_IOS_VERSION=18.0 cargo build --target aarch64-apple-ios
```

Available features: `macos-15`, `macos-26`, `ios-18`, `ios-26`

## Tests

```bash
cargo test --workspace -- --test-threads=1  # All 91 tests

cargo test -p swift-runtime-sys   # 66 runtime FFI tests
cargo test -p swift-runtime       # 13 safe API tests
cargo test -p swiftui --test pixel_parity  # 6 pixel parity (100% match)
```

## Examples

```bash
cargo run -p swiftui --example showcase          # Navigation + styles + state
cargo run -p swiftui --example reactive_counter  # Reactive counter
cargo run -p swiftui --example reactive_todo     # TODO app with progress
cargo run -p swiftui --example app_dashboard     # Multi-file dashboard
cargo run -p swiftui --example app_notes         # Multi-file notes app
cargo run -p swiftui --example clean_syntax      # Short enum names demo
cargo run -p swiftui --example host_config       # Window styles demo
cargo run -p swiftui --example scene_app         # Scene-based app
cargo run -p swiftui --example full_demo         # All widgets + modifiers
```

## Architecture

```
Your Rust App
    use swiftui::prelude::*;
    app("Title", w, h, |cx| { ... })
         │
    swiftui crate (DSL, state, navigation, styles)
         │
    swiftui-sys (function pointers via dlsym)
         │
    Swift Helper (auto-compiled by build.rs, ~400 lines)
         │
    SwiftUI.framework / RealityKit.framework
         │
    Native macOS / iOS rendering
```

## License

Apache-2.0
