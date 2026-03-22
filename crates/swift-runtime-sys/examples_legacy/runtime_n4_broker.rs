use std::env;
use std::fs;
use std::io::{self, Write};

use serde::{Deserialize, Serialize};
use swift_runtime_sys::RuntimeContract::RuntimeContract;
use swift_runtime_sys::RuntimeFactory::RuntimeFactory;

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

fn parse_request() -> BrokerRequest {
    let mut args = env::args().skip(1);
    let mut operation = String::new();
    let mut risk_level = "low".to_string();
    let mut allow_high_risk = false;
    let mut value = 0i32;
    let mut request_path: Option<String> = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--operation" => operation = args.next().unwrap_or_default(),
            "--risk" => risk_level = args.next().unwrap_or_else(|| "low".to_string()),
            "--allow-high" => allow_high_risk = true,
            "--value" => {
                value = args
                    .next()
                    .and_then(|v| v.parse::<i32>().ok())
                    .unwrap_or_default()
            }
            "--request" => request_path = args.next(),
            _ => {}
        }
    }

    if let Some(path) = request_path {
        let text = fs::read_to_string(path).expect("failed to read broker request");
        return serde_json::from_str(&text).expect("failed to parse broker request");
    }

    BrokerRequest {
        operation,
        risk_level,
        allow_high_risk,
        value,
    }
}

fn emit_stdout(event: &BrokerEvent) {
    println!("{}", serde_json::to_string(event).unwrap());
}

fn emit_stderr(event: &BrokerEvent) {
    eprintln!("{}", serde_json::to_string(event).unwrap());
    let _ = io::stderr().flush();
}

fn main() {
    let request = parse_request();
    let factory = RuntimeFactory::with_thunk_library("./libRustBridge.dylib", "./libRuntimeThunks.dylib")
        .or_else(|_| RuntimeFactory::new("./libRustBridge.dylib"))
        .unwrap_or_else(|e| panic!("failed to init RuntimeFactory: {e:?}"));
    factory
        .validate_runtime_contract(1)
        .unwrap_or_else(|e| panic!("runtime contract validation failed: {e:?}"));
    let contract = RuntimeContract::new(&factory);

    let policy_allows = match request.risk_level.as_str() {
        "low" | "medium" => true,
        "high" => request.allow_high_risk,
        _ => false,
    };

    if !policy_allows {
        emit_stdout(&BrokerEvent {
            operation: request.operation,
            risk_level: request.risk_level,
            allowed: false,
            event: "policy_denied".to_string(),
            value: request.value,
            result: None,
            signal: None,
            faulting_symbol: None,
            operation_context: "broker policy denied high-risk operation".to_string(),
            backtrace: None,
        });
        std::process::exit(3);
    }

    match request.operation.as_str() {
        "safe_ping" => {
            let result = contract.n4_safe_ping(request.value).unwrap_or(i32::MIN);
            emit_stdout(&BrokerEvent {
                operation: request.operation,
                risk_level: request.risk_level,
                allowed: true,
                event: "completed".to_string(),
                value: request.value,
                result: Some(result),
                signal: None,
                faulting_symbol: None,
                operation_context: "safe broker operation".to_string(),
                backtrace: None,
            });
        }
        "trigger_abort" => {
            let bt = contract
                .backtrace_capture()
                .unwrap_or_else(|_| "<backtrace unavailable>".to_string());
            emit_stderr(&BrokerEvent {
                operation: request.operation,
                risk_level: request.risk_level,
                allowed: true,
                event: "about_to_crash".to_string(),
                value: request.value,
                result: None,
                signal: Some(6),
                faulting_symbol: Some("swift_contract_n4_trigger_abort".to_string()),
                operation_context: "broker subprocess executing deliberate high-risk runtime abort"
                    .to_string(),
                backtrace: Some(bt),
            });
            let _ = contract.n4_trigger_abort();
            std::process::exit(99);
        }
        _ => {
            emit_stdout(&BrokerEvent {
                operation: request.operation,
                risk_level: request.risk_level,
                allowed: false,
                event: "unknown_operation".to_string(),
                value: request.value,
                result: None,
                signal: None,
                faulting_symbol: None,
                operation_context: "unknown broker operation".to_string(),
                backtrace: None,
            });
            std::process::exit(2);
        }
    }
}
