//! Combine pub/sub — send values through a Subject.
//!
//! cargo run -p combine-rs --example pubsub

fn main() {
    // Ensure Swift helper is loaded
    unsafe {
        use core::ffi::c_char;
        extern "C" {
            fn dlopen(path: *const c_char, mode: i32) -> *mut core::ffi::c_void;
        }
        let paths = [
            c"swift_helper/libSwiftUIHelper.dylib".as_ptr(),
            c"../../swift_helper/libSwiftUIHelper.dylib".as_ptr(),
        ];
        for p in paths {
            let h = dlopen(p, 2);
            if !h.is_null() {
                break;
            }
        }
    }

    println!("=== Combine: PassthroughSubject ===\n");

    let subject = combine_rs::Subject::new();

    // Subscribe before sending
    let _sub1 = subject.subscribe(|v| println!("  Subscriber A got: {v}"));
    let _sub2 = subject.subscribe(|v| println!("  Subscriber B got: {v}"));

    println!("Sending 1, 2, 3:");
    subject.send(1);
    subject.send(2);
    subject.send(3);

    println!("\n=== Combine: CurrentValueSubject ===\n");

    let current = combine_rs::CurrentValue::new(0);
    println!("Initial: {}", current.get());
    current.set(42);
    println!("After set(42): {}", current.get());
    current.set(100);
    println!("After set(100): {}", current.get());

    println!("\n=== Subscription auto-cancel on drop ===\n");
    {
        let subject2 = combine_rs::Subject::new();
        let _sub = subject2.subscribe(|v| println!("  Scoped sub got: {v}"));
        subject2.send(999);
        println!("  (subscription drops here)");
    }
    println!("Done.");
}
