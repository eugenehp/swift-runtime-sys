#!/bin/bash
# Build the Metal bridge dylib
set -e
cd "$(dirname "$0")"
swiftc -emit-library -o libMetalBridge.dylib MetalBridge.swift \
    -framework Metal -framework CoreGraphics
echo "Built libMetalBridge.dylib ($(wc -c < libMetalBridge.dylib | tr -d ' ') bytes)"
