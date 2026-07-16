//! Material 3 expressive loading indicator.

mod animation;
mod config;
mod geometry;
mod motion;
mod style;
#[cfg(test)]
mod tests;
mod timeline;
mod view;

use aura_anim::prelude::Tween;
use iced::time::Instant;
use iced_component_core::{
    anim::{MotionError, MotionRuntime},
    component::MotionSlot,
};

use crate::context::{Context, UpdateCx, ViewCx};

pub use animation::LoadingIndicatorAnimations;
pub use motion::LoadingIndicatorMotion;
pub use style::{LoadingIndicatorSnapshot, LoadingIndicatorStyle, LoadingIndicatorVisual};
use timeline::LoadingTimeline;

/// Progress mode rendered by a Material loading indicator.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum LoadingIndicatorMode {
    /// Continuously morph through the Material expressive shape sequence.
    #[default]
    Indeterminate,
    /// Morph from a circle toward the determinate completion shape.
    Determinate(f32),
}

/// Stateful Material 3 expressive loading indicator.
#[derive(Debug)]
pub struct LoadingIndicator {
    mode: LoadingIndicatorMode,
    contained: bool,
    size: Option<f32>,
    style: LoadingIndicatorStyle,
    timeline: LoadingTimeline,
    motion: MotionSlot<LoadingIndicatorMotion>,
}

impl LoadingIndicator {
    /// Creates an uncontained indeterminate loading indicator.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            mode: LoadingIndicatorMode::Indeterminate,
            contained: false,
            size: None,
            style: LoadingIndicatorStyle::new(),
            timeline: LoadingTimeline::new(),
            motion: MotionSlot::new(),
        }
    }

    /// Creates a determinate loading indicator.
    #[must_use]
    pub const fn determinate(progress: f32) -> Self {
        Self::new().with_mode(LoadingIndicatorMode::Determinate(progress))
    }

    /// Registers visual motion and precomputes Material shape morphs.
    pub fn register(&mut self, cx: &mut UpdateCx<'_>) {
        geometry::prepare();
        let initial = Self::motion_from_context(cx.context());
        let revision = cx.style_revision();
        let core = cx.core();
        let _ = self.motion.register(core.runtime, initial, revision);
    }

    /// Animates theme-resolved colors to the current theme revision.
    pub fn sync(&mut self, cx: &mut UpdateCx<'_>) -> Result<bool, MotionError> {
        if !self.motion.is_registered() {
            return Ok(false);
        }

        let target = Self::motion_from_context(cx.context());
        let initial = {
            let core = cx.core();
            self.motion.value(core.runtime)?.copied()
        }
        .unwrap_or(target);
        let animations = LoadingIndicatorAnimations::resolve(cx.animation_overrides());
        let mut core = cx.core();

        self.motion.play(
            Tween::between(initial, target, animations.theme_change),
            &mut core,
        )
    }

    /// Advances the indeterminate timeline to `now`.
    pub fn advance(&mut self, now: Instant) {
        if matches!(self.mode, LoadingIndicatorMode::Indeterminate) {
            self.timeline.advance(now);
        }
    }

    /// Restarts the indeterminate timeline from the next frame.
    pub fn reset(&mut self) {
        self.timeline.reset();
    }

    /// Returns the sampled indeterminate phase in the range `0..1`.
    #[must_use]
    pub fn phase(&self) -> f32 {
        self.timeline.phase()
    }

    /// Resolves the current theme and instance inputs for rendering.
    pub fn snapshot(&self, cx: &ViewCx<'_>) -> Result<LoadingIndicatorSnapshot, MotionError> {
        LoadingIndicatorSnapshot::resolve(self, cx)
    }

    /// Returns whether visual motion has been explicitly registered.
    #[must_use]
    pub fn is_registered(&self) -> bool {
        self.motion.is_registered()
    }

    /// Returns the raw registered visual motion value.
    pub fn motion_value(
        &self,
        runtime: &MotionRuntime,
    ) -> Result<Option<LoadingIndicatorMotion>, MotionError> {
        Ok(self.motion.value(runtime)?.copied())
    }

    pub(crate) fn motion_for_view(
        &self,
        cx: &ViewCx<'_>,
    ) -> Result<LoadingIndicatorMotion, MotionError> {
        let target = Self::motion_from_context(cx.context());

        Ok(self
            .motion
            .value_if_current(cx.core().runtime, cx.style_revision())?
            .copied()
            .unwrap_or(target))
    }

    fn motion_from_context(context: &Context) -> LoadingIndicatorMotion {
        LoadingIndicatorMotion::from_tokens(context.theme().pack().loading_indicator)
    }
}

impl Default for LoadingIndicator {
    fn default() -> Self {
        Self::new()
    }
}
