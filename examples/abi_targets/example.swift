// ABI Discovery Fixture Template
//
// This file defines the Swift types/protocols/enums you want to probe.
// Run: ./scripts/run_abi_discovery.sh examples/abi_targets/example.swift
//
// The script compiles this file, runs it (static layout JSON → stdout),
// then extracts dynamic data (exported symbols, metadata ptrs) and generates
// a Rust struct + RuntimeFactory template in target/abi-discovery/.
//
// WRITING YOUR OWN FIXTURE
// ────────────────────────
//  1. Copy this file to examples/abi_targets/my_types.swift
//  2. Replace the types below with your own
//  3. Call the probe_* helpers for each type at the bottom (// ── Probe ── section)
//  4. Run: ./scripts/run_abi_discovery.sh examples/abi_targets/my_types.swift

import Foundation
import ObjectiveC.runtime

// ── ABI probe output (do not modify) ─────────────────────────────────────

private func emit(_ dict: [String: Any]) {
    let sorted = dict.sorted { $0.key < $1.key }
    var out = "{"
    for (i, (k, v)) in sorted.enumerated() {
        let comma = i < sorted.count - 1 ? "," : ""
        switch v {
        case let n as Int:   out += "\"\(k)\":\(n)\(comma)"
        case let n as Int64: out += "\"\(k)\":\(n)\(comma)"
        case let b as Bool:  out += "\"\(k)\":\(b ? "true" : "false")\(comma)"
        case let s as String:
            let esc = s.replacingOccurrences(of: "\\", with: "\\\\")
                       .replacingOccurrences(of: "\"", with: "\\\"")
            out += "\"\(k)\":\"\(esc)\"\(comma)"
        case let a as [[String: Any]]:
            let inner = a.map { sub -> String in
                let s2 = sub.sorted { $0.key < $1.key }
                let parts = s2.map { (k2, v2) -> String in
                    switch v2 {
                    case let n as Int:    return "\"\(k2)\":\(n)"
                    case let s3 as String:
                        let e = s3.replacingOccurrences(of: "\\", with: "\\\\")
                                  .replacingOccurrences(of: "\"", with: "\\\"")
                        return "\"\(k2)\":\"\(e)\""
                    default: return "\"\(k2)\":null"
                    }
                }
                return "{" + parts.joined(separator: ",") + "}"
            }.joined(separator: ",")
            out += "\"\(k)\":[\(inner)]\(comma)"
        default: out += "\"\(k)\":null\(comma)"
        }
    }
    out += "}"
    print(out)
    fflush(stdout)
}

// Probe a value type (struct/enum): uses MemoryLayout for size/stride/alignment.
// Fields must be listed manually because key-path offset only works for writable
// stored properties. See probe_struct_field() below.
private func probe_value<T>(_ type: T.Type, swiftKind: String) {
    emit([
        "kind": swiftKind,
        "name": "\(T.self)",
        "size": MemoryLayout<T>.size,
        "stride": MemoryLayout<T>.stride,
        "alignment": MemoryLayout<T>.alignment,
        "fields": [[String: Any]]()   // populated separately via probe_struct_field
    ])
}

// Probe a single stored field of a struct via key-path offset.
private func probe_struct_field<Root, Field>(
    _ type: Root.Type,
    field name: String,
    keyPath: WritableKeyPath<Root, Field>
) {
    let offset = MemoryLayout<Root>.offset(of: keyPath)!
    emit([
        "kind": "field",
        "parent": "\(Root.self)",
        "name": name,
        "offset": offset,
        "size": MemoryLayout<Field>.size,
        "swift_type": "\(Field.self)",
    ])
}

// Probe a class type: uses class_getInstanceSize + class_copyIvarList for fields.
private func probe_class<T: AnyObject>(_ type: T.Type) {
    let cls: AnyClass = T.self
    emit([
        "kind": "class",
        "name": "\(T.self)",
        "reference_size": MemoryLayout<T>.size,   // always 8 on 64-bit
        "instance_size": Int(exactly: class_getInstanceSize(cls))!,
        "alignment": MemoryLayout<Int>.alignment,
        "fields": [[String: Any]]()   // populated by probe_class_ivars below
    ])

    var count: UInt32 = 0
    guard let ivars = class_copyIvarList(cls, &count) else { return }
    defer { free(ivars) }
    for i in 0..<Int(count) {
        let ivar = ivars[i]
        let name = String(cString: ivar_getName(ivar)!)
        let offset = Int(exactly: ivar_getOffset(ivar))!
        let encoding = ivar_getTypeEncoding(ivar).map { String(cString: $0) } ?? "?"
        emit([
            "kind": "field",
            "parent": "\(T.self)",
            "name": name,
            "offset": offset,
            "size": -1,           // encoding gives type info, not a direct size
            "encoding": encoding, // ObjC type encoding: i=int, q=long long, etc.
        ])
    }
}

// Emit metadata accessor symbol address (requires the binary to see itself).
// Works because this file is compiled as an executable with the types present.
private func probe_metadata<T>(_ type: T.Type) {
    // Swift's internal _typeByName is not stable; use Mirror kind as a proxy.
    let mirror = Mirror(reflecting: type)
    emit([
        "kind": "metadata_hint",
        "name": "\(T.self)",
        "mirror_display_style": "\(mirror.displayStyle as Any)",
    ])
}

// ── Define your types here ────────────────────────────────────────────────

// VALUE TYPES (structs)
struct ProbePoint {
    var x: Int32
    var y: Int32
}

struct ProbeRect {
    var origin: ProbePoint
    var width: Int32
    var height: Int32
}

struct ResilientPair {
    var a: Int64
    var b: Int64
}

// CLASS TYPES
final class ManagedNode {
    var id: Int32
    var score: Int64
    var tag: Int32

    init(id: Int32, score: Int64, tag: Int32) {
        self.id = id
        self.score = score
        self.tag = tag
    }
}

// ENUMS (simple raw-representable)
enum Color: Int32 {
    case red = 0, green = 1, blue = 2
}

// ENUMS (associated values)
enum ProbeShape {
    case circle(radius: Float)
    case rect(w: Float, h: Float)
}

// ── Probe ─────────────────────────────────────────────────────────────────
// Call one probe_* per type, then one probe_struct_field per stored property.

probe_value(ProbePoint.self, swiftKind: "struct")
probe_struct_field(ProbePoint.self, field: "x", keyPath: \.x)
probe_struct_field(ProbePoint.self, field: "y", keyPath: \.y)

probe_value(ProbeRect.self, swiftKind: "struct")
probe_struct_field(ProbeRect.self, field: "origin", keyPath: \.origin)
probe_struct_field(ProbeRect.self, field: "width",  keyPath: \.width)
probe_struct_field(ProbeRect.self, field: "height", keyPath: \.height)

probe_value(ResilientPair.self, swiftKind: "struct")
probe_struct_field(ResilientPair.self, field: "a", keyPath: \.a)
probe_struct_field(ResilientPair.self, field: "b", keyPath: \.b)

probe_class(ManagedNode.self)

probe_value(Color.self, swiftKind: "enum")

probe_value(ProbeShape.self, swiftKind: "enum")

probe_metadata(ProbePoint.self)
probe_metadata(ManagedNode.self)
probe_metadata(Color.self)
probe_metadata(ProbeShape.self)
