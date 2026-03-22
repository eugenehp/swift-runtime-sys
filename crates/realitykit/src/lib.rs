//! Ergonomic RealityKit 3D scene builder from Rust.
//!
//! ```ignore
//! use realitykit::prelude::*;
//!
//! let scene = Scene::new("scene_helper/libRealityKitHelper.dylib");
//!
//! let floor = scene.model(Mesh::plane(10.0, 10.0), Material::simple(0.3, 0.3, 0.3).roughness(0.8));
//! let sphere = scene.model(Mesh::sphere(0.5), Material::simple(1.0, 0.2, 0.2).metallic());
//! sphere.position(0.0, 0.5, 0.0);
//!
//! let anchor = scene.anchor_world(0.0, 0.0, -2.0);
//! anchor.add(floor);
//! anchor.add(sphere);
//! anchor.add(scene.point_light(1.0, 1.0, 0.9, 1000.0, 10.0).at(0.0, 3.0, 0.0));
//! ```

use core::ffi::c_void;
use realitykit_sys::Handle;

pub mod prelude {
    pub use crate::{EntityHandle, Material, Mesh, RKScene};
}

/// A loaded RealityKit scene context.
pub struct RKScene {
    fns: realitykit_sys::Fns,
}

/// An opaque handle to a RealityKit entity. Auto-releases on drop.
pub struct EntityHandle {
    ptr: Handle,
    release: unsafe extern "C" fn(Handle),
}

impl EntityHandle {
    fn new(ptr: Handle, release: unsafe extern "C" fn(Handle)) -> Self {
        Self { ptr, release }
    }

    pub fn raw(&self) -> Handle {
        self.ptr
    }

    /// Set the entity name.
    pub fn name(self, fns: &realitykit_sys::Fns, name: &str) -> Self {
        unsafe { (fns.entity_set_name)(self.ptr, name.as_ptr(), name.len()) };
        self
    }

    /// Set position.
    pub fn position(self, fns: &realitykit_sys::Fns, x: f32, y: f32, z: f32) -> Self {
        unsafe { (fns.entity_set_position)(self.ptr, x, y, z) };
        self
    }

    /// Set scale.
    pub fn scale(self, fns: &realitykit_sys::Fns, x: f32, y: f32, z: f32) -> Self {
        unsafe { (fns.entity_set_scale)(self.ptr, x, y, z) };
        self
    }

    /// Set uniform scale.
    pub fn uniform_scale(self, fns: &realitykit_sys::Fns, s: f32) -> Self {
        unsafe { (fns.entity_set_uniform_scale)(self.ptr, s) };
        self
    }

    /// Add a child entity.
    pub fn add(&self, fns: &realitykit_sys::Fns, child: &EntityHandle) {
        unsafe { (fns.entity_add_child)(self.ptr, child.ptr) };
    }
}

impl Drop for EntityHandle {
    fn drop(&mut self) {
        unsafe { (self.release)(self.ptr) };
    }
}

/// Mesh primitive builder.
pub enum Mesh {
    Box(f32, f32, f32),
    Sphere(f32),
    Plane(f32, f32),
    Cone(f32, f32),
    Cylinder(f32, f32),
    Text(String, f32, f32),
}

impl Mesh {
    pub fn cube(size: f32) -> Self {
        Mesh::Box(size, size, size)
    }
    pub fn box_(w: f32, h: f32, d: f32) -> Self {
        Mesh::Box(w, h, d)
    }
    pub fn sphere(radius: f32) -> Self {
        Mesh::Sphere(radius)
    }
    pub fn plane(w: f32, d: f32) -> Self {
        Mesh::Plane(w, d)
    }
    pub fn cone(height: f32, radius: f32) -> Self {
        Mesh::Cone(height, radius)
    }
    pub fn cylinder(height: f32, radius: f32) -> Self {
        Mesh::Cylinder(height, radius)
    }
    pub fn text(s: &str, depth: f32, size: f32) -> Self {
        Mesh::Text(s.to_string(), depth, size)
    }

    fn create(&self, fns: &realitykit_sys::Fns) -> Handle {
        unsafe {
            match self {
                Mesh::Box(w, h, d) => (fns.mesh_box)(*w, *h, *d),
                Mesh::Sphere(r) => (fns.mesh_sphere)(*r),
                Mesh::Plane(w, d) => (fns.mesh_plane)(*w, *d),
                Mesh::Cone(h, r) => (fns.mesh_cone)(*h, *r),
                Mesh::Cylinder(h, r) => (fns.mesh_cylinder)(*h, *r),
                Mesh::Text(s, depth, size) => (fns.mesh_text)(s.as_ptr(), s.len(), *depth, *size),
            }
        }
    }
}

/// Material builder.
pub struct Material {
    r: f32,
    g: f32,
    b: f32,
    roughness: f32,
    is_metallic: bool,
    is_unlit: bool,
}

impl Material {
    pub fn simple(r: f32, g: f32, b: f32) -> Self {
        Self {
            r,
            g,
            b,
            roughness: 0.5,
            is_metallic: false,
            is_unlit: false,
        }
    }

    pub fn unlit(r: f32, g: f32, b: f32) -> Self {
        Self {
            r,
            g,
            b,
            roughness: 0.0,
            is_metallic: false,
            is_unlit: true,
        }
    }

    pub fn roughness(mut self, v: f32) -> Self {
        self.roughness = v;
        self
    }

    pub fn metallic(mut self) -> Self {
        self.is_metallic = true;
        self
    }

    fn create(&self, fns: &realitykit_sys::Fns) -> Handle {
        unsafe {
            if self.is_unlit {
                (fns.material_unlit)(self.r, self.g, self.b)
            } else {
                (fns.material_simple)(self.r, self.g, self.b, self.roughness, self.is_metallic)
            }
        }
    }
}

impl RKScene {
    /// Load the RealityKit helper dylib.
    pub fn new(helper_path: &str) -> Result<Self, String> {
        Ok(Self {
            fns: realitykit_sys::load(helper_path)?,
        })
    }

    /// Create a model entity with mesh + material.
    pub fn model(&self, mesh: Mesh, material: Material) -> EntityHandle {
        let m = mesh.create(&self.fns);
        let mat = material.create(&self.fns);
        let entity = unsafe { (self.fns.model_entity_new)(m, mat) };
        EntityHandle::new(entity, self.fns.release)
    }

    /// Create an empty entity.
    pub fn entity(&self) -> EntityHandle {
        let e = unsafe { (self.fns.entity_new)() };
        EntityHandle::new(e, self.fns.release)
    }

    /// Create an anchor at the world origin.
    pub fn anchor(&self) -> EntityHandle {
        let a = unsafe { (self.fns.anchor_entity_new)() };
        EntityHandle::new(a, self.fns.release)
    }

    /// Create an anchor at a world position.
    pub fn anchor_world(&self, x: f32, y: f32, z: f32) -> EntityHandle {
        let a = unsafe { (self.fns.anchor_entity_world)(x, y, z) };
        EntityHandle::new(a, self.fns.release)
    }

    /// Create a point light.
    pub fn point_light(&self, r: f32, g: f32, b: f32, intensity: f32, radius: f32) -> EntityHandle {
        let l = unsafe { (self.fns.point_light)(r, g, b, intensity, radius) };
        EntityHandle::new(l, self.fns.release)
    }

    /// Create a directional light.
    pub fn directional_light(&self, r: f32, g: f32, b: f32, intensity: f32) -> EntityHandle {
        let l = unsafe { (self.fns.directional_light)(r, g, b, intensity) };
        EntityHandle::new(l, self.fns.release)
    }

    /// Get the underlying function pointers (for chaining).
    pub fn fns(&self) -> &realitykit_sys::Fns {
        &self.fns
    }
}
