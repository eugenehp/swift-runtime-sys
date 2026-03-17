# SwiftUI from Rust

If your goal is to build a SwiftUI app that uses Rust, the boundary should be:

- SwiftUI owns views, navigation, bindings, and platform integration.
- Rust owns pure logic and data processing.
- The bridge between them is a C ABI.

## What not to do

Do not try to instantiate SwiftUI view types from Rust through the Swift runtime.
Even with runtime symbols available, SwiftUI depends on Swift compiler features
that are not expressed as a stable runtime API.

## Recommended shape

Create a separate Rust crate for app logic:

```toml
[package]
name = "app-core"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["staticlib", "cdylib"]
```

Export a C ABI from Rust:

```rust
use std::ffi::{c_char, CString};

#[unsafe(no_mangle)]
pub extern "C" fn rust_add(a: i32, b: i32) -> i32 {
    a + b
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_greeting() -> *mut c_char {
    CString::new("Hello from Rust").unwrap().into_raw()
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_string_free(ptr: *mut c_char) {
    if ptr.is_null() {
        return;
    }

    unsafe {
        drop(CString::from_raw(ptr));
    }
}
```

Declare the bridge in Swift:

```c
// RustBridge.h
#include <stdint.h>

int32_t rust_add(int32_t a, int32_t b);
char *rust_greeting(void);
void rust_string_free(char *ptr);
```

Wrap it in a Swift-friendly type:

```swift
import Foundation

@MainActor
final class RustViewModel: ObservableObject {
    @Published var title = ""
    @Published var sum = 0

    func load() {
        sum = rust_add(20, 22)

        guard let raw = rust_greeting() else {
            title = "Rust returned null"
            return
        }

        title = String(cString: raw)
        rust_string_free(raw)
    }
}
```

Use that wrapper from SwiftUI:

```swift
import SwiftUI

struct ContentView: View {
    @StateObject private var model = RustViewModel()

    var body: some View {
        VStack(spacing: 16) {
            Text(model.title)
            Text("sum: \(model.sum)")
            Button("Load from Rust") {
                model.load()
            }
        }
        .padding()
    }
}
```

## Data flow guidance

- Keep FFI functions small and explicit.
- Pass integers, floats, byte buffers, and UTF-8 strings.
- Avoid exposing Rust structs directly across the boundary unless you control the layout.
- Prefer opaque handles for long-lived Rust state.
- Marshal async work by letting Swift trigger Rust operations and poll or receive callbacks.

## Construct Swift Values from Rust (struct/class/function)

If your specific goal is "from Rust, create Swift structs/classes and call Swift
functions," do it with explicit Swift-exported C ABI entry points.

Pattern:

1. Swift exports constructor/method/free-function shims with `@_cdecl`.
2. Rust declares those symbols in `unsafe extern "C"`.
3. Rust wraps opaque pointers in RAII types and calls matching `*_drop` shims.

See working files in this repo:

- [examples/RustBridge.swift](examples/RustBridge.swift)
- [examples/swift_bridge.rs](examples/swift_bridge.rs)

This gives you fully established Swift values because Swift itself performs the
construction; Rust only holds opaque handles and invokes ABI-safe entry points.

## Where this crate fits

This repository is useful if you are researching the Swift runtime itself. It is
not the right layer for normal SwiftUI application integration.

For an actual SwiftUI app backed by Rust, create a new Rust library crate for
your app logic and link that library into an Xcode project.