//! Swift Bridge Generator
//!
//! Reads a Swift API dump (from `swift-api-digester -dump-sdk`)
//! and generates:
//!   1. A Swift helper file with `@_cdecl` wrappers
//!   2. A Rust FFI module with matching `extern "C"` declarations
//!
//! Usage:
//!   xcrun swift-api-digester -dump-sdk -module Foundation \
//!     -target arm64-apple-macosx15.0 -sdk $(xcrun -sdk macosx --show-sdk-path) \
//!     -o foundation_api.json
//!
//!   swift-bridge-gen foundation_api.json --output-dir generated/

mod parser;
mod rust_gen;
mod swift_gen;

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "swift-bridge-gen")]
#[command(about = "Generate Rust↔Swift bridge code from Swift API dumps")]
struct Cli {
    /// Path to the API dump JSON (from swift-api-digester -dump-sdk)
    input: PathBuf,

    /// Output directory for generated files
    #[arg(short, long, default_value = "generated")]
    output_dir: PathBuf,

    /// Module name (auto-detected from JSON if not specified)
    #[arg(short, long)]
    module: Option<String>,

    /// Only generate for these types (comma-separated)
    #[arg(short, long)]
    types: Option<String>,

    /// Skip types matching these prefixes
    #[arg(long)]
    skip_prefix: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    let json_str = std::fs::read_to_string(&cli.input)
        .unwrap_or_else(|e| panic!("Failed to read {}: {e}", cli.input.display()));

    let api = parser::parse_api_dump(&json_str)
        .unwrap_or_else(|e| panic!("Failed to parse API dump: {e}"));

    let module_name = cli.module.as_deref().unwrap_or(&api.module_name);
    println!("Module: {module_name}");
    println!("Types: {}", api.types.len());
    println!(
        "  Structs: {}, Classes: {}, Enums: {}",
        api.types
            .iter()
            .filter(|t| t.kind == parser::TypeKind::Struct)
            .count(),
        api.types
            .iter()
            .filter(|t| t.kind == parser::TypeKind::Class)
            .count(),
        api.types
            .iter()
            .filter(|t| t.kind == parser::TypeKind::Enum)
            .count(),
    );

    let type_filter: Option<Vec<&str>> = cli.types.as_deref().map(|s| s.split(',').collect());
    let skip_prefix: Vec<&str> = cli
        .skip_prefix
        .as_deref()
        .map(|s| s.split(',').collect())
        .unwrap_or_default();

    let filtered: Vec<&parser::TypeInfo> = api
        .types
        .iter()
        .filter(|t| {
            if let Some(ref filter) = type_filter {
                if !filter.contains(&t.name.as_str()) {
                    return false;
                }
            }
            for pfx in &skip_prefix {
                if t.name.starts_with(pfx) {
                    return false;
                }
            }
            true
        })
        .collect();

    println!("Generating for {} types", filtered.len());

    std::fs::create_dir_all(&cli.output_dir).unwrap();

    let swift_code = swift_gen::generate_swift(module_name, &filtered);
    let rust_code = rust_gen::generate_rust(module_name, &filtered);

    let swift_path = cli.output_dir.join(format!("{module_name}Bridge.swift"));
    let rust_path = cli
        .output_dir
        .join(format!("{}.rs", module_name.to_lowercase()));

    std::fs::write(&swift_path, &swift_code).unwrap();
    std::fs::write(&rust_path, &rust_code).unwrap();

    println!("Generated:");
    println!(
        "  Swift: {} ({} bytes)",
        swift_path.display(),
        swift_code.len()
    );
    println!(
        "  Rust:  {} ({} bytes)",
        rust_path.display(),
        rust_code.len()
    );
}
