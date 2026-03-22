#if canImport(FoundationModels)
import FoundationModels
import Foundation

// ═══════════════════════════════════════════════════════════════════════════
// Availability
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("fm_available")
public func fmAvailable() -> Bool {
    SystemLanguageModel.default.isAvailable
}

// ═══════════════════════════════════════════════════════════════════════════
// Session management
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("fm_session_create")
public func fmSessionCreate(
    _ instructionsPtr: UnsafePointer<UInt8>?, _ instructionsLen: Int
) -> UnsafeMutableRawPointer? {
    let instructions: String? = instructionsPtr.flatMap { ptr in
        instructionsLen > 0 ? String(bytes: UnsafeBufferPointer(start: ptr, count: instructionsLen), encoding: .utf8) : nil
    }
    let session: LanguageModelSession
    if let inst = instructions {
        session = LanguageModelSession(instructions: Instructions(inst))
    } else {
        session = LanguageModelSession()
    }
    return Unmanaged.passRetained(session).toOpaque()
}

@_cdecl("fm_session_release")
public func fmSessionRelease(_ ptr: UnsafeMutableRawPointer) {
    Unmanaged<LanguageModelSession>.fromOpaque(ptr).release()
}

// ═══════════════════════════════════════════════════════════════════════════
// Generate (blocking — runs async internally)
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("fm_respond")
public func fmRespond(
    _ sessionPtr: UnsafeMutableRawPointer,
    _ promptPtr: UnsafePointer<UInt8>, _ promptLen: Int,
    _ outPtr: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outLen: UnsafeMutablePointer<Int>
) -> Bool {
    let session = Unmanaged<LanguageModelSession>.fromOpaque(sessionPtr).takeUnretainedValue()
    let prompt = String(bytes: UnsafeBufferPointer(start: promptPtr, count: promptLen), encoding: .utf8) ?? ""
    
    let sem = DispatchSemaphore(value: 0)
    var result: String?
    
    Task {
        do {
            let response = try await session.respond(to: prompt)
            result = response.content
        } catch {
            result = "Error: \(error.localizedDescription)"
        }
        sem.signal()
    }
    
    sem.wait()
    
    guard let text = result else { return false }
    let buf = UnsafeMutableRawPointer.allocate(byteCount: text.utf8.count, alignment: 1)
    text.withCString { ptr in buf.copyMemory(from: ptr, byteCount: text.utf8.count) }
    outPtr.pointee = buf; outLen.pointee = text.utf8.count
    return true
}

// ═══════════════════════════════════════════════════════════════════════════
// Stream (calls back per token)
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("fm_stream_respond")
public func fmStreamRespond(
    _ sessionPtr: UnsafeMutableRawPointer,
    _ promptPtr: UnsafePointer<UInt8>, _ promptLen: Int,
    _ tokenCb: @convention(c) (UnsafePointer<UInt8>, Int, UnsafeMutableRawPointer?) -> Void,
    _ doneCb: @convention(c) (UnsafeMutableRawPointer?) -> Void,
    _ ud: UnsafeMutableRawPointer?
) {
    let session = Unmanaged<LanguageModelSession>.fromOpaque(sessionPtr).takeUnretainedValue()
    let prompt = String(bytes: UnsafeBufferPointer(start: promptPtr, count: promptLen), encoding: .utf8) ?? ""
    let onToken = tokenCb; let onDone = doneCb; let userData = ud
    
    Task {
        do {
            let stream = session.streamResponse(to: prompt)
            for try await partial in stream {
                let text = partial.content
                text.withCString { ptr in
                    onToken(UnsafePointer(OpaquePointer(ptr)), text.utf8.count, userData)
                }
            }
        } catch {
            let msg = "Error: \(error.localizedDescription)"
            msg.withCString { ptr in
                onToken(UnsafePointer(OpaquePointer(ptr)), msg.utf8.count, userData)
            }
        }
        onDone(userData)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Model info
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("fm_is_responding")
public func fmIsResponding(_ sessionPtr: UnsafeMutableRawPointer) -> Bool {
    Unmanaged<LanguageModelSession>.fromOpaque(sessionPtr).takeUnretainedValue().isResponding
}

#endif
