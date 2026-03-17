import Foundation

public struct ExternalResilient {
    public var a: Int32
    public var b: Int64

    public init(a: Int32, b: Int64) {
        self.a = a
        self.b = b
    }
}

public func external_resilient_size() -> Int32 {
    Int32(MemoryLayout<ExternalResilient>.size)
}

public func external_resilient_stride() -> Int32 {
    Int32(MemoryLayout<ExternalResilient>.stride)
}

public func external_resilient_alignment() -> Int32 {
    Int32(MemoryLayout<ExternalResilient>.alignment)
}

public func external_resilient_b_offset() -> Int32 {
    Int32(MemoryLayout<ExternalResilient>.offset(of: \.b) ?? -1)
}

public func external_resilient_get_b(_ a: Int32, _ b: Int64) -> Int64 {
    ExternalResilient(a: a, b: b).b
}

public protocol ExternalCurrentLike {
    func current() -> Int32
}

public protocol ExternalRefCurrentLike: AnyObject {
    func current() -> Int32
}

public struct ExternalValueCounter: ExternalCurrentLike {
    public var value: Int32
    public init(_ value: Int32) {
        self.value = value
    }
    public func current() -> Int32 { value }
}

public final class ExternalRefCounter: ExternalCurrentLike, ExternalRefCurrentLike {
    public var value: Int32
    public init(_ value: Int32) {
        self.value = value
    }
    public func current() -> Int32 { value }
}

public func external_existential_value_current() -> Int32 {
    let anyValue: any ExternalCurrentLike = ExternalValueCounter(91)
    return anyValue.current()
}

public func external_existential_ref_current() -> Int32 {
    let anyRef: any ExternalCurrentLike = ExternalRefCounter(73)
    return anyRef.current()
}

public func external_class_existential_current() -> Int32 {
    let anyClassRef: any ExternalRefCurrentLike = ExternalRefCounter(64)
    return anyClassRef.current()
}