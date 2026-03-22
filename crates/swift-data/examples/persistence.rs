//! Persistent key-value storage — survives app restarts.
//!
//! cargo run -p swift-data --example persistence

fn main() {
    // Ensure Swift helper is loaded
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

    let db = swift_data::Store::new();

    println!("=== String values ===");
    db.set("users", "name", "Alice");
    db.set("users", "email", "alice@example.com");
    println!("  name:  {:?}", db.get("users", "name"));
    println!("  email: {:?}", db.get("users", "email"));
    println!("  missing: {:?}", db.get("users", "phone"));

    println!("\n=== Integer values ===");
    db.set_int("stats", "launches", 42);
    db.set_int("stats", "score", 9001);
    println!("  launches: {}", db.get_int("stats", "launches"));
    println!("  score:    {}", db.get_int("stats", "score"));

    println!("\n=== Boolean values ===");
    db.set_bool("settings", "dark_mode", true);
    db.set_bool("settings", "notifications", false);
    println!("  dark_mode:     {}", db.get_bool("settings", "dark_mode"));
    println!(
        "  notifications: {}",
        db.get_bool("settings", "notifications")
    );

    println!("\n=== Delete ===");
    db.delete("users", "email");
    println!("  email after delete: {:?}", db.get("users", "email"));

    println!("\n=== Update ===");
    let count = db.get_int("stats", "launches");
    db.set_int("stats", "launches", count + 1);
    println!(
        "  launches after increment: {}",
        db.get_int("stats", "launches")
    );

    println!("\nDone. Values persist across runs — run again to see incremented count.");
}
