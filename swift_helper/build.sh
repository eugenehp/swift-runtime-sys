#!/bin/bash
# Build the SwiftUI helper dylib.
# Auto-detects SDK version. Override with MACOS_VERSION / IOS_VERSION env vars.
set -e
cd "$(dirname "$0")"

SOURCES="SwiftUIHelper.swift SnapshotHelper.swift Platform.swift AppHost.swift RealityKitHelper.swift DataHelper.swift FrameworkHelpers.swift"

# Auto-detect versions from SDK
MACOS_VERSION="${MACOS_VERSION:-$(xcrun --sdk macosx --show-sdk-version 2>/dev/null | cut -d. -f1).0}"
IOS_VERSION="${IOS_VERSION:-$(xcrun --sdk iphoneos --show-sdk-version 2>/dev/null | cut -d. -f1).0}"
# Fallback
MACOS_VERSION="${MACOS_VERSION:-15.0}"
IOS_VERSION="${IOS_VERSION:-18.0}"

build_macos() {
    local SDK=$(xcrun -sdk macosx --show-sdk-path 2>/dev/null)
    if [ -z "$SDK" ]; then echo "No macOS SDK found"; return 1; fi
    xcrun swiftc -emit-library $SOURCES \
        -o libSwiftUIHelper.dylib \
        -target arm64-apple-macosx${MACOS_VERSION} \
        -sdk "$SDK"
    echo "Built libSwiftUIHelper.dylib (macOS arm64, deployment target ${MACOS_VERSION})"
}

build_ios_sim() {
    local SDK=$(xcrun -sdk iphonesimulator --show-sdk-path 2>/dev/null)
    if [ -z "$SDK" ]; then echo "No iOS Simulator SDK found (need Xcode)"; return 1; fi
    xcrun swiftc -emit-library $SOURCES \
        -o libSwiftUIHelper_iossim.dylib \
        -target arm64-apple-ios${IOS_VERSION}-simulator \
        -sdk "$SDK"
    echo "Built libSwiftUIHelper_iossim.dylib (iOS Simulator arm64, deployment target ${IOS_VERSION})"
}

build_ios_device() {
    local SDK=$(xcrun -sdk iphoneos --show-sdk-path 2>/dev/null)
    if [ -z "$SDK" ]; then echo "No iOS SDK found (need Xcode)"; return 1; fi
    xcrun swiftc -emit-library $SOURCES \
        -o libSwiftUIHelper_ios.dylib \
        -target arm64-apple-ios${IOS_VERSION} \
        -sdk "$SDK"
    echo "Built libSwiftUIHelper_ios.dylib (iOS device arm64, deployment target ${IOS_VERSION})"
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
        echo "  MACOS_VERSION=$MACOS_VERSION  IOS_VERSION=$IOS_VERSION"
        exit 1
        ;;
esac
