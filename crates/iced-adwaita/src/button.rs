//! Stateful Adwaita button primitives.

mod animation;
mod content;
mod motion;
mod state;
mod style;
#[cfg(test)]
mod tests;
mod view;

use iced::Length;
use iced_component_core::{
    anim::{MotionError, MotionRuntime},
    component::MotionSlot,
};
use spectrum_theme::Color;

use crate::context::{Context, UpdateCx, ViewCx};

pub use animation::{ButtonAnimationBuilder, ButtonAnimations, adwaita_button_timing};
pub use content::ButtonContent;
pub use motion::{ButtonMotion, ButtonMotionTransition};
pub use state::{ButtonSignal, ButtonSync};
pub use style::{
    ButtonResolvedStyle, ButtonRole, ButtonShape, ButtonSnapshot, ButtonStyleOverride,
    ButtonStyleState, ButtonTreatment, ButtonVariant,
};
pub use view::ButtonView;

/// Component-level button event.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ButtonEvent<Action> {
    /// Component signal that changes visual state or synchronizes style.
    Signal(ButtonSignal),
    /// A completed press action.
    Pressed(Action),
}

/// Stateful Adwaita button core without Iced content rendering.
#[derive(Debug)]
pub struct Button {
    content: ButtonContent,
    variant: ButtonVariant,
    style_override: ButtonStyleOverride,
    layout: ButtonLayout,
    state: state::ButtonState,
    motion: MotionSlot<ButtonMotion>,
}

/// Stable layout inputs for an Adwaita button.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ButtonLayout {
    /// Explicit width override.
    pub width: Option<Length>,
    /// Explicit height override.
    pub height: Option<Length>,
    /// Explicit horizontal padding override.
    pub padding_x: Option<f32>,
    /// Explicit vertical padding override.
    pub padding_y: Option<f32>,
}

impl Button {
    /// Creates a standard Adwaita button core with an unregistered motion slot.
    #[must_use]
    pub fn new(content: impl Into<ButtonContent>) -> Self {
        Self {
            content: content.into(),
            variant: ButtonVariant::STANDARD,
            style_override: ButtonStyleOverride::default(),
            layout: ButtonLayout::default(),
            state: state::ButtonState::default(),
            motion: MotionSlot::new(),
        }
    }

    /// Creates an empty standard Adwaita button core.
    #[must_use]
    pub fn empty() -> Self {
        Self::new(ButtonContent::Empty)
    }

    /// Creates a suggested-action Adwaita button.
    #[must_use]
    pub fn suggested(content: impl Into<ButtonContent>) -> Self {
        Self::new(content).with_variant(ButtonVariant::SUGGESTED)
    }

    /// Creates a destructive-action Adwaita button.
    #[must_use]
    pub fn destructive(content: impl Into<ButtonContent>) -> Self {
        Self::new(content).with_variant(ButtonVariant::DESTRUCTIVE)
    }

    /// Returns this button with different stored content.
    #[must_use]
    pub fn with_content(mut self, content: impl Into<ButtonContent>) -> Self {
        self.content = content.into();
        self
    }

    /// Replaces this button's stored content.
    pub fn set_content(&mut self, content: impl Into<ButtonContent>) {
        self.content = content.into();
    }

    /// Returns this button with a different style variant.
    #[must_use]
    pub const fn with_variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Replaces this button's style variant.
    pub fn set_variant(&mut self, variant: ButtonVariant) {
        self.variant = variant;
    }

    /// Returns this button as a flat button.
    #[must_use]
    pub const fn flat(self) -> Self {
        self.with_treatment(ButtonTreatment::Flat)
    }

    /// Returns this button with pill shape.
    #[must_use]
    pub const fn pill(self) -> Self {
        self.with_shape(ButtonShape::Pill)
    }

    /// Returns this button with circular shape.
    #[must_use]
    pub const fn circular(self) -> Self {
        self.with_shape(ButtonShape::Circular)
    }

    /// Returns this button with a different semantic role.
    #[must_use]
    pub const fn with_role(mut self, role: ButtonRole) -> Self {
        self.variant = self.variant.with_role(role);
        self
    }

    /// Returns this button with a different treatment.
    #[must_use]
    pub const fn with_treatment(mut self, treatment: ButtonTreatment) -> Self {
        self.variant = self.variant.with_treatment(treatment);
        self
    }

    /// Returns this button with a different outline shape.
    #[must_use]
    pub const fn with_shape(mut self, shape: ButtonShape) -> Self {
        self.variant = self.variant.with_shape(shape);
        self
    }

    /// Returns this button with style overrides.
    #[must_use]
    pub const fn with_style_override(mut self, style_override: ButtonStyleOverride) -> Self {
        self.style_override = style_override;
        self
    }

    /// Replaces this button's style overrides.
    pub fn set_style_override(&mut self, style_override: ButtonStyleOverride) {
        self.style_override = style_override;
    }

    /// Returns this button with an explicit background override.
    #[must_use]
    pub const fn background(mut self, color: Color) -> Self {
        self.style_override.background = Some(color);
        self
    }

    /// Returns this button with an explicit foreground override.
    #[must_use]
    pub const fn foreground(mut self, color: Color) -> Self {
        self.style_override.foreground = Some(color);
        self
    }

    /// Returns this button with an explicit width.
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.layout.width = Some(width.into());
        self
    }

    /// Returns this button with an explicit height.
    #[must_use]
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.layout.height = Some(height.into());
        self
    }

    /// Returns this button with explicit padding.
    #[must_use]
    pub const fn padding(mut self, padding: [f32; 2]) -> Self {
        self.layout.padding_x = Some(padding[0]);
        self.layout.padding_y = Some(padding[1]);
        self
    }

    /// Returns this button with equal width and height.
    #[must_use]
    pub fn square(mut self, size: f32) -> Self {
        self.layout.width = Some(Length::Fixed(size));
        self.layout.height = Some(Length::Fixed(size));
        self.layout.padding_x = Some(0.0);
        self.layout.padding_y = Some(0.0);
        self
    }

    /// Returns this button with a disabled initial state.
    #[must_use]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.state.apply(ButtonSignal::SetDisabled(disabled));
        self
    }

    /// Registers the button motion handle using the current Adwaita context.
    pub fn register(&mut self, cx: &mut UpdateCx<'_>) {
        if self.motion.is_registered() {
            #[cfg(feature = "tracing")]
            tracing::debug!(
                target: "iced_adwaita::button",
                op = "register_skip",
                state = ?self.state.style_state(),
                motion_id = ?self.motion.motion().map(iced_component_core::anim::Motion::motion_id),
                "button"
            );
            return;
        }

        let initial = self.motion_from_context(cx.context());
        let style_revision = cx.style_revision();
        let core = cx.core();

        let motion = self.motion.register(core.runtime, initial, style_revision);
        #[cfg(not(feature = "tracing"))]
        let _ = motion;
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: "iced_adwaita::button",
            op = "register",
            state = ?self.state.style_state(),
            motion_id = ?motion.motion_id(),
            ?style_revision,
            "button"
        );
    }

    /// Synchronizes this button's motion target with the current style.
    pub fn sync(&mut self, cx: &mut UpdateCx<'_>) -> Result<bool, MotionError> {
        self.sync_with(ButtonSync::Manual, cx)
    }

    /// Synchronizes this button's motion target with an explicit sync reason.
    pub fn sync_with(
        &mut self,
        sync: ButtonSync,
        cx: &mut UpdateCx<'_>,
    ) -> Result<bool, MotionError> {
        if !self.motion.is_registered() {
            #[cfg(feature = "tracing")]
            tracing::debug!(
                target: "iced_adwaita::button",
                op = "sync_skip",
                state = ?self.state.style_state(),
                "button"
            );
            return Ok(false);
        }

        let changed = self.animate_sync(sync, cx)?;
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: "iced_adwaita::button",
            op = "sync",
            state = ?self.state.style_state(),
            changed,
            "button"
        );
        Ok(changed)
    }

    /// Applies one button signal and updates the runtime motion target.
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
        let changed = self.animate_from_state(previous, signal, cx)?;
        trace_button_state_change("update", previous, self.state, signal, changed);

        Ok(changed)
    }

    /// Enables or disables this button and updates its runtime motion target.
    pub fn set_disabled(
        &mut self,
        disabled: bool,
        cx: &mut UpdateCx<'_>,
    ) -> Result<bool, MotionError> {
        self.update(ButtonSignal::SetDisabled(disabled), cx)
    }

    /// Applies a button event and returns its application action, if any.
    pub fn update_event<Action>(
        &mut self,
        event: ButtonEvent<Action>,
        cx: &mut UpdateCx<'_>,
    ) -> Result<Option<Action>, MotionError> {
        let previous = self.state;
        #[cfg(feature = "tracing")]
        let event_name = button_event_name(&event);
        let signal = match &event {
            ButtonEvent::Signal(signal) => *signal,
            ButtonEvent::Pressed(_) => ButtonSignal::PressUp,
        };
        let action = self.state.apply_event(event);

        let changed = match signal {
            ButtonSignal::Sync(sync) => self.sync_with(sync, cx)?,
            _ => self.animate_from_state(previous, signal, cx)?,
        };
        #[cfg(not(feature = "tracing"))]
        let _ = changed;
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: "iced_adwaita::button",
            event = event_name,
            from = ?previous.style_state(),
            to = ?self.state.style_state(),
            changed,
            action_emitted = action.is_some(),
            "button"
        );

        Ok(action)
    }

    /// Returns the raw runtime motion value, or `None` if not registered.
    pub fn motion_value(
        &self,
        runtime: &MotionRuntime,
    ) -> Result<Option<ButtonMotion>, MotionError> {
        Ok(self.motion.value(runtime)?.copied())
    }

    /// Returns a rendering snapshot without exposing internal state.
    pub fn snapshot(&self, cx: &ViewCx<'_>) -> Result<ButtonSnapshot, MotionError> {
        let style_state = self.state.style_state();
        let motion = self.motion_value_for_context(cx)?;

        Ok(ButtonSnapshot {
            style_state,
            style: ButtonResolvedStyle::from_tokens(motion.tokens),
            motion,
            disabled: self.state.is_disabled(),
            focused: self.state.is_focused(),
        })
    }

    /// Returns this button's stored content.
    #[must_use]
    pub const fn content(&self) -> &ButtonContent {
        &self.content
    }

    /// Returns this button's style variant.
    #[must_use]
    pub const fn variant(&self) -> ButtonVariant {
        self.variant
    }

    /// Returns this button's style override.
    #[must_use]
    pub const fn style_override(&self) -> ButtonStyleOverride {
        self.style_override
    }

    /// Returns this button's layout inputs.
    #[must_use]
    pub const fn layout(&self) -> ButtonLayout {
        self.layout
    }

    /// Returns whether this button has registered its motion slot.
    #[must_use]
    pub fn is_registered(&self) -> bool {
        self.motion.is_registered()
    }

    /// Returns whether this button is disabled.
    #[must_use]
    pub const fn is_disabled(&self) -> bool {
        self.state.is_disabled()
    }

    /// Returns whether this button has focus.
    #[must_use]
    pub const fn is_focused(&self) -> bool {
        self.state.is_focused()
    }

    /// Returns the current style state.
    #[must_use]
    pub const fn style_state(&self) -> ButtonStyleState {
        self.state.style_state()
    }

    fn motion_value_for_context(&self, cx: &ViewCx<'_>) -> Result<ButtonMotion, MotionError> {
        Ok(self
            .motion
            .value_if_current(cx.runtime(), cx.style_revision())?
            .copied()
            .unwrap_or_else(|| self.motion_from_context(cx.context())))
    }

    fn motion_from_context(&self, context: &Context) -> ButtonMotion {
        self.motion_from_state(context, self.state)
    }

    pub(crate) fn resolved_layout(
        &self,
        theme: &crate::theme::ThemePack,
    ) -> (Option<Length>, Option<Length>, f32, f32) {
        let default_padding_x = theme.button.padding_x.value();
        let default_padding_y = theme.button.padding_y.value();
        let default_text_height = theme.button.min_height.value() + default_padding_y * 2.0;

        let (width, height, padding_x, padding_y) = match self.variant.shape {
            ButtonShape::Rounded => (
                None,
                Some(Length::Fixed(default_text_height)),
                default_padding_x,
                default_padding_y,
            ),
            ButtonShape::Pill => (None, None, 32.0, 10.0),
            ButtonShape::Circular => (
                Some(Length::Fixed(34.0)),
                Some(Length::Fixed(34.0)),
                0.0,
                0.0,
            ),
        };

        (
            self.layout.width.or(width),
            self.layout.height.or(height),
            self.layout.padding_x.unwrap_or(padding_x),
            self.layout.padding_y.unwrap_or(padding_y),
        )
    }

    fn motion_from_state(&self, context: &Context, state: state::ButtonState) -> ButtonMotion {
        ButtonMotion::from_theme(
            context.theme().pack(),
            self.variant,
            self.style_override,
            state.style_state(),
            state.is_focused(),
        )
    }

    fn animate_from_state(
        &mut self,
        previous: state::ButtonState,
        signal: ButtonSignal,
        cx: &mut UpdateCx<'_>,
    ) -> Result<bool, MotionError> {
        if !self.motion.is_registered() {
            return Ok(false);
        }

        let initial = self
            .motion
            .value_if_current(cx.runtime(), cx.style_revision())?
            .copied()
            .unwrap_or_else(|| self.motion_from_state(cx.context(), previous));
        let target = self.motion_from_context(cx.context());

        self.play_motion(
            ButtonMotionTransition {
                from: initial,
                to: target,
                signal,
            },
            cx,
        )
    }

    fn animate_sync(
        &mut self,
        sync: ButtonSync,
        cx: &mut UpdateCx<'_>,
    ) -> Result<bool, MotionError> {
        let initial = self
            .motion
            .value(cx.runtime())?
            .copied()
            .unwrap_or_else(|| self.motion_from_context(cx.context()));
        let target = self.motion_from_context(cx.context());

        self.play_motion(
            ButtonMotionTransition {
                from: initial,
                to: target,
                signal: ButtonSignal::Sync(sync),
            },
            cx,
        )
    }

    fn play_motion(
        &mut self,
        transition: ButtonMotionTransition,
        cx: &mut UpdateCx<'_>,
    ) -> Result<bool, MotionError> {
        let signal = transition.signal;
        let animation = cx.button_animations().build(transition);
        let mut core = cx.core();

        let changed = self.motion.play(animation, &mut core)?;
        #[cfg(not(feature = "tracing"))]
        let _ = signal;
        #[cfg(feature = "tracing")]
        tracing::trace!(
            target: "iced_adwaita::button",
            op = "motion",
            state = ?self.state.style_state(),
            ?signal,
            changed,
            "button"
        );

        Ok(changed)
    }
}

impl Default for Button {
    fn default() -> Self {
        Self::empty()
    }
}

#[cfg(feature = "tracing")]
fn button_event_name<Action>(event: &ButtonEvent<Action>) -> &'static str {
    match event {
        ButtonEvent::Signal(ButtonSignal::HoverEnter) => "hover_enter",
        ButtonEvent::Signal(ButtonSignal::HoverExit) => "hover_exit",
        ButtonEvent::Signal(ButtonSignal::PressDown) => "press_down",
        ButtonEvent::Signal(ButtonSignal::PressUp) => "press_up",
        ButtonEvent::Signal(ButtonSignal::Focus) => "focus",
        ButtonEvent::Signal(ButtonSignal::Blur) => "blur",
        ButtonEvent::Signal(ButtonSignal::SetDisabled(true)) => "disable",
        ButtonEvent::Signal(ButtonSignal::SetDisabled(false)) => "enable",
        ButtonEvent::Signal(ButtonSignal::Sync(_)) => "sync",
        ButtonEvent::Pressed(_) => "pressed",
    }
}

#[cfg(feature = "tracing")]
fn trace_button_state_change(
    action: &'static str,
    previous: state::ButtonState,
    current: state::ButtonState,
    signal: ButtonSignal,
    changed: bool,
) {
    tracing::debug!(
        target: "iced_adwaita::button",
        action,
        from = ?previous.style_state(),
        to = ?current.style_state(),
        ?signal,
        changed,
        "button"
    );
}

#[cfg(not(feature = "tracing"))]
fn trace_button_state_change(
    _action: &'static str,
    _previous: state::ButtonState,
    _current: state::ButtonState,
    _signal: ButtonSignal,
    _changed: bool,
) {
}
