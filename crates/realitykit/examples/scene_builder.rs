//! RealityKit 3D scene — build a scene graph from Rust.
//!
//! cargo run -p realitykit --example scene_builder
//!
//! Note: this builds the scene graph but doesn't display it
//! (would need an ARView window). It demonstrates the API.

fn main() {
    // Ensure helper loaded
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

    let rk =
        realitykit::RealityKit::new().unwrap_or_else(|e| panic!("Failed to load RealityKit: {e}"));

    println!("=== RealityKit Scene Builder ===\n");

    // Create primitives
    let sphere = rk.sphere(0.5).at(0.0, 0.5, 0.0).name("sphere");
    println!("Created sphere at (0, 0.5, 0)");

    let cube = rk.cube(0.3).at(1.0, 0.15, 0.0).name("cube");
    println!("Created cube at (1, 0.15, 0)");

    let floor = rk.plane(10.0, 10.0);
    println!("Created floor plane 10×10");

    let cone = rk.cone(0.8, 0.3).at(-1.0, 0.4, 0.5);
    println!("Created cone at (-1, 0.4, 0.5)");

    let cylinder = rk.cylinder(1.0, 0.2).at(0.5, 0.5, -0.5);
    println!("Created cylinder at (0.5, 0.5, -0.5)");

    let text = rk.text("Hello 🦀", 0.05, 18.0).at(-0.5, 1.5, 0.0);
    println!("Created 3D text 'Hello 🦀'");

    // Lights
    let light = rk.point_light().at(2.0, 3.0, 2.0);
    println!("Created point light at (2, 3, 2)");

    let sun = rk.directional_light().at(0.0, 5.0, 0.0);
    println!("Created directional light");

    // Build scene graph
    let anchor = rk.anchor(0.0, 0.0, -3.0);
    anchor
        .add(&floor)
        .add(&sphere)
        .add(&cube)
        .add(&cone)
        .add(&cylinder)
        .add(&text)
        .add(&light)
        .add(&sun);
    println!("\nScene graph assembled with 8 entities under anchor at (0, 0, -3)");
    println!("Done.");
}
