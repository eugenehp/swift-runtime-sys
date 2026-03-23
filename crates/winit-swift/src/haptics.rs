//! Haptic feedback — Core Haptics engine and system feedback.

use crate::fns;

/// Haptic feedback style for quick impacts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum HapticStyle {
    /// Light impact.
    Light = 0,
    /// Generic / alignment feedback.
    Generic = 1,
    /// Medium impact.
    Medium = 2,
    /// Strong / level change feedback.
    Strong = 3,
}

/// Haptic feedback engine.
pub struct Haptics {
    _initialized: bool,
}

impl Haptics {
    /// Initialize the haptic engine. Returns `None` if haptics are not supported.
    pub fn new() -> Option<Self> {
        let ok = unsafe { (fns().haptic_init)() };
        if ok { Some(Haptics { _initialized: true }) } else { None }
    }

    /// Play a custom haptic pattern.
    ///
    /// - `intensity`: 0.0 to 1.0
    /// - `sharpness`: 0.0 (dull) to 1.0 (sharp)
    /// - `duration`: seconds
    pub fn play(&self, intensity: f32, sharpness: f32, duration: f32) -> bool {
        unsafe { (fns().haptic_play)(intensity, sharpness, duration) }
    }

    /// Perform a quick impact/feedback.
    pub fn impact(&self, style: HapticStyle) {
        unsafe { (fns().haptic_impact)(style as u8) };
    }

    /// Light tap feedback.
    pub fn tap(&self) {
        self.play(0.5, 0.5, 0.05);
    }

    /// Selection change feedback.
    pub fn selection(&self) {
        self.impact(HapticStyle::Generic);
    }

    /// Success feedback.
    pub fn success(&self) {
        self.play(0.8, 0.3, 0.1);
    }

    /// Warning feedback.
    pub fn warning(&self) {
        self.play(0.9, 0.7, 0.15);
    }

    /// Error feedback.
    pub fn error(&self) {
        self.play(1.0, 1.0, 0.2);
    }
}
