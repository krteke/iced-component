use crate::backend::{AdwaitaBackend, MaterialBackend};

use super::{
    AdapterContext, AdapterUpdateCx, AdapterViewCx, BackendSelection, ColorScheme, ThemeFamily,
};

/// Built-in Adwaita + Material adapter context.
pub type Context = AdapterContext<AdwaitaBackend, MaterialBackend>;
/// Update context for the built-in backend pair.
pub type UpdateCx<'a> = AdapterUpdateCx<'a, AdwaitaBackend, MaterialBackend>;
/// View context for the built-in backend pair.
pub type ViewCx<'a> = AdapterViewCx<'a, AdwaitaBackend, MaterialBackend>;

impl AdapterContext<AdwaitaBackend, MaterialBackend> {
    /// Creates a light context with the selected built-in family.
    #[must_use]
    pub fn new(family: ThemeFamily) -> Self {
        Self::light(family)
    }

    /// Creates a light context with the selected built-in family.
    #[must_use]
    pub fn light(family: ThemeFamily) -> Self {
        Self::with_color_scheme(family, ColorScheme::Light)
    }

    /// Creates a dark context with the selected built-in family.
    #[must_use]
    pub fn dark(family: ThemeFamily) -> Self {
        Self::with_color_scheme(family, ColorScheme::Dark)
    }

    /// Creates built-in contexts with one shared color scheme.
    #[must_use]
    pub fn with_color_scheme(family: ThemeFamily, color_scheme: ColorScheme) -> Self {
        let (adwaita, material) = match color_scheme {
            ColorScheme::Dark => (
                iced_adwaita::Context::dark(),
                iced_material::context::Context::dark(),
            ),
            ColorScheme::Light => (
                iced_adwaita::Context::light(),
                iced_material::context::Context::light(),
            ),
        };
        Self::from_backends(family, adwaita, material)
    }

    /// Returns the active built-in family.
    #[must_use]
    pub const fn family(&self) -> ThemeFamily {
        match self.selection() {
            BackendSelection::First => ThemeFamily::Adwaita,
            BackendSelection::Second => ThemeFamily::Material,
        }
    }

    /// Returns the retained Adwaita context.
    #[must_use]
    pub const fn adwaita(&self) -> &iced_adwaita::Context {
        self.first()
    }

    /// Returns the retained Material context.
    #[must_use]
    pub const fn material(&self) -> &iced_material::context::Context {
        self.second()
    }
}

impl Default for AdapterContext<AdwaitaBackend, MaterialBackend> {
    fn default() -> Self {
        Self::new(ThemeFamily::default())
    }
}

impl AdapterUpdateCx<'_, AdwaitaBackend, MaterialBackend> {
    /// Returns the active built-in family.
    #[must_use]
    pub const fn family(&self) -> ThemeFamily {
        self.context().family()
    }

    /// Selects a built-in family without a cross-theme transition.
    pub fn set_family(&mut self, family: ThemeFamily) -> bool {
        self.set_selection(family.into())
    }

    /// Selects the other built-in family.
    pub fn toggle_family(&mut self) -> ThemeFamily {
        match self.toggle_selection() {
            BackendSelection::First => ThemeFamily::Adwaita,
            BackendSelection::Second => ThemeFamily::Material,
        }
    }

    /// Creates the concrete Adwaita update context.
    pub fn adwaita(&mut self) -> iced_adwaita::context::UpdateCx<'_> {
        self.first()
    }

    /// Creates the concrete Material update context.
    pub fn material(&mut self) -> iced_material::context::UpdateCx<'_> {
        self.second()
    }
}

impl AdapterViewCx<'_, AdwaitaBackend, MaterialBackend> {
    /// Returns the active built-in family.
    #[must_use]
    pub const fn family(&self) -> ThemeFamily {
        self.context().family()
    }

    /// Creates the concrete Adwaita view context.
    #[must_use]
    pub fn adwaita(&self) -> iced_adwaita::context::ViewCx<'_> {
        self.first()
    }

    /// Creates the concrete Material view context.
    #[must_use]
    pub fn material(&self) -> iced_material::context::ViewCx<'_> {
        self.second()
    }
}
