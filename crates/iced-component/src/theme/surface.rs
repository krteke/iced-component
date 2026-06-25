use spectrum_theme::{Color, Radius, ShadowLayer};

use crate::{
    component::ComponentContext,
    theme::{ThemeContext, ThemePack},
};

/// Visual role for resolving surface theme tokens.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SurfaceRole {
    /// App or page background.
    Background,
    /// Regular component surface.
    Regular,
    /// Raised panel, popover, or interactive container.
    Raised,
}

/// Resolved theme tokens used by surface-like components.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SurfaceStyleTokens {
    /// Surface fill color.
    pub background: Color,
    /// Default foreground color.
    pub foreground: Color,
    /// Surface border color.
    pub border: Color,
    /// Surface corner radius.
    pub radius: Radius,
    /// Surface shadow.
    pub shadow: Option<ShadowLayer>,
}

impl SurfaceStyleTokens {
    /// Resolves surface style tokens from the theme baseline.
    #[must_use]
    pub fn from_theme(theme: &ThemePack, role: SurfaceRole) -> Self {
        let (background, radius, shadow) = match role {
            SurfaceRole::Background => (theme.app.bg, theme.surface.base.radius, None),
            SurfaceRole::Regular => (theme.surface.base.bg, theme.surface.base.radius, None),
            SurfaceRole::Raised => (
                theme.surface.raised.bg,
                theme.surface.raised.radius,
                Some(theme.surface.raised.shadow),
            ),
        };

        Self {
            background,
            foreground: match role {
                SurfaceRole::Background => theme.app.fg,
                SurfaceRole::Regular => theme.surface.base.fg,
                SurfaceRole::Raised => theme.surface.raised.fg,
            },
            border: match role {
                SurfaceRole::Background | SurfaceRole::Regular => theme.surface.base.border,
                SurfaceRole::Raised => theme.surface.raised.border,
            },
            radius,
            shadow,
        }
    }

    /// Resolves surface style tokens from a theme context.
    #[must_use]
    pub fn from_context(context: &ThemeContext, role: SurfaceRole) -> Self {
        Self::from_theme(context.theme(), role)
    }

    /// Resolves surface style tokens from a component context.
    #[must_use]
    pub fn from_component_context(context: &ComponentContext, role: SurfaceRole) -> Self {
        Self::from_context(context.theme(), role)
    }
}

#[cfg(test)]
mod tests {
    use spectrum_theme::Color;

    use crate::{
        component::ComponentContext,
        theme::{ThemeContext, ThemePack},
    };

    use super::{SurfaceRole, SurfaceStyleTokens};

    #[test]
    fn background_surface_has_no_shadow() {
        let theme = ThemePack::adwaita();
        let style = SurfaceStyleTokens::from_theme(&theme, SurfaceRole::Background);

        assert_eq!(style.background, theme.app.bg);
        assert_eq!(style.foreground, theme.app.fg);
        assert_eq!(style.shadow, None);
    }

    #[test]
    fn raised_surface_uses_theme_elevation() {
        let theme = ThemePack::adwaita();
        let style = SurfaceStyleTokens::from_theme(&theme, SurfaceRole::Raised);

        assert_eq!(style.background, theme.surface.raised.bg);
        assert_eq!(style.radius, theme.surface.raised.radius);
        assert_eq!(style.shadow, Some(theme.surface.raised.shadow));
    }

    #[test]
    fn surface_style_uses_scoped_context() {
        let context = ThemeContext::from_theme(&ThemePack::adwaita());
        let scoped_bg = Color::new(238, 244, 250);
        let scoped = context.scoped(|theme| theme.surface.raised.bg = scoped_bg);

        let style = SurfaceStyleTokens::from_context(&scoped, SurfaceRole::Raised);

        assert_eq!(style.background, scoped_bg);
        assert_ne!(context.theme().surface.raised.bg, scoped_bg);
    }

    #[test]
    fn surface_style_uses_component_context() {
        let scoped_bg = Color::new(238, 244, 250);
        let context =
            ComponentContext::current().scoped_theme(|theme| theme.surface.raised.bg = scoped_bg);

        let style = SurfaceStyleTokens::from_component_context(&context, SurfaceRole::Raised);

        assert_eq!(style.background, scoped_bg);
    }
}
