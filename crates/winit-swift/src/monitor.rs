//! Monitor/display enumeration.

use crate::fns;

/// Information about a connected display.
#[derive(Debug, Clone)]
pub struct MonitorInfo {
    /// Monitor index.
    pub index: usize,
    /// Position X (logical).
    pub x: i32,
    /// Position Y (logical).
    pub y: i32,
    /// Width (logical).
    pub width: u32,
    /// Height (logical).
    pub height: u32,
    /// Scale factor (e.g. 2.0 for Retina).
    pub scale_factor: f64,
    /// Display name.
    pub name: String,
    /// Maximum refresh rate in Hz.
    pub refresh_rate: f64,
}

/// Get the number of connected monitors.
pub fn monitor_count() -> usize {
    unsafe { (fns().monitor_count)() }
}

/// Get information about a monitor by index.
pub fn monitor_info(index: usize) -> Option<MonitorInfo> {
    if index >= monitor_count() {
        return None;
    }
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut w: u32 = 0;
    let mut h: u32 = 0;
    let mut scale: f64 = 0.0;
    let mut name_buf = vec![0u8; 256];

    let name_len = unsafe {
        (fns().monitor_info)(
            index,
            &mut x, &mut y, &mut w, &mut h, &mut scale,
            name_buf.as_mut_ptr(), name_buf.len(),
        )
    };

    let refresh = unsafe { (fns().monitor_refresh_rate)(index) };
    let name = String::from_utf8_lossy(&name_buf[..name_len]).to_string();

    Some(MonitorInfo {
        index, x, y,
        width: w, height: h,
        scale_factor: scale,
        name,
        refresh_rate: refresh,
    })
}

/// List all connected monitors.
pub fn monitors() -> Vec<MonitorInfo> {
    (0..monitor_count()).filter_map(monitor_info).collect()
}

/// Get the primary monitor.
pub fn primary_monitor() -> Option<MonitorInfo> {
    monitor_info(0)
}
