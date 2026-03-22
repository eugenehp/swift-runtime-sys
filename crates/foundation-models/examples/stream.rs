//! Streaming response from Apple Intelligence.
//!
//! cargo run -p foundation-models --example stream

fn main() {
    load_helper();
    println!("=== Apple Intelligence — Streaming ===\n");

    if !foundation_models::is_available() {
        println!("❌ Apple Intelligence not available.");
        return;
    }

    let session = foundation_models::Session::new(Some("You are a poet."));

    println!("Prompt: Write a haiku about Rust programming.\n");
    print!("Response: ");

    session.stream("Write a haiku about Rust programming.", |token| {
        print!("{token}");
    });

    // Give async stream time to complete
    std::thread::sleep(std::time::Duration::from_secs(3));
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
