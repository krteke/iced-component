//! Stateful Material button primitives without Iced content rendering.

mod motion;
mod state;
mod style;
#[cfg(test)]
mod tests;
mod view;

use aura_anim::prelude::{Timing, Tween};
use iced_component_core::{
    anim::{MotionError, MotionRuntime},
    component::{MotionSlot, button::ButtonInteractionState},
};

use crate::context::{Context, UpdateCx, ViewCx};

pub use iced_component_core::component::button::{ButtonEvent, ButtonSignal, ButtonSync};
pub use motion::{ButtonMotion, ButtonVisual};
use state::ButtonStateExt as _;
pub use style::{ButtonSnapshot, ButtonStyleState, ButtonVariant};
pub use view::ButtonView;

/// Stateful Material button core without Iced content rendering.
#[derive(Debug)]
pub struct Button {
    variant: ButtonVariant,
    state: ButtonInteractionState,
    motion: MotionSlot<ButtonMotion>,
}

impl Button {
    /// Creates an elevated Material button.
    #[must_use]
    pub const fn new() -> Self {
        Self::with_variant(ButtonVariant::ELEVATED)
    }

    /// Creates an elevated Material button.
    #[must_use]
    pub const fn elevated() -> Self {
        Self::new()
    }

    /// Creates a filled Material button.
    #[must_use]
    pub const fn filled() -> Self {
        Self::with_variant(ButtonVariant::FILLED)
    }

    /// Creates a filled tonal Material button.
    #[must_use]
    pub const fn filled_tonal() -> Self {
        Self::with_variant(ButtonVariant::FILLED_TONAL)
    }

    /// Creates an outlined Material button.
    #[must_use]
    pub const fn outlined() -> Self {
        Self::with_variant(ButtonVariant::OUTLINED)
    }

    /// Creates a text Material button.
    #[must_use]
    pub const fn text() -> Self {
        Self::with_variant(ButtonVariant::TEXT)
    }

    /// Creates a button with an explicit Material variant.
    #[must_use]
    pub const fn with_variant(variant: ButtonVariant) -> Self {
        Self {
            variant,
            state: ButtonInteractionState::new(),
            motion: MotionSlot::new(),
        }
    }

    /// Replaces the Material button variant.
    pub fn set_variant(&mut self, variant: ButtonVariant) {
        self.variant = variant;
    }

    /// Returns this button with a disabled initial state.
    #[must_use]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.state.apply(ButtonSignal::SetDisabled(disabled));
        self
    }

    /// Registers the visual motion slot with the application runtime.
    pub fn register(&mut self, cx: &mut UpdateCx<'_>) {
        let initial = self.motion_from_context(cx.context());
        let revision = cx.style_revision();
        let core = cx.core();

        let _ = self.motion.register(core.runtime, initial, revision);
    }

    /// Synchronizes the current visual target to the active theme pack.
    pub fn sync(&mut self, cx: &mut UpdateCx<'_>) -> Result<bool, MotionError> {
        self.sync_with(ButtonSync::Manual, cx)
    }

    /// Synchronizes the current visual target with an explicit reason.
    pub fn sync_with(
        &mut self,
        _sync: ButtonSync,
        cx: &mut UpdateCx<'_>,
    ) -> Result<bool, MotionError> {
        if !self.motion.is_registered() {
            return Ok(false);
        }

        let target = self.motion_from_context(cx.context());
        let initial = {
            let core = cx.core();
            self.motion.value(core.runtime)?.copied()
        }
        .unwrap_or(target);

        self.play(initial, target, state::sync_timing(), cx)
    }

    /// Applies one button signal and updates the registered visual motion.
    pub fn update(
        &mut self,
        signal: ButtonSignal,
        cx: &mut UpdateCx<'_>,
    ) -> Result<bool, MotionError> {
        if let ButtonSignal::Sync(sync) = signal {
            return self.sync_with(sync, cx);
        }

        let previous = self.state;
        self.state.apply(signal);
        self.animate_from(previous, state::interaction_timing(), cx)
    }

    /// Enables or disables this button.
    pub fn set_disabled(
        &mut self,
        disabled: bool,
        cx: &mut UpdateCx<'_>,
    ) -> Result<bool, MotionError> {
        self.update(ButtonSignal::SetDisabled(disabled), cx)
    }

    /// Applies an event and returns its application action, if any.
    pub fn update_event<Action>(
        &mut self,
        event: ButtonEvent<Action>,
        cx: &mut UpdateCx<'_>,
    ) -> Result<Option<Action>, MotionError> {
        let previous = self.state;
        let signal = match &event {
            ButtonEvent::Signal(signal) => *signal,
            ButtonEvent::Pressed(_) => ButtonSignal::PressUp,
        };
        let action = self.state.apply_event(event);

        if let ButtonSignal::Sync(sync) = signal {
            self.sync_with(sync, cx)?;
        } else {
            self.animate_from(previous, state::interaction_timing(), cx)?;
        }

        Ok(action)
    }

    /// Returns a rendering snapshot without exposing component internals.
    pub fn snapshot(&self, cx: &ViewCx<'_>) -> Result<ButtonSnapshot, MotionError> {
        let revision = cx.style_revision();
        let motion = {
            let core = cx.core();
            self.motion
                .value_if_current(core.runtime, revision)?
                .copied()
        }
        .unwrap_or_else(|| self.motion_from_context(cx.context()));

        Ok(ButtonSnapshot {
            style_state: self.state.style_state(),
            visual: motion.visual,
            disabled: self.state.is_disabled(),
            focused: self.state.is_focused(),
        })
    }

    /// Returns the raw registered motion value, if registration occurred.
    pub fn motion_value(
        &self,
        runtime: &MotionRuntime,
    ) -> Result<Option<ButtonMotion>, MotionError> {
        Ok(self.motion.value(runtime)?.copied())
    }

    /// Returns the Material visual variant.
    #[must_use]
    pub const fn variant(&self) -> ButtonVariant {
        self.variant
    }

    /// Returns whether a motion slot has been registered.
    #[must_use]
    pub fn is_registered(&self) -> bool {
        self.motion.is_registered()
    }

    /// Returns whether this button is disabled.
    #[must_use]
    pub const fn is_disabled(&self) -> bool {
        self.state.is_disabled()
    }

    /// Returns the current interaction state.
    #[must_use]
    pub fn style_state(&self) -> ButtonStyleState {
        self.state.style_state()
    }

    fn animate_from(
        &mut self,
        previous: ButtonInteractionState,
        timing: Timing,
        cx: &mut UpdateCx<'_>,
    ) -> Result<bool, MotionError> {
        if !self.motion.is_registered() {
            return Ok(false);
        }

        let revision = cx.style_revision();
        let initial = {
            let core = cx.core();
            self.motion
                .value_if_current(core.runtime, revision)?
                .copied()
        }
        .unwrap_or_else(|| self.motion_from_state(cx.context(), previous));
        let target = self.motion_from_context(cx.context());

        self.play(initial, target, timing, cx)
    }

    fn play(
        &mut self,
        initial: ButtonMotion,
        target: ButtonMotion,
        timing: Timing,
        cx: &mut UpdateCx<'_>,
    ) -> Result<bool, MotionError> {
        let mut core = cx.core();

        self.motion
            .play(Tween::between(initial, target, timing), &mut core)
    }

    fn motion_from_context(&self, context: &Context) -> ButtonMotion {
        self.motion_from_state(context, self.state)
    }

    fn motion_from_state(&self, context: &Context, state: ButtonInteractionState) -> ButtonMotion {
        ButtonMotion::from_theme(context.theme().pack(), self.variant, state.style_state())
    }
}

impl Default for Button {
    fn default() -> Self {
        Self::new()
    }
}
