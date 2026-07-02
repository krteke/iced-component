use aura_anim::{
    core::{interpolate::InterpolationProgress, traits::Interpolate},
    prelude::Animatable,
};

use crate::{
    button::{ButtonResolvedStyle, ButtonStyleState, ButtonVariant},
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
