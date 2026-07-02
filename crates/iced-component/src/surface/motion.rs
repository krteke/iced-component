use aura_anim::{
    core::{interpolate::InterpolationProgress, traits::Interpolate},
    prelude::Animatable,
};

use crate::{
    surface::{SurfaceStyleState, SurfaceTreatment, SurfaceVariant, style},
    theme::{SurfaceTokens, ThemePack, interpolate},
};

/// Animatable visual values for themed surfaces.
#[derive(Clone, Copy, Debug, PartialEq, Animatable)]
pub struct SurfaceMotion {
    /// Animated theme component tokens for the current state.
    pub tokens: SurfaceTokens,
    /// Shadow/elevation multiplier resolved by surface treatment.
    pub elevation: f32,
}

impl SurfaceMotion {
    pub(super) fn from_theme(
        theme: &ThemePack,
        variant: SurfaceVariant,
        state: SurfaceStyleState,
    ) -> Self {
        Self {
            tokens: style::tokens_from_theme(theme, variant, state),
            elevation: match variant.treatment {
                SurfaceTreatment::Plain => 0.0,
                SurfaceTreatment::Elevated => 1.0,
            },
        }
    }
}

impl Interpolate for SurfaceTokens {
    fn interpolate_progress(from: &Self, to: &Self, progress: InterpolationProgress) -> Self {
        Self {
            bg: interpolate::color(from.bg, to.bg, progress),
            fg: interpolate::color(from.fg, to.fg, progress),
            border: interpolate::color(from.border, to.border, progress),
            border_width: interpolate::length(from.border_width, to.border_width, progress),
            radius: interpolate::radius(from.radius, to.radius, progress),
            shadow: interpolate::shadow(from.shadow, to.shadow, progress),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SurfaceMotion;
    use crate::{
        surface::{SurfaceStyleState, SurfaceVariant},
        theme::ThemePack,
    };
    use float_cmp::assert_approx_eq;

    #[test]
    fn raised_surface_uses_elevation_without_hover_shadow_boost() {
        let theme = ThemePack::adwaita();
        let idle =
            SurfaceMotion::from_theme(&theme, SurfaceVariant::RAISED, SurfaceStyleState::Idle);
        let hovered =
            SurfaceMotion::from_theme(&theme, SurfaceVariant::RAISED, SurfaceStyleState::Hovered);

        assert_approx_eq!(f32, idle.elevation, 1.0);
        assert_approx_eq!(f32, hovered.elevation, 1.0);
        assert_eq!(hovered.tokens.shadow, idle.tokens.shadow);
        assert_ne!(hovered.tokens.bg, idle.tokens.bg);
    }

    #[test]
    fn non_elevated_surfaces_have_no_elevation() {
        let theme = ThemePack::adwaita();
        let regular =
            SurfaceMotion::from_theme(&theme, SurfaceVariant::REGULAR, SurfaceStyleState::Hovered);
        let background = SurfaceMotion::from_theme(
            &theme,
            SurfaceVariant::BACKGROUND,
            SurfaceStyleState::Hovered,
        );

        assert_approx_eq!(f32, regular.elevation, 0.0);
        assert_approx_eq!(f32, background.elevation, 0.0);
    }
}
