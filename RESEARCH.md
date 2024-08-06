git clone https://github.com/swiftlang/swift.git --depth 1
./swift/utils/build-toolchain RUST -n
cd swift
brew install cmark-gfm
cmake CMakeLists.txt

<!-- https://github.com/rust-lang/rust-bindgen/blob/main/bindgen-tests/build.rs -->

<!-- https://github.com/swiftlang/swift/blob/262ba2b24df0936dff62ac520638efcea9c2e16a/stdlib/public/SwiftShims/swift/shims/Visibility.h#L215 -->

// SWIFT_EXPORT_FROM(LIBRARY) declares something to be a C-linkage
// entity exported by the given library.
//
// SWIFT_RUNTIME_EXPORT is just SWIFT_EXPORT_FROM(swiftCore).

https://github.com/swiftlang/swift/blob/262ba2b24df0936dff62ac520638efcea9c2e16a/include/swift/Runtime/Config.h#L195

// Annotation for specifying a calling convention of
// a runtime function. It should be used with declarations
// of runtime functions like this:
// void runtime_function_name() SWIFT_CC(swift)
#define SWIFT_CC(CC) SWIFT_CC_##CC