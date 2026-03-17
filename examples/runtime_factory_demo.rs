use swift_runtime_sys::RuntimeFactory::RuntimeFactory;

fn unpack_person_bits(bits: u64) -> (i32, i32) {
    let low = (bits & 0xFFFF_FFFF) as u32;
    let high = ((bits >> 32) & 0xFFFF_FFFF) as u32;
    (low as i32, high as i32)
}

fn main() -> Result<(), String> {
    let factory = RuntimeFactory::with_thunk_library("./libRustBridge.dylib", "./libRuntimeThunks.dylib")
        .map_err(|e| format!("factory init failed: {e:?}"))?;

    let counter = factory
        .call_allocating_init_i32("$s10RustBridge7CounterC5startACs5Int32V_tcfC", 10)
        .map_err(|e| format!("counter init failed: {e:?}"))?;

    let retain_count_1 = factory
        .retain_count(counter)
        .map_err(|e| format!("retain_count failed: {e:?}"))?;

    let inc1 = factory
        .call_self_i32_to_i32("runtime_thunk_counter_increment_x20", counter, 5)
        .map_err(|e| format!("increment #1 failed: {e:?}"))?;
    let inc2 = factory
        .call_self_i32_to_i32("runtime_thunk_counter_increment_x20", counter, 3)
        .map_err(|e| format!("increment #2 failed: {e:?}"))?;
    let cur = factory
        .call_self_to_i32("runtime_thunk_counter_current_x20", counter)
        .map_err(|e| format!("current failed: {e:?}"))?;

    factory
        .call_self_i32_to_void("runtime_thunk_counter_reset_x20", counter, 4)
        .map_err(|e| format!("reset failed: {e:?}"))?;
    let after_reset = factory
        .call_self_to_i32("runtime_thunk_counter_current_x20", counter)
        .map_err(|e| format!("current after reset failed: {e:?}"))?;

    let add_pair = factory
        .call_self_i32_i32_to_i32("runtime_thunk_counter_add_pair_x20", counter, 6, 7)
        .map_err(|e| format!("addPair failed: {e:?}"))?;

    factory
        .call_self_to_void("runtime_thunk_counter_clear_x20", counter)
        .map_err(|e| format!("clear failed: {e:?}"))?;
    let after_clear = factory
        .call_self_to_i32("runtime_thunk_counter_current_x20", counter)
        .map_err(|e| format!("current after clear failed: {e:?}"))?;

    // Direct memory variable access: Counter.value is currently stored at byte offset 16.
    factory.write_i32_at_offset(counter, 16, 99);
    let direct_field_value = factory.read_i32_at_offset(counter, 16);
    let after_direct_write = factory
        .call_self_to_i32("runtime_thunk_counter_current_x20", counter)
        .map_err(|e| format!("current after direct write failed: {e:?}"))?;

    let person_bits = factory
        .call_struct_init_i32_i32_u64("$s10RustBridge6PersonV2id3ageACs5Int32V_AGtcfC", 7, 42)
        .map_err(|e| format!("person init failed: {e:?}"))?;
    let (pid, page) = unpack_person_bits(person_bits);

    let counter_metadata = factory
        .metadata_from_accessor_0("$s10RustBridge7CounterCMa")
        .map_err(|e| format!("counter metadata failed: {e:?}"))?;
    let raw_counter = factory
        .alloc_object(counter_metadata, 32, 7)
        .map_err(|e| format!("raw alloc failed: {e:?}"))?;
    factory
        .dealloc_class_instance(raw_counter, 32, 7)
        .map_err(|e| format!("raw dealloc failed: {e:?}"))?;

    let _retained = factory
        .retain(counter)
        .map_err(|e| format!("retain failed: {e:?}"))?;
    factory
        .release(counter)
        .map_err(|e| format!("release failed: {e:?}"))?;
    let retain_count_2 = factory
        .retain_count(counter)
        .map_err(|e| format!("retain_count #2 failed: {e:?}"))?;

    // Experimental protocol existential container construction.
    let witness_symbol = "$s10RustBridge7CounterCAA0C4LikeAAWP";
    let witness_addr = factory.symbol_address(witness_symbol).ok();
    let existential = factory.make_class_protocol_existential(
        counter,
        witness_addr.unwrap_or(std::ptr::null_mut()) as *const _,
    );

    println!(
        "demo ok: rc1={retain_count_1} inc=({inc1},{inc2}) cur={cur} reset={after_reset} add_pair={add_pair} clear={after_clear} direct={direct_field_value}/{after_direct_write} person=({pid},{page}) rc2={retain_count_2} existential={:?}",
        existential
    );

    Ok(())
}
