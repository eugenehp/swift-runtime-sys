//! Benchmark: hand-written SwiftUIHelper vs auto-generated SwiftUIGenBridge
//!
//! Compares:
//! 1. View construction speed (how fast each helper creates views)
//! 2. Pixel parity (are the outputs identical)
//! 3. Complex tree construction

use std::ffi::{c_char, c_void};
use std::time::Instant;

unsafe extern "C" {
    fn dlopen(path: *const c_char, mode: i32) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    fn NSApplicationLoad() -> bool;
}

#[link(name = "AppKit", kind = "framework")]
extern "C" {}

type V = *mut c_void;
type TextFn = unsafe extern "C" fn(*const u8, usize) -> V;
type StyledFn = unsafe extern "C" fn(*const u8, usize, f32, i32, f32, f32, f32, f32) -> V;
type VoidFn = unsafe extern "C" fn() -> V;
type StackFn = unsafe extern "C" fn(*const V, usize) -> V;
type ModFn = unsafe extern "C" fn(V, f32) -> V;
type BgFn = unsafe extern "C" fn(V, f32, f32, f32, f32) -> V;
type SnapFn = unsafe extern "C" fn(V, f32, f32, *mut *mut c_void, *mut usize) -> bool;
type FreeFn = unsafe extern "C" fn(*mut c_void, usize);
type RelFn = unsafe extern "C" fn(V);
type CmpFn = unsafe extern "C" fn(*const c_void, usize, *const c_void, usize, f32) -> f32;

struct Helper {
    text: TextFn,
    styled: StyledFn,
    spacer: VoidFn,
    divider: VoidFn,
    vstack: StackFn,
    hstack: StackFn,
    padding: ModFn,
    bg: BgFn,
    corner: ModFn,
    snap: SnapFn,
    free: FreeFn,
    release: RelFn,
}

static INIT: std::sync::Once = std::sync::Once::new();
static mut HAND: Option<Helper> = None;
static mut GEN: Option<Helper> = None;
static mut CMP: Option<CmpFn> = None;

fn init() {
    INIT.call_once(|| unsafe {
        NSApplicationLoad();
        dlopen(c"/System/Library/Frameworks/SwiftUI.framework/SwiftUI".as_ptr(), 1);

        let paths = [
            c"swift_helper/libSwiftUIHelper.dylib".as_ptr(),
            c"../../swift_helper/libSwiftUIHelper.dylib".as_ptr(),
        ];
        let mut hh = std::ptr::null_mut();
        for p in paths { hh = dlopen(p, 2); if !hh.is_null() { break; } }

        let paths_g = [
            c"generated/swiftui_gen/libSwiftUIGen.dylib".as_ptr(),
            c"../../generated/swiftui_gen/libSwiftUIGen.dylib".as_ptr(),
        ];
        let mut gh = std::ptr::null_mut();
        for p in paths_g { gh = dlopen(p, 2); if !gh.is_null() { break; } }

        if hh.is_null() || gh.is_null() { return; }

        HAND = Some(Helper {
            text: std::mem::transmute(dlsym(hh, c"swiftui_text".as_ptr())),
            styled: std::mem::transmute(dlsym(hh, c"swiftui_text_styled".as_ptr())),
            spacer: std::mem::transmute(dlsym(hh, c"swiftui_spacer".as_ptr())),
            divider: std::mem::transmute(dlsym(hh, c"swiftui_divider".as_ptr())),
            vstack: std::mem::transmute(dlsym(hh, c"swiftui_vstack".as_ptr())),
            hstack: std::mem::transmute(dlsym(hh, c"swiftui_hstack".as_ptr())),
            padding: std::mem::transmute(dlsym(hh, c"swiftui_padding".as_ptr())),
            bg: std::mem::transmute(dlsym(hh, c"swiftui_background_color".as_ptr())),
            corner: std::mem::transmute(dlsym(hh, c"swiftui_corner_radius".as_ptr())),
            snap: std::mem::transmute(dlsym(hh, c"snapshot_view_handle".as_ptr())),
            free: std::mem::transmute(dlsym(hh, c"free_snapshot".as_ptr())),
            release: std::mem::transmute(dlsym(hh, c"swiftui_release".as_ptr())),
        });

        GEN = Some(Helper {
            text: std::mem::transmute(dlsym(gh, c"gen_text".as_ptr())),
            styled: std::mem::transmute(dlsym(gh, c"gen_styled_text".as_ptr())),
            spacer: std::mem::transmute(dlsym(gh, c"gen_spacer".as_ptr())),
            divider: std::mem::transmute(dlsym(gh, c"gen_divider".as_ptr())),
            vstack: std::mem::transmute(dlsym(gh, c"gen_vstack".as_ptr())),
            hstack: std::mem::transmute(dlsym(gh, c"gen_hstack".as_ptr())),
            padding: std::mem::transmute(dlsym(gh, c"gen_padding".as_ptr())),
            bg: std::mem::transmute(dlsym(gh, c"gen_bg".as_ptr())),
            corner: std::mem::transmute(dlsym(gh, c"gen_corner".as_ptr())),
            snap: std::mem::transmute(dlsym(gh, c"gen_snapshot".as_ptr())),
            free: std::mem::transmute(dlsym(gh, c"gen_free".as_ptr())),
            release: std::mem::transmute(dlsym(gh, c"gen_release".as_ptr())),
        });

        CMP = Some(std::mem::transmute(dlsym(hh, c"compare_png_bytes".as_ptr())));
    });
}

fn hand() -> &'static Helper { unsafe { HAND.as_ref().expect("Init failed — build both helpers") } }
fn gen() -> &'static Helper { unsafe { GEN.as_ref().expect("Init failed — build gen helper") } }

fn build_simple(h: &Helper) -> V {
    unsafe {
        let s = b"Hello, World!";
        (h.text)(s.as_ptr(), s.len())
    }
}

fn build_styled(h: &Helper) -> V {
    unsafe { (h.styled)(b"Bold Red".as_ptr(), 8, 20.0, 1, 1.0, 0.0, 0.0, 1.0) }
}

fn build_complex(h: &Helper) -> V {
    unsafe {
        let title = (h.styled)(b"Title".as_ptr(), 5, 24.0, 1, 1.0, 1.0, 1.0, 1.0);
        let sub = (h.styled)(b"Subtitle".as_ptr(), 8, 14.0, 2, 0.5, 0.5, 0.5, 1.0);
        let sp = (h.spacer)();
        let div = (h.divider)();
        let t1 = (h.text)(b"Item 1".as_ptr(), 6);
        let t2 = (h.text)(b"Item 2".as_ptr(), 6);
        let t3 = (h.text)(b"Item 3".as_ptr(), 6);
        let row = [t1, t2, t3];
        let hst = (h.hstack)(row.as_ptr(), 3);
        let items = [title, sub, div, hst, sp];
        let vs = (h.vstack)(items.as_ptr(), 5);
        let vs = (h.padding)(vs, 16.0);
        let vs = (h.bg)(vs, 0.1, 0.1, 0.15, 1.0);
        (h.corner)(vs, 10.0)
    }
}

fn snapshot(h: &Helper, view: V, w: f32, ht: f32) -> Vec<u8> {
    unsafe {
        let mut ptr: *mut c_void = std::ptr::null_mut();
        let mut len: usize = 0;
        let ok = (h.snap)(view, w, ht, &mut ptr, &mut len);
        assert!(ok, "Snapshot failed");
        let data = std::slice::from_raw_parts(ptr as *const u8, len).to_vec();
        (h.free)(ptr, len);
        data
    }
}

fn compare(a: &[u8], b: &[u8]) -> f32 {
    unsafe {
        let cmp = CMP.unwrap();
        cmp(a.as_ptr() as _, a.len(), b.as_ptr() as _, b.len(), 2.0)
    }
}

fn bench_construction(name: &str, h: &Helper, builder: fn(&Helper) -> V, iterations: usize) -> std::time::Duration {
    // Warmup
    for _ in 0..5 { let v = builder(h); unsafe { (h.release)(v) }; }

    let start = Instant::now();
    for _ in 0..iterations {
        let v = builder(h);
        unsafe { (h.release)(v) };
    }
    let elapsed = start.elapsed();
    let per_op = elapsed / iterations as u32;
    println!("  {name}: {iterations} iterations in {elapsed:?} ({per_op:?}/op)");
    elapsed
}

// ═══════════════════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn bench_simple_text_construction() {
    init();
    let n = 1000;
    println!("\n=== Simple Text construction ({n} iterations) ===");
    let hand_time = bench_construction("hand-written", hand(), build_simple, n);
    let gen_time = bench_construction("auto-generated", gen(), build_simple, n);
    let ratio = gen_time.as_nanos() as f64 / hand_time.as_nanos() as f64;
    println!("  Ratio (gen/hand): {ratio:.2}x");
}

#[test]
fn bench_styled_text_construction() {
    init();
    let n = 1000;
    println!("\n=== Styled Text construction ({n} iterations) ===");
    let hand_time = bench_construction("hand-written", hand(), build_styled, n);
    let gen_time = bench_construction("auto-generated", gen(), build_styled, n);
    let ratio = gen_time.as_nanos() as f64 / hand_time.as_nanos() as f64;
    println!("  Ratio (gen/hand): {ratio:.2}x");
}

#[test]
fn bench_complex_tree_construction() {
    init();
    let n = 500;
    println!("\n=== Complex tree construction ({n} iterations) ===");
    let hand_time = bench_construction("hand-written", hand(), build_complex, n);
    let gen_time = bench_construction("auto-generated", gen(), build_complex, n);
    let ratio = gen_time.as_nanos() as f64 / hand_time.as_nanos() as f64;
    println!("  Ratio (gen/hand): {ratio:.2}x");
}

#[test]
fn pixel_parity_hand_vs_gen_simple() {
    init();
    let hand_view = build_simple(hand());
    let gen_view = build_simple(gen());
    let hand_png = snapshot(hand(), hand_view, 300.0, 50.0);
    let gen_png = snapshot(gen(), gen_view, 300.0, 50.0);
    let pct = compare(&hand_png, &gen_png);
    println!("\nSimple text pixel parity: {pct:.1}%");
    assert!(pct >= 99.0, "Simple: {pct:.1}% < 99%");
}

#[test]
fn pixel_parity_hand_vs_gen_styled() {
    init();
    let hand_view = build_styled(hand());
    let gen_view = build_styled(gen());
    let hand_png = snapshot(hand(), hand_view, 300.0, 50.0);
    let gen_png = snapshot(gen(), gen_view, 300.0, 50.0);
    let pct = compare(&hand_png, &gen_png);
    println!("\nStyled text pixel parity: {pct:.1}%");
    assert!(pct >= 99.0, "Styled: {pct:.1}% < 99%");
}

#[test]
fn pixel_parity_hand_vs_gen_complex() {
    init();
    let hand_view = build_complex(hand());
    let gen_view = build_complex(gen());
    let hand_png = snapshot(hand(), hand_view, 300.0, 200.0);
    let gen_png = snapshot(gen(), gen_view, 300.0, 200.0);
    let pct = compare(&hand_png, &gen_png);
    println!("\nComplex tree pixel parity: {pct:.1}%");
    assert!(pct >= 99.0, "Complex: {pct:.1}% < 99%");
}
