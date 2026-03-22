# swift-runtime-sys

Full Rust control over the Swift runtime on macOS and iOS. Build native SwiftUI apps from Rust with reactive state, pixel-perfect rendering, and a declarative DSL.

## Quick Start

```rust
use swiftui::prelude::*;

fn main() {
    app("My App", 400.0, 300.0, |cx| {
        let count = cx.state(0i32);

        vstack![
            text_fmt!("Count: {count}").bold().size(48.0),
            button("+1", count.bind(|n| n + 1)),
            button("Reset", count.set_to(0)),
        ]
        .padding(24.0)
        .bg(Color::DARKER)
    });
}
```

## Setup

```bash
# Clone
git clone https://github.com/eugenehp/swift-runtime-sys
cd swift-runtime-sys

# Build the Swift helper (required for SwiftUI)
swift_helper/build.sh

# Run the showcase
cargo run -p swiftui --example showcase

# Run tests
cargo test --workspace -- --test-threads=1
```

## Workspace

```
crates/
├── swift-runtime-sys/    Raw FFI bindings to the Swift runtime
├── swift-runtime/        Safe Rust wrappers (Metadata, types, Retained)
├── swiftui-sys/          Raw SwiftUI FFI (function pointers via dlsym)
├── swiftui-macros/       Proc macros (#[derive(View)], text_fmt!)
├── swiftui/              Ergonomic SwiftUI DSL
└── swift-bridge-gen/     Code generator for any Apple framework

swift_helper/
├── SwiftUIHelper.swift   @_cdecl view constructors + modifiers
├── SnapshotHelper.swift  Off-screen rendering for pixel tests
├── Platform.swift        macOS/iOS window hosting abstraction
└── build.sh              Build script [macos|ios-sim|ios|all]
```

## Crate Overview

### `swift-runtime-sys`

Complete FFI bindings to the Swift runtime — 490+ symbols across `libswiftCore`, `libswift_Concurrency`, and `libswiftRemoteMirror`.

- All ABI struct layouts (HeapObject, ValueWitnessTable, Metadata, descriptors)
- Arm64 inline assembly thunks for Swift CC, swiftasync CC, and error CC
- VTable and witness table dispatch
- Type introspection, dynamic casting, protocol conformance
- Concurrency (tasks, actors, executors, groups, async let, continuations)
- Concurrency executor hooks for custom Rust executors

### `swift-runtime`

Safe wrappers around `swift-runtime-sys`:

```rust
use swift_runtime::{types, metadata::Metadata};

let int_meta = types::int().unwrap();
println!("{:?}", int_meta);  // Metadata(Swift.Int, kind=Struct, size=8)
assert!(int_meta.is_pod());
assert_eq!(int_meta.descriptor_name(), Some("Int".to_string()));

let arr = types::array(&int_meta).unwrap();  // Array<Int>
```

### `swiftui`

Declarative SwiftUI DSL with reactive state:

#### Views
```rust
text("Hello")                          // Text
text("Hello").bold().size(24.0)        // Styled text
image("star.fill")                     // SF Symbol
label("Settings", "gear")             // Icon + text
button("Click", || println!("!"))      // Button with callback
toggle("Dark mode", true)             // Toggle switch
textfield("Search...", "")            // Text field
slider(0.5, 0.0, 1.0)                // Slider
progress(0.7, 1.0)                   // Progress bar
link("Rust", "https://rust-lang.org") // URL link
color(Color::RED)                     // Color swatch
spacer()                              // Flexible space
divider()                             // Line separator
```

#### Stacks
```rust
vstack![view1, view2, view3]          // Vertical
hstack![view1, view2]                 // Horizontal
zstack![background, foreground]       // Layered
```

#### Modifiers
```rust
view.padding(16.0)                    // Padding
view.frame(200.0, 100.0)             // Fixed size
view.bg(Color::DARK)                  // Background color
view.foreground(Color::BLUE)          // Text/content color
view.rounded(12.0)                    // Corner radius
view.opacity(0.5)                     // Transparency
view.shadow(Color::BLACK, 8.0, 0.0, 4.0)  // Drop shadow
view.offset(10.0, -5.0)              // Position offset
view.scale(1.5)                       // Scale transform
view.rotation(45.0)                   // Rotation (degrees)
view.border(Color::GRAY, 1.0)        // Border
view.clip_circle()                    // Circular clip
view.hidden()                         // Hide
view.disabled(true)                   // Disable interaction
view.overlay(badge)                   // Overlay another view
view.font(18.0, FontWeight::Bold)     // Font size + weight
view.scroll()                         // Wrap in ScrollView
view.style(StylePreset::Elevated)     // Apply style preset
```

#### Style Presets
```rust
text("Title").style(StylePreset::Title)        // Bold 28pt white
text("Sub").style(StylePreset::Subtitle)       // 14pt gray
text("Note").style(StylePreset::Caption)       // 11pt dim
card.style(StylePreset::CardDark)              // padding + dark bg + rounded
card.style(StylePreset::Elevated)              // card + shadow
page.style(StylePreset::Page)                  // padding + dark bg + scroll
text("Tag").style(StylePreset::Pill)           // small rounded pill
```

#### Reactive State
```rust
app("Counter", 400.0, 300.0, |cx| {
    let count = cx.state(0i32);          // Create reactive state
    let name = cx.state("World".into()); // Any Clone + Send type

    vstack![
        text_fmt!("Hello {name}!"),          // State interpolation macro
        text_fmt!("Count: {count}").size(48.0),
        button("+1", count.bind(|n| n + 1)), // Closure that updates state
        button("Reset", count.set_to(0)),    // Set to fixed value
    ]
});
// Clicking buttons triggers automatic UI rebuild
```

#### Conditional Views
```rust
when(is_premium, || text("Premium").style(StylePreset::Pill))
when_else(logged_in, || profile_view(), || login_view())
for_each(&items, |item| text(item))
for_each_enumerated(&items, |i, item| text(&format!("{i}. {item}")))
```

#### Navigation
```rust
#[derive(Clone, PartialEq)]
enum Screen { Home, Detail(i32), Settings }

app("Nav", 400.0, 600.0, |cx| {
    let screen = cx.state(Screen::Home);
    navigator(&screen, |s| match s {
        Screen::Home => vstack![
            text("Home").style(StylePreset::Title),
            nav_button("Settings", &screen, Screen::Settings),
            nav_button("Item 1", &screen, Screen::Detail(1)),
        ],
        Screen::Detail(id) => vstack![
            back_button(&screen, Screen::Home),
            text(&format!("Detail #{id}")).size(24.0),
        ],
        Screen::Settings => vstack![
            back_button(&screen, Screen::Home),
            toggle("Notifications", true),
        ],
    }).style(StylePreset::Page)
});
```

#### Colors
```rust
Color::RED                    // Named constants
Color::rgb(0.2, 0.4, 0.8)   // RGB
Color::rgba(1.0, 0.0, 0.0, 0.5)  // RGBA
rgb(0.2, 0.4, 0.8)          // Shorthand
hex(0x3366CC)                // Hex
```

### `swift-bridge-gen`

Auto-generate Rust↔Swift bridge code from any Apple framework:

```bash
# Dump the API
xcrun swift-api-digester -dump-sdk -module Foundation \
  -target arm64-apple-macosx15.0 \
  -sdk $(xcrun -sdk macosx --show-sdk-path) \
  -o foundation_api.json

# Generate bridge code
cargo run -p swift-bridge-gen -- foundation_api.json --types URL,UUID,Date
```

Generates:
- `FoundationBridge.swift` — `@_cdecl` wrappers for constructors, getters, methods
- `foundation.rs` — `extern "C"` FFI + RAII `Owned` wrapper + typed structs

## Platform Support

|  | macOS | iOS Simulator | iOS Device |
|--|-------|--------------|------------|
| SwiftUI views | ✅ | ✅ (needs Xcode) | ✅ (needs Xcode) |
| Reactive state | ✅ | ✅ | ✅ |
| Pixel parity tests | ✅ | ✅ | — |
| Runtime bindings | ✅ | ✅ | ✅ |

```bash
swift_helper/build.sh macos     # Default
swift_helper/build.sh ios-sim   # Needs Xcode
swift_helper/build.sh ios       # Needs Xcode
swift_helper/build.sh all       # All platforms
```

The SwiftUI DSL code is identical across platforms. Only window hosting differs (NSWindow vs UIWindow), handled by `Platform.swift`.

## Tests

```bash
# All tests (91 total)
cargo test --workspace -- --test-threads=1

# Runtime FFI tests (66)
cargo test -p swift-runtime-sys -- --test-threads=1

# Safe API tests (13)
cargo test -p swift-runtime -- --test-threads=1

# Pixel parity tests (6) — 100% match
cargo test -p swiftui --test pixel_parity -- --test-threads=1

# Benchmark: hand-written vs auto-generated (6)
cargo test -p swiftui --test benchmark_gen_vs_handwritten -- --test-threads=1
```

## Examples

```bash
# Reactive counter (+1/-1/reset with live UI updates)
cargo run -p swiftui --example reactive_counter

# TODO app with progress bar and state
cargo run -p swiftui --example reactive_todo

# Full showcase: navigation, styles, conditionals
cargo run -p swiftui --example showcase

# Complete widget catalog
cargo run -p swiftui --example full_demo

# DSL syntax demo
cargo run -p swiftui --example dsl_demo
```

## Architecture

```
┌─────────────────────────────────────────────────┐
│  Your Rust App                                   │
│  use swiftui::prelude::*;                        │
│  app("Title", w, h, |cx| { ... })               │
└──────────────┬──────────────────────────────────┘
               │ DSL calls
┌──────────────▼──────────────────────────────────┐
│  swiftui crate                                   │
│  Views, modifiers, state, navigation, styles     │
└──────────────┬──────────────────────────────────┘
               │ swiftui-sys function pointers
┌──────────────▼──────────────────────────────────┐
│  Swift Helper (SwiftUIHelper.swift, ~400 lines)  │
│  @_cdecl wrappers: C types → SwiftUI types       │
└──────────────┬──────────────────────────────────┘
               │
┌──────────────▼──────────────────────────────────┐
│  SwiftUI.framework + AttributeGraph              │
│  Native rendering on macOS / iOS                 │
└─────────────────────────────────────────────────┘
```

## License

Apache-2.0
