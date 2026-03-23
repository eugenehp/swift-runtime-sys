//! RealityKit scene — demonstrates 3D content creation for visionOS.
//!
//! On macOS, this creates a window and shows how RealityKit entities
//! would be composed. On visionOS, these entities would appear in
//! an immersive space or volume.
//!
//! ```bash
//! cargo run -p winit-swift --example realitykit_scene
//! ```
//!
//! ## RealityKit Architecture
//!
//! ```text
//! Scene
//! ├── AnchorEntity (world origin)
//! │   ├── ModelEntity (sphere)
//! │   │   ├── MeshResource.generateSphere(radius: 0.1)
//! │   │   └── SimpleMaterial(color: .blue)
//! │   ├── ModelEntity (floor plane)
//! │   │   ├── MeshResource.generatePlane(width: 2, depth: 2)
//! │   │   └── SimpleMaterial(color: .gray)
//! │   └── PointLight
//! │       └── intensity: 1000
//! └── Camera (automatic on visionOS)
//! ```
//!
//! ## How it works with winit-swift
//!
//! ```ignore
//! // On visionOS, you'd create entities like this:
//! let space = app.create_window(
//!     WindowAttributes::new("Scene", 1.0, 1.0)
//!         .volume(1.0, 1.0, 1.0)  // 1m³ volume
//! );
//!
//! // Add RealityKit entities
//! let scene = RealityScene::new();
//! scene.add_sphere(0.1, [0.0, 0.5, 0.0], Color::BLUE);
//! scene.add_plane(2.0, 2.0, [0.0, 0.0, 0.0], Color::GRAY);
//! scene.add_point_light([1.0, 2.0, 1.0], 1000.0);
//! space.set_scene(&scene);
//! ```

use winit_swift::*;

/// Represents a 3D entity in a RealityKit scene.
/// This is a preview/development representation — on visionOS,
/// these map directly to RealityKit Entity objects.
#[derive(Debug)]
enum Entity {
    Sphere { radius: f32, position: [f32; 3], color: [f32; 4] },
    Plane { width: f32, depth: f32, position: [f32; 3], color: [f32; 4] },
    PointLight { position: [f32; 3], intensity: f32 },
    Text { content: String, position: [f32; 3], size: f32 },
}

/// A simple scene graph for development preview.
struct Scene {
    entities: Vec<Entity>,
}

impl Scene {
    fn new() -> Self { Scene { entities: Vec::new() } }

    fn add_sphere(&mut self, radius: f32, pos: [f32; 3], color: [f32; 4]) {
        self.entities.push(Entity::Sphere { radius, position: pos, color });
    }

    fn add_plane(&mut self, width: f32, depth: f32, pos: [f32; 3], color: [f32; 4]) {
        self.entities.push(Entity::Plane { width, depth, position: pos, color });
    }

    fn add_point_light(&mut self, pos: [f32; 3], intensity: f32) {
        self.entities.push(Entity::PointLight { position: pos, intensity });
    }

    fn add_text(&mut self, content: &str, pos: [f32; 3], size: f32) {
        self.entities.push(Entity::Text {
            content: content.into(), position: pos, size
        });
    }
}

fn main() {
    let app = App::new();
    let gpu = app.metal_device().expect("No GPU");
    println!("GPU: {}", gpu.name());

    // Build the scene
    let mut scene = Scene::new();

    // Ground plane
    scene.add_plane(2.0, 2.0, [0.0, 0.0, 0.0],
                    [0.3, 0.3, 0.35, 1.0]); // gray

    // Spheres in a ring
    let n = 8;
    for i in 0..n {
        let angle = (i as f32) * std::f32::consts::TAU / (n as f32);
        let x = angle.cos() * 0.6;
        let z = angle.sin() * 0.6;
        let hue = (i as f32) / (n as f32);

        // HSV to RGB (simplified)
        let (r, g, b) = hsv_to_rgb(hue, 0.8, 0.9);
        scene.add_sphere(0.08, [x, 0.3, z], [r, g, b, 1.0]);
    }

    // Central larger sphere
    scene.add_sphere(0.15, [0.0, 0.5, 0.0], [0.9, 0.9, 1.0, 0.8]);

    // Lights
    scene.add_point_light([1.0, 2.0, 1.0], 1000.0);
    scene.add_point_light([-1.0, 1.5, -0.5], 500.0);

    // Title
    scene.add_text("RealityKit Scene", [0.0, 1.2, 0.0], 0.1);

    // Print scene summary
    println!("\nScene entities:");
    for (i, entity) in scene.entities.iter().enumerate() {
        match entity {
            Entity::Sphere { radius, position, .. } =>
                println!("  [{i}] Sphere r={radius:.2} at ({:.1}, {:.1}, {:.1})",
                         position[0], position[1], position[2]),
            Entity::Plane { width, depth, .. } =>
                println!("  [{i}] Plane {width:.1}×{depth:.1}"),
            Entity::PointLight { position, intensity } =>
                println!("  [{i}] Light intensity={intensity:.0} at ({:.1}, {:.1}, {:.1})",
                         position[0], position[1], position[2]),
            Entity::Text { content, .. } =>
                println!("  [{i}] Text \"{content}\""),
        }
    }

    // On macOS, render a preview window
    let window = app.create_window(
        WindowAttributes::new("winit-swift — RealityKit Scene Preview", 800.0, 600.0)
    );
    let layer = window.metal_layer().expect("No Metal layer");
    let queue = gpu.command_queue();

    println!("\nOn visionOS, these entities would appear as 3D objects in space.");
    println!("On macOS, showing a preview window with the scene background.");
    println!("Press Esc to exit.\n");

    app.run(move |event, control| {
        match event {
            Event::CloseRequested(_) | Event::KeyDown { keycode: 53, .. } => {
                control.exit();
            }
            Event::RedrawRequested(_) | Event::Resized { .. } => {
                let Some(drawable) = layer.next_drawable() else { return };
                let cmd = queue.command_buffer();

                // Simple clear to a "spatial computing" background
                let enc = cmd.render_encoder(
                    &drawable,
                    0.05, 0.05, 0.08, 1.0, // dark blue-gray
                );
                enc.end();
                cmd.present_and_commit(&drawable);
            }
            _ => {}
        }
    });
}

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
    let i = (h * 6.0).floor() as i32;
    let f = h * 6.0 - i as f32;
    let p = v * (1.0 - s);
    let q = v * (1.0 - f * s);
    let t = v * (1.0 - (1.0 - f) * s);
    match i % 6 {
        0 => (v, t, p),
        1 => (q, v, p),
        2 => (p, v, t),
        3 => (p, q, v),
        4 => (t, p, v),
        _ => (v, p, q),
    }
}
