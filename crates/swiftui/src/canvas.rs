//! Canvas drawing — command buffer pattern.
//!
//! ```ignore
//! use swiftui::canvas::*;
//!
//! let mut draw = DrawCommands::new(300.0, 200.0);
//! draw.fill_rect(10.0, 10.0, 100.0, 80.0, RED);
//! draw.fill_circle(150.0, 50.0, 30.0, BLUE);
//! draw.stroke_line(0.0, 0.0, 300.0, 200.0, WHITE, 2.0);
//! let view = draw.build();
//! ```

use crate::color::Color;
use crate::view::View;

/// A command buffer for Canvas drawing.
pub struct DrawCommands {
    width: f32,
    height: f32,
    cmds: Vec<f32>,
}

impl DrawCommands {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            cmds: Vec::new(),
        }
    }

    /// Fill a rectangle.
    pub fn fill_rect(&mut self, x: f32, y: f32, w: f32, h: f32, c: Color) -> &mut Self {
        self.cmds
            .extend_from_slice(&[0.0, x, y, w, h, c.r, c.g, c.b, c.a]);
        self
    }

    /// Fill a circle.
    pub fn fill_circle(&mut self, cx: f32, cy: f32, radius: f32, c: Color) -> &mut Self {
        self.cmds
            .extend_from_slice(&[1.0, cx, cy, radius, c.r, c.g, c.b, c.a]);
        self
    }

    /// Stroke a rectangle outline.
    pub fn stroke_rect(
        &mut self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        c: Color,
        line_width: f32,
    ) -> &mut Self {
        self.cmds
            .extend_from_slice(&[2.0, x, y, w, h, c.r, c.g, c.b, c.a, line_width]);
        self
    }

    /// Fill a rounded rectangle.
    pub fn fill_rounded_rect(
        &mut self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        corner_radius: f32,
        c: Color,
    ) -> &mut Self {
        self.cmds
            .extend_from_slice(&[3.0, x, y, w, h, corner_radius, c.r, c.g, c.b, c.a]);
        self
    }

    /// Fill an ellipse.
    pub fn fill_ellipse(&mut self, x: f32, y: f32, w: f32, h: f32, c: Color) -> &mut Self {
        self.cmds
            .extend_from_slice(&[4.0, x, y, w, h, c.r, c.g, c.b, c.a]);
        self
    }

    /// Stroke a line.
    pub fn stroke_line(
        &mut self,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        c: Color,
        line_width: f32,
    ) -> &mut Self {
        self.cmds
            .extend_from_slice(&[5.0, x1, y1, x2, y2, c.r, c.g, c.b, c.a, line_width]);
        self
    }

    /// Build the Canvas view from the command buffer.
    pub fn build(&self) -> View {
        crate::dsl::with_ui(|ui| {
            View::new(crate::handle::ViewHandle::new(
                unsafe {
                    (ui.fns.canvas_commands)(
                        self.width,
                        self.height,
                        self.cmds.as_ptr(),
                        self.cmds.len(),
                    )
                },
                ui.fns.release,
            ))
        })
    }
}
