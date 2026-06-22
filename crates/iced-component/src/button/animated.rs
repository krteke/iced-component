use aura_anim_core::{MotionError, MotionRuntime, timing::Timing};

use crate::{
    button::{
        ButtonAppearance, ButtonResolvedStyle, ButtonRole, ButtonStyleState, ButtonVariant,
        flags::ButtonFlags, motion::ButtonMotion,
    },
    component::{ComponentContext, ComponentMotion},
    motion::{Easing, MotionSpeed, MotionTransition},
};

/// Read-only button state consumed by rendering code.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AnimatedButtonSnapshot {
    /// Button visual variant.
    pub variant: ButtonVariant,
    /// Interaction state used to resolve component style.
    pub style_state: ButtonStyleState,
    /// Resolved theme style for the current interaction state.
    pub style: ButtonResolvedStyle,
    /// Current animated motion values.
    pub motion: ButtonMotion,
    /// Whether focus visuals are active.
    pub focused: bool,
    /// Whether the button is disabled.
    pub disabled: bool,
}

/// Button interaction message handled by [`AnimatedButton`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ButtonInteraction {
    /// Pointer entered the button.
    HoverEnter,
    /// Pointer left the button.
    HoverExit,
    /// Pointer pressed the button.
    PressDown,
    /// Pointer released the button.
    PressUp,
    /// Keyboard focus entered the button.
    Focus,
    /// Keyboard focus left the button.
    Blur,
    /// Enables or disables the button.
    SetDisabled(bool),
}

/// Button event that can carry an application action on release.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ButtonEvent<Action> {
    /// Internal interaction that only updates button state.
    Interaction(ButtonInteraction),
    /// Release event that first resets pressed state, then yields an action.
    Pressed(Action),
}

/// Stateful animated button core without Iced rendering.
#[derive(Debug)]
pub struct AnimatedButton {
    label: String,
    variant: ButtonVariant,
    flags: ButtonFlags,
    motion: ComponentMotion<ButtonMotion>,
}

impl AnimatedButton {
    /// Creates a button core with an unregistered motion handle.
    #[must_use]
    pub fn new(label: impl Into<String>, variant: ButtonVariant) -> Self {
        Self {
            label: label.into(),
            variant,
            flags: ButtonFlags::empty(),
            motion: ComponentMotion::new(ButtonMotion::idle(), Timing::linear(120.0)),
        }
    }

    /// Creates a standard animated button.
    #[must_use]
    pub fn standard(label: impl Into<String>) -> Self {
        Self::new(label, ButtonVariant::STANDARD)
    }

    /// Creates a suggested-action animated button.
    #[must_use]
    pub fn suggested(label: impl Into<String>) -> Self {
        Self::new(label, ButtonVariant::SUGGESTED)
    }

    /// Creates a suggested-action animated button.
    #[must_use]
    pub fn primary(label: impl Into<String>) -> Self {
        Self::suggested(label)
    }

    /// Creates a destructive-action animated button.
    #[must_use]
    pub fn destructive(label: impl Into<String>) -> Self {
        Self::new(label, ButtonVariant::DESTRUCTIVE)
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

    /// Returns this button with a different visual appearance.
    #[must_use]
    pub fn with_appearance(mut self, appearance: ButtonAppearance) -> Self {
        self.variant = self.variant.with_appearance(appearance);
        self
    }

    /// Returns this button with minimal low-emphasis styling.
    #[must_use]
    pub fn flat(self) -> Self {
        self.with_appearance(ButtonAppearance::Flat)
    }

    /// Returns this button with explicit raised styling.
    #[must_use]
    pub fn raised(self) -> Self {
        self.with_appearance(ButtonAppearance::Raised)
    }

    /// Returns this button with disabled state preconfigured.
    #[must_use]
    pub fn disabled(mut self, disabled: bool) -> Self {
        if disabled {
            self.flags.insert(ButtonFlags::DISABLED);
        } else {
            self.flags.remove(ButtonFlags::DISABLED);
        }
        self
    }

    /// Registers the button motion handle in the application runtime.
    pub fn register(&mut self, runtime: &mut MotionRuntime, context: &ComponentContext) {
        if self.motion.is_registered() {
            return;
        }

        let timing = context.motion_tokens().timing(
            MotionTransition::new(MotionSpeed::Fast, Easing::EaseOut),
            context.motion_preferences(),
        );
        self.motion = ComponentMotion::new(self.target_motion(), timing);
        let _ = self.motion.register(runtime);
    }

    /// Applies a button interaction and transitions motion when registered.
    pub fn update(
        &mut self,
        interaction: ButtonInteraction,
        runtime: &mut MotionRuntime,
    ) -> Result<bool, MotionError> {
        match interaction {
            ButtonInteraction::HoverEnter => self.flags.insert(ButtonFlags::HOVERED),
            ButtonInteraction::HoverExit => {
                self.flags
                    .remove(ButtonFlags::HOVERED | ButtonFlags::PRESSED);
            }
            ButtonInteraction::PressDown if !self.flags.contains(ButtonFlags::DISABLED) => {
                self.flags.insert(ButtonFlags::PRESSED);
            }
            ButtonInteraction::PressUp => self.flags.remove(ButtonFlags::PRESSED),
            ButtonInteraction::Focus => self.flags.insert(ButtonFlags::FOCUSED),
            ButtonInteraction::Blur => self.flags.remove(ButtonFlags::FOCUSED),
            ButtonInteraction::SetDisabled(disabled) => {
                if disabled {
                    self.flags.insert(ButtonFlags::DISABLED);
                    self.flags
                        .remove(ButtonFlags::HOVERED | ButtonFlags::PRESSED);
                } else {
                    self.flags.remove(ButtonFlags::DISABLED);
                }
            }
            ButtonInteraction::PressDown => {}
        }

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
        match event {
            ButtonEvent::Interaction(interaction) => {
                self.update(interaction, runtime)?;
                Ok(None)
            }
            ButtonEvent::Pressed(action) => {
                let disabled = self.flags.contains(ButtonFlags::DISABLED);
                self.update(ButtonInteraction::PressUp, runtime)?;
                Ok((!disabled).then_some(action))
            }
        }
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
    ) -> Result<AnimatedButtonSnapshot, MotionError> {
        let style_state = self.style_state();

        Ok(AnimatedButtonSnapshot {
            variant: self.variant,
            style_state,
            style: ButtonResolvedStyle::from_component_context(context, self.variant, style_state),
            motion: self.motion_value(runtime)?,
            focused: self.flags.contains(ButtonFlags::FOCUSED),
            disabled: self.flags.contains(ButtonFlags::DISABLED),
        })
    }

    /// Returns this button visual variant.
    #[must_use]
    pub const fn variant(&self) -> ButtonVariant {
        self.variant
    }

    /// Returns this button label.
    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }

    fn style_state(&self) -> ButtonStyleState {
        if self.flags.contains(ButtonFlags::DISABLED) {
            ButtonStyleState::Disabled
        } else if self.flags.contains(ButtonFlags::PRESSED) {
            ButtonStyleState::Pressed
        } else if self.flags.contains(ButtonFlags::HOVERED) {
            ButtonStyleState::Hovered
        } else {
            ButtonStyleState::Idle
        }
    }

    fn target_motion(&self) -> ButtonMotion {
        let focused = self.flags.contains(ButtonFlags::FOCUSED);

        if self.flags.contains(ButtonFlags::DISABLED) {
            return ButtonMotion::disabled(focused);
        }
        if self.flags.contains(ButtonFlags::PRESSED) {
            return ButtonMotion::pressed(focused);
        }
        if self.flags.contains(ButtonFlags::HOVERED) {
            return ButtonMotion::hovered(focused);
        }

        ButtonMotion::idle_with_focus(focused)
    }
}

#[cfg(test)]
mod tests {
    use aura_anim_core::{MotionRuntime, timing::Duration};
    use float_cmp::assert_approx_eq;

    use crate::{
        button::{ButtonAppearance, ButtonRole, ButtonStyleState, ButtonVariant},
        component::ComponentContext,
    };

    use super::{AnimatedButton, ButtonEvent, ButtonInteraction, ButtonMotion};

    #[test]
    fn interaction_before_registration_updates_target_without_runtime_motion() {
        let mut runtime = MotionRuntime::new();
        let mut button = AnimatedButton::standard("Save");

        let changed = button
            .update(ButtonInteraction::HoverEnter, &mut runtime)
            .unwrap();

        assert!(!changed);
        assert_eq!(runtime.motion_count(), 0);
        assert_eq!(
            button.motion_value(&runtime).unwrap(),
            ButtonMotion {
                scale: 1.0,
                shadow_y: 1.2,
                bg_alpha: 1.0,
                border_glow: 0.0,
                focus_alpha: 0.0,
            }
        );
    }

    #[test]
    fn registered_hover_transitions_runtime_motion() {
        let mut runtime = MotionRuntime::new();
        let context = ComponentContext::current();
        let mut button = AnimatedButton::suggested("Save");

        button.register(&mut runtime, &context);
        let changed = button
            .update(ButtonInteraction::HoverEnter, &mut runtime)
            .unwrap();
        runtime.tick(Duration::from_millis(120.0));

        assert!(changed);
        assert_eq!(runtime.motion_count(), 1);
        assert_approx_eq!(f32, button.motion_value(&runtime).unwrap().shadow_y, 1.2);
        assert_eq!(button.variant(), ButtonVariant::SUGGESTED);
    }

    #[test]
    fn builders_update_role_and_appearance() {
        let button = AnimatedButton::standard("Save")
            .with_role(ButtonRole::Suggested)
            .flat();

        assert_eq!(
            button.variant(),
            ButtonVariant::SUGGESTED.with_appearance(ButtonAppearance::Flat)
        );

        let button = AnimatedButton::destructive("Delete").raised();

        assert_eq!(
            button.variant(),
            ButtonVariant::DESTRUCTIVE.with_appearance(ButtonAppearance::Raised)
        );
    }

    #[test]
    fn disabled_button_ignores_press_down() {
        let mut runtime = MotionRuntime::new();
        let context = ComponentContext::current();
        let mut button = AnimatedButton::standard("Save");

        button.register(&mut runtime, &context);
        button
            .update(ButtonInteraction::SetDisabled(true), &mut runtime)
            .unwrap();
        button
            .update(ButtonInteraction::PressDown, &mut runtime)
            .unwrap();
        runtime.tick(Duration::from_millis(120.0));

        let motion = button.motion_value(&runtime).unwrap();
        assert_approx_eq!(f32, motion.scale, 1.0);
        assert_approx_eq!(f32, motion.bg_alpha, 0.45);
    }

    #[test]
    fn set_disabled_updates_button_state() {
        let mut runtime = MotionRuntime::new();
        let mut button = AnimatedButton::standard("Save");

        button.set_disabled(true, &mut runtime).unwrap();

        let snapshot = button
            .snapshot(&runtime, &ComponentContext::current())
            .unwrap();
        assert!(snapshot.disabled);
        assert_eq!(snapshot.style_state, ButtonStyleState::Disabled);
    }

    #[test]
    fn pressed_event_releases_button_and_returns_action() {
        let mut runtime = MotionRuntime::new();
        let mut button = AnimatedButton::standard("Save");

        button
            .update(ButtonInteraction::PressDown, &mut runtime)
            .unwrap();
        let action = button
            .update_event(ButtonEvent::Pressed("save"), &mut runtime)
            .unwrap();

        assert_eq!(action, Some("save"));
        assert_eq!(
            button
                .snapshot(&runtime, &ComponentContext::current())
                .unwrap()
                .style_state,
            ButtonStyleState::Idle
        );
    }

    #[test]
    fn update_event_with_invokes_action_only_for_pressed_event() {
        let mut runtime = MotionRuntime::new();
        let mut button = AnimatedButton::standard("Save");
        let mut action_count = 0;

        let handled = button
            .update_event_with(
                ButtonEvent::Interaction(ButtonInteraction::HoverEnter),
                &mut runtime,
                |()| action_count += 1,
            )
            .unwrap();

        assert!(!handled);
        assert_eq!(action_count, 0);

        let handled = button
            .update_event_with(ButtonEvent::Pressed(()), &mut runtime, |()| {
                action_count += 1;
            })
            .unwrap();

        assert!(handled);
        assert_eq!(action_count, 1);
    }

    #[test]
    fn update_event_with_ignores_pressed_action_when_disabled() {
        let mut runtime = MotionRuntime::new();
        let mut button = AnimatedButton::standard("Save");
        let mut action_count = 0;

        button
            .update(ButtonInteraction::SetDisabled(true), &mut runtime)
            .unwrap();
        let handled = button
            .update_event_with(ButtonEvent::Pressed(()), &mut runtime, |()| {
                action_count += 1;
            })
            .unwrap();

        assert!(!handled);
        assert_eq!(action_count, 0);
    }

    #[test]
    fn snapshot_combines_style_and_motion() {
        let mut runtime = MotionRuntime::new();
        let context = ComponentContext::current();
        let mut button = AnimatedButton::suggested("Save");

        button.register(&mut runtime, &context);
        button
            .update(ButtonInteraction::PressDown, &mut runtime)
            .unwrap();
        runtime.tick(Duration::from_millis(120.0));

        let snapshot = button.snapshot(&runtime, &context).unwrap();

        assert_eq!(snapshot.variant, ButtonVariant::SUGGESTED);
        assert_eq!(snapshot.style_state, ButtonStyleState::Pressed);
        assert_eq!(
            snapshot.style.background,
            context.theme().theme().button.suggested.pressed.bg
        );
        assert_approx_eq!(f32, snapshot.motion.scale, 0.98);
    }

    #[test]
    fn snapshot_reports_focus_and_disabled_flags() {
        let mut runtime = MotionRuntime::new();
        let context = ComponentContext::current();
        let mut button = AnimatedButton::standard("Save");

        button
            .update(ButtonInteraction::Focus, &mut runtime)
            .unwrap();
        button
            .update(ButtonInteraction::SetDisabled(true), &mut runtime)
            .unwrap();

        let snapshot = button.snapshot(&runtime, &context).unwrap();

        assert!(snapshot.focused);
        assert!(snapshot.disabled);
        assert_eq!(snapshot.style_state, ButtonStyleState::Disabled);
        assert_approx_eq!(f32, snapshot.motion.focus_alpha, 0.5);
    }
}
