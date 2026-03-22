fn main() {
    println!("cargo:rustc-link-arg=-Wl,-rpath,/usr/lib/swift");
    if check_framework("AppIntents") {
        println!("cargo:rustc-cfg=has_framework");
        println!("cargo:warning=AppIntents framework available");
    } else {
        println!("cargo:warning=AppIntents framework NOT available — functions will be no-ops");
    }
}

fn check_framework(name: &str) -> bool {
    let code = format!("import {name}");
    std::process::Command::new("xcrun")
        .args([
            "swiftc",
            "-typecheck",
            "-target",
            "arm64-apple-macosx15.0",
            "-sdk",
            &sdk_path(),
            "-",
        ])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .and_then(|mut c| {
            use std::io::Write;
            c.stdin.as_mut().unwrap().write_all(code.as_bytes()).ok();
            drop(c.stdin.take());
            c.wait()
        })
        .map(|s| s.success())
        .unwrap_or(false)
}

fn sdk_path() -> String {
    std::process::Command::new("xcrun")
        .args(["--sdk", "macosx", "--show-sdk-path"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_default()
}
