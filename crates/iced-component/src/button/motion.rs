use std::sync::Arc;

use aura_anim::{
    core::{
        interpolate::InterpolationProgress,
        traits::{BoxAnimation, Interpolate},
    },
    prelude::Animatable,
};

use crate::{
    button::{ButtonInteraction, ButtonResolvedStyle, ButtonStyleState, ButtonVariant},
    theme::{ButtonComponentTokens, ThemePack, interpolate},
};

/// Animatable visual values for an animated button.
#[derive(Clone, Copy, Debug, PartialEq, Animatable)]
pub struct ButtonMotion {
    /// Animated theme component tokens for the current state.
    pub tokens: ButtonComponentTokens,
    /// Focus ring opacity resolved by component interaction state.
    pub focus_ring_alpha: f32,
    /// Focus ring width added to the themed border width.
    pub focus_ring_width: f32,
}

/// Button motion transition trigger.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ButtonMotionTrigger {
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
    /// Button became disabled.
    Disable,
    /// Button became enabled.
    Enable,
    /// Theme or target synchronization.
    Sync,
}

/// Data passed to animation providers and builders.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ButtonMotionTransition {
    /// Current visual value used as animation start.
    pub from: ButtonMotion,
    /// Resolved target visual value.
    pub to: ButtonMotion,
    /// Transition trigger.
    pub trigger: ButtonMotionTrigger,
}

/// A function type for building button animations.
pub type ButtonAnimationBuilder = Arc<dyn Fn(ButtonMotionTransition) -> BoxAnimation<ButtonMotion>>;

/// Provides button animations for a button theme or variant family.
pub trait ButtonAnimationProvider: 'static {
    /// Returns the animation builder for one resolved transition.
    fn button_animation(&self, transition: &ButtonMotionTransition) -> ButtonAnimationBuilder;
}

/// Converts a [`ButtonInteraction`] to a [`ButtonMotionTrigger`].
#[must_use]
pub const fn trigger_from_interaction(interaction: ButtonInteraction) -> ButtonMotionTrigger {
    match interaction {
        ButtonInteraction::HoverEnter => ButtonMotionTrigger::HoverEnter,
        ButtonInteraction::HoverExit => ButtonMotionTrigger::HoverExit,
        ButtonInteraction::PressDown => ButtonMotionTrigger::PressDown,
        ButtonInteraction::PressUp => ButtonMotionTrigger::PressUp,
        ButtonInteraction::Focus => ButtonMotionTrigger::Focus,
        ButtonInteraction::Blur => ButtonMotionTrigger::Blur,
        ButtonInteraction::SetDisabled(true) => ButtonMotionTrigger::Disable,
        ButtonInteraction::SetDisabled(false) => ButtonMotionTrigger::Enable,
    }
}

impl ButtonMotion {
    pub(super) fn from_theme(
        theme: &ThemePack,
        variant: ButtonVariant,
        state: ButtonStyleState,
        focused: bool,
    ) -> Self {
        Self {
            tokens: ButtonResolvedStyle::component_tokens_from_theme(theme, variant, state),
            focus_ring_alpha: if focused {
                if matches!(state, ButtonStyleState::Disabled) {
                    0.5
                } else {
                    1.0
                }
            } else {
                0.0
            },
            focus_ring_width: if focused { 1.0 } else { 0.0 },
        }
    }
}

impl Interpolate for ButtonComponentTokens {
    fn interpolate_progress(from: &Self, to: &Self, progress: InterpolationProgress) -> Self {
        Self {
            bg: interpolate::color(from.bg, to.bg, progress),
            fg: interpolate::color(from.fg, to.fg, progress),
            border: interpolate::color(from.border, to.border, progress),
            border_width: interpolate::length(from.border_width, to.border_width, progress),
            radius: interpolate::radius(from.radius, to.radius, progress),
            focus_ring: interpolate::color(from.focus_ring, to.focus_ring, progress),
            shadow: interpolate::shadow(from.shadow, to.shadow, progress),
        }
    }
}
