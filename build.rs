use std::{fs, io::Write as _, path::PathBuf};

static SWIFT_RUNTIME: &str = "SWIFT_RUNTIME";

fn main() {
    let swift_runtime = match std::env::var(SWIFT_RUNTIME) {
        Ok(val) => val,
        Err(_) => {
            println!("Environment variable {} not found", SWIFT_RUNTIME);
            "/usr/lib/swift".to_string()
        }
    };

    println!("cargo:rerun-if-env-changed={}", SWIFT_RUNTIME);
    println!("cargo:rustc-link-search=native={}", swift_runtime);
    // println!("cargo:rerun-if-changed=swift");
    // println!("cargo:rustc-link-search=/usr/lib");
    println!("cargo:rustc-link-lib=dylib=swiftCore");

    let array = [
        // "swift/include/swift/Runtime/AccessibleFunction.h",
       "swift/include/swift/Runtime/Atomic.h",
        // "swift/include/swift/Runtime/AtomicWaitQueue.h",
        "swift/include/swift/Runtime/Backtrace.h",
        // "swift/include/swift/Runtime/Bincompat.h",
        // "swift/include/swift/Runtime/Casting.h",
        // "swift/include/swift/Runtime/Concurrency.h",
        // "swift/include/swift/Runtime/Concurrent.h",
       "swift/include/swift/Runtime/Config.h",
       "swift/include/swift/Runtime/CrashInfo.h",
        "swift/include/swift/Runtime/CustomRRABI.h",
        // "swift/include/swift/Runtime/Debug.h",
        // "swift/include/swift/Runtime/DispatchShims.h",
        // "swift/include/swift/Runtime/Enum.h",
        // "swift/include/swift/Runtime/EnvironmentVariables.h",
        // "swift/include/swift/Runtime/Error.h",
        "swift/include/swift/Runtime/Exception.h",
        "swift/include/swift/Runtime/Exclusivity.h",
        // "swift/include/swift/Runtime/ExistentialContainer.h",
        "swift/include/swift/Runtime/FoundationSupport.h",
        "swift/include/swift/Runtime/FunctionReplacement.h",
        // "swift/include/swift/Runtime/GenericMetadataBuilder.h",
      "swift/include/swift/Runtime/Heap.h",
//       "swift/include/swift/Runtime/HeapObject.h",
        "swift/include/swift/Runtime/InstrumentsSupport.h",
        // "swift/include/swift/Runtime/LibPrespecialized.h",
        // "swift/include/swift/Runtime/Metadata.h",
        // "swift/include/swift/Runtime/Numeric.h",
//        "swift/include/swift/Runtime/ObjCBridge.h",
        // "swift/include/swift/Runtime/Once.h",
//        "swift/include/swift/Runtime/Paths.h",
        // "swift/include/swift/Runtime/Portability.h",
        // "swift/include/swift/Runtime/PrebuiltStringMap.h",
        // "swift/include/swift/Runtime/Reflection.h",
        // "swift/include/swift/Runtime/RuntimeFnWrappersGen.h",
//        "swift/include/swift/Runtime/SwiftDtoa.h",
        // "swift/include/swift/Runtime/TracingCommon.h",
//        "swift/include/swift/Runtime/VoucherShims.h",
        // "swift/include/swift/Runtime/Win32.h",
    ];

    let lines = array
        .iter()
        .map(|header| {
            let filename = header
                .split('/')
                .last()
                .unwrap()
                .split(".")
                .collect::<Vec<&str>>();
            let filename = filename.first().unwrap();
            let line = format!("pub mod {filename};");
            let filename = format!("{filename}.rs");
            let filename = filename.as_str();
            build(header, filename);

            line
        })
        .collect::<Vec<String>>();
    // build();

    let librs_path = PathBuf::from("src").join("lib.rs");
    let code = lines.join("\n");
    let mut file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(librs_path)
        .unwrap();
    file.write_all(code.as_bytes()).unwrap();
}

fn build(header: &str, filename: &str) {
    let librs_path = PathBuf::from("src").join(filename);

    if librs_path.exists() {
        fs::remove_file(&librs_path).unwrap();
    }

    let bindings = bindgen::Builder::default()
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++14")
        // .header("wrapper.h")
        .header(header)
        .raw_line("#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals, improper_ctypes)]")
        .blocklist_item("template")
        .blocklist_item("char_type")
        // .blocklist_item("_Pred")
        // .blocklist_item("_Tp")
        // .blocklist_item("SideTableRefCountBits")
        // .blocklist_type("_Tp")
        .blocklist_type("template")
        // .blocklist_item("std_*")
        // .blocklist_item("std_char_*")
        .blocklist_item("char_type")
        .opaque_type("sizeof")
        .opaque_type("_Tp")
        .opaque_type("rep")
        .opaque_type("std::atomic")
        .opaque_type("std::.+")
        .opaque_type("char_type")
        // .enable_cxx_namespaces()
        // prevents: Unable to generate bindings: ClangDiagnostic("swift/include/swift/Runtime/Config.h:21:10: fatal error: 'swift/Runtime/CMakeConfig.h' file not found\n")
        .clang_arg("-Ifake")
        .clang_arg("-Iswift/include")
        .clang_arg("-Iswift/stdlib/public/SwiftShims/")
        .raw_line("pub type _Tp = ();")
        // .raw_line("pub mod std { pub mod __1 { pub type _Tp = (); } }")
        // .raw_line("pub type _Pred = ();")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    // bindings
    //     .write_to_file(librs_path)
    //     .expect("Couldn't write bindings!");

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
    let mut file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(librs_path)
        .unwrap();
    file.write_all(code.as_bytes()).unwrap();
}
