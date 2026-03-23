//! Platform detection and OS-specific constants.
//!
//! Provides compile-time platform identification and runtime helpers
//! for locating Swift runtime libraries across Apple platforms.

/// The current Apple platform.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplePlatform {
    MacOS,
    IOS,
    IOSSimulator,
    TvOS,
    TvOSSimulator,
    VisionOS,
    VisionOSSimulator,
    WatchOS,
    WatchOSSimulator,
}

impl ApplePlatform {
    /// Detect the current platform at compile time.
    ///
    /// Only available on Apple platforms. Returns `None` on non-Apple targets.
    #[cfg(any(
        target_os = "macos",
        target_os = "ios",
        target_os = "tvos",
        target_os = "xros",
        target_os = "watchos"
    ))]
    pub const CURRENT: ApplePlatform = {
        #[cfg(target_os = "macos")]
        {
            ApplePlatform::MacOS
        }
        #[cfg(all(target_os = "ios", not(target_abi = "sim")))]
        {
            ApplePlatform::IOS
        }
        #[cfg(all(target_os = "ios", target_abi = "sim"))]
        {
            ApplePlatform::IOSSimulator
        }
        #[cfg(all(target_os = "tvos", not(target_abi = "sim")))]
        {
            ApplePlatform::TvOS
        }
        #[cfg(all(target_os = "tvos", target_abi = "sim"))]
        {
            ApplePlatform::TvOSSimulator
        }
        #[cfg(all(target_os = "xros", not(target_abi = "sim")))]
        {
            ApplePlatform::VisionOS
        }
        #[cfg(all(target_os = "xros", target_abi = "sim"))]
        {
            ApplePlatform::VisionOSSimulator
        }
        #[cfg(all(target_os = "watchos", not(target_abi = "sim")))]
        {
            ApplePlatform::WatchOS
        }
        #[cfg(all(target_os = "watchos", target_abi = "sim"))]
        {
            ApplePlatform::WatchOSSimulator
        }
    };

    /// Whether this is a simulator target.
    pub const fn is_simulator(self) -> bool {
        matches!(
            self,
            ApplePlatform::IOSSimulator
                | ApplePlatform::TvOSSimulator
                | ApplePlatform::VisionOSSimulator
                | ApplePlatform::WatchOSSimulator
        )
    }

    /// Whether this platform has ObjC interop.
    pub const fn has_objc_interop(self) -> bool {
        // All Apple platforms have ObjC interop
        true
    }

    /// Whether this platform supports windowed apps (NSWindow/UIWindow).
    pub const fn has_windows(self) -> bool {
        matches!(
            self,
            ApplePlatform::MacOS
                | ApplePlatform::IOS
                | ApplePlatform::IOSSimulator
                | ApplePlatform::TvOS
                | ApplePlatform::TvOSSimulator
                | ApplePlatform::VisionOS
                | ApplePlatform::VisionOSSimulator
        )
    }

    /// The SDK name used by `xcrun --sdk`.
    pub const fn sdk_name(self) -> &'static str {
        match self {
            ApplePlatform::MacOS => "macosx",
            ApplePlatform::IOS => "iphoneos",
            ApplePlatform::IOSSimulator => "iphonesimulator",
            ApplePlatform::TvOS => "appletvos",
            ApplePlatform::TvOSSimulator => "appletvsimulator",
            ApplePlatform::VisionOS => "xros",
            ApplePlatform::VisionOSSimulator => "xrsimulator",
            ApplePlatform::WatchOS => "watchos",
            ApplePlatform::WatchOSSimulator => "watchsimulator",
        }
    }

    /// The Swift platform directory name under the SDK.
    pub const fn swift_platform_dir(self) -> &'static str {
        match self {
            ApplePlatform::MacOS => "macosx",
            ApplePlatform::IOS | ApplePlatform::IOSSimulator => "iphoneos",
            ApplePlatform::TvOS | ApplePlatform::TvOSSimulator => "appletvos",
            ApplePlatform::VisionOS | ApplePlatform::VisionOSSimulator => "xros",
            ApplePlatform::WatchOS | ApplePlatform::WatchOSSimulator => "watchos",
        }
    }

    /// The framework UI toolkit for this platform.
    pub const fn ui_framework(self) -> &'static str {
        match self {
            ApplePlatform::MacOS => "AppKit",
            _ => "UIKit",
        }
    }
}

// ── Convenience compile-time predicates ──

/// `true` on macOS
pub const IS_MACOS: bool = cfg!(target_os = "macos");
/// `true` on iOS (device or simulator)
pub const IS_IOS: bool = cfg!(target_os = "ios");
/// `true` on tvOS (device or simulator)
pub const IS_TVOS: bool = cfg!(target_os = "tvos");
/// `true` on visionOS (device or simulator)
pub const IS_VISIONOS: bool = cfg!(target_os = "xros");
/// `true` on watchOS (device or simulator)
pub const IS_WATCHOS: bool = cfg!(target_os = "watchos");
/// `true` on any Apple platform
pub const IS_APPLE: bool = IS_MACOS || IS_IOS || IS_TVOS || IS_VISIONOS || IS_WATCHOS;
/// `true` when targeting a simulator
pub const IS_SIMULATOR: bool = cfg!(target_abi = "sim");
