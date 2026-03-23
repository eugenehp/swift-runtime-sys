//! Accessibility and power/thermal state queries.

use crate::fns;

/// Accessibility settings.
pub struct AccessibilityState {
    /// VoiceOver screen reader is running.
    pub voiceover_running: bool,
    /// User prefers reduced motion.
    pub reduce_motion: bool,
    /// User prefers reduced transparency.
    pub reduce_transparency: bool,
    /// User prefers high contrast.
    pub high_contrast: bool,
}

/// Query current accessibility settings.
pub fn accessibility() -> AccessibilityState {
    let b = fns();
    AccessibilityState {
        voiceover_running: unsafe { (b.accessibility_is_voiceover_running)() },
        reduce_motion: unsafe { (b.accessibility_is_reduce_motion)() },
        reduce_transparency: unsafe { (b.accessibility_is_reduce_transparency)() },
        high_contrast: unsafe { (b.accessibility_is_high_contrast)() },
    }
}

/// System thermal state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThermalState {
    /// System is cool — no throttling.
    Nominal,
    /// System is warm — minor throttling may occur.
    Fair,
    /// System is hot — significant throttling.
    Serious,
    /// System is critically hot — maximum throttling.
    Critical,
}

/// Get the current thermal state.
pub fn thermal_state() -> ThermalState {
    match unsafe { (fns().thermal_state)() } {
        0 => ThermalState::Nominal,
        1 => ThermalState::Fair,
        2 => ThermalState::Serious,
        _ => ThermalState::Critical,
    }
}

/// Check if Low Power Mode is enabled.
pub fn is_low_power_mode() -> bool {
    unsafe { (fns().is_low_power_mode)() }
}
