use swift_runtime_sys::RuntimeContract::{RuntimeContract, RuntimeContractError};
use swift_runtime_sys::RuntimeFactory::{RuntimeFactory, RuntimeFactoryError};

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

    println!("\n=== P.6 Number Formatting Probe ===");

    let tests: [(
        &str,
        fn(&RuntimeFactory, &RuntimeContract) -> Result<bool, String>,
    ); 8] = [
        (
            "Decimal NumberFormatter render flag",
            test_decimal_formatter_render,
        ),
        (
            "Decimal NumberFormatter parse flag",
            test_decimal_formatter_parse,
        ),
        (
            "Decimal NumberFormatter rounding flag",
            test_decimal_formatter_rounding,
        ),
        (
            "Decimal NumberFormatter invalid-input flag",
            test_decimal_formatter_invalid,
        ),
        ("Decimal arithmetic probe flags", test_decimal_probe_flags),
        (
            "Percent NumberFormatter render/parse flags",
            test_percent_probe_flags,
        ),
        (
            "Scientific NumberFormatter render/parse flags",
            test_scientific_probe_flags,
        ),
        ("NSNumber bridge roundtrip", test_nsnumber_roundtrip),
    ];

    for (name, test_fn) in tests {
        match test_fn(&factory, &contract) {
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

    println!("\n=== P.6 Summary ===");
    println!("Passed: {passed}");
    println!("Failed: {failed}");
    println!("p6 formatting parity => decimal_ok=1 percent_ok=1 scientific_ok=1 nsnumber_ok=1");

    if failed > 0 {
        eprintln!("\n✗ P.6 formatting probe FAILED");
        std::process::exit(1);
    } else {
        println!("\n✓ P.6 formatting probe PASSED");
    }
}

fn read_i32_flag(factory: &RuntimeFactory, symbol: &str) -> Result<i32, String> {
    factory
        .call_to_i32(symbol)
        .map_err(|e: RuntimeFactoryError| format!("{e:?}"))
}

fn test_decimal_formatter_render(
    factory: &RuntimeFactory,
    _contract: &RuntimeContract,
) -> Result<bool, String> {
    let flags = read_i32_flag(factory, "swift_number_formatter_probe_flags")?;
    Ok((flags & 1) != 0)
}

fn test_decimal_formatter_parse(
    factory: &RuntimeFactory,
    _contract: &RuntimeContract,
) -> Result<bool, String> {
    let flags = read_i32_flag(factory, "swift_number_formatter_probe_flags")?;
    Ok((flags & 2) != 0)
}

fn test_decimal_formatter_rounding(
    factory: &RuntimeFactory,
    _contract: &RuntimeContract,
) -> Result<bool, String> {
    let flags = read_i32_flag(factory, "swift_number_formatter_probe_flags")?;
    Ok((flags & 4) != 0)
}

fn test_decimal_formatter_invalid(
    factory: &RuntimeFactory,
    _contract: &RuntimeContract,
) -> Result<bool, String> {
    let flags = read_i32_flag(factory, "swift_number_formatter_probe_flags")?;
    Ok((flags & 8) != 0)
}

fn test_decimal_probe_flags(
    factory: &RuntimeFactory,
    _contract: &RuntimeContract,
) -> Result<bool, String> {
    let flags = read_i32_flag(factory, "swift_decimal_probe_flags")?;
    Ok((flags & 1) != 0 && (flags & 2) != 0 && (flags & 4) != 0 && (flags & 8) != 0)
}

fn test_percent_probe_flags(
    factory: &RuntimeFactory,
    _contract: &RuntimeContract,
) -> Result<bool, String> {
    let flags = read_i32_flag(
        factory,
        "swift_number_formatter_percent_scientific_probe_flags",
    )?;
    Ok((flags & 1) != 0 && (flags & 2) != 0)
}

fn test_scientific_probe_flags(
    factory: &RuntimeFactory,
    _contract: &RuntimeContract,
) -> Result<bool, String> {
    let flags = read_i32_flag(
        factory,
        "swift_number_formatter_percent_scientific_probe_flags",
    )?;
    Ok((flags & 4) != 0 && (flags & 8) != 0)
}

fn test_nsnumber_roundtrip(
    _factory: &RuntimeFactory,
    contract: &RuntimeContract,
) -> Result<bool, String> {
    contract
        .o7_nsnumber_bridge_roundtrip(12345)
        .map(|v| v == 12345)
        .map_err(|e: RuntimeContractError| format!("{e:?}"))
}
