//! Inspect Swift types from Rust — metadata, VWT, descriptors.
//!
//! cargo run -p swift-runtime --example type_inspector

use swift_runtime::{metadata, types};

fn main() {
    println!("=== Swift Type Inspector ===\n");

    // Primitive types
    let primitives = [
        ("Int", types::int()),
        ("Bool", types::bool()),
        ("Double", types::double()),
        ("Float", types::float()),
        ("String", types::string()),
    ];

    println!("Primitive types:");
    for (name, meta) in &primitives {
        if let Some(m) = meta {
            println!(
                "  {name:8} kind={:?} size={} stride={} align={} pod={}",
                m.kind(),
                m.size(),
                m.stride(),
                m.alignment(),
                m.is_pod(),
            );
        }
    }

    // Type names
    println!("\nQualified names:");
    for (_, meta) in &primitives {
        if let Some(m) = meta {
            println!("  {}", m.type_name(true).unwrap_or("?".into()));
        }
    }

    // Descriptor names
    println!("\nDescriptor names:");
    for (_, meta) in &primitives {
        if let Some(m) = meta {
            println!("  {}", m.descriptor_name().unwrap_or("?".into()));
        }
    }

    // Generic types
    println!("\nGeneric types:");
    let int = types::int().unwrap();
    let str = types::string().unwrap();

    if let Some(opt) = types::optional(&int) {
        println!("  Optional<Int>: kind={:?} size={}", opt.kind(), opt.size());
    }
    if let Some(arr) = types::array(&int) {
        println!("  Array<Int>:    kind={:?} size={}", arr.kind(), arr.size());
    }
    if let Some(dict) = types::dictionary(&str, &int) {
        println!(
            "  Dict<Str,Int>: kind={:?} size={}",
            dict.kind(),
            dict.size()
        );
    }

    // Mangled name lookup
    println!("\nMangled name lookup:");
    if let Some(m) = metadata::lookup_type(b"Sf") {
        println!(
            "  'Sf' → {} (size={})",
            m.type_name(true).unwrap_or("?".into()),
            m.size()
        );
    }

    // Debug format
    println!("\nDebug format:");
    println!("  {:?}", types::int().unwrap());
    println!("  {:?}", types::string().unwrap());

    // Small string
    println!("\nSmall string:");
    if let Some(buf) = swift_runtime::string::create_small("hello") {
        if let Some(s) = swift_runtime::string::extract_small(&buf) {
            println!("  Created and extracted: '{s}'");
        }
    }
}
