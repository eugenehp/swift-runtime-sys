import Foundation
import Dispatch
#if canImport(Distributed)
import Distributed
#endif

#if canImport(Distributed)
@available(macOS 13.0, *)
distributed actor O9ImplProbeActor {
    typealias ActorSystem = LocalTestingDistributedActorSystem
    private var sequence: Int32 = 0

    distributed func metadataName() -> String {
        String(reflecting: O9ImplProbeActor.self)
    }

    distributed func echo(_ value: Int32) -> Int32 {
        value
    }

    distributed func failWithMarker() throws -> Int32 {
        throw O9ImplProbeError.synthetic
    }

    distributed func nextSequence() -> Int32 {
        sequence &+= 1
        return sequence
    }
}

@available(macOS 13.0, *)
enum O9ImplProbeError: Error {
    case synthetic
}

@available(macOS 13.0, *)
private func o9ImplRunAsync(_ body: @escaping () async throws -> Int32) -> Int32 {
    let semaphore = DispatchSemaphore(value: 0)
    var result: Int32 = 0

    Task {
        do {
            result = try await body()
        } catch {
            result = 0
        }
        semaphore.signal()
    }

    semaphore.wait()
    return result
}
#endif

private func o9ImplRuntimeReady() -> Bool {
#if canImport(Distributed)
    if #available(macOS 13.0, *) {
        return true
    }
#endif
    return false
}

@_cdecl("swift_contract_o9_impl_probe_version")
public func swift_contract_o9_impl_probe_version() -> Int32 {
    1
}

@_cdecl("swift_contract_o9_impl_runtime_ready")
public func swift_contract_o9_impl_runtime_ready() -> Int32 {
    o9ImplRuntimeReady() ? 1 : 0
}

@_cdecl("swift_contract_o9_impl_metadata_descriptor_probe")
public func swift_contract_o9_impl_metadata_descriptor_probe() -> Int32 {
#if canImport(Distributed)
    guard #available(macOS 13.0, *) else { return 0 }
    return o9ImplRunAsync {
        let actor = O9ImplProbeActor(actorSystem: LocalTestingDistributedActorSystem())
        let name = try await actor.metadataName()
        return name.contains("O9ImplProbeActor") ? 1 : 0
    }
#else
    return 0
#endif
}

@_cdecl("swift_contract_o9_impl_distributed_invocation_probe")
public func swift_contract_o9_impl_distributed_invocation_probe() -> Int32 {
#if canImport(Distributed)
    guard #available(macOS 13.0, *) else { return 0 }
    return o9ImplRunAsync {
        let actor = O9ImplProbeActor(actorSystem: LocalTestingDistributedActorSystem())
        let value = try await actor.echo(41)
        return value == 41 ? 1 : 0
    }
#else
    return 0
#endif
}

@_cdecl("swift_contract_o9_impl_result_handling_probe")
public func swift_contract_o9_impl_result_handling_probe() -> Int32 {
#if canImport(Distributed)
    guard #available(macOS 13.0, *) else { return 0 }
    return o9ImplRunAsync {
        let actor = O9ImplProbeActor(actorSystem: LocalTestingDistributedActorSystem())
        do {
            _ = try await actor.failWithMarker()
            return 0
        } catch O9ImplProbeError.synthetic {
            return 1
        } catch {
            return 0
        }
    }
#else
    return 0
#endif
}

@_cdecl("swift_contract_o9_impl_isolation_semantics_probe")
public func swift_contract_o9_impl_isolation_semantics_probe() -> Int32 {
#if canImport(Distributed)
    guard #available(macOS 13.0, *) else { return 0 }
    return o9ImplRunAsync {
        let actor = O9ImplProbeActor(actorSystem: LocalTestingDistributedActorSystem())
        let first = try await actor.nextSequence()
        let second = try await actor.nextSequence()
        return (first == 1 && second == 2) ? 1 : 0
    }
#else
    return 0
#endif
}
