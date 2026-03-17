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