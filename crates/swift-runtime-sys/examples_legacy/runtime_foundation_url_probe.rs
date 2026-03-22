/// Foundation URL / URLComponents probe for Track I.3.
/// Tests URL validation, scheme/host/path extraction, component construction.
use swift_runtime_sys::RuntimeContract::{RuntimeContract, RuntimeContractError};
use swift_runtime_sys::RuntimeFactory::RuntimeFactory;

fn main() {
    let factory = RuntimeFactory::with_thunk_library("./libRustBridge.dylib", "./libRuntimeThunks.dylib")
        .or_else(|_| RuntimeFactory::new("./libRustBridge.dylib"))
        .unwrap_or_else(|e| panic!("failed to init RuntimeFactory: {e:?}"));

    let _descriptor = factory
        .validate_runtime_contract(1)
        .unwrap_or_else(|e| panic!("runtime contract validation failed: {e:?}"));

    let contract = RuntimeContract::new(&factory);

    let mut passed = 0;
    let mut failed = 0;

    println!("\n=== Foundation URL / URLComponents (Track I.3) ===");

    let tests: [(&str, fn(&RuntimeContract) -> Result<bool, RuntimeContractError>); 7] = [
        ("HTTPS URL is valid", test_valid_https),
        ("Empty string is invalid URL", test_invalid_empty),
        ("Scheme extraction = 'https'", test_scheme),
        ("Host extraction = 'example.com'", test_host),
        ("Path extraction = '/path'", test_path),
        ("Build URL from components matches expected", test_build),
        ("Built URL is valid", test_build_valid),
    ];

    for (name, test_fn) in tests {
        match test_fn(&contract) {
            Ok(true) => { println!("✓ {name} PASS"); passed += 1; }
            Ok(false) => { println!("✗ {name} FAIL"); failed += 1; }
            Err(err) => { println!("✗ {name} FAIL ({err:?})"); failed += 1; }
        }
    }

    println!("\n=== Track I.3 Summary ===");
    println!("Tests Passed: {}", passed);
    println!("Tests Failed: {}", failed);

    if failed == 0 {
        println!("✓ All Track I.3 tests PASSED");
    } else {
        panic!("✗ Track I.3 tests FAILED");
    }
}

const TEST_URL: &str = "https://example.com/path";

fn test_valid_https(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.url_parse_valid(TEST_URL)
}

fn test_invalid_empty(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let valid = contract.url_parse_valid("")?;
    Ok(!valid)
}

fn test_scheme(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.url_scheme(TEST_URL)? == "https")
}

fn test_host(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.url_host(TEST_URL)? == "example.com")
}

fn test_path(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.url_path(TEST_URL)? == "/path")
}

fn test_build(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let url = contract.url_build_from_components("https", "example.com", "/test")?;
    Ok(url == "https://example.com/test")
}

fn test_build_valid(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let url = contract.url_build_from_components("https", "example.com", "/test")?;
    contract.url_parse_valid(&url)
}
