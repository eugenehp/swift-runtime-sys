//! Open a SwiftUI Text window from pure Rust.
//!
//! Prerequisites:
//!   1. Compile the SwiftUI helper:
//!      xcrun swiftc -emit-library /tmp/swiftui_analysis/probe_wt.swift \
//!        -o /tmp/swiftui_analysis/libProbeWT.dylib \
//!        -target arm64-apple-macosx15.0 -sdk $(xcrun -sdk macosx --show-sdk-path)
//!
//!   2. Run this example:
//!      cargo run --example swiftui_window

fn main() {
    println!("=== SwiftUI from Rust ===\n");

    unsafe {
        use core::ffi::{c_char, c_void};
        unsafe extern "C" {
            fn dlopen(path: *const c_char, mode: i32) -> *mut c_void;
            fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
        }

        // Load SwiftUI
        swift_runtime_sys::SwiftUIBridge::ensure_loaded()
            .expect("Failed to load SwiftUI");
        println!("SwiftUI loaded.");
        println!("Text value size: {:?} bytes", swift_runtime_sys::SwiftUIBridge::text_size());

        // Load our Swift helper
        let helper = dlopen(c"/tmp/swiftui_analysis/libProbeWT.dylib".as_ptr(), 1);
        assert!(!helper.is_null(), "Failed to load helper. Run:\n  \
            xcrun swiftc -emit-library /tmp/swiftui_analysis/probe_wt.swift \
            -o /tmp/swiftui_analysis/libProbeWT.dylib \
            -target arm64-apple-macosx15.0 -sdk $(xcrun -sdk macosx --show-sdk-path)");

        // Strategy: Use the Swift helper to create the Text existential,
        // then show it — proving the existential container format works.
        let create_fn = dlsym(helper, c"create_text_existential".as_ptr());
        let show_fn = dlsym(helper, c"show_existential_in_window".as_ptr());
        assert!(!create_fn.is_null() && !show_fn.is_null());

        type CreateFn = unsafe extern "C" fn(*mut c_void) -> usize;
        type ShowFn = unsafe extern "C" fn(*const c_void);
        let create: CreateFn = core::mem::transmute(create_fn);
        let show: ShowFn = core::mem::transmute(show_fn);

        // Create the existential via Swift, examine it from Rust
        let mut container = swift_runtime_sys::SwiftUIBridge::AnyViewExistential {
            inline_buffer: [0; 3],
            metadata: core::ptr::null(),
            witness_table: core::ptr::null(),
        };
        let size = create(&mut container as *mut _ as *mut c_void);
        println!("\nExistential created ({size} bytes):");
        println!("  inline_buffer: [{:016x}, {:016x}, {:016x}]",
            container.inline_buffer[0], container.inline_buffer[1], container.inline_buffer[2]);
        println!("  metadata:      {:?}", container.metadata);
        println!("  witness_table: {:?}", container.witness_table);

        // Verify: does the metadata match SwiftUI.Text?
        let text_meta_sym = dlsym((-2isize) as *mut c_void, c"$s7SwiftUI4TextVN".as_ptr());
        println!("\n  Text metadata symbol:  {:?}", text_meta_sym);
        println!("  Container metadata:    {:?}", container.metadata);
        println!("  Match: {}", text_meta_sym as usize == container.metadata as usize);

        // Verify: does the witness table match swift_conformsToProtocol?
        let wt = swift_runtime_sys::SwiftUIBridge::get_view_witness_table(container.metadata);
        println!("  conformsToProtocol WT: {:?}", wt);
        println!("  Container WT:          {:?}", container.witness_table);
        println!("  Match: {}", wt.map(|w| w as usize) == Some(container.witness_table as usize));

        // Use the simpler helper that takes a UTF-8 string directly
        let create_text_controller = dlsym(helper, c"create_text_hosting_controller".as_ptr());
        let show_window = dlsym(helper, c"show_window".as_ptr());

        if !create_text_controller.is_null() && !show_window.is_null() {
            type CreateTextFn = unsafe extern "C" fn(*const u8, usize) -> *mut c_void;
            type ShowWindowFn = unsafe extern "C" fn(*mut c_void);
            let create_ctrl: CreateTextFn = core::mem::transmute(create_text_controller);
            let show_win: ShowWindowFn = core::mem::transmute(show_window);

            let msg = "Hello from Rust! 🦀";
            println!("\nCreating hosting controller with: '{msg}'");
            let controller = create_ctrl(msg.as_ptr(), msg.len());
            println!("Controller: {:?}", controller);

            println!("Opening SwiftUI window...");
            show_win(controller);
        } else {
            println!("Helper functions not found");
        }
    }
}
