//! Pure Rust SwiftUI App — no Swift source files.
//!
//! Replicates what `swiftc` generates at the SIL/LLVM IR level:
//! type metadata, witness tables, and a direct call to SwiftUI.App.main().
//!
//! ```bash
//! cargo run -p winit-swift --example sil_app
//! ```
//!
//! This should open a SwiftUI window showing "Hello from Rust" — created
//! entirely from Rust via dlsym + inline asm, without any .swift files.

fn main() {
    println!("=== Pure Rust SwiftUI App (SIL-level) ===");
    println!("Constructing Swift type metadata in Rust...");
    println!("Building App protocol witness table...");
    println!("Calling SwiftUI.App.main() via inline asm...");
    println!();

    // This never returns — it enters the SwiftUI event loop
    winit_swift::sil::launch_app();
}
