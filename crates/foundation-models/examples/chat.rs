//! Chat with Apple Intelligence on-device LLM.
//!
//! cargo run -p foundation-models --example chat
//!
//! Requires macOS 26+ with Apple Intelligence enabled.

fn main() {
    // Load helper
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

    println!("=== Apple Intelligence from Rust ===\n");

    if !foundation_models::is_available() {
        println!("Apple Intelligence not available on this device.");
        println!("Requires macOS 26+ with Apple Intelligence enabled in System Settings.");
        return;
    }

    println!("✅ Apple Intelligence available!\n");

    let session = foundation_models::Session::new(Some(
        "You are a concise assistant. Keep responses under 50 words.",
    ));

    // Blocking response
    println!("Prompt: What is Rust?");
    match session.respond("What is Rust?") {
        Some(response) => println!("Response: {response}\n"),
        None => println!("No response received.\n"),
    }

    // Streaming response
    println!("Prompt: Tell me a short joke.");
    print!("Response: ");
    session.stream("Tell me a short joke.", |token| {
        print!("{token}");
    });
    println!("\n");

    println!("Done.");
}
