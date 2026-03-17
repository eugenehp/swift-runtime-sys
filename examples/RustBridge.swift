import Foundation
import Dispatch
import ObjectiveC.runtime
import ResilientFixtures

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

public enum MultiPayloadEncoding {
    case int(Int32)
    case double(Double)
    case none
}

public enum SpareBitRef {
    case object(NSObject)
    case none
}

public struct CodablePayload: Codable, Equatable {
    public var id: Int32
    public var name: String
    public var values: [Int32]
}

private func rawBytes<T>(of value: T) -> [UInt8] {
    withUnsafeBytes(of: value) { Array($0) }
}

@_cdecl("swift_enum_payload_probe_flags")
public func swift_enum_payload_probe_flags() -> Int32 {
    let intZero = MultiPayloadEncoding.int(0)
    let doubleZero = MultiPayloadEncoding.double(0)
    let noneValue = MultiPayloadEncoding.none

    let intBytes = rawBytes(of: intZero)
    let doubleBytes = rawBytes(of: doubleZero)
    let noneBytes = rawBytes(of: noneValue)

    let multiSemanticOk: Bool = {
        if case .int(let value) = MultiPayloadEncoding.int(41), value == 41 {
            if case .double(let value) = MultiPayloadEncoding.double(2.5), value == 2.5 {
                if case .none = MultiPayloadEncoding.none {
                    return true
                }
            }
        }
        return false
    }()

    let multiDistinct = (intBytes != doubleBytes && intBytes != noneBytes && doubleBytes != noneBytes)
    let multiLayoutSane = (MemoryLayout<MultiPayloadEncoding>.size <= 16 &&
        MemoryLayout<MultiPayloadEncoding>.stride <= 16 &&
        MemoryLayout<MultiPayloadEncoding>.alignment >= 4)

    let spareNone = SpareBitRef.none
    let spareObject = SpareBitRef.object(NSObject())
    let spareNoneBytes = rawBytes(of: spareNone)
    let spareObjectBytes = rawBytes(of: spareObject)

    let spareSemanticOk: Bool = {
        if case .none = spareNone {
            if case .object = spareObject {
                return true
            }
        }
        return false
    }()

    let spareNilZero = spareNoneBytes.allSatisfy { $0 == 0 }
    let spareSomeNonZero = spareObjectBytes.contains { $0 != 0 }
    let spareSizeEight = (MemoryLayout<SpareBitRef>.size == 8)

    var flags: Int32 = 0
    if multiSemanticOk { flags |= 1 << 0 }
    if multiDistinct { flags |= 1 << 1 }
    if multiLayoutSane { flags |= 1 << 2 }
    if spareSemanticOk { flags |= 1 << 3 }
    if spareNilZero { flags |= 1 << 4 }
    if spareSomeNonZero { flags |= 1 << 5 }
    if spareSizeEight { flags |= 1 << 6 }
    return flags
}

@_cdecl("swift_codable_probe_flags")
public func swift_codable_probe_flags() -> Int32 {
    let payload = CodablePayload(id: 7, name: "swift", values: [1, 2, 3])

    var flags: Int32 = 0
    let encoder = JSONEncoder()
    let decoder = JSONDecoder()

    if let data = try? encoder.encode(payload),
       let json = String(data: data, encoding: .utf8) {
        if json.contains("\"id\":7") && json.contains("\"name\":\"swift\"") {
            flags |= 1 << 0
        }
        if let decoded = try? decoder.decode(CodablePayload.self, from: data), decoded == payload {
            flags |= 1 << 1
        }
    }

    let known = "{\"id\":11,\"name\":\"bridge\",\"values\":[4,5]}".data(using: .utf8)
    if let known,
       let decoded = try? decoder.decode(CodablePayload.self, from: known),
       decoded == CodablePayload(id: 11, name: "bridge", values: [4, 5]) {
        flags |= 1 << 2
    }

    return flags
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

@inline(never)
private func applyTransform<T>(_ value: T, _ transform: (T) -> T) -> T {
    transform(value)
}

@inline(never)
private func liftTransform<T>(_ transform: @escaping (T) -> T) -> (T) -> T {
    { value in transform(value) }
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

@_cdecl("swift_generic_specialization_probe_flags")
public func swift_generic_specialization_probe_flags() -> Int32 {
    let intResult = applyTransform(Int32(21)) { $0 + 21 }
    let stringResult = applyTransform("ab") { $0 + "cd" }
    let lifted = liftTransform { (x: Int32) in x * 2 }
    let reabstractedResult = lifted(9)

    var flags: Int32 = 0
    if intResult == 42 { flags |= 1 }
    if stringResult == "abcd" { flags |= 2 }
    if reabstractedResult == 18 { flags |= 4 }
    return flags
}

// ── Synthesized witness parity (Equatable/Hashable) ───────────────────────
public struct SynthEqHash: Equatable, Hashable {
    public var a: Int32
    public var b: Int32
    public init(a: Int32, b: Int32) {
        self.a = a
        self.b = b
    }
}

@_cdecl("swift_synth_eq_hash_probe_flags")
public func swift_synth_eq_hash_probe_flags() -> Int32 {
    let lhs = SynthEqHash(a: 7, b: 9)
    let rhsSame = SynthEqHash(a: 7, b: 9)
    let rhsDiff = SynthEqHash(a: 7, b: 10)

    let eqTrue = (lhs == rhsSame)
    let eqFalse = (lhs != rhsDiff)

    var set = Set<SynthEqHash>()
    set.insert(lhs)
    set.insert(rhsSame)
    set.insert(rhsDiff)
    let dedupOk = (set.count == 2)

    var flags: Int32 = 0
    if eqTrue { flags |= 1 }
    if eqFalse { flags |= 2 }
    if dedupOk { flags |= 4 }
    return flags
}

// ── KeyPath synthesis parity ───────────────────────────────────────────────
public struct KeyPathSynthPoint {
    public var x: Int32
    public var y: Int32
    public init(x: Int32, y: Int32) {
        self.x = x
        self.y = y
    }
}

public struct KeyPathSynthContainer {
    public var point: KeyPathSynthPoint
    public init(point: KeyPathSynthPoint) {
        self.point = point
    }
}

@_cdecl("swift_keypath_synth_probe_flags")
public func swift_keypath_synth_probe_flags() -> Int32 {
    var value = KeyPathSynthContainer(point: KeyPathSynthPoint(x: 11, y: 4))

    let kpX = \KeyPathSynthContainer.point.x
    let writableX = \KeyPathSynthContainer.point.x
    let writableY = \KeyPathSynthContainer.point.y

    let readOk = (value[keyPath: kpX] == 11)
    value[keyPath: writableX] = 21
    let writeOk = (value.point.x == 21)

    let composed = writableY.appending(path: \.magnitude)
    let appendOk = (value[keyPath: composed] == 4)

    var flags: Int32 = 0
    if readOk { flags |= 1 }
    if writeOk { flags |= 2 }
    if appendOk { flags |= 4 }
    return flags
}

// ── Property-wrapper synthesis parity ──────────────────────────────────────
@propertyWrapper
public struct ClampedNonNegative {
    private var storage: Int32

    public var wrappedValue: Int32 {
        get { storage }
        set { storage = max(0, newValue) }
    }

    public var projectedValue: Int32 {
        storage
    }

    public init(wrappedValue: Int32) {
        self.storage = max(0, wrappedValue)
    }
}

public struct PropertyWrapperSynthCounter {
    @ClampedNonNegative public var value: Int32 = 3
}

@_cdecl("swift_property_wrapper_synth_probe_flags")
public func swift_property_wrapper_synth_probe_flags() -> Int32 {
    var sample = PropertyWrapperSynthCounter()

    let defaultInitOk = (sample.value == 3)
    sample.value = -9
    let clampOk = (sample.value == 0)
    let projectedOk = (sample.$value == 0)

    let memberwise = PropertyWrapperSynthCounter(value: 17)
    let memberwiseInitOk = (memberwise.value == 17)

    var flags: Int32 = 0
    if defaultInitOk { flags |= 1 }
    if clampOk { flags |= 2 }
    if projectedOk { flags |= 4 }
    if memberwiseInitOk { flags |= 8 }
    return flags
}

// ── Result-builder synthesis parity ───────────────────────────────────────
@resultBuilder
public enum IntSequenceBuilder {
    public static func buildBlock(_ components: [Int32]...) -> [Int32] {
        components.flatMap { $0 }
    }

    public static func buildExpression(_ expression: Int32) -> [Int32] {
        [expression]
    }

    public static func buildOptional(_ component: [Int32]?) -> [Int32] {
        component ?? []
    }

    public static func buildEither(first component: [Int32]) -> [Int32] {
        component
    }

    public static func buildEither(second component: [Int32]) -> [Int32] {
        component
    }

    public static func buildArray(_ components: [[Int32]]) -> [Int32] {
        components.flatMap { $0 }
    }
}

private func buildSequence(
    includeBranch: Bool,
    includeOptional: Bool,
    loopValues: [Int32]
) -> [Int32] {
    @IntSequenceBuilder var seq: [Int32] {
        1
        if includeBranch {
            2
        } else {
            3
        }
        if includeOptional {
            6
        }
        for value in loopValues {
            value
        }
    }
    return seq
}

@_cdecl("swift_result_builder_synth_probe_flags")
public func swift_result_builder_synth_probe_flags() -> Int32 {
    let first = buildSequence(includeBranch: true, includeOptional: true, loopValues: [4, 5])
    let second = buildSequence(includeBranch: false, includeOptional: false, loopValues: [7])

    let branchEitherOk = (first == [1, 2, 6, 4, 5] && second == [1, 3, 7])
    let optionalOk = (first.contains(6) && !second.contains(6))
    let loopOk = (first.suffix(2) == [4, 5] && second.suffix(1) == [7])

    var flags: Int32 = 0
    if branchEitherOk { flags |= 1 }
    if optionalOk { flags |= 2 }
    if loopOk { flags |= 4 }
    return flags
}

// ── Opaque return-type parity ─────────────────────────────────────────────
public protocol OpaqueReadable {
    func asI32() -> Int32
}

public struct OpaqueReadableImpl: OpaqueReadable, Equatable {
    public let value: Int32
    public func asI32() -> Int32 { value }
}

@inline(never)
public func makeOpaqueReadable(_ value: Int32) -> some OpaqueReadable {
    OpaqueReadableImpl(value: value)
}

@inline(never)
private func consumeOpaqueReadable<T: OpaqueReadable>(_ value: T) -> Int32 {
    value.asI32()
}

@_cdecl("swift_opaque_return_probe_flags")
public func swift_opaque_return_probe_flags() -> Int32 {
    let first = makeOpaqueReadable(7)
    let second = makeOpaqueReadable(9)

    let valueOk = (first.asI32() == 7 && second.asI32() == 9)
    let genericConsumeOk = (consumeOpaqueReadable(first) == 7)
    let stableUnderlyingTypeOk = (type(of: first) == type(of: second))

    var flags: Int32 = 0
    if valueOk { flags |= 1 }
    if genericConsumeOk { flags |= 2 }
    if stableUnderlyingTypeOk { flags |= 4 }
    return flags
}

// ── Task-local runtime parity ─────────────────────────────────────────────
enum RuntimeTaskLocal {
    @TaskLocal static var value: Int32 = -1
}

private func runTaskLocalProbe() -> (Int32, Int32, Int32, Int32) {
    let outside = RuntimeTaskLocal.value
    let (inside, nested) = RuntimeTaskLocal.$value.withValue(41) {
        let inside = RuntimeTaskLocal.value
        let nested = RuntimeTaskLocal.$value.withValue(99) {
            RuntimeTaskLocal.value
        }
        return (inside, nested)
    }
    let after = RuntimeTaskLocal.value
    return (outside, inside, nested, after)
}

@_cdecl("swift_task_local_probe_flags")
public func swift_task_local_probe_flags() -> Int32 {
    let sem = DispatchSemaphore(value: 0)
    var tuple = (Int32.min, Int32.min, Int32.min, Int32.min)
    Task {
        tuple = runTaskLocalProbe()
        sem.signal()
    }
    sem.wait()

    let outsideOk = (tuple.0 == -1)
    let insideOk = (tuple.1 == 41)
    let nestedOk = (tuple.2 == 99)
    let restoredOk = (tuple.3 == -1)

    var flags: Int32 = 0
    if outsideOk { flags |= 1 }
    if insideOk { flags |= 2 }
    if nestedOk { flags |= 4 }
    if restoredOk { flags |= 8 }
    return flags
}

// ── Dynamic replacement parity ───────────────────────────────────────────
public struct DynamicReplacementHarness {
    public init() {}

    public dynamic func target(_ value: Int32) -> Int32 {
        value + 1
    }
}

extension DynamicReplacementHarness {
    @_dynamicReplacement(for: target(_:))
    public func target_replacement(_ value: Int32) -> Int32 {
        value + 11
    }
}

@_cdecl("swift_dynamic_replacement_probe_flags")
public func swift_dynamic_replacement_probe_flags() -> Int32 {
    let harness = DynamicReplacementHarness()
    let direct = harness.target(5)
    let fnRef: (Int32) -> Int32 = harness.target
    let indirect = fnRef(6)

    let directOk = (direct == 16)
    let indirectOk = (indirect == 17)

    var flags: Int32 = 0
    if directOk { flags |= 1 }
    if indirectOk { flags |= 2 }
    return flags
}

// ── Sendable concurrency parity ──────────────────────────────────────────
public struct SendablePayload: Sendable {
    public var value: Int32
}

private func runSendableProbe() async -> (Int32, Int32, Int32) {
    let payload = SendablePayload(value: 41)
    let detached = await Task.detached(priority: nil) { payload.value + 1 }.value

    let childTask = Task { payload.value + 2 }
    let child = await childTask.value

    return (payload.value, detached, child)
}

@_cdecl("swift_sendable_probe_flags")
public func swift_sendable_probe_flags() -> Int32 {
    let sem = DispatchSemaphore(value: 0)
    var tuple = (Int32.min, Int32.min, Int32.min)
    Task {
        tuple = await runSendableProbe()
        sem.signal()
    }
    sem.wait()

    let payloadOk = (tuple.0 == 41)
    let detachedOk = (tuple.1 == 42)
    let childOk = (tuple.2 == 43)

    var flags: Int32 = 0
    if payloadOk { flags |= 1 }
    if detachedOk { flags |= 2 }
    if childOk { flags |= 4 }
    return flags
}

// ── Checked-continuation parity ────────────────────────────────────────────
private func runContinuationProbe() async -> (Int32, Int32, Int32) {
    // bit 1: async callback → continuation resume (71)
    let asyncVal: Int32 = await withCheckedContinuation { continuation in
        DispatchQueue.global().async {
            continuation.resume(returning: Int32(71))
        }
    }
    // bit 2: synchronous inline resume (72)
    let syncVal: Int32 = await withCheckedContinuation { continuation in
        continuation.resume(returning: Int32(72))
    }
    // bit 4: throwing continuation success path (73)
    let throwingVal: Int32 = (try? await withCheckedThrowingContinuation { continuation in
        continuation.resume(returning: Int32(73))
    }) ?? 0
    return (asyncVal, syncVal, throwingVal)
}

@_cdecl("swift_continuation_probe_flags")
public func swift_continuation_probe_flags() -> Int32 {
    let sem = DispatchSemaphore(value: 0)
    var tuple = (Int32.min, Int32.min, Int32.min)
    Task {
        tuple = await runContinuationProbe()
        sem.signal()
    }
    sem.wait()
    var flags: Int32 = 0
    if tuple.0 == 71 { flags |= 1 }
    if tuple.1 == 72 { flags |= 2 }
    if tuple.2 == 73 { flags |= 4 }
    return flags
}

// ── TaskGroup structured concurrency parity ──────────────────────────────────
private func runTaskGroupProbe() async -> (Int32, Int32, Int32) {
    // bit 1: sum of task outputs == 150 (10+20+30+40+50)
    let sum = await withTaskGroup(of: Int32.self) { group in
        for i: Int32 in [10, 20, 30, 40, 50] {
            group.addTask { i }
        }
        var total: Int32 = 0
        for await val in group { total += val }
        return total
    }
    // bit 2: throwing group success sum == 306 (101+102+103)
    let throwSum: Int32 = (try? await withThrowingTaskGroup(of: Int32.self) { group in
        for i: Int32 in [101, 102, 103] {
            group.addTask { i }
        }
        var total: Int32 = 0
        for try await val in group { total += val }
        return total
    }) ?? 0
    // bit 4: max from group == 99
    let maxVal = await withTaskGroup(of: Int32.self) { group in
        for v: Int32 in [7, 3, 99, 12] {
            group.addTask { v }
        }
        var m: Int32 = 0
        for await val in group { m = max(m, val) }
        return m
    }
    return (sum, throwSum, maxVal)
}

@_cdecl("swift_task_group_probe_flags")
public func swift_task_group_probe_flags() -> Int32 {
    let sem = DispatchSemaphore(value: 0)
    var tuple = (Int32.min, Int32.min, Int32.min)
    Task {
        tuple = await runTaskGroupProbe()
        sem.signal()
    }
    sem.wait()
    var flags: Int32 = 0
    if tuple.0 == 150 { flags |= 1 }
    if tuple.1 == 306 { flags |= 2 }
    if tuple.2 == 99  { flags |= 4 }
    return flags
}

// ── AsyncStream parity ──────────────────────────────────────────────────────
private func runAsyncStreamProbe() async -> (Int32, Int32) {
    let stream = AsyncStream<Int32> { continuation in
        for v: Int32 in [10, 20, 30, 40, 50] { continuation.yield(v) }
        continuation.finish()
    }
    var count: Int32 = 0
    var sum: Int32 = 0
    for await val in stream { count += 1; sum += val }
    return (count, sum)
}

@_cdecl("swift_async_stream_probe_flags")
public func swift_async_stream_probe_flags() -> Int32 {
    let sem = DispatchSemaphore(value: 0)
    var pair = (Int32.min, Int32.min)
    Task {
        pair = await runAsyncStreamProbe()
        sem.signal()
    }
    sem.wait()
    var flags: Int32 = 0
    if pair.0 == 5   { flags |= 1 }  // consumed count == 5
    if pair.1 == 150 { flags |= 2 }  // sum == 150
    if pair.0 == 5 && pair.1 == 150 { flags |= 4 }  // terminates cleanly (no extra values)
    return flags
}

// ── Unsafe memory layout parity ──────────────────────────────────────────────
private struct LayoutProbePoint { var x: Int32; var y: Int32 }

@_cdecl("swift_unsafe_memory_probe_flags")
public func swift_unsafe_memory_probe_flags() -> Int32 {
    var pt = LayoutProbePoint(x: Int32(bitPattern: 0xAABBCCDD), y: Int32(bitPattern: 0x11223344))
    var flags: Int32 = 0
    // bit 1: field x readable via withUnsafeBytes at offset 0
    withUnsafeBytes(of: &pt) { buf in
        if buf.load(fromByteOffset: 0, as: Int32.self) == Int32(bitPattern: 0xAABBCCDD) { flags |= 1 }
    }
    // bit 2: field y readable via withUnsafeBytes at offset 4
    withUnsafeBytes(of: &pt) { buf in
        if buf.load(fromByteOffset: 4, as: Int32.self) == Int32(bitPattern: 0x11223344) { flags |= 2 }
    }
    // bit 4: withUnsafeMutablePointer write + read roundtrip
    var val: Int32 = 77
    withUnsafeMutablePointer(to: &val) { ptr in ptr.pointee = 99 }
    if val == 99 { flags |= 4 }
    return flags
}

// ── Protocol composition existential parity ────────────────────────────────
public protocol Scalable { func scale(_ factor: Int32) -> Int32 }
public protocol Labelable { func label() -> Int32 }

public struct ComposedWidget: Scalable, Labelable {
    public var tag: Int32
    public init(_ tag: Int32) { self.tag = tag }
    public func scale(_ factor: Int32) -> Int32 { tag * factor }
    public func label() -> Int32 { tag + 100 }
}

@_cdecl("swift_protocol_composition_probe_flags")
public func swift_protocol_composition_probe_flags() -> Int32 {
    let widget = ComposedWidget(7)
    let composed: any Scalable & Labelable = widget
    var flags: Int32 = 0
    if composed.scale(3) == 21  { flags |= 1 }
    if composed.label() == 107  { flags |= 2 }
    if let concrete = composed as? ComposedWidget, concrete.tag == 7 { flags |= 4 }
    return flags
}

// ── Enum raw-value synthesis parity ─────────────────────────────────────────
public enum Planet: Int32 {
    case mercury = 1, venus, earth, mars, jupiter, saturn, uranus, neptune
}

@_cdecl("swift_enum_raw_value_probe_flags")
public func swift_enum_raw_value_probe_flags() -> Int32 {
    var flags: Int32 = 0
    if Planet.earth.rawValue == 3      { flags |= 1 }
    if Planet(rawValue: 5) == .jupiter { flags |= 2 }
    if Planet(rawValue: 99) == nil     { flags |= 4 }
    if Planet.neptune.rawValue == 8    { flags |= 8 }
    return flags
}

// ── OptionSet synthesis parity ───────────────────────────────────────────────
public struct RuntimeOptions: OptionSet {
    public let rawValue: Int32
    public init(rawValue: Int32) { self.rawValue = rawValue }
    public static let read = RuntimeOptions(rawValue: 1 << 0)
    public static let write = RuntimeOptions(rawValue: 1 << 1)
    public static let execute = RuntimeOptions(rawValue: 1 << 2)
}

@_cdecl("swift_option_set_probe_flags")
public func swift_option_set_probe_flags() -> Int32 {
    let base: RuntimeOptions = [.read, .write]
    var flags: Int32 = 0
    if base.contains(.read) && base.contains(.write) { flags |= 1 }
    if base.union(.execute).rawValue == 7 { flags |= 2 }
    if base.intersection(.write).rawValue == 2 { flags |= 4 }
    let fromRaw = RuntimeOptions(rawValue: 5)
    if fromRaw.contains(.read) && fromRaw.contains(.execute) { flags |= 8 }
    return flags
}

// ── CaseIterable synthesis parity ───────────────────────────────────────────
public enum BuildStage: Int32, CaseIterable {
    case parse = 10
    case typecheck = 20
    case emit = 30
}

@_cdecl("swift_case_iterable_probe_flags")
public func swift_case_iterable_probe_flags() -> Int32 {
    let cases = BuildStage.allCases
    var flags: Int32 = 0
    if cases.count == 3 { flags |= 1 }
    if cases.first == .parse && cases.last == .emit { flags |= 2 }
    if cases.map(\.rawValue).reduce(0, +) == 60 { flags |= 4 }
    if cases.map(\.rawValue) == [10, 20, 30] { flags |= 8 }
    return flags
}

// ── Set algebra parity ──────────────────────────────────────────────────────
@_cdecl("swift_set_algebra_probe_flags")
public func swift_set_algebra_probe_flags() -> Int32 {
    let a: Set<Int32> = [1, 2, 3]
    let b: Set<Int32> = [3, 4]
    var flags: Int32 = 0

    let union = a.union(b)
    if union.count == 4 && union.contains(4) { flags |= 1 }

    let intersection = a.intersection(b)
    if intersection.count == 1 && intersection.contains(3) { flags |= 2 }

    let subtracting = a.subtracting(b)
    if subtracting == Set([1, 2]) { flags |= 4 }

    let sym = a.symmetricDifference(b)
    if sym == Set([1, 2, 4]) { flags |= 8 }

    return flags
}

// ── Dictionary semantics parity ─────────────────────────────────────────────
@_cdecl("swift_dictionary_probe_flags")
public func swift_dictionary_probe_flags() -> Int32 {
    var dict: [String: Int32] = ["a": 1, "b": 2]
    var flags: Int32 = 0

    if dict["a"] == 1 { flags |= 1 }

    dict["c", default: 0] += 3
    if dict["c"] == 3 { flags |= 2 }

    let oldA = dict.updateValue(9, forKey: "a")
    if oldA == 1 && dict["a"] == 9 { flags |= 4 }

    let removedB = dict.removeValue(forKey: "b")
    if removedB == 2 && dict["b"] == nil && dict.count == 2 { flags |= 8 }

    return flags
}

// ── Comparable synthesis parity ─────────────────────────────────────────────
public struct RankPoint: Comparable {
    public var score: Int32
    public static func < (lhs: RankPoint, rhs: RankPoint) -> Bool {
        lhs.score < rhs.score
    }
}

@_cdecl("swift_comparable_probe_flags")
public func swift_comparable_probe_flags() -> Int32 {
    let values = [RankPoint(score: 7), RankPoint(score: 2), RankPoint(score: 5)]
    let sorted = values.sorted()
    var flags: Int32 = 0
    if sorted.map(\.score) == [2, 5, 7] { flags |= 1 }
    if RankPoint(score: 2) < RankPoint(score: 5) { flags |= 2 }
    if RankPoint(score: 7) > RankPoint(score: 5) { flags |= 4 }
    if RankPoint(score: 5) == RankPoint(score: 5) { flags |= 8 }
    return flags
}

// ── Result semantics parity ─────────────────────────────────────────────────
private enum RuntimeResultError: Error { case bad }

@_cdecl("swift_result_probe_flags")
public func swift_result_probe_flags() -> Int32 {
    let ok: Result<Int32, RuntimeResultError> = .success(41)
    let err: Result<Int32, RuntimeResultError> = .failure(.bad)
    var flags: Int32 = 0

    if (try? ok.get()) == 41 { flags |= 1 }
    if (try? err.get()) == nil { flags |= 2 }

    let mapped = ok.map { $0 + 1 }
    if (try? mapped.get()) == 42 { flags |= 4 }

    let recovered = err.mapError { _ in RuntimeResultError.bad }
    if case .failure(.bad) = recovered { flags |= 8 }

    return flags
}

// ── Data semantics parity ──────────────────────────────────────────────────
@_cdecl("swift_data_probe_flags")
public func swift_data_probe_flags() -> Int32 {
    var data = Data([1, 2, 3, 4])
    var flags: Int32 = 0

    if data.count == 4 { flags |= 1 }
    if data.reduce(Int32(0), { $0 + Int32($1) }) == 10 { flags |= 2 }

    data.append(5)
    if data.count == 5 && data.last == 5 { flags |= 4 }

    let first = data.withUnsafeBytes { rawBuf -> UInt8 in
        rawBuf.bindMemory(to: UInt8.self).first ?? 0
    }
    if first == 1 { flags |= 8 }

    return flags
}

// ── UUID semantics parity ──────────────────────────────────────────────────
@_cdecl("swift_uuid_probe_flags")
public func swift_uuid_probe_flags() -> Int32 {
    let upper = "01234567-89AB-CDEF-0123-456789ABCDEF"
    let lower = "01234567-89ab-cdef-0123-456789abcdef"
    var flags: Int32 = 0

    if let uuid = UUID(uuidString: upper) {
        flags |= 1
        if uuid.uuidString.lowercased() == lower { flags |= 2 }
        let byteCount = withUnsafeBytes(of: uuid.uuid) { $0.count }
        if byteCount == 16 { flags |= 4 }
    }

    if UUID(uuidString: "not-a-uuid") == nil { flags |= 8 }
    return flags
}

// ── CharacterSet semantics parity ──────────────────────────────────────────
@_cdecl("swift_character_set_probe_flags")
public func swift_character_set_probe_flags() -> Int32 {
    let digits = CharacterSet.decimalDigits
    var flags: Int32 = 0

    if "5".unicodeScalars.allSatisfy({ digits.contains($0) }) { flags |= 1 }
    if "A".unicodeScalars.allSatisfy({ !digits.contains($0) }) { flags |= 2 }

    let vowels = CharacterSet(charactersIn: "aeiouAEIOU")
    if "e".unicodeScalars.allSatisfy({ vowels.contains($0) }) { flags |= 4 }
    if "z".unicodeScalars.allSatisfy({ !vowels.contains($0) }) { flags |= 8 }

    return flags
}

// ── URLComponents semantics parity ─────────────────────────────────────────
@_cdecl("swift_url_components_probe_flags")
public func swift_url_components_probe_flags() -> Int32 {
    let urlString = "https://example.com:8080/path/to?q=1&name=swift#frag"
    var flags: Int32 = 0

    if let comps = URLComponents(string: urlString) {
        if comps.scheme == "https" && comps.host == "example.com" { flags |= 1 }
        if comps.port == 8080 && comps.path == "/path/to" { flags |= 2 }
        let q = comps.queryItems?.first(where: { $0.name == "q" })?.value
        let name = comps.queryItems?.first(where: { $0.name == "name" })?.value
        if q == "1" && name == "swift" { flags |= 4 }
        if comps.fragment == "frag" { flags |= 8 }
    }

    return flags
}

// ── Calendar semantics parity ──────────────────────────────────────────────
@_cdecl("swift_calendar_probe_flags")
public func swift_calendar_probe_flags() -> Int32 {
    var calendar = Calendar(identifier: .gregorian)
    calendar.timeZone = TimeZone(secondsFromGMT: 0)!
    var flags: Int32 = 0

    let comps = DateComponents(year: 2024, month: 2, day: 29, hour: 12)
    if let date = calendar.date(from: comps) {
        flags |= 1
        let out = calendar.dateComponents([.year, .month, .day, .hour], from: date)
        if out.year == 2024 && out.month == 2 && out.day == 29 && out.hour == 12 { flags |= 2 }
        if calendar.component(.weekday, from: date) == 5 { flags |= 4 } // Thursday in Gregorian, Sunday=1
    }

    if calendar.range(of: .day, in: .month, for: calendar.date(from: DateComponents(year: 2024, month: 2, day: 1))!)?.count == 29 {
        flags |= 8
    }

    return flags
}

// ── IndexSet semantics parity ──────────────────────────────────────────────
@_cdecl("swift_index_set_probe_flags")
public func swift_index_set_probe_flags() -> Int32 {
    var set = IndexSet([1, 3, 5])
    var flags: Int32 = 0

    if set.contains(3) && !set.contains(2) { flags |= 1 }

    set.insert(integersIn: 7..<10)
    if set.contains(8) && set.count == 6 { flags |= 2 }

    set.remove(3)
    if !set.contains(3) && set.count == 5 { flags |= 4 }

    if set.first == 1 && set.last == 9 { flags |= 8 }
    return flags
}

// ── TimeZone semantics parity ──────────────────────────────────────────────
@_cdecl("swift_time_zone_probe_flags")
public func swift_time_zone_probe_flags() -> Int32 {
    var flags: Int32 = 0
    if let gmt = TimeZone(secondsFromGMT: 0) {
        if gmt.secondsFromGMT() == 0 { flags |= 1 }
        if gmt.identifier == "GMT" { flags |= 2 }
    }
    if let kolkata = TimeZone(identifier: "Asia/Kolkata") {
        if kolkata.secondsFromGMT() == 19800 { flags |= 4 }
        if kolkata.identifier == "Asia/Kolkata" { flags |= 8 }
    }
    return flags
}

// ── Measurement conversion parity ──────────────────────────────────────────
@_cdecl("swift_measurement_probe_flags")
public func swift_measurement_probe_flags() -> Int32 {
    var flags: Int32 = 0

    let meters = Measurement(value: 1500.0, unit: UnitLength.meters)
    let km = meters.converted(to: .kilometers)
    if abs(km.value - 1.5) < 0.000_001 { flags |= 1 }

    let celsius = Measurement(value: 100.0, unit: UnitTemperature.celsius)
    let fahrenheit = celsius.converted(to: .fahrenheit)
    if abs(fahrenheit.value - 212.0) < 0.000_001 { flags |= 2 }

    let grams = Measurement(value: 750.0, unit: UnitMass.grams)
    let kilograms = grams.converted(to: .kilograms)
    if abs(kilograms.value - 0.75) < 0.000_001 { flags |= 4 }

    let mps = Measurement(value: 36.0, unit: UnitSpeed.kilometersPerHour).converted(to: .metersPerSecond)
    if abs(mps.value - 10.0) < 0.000_01 { flags |= 8 }

    return flags
}

// ── DateFormatter/ISO8601 parity ───────────────────────────────────────────
@_cdecl("swift_date_formatter_probe_flags")
public func swift_date_formatter_probe_flags() -> Int32 {
    var flags: Int32 = 0

    guard let utc = TimeZone(secondsFromGMT: 0) else { return flags }
    var calendar = Calendar(identifier: .gregorian)
    calendar.timeZone = utc

    var components = DateComponents()
    components.calendar = calendar
    components.timeZone = utc
    components.year = 2024
    components.month = 3
    components.day = 1
    components.hour = 15
    components.minute = 4
    components.second = 5
    guard let date = calendar.date(from: components) else { return flags }

    let formatter = DateFormatter()
    formatter.calendar = calendar
    formatter.locale = Locale(identifier: "en_US_POSIX")
    formatter.timeZone = utc
    formatter.dateFormat = "yyyy-MM-dd HH:mm:ss"

    let rendered = formatter.string(from: date)
    if rendered == "2024-03-01 15:04:05" { flags |= 1 }

    if let reparsed = formatter.date(from: rendered),
       abs(reparsed.timeIntervalSince1970 - date.timeIntervalSince1970) < 0.5 {
        flags |= 2
    }

    let iso = ISO8601DateFormatter()
    iso.timeZone = utc
    iso.formatOptions = [.withInternetDateTime]

    let isoRendered = iso.string(from: date)
    if isoRendered == "2024-03-01T15:04:05Z" { flags |= 4 }

    if let isoReparsed = iso.date(from: isoRendered),
       abs(isoReparsed.timeIntervalSince1970 - date.timeIntervalSince1970) < 0.5 {
        flags |= 8
    }

    return flags
}

// ── Scanner semantics parity ───────────────────────────────────────────────
@_cdecl("swift_scanner_probe_flags")
public func swift_scanner_probe_flags() -> Int32 {
    var flags: Int32 = 0
    let scanner = Scanner(string: "42 3.25 token")
    scanner.charactersToBeSkipped = CharacterSet.whitespaces

    if let intValue = scanner.scanInt(), intValue == 42 { flags |= 1 }
    if let doubleValue = scanner.scanDouble(), abs(doubleValue - 3.25) < 0.000_000_1 { flags |= 2 }
    if let token = scanner.scanString("token"), token == "token" { flags |= 4 }
    if scanner.isAtEnd { flags |= 8 }
    return flags
}

// ── Locale semantics parity ────────────────────────────────────────────────
@_cdecl("swift_locale_probe_flags")
public func swift_locale_probe_flags() -> Int32 {
    var flags: Int32 = 0
    let posix = Locale(identifier: "en_US_POSIX")

    if posix.identifier == "en_US_POSIX" { flags |= 1 }
    if Locale(identifier: "EN_us").identifier == "en_US" { flags |= 2 }
    if posix.decimalSeparator == "." { flags |= 4 }

    if posix.language.languageCode?.identifier == "en" &&
       posix.region?.identifier == "US" {
        flags |= 8
    }

    return flags
}

// ── NumberFormatter semantics parity ───────────────────────────────────────
@_cdecl("swift_number_formatter_probe_flags")
public func swift_number_formatter_probe_flags() -> Int32 {
    var flags: Int32 = 0

    let decimal = NumberFormatter()
    decimal.locale = Locale(identifier: "en_US_POSIX")
    decimal.numberStyle = .decimal
    decimal.usesGroupingSeparator = false
    decimal.minimumFractionDigits = 2
    decimal.maximumFractionDigits = 2

    let rendered = decimal.string(from: NSNumber(value: 1234.5))
    if rendered == "1234.50" { flags |= 1 }

    if let parsed = decimal.number(from: "1234.50"), abs(parsed.doubleValue - 1234.5) < 0.000_001 {
        flags |= 2
    }

    let rounded = NumberFormatter()
    rounded.locale = Locale(identifier: "en_US_POSIX")
    rounded.numberStyle = .decimal
    rounded.maximumFractionDigits = 0
    rounded.roundingMode = .halfUp
    if rounded.string(from: NSNumber(value: 2.6)) == "3" { flags |= 4 }

    if decimal.number(from: "not_a_number") == nil { flags |= 8 }
    return flags
}

// ── URL semantics parity ───────────────────────────────────────────────────
@_cdecl("swift_url_probe_flags")
public func swift_url_probe_flags() -> Int32 {
    var flags: Int32 = 0

    if let url = URL(string: "https://example.com/a%20b?q=1#frag") {
        if url.scheme == "https" && url.host == "example.com" && url.path == "/a b" { flags |= 1 }
        if url.query == "q=1" && url.fragment == "frag" { flags |= 2 }
        if url.absoluteString == "https://example.com/a%20b?q=1#frag" { flags |= 4 }
    }

    if let base = URL(string: "https://example.com/a/b/"),
       let resolved = URL(string: "../c", relativeTo: base)?.absoluteURL.absoluteString,
       resolved == "https://example.com/a/c" {
        flags |= 8
    }

    return flags
}

// ── Decimal semantics parity ───────────────────────────────────────────────
@_cdecl("swift_decimal_probe_flags")
public func swift_decimal_probe_flags() -> Int32 {
    var flags: Int32 = 0
    let posix = Locale(identifier: "en_US_POSIX")

    guard var left = Decimal(string: "1.20", locale: posix),
          var right = Decimal(string: "2.30", locale: posix) else {
        return flags
    }

    var sum = Decimal()
    NSDecimalAdd(&sum, &left, &right, .plain)
    if sum == Decimal(string: "3.50", locale: posix) { flags |= 1 }

    var product = Decimal()
    NSDecimalMultiply(&product, &left, &right, .plain)
    if product == Decimal(string: "2.760", locale: posix) { flags |= 2 }

    var input = Decimal(string: "2.676", locale: posix)!
    var rounded = Decimal()
    NSDecimalRound(&rounded, &input, 2, .plain)
    if rounded == Decimal(string: "2.68", locale: posix) { flags |= 4 }

    if Decimal(string: "not_a_number", locale: posix) == nil { flags |= 8 }

    return flags
}

// ── URLRequest semantics parity ────────────────────────────────────────────
@_cdecl("swift_url_request_probe_flags")
public func swift_url_request_probe_flags() -> Int32 {
    var flags: Int32 = 0

    guard let url = URL(string: "https://example.com/api") else { return flags }
    var request = URLRequest(url: url)
    request.httpMethod = "POST"
    request.setValue("application/json", forHTTPHeaderField: "Content-Type")
    request.timeoutInterval = 12.5

    let body = Data("{\"x\":1}".utf8)
    request.httpBody = body

    if request.url?.absoluteString == "https://example.com/api" && request.httpMethod == "POST" { flags |= 1 }
    if request.value(forHTTPHeaderField: "content-type") == "application/json" { flags |= 2 }
    if abs(request.timeoutInterval - 12.5) < 0.000_001 { flags |= 4 }
    if request.httpBody == body { flags |= 8 }

    return flags
}

// ── Data base64 semantics parity ───────────────────────────────────────────
@_cdecl("swift_data_base64_probe_flags")
public func swift_data_base64_probe_flags() -> Int32 {
    var flags: Int32 = 0
    let source = Data([0x53, 0x77, 0x69, 0x66, 0x74]) // "Swift"

    let encoded = source.base64EncodedString()
    if encoded == "U3dpZnQ=" { flags |= 1 }

    if let decoded = Data(base64Encoded: encoded), decoded == source { flags |= 2 }

    if let decodedIgnore = Data(base64Encoded: "U3dp\nZnQ=", options: .ignoreUnknownCharacters), decodedIgnore == source {
        flags |= 4
    }

    if Data(base64Encoded: "not-base64") == nil { flags |= 8 }

    return flags
}

@_cdecl("swift_http_url_response_probe_flags")
public func swift_http_url_response_probe_flags() -> Int32 {
    var flags: Int32 = 0
    let testURL = URL(string: "https://example.com/path")!
    let response = HTTPURLResponse(url: testURL, statusCode: 200, httpVersion: "HTTP/1.1", headerFields: ["Content-Type": "application/json", "X-Custom": "test-value"])!

    if response.statusCode == 200 { flags |= 1 }

    if let header = response.value(forHTTPHeaderField: "X-Custom"), header == "test-value" { flags |= 2 }

    if response.url == testURL { flags |= 4 }

    if response.value(forHTTPHeaderField: "Content-Type") == "application/json" { flags |= 8 }

    return flags
}

@_cdecl("swift_json_encoder_probe_flags")
public func swift_json_encoder_probe_flags() -> Int32 {
    var flags: Int32 = 0

    struct TestPayload: Codable {
        let id: Int
        let name: String
        let nested: NestedData
        let optional: String?
    }

    struct NestedData: Codable {
        let value: Double
    }

    let encoder = JSONEncoder()
    let decoder = JSONDecoder()

    let original = TestPayload(id: 42, name: "test", nested: NestedData(value: 3.14), optional: nil)

    do {
        let json = try encoder.encode(original)
        if !json.isEmpty { flags |= 1 }

        let decoded = try decoder.decode(TestPayload.self, from: json)
        if decoded.id == original.id && decoded.name == original.name { flags |= 2 }

        if abs(decoded.nested.value - original.nested.value) < 0.001 { flags |= 4 }

        if decoded.optional == nil { flags |= 8 }
    } catch { }

    return flags
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

@_cdecl("swift_layout_external_resilient_size")
public func swift_layout_external_resilient_size() -> Int32 {
    external_resilient_size()
}

@_cdecl("swift_layout_external_resilient_stride")
public func swift_layout_external_resilient_stride() -> Int32 {
    external_resilient_stride()
}

@_cdecl("swift_layout_external_resilient_alignment")
public func swift_layout_external_resilient_alignment() -> Int32 {
    external_resilient_alignment()
}

@_cdecl("swift_layout_external_resilient_b_offset")
public func swift_layout_external_resilient_b_offset() -> Int32 {
    external_resilient_b_offset()
}

@_cdecl("swift_layout_external_resilient_sample_b")
public func swift_layout_external_resilient_sample_b() -> Int32 {
    external_resilient_get_b(11, 2222) == 2222 ? 1 : 0
}

@_cdecl("swift_external_existential_value_current")
public func swift_external_existential_value_current() -> Int32 {
    external_existential_value_current()
}

@_cdecl("swift_external_existential_ref_current")
public func swift_external_existential_ref_current() -> Int32 {
    external_existential_ref_current()
}

@_cdecl("swift_external_class_existential_current")
public func swift_external_class_existential_current() -> Int32 {
    external_class_existential_current()
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
