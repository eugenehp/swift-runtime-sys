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
    if test_empty_dictionary(&contract).unwrap_or(false) {
        passed += 1;
        println!("PASS: Empty dictionary");
    } else {
        println!("FAIL: Empty dictionary");
    }

    total += 1;
    if test_insert_and_get(&contract).unwrap_or(false) {
        passed += 1;
        println!("PASS: Insert and get");
    } else {
        println!("FAIL: Insert and get");
    }

    total += 1;
    if test_upsert_updates_value(&contract).unwrap_or(false) {
        passed += 1;
        println!("PASS: Upsert updates value");
    } else {
        println!("FAIL: Upsert updates value");
    }

    total += 1;
    if test_remove_existing_key(&contract).unwrap_or(false) {
        passed += 1;
        println!("PASS: Remove existing key");
    } else {
        println!("FAIL: Remove existing key");
    }

    total += 1;
    if test_missing_key_paths(&contract).unwrap_or(false) {
        passed += 1;
        println!("PASS: Missing key paths");
    } else {
        println!("FAIL: Missing key paths");
    }

    total += 1;
    if test_hash_collision_safety(&contract).unwrap_or(false) {
        passed += 1;
        println!("PASS: Hash collision safety (50 keys)");
    } else {
        println!("FAIL: Hash collision safety (50 keys)");
    }

    total += 1;
    if test_dict_opaque_ref_basic(&contract).unwrap_or(false) {
        passed += 1;
        println!("PASS: Dict<Int32,OpaqueRef> basic");
    } else {
        println!("FAIL: Dict<Int32,OpaqueRef> basic");
    }

    total += 1;
    if test_dict_opaque_ref_remove(&contract).unwrap_or(false) {
        passed += 1;
        println!("PASS: Dict<Int32,OpaqueRef> remove");
    } else {
        println!("FAIL: Dict<Int32,OpaqueRef> remove");
    }

    println!("Dictionary parity probe results: {}/{} PASS", passed, total);
    if passed != total {
        std::process::exit(1);
    }
}

fn test_empty_dictionary(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let dict = contract.construct_dict_i32_i32(0)?;
    let len = contract.dict_i32_i32_len(dict.object)?;
    let contains = contract.dict_i32_i32_contains(dict.object, 42)?;

    contract.release(dict)?;
    Ok(len == 0 && !contains)
}

fn test_insert_and_get(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let dict = contract.construct_dict_i32_i32(2)?;

    let count = contract.dict_i32_i32_set(dict.object, 7, 70)?;
    let value = contract.dict_i32_i32_get(dict.object, 7)?;

    contract.release(dict)?;
    Ok(count == 1 && value == Some(70))
}

fn test_upsert_updates_value(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let dict = contract.construct_dict_i32_i32(1)?;

    let first_count = contract.dict_i32_i32_set(dict.object, 9, 90)?;
    let second_count = contract.dict_i32_i32_set(dict.object, 9, 900)?;
    let value = contract.dict_i32_i32_get(dict.object, 9)?;
    let len = contract.dict_i32_i32_len(dict.object)?;

    contract.release(dict)?;
    Ok(first_count == 1 && second_count == 1 && value == Some(900) && len == 1)
}

fn test_remove_existing_key(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let dict = contract.construct_dict_i32_i32(2)?;

    contract.dict_i32_i32_set(dict.object, 1, 10)?;
    contract.dict_i32_i32_set(dict.object, 2, 20)?;

    let removed = contract.dict_i32_i32_remove(dict.object, 1)?;
    let contains_removed = contract.dict_i32_i32_contains(dict.object, 1)?;
    let len = contract.dict_i32_i32_len(dict.object)?;

    contract.release(dict)?;
    Ok(removed == Some(10) && !contains_removed && len == 1)
}

fn test_missing_key_paths(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let dict = contract.construct_dict_i32_i32(1)?;

    let missing_get = contract.dict_i32_i32_get(dict.object, 111)?;
    let missing_remove = contract.dict_i32_i32_remove(dict.object, 111)?;

    contract.release(dict)?;
    Ok(missing_get.is_none() && missing_remove.is_none())
}

/// Test hash collision safety: insert 50 distinct Int32 keys, then verify all are retrievable.
fn test_hash_collision_safety(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let n = 50i32;
    let dict = contract.construct_dict_i32_i32(n)?;

    for i in 0..n {
        contract.dict_i32_i32_set(dict.object, i, i * 100)?;
    }

    let len = contract.dict_i32_i32_len(dict.object)?;
    if len != n {
        contract.release(dict)?;
        return Ok(false);
    }

    for i in 0..n {
        let v = contract.dict_i32_i32_get(dict.object, i)?;
        if v != Some(i * 100) {
            contract.release(dict)?;
            return Ok(false);
        }
        if !contract.dict_i32_i32_contains(dict.object, i)? {
            contract.release(dict)?;
            return Ok(false);
        }
    }

    contract.release(dict)?;
    Ok(true)
}

/// Test Dict<Int32, OpaqueRef>: insert String objects as values, retrieve and verify identity.
fn test_dict_opaque_ref_basic(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let dict = contract.construct_dict_ref(2)?;
    let str_a = contract.construct_string(b"alpha")?;
    let str_b = contract.construct_string(b"beta")?;

    let c1 = contract.dict_ref_set(dict.object, 1, str_a.object)?;
    let c2 = contract.dict_ref_set(dict.object, 2, str_b.object)?;
    let len = contract.dict_ref_len(dict.object)?;

    let got_a = contract.dict_ref_get(dict.object, 1)?;
    let got_b = contract.dict_ref_get(dict.object, 2)?;
    let missing = contract.dict_ref_get(dict.object, 99)?;
    let has_1 = contract.dict_ref_contains(dict.object, 1)?;
    let has_99 = contract.dict_ref_contains(dict.object, 99)?;

    contract.release(dict)?;
    contract.release(str_a)?;
    contract.release(str_b)?;

    Ok(c1 == 1
        && c2 == 2
        && len == 2
        && got_a == Some(str_a.object)
        && got_b == Some(str_b.object)
        && missing.is_none()
        && has_1
        && !has_99)
}

/// Test Dict<Int32, OpaqueRef>: remove returns the stored pointer, subsequent lookup is absent.
fn test_dict_opaque_ref_remove(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let dict = contract.construct_dict_ref(1)?;
    let str_a = contract.construct_string(b"gamma")?;

    contract.dict_ref_set(dict.object, 10, str_a.object)?;
    let removed = contract.dict_ref_remove(dict.object, 10)?;
    let after = contract.dict_ref_get(dict.object, 10)?;
    let len = contract.dict_ref_len(dict.object)?;

    contract.release(dict)?;
    contract.release(str_a)?;

    Ok(removed == Some(str_a.object) && after.is_none() && len == 0)
}
