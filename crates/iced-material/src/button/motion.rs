use aura_anim::{
    core::{interpolate::InterpolationProgress, traits::Interpolate},
    prelude::Animatable,
};
use spectrum_theme::Color;

use crate::{
    button::{ButtonStyleState, ButtonVariant, style::visual_from_theme},
    theme::tokens::{ButtonStyleTokens, ThemePack},
};

/// Interpolated Material button visual values.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ButtonVisual {
    /// Base container color.
    pub background: Color,
    /// Base container alpha composited by the view layer.
    pub background_opacity: f32,
    /// Content color.
    pub foreground: Color,
    /// Content alpha composited by the view layer.
    pub foreground_opacity: f32,
    /// Border color.
    pub border: Color,
    /// Border alpha composited by the view layer.
    pub border_opacity: f32,
    /// Border width in logical pixels.
    pub border_width: f32,
    /// Uniform corner radius in logical pixels.
    pub radius: f32,
    /// Ambient shadow color.
    pub shadow: Color,
    /// Ambient shadow alpha.
    pub shadow_opacity: f32,
    /// Ambient shadow Y offset in logical pixels.
    pub shadow_y: f32,
    /// Ambient shadow blur in logical pixels.
    pub shadow_blur: f32,
    /// State layer color composited above content.
    pub state_layer: Color,
    /// State layer alpha.
    pub state_layer_opacity: f32,
}

/// Animatable Material button visual state.
#[derive(Clone, Copy, Debug, PartialEq, Animatable)]
pub struct ButtonMotion {
    /// Interpolated visual values used by the Iced view layer.
    pub visual: ButtonVisual,
}

impl ButtonMotion {
    pub(crate) fn from_theme(
        theme: &ThemePack,
        variant: ButtonVariant,
        state: ButtonStyleState,
    ) -> Self {
        Self {
            visual: visual_from_theme(theme, variant, state),
        }
    }
}

impl ButtonVisual {
    pub(crate) fn from_tokens(tokens: ButtonStyleTokens) -> Self {
        Self {
            background: tokens.background,
            background_opacity: tokens.background_opacity.value(),
            foreground: tokens.foreground,
            foreground_opacity: tokens.foreground_opacity.value(),
            border: tokens.border,
            border_opacity: tokens.border_opacity.value(),
            border_width: tokens.border_width.value(),
            radius: tokens.radius.length().value(),
            shadow: tokens.shadow,
            shadow_opacity: tokens.shadow_opacity.value(),
            shadow_y: tokens.shadow_y.value(),
            shadow_blur: tokens.shadow_blur.value(),
            state_layer: tokens.state_layer,
            state_layer_opacity: tokens.state_layer_opacity.value(),
        }
    }
}

impl Interpolate for ButtonVisual {
    fn interpolate_progress(from: &Self, to: &Self, progress: InterpolationProgress) -> Self {
        Self {
            background: color(from.background, to.background, progress),
            background_opacity: f32::interpolate_progress(
                &from.background_opacity,
                &to.background_opacity,
                progress,
            ),
            foreground: color(from.foreground, to.foreground, progress),
            foreground_opacity: f32::interpolate_progress(
                &from.foreground_opacity,
                &to.foreground_opacity,
                progress,
            ),
            border: color(from.border, to.border, progress),
            border_opacity: f32::interpolate_progress(
                &from.border_opacity,
                &to.border_opacity,
                progress,
            ),
            border_width: f32::interpolate_progress(&from.border_width, &to.border_width, progress),
            radius: f32::interpolate_progress(&from.radius, &to.radius, progress),
            shadow: color(from.shadow, to.shadow, progress),
            shadow_opacity: f32::interpolate_progress(
                &from.shadow_opacity,
                &to.shadow_opacity,
                progress,
            ),
            shadow_y: f32::interpolate_progress(&from.shadow_y, &to.shadow_y, progress),
            shadow_blur: f32::interpolate_progress(&from.shadow_blur, &to.shadow_blur, progress),
            state_layer: color(from.state_layer, to.state_layer, progress),
            state_layer_opacity: f32::interpolate_progress(
                &from.state_layer_opacity,
                &to.state_layer_opacity,
                progress,
            ),
        }
    }
}

fn color(from: Color, to: Color, progress: InterpolationProgress) -> Color {
    Color::new_rgba(
        u8::interpolate_progress(&from.red(), &to.red(), progress),
        u8::interpolate_progress(&from.green(), &to.green(), progress),
        u8::interpolate_progress(&from.blue(), &to.blue(), progress),
        u8::interpolate_progress(&from.alpha(), &to.alpha(), progress),
    )
}
