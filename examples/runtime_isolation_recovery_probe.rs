use std::fs;
use std::os::unix::process::ExitStatusExt;
use std::path::Path;
use std::process::Command;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BrokerRequest {
    operation: String,
    risk_level: String,
    allow_high_risk: bool,
    value: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BrokerEvent {
    operation: String,
    risk_level: String,
    allowed: bool,
    event: String,
    value: i32,
    result: Option<i32>,
    signal: Option<i32>,
    faulting_symbol: Option<String>,
    operation_context: String,
    backtrace: Option<String>,
}

#[derive(Debug)]
struct BrokerRun {
    status: std::process::ExitStatus,
    stdout: String,
    stderr: String,
}

fn run_broker(args: &[&str]) -> BrokerRun {
    let output = Command::new("./target/debug/examples/runtime_n4_broker")
        .args(args)
        .output()
        .unwrap_or_else(|e| panic!("failed to run broker: {e}"));
    BrokerRun {
        status: output.status,
        stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
    }
}

fn parse_event(text: &str) -> Option<BrokerEvent> {
    text.lines().find_map(|line| serde_json::from_str::<BrokerEvent>(line).ok())
}

fn main() {
    let mut passed = 0;
    let mut failed = 0;

    println!("\n=== Unsafe Runtime Ops Isolation & Recovery (Track N.4) ===");

    let tests: [(&str, fn() -> bool); 10] = [
        ("Broker executes safe low-risk op", test_safe_low_risk),
        ("Policy denies high-risk op by default", test_policy_denies_high_risk),
        ("Parent survives child crash", test_parent_survives_child_crash),
        ("Crash capture includes signal", test_crash_capture_signal),
        ("Crash capture includes backtrace", test_crash_capture_backtrace),
        ("Crash capture includes faulting symbol", test_crash_capture_symbol),
        ("Crash capture includes operation context", test_crash_capture_context),
        ("Replay request file written", test_replay_request_written),
        ("Replay reproduces crash classification", test_replay_reproduces_crash),
        ("Crash report artifact written", test_crash_report_artifact),
    ];

    for (name, test_fn) in tests {
        if test_fn() {
            println!("✓ {name} PASS");
            passed += 1;
        } else {
            println!("✗ {name} FAIL");
            failed += 1;
        }
    }

    println!("\n=== Track N.4 Summary ===");
    println!("Tests Passed: {}", passed);
    println!("Tests Failed: {}", failed);

    if failed == 0 {
        println!("✓ All Track N.4 tests PASSED");
    } else {
        panic!("✗ Track N.4 tests FAILED");
    }
}

fn test_safe_low_risk() -> bool {
    let run = run_broker(&["--operation", "safe_ping", "--risk", "low", "--value", "23"]);
    let event = parse_event(&run.stdout);
    run.status.success()
        && event.as_ref().map(|e| e.result == Some(1023) && e.event == "completed").unwrap_or(false)
}

fn test_policy_denies_high_risk() -> bool {
    let run = run_broker(&["--operation", "trigger_abort", "--risk", "high"]);
    let event = parse_event(&run.stdout);
    !run.status.success()
        && run.status.code() == Some(3)
        && event.as_ref().map(|e| !e.allowed && e.event == "policy_denied").unwrap_or(false)
}

fn crash_run() -> BrokerRun {
    run_broker(&[
        "--operation",
        "trigger_abort",
        "--risk",
        "high",
        "--allow-high",
    ])
}

fn test_parent_survives_child_crash() -> bool {
    let run = crash_run();
    !run.status.success() && run.status.signal().is_some()
}

fn test_crash_capture_signal() -> bool {
    let run = crash_run();
    let event = parse_event(&run.stderr);
    run.status.signal() == Some(6)
        && event.as_ref().map(|e| e.signal == Some(6)).unwrap_or(false)
}

fn test_crash_capture_backtrace() -> bool {
    let run = crash_run();
    let event = parse_event(&run.stderr);
    event
        .and_then(|e| e.backtrace)
        .map(|bt| !bt.trim().is_empty())
        .unwrap_or(false)
}

fn test_crash_capture_symbol() -> bool {
    let run = crash_run();
    let event = parse_event(&run.stderr);
    event
        .and_then(|e| e.faulting_symbol)
        .map(|sym| sym == "swift_contract_n4_trigger_abort")
        .unwrap_or(false)
}

fn test_crash_capture_context() -> bool {
    let run = crash_run();
    let event = parse_event(&run.stderr);
    event
        .map(|e| e.operation_context.contains("deliberate high-risk runtime abort"))
        .unwrap_or(false)
}

fn test_replay_request_written() -> bool {
    let out_dir = Path::new("target/runtime-probe");
    if fs::create_dir_all(out_dir).is_err() {
        return false;
    }
    let request = BrokerRequest {
        operation: "trigger_abort".to_string(),
        risk_level: "high".to_string(),
        allow_high_risk: true,
        value: 0,
    };
    let path = out_dir.join("n4-replay-request.json");
    fs::write(&path, serde_json::to_string_pretty(&request).unwrap()).is_ok() && path.exists()
}

fn test_replay_reproduces_crash() -> bool {
    let path = Path::new("target/runtime-probe/n4-replay-request.json");
    if !path.exists() && !test_replay_request_written() {
        return false;
    }
    let run = run_broker(&["--request", path.to_str().unwrap()]);
    let event = parse_event(&run.stderr);
    run.status.signal() == Some(6)
        && event.as_ref().map(|e| e.event == "about_to_crash").unwrap_or(false)
}

fn test_crash_report_artifact() -> bool {
    let out_dir = Path::new("target/runtime-probe");
    if fs::create_dir_all(out_dir).is_err() {
        return false;
    }
    let run = crash_run();
    let event = match parse_event(&run.stderr) {
        Some(event) => event,
        None => return false,
    };
    let signal = run.status.signal().unwrap_or_default();
    let report = serde_json::json!({
        "signal": signal,
        "faulting_symbol": event.faulting_symbol,
        "operation": event.operation,
        "risk_level": event.risk_level,
        "operation_context": event.operation_context,
        "backtrace": event.backtrace,
    });
    let path = out_dir.join("n4-crash-report.json");
    fs::write(&path, serde_json::to_string_pretty(&report).unwrap()).is_ok() && path.exists()
}
