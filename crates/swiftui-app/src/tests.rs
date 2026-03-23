//! Tests for the swiftui-app crate.

// ── Codegen tests (verify generated Swift is valid) ─────────────────────────

#[test]
fn codegen_single_window_produces_valid_swift() {
    let swift = crate::codegen_for_test("TestApp", &[("main", true)], &[], &[]);
    assert!(swift.contains("import SwiftUI"));
    assert!(swift.contains("struct TestApp: App"));
    assert!(swift.contains("WindowGroup {"));
    assert!(swift.contains("RustMetalView_main()"));
    assert!(swift.contains("rustOnInit?()"));
    assert!(swift.contains("swiftui_app_launch"));
    assert!(swift.contains("swiftui_app_set_on_init"));
    assert!(swift.contains("swiftui_app_set_on_frame_main"));
    // Should have AppKit and UIKit variants
    assert!(swift.contains("#if canImport(AppKit)"));
    assert!(swift.contains("#elseif canImport(UIKit)"));
    assert!(swift.contains("NSViewRepresentable"));
    assert!(swift.contains("UIViewRepresentable"));
    assert!(swift.contains("MTKViewDelegate"));
}

#[test]
fn codegen_multi_window() {
    let swift = crate::codegen_for_test(
        "MultiApp",
        &[("main", true), ("settings", true), ("preview", false)],
        &[],
        &[],
    );
    assert!(swift.contains("struct MultiApp: App"));
    // First window has no id
    assert!(swift.contains("WindowGroup {"));
    // Subsequent windows have ids
    assert!(swift.contains(r#"WindowGroup(id: "settings")"#));
    assert!(swift.contains(r#"WindowGroup(id: "preview")"#));
    // Metal views for metal windows
    assert!(swift.contains("RustMetalView_main"));
    assert!(swift.contains("RustMetalView_settings"));
    // Non-metal window has no view
    assert!(!swift.contains("RustMetalView_preview"));
    // Callbacks for metal windows
    assert!(swift.contains("swiftui_app_set_on_frame_main"));
    assert!(swift.contains("swiftui_app_set_on_frame_settings"));
    // No callback for non-metal window
    assert!(!swift.contains("swiftui_app_set_on_frame_preview"));
}

#[test]
fn codegen_immersive_space() {
    let swift = crate::codegen_for_test(
        "VisionApp",
        &[("main", true)],
        &["world"],
        &[],
    );
    assert!(swift.contains("#if os(visionOS)"));
    assert!(swift.contains("import RealityKit"));
    assert!(swift.contains(r#"ImmersiveSpace(id: "world")"#));
    assert!(swift.contains("RealityView"));
    assert!(swift.contains("AnchorEntity(.head)"));
}

#[test]
fn codegen_volume() {
    let swift = crate::codegen_for_test(
        "VolApp",
        &[("main", true)],
        &[],
        &[("preview", 0.5, 0.5, 0.5)],
    );
    assert!(swift.contains("#if os(visionOS)"));
    assert!(swift.contains(r#"WindowGroup(id: "preview")"#));
    assert!(swift.contains(".windowStyle(.volumetric)"));
    assert!(swift.contains("width: 0.5, height: 0.5, depth: 0.5"));
}

#[test]
fn codegen_full_visionos_app() {
    let swift = crate::codegen_for_test(
        "FullVisionApp",
        &[("main", true), ("inspector", true)],
        &["immersive"],
        &[("model", 1.0, 1.0, 1.0)],
    );
    // Should have everything
    assert!(swift.contains("struct FullVisionApp: App"));
    assert!(swift.contains("WindowGroup {"));
    assert!(swift.contains(r#"WindowGroup(id: "inspector")"#));
    assert!(swift.contains(r#"ImmersiveSpace(id: "immersive")"#));
    assert!(swift.contains(r#"WindowGroup(id: "model")"#));
    assert!(swift.contains(".windowStyle(.volumetric)"));
    // Metal views
    assert!(swift.contains("RustMetalView_main"));
    assert!(swift.contains("RustMetalView_inspector"));
    assert!(swift.contains("Coord_main"));
    assert!(swift.contains("Coord_inspector"));
}

#[test]
fn codegen_empty_app_still_valid() {
    let swift = crate::codegen_for_test("EmptyApp", &[], &[], &[]);
    assert!(swift.contains("import SwiftUI"));
    assert!(swift.contains("struct EmptyApp: App"));
    assert!(swift.contains("swiftui_app_launch"));
}

#[test]
fn codegen_no_immersive_skips_visionos_guard() {
    let swift = crate::codegen_for_test("MacApp", &[("main", true)], &[], &[]);
    // Should NOT have visionOS guards when no immersive/volume scenes
    assert!(!swift.contains("#if os(visionOS)"));
    assert!(!swift.contains("import RealityKit"));
}

#[test]
fn codegen_callbacks_are_optional() {
    let swift = crate::codegen_for_test("App", &[("main", true)], &[], &[]);
    // Callbacks use optional chaining (?)
    assert!(swift.contains("rustOnInit?()"));
    assert!(swift.contains("rustOnFrame_main?("));
}

#[test]
fn codegen_swift_compiles() {
    // Generate Swift and verify it compiles with swiftc
    let swift = crate::codegen_for_test("CompileTest", &[("main", true)], &[], &[]);

    let dir = std::env::temp_dir().join("swiftui_app_test");
    std::fs::create_dir_all(&dir).unwrap();
    let swift_path = dir.join("CompileTest.swift");
    let dylib_path = dir.join("libCompileTest.dylib");

    std::fs::write(&swift_path, &swift).unwrap();

    let status = std::process::Command::new("swiftc")
        .args([
            "-parse-as-library",
            "-emit-library",
            "-O",
            "-o", dylib_path.to_str().unwrap(),
            swift_path.to_str().unwrap(),
            "-framework", "MetalKit",
        ])
        .status();

    match status {
        Ok(s) => assert!(s.success(), "Generated Swift failed to compile:\n{swift}"),
        Err(e) => {
            eprintln!("swiftc not available ({e}), skipping compile test");
        }
    }

    // Cleanup
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn codegen_multi_window_compiles() {
    let swift = crate::codegen_for_test(
        "MultiCompile",
        &[("main", true), ("aux", true)],
        &[],
        &[],
    );

    let dir = std::env::temp_dir().join("swiftui_app_test_multi");
    std::fs::create_dir_all(&dir).unwrap();
    let swift_path = dir.join("MultiCompile.swift");
    let dylib_path = dir.join("libMultiCompile.dylib");

    std::fs::write(&swift_path, &swift).unwrap();

    let status = std::process::Command::new("swiftc")
        .args([
            "-parse-as-library",
            "-emit-library",
            "-O",
            "-o", dylib_path.to_str().unwrap(),
            swift_path.to_str().unwrap(),
            "-framework", "MetalKit",
        ])
        .status();

    match status {
        Ok(s) => assert!(s.success(), "Multi-window Swift failed to compile"),
        Err(_) => eprintln!("swiftc not available, skipping"),
    }

    let _ = std::fs::remove_dir_all(&dir);
}

// ── Event type tests ────────────────────────────────────────────────────────

#[test]
fn event_debug_format() {
    let init = crate::Event::Init;
    let frame = crate::Event::Frame {
        window: "main",
        texture: std::ptr::null_mut(),
    };
    let term = crate::Event::Terminate;

    assert_eq!(format!("{init:?}"), "Init");
    assert!(format!("{frame:?}").contains("main"));
    assert_eq!(format!("{term:?}"), "Terminate");
}
