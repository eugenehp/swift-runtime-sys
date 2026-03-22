//! Pixel-parity tests: compare Swift-native rendering vs Rust-constructed rendering.
//!
//! For each test case, we:
//! 1. Render the view using native Swift (ground truth)
//! 2. Render the same view built via our Rust SwiftUI bridge
//! 3. Compare every pixel with a tolerance
//! 4. Assert ≥99% match

use std::ffi::{c_char, c_void};
use std::path::Path;

unsafe extern "C" {
    fn dlopen(path: *const c_char, mode: i32) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    fn NSApplicationLoad() -> bool;
}

#[link(name = "AppKit", kind = "framework")]
extern "C" {}

const HELPER: &str = "swift_helper/libSwiftUIHelper.dylib";
const W: f32 = 300.0;
const H: f32 = 100.0;
const PASS_THRESHOLD: f32 = 99.0; // percent

static INIT: std::sync::Once = std::sync::Once::new();
static mut LIB: *mut c_void = std::ptr::null_mut();

fn ensure_init() {
    INIT.call_once(|| {
        unsafe {
            NSApplicationLoad();
            dlopen(c"/System/Library/Frameworks/SwiftUI.framework/SwiftUI".as_ptr(), 1);
            // Try multiple paths
            let paths = [
                c"swift_helper/libSwiftUIHelper.dylib".as_ptr(),
                c"../../swift_helper/libSwiftUIHelper.dylib".as_ptr(),
                c"/Users/Shared/swift-runtime-sys/swift_helper/libSwiftUIHelper.dylib".as_ptr(),
            ];
            for p in paths {
                LIB = dlopen(p, 2);
                if !LIB.is_null() { break; }
            }
        }
    });
    assert!(!unsafe { LIB }.is_null(), "Build the helper first");
}

fn lib() -> *mut c_void { unsafe { LIB } }

/// Snapshot a Swift-native view (ground truth).
fn snapshot_swift_text(text: &str, w: f32, h: f32) -> Vec<u8> {
    ensure_init();
    type F = unsafe extern "C" fn(*const u8, usize, f32, f32, *mut *mut c_void, *mut usize) -> bool;
    let f: F = unsafe { std::mem::transmute(dlsym(lib(), c"snapshot_text".as_ptr())) };
    let mut ptr: *mut c_void = std::ptr::null_mut();
    let mut len: usize = 0;
    let ok = unsafe { f(text.as_ptr(), text.len(), w, h, &mut ptr, &mut len) };
    assert!(ok, "snapshot_text failed");
    let data = unsafe { std::slice::from_raw_parts(ptr as *const u8, len).to_vec() };
    unsafe {
        type Free = unsafe extern "C" fn(*mut c_void, usize);
        let free_fn: Free = std::mem::transmute(dlsym(lib(), c"free_snapshot".as_ptr()));
        free_fn(ptr, len);
    }
    data
}

/// Snapshot a Swift-native styled text.
fn snapshot_swift_styled_text(text: &str, size: f32, weight: i32, r: f32, g: f32, b: f32, a: f32, w: f32, h: f32) -> Vec<u8> {
    ensure_init();
    type F = unsafe extern "C" fn(*const u8, usize, f32, i32, f32, f32, f32, f32, f32, f32, *mut *mut c_void, *mut usize) -> bool;
    let f: F = unsafe { std::mem::transmute(dlsym(lib(), c"snapshot_styled_text".as_ptr())) };
    let mut ptr: *mut c_void = std::ptr::null_mut();
    let mut len: usize = 0;
    let ok = unsafe { f(text.as_ptr(), text.len(), size, weight, r, g, b, a, w, h, &mut ptr, &mut len) };
    assert!(ok, "snapshot_styled_text failed");
    let data = unsafe { std::slice::from_raw_parts(ptr as *const u8, len).to_vec() };
    unsafe {
        type Free = unsafe extern "C" fn(*mut c_void, usize);
        let free_fn: Free = std::mem::transmute(dlsym(lib(), c"free_snapshot".as_ptr()));
        free_fn(ptr, len);
    }
    data
}

/// Snapshot a Swift-native VStack of texts.
fn snapshot_swift_vstack_texts(texts: &[&str], w: f32, h: f32) -> Vec<u8> {
    ensure_init();
    type F = unsafe extern "C" fn(*const *const u8, *const usize, usize, f32, f32, *mut *mut c_void, *mut usize) -> bool;
    let f: F = unsafe { std::mem::transmute(dlsym(lib(), c"snapshot_vstack_texts".as_ptr())) };
    let ptrs: Vec<*const u8> = texts.iter().map(|s| s.as_ptr()).collect();
    let lens: Vec<usize> = texts.iter().map(|s| s.len()).collect();
    let mut ptr: *mut c_void = std::ptr::null_mut();
    let mut len: usize = 0;
    let ok = unsafe { f(ptrs.as_ptr(), lens.as_ptr(), texts.len(), w, h, &mut ptr, &mut len) };
    assert!(ok, "snapshot_vstack_texts failed");
    let data = unsafe { std::slice::from_raw_parts(ptr as *const u8, len).to_vec() };
    unsafe {
        type Free = unsafe extern "C" fn(*mut c_void, usize);
        let free_fn: Free = std::mem::transmute(dlsym(lib(), c"free_snapshot".as_ptr()));
        free_fn(ptr, len);
    }
    data
}

/// Snapshot a Rust-constructed view (via ViewHandle).
fn snapshot_rust_view(handle: *mut c_void, w: f32, h: f32) -> Vec<u8> {
    ensure_init();
    type F = unsafe extern "C" fn(*mut c_void, f32, f32, *mut *mut c_void, *mut usize) -> bool;
    let f: F = unsafe { std::mem::transmute(dlsym(lib(), c"snapshot_view_handle".as_ptr())) };
    let mut ptr: *mut c_void = std::ptr::null_mut();
    let mut len: usize = 0;
    let ok = unsafe { f(handle, w, h, &mut ptr, &mut len) };
    assert!(ok, "snapshot_view_handle failed");
    let data = unsafe { std::slice::from_raw_parts(ptr as *const u8, len).to_vec() };
    unsafe {
        type Free = unsafe extern "C" fn(*mut c_void, usize);
        let free_fn: Free = std::mem::transmute(dlsym(lib(), c"free_snapshot".as_ptr()));
        free_fn(ptr, len);
    }
    data
}

/// Compare two PNGs, return % matching pixels.
fn compare(a: &[u8], b: &[u8], tolerance_percent: f32) -> f32 {
    ensure_init();
    type F = unsafe extern "C" fn(*const c_void, usize, *const c_void, usize, f32) -> f32;
    let f: F = unsafe { std::mem::transmute(dlsym(lib(), c"compare_png_bytes".as_ptr())) };
    unsafe { f(a.as_ptr() as _, a.len(), b.as_ptr() as _, b.len(), tolerance_percent) }
}

/// Create a Rust view via the helper.
fn rust_text(s: &str) -> *mut c_void {
    ensure_init();
    type F = unsafe extern "C" fn(*const u8, usize) -> *mut c_void;
    let f: F = unsafe { std::mem::transmute(dlsym(lib(), c"swiftui_text".as_ptr())) };
    unsafe { f(s.as_ptr(), s.len()) }
}

fn rust_styled_text(s: &str, size: f32, weight: i32, r: f32, g: f32, b: f32, a: f32) -> *mut c_void {
    ensure_init();
    type F = unsafe extern "C" fn(*const u8, usize, f32, i32, f32, f32, f32, f32) -> *mut c_void;
    let f: F = unsafe { std::mem::transmute(dlsym(lib(), c"swiftui_text_styled".as_ptr())) };
    unsafe { f(s.as_ptr(), s.len(), size, weight, r, g, b, a) }
}

fn rust_vstack(children: &[*mut c_void]) -> *mut c_void {
    ensure_init();
    type F = unsafe extern "C" fn(*const *mut c_void, usize) -> *mut c_void;
    let f: F = unsafe { std::mem::transmute(dlsym(lib(), c"swiftui_vstack".as_ptr())) };
    unsafe { f(children.as_ptr(), children.len()) }
}

fn save_png(name: &str, data: &[u8]) {
    let dir = Path::new("target/snapshots");
    std::fs::create_dir_all(dir).ok();
    std::fs::write(dir.join(format!("{name}.png")), data).ok();
}

// ═══════════════════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn pixel_parity_plain_text() {
    let swift_png = snapshot_swift_text("Hello, World!", W, H);
    let rust_handle = rust_text("Hello, World!");
    let rust_png = snapshot_rust_view(rust_handle, W, H);

    save_png("plain_text_swift", &swift_png);
    save_png("plain_text_rust", &rust_png);

    let match_pct = compare(&swift_png, &rust_png, 2.0);
    println!("plain_text: {match_pct:.1}% pixel match");
    assert!(match_pct >= PASS_THRESHOLD, "Plain text: {match_pct:.1}% < {PASS_THRESHOLD}%");
}

#[test]
fn pixel_parity_styled_text() {
    let swift_png = snapshot_swift_styled_text("Bold Red", 20.0, 1, 1.0, 0.0, 0.0, 1.0, W, H);
    let rust_handle = rust_styled_text("Bold Red", 20.0, 1, 1.0, 0.0, 0.0, 1.0);
    let rust_png = snapshot_rust_view(rust_handle, W, H);

    save_png("styled_text_swift", &swift_png);
    save_png("styled_text_rust", &rust_png);

    let match_pct = compare(&swift_png, &rust_png, 2.0);
    println!("styled_text: {match_pct:.1}% pixel match");
    assert!(match_pct >= PASS_THRESHOLD, "Styled text: {match_pct:.1}% < {PASS_THRESHOLD}%");
}

#[test]
fn pixel_parity_vstack() {
    let swift_png = snapshot_swift_vstack_texts(&["First", "Second", "Third"], W, 200.0);

    let children = [rust_text("First"), rust_text("Second"), rust_text("Third")];
    let rust_handle = rust_vstack(&children);
    let rust_png = snapshot_rust_view(rust_handle, W, 200.0);

    save_png("vstack_swift", &swift_png);
    save_png("vstack_rust", &rust_png);

    let match_pct = compare(&swift_png, &rust_png, 2.0);
    println!("vstack: {match_pct:.1}% pixel match");
    assert!(match_pct >= PASS_THRESHOLD, "VStack: {match_pct:.1}% < {PASS_THRESHOLD}%");
}

#[test]
fn pixel_parity_emoji_text() {
    let swift_png = snapshot_swift_text("🦀 Rust + SwiftUI 🚀", W, H);
    let rust_handle = rust_text("🦀 Rust + SwiftUI 🚀");
    let rust_png = snapshot_rust_view(rust_handle, W, H);

    save_png("emoji_swift", &swift_png);
    save_png("emoji_rust", &rust_png);

    let match_pct = compare(&swift_png, &rust_png, 2.0);
    println!("emoji: {match_pct:.1}% pixel match");
    assert!(match_pct >= PASS_THRESHOLD, "Emoji: {match_pct:.1}% < {PASS_THRESHOLD}%");
}

#[test]
fn pixel_parity_italic_blue() {
    let swift_png = snapshot_swift_styled_text("Italic Blue", 16.0, 2, 0.2, 0.4, 0.9, 1.0, W, H);
    let rust_handle = rust_styled_text("Italic Blue", 16.0, 2, 0.2, 0.4, 0.9, 1.0);
    let rust_png = snapshot_rust_view(rust_handle, W, H);

    save_png("italic_blue_swift", &swift_png);
    save_png("italic_blue_rust", &rust_png);

    let match_pct = compare(&swift_png, &rust_png, 2.0);
    println!("italic_blue: {match_pct:.1}% pixel match");
    assert!(match_pct >= PASS_THRESHOLD, "Italic blue: {match_pct:.1}% < {PASS_THRESHOLD}%");
}

#[test]
fn pixel_parity_long_text() {
    let long = "The quick brown fox jumps over the lazy dog. Pack my box with five dozen liquor jugs.";
    let swift_png = snapshot_swift_text(long, 400.0, 100.0);
    let rust_handle = rust_text(long);
    let rust_png = snapshot_rust_view(rust_handle, 400.0, 100.0);

    save_png("long_text_swift", &swift_png);
    save_png("long_text_rust", &rust_png);

    let match_pct = compare(&swift_png, &rust_png, 2.0);
    println!("long_text: {match_pct:.1}% pixel match");
    assert!(match_pct >= PASS_THRESHOLD, "Long text: {match_pct:.1}% < {PASS_THRESHOLD}%");
}
