# Platform Support

This project provides Rust bindings to **110 Apple framework crates** across all Apple platforms.
Each crate documents its platform availability and uses `#[cfg(target_os)]` to provide
real implementations on supported platforms and graceful no-op stubs elsewhere.

## Rust Target Triples

| Platform             | Target triple                    | Status   |
|----------------------|----------------------------------|----------|
| macOS (Apple Silicon)| `aarch64-apple-darwin`           | Stable   |
| macOS (Intel)        | `x86_64-apple-darwin`            | Stable   |
| iOS (device)         | `aarch64-apple-ios`              | Stable   |
| iOS (simulator)      | `aarch64-apple-ios-sim`          | Stable   |
| tvOS (device)        | `aarch64-apple-tvos`             | Nightly  |
| tvOS (simulator)     | `aarch64-apple-tvos-sim`         | Nightly  |
| visionOS (device)    | `aarch64-apple-xros`             | Nightly  |
| visionOS (simulator) | `aarch64-apple-xros-sim`         | Nightly  |
| watchOS (device)     | `aarch64-apple-watchos`          | Nightly  |
| watchOS (simulator)  | `aarch64-apple-watchos-sim`      | Nightly  |

## Framework Availability

Legend: ✅ = full support, ❌ = stub/no-op (compiles but returns defaults)

### Core Runtime

| Crate | macOS | iOS | tvOS | visionOS | watchOS |
|-------|:-----:|:---:|:----:|:--------:|:-------:|
| `swift-runtime-sys` | ✅ | ✅ | ✅ | ✅ | ✅ |
| `swift-runtime` | ✅ | ✅ | ✅ | ✅ | ✅ |
| `swiftui-sys` | ✅ | ✅ | ✅ | ✅ | ❌ |
| `swiftui` | ✅ | ✅ | ✅ | ✅ | ❌ |
| `realitykit-sys` | ✅ | ✅ | ❌ | ✅ | ❌ |
| `realitykit` | ✅ | ✅ | ❌ | ✅ | ❌ |
| `combine-rs` | ✅ | ✅ | ✅ | ✅ | ✅ |
| `swift-data` | ✅ | ✅ | ✅ | ✅ | ✅ |
| `spatial-rs` | ✅ | ✅ | ❌ | ✅ | ❌ |
| `foundation-models` | ✅ | ✅ | ❌ | ❌ | ❌ |

### Media & AV

| Crate | macOS | iOS | tvOS | visionOS | watchOS |
|-------|:-----:|:---:|:----:|:--------:|:-------:|
| `avfaudio-rs` | ✅ | ✅ | ✅ | ✅ | ✅ |
| `avfoundation-rs` | ✅ | ✅ | ✅ | ✅ | ❌ |
| `avkit-rs` | ✅ | ✅ | ✅ | ✅ | ❌ |
| `mediaplayer-rs` | ✅ | ✅ | ✅ | ❌ | ✅ |
| `musickit-rs` | ✅ | ✅ | ✅ | ❌ | ✅ |
| `cinematic-rs` | ✅ | ✅ | ❌ | ❌ | ❌ |
| `shazamkit-rs` | ✅ | ✅ | ✅ | ✅ | ✅ |
| `replaykit-rs` | ✅ | ✅ | ✅ | ❌ | ❌ |

### Vision, ML & AI

| Crate | macOS | iOS | tvOS | visionOS | watchOS |
|-------|:-----:|:---:|:----:|:--------:|:-------:|
| `coreml-rs` | ✅ | ✅ | ✅ | ✅ | ✅ |
| `vision-rs` | ✅ | ✅ | ✅ | ✅ | ❌ |
| `visionkit-rs` | ✅ | ✅ | ❌ | ✅ | ❌ |
| `createml-rs` | ✅ | ✅ | ❌ | ❌ | ❌ |
| `naturallanguage-rs` | ✅ | ✅ | ✅ | ✅ | ✅ |
| `soundanalysis-rs` | ✅ | ✅ | ✅ | ✅ | ✅ |
| `speech-rs` | ✅ | ✅ | ❌ | ✅ | ❌ |
| `imageplayground-rs` | ✅ | ✅ | ❌ | ❌ | ❌ |
| `sensitivecontentanalysis-rs` | ✅ | ✅ | ❌ | ✅ | ❌ |

### Data & Cloud

| Crate | macOS | iOS | tvOS | visionOS | watchOS |
|-------|:-----:|:---:|:----:|:--------:|:-------:|
| `cloudkit-rs` | ✅ | ✅ | ✅ | ✅ | ✅ |
| `coredata-rs` | ✅ | ✅ | ✅ | ✅ | ✅ |
| `swiftdata-rs` | ✅ | ✅ | ✅ | ✅ | ✅ |
| `tabulardata-rs` | ✅ | ✅ | ✅ | ✅ | ✅ |

### UI, Graphics & Rendering

| Crate | macOS | iOS | tvOS | visionOS | watchOS |
|-------|:-----:|:---:|:----:|:--------:|:-------:|
| `swift-charts` | ✅ | ✅ | ✅ | ✅ | ✅ |
| `coregraphics-rs` | ✅ | ✅ | ✅ | ✅ | ✅ |
| `coreimage-rs` | ✅ | ✅ | ✅ | ✅ | ❌ |
| `metal-rs` | ✅ | ✅ | ✅ | ✅ | ❌ |
| `metalkit-rs` | ✅ | ✅ | ✅ | ✅ | ❌ |
| `scenekit-rs` | ✅ | ✅ | ✅ | ✅ | ✅ |
| `spritekit-rs` | ✅ | ✅ | ✅ | ✅ | ✅ |
| `modelio-rs` | ✅ | ✅ | ✅ | ✅ | ❌ |
| `pencilkit-rs` | ✅ | ✅ | ❌ | ✅ | ❌ |
| `pdfkit-rs` | ✅ | ✅ | ❌ | ✅ | ❌ |
| `coretext-rs` | ✅ | ✅ | ✅ | ✅ | ✅ |
| `symbols-rs` | ✅ | ✅ | ✅ | ✅ | ✅ |

### Networking & Communication

| Crate | macOS | iOS | tvOS | visionOS | watchOS |
|-------|:-----:|:---:|:----:|:--------:|:-------:|
| `network-rs` | ✅ | ✅ | ✅ | ✅ | ✅ |
| `networkextension-rs` | ✅ | ✅ | ✅ | ✅ | ❌ |
| `multipeerconnectivity-rs` | ✅ | ✅ | ✅ | ✅ | ❌ |
| `nearbyinteraction-rs` | ✅ | ✅ | ❌ | ✅ | ✅ |
| `pushkit-rs` | ✅ | ✅ | ❌ | ✅ | ✅ |
| `callkit-rs` | ✅ | ✅ | ❌ | ❌ | ✅ |
| `livecommunicationkit-rs` | ✅ | ✅ | ❌ | ❌ | ❌ |

### Security & Authentication

| Crate | macOS | iOS | tvOS | visionOS | watchOS |
|-------|:-----:|:---:|:----:|:--------:|:-------:|
| `cryptokit-rs` | ✅ | ✅ | ✅ | ✅ | ✅ |
| `localauthentication-rs` | ✅ | ✅ | ❌ | ✅ | ✅ |
| `authenticationservices-rs` | ✅ | ✅ | ✅ | ✅ | ✅ |

### Location & Maps

| Crate | macOS | iOS | tvOS | visionOS | watchOS |
|-------|:-----:|:---:|:----:|:--------:|:-------:|
| `corelocation-rs` | ✅ | ✅ | ✅ | ✅ | ✅ |
| `mapkit-rs` | ✅ | ✅ | ✅ | ✅ | ✅ |

### Contacts & Calendar

| Crate | macOS | iOS | tvOS | visionOS | watchOS |
|-------|:-----:|:---:|:----:|:--------:|:-------:|
| `contacts-rs` | ✅ | ✅ | ❌ | ✅ | ✅ |
| `eventkit-rs` | ✅ | ✅ | ❌ | ✅ | ✅ |

### Photos

| Crate | macOS | iOS | tvOS | visionOS | watchOS |
|-------|:-----:|:---:|:----:|:--------:|:-------:|
| `photos-rs` | ✅ | ✅ | ✅ | ✅ | ❌ |
| `photosui-rs` | ✅ | ✅ | ❌ | ✅ | ✅ |

### Games

| Crate | macOS | iOS | tvOS | visionOS | watchOS |
|-------|:-----:|:---:|:----:|:--------:|:-------:|
| `gamecontroller-rs` | ✅ | ✅ | ✅ | ✅ | ❌ |
| `gamekit-rs` | ✅ | ✅ | ✅ | ✅ | ❌ |
| `gameplaykit-rs` | ✅ | ✅ | ✅ | ✅ | ❌ |

### Health & Fitness

| Crate | macOS | iOS | tvOS | visionOS | watchOS |
|-------|:-----:|:---:|:----:|:--------:|:-------:|
| `healthkit-rs` | ✅ | ✅ | ❌ | ✅ | ✅ |
| `workoutkit-rs` | ❌ | ✅ | ❌ | ❌ | ✅ |

### Payments & Commerce

| Crate | macOS | iOS | tvOS | visionOS | watchOS |
|-------|:-----:|:---:|:----:|:--------:|:-------:|
| `storekit-rs` | ✅ | ✅ | ✅ | ✅ | ✅ |
| `passkit-rs` | ✅ | ✅ | ❌ | ✅ | ✅ |
| `financekit-rs` | ✅ | ✅ | ❌ | ❌ | ❌ |

### Notifications & Activities

| Crate | macOS | iOS | tvOS | visionOS | watchOS |
|-------|:-----:|:---:|:----:|:--------:|:-------:|
| `usernotifications-rs` | ✅ | ✅ | ✅ | ✅ | ✅ |
| `activitykit-rs` | ❌ | ✅ | ❌ | ❌ | ❌ |
| `widgetkit-rs` | ✅ | ✅ | ❌ | ❌ | ✅ |

### Sharing & Social

| Crate | macOS | iOS | tvOS | visionOS | watchOS |
|-------|:-----:|:---:|:----:|:--------:|:-------:|
| `groupactivities-rs` | ✅ | ✅ | ✅ | ✅ | ❌ |
| `sharedwithyou-rs` | ✅ | ✅ | ✅ | ✅ | ❌ |
| `linkpresentation-rs` | ✅ | ✅ | ❌ | ✅ | ❌ |

### Search & Spotlight

| Crate | macOS | iOS | tvOS | visionOS | watchOS |
|-------|:-----:|:---:|:----:|:--------:|:-------:|
| `corespotlight-rs` | ✅ | ✅ | ❌ | ✅ | ❌ |

### Sensors & Motion

| Crate | macOS | iOS | tvOS | visionOS | watchOS |
|-------|:-----:|:---:|:----:|:--------:|:-------:|
| `coremotion-rs` | ❌ | ✅ | ❌ | ✅ | ✅ |
| `corehaptics-rs` | ✅ | ✅ | ❌ | ❌ | ❌ |
| `sensorkit-rs` | ❌ | ✅ | ❌ | ❌ | ❌ |

### Bluetooth & Accessories

| Crate | macOS | iOS | tvOS | visionOS | watchOS |
|-------|:-----:|:---:|:----:|:--------:|:-------:|
| `corebluetooth-rs` | ✅ | ✅ | ✅ | ✅ | ✅ |
| `accessorysetupkit-rs` | ✅ | ✅ | ❌ | ❌ | ❌ |
| `dockkit-rs` | ❌ | ✅ | ❌ | ❌ | ❌ |

### Text & Data Detection

| Crate | macOS | iOS | tvOS | visionOS | watchOS |
|-------|:-----:|:---:|:----:|:--------:|:-------:|
| `datadetection-rs` | ✅ | ✅ | ❌ | ✅ | ❌ |
| `translation-rs` | ✅ | ✅ | ❌ | ❌ | ❌ |

### Web & Safari

| Crate | macOS | iOS | tvOS | visionOS | watchOS |
|-------|:-----:|:---:|:----:|:--------:|:-------:|
| `webkit-rs` | ✅ | ✅ | ❌ | ✅ | ❌ |
| `safariservices-rs` | ✅ | ✅ | ❌ | ✅ | ❌ |

### Background & Extensions

| Crate | macOS | iOS | tvOS | visionOS | watchOS |
|-------|:-----:|:---:|:----:|:--------:|:-------:|
| `backgroundtasks-rs` | ✅ | ✅ | ✅ | ✅ | ✅ |
| `extensionkit-rs` | ✅ | ✅ | ✅ | ✅ | ✅ |
| `appintents-rs` | ✅ | ✅ | ✅ | ✅ | ✅ |

### File & Storage

| Crate | macOS | iOS | tvOS | visionOS | watchOS |
|-------|:-----:|:---:|:----:|:--------:|:-------:|
| `fileprovider-rs` | ✅ | ✅ | ❌ | ✅ | ❌ |
| `quicklook-rs` | ✅ | ✅ | ❌ | ✅ | ❌ |
| `uniformtypeidentifiers-rs` | ✅ | ✅ | ✅ | ✅ | ✅ |
| `coretransferable-rs` | ✅ | ✅ | ✅ | ✅ | ✅ |

### Weather

| Crate | macOS | iOS | tvOS | visionOS | watchOS |
|-------|:-----:|:---:|:----:|:--------:|:-------:|
| `weatherkit-rs` | ✅ | ✅ | ✅ | ✅ | ✅ |

### AR & Immersive

| Crate | macOS | iOS | tvOS | visionOS | watchOS |
|-------|:-----:|:---:|:----:|:--------:|:-------:|
| `arkit-rs` | ❌ | ✅ | ❌ | ✅ | ❌ |
| `compositorservices-rs` | ❌ | ❌ | ❌ | ✅ | ❌ |

### Accessibility

| Crate | macOS | iOS | tvOS | visionOS | watchOS |
|-------|:-----:|:---:|:----:|:--------:|:-------:|
| `accessibility-rs` | ✅ | ✅ | ✅ | ✅ | ✅ |

### Device Management

| Crate | macOS | iOS | tvOS | visionOS | watchOS |
|-------|:-----:|:---:|:----:|:--------:|:-------:|
| `devicecheck-rs` | ✅ | ✅ | ✅ | ✅ | ✅ |
| `deviceactivity-rs` | ✅ | ✅ | ❌ | ❌ | ❌ |
| `familycontrols-rs` | ✅ | ✅ | ❌ | ❌ | ❌ |
| `managedsettings-rs` | ✅ | ✅ | ❌ | ❌ | ❌ |

### Analytics & Logging

| Crate | macOS | iOS | tvOS | visionOS | watchOS |
|-------|:-----:|:---:|:----:|:--------:|:-------:|
| `oslog-rs` | ✅ | ✅ | ✅ | ✅ | ✅ |
| `metrickit-rs` | ✅ | ✅ | ❌ | ✅ | ❌ |
| `tipkit-rs` | ✅ | ✅ | ✅ | ✅ | ✅ |

### Screen Capture & Recording

| Crate | macOS | iOS | tvOS | visionOS | watchOS |
|-------|:-----:|:---:|:----:|:--------:|:-------:|
| `screencapturekit-rs` | ✅ | ❌ | ❌ | ❌ | ❌ |

### Smart Home

| Crate | macOS | iOS | tvOS | visionOS | watchOS |
|-------|:-----:|:---:|:----:|:--------:|:-------:|
| `matter-rs` | ✅ | ✅ | ✅ | ❌ | ❌ |

### Virtualization

| Crate | macOS | iOS | tvOS | visionOS | watchOS |
|-------|:-----:|:---:|:----:|:--------:|:-------:|
| `virtualization-rs` | ✅ | ❌ | ❌ | ❌ | ❌ |

### Ad & Attribution

| Crate | macOS | iOS | tvOS | visionOS | watchOS |
|-------|:-----:|:---:|:----:|:--------:|:-------:|
| `adservices-rs` | ✅ | ✅ | ❌ | ❌ | ❌ |
| `adsupport-rs` | ✅ | ✅ | ✅ | ✅ | ❌ |
| `apptrackingtransparency-rs` | ✅ | ✅ | ✅ | ✅ | ❌ |

## Cross-Compilation

### Building for iOS

```bash
rustup target add aarch64-apple-ios
cargo build --target aarch64-apple-ios

# Simulator
cargo build --target aarch64-apple-ios-sim
```

### Building for visionOS

```bash
rustup target add aarch64-apple-xros --toolchain nightly
cargo +nightly build --target aarch64-apple-xros
```

### Building for tvOS

```bash
rustup target add aarch64-apple-tvos --toolchain nightly
cargo +nightly build --target aarch64-apple-tvos
```

## Build Configuration

### Environment Variables

| Variable                          | Description                                    |
|-----------------------------------|------------------------------------------------|
| `SWIFT_RUNTIME`                   | Override Swift runtime library search path      |
| `SWIFTUI_HELPER`                  | Override path to SwiftUI helper dylib/framework |
| `SWIFT_RUNTIME_SYS_GENERATE_BINDINGS` | Re-generate bindgen FFI bindings          |

### build.rs Platform Detection

The `swift-runtime-sys` build.rs automatically:

1. Detects the target OS via `CARGO_CFG_TARGET_OS`
2. Resolves the correct SDK path via `xcrun --sdk <name>`
3. Sets linker search paths for the Swift runtime
4. Links the appropriate UI framework (`AppKit` on macOS, `UIKit` on iOS/tvOS/visionOS)
5. Exports `swift_platform="<os>"` and `swift_simulator` cfg flags

Each framework crate's `build.rs` conditionally links its framework based on platform.

### Cargo Features (swiftui crate)

Deployment target features: `macos-15`, `macos-26`, `ios-18`, `ios-26`,
`tvos-18`, `tvos-26`, `visionos-1`, `visionos-2`.

Framework features: `spatial`, `translation`, `widgetkit`, `appintents`,
`activitykit`, `realitykit`, `charts`, `all-frameworks`.

## Platform-Specific APIs

### macOS Only
- `App::titlebar_hidden()` — hide the title bar
- `App::hide_dock()` — hide the dock icon
- `App::material()` — window vibrancy/material
- `WindowStyle::Floating` — always-on-top window
- `screencapturekit` — screen recording
- `virtualization` — virtual machines

### visionOS Only
- `WindowStyle::Volumetric` — 3D volumetric window
- `WindowStyle::ImmersiveSpace` — full immersive experience
- `compositorservices` — low-level Metal rendering for visionOS

### iOS Only
- `activitykit` — Live Activities and Dynamic Island
- `dockkit` — motorized camera stand control
- `sensorkit` — research sensor data
