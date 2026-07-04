use crate::{
    component::ComponentContext,
    surface::{ResolvedSurfaceLayout, SurfaceLayout},
};

/// Stable panel layout overrides.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PanelLayout {
    pub(crate) spacing: Option<f32>,
    pub(crate) title_size: Option<f32>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct ResolvedPanelLayout {
    pub(crate) surface: ResolvedSurfaceLayout,
    pub(crate) spacing: f32,
    pub(crate) title_size: f32,
}

impl PanelLayout {
    /// Creates a panel layout override set.
    #[must_use]
    pub const fn new(spacing: Option<f32>, title_size: Option<f32>) -> Self {
        Self {
            spacing,
            title_size,
        }
    }

    /// Returns the explicit vertical spacing override.
    #[must_use]
    pub const fn spacing(self) -> Option<f32> {
        self.spacing
    }

    /// Returns the explicit title text size override.
    #[must_use]
    pub const fn title_size(self) -> Option<f32> {
        self.title_size
    }

    pub(crate) fn resolve(
        self,
        surface: SurfaceLayout,
        context: &ComponentContext,
    ) -> ResolvedPanelLayout {
        let tokens = context.theme().theme().panel.regular;

        ResolvedPanelLayout {
            surface: surface.resolve_with_padding(tokens.padding.value()),
            spacing: self.spacing.unwrap_or_else(|| tokens.spacing.value()),
            title_size: self.title_size.unwrap_or_else(|| tokens.title_size.value()),
        }
    }
}

#[cfg(test)]
mod tests {
    use float_cmp::assert_approx_eq;
    use spectrum_theme::{Length as SpectrumLength, LengthUnit};

    use super::PanelLayout;
    use crate::{component::ComponentContext, surface::SurfaceLayout};

    #[test]
    fn panel_layout_resolves_defaults_from_theme() {
        let mut context = ComponentContext::adwaita();
        context.patch_theme(|theme| {
            theme.panel.regular.padding = SpectrumLength::new(20.0, LengthUnit::Px).unwrap();
            theme.panel.regular.spacing = SpectrumLength::new(9.0, LengthUnit::Px).unwrap();
            theme.panel.regular.title_size = SpectrumLength::new(19.0, LengthUnit::Px).unwrap();
        });

        let resolved = PanelLayout::default().resolve(SurfaceLayout::empty(), &context);

        assert_approx_eq!(f32, resolved.surface.padding, 20.0);
        assert_approx_eq!(f32, resolved.spacing, 9.0);
        assert_approx_eq!(f32, resolved.title_size, 19.0);
    }

    #[test]
    fn panel_layout_overrides_theme_defaults() {
        let context = ComponentContext::adwaita();
        let resolved = PanelLayout::new(Some(7.0), Some(15.0))
            .resolve(SurfaceLayout::new(Some(13.0), None, None), &context);

        assert_approx_eq!(f32, resolved.surface.padding, 13.0);
        assert_approx_eq!(f32, resolved.spacing, 7.0);
        assert_approx_eq!(f32, resolved.title_size, 15.0);
    }
}
