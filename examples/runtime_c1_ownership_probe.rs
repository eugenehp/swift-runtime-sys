/// Phase C.1 probe: ownership, lifetime, and allocator safety hardening.
use swift_runtime_sys::RuntimeContract::{RuntimeContract, RuntimeContractError};
use swift_runtime_sys::RuntimeFactory::RuntimeFactory;

fn main() {
    let factory = RuntimeFactory::with_thunk_library("./libRustBridge.dylib", "./libRuntimeThunks.dylib")
        .or_else(|_| RuntimeFactory::new("./libRustBridge.dylib"))
        .unwrap_or_else(|e| panic!("failed to init RuntimeFactory: {e:?}"));

    factory
        .validate_runtime_contract(1)
        .unwrap_or_else(|e| panic!("runtime contract validation failed: {e:?}"));

    let contract = RuntimeContract::new(&factory);
    let mut passed = 0;
    let mut failed = 0;

    println!("\n=== Phase C.1 Ownership/Lifetime Hardening ===");

    let tests: [(&str, fn(&RuntimeContract) -> Result<bool, RuntimeContractError>); 10] = [
        ("Weak lifecycle clears after drop", test_weak_lifecycle),
        ("Unowned dangling is detected", test_unowned_dangling),
        ("Retain delta remains positive", test_retain_delta),
        ("Allocator tracker reset starts empty", test_tracker_reset),
        ("Allocate increments unreleased count", test_alloc_increments),
        ("Release decrements unreleased count", test_release_decrements),
        ("Double release is safely rejected", test_double_release_guard),
        ("Per-site live counters are stable", test_live_count_site),
        ("Tagged alloc usage increments", test_m3_usage_increment),
        ("Tagged alloc release returns usage to zero", test_m3_usage_cleanup),
    ];

    for (name, f) in tests {
        match f(&contract) {
            Ok(true) => {
                println!("PASS: {name}");
                passed += 1;
            }
            Ok(false) => {
                println!("FAIL: {name}");
                failed += 1;
            }
            Err(err) => {
                println!("FAIL: {name} ({err:?})");
                failed += 1;
            }
        }
    }

    println!("\n=== C.1 Summary ===");
    println!("Passed: {}/{}", passed, passed + failed);
    if failed == 0 {
        println!("Status: ALL TESTS PASSED");
    } else {
        println!("Status: {} TESTS FAILED", failed);
        std::process::exit(1);
    }
}

fn test_weak_lifecycle(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    c.k1_weak_lifecycle()
}

fn test_unowned_dangling(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    c.k1_unowned_dangling_detected()
}

fn test_retain_delta(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(c.k2_retain_delta()? >= 1)
}

fn test_tracker_reset(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    c.k3_tracker_reset()?;
    Ok(c.k3_sweep_unreleased_count()? == 0)
}

fn test_alloc_increments(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    c.k3_tracker_reset()?;
    let _ = c.k3_alloc(1001)?;
    Ok(c.k3_sweep_unreleased_count()? == 1)
}

fn test_release_decrements(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    c.k3_tracker_reset()?;
    let token = c.k3_alloc(1002)?;
    let released = c.k3_release(token)?;
    Ok(released && c.k3_sweep_unreleased_count()? == 0)
}

fn test_double_release_guard(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    c.k3_tracker_reset()?;
    let token = c.k3_alloc(1003)?;
    let first = c.k3_release(token)?;
    let second = c.k3_release(token)?;
    Ok(first && !second)
}

fn test_live_count_site(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    c.k3_tracker_reset()?;
    let _a = c.k3_alloc(2001)?;
    let _b = c.k3_alloc(2001)?;
    let _c = c.k3_alloc(2002)?;
    Ok(c.k3_live_count_for_site(2001)? == 2 && c.k3_live_count_for_site(2002)? == 1)
}

fn test_m3_usage_increment(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    c.m3_reset()?;
    let _token = c.m3_tag_alloc("c1-ownership", 96)?;
    Ok(c.m3_usage_for_subsystem("c1-ownership")? == 96)
}

fn test_m3_usage_cleanup(c: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    c.m3_reset()?;
    let token = c.m3_tag_alloc("c1-ownership", 96)?;
    let ok = c.m3_release_alloc(token)?;
    Ok(ok && c.m3_usage_for_subsystem("c1-ownership")? == 0)
}
