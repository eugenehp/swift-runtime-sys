use std::env;

use swift_runtime_sys::RuntimeContract::{RuntimeContract, RuntimeContractError};
use swift_runtime_sys::RuntimeFactory::RuntimeFactory;

fn main() {
    let shape = env::var("ABI_SHAPE_ID").unwrap_or_else(|_| "all".to_string());

    let factory =
        RuntimeFactory::with_thunk_library("./libRustBridge.dylib", "./libRuntimeThunks.dylib")
            .or_else(|_| RuntimeFactory::new("./libRustBridge.dylib"))
            .unwrap_or_else(|e| panic!("failed to init RuntimeFactory: {e:?}"));

    factory
        .validate_runtime_contract(1)
        .unwrap_or_else(|e| panic!("runtime contract validation failed: {e:?}"));

    let contract = RuntimeContract::new(&factory);

    let run = |id: &str,
               f: fn(&RuntimeContract) -> Result<bool, RuntimeContractError>,
               c: &RuntimeContract| {
        let ok = match f(c) {
            Ok(v) => v,
            Err(_) => false,
        };
        println!("shape={} result={}", id, if ok { "PASS" } else { "FAIL" });
        ok
    };

    let ok = match shape.as_str() {
        "direct.value.i32_i32_to_i32" => {
            run("direct.value.i32_i32_to_i32", shape_direct_value, &contract)
        }
        "inout.mutating.i32ptr_i32_to_i32" => run(
            "inout.mutating.i32ptr_i32_to_i32",
            shape_inout_mutating,
            &contract,
        ),
        "indirect_ret.pair.i32_i32_to_pair" => run(
            "indirect_ret.pair.i32_i32_to_pair",
            shape_indirect_pair,
            &contract,
        ),
        "throwing.success.i32_to_i32" => run(
            "throwing.success.i32_to_i32",
            shape_throwing_success,
            &contract,
        ),
        "throwing.error.i32_to_i32" => {
            run("throwing.error.i32_to_i32", shape_throwing_error, &contract)
        }
        "async.value.i32_to_i32" => run("async.value.i32_to_i32", shape_async_value, &contract),
        "resilient.counter_addpair.i32_i32_to_i32" => run(
            "resilient.counter_addpair.i32_i32_to_i32",
            shape_resilient_counter_addpair,
            &contract,
        ),
        "all" => {
            let all = [
                run("direct.value.i32_i32_to_i32", shape_direct_value, &contract),
                run(
                    "inout.mutating.i32ptr_i32_to_i32",
                    shape_inout_mutating,
                    &contract,
                ),
                run(
                    "indirect_ret.pair.i32_i32_to_pair",
                    shape_indirect_pair,
                    &contract,
                ),
                run(
                    "throwing.success.i32_to_i32",
                    shape_throwing_success,
                    &contract,
                ),
                run("throwing.error.i32_to_i32", shape_throwing_error, &contract),
                run("async.value.i32_to_i32", shape_async_value, &contract),
                run(
                    "resilient.counter_addpair.i32_i32_to_i32",
                    shape_resilient_counter_addpair,
                    &contract,
                ),
            ];
            all.into_iter().all(|x| x)
        }
        _ => {
            eprintln!("unknown ABI_SHAPE_ID: {}", shape);
            false
        }
    };

    if !ok {
        std::process::exit(1);
    }
}

fn shape_direct_value(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.n2_direct_add(11, 31)? == 42)
}

fn shape_inout_mutating(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let (updated, returned) = contract.n2_inout_add_assign(7, 5)?;
    Ok(updated == 12 && returned == 12)
}

fn shape_indirect_pair(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let (sum, diff) = contract.n2_indirect_pair_sum_diff(20, 6)?;
    Ok(sum == 26 && diff == 14)
}

fn shape_throwing_success(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.n2_throwing_require_non_negative(9)? == 9)
}

fn shape_throwing_error(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.n2_throwing_require_non_negative(-3).is_err())
}

fn shape_async_value(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.n2_async_double(21)? == 42)
}

fn shape_resilient_counter_addpair(
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    Ok(contract.n2_resilient_counter_addpair(19, 23)? == 42)
}
