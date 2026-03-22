use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    println!("cargo:rustc-link-arg=-Wl,-rpath,/usr/lib/swift");

    // Auto-compile the Swift helper if source is newer than dylib
    let helper_dir = find_helper_dir();
    if let Some(dir) = helper_dir {
        let dylib = dir.join("libSwiftUIHelper.dylib");
        let sources = [
            "SwiftUIHelper.swift",
            "SnapshotHelper.swift",
            "Platform.swift",
            "AppHost.swift",
            "RealityKitHelper.swift",
        ];

        // Tell cargo to rerun if any Swift source changes
        for src in &sources {
            let path = dir.join(src);
            if path.exists() {
                println!("cargo:rerun-if-changed={}", path.display());
            }
        }

        // Check if rebuild is needed
        let needs_build = if dylib.exists() {
            let dylib_modified = std::fs::metadata(&dylib).and_then(|m| m.modified()).ok();
            sources.iter().any(|src| {
                let src_path = dir.join(src);
                if !src_path.exists() {
                    return false;
                }
                let src_modified = std::fs::metadata(&src_path).and_then(|m| m.modified()).ok();
                match (src_modified, dylib_modified) {
                    (Some(s), Some(d)) => s > d,
                    _ => true,
                }
            })
        } else {
            true
        };

        if needs_build {
            compile_helper(&dir, &sources);
        }

        // Tell downstream crates where to find the dylib
        println!("cargo:rustc-env=SWIFTUI_HELPER={}", dylib.display());
    }
}

fn find_helper_dir() -> Option<PathBuf> {
    // Search relative to the manifest directory (workspace root)
    let manifest = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());

    let candidates = [
        manifest.join("../../swift_helper"), // from crates/swiftui/
        manifest.join("swift_helper"),
        PathBuf::from("swift_helper"),
    ];

    for c in &candidates {
        if c.join("SwiftUIHelper.swift").exists() {
            return Some(c.clone());
        }
    }
    None
}

fn compile_helper(dir: &Path, sources: &[&str]) {
    // Find SDK
    let sdk = Command::new("xcrun")
        .args(["--sdk", "macosx", "--show-sdk-path"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string());

    let Some(sdk) = sdk else {
        println!("cargo:warning=No macOS SDK found, skipping Swift helper compilation");
        return;
    };

    // Determine target triple
    let target = std::env::var("TARGET").unwrap_or_else(|_| "arm64-apple-macosx15.0".into());
    let swift_target = if target.contains("aarch64-apple-darwin") || target.contains("arm64") {
        "arm64-apple-macosx15.0"
    } else if target.contains("x86_64-apple-darwin") {
        "x86_64-apple-macosx15.0"
    } else {
        "arm64-apple-macosx15.0"
    };

    // Collect source files that exist
    let source_paths: Vec<PathBuf> = sources
        .iter()
        .map(|s| dir.join(s))
        .filter(|p| p.exists())
        .collect();

    if source_paths.is_empty() {
        println!(
            "cargo:warning=No Swift source files found in {}",
            dir.display()
        );
        return;
    }

    let output = dir.join("libSwiftUIHelper.dylib");

    println!(
        "cargo:warning=Compiling Swift helper ({} sources) → {}",
        source_paths.len(),
        output.display()
    );

    let mut cmd = Command::new("xcrun");
    cmd.arg("swiftc")
        .arg("-emit-library")
        .args(source_paths.iter().map(|p| p.as_os_str()))
        .arg("-o")
        .arg(&output)
        .arg("-target")
        .arg(swift_target)
        .arg("-sdk")
        .arg(&sdk);

    let result = cmd.output();
    match result {
        Ok(output) if output.status.success() => {
            println!("cargo:warning=Swift helper compiled successfully");
        }
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            println!("cargo:warning=Swift compilation failed: {stderr}");
        }
        Err(e) => {
            println!("cargo:warning=Failed to run swiftc: {e}");
        }
    }
}
