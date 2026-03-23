//! Launch a SwiftUI App at the SIL level from Rust.
//!
//! Replicates the exact mechanism the Swift compiler generates:
//! 1. Rust provides `main()`
//! 2. `main()` configures the app (title, callbacks)
//! 3. `main()` calls `ShellApp.main()` which calls `SwiftUI.App.main<ShellApp>()`
//! 4. The Swift runtime resolves the type metadata, witness table, and
//!    protocol conformance descriptor — all from the shell's compiled object code
//! 5. `App.main()` reads `body.getter` from the witness table, calls it,
//!    gets back a `WindowGroup<Text>`, and enters the SwiftUI event loop
//!
//! The conformance descriptor, type metadata, and witness table thunks live
//! in `libShellApp.dylib` — compiled from 25 lines of Swift by `build.rs`.
//! Rust controls WHAT the app displays by setting the title before launch.

use core::ffi::c_void;

unsafe extern "C" {
    fn dlopen(path: *const core::ffi::c_char, mode: i32) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const core::ffi::c_char) -> *mut c_void;
}

/// Launch a SwiftUI App from Rust using the SIL-level approach.
///
/// - `title`: The text displayed in the SwiftUI window
///
/// This function never returns — it enters the SwiftUI event loop.
///
/// Under the hood, this does exactly what `@main struct MyApp: App` compiles to:
/// ```text
/// main() {
///     let metatype = ShellApp.self
///     let wt = swift_getWitnessTable(ShellApp_App_conformance, metatype, nil)
///     SwiftUI.App.main(metatype, wt, swiftself: metatype)
/// }
/// ```
pub fn launch_app() -> ! {
    launch_with_title("Hello from pure Rust! 🦀")
}

/// Launch with a custom title.
pub fn launch_with_title(title: &str) -> ! {
    unsafe { launch_inner(title) }
}

unsafe fn launch_inner(title: &str) -> ! {
    // 1. Load the shell app dylib (compiled from ShellApp.swift by build.rs)
    let shell_paths = [
        "crates/winit-swift/swift/libShellApp.dylib",
        "swift/libShellApp.dylib",
        "libShellApp.dylib",
        "../../crates/winit-swift/swift/libShellApp.dylib",
    ];

    let mut shell_handle: *mut c_void = std::ptr::null_mut();
    for path in shell_paths {
        let cpath = std::ffi::CString::new(path).unwrap();
        let h = dlopen(cpath.as_ptr(), 1);
        if !h.is_null() {
            shell_handle = h;
            println!("[SIL] Shell app loaded from: {path}");
            break;
        }
    }
    assert!(!shell_handle.is_null(),
        "[SIL] libShellApp.dylib not found. Run from the workspace root.");

    // 2. Set the title from Rust before launch
    type SetTitleFn = unsafe extern "C" fn(*const u8, usize);
    let set_title_sym = dlsym(shell_handle, c"shell_app_set_title_str".as_ptr());
    assert!(!set_title_sym.is_null(), "[SIL] shell_app_set_title_str not found");
    let set_title: SetTitleFn = core::mem::transmute(set_title_sym);
    set_title(title.as_ptr(), title.len());
    println!("[SIL] Title set to: \"{title}\"");

    // 3. Verify the metadata is valid
    type GetMetaFn = unsafe extern "C" fn() -> *const c_void;
    let get_meta = dlsym(shell_handle, c"shell_app_get_metadata".as_ptr());
    assert!(!get_meta.is_null());
    let meta: GetMetaFn = core::mem::transmute(get_meta);
    let shell_meta = meta();
    let kind = *(shell_meta as *const usize);
    println!("[SIL] ShellApp metadata at: {shell_meta:?} (kind=0x{kind:x})");
    assert!(kind == 0x200, "[SIL] Expected struct metadata kind 0x200, got 0x{kind:x}");

    // 4. Launch — this calls ShellApp.main() which calls App.main<ShellApp>()
    //    The Swift runtime handles:
    //    - Instantiating the witness table from the conformance descriptor
    //    - Calling body.getter to get the scene tree
    //    - Creating the UIApplication/NSApplication
    //    - Entering the event loop
    type LaunchFn = unsafe extern "C" fn();
    let launch_sym = dlsym(shell_handle, c"shell_app_launch".as_ptr());
    assert!(!launch_sym.is_null(), "[SIL] shell_app_launch not found");
    let launch: LaunchFn = core::mem::transmute(launch_sym);

    println!("[SIL] Calling ShellApp.main() → SwiftUI.App.main<ShellApp>()");
    println!("[SIL] This is the exact same call chain as @main in Swift.");
    launch();

    std::process::exit(0);
}
