use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

// ═══════════════════════════════════════════════════════════════════════════
// Configuration
// ═══════════════════════════════════════════════════════════════════════════

const ENV_SWIFT_RUNTIME: &str = "SWIFT_RUNTIME";
const ENV_GENERATE_BINDINGS: &str = "SWIFT_RUNTIME_SYS_GENERATE_BINDINGS";

/// Headers to generate bindgen bindings from (relative to crate root).
const RUNTIME_HEADERS: &[&str] = &[
    "../../swift/include/swift/Runtime/Atomic.h",
    "../../swift/include/swift/Runtime/Backtrace.h",
    "../../swift/include/swift/Runtime/Config.h",
    "../../swift/include/swift/Runtime/CrashInfo.h",
    "../../swift/include/swift/Runtime/CustomRRABI.h",
    "../../swift/include/swift/Runtime/Exception.h",
    "../../swift/include/swift/Runtime/Exclusivity.h",
    "../../swift/include/swift/Runtime/FoundationSupport.h",
    "../../swift/include/swift/Runtime/FunctionReplacement.h",
    "../../swift/include/swift/Runtime/Heap.h",
    "../../swift/include/swift/Runtime/InstrumentsSupport.h",
    "../../swift/include/swift/Runtime/Paths.h",
    "../../swift/include/swift/Runtime/Portability.h",
    "../../swift/include/swift/Runtime/PrebuiltStringMap.h",
    "../../swift/include/swift/Runtime/SwiftDtoa.h",
    "../../swift/include/swift/Runtime/TracingCommon.h",
    "../../swift/include/swift/Runtime/VoucherShims.h",
    #[cfg(target_os = "windows")]
    "../../swift/include/swift/Runtime/Win32.h",
];

// ═══════════════════════════════════════════════════════════════════════════
// Platform detection
// ═══════════════════════════════════════════════════════════════════════════

fn target_os() -> String {
    std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_else(|_| "macos".into())
}

fn is_simulator() -> bool {
    std::env::var("CARGO_CFG_TARGET_ABI")
        .map(|v| v == "sim")
        .unwrap_or(false)
}

/// Return the `xcrun --sdk <name>` SDK name for the current target.
fn sdk_name() -> &'static str {
    match (target_os().as_str(), is_simulator()) {
        ("macos", _) => "macosx",
        ("ios", false) => "iphoneos",
        ("ios", true) => "iphonesimulator",
        ("tvos", false) => "appletvos",
        ("tvos", true) => "appletvsimulator",
        ("xros", false) => "xros",
        ("xros", true) => "xrsimulator",
        ("watchos", false) => "watchos",
        ("watchos", true) => "watchsimulator",
        _ => "macosx",
    }
}

fn sdk_path() -> Option<String> {
    let out = Command::new("xcrun")
        .args(["--sdk", sdk_name(), "--show-sdk-path"])
        .output()
        .ok()?;
    out.status
        .success()
        .then(|| String::from_utf8_lossy(&out.stdout).trim().to_string())
}

// ═══════════════════════════════════════════════════════════════════════════
// Swift library resolution
// ═══════════════════════════════════════════════════════════════════════════

fn swift_lib_path() -> String {
    // 1. Explicit override
    if let Ok(val) = std::env::var(ENV_SWIFT_RUNTIME) {
        return val;
    }

    let os = target_os();

    // 2. macOS system default
    if os == "macos" {
        return "/usr/lib/swift".into();
    }

    // 3. Inside SDK (iOS / tvOS / visionOS / watchOS)
    if let Some(sdk) = sdk_path() {
        let path = format!("{sdk}/usr/lib/swift");
        if Path::new(&path).exists() {
            return path;
        }
    }

    // 4. Toolchain fallback
    if let Some(path) = toolchain_swift_lib() {
        return path;
    }

    "/usr/lib/swift".into()
}

fn toolchain_swift_lib() -> Option<String> {
    let out = Command::new("xcrun")
        .args(["--toolchain", "default", "--find", "swift"])
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let swift_bin = String::from_utf8_lossy(&out.stdout).trim().to_string();
    let lib_dir = PathBuf::from(&swift_bin)
        .parent()? // bin/
        .parent()? // toolchain root
        .join("lib/swift")
        .join(sdk_name());
    lib_dir
        .exists()
        .then(|| lib_dir.to_string_lossy().into_owned())
}

// ═══════════════════════════════════════════════════════════════════════════
// Main
// ═══════════════════════════════════════════════════════════════════════════

fn main() {
    let os = target_os();
    let swift_lib = swift_lib_path();

    // Re-run triggers
    println!("cargo:rerun-if-env-changed={ENV_SWIFT_RUNTIME}");
    println!("cargo:rerun-if-env-changed={ENV_GENERATE_BINDINGS}");

    // Link search path & Swift core libs
    println!("cargo:rustc-link-search=native={swift_lib}");
    println!("cargo:rustc-link-lib=dylib=swiftCore");
    println!("cargo:rustc-link-lib=dylib=swift_Concurrency");
    println!("cargo:rustc-link-arg=-Wl,-rpath,{swift_lib}");

    if os == "macos" {
        println!("cargo:rustc-link-arg=-Wl,-rpath,/usr/lib/swift");
    }

    // UI framework
    match os.as_str() {
        "macos" => println!("cargo:rustc-link-lib=framework=AppKit"),
        "ios" | "tvos" | "xros" => println!("cargo:rustc-link-lib=framework=UIKit"),
        _ => {}
    }

    // Platform cfg
    println!("cargo:rustc-cfg=swift_platform=\"{os}\"");
    if is_simulator() {
        println!("cargo:rustc-cfg=swift_simulator");
    }

    // Optional: regenerate bindings from Swift headers
    if std::env::var_os(ENV_GENERATE_BINDINGS).is_some() {
        generate_all_bindings();
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Bindgen code generation (only when SWIFT_RUNTIME_SYS_GENERATE_BINDINGS is set)
// ═══════════════════════════════════════════════════════════════════════════

#[allow(dead_code)]
fn generate_all_bindings() {
    let mut mod_lines = Vec::new();

    for header in RUNTIME_HEADERS {
        let stem = Path::new(header)
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap();

        generate_bindings_for(header, &format!("{stem}.rs"));
        mod_lines.push(format!("pub mod {stem};"));
    }

    let lib_rs = PathBuf::from("src/lib.rs");
    fs::write(&lib_rs, mod_lines.join("\n")).expect("failed to write lib.rs");
}

#[allow(dead_code)]
fn generate_bindings_for(header: &str, out_filename: &str) {
    let out_path = PathBuf::from("src").join(out_filename);
    let _ = fs::remove_file(&out_path);

    let bindings = bindgen::Builder::default()
        .clang_args(["-x", "c++", "-std=c++17"])
        .header(header)
        .raw_line(
            "#![allow(dead_code, non_snake_case, non_camel_case_types, \
             non_upper_case_globals, improper_ctypes)]",
        )
        .blocklist_item("template")
        .blocklist_item("char_type")
        .opaque_type("sizeof")
        .opaque_type("_Tp")
        .opaque_type("rep")
        .opaque_type("std::atomic")
        .opaque_type("std::.+")
        .opaque_type("char_type")
        .clang_args([
            "-I../../fake",
            "-I../../swift/include",
            "-I../../swift/stdlib/public/SwiftShims/",
        ])
        .raw_line("pub type _Tp = ();")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("bindgen failed");

    let code = bindings
        .to_string()
        .replace("extern \"swift\" {", "extern \"C\" {")
        .replace(
            "pub type rep = u64;\npub type rep = u64;",
            "pub type rep = u64;",
        )
        .replace(
            "    pub type rep = u64;\n    pub type rep = u64;",
            "pub type rep = u64;",
        );

    fs::write(&out_path, code).expect("failed to write bindings");
}
