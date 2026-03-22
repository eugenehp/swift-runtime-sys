#!/bin/bash
# Build the SwiftUI helper dylib for macOS and/or iOS
set -e
cd "$(dirname "$0")"

SOURCES="SwiftUIHelper.swift SnapshotHelper.swift Platform.swift AppHost.swift"

build_macos() {
    local SDK=$(xcrun -sdk macosx --show-sdk-path 2>/dev/null)
    if [ -z "$SDK" ]; then echo "No macOS SDK found"; return 1; fi
    xcrun swiftc -emit-library $SOURCES \
        -o libSwiftUIHelper.dylib \
        -target arm64-apple-macosx15.0 \
        -sdk "$SDK"
    echo "Built libSwiftUIHelper.dylib (macOS arm64)"
}

build_ios_sim() {
    local SDK=$(xcrun -sdk iphonesimulator --show-sdk-path 2>/dev/null)
    if [ -z "$SDK" ]; then echo "No iOS Simulator SDK found (need Xcode)"; return 1; fi
    xcrun swiftc -emit-library $SOURCES \
        -o libSwiftUIHelper_iossim.dylib \
        -target arm64-apple-ios18.0-simulator \
        -sdk "$SDK"
    echo "Built libSwiftUIHelper_iossim.dylib (iOS Simulator arm64)"
}

build_ios_device() {
    local SDK=$(xcrun -sdk iphoneos --show-sdk-path 2>/dev/null)
    if [ -z "$SDK" ]; then echo "No iOS SDK found (need Xcode)"; return 1; fi
    xcrun swiftc -emit-library $SOURCES \
        -o libSwiftUIHelper_ios.dylib \
        -target arm64-apple-ios18.0 \
        -sdk "$SDK"
    echo "Built libSwiftUIHelper_ios.dylib (iOS device arm64)"
}

case "${1:-macos}" in
    macos)   build_macos ;;
    ios-sim) build_ios_sim ;;
    ios)     build_ios_device ;;
    all)
        build_macos
        build_ios_sim 2>/dev/null || true
        build_ios_device 2>/dev/null || true
        ;;
    *)
        echo "Usage: $0 [macos|ios-sim|ios|all]"
        exit 1
        ;;
esac
