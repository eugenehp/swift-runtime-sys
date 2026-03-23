//! Raw FFI bindings to RealityKit via a Swift @_cdecl bridge.
//!
//! **Platform support:** macOS 12+, iOS 15+, visionOS 1+ (not available on tvOS or watchOS).

#![allow(non_snake_case, dead_code)]

// Note: RealityKit is available on macOS 12+, iOS 15+, visionOS 1+.
// On tvOS/watchOS, all symbols will be null and operations will be no-ops.

use core::ffi::{c_char, c_void};
use std::ffi::CString;

pub type Handle = *mut c_void;

unsafe extern "C" {
    fn dlopen(path: *const c_char, mode: i32) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
}

fn sym(h: *mut c_void, name: &core::ffi::CStr) -> *mut c_void {
    let p = unsafe { dlsym(h, name.as_ptr()) };
    assert!(!p.is_null(), "Symbol not found: {}", name.to_str().unwrap());
    p
}

pub struct Fns {
    // Entity
    pub entity_new: unsafe extern "C" fn() -> Handle,
    pub entity_set_name: unsafe extern "C" fn(Handle, *const u8, usize),
    pub entity_add_child: unsafe extern "C" fn(Handle, Handle),
    pub entity_set_position: unsafe extern "C" fn(Handle, f32, f32, f32),
    pub entity_set_scale: unsafe extern "C" fn(Handle, f32, f32, f32),
    pub entity_set_uniform_scale: unsafe extern "C" fn(Handle, f32),

    // ModelEntity
    pub model_entity_new: unsafe extern "C" fn(Handle, Handle) -> Handle,

    // AnchorEntity
    pub anchor_entity_new: unsafe extern "C" fn() -> Handle,
    pub anchor_entity_world: unsafe extern "C" fn(f32, f32, f32) -> Handle,

    // Mesh primitives
    pub mesh_box: unsafe extern "C" fn(f32, f32, f32) -> Handle,
    pub mesh_sphere: unsafe extern "C" fn(f32) -> Handle,
    pub mesh_plane: unsafe extern "C" fn(f32, f32) -> Handle,
    pub mesh_cone: unsafe extern "C" fn(f32, f32) -> Handle,
    pub mesh_cylinder: unsafe extern "C" fn(f32, f32) -> Handle,
    pub mesh_text: unsafe extern "C" fn(*const u8, usize, f32, f32) -> Handle,

    // Materials
    pub material_simple: unsafe extern "C" fn(f32, f32, f32, f32, bool) -> Handle,
    pub material_unlit: unsafe extern "C" fn(f32, f32, f32) -> Handle,

    // Lights
    pub point_light: unsafe extern "C" fn(f32, f32, f32, f32, f32) -> Handle,
    pub directional_light: unsafe extern "C" fn(f32, f32, f32, f32) -> Handle,

    // Scene
    pub scene_add_anchor: unsafe extern "C" fn(Handle, Handle),

    // Lifecycle
    pub release: unsafe extern "C" fn(Handle),
    pub retain: unsafe extern "C" fn(Handle),
}

pub fn load(path: &str) -> Result<Fns, String> {
    unsafe {
        let cpath = CString::new(path).unwrap();
        let h = dlopen(cpath.as_ptr(), 2);
        if h.is_null() {
            return Err(format!("Failed to load {path}"));
        }
        Ok(Fns {
            entity_new: std::mem::transmute(sym(h, c"rk_entity_new")),
            entity_set_name: std::mem::transmute(sym(h, c"rk_entity_set_name")),
            entity_add_child: std::mem::transmute(sym(h, c"rk_entity_add_child")),
            entity_set_position: std::mem::transmute(sym(h, c"rk_entity_set_position")),
            entity_set_scale: std::mem::transmute(sym(h, c"rk_entity_set_scale")),
            entity_set_uniform_scale: std::mem::transmute(sym(h, c"rk_entity_set_uniform_scale")),
            model_entity_new: std::mem::transmute(sym(h, c"rk_model_entity_new")),
            anchor_entity_new: std::mem::transmute(sym(h, c"rk_anchor_entity_new")),
            anchor_entity_world: std::mem::transmute(sym(h, c"rk_anchor_entity_world")),
            mesh_box: std::mem::transmute(sym(h, c"rk_mesh_box")),
            mesh_sphere: std::mem::transmute(sym(h, c"rk_mesh_sphere")),
            mesh_plane: std::mem::transmute(sym(h, c"rk_mesh_plane")),
            mesh_cone: std::mem::transmute(sym(h, c"rk_mesh_cone")),
            mesh_cylinder: std::mem::transmute(sym(h, c"rk_mesh_cylinder")),
            mesh_text: std::mem::transmute(sym(h, c"rk_mesh_text")),
            material_simple: std::mem::transmute(sym(h, c"rk_material_simple")),
            material_unlit: std::mem::transmute(sym(h, c"rk_material_unlit")),
            point_light: std::mem::transmute(sym(h, c"rk_point_light")),
            directional_light: std::mem::transmute(sym(h, c"rk_directional_light")),
            scene_add_anchor: std::mem::transmute(sym(h, c"rk_scene_add_anchor")),
            release: std::mem::transmute(sym(h, c"rk_release")),
            retain: std::mem::transmute(sym(h, c"rk_retain")),
        })
    }
}
