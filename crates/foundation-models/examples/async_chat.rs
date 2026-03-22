//! Async chat with Apple Intelligence using tokio.
//!
//! cargo run -p foundation-models --example async_chat

#[tokio::main]
async fn main() {
    load_helper();
    println!("=== Apple Intelligence — Async ===\n");

    if !foundation_models::is_available() {
        println!("❌ Apple Intelligence not available.");
        return;
    }

    let session = foundation_models::Session::new(Some(
        "You are a concise coding assistant. Respond in under 50 words.",
    ));

    // Multiple async requests
    let prompts = vec![
        "What is a closure in Rust?",
        "What is SwiftUI's @State?",
        "Explain async/await in one sentence.",
    ];

    for prompt in prompts {
        println!("User: {prompt}");
        match session.respond_async(prompt).await {
            Some(r) => println!("  AI: {r}\n"),
            None => println!("  AI: (no response)\n"),
        }
    }

    // Channel-based streaming
    println!("--- Streaming via channel ---\n");
    println!("User: Write a limerick about Rust.");
    print!("  AI: ");

    let rx = session.stream_channel("Write a limerick about Rust.");
    while let Ok(token) = rx.recv_timeout(std::time::Duration::from_secs(5)) {
        print!("{token}");
    }
    println!("\n\nDone.");
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
