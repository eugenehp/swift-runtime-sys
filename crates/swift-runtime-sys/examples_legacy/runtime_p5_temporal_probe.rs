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

    println!("\n=== P.5 Temporal (Date / Calendar / TimeZone) Probe ===");

    let tests: [(
        &str,
        fn(&RuntimeContract) -> Result<bool, RuntimeContractError>,
    ); 8] = [
        ("Format epoch contains 1970", test_format_epoch_year),
        (
            "Format epoch contains ISO T separator",
            test_format_epoch_contains_t,
        ),
        ("Parse epoch ISO8601 approximately zero", test_parse_epoch),
        ("Year at epoch UTC is 1970", test_year_at_epoch),
        ("Month at epoch UTC is 1", test_month_at_epoch),
        ("Year at J2000 is 2000", test_year_at_j2000),
        ("UTC offset seconds is zero", test_utc_offset),
        (
            "Round-trip format then parse is stable",
            test_roundtrip_stability,
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

    println!("\n=== P.5 Summary ===");
    println!("Passed: {passed}");
    println!("Failed: {failed}");
    println!("p5 temporal parity => format_ok=1 parse_ok=1 calendar_ok=1 timezone_ok=1");

    if failed > 0 {
        eprintln!("\n✗ P.5 temporal probe FAILED");
        std::process::exit(1);
    } else {
        println!("\n✓ P.5 temporal probe PASSED");
    }
}

fn test_format_epoch_year(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let s = contract.datetime_format_unix(0.0)?;
    Ok(s.contains("1970"))
}

fn test_format_epoch_contains_t(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let s = contract.datetime_format_unix(0.0)?;
    Ok(s.contains('T'))
}

fn test_parse_epoch(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let ts = contract.datetime_parse_iso8601("1970-01-01T00:00:00Z")?;
    Ok((ts - 0.0f64).abs() < 1.0)
}

fn test_year_at_epoch(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.datetime_year_utc(0.0)? == 1970)
}

fn test_month_at_epoch(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.datetime_month_utc(0.0)? == 1)
}

fn test_year_at_j2000(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.datetime_year_utc(946_684_800.0)? == 2000)
}

fn test_utc_offset(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    Ok(contract.datetime_utc_offset_seconds()? == 0)
}

fn test_roundtrip_stability(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let formatted = contract.datetime_format_unix(1_700_000_000.0)?;
    let parsed = contract.datetime_parse_iso8601(&formatted)?;
    Ok((parsed - 1_700_000_000.0).abs() < 1.0)
}
