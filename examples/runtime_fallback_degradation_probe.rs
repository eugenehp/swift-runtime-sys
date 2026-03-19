// Phase B.4 Probe: Fallback & Graceful Degradation
// 12 test cases for capability negotiation and safe unsupported-op handling

use std::ffi::CString;

mod bridge {
    use std::ffi::{CStr, CString};

    extern "C" {
        pub fn swift_contract_b4_is_feature_supported(feature_name: *const i8) -> i32;
        pub fn swift_contract_b4_supported_features_json() -> *mut i8;
        pub fn swift_contract_b3_runtime_version_json() -> *mut i8;
    }

    pub fn parse_json_string(ptr: *mut i8) -> Option<String> {
        if ptr.is_null() {
            return None;
        }
        unsafe {
            let c_str = CStr::from_ptr(ptr);
            let result = c_str.to_string_lossy().to_string();
            libc::free(ptr as *mut std::ffi::c_void);
            Some(result)
        }
    }

    pub fn is_feature_supported(name: &str) -> bool {
        let c = CString::new(name).unwrap();
        unsafe { swift_contract_b4_is_feature_supported(c.as_ptr()) != 0 }
    }
}

fn main() {
    println!("=== Phase B.4 Fallback & Degradation Probe ===\n");

    let mut pass_count = 0;
    let mut total_count = 0;

    // Test 1: Capability negotiation function is reachable
    total_count += 1;
    {
        let c = CString::new("metadata_introspection").unwrap();
        let result = unsafe { bridge::swift_contract_b4_is_feature_supported(c.as_ptr()) };
        // Should return 0 or 1; not a crash
        println!("✓ Test 1: capability negotiation is reachable (result={})", result);
        pass_count += 1;
    }

    // Test 2: Known supported feature "metadata_introspection" is reported supported
    total_count += 1;
    {
        if bridge::is_feature_supported("metadata_introspection") {
            println!("✓ Test 2: metadata_introspection is supported");
            pass_count += 1;
        } else {
            println!("✗ Test 2: metadata_introspection should be supported");
        }
    }

    // Test 3: Known supported feature "witness_table_resolution" is reported supported
    total_count += 1;
    {
        if bridge::is_feature_supported("witness_table_resolution") {
            println!("✓ Test 3: witness_table_resolution is supported");
            pass_count += 1;
        } else {
            println!("✗ Test 3: witness_table_resolution should be supported");
        }
    }

    // Test 4: Known supported feature "version_adapter" is reported supported
    total_count += 1;
    {
        if bridge::is_feature_supported("version_adapter") {
            println!("✓ Test 4: version_adapter is supported");
            pass_count += 1;
        } else {
            println!("✗ Test 4: version_adapter should be supported");
        }
    }

    // Test 5: Nonexistent feature returns unsupported
    total_count += 1;
    {
        if !bridge::is_feature_supported("__nonexistent_feature_xyz__") {
            println!("✓ Test 5: nonexistent feature correctly unsupported");
            pass_count += 1;
        } else {
            println!("✗ Test 5: nonexistent feature should not be supported");
        }
    }

    // Test 6: Null feature name does not crash, returns unsupported
    total_count += 1;
    {
        let result = unsafe { bridge::swift_contract_b4_is_feature_supported(std::ptr::null()) };
        if result == 0 {
            println!("✓ Test 6: null feature name gracefully handled");
            pass_count += 1;
        } else {
            println!("✗ Test 6: null feature name should return 0");
        }
    }

    // Test 7: Get full supported features list as JSON
    total_count += 1;
    {
        let json_ptr = unsafe { bridge::swift_contract_b4_supported_features_json() };
        if let Some(json_str) = bridge::parse_json_string(json_ptr) {
            if json_str.starts_with('[') && json_str.ends_with(']') {
                println!("✓ Test 7: supported_features_json returns valid JSON array");
                pass_count += 1;
            } else {
                println!("✗ Test 7: supported_features_json malformed: {}", json_str);
            }
        } else {
            println!("✗ Test 7: supported_features_json returned null");
        }
    }

    // Test 8: Features list contains at least 5 entries
    total_count += 1;
    {
        let json_ptr = unsafe { bridge::swift_contract_b4_supported_features_json() };
        if let Some(json_str) = bridge::parse_json_string(json_ptr) {
            if let Ok(features) = serde_json::from_str::<Vec<String>>(&json_str) {
                if features.len() >= 5 {
                    println!("✓ Test 8: features list has {} entries (>= 5)", features.len());
                    pass_count += 1;
                } else {
                    println!("✗ Test 8: features list too short: {}", features.len());
                }
            } else {
                println!("✗ Test 8: could not parse features JSON");
            }
        }
    }

    // Test 9: Features list includes "error_propagation"
    total_count += 1;
    {
        let json_ptr = unsafe { bridge::swift_contract_b4_supported_features_json() };
        if let Some(json_str) = bridge::parse_json_string(json_ptr) {
            if let Ok(features) = serde_json::from_str::<Vec<String>>(&json_str) {
                if features.contains(&"error_propagation".to_string()) {
                    println!("✓ Test 9: error_propagation in features list");
                    pass_count += 1;
                } else {
                    println!("✗ Test 9: error_propagation missing from features list");
                }
            }
        }
    }

    // Test 10: Features list includes "array_bridging"
    total_count += 1;
    {
        let json_ptr = unsafe { bridge::swift_contract_b4_supported_features_json() };
        if let Some(json_str) = bridge::parse_json_string(json_ptr) {
            if let Ok(features) = serde_json::from_str::<Vec<String>>(&json_str) {
                if features.contains(&"array_bridging".to_string()) {
                    println!("✓ Test 10: array_bridging in features list");
                    pass_count += 1;
                } else {
                    println!("✗ Test 10: array_bridging missing from features list");
                }
            }
        }
    }

    // Test 11: Features list is sorted (verified alphabetically)
    total_count += 1;
    {
        let json_ptr = unsafe { bridge::swift_contract_b4_supported_features_json() };
        if let Some(json_str) = bridge::parse_json_string(json_ptr) {
            if let Ok(features) = serde_json::from_str::<Vec<String>>(&json_str) {
                let mut sorted = features.clone();
                sorted.sort();
                if features == sorted {
                    println!("✓ Test 11: features list is sorted");
                    pass_count += 1;
                } else {
                    println!("✗ Test 11: features list is not sorted");
                }
            }
        }
    }

    // Test 12: Version detection + capability negotiation are consistent (both available)
    total_count += 1;
    {
        let version_ptr = unsafe { bridge::swift_contract_b3_runtime_version_json() };
        let version_ok = bridge::parse_json_string(version_ptr).map(|j| j.contains("major")).unwrap_or(false);
        let capability_ok = bridge::is_feature_supported("version_adapter");

        if version_ok && capability_ok {
            println!("✓ Test 12: version detection and capability negotiation consistent");
            pass_count += 1;
        } else {
            println!("✗ Test 12: version={}, capability={}", version_ok, capability_ok);
        }
    }

    println!("\n=== Summary ===");
    println!("Passed: {}/{}", pass_count, total_count);
    if pass_count == total_count {
        println!("Status: ALL TESTS PASSED ✓");
        std::process::exit(0);
    } else {
        println!("Status: {} TESTS FAILED ✗", total_count - pass_count);
        std::process::exit(1);
    }
}
