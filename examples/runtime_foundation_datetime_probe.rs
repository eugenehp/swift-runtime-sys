/// Foundation Date/Time probe for Track I.1.
/// Tests ISO 8601 formatting, parsing, Calendar year/month extraction, and UTC offset.
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

    println!("\n=== Foundation Date/Time (Track I.1) ===");

    let tests: [(
        &str,
        fn(&RuntimeContract) -> Result<bool, RuntimeContractError>,
    ); 7] = [
        ("Format epoch contains '1970'", test_format_epoch_year),
        ("Format epoch contains 'T'", test_format_epoch_contains_t),
        ("Parse epoch ISO string → ≈0.0", test_parse_epoch),
        ("Year at epoch (UTC) = 1970", test_year_at_epoch),
        ("Month at epoch (UTC) = 1", test_month_at_epoch),
        ("Year at J2000 (946684800) = 2000", test_year_at_j2000),
        ("UTC offset = 0 seconds", test_utc_offset),
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

    println!("\n=== Track I.1 Summary ===");
    println!("Tests Passed: {}", passed);
    println!("Tests Failed: {}", failed);

    if failed == 0 {
        println!("✓ All Track I.1 tests PASSED");
    } else {
        panic!("✗ Track I.1 tests FAILED");
    }
}

fn test_format_epoch_year(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let s = contract.datetime_format_unix(0.0)?;
    Ok(s.contains("1970"))
}

fn test_format_epoch_contains_t(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let s = contract.datetime_format_unix(0.0)?;
    // ISO 8601 uses 'T' as separator between date and time
    Ok(s.contains('T'))
}

fn test_parse_epoch(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    // ISO 8601 representation of the Unix epoch
    let ts = contract.datetime_parse_iso8601("1970-01-01T00:00:00Z")?;
    // Allow ±1 second for any timezone edge
    Ok((ts - 0.0f64).abs() < 1.0)
}

fn test_year_at_epoch(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.datetime_year_utc(0.0)? == 1970)
}

fn test_month_at_epoch(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.datetime_month_utc(0.0)? == 1)
}

fn test_year_at_j2000(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    // 2000-01-01T00:00:00Z = 946684800 seconds since epoch
    Ok(contract.datetime_year_utc(946_684_800.0)? == 2000)
}

fn test_utc_offset(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.datetime_utc_offset_seconds()? == 0)
}
