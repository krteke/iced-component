//! Button style and state primitives.

mod animated;
mod content;
mod icon;
mod motion;
mod state;
mod style;
#[cfg(test)]
mod tests;
mod view;

use aura_anim_core::{MotionError, MotionRuntime, timing::Timing};
use iced::Length;

use crate::{
    button::state::ButtonState,
    component::{ComponentContext, ComponentMotion},
};

pub use animated::{ButtonEvent, ButtonInteraction, ButtonSnapshot};
pub use content::{ButtonContent, ButtonLayout};
pub use icon::{IconButton, IconButtonSize, IconSource};
pub use motion::ButtonMotion;
pub use style::{
    ButtonResolvedStyle, ButtonRole, ButtonShape, ButtonStyleState, ButtonTreatment, ButtonVariant,
};
pub use view::{ButtonView, button_style};

/// Stateful animated button core without Iced rendering.
#[derive(Debug)]
pub struct Button {
    content: ButtonContent,
    layout: ButtonLayout,
    variant: ButtonVariant,
    state: ButtonState,
    motion: ComponentMotion<ButtonMotion>,
}

impl Button {
    /// Creates a button core with an unregistered motion handle.
    #[must_use]
    pub fn new(content: impl Into<ButtonContent>, variant: ButtonVariant) -> Self {
        Self {
            content: content.into(),
            layout: ButtonLayout::default(),
            variant,
            state: ButtonState::default(),
            motion: ComponentMotion::new(ButtonMotion::idle(), Timing::default()),
        }
    }

    /// Creates a button core without default content.
    #[must_use]
    pub fn empty(variant: ButtonVariant) -> Self {
        Self::new(ButtonContent::Empty, variant)
    }

    /// Creates a standard animated button.
    #[must_use]
    pub fn standard(content: impl Into<ButtonContent>) -> Self {
        Self::new(content, ButtonVariant::STANDARD)
    }

    /// Creates a suggested-action animated button.
    #[must_use]
    pub fn suggested(content: impl Into<ButtonContent>) -> Self {
        Self::new(content, ButtonVariant::SUGGESTED)
    }

    /// Creates a suggested-action animated button.
    #[must_use]
    pub fn primary(content: impl Into<ButtonContent>) -> Self {
        Self::suggested(content)
    }

    /// Creates a destructive-action animated button.
    #[must_use]
    pub fn destructive(content: impl Into<ButtonContent>) -> Self {
        Self::new(content, ButtonVariant::DESTRUCTIVE)
    }

    /// Returns this button with different stable content.
    #[must_use]
    pub fn with_content(mut self, content: impl Into<ButtonContent>) -> Self {
        self.content = content.into();
        self
    }

    /// Replaces this button's stable content.
    pub fn set_content(&mut self, content: impl Into<ButtonContent>) {
        self.content = content.into();
    }

    /// Returns this button with a different visual variant.
    #[must_use]
    pub fn with_variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Returns this button with a different semantic role.
    #[must_use]
    pub fn with_role(mut self, role: ButtonRole) -> Self {
        self.variant = self.variant.with_role(role);
        self
    }

    /// Returns this button with a different visual treatment.
    #[must_use]
    pub fn with_treatment(mut self, treatment: ButtonTreatment) -> Self {
        self.variant = self.variant.with_treatment(treatment);
        self
    }

    /// Returns this button with a different outline shape.
    #[must_use]
    pub fn with_shape(mut self, shape: ButtonShape) -> Self {
        self.variant = self.variant.with_shape(shape);
        self
    }

    /// Returns this button as a standard action.
    #[must_use]
    pub fn as_standard(self) -> Self {
        self.with_role(ButtonRole::Standard)
    }

    /// Returns this button as a suggested action.
    #[must_use]
    pub fn as_suggested(self) -> Self {
        self.with_role(ButtonRole::Suggested)
    }

    /// Returns this button as a destructive action.
    #[must_use]
    pub fn as_destructive(self) -> Self {
        self.with_role(ButtonRole::Destructive)
    }

    /// Returns this button with filled treatment.
    #[must_use]
    pub fn filled(self) -> Self {
        self.with_treatment(ButtonTreatment::Filled)
    }

    /// Returns this button with rounded shape.
    #[must_use]
    pub fn rounded(self) -> Self {
        self.with_shape(ButtonShape::Rounded)
    }

    /// Returns this button with minimal low-emphasis styling.
    #[must_use]
    pub fn flat(self) -> Self {
        self.with_treatment(ButtonTreatment::Flat)
    }

    /// Returns this button with explicit raised styling.
    #[must_use]
    pub fn raised(self) -> Self {
        self.with_treatment(ButtonTreatment::Raised)
    }

    /// Returns this button with fully rounded capsule styling.
    #[must_use]
    pub fn pill(self) -> Self {
        self.with_shape(ButtonShape::Pill)
    }

    /// Returns this button with circular styling.
    ///
    /// Pair this with view sizing such as `square(34.0)` for icon-style
    /// buttons.
    #[must_use]
    pub fn circular(self) -> Self {
        self.with_shape(ButtonShape::Circular)
    }

    /// Returns this button with explicit padding.
    #[must_use]
    pub const fn padding(mut self, padding: [f32; 2]) -> Self {
        self.layout.padding = Some(padding);
        self
    }

    /// Returns this button with a fixed rendered width.
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.layout.width = Some(width.into());
        self
    }

    /// Returns this button with a fixed rendered height.
    #[must_use]
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.layout.height = Some(height.into());
        self
    }

    /// Returns this button with equal width and height.
    #[must_use]
    pub fn square(mut self, size: f32) -> Self {
        self.layout.width = Some(Length::Fixed(size));
        self.layout.height = Some(Length::Fixed(size));
        self.layout.padding = Some([0.0, 0.0]);
        self.layout.center_content = true;
        self
    }

    /// Returns this button with disabled state preconfigured.
    #[must_use]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.state.apply(ButtonInteraction::SetDisabled(disabled));
        self
    }

    /// Registers the button motion handle in the application runtime.
    pub fn register(&mut self, runtime: &mut MotionRuntime, context: &ComponentContext) {
        if self.motion.is_registered() {
            return;
        }

        let motion_tokens = context.motion_tokens();
        let timing = motion_tokens.timing(motion_tokens.interaction, context.motion_preferences());
        self.motion = ComponentMotion::new(self.target_motion(), timing);
        let _ = self.motion.register(runtime);
    }

    /// Applies a button interaction and transitions motion when registered.
    pub fn update(
        &mut self,
        interaction: ButtonInteraction,
        runtime: &mut MotionRuntime,
    ) -> Result<bool, MotionError> {
        self.state.apply(interaction);
        self.motion.transition_to(self.target_motion(), runtime)
    }

    /// Enables or disables this button and updates its motion target.
    pub fn set_disabled(
        &mut self,
        disabled: bool,
        runtime: &mut MotionRuntime,
    ) -> Result<bool, MotionError> {
        self.update(ButtonInteraction::SetDisabled(disabled), runtime)
    }

    /// Applies a button event and returns its application action, if any.
    pub fn update_event<Action>(
        &mut self,
        event: ButtonEvent<Action>,
        runtime: &mut MotionRuntime,
    ) -> Result<Option<Action>, MotionError> {
        let action = self.state.apply_event(event);
        self.motion.transition_to(self.target_motion(), runtime)?;
        Ok(action)
    }

    /// Applies a button event and invokes `on_action` when release yields an action.
    pub fn update_event_with<Action>(
        &mut self,
        event: ButtonEvent<Action>,
        runtime: &mut MotionRuntime,
        on_action: impl FnOnce(Action),
    ) -> Result<bool, MotionError> {
        if let Some(action) = self.update_event(event, runtime)? {
            on_action(action);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Returns the current runtime motion value, or the target value before registration.
    pub fn motion_value(&self, runtime: &MotionRuntime) -> Result<ButtonMotion, MotionError> {
        Ok(self
            .motion
            .value(runtime)?
            .copied()
            .unwrap_or_else(|| self.target_motion()))
    }

    /// Returns a rendering snapshot without exposing internal state.
    pub fn snapshot(
        &self,
        runtime: &MotionRuntime,
        context: &ComponentContext,
    ) -> Result<ButtonSnapshot, MotionError> {
        let style_state = self.state.style_state();

        Ok(ButtonSnapshot {
            variant: self.variant,
            style_state,
            style: ButtonResolvedStyle::from_component_context(context, self.variant, style_state),
            motion: self.motion_value(runtime)?,
            focused: self.state.is_focused(),
            disabled: self.state.is_disabled(),
        })
    }

    /// Returns this button visual variant.
    #[must_use]
    pub const fn variant(&self) -> ButtonVariant {
        self.variant
    }

    /// Returns this button's stable content.
    #[must_use]
    pub const fn content(&self) -> &ButtonContent {
        &self.content
    }

    /// Returns this button's stable layout configuration.
    #[must_use]
    pub const fn layout(&self) -> ButtonLayout {
        self.layout
    }

    fn target_motion(&self) -> ButtonMotion {
        self.state.target_motion()
    }
}
