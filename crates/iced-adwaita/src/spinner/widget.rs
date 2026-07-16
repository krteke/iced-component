use aura_anim::core::interpolate::InterpolationProgress;
use iced::{
    Element, Length,
    time::Instant,
    widget::{container, shader as shader_widget},
};
use spectrum_theme::{Color, iced::IcedColorAdapter};

use crate::context::ViewCx;

use super::{
    SpinnerAppearance, SpinnerCadence, SpinnerPlayback, SpinnerSample, SpinnerTimeline,
    shader::SpinnerShader,
};

/// Stateful indeterminate spinner using an independent adwaita-like visual profile.
///
/// This component is not produced, affiliated with, or endorsed by GNOME or
/// libadwaita. Its cadence remains active when reduced motion is enabled because
/// it communicates an ongoing operation rather than decorative feedback.
#[derive(Debug)]
pub struct Spinner {
    size: Option<f32>,
    foreground: Option<Color>,
    track: Option<Color>,
    cadence: SpinnerCadence,
    timeline: SpinnerTimeline,
}

impl Spinner {
    /// Creates a running spinner that begins sampling on its first frame tick.
    #[must_use]
    pub fn new() -> Self {
        Self {
            size: None,
            foreground: None,
            track: None,
            cadence: SpinnerCadence::default(),
            timeline: SpinnerTimeline::new(),
        }
    }

    /// Sets the requested square allocation size in logical pixels.
    #[must_use]
    pub const fn size(mut self, size: f32) -> Self {
        self.size = Some(size);
        self
    }

    /// Sets the requested square allocation size in logical pixels.
    pub fn set_size(&mut self, size: f32) {
        self.size = Some(size);
    }

    /// Clears the instance size override.
    pub fn clear_size(&mut self) {
        self.size = None;
    }

    /// Returns the explicit square size override.
    #[must_use]
    pub const fn explicit_size(&self) -> Option<f32> {
        self.size
    }

    /// Returns this spinner with an arc color override.
    #[must_use]
    pub const fn foreground(mut self, color: Color) -> Self {
        self.foreground = Some(color);
        self
    }

    /// Sets an instance-level arc color override.
    pub fn set_foreground(&mut self, color: Color) {
        self.foreground = Some(color);
    }

    /// Clears the instance-level arc color override.
    pub fn clear_foreground(&mut self) {
        self.foreground = None;
    }

    /// Returns the instance-level arc color override.
    #[must_use]
    pub const fn foreground_override(&self) -> Option<Color> {
        self.foreground
    }

    /// Returns this spinner with a track color override.
    #[must_use]
    pub const fn track(mut self, color: Color) -> Self {
        self.track = Some(color);
        self
    }

    /// Sets an instance-level track color override.
    pub fn set_track(&mut self, color: Color) {
        self.track = Some(color);
    }

    /// Clears the instance-level track color override.
    pub fn clear_track(&mut self) {
        self.track = None;
    }

    /// Returns the instance-level track color override.
    #[must_use]
    pub const fn track_override(&self) -> Option<Color> {
        self.track
    }

    /// Returns this spinner with a custom indeterminate cadence.
    #[must_use]
    pub const fn cadence(mut self, cadence: SpinnerCadence) -> Self {
        self.cadence = cadence;
        self
    }

    /// Replaces the cadence used to sample the active arc.
    pub fn set_cadence(&mut self, cadence: SpinnerCadence) {
        self.cadence = cadence;
    }

    /// Returns the configured indeterminate cadence.
    #[must_use]
    pub const fn cadence_config(&self) -> SpinnerCadence {
        self.cadence
    }

    /// Starts a fresh spinner timeline.
    pub fn start(&mut self) {
        self.timeline.start();
        trace_playback("start", self.timeline.playback());
    }

    /// Stops the spinner and clears its elapsed time.
    pub fn stop(&mut self) {
        self.timeline.stop();
        trace_playback("stop", self.timeline.playback());
    }

    /// Pauses the timeline at `now` without rewinding its sample.
    pub fn pause(&mut self, now: Instant) -> bool {
        let changed = self.timeline.pause(now);
        if changed {
            trace_playback("pause", self.timeline.playback());
        }
        changed
    }

    /// Resumes a paused timeline without rewinding its sample.
    pub fn resume(&mut self) -> bool {
        let changed = self.timeline.resume();
        if changed {
            trace_playback("resume", self.timeline.playback());
        }
        changed
    }

    /// Returns the current playback state.
    #[must_use]
    pub const fn playback(&self) -> SpinnerPlayback {
        self.timeline.playback()
    }

    /// Returns whether the spinner is accepting frame ticks.
    #[must_use]
    pub const fn is_running(&self) -> bool {
        matches!(self.playback(), SpinnerPlayback::Running)
    }

    /// Advances the timeline to `now` and reports whether its sample changed.
    pub fn advance(&mut self, now: Instant) -> bool {
        self.timeline.advance(now)
    }

    /// Restarts the current playback state from its first cadence sample.
    pub fn reset(&mut self) {
        self.timeline.reset();
        #[cfg(feature = "tracing")]
        tracing::debug!(target: "iced_adwaita::spinner", "spinner timeline reset");
    }

    /// Returns the current timeline sample consumed by the shader.
    #[must_use]
    pub fn sample(&self) -> SpinnerSample {
        self.timeline.sample(self.cadence)
    }

    /// Returns current resolved visual inputs.
    #[must_use]
    pub fn appearance(&self, context: &ViewCx<'_>) -> SpinnerAppearance {
        let tokens = context.theme().pack().spinner;
        let (foreground, track) =
            context
                .style_transition()
                .map_or((tokens.foreground, tokens.track), |transition| {
                    let progress = InterpolationProgress::new(transition.progress);
                    (
                        crate::theme::interpolate::color(
                            transition.from.spinner.foreground,
                            tokens.foreground,
                            progress,
                        ),
                        crate::theme::interpolate::color(
                            transition.from.spinner.track,
                            tokens.track,
                            progress,
                        ),
                    )
                });

        SpinnerAppearance::from_tokens(
            tokens,
            self.size,
            self.foreground.or(Some(foreground)),
            self.track.or(Some(track)),
        )
    }

    /// Builds the Iced view.
    #[must_use]
    pub fn view<Message>(&self, context: &ViewCx<'_>) -> Element<'static, Message>
    where
        Message: 'static,
    {
        SpinnerRender::new(self.sample(), self.appearance(context)).into()
    }
}

impl Default for Spinner {
    fn default() -> Self {
        Self::new()
    }
}

/// Shader-rendered spinner arc.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpinnerRender {
    sample: SpinnerSample,
    appearance: SpinnerAppearance,
}

impl SpinnerRender {
    /// Creates a render primitive from one timeline sample and visual appearance.
    #[must_use]
    pub const fn new(sample: SpinnerSample, appearance: SpinnerAppearance) -> Self {
        Self { sample, appearance }
    }
}

impl<Message> From<SpinnerRender> for Element<'_, Message>
where
    Message: 'static,
{
    fn from(render: SpinnerRender) -> Self {
        let size = Length::Fixed(render.appearance.allocation_size());

        container(
            shader_widget(SpinnerShader {
                sample: render.sample,
                diameter: render.appearance.diameter(),
                foreground: render.appearance.foreground.color(),
                track: render.appearance.track.color(),
                stroke_width: render.appearance.stroke_width(),
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

#[cfg(feature = "tracing")]
fn trace_playback(action: &'static str, playback: SpinnerPlayback) {
    tracing::debug!(target: "iced_adwaita::spinner", action, ?playback, "spinner playback changed");
}

#[cfg(not(feature = "tracing"))]
fn trace_playback(_action: &'static str, _playback: SpinnerPlayback) {}

#[cfg(test)]
mod tests {
    use iced::time::{Duration, Instant};
    use iced_component_core::anim::MotionRuntime;
    use spectrum_theme::Color;

    use crate::{
        Context,
        context::{UpdateCx, ViewCx},
    };

    use super::{Spinner, SpinnerPlayback};

    #[test]
    fn spinner_starts_from_the_initial_timeline_sample() {
        let spinner = Spinner::new();

        assert_eq!(spinner.playback(), SpinnerPlayback::Running);
        assert_eq!(spinner.sample(), spinner.sample());
    }

    #[test]
    fn spinner_pause_and_resume_preserve_the_current_sample() {
        let start = Instant::now();
        let mut spinner = Spinner::new();
        let _ = spinner.advance(start);
        let _ = spinner.advance(start + Duration::from_millis(200));
        let sample = spinner.sample();

        assert!(spinner.pause(start + Duration::from_millis(200)));
        assert!(!spinner.advance(start + Duration::from_secs(2)));
        assert_eq!(spinner.sample(), sample);
        assert!(spinner.resume());
    }

    #[test]
    fn builder_updates_visual_inputs() {
        let context = Context::light();
        let runtime = MotionRuntime::new();
        let view = ViewCx::new(&runtime, &context);
        let spinner = Spinner::new().size(48.0);

        assert_close(spinner.appearance(&view).size, 48.0);
        assert_close(spinner.appearance(&view).diameter(), 48.0);
    }

    #[test]
    fn mutable_overrides_can_be_applied_and_cleared() {
        let mut spinner = Spinner::new();
        let foreground = Color::new(1, 2, 3);
        let track = Color::new(4, 5, 6);

        spinner.set_size(64.0);
        spinner.set_foreground(foreground);
        spinner.set_track(track);
        assert_eq!(spinner.explicit_size(), Some(64.0));
        assert_eq!(spinner.foreground_override(), Some(foreground));
        assert_eq!(spinner.track_override(), Some(track));

        spinner.clear_size();
        spinner.clear_foreground();
        spinner.clear_track();
        assert_eq!(spinner.explicit_size(), None);
        assert_eq!(spinner.foreground_override(), None);
        assert_eq!(spinner.track_override(), None);
    }

    #[test]
    fn appearance_interpolates_theme_colors() {
        let mut context = Context::light();
        let mut runtime = MotionRuntime::new();
        let initial = Spinner::new().appearance(&ViewCx::new(&runtime, &context));

        UpdateCx::new(&mut runtime, &mut context).patch_theme(|theme| {
            theme.spinner.foreground = Color::new(1, 2, 3);
            theme.spinner.track = Color::new(4, 5, 6);
        });

        let view = ViewCx::new(&runtime, &context);
        assert_eq!(
            Spinner::new().appearance(&view).foreground.rgba(),
            initial.foreground.rgba()
        );

        runtime.tick(aura_anim::prelude::Duration::from_millis(200.0));
        let view = ViewCx::new(&runtime, &context);
        let appearance = Spinner::new().appearance(&view);

        assert_eq!(appearance.foreground.rgba(), Color::new(1, 2, 3).rgba());
        assert_eq!(appearance.track.rgba(), Color::new(4, 5, 6).rgba());
    }

    fn assert_close(actual: f32, expected: f32) {
        assert!((actual - expected).abs() < 0.001);
    }
}
