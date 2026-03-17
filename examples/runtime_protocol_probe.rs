use std::env;
use swift_runtime_sys::RuntimeFactory::RuntimeFactory;

fn run_variant(factory: &RuntimeFactory, variant: &str) -> Result<i32, String> {
    let counter = factory
        .call_allocating_init_i32("$s10RustBridge7CounterC5startACs5Int32V_tcfC", 10)
        .map_err(|e| format!("counter init failed: {e:?}"))?;

    factory
        .call_self_i32_to_void("runtime_thunk_counter_reset_x20", counter, 99)
        .map_err(|e| format!("reset failed: {e:?}"))?;

    let witness_symbol = "$s10RustBridge7CounterCAA0C4LikeAAWP";
    let witness = factory
        .symbol_address(witness_symbol)
        .map_err(|e| format!("witness symbol failed: {e:?}"))?;

    let slot1 = factory.read_ptr_at_offset(witness as *const _, 8);
    if slot1.is_null() {
        return Err("witness slot1 null".to_string());
    }

    let witness_table = witness as *const _;

    let out = match variant {
        "x20" => factory.call_self_to_i32_by_address_x20(slot1, counter),
        "x0" => factory.call_self_to_i32_by_address_x0(slot1, counter),
        "x20x0" => factory.call_self_to_i32_by_address_x20_x0(slot1, counter),
        "x0x1" => factory.call_witness_self_to_i32_by_address_x0_x1(slot1, counter, witness_table),
        "x20x1" => factory.call_witness_self_to_i32_by_address_x20_x1(slot1, counter, witness_table),
        "existential" => factory.call_existential_class_to_i32_by_address(slot1, counter),
        _ => return Err(format!("unknown variant: {variant}")),
    }
    .map_err(|e| format!("dispatch failed: {e:?}"))?;

    let direct = factory
        .call_self_to_i32("runtime_thunk_counter_current_x20", counter)
        .map_err(|e| format!("direct current failed: {e:?}"))?;

    println!("variant={variant} dispatch={out} direct={direct}");
    Ok(out)
}

fn main() {
    let variant = env::var("RUNTIME_PROTOCOL_VARIANT").unwrap_or_else(|_| "x20".to_string());

    let factory = RuntimeFactory::with_thunk_library("./libRustBridge.dylib", "./libRuntimeThunks.dylib")
        .unwrap_or_else(|e| panic!("factory init failed: {e:?}"));

    match run_variant(&factory, &variant) {
        Ok(value) => {
            if value == 99 {
                println!("semantic=PASS");
            } else {
                println!("semantic=FAIL");
            }
        }
        Err(err) => {
            eprintln!("error={err}");
            std::process::exit(2);
        }
    }
}
