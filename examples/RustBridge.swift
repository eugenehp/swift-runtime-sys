import Foundation

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
