extern crate swift_runtime_sys;

use swift_runtime_sys::RuntimeContract::{RuntimeContract, RuntimeContractError};
use swift_runtime_sys::RuntimeFactory::RuntimeFactory;

fn main() {
    let factory =
        RuntimeFactory::with_thunk_library("./libRustBridge.dylib", "./libRuntimeThunks.dylib")
            .or_else(|_| RuntimeFactory::new("./libRustBridge.dylib"))
            .unwrap_or_else(|e| panic!("failed to init RuntimeFactory: {e:?}"));

    let _descriptor = factory
        .validate_runtime_contract(1)
        .unwrap_or_else(|e| panic!("runtime contract validation failed: {e:?}"));

    let contract = RuntimeContract::new(&factory);

    let mut passed = 0;
    let mut total = 0;

    total += 1;
    match test_empty_array(&contract) {
        Ok(success) if success => {
            passed += 1;
            println!("✓ Empty array");
        }
        Ok(_) => println!("✗ Empty array (returned false)"),
        Err(e) => println!("✗ Empty array (error: {:?})", e),
    }
    total += 1;
    if test_single_element(&contract).unwrap_or(false) {
        passed += 1;
        println!("✓ Single element");
    } else {
        println!("✗ Single element");
    }

    total += 1;
    if test_multi_element(&contract).unwrap_or(false) {
        passed += 1;
        println!("✓ Multi-element");
    } else {
        println!("✗ Multi-element");
    }

    total += 1;
    if test_array_operations(&contract).unwrap_or(false) {
        passed += 1;
        println!("✓ Array operations");
    } else {
        println!("✗ Array operations");
    }

    total += 1;
    if test_capacity_expansion(&contract).unwrap_or(false) {
        passed += 1;
        println!("✓ Capacity expansion");
    } else {
        println!("✗ Capacity expansion");
    }

    total += 1;
    if test_pointer_iteration(&contract).unwrap_or(false) {
        passed += 1;
        println!("✓ Pointer iteration");
    } else {
        println!("✗ Pointer iteration");
    }

    total += 1;
    if test_opaque_ref_array_basic(&contract).unwrap_or(false) {
        passed += 1;
        println!("✓ OpaqueRef array basics");
    } else {
        println!("✗ OpaqueRef array basics");
    }

    total += 1;
    if test_opaque_ref_array_set(&contract).unwrap_or(false) {
        passed += 1;
        println!("✓ OpaqueRef array set/get");
    } else {
        println!("✗ OpaqueRef array set/get");
    }

    println!("Array parity probe results: {}/{} PASS", passed, total);
    if passed != total {
        std::process::exit(1);
    }
}

/// Test empty array creation and length.
fn test_empty_array(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let array = contract.construct_array(0)?;
    let len = contract.array_len(array.object)?;
    contract.release(array)?;
    ok(len == 0)
}

/// Test single element: create, append, verify.
fn test_single_element(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let array = contract.construct_array(1)?;

    let new_count = contract.array_append(array.object, 42)?;
    if new_count != 1 {
        contract.release(array)?;
        return ok(false);
    }

    let len = contract.array_len(array.object)?;
    if len != 1 {
        contract.release(array)?;
        return ok(false);
    }

    let value = contract.array_get(array.object, 0)?;
    contract.release(array)?;

    ok(value == 42)
}

/// Test multiple elements: create, append many, verify all.
fn test_multi_element(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let array = contract.construct_array(5)?;

    // Append 5 values
    for i in 0..5 {
        contract.array_append(array.object, i * 10)?;
    }

    let len = contract.array_len(array.object)?;
    if len != 5 {
        contract.release(array)?;
        return ok(false);
    }

    // Verify each value
    for i in 0..5 {
        let value = contract.array_get(array.object, i)?;
        if value != i * 10 {
            contract.release(array)?;
            return ok(false);
        }
    }

    contract.release(array)?;
    ok(true)
}

/// Test array mutation: create, set, get, verify.
fn test_array_operations(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let array = contract.construct_array(3)?;

    // Append 3 values
    contract.array_append(array.object, 1)?;
    contract.array_append(array.object, 2)?;
    contract.array_append(array.object, 3)?;

    // Modify index 1 from 2 to 20
    contract.array_set(array.object, 1, 20)?;

    // Verify all values
    let v0 = contract.array_get(array.object, 0)?;
    let v1 = contract.array_get(array.object, 1)?;
    let v2 = contract.array_get(array.object, 2)?;

    contract.release(array)?;

    ok(v0 == 1 && v1 == 20 && v2 == 3)
}

/// Test capacity growth: create with 0, append many, verify all.
fn test_capacity_expansion(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let array = contract.construct_array(0)?;

    // Append 10 values to initially empty array
    for i in 0..10 {
        contract.array_append(array.object, i)?;
    }

    let len = contract.array_len(array.object)?;
    if len != 10 {
        contract.release(array)?;
        return ok(false);
    }

    // Verify all values
    for i in 0..10 {
        let value = contract.array_get(array.object, i)?;
        if value != i {
            contract.release(array)?;
            return ok(false);
        }
    }

    contract.release(array)?;
    ok(true)
}

/// Test contiguous pointer export for read-only iteration.
fn test_pointer_iteration(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let array = contract.construct_array(4)?;

    for value in [7, 11, 13, 17] {
        contract.array_append(array.object, value)?;
    }

    let values = contract.array_elements_via_pointer(array.object)?;
    contract.release(array)?;

    ok(values == vec![7, 11, 13, 17])
}

/// Test opaque reference array append/get/len behavior.
fn test_opaque_ref_array_basic(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let array = contract.construct_array_ref(2)?;
    let str_a = contract.construct_string(b"A")?;
    let str_b = contract.construct_string(b"B")?;

    contract.array_ref_append(array.object, str_a.object)?;
    contract.array_ref_append(array.object, str_b.object)?;

    let len = contract.array_ref_len(array.object)?;
    if len != 2 {
        contract.release(array)?;
        contract.release(str_a)?;
        contract.release(str_b)?;
        return ok(false);
    }

    let first = contract.array_ref_get(array.object, 0)?;
    let second = contract.array_ref_get(array.object, 1)?;

    contract.release(array)?;
    contract.release(str_a)?;
    contract.release(str_b)?;

    ok(first == str_a.object && second == str_b.object)
}

/// Test opaque reference array set behavior.
fn test_opaque_ref_array_set(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let array = contract.construct_array_ref(2)?;
    let str_a = contract.construct_string(b"alpha")?;
    let str_b = contract.construct_string(b"beta")?;

    contract.array_ref_append(array.object, str_a.object)?;
    contract.array_ref_append(array.object, str_b.object)?;
    contract.array_ref_set(array.object, 1, str_a.object)?;

    let updated = contract.array_ref_get(array.object, 1)?;

    contract.release(array)?;
    contract.release(str_a)?;
    contract.release(str_b)?;

    ok(updated == str_a.object)
}

/// Helper to convert bool to Result.
fn ok(result: bool) -> Result<bool, RuntimeContractError> {
    Ok(result)
}
