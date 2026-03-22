#!/bin/bash
# Build the SwiftUI helper dylib
set -e
cd "$(dirname "$0")"
SDK=$(xcrun -sdk macosx --show-sdk-path)
xcrun swiftc -emit-library \
    SwiftUIHelper.swift SnapshotHelper.swift \
    -o libSwiftUIHelper.dylib \
    -target arm64-apple-macosx15.0 \
    -sdk "$SDK"
echo "Built swift_helper/libSwiftUIHelper.dylib"
