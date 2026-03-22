/// Backtrace and crash-symbolication probe for Track E.2.
/// Tests: Swift stack capture, runtime demangling, DWARF access, and artifact generation.
use std::fs;
use std::path::Path;
use std::process::Command;

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

    println!("\n=== Backtrace Capture & Symbolication (Track E.2) ===");

    match test_backtrace_capture_non_empty(&contract) {
        Ok(true) => {
            println!("✓ Backtrace capture non-empty PASS");
            passed += 1;
        }
        _ => {
            println!("✗ Backtrace capture non-empty FAIL");
            failed += 1;
        }
    }

    match test_backtrace_contains_probe_frames(&contract) {
        Ok(true) => {
            println!("✓ Backtrace contains probe frame markers PASS");
            passed += 1;
        }
        _ => {
            println!("✗ Backtrace contains probe frame markers FAIL");
            failed += 1;
        }
    }

    match test_runtime_demangle_known_symbol(&contract) {
        Ok(true) => {
            println!("✓ Runtime demangle known symbol PASS");
            passed += 1;
        }
        _ => {
            println!("✗ Runtime demangle known symbol FAIL");
            failed += 1;
        }
    }

    match test_anchor_runtime_address(&contract) {
        Ok(true) => {
            println!("✓ Anchor runtime address PASS");
            passed += 1;
        }
        _ => {
            println!("✗ Anchor runtime address FAIL");
            failed += 1;
        }
    }

    match test_dwarf_uuid_access() {
        Ok(true) => {
            println!("✓ DWARF UUID access PASS");
            passed += 1;
        }
        _ => {
            println!("✗ DWARF UUID access FAIL");
            failed += 1;
        }
    }

    match test_atos_symbolication() {
        Ok(true) => {
            println!("✓ atos symbolication PASS");
            passed += 1;
        }
        _ => {
            println!("✗ atos symbolication FAIL");
            failed += 1;
        }
    }

    match test_crash_report_artifact(&contract) {
        Ok(true) => {
            println!("✓ Crash report artifact generation PASS");
            passed += 1;
        }
        _ => {
            println!("✗ Crash report artifact generation FAIL");
            failed += 1;
        }
    }

    println!("\n=== Track E.2 Summary ===");
    println!("Tests Passed: {}", passed);
    println!("Tests Failed: {}", failed);

    if failed == 0 {
        println!("✓ All Track E.2 tests PASSED");
    } else {
        panic!("✗ Track E.2 tests FAILED");
    }
}

fn test_backtrace_capture_non_empty(
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    let bt = contract.backtrace_capture()?;
    Ok(!bt.trim().is_empty())
}

fn test_backtrace_contains_probe_frames(
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    let bt = contract.backtrace_capture()?;
    Ok(bt.contains("_swift_contract_backtrace_frame")
        || bt.contains("swift_contract_backtrace_capture"))
}

fn test_runtime_demangle_known_symbol(
    contract: &RuntimeContract,
) -> Result<bool, RuntimeContractError> {
    let mangled = "_$s10RustBridge23swift_contract_any_wrapySvSgs5Int32V_ACtF";
    let demangled = contract.backtrace_demangle_symbol(mangled)?;
    Ok(demangled.contains("swift_contract_any_wrap") || demangled.contains("any_wrap"))
}

fn test_anchor_runtime_address(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let addr = contract.backtrace_anchor_address()?;
    Ok(addr > 0)
}

fn test_dwarf_uuid_access() -> Result<bool, RuntimeContractError> {
    let output = Command::new("xcrun")
        .args(["dwarfdump", "--uuid", "./libRustBridge.dylib"])
        .output()
        .map_err(|_| RuntimeContractError::InvalidInvoke {
            type_id: 17,
            method_id: 4,
        })?;

    if !output.status.success() {
        return Ok(false);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.contains("UUID:"))
}

fn test_atos_symbolication() -> Result<bool, RuntimeContractError> {
    let nm_output = Command::new("nm")
        .args(["-arch", "arm64", "-gU", "./libRustBridge.dylib"])
        .output()
        .map_err(|_| RuntimeContractError::InvalidInvoke {
            type_id: 17,
            method_id: 5,
        })?;

    if !nm_output.status.success() {
        return Ok(false);
    }

    let stdout = String::from_utf8_lossy(&nm_output.stdout);
    let mut target_addr = None;
    for line in stdout.lines() {
        if line.contains("_swift_contract_backtrace_anchor") {
            let mut parts = line.split_whitespace();
            if let Some(addr) = parts.next() {
                target_addr = Some(format!("0x{}", addr));
                break;
            }
        }
    }

    let Some(addr) = target_addr else {
        return Ok(false);
    };

    let atos_output = Command::new("xcrun")
        .args([
            "atos",
            "-o",
            "./libRustBridge.dylib",
            "-arch",
            "arm64",
            &addr,
        ])
        .output()
        .map_err(|_| RuntimeContractError::InvalidInvoke {
            type_id: 17,
            method_id: 5,
        })?;

    if !atos_output.status.success() {
        return Ok(false);
    }

    let atos_stdout = String::from_utf8_lossy(&atos_output.stdout);
    Ok(!atos_stdout.trim().is_empty() && !atos_stdout.contains("??"))
}

fn test_crash_report_artifact(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let out_dir = Path::new("target/runtime-probe");
    fs::create_dir_all(out_dir).map_err(|_| RuntimeContractError::InvalidInvoke {
        type_id: 17,
        method_id: 6,
    })?;

    let bt = contract.backtrace_capture()?;
    let sample_symbol = "_$s10RustBridge23swift_contract_any_wrapySvSgs5Int32V_ACtF";
    let demangled = contract.backtrace_demangle_symbol(sample_symbol)?;

    let dwarf_uuid = Command::new("xcrun")
        .args(["dwarfdump", "--uuid", "./libRustBridge.dylib"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .unwrap_or_else(|| "<dwarfdump unavailable>".to_string());

    let report = format!(
        "# Crash Symbolication Report\n\n## Swift Backtrace Capture\n\n```text\n{}\n```\n\n## Runtime Demangle Sample\n\n- mangled: {}\n- demangled: {}\n\n## DWARF UUID\n\n```text\n{}\n```\n",
        bt,
        sample_symbol,
        demangled,
        dwarf_uuid.trim()
    );

    let report_path = out_dir.join("crash-symbolication-report.md");
    fs::write(&report_path, report).map_err(|_| RuntimeContractError::InvalidInvoke {
        type_id: 17,
        method_id: 6,
    })?;

    Ok(report_path.exists())
}
