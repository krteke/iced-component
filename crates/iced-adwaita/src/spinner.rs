//! Adwaita spinner rendering primitives.

mod shader;

use iced::{
    Element, Length,
    widget::{container, shader as shader_widget},
};
use spectrum_theme::{Color, iced::IcedColorAdapter};

use self::shader::SpinnerShader;

/// One sampled spinner frame.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpinnerFrame {
    /// Clockwise rotation in degrees.
    pub rotation: f32,
}

impl SpinnerFrame {
    /// Creates a spinner frame from a rotation angle in degrees.
    #[must_use]
    pub const fn new(rotation: f32) -> Self {
        Self { rotation }
    }
}

/// Resolved spinner visual inputs.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpinnerVisual {
    /// Active arc color.
    pub fg: Color,
    /// Track color.
    pub track: Color,
    /// Visible outer diameter in logical pixels.
    pub size: f32,
    /// Stroke width in logical pixels.
    pub stroke_width: f32,
}

impl SpinnerVisual {
    /// Creates spinner visual inputs.
    #[must_use]
    pub const fn new(fg: Color, track: Color, size: f32, stroke_width: f32) -> Self {
        Self {
            fg,
            track,
            size,
            stroke_width,
        }
    }
}

/// Shader-rendered Adwaita spinner arc.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpinnerArc {
    frame: SpinnerFrame,
    visual: SpinnerVisual,
}

impl SpinnerArc {
    /// Creates a spinner arc from one sampled frame and resolved visual inputs.
    #[must_use]
    pub const fn new(frame: SpinnerFrame, visual: SpinnerVisual) -> Self {
        Self { frame, visual }
    }
}

impl<Message> From<SpinnerArc> for Element<'_, Message>
where
    Message: 'static,
{
    fn from(arc: SpinnerArc) -> Self {
        let size = Length::Fixed(arc.visual.size);

        container(
            shader_widget(SpinnerShader {
                frame: arc.frame,
                fg: arc.visual.fg.color(),
                track: arc.visual.track.color(),
                stroke_width: arc.visual.stroke_width,
            })
            .width(size)
            .height(size),
        )
        .width(size)
        .height(size)
        .center(size)
        .into()
    }
}

#[cfg(test)]
mod tests {
    use spectrum_theme::Color;

    use super::{SpinnerFrame, SpinnerVisual};

    #[test]
    fn frame_stores_rotation_without_animation_policy() {
        assert_close(SpinnerFrame::new(90.0).rotation, 90.0);
    }

    #[test]
    fn visual_stores_explicit_render_inputs() {
        let visual = SpinnerVisual::new(Color::new(1, 2, 3), Color::new(4, 5, 6), 24.0, 2.5);

        assert_eq!(visual.fg, Color::new(1, 2, 3));
        assert_eq!(visual.track, Color::new(4, 5, 6));
        assert_close(visual.size, 24.0);
        assert_close(visual.stroke_width, 2.5);
    }

    fn assert_close(actual: f32, expected: f32) {
        assert!((actual - expected).abs() < 0.001);
    }
}
