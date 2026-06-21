use aura_anim_core::{Animatable, MotionError, MotionRuntime, timing::Timing};
use iced::animation::Easing;

use crate::{
    button::{ButtonResolvedStyle, ButtonStyleState, ButtonVariant},
    component::{ComponentContext, ComponentMotion},
    motion::{MotionSpeed, MotionTransition},
};

/// Animatable visual values for an animated button.
#[derive(Clone, Copy, Debug, PartialEq, Animatable)]
pub struct ButtonMotion {
    /// Content scale multiplier.
    pub scale: f32,
    /// Vertical shadow offset multiplier.
    pub shadow_y: f32,
    /// Background emphasis multiplier.
    pub bg_alpha: f32,
    /// Border glow opacity.
    pub border_glow: f32,
    /// Focus ring opacity.
    pub focus_alpha: f32,
}

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
        Self::new(label, ButtonVariant::Standard)
    }

    /// Creates a primary animated button.
    #[must_use]
    pub fn primary(label: impl Into<String>) -> Self {
        Self::new(label, ButtonVariant::Primary)
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

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct ButtonFlags(u8);

impl ButtonFlags {
    const HOVERED: Self = Self(1 << 0);
    const PRESSED: Self = Self(1 << 1);
    const FOCUSED: Self = Self(1 << 2);
    const DISABLED: Self = Self(1 << 3);

    const fn empty() -> Self {
        Self(0)
    }

    const fn contains(self, flags: Self) -> bool {
        self.0 & flags.0 == flags.0
    }

    fn insert(&mut self, flags: Self) {
        self.0 |= flags.0;
    }

    fn remove(&mut self, flags: Self) {
        self.0 &= !flags.0;
    }
}

impl core::ops::BitOr for ButtonFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl ButtonMotion {
    const fn idle() -> Self {
        Self::idle_with_focus(false)
    }

    const fn idle_with_focus(focused: bool) -> Self {
        Self {
            scale: 1.0,
            shadow_y: 1.0,
            bg_alpha: 1.0,
            border_glow: if focused { 1.0 } else { 0.0 },
            focus_alpha: if focused { 1.0 } else { 0.0 },
        }
    }

    const fn hovered(focused: bool) -> Self {
        Self {
            shadow_y: 1.2,
            ..Self::idle_with_focus(focused)
        }
    }

    const fn pressed(focused: bool) -> Self {
        Self {
            scale: 0.98,
            shadow_y: 0.35,
            bg_alpha: 0.95,
            ..Self::idle_with_focus(focused)
        }
    }

    const fn disabled(focused: bool) -> Self {
        Self {
            scale: 1.0,
            shadow_y: 0.0,
            bg_alpha: 0.45,
            border_glow: 0.0,
            focus_alpha: if focused { 0.5 } else { 0.0 },
        }
    }
}

#[cfg(test)]
mod tests {
    use aura_anim_core::{MotionRuntime, timing::Duration};
    use float_cmp::assert_approx_eq;

    use crate::{
        button::{ButtonStyleState, ButtonVariant},
        component::ComponentContext,
    };

    use super::{AnimatedButton, ButtonInteraction, ButtonMotion};

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
        let mut button = AnimatedButton::primary("Save");

        button.register(&mut runtime, &context);
        let changed = button
            .update(ButtonInteraction::HoverEnter, &mut runtime)
            .unwrap();
        runtime.tick(Duration::from_millis(120.0));

        assert!(changed);
        assert_eq!(runtime.motion_count(), 1);
        assert_approx_eq!(f32, button.motion_value(&runtime).unwrap().shadow_y, 1.2);
        assert_eq!(button.variant(), ButtonVariant::Primary);
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
    fn snapshot_combines_style_and_motion() {
        let mut runtime = MotionRuntime::new();
        let context = ComponentContext::current();
        let mut button = AnimatedButton::primary("Save");

        button.register(&mut runtime, &context);
        button
            .update(ButtonInteraction::PressDown, &mut runtime)
            .unwrap();
        runtime.tick(Duration::from_millis(120.0));

        let snapshot = button.snapshot(&runtime, &context).unwrap();

        assert_eq!(snapshot.variant, ButtonVariant::Primary);
        assert_eq!(snapshot.style_state, ButtonStyleState::Pressed);
        assert_eq!(
            snapshot.style.background,
            context.theme().theme().button.primary.pressed.bg
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
