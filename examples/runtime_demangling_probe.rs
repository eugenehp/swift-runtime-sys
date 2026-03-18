extern crate swift_runtime_sys;

use swift_runtime_sys::SymbolDemangler::SymbolDemangler;

fn main() {
    let mut demangler = SymbolDemangler::new()
        .unwrap_or_else(|e| panic!("failed to create demangler: {e}"));

    let mut passed = 0;
    let mut total = 0;

    // Test vector: (mangled_symbol, expected_substring_in_demangled_output)
    let test_cases = vec![
        // ContractAnyBox exports
        ("_$s10RustBridge23swift_contract_any_wrapySvSgs5Int32V_ACtF", "any_wrap"),
        ("_$s10RustBridge26swift_contract_any_type_idys5Int32VSvSgF", "any_type_id"),
        ("_$s10RustBridge27swift_contract_dynamic_castySvSgAC_s5Int32Vt", "dynamic_cast"),
        
        // String operations
        ("_$s10RustBridge31swift_contract_construct_stringySvSgSVSg_s5Int32VtF", "construct_string"),
        ("_$s10RustBridge24swift_contract_string_lenys5Int32VSvSgF", "string_len"),
        
        // Array operations
        ("_$s10RustBridge27swift_contract_array_makeySvSgs5Int32VF", "array_make"),
        ("_$s10RustBridge25swift_contract_array_lenys5Int32VSvSgF", "array_len"),
        
        // Dictionary operations
        ("_$s10RustBridge28swift_contract_dict_i32_makeySvSgs5Int32VF", "dict_i32_make"),
        ("_$s10RustBridge26swift_contract_dict_i32_lenys5Int32VSvSgF", "dict_i32_len"),
        
        // Release operation
        ("_$s10RustBridge25swift_contract_releaseySiSs5Int32V_SvSgtF", "contract_release"),
    ];

    println!("Symbol demangling parity probe\n");
    println!("Testing {} symbols:\n", test_cases.len());

    for (mangled, expected_substr) in test_cases {
        total += 1;
        let demangled = demangler.demangle(mangled);
        
        if demangled.contains(expected_substr) {
            passed += 1;
            println!("PASS: {}", expected_substr);
            println!("      Demangled: {}", demangled);
        } else {
            println!("FAIL: Expected substring '{}' not found", expected_substr);
            println!("      Mangled:   {}", mangled);
            println!("      Demangled: {}", demangled);
        }
    }

    println!("\nSymbol demangling parity probe results: {}/{} PASS", passed, total);
    println!("Cache size (symbols demangled): {}", demangler.cache_size());
    
    if passed != total {
        std::process::exit(1);
    }
}
