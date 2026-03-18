extern crate swift_runtime_sys;

use swift_runtime_sys::RuntimeContract::{
    ContractArgBlob, ContractArgValue, ContractObject, ContractOwnership, RuntimeContract,
    RuntimeContractError,
};
use swift_runtime_sys::RuntimeFactory::RuntimeFactory;

const TYPE_PERSON: i32 = 1;
const TYPE_COUNTER: i32 = 2;
const TYPE_ANY_BOX: i32 = 8;

const COUNTER_CURRENT: i32 = 2;

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
    if test_metatype_identity(&contract).unwrap_or(false) {
        passed += 1;
        println!("PASS: Metatype identity (Counter type_id preserved in AnyBox)");
    } else {
        println!("FAIL: Metatype identity (Counter type_id preserved in AnyBox)");
    }

    total += 1;
    if test_successful_narrow_cast(&contract).unwrap_or(false) {
        passed += 1;
        println!("PASS: Successful narrow cast (Counter -> type_id=2)");
    } else {
        println!("FAIL: Successful narrow cast (Counter -> type_id=2)");
    }

    total += 1;
    if test_failed_narrow_cast(&contract).unwrap_or(false) {
        passed += 1;
        println!("PASS: Failed narrow cast (Counter box does not cast to Person)");
    } else {
        println!("FAIL: Failed narrow cast (Counter box does not cast to Person)");
    }

    total += 1;
    if test_metatype_comparison(&contract).unwrap_or(false) {
        passed += 1;
        println!("PASS: Metatype comparison (two Persons share same type_id)");
    } else {
        println!("FAIL: Metatype comparison (two Persons share same type_id)");
    }

    total += 1;
    if test_round_trip_cast_and_dispatch(&contract).unwrap_or(false) {
        passed += 1;
        println!("PASS: Round-trip cast preserves object pointer identity");
    } else {
        println!("FAIL: Round-trip cast preserves object pointer identity");
    }

    println!("Dynamic cast parity probe results: {}/{} PASS", passed, total);
    if passed != total {
        std::process::exit(1);
    }
}

/// Wrap a Counter in an AnyBox and confirm the stored type_id is 2 (metatype identity check).
fn test_metatype_identity(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let counter = contract.construct(
        TYPE_COUNTER,
        &ContractArgBlob::from_values(&[ContractArgValue::I32(0)]),
    )?;

    let any_box = contract.wrap_any_object(TYPE_COUNTER, counter.object)?;
    let stored_type_id = contract.any_object_type_id(any_box)?;

    let any_box_obj = ContractObject {
        type_id: TYPE_ANY_BOX,
        object: any_box,
        ownership: ContractOwnership::SwiftRetained,
    };
    contract.release(any_box_obj)?;
    contract.release(counter)?;

    Ok(stored_type_id == TYPE_COUNTER)
}

/// Wrap a Counter; casting to its own type_id should succeed (returns Some).
fn test_successful_narrow_cast(
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    let counter = contract.construct(
        TYPE_COUNTER,
        &ContractArgBlob::from_values(&[ContractArgValue::I32(5)]),
    )?;

    let any_box = contract.wrap_any_object(TYPE_COUNTER, counter.object)?;
    let cast_result = contract.dynamic_cast(any_box, TYPE_COUNTER)?;

    let success = cast_result == Some(counter.object);

    let any_box_obj = ContractObject {
        type_id: TYPE_ANY_BOX,
        object: any_box,
        ownership: ContractOwnership::SwiftRetained,
    };
    contract.release(any_box_obj)?;
    contract.release(counter)?;

    Ok(success)
}

/// Wrap a Counter; casting to a different type_id (Person=1) must return None (cast failure).
fn test_failed_narrow_cast(
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    let counter = contract.construct(
        TYPE_COUNTER,
        &ContractArgBlob::from_values(&[ContractArgValue::I32(7)]),
    )?;

    let any_box = contract.wrap_any_object(TYPE_COUNTER, counter.object)?;
    let cast_result = contract.dynamic_cast(any_box, TYPE_PERSON)?;

    let is_none = cast_result.is_none();

    let any_box_obj = ContractObject {
        type_id: TYPE_ANY_BOX,
        object: any_box,
        ownership: ContractOwnership::SwiftRetained,
    };
    contract.release(any_box_obj)?;
    contract.release(counter)?;

    Ok(is_none)
}

/// Wrap two Persons independently; both AnyBoxes must report the same metatype (type_id=1).
fn test_metatype_comparison(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let person_a = contract.construct(
        TYPE_PERSON,
        &ContractArgBlob::from_values(&[ContractArgValue::I32(1), ContractArgValue::I32(30)]),
    )?;
    let person_b = contract.construct(
        TYPE_PERSON,
        &ContractArgBlob::from_values(&[ContractArgValue::I32(2), ContractArgValue::I32(25)]),
    )?;

    let any_a = contract.wrap_any_object(TYPE_PERSON, person_a.object)?;
    let any_b = contract.wrap_any_object(TYPE_PERSON, person_b.object)?;

    let id_a = contract.any_object_type_id(any_a)?;
    let id_b = contract.any_object_type_id(any_b)?;

    let same_metatype = id_a == id_b && id_a == TYPE_PERSON;

    for (any_ptr, inner) in [(any_a, person_a), (any_b, person_b)] {
        let any_box_obj = ContractObject {
            type_id: TYPE_ANY_BOX,
            object: any_ptr,
            ownership: ContractOwnership::SwiftRetained,
        };
        contract.release(any_box_obj)?;
        contract.release(inner)?;
    }

    Ok(same_metatype)
}

/// Wrap a Counter, narrow-cast back to Counter, verify the pointer identity is preserved.
fn test_round_trip_cast_and_dispatch(
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    let counter = contract.construct(
        TYPE_COUNTER,
        &ContractArgBlob::from_values(&[ContractArgValue::I32(42)]),
    )?;
    let original_ptr = counter.object;

    let any_box = contract.wrap_any_object(TYPE_COUNTER, counter.object)?;

    let cast_result = contract.dynamic_cast(any_box, TYPE_COUNTER)?;
    let inner_ptr = match cast_result {
        Some(ptr) => ptr,
        None => {
            // cleanup
            let any_box_obj = ContractObject {
                type_id: TYPE_ANY_BOX,
                object: any_box,
                ownership: ContractOwnership::SwiftRetained,
            };
            contract.release(any_box_obj)?;
            contract.release(counter)?;
            return Ok(false);
        }
    };

    // The key assertion: after wrapping and casting, the inner pointer should match the original pointer
    let pointers_match = inner_ptr == original_ptr;

    let any_box_obj = ContractObject {
        type_id: TYPE_ANY_BOX,
        object: any_box,
        ownership: ContractOwnership::SwiftRetained,
    };
    contract.release(any_box_obj)?;
    contract.release(counter)?;

    Ok(pointers_match)
}
