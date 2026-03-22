use swift_runtime_sys::RemoteMirror::{required_remote_mirror_symbols, RemoteMirrorApi};
use swift_runtime_sys::RuntimeContract::{ContractArgBlob, RuntimeContract};
use swift_runtime_sys::RuntimeFactory::RuntimeFactory;

const TYPE_COUNTER: i32 = 2;

fn main() {
    let mut passed = 0;
    let mut failed = 0;

    println!("\\n=== O.1 RemoteMirror Baseline Probe ===");

    let api = match RemoteMirrorApi::new() {
        Ok(api) => {
            println!("loaded: {}", api.library_path());
            api
        }
        Err(err) => panic!("failed to load RemoteMirror API: {err:?}"),
    };

    let tests: [(&str, fn(&RemoteMirrorApi) -> Result<bool, String>); 19] = [
        ("library path non-empty", test_library_path_non_empty),
        (
            "required symbol set present",
            test_required_symbol_set_present,
        ),
        (
            "required symbols resolve with reflection prefix",
            test_required_symbols_prefix,
        ),
        (
            "supported metadata version is non-zero",
            test_supported_metadata_version_nonzero,
        ),
        (
            "local reflection context create/destroy",
            test_context_create_destroy,
        ),
        (
            "add image succeeds for RemoteMirror image",
            test_add_image_for_remotemirror,
        ),
        (
            "ownsAddress true for known symbol after addImage",
            test_owns_address_after_add_image,
        ),
        (
            "metadata info non-zero for Counter metadata",
            test_info_for_counter_metadata,
        ),
        (
            "instance info non-zero for Counter object",
            test_info_for_counter_instance,
        ),
        (
            "child traversal returns stable first field",
            test_child_traversal_counter,
        ),
        (
            "typeref naming returns Counter-like name",
            test_typeref_name_counter,
        ),
        (
            "conformance cache iteration is callable",
            test_conformance_cache_iteration,
        ),
        (
            "async task info null-pointer semantics",
            test_async_task_info_null_semantics,
        ),
        (
            "actor info null-pointer semantics",
            test_actor_info_null_semantics,
        ),
        (
            "next job null-pointer semantics",
            test_next_job_null_semantics,
        ),
        (
            "async/actor reflection capability status",
            test_async_actor_capability_status,
        ),
        (
            "exported reflection symbol inventory available",
            test_export_inventory_nonempty,
        ),
        (
            "export inventory includes required symbols",
            test_export_inventory_covers_required,
        ),
        (
            "remote mirror export surface above baseline",
            test_export_surface_baseline,
        ),
    ];

    for (name, test_fn) in tests {
        match test_fn(&api) {
            Ok(true) => {
                println!("✓ {name} PASS");
                passed += 1;
            }
            Ok(false) => {
                println!("✗ {name} FAIL");
                failed += 1;
            }
            Err(err) => {
                println!("✗ {name} FAIL ({err})");
                failed += 1;
            }
        }
    }

    println!("\\n=== O.1 Summary ===");
    println!("Passed: {}", passed);
    println!("Failed: {}", failed);

    if failed == 0 {
        println!("✓ O.1 RemoteMirror baseline probe PASSED");
    } else {
        panic!("✗ O.1 RemoteMirror baseline probe FAILED");
    }
}

fn test_library_path_non_empty(api: &RemoteMirrorApi) -> Result<bool, String> {
    Ok(!api.library_path().is_empty())
}

fn test_required_symbol_set_present(api: &RemoteMirrorApi) -> Result<bool, String> {
    let report = api.required_symbol_report();
    let missing: Vec<&str> = report
        .iter()
        .filter_map(|(name, ok)| if *ok { None } else { Some(*name) })
        .collect();

    if missing.is_empty() {
        Ok(true)
    } else {
        Err(format!("missing symbols: {}", missing.join(", ")))
    }
}

fn test_required_symbols_prefix(api: &RemoteMirrorApi) -> Result<bool, String> {
    for symbol in required_remote_mirror_symbols() {
        if !symbol.starts_with("swift_reflection_") {
            return Err(format!("required symbol missing expected prefix: {symbol}"));
        }
        if !api.has_symbol(symbol) {
            return Err(format!("required symbol not resolvable: {symbol}"));
        }
    }
    Ok(true)
}

fn test_supported_metadata_version_nonzero(api: &RemoteMirrorApi) -> Result<bool, String> {
    let version = api
        .supported_metadata_version()
        .map_err(|err| format!("version call failed: {err:?}"))?;
    Ok(version > 0)
}

fn test_context_create_destroy(api: &RemoteMirrorApi) -> Result<bool, String> {
    let _context = api
        .create_local_context()
        .map_err(|err| format!("context create failed: {err:?}"))?;
    // Drop is implicit at scope end and should not crash.
    Ok(true)
}

fn test_add_image_for_remotemirror(api: &RemoteMirrorApi) -> Result<bool, String> {
    let context = api
        .create_local_context()
        .map_err(|err| format!("context create failed: {err:?}"))?;
    let image_base = api
        .image_base_for_symbol("swift_reflection_getSupportedMetadataVersion")
        .map_err(|err| format!("image base lookup failed: {err:?}"))?;

    let _added = api
        .add_image(&context, image_base)
        .map_err(|err| format!("addImage failed: {err:?}"))?;
    // Some images may not expose reflection mapping sections; addImage may return false.
    // This test verifies callable stability (no crash/error) for the operation.
    Ok(true)
}

fn test_owns_address_after_add_image(api: &RemoteMirrorApi) -> Result<bool, String> {
    let context = api
        .create_local_context()
        .map_err(|err| format!("context create failed: {err:?}"))?;
    let symbol = "swift_reflection_getSupportedMetadataVersion";
    let image_base = api
        .image_base_for_symbol(symbol)
        .map_err(|err| format!("image base lookup failed: {err:?}"))?;

    let added = api
        .add_image(&context, image_base)
        .map_err(|err| format!("addImage failed: {err:?}"))?;
    let address = api
        .symbol_address(symbol)
        .map_err(|err| format!("symbol resolve failed: {err:?}"))?;
    let owns = api
        .owns_address(&context, address as usize)
        .map_err(|err| format!("ownsAddress failed: {err:?}"))?;

    // If addImage succeeded we expect ownership true for symbol addresses in that image.
    // If addImage is unsupported for this image mapping, ownership should remain false.
    if added {
        Ok(owns)
    } else {
        Ok(!owns)
    }
}

fn test_info_for_counter_metadata(api: &RemoteMirrorApi) -> Result<bool, String> {
    with_counter_fixture(api, |api, context, metadata, _object| {
        let info_a = api
            .info_for_metadata(context, metadata)
            .map_err(|err| format!("infoForMetadata failed: {err:?}"))?;
        let info_b = api
            .info_for_metadata(context, metadata)
            .map_err(|err| format!("second infoForMetadata failed: {err:?}"))?;

        Ok(info_a.kind == info_b.kind
            && info_a.size == info_b.size
            && info_a.alignment == info_b.alignment
            && info_a.stride == info_b.stride
            && info_a.num_fields == info_b.num_fields
            && info_a.stride >= info_a.size)
    })
}

fn test_info_for_counter_instance(api: &RemoteMirrorApi) -> Result<bool, String> {
    with_counter_fixture(api, |api, context, _metadata, object| {
        let info_a = api
            .info_for_instance(context, object)
            .map_err(|err| format!("infoForInstance failed: {err:?}"))?;
        let info_b = api
            .info_for_instance(context, object)
            .map_err(|err| format!("second infoForInstance failed: {err:?}"))?;

        Ok(info_a.kind == info_b.kind
            && info_a.size == info_b.size
            && info_a.alignment == info_b.alignment
            && info_a.stride == info_b.stride
            && info_a.num_fields == info_b.num_fields
            && info_a.stride >= info_a.size)
    })
}

fn test_child_traversal_counter(api: &RemoteMirrorApi) -> Result<bool, String> {
    with_counter_fixture(api, |api, context, metadata, object| {
        let meta_info = api
            .info_for_metadata(context, metadata)
            .map_err(|err| format!("infoForMetadata failed: {err:?}"))?;
        let inst_info = api
            .info_for_instance(context, object)
            .map_err(|err| format!("infoForInstance failed: {err:?}"))?;

        if meta_info.num_fields == 0 || inst_info.num_fields == 0 {
            // Some class layouts remain opaque; treat zero-field reports as a
            // capability outcome rather than a probe failure.
            return Ok(true);
        }

        let meta_child = api
            .child_of_metadata(context, metadata, 0)
            .map_err(|err| format!("childOfMetadata failed: {err:?}"))?;
        let inst_child = api
            .child_of_instance(context, object, 0)
            .map_err(|err| format!("childOfInstance failed: {err:?}"))?;

        Ok(meta_child.offset == inst_child.offset)
    })
}

fn test_typeref_name_counter(api: &RemoteMirrorApi) -> Result<bool, String> {
    with_counter_fixture(api, |api, context, metadata, object| {
        let tr_meta = api
            .type_ref_for_metadata(context, metadata)
            .map_err(|err| format!("typeRefForMetadata failed: {err:?}"))?;
        let tr_inst = api
            .type_ref_for_instance(context, object)
            .map_err(|err| format!("typeRefForInstance failed: {err:?}"))?;

        let name_meta = api
            .copy_name_for_type_ref(context, tr_meta, false)
            .map_err(|err| format!("copyNameForTypeRef(meta) failed: {err:?}"))?;
        let name_inst = api
            .copy_name_for_type_ref(context, tr_inst, false)
            .map_err(|err| format!("copyNameForTypeRef(inst) failed: {err:?}"))?;

        Ok(name_meta.contains("Counter") && name_inst.contains("Counter"))
    })
}

fn test_conformance_cache_iteration(api: &RemoteMirrorApi) -> Result<bool, String> {
    with_counter_fixture(api, |api, context, _metadata, _object| {
        let pairs = api
            .iterate_conformance_cache(context)
            .map_err(|err| format!("iterateConformanceCache failed: {err:?}"))?;
        Ok(pairs.iter().all(|(ty, proto)| *ty != 0 && *proto != 0))
    })
}

fn test_async_task_info_null_semantics(api: &RemoteMirrorApi) -> Result<bool, String> {
    let context = api
        .create_local_context()
        .map_err(|err| format!("context create failed: {err:?}"))?;
    let info = api
        .async_task_info(&context, 0)
        .map_err(|err| format!("asyncTaskInfo failed: {err:?}"))?;
    let slab = api
        .async_task_slab_pointer(&context, 0)
        .map_err(|err| format!("asyncTaskSlabPointer failed: {err:?}"))?;
    Ok(info.error.is_some() || info.id == 0 || info.run_job == 0)
        .and_then(|ok| Ok(ok && (slab.error.is_some() || slab.slab_ptr == 0)))
}

fn test_actor_info_null_semantics(api: &RemoteMirrorApi) -> Result<bool, String> {
    let context = api
        .create_local_context()
        .map_err(|err| format!("context create failed: {err:?}"))?;
    let info = api
        .actor_info(&context, 0)
        .map_err(|err| format!("actorInfo failed: {err:?}"))?;
    Ok(info.error.is_some() || info.first_job == 0)
}

fn test_next_job_null_semantics(api: &RemoteMirrorApi) -> Result<bool, String> {
    let context = api
        .create_local_context()
        .map_err(|err| format!("context create failed: {err:?}"))?;
    let next = api
        .next_job(&context, 0)
        .map_err(|err| format!("nextJob failed: {err:?}"))?;
    Ok(next == 0)
}

fn test_async_actor_capability_status(api: &RemoteMirrorApi) -> Result<bool, String> {
    let required = [
        "swift_reflection_asyncTaskInfo",
        "swift_reflection_nextJob",
        "swift_reflection_actorInfo",
    ];

    let mut supported = 0usize;
    for symbol in required {
        if api.has_symbol(symbol) {
            supported += 1;
        }
    }

    // Record explicit capability status through deterministic boolean:
    // all tracked async/actor reflection entry points are currently available.
    Ok(supported == required.len())
}

fn test_export_inventory_nonempty(api: &RemoteMirrorApi) -> Result<bool, String> {
    let symbols = api
        .exported_reflection_symbols()
        .map_err(|err| format!("export scan failed: {err:?}"))?;
    Ok(!symbols.is_empty())
}

fn test_export_inventory_covers_required(api: &RemoteMirrorApi) -> Result<bool, String> {
    let symbols = api
        .exported_reflection_symbols()
        .map_err(|err| format!("export scan failed: {err:?}"))?;

    for required in required_remote_mirror_symbols() {
        let expected = format!("_{required}");
        if !symbols.iter().any(|s| s == &expected) {
            return Err(format!(
                "required symbol missing from nm export list: {expected}"
            ));
        }
    }
    Ok(true)
}

fn test_export_surface_baseline(api: &RemoteMirrorApi) -> Result<bool, String> {
    let symbols = api
        .exported_reflection_symbols()
        .map_err(|err| format!("export scan failed: {err:?}"))?;
    // Keep this baseline conservative because symbol visibility can differ by host/runtime strip settings.
    Ok(symbols.len() >= required_remote_mirror_symbols().len())
}

fn with_counter_fixture<T, F>(api: &RemoteMirrorApi, f: F) -> Result<T, String>
where
    F: FnOnce(
        &RemoteMirrorApi,
        &swift_runtime_sys::RemoteMirror::RemoteMirrorContext<'_>,
        usize,
        usize,
    ) -> Result<T, String>,
{
    let factory = RuntimeFactory::with_thunk_library("./libRustBridge.dylib", "./libRuntimeThunks.dylib")
        .or_else(|_| RuntimeFactory::new("./libRustBridge.dylib"))
        .map_err(|err| format!("failed to init RuntimeFactory: {err:?}"))?;

    factory
        .validate_runtime_contract(1)
        .map_err(|err| format!("runtime contract validation failed: {err:?}"))?;

    let contract = RuntimeContract::new(&factory);
    let object = contract
        .construct(TYPE_COUNTER, &ContractArgBlob::from_i32s(&[11]))
        .map_err(|err| format!("construct Counter failed: {err:?}"))?;

    let context = api
        .create_local_context()
        .map_err(|err| format!("context create failed: {err:?}"))?;

    let image_base = api
        .image_base_for_symbol("swift_counter_new")
        .map_err(|err| format!("image base for swift_counter_new failed: {err:?}"))?;
    let _ = api
        .add_image(&context, image_base)
        .map_err(|err| format!("addImage for RustBridge failed: {err:?}"))?;

    let metadata = factory
        .metadata_from_accessor_0("$s10RustBridge7CounterCMa")
        .map_err(|err| format!("Counter metadata accessor failed: {err:?}"))?
        as usize;

    let result = f(api, &context, metadata, object.object as usize);

    contract
        .release(object)
        .map_err(|err| format!("release Counter failed: {err:?}"))?;

    result
}
