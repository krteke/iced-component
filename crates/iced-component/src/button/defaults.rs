use crate::backend::{AdwaitaBackend, MaterialBackend};

use super::{AdaptiveButton, AdaptiveButtonView, ButtonStyle};

/// Built-in Adwaita + Material button adapter.
pub type Button = AdaptiveButton<AdwaitaBackend, MaterialBackend>;
/// View builder for the built-in button adapter.
pub type ButtonView<'a, Message> = AdaptiveButtonView<'a, Message, AdwaitaBackend, MaterialBackend>;

impl AdaptiveButton<AdwaitaBackend, MaterialBackend> {
    /// Creates a built-in adapter button from a non-persistent style preset.
    #[must_use]
    pub fn new(style: ButtonStyle) -> Self {
        let (adwaita, material) = super::style::buttons(style);
        Self::from_backends(adwaita, material)
    }

    /// Creates the primary action preset.
    #[must_use]
    pub fn primary() -> Self {
        Self::new(ButtonStyle::Primary)
    }

    /// Creates the secondary action preset.
    #[must_use]
    pub fn secondary() -> Self {
        Self::new(ButtonStyle::Secondary)
    }

    /// Creates the low-emphasis action preset.
    #[must_use]
    pub fn quiet() -> Self {
        Self::new(ButtonStyle::Quiet)
    }

    /// Returns the concrete Adwaita button.
    #[must_use]
    pub const fn adwaita(&self) -> &iced_adwaita::button::Button {
        self.first()
    }

    /// Returns the mutable concrete Adwaita button.
    pub fn adwaita_mut(&mut self) -> &mut iced_adwaita::button::Button {
        self.first_mut()
    }

    /// Returns the concrete Material button.
    #[must_use]
    pub const fn material(&self) -> &iced_material::button::Button {
        self.second()
    }

    /// Returns the mutable concrete Material button.
    pub fn material_mut(&mut self) -> &mut iced_material::button::Button {
        self.second_mut()
    }
}

impl Default for AdaptiveButton<AdwaitaBackend, MaterialBackend> {
    fn default() -> Self {
        Self::new(ButtonStyle::Standard)
    }
}
