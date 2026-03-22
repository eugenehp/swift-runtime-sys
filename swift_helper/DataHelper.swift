import Combine
import Foundation

// ═══════════════════════════════════════════════════════════════════════════
// Persistence — UserDefaults-based key-value store (works without @Model)
// ═══════════════════════════════════════════════════════════════════════════

class KVStore {
    let defaults: UserDefaults
    let prefix: String
    init(suite: String?) {
        self.defaults = suite.flatMap { UserDefaults(suiteName: $0) } ?? .standard
        self.prefix = (suite ?? "") + "."
    }
    func k(_ table: String, _ key: String) -> String { "\(prefix)\(table).\(key)" }
}

private var kvStores: [UnsafeMutableRawPointer: KVStore] = [:]

@_cdecl("kv_store_create")
public func kvStoreCreate(_ suitePtr: UnsafePointer<UInt8>?, _ suiteLen: Int) -> UnsafeMutableRawPointer {
    let suite: String? = suitePtr.flatMap { ptr in
        suiteLen > 0 ? String(bytes: UnsafeBufferPointer(start: ptr, count: suiteLen), encoding: .utf8) : nil
    }
    let store = KVStore(suite: suite)
    let ptr = Unmanaged.passRetained(store as AnyObject).toOpaque()
    kvStores[ptr] = store
    return ptr
}

@_cdecl("kv_store_set_string")
public func kvStoreSetString(_ storePtr: UnsafeMutableRawPointer, _ tP: UnsafePointer<UInt8>, _ tL: Int, _ kP: UnsafePointer<UInt8>, _ kL: Int, _ vP: UnsafePointer<UInt8>, _ vL: Int) {
    guard let s = kvStores[storePtr] else { return }
    let t = String(bytes: UnsafeBufferPointer(start: tP, count: tL), encoding: .utf8) ?? ""
    let k = String(bytes: UnsafeBufferPointer(start: kP, count: kL), encoding: .utf8) ?? ""
    let v = String(bytes: UnsafeBufferPointer(start: vP, count: vL), encoding: .utf8) ?? ""
    s.defaults.set(v, forKey: s.k(t, k))
}

@_cdecl("kv_store_get_string")
public func kvStoreGetString(_ storePtr: UnsafeMutableRawPointer, _ tP: UnsafePointer<UInt8>, _ tL: Int, _ kP: UnsafePointer<UInt8>, _ kL: Int, _ outP: UnsafeMutablePointer<UnsafeMutableRawPointer?>, _ outL: UnsafeMutablePointer<Int>) -> Bool {
    guard let s = kvStores[storePtr] else { return false }
    let t = String(bytes: UnsafeBufferPointer(start: tP, count: tL), encoding: .utf8) ?? ""
    let k = String(bytes: UnsafeBufferPointer(start: kP, count: kL), encoding: .utf8) ?? ""
    guard let v = s.defaults.string(forKey: s.k(t, k)) else { return false }
    let buf = UnsafeMutableRawPointer.allocate(byteCount: v.utf8.count, alignment: 1)
    v.withCString { ptr in buf.copyMemory(from: ptr, byteCount: v.utf8.count) }
    outP.pointee = buf; outL.pointee = v.utf8.count; return true
}

@_cdecl("kv_store_set_int")
public func kvStoreSetInt(_ storePtr: UnsafeMutableRawPointer, _ tP: UnsafePointer<UInt8>, _ tL: Int, _ kP: UnsafePointer<UInt8>, _ kL: Int, _ v: Int) {
    guard let s = kvStores[storePtr] else { return }
    let t = String(bytes: UnsafeBufferPointer(start: tP, count: tL), encoding: .utf8) ?? ""
    let k = String(bytes: UnsafeBufferPointer(start: kP, count: kL), encoding: .utf8) ?? ""
    s.defaults.set(v, forKey: s.k(t, k))
}

@_cdecl("kv_store_get_int")
public func kvStoreGetInt(_ storePtr: UnsafeMutableRawPointer, _ tP: UnsafePointer<UInt8>, _ tL: Int, _ kP: UnsafePointer<UInt8>, _ kL: Int) -> Int {
    guard let s = kvStores[storePtr] else { return 0 }
    let t = String(bytes: UnsafeBufferPointer(start: tP, count: tL), encoding: .utf8) ?? ""
    let k = String(bytes: UnsafeBufferPointer(start: kP, count: kL), encoding: .utf8) ?? ""
    return s.defaults.integer(forKey: s.k(t, k))
}

@_cdecl("kv_store_set_bool")
public func kvStoreSetBool(_ storePtr: UnsafeMutableRawPointer, _ tP: UnsafePointer<UInt8>, _ tL: Int, _ kP: UnsafePointer<UInt8>, _ kL: Int, _ v: Bool) {
    guard let s = kvStores[storePtr] else { return }
    let t = String(bytes: UnsafeBufferPointer(start: tP, count: tL), encoding: .utf8) ?? ""
    let k = String(bytes: UnsafeBufferPointer(start: kP, count: kL), encoding: .utf8) ?? ""
    s.defaults.set(v, forKey: s.k(t, k))
}

@_cdecl("kv_store_get_bool")
public func kvStoreGetBool(_ storePtr: UnsafeMutableRawPointer, _ tP: UnsafePointer<UInt8>, _ tL: Int, _ kP: UnsafePointer<UInt8>, _ kL: Int) -> Bool {
    guard let s = kvStores[storePtr] else { return false }
    let t = String(bytes: UnsafeBufferPointer(start: tP, count: tL), encoding: .utf8) ?? ""
    let k = String(bytes: UnsafeBufferPointer(start: kP, count: kL), encoding: .utf8) ?? ""
    return s.defaults.bool(forKey: s.k(t, k))
}

@_cdecl("kv_store_delete")
public func kvStoreDelete(_ storePtr: UnsafeMutableRawPointer, _ tP: UnsafePointer<UInt8>, _ tL: Int, _ kP: UnsafePointer<UInt8>, _ kL: Int) {
    guard let s = kvStores[storePtr] else { return }
    let t = String(bytes: UnsafeBufferPointer(start: tP, count: tL), encoding: .utf8) ?? ""
    let k = String(bytes: UnsafeBufferPointer(start: kP, count: kL), encoding: .utf8) ?? ""
    s.defaults.removeObject(forKey: s.k(t, k))
}

@_cdecl("kv_store_release")
public func kvStoreRelease(_ ptr: UnsafeMutableRawPointer) {
    kvStores.removeValue(forKey: ptr)
    Unmanaged<AnyObject>.fromOpaque(ptr).release()
}

// ═══════════════════════════════════════════════════════════════════════════
// Combine — Publisher/Subscriber bridge
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("combine_subject_create")
public func combineSubjectCreate() -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(PassthroughSubject<Int, Never>() as AnyObject).toOpaque()
}

@_cdecl("combine_subject_send")
public func combineSubjectSend(_ ptr: UnsafeMutableRawPointer, _ value: Int) {
    let subject = Unmanaged<AnyObject>.fromOpaque(ptr).takeUnretainedValue() as! PassthroughSubject<Int, Never>
    subject.send(value)
}

@_cdecl("combine_subject_subscribe")
public func combineSubjectSubscribe(_ ptr: UnsafeMutableRawPointer, _ cb: @convention(c) (Int, UnsafeMutableRawPointer?) -> Void, _ ud: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer {
    let subject = Unmanaged<AnyObject>.fromOpaque(ptr).takeUnretainedValue() as! PassthroughSubject<Int, Never>
    let cancellable = subject.sink { value in cb(value, ud) }
    return Unmanaged.passRetained(cancellable as AnyObject).toOpaque()
}

@_cdecl("combine_cancel")
public func combineCancel(_ ptr: UnsafeMutableRawPointer) { Unmanaged<AnyObject>.fromOpaque(ptr).release() }

@_cdecl("combine_current_value_create")
public func combineCurrentValueCreate(_ initial: Int) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(CurrentValueSubject<Int, Never>(initial) as AnyObject).toOpaque()
}

@_cdecl("combine_current_value_get")
public func combineCurrentValueGet(_ ptr: UnsafeMutableRawPointer) -> Int {
    (Unmanaged<AnyObject>.fromOpaque(ptr).takeUnretainedValue() as! CurrentValueSubject<Int, Never>).value
}

@_cdecl("combine_current_value_set")
public func combineCurrentValueSet(_ ptr: UnsafeMutableRawPointer, _ value: Int) {
    (Unmanaged<AnyObject>.fromOpaque(ptr).takeUnretainedValue() as! CurrentValueSubject<Int, Never>).send(value)
}

@_cdecl("combine_release")
public func combineRelease(_ ptr: UnsafeMutableRawPointer) { Unmanaged<AnyObject>.fromOpaque(ptr).release() }
