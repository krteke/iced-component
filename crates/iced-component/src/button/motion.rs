use aura_anim::{
    core::{interpolate::InterpolationProgress, traits::Interpolate},
    prelude::Animatable,
};
use spectrum_theme::{Color, Length, Radius, ShadowLayer};

use crate::{
    button::{ButtonResolvedStyle, ButtonStyleState, ButtonVariant},
    theme::{ButtonComponentTokens, ThemePack},
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
            bg: interpolate_color(from.bg, to.bg, progress),
            fg: interpolate_color(from.fg, to.fg, progress),
            border: interpolate_color(from.border, to.border, progress),
            border_width: interpolate_length(from.border_width, to.border_width, progress),
            radius: interpolate_radius(from.radius, to.radius, progress),
            focus_ring: interpolate_color(from.focus_ring, to.focus_ring, progress),
            shadow: interpolate_shadow(from.shadow, to.shadow, progress),
        }
    }
}

fn interpolate_color(from: Color, to: Color, progress: InterpolationProgress) -> Color {
    Color::new_rgba(
        u8::interpolate_progress(&from.red(), &to.red(), progress),
        u8::interpolate_progress(&from.green(), &to.green(), progress),
        u8::interpolate_progress(&from.blue(), &to.blue(), progress),
        u8::interpolate_progress(&from.alpha(), &to.alpha(), progress),
    )
}

fn interpolate_length(from: Length, to: Length, progress: InterpolationProgress) -> Length {
    Length::new(
        f32::interpolate_progress(&from.value(), &to.value(), progress),
        to.unit(),
    )
    .expect("interpolated theme length remains finite")
}

fn interpolate_radius(from: Radius, to: Radius, progress: InterpolationProgress) -> Radius {
    Radius::new(interpolate_length(from.length(), to.length(), progress))
        .expect("interpolated theme radius remains non-negative")
}

fn interpolate_shadow(
    from: ShadowLayer,
    to: ShadowLayer,
    progress: InterpolationProgress,
) -> ShadowLayer {
    ShadowLayer::new(
        interpolate_color(from.color(), to.color(), progress),
        interpolate_length(from.offset_x(), to.offset_x(), progress),
        interpolate_length(from.offset_y(), to.offset_y(), progress),
        interpolate_length(from.blur(), to.blur(), progress),
        interpolate_length(from.spread(), to.spread(), progress),
    )
    .expect("interpolated theme shadow remains valid")
}
