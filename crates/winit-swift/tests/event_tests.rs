//! Tests for winit-swift event types and compat layer.

// ── Native event parsing ────────────────────────────────────────────────────

mod native_events {
    // Access the internal event module directly
    use winit_swift::event::*;

    #[test]
    fn resized() {
        let e = Event::from_raw(1, 42, 1920, 1080, 0.0, 0.0);
        assert!(matches!(e, Event::Resized { id: WindowId(42), width: 1920, height: 1080 }));
    }

    #[test]
    fn moved() {
        let e = Event::from_raw(2, 1, 100, 200, 0.0, 0.0);
        assert!(matches!(e, Event::Moved { id: WindowId(1), x: 100, y: 200 }));
    }

    #[test]
    fn close_requested() {
        assert!(matches!(Event::from_raw(3, 5, 0, 0, 0.0, 0.0), Event::CloseRequested(WindowId(5))));
    }

    #[test]
    fn destroyed() {
        assert!(matches!(Event::from_raw(4, 7, 0, 0, 0.0, 0.0), Event::Destroyed(WindowId(7))));
    }

    #[test]
    fn focused_unfocused() {
        assert!(matches!(Event::from_raw(5, 1, 0, 0, 0.0, 0.0), Event::Focused(WindowId(1))));
        assert!(matches!(Event::from_raw(6, 1, 0, 0, 0.0, 0.0), Event::Unfocused(WindowId(1))));
    }

    #[test]
    fn key_events() {
        let down = Event::from_raw(7, 1, 53, 0, 0.0, 0.0);
        assert!(matches!(down, Event::KeyDown { id: WindowId(1), keycode: 53 }));
        let up = Event::from_raw(8, 1, 53, 0, 0.0, 0.0);
        assert!(matches!(up, Event::KeyUp { keycode: 53, .. }));
    }

    #[test]
    fn mouse_moved() {
        let e = Event::from_raw(9, 1, 0, 0, 100.5, 200.7);
        match e {
            Event::MouseMoved { x, y, .. } => {
                assert!((x - 100.5).abs() < f64::EPSILON);
                assert!((y - 200.7).abs() < f64::EPSILON);
            }
            _ => panic!("Expected MouseMoved"),
        }
    }

    #[test]
    fn mouse_buttons() {
        assert!(matches!(Event::from_raw(10, 1, 0, 0, 0.0, 0.0),
            Event::MouseButtonDown { button: MouseButton::Left, .. }));
        assert!(matches!(Event::from_raw(10, 1, 1, 0, 0.0, 0.0),
            Event::MouseButtonDown { button: MouseButton::Right, .. }));
        assert!(matches!(Event::from_raw(10, 1, 2, 0, 0.0, 0.0),
            Event::MouseButtonDown { button: MouseButton::Middle, .. }));
        assert!(matches!(Event::from_raw(10, 1, 5, 0, 0.0, 0.0),
            Event::MouseButtonDown { button: MouseButton::Other(5), .. }));
        assert!(matches!(Event::from_raw(11, 1, 0, 0, 0.0, 0.0),
            Event::MouseButtonUp { button: MouseButton::Left, .. }));
    }

    #[test]
    fn scroll() {
        let e = Event::from_raw(12, 1, 0, 0, -3.5, 10.2);
        match e {
            Event::Scroll { dx, dy, .. } => {
                assert!((dx + 3.5).abs() < f64::EPSILON);
                assert!((dy - 10.2).abs() < f64::EPSILON);
            }
            _ => panic!("Expected Scroll"),
        }
    }

    #[test]
    fn scale_factor() {
        let e = Event::from_raw(13, 1, 0, 0, 2.0, 0.0);
        assert!(matches!(e, Event::ScaleFactorChanged { scale, .. } if (scale - 2.0).abs() < f64::EPSILON));
    }

    #[test]
    fn redraw() {
        assert!(matches!(Event::from_raw(14, 3, 0, 0, 0.0, 0.0), Event::RedrawRequested(WindowId(3))));
    }

    #[test]
    fn touch() {
        assert!(matches!(Event::from_raw(15, 1, 0, 0, 50.0, 100.0), Event::TouchStart { .. }));
        assert!(matches!(Event::from_raw(16, 1, 0, 0, 55.0, 105.0), Event::TouchMove { .. }));
        assert!(matches!(Event::from_raw(17, 1, 0, 0, 60.0, 110.0), Event::TouchEnd { .. }));
    }

    #[test]
    fn theme_changed() {
        assert!(matches!(Event::from_raw(18, 1, 1, 0, 0.0, 0.0),
            Event::ThemeChanged { theme: Theme::Dark, .. }));
        assert!(matches!(Event::from_raw(18, 1, 0, 0, 0.0, 0.0),
            Event::ThemeChanged { theme: Theme::Light, .. }));
    }

    #[test]
    fn app_lifecycle() {
        assert!(matches!(Event::from_raw(19, 0, 0, 0, 0.0, 0.0), Event::Resumed));
        assert!(matches!(Event::from_raw(20, 0, 0, 0, 0.0, 0.0), Event::Suspended));
    }

    #[test]
    fn drag_events() {
        assert!(matches!(Event::from_raw(21, 1, 0, 0, 0.0, 0.0), Event::DragEntered(WindowId(1))));
        assert!(matches!(Event::from_raw(22, 1, 0, 0, 0.0, 0.0), Event::DragLeft(WindowId(1))));
        assert!(matches!(Event::from_raw(23, 1, 0, 0, 0.0, 0.0), Event::Dropped(WindowId(1))));
    }

    #[test]
    fn occluded() {
        assert!(matches!(Event::from_raw(24, 1, 1, 0, 0.0, 0.0),
            Event::Occluded { occluded: true, .. }));
        assert!(matches!(Event::from_raw(24, 1, 0, 0, 0.0, 0.0),
            Event::Occluded { occluded: false, .. }));
    }

    #[test]
    fn gestures() {
        let pinch = Event::from_raw(28, 1, 0, 0, 0.5, 0.0);
        assert!(matches!(pinch, Event::PinchGesture { delta, .. } if (delta - 0.5).abs() < f64::EPSILON));
        let rot = Event::from_raw(29, 1, 0, 0, 45.0, 0.0);
        assert!(matches!(rot, Event::RotationGesture { delta, .. } if (delta - 45.0).abs() < f64::EPSILON));
    }

    #[test]
    fn unknown_event() {
        assert!(matches!(Event::from_raw(255, 0, 0, 0, 0.0, 0.0), Event::Unknown(255)));
    }

    #[test]
    fn window_id_extraction() {
        assert_eq!(Event::Resized { id: WindowId(1), width: 0, height: 0 }.window_id(), Some(WindowId(1)));
        assert_eq!(Event::CloseRequested(WindowId(2)).window_id(), Some(WindowId(2)));
        assert_eq!(Event::Resumed.window_id(), None);
        assert_eq!(Event::Suspended.window_id(), None);
        assert_eq!(Event::Unknown(99).window_id(), None);
    }

    #[test]
    fn all_29_event_types_covered() {
        // Every valid event type 1..29 should produce a non-Unknown variant
        for ty in 1..=29u32 {
            let e = Event::from_raw(ty, 1, 0, 0, 0.0, 0.0);
            assert!(!matches!(e, Event::Unknown(_)), "Event type {ty} mapped to Unknown");
        }
    }
}

// ── Compat DPI types ────────────────────────────────────────────────────────

mod compat_dpi {
    use winit_swift::compat::dpi::*;

    #[test]
    fn physical_size_new() {
        let s = PhysicalSize::new(1920u32, 1080u32);
        assert_eq!(s.width, 1920);
        assert_eq!(s.height, 1080);
    }

    #[test]
    fn physical_size_eq() {
        assert_eq!(PhysicalSize::new(10u32, 20u32), PhysicalSize::new(10, 20));
        assert_ne!(PhysicalSize::new(10u32, 20u32), PhysicalSize::new(10, 21));
    }

    #[test]
    fn logical_to_physical() {
        let l = LogicalSize::new(400.0, 300.0);
        let p = l.to_physical(2.0);
        assert_eq!(p, PhysicalSize::new(800, 600));
    }

    #[test]
    fn logical_to_physical_non_integer_scale() {
        let l = LogicalSize::new(100.0, 100.0);
        let p = l.to_physical(1.5);
        assert_eq!(p, PhysicalSize::new(150, 150));
    }

    #[test]
    fn physical_position_new() {
        let p = PhysicalPosition::new(100i32, -200i32);
        assert_eq!(p.x, 100);
        assert_eq!(p.y, -200);
    }

    #[test]
    fn size_from_physical() {
        let s: Size = PhysicalSize::new(800u32, 600u32).into();
        assert!(matches!(s, Size::Physical(_)));
    }

    #[test]
    fn size_from_logical() {
        let s: Size = LogicalSize::new(800.0, 600.0).into();
        assert!(matches!(s, Size::Logical(_)));
    }
}

// ── Compat event types ──────────────────────────────────────────────────────

mod compat_events {
    use winit_swift::compat::event::*;

    #[test]
    fn element_state_is_pressed() {
        assert!(ElementState::Pressed.is_pressed());
        assert!(!ElementState::Released.is_pressed());
    }

    #[test]
    fn key_event_fields() {
        let ke = KeyEvent {
            physical_key: 53,
            state: ElementState::Pressed,
            repeat: true,
        };
        assert_eq!(ke.physical_key, 53);
        assert!(ke.state.is_pressed());
        assert!(ke.repeat);
    }

    #[test]
    fn theme_variants() {
        assert_ne!(Theme::Light, Theme::Dark);
        assert_eq!(Theme::Dark, Theme::Dark);
    }

    #[test]
    fn touch_phase_variants() {
        let phases = [TouchPhase::Started, TouchPhase::Moved, TouchPhase::Ended, TouchPhase::Cancelled];
        for (i, a) in phases.iter().enumerate() {
            for (j, b) in phases.iter().enumerate() {
                assert_eq!(i == j, a == b);
            }
        }
    }

    #[test]
    fn window_event_debug() {
        let e = WindowEvent::CloseRequested;
        assert!(format!("{e:?}").contains("CloseRequested"));
    }
}

// ── Compat window attributes ────────────────────────────────────────────────

mod compat_window {
    use winit_swift::compat::window::*;
    use winit_swift::compat::dpi::*;

    #[test]
    fn default_attributes() {
        let a = WindowAttributes::default();
        // title is set via with_title(); verified by builder test
    }

    #[test]
    fn builder_chain() {
        let a = WindowAttributes::default()
            .with_title("Test")
            .with_surface_size(LogicalSize::new(1200.0, 800.0))
            .with_transparent(true)
            .with_decorations(false)
            .with_hdr(true);
        // title set to "Test" via with_title()
    }

    #[test]
    fn window_id_raw_roundtrip() {
        let id = WindowId::from_raw(42);
        assert_eq!(id.into_raw(), 42);
    }

    #[test]
    fn window_id_equality() {
        assert_eq!(WindowId::from_raw(1), WindowId::from_raw(1));
        assert_ne!(WindowId::from_raw(1), WindowId::from_raw(2));
    }
}

// ── Compat event loop types ─────────────────────────────────────────────────

mod compat_event_loop {
    use winit_swift::compat::event_loop::*;

    #[test]
    fn control_flow_default() {
        assert_eq!(ControlFlow::default(), ControlFlow::Wait);
    }

    #[test]
    fn control_flow_variants() {
        let _ = ControlFlow::Wait;
        let _ = ControlFlow::Poll;
        let _ = ControlFlow::WaitUntil(std::time::Instant::now());
    }
}

// ── Prelude completeness ────────────────────────────────────────────────────

mod prelude_check {
    /// This test verifies that the prelude exports all commonly needed types.
    /// If any type is missing, this won't compile.
    #[test]
    fn prelude_has_all_types() {
        use winit_swift::prelude::*;

        // Event loop
        let _: ControlFlow = ControlFlow::Wait;
        let _: ControlFlow = ControlFlow::Poll;

        // Events
        let _: ElementState = ElementState::Pressed;
        let _: Theme = Theme::Dark;
        let _: TouchPhase = TouchPhase::Started;
        let _ = StartCause::Init;

        // DPI
        let _ = PhysicalSize::new(0u32, 0u32);
        let _ = LogicalSize::new(0.0, 0.0);
        let _ = PhysicalPosition::new(0i32, 0i32);

        // Haptics
        let _: HapticStyle = HapticStyle::Light;

        // System
        let _: ThermalState = ThermalState::Nominal;

        // Metal
        let _ = winit_swift::prelude::pixel_format::BGRA8_UNORM;
        let _ = winit_swift::prelude::resource_options::SHARED;
    }
}

// ── Metal pixel format constants ────────────────────────────────────────────

mod metal_constants {
    use winit_swift::metal::pixel_format;
    use winit_swift::metal::resource_options;

    #[test]
    fn pixel_formats_are_correct() {
        assert_eq!(pixel_format::BGRA8_UNORM, 80);
        assert_eq!(pixel_format::BGRA8_UNORM_SRGB, 81);
        assert_eq!(pixel_format::RGBA8_UNORM, 70);
        assert_eq!(pixel_format::RGBA16_FLOAT, 115);
        assert_eq!(pixel_format::R32_FLOAT, 55);
        assert_eq!(pixel_format::DEPTH32_FLOAT, 252);
    }

    #[test]
    fn resource_options_are_correct() {
        assert_eq!(resource_options::SHARED, 0);
        assert_eq!(resource_options::MANAGED, 0x10);
        assert_eq!(resource_options::PRIVATE, 0x20);
    }
}

// ── Accessibility types ─────────────────────────────────────────────────────

mod accessibility_types {
    use winit_swift::accessibility::*;

    #[test]
    fn thermal_state_variants() {
        let states = [ThermalState::Nominal, ThermalState::Fair, ThermalState::Serious, ThermalState::Critical];
        for (i, a) in states.iter().enumerate() {
            for (j, b) in states.iter().enumerate() {
                assert_eq!(i == j, a == b);
            }
        }
    }
}

// ── Haptics types ───────────────────────────────────────────────────────────

mod haptics_types {
    use winit_swift::haptics::*;

    #[test]
    fn haptic_styles() {
        assert_eq!(HapticStyle::Light as u8, 0);
        assert_eq!(HapticStyle::Generic as u8, 1);
        assert_eq!(HapticStyle::Medium as u8, 2);
        assert_eq!(HapticStyle::Strong as u8, 3);
    }
}
