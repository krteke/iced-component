use crate::backend::{AdwaitaBackend, MaterialBackend};

use super::AdaptiveLoadingIndicator;

/// Built-in Adwaita + Material loading indicator adapter.
pub type LoadingIndicator = AdaptiveLoadingIndicator<AdwaitaBackend, MaterialBackend>;

impl AdaptiveLoadingIndicator<AdwaitaBackend, MaterialBackend> {
    /// Creates a default Adwaita spinner and Material expressive indicator.
    #[must_use]
    pub fn new() -> Self {
        Self::from_backends(
            iced_adwaita::spinner::Spinner::new(),
            iced_material::loading_indicator::LoadingIndicator::new(),
        )
    }

    /// Returns the concrete Adwaita spinner.
    #[must_use]
    pub const fn adwaita(&self) -> &iced_adwaita::spinner::Spinner {
        self.first()
    }

    /// Returns the mutable concrete Adwaita spinner.
    pub fn adwaita_mut(&mut self) -> &mut iced_adwaita::spinner::Spinner {
        self.first_mut()
    }

    /// Returns the concrete Material loading indicator.
    #[must_use]
    pub const fn material(&self) -> &iced_material::loading_indicator::LoadingIndicator {
        self.second()
    }

    /// Returns the mutable concrete Material loading indicator.
    pub fn material_mut(&mut self) -> &mut iced_material::loading_indicator::LoadingIndicator {
        self.second_mut()
    }
}

impl Default for AdaptiveLoadingIndicator<AdwaitaBackend, MaterialBackend> {
    fn default() -> Self {
        Self::new()
    }
}
