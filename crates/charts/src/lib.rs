//! Swift Charts — data visualization from Rust.
//!
//! **Platform support:** macOS 13+, iOS 16+, tvOS 16+, visionOS 1+, watchOS 9+.
//!
//! ```ignore
//! use swift_charts::*;
//!
//! let chart = bar_chart(&[
//!     ("Mon", 10.0), ("Tue", 25.0), ("Wed", 15.0),
//!     ("Thu", 30.0), ("Fri", 20.0),
//! ]).size(300.0, 200.0).x_label("Day").y_label("Sales");
//! ```
//!
//! Returns SwiftUI `ViewHandle`s — embed in any SwiftUI layout.

use core::ffi::c_void;

pub type Handle = *mut c_void;

/// A chart view that can be modified with size, labels, etc.
pub struct Chart {
    handle: Handle,
}

impl Chart {
    /// Set chart frame size.
    pub fn size(self, width: f32, height: f32) -> Self {
        let f = apple_sys_helpers::sym(c"charts_frame");
        if f.is_null() {
            return self;
        }
        type F = unsafe extern "C" fn(Handle, f32, f32) -> Handle;
        Self {
            handle: unsafe { (std::mem::transmute::<_, F>(f))(self.handle, width, height) },
        }
    }

    /// Set X axis label.
    pub fn x_label(self, label: &str) -> Self {
        let f = apple_sys_helpers::sym(c"charts_x_label");
        if f.is_null() {
            return self;
        }
        type F = unsafe extern "C" fn(Handle, *const u8, usize) -> Handle;
        Self {
            handle: unsafe {
                (std::mem::transmute::<_, F>(f))(self.handle, label.as_ptr(), label.len())
            },
        }
    }

    /// Set Y axis label.
    pub fn y_label(self, label: &str) -> Self {
        let f = apple_sys_helpers::sym(c"charts_y_label");
        if f.is_null() {
            return self;
        }
        type F = unsafe extern "C" fn(Handle, *const u8, usize) -> Handle;
        Self {
            handle: unsafe {
                (std::mem::transmute::<_, F>(f))(self.handle, label.as_ptr(), label.len())
            },
        }
    }

    /// Get the raw ViewHandle for embedding in SwiftUI.
    pub fn handle(&self) -> Handle {
        self.handle
    }
}

fn build_chart(sym_name: &core::ffi::CStr, data: &[(&str, f64)]) -> Chart {
    let f = apple_sys_helpers::sym(sym_name);
    assert!(!f.is_null(), "Charts framework not available");
    let labels: Vec<*const u8> = data.iter().map(|(l, _)| l.as_ptr()).collect();
    let lens: Vec<usize> = data.iter().map(|(l, _)| l.len()).collect();
    let values: Vec<f64> = data.iter().map(|(_, v)| *v).collect();
    type F = unsafe extern "C" fn(*const *const u8, *const usize, *const f64, usize) -> Handle;
    let handle = unsafe {
        (std::mem::transmute::<_, F>(f))(
            labels.as_ptr(),
            lens.as_ptr(),
            values.as_ptr(),
            data.len(),
        )
    };
    Chart { handle }
}

/// Create a bar chart.
pub fn bar_chart(data: &[(&str, f64)]) -> Chart {
    build_chart(c"charts_bar", data)
}

/// Create a line chart.
pub fn line_chart(data: &[(&str, f64)]) -> Chart {
    build_chart(c"charts_line", data)
}

/// Create an area chart.
pub fn area_chart(data: &[(&str, f64)]) -> Chart {
    build_chart(c"charts_area", data)
}

/// Create a scatter / point chart.
pub fn point_chart(data: &[(&str, f64)]) -> Chart {
    build_chart(c"charts_point", data)
}

/// Create a pie chart (or donut with inner_radius > 0).
pub fn pie_chart(data: &[(&str, f64)], inner_radius: f32) -> Chart {
    let f = apple_sys_helpers::sym(c"charts_pie");
    assert!(!f.is_null(), "Charts framework not available");
    let labels: Vec<*const u8> = data.iter().map(|(l, _)| l.as_ptr()).collect();
    let lens: Vec<usize> = data.iter().map(|(l, _)| l.len()).collect();
    let values: Vec<f64> = data.iter().map(|(_, v)| *v).collect();
    type F = unsafe extern "C" fn(*const *const u8, *const usize, *const f64, usize, f32) -> Handle;
    let handle = unsafe {
        (std::mem::transmute::<_, F>(f))(
            labels.as_ptr(),
            lens.as_ptr(),
            values.as_ptr(),
            data.len(),
            inner_radius,
        )
    };
    Chart { handle }
}

/// Create a donut chart (pie with hole).
pub fn donut_chart(data: &[(&str, f64)]) -> Chart {
    pie_chart(data, 0.5)
}
