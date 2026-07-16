use aura_anim::core::{interpolate::InterpolationProgress, traits::Interpolate};
use aura_anim::prelude::Animatable;
use spectrum_theme::Color;

use crate::theme::{interpolate::color, tokens::LoadingIndicatorTokens};

/// Animatable colors resolved from the active Material theme pack.
#[derive(Clone, Copy, Debug, PartialEq, Animatable)]
pub struct LoadingIndicatorMotion {
    colors: ThemeColors,
}

impl LoadingIndicatorMotion {
    pub(super) fn from_tokens(tokens: LoadingIndicatorTokens) -> Self {
        Self {
            colors: ThemeColors {
                active: tokens.active,
                container: tokens.container,
                contained_active: tokens.contained_active,
            },
        }
    }

    /// Returns the interpolated uncontained active color.
    #[must_use]
    pub const fn active(&self) -> Color {
        self.colors.active
    }

    /// Returns the interpolated container color.
    #[must_use]
    pub const fn container(&self) -> Color {
        self.colors.container
    }

    /// Returns the interpolated contained active color.
    #[must_use]
    pub const fn contained_active(&self) -> Color {
        self.colors.contained_active
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct ThemeColors {
    active: Color,
    container: Color,
    contained_active: Color,
}

impl Interpolate for ThemeColors {
    fn interpolate_progress(from: &Self, to: &Self, progress: InterpolationProgress) -> Self {
        Self {
            active: color(from.active, to.active, progress),
            container: color(from.container, to.container, progress),
            contained_active: color(from.contained_active, to.contained_active, progress),
        }
    }
}
