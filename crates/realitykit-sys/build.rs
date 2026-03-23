fn main() {
    let os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let supported = ["macos", "ios", "xros"];
    if supported.contains(&os.as_str()) {
        // Framework is loaded at runtime via dlsym — no link-time dependency needed.
        println!("cargo:warning=RealityKit framework available");
    } else {
        println!("cargo:warning=RealityKit framework not available on {}", os);
    }
}
