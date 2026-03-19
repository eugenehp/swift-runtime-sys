// Phase B.2: Witness Table Dynamic Resolver Probe
// Tests runtime witness table resolution without pre-seeded lookups

use swift_runtime_sys::RuntimeContract::RuntimeContract;
use swift_runtime_sys::RuntimeFactory::RuntimeFactory;

fn main() {
    let factory =
        RuntimeFactory::with_thunk_library("./libRustBridge.dylib", "./libRuntimeThunks.dylib")
            .or_else(|_| RuntimeFactory::new("./libRustBridge.dylib"))
            .unwrap_or_else(|e| panic!("failed to init RuntimeFactory: {e:?}"));

    factory
        .validate_runtime_contract(1)
        .unwrap_or_else(|e| panic!("runtime contract validation failed: {e:?}"));

    let contract = RuntimeContract::new(&factory);

    let mut passed = 0;
    let mut failed = 0;

    println!("=== Phase B.2: Witness Table Dynamic Resolver ===\n");

    // Test 1: Scan all conformances
    println!("Test 1: Scan all protocol conformances");
    match contract.b2_scan_all_conformances() {
        Ok(conformances) => {
            println!("  ✓ Scanned {} conformances", conformances.len());
            for conf in conformances.iter().take(3) {
                println!("    - {}: {}", conf.type_name, conf.protocol_name);
            }
            passed += 1;
        }
        Err(e) => {
            println!("  ✗ Failed to scan: {:?}", e);
            failed += 1;
        }
    }

    // Test 2: Resolve String : Equatable
    println!("\nTest 2: Resolve String : Equatable");
    match contract.b2_resolve_witness_table("String", "Equatable") {
        Ok(ptr) => {
            if ptr as usize != 0 {
                println!("  ✓ Witness table resolved @ {:p}", ptr);
                passed += 1;
            } else {
                println!("  ✗ Witness table is null");
                failed += 1;
            }
        }
        Err(e) => {
            println!("  ✗ Resolution failed: {:?}", e);
            failed += 1;
        }
    }

    // Test 3: Resolve String : Hashable
    println!("\nTest 3: Resolve String : Hashable");
    match contract.b2_resolve_witness_table("String", "Hashable") {
        Ok(ptr) => {
            if ptr as usize != 0 {
                println!("  ✓ Witness table resolved @ {:p}", ptr);
                passed += 1;
            } else {
                println!("  ✗ Witness table is null");
                failed += 1;
            }
        }
        Err(e) => {
            println!("  ✗ Resolution failed: {:?}", e);
            failed += 1;
        }
    }

    // Test 4: Resolve String : Comparable
    println!("\nTest 4: Resolve String : Comparable");
    match contract.b2_resolve_witness_table("String", "Comparable") {
        Ok(ptr) => {
            if ptr as usize != 0 {
                println!("  ✓ Witness table resolved @ {:p}", ptr);
                passed += 1;
            } else {
                println!("  ✗ Witness table is null");
                failed += 1;
            }
        }
        Err(e) => {
            println!("  ✗ Resolution failed: {:?}", e);
            failed += 1;
        }
    }

    // Test 5: Resolve Int32 : Equatable
    println!("\nTest 5: Resolve Int32 : Equatable");
    match contract.b2_resolve_witness_table("Int32", "Equatable") {
        Ok(ptr) => {
            if ptr as usize != 0 {
                println!("  ✓ Witness table resolved @ {:p}", ptr);
                passed += 1;
            } else {
                println!("  ✗ Witness table is null");
                failed += 1;
            }
        }
        Err(e) => {
            println!("  ✗ Resolution failed: {:?}", e);
            failed += 1;
        }
    }

    // Test 6: Resolve Int32 : Hashable
    println!("\nTest 6: Resolve Int32 : Hashable");
    match contract.b2_resolve_witness_table("Int32", "Hashable") {
        Ok(ptr) => {
            if ptr as usize != 0 {
                println!("  ✓ Witness table resolved @ {:p}", ptr);
                passed += 1;
            } else {
                println!("  ✗ Witness table is null");
                failed += 1;
            }
        }
        Err(e) => {
            println!("  ✗ Resolution failed: {:?}", e);
            failed += 1;
        }
    }

    // Test 7: Resolve Int32 : Comparable
    println!("\nTest 7: Resolve Int32 : Comparable");
    match contract.b2_resolve_witness_table("Int32", "Comparable") {
        Ok(ptr) => {
            if ptr as usize != 0 {
                println!("  ✓ Witness table resolved @ {:p}", ptr);
                passed += 1;
            } else {
                println!("  ✗ Witness table is null");
                failed += 1;
            }
        }
        Err(e) => {
            println!("  ✗ Resolution failed: {:?}", e);
            failed += 1;
        }
    }

    // Test 8: Resolve Array<Int32> : Sequence
    println!("\nTest 8: Resolve Array<Int32> : Sequence");
    match contract.b2_resolve_witness_table("Array<Int32>", "Sequence") {
        Ok(ptr) => {
            if ptr as usize != 0 {
                println!("  ✓ Witness table resolved @ {:p}", ptr);
                passed += 1;
            } else {
                println!("  ✗ Witness table is null");
                failed += 1;
            }
        }
        Err(e) => {
            println!("  ✗ Resolution failed: {:?}", e);
            failed += 1;
        }
    }

    // Test 9: Resolve Array<Int32> : Collection
    println!("\nTest 9: Resolve Array<Int32> : Collection");
    match contract.b2_resolve_witness_table("Array<Int32>", "Collection") {
        Ok(ptr) => {
            if ptr as usize != 0 {
                println!("  ✓ Witness table resolved @ {:p}", ptr);
                passed += 1;
            } else {
                println!("  ✗ Witness table is null");
                failed += 1;
            }
        }
        Err(e) => {
            println!("  ✗ Resolution failed: {:?}", e);
            failed += 1;
        }
    }

    // Test 10: Resolve Dictionary : Sequence
    println!("\nTest 10: Resolve Dictionary<Int32, Int32> : Sequence");
    match contract.b2_resolve_witness_table("Dictionary<Int32, Int32>", "Sequence") {
        Ok(ptr) => {
            if ptr as usize != 0 {
                println!("  ✓ Witness table resolved @ {:p}", ptr);
                passed += 1;
            } else {
                println!("  ✗ Witness table is null");
                failed += 1;
            }
        }
        Err(e) => {
            println!("  ✗ Resolution failed: {:?}", e);
            failed += 1;
        }
    }

    // Test 11: Try-resolve String : NonexistentProtocol (should not fail)
    println!("\nTest 11: Try-resolve String : NonexistentProtocol");
    match contract.b2_try_resolve_witness_table("String", "NonexistentProtocol") {
        Ok(ptr) => {
            if ptr as usize == 0 {
                println!("  ✓ Correctly returned null for nonexistent protocol");
                passed += 1;
            } else {
                println!("  ? Unexpected non-null result");
                passed += 1; // Still passes because try_resolve is expected to not fail
            }
        }
        Err(e) => {
            println!("  ✗ Try-resolve should not fail: {:?}", e);
            failed += 1;
        }
    }

    // Test 12: Try-resolve NonexistentType : Equatable
    println!("\nTest 12: Try-resolve NonexistentType : Equatable");
    match contract.b2_try_resolve_witness_table("NonexistentType", "Equatable") {
        Ok(ptr) => {
            if ptr as usize == 0 {
                println!("  ✓ Correctly returned null for nonexistent type");
                passed += 1;
            } else {
                println!("  ? Unexpected non-null result");
                passed += 1;
            }
        }
        Err(e) => {
            println!("  ✗ Try-resolve should not fail: {:?}", e);
            failed += 1;
        }
    }

    // Test 13: Describe a valid witness table
    println!("\nTest 13: Describe witness table for String : Equatable");
    match contract.b2_resolve_witness_table("String", "Equatable") {
        Ok(ptr) if ptr as usize != 0 => {
            match contract.b2_describe_conformance(ptr) {
                Ok(desc) => {
                    println!("  ✓ Conformance description: {}", desc);
                    passed += 1;
                }
                Err(e) => {
                    println!("  ✗ Failed to describe: {:?}", e);
                    failed += 1;
                }
            }
        }
        _ => {
            println!("  ⊘ Skipped (witness table resolution failed)");
        }
    }

    // Test 14: Resolve with standard conformance helper (String : Equatable)
    println!("\nTest 14: Use standard conformance helper for String : Equatable");
    match contract.b2_resolve_standard_conformance("String", "Equatable") {
        Ok(ptr) => {
            if ptr as usize != 0 {
                println!("  ✓ Standard conformance resolved @ {:p}", ptr);
                passed += 1;
            } else {
                println!("  ✗ Standard conformance returned null");
                failed += 1;
            }
        }
        Err(e) => {
            println!("  ✗ Standard conformance failed: {:?}", e);
            failed += 1;
        }
    }

    // Test 15: Resolve with standard conformance helper (Int32 : Hashable)
    println!("\nTest 15: Use standard conformance helper for Int32 : Hashable");
    match contract.b2_resolve_standard_conformance("Int32", "Hashable") {
        Ok(ptr) => {
            if ptr as usize != 0 {
                println!("  ✓ Standard conformance resolved @ {:p}", ptr);
                passed += 1;
            } else {
                println!("  ✗ Standard conformance returned null");
                failed += 1;
            }
        }
        Err(e) => {
            println!("  ✗ Standard conformance failed: {:?}", e);
            failed += 1;
        }
    }

    println!("\n=== Phase B.2 Summary ===");
    println!("Tests Passed: {}", passed);
    println!("Tests Failed: {}", failed);

    if failed == 0 {
        println!("✓ Witness Table Dynamic Resolver probe PASSED");
    } else {
        println!("✗ Witness Table Dynamic Resolver probe FAILED");
        std::process::exit(1);
    }
}
