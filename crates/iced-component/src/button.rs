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

use aura_anim::core::runtime::{MotionError, MotionRuntime};
use iced::Length;

use crate::{
    button::state::ButtonState,
    component::{ComponentContext, ComponentUpdateCx, ComponentViewCx, MotionSlot},
};

pub use animated::{ButtonEvent, ButtonInteraction, ButtonSnapshot};
pub use content::{ButtonContent, ButtonLayout};
pub use icon::{IconButton, IconButtonSize, IconSource};
pub use motion::{
    ButtonAnimationBuilder, ButtonAnimationProvider, ButtonMotion, ButtonMotionTransition,
    ButtonMotionTrigger, trigger_from_interaction,
};
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
    motion: MotionSlot<ButtonMotion>,
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
            motion: MotionSlot::new(),
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

    /// Clears this button's stable content.
    pub fn clear_content(&mut self) {
        self.content = ButtonContent::Empty;
    }

    /// Returns this button with a different stable layout configuration.
    #[must_use]
    pub const fn with_layout(mut self, layout: ButtonLayout) -> Self {
        self.layout = layout;
        self
    }

    /// Replaces this button's stable layout configuration.
    pub fn set_layout(&mut self, layout: ButtonLayout) {
        self.layout = layout;
    }

    /// Returns this button with a different visual variant.
    #[must_use]
    pub fn with_variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Replaces this button's visual variant.
    pub fn set_variant(&mut self, variant: ButtonVariant) {
        self.variant = variant;
    }

    /// Returns this button with a different semantic role.
    #[must_use]
    pub fn with_role(mut self, role: ButtonRole) -> Self {
        self.variant = self.variant.with_role(role);
        self
    }

    /// Updates this button's semantic role.
    pub fn set_role(&mut self, role: ButtonRole) {
        self.variant = self.variant.with_role(role);
    }

    /// Returns this button with a different visual treatment.
    #[must_use]
    pub fn with_treatment(mut self, treatment: ButtonTreatment) -> Self {
        self.variant = self.variant.with_treatment(treatment);
        self
    }

    /// Updates this button's visual treatment.
    pub fn set_treatment(&mut self, treatment: ButtonTreatment) {
        self.variant = self.variant.with_treatment(treatment);
    }

    /// Returns this button with a different outline shape.
    #[must_use]
    pub fn with_shape(mut self, shape: ButtonShape) -> Self {
        self.variant = self.variant.with_shape(shape);
        self
    }

    /// Updates this button's outline shape.
    pub fn set_shape(&mut self, shape: ButtonShape) {
        self.variant = self.variant.with_shape(shape);
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
    pub const fn with_padding(self, padding: [f32; 2]) -> Self {
        self.padding(padding)
    }

    /// Returns this button with explicit padding.
    #[must_use]
    pub const fn padding(mut self, padding: [f32; 2]) -> Self {
        self.layout.padding = Some(padding);
        self
    }

    /// Returns this button with a fixed rendered width.
    #[must_use]
    pub fn with_width(self, width: impl Into<Length>) -> Self {
        self.width(width)
    }

    /// Returns this button with a fixed rendered width.
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.layout.width = Some(width.into());
        self
    }

    /// Returns this button with a fixed rendered height.
    #[must_use]
    pub fn with_height(self, height: impl Into<Length>) -> Self {
        self.height(height)
    }

    /// Returns this button with a fixed rendered height.
    #[must_use]
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.layout.height = Some(height.into());
        self
    }

    /// Returns this button with equal width and height.
    #[must_use]
    pub fn with_square(self, size: f32) -> Self {
        self.square(size)
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
    pub fn with_disabled(self, disabled: bool) -> Self {
        self.disabled(disabled)
    }

    /// Returns this button with disabled state preconfigured.
    #[must_use]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.state.apply(ButtonInteraction::SetDisabled(disabled));
        self
    }

    /// Updates this button's explicit padding.
    pub fn set_padding(&mut self, padding: [f32; 2]) {
        self.layout.padding = Some(padding);
    }

    /// Clears this button's explicit padding.
    pub fn clear_padding(&mut self) {
        self.layout.padding = None;
    }

    /// Updates this button's fixed rendered width.
    pub fn set_width(&mut self, width: impl Into<Length>) {
        self.layout.width = Some(width.into());
    }

    /// Clears this button's fixed rendered width.
    pub fn clear_width(&mut self) {
        self.layout.width = None;
    }

    /// Updates this button's fixed rendered height.
    pub fn set_height(&mut self, height: impl Into<Length>) {
        self.layout.height = Some(height.into());
    }

    /// Clears this button's fixed rendered height.
    pub fn clear_height(&mut self) {
        self.layout.height = None;
    }

    /// Updates this button to equal width and height.
    pub fn set_square(&mut self, size: f32) {
        self.layout.width = Some(Length::Fixed(size));
        self.layout.height = Some(Length::Fixed(size));
        self.layout.padding = Some([0.0, 0.0]);
        self.layout.center_content = true;
    }

    /// Updates whether this button's content should be centered.
    pub fn set_center_content(&mut self, center_content: bool) {
        self.layout.center_content = center_content;
    }

    /// Registers the button motion handle using the current component context.
    pub fn register(&mut self, cx: &mut ComponentUpdateCx<'_>) {
        if self.motion.is_registered() {
            return;
        }

        let _ = self.motion.register(
            cx.runtime,
            self.motion_from_ctx(cx.context()),
            cx.context().theme_revision(),
        );
    }

    /// Synchronizes this button's current motion target with the runtime.
    pub fn sync(&mut self, cx: &mut ComponentUpdateCx<'_>) -> Result<bool, MotionError> {
        if !self.motion.is_registered() {
            return Ok(false);
        }

        self.animate_to_current(ButtonMotionTrigger::Sync, cx)
    }

    /// Applies a button interaction and transitions motion when registered.
    pub fn update(
        &mut self,
        interaction: ButtonInteraction,
        cx: &mut ComponentUpdateCx<'_>,
    ) -> Result<bool, MotionError> {
        let previous = self.state;
        self.state.apply(interaction);

        self.animate_from_state(previous, motion::trigger_from_interaction(interaction), cx)
    }

    /// Enables or disables this button and updates its motion target.
    pub fn set_disabled(
        &mut self,
        disabled: bool,
        cx: &mut ComponentUpdateCx<'_>,
    ) -> Result<bool, MotionError> {
        self.update(ButtonInteraction::SetDisabled(disabled), cx)
    }

    /// Applies a button event and returns its application action, if any.
    pub fn update_event<Action>(
        &mut self,
        event: ButtonEvent<Action>,
        cx: &mut ComponentUpdateCx<'_>,
    ) -> Result<Option<Action>, MotionError> {
        let previous = self.state;
        let timing = match &event {
            ButtonEvent::Interaction(interaction) => motion::trigger_from_interaction(*interaction),
            ButtonEvent::Pressed(_) => ButtonMotionTrigger::PressUp,
        };
        let action = self.state.apply_event(event);
        self.animate_from_state(previous, timing, cx)?;
        Ok(action)
    }

    /// Applies a button event and invokes `on_action` when release yields an action.
    pub fn update_event_with<Action>(
        &mut self,
        event: ButtonEvent<Action>,
        cx: &mut ComponentUpdateCx<'_>,
        on_action: impl FnOnce(Action),
    ) -> Result<bool, MotionError> {
        if let Some(action) = self.update_event(event, cx)? {
            on_action(action);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Returns the raw runtime motion value, or `None` if not registered.
    ///
    /// This does not validate the current theme revision. Rendering code should
    /// use [`snapshot`](Self::snapshot), which falls back to current context
    /// tokens when the runtime value belongs to an older theme.
    pub fn motion_value(
        &self,
        runtime: &MotionRuntime,
    ) -> Result<Option<ButtonMotion>, MotionError> {
        Ok(self.motion.value(runtime)?.copied())
    }

    fn motion_value_for_context(
        &self,
        runtime: &MotionRuntime,
        context: &ComponentContext,
    ) -> Result<ButtonMotion, MotionError> {
        Ok(self
            .motion
            .value_if_current(runtime, context.theme_revision())?
            .copied()
            .unwrap_or_else(|| self.motion_from_ctx(context)))
    }

    /// Returns a rendering snapshot without exposing internal state.
    pub fn snapshot(&self, cx: &ComponentViewCx<'_>) -> Result<ButtonSnapshot, MotionError> {
        let style_state = self.state.style_state();
        let motion = self.motion_value_for_context(cx.runtime, cx.context())?;

        Ok(ButtonSnapshot {
            variant: self.variant,
            style_state,
            style: ButtonResolvedStyle::from_component_tokens(motion.tokens),
            motion,
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

    /// Returns whether this button is disabled.
    #[must_use]
    pub const fn is_disabled(&self) -> bool {
        self.state.is_disabled()
    }

    /// Returns whether this button is focused.
    #[must_use]
    pub const fn is_focused(&self) -> bool {
        self.state.is_focused()
    }

    /// Returns this button's current style state.
    #[must_use]
    pub const fn style_state(&self) -> ButtonStyleState {
        self.state.style_state()
    }

    fn motion_from_ctx(&self, context: &ComponentContext) -> ButtonMotion {
        self.motion_from_state(context, self.state)
    }

    fn motion_from_state(&self, context: &ComponentContext, state: ButtonState) -> ButtonMotion {
        ButtonMotion::from_theme(
            context.theme().theme(),
            self.variant,
            state.style_state(),
            state.is_focused(),
        )
    }

    fn animate_from_state(
        &mut self,
        previous: ButtonState,
        trigger: ButtonMotionTrigger,
        cx: &mut ComponentUpdateCx<'_>,
    ) -> Result<bool, MotionError> {
        if previous == self.state && !self.motion.is_registered() {
            return Ok(false);
        }

        let initial = self
            .motion
            .value_if_current(cx.runtime, cx.context().theme_revision())?
            .copied()
            .unwrap_or_else(|| self.motion_from_state(cx.context(), previous));
        let target = self.motion_from_ctx(cx.context());

        self.play_motion(
            ButtonMotionTransition {
                from: initial,
                to: target,
                trigger,
            },
            cx,
        )
    }

    fn animate_to_current(
        &mut self,
        trigger: ButtonMotionTrigger,
        cx: &mut ComponentUpdateCx<'_>,
    ) -> Result<bool, MotionError> {
        let initial = self
            .motion
            .value_if_current(cx.runtime, cx.context().theme_revision())?
            .copied()
            .unwrap_or_else(|| self.motion_from_ctx(cx.context()));
        let target = self.motion_from_ctx(cx.context());

        self.play_motion(
            ButtonMotionTransition {
                from: initial,
                to: target,
                trigger,
            },
            cx,
        )
    }

    fn play_motion(
        &mut self,
        transition: ButtonMotionTransition,
        cx: &mut ComponentUpdateCx<'_>,
    ) -> Result<bool, MotionError> {
        let initial = transition.from;
        let animation = cx.context().animation().button().build(&transition);

        self.motion.play_from_or_finish(initial, animation, cx)
    }
}
