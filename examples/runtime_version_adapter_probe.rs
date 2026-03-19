// Phase B.3 Probe: Cross-Version ABI Compatibility Shim
// RuntimeContract-based validation of version detection and adapter selection.

use swift_runtime_sys::RuntimeContract::RuntimeContract;
use swift_runtime_sys::RuntimeFactory::RuntimeFactory;

fn main() {
    let factory = RuntimeFactory::with_thunk_library("./libRustBridge.dylib", "./libRuntimeThunks.dylib")
        .or_else(|_| RuntimeFactory::new("./libRustBridge.dylib"))
        .unwrap_or_else(|e| panic!("failed to init RuntimeFactory: {e:?}"));

    factory
        .validate_runtime_contract(1)
        .unwrap_or_else(|e| panic!("runtime contract validation failed: {e:?}"));

    let contract = RuntimeContract::new(&factory);

    println!("=== Phase B.3 Runtime Version Adapter Probe ===\n");

    let mut pass = 0;
    let mut total = 0;

    total += 1;
    match contract.b3_detect_runtime_version() {
        Ok(v) if !v.version_string.is_empty() => {
            println!("PASS 1 version {}.{}.{} ({})", v.major, v.minor, v.patch, v.version_string);
            pass += 1;
        }
        Ok(v) => println!("FAIL 1 empty version_string: {:?}", v),
        Err(e) => println!("FAIL 1 detect version: {:?}", e),
    }

    total += 1;
    match contract.b3_get_adapter_table("swift_6_2_arm64_macos") {
        Ok(layouts) if !layouts.is_empty() => {
            println!("PASS 2 adapter table entries={}", layouts.len());
            pass += 1;
        }
        Ok(_) => println!("FAIL 2 adapter table empty"),
        Err(e) => println!("FAIL 2 adapter table error: {:?}", e),
    }

    total += 1;
    match contract.b3_get_adapter_table("swift_6_2_arm64_macos") {
        Ok(layouts) if layouts.iter().any(|l| l.type_name == "String") => {
            println!("PASS 3 found String layout");
            pass += 1;
        }
        Ok(_) => println!("FAIL 3 missing String layout"),
        Err(e) => println!("FAIL 3 parse error: {:?}", e),
    }

    total += 1;
    match contract.b3_select_adapter_profile("swift_6_2_arm64_macos") {
        Ok(true) => {
            println!("PASS 4 valid profile selected");
            pass += 1;
        }
        other => println!("FAIL 4 valid profile select failed: {:?}", other),
    }

    total += 1;
    if contract.b3_select_adapter_profile("invalid_profile").is_err() {
        println!("PASS 5 invalid profile rejected");
        pass += 1;
    } else {
        println!("FAIL 5 invalid profile should fail");
    }

    total += 1;
    if contract.b3_auto_select_profile().is_ok() {
        println!("PASS 6 auto-select profile");
        pass += 1;
    } else {
        println!("FAIL 6 auto-select profile failed");
    }

    total += 1;
    match contract.b3_get_field_offset("String", "_guts") {
        Ok(offset) => {
            println!("PASS 7 field offset String._guts={}", offset);
            pass += 1;
        }
        Err(e) => println!("FAIL 7 field offset lookup failed: {:?}", e),
    }

    total += 1;
    let v1 = contract.b3_detect_runtime_version();
    let v2 = contract.b3_detect_runtime_version();
    match (v1, v2) {
        (Ok(a), Ok(b)) if a.major == b.major && a.minor == b.minor && a.patch == b.patch => {
            println!("PASS 8 version detection stable");
            pass += 1;
        }
        (Ok(a), Ok(b)) => println!("FAIL 8 unstable version detect: {:?} vs {:?}", a, b),
        _ => println!("FAIL 8 version detection failed"),
    }

    println!("\n=== Summary ===");
    println!("Passed: {}/{}", pass, total);
    if pass == total {
        println!("Status: ALL TESTS PASSED");
        std::process::exit(0);
    } else {
        println!("Status: {} TESTS FAILED", total - pass);
        std::process::exit(1);
    }
}
