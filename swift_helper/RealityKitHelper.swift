import RealityKit
import Foundation

#if os(macOS)
import AppKit
#else
import UIKit
#endif

public typealias RKHandle = UnsafeMutableRawPointer

func rkBox<T: AnyObject>(_ obj: T) -> RKHandle { Unmanaged.passRetained(obj).toOpaque() }
func rkBoxValue<T>(_ val: T) -> RKHandle { Unmanaged.passRetained(RKWrapper(val)).toOpaque() }
func rkUnbox<T>(_ h: RKHandle) -> T { Unmanaged<RKWrapper<T>>.fromOpaque(h).takeUnretainedValue().value }
func rkUnboxEntity(_ h: RKHandle) -> Entity { Unmanaged<Entity>.fromOpaque(h).takeUnretainedValue() }
class RKWrapper<T>: NSObject { let value: T; init(_ v: T) { self.value = v } }

@_cdecl("rk_release") public func rkRelease(_ h: RKHandle) { Unmanaged<AnyObject>.fromOpaque(h).release() }
@_cdecl("rk_retain") public func rkRetain(_ h: RKHandle) { _ = Unmanaged<AnyObject>.fromOpaque(h).retain() }

// ═══════════════════════════════════════════════════════════════════════════
// Entity
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("rk_entity_new") public func rkEntityNew() -> RKHandle { rkBox(Entity()) }

@_cdecl("rk_entity_set_name")
public func rkEntitySetName(_ h: RKHandle, _ ptr: UnsafePointer<UInt8>, _ len: Int) {
    let e = rkUnboxEntity(h)
    e.name = String(bytes: UnsafeBufferPointer(start: ptr, count: len), encoding: .utf8) ?? ""
}

@_cdecl("rk_entity_add_child")
public func rkEntityAddChild(_ parent: RKHandle, _ child: RKHandle) {
    rkUnboxEntity(parent).addChild(rkUnboxEntity(child))
}

@_cdecl("rk_entity_set_position")
public func rkEntitySetPosition(_ h: RKHandle, _ x: Float, _ y: Float, _ z: Float) {
    rkUnboxEntity(h).position = SIMD3(x, y, z)
}

@_cdecl("rk_entity_set_scale")
public func rkEntitySetScale(_ h: RKHandle, _ x: Float, _ y: Float, _ z: Float) {
    rkUnboxEntity(h).scale = SIMD3(x, y, z)
}

@_cdecl("rk_entity_set_uniform_scale")
public func rkEntitySetUniformScale(_ h: RKHandle, _ s: Float) {
    rkUnboxEntity(h).scale = SIMD3(repeating: s)
}

// ═══════════════════════════════════════════════════════════════════════════
// ModelEntity — entity with mesh + material
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("rk_model_entity_new")
public func rkModelEntityNew(_ mesh: RKHandle, _ material: RKHandle) -> RKHandle {
    let m: MeshResource = rkUnbox(mesh)
    let mat: any Material = rkUnbox(material)
    return rkBox(ModelEntity(mesh: m, materials: [mat]))
}

// ═══════════════════════════════════════════════════════════════════════════
// AnchorEntity
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("rk_anchor_entity_new") public func rkAnchorEntityNew() -> RKHandle { rkBox(AnchorEntity()) }

@_cdecl("rk_anchor_entity_world")
public func rkAnchorEntityWorld(_ x: Float, _ y: Float, _ z: Float) -> RKHandle {
    rkBox(AnchorEntity(world: SIMD3(x, y, z)))
}

// ═══════════════════════════════════════════════════════════════════════════
// Mesh primitives
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("rk_mesh_box")
public func rkMeshBox(_ w: Float, _ h: Float, _ d: Float) -> RKHandle {
    rkBoxValue(MeshResource.generateBox(width: w, height: h, depth: d))
}

@_cdecl("rk_mesh_sphere")
public func rkMeshSphere(_ radius: Float) -> RKHandle {
    rkBoxValue(MeshResource.generateSphere(radius: radius))
}

@_cdecl("rk_mesh_plane")
public func rkMeshPlane(_ w: Float, _ d: Float) -> RKHandle {
    rkBoxValue(MeshResource.generatePlane(width: w, depth: d))
}

@_cdecl("rk_mesh_cone")
public func rkMeshCone(_ h: Float, _ r: Float) -> RKHandle {
    rkBoxValue(MeshResource.generateCone(height: h, radius: r))
}

@_cdecl("rk_mesh_cylinder")
public func rkMeshCylinder(_ h: Float, _ r: Float) -> RKHandle {
    rkBoxValue(MeshResource.generateCylinder(height: h, radius: r))
}

@_cdecl("rk_mesh_text")
public func rkMeshText(_ ptr: UnsafePointer<UInt8>, _ len: Int, _ depth: Float, _ size: Float) -> RKHandle {
    let s = String(bytes: UnsafeBufferPointer(start: ptr, count: len), encoding: .utf8) ?? ""
    return rkBoxValue(MeshResource.generateText(s, extrusionDepth: depth, font: .systemFont(ofSize: CGFloat(size))))
}

// ═══════════════════════════════════════════════════════════════════════════
// Materials
// ═══════════════════════════════════════════════════════════════════════════

#if os(macOS)
typealias PlatformColor = NSColor
#else
typealias PlatformColor = UIColor
#endif

@_cdecl("rk_material_simple")
public func rkMaterialSimple(_ r: Float, _ g: Float, _ b: Float, _ roughness: Float, _ metallic: Bool) -> RKHandle {
    var mat = SimpleMaterial()
    mat.color = .init(tint: PlatformColor(red: CGFloat(r), green: CGFloat(g), blue: CGFloat(b), alpha: 1))
    mat.roughness = .float(roughness)
    mat.metallic = .float(metallic ? 1.0 : 0.0)
    return rkBoxValue(mat as any Material)
}

@_cdecl("rk_material_unlit")
public func rkMaterialUnlit(_ r: Float, _ g: Float, _ b: Float) -> RKHandle {
    var mat = UnlitMaterial()
    mat.color = .init(tint: PlatformColor(red: CGFloat(r), green: CGFloat(g), blue: CGFloat(b), alpha: 1))
    return rkBoxValue(mat as any Material)
}

// ═══════════════════════════════════════════════════════════════════════════
// Lights
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("rk_point_light")
public func rkPointLight(_ r: Float, _ g: Float, _ b: Float, _ intensity: Float, _ radius: Float) -> RKHandle {
    let light = PointLight()
    light.light.color = PlatformColor(red: CGFloat(r), green: CGFloat(g), blue: CGFloat(b), alpha: 1)
    light.light.intensity = intensity
    light.light.attenuationRadius = radius
    return rkBox(light)
}

@_cdecl("rk_directional_light")
public func rkDirectionalLight(_ r: Float, _ g: Float, _ b: Float, _ intensity: Float) -> RKHandle {
    let light = DirectionalLight()
    light.light.color = PlatformColor(red: CGFloat(r), green: CGFloat(g), blue: CGFloat(b), alpha: 1)
    light.light.intensity = intensity
    return rkBox(light)
}

// ═══════════════════════════════════════════════════════════════════════════
// Scene + ARView
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("rk_scene_add_anchor")
public func rkSceneAddAnchor(_ arview: RKHandle, _ anchor: RKHandle) {
    #if os(macOS)
    let view = Unmanaged<ARView>.fromOpaque(arview).takeUnretainedValue()
    let a = Unmanaged<AnchorEntity>.fromOpaque(anchor).takeUnretainedValue()
    view.scene.addAnchor(a)
    #endif
}

#if os(macOS)
@_cdecl("rk_arview_new")
public func rkARViewNew(_ w: Float, _ h: Float) -> RKHandle {
    let view = ARView(frame: NSRect(x: 0, y: 0, width: CGFloat(w), height: CGFloat(h)))
    return Unmanaged.passRetained(view).toOpaque()
}
#endif
