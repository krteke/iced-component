use spectrum_theme::Color;

use crate::theme::tokens::SpinnerTokens;

/// Fully resolved visual values used to render a spinner.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpinnerAppearance {
    /// Square allocation size in logical pixels.
    pub size: f32,
    /// Arc color.
    pub foreground: Color,
    /// Background ring color.
    pub track: Color,
    diameter: f32,
    stroke_width: f32,
}

impl SpinnerAppearance {
    pub(super) fn from_tokens(
        tokens: SpinnerTokens,
        size_override: Option<f32>,
        foreground_override: Option<Color>,
        track_override: Option<Color>,
    ) -> Self {
        let minimum_size = tokens.minimum_size.value().max(1.0);
        let maximum_size = tokens.maximum_size.value().max(minimum_size);
        let size = size_override
            .unwrap_or_else(|| tokens.size.value())
            .max(minimum_size);
        let diameter = size.clamp(minimum_size, maximum_size);
        let span = (maximum_size - minimum_size).max(f32::EPSILON);
        let progress = (diameter - minimum_size) / span;
        let stroke_width = interpolate(
            tokens.minimum_stroke.value(),
            tokens.maximum_stroke.value(),
            progress,
        )
        .clamp(0.0, diameter);

        Self {
            size,
            foreground: foreground_override.unwrap_or(tokens.foreground),
            track: track_override.unwrap_or(tokens.track),
            diameter,
            stroke_width,
        }
    }

    /// Returns the effective painted diameter.
    #[must_use]
    pub const fn diameter(self) -> f32 {
        self.diameter
    }

    /// Returns the allocation size consumed by the Iced layout tree.
    #[must_use]
    pub const fn allocation_size(self) -> f32 {
        self.size
    }

    /// Returns the responsive stroke width for the effective diameter.
    #[must_use]
    pub const fn stroke_width(self) -> f32 {
        self.stroke_width
    }
}

fn interpolate(from: f32, to: f32, progress: f32) -> f32 {
    from + (to - from) * progress.clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use spectrum_theme::{Color, Length, LengthUnit};

    use super::SpinnerAppearance;
    use crate::theme::tokens::SpinnerTokens;

    #[test]
    fn dimensions_and_stroke_are_resolved_from_theme_metrics() {
        let appearance = SpinnerAppearance::from_tokens(tokens(), Some(48.0), None, None);

        assert_close(appearance.size, 48.0);
        assert_close(appearance.diameter(), 48.0);
        assert_close(appearance.stroke_width(), 5.0);
    }

    #[test]
    fn diameter_and_stroke_clamp_to_the_declared_theme_range() {
        let appearance = SpinnerAppearance::from_tokens(tokens(), Some(200.0), None, None);

        assert_close(appearance.size, 200.0);
        assert_close(appearance.diameter(), 72.0);
        assert_close(appearance.stroke_width(), 7.0);
    }

    fn tokens() -> SpinnerTokens {
        SpinnerTokens {
            foreground: Color::new(1, 2, 3),
            track: Color::new(4, 5, 6),
            size: px(16.0),
            minimum_size: px(12.0),
            maximum_size: px(72.0),
            minimum_stroke: px(2.0),
            maximum_stroke: px(7.0),
        }
    }

    fn px(value: f32) -> Length {
        Length::new(value, LengthUnit::Px).expect("valid pixel length")
    }

    fn assert_close(actual: f32, expected: f32) {
        assert!((actual - expected).abs() < 0.001);
    }
}
