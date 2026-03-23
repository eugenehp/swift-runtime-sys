//! Prelude — single import for the most common types.
//!
//! ```rust
//! use winit_swift::prelude::*;
//! ```
//!
//! This gives you everything you need for a typical app:
//! - Event loop: `EventLoop`, `ActiveEventLoop`, `ControlFlow`
//! - Application: `ApplicationHandler`
//! - Window: `Window`, `WindowAttributes`, `WindowId`
//! - Events: `WindowEvent`, `ElementState`, `KeyEvent`, `MouseButton`, `Theme`
//! - DPI: `PhysicalSize`, `LogicalSize`, `PhysicalPosition`
//! - Metal: `MetalDevice`, `MetalLayer`, `MetalDrawable`, `CommandQueue`, `pixel_format`, `resource_options`
//! - Apple extras: `Haptics`, `accessibility`, `thermal_state`, `monitors`

// ── Compat (winit-shaped) API ──
pub use crate::compat::application::ApplicationHandler;
pub use crate::compat::dpi::{LogicalSize, PhysicalInsets, PhysicalPosition, PhysicalSize, Size};
pub use crate::compat::event::{
    ElementState, KeyEvent, MouseButton, StartCause, Theme, TouchPhase, WindowEvent,
};
pub use crate::compat::event_loop::{ActiveEventLoop, ControlFlow, EventLoop, EventLoopProxy};
pub use crate::compat::window::{Window, WindowAttributes, WindowId};

// ── Metal ──
pub use crate::metal::{
    pixel_format, resource_options, CommandBuffer, CommandQueue, ComputePipeline, MetalBuffer,
    MetalDevice, RenderEncoder, RenderPipeline, ShaderFunction, ShaderLibrary,
};

// ── Window extras (Metal layer, drawable) ──
pub use crate::window::{MetalDrawable, MetalLayer, WindowLevel};

// ── Haptics ──
pub use crate::haptics::{HapticStyle, Haptics};

// ── Monitor ──
pub use crate::monitor::{monitor_count, monitor_info, monitors, primary_monitor, MonitorInfo};

// ── Accessibility & system ──
pub use crate::accessibility::{
    accessibility, is_low_power_mode, thermal_state, AccessibilityState, ThermalState,
};
