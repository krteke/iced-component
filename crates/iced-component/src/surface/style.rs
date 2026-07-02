use crate::{
    surface::SurfaceStyleState,
    theme::{
        SurfaceBackgroundState, SurfaceBackgroundTokens, SurfaceRaisedState, SurfaceRaisedTokens,
        SurfaceRegularState, SurfaceRegularTokens, SurfaceTokens, ThemePack,
    },
};

/// Semantic role for resolving surface theme tokens.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SurfaceRole {
    /// App or page background.
    Background,
    /// Content container surface.
    Container,
}

/// Visual treatment for a surface.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SurfaceTreatment {
    /// Plain non-elevated surface.
    Plain,
    /// Elevated panel, popover, or card surface.
    Elevated,
}

/// Complete visual variant for resolving surface style.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SurfaceVariant {
    /// Semantic role.
    pub role: SurfaceRole,
    /// Visual treatment.
    pub treatment: SurfaceTreatment,
}

impl SurfaceVariant {
    /// App/page background surface.
    pub const BACKGROUND: Self = Self::new(SurfaceRole::Background, SurfaceTreatment::Plain);
    /// Regular content container.
    pub const REGULAR: Self = Self::new(SurfaceRole::Container, SurfaceTreatment::Plain);
    /// Elevated content container.
    pub const RAISED: Self = Self::new(SurfaceRole::Container, SurfaceTreatment::Elevated);

    /// Creates a surface variant from role and treatment.
    #[must_use]
    pub const fn new(role: SurfaceRole, treatment: SurfaceTreatment) -> Self {
        Self { role, treatment }
    }

    /// Returns this variant with a different semantic role.
    #[must_use]
    pub const fn with_role(mut self, role: SurfaceRole) -> Self {
        self.role = role;
        if matches!(role, SurfaceRole::Background) {
            self.treatment = SurfaceTreatment::Plain;
        }
        self
    }

    /// Returns this variant with a different visual treatment.
    #[must_use]
    pub const fn with_treatment(mut self, treatment: SurfaceTreatment) -> Self {
        self.treatment = if matches!(self.role, SurfaceRole::Background) {
            SurfaceTreatment::Plain
        } else {
            treatment
        };
        self
    }

    /// Returns this variant as an app/page background.
    #[must_use]
    pub const fn set_background(self) -> Self {
        Self::BACKGROUND
    }

    /// Returns this variant as a regular container.
    #[must_use]
    pub const fn set_regular(self) -> Self {
        Self::REGULAR
    }

    /// Returns this variant as an elevated container.
    #[must_use]
    pub const fn set_raised(self) -> Self {
        Self::RAISED
    }

    /// Returns this variant with plain treatment.
    #[must_use]
    pub const fn set_plain(self) -> Self {
        self.with_treatment(SurfaceTreatment::Plain)
    }

    /// Returns this variant with elevated treatment.
    #[must_use]
    pub const fn set_elevated(self) -> Self {
        self.with_treatment(SurfaceTreatment::Elevated)
    }
}

pub(crate) fn tokens_from_theme(
    theme: &ThemePack,
    variant: SurfaceVariant,
    state: SurfaceStyleState,
) -> SurfaceTokens {
    *match (variant.role, variant.treatment) {
        (SurfaceRole::Background, _) => theme.surface.background.get_style(state),
        (SurfaceRole::Container, SurfaceTreatment::Plain) => theme.surface.regular.get_style(state),
        (SurfaceRole::Container, SurfaceTreatment::Elevated) => {
            theme.surface.raised.get_style(state)
        }
    }
}

trait SurfaceStateTokens {
    fn get_style(&self, state: SurfaceStyleState) -> &SurfaceTokens;
}

macro_rules! impl_surface_state_tokens {
    ($tokens:ty, $state:ty) => {
        impl SurfaceStateTokens for $tokens {
            fn get_style(&self, state: SurfaceStyleState) -> &SurfaceTokens {
                self.get(match state {
                    SurfaceStyleState::Idle => <$state>::Idle,
                    SurfaceStyleState::Hovered => <$state>::Hover,
                })
            }
        }
    };
}

impl_surface_state_tokens!(SurfaceBackgroundTokens, SurfaceBackgroundState);
impl_surface_state_tokens!(SurfaceRegularTokens, SurfaceRegularState);
impl_surface_state_tokens!(SurfaceRaisedTokens, SurfaceRaisedState);

#[cfg(test)]
mod tests {
    use float_cmp::assert_approx_eq;
    use spectrum_theme::Color;

    use super::{SurfaceRole, SurfaceTreatment, SurfaceVariant, tokens_from_theme};
    use crate::{surface::SurfaceStyleState, theme::ThemePack};

    #[test]
    fn background_surface_uses_background_state_tokens() {
        let theme = ThemePack::adwaita();
        let tokens = tokens_from_theme(&theme, SurfaceVariant::BACKGROUND, SurfaceStyleState::Idle);

        assert_eq!(tokens.bg, theme.surface.background.idle.bg);
        assert_eq!(tokens.fg, theme.surface.background.idle.fg);
        assert_eq!(tokens.shadow.color().alpha(), 0);
        assert_approx_eq!(f32, tokens.border_width.value(), 0.0);
    }

    #[test]
    fn raised_surface_uses_raised_state_tokens() {
        let theme = ThemePack::adwaita();
        let tokens = tokens_from_theme(&theme, SurfaceVariant::RAISED, SurfaceStyleState::Idle);

        assert_eq!(tokens.bg, theme.surface.raised.idle.bg);
        assert_eq!(tokens.border_width, theme.surface.raised.idle.border_width);
        assert_eq!(tokens.radius, theme.surface.raised.idle.radius);
        assert_eq!(tokens.shadow, theme.surface.raised.idle.shadow);
    }

    #[test]
    fn hover_state_uses_component_state_tokens() {
        let mut theme = ThemePack::adwaita();
        let hover = Color::new(230, 240, 250);
        theme.surface.regular.hover.bg = hover;

        let tokens = tokens_from_theme(&theme, SurfaceVariant::REGULAR, SurfaceStyleState::Hovered);

        assert_eq!(tokens.bg, hover);
    }

    #[test]
    fn surface_variant_keeps_role_and_treatment_separate() {
        let variant = SurfaceVariant::new(SurfaceRole::Container, SurfaceTreatment::Elevated);

        assert_eq!(variant.role, SurfaceRole::Container);
        assert_eq!(variant.treatment, SurfaceTreatment::Elevated);
        assert_eq!(
            SurfaceVariant::BACKGROUND.set_elevated(),
            SurfaceVariant::BACKGROUND
        );
    }
}
