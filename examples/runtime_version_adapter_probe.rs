// Phase B.3 Probe: Cross-Version ABI Compatibility Shim
// 15 test cases for version detection and adapter selection

use std::ffi::CString;

mod bridge {
    use std::ffi::CStr;

    #[repr(C)]
    pub struct ContractObject {
        ptr: *mut std::ffi::c_void,
    }

    extern "C" {
        pub fn swift_runtime_create_empty() -> ContractObject;
        pub fn swift_contract_b3_runtime_version_json() -> *mut i8;
        pub fn swift_contract_b3_get_adapter_table_json(
            profile_id: *const i8,
        ) -> *mut i8;
        pub fn swift_contract_b3_select_adapter_profile(profile_id: *const i8) -> i32;
    }

    pub fn parse_json_string(ptr: *mut i8) -> String {
        if ptr.is_null() {
            return "null".to_string();
        }
        unsafe {
            let c_str = CStr::from_ptr(ptr);
            let result = c_str.to_string_lossy().to_string();
            libc::free(ptr as *mut std::ffi::c_void);
            result
        }
    }
}

fn main() {
    println!("=== Phase B.3 Runtime Version Adapter Probe ===\n");

    let mut pass_count = 0;
    let mut total_count = 0;

    // Test 1: Detect current runtime version
    total_count += 1;
    {
        let json_ptr = unsafe { bridge::swift_contract_b3_runtime_version_json() };
        if !json_ptr.is_null() {
            let json_str = bridge::parse_json_string(json_ptr);
            if json_str.contains("\"major\"") && json_str.contains("\"minor\"") {
                println!("✓ Test 1: Version detection returns valid JSON");
                pass_count += 1;
            } else {
                println!("✗ Test 1: Version JSON malformed: {}", json_str);
            }
        } else {
            println!("✗ Test 1: Version detection returned null");
        }
    }

    // Test 2: Version JSON contains major version
    total_count += 1;
    {
        let json_ptr = unsafe { bridge::swift_contract_b3_runtime_version_json() };
        let json_str = bridge::parse_json_string(json_ptr);
        if json_str.contains("\"major\":") {
            if let Ok(parsed) = json_str.parse::<serde_json::Value>() {
                if let Some(major) = parsed.get("major").and_then(|v| v.as_i64()) {
                    if major >= 6 {
                        println!("✓ Test 2: Major version {} detected", major);
                        pass_count += 1;
                    } else {
                        println!("✗ Test 2: Invalid major version: {}", major);
                    }
                } else {
                    println!("✗ Test 2: Could not parse major version");
                }
            }
        } else {
            println!("✗ Test 2: JSON missing major field");
        }
    }

    // Test 3: Version JSON contains minor version
    total_count += 1;
    {
        let json_ptr = unsafe { bridge::swift_contract_b3_runtime_version_json() };
        let json_str = bridge::parse_json_string(json_ptr);
        if let Ok(parsed) = json_str.parse::<serde_json::Value>() {
            if let Some(minor) = parsed.get("minor").and_then(|v| v.as_i64()) {
                if minor >= 0 && minor <= 10 {
                    println!("✓ Test 3: Minor version {} detected", minor);
                    pass_count += 1;
                } else {
                    println!("✗ Test 3: Invalid minor version: {}", minor);
                }
            } else {
                println!("✗ Test 3: Could not parse minor version");
            }
        }
    }

    // Test 4: Version JSON contains patch version
    total_count += 1;
    {
        let json_ptr = unsafe { bridge::swift_contract_b3_runtime_version_json() };
        let json_str = bridge::parse_json_string(json_ptr);
        if let Ok(parsed) = json_str.parse::<serde_json::Value>() {
            if let Some(patch) = parsed.get("patch").and_then(|v| v.as_i64()) {
                if patch >= 0 {
                    println!("✓ Test 4: Patch version {} detected", patch);
                    pass_count += 1;
                } else {
                    println!("✗ Test 4: Invalid patch version: {}", patch);
                }
            } else {
                println!("✗ Test 4: Could not parse patch version");
            }
        }
    }

    // Test 5: Version JSON contains version_string
    total_count += 1;
    {
        let json_ptr = unsafe { bridge::swift_contract_b3_runtime_version_json() };
        let json_str = bridge::parse_json_string(json_ptr);
        if let Ok(parsed) = json_str.parse::<serde_json::Value>() {
            if let Some(version_str) = parsed.get("version_string").and_then(|v| v.as_str()) {
                if version_str.contains(".") {
                    println!("✓ Test 5: Version string '{}' detected", version_str);
                    pass_count += 1;
                } else {
                    println!("✗ Test 5: Version string malformed: {}", version_str);
                }
            } else {
                println!("✗ Test 5: Could not extract version_string");
            }
        }
    }

    // Test 6: Get adapter table for Swift 6.2
    total_count += 1;
    {
        let profile_id = CString::new("swift_6_2_arm64_macos").unwrap();
        let json_ptr = unsafe { bridge::swift_contract_b3_get_adapter_table_json(profile_id.as_ptr()) };
        if !json_ptr.is_null() {
            let json_str = bridge::parse_json_string(json_ptr);
            if json_str.contains("type_name") {
                println!("✓ Test 6: Adapter table for 6.2 retrieved");
                pass_count += 1;
            } else {
                println!("✗ Test 6: Adapter table missing type_name fields");
            }
        } else {
            println!("✗ Test 6: Adapter table retrieval returned null");
        }
    }

    // Test 7: Adapter table contains String type layout
    total_count += 1;
    {
        let profile_id = CString::new("swift_6_2_arm64_macos").unwrap();
        let json_ptr = unsafe { bridge::swift_contract_b3_get_adapter_table_json(profile_id.as_ptr()) };
        let json_str = bridge::parse_json_string(json_ptr);
        if let Ok(parsed) = json_str.parse::<serde_json::Value>() {
            if let Some(arr) = parsed.as_array() {
                for item in arr {
                    if let Some(type_name) = item.get("type_name").and_then(|v| v.as_str()) {
                        if type_name == "String" {
                            println!("✓ Test 7: String type layout found in adapter table");
                            pass_count += 1;
                            break;
                        }
                    }
                }
                if pass_count < total_count {
                    println!("✗ Test 7: String type layout not found");
                }
            }
        }
    }

    // Test 8: Adapter table contains Array<Int32> type layout
    total_count += 1;
    {
        let profile_id = CString::new("swift_6_2_arm64_macos").unwrap();
        let json_ptr = unsafe { bridge::swift_contract_b3_get_adapter_table_json(profile_id.as_ptr()) };
        let json_str = bridge::parse_json_string(json_ptr);
        if let Ok(parsed) = json_str.parse::<serde_json::Value>() {
            if let Some(arr) = parsed.as_array() {
                for item in arr {
                    if let Some(type_name) = item.get("type_name").and_then(|v| v.as_str()) {
                        if type_name.contains("Array") {
                            println!("✓ Test 8: Array type layout found");
                            pass_count += 1;
                            break;
                        }
                    }
                }
                if pass_count < total_count {
                    println!("✗ Test 8: Array type layout not found");
                }
            }
        }
    }

    // Test 9: Adapter table type layouts include size field
    total_count += 1;
    {
        let profile_id = CString::new("swift_6_2_arm64_macos").unwrap();
        let json_ptr = unsafe { bridge::swift_contract_b3_get_adapter_table_json(profile_id.as_ptr()) };
        let json_str = bridge::parse_json_string(json_ptr);
        if let Ok(parsed) = json_str.parse::<serde_json::Value>() {
            if let Some(arr) = parsed.as_array() {
                for item in arr {
                    if let Some(size) = item.get("size").and_then(|v| v.as_i64()) {
                        if size > 0 {
                            println!("✓ Test 9: Type layout size field present ({})", size);
                            pass_count += 1;
                            break;
                        }
                    }
                }
                if pass_count < total_count {
                    println!("✗ Test 9: Size field missing or invalid");
                }
            }
        }
    }

    // Test 10: Adapter table type layouts include field offsets
    total_count += 1;
    {
        let profile_id = CString::new("swift_6_2_arm64_macos").unwrap();
        let json_ptr = unsafe { bridge::swift_contract_b3_get_adapter_table_json(profile_id.as_ptr()) };
        let json_str = bridge::parse_json_string(json_ptr);
        if let Ok(parsed) = json_str.parse::<serde_json::Value>() {
            if let Some(arr) = parsed.as_array() {
                for item in arr {
                    if let Some(fields) = item.get("fields").and_then(|v| v.as_array()) {
                        if !fields.is_empty() {
                            if let Some(field) = fields.first() {
                                if field.get("offset").is_some() {
                                    println!("✓ Test 10: Field offset information present");
                                    pass_count += 1;
                                    break;
                                }
                            }
                        }
                    }
                }
                if pass_count < total_count {
                    println!("✗ Test 10: Field offset information missing");
                }
            }
        }
    }

    // Test 11: Select valid adapter profile succeeds
    total_count += 1;
    {
        let profile_id = CString::new("swift_6_2_arm64_macos").unwrap();
        let result = unsafe { bridge::swift_contract_b3_select_adapter_profile(profile_id.as_ptr()) };
        if result != 0 {
            println!("✓ Test 11: Valid profile selection succeeded");
            pass_count += 1;
        } else {
            println!("✗ Test 11: Valid profile selection failed");
        }
    }

    // Test 12: Select invalid profile ID fails correctly
    total_count += 1;
    {
        let profile_id = CString::new("invalid_profile").unwrap();
        let result = unsafe { bridge::swift_contract_b3_select_adapter_profile(profile_id.as_ptr()) };
        if result == 0 {
            println!("✓ Test 12: Invalid profile correctly rejected");
            pass_count += 1;
        } else {
            println!("✗ Test 12: Invalid profile should have failed");
        }
    }

    // Test 13: Select null profile ID fails safely
    total_count += 1;
    {
        let result = unsafe { bridge::swift_contract_b3_select_adapter_profile(std::ptr::null()) };
        if result == 0 {
            println!("✓ Test 13: Null profile ID handled safely");
            pass_count += 1;
        } else {
            println!("✗ Test 13: Null profile ID not handled");
        }
    }

    // Test 14: x86_64 architecture profile is supported
    total_count += 1;
    {
        let profile_id = CString::new("swift_6_2_x86_64_macos").unwrap();
        let result = unsafe { bridge::swift_contract_b3_select_adapter_profile(profile_id.as_ptr()) };
        if result != 0 {
            println!("✓ Test 14: x86_64 architecture profile supported");
            pass_count += 1;
        } else {
            println!("✗ Test 14: x86_64 architecture profile not supported");
        }
    }

    // Test 15: Version consistency across multiple calls
    total_count += 1;
    {
        let json_ptr1 = unsafe { bridge::swift_contract_b3_runtime_version_json() };
        let json_str1 = bridge::parse_json_string(json_ptr1);
        let json_ptr2 = unsafe { bridge::swift_contract_b3_runtime_version_json() };
        let json_str2 = bridge::parse_json_string(json_ptr2);
        
        if json_str1 == json_str2 {
            println!("✓ Test 15: Version detection consistent across calls");
            pass_count += 1;
        } else {
            println!("✗ Test 15: Version detection inconsistent");
            println!("  First call:  {}", json_str1);
            println!("  Second call: {}", json_str2);
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
