import SwiftUI

private var rustTitle: String = "Hello from Rust (SIL)"

@_cdecl("shell_app_set_title_str")
public func shell_app_set_title_str(_ ptr: UnsafePointer<UInt8>, _ len: Int) {
    rustTitle = String(bytes: UnsafeBufferPointer(start: ptr, count: len), encoding: .utf8) ?? "Rust"
}

struct ShellApp: App {
    var body: some SwiftUI.Scene {
        WindowGroup {
            Text(rustTitle)
                .font(.largeTitle)
                .padding()
        }
    }
}

@_cdecl("shell_app_get_metadata")
public func shell_app_get_metadata() -> UnsafeRawPointer {
    unsafeBitCast(ShellApp.self as ShellApp.Type, to: UnsafeRawPointer.self)
}

@_cdecl("shell_app_launch")
public func shell_app_launch() {
    ShellApp.main()
}
