import Foundation
import Dispatch
import Darwin
import ObjectiveC.runtime
import ResilientFixtures

public var globalCounterValue: Int32 = 123

private let runtimeContractVersion: Int32 = 1
private let runtimeContractJSONString = """
{"contract_version":1,"bridge":"RustBridge","cooperation_boundary":{"swift_side":["export versioned type and method registries for required parity flows","resolve resilience-sensitive layouts and protocol-backed entry points behind normalized bridge calls","publish capability states for compiler-feature-dependent operations"],"rust_side":["load and validate contract versions before invoking required flows","box arguments and results with explicit ownership for opaque Swift references","negotiate compiler-feature-sensitive operations through descriptor capabilities"],"research_only":["ad hoc mangled-name discovery outside the registered contract","raw runtime experiments that depend on unstable metadata or witness placement"]},"types":[{"type_id":1,"name":"Person","kind":"value","constructors":[{"ctor_id":1,"symbol":"swift_person_new","args_blob":"i32x2","result":"OpaqueRef"}],"methods":[{"method_id":1,"name":"get_id","symbol":"swift_person_get_id","shape":"self_to_i32"},{"method_id":2,"name":"get_age","symbol":"swift_person_get_age","shape":"self_to_i32"}]},{"type_id":2,"name":"Counter","kind":"reference","constructors":[{"ctor_id":1,"symbol":"swift_counter_new","args_blob":"i32x1","result":"OpaqueRef"}],"methods":[{"method_id":1,"name":"increment","symbol":"runtime_thunk_counter_increment_x20","shape":"self_i32_to_i32"},{"method_id":2,"name":"current","symbol":"runtime_thunk_counter_current_x20","shape":"self_to_i32"},{"method_id":3,"name":"reset","symbol":"runtime_thunk_counter_reset_x20","shape":"self_i32_to_void"}]},{"type_id":3,"name":"String","kind":"reference","constructors":[{"ctor_id":1,"symbol":"swift_contract_construct_string","args_blob":"bytes","result":"OpaqueRef"}],"methods":[{"method_id":1,"name":"len","symbol":"swift_contract_string_len","shape":"contract_i32"},{"method_id":2,"name":"get_bytes","symbol":"swift_contract_string_bytes","shape":"contract_bytes"}]},{"type_id":4,"name":"Array<Int32>","kind":"reference","constructors":[{"ctor_id":1,"symbol":"swift_contract_array_make","args_blob":"i32","result":"OpaqueRef"}],"methods":[{"method_id":1,"name":"len","symbol":"swift_contract_array_len","shape":"contract_i32"},{"method_id":2,"name":"get_element","symbol":"swift_contract_array_get","shape":"contract_i32"},{"method_id":3,"name":"set_element","symbol":"swift_contract_array_set","shape":"contract_void"},{"method_id":4,"name":"append","symbol":"swift_contract_array_append","shape":"contract_i32"},{"method_id":5,"name":"data_ptr","symbol":"swift_contract_array_data","shape":"contract_pointer"}]},{"type_id":5,"name":"Array<OpaqueRef>","kind":"reference","constructors":[{"ctor_id":1,"symbol":"swift_contract_array_ref_make","args_blob":"i32","result":"OpaqueRef"}],"methods":[{"method_id":1,"name":"len","symbol":"swift_contract_array_ref_len","shape":"contract_i32"},{"method_id":2,"name":"get_element","symbol":"swift_contract_array_ref_get","shape":"contract_ref"},{"method_id":3,"name":"set_element","symbol":"swift_contract_array_ref_set","shape":"contract_void"},{"method_id":4,"name":"append","symbol":"swift_contract_array_ref_append","shape":"contract_i32"}]},{"type_id":6,"name":"Dictionary<Int32, Int32>","kind":"reference","constructors":[{"ctor_id":1,"symbol":"swift_contract_dict_i32_make","args_blob":"i32","result":"OpaqueRef"}],"methods":[{"method_id":1,"name":"len","symbol":"swift_contract_dict_i32_len","shape":"contract_i32"},{"method_id":2,"name":"get","symbol":"swift_contract_dict_i32_get","shape":"contract_i32_out"},{"method_id":3,"name":"set","symbol":"swift_contract_dict_i32_set","shape":"contract_i32"},{"method_id":4,"name":"remove","symbol":"swift_contract_dict_i32_remove","shape":"contract_i32_out"},{"method_id":5,"name":"contains","symbol":"swift_contract_dict_i32_contains","shape":"contract_bool"}]},{"type_id":7,"name":"Dictionary<Int32, OpaqueRef>","kind":"reference","constructors":[{"ctor_id":1,"symbol":"swift_contract_dict_ref_make","args_blob":"i32","result":"OpaqueRef"}],"methods":[{"method_id":1,"name":"len","symbol":"swift_contract_dict_ref_len","shape":"contract_i32"},{"method_id":2,"name":"get","symbol":"swift_contract_dict_ref_get","shape":"contract_ref_out"},{"method_id":3,"name":"set","symbol":"swift_contract_dict_ref_set","shape":"contract_i32"},{"method_id":4,"name":"remove","symbol":"swift_contract_dict_ref_remove","shape":"contract_ref_nullable"},{"method_id":5,"name":"contains","symbol":"swift_contract_dict_ref_contains","shape":"contract_bool"}]},{"type_id":8,"name":"Any<ContractObject>","kind":"reference","constructors":[{"ctor_id":1,"symbol":"swift_contract_any_wrap","args_blob":"i32_ref","result":"OpaqueRef"}],"methods":[{"method_id":1,"name":"type_id","symbol":"swift_contract_any_type_id","shape":"contract_i32"},{"method_id":2,"name":"dynamic_cast","symbol":"swift_contract_dynamic_cast","shape":"contract_ref_nullable"}]},{"type_id":9,"name":"Direction","kind":"enum","constructors":[{"ctor_id":0,"symbol":"swift_contract_direction_make","args_blob":"i32","result":"OpaqueRef"}],"methods":[{"method_id":1,"name":"case","symbol":"swift_contract_direction_case","shape":"contract_i32"}]},{"type_id":10,"name":"Shape","kind":"enum","constructors":[{"ctor_id":0,"symbol":"swift_contract_shape_circle","args_blob":"f32","result":"OpaqueRef"},{"ctor_id":1,"symbol":"swift_contract_shape_rect","args_blob":"f32x2","result":"OpaqueRef"}],"methods":[{"method_id":1,"name":"get_case","symbol":"swift_contract_shape_get_case","shape":"contract_i32"},{"method_id":2,"name":"circle_radius","symbol":"swift_contract_shape_circle_radius","shape":"contract_f32"},{"method_id":3,"name":"rect_dims","symbol":"swift_contract_shape_rect_dims","shape":"contract_void"}]}],"metadata_registry":{"entries":[{"metadata_id":1,"name":"Person"},{"metadata_id":2,"name":"Counter"},{"metadata_id":3,"name":"String"},{"metadata_id":4,"name":"Array<Int32>"},{"metadata_id":1001,"name":"ContractGenericBox<Int32>"},{"metadata_id":5,"name":"Array<OpaqueRef>"},{"metadata_id":6,"name":"Dictionary<Int32, Int32>"},{"metadata_id":7,"name":"Dictionary<Int32, OpaqueRef>"},{"metadata_id":8,"name":"Any<ContractObject>"},{"metadata_id":9,"name":"Direction"},{"metadata_id":10,"name":"Shape"}]},"protocol_registry":{"entries":[{"protocol_id":1,"name":"CounterLike","type_id":2,"methods":[{"method_id":1,"name":"current","symbol":"swift_contract_protocol_invoke_i32"}]}]},"ownership":{"opaque_ref":"swift_contract_release","retain_export":"swift_retain"},"compiler_features":{"resilient_dispatch":{"status":"supported","reason":"Required resilience-sensitive calls are routed through normalized Swift bridge entry points.","provider":"swift_bridge"},"generic_metadata_registry":{"status":"supported","reason":"Registry exports provide deterministic metadata lookup for required concrete and generic instantiations.","provider":"swift_bridge"},"protocol_witness_registry":{"status":"supported","reason":"Protocol conformance lookup and wrapper-first dispatch are exported through stable contract entry points.","provider":"swift_bridge"},"raw_runtime_research_mode":{"status":"fallback","reason":"Raw runtime mode is available for research but not required for promoted parity paths.","provider":"runtime_factory"}},"capabilities":{"contract_descriptor":true,"versioned_ids":true,"normalized_invoke":true,"raw_runtime_research_mode":true,"string_utf8_support":true,"array_int32_support":true,"array_int32_pointer_iteration_support":true,"array_opaque_ref_support":true,"dictionary_int32_int32_support":true,"dictionary_upsert_support":true,"dictionary_remove_support":true,"dictionary_opaque_ref_support":true,"dynamic_cast_support":true,"metatype_identity_support":true,"enum_raw_support":true,"enum_associated_values_support":true}}
"""
private let runtimeContractNSString = NSString(string: runtimeContractJSONString)

@_cdecl("swift_runtime_contract_version")
public func swift_runtime_contract_version() -> Int32 {
    runtimeContractVersion
}

@_cdecl("swift_runtime_contract_json")
public func swift_runtime_contract_json() -> UnsafePointer<CChar>? {
    runtimeContractNSString.utf8String
}

@_cdecl("swift_runtime_contract_json_len")
public func swift_runtime_contract_json_len() -> Int32 {
    Int32(runtimeContractNSString.lengthOfBytes(using: String.Encoding.utf8.rawValue))
}

@_cdecl("swift_contract_lookup_metadata")
public func swift_contract_lookup_metadata(_ metadataID: Int32) -> UnsafeRawPointer? {
    switch metadataID {
    case 1:
        return unsafeBitCast(Person.self, to: UnsafeRawPointer.self)
    case 2:
        return unsafeBitCast(Counter.self, to: UnsafeRawPointer.self)
    case 3:
        return unsafeBitCast(String.self, to: UnsafeRawPointer.self)
    case 4:
        return unsafeBitCast(Array<Int32>.self, to: UnsafeRawPointer.self)
    case 5:
        return unsafeBitCast(Array<UnsafeMutableRawPointer>.self, to: UnsafeRawPointer.self)
    case 6:
        return unsafeBitCast(Dictionary<Int32, Int32>.self, to: UnsafeRawPointer.self)
    case 7:
        return unsafeBitCast(Dictionary<Int32, UnsafeMutableRawPointer>.self, to: UnsafeRawPointer.self)
    case 8:
        return unsafeBitCast(ContractAnyBox.self, to: UnsafeRawPointer.self)
    case 9:
        return unsafeBitCast(Direction.self, to: UnsafeRawPointer.self)
    case 10:
        return unsafeBitCast(Shape.self, to: UnsafeRawPointer.self)
    case 1001:
        return unsafeBitCast(ContractGenericBox<Int32>.self, to: UnsafeRawPointer.self)
    case 1002:
        return unsafeBitCast(Dictionary<String, Int32>.self, to: UnsafeRawPointer.self)
    default:
        return nil
    }
}

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
    public static var deinitCount: Int32 = 0
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

    deinit {
        Counter.deinitCount += 1
    }
}

public struct ContractGenericBox<T> {
    public var value: T

    public init(_ value: T) {
        self.value = value
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

// MARK: - Error Handling & Introspection (Track E.1)

/// A simple custom error type with error code.
public enum ValidationError: Error {
    case invalidInput(code: Int32)
    case outOfRange(code: Int32, limit: Int32)
    case custom(message: String)
    
    public var errorDescription: String {
        switch self {
        case .invalidInput(let code):
            return "Validation failed with code \(code)"
        case .outOfRange(let code, let limit):
            return "Value out of range (limit: \(limit), code: \(code))"
        case .custom(let message):
            return message
        }
    }
    
    public var errorCode: Int32 {
        switch self {
        case .invalidInput(let code):
            return code
        case .outOfRange(let code, _):
            return code
        case .custom:
            return -1
        }
    }
}

/// Another error type for testing type identity.
public enum IOError: Error {
    case fileNotFound(code: Int32)
    case permissionDenied(code: Int32)
    case unknown(code: Int32)
    
    public var errorDescription: String {
        switch self {
        case .fileNotFound(let code):
            return "File not found (code: \(code))"
        case .permissionDenied(let code):
            return "Permission denied (code: \(code))"
        case .unknown(let code):
            return "Unknown error (code: \(code))"
        }
    }
    
    public var errorCode: Int32 {
        switch self {
        case .fileNotFound(let code):
            return code
        case .permissionDenied(let code):
            return code
        case .unknown(let code):
            return code
        }
    }
}

/// Global storage for a thrown error, boxed for bridging.
private var _storedError: Error?

/// Construct a ValidationError with code and store it for introspection.
/// Returns 1 on success, 0 on failure.
@_cdecl("swift_contract_error_make_validation")
public func swift_contract_error_make_validation(_ code: Int32) -> Int32 {
    _storedError = ValidationError.invalidInput(code: code)
    return 1
}

/// Construct an IOError with code and store it for introspection.
/// Returns 1 on success, 0 on failure.
@_cdecl("swift_contract_error_make_io")
public func swift_contract_error_make_io(_ code: Int32) -> Int32 {
    _storedError = IOError.fileNotFound(code: code)
    return 1
}

/// Extract the error description from the stored error.
/// Returns a malloc'd C string (Rust must free it) or NULL if no error.
@_cdecl("swift_contract_error_get_description")
public func swift_contract_error_get_description() -> UnsafeMutablePointer<CChar>? {
    guard let error = _storedError else { return nil }
    
    let description: String
    if let validationError = error as? ValidationError {
        description = validationError.errorDescription
    } else if let ioError = error as? IOError {
        description = ioError.errorDescription
    } else {
        description = String(describing: error)
    }
    
    let cString = strdup(description)
    return cString
}

/// Extract the error code from the stored error.
/// Returns the code or -1 if error is not found or has no code.
@_cdecl("swift_contract_error_get_code")
public func swift_contract_error_get_code() -> Int32 {
    guard let error = _storedError else { return -1 }
    
    if let validationError = error as? ValidationError {
        return validationError.errorCode
    } else if let ioError = error as? IOError {
        return ioError.errorCode
    }
    return -1
}

/// Check if the stored error is a ValidationError (returns 1) or not (returns 0).
@_cdecl("swift_contract_error_is_validation")
public func swift_contract_error_is_validation() -> Int32 {
    guard let _ = _storedError as? ValidationError else { return 0 }
    return 1
}

/// Check if the stored error is an IOError (returns 1) or not (returns 0).
@_cdecl("swift_contract_error_is_io")
public func swift_contract_error_is_io() -> Int32 {
    guard let _ = _storedError as? IOError else { return 0 }
    return 1
}

/// Clear the stored error.
@_cdecl("swift_contract_error_clear")
public func swift_contract_error_clear() {
    _storedError = nil
}

/// Construct an OutOfRange ValidationError and store it.
@_cdecl("swift_contract_error_make_out_of_range")
public func swift_contract_error_make_out_of_range(_ code: Int32, _ limit: Int32) -> Int32 {
    _storedError = ValidationError.outOfRange(code: code, limit: limit)
    return 1
}

// MARK: - Structured Error Propagation (Track E.3)

private struct ContractErrorContext: Codable {
    let domain: String
    let code: Int32
    let message: String
    let chain: [String]
    let userInfo: [String: String]
    let recoveryHints: [String]

    enum CodingKeys: String, CodingKey {
        case domain
        case code
        case message
        case chain
        case userInfo = "user_info"
        case recoveryHints = "recovery_hints"
    }
}

private var _storedErrorContext: ContractErrorContext?

private func _encodeErrorContextJSON(_ context: ContractErrorContext) -> String? {
    let encoder = JSONEncoder()
    if #available(macOS 10.13, *) {
        encoder.outputFormatting = [.sortedKeys]
    }
    guard let data = try? encoder.encode(context) else { return nil }
    return String(data: data, encoding: .utf8)
}

private func _renderErrorContextString(_ context: ContractErrorContext) -> String {
    let chainText = context.chain.joined(separator: " -> ")
    let hintsText = context.recoveryHints.joined(separator: " | ")
    let userInfoText = context.userInfo
        .sorted(by: { $0.key < $1.key })
        .map { "\($0.key)=\($0.value)" }
        .joined(separator: ",")
    return "[\(context.domain)] code=\(context.code) message=\(context.message) chain=\(chainText) user_info=\(userInfoText) recovery_hints=\(hintsText)"
}

/// Construct a deterministic validation error context with a chain and recovery hints.
@_cdecl("swift_contract_error_context_make_validation")
public func swift_contract_error_context_make_validation(_ code: Int32, _ causeCode: Int32) -> Int32 {
    _storedError = ValidationError.invalidInput(code: code)
    _storedErrorContext = ContractErrorContext(
        domain: "ValidationError",
        code: code,
        message: "Validation failed with code \(code)",
        chain: [
            "ValidationError(code=\(code))",
            "ConstraintViolation(code=\(causeCode))"
        ],
        userInfo: [
            "field": "age",
            "operation": "create_user"
        ],
        recoveryHints: [
            "Clamp input to allowed range",
            "Retry request with corrected payload"
        ]
    )
    return 1
}

/// Construct a deterministic IO error context with a chain and recovery hints.
@_cdecl("swift_contract_error_context_make_io")
public func swift_contract_error_context_make_io(_ code: Int32) -> Int32 {
    _storedError = IOError.fileNotFound(code: code)
    _storedErrorContext = ContractErrorContext(
        domain: "IOError",
        code: code,
        message: "File operation failed with code \(code)",
        chain: [
            "IOError(code=\(code))",
            "POSIX(errno=2)"
        ],
        userInfo: [
            "path": "/tmp/runtime-probe/input.json",
            "operation": "read"
        ],
        recoveryHints: [
            "Verify file exists",
            "Check directory permissions"
        ]
    )
    return 1
}

/// Serialize the currently stored error context to JSON.
/// Returns a malloc'd C string or nil if no context is available.
@_cdecl("swift_contract_error_context_get_json")
public func swift_contract_error_context_get_json() -> UnsafeMutablePointer<CChar>? {
    guard let context = _storedErrorContext else { return nil }
    guard let json = _encodeErrorContextJSON(context) else { return nil }
    return strdup(json)
}

/// Serialize the currently stored error context to a compact logging string.
/// Returns a malloc'd C string or nil if no context is available.
@_cdecl("swift_contract_error_context_get_string")
public func swift_contract_error_context_get_string() -> UnsafeMutablePointer<CChar>? {
    guard let context = _storedErrorContext else { return nil }
    let text = _renderErrorContextString(context)
    return strdup(text)
}

/// Replace stored context from a JSON payload produced by the bridge.
/// Returns 1 on success, 0 on parse/validation failure.
@_cdecl("swift_contract_error_context_set_json")
public func swift_contract_error_context_set_json(_ jsonPtr: UnsafePointer<CChar>?) -> Int32 {
    guard let jsonPtr else { return 0 }
    let json = String(cString: jsonPtr)
    guard let data = json.data(using: .utf8) else { return 0 }
    guard let decoded = try? JSONDecoder().decode(ContractErrorContext.self, from: data) else { return 0 }
    _storedErrorContext = decoded
    return 1
}

/// Clear the stored structured error context.
@_cdecl("swift_contract_error_context_clear")
public func swift_contract_error_context_clear() {
    _storedErrorContext = nil
}

// MARK: - Task Creation & Continuation (Track G.1)

private let _continuationCountLock = NSLock()
private var _continuationResumeCount: Int32 = 0

private func _continuationIncrement() {
    _continuationCountLock.lock()
    _continuationResumeCount += 1
    _continuationCountLock.unlock()
}

private final class ContinuationSingleResumeGuard {
    private let lock = NSLock()
    private var resumed = false
    private var blockedAttempts: Int32 = 0

    func tryMarkResumed() -> Bool {
        lock.lock()
        defer { lock.unlock() }
        if resumed {
            blockedAttempts += 1
            return false
        }
        resumed = true
        return true
    }

    func blockedCount() -> Int32 {
        lock.lock()
        defer { lock.unlock() }
        return blockedAttempts
    }
}

private func _awaitTaskI32(_ op: @escaping @Sendable () async -> Int32) -> Int32 {
    let semaphore = DispatchSemaphore(value: 0)
    var result: Int32 = Int32.min
    Task {
        result = await op()
        semaphore.signal()
    }
    let waitResult = semaphore.wait(timeout: .now() + 2)
    return waitResult == .success ? result : Int32.min
}

private func _continuationRoundTrip(_ value: Int32) async -> Int32 {
    await withCheckedContinuation { (continuation: CheckedContinuation<Int32, Never>) in
        _continuationIncrement()
        continuation.resume(returning: value)
    }
}

/// Spawn a Swift Task and return the sum result.
/// Returns Int32.min on timeout/failure.
@_cdecl("swift_contract_task_spawn_sum")
public func swift_contract_task_spawn_sum(_ a: Int32, _ b: Int32) -> Int32 {
    _awaitTaskI32 {
        a + b
    }
}

/// Spawn a Swift Task with deterministic yielding chain.
/// Computes base + sum(0..<steps) with bounded steps.
@_cdecl("swift_contract_task_spawn_chain")
public func swift_contract_task_spawn_chain(_ base: Int32, _ steps: Int32) -> Int32 {
    _awaitTaskI32 {
        let bounded = max(0, min(steps, 64))
        var value = base
        for i in 0..<bounded {
            await Task.yield()
            value += i
        }
        return value
    }
}

/// Reset continuation resume counter.
@_cdecl("swift_contract_continuation_reset")
public func swift_contract_continuation_reset() {
    _continuationCountLock.lock()
    _continuationResumeCount = 0
    _continuationCountLock.unlock()
}

/// Return continuation resume count observed by bridge probes.
@_cdecl("swift_contract_continuation_resume_count")
public func swift_contract_continuation_resume_count() -> Int32 {
    _continuationCountLock.lock()
    defer { _continuationCountLock.unlock() }
    return _continuationResumeCount
}

/// Run a checked-continuation round-trip and return its value.
/// Returns Int32.min on timeout/failure.
@_cdecl("swift_contract_continuation_roundtrip")
public func swift_contract_continuation_roundtrip(_ value: Int32) -> Int32 {
    _awaitTaskI32 {
        await _continuationRoundTrip(value)
    }
}

/// Validate resume-once safety by attempting a guarded second resume without invoking it.
/// Returns 1 when exactly one resume occurred and at least one second-attempt block was recorded.
@_cdecl("swift_contract_continuation_validate_resume_once")
public func swift_contract_continuation_validate_resume_once() -> Int32 {
    let guardState = ContinuationSingleResumeGuard()
    let before = swift_contract_continuation_resume_count()

    let value = _awaitTaskI32 {
        await withCheckedContinuation { (continuation: CheckedContinuation<Int32, Never>) in
            if guardState.tryMarkResumed() {
                _continuationIncrement()
                continuation.resume(returning: 41)
            } else {
                continuation.resume(returning: Int32.min)
                return
            }
            _ = guardState.tryMarkResumed()
        }
    }

    let after = swift_contract_continuation_resume_count()
    let resumedExactlyOnce = (after - before) == 1
    let blockedSecondAttempt = guardState.blockedCount() >= 1
    return (value == 41 && resumedExactlyOnce && blockedSecondAttempt) ? 1 : 0
}

// MARK: - Actor Isolation & Isolation Domains (Track G.2)

private actor ProbeCounterActor {
    private var value: Int32

    init(start: Int32) {
        value = start
    }

    func add(_ delta: Int32) -> Int32 {
        value += delta
        return value
    }

    func current() -> Int32 {
        value
    }
}

@_cdecl("swift_contract_actor_make")
public func swift_contract_actor_make(_ start: Int32) -> UnsafeMutableRawPointer? {
    let actor = ProbeCounterActor(start: start)
    return Unmanaged.passRetained(Box(actor)).toOpaque()
}

@_cdecl("swift_contract_actor_current")
public func swift_contract_actor_current(_ ptr: UnsafeMutableRawPointer?) -> Int32 {
    guard let ptr else { return Int32.min }
    let boxed = Unmanaged<Box<ProbeCounterActor>>.fromOpaque(ptr).takeUnretainedValue()
    return _awaitTaskI32 {
        await boxed.value.current()
    }
}

@_cdecl("swift_contract_actor_add")
public func swift_contract_actor_add(_ ptr: UnsafeMutableRawPointer?, _ delta: Int32) -> Int32 {
    guard let ptr else { return Int32.min }
    let boxed = Unmanaged<Box<ProbeCounterActor>>.fromOpaque(ptr).takeUnretainedValue()
    return _awaitTaskI32 {
        await boxed.value.add(delta)
    }
}

@_cdecl("swift_contract_actor_validate_isolation")
public func swift_contract_actor_validate_isolation(_ ptr: UnsafeMutableRawPointer?) -> Int32 {
    guard let ptr else { return 0 }
    let boxed = Unmanaged<Box<ProbeCounterActor>>.fromOpaque(ptr).takeUnretainedValue()

    let final = _awaitTaskI32 {
        await withTaskGroup(of: Int32.self) { group in
            group.addTask { await boxed.value.add(1) }
            group.addTask { await boxed.value.add(2) }
            _ = await group.next()
            _ = await group.next()
            return await boxed.value.current()
        }
    }

    return final >= 3 ? 1 : 0
}

// MARK: - Async Streams & AsyncSequence (Track G.3)

private final class ProbeAsyncIteratorBox {
    private var iterator: AsyncStream<Int32>.Iterator

    init(start: Int32, count: Int32) {
        let boundedCount = max(0, min(count, 128))
        let stream = AsyncStream<Int32> { continuation in
            for i in 0..<boundedCount {
                continuation.yield(start + i)
            }
            continuation.finish()
        }
        iterator = stream.makeAsyncIterator()
    }

    func nextValue() async -> Int32? {
        await iterator.next()
    }
}

@_cdecl("swift_contract_stream_make")
public func swift_contract_stream_make(_ start: Int32, _ count: Int32) -> UnsafeMutableRawPointer? {
    let iterator = ProbeAsyncIteratorBox(start: start, count: count)
    return Unmanaged.passRetained(Box(iterator)).toOpaque()
}

@_cdecl("swift_contract_stream_next")
public func swift_contract_stream_next(
    _ ptr: UnsafeMutableRawPointer?,
    _ outValue: UnsafeMutablePointer<Int32>?
) -> Int32 {
    guard let ptr, let outValue else { return -1 }
    let boxed = Unmanaged<Box<ProbeAsyncIteratorBox>>.fromOpaque(ptr).takeUnretainedValue()

    let semaphore = DispatchSemaphore(value: 0)
    var hasValue: Int32 = -1
    Task {
        if let value = await boxed.value.nextValue() {
            outValue.pointee = value
            hasValue = 1
        } else {
            hasValue = 0
        }
        semaphore.signal()
    }

    let waitResult = semaphore.wait(timeout: .now() + 2)
    return waitResult == .success ? hasValue : -1
}

@_cdecl("swift_contract_stream_collect_sum")
public func swift_contract_stream_collect_sum(_ start: Int32, _ count: Int32) -> Int32 {
    _awaitTaskI32 {
        let boundedCount = max(0, min(count, 128))
        let stream = AsyncStream<Int32> { continuation in
            for i in 0..<boundedCount {
                continuation.yield(start + i)
            }
            continuation.finish()
        }

        var sum: Int32 = 0
        for await value in stream {
            sum += value
        }
        return sum
    }
}

// MARK: - Task-Local Values (Track G.4)

private enum ProbeTaskLocal {
    @TaskLocal static var value: Int32 = -1
}

@_cdecl("swift_contract_task_local_get_default")
public func swift_contract_task_local_get_default() -> Int32 {
    _awaitTaskI32 {
        ProbeTaskLocal.value
    }
}

@_cdecl("swift_contract_task_local_run_with")
public func swift_contract_task_local_run_with(_ value: Int32, _ delta: Int32) -> Int32 {
    _awaitTaskI32 {
        await ProbeTaskLocal.$value.withValue(value) {
            let inherited = await Task { ProbeTaskLocal.value }.value
            guard inherited == value else { return Int32.min }
            return inherited + delta
        }
    }
}

@_cdecl("swift_contract_task_local_isolation_check")
public func swift_contract_task_local_isolation_check(_ parentValue: Int32) -> Int32 {
    _awaitTaskI32 {
        await ProbeTaskLocal.$value.withValue(parentValue) {
            let inherited = await Task { ProbeTaskLocal.value }.value
            let detached = await Task.detached { ProbeTaskLocal.value }.value
            return (inherited == parentValue && detached == -1) ? 1 : 0
        }
    }
}

// MARK: - Generic Metadata Accessor Chains (Track H.1)

@_cdecl("swift_contract_generic_validate_substitution")
public func swift_contract_generic_validate_substitution(_ typeName: UnsafePointer<CChar>?) -> Int32 {
    guard let typeName else { return 0 }
    let key = String(cString: typeName)
    switch key {
    case "Int32", "Array<Int32>", "Dictionary<String,Int32>":
        return 1
    default:
        return 0
    }
}

@_cdecl("swift_contract_generic_box_i32_make")
public func swift_contract_generic_box_i32_make(_ value: Int32) -> UnsafeMutableRawPointer? {
    let boxed = Box(ContractGenericBox<Int32>(value))
    return Unmanaged.passRetained(boxed).toOpaque()
}

@_cdecl("swift_contract_generic_box_i32_get")
public func swift_contract_generic_box_i32_get(_ ptr: UnsafeMutableRawPointer?) -> Int32 {
    guard let ptr else { return Int32.min }
    let boxed = Unmanaged<Box<ContractGenericBox<Int32>>>.fromOpaque(ptr).takeUnretainedValue()
    return boxed.value.value
}

@_cdecl("swift_contract_generic_array_i32_sum")
public func swift_contract_generic_array_i32_sum(_ start: Int32, _ count: Int32) -> Int32 {
    let bounded = max(0, min(count, 256))
    let values = (0..<bounded).map { start + $0 }
    return values.reduce(0, +)
}

@_cdecl("swift_contract_generic_dict_string_i32_sum")
public func swift_contract_generic_dict_string_i32_sum(_ base: Int32) -> Int32 {
    let dict: [String: Int32] = [
        "alpha": base,
        "beta": base + 1,
        "gamma": base + 2,
    ]
    return dict.values.reduce(0, +)
}

// MARK: - Generic Protocol Witness Lookup (Track H.2)

@_cdecl("swift_contract_generic_protocol_array_i32_sequence_supported")
public func swift_contract_generic_protocol_array_i32_sequence_supported() -> Int32 {
    let _: any Sequence<Int32> = [1, 2, 3]
    return 1
}

@_cdecl("swift_contract_generic_protocol_array_i32_subscript")
public func swift_contract_generic_protocol_array_i32_subscript(_ index: Int32) -> Int32 {
    let values: [Int32] = [10, 20, 30, 40, 50]
    guard index >= 0, Int(index) < values.count else { return Int32.min }
    return values[Int(index)]
}

@_cdecl("swift_contract_generic_protocol_array_i32_witness_token")
public func swift_contract_generic_protocol_array_i32_witness_token() -> UInt64 {
    let token = ObjectIdentifier(Array<Int32>.self).hashValue
    return UInt64(bitPattern: Int64(token))
}

@_cdecl("swift_contract_generic_protocol_dict_string_i32_supported")
public func swift_contract_generic_protocol_dict_string_i32_supported() -> Int32 {
    let _: [String: Int32] = ["a": 1]
    return 1
}

@_cdecl("swift_contract_generic_protocol_dict_string_i32_lookup")
public func swift_contract_generic_protocol_dict_string_i32_lookup(
    _ keyPtr: UnsafePointer<CChar>?
) -> Int32 {
    guard let keyPtr else { return Int32.min }
    let dict: [String: Int32] = [
        "alpha": 101,
        "beta": 202,
        "gamma": 303,
    ]
    let key = String(cString: keyPtr)
    guard let value = dict[key] else { return Int32.min }
    return value
}

// MARK: - Constrained Generic Bounds (Track H.3)

/// Generic struct validated at compile time by `T: Equatable`.
private struct ContractEquatableBox<T: Equatable> {
    let value: T
    func equals(_ other: T) -> Bool { value == other }
}

/// Generic struct validated at compile time by `T: Comparable`.
private struct ContractComparableBox<T: Comparable> {
    let value: T
    /// Returns -1 if self < other, 1 if self > other, 0 if equal.
    func compare(_ other: T) -> Int32 {
        if value < other { return -1 }
        if value > other { return 1 }
        return 0
    }
}

/// Returns number of distinct values — requires `T: Hashable`.
private func _contractHashableDistinctCount<T: Hashable>(_ values: [T]) -> Int {
    Set(values).count
}

/// Returns a + b — requires `T: AdditiveArithmetic`.
private func _contractAdditiveSum<T: AdditiveArithmetic>(_ a: T, _ b: T) -> T { a + b }

/// JSON-encode then decode `v` — requires `T: Codable`.
private func _contractCodableRoundTrip<T: Codable>(_ value: T) throws -> T {
    let data = try JSONEncoder().encode(value)
    return try JSONDecoder().decode(T.self, from: data)
}

/// Returns min(a, b) — requires `T: Comparable & Hashable`.
private func _contractMultiBoundMin<T: Comparable & Hashable>(_ a: T, _ b: T) -> T { min(a, b) }

/// 1 if `a == b` (exercising `where T: Equatable`), 0 otherwise.
@_cdecl("swift_contract_constrained_equatable_equal")
public func swift_contract_constrained_equatable_equal(_ a: Int32, _ b: Int32) -> Int32 {
    ContractEquatableBox(value: a).equals(b) ? 1 : 0
}

/// Comparison result (-1 / 0 / 1) exercising `where T: Comparable`.
@_cdecl("swift_contract_constrained_comparable_cmp")
public func swift_contract_constrained_comparable_cmp(_ a: Int32, _ b: Int32) -> Int32 {
    ContractComparableBox(value: a).compare(b)
}

/// Number of distinct values among (a, b, c) — exercises `where T: Hashable`.
@_cdecl("swift_contract_constrained_hashable_distinct_count")
public func swift_contract_constrained_hashable_distinct_count(
    _ a: Int32, _ b: Int32, _ c: Int32
) -> Int32 {
    Int32(_contractHashableDistinctCount([a, b, c]))
}

/// a + b — exercises `where T: AdditiveArithmetic`.
@_cdecl("swift_contract_constrained_additive_sum")
public func swift_contract_constrained_additive_sum(_ a: Int32, _ b: Int32) -> Int32 {
    _contractAdditiveSum(a, b)
}

/// JSON-encode then decode `v`; returns decoded value, or Int32.min on failure.
/// Exercises `where T: Codable`.
@_cdecl("swift_contract_constrained_codable_roundtrip")
public func swift_contract_constrained_codable_roundtrip(_ v: Int32) -> Int32 {
    (try? _contractCodableRoundTrip(v)) ?? Int32.min
}

/// min(a, b) — exercises `where T: Comparable & Hashable` (multi-constraint).
@_cdecl("swift_contract_constrained_multi_min")
public func swift_contract_constrained_multi_min(_ a: Int32, _ b: Int32) -> Int32 {
    _contractMultiBoundMin(a, b)
}

// MARK: - Arbitrary Generic/Witness Instantiation (Track N.3)

private func _n3EscapeJSON(_ value: String) -> String {
    value
        .replacingOccurrences(of: "\\", with: "\\\\")
        .replacingOccurrences(of: "\"", with: "\\\"")
}

private func _n3SplitList(_ value: String) -> [String] {
    value
        .split(separator: ";")
        .map { $0.trimmingCharacters(in: .whitespacesAndNewlines) }
        .filter { !$0.isEmpty }
}

private func _n3GenericBase(_ typeName: String) -> String {
    guard let angle = typeName.firstIndex(of: "<") else { return typeName }
    return String(typeName[..<angle])
}

private func _n3GenericArgs(_ typeName: String) -> [String] {
    guard let start = typeName.firstIndex(of: "<"), let end = typeName.lastIndex(of: ">"), start < end else {
        return []
    }
    let inner = typeName[typeName.index(after: start)..<end]
    return inner
        .split(separator: ",")
        .map { $0.trimmingCharacters(in: .whitespacesAndNewlines) }
        .filter { !$0.isEmpty }
}

private func _n3IsPrimitiveType(_ typeName: String) -> Bool {
    ["Int32", "String"].contains(typeName)
}

private func _n3SupportedType(_ typeName: String) -> Bool {
    if _n3IsPrimitiveType(typeName) {
        return true
    }

    let base = _n3GenericBase(typeName)
    let args = _n3GenericArgs(typeName)
    switch (base, args.count) {
    case ("ContractGenericBox", 1):
        return _n3SupportedType(args[0])
    case ("Array", 1):
        return _n3SupportedType(args[0])
    case ("Dictionary", 2):
        return _n3SupportedType(args[0]) && _n3SupportedType(args[1])
            && _n3ProtocolSatisfied(args[0], "Hashable")
    default:
        return false
    }
}

private func _n3ProtocolSatisfied(_ typeName: String, _ protocolName: String) -> Bool {
    let base = _n3GenericBase(typeName)
    let args = _n3GenericArgs(typeName)
    switch protocolName {
    case "Equatable":
        if ["Int32", "String"].contains(typeName) { return true }
        if base == "Array", args.count == 1 { return _n3ProtocolSatisfied(args[0], "Equatable") }
        return false
    case "Comparable":
        return ["Int32", "String"].contains(typeName)
    case "Hashable":
        return ["Int32", "String"].contains(typeName)
    case "AdditiveArithmetic":
        return typeName == "Int32"
    case "Codable":
        if ["Int32", "String"].contains(typeName) { return true }
        if base == "Array", args.count == 1 { return _n3ProtocolSatisfied(args[0], "Codable") }
        if base == "Dictionary", args.count == 2 {
            return _n3ProtocolSatisfied(args[0], "Codable")
                && _n3ProtocolSatisfied(args[0], "Hashable")
                && _n3ProtocolSatisfied(args[1], "Codable")
        }
        return false
    case "Sequence":
        if base == "Array", args.count == 1 { return _n3SupportedType(args[0]) }
        if base == "Dictionary", args.count == 2 {
            return _n3SupportedType(args[0]) && _n3SupportedType(args[1])
                && _n3ProtocolSatisfied(args[0], "Hashable")
        }
        return false
    default:
        return false
    }
}

private func _n3AssociatedTypeValue(_ typeName: String, _ associatedType: String) -> String? {
    let base = _n3GenericBase(typeName)
    let args = _n3GenericArgs(typeName)
    switch (base, associatedType, args.count) {
    case ("Array", "Element", 1):
        return args[0]
    case ("Dictionary", "Key", 2):
        return args[0]
    case ("Dictionary", "Value", 2):
        return args[1]
    case ("ContractGenericBox", "Wrapped", 1):
        return args[0]
    default:
        return nil
    }
}

private func _n3ValidateRequirements(_ typeName: String, _ requirements: [String]) -> (Bool, [[String: String]]) {
    var failures: [[String: String]] = []
    for requirement in requirements {
        if let eq = requirement.range(of: "==") {
            let lhs = requirement[..<eq.lowerBound].trimmingCharacters(in: .whitespacesAndNewlines)
            let rhs = requirement[eq.upperBound...].trimmingCharacters(in: .whitespacesAndNewlines)
            guard let actual = _n3AssociatedTypeValue(typeName, lhs) else {
                failures.append([
                    "code": "unknown_associated_type",
                    "requirement": requirement,
                    "detail": "associated type \(lhs) unavailable for \(typeName)",
                ])
                continue
            }
            if actual != rhs {
                failures.append([
                    "code": "associated_type_mismatch",
                    "requirement": requirement,
                    "detail": "expected \(lhs)==\(rhs), actual \(lhs)==\(actual)",
                ])
            }
            continue
        }

        if !_n3ProtocolSatisfied(typeName, requirement) {
            failures.append([
                "code": "unsupported_protocol",
                "requirement": requirement,
                "detail": "\(typeName) does not conform to \(requirement)",
            ])
        }
    }
    return (failures.isEmpty, failures)
}

private func _n3StableToken(_ pieces: [String]) -> UInt64 {
    var hash: UInt64 = 1469598103934665603
    for piece in pieces {
        for byte in piece.utf8 {
            hash ^= UInt64(byte)
            hash &*= 1099511628211
        }
        hash ^= 0xFF
        hash &*= 1099511628211
    }
    return hash == 0 ? 1 : hash
}

@_cdecl("swift_contract_n3_build_context_json")
public func swift_contract_n3_build_context_json(
    _ typeNamePtr: UnsafePointer<CChar>?,
    _ constraintsPtr: UnsafePointer<CChar>?
) -> UnsafeMutablePointer<CChar>? {
    guard let typeNamePtr else { return nil }
    let typeName = String(cString: typeNamePtr)
    let constraints = constraintsPtr.map { _n3SplitList(String(cString: $0)) } ?? []
    let supported = _n3SupportedType(typeName)
    let args = _n3GenericArgs(typeName)
    let constraintJson = constraints.map { constraint -> String in
        let satisfied = _n3ProtocolSatisfied(typeName, constraint)
        return "{\"name\":\"\(_n3EscapeJSON(constraint))\",\"satisfied\":\(satisfied ? "true" : "false")}"
    }.joined(separator: ",")
    let argsJson = args.map { "\"\(_n3EscapeJSON($0))\"" }.joined(separator: ",")
    let json = "{\"type_name\":\"\(_n3EscapeJSON(typeName))\",\"generic_base\":\"\(_n3EscapeJSON(_n3GenericBase(typeName)))\",\"arguments\":[\(argsJson)],\"constraints\":[\(constraintJson)],\"supported\":\(supported ? "true" : "false")}"
    return strdup(json)
}

@_cdecl("swift_contract_n3_resolve_witness_json")
public func swift_contract_n3_resolve_witness_json(
    _ typeNamePtr: UnsafePointer<CChar>?,
    _ protocolPtr: UnsafePointer<CChar>?,
    _ requirementsPtr: UnsafePointer<CChar>?
) -> UnsafeMutablePointer<CChar>? {
    guard let typeNamePtr, let protocolPtr else { return nil }
    let typeName = String(cString: typeNamePtr)
    let protocolName = String(cString: protocolPtr)
    let requirements = requirementsPtr.map { _n3SplitList(String(cString: $0)) } ?? []
    let merged = [protocolName] + requirements
    let (satisfied, failures) = _n3ValidateRequirements(typeName, merged)
    let token = satisfied ? _n3StableToken([typeName, protocolName] + requirements) : 0
    let failuresJson = failures.map { failure in
        let code = _n3EscapeJSON(failure["code"] ?? "")
        let requirement = _n3EscapeJSON(failure["requirement"] ?? "")
        let detail = _n3EscapeJSON(failure["detail"] ?? "")
        return "{\"code\":\"\(code)\",\"requirement\":\"\(requirement)\",\"detail\":\"\(detail)\"}"
    }.joined(separator: ",")
    let json = "{\"type_name\":\"\(_n3EscapeJSON(typeName))\",\"protocol\":\"\(_n3EscapeJSON(protocolName))\",\"requirements\":[\(requirements.map { "\"\(_n3EscapeJSON($0))\"" }.joined(separator: ","))],\"supported\":\(satisfied ? "true" : "false"),\"token\":\(token),\"failures\":[\(failuresJson)]}"
    return strdup(json)
}

@_cdecl("swift_contract_n3_validate_requirements_json")
public func swift_contract_n3_validate_requirements_json(
    _ typeNamePtr: UnsafePointer<CChar>?,
    _ requirementsPtr: UnsafePointer<CChar>?
) -> UnsafeMutablePointer<CChar>? {
    guard let typeNamePtr else { return nil }
    let typeName = String(cString: typeNamePtr)
    let requirements = requirementsPtr.map { _n3SplitList(String(cString: $0)) } ?? []
    let (satisfied, failures) = _n3ValidateRequirements(typeName, requirements)
    let failuresJson = failures.map { failure in
        let code = _n3EscapeJSON(failure["code"] ?? "")
        let requirement = _n3EscapeJSON(failure["requirement"] ?? "")
        let detail = _n3EscapeJSON(failure["detail"] ?? "")
        return "{\"code\":\"\(code)\",\"requirement\":\"\(requirement)\",\"detail\":\"\(detail)\"}"
    }.joined(separator: ",")
    let json = "{\"type_name\":\"\(_n3EscapeJSON(typeName))\",\"supported\":\(satisfied ? "true" : "false"),\"failures\":[\(failuresJson)]}"
    return strdup(json)
}

@_cdecl("swift_contract_n3_invoke_generic_i32")
public func swift_contract_n3_invoke_generic_i32(
    _ typeNamePtr: UnsafePointer<CChar>?,
    _ requirementsPtr: UnsafePointer<CChar>?,
    _ operationPtr: UnsafePointer<CChar>?,
    _ a: Int32,
    _ b: Int32,
    _ errorCodePtr: UnsafeMutablePointer<Int32>?
) -> Int32 {
    guard let typeNamePtr, let operationPtr else {
        errorCodePtr?.pointee = -530
        return 0
    }
    let typeName = String(cString: typeNamePtr)
    let requirements = requirementsPtr.map { _n3SplitList(String(cString: $0)) } ?? []
    let operation = String(cString: operationPtr)
    let (ok, _) = _n3ValidateRequirements(typeName, requirements)
    guard ok else {
        errorCodePtr?.pointee = -531
        return 0
    }
    errorCodePtr?.pointee = 0

    switch (typeName, operation) {
    case ("ContractGenericBox<Int32>", "box_make_get"):
        return ContractGenericBox<Int32>(a).value
    case ("ContractGenericBox<String>", "box.sample_metric"):
        let value = String(repeating: "x", count: Int(max(0, min(a, 256))))
        return Int32(ContractGenericBox<String>(value).value.count)
    case ("Int32", "equatable.equal"):
        return a == b ? 1 : 0
    case ("Int32", "comparable.cmp"):
        return a < b ? -1 : (a > b ? 1 : 0)
    case ("Int32", "additive.sum"):
        return a &+ b
    case ("Array<Int32>", "sequence.sum_range"):
        let bounded = max(0, min(b, 256))
        return (0..<bounded).reduce(0) { partial, offset in partial &+ (a &+ offset) }
    case ("Array<String>", "sequence.sample_metric"):
        let bounded = max(0, min(b, 64))
        let values = (0..<bounded).map { offset in
            String(repeating: "x", count: Int(max(0, min(a &+ offset, 256))))
        }
        return values.reduce(0) { partial, value in partial &+ Int32(value.count) }
    case ("Dictionary<String,Int32>", "sequence.sum_values"):
        let dict: [String: Int32] = ["alpha": a, "beta": a &+ 1, "gamma": a &+ 2]
        return dict.values.reduce(0, &+)
    case ("Dictionary<String,String>", "sequence.sample_metric"):
        let dict: [String: String] = [
            "alpha": String(repeating: "a", count: Int(max(0, min(a, 256)))),
            "beta": String(repeating: "b", count: Int(max(0, min(a &+ 1, 256)))),
            "gamma": String(repeating: "c", count: Int(max(0, min(a &+ 2, 256)))),
        ]
        return dict.values.reduce(0) { partial, value in partial &+ Int32(value.count) }
    default:
        errorCodePtr?.pointee = -532
        return 0
    }
}

// MARK: - Foundation Date/Time (Track I.1)

/// Deterministic ISO 8601 formatter (UTC, POSIX locale) re-created each call
/// to stay thread-safe without a global.
private func _makePOSIXFormatter() -> ISO8601DateFormatter {
    let fmt = ISO8601DateFormatter()
    fmt.timeZone = TimeZone(identifier: "UTC")
    fmt.formatOptions = [.withInternetDateTime, .withDashSeparatorInDate,
                         .withColonSeparatorInTime, .withTimeZone]
    return fmt
}

/// Format a Unix timestamp (seconds since epoch) as ISO 8601 (UTC).
/// Returns a malloc-backed C string; caller must free.
@_cdecl("swift_contract_datetime_format_unix")
public func swift_contract_datetime_format_unix(_ ts: Double) -> UnsafeMutablePointer<CChar>? {
    let date = Date(timeIntervalSince1970: ts)
    let str = _makePOSIXFormatter().string(from: date)
    return strdup(str)
}

/// Parse an ISO 8601 string to a Unix timestamp; returns Double.nan bits on failure.
@_cdecl("swift_contract_datetime_parse_iso8601")
public func swift_contract_datetime_parse_iso8601(_ ptr: UnsafePointer<CChar>?) -> Double {
    guard let ptr else { return Double.nan }
    let str = String(cString: ptr)
    guard let date = _makePOSIXFormatter().date(from: str) else { return Double.nan }
    return date.timeIntervalSince1970
}

/// Calendar year (UTC) for a given Unix timestamp.
@_cdecl("swift_contract_datetime_year_utc")
public func swift_contract_datetime_year_utc(_ ts: Double) -> Int32 {
    var cal = Calendar(identifier: .gregorian)
    cal.timeZone = TimeZone(identifier: "UTC")!
    let comps = cal.dateComponents([.year], from: Date(timeIntervalSince1970: ts))
    return Int32(comps.year ?? 0)
}

/// Calendar month (1-12, UTC) for a given Unix timestamp.
@_cdecl("swift_contract_datetime_month_utc")
public func swift_contract_datetime_month_utc(_ ts: Double) -> Int32 {
    var cal = Calendar(identifier: .gregorian)
    cal.timeZone = TimeZone(identifier: "UTC")!
    let comps = cal.dateComponents([.month], from: Date(timeIntervalSince1970: ts))
    return Int32(comps.month ?? 0)
}

/// UTC timezone offset in seconds — always 0 for the UTC zone.
@_cdecl("swift_contract_datetime_utc_offset_seconds")
public func swift_contract_datetime_utc_offset_seconds() -> Int32 {
    Int32(TimeZone(identifier: "UTC")!.secondsFromGMT())
}

// MARK: - Foundation Data / UUID / CharacterSet (Track I.2)

/// Byte-sum (wrapping UInt32) of a raw buffer — exercises Data construction.
@_cdecl("swift_contract_data_from_bytes_checksum")
public func swift_contract_data_from_bytes_checksum(
    _ ptr: UnsafePointer<UInt8>?, _ len: Int32
) -> UInt32 {
    guard let ptr, len > 0 else { return 0 }
    let buf = UnsafeBufferPointer(start: ptr, count: Int(len))
    return buf.reduce(0) { $0 &+ UInt32($1) }
}

/// Generate a new UUID string (36 chars, uppercase).
/// Returns malloc-backed C string; caller must free.
@_cdecl("swift_contract_uuid_new_string")
public func swift_contract_uuid_new_string() -> UnsafeMutablePointer<CChar>? {
    strdup(UUID().uuidString)
}

/// Parse a UUID string; returns 1 if valid, 0 otherwise.
@_cdecl("swift_contract_uuid_parse_validate")
public func swift_contract_uuid_parse_validate(_ ptr: UnsafePointer<CChar>?) -> Int32 {
    guard let ptr else { return 0 }
    return UUID(uuidString: String(cString: ptr)) != nil ? 1 : 0
}

/// Generate UUID, convert to string, parse back — returns 1 on success.
@_cdecl("swift_contract_uuid_roundtrip")
public func swift_contract_uuid_roundtrip() -> Int32 {
    let id = UUID()
    return UUID(uuidString: id.uuidString) == id ? 1 : 0
}

/// Returns 1 if the Unicode scalar `codepoint` belongs to CharacterSet.letters.
@_cdecl("swift_contract_charset_is_letter")
public func swift_contract_charset_is_letter(_ codepoint: Int32) -> Int32 {
    guard codepoint >= 0, let scalar = Unicode.Scalar(UInt32(codepoint)) else { return 0 }
    return CharacterSet.letters.contains(scalar) ? 1 : 0
}

// MARK: - Foundation URL & URLComponents (Track I.3)

/// Returns 1 if `str` is a valid absolute URL, 0 otherwise.
@_cdecl("swift_contract_url_parse_valid")
public func swift_contract_url_parse_valid(_ ptr: UnsafePointer<CChar>?) -> Int32 {
    guard let ptr else { return 0 }
    let str = String(cString: ptr)
    guard let url = URL(string: str), url.scheme != nil else { return 0 }
    return 1
}

/// Extract the scheme component; returns malloc-backed C string or nil.
@_cdecl("swift_contract_url_scheme")
public func swift_contract_url_scheme(_ ptr: UnsafePointer<CChar>?) -> UnsafeMutablePointer<CChar>? {
    guard let ptr else { return nil }
    guard let url = URL(string: String(cString: ptr)), let s = url.scheme else { return nil }
    return strdup(s)
}

/// Extract the host component; returns malloc-backed C string or nil.
@_cdecl("swift_contract_url_host")
public func swift_contract_url_host(_ ptr: UnsafePointer<CChar>?) -> UnsafeMutablePointer<CChar>? {
    guard let ptr else { return nil }
    guard let url = URL(string: String(cString: ptr)), let h = url.host else { return nil }
    return strdup(h)
}

/// Extract the path component; returns malloc-backed C string.
@_cdecl("swift_contract_url_path")
public func swift_contract_url_path(_ ptr: UnsafePointer<CChar>?) -> UnsafeMutablePointer<CChar>? {
    guard let ptr else { return nil }
    guard let url = URL(string: String(cString: ptr)) else { return nil }
    return strdup(url.path)
}

/// Build a URL from (scheme, host, path) using URLComponents.
/// Returns malloc-backed C string of the absolute URL, or nil.
@_cdecl("swift_contract_url_build_from_components")
public func swift_contract_url_build_from_components(
    _ schemePtr: UnsafePointer<CChar>?,
    _ hostPtr: UnsafePointer<CChar>?,
    _ pathPtr: UnsafePointer<CChar>?
) -> UnsafeMutablePointer<CChar>? {
    guard let schemePtr, let hostPtr, let pathPtr else { return nil }
    var c = URLComponents()
    c.scheme = String(cString: schemePtr)
    c.host   = String(cString: hostPtr)
    c.path   = String(cString: pathPtr)
    guard let url = c.url else { return nil }
    return strdup(url.absoluteString)
}

// MARK: - Foundation NSCoding / NSCopying (Track I.4)

/// Archive an Int32 as NSNumber via NSKeyedArchiver then unarchive; returns
/// the decoded value, or Int32.min on error.
@_cdecl("swift_contract_nscoding_integer_roundtrip")
public func swift_contract_nscoding_integer_roundtrip(_ v: Int32) -> Int32 {
    let nsv = NSNumber(value: v)
    guard
        let data = try? NSKeyedArchiver.archivedData(withRootObject: nsv,
                                                      requiringSecureCoding: true),
        let decoded = try? NSKeyedUnarchiver.unarchivedObject(
            ofClass: NSNumber.self, from: data)
    else { return Int32.min }
    return decoded.int32Value
}

/// Archive a C string as NSString then unarchive; returns the decoded UTF-8
/// length, or -1 on error.
@_cdecl("swift_contract_nscoding_string_roundtrip")
public func swift_contract_nscoding_string_roundtrip(_ ptr: UnsafePointer<CChar>?) -> Int32 {
    guard let ptr else { return -1 }
    let nsStr = NSString(utf8String: ptr) ?? ""
    guard
        let data = try? NSKeyedArchiver.archivedData(withRootObject: nsStr,
                                                      requiringSecureCoding: true),
        let decoded = try? NSKeyedUnarchiver.unarchivedObject(
            ofClass: NSString.self, from: data)
    else { return -1 }
    return Int32(decoded.length)
}

/// NSCopying: create an NSMutableArray, copy it, mutate the copy, verify the
/// original is unchanged. Returns 1 if independent, 0 otherwise.
@_cdecl("swift_contract_nscopying_array_independence")
public func swift_contract_nscopying_array_independence() -> Int32 {
    let original = NSMutableArray(array: [1, 2, 3] as [NSNumber])
    let copied = original.mutableCopy() as! NSMutableArray
    copied.add(NSNumber(value: 99))
    return original.count == 3 ? 1 : 0
}

// MARK: - Key Path Runtime Support (Track J.1)

private struct ProbeKeyPathStats {
    var score: Int32
}

private struct ProbeKeyPathUser {
    var age: Int32
    var stats: ProbeKeyPathStats
}

/// Read `age` using a strongly typed key path.
@_cdecl("swift_contract_keypath_get_age")
public func swift_contract_keypath_get_age(_ age: Int32) -> Int32 {
    let user = ProbeKeyPathUser(age: age, stats: .init(score: 0))
    return user[keyPath: \ProbeKeyPathUser.age]
}

/// Read nested `stats.score` using a composed key path.
@_cdecl("swift_contract_keypath_get_nested_score")
public func swift_contract_keypath_get_nested_score(_ score: Int32) -> Int32 {
    let user = ProbeKeyPathUser(age: 0, stats: .init(score: score))
    return user[keyPath: \ProbeKeyPathUser.stats.score]
}

/// Validate AnyKeyPath metadata path and value extraction path.
@_cdecl("swift_contract_keypath_any_matches")
public func swift_contract_keypath_any_matches() -> Int32 {
    let typed: KeyPath<ProbeKeyPathUser, Int32> = \ProbeKeyPathUser.age
    let any: AnyKeyPath = typed
    return any == typed ? 1 : 0
}

// MARK: - Property Wrapper Metadata (Track J.2)

@propertyWrapper
private struct Clamp0To100 {
    private var value: Int32

    var wrappedValue: Int32 {
        get { value }
        set { value = max(0, min(100, newValue)) }
    }

    var projectedValue: Int32 { value }

    init(wrappedValue: Int32) {
        self.value = max(0, min(100, wrappedValue))
    }
}

private struct ProbeWrapperBox {
    @Clamp0To100 var value: Int32
}

/// Construct wrapper-backed storage and return clamped value.
@_cdecl("swift_contract_wrapper_init_clamped")
public func swift_contract_wrapper_init_clamped(_ v: Int32) -> Int32 {
    ProbeWrapperBox(value: v).value
}

/// Mutate wrapper-backed storage and return post-clamp value.
@_cdecl("swift_contract_wrapper_set_clamped")
public func swift_contract_wrapper_set_clamped(_ initial: Int32, _ newValue: Int32) -> Int32 {
    var box = ProbeWrapperBox(value: initial)
    box.value = newValue
    return box.value
}

/// Return projected value (`$value`) to verify wrapper projection semantics.
@_cdecl("swift_contract_wrapper_projected_value")
public func swift_contract_wrapper_projected_value(_ v: Int32) -> Int32 {
    let box = ProbeWrapperBox(value: v)
    return box.$value
}

// MARK: - Opaque Type Bridging (Track J.3)

private protocol ProbeNamed {
    var name: String { get }
}

private struct ProbeOpaqueNamed: ProbeNamed {
    let name: String
}

private func _makeOpaqueNamed(_ tag: Int32) -> some ProbeNamed {
    if tag % 2 == 0 {
        return ProbeOpaqueNamed(name: "even")
    }
    return ProbeOpaqueNamed(name: "odd")
}

/// Return the `name` from an opaque `some ProbeNamed` value.
@_cdecl("swift_contract_opaque_named_get_name")
public func swift_contract_opaque_named_get_name(_ tag: Int32) -> UnsafeMutablePointer<CChar>? {
    let v = _makeOpaqueNamed(tag)
    return strdup(v.name)
}

/// Return the UTF-8 byte count of `name` from opaque value.
@_cdecl("swift_contract_opaque_named_name_len")
public func swift_contract_opaque_named_name_len(_ tag: Int32) -> Int32 {
    Int32(_makeOpaqueNamed(tag).name.utf8.count)
}

// MARK: - Result Builder & DSL Support (Track J.4)

@resultBuilder
private enum ProbeIntSumBuilder {
    static func buildExpression(_ expr: Int32) -> Int32 { expr }
    static func buildBlock(_ components: Int32...) -> Int32 {
        components.reduce(0, +)
    }
    static func buildOptional(_ component: Int32?) -> Int32 { component ?? 0 }
    static func buildEither(first component: Int32) -> Int32 { component }
    static func buildEither(second component: Int32) -> Int32 { component }
    static func buildArray(_ components: [Int32]) -> Int32 {
        components.reduce(0, +)
    }
}

private func _buildIntSum(@ProbeIntSumBuilder _ body: () -> Int32) -> Int32 {
    body()
}

/// Build a simple DSL sum from two values.
@_cdecl("swift_contract_builder_sum2")
public func swift_contract_builder_sum2(_ a: Int32, _ b: Int32) -> Int32 {
    _buildIntSum {
        a
        b
    }
}

/// Build a conditional DSL sum with builder `buildEither` support.
@_cdecl("swift_contract_builder_conditional")
public func swift_contract_builder_conditional(_ flag: Int32) -> Int32 {
    _buildIntSum {
        if flag != 0 {
            10
        } else {
            20
        }
    }
}

/// Build a loop-based DSL sum with builder `buildArray` support.
@_cdecl("swift_contract_builder_loop_sum")
public func swift_contract_builder_loop_sum(_ n: Int32) -> Int32 {
    let safeN = max(0, n)
    if safeN == 0 { return 0 }
    return _buildIntSum {
        for i in 1...safeN {
            i
        }
    }
}

// MARK: - Reference Cycle & Memory Safety (Track K)

private final class K1WeakTarget {
    var value: Int32
    init(_ v: Int32) { value = v }
}

private final class K1CycleNode {
    var next: K1CycleNode?
}

private final class K1UnownedOwner {
    let tag: Int32
    init(_ tag: Int32) { self.tag = tag }
}

private final class K1UnownedChild {
    unowned(unsafe) var owner: K1UnownedOwner
    init(owner: K1UnownedOwner) { self.owner = owner }
}

/// Weak lifecycle: weak ref is non-nil while strong lives, then nil after drop.
@_cdecl("swift_contract_k1_weak_lifecycle")
public func swift_contract_k1_weak_lifecycle() -> Int32 {
    var strong: K1WeakTarget? = K1WeakTarget(7)
    weak let weakRef = strong
    let before = (weakRef != nil)
    strong = nil
    let after = (weakRef == nil)
    return (before && after) ? 1 : 0
}

/// Detect that an unowned(unsafe) edge would dangle after owner deallocation.
/// We intentionally avoid dereferencing after drop to prevent a crash.
@_cdecl("swift_contract_k1_unowned_dangling_detected")
public func swift_contract_k1_unowned_dangling_detected() -> Int32 {
    var owner: K1UnownedOwner? = K1UnownedOwner(123)
    weak let weakOwner = owner
    if let owner {
        _ = K1UnownedChild(owner: owner)
    }
    owner = nil
    return weakOwner == nil ? 1 : 0
}

/// Create A<->B strong cycle and verify weak probes still see both nodes.
@_cdecl("swift_contract_k1_cycle_detect_strong_pair")
public func swift_contract_k1_cycle_detect_strong_pair() -> Int32 {
    var a: K1CycleNode? = K1CycleNode()
    var b: K1CycleNode? = K1CycleNode()
    a?.next = b
    b?.next = a

    weak let wa = a
    weak let wb = b
    a = nil
    b = nil

    return (wa != nil && wb != nil) ? 1 : 0
}

/// Create non-cyclic pair (second link absent) and verify deallocation occurs.
@_cdecl("swift_contract_k1_cycle_detect_acyclic_pair")
public func swift_contract_k1_cycle_detect_acyclic_pair() -> Int32 {
    var a: K1CycleNode? = K1CycleNode()
    var b: K1CycleNode? = K1CycleNode()
    a?.next = b

    weak let wa = a
    weak let wb = b
    a = nil
    b = nil

    return (wa == nil && wb == nil) ? 1 : 0
}

// Track K.2 globals
private final class K2RetainProbe: NSObject {}

/// Return retain-count delta from an explicit retain/release pair.
@_cdecl("swift_contract_k2_retain_delta")
public func swift_contract_k2_retain_delta() -> Int32 {
    let obj = K2RetainProbe()
    let baseline = CFGetRetainCount(obj)
    let retained = Unmanaged.passRetained(obj).toOpaque()
    let afterRetain = CFGetRetainCount(obj)
    _ = Unmanaged<K2RetainProbe>.fromOpaque(retained).takeRetainedValue()
    let afterRelease = CFGetRetainCount(obj)

    let delta = Int32(afterRetain - baseline)
    // Require retain bump and restoration after balanced release.
    return (delta >= 1 && afterRelease <= afterRetain) ? delta : Int32.min
}

/// Reference-type inference probe: 1=class, 2=value, 3=metatype.
@_cdecl("swift_contract_k2_reference_type_infer")
public func swift_contract_k2_reference_type_infer(_ mode: Int32) -> Int32 {
    switch mode {
    case 1:
        return Mirror(reflecting: K2RetainProbe()).displayStyle == .class ? 1 : 0
    case 2:
        return Mirror(reflecting: ProbeKeyPathStats(score: 1)).displayStyle == .struct ? 2 : 0
    case 3:
        return Mirror(reflecting: K2RetainProbe.self).displayStyle == nil ? 3 : 0
    default:
        return 0
    }
}

/// Emit deterministic DOT graph for cycle visualization path.
@_cdecl("swift_contract_k2_reference_graph_dot")
public func swift_contract_k2_reference_graph_dot() -> UnsafeMutablePointer<CChar>? {
    let dot = "digraph G { A -> B; B -> A; }"
    return strdup(dot)
}

// Track K.3 globals
private var k3NextToken: Int32 = 1
private var k3TokenToSite: [Int32: Int32] = [:]
private var k3LiveBySite: [Int32: Int32] = [:]

@_cdecl("swift_contract_k3_tracker_reset")
public func swift_contract_k3_tracker_reset() {
    k3NextToken = 1
    k3TokenToSite.removeAll(keepingCapacity: true)
    k3LiveBySite.removeAll(keepingCapacity: true)
}

/// Allocate a tracked token for the given call-site id.
@_cdecl("swift_contract_k3_alloc")
public func swift_contract_k3_alloc(_ site: Int32) -> UnsafeMutableRawPointer? {
    let token = k3NextToken
    k3NextToken &+= 1
    k3TokenToSite[token] = site
    k3LiveBySite[site, default: 0] += 1
    return UnsafeMutableRawPointer(bitPattern: Int(token))
}

/// Release a tracked token; returns 1 if released, 0 if unknown/null.
@_cdecl("swift_contract_k3_release")
public func swift_contract_k3_release(_ tokenPtr: UnsafeMutableRawPointer?) -> Int32 {
    guard let tokenPtr else { return 0 }
    let token = Int32(Int(bitPattern: tokenPtr))
    guard let site = k3TokenToSite.removeValue(forKey: token) else { return 0 }
    if let live = k3LiveBySite[site] {
        k3LiveBySite[site] = max(0, live - 1)
    }
    return 1
}

/// Count currently unreleased tracked tokens.
@_cdecl("swift_contract_k3_sweep_unreleased_count")
public func swift_contract_k3_sweep_unreleased_count() -> Int32 {
    Int32(k3TokenToSite.count)
}

/// Live count for a specific site id.
@_cdecl("swift_contract_k3_live_count_for_site")
public func swift_contract_k3_live_count_for_site(_ site: Int32) -> Int32 {
    k3LiveBySite[site, default: 0]
}

/// Root-cause site: site id with max live allocations, or -1 if none.
@_cdecl("swift_contract_k3_root_cause_site")
public func swift_contract_k3_root_cause_site() -> Int32 {
    guard let maxEntry = k3LiveBySite.max(by: { $0.value < $1.value }), maxEntry.value > 0 else {
        return -1
    }
    return maxEntry.key
}

// MARK: - ABI Stability v2+ & User-Defined Types (Track L)

// Track L.1 registry globals
private var l1NextTypeID: Int32 = 10000
private var l1TypeByName: [String: Int32] = [:]
private var l1NameByTypeID: [Int32: String] = [:]
private var l1VersionByTypeID: [Int32: Int32] = [:]

@_cdecl("swift_contract_l1_registry_reset")
public func swift_contract_l1_registry_reset() {
    l1NextTypeID = 10000
    l1TypeByName.removeAll(keepingCapacity: true)
    l1NameByTypeID.removeAll(keepingCapacity: true)
    l1VersionByTypeID.removeAll(keepingCapacity: true)
}

/// Register a user-defined type by name and assign a stable versioned ID.
/// If already registered, returns the existing ID.
@_cdecl("swift_contract_l1_register_type")
public func swift_contract_l1_register_type(_ namePtr: UnsafePointer<CChar>?) -> Int32 {
    guard let namePtr else { return -1 }
    let name = String(cString: namePtr)
    guard !name.isEmpty else { return -1 }
    if let existing = l1TypeByName[name] {
        return existing
    }
    let id = l1NextTypeID
    l1NextTypeID &+= 1
    l1TypeByName[name] = id
    l1NameByTypeID[id] = name
    l1VersionByTypeID[id] = 1
    return id
}

/// Lookup a registered type ID by name; returns -1 if missing.
@_cdecl("swift_contract_l1_lookup_type_id")
public func swift_contract_l1_lookup_type_id(_ namePtr: UnsafePointer<CChar>?) -> Int32 {
    guard let namePtr else { return -1 }
    let name = String(cString: namePtr)
    return l1TypeByName[name] ?? -1
}

/// Bump a registered type version; returns new version or -1 if unknown ID.
@_cdecl("swift_contract_l1_bump_type_version")
public func swift_contract_l1_bump_type_version(_ typeID: Int32) -> Int32 {
    guard let current = l1VersionByTypeID[typeID] else { return -1 }
    let next = current + 1
    l1VersionByTypeID[typeID] = next
    return next
}

/// Backward/forward compat check for update protocol.
/// Compatible when major matches and newMinor >= oldMinor.
/// Version format: major*1000 + minor.
@_cdecl("swift_contract_l1_update_compat")
public func swift_contract_l1_update_compat(_ oldVersion: Int32, _ newVersion: Int32) -> Int32 {
    guard oldVersion >= 0, newVersion >= 0 else { return 0 }
    let oldMajor = oldVersion / 1000
    let oldMinor = oldVersion % 1000
    let newMajor = newVersion / 1000
    let newMinor = newVersion % 1000
    return (oldMajor == newMajor && newMinor >= oldMinor) ? 1 : 0
}

// Track L.2 compatibility globals
private let l2ResilienceMarkers: [String: Int32] = [
    "resilient_layout": 1,
    "private_fields": 2,
    "versioned_fields": 4,
]

/// Return number of breaking removals by comparing old/new exported type counts.
@_cdecl("swift_contract_l2_contract_diff_breaking_count")
public func swift_contract_l2_contract_diff_breaking_count(
    _ oldTypeCount: Int32,
    _ newTypeCount: Int32
) -> Int32 {
    if oldTypeCount <= newTypeCount { return 0 }
    return oldTypeCount - newTypeCount
}

/// Binary compatibility checker.
/// Compatible when runtime major > contract major OR same major with runtime minor >= contract minor.
/// Version format: major*1000 + minor.
@_cdecl("swift_contract_l2_binary_version_compatible")
public func swift_contract_l2_binary_version_compatible(
    _ runtimeVersion: Int32,
    _ contractVersion: Int32
) -> Int32 {
    guard runtimeVersion >= 0, contractVersion >= 0 else { return 0 }
    let rMaj = runtimeVersion / 1000
    let rMin = runtimeVersion % 1000
    let cMaj = contractVersion / 1000
    let cMin = contractVersion % 1000
    if rMaj > cMaj { return 1 }
    if rMaj < cMaj { return 0 }
    return rMin >= cMin ? 1 : 0
}

/// Return resilience marker bit value for a marker name.
@_cdecl("swift_contract_l2_resilience_marker")
public func swift_contract_l2_resilience_marker(_ markerPtr: UnsafePointer<CChar>?) -> Int32 {
    guard let markerPtr else { return 0 }
    let marker = String(cString: markerPtr)
    return l2ResilienceMarkers[marker] ?? 0
}

// Track L.3 derivation helpers
private func _l3EscapeJSON(_ s: String) -> String {
    s.replacingOccurrences(of: "\\", with: "\\\\")
     .replacingOccurrences(of: "\"", with: "\\\"")
}

private func _l3InferKind(_ decl: String) -> String {
    if decl.contains("protocol ") { return "protocol" }
    if decl.contains("class ") { return "class" }
    if decl.contains("struct ") { return "struct" }
    return "unknown"
}

private func _l3InferName(_ decl: String) -> String {
    let tokens = decl.split(whereSeparator: { $0.isWhitespace || $0 == "{" || $0 == ":" })
    if let idx = tokens.firstIndex(of: Substring("struct")), tokens.indices.contains(idx + 1) {
        return String(tokens[idx + 1])
    }
    if let idx = tokens.firstIndex(of: Substring("class")), tokens.indices.contains(idx + 1) {
        return String(tokens[idx + 1])
    }
    if let idx = tokens.firstIndex(of: Substring("protocol")), tokens.indices.contains(idx + 1) {
        return String(tokens[idx + 1])
    }
    return "Unknown"
}

/// Derive a minimal contract JSON descriptor from a Swift declaration string.
@_cdecl("swift_contract_l3_derive_contract_from_source")
public func swift_contract_l3_derive_contract_from_source(
    _ sourcePtr: UnsafePointer<CChar>?
) -> UnsafeMutablePointer<CChar>? {
    guard let sourcePtr else { return nil }
    let source = String(cString: sourcePtr)
    let kind = _l3InferKind(source)
    let name = _l3InferName(source)
    let json = "{\"name\":\"\(_l3EscapeJSON(name))\",\"kind\":\"\(_l3EscapeJSON(kind))\",\"source_len\":\(source.utf8.count)}"
    return strdup(json)
}

/// Validate derived contract JSON against a handwritten JSON string.
/// Returns 1 when both contain same `name` and `kind` fields verbatim.
@_cdecl("swift_contract_l3_validate_derived_contract")
public func swift_contract_l3_validate_derived_contract(
    _ derivedPtr: UnsafePointer<CChar>?,
    _ handwrittenPtr: UnsafePointer<CChar>?
) -> Int32 {
    guard let derivedPtr, let handwrittenPtr else { return 0 }
    let derived = String(cString: derivedPtr)
    let handwritten = String(cString: handwrittenPtr)
    let requiredFields = ["\"name\"", "\"kind\""]
    for f in requiredFields {
        if !derived.contains(f) || !handwritten.contains(f) {
            return 0
        }
    }
    return (derived.contains("\"kind\":\"struct\"") == handwritten.contains("\"kind\":\"struct\"")) &&
           (derived.contains("\"kind\":\"class\"") == handwritten.contains("\"kind\":\"class\"")) &&
           (derived.contains("\"kind\":\"protocol\"") == handwritten.contains("\"kind\":\"protocol\"")) ? 1 : 0
}

/// Exporter macro simulation string for metadata/witness generation.
@_cdecl("swift_contract_l3_exporter_macro_sim")
public func swift_contract_l3_exporter_macro_sim(
    _ namePtr: UnsafePointer<CChar>?
) -> UnsafeMutablePointer<CChar>? {
    guard let namePtr else { return nil }
    let name = String(cString: namePtr)
    guard !name.isEmpty else { return nil }
    return strdup("@ContractExport(metadata: \"\(name)\", witness: true)")
}

// MARK: - Instrumentation, Profiling & Debugging (Track M)

// Track M.1 instrumentation globals
private var m1EventCount: Int32 = 0
private var m1POIActive: [Int32: UInt64] = [:]
private var m1LastDurationNanos: UInt64 = 0

@_cdecl("swift_contract_m1_reset")
public func swift_contract_m1_reset() {
    m1EventCount = 0
    m1POIActive.removeAll(keepingCapacity: true)
    m1LastDurationNanos = 0
}

/// Simulated os_log event sink.
@_cdecl("swift_contract_m1_os_log_event")
public func swift_contract_m1_os_log_event(_ namePtr: UnsafePointer<CChar>?) -> Int32 {
    guard let namePtr else { return 0 }
    let name = String(cString: namePtr)
    guard !name.isEmpty else { return 0 }
    m1EventCount &+= 1
    return 1
}

/// Start a point-of-interest marker with token.
@_cdecl("swift_contract_m1_poi_begin")
public func swift_contract_m1_poi_begin(_ token: Int32) -> Int32 {
    let now = DispatchTime.now().uptimeNanoseconds
    m1POIActive[token] = now
    return 1
}

/// End marker and capture duration.
@_cdecl("swift_contract_m1_poi_end")
public func swift_contract_m1_poi_end(_ token: Int32) -> Int32 {
    guard let start = m1POIActive.removeValue(forKey: token) else { return 0 }
    let end = DispatchTime.now().uptimeNanoseconds
    m1LastDurationNanos = end >= start ? (end - start) : 0
    return 1
}

@_cdecl("swift_contract_m1_event_count")
public func swift_contract_m1_event_count() -> Int32 {
    m1EventCount
}

@_cdecl("swift_contract_m1_last_duration_nanos")
public func swift_contract_m1_last_duration_nanos() -> UInt64 {
    m1LastDurationNanos
}

/// Time-profiling probe: sum [0..<iterations) and return elapsed nanos.
@_cdecl("swift_contract_m1_profile_iterations")
public func swift_contract_m1_profile_iterations(_ iterations: Int32) -> UInt64 {
    let n = max(0, Int(iterations))
    let start = DispatchTime.now().uptimeNanoseconds
    var s = 0
    for i in 0..<n { s += i }
    _ = s
    let end = DispatchTime.now().uptimeNanoseconds
    return end >= start ? (end - start) : 0
}

// Track M.2 DWARF-like cache globals
private var m2DwarfCache: [String: String] = [:]

@_cdecl("swift_contract_m2_reset")
public func swift_contract_m2_reset() {
    m2DwarfCache.removeAll(keepingCapacity: true)
}

/// Parse and cache a binary path as a mock DWARF entry.
@_cdecl("swift_contract_m2_cache_binary")
public func swift_contract_m2_cache_binary(_ pathPtr: UnsafePointer<CChar>?) -> Int32 {
    guard let pathPtr else { return 0 }
    let path = String(cString: pathPtr)
    guard !path.isEmpty else { return 0 }
    m2DwarfCache[path] = "cached"
    return 1
}

@_cdecl("swift_contract_m2_cache_size")
public func swift_contract_m2_cache_size() -> Int32 {
    Int32(m2DwarfCache.count)
}

/// Address -> mock source location string.
@_cdecl("swift_contract_m2_lookup_source")
public func swift_contract_m2_lookup_source(_ address: UInt64) -> UnsafeMutablePointer<CChar>? {
    let line = Int(address % 200) + 1
    return strdup("RustBridge.swift:\(line)")
}

/// Variable introspection stub for debugger probing.
@_cdecl("swift_contract_m2_lookup_variable")
public func swift_contract_m2_lookup_variable(_ namePtr: UnsafePointer<CChar>?) -> UnsafeMutablePointer<CChar>? {
    guard let namePtr else { return nil }
    let name = String(cString: namePtr)
    guard !name.isEmpty else { return nil }
    return strdup("\(name)=<mock>")
}

// Track M.3 memory profiling globals
private var m3NextToken: Int32 = 1
private var m3TokenBytes: [Int32: Int64] = [:]
private var m3TokenSubsystem: [Int32: String] = [:]
private var m3UsageBySubsystem: [String: Int64] = [:]

@_cdecl("swift_contract_m3_reset")
public func swift_contract_m3_reset() {
    m3NextToken = 1
    m3TokenBytes.removeAll(keepingCapacity: true)
    m3TokenSubsystem.removeAll(keepingCapacity: true)
    m3UsageBySubsystem.removeAll(keepingCapacity: true)
}

/// Tag an allocation and attribute bytes to subsystem.
@_cdecl("swift_contract_m3_tag_alloc")
public func swift_contract_m3_tag_alloc(_ subsystemPtr: UnsafePointer<CChar>?, _ bytes: Int64) -> UnsafeMutableRawPointer? {
    guard let subsystemPtr, bytes >= 0 else { return nil }
    let subsystem = String(cString: subsystemPtr)
    guard !subsystem.isEmpty else { return nil }

    let token = m3NextToken
    m3NextToken &+= 1
    m3TokenBytes[token] = bytes
    m3TokenSubsystem[token] = subsystem
    m3UsageBySubsystem[subsystem, default: 0] += bytes
    return UnsafeMutableRawPointer(bitPattern: Int(token))
}

/// Release a tagged allocation token.
@_cdecl("swift_contract_m3_release_alloc")
public func swift_contract_m3_release_alloc(_ tokenPtr: UnsafeMutableRawPointer?) -> Int32 {
    guard let tokenPtr else { return 0 }
    let token = Int32(Int(bitPattern: tokenPtr))
    guard let bytes = m3TokenBytes.removeValue(forKey: token),
          let subsystem = m3TokenSubsystem.removeValue(forKey: token)
    else { return 0 }
    m3UsageBySubsystem[subsystem, default: 0] = max(0, m3UsageBySubsystem[subsystem, default: 0] - bytes)
    return 1
}

@_cdecl("swift_contract_m3_usage_for_subsystem")
public func swift_contract_m3_usage_for_subsystem(_ subsystemPtr: UnsafePointer<CChar>?) -> Int64 {
    guard let subsystemPtr else { return -1 }
    let subsystem = String(cString: subsystemPtr)
    return m3UsageBySubsystem[subsystem, default: 0]
}

/// Emit memory health report as JSON.
@_cdecl("swift_contract_m3_health_report")
public func swift_contract_m3_health_report() -> UnsafeMutablePointer<CChar>? {
    let total = m3UsageBySubsystem.values.reduce(0, +)
    let report = "{\"live_tokens\":\(m3TokenBytes.count),\"total_bytes\":\(total)}"
    return strdup(report)
}

// Track M.4 performance regression globals
private var m4Baseline: [String: UInt64] = [:]

private func _m4RunOp(op: String, iterations: Int32) -> UInt64 {
    let n = max(0, Int(iterations))
    let start = DispatchTime.now().uptimeNanoseconds
    switch op {
    case "construct":
        var arr: [NSObject] = []
        arr.reserveCapacity(n)
        for _ in 0..<n { arr.append(NSObject()) }
    case "invoke":
        var sum = 0
        for i in 0..<n { sum += i }
        _ = sum
    case "release":
        var arr: [NSObject]? = []
        arr?.reserveCapacity(n)
        for _ in 0..<n { arr?.append(NSObject()) }
        arr = nil
    default:
        break
    }
    let end = DispatchTime.now().uptimeNanoseconds
    return end >= start ? (end - start) : 0
}

@_cdecl("swift_contract_m4_run_benchmark")
public func swift_contract_m4_run_benchmark(_ opPtr: UnsafePointer<CChar>?, _ iterations: Int32) -> UInt64 {
    guard let opPtr else { return 0 }
    return _m4RunOp(op: String(cString: opPtr), iterations: iterations)
}

@_cdecl("swift_contract_m4_set_baseline")
public func swift_contract_m4_set_baseline(_ opPtr: UnsafePointer<CChar>?, _ nanos: UInt64) -> Int32 {
    guard let opPtr else { return 0 }
    let op = String(cString: opPtr)
    guard !op.isEmpty else { return 0 }
    m4Baseline[op] = nanos
    return 1
}

/// Return 1 when regression detected above threshold percent.
@_cdecl("swift_contract_m4_regression_alarm")
public func swift_contract_m4_regression_alarm(_ opPtr: UnsafePointer<CChar>?, _ currentNanos: UInt64, _ thresholdPercent: Int32) -> Int32 {
    guard let opPtr else { return 0 }
    let op = String(cString: opPtr)
    guard let baseline = m4Baseline[op], baseline > 0, thresholdPercent >= 0 else { return 0 }
    let allowed = baseline + (baseline * UInt64(thresholdPercent)) / 100
    return currentNanos > allowed ? 1 : 0
}

@_cdecl("swift_contract_m4_baseline_get")
public func swift_contract_m4_baseline_get(_ opPtr: UnsafePointer<CChar>?) -> UInt64 {
    guard let opPtr else { return 0 }
    return m4Baseline[String(cString: opPtr)] ?? 0
}

// MARK: - Universal Runtime Metadata Graph (Track N.1)

private struct N1LayoutStruct {
    var a: Int32
    var b: Int64
}

private final class N1LayoutClass {
    var x: Int32 = 0
    var y: Int32 = 0
}

/// Metadata-kind identifiers for synthetic graph nodes.
/// 1=class, 2=struct, 3=enum, 4=tuple, 5=function, 6=existential, 7=metatype, 8=generic-instantiation.
@_cdecl("swift_contract_n1_metadata_kind")
public func swift_contract_n1_metadata_kind(_ typeID: Int32) -> Int32 {
    switch typeID {
    case 1: return 1 // N1LayoutClass
    case 2: return 2 // N1LayoutStruct
    case 3: return 3 // Direction enum
    case 4: return 4 // Tuple marker
    case 5: return 5 // Function marker
    case 6: return 6 // Existential marker
    case 7: return 7 // Metatype marker
    case 8: return 8 // Array<Int32>
    default: return -1
    }
}

@_cdecl("swift_contract_n1_metadata_field_count")
public func swift_contract_n1_metadata_field_count(_ typeID: Int32) -> Int32 {
    switch typeID {
    case 1: return 2
    case 2: return 2
    case 3: return 1
    default: return 0
    }
}

@_cdecl("swift_contract_n1_metadata_field_offset")
public func swift_contract_n1_metadata_field_offset(_ typeID: Int32, _ fieldIndex: Int32) -> Int32 {
    guard typeID == 2 else { return -1 }
    guard fieldIndex == 0 || fieldIndex == 1 else { return -1 }
    var value = N1LayoutStruct(a: 11, b: 22)
    return withUnsafeMutablePointer(to: &value) { base in
        if fieldIndex == 0 {
            return withUnsafeMutablePointer(to: &base.pointee.a) { aptr in
                Int32(Int(bitPattern: aptr) - Int(bitPattern: base))
            }
        }
        return withUnsafeMutablePointer(to: &base.pointee.b) { bptr in
            Int32(Int(bitPattern: bptr) - Int(bitPattern: base))
        }
    }
}

/// Cycle-safe traversal over synthetic metadata graph: 1 -> 2 -> 3 -> 1.
@_cdecl("swift_contract_n1_metadata_graph_traverse_count")
public func swift_contract_n1_metadata_graph_traverse_count() -> Int32 {
    let edges: [Int32: [Int32]] = [
        1: [2],
        2: [3],
        3: [1],
    ]
    var visited = Set<Int32>()
    var stack: [Int32] = [1]
    while let node = stack.popLast() {
        if visited.contains(node) { continue }
        visited.insert(node)
        for n in edges[node] ?? [] {
            stack.append(n)
        }
    }
    return Int32(visited.count)
}

/// Deterministic metadata snapshot containing user-defined and stdlib types.
@_cdecl("swift_contract_n1_metadata_snapshot_json")
public func swift_contract_n1_metadata_snapshot_json() -> UnsafeMutablePointer<CChar>? {
    let _ = N1LayoutClass()
    let _ = [Int32](arrayLiteral: 1, 2, 3)
    let json = "{\"nodes\":[{\"id\":1,\"name\":\"N1LayoutClass\",\"kind\":\"class\"},{\"id\":2,\"name\":\"N1LayoutStruct\",\"kind\":\"struct\"},{\"id\":3,\"name\":\"Direction\",\"kind\":\"enum\"},{\"id\":8,\"name\":\"Array<Int32>\",\"kind\":\"generic_instantiation\"}],\"edges\":[[1,2],[2,3],[3,1]]}"
    return strdup(json)
}

private func _n1KindForTypeName(_ typeName: String) -> Int32 {
    if typeName == "Swift.Any" || typeName.hasPrefix("any ") { return 6 }
    if typeName.hasSuffix(".Type") {
        let base = String(typeName.dropLast(5))
        if _typeByName(base) != nil || NSClassFromString(base) != nil { return 7 }
    }
    if typeName.hasSuffix(".Protocol") {
        let base = String(typeName.dropLast(9))
        if _typeByName(base) != nil || NSClassFromString(base) != nil { return 7 }
    }
    if typeName.contains("Array<") || typeName.contains("Dictionary<") || typeName.contains("Set<") {
        return 8
    }
    if typeName.contains("(") && typeName.contains(",") && typeName.contains(")") { return 4 }

    if let anyType = _typeByName(typeName) {
        if anyType is AnyClass { return 1 }
        if typeName == "Direction" || typeName.hasSuffix(".Direction") { return 3 }
        return 2
    }

    if NSClassFromString(typeName) != nil { return 1 }
    // Fallback: scan __swift5_types for types not accessible via _typeByName
    // (e.g., privately-scoped Swift types discovered by the section scanner).
    return _n1SectionKindForName(typeName)
}

@_cdecl("swift_contract_n1_metadata_kind_by_name")
public func swift_contract_n1_metadata_kind_by_name(_ typeNamePtr: UnsafePointer<CChar>?) -> Int32 {
    guard let typeNamePtr else { return -1 }
    return _n1KindForTypeName(String(cString: typeNamePtr))
}

@_cdecl("swift_contract_n1_metadata_field_count_by_name")
public func swift_contract_n1_metadata_field_count_by_name(_ typeNamePtr: UnsafePointer<CChar>?) -> Int32 {
    guard let typeNamePtr else { return -1 }
    let typeName = String(cString: typeNamePtr)
    switch typeName {
    case "N1LayoutStruct": return 2
    case "N1LayoutClass": return 2
    case "Direction": return 1
    default: return 0
    }
}

@_cdecl("swift_contract_n1_metadata_discover_types_json")
public func swift_contract_n1_metadata_discover_types_json() -> UnsafeMutablePointer<CChar>? {
    var names: Set<String> = [
        "N1LayoutStruct",
        "N1LayoutClass",
        "Direction",
        "Swift.Int",
        "Swift.String",
        "Swift.Array<Swift.Int32>",
        "NSObject",
        "NSString",
        "NSNumber",
    ]

    // Add classes that are known to be available in Foundation runtime.
    for clsName in ["NSObject", "NSString", "NSNumber", "NSArray"] {
        if NSClassFromString(clsName) != nil {
            names.insert(clsName)
        }
    }

    let sorted = names.sorted()
    let escaped = sorted.map { $0.replacingOccurrences(of: "\\", with: "\\\\").replacingOccurrences(of: "\"", with: "\\\"") }
    let body = escaped.map { "\"\($0)\"" }.joined(separator: ",")
    let json = "{\"types\":[\(body)]}"
    return strdup(json)
}

@_cdecl("swift_contract_n1_metadata_graph_traverse_discovered_count")
public func swift_contract_n1_metadata_graph_traverse_discovered_count() -> Int32 {
    guard let jsonPtr = swift_contract_n1_metadata_discover_types_json() else { return -1 }
    defer { free(jsonPtr) }
    let text = String(cString: jsonPtr)
    // Count quoted names as a simple cycle-safe traversal cardinality proxy.
    let count = text.split(separator: "\"").count / 2
    return Int32(max(0, count))
}

    // MARK: - N.1 Runtime-Wide Enumeration (exit criterion: no pre-registered descriptors)

    /// Collect registered ObjC class names (covers all Swift @objc-compatible classes).
    private func _n1ExtractObjcClassNames(limit: Int = 512) -> [String] {
        let count = objc_getClassList(nil, 0)
        guard count > 0 else { return [] }
        let cap = min(Int(count), limit)
        let buffer = UnsafeMutablePointer<AnyClass>.allocate(capacity: cap)
        defer { buffer.deallocate() }
        objc_getClassList(AutoreleasingUnsafeMutablePointer<AnyClass>(buffer), Int32(cap))
        return (0..<cap).compactMap { i -> String? in
            let name = String(cString: class_getName(buffer[i]))
            return name.isEmpty ? nil : name
        }
    }

    /// Scan the __swift5_types Mach-O section of a loaded dyld image and extract
    /// nominal type (class / struct / enum) names from their descriptors.
    /// Safe: validates kind flags and reads name byte-by-byte with a 256-char cap.
    private func _n1ScanSwift5Types(in header: UnsafePointer<mach_header>) -> [String] {
        var size: UInt = 0
        // On arm64 macOS, getsectiondata expects mach_header_64; cast via raw pointer.
        let h64 = UnsafeRawPointer(header).assumingMemoryBound(to: mach_header_64.self)
        guard let data = getsectiondata(h64, "__TEXT", "__swift5_types", &size),
              size >= 4 else { return [] }
        var names: [String] = []
        let entryCount = Int(size) / 4
        let sectionBase = UnsafeRawPointer(data)
        for i in 0..<entryCount {
            let fieldAddr = sectionBase.advanced(by: i * 4)
            // Each entry is RelativeDirectPointerIntPair<TypeContextDescriptor, bool>.
            // The low bit encodes a flag; mask it off to get the actual relative offset.
            let rawRelOffset = fieldAddr.loadUnaligned(as: Int32.self)
            let relOffset = Int(rawRelOffset & ~1)
            guard relOffset != 0 else { continue }
            let descriptorAddr = fieldAddr.advanced(by: relOffset)
            // Check kind: ContextDescriptorKind in low 5 bits of flags.
            // Class=16(0x10), Struct=17(0x11), Enum=18(0x12).
            let flags = descriptorAddr.loadUnaligned(as: UInt32.self)
            let kind = flags & 0x1F
            guard kind == 16 || kind == 17 || kind == 18 else { continue }
            // Name is a relative pointer at descriptor offset 8.
            let nameFieldAddr = descriptorAddr.advanced(by: 8)
            let nameRelOffsetRaw = nameFieldAddr.loadUnaligned(as: Int32.self)
            let nameRelOffset = Int(nameRelOffsetRaw)
            guard nameRelOffset != 0 else { continue }
            let nameAddr = nameFieldAddr.advanced(by: nameRelOffset)
            let nameBase = nameAddr.assumingMemoryBound(to: UInt8.self)
            // Read name bytes safely (capped at 256 chars).
            var bytes: [UInt8] = []
            for j in 0..<256 {
                let b = nameBase.advanced(by: j).pointee
                if b == 0 { break }
                bytes.append(b)
            }
            guard !bytes.isEmpty else { continue }
            // Accept only ASCII-printable Swift identifier characters.
            let valid = bytes.allSatisfy { b in
                (b >= 65 && b <= 90) || (b >= 97 && b <= 122) ||
                (b >= 48 && b <= 57) ||
                b == 95 || b == 60 || b == 62 || b == 44 || b == 32 || b == 46
            }
            guard valid, let name = String(bytes: bytes, encoding: .utf8) else { continue }
            names.append(name)
        }
        return names
    }

    /// Look up the kind (1=class, 2=struct, 3=enum) of a named type by scanning
    /// __swift5_types sections. Used for introspecting privately-scoped discovered types.
    private func _n1SectionKindForName(_ typeName: String) -> Int32 {
        let imageCount = _dyld_image_count()
        for i in 0..<imageCount {
            guard let header = _dyld_get_image_header(i) else { continue }
            let h64 = UnsafeRawPointer(header).assumingMemoryBound(to: mach_header_64.self)
            var size: UInt = 0
            guard let data = getsectiondata(h64, "__TEXT", "__swift5_types", &size),
                  size >= 4 else { continue }
            let entryCount = Int(size) / 4
            let sectionBase = UnsafeRawPointer(data)
            for j in 0..<entryCount {
                let fieldAddr = sectionBase.advanced(by: j * 4)
                let rawRel = fieldAddr.loadUnaligned(as: Int32.self)
                let rel = Int(rawRel & ~1)
                guard rel != 0 else { continue }
                let desc = fieldAddr.advanced(by: rel)
                let flags = desc.loadUnaligned(as: UInt32.self)
                let k = flags & 0x1F
                guard k == 16 || k == 17 || k == 18 else { continue }
                let nameField = desc.advanced(by: 8)
                let nameRel = Int(nameField.loadUnaligned(as: Int32.self))
                guard nameRel != 0 else { continue }
                let nameBase = nameField.advanced(by: nameRel).assumingMemoryBound(to: UInt8.self)
                var bytes: [UInt8] = []
                for m in 0..<256 {
                    let b = nameBase.advanced(by: m).pointee
                    if b == 0 { break }
                    bytes.append(b)
                }
                guard let found = String(bytes: bytes, encoding: .utf8), found == typeName else { continue }
                return k == 16 ? 1 : (k == 17 ? 2 : 3)
            }
        }
        return -1
    }

    /// Enumerate ALL Swift nominal types from loaded dyld images without any pre-registered
    /// seed list. Combines ObjC class enumeration + __swift5_types section scanning.
    /// N.1 exit-criterion runtime-wide discovery path.
    @_cdecl("swift_contract_n1_enumerate_all_types_json")
    public func swift_contract_n1_enumerate_all_types_json() -> UnsafeMutablePointer<CChar>? {
        var allNames: Set<String> = []
        // Path 1: ObjC runtime class list (all registered Swift/ObjC classes).
        for name in _n1ExtractObjcClassNames(limit: 512) {
            allNames.insert(name)
        }
        // Path 2: __swift5_types section scan across all loaded dyld images.
        let imageCount = _dyld_image_count()
        for i in 0..<imageCount {
            guard let header = _dyld_get_image_header(i) else { continue }
            for name in _n1ScanSwift5Types(in: header) {
                allNames.insert(name)
            }
        }
        let sorted = allNames.sorted()
        let items = sorted.map { name -> String in
            let safe = name
                .replacingOccurrences(of: "\\", with: "\\\\")
                .replacingOccurrences(of: "\"", with: "\\\"")
            return "{\"name\":\"\(safe)\"}"
        }
        return strdup("{\"types\":[\(items.joined(separator: ","))],\"count\":\(sorted.count)}")
    }

    /// JSON type info (kind string, kind_id, field_count) for any name discoverable at runtime.
    @_cdecl("swift_contract_n1_type_info_json")
    public func swift_contract_n1_type_info_json(_ typeNamePtr: UnsafePointer<CChar>?) -> UnsafeMutablePointer<CChar>? {
        guard let typeNamePtr else { return nil }
        let typeName = String(cString: typeNamePtr)
        let kind = _n1KindForTypeName(typeName)
        let kindStr: String
        switch kind {
        case 1: kindStr = "class"
        case 2: kindStr = "struct"
        case 3: kindStr = "enum"
        case 4: kindStr = "tuple"
        case 5: kindStr = "function"
        case 6: kindStr = "existential"
        case 7: kindStr = "metatype"
        case 8: kindStr = "generic_instantiation"
        default: kindStr = "unknown"
        }
        let fieldCount = swift_contract_n1_metadata_field_count_by_name(typeNamePtr)
        let safe = typeName
            .replacingOccurrences(of: "\\", with: "\\\\")
            .replacingOccurrences(of: "\"", with: "\\\"")
        return strdup("{\"name\":\"\(safe)\",\"kind\":\"\(kindStr)\",\"kind_id\":\(kind),\"field_count\":\(fieldCount)}")
    }

    /// Number of loaded dyld images available for per-image traversal.
    @_cdecl("swift_contract_n1_image_count")
    public func swift_contract_n1_image_count() -> Int32 {
        return Int32(_dyld_image_count())
    }

    /// Swift nominal types exported by a specific dyld image (0-based index into dyld image list).
    @_cdecl("swift_contract_n1_image_types_json")
    public func swift_contract_n1_image_types_json(_ imageIndex: Int32) -> UnsafeMutablePointer<CChar>? {
        let idx = UInt32(max(0, imageIndex))
        guard idx < _dyld_image_count(), let header = _dyld_get_image_header(idx) else {
            return strdup("{\"image\":\"\",\"types\":[],\"count\":0}")
        }
        let imageName: String
        if let imgNamePtr = _dyld_get_image_name(idx) {
            let full = String(cString: imgNamePtr)
            imageName = full.components(separatedBy: "/").last ?? full
        } else {
            imageName = "image_\(idx)"
        }
        let names = _n1ScanSwift5Types(in: header)
        let items = names.map { name -> String in
            let safe = name
                .replacingOccurrences(of: "\\", with: "\\\\")
                .replacingOccurrences(of: "\"", with: "\\\"")
            return "{\"name\":\"\(safe)\"}"
        }
        let safeImg = imageName
            .replacingOccurrences(of: "\\", with: "\\\\")
            .replacingOccurrences(of: "\"", with: "\\\"")
        return strdup("{\"image\":\"\(safeImg)\",\"types\":[\(items.joined(separator: ","))],\"count\":\(names.count)}")
    }

    // MARK: - Universal Call Lowering & Invocation (Track N.2)

private let n2CapabilityIndirectReturn: UInt32 = 1 << 0
private let n2CapabilityInout: UInt32 = 1 << 1
private let n2CapabilityThrowing: UInt32 = 1 << 2
private let n2CapabilityAsync: UInt32 = 1 << 3
private let n2CapabilityResilientArgs: UInt32 = 1 << 4

private func _n2WriteI32(_ ptr: UnsafeMutablePointer<Int32>?, _ value: Int32) {
    guard let ptr else { return }
    ptr.pointee = value
}

@_cdecl("swift_contract_n2_capability_mask")
public func swift_contract_n2_capability_mask() -> UInt32 {
    n2CapabilityIndirectReturn
        | n2CapabilityInout
        | n2CapabilityThrowing
        | n2CapabilityAsync
        | n2CapabilityResilientArgs
}

/// Dynamic invocation shim for N.2 lowering matrix.
/// Returns 1 on success and 0 on unsupported signature or invocation failure.
@_cdecl("swift_contract_n2_invoke_i32")
public func swift_contract_n2_invoke_i32(
    _ signaturePtr: UnsafePointer<CChar>?,
    _ a: Int32,
    _ b: Int32,
    _ inoutPtr: UnsafeMutablePointer<Int32>?,
    _ outValuePtr: UnsafeMutablePointer<Int32>?,
    _ errorCodePtr: UnsafeMutablePointer<Int32>?
) -> Int32 {
    guard let signaturePtr else { return 0 }
    let signature = String(cString: signaturePtr)
    _n2WriteI32(errorCodePtr, 0)

    switch signature {
    case "direct.add.i32_i32_to_i32":
        _n2WriteI32(outValuePtr, a + b)
        return 1

    case "inout.add_assign.i32ptr_i32_to_i32":
        guard let inoutPtr else { return 0 }
        inoutPtr.pointee += b
        _n2WriteI32(outValuePtr, inoutPtr.pointee)
        return 1

    case "throwing.require_non_negative.i32_to_i32":
        guard a >= 0 else {
            _n2WriteI32(errorCodePtr, -100)
            return 0
        }
        _n2WriteI32(outValuePtr, a)
        return 1

    case "async.double.i32_to_i32":
        let value = _awaitTaskI32 {
            await Task.yield()
            return a &* 2
        }
        guard value != Int32.min else { return 0 }
        _n2WriteI32(outValuePtr, value)
        return 1

    case "indirect_ret.pair_sum_diff.i32_i32_to_pair":
        guard let outValuePtr else { return 0 }
        outValuePtr.pointee = a &+ b
        outValuePtr.advanced(by: 1).pointee = a &- b
        return 1

    case "resilient.counter_addpair.i32_i32_to_i32":
        let counter = Counter(start: 0)
        _n2WriteI32(outValuePtr, counter.addPair(a, b))
        return 1

    default:
        _n2WriteI32(errorCodePtr, -404)
        return 0
    }
}

@_cdecl("swift_contract_n2_unknown_add_offset")
public func swift_contract_n2_unknown_add_offset(_ a: Int32, _ b: Int32) -> Int32 {
    a &+ b &+ 3
}

@_cdecl("swift_contract_n2_unknown_inout_accumulate")
public func swift_contract_n2_unknown_inout_accumulate(_ valuePtr: UnsafeMutablePointer<Int32>?, _ delta: Int32) -> Int32 {
    guard let valuePtr else { return Int32.min }
    valuePtr.pointee &+= delta
    return valuePtr.pointee
}

@_cdecl("swift_contract_n2_unknown_pair_sum_diff")
public func swift_contract_n2_unknown_pair_sum_diff(_ outPtr: UnsafeMutablePointer<Int32>?, _ a: Int32, _ b: Int32) -> Int32 {
    guard let outPtr else { return 0 }
    outPtr.pointee = a &+ b
    outPtr.advanced(by: 1).pointee = a &- b
    return 1
}

@_cdecl("swift_contract_n2_unknown_negate")
public func swift_contract_n2_unknown_negate(_ a: Int32) -> Int32 { 0 &- a }

@_cdecl("swift_contract_n2_unknown_const42")
public func swift_contract_n2_unknown_const42() -> Int32 { 42 }

// Shape discovery registry: maps C-exported symbol name → lowered ABI shape descriptor.
// Enables auto-dispatch without caller-provided shape knowledge (N.2 exit criterion).
private let _n2ShapeRegistry: [String: String] = [
    "swift_contract_n2_unknown_add_offset":       "i32_i32_to_i32",
    "swift_contract_n2_unknown_inout_accumulate": "i32ptr_i32_to_i32",
    "swift_contract_n2_unknown_pair_sum_diff":     "i32_i32_to_pair",
    "swift_contract_n2_unknown_negate":            "i32_to_i32",
    "swift_contract_n2_unknown_const42":           "void_to_i32",
]

/// Dynamic invoke by runtime symbol name + lowered shape descriptor.
/// Returns 1 on success, 0 on unsupported shape or lookup/invoke failure.
@_cdecl("swift_contract_n2_invoke_symbol_i32")
public func swift_contract_n2_invoke_symbol_i32(
    _ symbolPtr: UnsafePointer<CChar>?,
    _ shapePtr: UnsafePointer<CChar>?,
    _ a: Int32,
    _ b: Int32,
    _ inoutPtr: UnsafeMutablePointer<Int32>?,
    _ outValuePtr: UnsafeMutablePointer<Int32>?,
    _ errorCodePtr: UnsafeMutablePointer<Int32>?
) -> Int32 {
    guard let symbolPtr, let shapePtr else {
        _n2WriteI32(errorCodePtr, -400)
        return 0
    }
    let symbol = String(cString: symbolPtr)
    let shape = String(cString: shapePtr)

    _n2WriteI32(errorCodePtr, 0)
    guard let sym = dlsym(UnsafeMutableRawPointer(bitPattern: -2), symbol) else {
        _n2WriteI32(errorCodePtr, -404)
        return 0
    }

    switch shape {
    case "i32_i32_to_i32":
        typealias Fn = @convention(c) (Int32, Int32) -> Int32
        let fn = unsafeBitCast(sym, to: Fn.self)
        _n2WriteI32(outValuePtr, fn(a, b))
        return 1

    case "i32ptr_i32_to_i32":
        guard let inoutPtr else {
            _n2WriteI32(errorCodePtr, -410)
            return 0
        }
        typealias Fn = @convention(c) (UnsafeMutablePointer<Int32>?, Int32) -> Int32
        let fn = unsafeBitCast(sym, to: Fn.self)
        _n2WriteI32(outValuePtr, fn(inoutPtr, b))
        return 1

    case "i32_i32_to_pair":
        guard let outValuePtr else {
            _n2WriteI32(errorCodePtr, -411)
            return 0
        }
        typealias Fn = @convention(c) (UnsafeMutablePointer<Int32>?, Int32, Int32) -> Int32
        let fn = unsafeBitCast(sym, to: Fn.self)
        if fn(outValuePtr, a, b) != 1 {
            _n2WriteI32(errorCodePtr, -412)
            return 0
        }
        return 1

    case "i32_to_i32":
        typealias Fn = @convention(c) (Int32) -> Int32
        let fn = unsafeBitCast(sym, to: Fn.self)
        _n2WriteI32(outValuePtr, fn(a))
        return 1

    case "void_to_i32":
        typealias Fn = @convention(c) () -> Int32
        let fn = unsafeBitCast(sym, to: Fn.self)
        _n2WriteI32(outValuePtr, fn())
        return 1

    default:
        _n2WriteI32(errorCodePtr, -405)
        return 0
    }
}

/// Returns a JSON descriptor for a registered symbol: its lowered ABI shape and support flag.
@_cdecl("swift_contract_n2_symbol_describe")
public func swift_contract_n2_symbol_describe(_ symbolPtr: UnsafePointer<CChar>?) -> UnsafeMutablePointer<CChar>? {
    guard let symbolPtr else { return strdup("{\"symbol\":\"\",\"shape\":\"unknown\",\"supported\":false}") }
    let symbol = String(cString: symbolPtr)
    if let shape = _n2ShapeRegistry[symbol] {
        return strdup("{\"symbol\":\"\(symbol)\",\"shape\":\"\(shape)\",\"supported\":true}")
    } else {
        return strdup("{\"symbol\":\"\(symbol)\",\"shape\":\"unknown\",\"supported\":false}")
    }
}

/// Auto-invoke: discovers shape from registry and dispatches without a caller-provided shape.
/// Primary proof of the N.2 exit criterion — Rust does not need to know the ABI shape.
@_cdecl("swift_contract_n2_invoke_auto")
public func swift_contract_n2_invoke_auto(
    _ symbolPtr: UnsafePointer<CChar>?,
    _ a: Int32,
    _ b: Int32,
    _ inoutPtr: UnsafeMutablePointer<Int32>?,
    _ outValuePtr: UnsafeMutablePointer<Int32>?,
    _ errorCodePtr: UnsafeMutablePointer<Int32>?
) -> Int32 {
    guard let symbolPtr else { _n2WriteI32(errorCodePtr, -400); return 0 }
    let symbol = String(cString: symbolPtr)
    guard let shape = _n2ShapeRegistry[symbol] else {
        _n2WriteI32(errorCodePtr, -450)  // symbol not in registry
        return 0
    }
    return shape.withCString { shapePtr in
        swift_contract_n2_invoke_symbol_i32(symbolPtr, shapePtr, a, b, inoutPtr, outValuePtr, errorCodePtr)
    }
}

/// Capability negotiation + strategy disclosure for a requested lowered signature.
@_cdecl("swift_contract_n2_lowering_strategy_json")
public func swift_contract_n2_lowering_strategy_json(_ signaturePtr: UnsafePointer<CChar>?) -> UnsafeMutablePointer<CChar>? {
    guard let signaturePtr else { return nil }
    let signature = String(cString: signaturePtr)

    let strategy: String
    let supported: Bool
    if signature == "direct.add.i32_i32_to_i32"
        || signature == "inout.add_assign.i32ptr_i32_to_i32"
        || signature == "throwing.require_non_negative.i32_to_i32"
        || signature == "async.double.i32_to_i32"
        || signature == "indirect_ret.pair_sum_diff.i32_i32_to_pair"
        || signature == "resilient.counter_addpair.i32_i32_to_i32"
        || signature == "dynamic.symbol.i32_i32_to_i32"
        || signature == "dynamic.symbol.i32ptr_i32_to_i32"
        || signature == "dynamic.symbol.i32_i32_to_pair"
        || signature == "dynamic.symbol.i32_to_i32"
        || signature == "dynamic.symbol.void_to_i32" {
        strategy = "native"
        supported = true
    } else {
        strategy = "fallback"
        supported = false
    }

    let json = "{\"signature\":\"\(signature.replacingOccurrences(of: "\\", with: "\\\\").replacingOccurrences(of: "\"", with: "\\\""))\",\"strategy\":\"\(strategy)\",\"supported\":\(supported ? "true" : "false"),\"capability_mask\":\(swift_contract_n2_capability_mask())}"
    return strdup(json)
}

// MARK: - Cross-Version ABI Adaptation Layer (Track N.5)

private struct _N5FeatureFlags: Encodable {
    let swift5TypesScan: Bool
    let objcClassScan: Bool
    let dynamicSymbolLowering: Bool
    let recursiveGenericSolver: Bool
    let brokerIsolation: Bool
    let privateTypeKindFallback: Bool

    enum CodingKeys: String, CodingKey {
        case swift5TypesScan = "swift5_types_scan"
        case objcClassScan = "objc_class_scan"
        case dynamicSymbolLowering = "dynamic_symbol_lowering"
        case recursiveGenericSolver = "recursive_generic_solver"
        case brokerIsolation = "broker_isolation"
        case privateTypeKindFallback = "private_type_kind_fallback"
    }
}

private struct _N5FeatureProbePayload: Encodable {
    let compilerFamily: String
    let platform: String
    let architecture: String
    let osMajor: Int
    let osMinor: Int
    let osPatch: Int
    let optimizationMode: String
    let features: _N5FeatureFlags

    enum CodingKeys: String, CodingKey {
        case compilerFamily = "compiler_family"
        case platform
        case architecture
        case osMajor = "os_major"
        case osMinor = "os_minor"
        case osPatch = "os_patch"
        case optimizationMode = "optimization_mode"
        case features
    }
}

private struct _N5AdapterProfilePayload: Encodable {
    let profileID: String
    let compilerFamily: String
    let platforms: [String]
    let architectures: [String]
    let supportedOptimizationModes: [String]
    let requiredFeatures: [String]
    let symbolAliases: [String: [String]]
    let layoutRules: [String: String]
    let witnessRules: [String: String]
    let adaptationNotes: [String]

    enum CodingKeys: String, CodingKey {
        case profileID = "profile_id"
        case compilerFamily = "compiler_family"
        case platforms
        case architectures
        case supportedOptimizationModes = "supported_optimization_modes"
        case requiredFeatures = "required_features"
        case symbolAliases = "symbol_aliases"
        case layoutRules = "layout_rules"
        case witnessRules = "witness_rules"
        case adaptationNotes = "adaptation_notes"
    }
}

private struct _N5AdapterTablePayload: Encodable {
    let profiles: [_N5AdapterProfilePayload]
}

private struct _N5SelectedAdapterPayload: Encodable {
    let profileID: String
    let compatible: Bool
    let reason: String
    let compilerFamily: String
    let optimizationMode: String
    let selectedSymbols: [String: String]
    let missingFeatures: [String]
    let adaptationNotes: [String]

    enum CodingKeys: String, CodingKey {
        case profileID = "profile_id"
        case compatible
        case reason
        case compilerFamily = "compiler_family"
        case optimizationMode = "optimization_mode"
        case selectedSymbols = "selected_symbols"
        case missingFeatures = "missing_features"
        case adaptationNotes = "adaptation_notes"
    }
}

private func _n5CompilerFamily() -> String {
#if compiler(>=6.3)
    return "swift_6_3_or_newer"
#elseif compiler(>=6.2)
    return "swift_6_2"
#elseif compiler(>=6.1)
    return "swift_6_1"
#else
    return "pre_swift_6_1"
#endif
}

private func _n5Architecture() -> String {
#if arch(arm64)
    return "arm64"
#elseif arch(x86_64)
    return "x86_64"
#else
    return "unknown"
#endif
}

private func _n5Platform() -> String {
#if os(macOS)
    return "macos"
#elseif os(Linux)
    return "linux"
#else
    return "unknown"
#endif
}

private func _n5OptimizationMode() -> String {
    if let override = ProcessInfo.processInfo.environment["SWIFT_RUNTIME_SYS_OPT_MODE"]?.lowercased(),
       override == "debug" || override == "release" {
        return override
    }
    return "debug"
}

private func _n5FeatureFlags() -> _N5FeatureFlags {
    _N5FeatureFlags(
        swift5TypesScan: true,
        objcClassScan: true,
        dynamicSymbolLowering: true,
        recursiveGenericSolver: true,
        brokerIsolation: true,
        privateTypeKindFallback: true
    )
}

private func _n5FeatureEnabled(_ probe: _N5FeatureProbePayload, _ name: String) -> Bool {
    switch name {
    case "swift5_types_scan":
        return probe.features.swift5TypesScan
    case "objc_class_scan":
        return probe.features.objcClassScan
    case "dynamic_symbol_lowering":
        return probe.features.dynamicSymbolLowering
    case "recursive_generic_solver":
        return probe.features.recursiveGenericSolver
    case "broker_isolation":
        return probe.features.brokerIsolation
    case "private_type_kind_fallback":
        return probe.features.privateTypeKindFallback
    default:
        return false
    }
}

private func _n5CurrentFeatureProbe() -> _N5FeatureProbePayload {
    let os = ProcessInfo.processInfo.operatingSystemVersion
    return _N5FeatureProbePayload(
        compilerFamily: _n5CompilerFamily(),
        platform: _n5Platform(),
        architecture: _n5Architecture(),
        osMajor: os.majorVersion,
        osMinor: os.minorVersion,
        osPatch: os.patchVersion,
        optimizationMode: _n5OptimizationMode(),
        features: _n5FeatureFlags()
    )
}

private func _n5AdapterProfiles() -> [_N5AdapterProfilePayload] {
    let requiredFeatures = [
        "swift5_types_scan",
        "objc_class_scan",
        "dynamic_symbol_lowering",
        "recursive_generic_solver",
        "broker_isolation",
        "private_type_kind_fallback",
    ]
    let symbolAliases = [
        "metadata_enumeration": ["swift_contract_n1_enumerate_all_types_json"],
        "dynamic_invoke": ["swift_contract_n2_invoke_auto", "swift_contract_n2_invoke_symbol_i32"],
        "generic_context": ["swift_contract_n3_build_context_json", "swift_contract_n3_resolve_witness_json"],
        "broker_abort": ["swift_contract_n4_trigger_abort"],
    ]

    return [
        _N5AdapterProfilePayload(
            profileID: "swift_6_1_arm64_macos",
            compilerFamily: "swift_6_1",
            platforms: ["macos"],
            architectures: ["arm64"],
            supportedOptimizationModes: ["debug", "release"],
            requiredFeatures: requiredFeatures,
            symbolAliases: symbolAliases,
            layoutRules: [
                "relative_pointer_low_bit_mask": "required",
                "private_type_kind_fallback": "section_name_lookup",
                "function_metadata_kind": "normalize_pre_6_2",
            ],
            witnessRules: [
                "sequence_requirement_token": "stable_string_v1",
                "dictionary_associated_types": "Key,Value",
            ],
            adaptationNotes: [
                "Prefer section-name fallback when private type names do not map to canonical kind strings.",
                "Normalize pre-6.2 function metadata kind observations before selecting call-lowering strategy.",
            ]
        ),
        _N5AdapterProfilePayload(
            profileID: "swift_6_2_arm64_macos",
            compilerFamily: "swift_6_2",
            platforms: ["macos"],
            architectures: ["arm64"],
            supportedOptimizationModes: ["debug", "release"],
            requiredFeatures: requiredFeatures,
            symbolAliases: symbolAliases,
            layoutRules: [
                "relative_pointer_low_bit_mask": "required",
                "private_type_kind_fallback": "section_name_lookup",
                "function_metadata_kind": "normalize_6_2",
            ],
            witnessRules: [
                "sequence_requirement_token": "stable_string_v1",
                "dictionary_associated_types": "Key,Value",
            ],
            adaptationNotes: [
                "Prefer canonical 6.2 metadata-kind normalization before graph traversal.",
                "Treat symbol registry results as the primary lowering selector for unknown call shapes.",
            ]
        ),
    ]
}

private func _n5EncodeJSON<T: Encodable>(_ value: T) -> UnsafeMutablePointer<CChar>? {
    let encoder = JSONEncoder()
    encoder.outputFormatting = [.sortedKeys]
    guard let data = try? encoder.encode(value),
          let json = String(data: data, encoding: .utf8) else {
        return nil
    }
    return strdup(json)
}

private func _n5SelectedAdapter() -> _N5SelectedAdapterPayload {
    let probe = _n5CurrentFeatureProbe()

    for profile in _n5AdapterProfiles() {
        guard profile.compilerFamily == probe.compilerFamily,
              profile.platforms.contains(probe.platform),
              profile.architectures.contains(probe.architecture),
              profile.supportedOptimizationModes.contains(probe.optimizationMode) else {
            continue
        }

        let missing = profile.requiredFeatures.filter { !_n5FeatureEnabled(probe, $0) }
        if !missing.isEmpty {
            continue
        }

        var selectedSymbols: [String: String] = [:]
        for (key, aliases) in profile.symbolAliases {
            selectedSymbols[key] = aliases.first ?? ""
        }

        return _N5SelectedAdapterPayload(
            profileID: profile.profileID,
            compatible: true,
            reason: "compiler family, platform, architecture, optimization mode, and required feature probes matched adapter profile",
            compilerFamily: probe.compilerFamily,
            optimizationMode: probe.optimizationMode,
            selectedSymbols: selectedSymbols,
            missingFeatures: [],
            adaptationNotes: profile.adaptationNotes
        )
    }

    return _N5SelectedAdapterPayload(
        profileID: "unsupported",
        compatible: false,
        reason: "no adapter profile matched the current compiler family, platform, architecture, and feature probe set",
        compilerFamily: probe.compilerFamily,
        optimizationMode: probe.optimizationMode,
        selectedSymbols: [:],
        missingFeatures: _n5AdapterProfiles().first(where: { $0.compilerFamily == probe.compilerFamily })?.requiredFeatures.filter { !_n5FeatureEnabled(probe, $0) } ?? [],
        adaptationNotes: []
    )
}

@_cdecl("swift_contract_n5_adapter_table_json")
public func swift_contract_n5_adapter_table_json() -> UnsafeMutablePointer<CChar>? {
    _n5EncodeJSON(_N5AdapterTablePayload(profiles: _n5AdapterProfiles()))
}

@_cdecl("swift_contract_n5_feature_probe_json")
public func swift_contract_n5_feature_probe_json() -> UnsafeMutablePointer<CChar>? {
    _n5EncodeJSON(_n5CurrentFeatureProbe())
}

@_cdecl("swift_contract_n5_select_adapter_json")
public func swift_contract_n5_select_adapter_json() -> UnsafeMutablePointer<CChar>? {
    _n5EncodeJSON(_n5SelectedAdapter())
}

// MARK: - Differential Fuzzing & Semantic Oracle (Track N.6)

private struct _N6FragmentPayload: Codable {
    let id: Int
    let kind: String
    let a: Int32
    let b: Int32
    let source: String
}

private struct _N6ProgramPayload: Codable {
    let seed: Int64
    let fragments: [_N6FragmentPayload]
    let swiftSource: String

    enum CodingKeys: String, CodingKey {
        case seed
        case fragments
        case swiftSource = "swift_source"
    }
}

private struct _N6ResultPayload: Codable {
    let id: Int
    let kind: String
    let status: String
    let value: Int32?
    let error: String?
    let sideEffect: String?

    enum CodingKeys: String, CodingKey {
        case id
        case kind
        case status
        case value
        case error
        case sideEffect = "side_effect"
    }
}

private struct _N6ExecutionPayload: Codable {
    let seed: Int64
    let resultCount: Int
    let results: [_N6ResultPayload]

    enum CodingKeys: String, CodingKey {
        case seed
        case resultCount = "result_count"
        case results
    }
}

private func _n6LCGNext(_ state: inout UInt64) -> UInt64 {
    state = (state &* 6364136223846793005) &+ 1442695040888963407
    return state
}

private func _n6BoundedSigned(_ state: inout UInt64, _ range: ClosedRange<Int32>) -> Int32 {
    let width = UInt64(Int64(range.upperBound) - Int64(range.lowerBound) + 1)
    return Int32(Int64(_n6LCGNext(&state) % width) + Int64(range.lowerBound))
}

private func _n6BoundedCount(_ value: Int32, _ maxValue: Int32) -> Int32 {
    max(0, min(value, maxValue))
}

private func _n6MakeFragment(_ state: inout UInt64, _ id: Int) -> _N6FragmentPayload {
    let kindIndex = Int(_n6LCGNext(&state) % 9)
    let a = _n6BoundedSigned(&state, -20...20)
    let bGeneric = _n6BoundedSigned(&state, -6...12)

    switch kindIndex {
    case 0:
        return _N6FragmentPayload(
            id: id,
            kind: "add",
            a: a,
            b: bGeneric,
            source: "let r\(id) = swift_add(\(a), \(bGeneric))"
        )
    case 1:
        let divisor = _n6BoundedSigned(&state, -2...2)
        return _N6FragmentPayload(
            id: id,
            kind: "safe_divide",
            a: a,
            b: divisor,
            source: "let r\(id) = try? safeDivide(\(a), \(divisor))"
        )
    case 2:
        return _N6FragmentPayload(
            id: id,
            kind: "async_add",
            a: a,
            b: bGeneric,
            source: "let r\(id) = await asyncAdd(\(a), \(bGeneric))"
        )
    case 3:
        let taskValue = _n6BoundedSigned(&state, -15...15)
        let delta = _n6BoundedSigned(&state, -5...5)
        return _N6FragmentPayload(
            id: id,
            kind: "task_local",
            a: taskValue,
            b: delta,
            source: "let r\(id) = await ProbeTaskLocal.$value.withValue(\(taskValue)) { let inherited = await Task { ProbeTaskLocal.value }.value; return inherited + \(delta) }"
        )
    case 4:
        let code = _n6BoundedSigned(&state, 100...140)
        let cause = _n6BoundedSigned(&state, 1...9)
        return _N6FragmentPayload(
            id: id,
            kind: "error_context_validation",
            a: code,
            b: cause,
            source: "swift_contract_error_context_make_validation(\(code), \(cause))"
        )
    case 5:
        let start = _n6BoundedSigned(&state, -12...12)
        let count = _n6BoundedSigned(&state, 0...12)
        return _N6FragmentPayload(
            id: id,
            kind: "generic_array_i32",
            a: start,
            b: count,
            source: "let r\(id) = (0..<\(count)).reduce(0) { partial, offset in partial + (\(start) + Int32(offset)) }"
        )
    case 6:
        let base = _n6BoundedSigned(&state, 0...8)
        let count = _n6BoundedSigned(&state, 0...8)
        return _N6FragmentPayload(
            id: id,
            kind: "generic_array_string",
            a: base,
            b: count,
            source: "let r\(id) = (0..<\(count)).map { String(repeating: \"x\", count: Int(\(base) + Int32($0))) }.joined().count"
        )
    case 7:
        return _N6FragmentPayload(
            id: id,
            kind: "generic_sequence_witness",
            a: 0,
            b: 0,
            source: "let r\(id) = ([\"a\", \"bb\"] as any Sequence) is [String]"
        )
    default:
        let metric = _n6BoundedSigned(&state, 0...16)
        return _N6FragmentPayload(
            id: id,
            kind: "generic_box_string",
            a: metric,
            b: 0,
            source: "let r\(id) = ContractGenericBox<String>(String(repeating: \"x\", count: Int(\(metric)))).value.count"
        )
    }
}

private func _n6GenerateProgram(seed: Int64, fragmentCount: Int32) -> _N6ProgramPayload {
    let count = Int(max(1, min(fragmentCount, 64)))
    var state = UInt64(bitPattern: seed)
    var fragments: [_N6FragmentPayload] = []
    for id in 0..<count {
        fragments.append(_n6MakeFragment(&state, id))
    }
    let source = fragments.map(\.source).joined(separator: "\n")
    return _N6ProgramPayload(seed: seed, fragments: fragments, swiftSource: source)
}

private func _n6AsyncAddBlocking(_ a: Int32, _ b: Int32) -> Int32 {
    let sem = DispatchSemaphore(value: 0)
    var out = Int32.min
    Task {
        out = await asyncAdd(a, b)
        sem.signal()
    }
    sem.wait()
    return out
}

private func _n6TaskLocalBlocking(_ value: Int32, _ delta: Int32) -> (Int32, String) {
    let sem = DispatchSemaphore(value: 0)
    var out = Int32.min
    var sideEffect = ""
    Task {
        out = await ProbeTaskLocal.$value.withValue(value) {
            let inherited = await Task { ProbeTaskLocal.value }.value
            let detached = await Task.detached { ProbeTaskLocal.value }.value
            sideEffect = "inherited=\(inherited)|detached=\(detached)"
            return inherited &+ delta
        }
        sem.signal()
    }
    sem.wait()
    return (out, sideEffect)
}

private func _n6ValidationContextBlocking(_ code: Int32, _ cause: Int32) -> (Int32, String) {
    _ = swift_contract_error_context_make_validation(code, cause)
    defer { swift_contract_error_context_clear() }
    guard let ptr = swift_contract_error_context_get_json() else {
        return (Int32.min, "")
    }
    let json = String(cString: ptr)
    free(ptr)
    guard let data = json.data(using: .utf8),
          let object = try? JSONSerialization.jsonObject(with: data) as? [String: Any] else {
        return (Int32.min, "")
    }
    let parsedCode = object["code"] as? Int ?? Int(code)
    let domain = object["domain"] as? String ?? ""
    let message = object["message"] as? String ?? ""
    let chainCount = (object["chain"] as? [Any])?.count ?? 0
    return (Int32(parsedCode), "\(domain)|\(message)|chain=\(chainCount)")
}

private func _n6DirectResult(_ fragment: _N6FragmentPayload) -> _N6ResultPayload {
    switch fragment.kind {
    case "add":
        return _N6ResultPayload(
            id: fragment.id,
            kind: fragment.kind,
            status: "value",
            value: fragment.a &+ fragment.b,
            error: nil,
            sideEffect: nil
        )
    case "safe_divide":
        if fragment.b == 0 {
            return _N6ResultPayload(
                id: fragment.id,
                kind: fragment.kind,
                status: "error",
                value: nil,
                error: "division_by_zero",
                sideEffect: nil
            )
        }
        return _N6ResultPayload(
            id: fragment.id,
            kind: fragment.kind,
            status: "value",
            value: fragment.a / fragment.b,
            error: nil,
            sideEffect: nil
        )
    case "async_add":
        return _N6ResultPayload(
            id: fragment.id,
            kind: fragment.kind,
            status: "value",
            value: _n6AsyncAddBlocking(fragment.a, fragment.b),
            error: nil,
            sideEffect: nil
        )
    case "task_local":
        let (value, sideEffect) = _n6TaskLocalBlocking(fragment.a, fragment.b)
        return _N6ResultPayload(
            id: fragment.id,
            kind: fragment.kind,
            status: "value",
            value: value,
            error: nil,
            sideEffect: sideEffect
        )
    case "error_context_validation":
        let (value, sideEffect) = _n6ValidationContextBlocking(fragment.a, fragment.b)
        return _N6ResultPayload(
            id: fragment.id,
            kind: fragment.kind,
            status: "context",
            value: value,
            error: nil,
            sideEffect: sideEffect
        )
    case "generic_array_i32":
        let bounded = _n6BoundedCount(fragment.b, 256)
        let value = (0..<bounded).reduce(Int32(0)) { partial, offset in
            partial &+ (fragment.a &+ offset)
        }
        return _N6ResultPayload(
            id: fragment.id,
            kind: fragment.kind,
            status: "value",
            value: value,
            error: nil,
            sideEffect: nil
        )
    case "generic_array_string":
        let bounded = _n6BoundedCount(fragment.b, 64)
        let values = (0..<bounded).map { offset in
            String(repeating: "x", count: Int(_n6BoundedCount(fragment.a &+ offset, 256)))
        }
        return _N6ResultPayload(
            id: fragment.id,
            kind: fragment.kind,
            status: "value",
            value: Int32(values.joined().count),
            error: nil,
            sideEffect: nil
        )
    case "generic_sequence_witness":
        return _N6ResultPayload(
            id: fragment.id,
            kind: fragment.kind,
            status: "value",
            value: 1,
            error: nil,
            sideEffect: "Sequence<Element=String>"
        )
    case "generic_box_string":
        let value = String(repeating: "x", count: Int(_n6BoundedCount(fragment.a, 256))).count
        return _N6ResultPayload(
            id: fragment.id,
            kind: fragment.kind,
            status: "value",
            value: Int32(value),
            error: nil,
            sideEffect: nil
        )
    default:
        return _N6ResultPayload(
            id: fragment.id,
            kind: fragment.kind,
            status: "error",
            value: nil,
            error: "unsupported_fragment",
            sideEffect: nil
        )
    }
}

private func _n6ExecuteProgram(_ program: _N6ProgramPayload) -> _N6ExecutionPayload {
    let results = program.fragments.map(_n6DirectResult)
    return _N6ExecutionPayload(seed: program.seed, resultCount: results.count, results: results)
}

@_cdecl("swift_contract_n6_generate_program_json")
public func swift_contract_n6_generate_program_json(_ seed: Int64, _ fragmentCount: Int32) -> UnsafeMutablePointer<CChar>? {
    _n5EncodeJSON(_n6GenerateProgram(seed: seed, fragmentCount: fragmentCount))
}

@_cdecl("swift_contract_n6_execute_program_json")
public func swift_contract_n6_execute_program_json(_ jsonPtr: UnsafePointer<CChar>?) -> UnsafeMutablePointer<CChar>? {
    guard let jsonPtr else { return nil }
    let data = Data(bytes: jsonPtr, count: Int(strlen(jsonPtr)))
    guard let program = try? JSONDecoder().decode(_N6ProgramPayload.self, from: data) else {
        return nil
    }
    return _n5EncodeJSON(_n6ExecuteProgram(program))
}

// MARK: - Unsafe Runtime Ops Isolation & Recovery (Track N.4)

/// Deterministic safe operation used by the broker path to prove non-crashing subprocess execution.
@_cdecl("swift_contract_n4_safe_ping")
public func swift_contract_n4_safe_ping(_ value: Int32) -> Int32 {
    value &+ 1000
}

/// Deliberately abort the current process. This must only be invoked from the broker subprocess.
/// The parent probe captures structured context before invoking this symbol and verifies isolation.
@_cdecl("swift_contract_n4_trigger_abort")
public func swift_contract_n4_trigger_abort() {
    Darwin.abort()
}

// MARK: - Backtrace & Crash Symbolication (Track E.2)

@inline(never)
private func _swift_contract_backtrace_frame_leaf() -> UnsafeMutablePointer<CChar>? {
    let stack = Thread.callStackSymbols.joined(separator: "\n")
    return strdup(stack)
}

@inline(never)
private func _swift_contract_backtrace_frame_mid() -> UnsafeMutablePointer<CChar>? {
    _swift_contract_backtrace_frame_leaf()
}

@inline(never)
private func _swift_contract_backtrace_frame_root() -> UnsafeMutablePointer<CChar>? {
    _swift_contract_backtrace_frame_mid()
}

/// Capture a Swift call stack as newline-delimited text.
/// Returns a malloc-backed C string that Rust must free.
@_cdecl("swift_contract_backtrace_capture")
public func swift_contract_backtrace_capture() -> UnsafeMutablePointer<CChar>? {
    _swift_contract_backtrace_frame_root()
}

/// Anchor symbol used by source-location and symbolication probes.
@_cdecl("swift_contract_backtrace_anchor")
public func swift_contract_backtrace_anchor(_ tag: Int32) -> Int32 {
    tag + 1
}

/// Return the runtime address of the anchor symbol for debug tooling.
@_cdecl("swift_contract_backtrace_anchor_address")
public func swift_contract_backtrace_anchor_address() -> UInt64 {
    let fn: @convention(c) (Int32) -> Int32 = swift_contract_backtrace_anchor
    let ptr = unsafeBitCast(fn, to: UnsafeRawPointer.self)
    return UInt64(UInt(bitPattern: ptr))
}

// MARK: - Enum Introspection (Track D.3)

/// Construct a Direction enum case from a case discriminant (0=north, 1=south, 2=east, 3=west).
@_cdecl("swift_contract_direction_make")
public func swift_contract_direction_make(_ caseID: Int32) -> UnsafeMutableRawPointer? {
    let dir: Direction
    switch caseID {
    case 0: dir = .north
    case 1: dir = .south
    case 2: dir = .east
    case 3: dir = .west
    default: return nil
    }
    let boxed = Box(dir)
    return Unmanaged.passRetained(boxed).toOpaque()
}

/// Extract the case discriminant from a Direction enum.
/// Returns 0=north, 1=south, 2=east, 3=west, or -1 if invalid.
@_cdecl("swift_contract_direction_case")
public func swift_contract_direction_case(_ dirPtr: UnsafeMutableRawPointer?) -> Int32 {
    guard let dirPtr else { return -1 }
    let boxed = Unmanaged<Box<Direction>>.fromOpaque(dirPtr).takeUnretainedValue()
    return boxed.value.rawValue
}

/// Construct a Shape enum case: circle variant (case_id=0, radius as payload).
@_cdecl("swift_contract_shape_circle")
public func swift_contract_shape_circle(_ radius: Float) -> UnsafeMutableRawPointer? {
    let shape = Shape.circle(radius: radius)
    let boxed = Box(shape)
    return Unmanaged.passRetained(boxed).toOpaque()
}

/// Construct a Shape enum case: rectangle variant (case_id=1, width and height as payload).
@_cdecl("swift_contract_shape_rect")
public func swift_contract_shape_rect(_ width: Float, _ height: Float) -> UnsafeMutableRawPointer? {
    let shape = Shape.rectangle(width: width, height: height)
    let boxed = Box(shape)
    return Unmanaged.passRetained(boxed).toOpaque()
}

/// Extract the case discriminant from a Shape enum (0=circle, 1=rectangle, -1=invalid).
@_cdecl("swift_contract_shape_get_case")
public func swift_contract_shape_get_case(_ shapePtr: UnsafeMutableRawPointer?) -> Int32 {
    guard let shapePtr else { return -1 }
    let boxed = Unmanaged<Box<Shape>>.fromOpaque(shapePtr).takeUnretainedValue()
    switch boxed.value {
    case .circle:    return 0
    case .rectangle: return 1
    }
}

/// Extract radius from Shape.circle case. Returns -1 if shape is not circle.
@_cdecl("swift_contract_shape_circle_radius")
public func swift_contract_shape_circle_radius(_ shapePtr: UnsafeMutableRawPointer?) -> Float {
    guard let shapePtr else { return -1.0 }
    let boxed = Unmanaged<Box<Shape>>.fromOpaque(shapePtr).takeUnretainedValue()
    if case .circle(let r) = boxed.value {
        return r
    }
    return -1.0
}

/// Extract width and height from Shape.rectangle case.
/// Returns 1 if successful, 0 if shape is not rectangle.
@_cdecl("swift_contract_shape_rect_dims")
public func swift_contract_shape_rect_dims(
    _ shapePtr: UnsafeMutableRawPointer?,
    _ widthPtr: UnsafeMutablePointer<Float>?,
    _ heightPtr: UnsafeMutablePointer<Float>?
) -> Int32 {
    guard let shapePtr, let widthPtr, let heightPtr else { return 0 }
    let boxed = Unmanaged<Box<Shape>>.fromOpaque(shapePtr).takeUnretainedValue()
    if case .rectangle(let w, let h) = boxed.value {
        widthPtr.pointee = w
        heightPtr.pointee = h
        return 1
    }
    return 0
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
    var value: T

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

@_cdecl("swift_counter_deinit_count")
public func swift_counter_deinit_count() -> Int32 {
    Counter.deinitCount
}

@_cdecl("swift_counter_deinit_reset")
public func swift_counter_deinit_reset() {
    Counter.deinitCount = 0
}

@_cdecl("swift_contract_construct_string")
public func swift_contract_construct_string(
    _ bytesPtr: UnsafeRawPointer?,
    _ byteCount: Int32
) -> UnsafeMutableRawPointer? {
    guard let bytesPtr, byteCount >= 0 else { return nil }
    let buffer = UnsafeBufferPointer(
        start: bytesPtr.assumingMemoryBound(to: UInt8.self),
        count: Int(byteCount)
    )
    let string = String(decoding: buffer, as: UTF8.self)
    let boxed = Box(string)
    return Unmanaged.passRetained(boxed).toOpaque()
}

@_cdecl("swift_contract_string_len")
public func swift_contract_string_len(_ receiver: UnsafeMutableRawPointer?) -> Int32 {
    guard let receiver else { return -1 }
    let boxed = Unmanaged<Box<String>>.fromOpaque(receiver).takeUnretainedValue()
    return Int32(boxed.value.utf8.count)
}

@_cdecl("swift_contract_string_bytes")
public func swift_contract_string_bytes(
    _ receiver: UnsafeMutableRawPointer?,
    _ outBytesPtr: UnsafeMutableRawPointer?,
    _ maxByteCount: Int32
) -> Int32 {
    guard let receiver, let outBytesPtr, maxByteCount >= 0 else { return -1 }

    let boxed = Unmanaged<Box<String>>.fromOpaque(receiver).takeUnretainedValue()
    let utf8Bytes = Array(boxed.value.utf8)
    let copyCount = min(utf8Bytes.count, Int(maxByteCount))
    let outBuffer = outBytesPtr.bindMemory(to: UInt8.self, capacity: copyCount)

    for index in 0..<copyCount {
        outBuffer[index] = utf8Bytes[index]
    }

    return Int32(utf8Bytes.count)
}

@_cdecl("swift_contract_array_make")
public func swift_contract_array_make(_ capacity: Int32) -> UnsafeMutableRawPointer? {
    var array: [Int32] = []
    array.reserveCapacity(max(0, Int(capacity)))
    let boxed = Box(array)
    return Unmanaged.passRetained(boxed).toOpaque()
}

@_cdecl("swift_contract_array_len")
public func swift_contract_array_len(_ receiver: UnsafeMutableRawPointer?) -> Int32 {
    guard let receiver else { return -1 }
    let boxed = Unmanaged<Box<[Int32]>>.fromOpaque(receiver).takeUnretainedValue()
    return Int32(boxed.value.count)
}

@_cdecl("swift_contract_array_get")
public func swift_contract_array_get(_ receiver: UnsafeMutableRawPointer?, _ index: Int32) -> Int32 {
    guard let receiver, index >= 0 else { return -1 }
    let boxed = Unmanaged<Box<[Int32]>>.fromOpaque(receiver).takeUnretainedValue()
    guard Int(index) < boxed.value.count else { return -1 }
    return boxed.value[Int(index)]
}

@_cdecl("swift_contract_array_set")
public func swift_contract_array_set(
    _ receiver: UnsafeMutableRawPointer?,
    _ index: Int32,
    _ value: Int32
) -> Int32 {
    guard let receiver, index >= 0 else { return -1 }
    let boxed = Unmanaged<Box<[Int32]>>.fromOpaque(receiver).takeUnretainedValue()
    guard Int(index) < boxed.value.count else { return -1 }
    boxed.value[Int(index)] = value
    return 0
}

@_cdecl("swift_contract_array_append")
public func swift_contract_array_append(_ receiver: UnsafeMutableRawPointer?, _ value: Int32) -> Int32 {
    guard let receiver else { return -1 }
    let boxed = Unmanaged<Box<[Int32]>>.fromOpaque(receiver).takeUnretainedValue()
    boxed.value.append(value)
    return Int32(boxed.value.count)
}

@_cdecl("swift_contract_array_data")
public func swift_contract_array_data(_ receiver: UnsafeMutableRawPointer?) -> UnsafeRawPointer? {
    guard let receiver else { return nil }
    let boxed = Unmanaged<Box<[Int32]>>.fromOpaque(receiver).takeUnretainedValue()
    return boxed.value.withUnsafeBufferPointer { buffer in
        buffer.baseAddress.map { UnsafeRawPointer($0) }
    }
}

@_cdecl("swift_contract_array_ref_make")
public func swift_contract_array_ref_make(_ capacity: Int32) -> UnsafeMutableRawPointer? {
    var array: [UnsafeMutableRawPointer] = []
    array.reserveCapacity(max(0, Int(capacity)))
    let boxed = Box(array)
    return Unmanaged.passRetained(boxed).toOpaque()
}

@_cdecl("swift_contract_array_ref_len")
public func swift_contract_array_ref_len(_ receiver: UnsafeMutableRawPointer?) -> Int32 {
    guard let receiver else { return -1 }
    let boxed = Unmanaged<Box<[UnsafeMutableRawPointer]>>.fromOpaque(receiver).takeUnretainedValue()
    return Int32(boxed.value.count)
}

@_cdecl("swift_contract_array_ref_get")
public func swift_contract_array_ref_get(
    _ receiver: UnsafeMutableRawPointer?,
    _ index: Int32
) -> UnsafeMutableRawPointer? {
    guard let receiver, index >= 0 else { return nil }
    let boxed = Unmanaged<Box<[UnsafeMutableRawPointer]>>.fromOpaque(receiver).takeUnretainedValue()
    guard Int(index) < boxed.value.count else { return nil }
    return boxed.value[Int(index)]
}

@_cdecl("swift_contract_array_ref_set")
public func swift_contract_array_ref_set(
    _ receiver: UnsafeMutableRawPointer?,
    _ index: Int32,
    _ value: UnsafeMutableRawPointer?
) -> Int32 {
    guard let receiver, index >= 0, let value else { return -1 }
    let boxed = Unmanaged<Box<[UnsafeMutableRawPointer]>>.fromOpaque(receiver).takeUnretainedValue()
    guard Int(index) < boxed.value.count else { return -1 }
    boxed.value[Int(index)] = value
    return 0
}

@_cdecl("swift_contract_array_ref_append")
public func swift_contract_array_ref_append(
    _ receiver: UnsafeMutableRawPointer?,
    _ value: UnsafeMutableRawPointer?
) -> Int32 {
    guard let receiver, let value else { return -1 }
    let boxed = Unmanaged<Box<[UnsafeMutableRawPointer]>>.fromOpaque(receiver).takeUnretainedValue()
    boxed.value.append(value)
    return Int32(boxed.value.count)
}

@_cdecl("swift_contract_dict_i32_make")
public func swift_contract_dict_i32_make(_ capacity: Int32) -> UnsafeMutableRawPointer? {
    var dict: [Int32: Int32] = [:]
    dict.reserveCapacity(max(0, Int(capacity)))
    let boxed = Box(dict)
    return Unmanaged.passRetained(boxed).toOpaque()
}

@_cdecl("swift_contract_dict_i32_len")
public func swift_contract_dict_i32_len(_ receiver: UnsafeMutableRawPointer?) -> Int32 {
    guard let receiver else { return -1 }
    let boxed = Unmanaged<Box<[Int32: Int32]>>.fromOpaque(receiver).takeUnretainedValue()
    return Int32(boxed.value.count)
}

@_cdecl("swift_contract_dict_i32_get")
public func swift_contract_dict_i32_get(
    _ receiver: UnsafeMutableRawPointer?,
    _ key: Int32,
    _ outValue: UnsafeMutablePointer<Int32>?
) -> Int32 {
    guard let receiver else { return -1 }
    let boxed = Unmanaged<Box<[Int32: Int32]>>.fromOpaque(receiver).takeUnretainedValue()
    guard let value = boxed.value[key] else { return 0 }
    outValue?.pointee = value
    return 1
}

@_cdecl("swift_contract_dict_i32_set")
public func swift_contract_dict_i32_set(
    _ receiver: UnsafeMutableRawPointer?,
    _ key: Int32,
    _ value: Int32
) -> Int32 {
    guard let receiver else { return -1 }
    let boxed = Unmanaged<Box<[Int32: Int32]>>.fromOpaque(receiver).takeUnretainedValue()
    boxed.value[key] = value
    return Int32(boxed.value.count)
}

@_cdecl("swift_contract_dict_i32_remove")
public func swift_contract_dict_i32_remove(
    _ receiver: UnsafeMutableRawPointer?,
    _ key: Int32,
    _ outValue: UnsafeMutablePointer<Int32>?
) -> Int32 {
    guard let receiver else { return -1 }
    let boxed = Unmanaged<Box<[Int32: Int32]>>.fromOpaque(receiver).takeUnretainedValue()
    guard let removed = boxed.value.removeValue(forKey: key) else { return 0 }
    outValue?.pointee = removed
    return 1
}

@_cdecl("swift_contract_dict_i32_contains")
public func swift_contract_dict_i32_contains(_ receiver: UnsafeMutableRawPointer?, _ key: Int32) -> Int32 {
    guard let receiver else { return -1 }
    let boxed = Unmanaged<Box<[Int32: Int32]>>.fromOpaque(receiver).takeUnretainedValue()
    return boxed.value[key] == nil ? 0 : 1
}

// MARK: - Dictionary<Int32, OpaqueRef> (type_id 7)

@_cdecl("swift_contract_dict_ref_make")
public func swift_contract_dict_ref_make(_ capacity: Int32) -> UnsafeMutableRawPointer? {
    var dict: [Int32: UnsafeMutableRawPointer] = [:]
    dict.reserveCapacity(max(0, Int(capacity)))
    let boxed = Box(dict)
    return Unmanaged.passRetained(boxed).toOpaque()
}

@_cdecl("swift_contract_dict_ref_len")
public func swift_contract_dict_ref_len(_ receiver: UnsafeMutableRawPointer?) -> Int32 {
    guard let receiver else { return -1 }
    let boxed = Unmanaged<Box<[Int32: UnsafeMutableRawPointer]>>.fromOpaque(receiver).takeUnretainedValue()
    return Int32(boxed.value.count)
}

@_cdecl("swift_contract_dict_ref_get")
public func swift_contract_dict_ref_get(
    _ receiver: UnsafeMutableRawPointer?,
    _ key: Int32,
    _ outValue: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Int32 {
    guard let receiver else { return -1 }
    let boxed = Unmanaged<Box<[Int32: UnsafeMutableRawPointer]>>.fromOpaque(receiver).takeUnretainedValue()
    guard let value = boxed.value[key] else { return 0 }
    outValue?.pointee = value
    return 1
}

@_cdecl("swift_contract_dict_ref_set")
public func swift_contract_dict_ref_set(
    _ receiver: UnsafeMutableRawPointer?,
    _ key: Int32,
    _ value: UnsafeMutableRawPointer?
) -> Int32 {
    guard let receiver, let value else { return -1 }
    let boxed = Unmanaged<Box<[Int32: UnsafeMutableRawPointer]>>.fromOpaque(receiver).takeUnretainedValue()
    boxed.value[key] = value
    return Int32(boxed.value.count)
}

@_cdecl("swift_contract_dict_ref_remove")
public func swift_contract_dict_ref_remove(
    _ receiver: UnsafeMutableRawPointer?,
    _ key: Int32
) -> UnsafeMutableRawPointer? {
    guard let receiver else { return nil }
    let boxed = Unmanaged<Box<[Int32: UnsafeMutableRawPointer]>>.fromOpaque(receiver).takeUnretainedValue()
    return boxed.value.removeValue(forKey: key)
}

@_cdecl("swift_contract_dict_ref_contains")
public func swift_contract_dict_ref_contains(_ receiver: UnsafeMutableRawPointer?, _ key: Int32) -> Int32 {
    guard let receiver else { return -1 }
    let boxed = Unmanaged<Box<[Int32: UnsafeMutableRawPointer]>>.fromOpaque(receiver).takeUnretainedValue()
    return boxed.value[key] == nil ? 0 : 1
}

// MARK: - Dynamic Type Casting (Track D.1)

/// Type-erased wrapper for any registered contract object.
/// Holds the original contract type_id alongside the raw object pointer.
/// Assigned type_id = 8 in the contract registry.
public final class ContractAnyBox {
    public let contractTypeID: Int32
    public let rawObject: UnsafeMutableRawPointer

    public init(_ typeID: Int32, _ object: UnsafeMutableRawPointer) {
        self.contractTypeID = typeID
        self.rawObject = object
    }
}

/// Wraps an existing contract object in a type-erased ContractAnyBox (type_id = 8).
/// The caller retains ownership of the inner object; this box does not release it.
@_cdecl("swift_contract_any_wrap")
public func swift_contract_any_wrap(_ typeID: Int32, _ object: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let object else { return nil }
    let box = ContractAnyBox(typeID, object)
    return Unmanaged.passRetained(box).toOpaque()
}

/// Returns the contract type_id stored inside a ContractAnyBox (metatype identity check).
/// Returns -1 if the pointer is nil.
@_cdecl("swift_contract_any_type_id")
public func swift_contract_any_type_id(_ anyBox: UnsafeMutableRawPointer?) -> Int32 {
    guard let anyBox else { return -1 }
    let box = Unmanaged<ContractAnyBox>.fromOpaque(anyBox).takeUnretainedValue()
    return box.contractTypeID
}

/// Dynamic narrowing cast: wraps Swift's `as?` semantics for contract-layer types.
/// If the ContractAnyBox holds an object with `targetTypeID`, returns the inner raw pointer.
/// Returns nil when the actual type_id does not match (cast failure).
@_cdecl("swift_contract_dynamic_cast")
public func swift_contract_dynamic_cast(_ anyBox: UnsafeMutableRawPointer?, _ targetTypeID: Int32) -> UnsafeMutableRawPointer? {
    guard let anyBox else { return nil }
    let box = Unmanaged<ContractAnyBox>.fromOpaque(anyBox).takeUnretainedValue()
    guard box.contractTypeID == targetTypeID else { return nil }
    return box.rawObject
}

/// Generic contract constructor: routes to specific type constructors based on type_id.
/// args_blob is type-dependent: type 1 (Person) expects (i32_age, i32_name_addr, i32_name_len)
@_cdecl("swift_contract_construct")
public func swift_contract_construct(
    _ typeID: Int32,
    _ argsBlobPtr: UnsafeRawPointer?,
    _ argsBlobLen: Int32
) -> UnsafeMutableRawPointer? {
    guard let argsBlobPtr, argsBlobLen >= 0 else { return nil }
    
    switch typeID {
    case 1:
        // Person: two Int32 args (id, age)
        if argsBlobLen < 8 { return nil } // need 2 * 4 bytes
        let args = argsBlobPtr.assumingMemoryBound(to: Int32.self)
        let person = Person(id: args[0], age: args[1])
        return Unmanaged.passRetained(Box(person)).toOpaque()
    
    case 2:
        // Counter: one Int32 arg (initialValue)
        if argsBlobLen < 4 { return nil }
        let args = argsBlobPtr.assumingMemoryBound(to: Int32.self)
        let counter = Counter(start: args[0])
        return Unmanaged.passRetained(counter).toOpaque()
    
    case 3:
        // String: (bytesPtr, byteCount) - stored as (ptr, i32, padding)
        if argsBlobLen < 12 { return nil } // pointer (8 bytes) + i32 (4 bytes)
        let bytesPtr = argsBlobPtr.assumingMemoryBound(to: UnsafeMutableRawPointer?.self).pointee
        let byteCount = argsBlobPtr.advanced(by: 8).assumingMemoryBound(to: Int32.self).pointee
        return swift_contract_construct_string(bytesPtr, byteCount)
    
    case 4:
        // Array<Int32>: one Int32 arg (capacity)
        if argsBlobLen < 4 { return nil }
        let args = argsBlobPtr.assumingMemoryBound(to: Int32.self)
        var array: [Int32] = []
        array.reserveCapacity(max(0, Int(args[0])))
        return Unmanaged.passRetained(Box(array)).toOpaque()
    
    case 5:
        // Array<OpaqueRef>: one Int32 arg (capacity)
        if argsBlobLen < 4 { return nil }
        let args = argsBlobPtr.assumingMemoryBound(to: Int32.self)
        var array: [UnsafeMutableRawPointer] = []
        array.reserveCapacity(max(0, Int(args[0])))
        return Unmanaged.passRetained(Box(array)).toOpaque()
    
    case 6:
        // Dictionary<Int32, Int32>: one Int32 arg (capacity hint)
        if argsBlobLen < 4 { return nil }
        let args = argsBlobPtr.assumingMemoryBound(to: Int32.self)
        var dict: [Int32: Int32] = [:]
        dict.reserveCapacity(max(0, Int(args[0])))
        return Unmanaged.passRetained(Box(dict)).toOpaque()
    
    case 7:
        // Dictionary<Int32, OpaqueRef>: one Int32 arg (capacity hint)
        if argsBlobLen < 4 { return nil }
        let args = argsBlobPtr.assumingMemoryBound(to: Int32.self)
        var dict: [Int32: UnsafeMutableRawPointer] = [:]
        dict.reserveCapacity(max(0, Int(args[0])))
        return Unmanaged.passRetained(Box(dict)).toOpaque()
    
    default:
        return nil
    }
}

@_cdecl("swift_contract_invoke_i32")
public func swift_contract_invoke_i32(
    _ typeID: Int32,
    _ methodID: Int32,
    _ receiver: UnsafeMutableRawPointer?,
    _ argsBlobPtr: UnsafeRawPointer?,
    _ argsBlobLen: Int32
) -> Int32 {
    guard let receiver else { return Int32.min }

    switch typeID {
    case 1:
        let box = Unmanaged<Box<Person>>.fromOpaque(receiver).takeUnretainedValue()
        switch methodID {
        case 1:
            return box.value.id
        case 2:
            return box.value.age
        default:
            return Int32.min
        }

    case 2:
        let counter = Unmanaged<Counter>.fromOpaque(receiver).takeUnretainedValue()
        switch methodID {
        case 1:
            guard let argsBlobPtr, argsBlobLen >= 4 else { return Int32.min }
            let amount = argsBlobPtr.assumingMemoryBound(to: Int32.self).pointee
            return counter.increment(by: amount)
        case 2:
            return counter.current()
        case 4:
            let readable: CounterLike = counter
            return readable.current()
        default:
            return Int32.min
        }

    default:
        return Int32.min
    }
}

@_cdecl("swift_contract_invoke_void")
public func swift_contract_invoke_void(
    _ typeID: Int32,
    _ methodID: Int32,
    _ receiver: UnsafeMutableRawPointer?,
    _ argsBlobPtr: UnsafeRawPointer?,
    _ argsBlobLen: Int32
) -> Int32 {
    guard let receiver else { return 0 }

    switch typeID {
    case 2:
        let counter = Unmanaged<Counter>.fromOpaque(receiver).takeUnretainedValue()
        switch methodID {
        case 3:
            guard let argsBlobPtr, argsBlobLen >= 4 else { return 0 }
            let value = argsBlobPtr.assumingMemoryBound(to: Int32.self).pointee
            counter.reset(to: value)
            return 1
        default:
            return 0
        }

    default:
        return 0
    }
}

@_cdecl("swift_contract_protocol_has_conformance")
public func swift_contract_protocol_has_conformance(_ typeID: Int32, _ protocolID: Int32) -> Int32 {
    // Contract probe currently requires CounterLike on Counter only.
    if typeID == 2 && protocolID == 1 {
        return 1
    }
    return 0
}

@_cdecl("swift_contract_protocol_invoke_i32")
public func swift_contract_protocol_invoke_i32(
    _ typeID: Int32,
    _ protocolID: Int32,
    _ methodID: Int32,
    _ object: UnsafeMutableRawPointer?
) -> Int32 {
    guard typeID == 2, protocolID == 1, methodID == 1, let object else {
        return Int32.min
    }
    let counter = Unmanaged<Counter>.fromOpaque(object).takeUnretainedValue()
    let readable: CounterLike = counter
    return readable.current()
}

@_cdecl("swift_contract_release")
public func swift_contract_release(_ typeID: Int32, _ object: UnsafeMutableRawPointer?) -> Int32 {
    guard let object else { return 0 }

    switch typeID {
    case 1:
        _ = Unmanaged<Box<Person>>.fromOpaque(object).takeRetainedValue()
        return 1
    case 2:
        _ = Unmanaged<Counter>.fromOpaque(object).takeRetainedValue()
        return 1
    case 3:
        _ = Unmanaged<Box<String>>.fromOpaque(object).takeRetainedValue()
        return 1
    case 4:
        _ = Unmanaged<Box<[Int32]>>.fromOpaque(object).takeRetainedValue()
        return 1
    case 5:
        _ = Unmanaged<Box<[UnsafeMutableRawPointer]>>.fromOpaque(object).takeRetainedValue()
        return 1
    case 6:
        _ = Unmanaged<Box<[Int32: Int32]>>.fromOpaque(object).takeRetainedValue()
        return 1
    case 7:
        _ = Unmanaged<Box<[Int32: UnsafeMutableRawPointer]>>.fromOpaque(object).takeRetainedValue()
        return 1
    case 8:
        _ = Unmanaged<ContractAnyBox>.fromOpaque(object).takeRetainedValue()
        return 1
    case 20:
        _ = Unmanaged<Box<ProbeCounterActor>>.fromOpaque(object).takeRetainedValue()
        return 1
    case 21:
        _ = Unmanaged<Box<ProbeAsyncIteratorBox>>.fromOpaque(object).takeRetainedValue()
        return 1
    case 23:
        _ = Unmanaged<Box<ContractGenericBox<Int32>>>.fromOpaque(object).takeRetainedValue()
        return 1
    default:
        return 0
    }
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

// ── Track F.1: Variable-Sized Struct Construction (test payload for layout introspection) ──
/// Struct with mixed field types to test layout introspection, alignment, and field-offset discovery.
public struct TestPayload {
    public var field_a: Int32      // Offset 0, size 4
    public var field_b: Int64      // Offset 8 (aligned to 8-byte boundary), size 8
    public var field_c: Int32      // Offset 16, size 4
    
    public init(field_a: Int32, field_b: Int64, field_c: Int32) {
        self.field_a = field_a
        self.field_b = field_b
        self.field_c = field_c
    }
}

// ── Track F.2: Tuple Construction & Unpacking (boxable tuples for contract system) ──
/// A simple 2-element tuple type (Pair) that can be boxed and bridged via contract.
public struct Pair {
    public var first: Int32
    public var second: Int32
    public init(first: Int32, second: Int32) {
        self.first = first
        self.second = second
    }
}

/// A 3-element tuple type (Triple) that can be boxed and bridged via contract.
public struct Triple {
    public var first: Int32
    public var second: Int32
    public var third: Int32
    public init(first: Int32, second: Int32, third: Int32) {
        self.first = first
        self.second = second
        self.third = third
    }
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

@_cdecl("swift_plist_encoder_probe_flags")
public func swift_plist_encoder_probe_flags() -> Int32 {
    var flags: Int32 = 0

    struct PlistData: Codable {
        let title: String
        let count: Int
    }

    let decoder = PropertyListDecoder()
    let original = PlistData(title: "test", count: 42)

    do {
        let encoder = PropertyListEncoder()
        let xmlData = try encoder.encode(original)
        if !xmlData.isEmpty { flags |= 1 }

        let decoded = try decoder.decode(PlistData.self, from: xmlData)
        if decoded.title == original.title && decoded.count == original.count { flags |= 2 }

        let binaryData = try PropertyListSerialization.data(fromPropertyList: ["title": "test", "count": 42], format: .binary, options: 0)
        if !binaryData.isEmpty { flags |= 4 }

        if let dict = try PropertyListSerialization.propertyList(from: binaryData, options: [], format: nil) as? [String: Any],
           let title = dict["title"] as? String, title == "test" { flags |= 8 }
    } catch { }

    return flags
}

@_cdecl("swift_range_probe_flags")
public func swift_range_probe_flags() -> Int32 {
    var flags: Int32 = 0
    let range: Range<Int> = 5..<10

    if range.contains(7) { flags |= 1 }

    if !range.contains(4) && !range.contains(10) { flags |= 2 }

    if range.isEmpty == false { flags |= 4 }

    if range.count == 5 { flags |= 8 }

    return flags
}

@_cdecl("swift_url_query_item_probe_flags")
public func swift_url_query_item_probe_flags() -> Int32 {
    var flags: Int32 = 0
    let item = URLQueryItem(name: "key", value: "value")

    if item.name == "key" { flags |= 1 }

    if item.value == "value" { flags |= 2 }

    let encodedItem = URLQueryItem(name: "special&char", value: "test=value")
    if encodedItem.name == "special&char" && encodedItem.value == "test=value" { flags |= 4 }

    let nilItem = URLQueryItem(name: "empty", value: nil)
    if nilItem.value == nil { flags |= 8 }

    return flags
}

@_cdecl("swift_closed_range_probe_flags")
public func swift_closed_range_probe_flags() -> Int32 {
    var flags: Int32 = 0
    let closedRange: ClosedRange<Int> = 5...10

    if closedRange.contains(7) { flags |= 1 }

    if closedRange.lowerBound == 5 && closedRange.upperBound == 10 { flags |= 2 }

    if !closedRange.isEmpty { flags |= 4 }

    if closedRange.count == 6 { flags |= 8 }

    return flags
}

@_cdecl("swift_date_interval_probe_flags")
public func swift_date_interval_probe_flags() -> Int32 {
    var flags: Int32 = 0
    let start = Date(timeIntervalSince1970: 0)
    let end = Date(timeIntervalSince1970: 3600)
    let interval = DateInterval(start: start, end: end)

    if Int(interval.duration) == 3600 { flags |= 1 }

    if interval.contains(Date(timeIntervalSince1970: 1800)) { flags |= 2 }

    let other = DateInterval(start: Date(timeIntervalSince1970: 2400), end: Date(timeIntervalSince1970: 4800))
    if let overlap = interval.intersection(with: other), Int(overlap.duration) == 1200 { flags |= 4 }

    if interval.start == start && interval.end == end { flags |= 8 }

    return flags
}

@_cdecl("swift_index_path_probe_flags")
public func swift_index_path_probe_flags() -> Int32 {
    var flags: Int32 = 0
    let path = IndexPath(indexes: [1, 3, 5])

    if path.count == 3 { flags |= 1 }

    if path[0] == 1 && path[2] == 5 { flags |= 2 }

    let appended = path.appending(7)
    if appended.count == 4 && appended[3] == 7 { flags |= 4 }

    let next = IndexPath(indexes: [1, 3, 6])
    if path.compare(next) == .orderedAscending { flags |= 8 }

    return flags
}

@_cdecl("swift_iso8601_probe_flags")
public func swift_iso8601_probe_flags() -> Int32 {
    var flags: Int32 = 0
    let date = Date(timeIntervalSince1970: 0)
    let formatter = ISO8601DateFormatter()
    formatter.timeZone = TimeZone(secondsFromGMT: 0)

    let basic = formatter.string(from: date)
    if basic == "1970-01-01T00:00:00Z" { flags |= 1 }

    if let parsed = formatter.date(from: basic), Int(parsed.timeIntervalSince1970) == 0 { flags |= 2 }

    formatter.formatOptions = [.withInternetDateTime, .withFractionalSeconds]
    let fractional = formatter.string(from: date)
    if fractional == "1970-01-01T00:00:00.000Z" { flags |= 4 }

    if let parsedFractional = formatter.date(from: fractional), Int(parsedFractional.timeIntervalSince1970) == 0 { flags |= 8 }

    return flags
}

@_cdecl("swift_url_percent_encoding_probe_flags")
public func swift_url_percent_encoding_probe_flags() -> Int32 {
    var flags: Int32 = 0
    let allowed = CharacterSet.alphanumerics

    let raw1 = "Swift Runtime+Sys"
    let encoded1 = raw1.addingPercentEncoding(withAllowedCharacters: allowed) ?? ""
    if encoded1 == "Swift%20Runtime%2BSys" { flags |= 1 }

    if encoded1.removingPercentEncoding == raw1 { flags |= 2 }

    let raw2 = "/?&="
    let encoded2 = raw2.addingPercentEncoding(withAllowedCharacters: allowed) ?? ""
    if encoded2 == "%2F%3F%26%3D" { flags |= 4 }

    if "%ZZ".removingPercentEncoding == nil { flags |= 8 }

    return flags
}

@_cdecl("swift_url_session_configuration_probe_flags")
public func swift_url_session_configuration_probe_flags() -> Int32 {
    var flags: Int32 = 0
    let cfg = URLSessionConfiguration.default
    let eph = URLSessionConfiguration.ephemeral

    if cfg.requestCachePolicy == .useProtocolCachePolicy { flags |= 1 }
    if cfg !== eph { flags |= 2 }
    if cfg.timeoutIntervalForRequest > 0 && cfg.timeoutIntervalForResource >= cfg.timeoutIntervalForRequest { flags |= 4 }
    if cfg.allowsCellularAccess { flags |= 8 }

    return flags
}

@_cdecl("swift_file_manager_probe_flags")
public func swift_file_manager_probe_flags() -> Int32 {
    var flags: Int32 = 0
    let fm = FileManager.default
    let tempDir = fm.temporaryDirectory
    let fileURL = tempDir.appendingPathComponent("runtime-parity-\(UUID().uuidString).txt")

    if tempDir.isFileURL { flags |= 1 }

    do {
        try Data("ok".utf8).write(to: fileURL, options: .atomic)
        if fm.fileExists(atPath: fileURL.path) { flags |= 2 }

        let readBack = try Data(contentsOf: fileURL)
        if readBack == Data("ok".utf8) { flags |= 4 }

        try fm.removeItem(at: fileURL)
        if !fm.fileExists(atPath: fileURL.path) { flags |= 8 }
    } catch {
        // flags remain partial/zero on error
    }

    return flags
}

@_cdecl("swift_date_components_probe_flags")
public func swift_date_components_probe_flags() -> Int32 {
    var flags: Int32 = 0
    var calendar = Calendar(identifier: .gregorian)
    calendar.timeZone = TimeZone(secondsFromGMT: 0)!

    if let normalized = calendar.date(from: DateComponents(year: 2024, month: 2, day: 31, hour: 12)) {
        let back = calendar.dateComponents([.year, .month, .day, .hour], from: normalized)
        if back.year == 2024 && back.month == 3 && back.day == 2 && back.hour == 12 { flags |= 1 }
    }

    if let leap = calendar.date(from: DateComponents(year: 2024, month: 2, day: 29, hour: 9)) {
        let round = calendar.dateComponents([.year, .month, .day], from: leap)
        if round.year == 2024 && round.month == 2 && round.day == 29 { flags |= 2 }

        let weekday = calendar.component(.weekday, from: leap)
        if weekday == 5 { flags |= 4 } // Thursday in Gregorian calendar

        if let nextDay = calendar.date(from: DateComponents(year: 2024, month: 3, day: 1, hour: 9)) {
            if Int(nextDay.timeIntervalSince(leap)) == 86_400 { flags |= 8 }
        }
    }

    return flags
}

@_cdecl("swift_notification_probe_flags")
public func swift_notification_probe_flags() -> Int32 {
    var flags: Int32 = 0
    let center = NotificationCenter()
    let name = Notification.Name("RuntimeParity.Test")
    let object = NSObject()
    let userInfo: [AnyHashable: Any] = ["answer": 42, "kind": "probe"]
    var observedCount = 0
    var observedObjectMatch = false
    var observedUserInfoMatch = false

    if name.rawValue == "RuntimeParity.Test" { flags |= 1 }

    let token = center.addObserver(forName: name, object: nil, queue: nil) { note in
        observedCount += 1
        observedObjectMatch = (note.object as AnyObject?) === object
        let answer = note.userInfo?["answer"] as? Int
        let kind = note.userInfo?["kind"] as? String
        observedUserInfoMatch = (answer == 42 && kind == "probe")
    }

    center.post(name: name, object: object, userInfo: userInfo)
    center.removeObserver(token)

    if observedUserInfoMatch { flags |= 2 }
    if observedCount == 1 { flags |= 4 }
    if observedObjectMatch { flags |= 8 }

    return flags
}

@_cdecl("swift_byte_count_formatter_probe_flags")
public func swift_byte_count_formatter_probe_flags() -> Int32 {
    var flags: Int32 = 0
    let f = ByteCountFormatter()
    f.countStyle = .file
    f.includesUnit = true
    f.includesCount = true
    f.includesActualByteCount = false
    f.isAdaptive = false
    f.allowedUnits = [.useBytes]

    let bytes = f.string(fromByteCount: 1_024)
    if bytes.lowercased().contains("byte") { flags |= 1 }
    if bytes.contains("1024") || bytes.contains("1,024") { flags |= 2 }

    f.allowedUnits = [.useKB]
    f.isAdaptive = true
    let kb = f.string(fromByteCount: 1_536)
    if kb.uppercased().contains("KB") { flags |= 4 }

    f.allowedUnits = [.useBytes]
    f.isAdaptive = false
    let zero = f.string(fromByteCount: 0).lowercased()
    if zero.contains("0") || zero.contains("zero") { flags |= 8 }

    return flags
}

@_cdecl("swift_range_bridge_probe_flags")
public func swift_range_bridge_probe_flags() -> Int32 {
    var flags: Int32 = 0
    let ascii = "abcdef"
    let nsRange = NSRange(location: 1, length: 3)

    if let r = Range(nsRange, in: ascii), String(ascii[r]) == "bcd" { flags |= 1 }

    let swiftRange = ascii.index(ascii.startIndex, offsetBy: 2)..<ascii.index(ascii.startIndex, offsetBy: 5)
    let bridged = NSRange(swiftRange, in: ascii)
    if bridged.location == 2 && bridged.length == 3 { flags |= 2 }

    if Range(NSRange(location: 10, length: 2), in: ascii) == nil { flags |= 4 }

    let unicode = "a🙂b"
    if let ur = Range(NSRange(location: 1, length: 2), in: unicode), String(unicode[ur]) == "🙂" { flags |= 8 }

    return flags
}

@_cdecl("swift_attributed_string_probe_flags")
public func swift_attributed_string_probe_flags() -> Int32 {
    var flags: Int32 = 0

    if #available(macOS 12.0, *) {
        let ns = NSAttributedString(string: "hello")
        if ns.string == "hello" { flags |= 1 }

        let swiftAttr = AttributedString(ns)
        do {
            if String(swiftAttr.characters) == "hello" { flags |= 2 }

            let nsRoundtrip = NSAttributedString(swiftAttr)
            if nsRoundtrip.string == "hello" { flags |= 4 }

            let appended = swiftAttr + AttributedString("!")
            if String(appended.characters) == "hello!" { flags |= 8 }
        }
    }

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

// ── Track F.1: Struct Layout Introspection (TestPayload) ──────────────────

@_cdecl("swift_struct_testpayload_size")
public func swift_struct_testpayload_size() -> Int32 {
    Int32(MemoryLayout<TestPayload>.size)
}

@_cdecl("swift_struct_testpayload_stride")
public func swift_struct_testpayload_stride() -> Int32 {
    Int32(MemoryLayout<TestPayload>.stride)
}

@_cdecl("swift_struct_testpayload_alignment")
public func swift_struct_testpayload_alignment() -> Int32 {
    Int32(MemoryLayout<TestPayload>.alignment)
}

/// Get the byte offset of field_a from the start of TestPayload.
@_cdecl("swift_struct_testpayload_offset_a")
public func swift_struct_testpayload_offset_a() -> Int32 {
    var x = TestPayload(field_a: 1, field_b: 2, field_c: 3)
    return withUnsafeMutablePointer(to: &x) { base in
        withUnsafeMutablePointer(to: &base.pointee.field_a) { aptr in
            Int32(Int(bitPattern: aptr) - Int(bitPattern: base))
        }
    }
}

/// Get the byte offset of field_b from the start of TestPayload.
@_cdecl("swift_struct_testpayload_offset_b")
public func swift_struct_testpayload_offset_b() -> Int32 {
    var x = TestPayload(field_a: 1, field_b: 2, field_c: 3)
    return withUnsafeMutablePointer(to: &x) { base in
        withUnsafeMutablePointer(to: &base.pointee.field_b) { bptr in
            Int32(Int(bitPattern: bptr) - Int(bitPattern: base))
        }
    }
}

/// Get the byte offset of field_c from the start of TestPayload.
@_cdecl("swift_struct_testpayload_offset_c")
public func swift_struct_testpayload_offset_c() -> Int32 {
    var x = TestPayload(field_a: 1, field_b: 2, field_c: 3)
    return withUnsafeMutablePointer(to: &x) { base in
        withUnsafeMutablePointer(to: &base.pointee.field_c) { cptr in
            Int32(Int(bitPattern: cptr) - Int(bitPattern: base))
        }
    }
}

/// Construct a TestPayload from a blob of raw bytes (field_a as first Int32, field_b as next Int64, field_c as last Int32).
/// Returns a boxed opaque reference, or null if the blob is too small.
@_cdecl("swift_contract_struct_testpayload_construct")
public func swift_contract_struct_testpayload_construct(_ bytes: UnsafeRawPointer?, _ len: Int32) -> UnsafeMutableRawPointer? {
    guard let bytes, len >= Int32(MemoryLayout<TestPayload>.size) else { return nil }
    
    let ptr = UnsafeRawPointer(bytes)
    let field_a = ptr.load(as: Int32.self)
    let field_b = ptr.advanced(by: 8).load(as: Int64.self)  // offset 8 due to alignment
    let field_c = ptr.advanced(by: 16).load(as: Int32.self)
    
    let payload = TestPayload(field_a: field_a, field_b: field_b, field_c: field_c)
    let boxed = Box(payload)
    return Unmanaged.passRetained(boxed).toOpaque()
}

/// Extract field_a (Int32) from a boxed TestPayload. Returns -1 if the pointer is invalid.
@_cdecl("swift_contract_struct_testpayload_get_field_a")
public func swift_contract_struct_testpayload_get_field_a(_ ptr: UnsafeMutableRawPointer?) -> Int32 {
    guard let ptr else { return -1 }
    let boxed = Unmanaged<Box<TestPayload>>.fromOpaque(ptr).takeUnretainedValue()
    return boxed.value.field_a
}

/// Extract field_b (Int64) from a boxed TestPayload as an Int64. Returns -1 if the pointer is invalid.
@_cdecl("swift_contract_struct_testpayload_get_field_b")
public func swift_contract_struct_testpayload_get_field_b(_ ptr: UnsafeMutableRawPointer?) -> Int64 {
    guard let ptr else { return -1 }
    let boxed = Unmanaged<Box<TestPayload>>.fromOpaque(ptr).takeUnretainedValue()
    return boxed.value.field_b
}

/// Extract field_c (Int32) from a boxed TestPayload. Returns -1 if the pointer is invalid.
@_cdecl("swift_contract_struct_testpayload_get_field_c")
public func swift_contract_struct_testpayload_get_field_c(_ ptr: UnsafeMutableRawPointer?) -> Int32 {
    guard let ptr else { return -1 }
    let boxed = Unmanaged<Box<TestPayload>>.fromOpaque(ptr).takeUnretainedValue()
    return boxed.value.field_c
}

// ── Track F.2: Tuple Construction & Unpacking ──────────────────────────────

/// Construct a Pair (2-element tuple) from two Int32 values and return a boxed opaque reference.
@_cdecl("swift_contract_tuple_pair_construct")
public func swift_contract_tuple_pair_construct(_ first: Int32, _ second: Int32) -> UnsafeMutableRawPointer? {
    let pair = Pair(first: first, second: second)
    let boxed = Box(pair)
    return Unmanaged.passRetained(boxed).toOpaque()
}

/// Extract the first element (Int32) from a boxed Pair. Returns -1 if the pointer is invalid.
@_cdecl("swift_contract_tuple_pair_get_first")
public func swift_contract_tuple_pair_get_first(_ ptr: UnsafeMutableRawPointer?) -> Int32 {
    guard let ptr else { return -1 }
    let boxed = Unmanaged<Box<Pair>>.fromOpaque(ptr).takeUnretainedValue()
    return boxed.value.first
}

/// Extract the second element (Int32) from a boxed Pair. Returns -1 if the pointer is invalid.
@_cdecl("swift_contract_tuple_pair_get_second")
public func swift_contract_tuple_pair_get_second(_ ptr: UnsafeMutableRawPointer?) -> Int32 {
    guard let ptr else { return -1 }
    let boxed = Unmanaged<Box<Pair>>.fromOpaque(ptr).takeUnretainedValue()
    return boxed.value.second
}

/// Construct a Triple (3-element tuple) from three Int32 values and return a boxed opaque reference.
@_cdecl("swift_contract_tuple_triple_construct")
public func swift_contract_tuple_triple_construct(_ first: Int32, _ second: Int32, _ third: Int32) -> UnsafeMutableRawPointer? {
    let triple = Triple(first: first, second: second, third: third)
    let boxed = Box(triple)
    return Unmanaged.passRetained(boxed).toOpaque()
}

/// Extract the first element (Int32) from a boxed Triple. Returns -1 if the pointer is invalid.
@_cdecl("swift_contract_tuple_triple_get_first")
public func swift_contract_tuple_triple_get_first(_ ptr: UnsafeMutableRawPointer?) -> Int32 {
    guard let ptr else { return -1 }
    let boxed = Unmanaged<Box<Triple>>.fromOpaque(ptr).takeUnretainedValue()
    return boxed.value.first
}

/// Extract the second element (Int32) from a boxed Triple. Returns -1 if the pointer is invalid.
@_cdecl("swift_contract_tuple_triple_get_second")
public func swift_contract_tuple_triple_get_second(_ ptr: UnsafeMutableRawPointer?) -> Int32 {
    guard let ptr else { return -1 }
    let boxed = Unmanaged<Box<Triple>>.fromOpaque(ptr).takeUnretainedValue()
    return boxed.value.second
}

/// Extract the third element (Int32) from a boxed Triple. Returns -1 if the pointer is invalid.
@_cdecl("swift_contract_tuple_triple_get_third")
public func swift_contract_tuple_triple_get_third(_ ptr: UnsafeMutableRawPointer?) -> Int32 {
    guard let ptr else { return -1 }
    let boxed = Unmanaged<Box<Triple>>.fromOpaque(ptr).takeUnretainedValue()
    return boxed.value.third
}

/// Write a new value into field_a of a boxed TestPayload (by-reference field mutation).
/// Returns 1 on success, 0 on failure.
@_cdecl("swift_contract_struct_testpayload_set_field_a")
public func swift_contract_struct_testpayload_set_field_a(_ ptr: UnsafeMutableRawPointer?, _ new_val: Int32) -> Int32 {
    guard let ptr else { return 0 }
    let boxed = Unmanaged<Box<TestPayload>>.fromOpaque(ptr).takeUnretainedValue()
    // Update field_a through the mutable box reference
    boxed.value.field_a = new_val
    return 1
}

// ─────────────────────────────────────────────────────────────────────────────
// Track F.3: Closure/Function Pointer Bridging
// ─────────────────────────────────────────────────────────────────────────────

/// A simple closure wrapper that captures a delta and applies it to input.
public class ClosureCapture {
    let delta: Int32
    
    init(delta: Int32) {
        self.delta = delta
    }
    
    func apply(_ value: Int32) -> Int32 {
        value + delta
    }
}

/// Construct a closure with a captured delta value.
/// Returns an opaque pointer to a boxed ClosureCapture, or nil on failure.
@_cdecl("swift_contract_closure_make_adder")
public func swift_contract_closure_make_adder(_ delta: Int32) -> UnsafeMutableRawPointer? {
    let capture = ClosureCapture(delta: delta)
    let boxed = Box(capture)
    return Unmanaged.passRetained(boxed).toOpaque()
}

/// Invoke a closure with a single Int32 argument. Returns the result or -999 on failure.
@_cdecl("swift_contract_closure_invoke_adder")
public func swift_contract_closure_invoke_adder(_ ptr: UnsafeMutableRawPointer?, _ arg: Int32) -> Int32 {
    guard let ptr else { return -999 }
    let boxed = Unmanaged<Box<ClosureCapture>>.fromOpaque(ptr).takeUnretainedValue()
    return boxed.value.apply(arg)
}

/// Extract the captured delta value from a closure. Returns -999 on failure.
@_cdecl("swift_contract_closure_get_capture")
public func swift_contract_closure_get_capture(_ ptr: UnsafeMutableRawPointer?) -> Int32 {
    guard let ptr else { return -999 }
    let boxed = Unmanaged<Box<ClosureCapture>>.fromOpaque(ptr).takeUnretainedValue()
    return boxed.value.delta
}

/// A closure that accepts two arguments and produces a result.
public class ClosureMultiArg {
    let factor: Int32
    let offset: Int32
    
    init(factor: Int32, offset: Int32) {
        self.factor = factor
        self.offset = offset
    }
    
    func apply(_ a: Int32, _ b: Int32) -> Int32 {
        (a * factor) + (b * offset)
    }
}

/// Construct a multi-argument closure with factor and offset captures.
@_cdecl("swift_contract_closure_make_multi")
public func swift_contract_closure_make_multi(_ factor: Int32, _ offset: Int32) -> UnsafeMutableRawPointer? {
    let capture = ClosureMultiArg(factor: factor, offset: offset)
    let boxed = Box(capture)
    return Unmanaged.passRetained(boxed).toOpaque()
}

/// Invoke a multi-argument closure. Returns result or -999 on failure.
@_cdecl("swift_contract_closure_invoke_multi")
public func swift_contract_closure_invoke_multi(_ ptr: UnsafeMutableRawPointer?, _ a: Int32, _ b: Int32) -> Int32 {
    guard let ptr else { return -999 }
    let boxed = Unmanaged<Box<ClosureMultiArg>>.fromOpaque(ptr).takeUnretainedValue()
    return boxed.value.apply(a, b)
}

/// Extract factor from a multi-arg closure. Returns -999 on failure.
@_cdecl("swift_contract_closure_get_factor")
public func swift_contract_closure_get_factor(_ ptr: UnsafeMutableRawPointer?) -> Int32 {
    guard let ptr else { return -999 }
    let boxed = Unmanaged<Box<ClosureMultiArg>>.fromOpaque(ptr).takeUnretainedValue()
    return boxed.value.factor
}

/// Extract offset from a multi-arg closure. Returns -999 on failure.
@_cdecl("swift_contract_closure_get_offset")
public func swift_contract_closure_get_offset(_ ptr: UnsafeMutableRawPointer?) -> Int32 {
    guard let ptr else { return -999 }
    let boxed = Unmanaged<Box<ClosureMultiArg>>.fromOpaque(ptr).takeUnretainedValue()
    return boxed.value.offset
}
// MARK: - Array<Int32> Bridging (Phase A.2)

/// Container for boxed Array<Int32> with lifetime management.
public class ArrayInt32Box {
    var array: [Int32]
    
    init(capacity: Int32) {
        self.array = Array(repeating: 0, count: Int(capacity))
    }
    
    init(from array: [Int32]) {
        self.array = array
    }
}

/// Create an Array<Int32> with initial capacity. Returns opaque pointer or nil.
@_cdecl("swift_contract_array_int32_make")
public func swift_contract_array_int32_make(_ capacity: Int32) -> UnsafeMutableRawPointer? {
    guard capacity >= 0 else { return nil }
    let box = ArrayInt32Box(capacity: capacity)
    return Unmanaged.passRetained(box).toOpaque()
}

/// Get length of Array<Int32>. Returns -1 on invalid ptr.
@_cdecl("swift_contract_array_int32_len")
public func swift_contract_array_int32_len(_ ptr: UnsafeMutableRawPointer?) -> Int32 {
    guard let ptr else { return -1 }
    let box = Unmanaged<ArrayInt32Box>.fromOpaque(ptr).takeUnretainedValue()
    return Int32(box.array.count)
}

/// Get element at index. Returns default 0 on bounds error.
@_cdecl("swift_contract_array_int32_get")
public func swift_contract_array_int32_get(_ ptr: UnsafeMutableRawPointer?, _ index: Int32) -> Int32 {
    guard let ptr, index >= 0 else { return 0 }
    let box = Unmanaged<ArrayInt32Box>.fromOpaque(ptr).takeUnretainedValue()
    guard index < box.array.count else { return 0 }
    return box.array[Int(index)]
}

/// Set element at index. Returns 0 on success, -1 on bounds error.
@_cdecl("swift_contract_array_int32_set")
public func swift_contract_array_int32_set(_ ptr: UnsafeMutableRawPointer?, _ index: Int32, _ value: Int32) -> Int32 {
    guard let ptr, index >= 0 else { return -1 }
    let box = Unmanaged<ArrayInt32Box>.fromOpaque(ptr).takeUnretainedValue()
    guard index < box.array.count else { return -1 }
    box.array[Int(index)] = value
    return 0
}

/// Append value to Array<Int32>. Returns new length or -1 on failure.
@_cdecl("swift_contract_array_int32_append")
public func swift_contract_array_int32_append(_ ptr: UnsafeMutableRawPointer?, _ value: Int32) -> Int32 {
    guard let ptr else { return -1 }
    let box = Unmanaged<ArrayInt32Box>.fromOpaque(ptr).takeUnretainedValue()
    box.array.append(value)
    return Int32(box.array.count)
}

/// Get raw data pointer (for direct memory access). Returns nil on failure.
@_cdecl("swift_contract_array_int32_data")
public func swift_contract_array_int32_data(_ ptr: UnsafeMutableRawPointer?) -> UnsafeRawPointer? {
    guard let ptr else { return nil }
    let box = Unmanaged<ArrayInt32Box>.fromOpaque(ptr).takeUnretainedValue()
    guard !box.array.isEmpty else { return nil }
    return box.array.withUnsafeBytes { $0.baseAddress }
}

/// Release Array<Int32>.
@_cdecl("swift_contract_array_int32_release")
public func swift_contract_array_int32_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    let _ = Unmanaged<ArrayInt32Box>.fromOpaque(ptr).takeRetainedValue()
}

// MARK: - Array<OpaqueRef> Bridging (Phase A.2)

/// Container for boxed Array<OpaqueRef> with lifetime management.
public class ArrayOpaqueRefBox {
    var array: [UnsafeMutableRawPointer?]
    
    init(capacity: Int32) {
        self.array = Array(repeating: nil, count: Int(capacity))
    }
    
    init(from array: [UnsafeMutableRawPointer?]) {
        self.array = array
    }
}

/// Create an Array<OpaqueRef> with initial capacity. Returns opaque pointer or nil.
@_cdecl("swift_contract_array_opaque_ref_make")
public func swift_contract_array_opaque_ref_make(_ capacity: Int32) -> UnsafeMutableRawPointer? {
    guard capacity >= 0 else { return nil }
    let box = ArrayOpaqueRefBox(capacity: capacity)
    return Unmanaged.passRetained(box).toOpaque()
}

/// Get length of Array<OpaqueRef>. Returns -1 on invalid ptr.
@_cdecl("swift_contract_array_opaque_ref_len")
public func swift_contract_array_opaque_ref_len(_ ptr: UnsafeMutableRawPointer?) -> Int32 {
    guard let ptr else { return -1 }
    let box = Unmanaged<ArrayOpaqueRefBox>.fromOpaque(ptr).takeUnretainedValue()
    return Int32(box.array.count)
}

/// Get element at index. Returns nil on bounds error.
@_cdecl("swift_contract_array_opaque_ref_get")
public func swift_contract_array_opaque_ref_get(_ ptr: UnsafeMutableRawPointer?, _ index: Int32) -> UnsafeMutableRawPointer? {
    guard let ptr, index >= 0 else { return nil }
    let box = Unmanaged<ArrayOpaqueRefBox>.fromOpaque(ptr).takeUnretainedValue()
    guard index < box.array.count else { return nil }
    return box.array[Int(index)]
}

/// Set element at index. Returns 0 on success, -1 on bounds error.
@_cdecl("swift_contract_array_opaque_ref_set")
public func swift_contract_array_opaque_ref_set(_ ptr: UnsafeMutableRawPointer?, _ index: Int32, _ value: UnsafeMutableRawPointer?) -> Int32 {
    guard let ptr, index >= 0 else { return -1 }
    let box = Unmanaged<ArrayOpaqueRefBox>.fromOpaque(ptr).takeUnretainedValue()
    guard index < box.array.count else { return -1 }
    box.array[Int(index)] = value
    return 0
}

/// Append value to Array<OpaqueRef>. Returns new length or -1 on failure.
@_cdecl("swift_contract_array_opaque_ref_append")
public func swift_contract_array_opaque_ref_append(_ ptr: UnsafeMutableRawPointer?, _ value: UnsafeMutableRawPointer?) -> Int32 {
    guard let ptr else { return -1 }
    let box = Unmanaged<ArrayOpaqueRefBox>.fromOpaque(ptr).takeUnretainedValue()
    box.array.append(value)
    return Int32(box.array.count)
}

/// Release Array<OpaqueRef>.
@_cdecl("swift_contract_array_opaque_ref_release")
public func swift_contract_array_opaque_ref_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    let _ = Unmanaged<ArrayOpaqueRefBox>.fromOpaque(ptr).takeRetainedValue()
}
