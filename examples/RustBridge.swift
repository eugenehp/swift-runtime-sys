import Foundation
import Dispatch
import ObjectiveC.runtime

public var globalCounterValue: Int32 = 123

public struct Person {
    public let id: Int32
    public let age: Int32

    public init(id: Int32, age: Int32) {
        self.id = id
        self.age = age
    }
}

public protocol CounterLike {
    func current() -> Int32
}

public final class Counter {
    private var value: Int32

    public init(start: Int32) {
        self.value = start
    }

    public func increment(by delta: Int32) -> Int32 {
        value += delta
        return value
    }

    public func current() -> Int32 {
        value
    }

    public func reset(to newValue: Int32) {
        value = newValue
    }

    public func addPair(_ a: Int32, _ b: Int32) -> Int32 {
        value += a + b
        return value
    }

    public func clear() {
        value = 0
    }
}

extension Counter: CounterLike {}

// --- Enum: simple raw-representable (Int32 discriminant, 4 bytes) ---
public enum Direction: Int32 {
    case north = 0
    case south = 1
    case east  = 2
    case west  = 3
}

/// Global storage for Direction — manipulated directly from Rust via mangled symbol.
public var currentDirection: Direction = .north

@_cdecl("swift_direction_raw")
public func swift_direction_raw() -> Int32 {
    currentDirection.rawValue
}

// --- Enum: associated values (circle has 1 Float, rectangle has 2 Floats) ---
public enum Shape {
    case circle(radius: Float)
    case rectangle(width: Float, height: Float)

    public func area() -> Float {
        switch self {
        case .circle(let r):           return Float.pi * r * r
        case .rectangle(let w, let h): return w * h
        }
    }
}

@_cdecl("swift_shape_circle_area")
public func swift_shape_circle_area(_ radius: Float) -> Float {
    Shape.circle(radius: radius).area()
}

@_cdecl("swift_shape_rect_area")
public func swift_shape_rect_area(_ w: Float, _ h: Float) -> Float {
    Shape.rectangle(width: w, height: h).area()
}

private final class Box<T> {
    let value: T

    init(_ value: T) {
        self.value = value
    }
}

@_cdecl("swift_person_new")
public func swift_person_new(_ id: Int32, _ age: Int32) -> UnsafeMutableRawPointer {
    let boxed = Box(Person(id: id, age: age))
    return Unmanaged.passRetained(boxed).toOpaque()
}

@_cdecl("swift_person_get_id")
public func swift_person_get_id(_ person: UnsafeMutableRawPointer?) -> Int32 {
    guard let person else { return 0 }
    let boxed = Unmanaged<Box<Person>>.fromOpaque(person).takeUnretainedValue()
    return boxed.value.id
}

@_cdecl("swift_person_get_age")
public func swift_person_get_age(_ person: UnsafeMutableRawPointer?) -> Int32 {
    guard let person else { return 0 }
    let boxed = Unmanaged<Box<Person>>.fromOpaque(person).takeUnretainedValue()
    return boxed.value.age
}

@_cdecl("swift_person_drop")
public func swift_person_drop(_ person: UnsafeMutableRawPointer?) {
    guard let person else { return }
    _ = Unmanaged<Box<Person>>.fromOpaque(person).takeRetainedValue()
}

@_cdecl("swift_counter_new")
public func swift_counter_new(_ start: Int32) -> UnsafeMutableRawPointer {
    let counter = Counter(start: start)
    return Unmanaged.passRetained(counter).toOpaque()
}

@_cdecl("swift_counter_increment")
public func swift_counter_increment(_ counter: UnsafeMutableRawPointer?, _ delta: Int32) -> Int32 {
    guard let counter else { return 0 }
    let instance = Unmanaged<Counter>.fromOpaque(counter).takeUnretainedValue()
    return instance.increment(by: delta)
}

@_cdecl("swift_counter_drop")
public func swift_counter_drop(_ counter: UnsafeMutableRawPointer?) {
    guard let counter else { return }
    _ = Unmanaged<Counter>.fromOpaque(counter).takeRetainedValue()
}

@_cdecl("swift_add")
public func swift_add(_ a: Int32, _ b: Int32) -> Int32 {
    a + b
}

@_cdecl("swift_greet")
public func swift_greet() -> UnsafePointer<CChar>? {
    strdup("Hello from Swift").map { UnsafePointer($0) }
}

@_cdecl("swift_string_free")
public func swift_string_free(_ ptr: UnsafeMutablePointer<CChar>?) {
    guard let ptr else { return }
    free(ptr)
}

// ── throws ─────────────────────────────────────────────────────────────────
public enum MathError: Error {
    case divisionByZero
}

public func safeDivide(_ a: Int32, _ b: Int32) throws -> Int32 {
    if b == 0 { throw MathError.divisionByZero }
    return a / b
}

@_cdecl("swift_safe_divide_try")
public func swift_safe_divide_try(_ a: Int32, _ b: Int32) -> Int32 {
    do {
        return try safeDivide(a, b)
    } catch {
        return Int32.min
    }
}

@_cdecl("swift_safe_divide_did_throw")
public func swift_safe_divide_did_throw(_ a: Int32, _ b: Int32) -> Int32 {
    do {
        _ = try safeDivide(a, b)
        return 0
    } catch {
        return 1
    }
}

// ── Generic class TypedBox<T> ───────────────────────────────────────────────
public final class TypedBox<T> {
    public var value: T
    public init(_ v: T) { self.value = v }
}

@_cdecl("swift_typed_box_i32_new")
public func swift_typed_box_i32_new(_ v: Int32) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(TypedBox<Int32>(v)).toOpaque()
}

@_cdecl("swift_typed_box_i32_get")
public func swift_typed_box_i32_get(_ p: UnsafeMutableRawPointer) -> Int32 {
    Unmanaged<TypedBox<Int32>>.fromOpaque(p).takeUnretainedValue().value
}

@_cdecl("swift_typed_box_i32_set")
public func swift_typed_box_i32_set(_ p: UnsafeMutableRawPointer, _ v: Int32) {
    Unmanaged<TypedBox<Int32>>.fromOpaque(p).takeUnretainedValue().value = v
}

@_cdecl("swift_typed_box_i32_drop")
public func swift_typed_box_i32_drop(_ p: UnsafeMutableRawPointer) {
    _ = Unmanaged<TypedBox<Int32>>.fromOpaque(p).takeRetainedValue()
}

// ── String (heap) ──────────────────────────────────────────────────────────
private final class StringBox {
    var value: String
    init(_ s: String) { self.value = s }
}

@_cdecl("swift_string_new")
public func swift_string_new(_ ptr: UnsafePointer<CChar>) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(StringBox(String(cString: ptr))).toOpaque()
}

@_cdecl("swift_string_length")
public func swift_string_length(_ p: UnsafeMutableRawPointer) -> Int32 {
    Int32(Unmanaged<StringBox>.fromOpaque(p).takeUnretainedValue().value.count)
}

@_cdecl("swift_string_utf8_length")
public func swift_string_utf8_length(_ p: UnsafeMutableRawPointer) -> Int32 {
    Int32(Unmanaged<StringBox>.fromOpaque(p).takeUnretainedValue().value.utf8.count)
}

@_cdecl("swift_string_storage_probe_flags")
public func swift_string_storage_probe_flags() -> Int32 {
    let short = "abc"
    let long = String(repeating: "a", count: 80)

    let nsShort = short as NSString
    let nsLong = long as NSString
    let shortClassName = String(cString: object_getClassName(nsShort))
    let longClassName = String(cString: object_getClassName(nsLong))

    var flags: Int32 = 0
    if shortClassName != longClassName { flags |= 1 }
    if short.utf8.count == 3 { flags |= 2 }
    if long.utf8.count == 80 { flags |= 4 }
    return flags
}

@_cdecl("swift_string_drop")
public func swift_string_drop(_ p: UnsafeMutableRawPointer) {
    _ = Unmanaged<StringBox>.fromOpaque(p).takeRetainedValue()
}

// ── Point struct (direct mangled-symbol method dispatch, no @_cdecl wrapper) ──
public struct Point {
    public var x: Int32
    public var y: Int32
    public init(x: Int32, y: Int32) { self.x = x; self.y = y }
    public func sum() -> Int32 { x + y }
    public func product() -> Int32 { x * y }
}

// ── Tuple return ───────────────────────────────────────────────────────────
// Not @_cdecl (tuples not allowed there); called via mangled symbol.
public func splitAdd(_ a: Int32, _ b: Int32) -> (Int32, Int32) { (a + b, a - b) }

// ── Optional<T> layout ─────────────────────────────────────────────────────
public var optionalNone: Int32? = nil
public var optionalSome: Int32? = 42

@_cdecl("swift_optional_none_get")
public func swift_optional_none_get() -> Int32 { optionalNone ?? -999 }

@_cdecl("swift_optional_some_get")
public func swift_optional_some_get() -> Int32 { optionalSome ?? -999 }

// ── Array<T> ───────────────────────────────────────────────────────────────
public var sharedArray: [Int32] = [10, 20, 30, 40, 50]

@_cdecl("swift_array_count")
public func swift_array_count() -> Int32 { Int32(sharedArray.count) }

@_cdecl("swift_array_get")
public func swift_array_get(_ idx: Int32) -> Int32 {
    guard idx >= 0, Int(idx) < sharedArray.count else { return Int32.min }
    return sharedArray[Int(idx)]
}

@_cdecl("swift_array_append")
public func swift_array_append(_ v: Int32) { sharedArray.append(v) }

@_cdecl("swift_array_cow_probe_flags")
public func swift_array_cow_probe_flags() -> Int32 {
    let a: [Int32] = [1, 2, 3, 4]
    var b = a

    let sharedBefore = a.withUnsafeBufferPointer { ap in
        b.withUnsafeBufferPointer { bp in
            ap.baseAddress == bp.baseAddress
        }
    }

    b.append(5)

    let splitAfter = a.withUnsafeBufferPointer { ap in
        b.withUnsafeBufferPointer { bp in
            ap.baseAddress != bp.baseAddress
        }
    }

    let originalUnchanged = (a.count == 4 && b.count == 5 && a[0] == 1 && a[3] == 4)

    var flags: Int32 = 0
    if sharedBefore { flags |= 1 }
    if splitAfter { flags |= 2 }
    if originalUnchanged { flags |= 4 }
    return flags
}

// ── Closure (thick fn ptr stored globally, invoked via bridge) ─────────────
private var _storedClosure: ((Int32) -> Int32)?

@_cdecl("swift_store_adder_closure")
public func swift_store_adder_closure(_ delta: Int32) {
    _storedClosure = { x in x + delta }
}

@_cdecl("swift_invoke_stored_closure")
public func swift_invoke_stored_closure(_ x: Int32) -> Int32 {
    _storedClosure?(x) ?? Int32.min
}

// ── Reflection (Mirror) ────────────────────────────────────────────────────
@_cdecl("swift_point_field_count")
public func swift_point_field_count() -> Int32 {
    Int32(Array(Mirror(reflecting: Point(x: 0, y: 0)).children).count)
}

@_cdecl("swift_point_first_field_is_x")
public func swift_point_first_field_is_x() -> Int32 {
    Mirror(reflecting: Point(x: 0, y: 0)).children.first?.label == "x" ? 1 : 0
}

// ── Error boxing ───────────────────────────────────────────────────────────
@_cdecl("swift_make_math_error")
public func swift_make_math_error() -> UnsafeMutableRawPointer {
    let e = NSError(domain: "RustBridge.MathError", code: 0,
                    userInfo: [NSLocalizedDescriptionKey: "divisionByZero"])
    return Unmanaged.passRetained(e).toOpaque()
}

@_cdecl("swift_drop_error")
public func swift_drop_error(_ p: UnsafeMutableRawPointer) {
    _ = Unmanaged<NSError>.fromOpaque(p).takeRetainedValue()
}

@_cdecl("swift_check_math_error")
public func swift_check_math_error(_ p: UnsafeMutableRawPointer) -> Int32 {
    let e = Unmanaged<NSError>.fromOpaque(p).takeUnretainedValue()
    var flags: Int32 = 0
    if e.domain == "RustBridge.MathError" { flags |= 1 }
    if e.code == 0 { flags |= 2 }
    if e.localizedDescription.contains("divisionByZero") { flags |= 4 }
    return flags
}

@objcMembers
public final class ObjCBridgeCounter: NSObject {
    public dynamic var value: Int32

    public init(_ start: Int32) {
        self.value = start
        super.init()
    }

    public dynamic func bump(_ delta: NSNumber) -> NSNumber {
        value += delta.int32Value
        return NSNumber(value: value)
    }
}

@_cdecl("swift_objc_interop_probe_flags")
public func swift_objc_interop_probe_flags() -> Int32 {
    let counter = ObjCBridgeCounter(10)
    let bumpSel = #selector(ObjCBridgeCounter.bump(_:))
    let responds = counter.responds(to: bumpSel)

    var selectorOk = false
    if responds,
       let out = counter.perform(bumpSel, with: NSNumber(value: 5))?.takeUnretainedValue() as? NSNumber {
        selectorOk = (out.int32Value == 15)
    }

    let nsStr = NSString(string: "bridge")
    let stringBridgeOk = ((nsStr as String) == "bridge")

    let nsArr: NSArray = [NSNumber(value: 1), NSNumber(value: 2), NSNumber(value: 3)]
    let swiftArr = nsArr as? [NSNumber]
    let arrayBridgeOk = (swiftArr?.count == 3 && swiftArr?[2].int32Value == 3)

    var flags: Int32 = 0
    if selectorOk { flags |= 1 }
    if stringBridgeOk { flags |= 2 }
    if arrayBridgeOk { flags |= 4 }
    return flags
}

// ── Async/task runtime ───────────────────────────────────────────────────────
public func asyncAdd(_ a: Int32, _ b: Int32) async -> Int32 {
    a + b
}

public func asyncSafeDivide(_ a: Int32, _ b: Int32) async throws -> Int32 {
    try safeDivide(a, b)
}

@_cdecl("swift_async_add_blocking")
public func swift_async_add_blocking(_ a: Int32, _ b: Int32) -> Int32 {
    let sem = DispatchSemaphore(value: 0)
    var out = Int32.min
    Task {
        out = await asyncAdd(a, b)
        sem.signal()
    }
    sem.wait()
    return out
}

@_cdecl("swift_async_divide_try_blocking")
public func swift_async_divide_try_blocking(_ a: Int32, _ b: Int32) -> Int32 {
    let sem = DispatchSemaphore(value: 0)
    var out = Int32.min
    Task {
        do {
            out = try await asyncSafeDivide(a, b)
        } catch {
            out = Int32.min
        }
        sem.signal()
    }
    sem.wait()
    return out
}

@_cdecl("swift_async_divide_did_throw_blocking")
public func swift_async_divide_did_throw_blocking(_ a: Int32, _ b: Int32) -> Int32 {
    let sem = DispatchSemaphore(value: 0)
    var didThrow: Int32 = 0
    Task {
        do {
            _ = try await asyncSafeDivide(a, b)
            didThrow = 0
        } catch {
            didThrow = 1
        }
        sem.signal()
    }
    sem.wait()
    return didThrow
}

// ── Actor/executor behavior ─────────────────────────────────────────────────
public actor CounterActor {
    private var value: Int32

    public init(start: Int32) {
        self.value = start
    }

    public func increment(by delta: Int32) -> Int32 {
        value += delta
        return value
    }

    public func current() -> Int32 {
        value
    }
}

private var _sharedActorCounter: CounterActor?

@_cdecl("swift_actor_counter_create")
public func swift_actor_counter_create(_ start: Int32) -> Int32 {
    _sharedActorCounter = CounterActor(start: start)
    return 1
}

@_cdecl("swift_actor_counter_increment_blocking")
public func swift_actor_counter_increment_blocking(_ delta: Int32) -> Int32 {
    guard let actor = _sharedActorCounter else { return Int32.min }
    let sem = DispatchSemaphore(value: 0)
    var out = Int32.min
    Task {
        out = await actor.increment(by: delta)
        sem.signal()
    }
    sem.wait()
    return out
}

@_cdecl("swift_actor_counter_current_blocking")
public func swift_actor_counter_current_blocking() -> Int32 {
    guard let actor = _sharedActorCounter else { return Int32.min }
    let sem = DispatchSemaphore(value: 0)
    var out = Int32.min
    Task {
        out = await actor.current()
        sem.signal()
    }
    sem.wait()
    return out
}

// ── Generic metadata parity (multi-type instantiation) ─────────────────────
private protocol ValueLike {
    func value() -> Int32
}

private struct ValueHolder: ValueLike {
    let v: Int32
    func value() -> Int32 { v }
}

private func consumeValueLike<T: ValueLike>(_ t: T) -> Int32 {
    t.value()
}

@_cdecl("swift_generic_metadata_distinct")
public func swift_generic_metadata_distinct() -> Int32 {
    let a = ObjectIdentifier(TypedBox<Int32>.self)
    let b = ObjectIdentifier(TypedBox<String>.self)
    let c = ObjectIdentifier(TypedBox<TypedBox<Int32>>.self)
    return (a != b && b != c && a != c) ? 1 : 0
}

@_cdecl("swift_generic_constrained_call")
public func swift_generic_constrained_call() -> Int32 {
    consumeValueLike(ValueHolder(v: 77))
}

// ── Value existential dispatch parity ───────────────────────────────────────
public protocol ValueCurrentLike {
    func current() -> Int32
}

public struct ValueCounter: ValueCurrentLike {
    public var value: Int32
    public init(_ value: Int32) { self.value = value }
    public func current() -> Int32 { value }
}

@_cdecl("swift_value_existential_current")
public func swift_value_existential_current() -> Int32 {
    let anyValue: any ValueCurrentLike = ValueCounter(88)
    return anyValue.current()
}

// ── Resilient layout parity probes ──────────────────────────────────────────
public struct ResilientLike {
    public var a: Int32
    public var b: Int64
    public init(a: Int32, b: Int64) {
        self.a = a
        self.b = b
    }
}

@_cdecl("swift_layout_point_size")
public func swift_layout_point_size() -> Int32 {
    Int32(MemoryLayout<Point>.size)
}

@_cdecl("swift_layout_point_stride")
public func swift_layout_point_stride() -> Int32 {
    Int32(MemoryLayout<Point>.stride)
}

@_cdecl("swift_layout_point_alignment")
public func swift_layout_point_alignment() -> Int32 {
    Int32(MemoryLayout<Point>.alignment)
}

@_cdecl("swift_layout_resilient_size")
public func swift_layout_resilient_size() -> Int32 {
    Int32(MemoryLayout<ResilientLike>.size)
}

@_cdecl("swift_layout_resilient_stride")
public func swift_layout_resilient_stride() -> Int32 {
    Int32(MemoryLayout<ResilientLike>.stride)
}

@_cdecl("swift_layout_resilient_alignment")
public func swift_layout_resilient_alignment() -> Int32 {
    Int32(MemoryLayout<ResilientLike>.alignment)
}

@_cdecl("swift_layout_resilient_b_offset")
public func swift_layout_resilient_b_offset() -> Int32 {
    var x = ResilientLike(a: 1, b: 2)
    return withUnsafeMutablePointer(to: &x) { base in
        withUnsafeMutablePointer(to: &base.pointee.b) { bptr in
            Int32(Int(bitPattern: bptr) - Int(bitPattern: base))
        }
    }
}

// ── ARC edge-case stress probes ─────────────────────────────────────────────
private final class ArcTracked {
    static var deinitCount: Int32 = 0
    deinit { ArcTracked.deinitCount += 1 }
}

@_cdecl("swift_arc_edge_stress")
public func swift_arc_edge_stress(_ iterations: Int32) -> Int32 {
    ArcTracked.deinitCount = 0
    var strong: ArcTracked? = ArcTracked()
    weak let weakRef = strong

    if let obj = strong {
        let n = max(0, Int(iterations))
        for _ in 0..<n {
            let p = Unmanaged.passRetained(obj).toOpaque()
            _ = Unmanaged<ArcTracked>.fromOpaque(p).takeRetainedValue()
        }
    }

    let beforeDrop = ArcTracked.deinitCount
    strong = nil
    let afterDrop = ArcTracked.deinitCount
    let weakCleared = (weakRef == nil)

    return (beforeDrop == 0 && afterDrop == 1 && weakCleared) ? 1 : 0
}
