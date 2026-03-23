//! FFI bridge function table loaded from the Swift dylib.

use core::ffi::{c_char, c_void};
use std::ffi::CString;

type Handle = *mut c_void;

unsafe extern "C" {
    fn dlopen(path: *const c_char, mode: i32) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
}

fn sym(h: *mut c_void, name: &core::ffi::CStr) -> *mut c_void {
    unsafe { dlsym(h, name.as_ptr()) }
}

pub(crate) struct Bridge {
    // Init
    pub init: unsafe extern "C" fn(extern "C" fn(u32, u64, i64, i64, f64, f64)) -> bool,
    pub metal_device: unsafe extern "C" fn() -> Handle,

    // Window creation/destruction
    pub create_window: unsafe extern "C" fn(*const u8, usize, f64, f64, u64) -> u64,
    pub destroy_window: unsafe extern "C" fn(u64),

    // Window getters
    pub window_metal_layer: unsafe extern "C" fn(u64) -> Handle,
    pub window_title: unsafe extern "C" fn(u64, *mut u8, usize) -> usize,
    pub window_size: unsafe extern "C" fn(u64, *mut u32, *mut u32),
    pub window_scale_factor: unsafe extern "C" fn(u64) -> f64,
    pub window_outer_position: unsafe extern "C" fn(u64, *mut i32, *mut i32),
    pub window_outer_size: unsafe extern "C" fn(u64, *mut u32, *mut u32),
    pub window_safe_area: unsafe extern "C" fn(u64, *mut u32, *mut u32, *mut u32, *mut u32),
    pub window_theme: unsafe extern "C" fn(u64) -> u8,
    pub window_has_focus: unsafe extern "C" fn(u64) -> bool,
    pub window_is_visible: unsafe extern "C" fn(u64) -> bool,
    pub window_is_minimized: unsafe extern "C" fn(u64) -> bool,
    pub window_is_maximized: unsafe extern "C" fn(u64) -> bool,
    pub window_is_fullscreen: unsafe extern "C" fn(u64) -> bool,
    pub window_is_resizable: unsafe extern "C" fn(u64) -> bool,
    pub window_is_decorated: unsafe extern "C" fn(u64) -> bool,
    pub window_raw_handle: unsafe extern "C" fn(u64) -> Handle,

    // Window setters
    pub window_set_title: unsafe extern "C" fn(u64, *const u8, usize),
    pub window_set_visible: unsafe extern "C" fn(u64, bool),
    pub window_set_fullscreen: unsafe extern "C" fn(u64, bool),
    pub window_set_minimized: unsafe extern "C" fn(u64, bool),
    pub window_set_maximized: unsafe extern "C" fn(u64, bool),
    pub window_set_resizable: unsafe extern "C" fn(u64, bool),
    pub window_set_decorations: unsafe extern "C" fn(u64, bool),
    pub window_set_outer_position: unsafe extern "C" fn(u64, i32, i32),
    pub window_set_min_size: unsafe extern "C" fn(u64, f64, f64),
    pub window_set_max_size: unsafe extern "C" fn(u64, f64, f64),
    pub window_set_blur: unsafe extern "C" fn(u64, bool),
    pub window_set_content_protected: unsafe extern "C" fn(u64, bool),
    pub window_set_window_level: unsafe extern "C" fn(u64, i32),
    pub window_set_transparent: unsafe extern "C" fn(u64, bool),
    pub window_set_theme: unsafe extern "C" fn(u64, i8),
    pub window_set_cursor_visible: unsafe extern "C" fn(u64, bool),
    pub window_set_cursor_position: unsafe extern "C" fn(u64, f64, f64),
    pub window_request_redraw: unsafe extern "C" fn(u64),
    pub window_request_attention: unsafe extern "C" fn(u64, bool),
    pub window_focus: unsafe extern "C" fn(u64),
    pub window_drag: unsafe extern "C" fn(u64),

    // Event loop
    pub run_event_loop: unsafe extern "C" fn(),
    pub poll_events: unsafe extern "C" fn() -> bool,
    pub poll_events_timeout: unsafe extern "C" fn(f64) -> bool,
    pub stop_event_loop: unsafe extern "C" fn(),

    // Metal
    pub metal_create_command_queue: unsafe extern "C" fn() -> Handle,
    pub metal_next_drawable: unsafe extern "C" fn(Handle) -> Handle,
    pub metal_drawable_texture: unsafe extern "C" fn(Handle) -> Handle,
    pub metal_present_drawable: unsafe extern "C" fn(Handle, Handle),
    pub metal_command_buffer: unsafe extern "C" fn(Handle) -> Handle,
    pub metal_commit: unsafe extern "C" fn(Handle),
    pub metal_wait: unsafe extern "C" fn(Handle),
    pub metal_make_library: unsafe extern "C" fn(*const u8, usize, *mut u8, usize) -> Handle,
    pub metal_make_function: unsafe extern "C" fn(Handle, *const u8, usize) -> Handle,
    pub metal_make_render_pipeline: unsafe extern "C" fn(Handle, Handle, u64, *mut u8, usize) -> Handle,
    pub metal_make_compute_pipeline: unsafe extern "C" fn(Handle, *mut u8, usize) -> Handle,
    pub metal_make_buffer: unsafe extern "C" fn(usize, u64) -> Handle,
    pub metal_make_buffer_data: unsafe extern "C" fn(*const c_void, usize, u64) -> Handle,
    pub metal_buffer_contents: unsafe extern "C" fn(Handle) -> *mut c_void,
    pub metal_render_encoder: unsafe extern "C" fn(Handle, Handle, f64, f64, f64, f64) -> Handle,
    pub metal_render_set_pipeline: unsafe extern "C" fn(Handle, Handle),
    pub metal_render_set_vertex_buffer: unsafe extern "C" fn(Handle, Handle, usize, usize),
    pub metal_render_draw: unsafe extern "C" fn(Handle, usize, usize),
    pub metal_render_end: unsafe extern "C" fn(Handle),
    pub metal_release: unsafe extern "C" fn(Handle),
    pub metal_device_name: unsafe extern "C" fn(*mut u8, usize) -> usize,
    pub metal_layer_set_vsync: unsafe extern "C" fn(Handle, bool),
    pub metal_layer_set_pixel_format: unsafe extern "C" fn(Handle, u64),
    pub metal_layer_set_drawable_count: unsafe extern "C" fn(Handle, usize),
    pub metal_layer_set_hdr: unsafe extern "C" fn(Handle, bool),

    // Haptics
    pub haptic_init: unsafe extern "C" fn() -> bool,
    pub haptic_play: unsafe extern "C" fn(f32, f32, f32) -> bool,
    pub haptic_impact: unsafe extern "C" fn(u8),

    // Monitor
    pub monitor_count: unsafe extern "C" fn() -> usize,
    pub monitor_info: unsafe extern "C" fn(usize, *mut i32, *mut i32, *mut u32, *mut u32, *mut f64, *mut u8, usize) -> usize,
    pub monitor_refresh_rate: unsafe extern "C" fn(usize) -> f64,

    // Accessibility
    pub accessibility_is_voiceover_running: unsafe extern "C" fn() -> bool,
    pub accessibility_is_reduce_motion: unsafe extern "C" fn() -> bool,
    pub accessibility_is_reduce_transparency: unsafe extern "C" fn() -> bool,
    pub accessibility_is_high_contrast: unsafe extern "C" fn() -> bool,

    // Power
    pub thermal_state: unsafe extern "C" fn() -> u8,
    pub is_low_power_mode: unsafe extern "C" fn() -> bool,
}

pub(crate) fn load_bridge(dylib_path: &str) -> Bridge {
    let cpath = CString::new(dylib_path).unwrap();
    let h = unsafe { dlopen(cpath.as_ptr(), 2) };
    assert!(!h.is_null(), "Failed to load winit-swift bridge: {dylib_path}");

    macro_rules! f {
        ($name:expr) => {
            unsafe { std::mem::transmute(sym(h, $name)) }
        };
    }

    Bridge {
        init: f!(c"ws_init"),
        metal_device: f!(c"ws_metal_device"),

        create_window: f!(c"ws_create_window"),
        destroy_window: f!(c"ws_destroy_window"),

        window_metal_layer: f!(c"ws_window_metal_layer"),
        window_title: f!(c"ws_window_title"),
        window_size: f!(c"ws_window_size"),
        window_scale_factor: f!(c"ws_window_scale_factor"),
        window_outer_position: f!(c"ws_window_outer_position"),
        window_outer_size: f!(c"ws_window_outer_size"),
        window_safe_area: f!(c"ws_window_safe_area"),
        window_theme: f!(c"ws_window_theme"),
        window_has_focus: f!(c"ws_window_has_focus"),
        window_is_visible: f!(c"ws_window_is_visible"),
        window_is_minimized: f!(c"ws_window_is_minimized"),
        window_is_maximized: f!(c"ws_window_is_maximized"),
        window_is_fullscreen: f!(c"ws_window_is_fullscreen"),
        window_is_resizable: f!(c"ws_window_is_resizable"),
        window_is_decorated: f!(c"ws_window_is_decorated"),
        window_raw_handle: f!(c"ws_window_raw_handle"),

        window_set_title: f!(c"ws_window_set_title"),
        window_set_visible: f!(c"ws_window_set_visible"),
        window_set_fullscreen: f!(c"ws_window_set_fullscreen"),
        window_set_minimized: f!(c"ws_window_set_minimized"),
        window_set_maximized: f!(c"ws_window_set_maximized"),
        window_set_resizable: f!(c"ws_window_set_resizable"),
        window_set_decorations: f!(c"ws_window_set_decorations"),
        window_set_outer_position: f!(c"ws_window_set_outer_position"),
        window_set_min_size: f!(c"ws_window_set_min_size"),
        window_set_max_size: f!(c"ws_window_set_max_size"),
        window_set_blur: f!(c"ws_window_set_blur"),
        window_set_content_protected: f!(c"ws_window_set_content_protected"),
        window_set_window_level: f!(c"ws_window_set_window_level"),
        window_set_transparent: f!(c"ws_window_set_transparent"),
        window_set_theme: f!(c"ws_window_set_theme"),
        window_set_cursor_visible: f!(c"ws_window_set_cursor_visible"),
        window_set_cursor_position: f!(c"ws_window_set_cursor_position"),
        window_request_redraw: f!(c"ws_window_request_redraw"),
        window_request_attention: f!(c"ws_window_request_attention"),
        window_focus: f!(c"ws_window_focus"),
        window_drag: f!(c"ws_window_drag"),

        run_event_loop: f!(c"ws_run_event_loop"),
        poll_events: f!(c"ws_poll_events"),
        poll_events_timeout: f!(c"ws_poll_events_timeout"),
        stop_event_loop: f!(c"ws_stop_event_loop"),

        metal_create_command_queue: f!(c"ws_metal_create_command_queue"),
        metal_next_drawable: f!(c"ws_metal_next_drawable"),
        metal_drawable_texture: f!(c"ws_metal_drawable_texture"),
        metal_present_drawable: f!(c"ws_metal_present_drawable"),
        metal_command_buffer: f!(c"ws_metal_command_buffer"),
        metal_commit: f!(c"ws_metal_commit"),
        metal_wait: f!(c"ws_metal_wait"),
        metal_make_library: f!(c"ws_metal_make_library"),
        metal_make_function: f!(c"ws_metal_make_function"),
        metal_make_render_pipeline: f!(c"ws_metal_make_render_pipeline"),
        metal_make_compute_pipeline: f!(c"ws_metal_make_compute_pipeline"),
        metal_make_buffer: f!(c"ws_metal_make_buffer"),
        metal_make_buffer_data: f!(c"ws_metal_make_buffer_data"),
        metal_buffer_contents: f!(c"ws_metal_buffer_contents"),
        metal_render_encoder: f!(c"ws_metal_render_encoder"),
        metal_render_set_pipeline: f!(c"ws_metal_render_set_pipeline"),
        metal_render_set_vertex_buffer: f!(c"ws_metal_render_set_vertex_buffer"),
        metal_render_draw: f!(c"ws_metal_render_draw"),
        metal_render_end: f!(c"ws_metal_render_end"),
        metal_release: f!(c"ws_metal_release"),
        metal_device_name: f!(c"ws_metal_device_name"),
        metal_layer_set_vsync: f!(c"ws_metal_layer_set_vsync"),
        metal_layer_set_pixel_format: f!(c"ws_metal_layer_set_pixel_format"),
        metal_layer_set_drawable_count: f!(c"ws_metal_layer_set_drawable_count"),
        metal_layer_set_hdr: f!(c"ws_metal_layer_set_hdr"),

        haptic_init: f!(c"ws_haptic_init"),
        haptic_play: f!(c"ws_haptic_play"),
        haptic_impact: f!(c"ws_haptic_impact"),

        monitor_count: f!(c"ws_monitor_count"),
        monitor_info: f!(c"ws_monitor_info"),
        monitor_refresh_rate: f!(c"ws_monitor_refresh_rate"),

        accessibility_is_voiceover_running: f!(c"ws_accessibility_is_voiceover_running"),
        accessibility_is_reduce_motion: f!(c"ws_accessibility_is_reduce_motion"),
        accessibility_is_reduce_transparency: f!(c"ws_accessibility_is_reduce_transparency"),
        accessibility_is_high_contrast: f!(c"ws_accessibility_is_high_contrast"),

        thermal_state: f!(c"ws_thermal_state"),
        is_low_power_mode: f!(c"ws_is_low_power_mode"),
    }
}
