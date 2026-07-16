//! Extensible themed-backend contracts used by the adapter.

#[cfg(feature = "adwaita")]
mod adwaita;
/// Capability contract for themed loading indicators.
pub mod loading_indicator;
#[cfg(feature = "material")]
mod material;

use iced::Element;
use iced_component_core::anim::{MotionError, MotionRuntime};

use crate::{
    button::{ButtonEvent, ButtonOutcome},
    context::ColorScheme,
};

#[cfg(feature = "adwaita")]
pub use adwaita::AdwaitaBackend;
#[cfg(feature = "material")]
pub use material::MaterialBackend;

/// A concrete themed component library that can participate in an adapter.
///
/// Implementations own the conversion between the adapter's common settings
/// and their concrete context API. Theme-specific controls remain available
/// through the associated update and view context types.
pub trait ThemeBackend: Sized {
    /// Persistent context owned by an adapter context.
    type Context;
    /// Mutable context facade used during application updates.
    type UpdateCx<'a>
    where
        Self: 'a;
    /// Read-only context facade used while building views.
    type ViewCx<'a>
    where
        Self: 'a;

    /// Creates this backend's mutable context facade.
    fn update_cx<'a>(
        runtime: &'a mut MotionRuntime,
        context: &'a mut Self::Context,
    ) -> Self::UpdateCx<'a>;

    /// Creates this backend's read-only context facade.
    fn view_cx<'a>(runtime: &'a MotionRuntime, context: &'a Self::Context) -> Self::ViewCx<'a>;

    /// Returns the backend's active light or dark color scheme.
    fn color_scheme(context: &Self::Context) -> ColorScheme;

    /// Returns whether this backend reduces non-essential motion.
    fn reduce_motion(context: &Self::Context) -> bool;

    /// Applies a light or dark color scheme.
    fn set_color_scheme(cx: &mut Self::UpdateCx<'_>, color_scheme: ColorScheme) -> bool;

    /// Applies the reduced-motion preference.
    fn set_reduce_motion(cx: &mut Self::UpdateCx<'_>, reduce_motion: bool);
}

/// Operations required from a concrete themed button view builder.
pub trait ButtonViewBackend<'a, Message>: Into<Element<'a, Message>>
where
    Message: Clone + 'a,
{
    /// Replaces the content rendered by this view.
    #[must_use]
    fn content(self, content: Element<'a, Message>) -> Self;

    /// Maps theme-independent button events into application messages.
    #[must_use]
    fn on_event<F>(self, mapper: F) -> Self
    where
        F: Fn(ButtonEvent) -> Message + 'a;
}

/// Button capability implemented by a [`ThemeBackend`].
pub trait ButtonBackend: ThemeBackend {
    /// Concrete persistent button type.
    type Button;
    /// Concrete temporary button view type.
    type View<'a, Message>: ButtonViewBackend<'a, Message>
    where
        Self: 'a,
        Message: Clone + 'a;

    /// Applies an initial disabled state without requiring a runtime.
    fn disabled(button: Self::Button, disabled: bool) -> Self::Button;

    /// Explicitly registers the button's motion state.
    fn register(button: &mut Self::Button, cx: &mut Self::UpdateCx<'_>);

    /// Synchronizes the button with its current concrete theme.
    fn sync(button: &mut Self::Button, cx: &mut Self::UpdateCx<'_>) -> Result<bool, MotionError>;

    /// Updates the button's disabled state.
    fn set_disabled(
        button: &mut Self::Button,
        disabled: bool,
        cx: &mut Self::UpdateCx<'_>,
    ) -> Result<bool, MotionError>;

    /// Applies a rendered event to the concrete button.
    fn update_event(
        button: &mut Self::Button,
        event: ButtonEvent,
        cx: &mut Self::UpdateCx<'_>,
    ) -> Result<ButtonOutcome, MotionError>;

    /// Builds the concrete themed button view.
    fn view<'a, Message>(
        button: &'a Self::Button,
        cx: &Self::ViewCx<'_>,
    ) -> Self::View<'a, Message>
    where
        Message: Clone + 'a;

    /// Returns whether this button's motion state is registered.
    fn is_registered(button: &Self::Button) -> bool;
}
