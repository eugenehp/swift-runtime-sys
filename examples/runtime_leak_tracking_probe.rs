/// Leak tracking and root-cause attribution probe for Track K.3.
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

    println!("\n=== Leak Tracking & Root Cause (Track K.3) ===");

    let tests: [(&str, fn(&RuntimeContract) -> Result<bool, RuntimeContractError>); 7] = [
        ("Tracker reset starts empty", test_reset_empty),
        ("Allocate increases unreleased count", test_allocate_increases_count),
        ("Release decreases unreleased count", test_release_decreases_count),
        ("Live count by site accumulates", test_live_count_per_site),
        ("Root cause site selects max live", test_root_cause_max_site),
        ("Sweep finds unreleased tokens", test_sweep_detects_leaks),
        ("Full release returns to zero", test_full_release_zero),
    ];

    for (name, test_fn) in tests {
        match test_fn(&contract) {
            Ok(true) => {
                println!("✓ {name} PASS");
                passed += 1;
            }
            Ok(false) => {
                println!("✗ {name} FAIL");
                failed += 1;
            }
            Err(err) => {
                println!("✗ {name} FAIL ({err:?})");
                failed += 1;
            }
        }
    }

    println!("\n=== Track K.3 Summary ===");
    println!("Tests Passed: {}", passed);
    println!("Tests Failed: {}", failed);

    if failed == 0 {
        println!("✓ All Track K.3 tests PASSED");
    } else {
        panic!("✗ Track K.3 tests FAILED");
    }
}

fn test_reset_empty(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.k3_tracker_reset()?;
    Ok(contract.k3_sweep_unreleased_count()? == 0)
}

fn test_allocate_increases_count(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.k3_tracker_reset()?;
    let _a = contract.k3_alloc(10)?;
    Ok(contract.k3_sweep_unreleased_count()? == 1)
}

fn test_release_decreases_count(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.k3_tracker_reset()?;
    let a = contract.k3_alloc(10)?;
    let released = contract.k3_release(a)?;
    Ok(released && contract.k3_sweep_unreleased_count()? == 0)
}

fn test_live_count_per_site(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.k3_tracker_reset()?;
    let _a = contract.k3_alloc(21)?;
    let _b = contract.k3_alloc(21)?;
    Ok(contract.k3_live_count_for_site(21)? == 2)
}

fn test_root_cause_max_site(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.k3_tracker_reset()?;
    let _a = contract.k3_alloc(1)?;
    let _b = contract.k3_alloc(2)?;
    let _c = contract.k3_alloc(2)?;
    Ok(contract.k3_root_cause_site()? == 2)
}

fn test_sweep_detects_leaks(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.k3_tracker_reset()?;
    let _a = contract.k3_alloc(7)?;
    let _b = contract.k3_alloc(7)?;
    Ok(contract.k3_sweep_unreleased_count()? == 2)
}

fn test_full_release_zero(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.k3_tracker_reset()?;
    let a = contract.k3_alloc(5)?;
    let b = contract.k3_alloc(5)?;
    let c = contract.k3_alloc(6)?;
    let r1 = contract.k3_release(a)?;
    let r2 = contract.k3_release(b)?;
    let r3 = contract.k3_release(c)?;
    Ok(r1 && r2 && r3 && contract.k3_sweep_unreleased_count()? == 0)
}
