use spectrum_theme::Color;

const MIN_DIAMETER: f32 = 16.0;
const MAX_DIAMETER: f32 = 64.0;
const TRACK_ALPHA: u8 = 38;

/// Resolved Adwaita spinner visual inputs.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpinnerVisual {
    /// Widget allocation size in logical pixels.
    pub size: f32,
    /// Current CSS-like color used by the active arc.
    pub color: Color,
}

impl SpinnerVisual {
    /// Creates spinner visual inputs from a widget allocation size and current color.
    #[must_use]
    pub const fn new(size: f32, color: Color) -> Self {
        Self { size, color }
    }

    /// Returns the effective painted diameter following Adwaita's paintable clamp.
    #[must_use]
    pub fn diameter(self) -> f32 {
        let radius = (self.allocation_size() * 0.5)
            .floor()
            .min(MAX_DIAMETER * 0.5);

        (radius * 2.0).max(MIN_DIAMETER)
    }

    /// Returns the square widget allocation size.
    #[must_use]
    pub fn allocation_size(self) -> f32 {
        self.size.max(MIN_DIAMETER)
    }

    /// Returns the stroke width for the effective painted diameter.
    #[must_use]
    pub fn stroke_width(self) -> f32 {
        self.diameter() / 8.0
    }

    pub(super) fn active_color(self) -> Color {
        self.color
    }

    pub(super) fn track_color(self) -> Color {
        Color::new_rgba(
            self.color.red(),
            self.color.green(),
            self.color.blue(),
            TRACK_ALPHA,
        )
    }
}

#[cfg(test)]
mod tests {
    use spectrum_theme::Color;

    use super::SpinnerVisual;

    #[test]
    fn diameter_is_clamped_to_adwaita_paintable_range() {
        assert_close(
            SpinnerVisual::new(8.0, Color::new(1, 2, 3)).diameter(),
            16.0,
        );
        assert_close(
            SpinnerVisual::new(128.0, Color::new(1, 2, 3)).diameter(),
            64.0,
        );
    }

    #[test]
    fn diameter_uses_even_floor_like_adwaita_radius_math() {
        assert_close(
            SpinnerVisual::new(17.0, Color::new(1, 2, 3)).diameter(),
            16.0,
        );
        assert_close(
            SpinnerVisual::new(63.0, Color::new(1, 2, 3)).diameter(),
            62.0,
        );
    }

    #[test]
    fn stroke_width_is_one_eighth_of_painted_diameter() {
        assert_close(
            SpinnerVisual::new(16.0, Color::new(1, 2, 3)).stroke_width(),
            2.0,
        );
        assert_close(
            SpinnerVisual::new(64.0, Color::new(1, 2, 3)).stroke_width(),
            8.0,
        );
    }

    #[test]
    fn track_color_uses_current_color_with_adwaita_mask_alpha() {
        let track = SpinnerVisual::new(16.0, Color::new(0x8c, 0x8c, 0x90)).track_color();

        assert_eq!(track.red(), 0x8c);
        assert_eq!(track.green(), 0x8c);
        assert_eq!(track.blue(), 0x90);
        assert_eq!(track.alpha(), 38);
    }

    fn assert_close(actual: f32, expected: f32) {
        assert!(
            (actual - expected).abs() < 0.001,
            "{actual} was not close to {expected}"
        );
    }
}
