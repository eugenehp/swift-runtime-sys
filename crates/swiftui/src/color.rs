/// An RGBA color.
#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    pub const WHITE: Self = Self::rgb(1.0, 1.0, 1.0);
    pub const BLACK: Self = Self::rgb(0.0, 0.0, 0.0);
    pub const RED: Self = Self::rgb(1.0, 0.3, 0.3);
    pub const GREEN: Self = Self::rgb(0.3, 0.8, 0.3);
    pub const BLUE: Self = Self::rgb(0.3, 0.5, 1.0);
    pub const YELLOW: Self = Self::rgb(1.0, 0.8, 0.2);
    pub const PURPLE: Self = Self::rgb(0.8, 0.4, 1.0);
    pub const GRAY: Self = Self::rgb(0.5, 0.5, 0.5);
    pub const DARK: Self = Self::rgb(0.1, 0.1, 0.15);
    pub const DARKER: Self = Self::rgb(0.05, 0.05, 0.08);
    pub const CLEAR: Self = Self::rgba(0.0, 0.0, 0.0, 0.0);
}

/// Shorthand: `rgb(0.2, 0.4, 0.8)`
pub const fn rgb(r: f32, g: f32, b: f32) -> Color {
    Color::rgb(r, g, b)
}

/// Shorthand: `rgba(0.2, 0.4, 0.8, 0.5)`
pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Color {
    Color::rgba(r, g, b, a)
}

/// Hex color: `hex(0x3366CC)`
pub const fn hex(v: u32) -> Color {
    Color::rgb(
        ((v >> 16) & 0xFF) as f32 / 255.0,
        ((v >> 8) & 0xFF) as f32 / 255.0,
        (v & 0xFF) as f32 / 255.0,
    )
}
