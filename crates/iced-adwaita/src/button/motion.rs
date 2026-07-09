use aura_anim::{
    core::{interpolate::InterpolationProgress, traits::Interpolate},
    prelude::Animatable,
};

use crate::{
    button::{
        ButtonSignal, ButtonStyleOverride, ButtonStyleState, ButtonVariant,
        style::component_tokens_from_theme,
    },
    theme::{
        interpolate,
        tokens::{ButtonTokens, ThemePack},
    },
};

/// Animatable visual values for an Adwaita button.
#[derive(Clone, Copy, Debug, PartialEq, Animatable)]
pub struct ButtonMotion {
    /// Animated theme component tokens for the current state.
    pub tokens: ButtonTokens,
    /// Width of the focus ring in logical pixels.
    pub focus_ring_width: f32,
}

/// Data passed to animation providers and builders.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ButtonMotionTransition {
    /// Current visual value used as animation start.
    pub from: ButtonMotion,
    /// Resolved target visual value.
    pub to: ButtonMotion,
    /// State or synchronization signal that requested the transition.
    pub signal: ButtonSignal,
}

impl ButtonMotion {
    pub(crate) fn from_theme(
        theme: &ThemePack,
        variant: ButtonVariant,
        overrides: ButtonStyleOverride,
        state: ButtonStyleState,
        focused: bool,
    ) -> Self {
        Self {
            tokens: component_tokens_from_theme(theme, variant, overrides, state),
            focus_ring_width: if focused && !matches!(state, ButtonStyleState::Disabled) {
                2.0
            } else {
                0.0
            },
        }
    }
}

impl Interpolate for ButtonTokens {
    fn interpolate_progress(from: &Self, to: &Self, progress: InterpolationProgress) -> Self {
        Self {
            bg: interpolate::color(from.bg, to.bg, progress),
            fg: interpolate::color(from.fg, to.fg, progress),
            border: interpolate::color(from.border, to.border, progress),
            border_width: interpolate::length(from.border_width, to.border_width, progress),
            radius: interpolate::radius(from.radius, to.radius, progress),
            focus_ring: interpolate::color(from.focus_ring, to.focus_ring, progress),
        }
    }
}
