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

    // Determine deployment targets from features or auto-detect
    let detected = detect_macos_version();
    let macos_ver = option_env!("SWIFTUI_MACOS_VERSION").unwrap_or(if cfg!(feature = "macos-26") {
        "26.0"
    } else if cfg!(feature = "macos-15") {
        "15.0"
    } else {
        &detected
    });

    let target = std::env::var("TARGET").unwrap_or_default();
    let swift_target = if target.contains("aarch64-apple-darwin") || target.contains("arm64") {
        format!("arm64-apple-macosx{macos_ver}")
    } else if target.contains("x86_64-apple-darwin") {
        format!("x86_64-apple-macosx{macos_ver}")
    } else if target.contains("aarch64-apple-ios-sim") {
        let ios_ver = ios_version();
        format!("arm64-apple-ios{ios_ver}-simulator")
    } else if target.contains("aarch64-apple-ios") {
        let ios_ver = ios_version();
        format!("arm64-apple-ios{ios_ver}")
    } else {
        format!("arm64-apple-macosx{macos_ver}")
    };
    let swift_target = swift_target.as_str();

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

fn detect_macos_version() -> String {
    // Try to get SDK version
    Command::new("xcrun")
        .args(["--sdk", "macosx", "--show-sdk-version"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| {
            let v = s.trim().to_string();
            // Use major version only: "26.2" → "26.0"
            if let Some(dot) = v.find('.') {
                format!("{}.0", &v[..dot])
            } else {
                format!("{v}.0")
            }
        })
        .unwrap_or_else(|| "15.0".to_string())
}

fn ios_version() -> String {
    option_env!("SWIFTUI_IOS_VERSION")
        .map(|s| s.to_string())
        .unwrap_or_else(|| {
            if cfg!(feature = "ios-26") {
                "26.0".to_string()
            } else if cfg!(feature = "ios-18") {
                "18.0".to_string()
            } else {
                // Try to detect from SDK
                Command::new("xcrun")
                    .args(["--sdk", "iphoneos", "--show-sdk-version"])
                    .output()
                    .ok()
                    .and_then(|o| String::from_utf8(o.stdout).ok())
                    .map(|s| {
                        let v = s.trim().to_string();
                        if let Some(dot) = v.find('.') {
                            format!("{}.0", &v[..dot])
                        } else {
                            format!("{v}.0")
                        }
                    })
                    .unwrap_or_else(|| "18.0".to_string())
            }
        })
}
