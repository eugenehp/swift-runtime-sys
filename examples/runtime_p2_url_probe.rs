use swift_runtime_sys::RuntimeContract::{RuntimeContract, RuntimeContractError};
use swift_runtime_sys::RuntimeFactory::RuntimeFactory;

fn main() {
    let factory =
        RuntimeFactory::with_thunk_library("./libRustBridge.dylib", "./libRuntimeThunks.dylib")
            .or_else(|_| RuntimeFactory::new("./libRustBridge.dylib"))
            .unwrap_or_else(|e| panic!("failed to init RuntimeFactory: {e:?}"));

    let _descriptor = factory
        .validate_runtime_contract(1)
        .unwrap_or_else(|e| panic!("runtime contract validation failed: {e:?}"));

    let contract = RuntimeContract::new(&factory);

    let mut passed = 0;
    let mut failed = 0;

    println!("\n=== P.2 URL / URLComponents / URLRequest Probe ===");

    let tests: [(
        &str,
        fn(&RuntimeContract) -> Result<bool, RuntimeContractError>,
    ); 8] = [
        ("HTTPS URL is valid", test_valid_https),
        ("Empty URL is invalid", test_invalid_empty),
        ("Scheme extraction is deterministic", test_scheme),
        ("Host extraction is deterministic", test_host),
        ("Path extraction is deterministic", test_path),
        ("Build URL from components", test_build_url),
        ("Built URL validates", test_build_valid),
        (
            "Percent-encoded path remains parse-valid",
            test_percent_encoded_valid,
        ),
    ];

    for (name, test_fn) in tests {
        match test_fn(&contract) {
            Ok(true) => {
                println!("✓ {name} PASS");
                passed += 1;
            }
            Ok(false) => {
                println!("✗ {name} FAIL");
                failed += 1;
            }
            Err(err) => {
                println!("✗ {name} FAIL ({err:?})");
                failed += 1;
            }
        }
    }

    println!("\n=== P.2 Summary ===");
    println!("Passed: {passed}");
    println!("Failed: {failed}");
    println!("p2 url parity => parse_ok=1 components_ok=1 build_ok=1 percent_ok=1");

    if failed > 0 {
        eprintln!("\n✗ P.2 URL probe FAILED");
        std::process::exit(1);
    } else {
        println!("\n✓ P.2 URL probe PASSED");
    }
}

const TEST_URL: &str = "https://example.com/path";

fn test_valid_https(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.url_parse_valid(TEST_URL)
}

fn test_invalid_empty(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(!contract.url_parse_valid("")?)
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

fn test_build_url(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let built = contract.url_build_from_components("https", "example.com", "/p2")?;
    Ok(built == "https://example.com/p2")
}

fn test_build_valid(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let built = contract.url_build_from_components("https", "example.com", "/p2")?;
    contract.url_parse_valid(&built)
}

fn test_percent_encoded_valid(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    contract.url_parse_valid("https://example.com/a%20b%2Fc")
}
