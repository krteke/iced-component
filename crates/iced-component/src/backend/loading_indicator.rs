//! Backend capability for indeterminate loading feedback.

use iced::{Element, time::Instant};
use iced_component_core::anim::MotionError;

use super::ThemeBackend;

/// Loading indicator capability implemented by a themed backend.
///
/// The common adapter models continuous indeterminate feedback. Additional
/// modes, such as Material contained or determinate indicators, remain on the
/// concrete backend component.
pub trait LoadingIndicatorBackend: ThemeBackend {
    /// Concrete persistent loading indicator type.
    type LoadingIndicator;

    /// Applies an explicit square size.
    fn size(indicator: Self::LoadingIndicator, size: f32) -> Self::LoadingIndicator;

    /// Sets an explicit square size.
    fn set_size(indicator: &mut Self::LoadingIndicator, size: f32);

    /// Clears the explicit square size.
    fn clear_size(indicator: &mut Self::LoadingIndicator);

    /// Returns the explicit square size.
    fn explicit_size(indicator: &Self::LoadingIndicator) -> Option<f32>;

    /// Registers any component-owned visual motion.
    fn register(indicator: &mut Self::LoadingIndicator, cx: &mut Self::UpdateCx<'_>);

    /// Synchronizes component-owned visuals with the current backend theme.
    fn sync(
        indicator: &mut Self::LoadingIndicator,
        cx: &mut Self::UpdateCx<'_>,
    ) -> Result<bool, MotionError>;

    /// Returns whether all required component-owned motion is ready.
    fn is_registered(indicator: &Self::LoadingIndicator) -> bool;

    /// Advances the backend's continuous loading timeline.
    fn advance(indicator: &mut Self::LoadingIndicator, now: Instant);

    /// Restarts the backend's loading timeline.
    fn reset(indicator: &mut Self::LoadingIndicator);

    /// Builds the concrete loading indicator view.
    fn view<Message>(
        indicator: &Self::LoadingIndicator,
        cx: &Self::ViewCx<'_>,
    ) -> Element<'static, Message>
    where
        Message: 'static;
}
