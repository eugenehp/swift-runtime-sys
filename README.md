# swift-runtime-sys

An attempt to have Rust bindings generated for the Swift Runtime.

**Status**: WIP!

## Pick Swift version

```shell
cd swift && git checkout swift-6.0.1-RELEASE
```

## Build the bindings

```shell
export SWIFT_RUNTIME=$(xcrun -show-sdk-path)/usr/lib/swift
cargo build
```

## License

[Apache 2.0](/LICENSE)