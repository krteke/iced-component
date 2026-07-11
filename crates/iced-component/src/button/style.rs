//! Construction-only presets for the built-in backend pair.

use iced_adwaita::button::{ButtonContentLayout, ButtonTreatment, ButtonVariant as AdwaitaVariant};
use iced_material::button::ButtonVariant as MaterialVariant;

/// Common construction preset for the built-in Adwaita + Material button.
///
/// The adapter maps this value once and does not retain it. Concrete variants
/// can be changed independently after construction.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ButtonStyle {
    /// Normal emphasis in the active design language.
    #[default]
    Standard,
    /// Primary or suggested action.
    Primary,
    /// Secondary or tonal action.
    Secondary,
    /// Low-emphasis flat or text action.
    Quiet,
}

pub(super) fn buttons(
    style: ButtonStyle,
) -> (iced_adwaita::button::Button, iced_material::button::Button) {
    let (adwaita, material) = match style {
        ButtonStyle::Standard => (AdwaitaVariant::STANDARD, MaterialVariant::ELEVATED),
        ButtonStyle::Primary => (AdwaitaVariant::SUGGESTED, MaterialVariant::FILLED),
        ButtonStyle::Secondary => (AdwaitaVariant::STANDARD, MaterialVariant::FILLED_TONAL),
        ButtonStyle::Quiet => (
            AdwaitaVariant::STANDARD.with_treatment(ButtonTreatment::Flat),
            MaterialVariant::TEXT,
        ),
    };

    (
        iced_adwaita::button::Button::empty()
            .with_content_layout(ButtonContentLayout::Text)
            .with_variant(adwaita),
        iced_material::button::Button::with_variant(material),
    )
}
