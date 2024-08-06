use std::{fs, path::PathBuf};

fn main() {
    let librs_path = PathBuf::from("src").join("lib.rs");

    println!("cargo:rerun-if-changed=swift");
    // println!("cargo:rustc-link-search=/path/to/lib");
    println!("cargo:rustc-link-lib=swiftCore");

    // enable-cxx11

    if librs_path.exists() {
        fs::remove_file(&librs_path).unwrap();
    }

    let bindings = bindgen::Builder::default()
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++14")
        .header("wrapper.h")
        .blocklist_item("template")
        .blocklist_item("_Pred")
        .blocklist_item("_Tp")
        .blocklist_type("_Tp")
        // .blocklist_item("std_*")
        // .blocklist_item("std_char_*")
        // .opaque_type("sizeof")
        .opaque_type("std::.+")
        .enable_cxx_namespaces()
        // prevents: Unable to generate bindings: ClangDiagnostic("swift/include/swift/Runtime/Config.h:21:10: fatal error: 'swift/Runtime/CMakeConfig.h' file not found\n")
        .clang_arg("-Ifake")
        .clang_arg("-Iswift/include")
        .clang_arg("-Iswift/stdlib/public/SwiftShims/")
        .raw_line("#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
")
        .raw_line("pub type _Tp = ();")
        .raw_line("pub type _Pred = ();")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(librs_path)
        .expect("Couldn't write bindings!");
}