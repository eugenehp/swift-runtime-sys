import Foundation
import Translation
import Spatial

// ═══════════════════════════════════════════════════════════════════════════
// Translation
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("translation_available")
public func translationAvailable() -> Bool {
    true // framework is loaded
}

@_cdecl("translation_supported_languages")
public func translationSupportedLanguages(
    _ outPtr: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outLen: UnsafeMutablePointer<Int>
) -> Bool {
    Task {
        let avail = LanguageAvailability()
        let langs = await avail.supportedLanguages
        let joined = langs.map { $0.minimalIdentifier }.joined(separator: ",")
        let buf = UnsafeMutableRawPointer.allocate(byteCount: joined.utf8.count, alignment: 1)
        joined.withCString { ptr in buf.copyMemory(from: ptr, byteCount: joined.utf8.count) }
        outPtr.pointee = buf; outLen.pointee = joined.utf8.count
    }
    return true
}

// ═══════════════════════════════════════════════════════════════════════════
// Spatial — 3D math types
// ═══════════════════════════════════════════════════════════════════════════

// Point3D
@_cdecl("spatial_point3d_new")
public func spatialPoint3dNew(_ x: Double, _ y: Double, _ z: Double) -> UnsafeMutableRawPointer {
    let p = Point3D(x: x, y: y, z: z)
    let buf = UnsafeMutableRawPointer.allocate(byteCount: 24, alignment: 8)
    buf.storeBytes(of: p.x, toByteOffset: 0, as: Double.self)
    buf.storeBytes(of: p.y, toByteOffset: 8, as: Double.self)
    buf.storeBytes(of: p.z, toByteOffset: 16, as: Double.self)
    return buf
}

@_cdecl("spatial_point3d_distance")
public func spatialPoint3dDistance(_ ax: Double, _ ay: Double, _ az: Double, _ bx: Double, _ by: Double, _ bz: Double) -> Double {
    let a = Point3D(x: ax, y: ay, z: az)
    let b = Point3D(x: bx, y: by, z: bz)
    return a.distance(to: b)
}

// Rotation3D
@_cdecl("spatial_rotation3d_from_axis_angle")
public func spatialRotation3dFromAxisAngle(_ ax: Double, _ ay: Double, _ az: Double, _ angle: Double, _ out: UnsafeMutablePointer<Double>) {
    let r = Rotation3D(angle: Angle2D(radians: angle), axis: RotationAxis3D(x: ax, y: ay, z: az))
    let q = r.quaternion
    out[0] = q.vector.x; out[1] = q.vector.y; out[2] = q.vector.z; out[3] = q.vector.w
}

@_cdecl("spatial_rotation3d_from_euler")
public func spatialRotation3dFromEuler(_ pitch: Double, _ yaw: Double, _ roll: Double, _ out: UnsafeMutablePointer<Double>) {
    let r = Rotation3D(
        eulerAngles: EulerAngles(angles: SIMD3(pitch, yaw, roll), order: .xyz)
    )
    let q = r.quaternion
    out[0] = q.vector.x; out[1] = q.vector.y; out[2] = q.vector.z; out[3] = q.vector.w
}

// Pose3D
@_cdecl("spatial_pose3d_new")
public func spatialPose3dNew(_ px: Double, _ py: Double, _ pz: Double, _ qx: Double, _ qy: Double, _ qz: Double, _ qw: Double, _ out: UnsafeMutablePointer<Double>) {
    let pose = Pose3D(
        position: Point3D(x: px, y: py, z: pz),
        rotation: Rotation3D(quaternion: .init(vector: SIMD4(qx, qy, qz, qw)))
    )
    out[0] = pose.position.x; out[1] = pose.position.y; out[2] = pose.position.z
    let q = pose.rotation.quaternion
    out[3] = q.vector.x; out[4] = q.vector.y; out[5] = q.vector.z; out[6] = q.vector.w
}

// Ray3D
@_cdecl("spatial_ray3d_new")
public func spatialRay3dNew(_ ox: Double, _ oy: Double, _ oz: Double, _ dx: Double, _ dy: Double, _ dz: Double, _ out: UnsafeMutablePointer<Double>) {
    let ray = Ray3D(origin: Point3D(x: ox, y: oy, z: oz), direction: Vector3D(x: dx, y: dy, z: dz))
    out[0] = ray.origin.x; out[1] = ray.origin.y; out[2] = ray.origin.z
    out[3] = ray.direction.x; out[4] = ray.direction.y; out[5] = ray.direction.z
}

// AffineTransform3D
@_cdecl("spatial_transform_identity")
public func spatialTransformIdentity(_ out: UnsafeMutablePointer<Double>) {
    let t = AffineTransform3D.identity
    // Output as 4x3 matrix (12 doubles)
    let cols = t.matrix4x4
    for r in 0..<4 { for c in 0..<3 { out[r*3+c] = cols[r][c] } }
}

@_cdecl("spatial_transform_translate")
public func spatialTransformTranslate(_ x: Double, _ y: Double, _ z: Double, _ out: UnsafeMutablePointer<Double>) {
    let t = AffineTransform3D(translation: Vector3D(x: x, y: y, z: z))
    let cols = t.matrix4x4
    for r in 0..<4 { for c in 0..<3 { out[r*3+c] = cols[r][c] } }
}

@_cdecl("spatial_transform_scale")
public func spatialTransformScale(_ x: Double, _ y: Double, _ z: Double, _ out: UnsafeMutablePointer<Double>) {
    let t = AffineTransform3D(scale: Size3D(width: x, height: y, depth: z))
    let cols = t.matrix4x4
    for r in 0..<4 { for c in 0..<3 { out[r*3+c] = cols[r][c] } }
}
