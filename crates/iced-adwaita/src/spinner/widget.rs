use aura_anim::core::interpolate::InterpolationProgress;
use iced::{
    Element, Length,
    time::{Duration, Instant},
    widget::{container, shader as shader_widget},
};
use spectrum_theme::{Color, iced::IcedColorAdapter};

use crate::context::ViewCx;

use super::{SpinnerFrame, SpinnerVisual, shader::SpinnerShader};

/// Stateful Adwaita spinner component.
#[derive(Clone, Copy, Debug)]
pub struct Spinner {
    size: Option<f32>,
    color: Option<Color>,
    started_at: Option<Instant>,
    elapsed: Duration,
}

impl Spinner {
    /// Creates a spinner using the current Adwaita context visual defaults.
    #[must_use]
    pub fn new() -> Self {
        Self {
            size: None,
            color: None,
            started_at: None,
            elapsed: Duration::ZERO,
        }
    }

    /// Sets the requested square allocation size in logical pixels.
    #[must_use]
    pub const fn size(mut self, size: f32) -> Self {
        self.size = Some(size);
        self
    }

    /// Sets an instance-level current CSS-like spinner color override.
    #[must_use]
    pub const fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Advances the spinner sampling clock.
    pub fn advance(&mut self, now: Instant) {
        #[cfg(feature = "tracing")]
        let was_started = self.started_at.is_some();
        let started_at = *self.started_at.get_or_insert(now);
        self.elapsed = now.duration_since(started_at);
        #[cfg(feature = "tracing")]
        if !was_started {
            tracing::debug!(
                target: "iced_adwaita::spinner",
                ?now,
                "spinner timeline started"
            );
        }
    }

    /// Restarts the spinner timeline from the next frame.
    pub fn reset(&mut self) {
        self.started_at = None;
        self.elapsed = Duration::ZERO;
        #[cfg(feature = "tracing")]
        tracing::debug!(target: "iced_adwaita::spinner", "spinner timeline reset");
    }

    /// Returns the current sampled frame.
    #[must_use]
    pub fn frame(self) -> SpinnerFrame {
        if self.started_at.is_some() {
            SpinnerFrame::from_elapsed(self.elapsed)
        } else {
            SpinnerFrame::static_frame()
        }
    }

    /// Returns the current resolved visual inputs.
    #[must_use]
    pub fn visual(self, context: &ViewCx<'_>) -> SpinnerVisual {
        let tokens = context.theme().pack().spinner;
        let color = self.color.unwrap_or_else(|| {
            context
                .style_transition()
                .map_or(tokens.color, |transition| {
                    crate::theme::interpolate::color(
                        transition.from.spinner.color,
                        tokens.color,
                        InterpolationProgress::new(transition.progress),
                    )
                })
        });

        SpinnerVisual::new(self.size.unwrap_or_else(|| tokens.size.value()), color)
    }

    /// Builds the Iced view.
    #[must_use]
    pub fn view<Message>(self, context: &ViewCx<'_>) -> Element<'static, Message>
    where
        Message: 'static,
    {
        SpinnerArc::new(self.frame(), self.visual(context)).into()
    }
}

impl Default for Spinner {
    fn default() -> Self {
        Self::new()
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
        let size = Length::Fixed(arc.visual.allocation_size());

        container(
            shader_widget(SpinnerShader {
                frame: arc.frame,
                diameter: arc.visual.diameter(),
                fg: arc.visual.active_color().color(),
                track: arc.visual.track_color().color(),
                stroke_width: arc.visual.stroke_width(),
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
    use iced_component_core::anim::MotionRuntime;
    use spectrum_theme::Color;

    use crate::{
        Context,
        context::{UpdateCx, ViewCx},
    };

    use super::Spinner;

    #[test]
    fn spinner_uses_static_frame_before_first_tick() {
        assert_eq!(Spinner::new().frame(), super::SpinnerFrame::static_frame());
    }

    #[test]
    fn builder_updates_visual_inputs() {
        let context = Context::light();
        let runtime = MotionRuntime::new();
        let view = ViewCx::new(&runtime, &context);
        let spinner = Spinner::new().size(48.0);

        assert_close(spinner.visual(&view).size, 48.0);
        assert_close(spinner.visual(&view).diameter(), 48.0);
    }

    #[test]
    fn visual_defaults_are_resolved_from_context_theme() {
        let mut context = Context::light();
        let mut runtime = MotionRuntime::new();
        let initial = Spinner::new().visual(&ViewCx::new(&runtime, &context));

        UpdateCx::new(&mut runtime, &mut context).patch_theme(|theme| {
            theme.spinner.color = Color::new(1, 2, 3);
        });

        let view = ViewCx::new(&runtime, &context);
        assert_eq!(Spinner::new().visual(&view).color, initial.color);

        runtime.tick(aura_anim::prelude::Duration::from_millis(200.0));
        let view = ViewCx::new(&runtime, &context);

        assert_eq!(Spinner::new().visual(&view).color, Color::new(1, 2, 3));
    }

    fn assert_close(actual: f32, expected: f32) {
        assert!(
            (actual - expected).abs() < 0.001,
            "{actual} was not close to {expected}"
        );
    }
}
