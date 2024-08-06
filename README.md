# swift-runtime-sys

An attempt to have Rust bindings generated for the Swift Runtime.

**Status**: WIP!

## Pick Swift version

```shell
git checkout swift-5.10.1-RELEASE
```

## Build the bindings

```shell
export SWIFT_RUNTIME=$(xcrun -show-sdk-path)/usr/lib/swift
cargo build
```

## License

[Apache 2.0](/LICENSE)