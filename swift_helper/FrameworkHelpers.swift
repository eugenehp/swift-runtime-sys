import Foundation

// ═══════════════════════════════════════════════════════════════════════════
// Translation
// ═══════════════════════════════════════════════════════════════════════════

#if canImport(Translation)
import Translation

@_cdecl("translation_available")
public func translationAvailable() -> Bool { true }
#endif

// ═══════════════════════════════════════════════════════════════════════════
// Spatial — 3D math types
// ═══════════════════════════════════════════════════════════════════════════

#if canImport(Spatial)
import Spatial

@_cdecl("spatial_point3d_distance")
public func spatialPoint3dDistance(_ ax: Double, _ ay: Double, _ az: Double, _ bx: Double, _ by: Double, _ bz: Double) -> Double {
    Point3D(x: ax, y: ay, z: az).distance(to: Point3D(x: bx, y: by, z: bz))
}

@_cdecl("spatial_rotation3d_from_axis_angle")
public func spatialRotation3dFromAxisAngle(_ ax: Double, _ ay: Double, _ az: Double, _ angle: Double, _ out: UnsafeMutablePointer<Double>) {
    let q = Rotation3D(angle: Angle2D(radians: angle), axis: RotationAxis3D(x: ax, y: ay, z: az)).quaternion
    out[0] = q.vector.x; out[1] = q.vector.y; out[2] = q.vector.z; out[3] = q.vector.w
}

@_cdecl("spatial_rotation3d_from_euler")
public func spatialRotation3dFromEuler(_ p: Double, _ y: Double, _ r: Double, _ out: UnsafeMutablePointer<Double>) {
    let q = Rotation3D(eulerAngles: EulerAngles(angles: SIMD3(p, y, r), order: .xyz)).quaternion
    out[0] = q.vector.x; out[1] = q.vector.y; out[2] = q.vector.z; out[3] = q.vector.w
}
#endif

// ═══════════════════════════════════════════════════════════════════════════
// WidgetKit
// ═══════════════════════════════════════════════════════════════════════════

#if canImport(WidgetKit)
import WidgetKit

@_cdecl("widgetkit_reload_all")
public func widgetkitReloadAll() { WidgetCenter.shared.reloadAllTimelines() }

@_cdecl("widgetkit_reload_kind")
public func widgetkitReloadKind(_ kindPtr: UnsafePointer<UInt8>, _ kindLen: Int) {
    let kind = String(bytes: UnsafeBufferPointer(start: kindPtr, count: kindLen), encoding: .utf8) ?? ""
    WidgetCenter.shared.reloadTimelines(ofKind: kind)
}

@_cdecl("widgetkit_get_configurations")
public func widgetkitGetConfigurations(
    _ cb: @convention(c) (UnsafePointer<UInt8>, Int, UnsafeMutableRawPointer?) -> Void,
    _ ud: UnsafeMutableRawPointer?
) {
    WidgetCenter.shared.getCurrentConfigurations { result in
        if let configs = try? result.get() {
            let json = configs.map { $0.kind }.joined(separator: ",")
            json.withCString { ptr in cb(UnsafePointer(OpaquePointer(ptr)), json.utf8.count, ud) }
        }
    }
}
#endif

// ═══════════════════════════════════════════════════════════════════════════
// AppIntents
// ═══════════════════════════════════════════════════════════════════════════

#if canImport(AppIntents)
import AppIntents

@_cdecl("appintents_available")
public func appintentsAvailable() -> Bool { true }
#endif

// ═══════════════════════════════════════════════════════════════════════════
// ActivityKit
// ═══════════════════════════════════════════════════════════════════════════

#if canImport(ActivityKit) && os(iOS)
import ActivityKit

@_cdecl("activitykit_available")
public func activitykitAvailable() -> Bool {
    ActivityAuthorizationInfo().areActivitiesEnabled
}
#endif
