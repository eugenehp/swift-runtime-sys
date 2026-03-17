use core::ffi::c_void;
use swift_runtime_sys::RuntimeFactory::RuntimeFactory;

unsafe extern "C" {
    fn malloc_size(ptr: *const c_void) -> usize;
}

fn dump_words(ptr: *const c_void, bytes: usize) {
    let words = bytes / 8;
    let p = ptr as *const u64;
    for i in 0..words {
        let w = unsafe { *p.add(i) };
        println!("word[{i:02}] = 0x{w:016x}");
    }
}

fn unpack_person_bits(bits: u64) -> (i32, i32) {
    let low = (bits & 0xFFFF_FFFF) as u32;
    let high = ((bits >> 32) & 0xFFFF_FFFF) as u32;
    (low as i32, high as i32)
}

fn lcg_next(state: &mut u64) -> u64 {
    *state = state
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    *state
}

fn main() {
    let factory =
        RuntimeFactory::with_thunk_library("./libRustBridge.dylib", "./libRuntimeThunks.dylib")
            .unwrap_or_else(|e| panic!("failed creating RuntimeFactory: {e:?}"));

    println!("== direct mangled function call ==");
    let sum = factory
        .call_i32_i32_to_i32("$s10RustBridge9swift_addys5Int32VAD_ADtF", 20, 22)
        .unwrap_or_else(|e| panic!("swift_add failed: {e:?}"));
    println!("swift_add mangled result: {sum}");

    println!("== direct global variable access ==");
    let global_addr = factory
        .symbol_address("$s10RustBridge18globalCounterValues5Int32Vvp")
        .unwrap_or_else(|e| panic!("global symbol address failed: {e:?}"));
    let global_initial = factory.read_i32(global_addr as *const c_void);
    factory.write_i32(global_addr, 321);
    let global_after = factory.read_i32(global_addr as *const c_void);
    println!(
        "global variable => initial={global_initial}; after_write={global_after}; addr={:p}",
        global_addr
    );

    println!("== direct metadata lookup ==");
    let person_meta = factory
        .metadata_from_accessor_0("$s10RustBridge6PersonVMa")
        .unwrap_or_else(|e| panic!("person metadata failed: {e:?}"));
    let counter_meta = factory
        .metadata_from_accessor_0("$s10RustBridge7CounterCMa")
        .unwrap_or_else(|e| panic!("counter metadata failed: {e:?}"));
    println!(
        "person metadata={:p} state={:p}",
        person_meta,
        std::ptr::null::<c_void>()
    );
    println!(
        "counter metadata={:p} state={:p}",
        counter_meta,
        std::ptr::null::<c_void>()
    );

    println!("== direct struct value construction (mangled init) ==");
    let person_bits = factory
        .call_struct_init_i32_i32_u64("$s10RustBridge6PersonV2id3ageACs5Int32V_AGtcfC", 7, 42)
        .unwrap_or_else(|e| panic!("person init failed: {e:?}"));
    let (id, age) = unpack_person_bits(person_bits);
    println!("person raw bits=0x{person_bits:016x} unpacked id={id} age={age}");

    println!("== direct class construction (mangled allocating init) ==");
    let counter = factory
        .call_allocating_init_i32("$s10RustBridge7CounterC5startACs5Int32V_tcfC", 10)
        .unwrap_or_else(|e| panic!("counter init failed: {e:?}"));
    if counter.is_null() {
        panic!("counter_allocating_init returned null");
    }
    println!("counter object={:p}", counter);

    let rc1 = factory
        .retain_count(counter)
        .unwrap_or_else(|e| panic!("retain count failed: {e:?}"));
    println!("retain count after init={rc1}");

    if std::env::var("RUNTIME_TRY_INCREMENT").as_deref() == Ok("1") {
        // This path is experimental and may crash if ABI assumptions drift.
        let inc1 = factory
            .call_self_i32_to_i32("runtime_thunk_counter_increment_x20", counter, 5)
            .unwrap_or_else(|e| panic!("increment #1 failed: {e:?}"));
        let inc2 = factory
            .call_self_i32_to_i32("runtime_thunk_counter_increment_x20", counter, 3)
            .unwrap_or_else(|e| panic!("increment #2 failed: {e:?}"));
        let current = factory
            .call_self_to_i32("runtime_thunk_counter_current_x20", counter)
            .unwrap_or_else(|e| panic!("current failed: {e:?}"));
        factory
            .call_self_i32_to_void("runtime_thunk_counter_reset_x20", counter, 4)
            .unwrap_or_else(|e| panic!("reset failed: {e:?}"));
        let reset_rc = 0;
        let after_reset = factory
            .call_self_to_i32("runtime_thunk_counter_current_x20", counter)
            .unwrap_or_else(|e| panic!("after reset current failed: {e:?}"));
        let add_pair = factory
            .call_self_i32_i32_to_i32("runtime_thunk_counter_add_pair_x20", counter, 6, 7)
            .unwrap_or_else(|e| panic!("addPair failed: {e:?}"));
        factory
            .call_self_to_void("runtime_thunk_counter_clear_x20", counter)
            .unwrap_or_else(|e| panic!("clear failed: {e:?}"));
        let clear_rc = 0;
        let after_clear = factory
            .call_self_to_i32("runtime_thunk_counter_current_x20", counter)
            .unwrap_or_else(|e| panic!("after clear current failed: {e:?}"));

        factory.write_i32_at_offset(counter, 16, 99);
        let direct_field = factory.read_i32_at_offset(counter, 16);
        let after_direct = factory
            .call_self_to_i32("runtime_thunk_counter_current_x20", counter)
            .unwrap_or_else(|e| panic!("after direct write current failed: {e:?}"));

        let witness_symbol = "$s10RustBridge7CounterCAA0C4LikeAAWP";
        let witness = factory.symbol_address(witness_symbol).ok();
        let existential = factory.make_class_protocol_existential(
            counter,
            witness.unwrap_or(std::ptr::null_mut()) as *const _,
        );
        let witness_nonnull = (!existential.witness_table.is_null()) as i32;
        let slot0 = if existential.witness_table.is_null() {
            std::ptr::null()
        } else {
            factory.read_ptr_at_offset(existential.witness_table as *const c_void, 0)
        };
        let slot1 = if existential.witness_table.is_null() {
            std::ptr::null()
        } else {
            factory.read_ptr_at_offset(existential.witness_table as *const c_void, 8)
        };
        let slot2 = if existential.witness_table.is_null() {
            std::ptr::null()
        } else {
            factory.read_ptr_at_offset(existential.witness_table as *const c_void, 16)
        };
        let slot0_name = factory
            .symbol_name_for_address(slot0)
            .unwrap_or_else(|| "<unknown>".to_string());
        let slot1_name = factory
            .symbol_name_for_address(slot1)
            .unwrap_or_else(|| "<unknown>".to_string());
        let slot2_name = factory
            .symbol_name_for_address(slot2)
            .unwrap_or_else(|| "<unknown>".to_string());
        // Correct calling convention: TW witness thunk does `ldr x20,[x20]` on entry,
        // so x20 must be a pointer to the object pointer (existential indirect-self).
        let witness_dispatch_existential = if slot1.is_null() {
            i32::MIN
        } else {
            factory
                .call_existential_class_to_i32_by_address(slot1, counter)
                .unwrap_or(i32::MIN)
        };
        // Legacy experimental variants kept for ABI reference (guarded to avoid crashes).
        let witness_dispatch_current_x20 = if slot1.is_null() {
            i32::MIN
        } else {
            factory
                .call_self_to_i32_by_address_x20(slot1, counter)
                .unwrap_or(i32::MIN)
        };
        let witness_dispatch_current_x0 =
            if std::env::var("RUNTIME_TRY_WITNESS_X0").as_deref() == Ok("1") {
                if slot1.is_null() {
                    i32::MIN
                } else {
                    factory
                        .call_self_to_i32_by_address_x0(slot1, counter)
                        .unwrap_or(i32::MIN)
                }
            } else {
                i32::MIN
            };

        println!(
            "counter increments via x20 thunk => {inc1}, {inc2}; current={current}; reset_rc={reset_rc}; after_reset={after_reset}; add_pair={add_pair}; clear_rc={clear_rc}; after_clear={after_clear}"
        );
        println!("direct field write => direct={direct_field}; current={after_direct}");
        println!(
            "protocol witness => nonnull={witness_nonnull}; addr={:p}",
            existential.witness_table
        );
        println!(
            "protocol witness slot0 => addr={:p}; symbol={}",
            slot0, slot0_name
        );
        println!(
            "protocol witness slot1 => addr={:p}; symbol={}",
            slot1, slot1_name
        );
        println!(
            "protocol witness slot2 => addr={:p}; symbol={}",
            slot2, slot2_name
        );
        println!(
            "protocol witness dispatch => existential={witness_dispatch_existential}; x20={witness_dispatch_current_x20}; x0={witness_dispatch_current_x0}"
        );
    } else {
        println!("skipping unsafe direct method call; set RUNTIME_TRY_INCREMENT=1 to attempt it");
    }

    let obj_size = unsafe { malloc_size(counter) };
    println!("malloc_size(counter)={obj_size}");
    println!("counter first 32 bytes:");
    dump_words(counter, 32);

    println!("== direct runtime alloc/dealloc parity sample ==");
    let raw_counter = factory
        .alloc_object(counter_meta, 32, 7)
        .unwrap_or_else(|e| panic!("raw alloc failed: {e:?}"));
    println!("raw counter object={:p}", raw_counter);
    println!("malloc_size(raw_counter)={}", unsafe {
        malloc_size(raw_counter)
    });
    println!("raw counter first 32 bytes:");
    dump_words(raw_counter, 32);
    let raw_header_meta = factory.read_ptr_at_offset(raw_counter as *const c_void, 0);
    let raw_meta_match = (raw_header_meta == counter_meta) as i32;
    println!(
        "raw metadata parity => expected={:p}; actual={:p}; match={raw_meta_match}",
        counter_meta, raw_header_meta
    );
    factory
        .dealloc_class_instance(raw_counter, 32, 7)
        .unwrap_or_else(|e| panic!("raw dealloc failed: {e:?}"));
    println!("raw counter deallocated via swift_deallocClassInstance");

    let retained = factory
        .retain(counter)
        .unwrap_or_else(|e| panic!("retain failed: {e:?}"));
    println!("retained object={:p}", retained);
    let rc2 = factory
        .retain_count(counter)
        .unwrap_or_else(|e| panic!("retain count #2 failed: {e:?}"));
    println!("retain count after retain={rc2}");

    factory
        .release(counter)
        .unwrap_or_else(|e| panic!("release failed: {e:?}"));
    let rc3 = factory
        .retain_count(counter)
        .unwrap_or_else(|e| panic!("retain count #3 failed: {e:?}"));
    println!("retain count after one release={rc3}");

    println!("leaving final retain in place to avoid teardown crash in experimental probe");

    // ── Enum parity ──────────────────────────────────────────────────────────

    // Test 1: Direction (RawRepresentable Int32, 4-byte storage).
    // Write the tag directly to the global storage symbol, then read back
    // via the @_cdecl bridge to confirm the round-trip.
    let dir_symbol = "$s10RustBridge16currentDirectionAA0D0Ovp";
    let dir_storage = factory.symbol_address(dir_symbol);
    let (enum_raw_initial, enum_raw_after_write) = match dir_storage {
        Ok(ptr) => {
            let initial = factory.read_i32(ptr as *const _);
            // Write Direction.east (2) directly into the storage slot.
            factory.write_i32(ptr as *mut _, 2);
            // Read back through Swift's @_cdecl bridge — it reads from the same global.
            let from_bridge = factory
                .call_to_i32("swift_direction_raw")
                .unwrap_or(i32::MIN);
            (initial, from_bridge)
        }
        Err(_) => (i32::MIN, i32::MIN),
    };
    println!("enum Direction tag => initial={enum_raw_initial} after_write={enum_raw_after_write}");

    // Test 2: Shape (associated values) area computation via @_cdecl bridge.
    // circle(radius=5.0) -> pi*25 ≈ 78.54
    // rectangle(3.0, 4.0) -> 12.0
    let circle_area = factory
        .call_f32_to_f32("swift_shape_circle_area", 5.0_f32)
        .unwrap_or(f32::MIN);
    let rect_area = factory
        .call_f32_f32_to_f32("swift_shape_rect_area", 3.0_f32, 4.0_f32)
        .unwrap_or(f32::MIN);
    println!("enum Shape area => circle={circle_area:.4} rect={rect_area:.4}");

    // ── throws ABI ────────────────────────────────────────────────────────────
    let throws_sym = "$s10RustBridge10safeDivideys5Int32VAD_ADtKF";

    let throws_ok = factory.call_throws_i32_i32(throws_sym, 10, 2);
    let (throws_ok_result, throws_ok_err_null) = match throws_ok {
        Ok(swift_runtime_sys::RuntimeFactory::ThrowsResult::Ok(v)) => (v, 1i32),
        Ok(swift_runtime_sys::RuntimeFactory::ThrowsResult::Threw(_)) => (-1, 0),
        Err(_) => (-2, 0),
    };

    let throws_err = factory.call_throws_i32_i32(throws_sym, 10, 0);
    let throws_err_nonnull = match throws_err {
        Ok(swift_runtime_sys::RuntimeFactory::ThrowsResult::Ok(_)) => 0i32,
        Ok(swift_runtime_sys::RuntimeFactory::ThrowsResult::Threw(_)) => 1,
        Err(_) => 0,
    };
    println!("throws safeDivide => ok_result={throws_ok_result} err_null={throws_ok_err_null}");
    println!("throws safeDivide error => throws_nonnull={throws_err_nonnull}");

    // ── Generic type (TypedBox<Int32>) ────────────────────────────────────────
    // The @_cdecl bridge takes/returns raw pointers (full 64-bit), so we transmute
    // each function to the correct signature rather than using call_i32_to_i32.
    type CdeclI32ToPtr = unsafe extern "C" fn(i32) -> *mut c_void;
    type CdeclPtrToI32 = unsafe extern "C" fn(*mut c_void) -> i32;
    type CdeclPtrI32ToVoid = unsafe extern "C" fn(*mut c_void, i32);
    type CdeclPtrToVoid = unsafe extern "C" fn(*mut c_void);
    let generic_get1_v2: i32;
    let generic_get2_v2: i32;
    unsafe {
        let new_fn = factory
            .symbol_address("swift_typed_box_i32_new")
            .map(|p| std::mem::transmute::<*mut c_void, CdeclI32ToPtr>(p));
        let get_fn = factory
            .symbol_address("swift_typed_box_i32_get")
            .map(|p| std::mem::transmute::<*mut c_void, CdeclPtrToI32>(p));
        let set_fn = factory
            .symbol_address("swift_typed_box_i32_set")
            .map(|p| std::mem::transmute::<*mut c_void, CdeclPtrI32ToVoid>(p));
        let drop_fn = factory
            .symbol_address("swift_typed_box_i32_drop")
            .map(|p| std::mem::transmute::<*mut c_void, CdeclPtrToVoid>(p));
        match (new_fn, get_fn, set_fn, drop_fn) {
            (Ok(new_f), Ok(get_f), Ok(set_f), Ok(drop_f)) => {
                let obj = new_f(42);
                generic_get1_v2 = get_f(obj);
                set_f(obj, 99);
                generic_get2_v2 = get_f(obj);
                drop_f(obj);
            }
            _ => {
                generic_get1_v2 = i32::MIN;
                generic_get2_v2 = i32::MIN;
            }
        }
    }
    println!("generic TypedBox => get1={generic_get1_v2} get2={generic_get2_v2}");

    // ── String (heap) ─────────────────────────────────────────────────────────
    type CdeclCStrToPtr = unsafe extern "C" fn(*const core::ffi::c_char) -> *mut c_void;
    let (str_char_len, str_utf8_len) = unsafe {
        let new_fn = factory
            .symbol_address("swift_string_new")
            .map(|p| std::mem::transmute::<*mut c_void, CdeclCStrToPtr>(p));
        let len_fn = factory
            .symbol_address("swift_string_length")
            .map(|p| std::mem::transmute::<*mut c_void, CdeclPtrToI32>(p));
        let utf8_fn = factory
            .symbol_address("swift_string_utf8_length")
            .map(|p| std::mem::transmute::<*mut c_void, CdeclPtrToI32>(p));
        let drop_fn = factory
            .symbol_address("swift_string_drop")
            .map(|p| std::mem::transmute::<*mut c_void, CdeclPtrToVoid>(p));
        match (new_fn, len_fn, utf8_fn, drop_fn) {
            (Ok(new_f), Ok(len_f), Ok(utf8_f), Ok(drop_f)) => {
                let hello = b"Hello\0".as_ptr() as *const core::ffi::c_char;
                let sobj = new_f(hello);
                let clen = len_f(sobj);
                let ulen = utf8_f(sobj);
                drop_f(sobj);
                (clen, ulen)
            }
            _ => (i32::MIN, i32::MIN),
        }
    };
    println!("string => char_len={str_char_len} utf8_len={str_utf8_len}");

    // ── Struct method dispatch (direct mangled symbol, no @_cdecl wrapper) ────
    // Point.sum(): x0 carries the Point value bits (x and y packed into 8 bytes)
    let point_sum_sym = "$s10RustBridge5PointV3sums5Int32VyF";
    let point_sum_fn = factory.symbol_address(point_sum_sym);
    let (struct_point_sum, struct_point_product) = match point_sum_fn {
        Ok(fn_addr) => {
            // Swift passes small value-type `self` directly in x0 for this method.
            // Pack Point{x=3,y=4} into x0 as low/high 32-bit lanes.
            let point_bits = ((((4i32 as u32) as u64) << 32) | ((3i32 as u32) as u64)) as usize;
            let point_ptr = point_bits as *mut c_void;
            let sum = factory
                .call_self_to_i32_by_address_x0(fn_addr as *const c_void, point_ptr)
                .unwrap_or(i32::MIN);
            let prod_sym = "$s10RustBridge5PointV7products5Int32VyF";
            let prod = factory
                .symbol_address(prod_sym)
                .and_then(|f| factory.call_self_to_i32_by_address_x0(f as *const c_void, point_ptr))
                .unwrap_or(i32::MIN);
            (sum, prod)
        }
        Err(_) => (i32::MIN, i32::MIN),
    };
    println!("struct method => point_sum={struct_point_sum} point_product={struct_point_product}");

    // ── Tuple return ──────────────────────────────────────────────────────────
    // splitAdd(10, 3) -> (13, 7): tuple is packed into x0 (low/high 32-bit).
    let tuple_sym = "$s10RustBridge8splitAddys5Int32V_ADtAD_ADtF";
    let tuple_result = factory.call_i32_i32_to_i32_pair(tuple_sym, 10, 3);
    let (tuple_first, tuple_second) = match tuple_result {
        Ok(p) => (p.first, p.second),
        Err(_) => (i32::MIN, i32::MIN),
    };
    println!("tuple => first={tuple_first} second={tuple_second}");

    // ── Optional<T> layout ────────────────────────────────────────────────────
    let opt_none = factory
        .call_to_i32("swift_optional_none_get")
        .unwrap_or(i32::MIN);
    let opt_some = factory
        .call_to_i32("swift_optional_some_get")
        .unwrap_or(i32::MIN);
    // Also read raw memory layout: optionalSome global byte[0..3]=42, byte[4]=0 (.some)
    type OptionalAccessorFn = unsafe extern "C" fn() -> *mut c_void;
    let opt_some_layout_ok =
        match factory.symbol_address("$s10RustBridge12optionalSomes5Int32VSgvau") {
            Ok(accessor) => {
                let addr =
                    unsafe { std::mem::transmute::<*mut c_void, OptionalAccessorFn>(accessor)() };
                let val = factory.read_i32(addr as *const _);
                let disc = factory.read_u8_at_offset(addr as *const _, 4);
                // For Optional<Int32> in this build, tag bit 0 means .some and 1 means .none.
                (val == 42 && disc == 0) as i32
            }
            Err(_) => 0,
        };
    println!(
        "optional => none_get={opt_none} some_get={opt_some} some_layout_ok={opt_some_layout_ok}"
    );

    // ── Array<T> ──────────────────────────────────────────────────────────────
    let arr_count = factory.call_to_i32("swift_array_count").unwrap_or(i32::MIN);
    let arr_elem2 = factory
        .call_i32_to_i32("swift_array_get", 2)
        .unwrap_or(i32::MIN);
    factory.call_i32_to_i32("swift_array_append", 99).ok();
    let arr_count_after = factory.call_to_i32("swift_array_count").unwrap_or(i32::MIN);
    println!("array => count={arr_count} elem2={arr_elem2} count_after_append={arr_count_after}");

    // String/Array storage internals
    let str_storage_flags = factory
        .call_to_i32("swift_string_storage_probe_flags")
        .unwrap_or(0);
    let str_tagged_diff = (str_storage_flags & 1) != 0;
    let str_short_utf8_ok = (str_storage_flags & 2) != 0;
    let str_long_utf8_ok = (str_storage_flags & 4) != 0;
    println!(
        "string storage => flags={str_storage_flags} tagged_diff={} short_utf8_ok={} long_utf8_ok={}",
        str_tagged_diff as i32,
        str_short_utf8_ok as i32,
        str_long_utf8_ok as i32,
    );

    let arr_cow_flags = factory.call_to_i32("swift_array_cow_probe_flags").unwrap_or(0);
    let arr_shared_before = (arr_cow_flags & 1) != 0;
    let arr_split_after = (arr_cow_flags & 2) != 0;
    let arr_original_unchanged = (arr_cow_flags & 4) != 0;
    println!(
        "array storage => flags={arr_cow_flags} shared_before={} split_after={} original_unchanged={}",
        arr_shared_before as i32,
        arr_split_after as i32,
        arr_original_unchanged as i32,
    );

    // ── Closure ───────────────────────────────────────────────────────────────
    factory.call_i32_to_i32("swift_store_adder_closure", 7).ok();
    let closure_result = factory
        .call_i32_to_i32("swift_invoke_stored_closure", 5)
        .unwrap_or(i32::MIN);
    println!("closure adder => result={closure_result}");

    // ── Reflection (Mirror) ───────────────────────────────────────────────────
    let field_count = factory
        .call_to_i32("swift_point_field_count")
        .unwrap_or(i32::MIN);
    let first_is_x = factory
        .call_to_i32("swift_point_first_field_is_x")
        .unwrap_or(i32::MIN);
    println!("reflection => point_fields={field_count} first_field_x={first_is_x}");

    // ── Error boxing ─────────────────────────────────────────────────────────
    type CdeclToPtr = unsafe extern "C" fn() -> *mut c_void;
    let error_nonnull: i32;
    let error_rc: i32;
    let error_semantic_ok: i32;
    unsafe {
        let make_fn = factory
            .symbol_address("swift_make_math_error")
            .map(|p| std::mem::transmute::<*mut c_void, CdeclToPtr>(p));
        let check_fn = factory
            .symbol_address("swift_check_math_error")
            .map(|p| std::mem::transmute::<*mut c_void, CdeclPtrToI32>(p));
        let drop_fn = factory
            .symbol_address("swift_drop_error")
            .map(|p| std::mem::transmute::<*mut c_void, CdeclPtrToVoid>(p));
        match (make_fn, check_fn, drop_fn) {
            (Ok(make_f), Ok(check_f), Ok(drop_f)) => {
                let eptr = make_f();
                error_nonnull = if eptr.is_null() { 0 } else { 1 };
                error_rc = factory.retain_count(eptr).unwrap_or(0) as i32;
                let semantic_flags = check_f(eptr);
                error_semantic_ok = if semantic_flags == 7 { 1 } else { 0 };
                drop_f(eptr);
            }
            _ => {
                error_nonnull = 0;
                error_rc = 0;
                error_semantic_ok = 0;
            }
        }
    }
    println!("error boxing => nonnull={error_nonnull} rc={error_rc}");
    println!("error roundtrip => semantic_ok={error_semantic_ok}");

    // ── Objective-C interop parity ──────────────────────────────────────────
    let objc_flags = factory
        .call_to_i32("swift_objc_interop_probe_flags")
        .unwrap_or(0);
    let objc_selector_ok = (objc_flags & 1) != 0;
    let objc_string_bridge_ok = (objc_flags & 2) != 0;
    let objc_array_bridge_ok = (objc_flags & 4) != 0;
    println!(
        "objc interop => flags={objc_flags} selector_ok={} string_bridge_ok={} array_bridge_ok={}",
        objc_selector_ok as i32,
        objc_string_bridge_ok as i32,
        objc_array_bridge_ok as i32,
    );

    // ── Weak reference ────────────────────────────────────────────────────────
    let mut weak_slot: u64 = 0;
    let weak_slot_ptr = &mut weak_slot as *mut u64 as *mut c_void;
    let weak_loaded_eq = factory
        .weak_init(weak_slot_ptr, counter)
        .and_then(|_| factory.weak_load_strong(weak_slot_ptr))
        .map(|loaded| {
            let eq = (loaded == counter) as i32;
            factory.release(loaded).ok();
            factory.weak_destroy(weak_slot_ptr).ok();
            eq
        })
        .unwrap_or(0);
    println!("weak ref => loaded_eq_original={weak_loaded_eq}");

    // ── swift_conformsToProtocol ──────────────────────────────────────────────
    let protocol_desc = factory.symbol_address("$s10RustBridge11CounterLikeMp");
    let conformance_nonnull = match protocol_desc {
        Ok(desc) => factory
            .conforms_to_protocol(counter_meta, desc as *const c_void)
            .map(|wt| if wt.is_null() { 0i32 } else { 1 })
            .unwrap_or(0),
        Err(_) => 0,
    };
    println!("conformance => witness_nonnull={conformance_nonnull}");

    // ── Async/task runtime ABI (via blocking bridges) ───────────────────────
    let async_add = factory
        .call_i32_i32_to_i32("swift_async_add_blocking", 20, 22)
        .unwrap_or(i32::MIN);
    let async_div_ok = factory
        .call_i32_i32_to_i32("swift_async_divide_try_blocking", 10, 2)
        .unwrap_or(i32::MIN);
    let async_div_throw = factory
        .call_i32_i32_to_i32("swift_async_divide_did_throw_blocking", 10, 0)
        .unwrap_or(0);
    println!(
        "async task => add={async_add} divide_ok={async_div_ok} divide_throw={async_div_throw}"
    );

    // ── Actor isolation/executor behavior (via blocking bridges) ─────────────
    let actor_create = factory
        .call_i32_to_i32("swift_actor_counter_create", 10)
        .unwrap_or(0);
    let actor_inc = factory
        .call_i32_to_i32("swift_actor_counter_increment_blocking", 5)
        .unwrap_or(i32::MIN);
    let actor_cur = factory
        .call_to_i32("swift_actor_counter_current_blocking")
        .unwrap_or(i32::MIN);
    println!("actor => create={actor_create} inc={actor_inc} cur={actor_cur}");

    // ── Generic metadata instantiation parity ───────────────────────────────
    let generic_meta_distinct = factory
        .call_to_i32("swift_generic_metadata_distinct")
        .unwrap_or(0);
    let generic_constrained = factory
        .call_to_i32("swift_generic_constrained_call")
        .unwrap_or(i32::MIN);
    println!(
        "generic metadata => distinct={generic_meta_distinct} constrained={generic_constrained}"
    );

    // ── Value existential dispatch parity ───────────────────────────────────
    let value_existential_current = factory
        .call_to_i32("swift_value_existential_current")
        .unwrap_or(i32::MIN);
    println!("value existential => current={value_existential_current}");

    // ── Resilient layout parity checks ─────────────────────────────────────
    let point_size = factory.call_to_i32("swift_layout_point_size").unwrap_or(i32::MIN);
    let point_stride = factory
        .call_to_i32("swift_layout_point_stride")
        .unwrap_or(i32::MIN);
    let point_align = factory
        .call_to_i32("swift_layout_point_alignment")
        .unwrap_or(i32::MIN);
    let resilient_size = factory
        .call_to_i32("swift_layout_resilient_size")
        .unwrap_or(i32::MIN);
    let resilient_stride = factory
        .call_to_i32("swift_layout_resilient_stride")
        .unwrap_or(i32::MIN);
    let resilient_align = factory
        .call_to_i32("swift_layout_resilient_alignment")
        .unwrap_or(i32::MIN);
    let resilient_b_offset = factory
        .call_to_i32("swift_layout_resilient_b_offset")
        .unwrap_or(i32::MIN);
    println!(
        "resilient layout => point_size={point_size} point_stride={point_stride} point_align={point_align} resilient_size={resilient_size} resilient_stride={resilient_stride} resilient_align={resilient_align} b_offset={resilient_b_offset}"
    );

    // ── ARC edge-case stress checks ────────────────────────────────────────
    let arc_swift_edge = factory
        .call_i32_to_i32("swift_arc_edge_stress", 1000)
        .unwrap_or(0);

    let arc_runtime_balanced = unsafe {
        let new_counter = factory
            .symbol_address("swift_counter_new")
            .map(|p| std::mem::transmute::<*mut c_void, CdeclI32ToPtr>(p));
        let drop_counter = factory
            .symbol_address("swift_counter_drop")
            .map(|p| std::mem::transmute::<*mut c_void, CdeclPtrToVoid>(p));

        match (new_counter, drop_counter) {
            (Ok(new_f), Ok(drop_f)) => {
                let obj = new_f(1);
                let before = factory.retain_count(obj).unwrap_or(0);
                for _ in 0..1000 {
                    let _ = factory.retain(obj);
                    let _ = factory.release(obj);
                }
                let after = factory.retain_count(obj).unwrap_or(0);
                drop_f(obj);
                if before == 1 && after == 1 { 1 } else { 0 }
            }
            _ => 0,
        }
    };
    println!("arc stress => swift_edge={arc_swift_edge} runtime_balance={arc_runtime_balanced}");

    // ── Randomized parity fuzz checks (seeded, deterministic per seed) ─────
    let fuzz_cases = std::env::var("RUNTIME_FUZZ_CASES")
        .ok()
        .and_then(|s| s.parse::<usize>().ok())
        .filter(|n| *n > 0)
        .unwrap_or(64);
    let fuzz_seed = std::env::var("RUNTIME_FUZZ_SEED")
        .ok()
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(0x5A17_62F4);

    let mut state = fuzz_seed;
    let mut add_ok = 1i32;
    let mut divide_ok = 1i32;
    let mut throw_ok = 1i32;
    let throws_sym = "$s10RustBridge10safeDivideys5Int32VAD_ADtKF";

    for _ in 0..fuzz_cases {
        let a = ((lcg_next(&mut state) % 1_000_001) as i64 - 500_000) as i32;
        let b_raw = ((lcg_next(&mut state) % 1_001) as i64 - 500) as i32;
        let b = if b_raw == 0 { 1 } else { b_raw };

        let add_actual = factory
            .call_i32_i32_to_i32("$s10RustBridge9swift_addys5Int32VAD_ADtF", a, b)
            .unwrap_or(i32::MIN);
        if add_actual != a.wrapping_add(b) {
            add_ok = 0;
        }

        let div_result = factory.call_throws_i32_i32(throws_sym, a, b);
        match div_result {
            Ok(swift_runtime_sys::RuntimeFactory::ThrowsResult::Ok(v)) => {
                if v != (a / b) {
                    divide_ok = 0;
                }
            }
            _ => {
                divide_ok = 0;
            }
        }

        let should_throw = (lcg_next(&mut state) & 1) == 0;
        let throw_b = if should_throw { 0 } else { b };
        let throw_result = factory.call_throws_i32_i32(throws_sym, a, throw_b);
        match (should_throw, throw_result) {
            (true, Ok(swift_runtime_sys::RuntimeFactory::ThrowsResult::Threw(_))) => {}
            (false, Ok(swift_runtime_sys::RuntimeFactory::ThrowsResult::Ok(_))) => {}
            _ => {
                throw_ok = 0;
            }
        }
    }

    println!(
        "fuzz parity => add_ok={add_ok} divide_ok={divide_ok} throw_ok={throw_ok} cases={fuzz_cases} seed={fuzz_seed}"
    );
}
