//! Chat with Apple Intelligence — blocking API.
//!
//! cargo run -p foundation-models --example chat

fn main() {
    load_helper();
    println!("=== Apple Intelligence — Blocking ===\n");

    if !foundation_models::is_available() {
        println!("❌ Apple Intelligence not available.");
        println!("   Requires macOS 26+ with Apple Intelligence enabled.");
        return;
    }
    println!("✅ Apple Intelligence available!\n");

    let session = foundation_models::Session::new(Some(
        "You are a concise assistant. Keep responses under 30 words.",
    ));

    let prompts = [
        "What is Rust in one sentence?",
        "What is SwiftUI?",
        "Name 3 planets.",
    ];

    for prompt in prompts {
        println!("User: {prompt}");
        match session.respond(prompt) {
            Some(r) => println!("  AI: {r}\n"),
            None => println!("  AI: (no response)\n"),
        }
    }
}

fn load_helper() {
    unsafe {
        use core::ffi::c_char;
        extern "C" {
            fn dlopen(path: *const c_char, mode: i32) -> *mut core::ffi::c_void;
        }
        for p in [
            c"swift_helper/libSwiftUIHelper.dylib".as_ptr(),
            c"../../swift_helper/libSwiftUIHelper.dylib".as_ptr(),
        ] {
            if !dlopen(p, 2).is_null() {
                break;
            }
        }
    }
}
