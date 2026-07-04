//! Animated loading spinner component.

mod motion;
mod shader;
#[cfg(test)]
mod tests;
mod view;

use aura_anim::prelude::{MotionError, MotionRuntime};

use crate::{
    component::{ComponentUpdateCx, ComponentViewCx, MotionSlot},
    theme::SpinnerComponentTokens,
};

pub use motion::{
    SpinnerAnimationBuilder, SpinnerAnimationProvider, SpinnerMotion, SpinnerMotionTransition,
    SpinnerMotionTrigger,
};
pub use view::SpinnerView;

/// Read-only spinner state consumed by rendering code.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpinnerSnapshot {
    /// Current animated motion values.
    pub motion: SpinnerMotion,
    /// Resolved spinner visual tokens.
    pub tokens: SpinnerComponentTokens,
    /// Resolved rendered size in pixels.
    pub size: f32,
    /// Resolved stroke width in pixels.
    pub stroke_width: f32,
    /// Whether this spinner should be actively rotating.
    pub spinning: bool,
}

/// Stateful animated loading spinner.
#[derive(Debug)]
pub struct Spinner {
    spinning: bool,
    size: Option<f32>,
    stroke_width: Option<f32>,
    motion: MotionSlot<SpinnerMotion>,
}

impl Spinner {
    /// Creates a spinner that starts when registered.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            spinning: true,
            size: None,
            stroke_width: None,
            motion: MotionSlot::new(),
        }
    }

    /// Creates a spinner that remains still until [`start`](Self::start).
    #[must_use]
    pub const fn stopped() -> Self {
        Self {
            spinning: false,
            size: None,
            stroke_width: None,
            motion: MotionSlot::new(),
        }
    }

    /// Returns this spinner with a different initial spinning state.
    #[must_use]
    pub const fn with_spinning(mut self, spinning: bool) -> Self {
        self.spinning = spinning;
        self
    }

    /// Returns this spinner with an explicit rendered size.
    #[must_use]
    pub const fn with_size(mut self, size: f32) -> Self {
        self.size = Some(size);
        self
    }

    /// Returns this spinner with an explicit base stroke width.
    ///
    /// The rendered stroke scales with the current size relative to the theme
    /// default size.
    #[must_use]
    pub const fn with_stroke_width(mut self, stroke_width: f32) -> Self {
        self.stroke_width = Some(stroke_width);
        self
    }

    /// Updates this spinner's explicit rendered size.
    pub fn set_size(&mut self, size: f32) {
        self.size = Some(size);
    }

    /// Clears this spinner's explicit rendered size.
    pub fn clear_size(&mut self) {
        self.size = None;
    }

    /// Updates this spinner's explicit base stroke width.
    ///
    /// The rendered stroke scales with the current size relative to the theme
    /// default size.
    pub fn set_stroke_width(&mut self, stroke_width: f32) {
        self.stroke_width = Some(stroke_width);
    }

    /// Clears this spinner's explicit base stroke width.
    pub fn clear_stroke_width(&mut self) {
        self.stroke_width = None;
    }

    /// Registers this spinner and starts it when configured as spinning.
    pub fn register(&mut self, cx: &mut ComponentUpdateCx<'_>) {
        let was_registered = self.motion.is_registered();
        let _ = self.motion.register(
            cx.runtime,
            SpinnerMotion::default(),
            cx.context().theme_revision(),
        );

        if self.spinning && !was_registered {
            let _ = self.play_motion(SpinnerMotionTrigger::Start, cx);
        }
    }

    /// Starts continuous spinning.
    pub fn start(&mut self, cx: &mut ComponentUpdateCx<'_>) -> Result<bool, MotionError> {
        if self.spinning && self.motion.is_registered() {
            return Ok(false);
        }

        self.spinning = true;
        self.play_motion(SpinnerMotionTrigger::Start, cx)
    }

    /// Stops spinning and keeps the current visual angle.
    pub fn stop(&mut self, cx: &mut ComponentUpdateCx<'_>) -> Result<bool, MotionError> {
        if !self.spinning && !self.motion.is_registered() {
            return Ok(false);
        }

        self.spinning = false;
        self.play_motion(SpinnerMotionTrigger::Stop, cx)
    }

    /// Updates whether this spinner is actively rotating.
    pub fn set_spinning(
        &mut self,
        spinning: bool,
        cx: &mut ComponentUpdateCx<'_>,
    ) -> Result<bool, MotionError> {
        if spinning {
            self.start(cx)
        } else {
            self.stop(cx)
        }
    }

    /// Synchronizes the runtime motion with the current spinning state.
    pub fn sync(&mut self, cx: &mut ComponentUpdateCx<'_>) -> Result<bool, MotionError> {
        if !self.motion.is_registered() {
            return Ok(false);
        }

        self.play_motion(
            if self.spinning {
                SpinnerMotionTrigger::Sync
            } else {
                SpinnerMotionTrigger::Stop
            },
            cx,
        )
    }

    /// Returns the raw runtime motion value, or `None` if not registered.
    pub fn motion_value(
        &self,
        runtime: &MotionRuntime,
    ) -> Result<Option<SpinnerMotion>, MotionError> {
        Ok(self.motion.value(runtime)?.copied())
    }

    /// Returns a rendering snapshot without exposing internal state.
    pub fn snapshot(&self, cx: &ComponentViewCx<'_>) -> Result<SpinnerSnapshot, MotionError> {
        let tokens = cx.context().theme().theme().spinner.regular;
        let size = self.size.unwrap_or_else(|| tokens.size.value());
        let base_stroke_width = self
            .stroke_width
            .unwrap_or_else(|| tokens.stroke_width.value());

        Ok(SpinnerSnapshot {
            motion: self.motion.value(cx.runtime)?.copied().unwrap_or_default(),
            tokens,
            size,
            stroke_width: scaled_stroke_width(base_stroke_width, size, tokens.size.value()),
            spinning: self.spinning,
        })
    }

    /// Returns whether this spinner is configured to rotate.
    #[must_use]
    pub const fn is_spinning(&self) -> bool {
        self.spinning
    }

    /// Returns the explicit rendered size override.
    #[must_use]
    pub const fn size(&self) -> Option<f32> {
        self.size
    }

    /// Returns the explicit base stroke width override.
    #[must_use]
    pub const fn stroke_width(&self) -> Option<f32> {
        self.stroke_width
    }

    fn play_motion(
        &mut self,
        trigger: SpinnerMotionTrigger,
        cx: &mut ComponentUpdateCx<'_>,
    ) -> Result<bool, MotionError> {
        let initial = self.motion.value(cx.runtime)?.copied().unwrap_or_default();
        let target = match trigger {
            SpinnerMotionTrigger::Start | SpinnerMotionTrigger::Sync => SpinnerMotion {
                rotation: initial.rotation + 360.0,
            },
            SpinnerMotionTrigger::Stop => initial,
        };

        let transition = SpinnerMotionTransition {
            from: initial,
            to: target,
            trigger,
        };
        let animation = cx.context().animation().spinner().build(&transition);

        self.motion.play_from(initial, animation, cx)
    }
}

impl Default for Spinner {
    fn default() -> Self {
        Self::new()
    }
}

fn scaled_stroke_width(base_stroke_width: f32, size: f32, base_size: f32) -> f32 {
    if base_size > 0.0 {
        base_stroke_width * size / base_size
    } else {
        base_stroke_width
    }
}
