use std::path::PathBuf;

fn main() {
    let librs_path = PathBuf::from("src").join("lib.rs");

    println!("cargo:rerun-if-changed=swift");
    // println!("cargo:rustc-link-search=/path/to/lib");
    println!("cargo:rustc-link-lib=swiftCore");

    // enable-cxx11

    let bindings = bindgen::Builder::default()
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++14")
        .header("wrapper.h")
        .clang_arg("-Iswift/include")
        .clang_arg("-Iswift/stdlib/public/SwiftShims/")
        .opaque_type("SWIFTCC")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(librs_path)
        .expect("Couldn't write bindings!");
}