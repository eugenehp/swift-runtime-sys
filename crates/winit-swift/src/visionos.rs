//! visionOS immersive app support.
//!
//! Launches a full SwiftUI App with WindowGroup + ImmersiveSpace.
//! Rust owns `main()` and provides the Metal rendering callback.
//!
//! ```ignore
//! use winit_swift::visionos;
//!
//! fn main() {
//!     visionos::launch(on_init, on_frame);
//! }
//!
//! extern "C" fn on_init() {
//!     println!("SwiftUI app launched, surfaces ready");
//! }
//!
//! extern "C" fn on_frame(texture: *mut std::ffi::c_void) {
//!     // texture is an MTLTexture* — render into it with Metal
//! }
//! ```

use core::ffi::c_void;

type Handle = *mut c_void;

unsafe extern "C" {
    fn ws_visionos_configure(
        on_init: extern "C" fn(),
        on_frame: extern "C" fn(Handle),
    );
    fn ws_visionos_launch();
}

/// Launch a visionOS app with Rust Metal rendering.
///
/// This function **never returns**. It enters the SwiftUI event loop.
///
/// - `on_init`: Called once when the SwiftUI view appears (surfaces ready).
/// - `on_frame`: Called every frame with an `MTLTexture` pointer to render into.
///
/// On macOS, this creates a SwiftUI window with an MTKView.
/// On visionOS, this creates a WindowGroup + ImmersiveSpace.
pub fn launch(
    on_init: extern "C" fn(),
    on_frame: extern "C" fn(Handle),
) -> ! {
    unsafe {
        ws_visionos_configure(on_init, on_frame);
        ws_visionos_launch();
    }
    // SwiftUI.App.main() never returns, but if it somehow does:
    std::process::exit(0);
}
