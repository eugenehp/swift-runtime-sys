//! Style presets for common view patterns.

use crate::color::Color;
use crate::view::View;

/// Apply a named style preset to a view.
pub trait Styled {
    fn style(self, style: StylePreset) -> View;
}

impl Styled for crate::dsl::TextView {
    fn style(self, style: StylePreset) -> View {
        let view: View = self.into();
        view.style(style)
    }
}

impl Styled for View {
    fn style(self, style: StylePreset) -> View {
        match style {
            StylePreset::Title => self
                .font(28.0, crate::view::FontWeight::Bold)
                .foreground(Color::WHITE),
            StylePreset::Subtitle => self
                .font(14.0, crate::view::FontWeight::Regular)
                .foreground(Color::GRAY),
            StylePreset::Caption => self
                .font(11.0, crate::view::FontWeight::Regular)
                .foreground(rgb(0.4, 0.4, 0.4)),
            StylePreset::Heading => self
                .font(22.0, crate::view::FontWeight::Semibold)
                .foreground(Color::WHITE),
            StylePreset::Body => self
                .font(15.0, crate::view::FontWeight::Regular)
                .foreground(Color::WHITE),
            StylePreset::CardDark => self.padding(16.0).bg(Color::DARK).rounded(12.0),
            StylePreset::CardLight => self.padding(16.0).bg(rgb(0.95, 0.95, 0.97)).rounded(12.0),
            StylePreset::Pill => self.padding(8.0).bg(Color::BLUE).rounded(20.0),
            StylePreset::Elevated => self.padding(16.0).bg(Color::DARK).rounded(12.0).shadow(
                rgb(0.0, 0.0, 0.0),
                8.0,
                0.0,
                4.0,
            ),
            StylePreset::Page => self.padding(20.0).bg(Color::DARKER).scroll(),
        }
    }
}

fn rgb(r: f32, g: f32, b: f32) -> Color {
    Color::rgb(r, g, b)
}

/// Apply multiple styles by chaining.
/// ```ignore
/// view.styles(&[Title, CardDark])  // title text inside a dark card
/// ```
pub trait MultiStyled {
    fn styles(self, presets: &[StylePreset]) -> View;
}

impl MultiStyled for View {
    fn styles(self, presets: &[StylePreset]) -> View {
        presets.iter().fold(self, |v, s| v.style(*s))
    }
}

impl MultiStyled for crate::dsl::TextView {
    fn styles(self, presets: &[StylePreset]) -> View {
        let view: View = self.into();
        presets.iter().fold(view, |v, s| v.style(*s))
    }
}

/// Predefined style presets.
#[derive(Clone, Copy, Debug)]
pub enum StylePreset {
    /// Large bold white text
    Title,
    /// Small gray italic text
    Subtitle,
    /// Tiny dim text
    Caption,
    /// Medium semibold text
    Heading,
    /// Regular body text
    Body,
    /// Dark card with padding + rounded corners
    CardDark,
    /// Light card
    CardLight,
    /// Small rounded pill shape
    Pill,
    /// Dark card with shadow
    Elevated,
    /// Full page with padding, background, scroll
    Page,
}
