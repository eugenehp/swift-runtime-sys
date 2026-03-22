//! Spatial 3D math — points, rotations, poses, rays.
//!
//! cargo run -p spatial-rs --example math3d

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

    println!("=== Spatial 3D Math ===\n");

    // Points
    let a = spatial::Point3D::new(1.0, 2.0, 3.0);
    let b = spatial::Point3D::new(4.0, 6.0, 3.0);
    println!("Point A: {:?}", a);
    println!("Point B: {:?}", b);
    println!("Distance: {:.3}", a.distance_to(&b));

    // Rotation
    let rot = spatial::Rotation3D::from_axis_angle(0.0, 1.0, 0.0, std::f64::consts::FRAC_PI_2);
    println!("\nRotation (90° around Y): {:?}", rot);

    let euler = spatial::Rotation3D::from_euler(0.5, 0.3, 0.1);
    println!("Euler rotation: {:?}", euler);

    // Pose
    let pose = spatial::Pose3D::new(a, rot);
    println!("\nPose: {:?}", pose);

    // Ray
    let ray = spatial::Ray3D::new(
        spatial::Point3D::origin(),
        spatial::Point3D::new(0.0, 0.0, -1.0),
    );
    println!("\nRay: {:?}", ray);
    println!("Point at t=5: {:?}", ray.point_at(5.0));

    println!("\nDone.");
}
