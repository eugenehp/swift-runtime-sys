//! Apple Spatial framework — 3D math types from Rust.
//!
//! Provides Point3D, Rotation3D, Pose3D, Ray3D, and AffineTransform3D
//! backed by Apple's Spatial framework for visionOS-compatible math.
//!
//! ```ignore
//! use spatial::*;
//!
//! let a = Point3D::new(1.0, 2.0, 3.0);
//! let b = Point3D::new(4.0, 5.0, 6.0);
//! println!("Distance: {}", a.distance_to(&b));
//!
//! let rot = Rotation3D::from_axis_angle(0.0, 1.0, 0.0, std::f64::consts::FRAC_PI_2);
//! let pose = Pose3D::new(a, rot);
//! ```

use core::ffi::{c_char, c_void};

unsafe extern "C" {
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
}

fn sym(name: &core::ffi::CStr) -> *const c_void {
    unsafe { dlsym((-2isize) as *mut c_void, name.as_ptr()) as *const c_void }
}

/// A 3D point.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    pub fn origin() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
    pub fn distance_to(&self, other: &Point3D) -> f64 {
        type F = unsafe extern "C" fn(f64, f64, f64, f64, f64, f64) -> f64;
        let f: F = unsafe { std::mem::transmute(sym(c"spatial_point3d_distance")) };
        unsafe { f(self.x, self.y, self.z, other.x, other.y, other.z) }
    }
}

/// A 3D rotation (stored as quaternion).
#[derive(Debug, Clone, Copy)]
pub struct Rotation3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Rotation3D {
    pub fn identity() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        }
    }

    pub fn from_axis_angle(ax: f64, ay: f64, az: f64, angle_radians: f64) -> Self {
        type F = unsafe extern "C" fn(f64, f64, f64, f64, *mut f64);
        let f: F = unsafe { std::mem::transmute(sym(c"spatial_rotation3d_from_axis_angle")) };
        let mut out = [0.0f64; 4];
        unsafe { f(ax, ay, az, angle_radians, out.as_mut_ptr()) };
        Self {
            x: out[0],
            y: out[1],
            z: out[2],
            w: out[3],
        }
    }

    pub fn from_euler(pitch: f64, yaw: f64, roll: f64) -> Self {
        type F = unsafe extern "C" fn(f64, f64, f64, *mut f64);
        let f: F = unsafe { std::mem::transmute(sym(c"spatial_rotation3d_from_euler")) };
        let mut out = [0.0f64; 4];
        unsafe { f(pitch, yaw, roll, out.as_mut_ptr()) };
        Self {
            x: out[0],
            y: out[1],
            z: out[2],
            w: out[3],
        }
    }
}

/// A 3D pose (position + rotation).
#[derive(Debug, Clone, Copy)]
pub struct Pose3D {
    pub position: Point3D,
    pub rotation: Rotation3D,
}

impl Pose3D {
    pub fn new(position: Point3D, rotation: Rotation3D) -> Self {
        Self { position, rotation }
    }

    pub fn identity() -> Self {
        Self {
            position: Point3D::origin(),
            rotation: Rotation3D::identity(),
        }
    }
}

/// A 3D ray (origin + direction).
#[derive(Debug, Clone, Copy)]
pub struct Ray3D {
    pub origin: Point3D,
    pub direction: Point3D,
}

impl Ray3D {
    pub fn new(origin: Point3D, direction: Point3D) -> Self {
        Self { origin, direction }
    }

    pub fn point_at(&self, t: f64) -> Point3D {
        Point3D {
            x: self.origin.x + self.direction.x * t,
            y: self.origin.y + self.direction.y * t,
            z: self.origin.z + self.direction.z * t,
        }
    }
}
