use iced::{Element, time::Instant};
use iced_component_core::anim::MotionError;

use crate::backend::{AdwaitaBackend, loading_indicator::LoadingIndicatorBackend};

impl LoadingIndicatorBackend for AdwaitaBackend {
    type LoadingIndicator = iced_adwaita::spinner::Spinner;

    fn size(indicator: Self::LoadingIndicator, size: f32) -> Self::LoadingIndicator {
        indicator.size(size)
    }

    fn set_size(indicator: &mut Self::LoadingIndicator, size: f32) {
        indicator.set_size(size);
    }

    fn clear_size(indicator: &mut Self::LoadingIndicator) {
        indicator.clear_size();
    }

    fn explicit_size(indicator: &Self::LoadingIndicator) -> Option<f32> {
        indicator.explicit_size()
    }

    fn register(_indicator: &mut Self::LoadingIndicator, _cx: &mut Self::UpdateCx<'_>) {}

    fn sync(
        _indicator: &mut Self::LoadingIndicator,
        _cx: &mut Self::UpdateCx<'_>,
    ) -> Result<bool, MotionError> {
        Ok(false)
    }

    fn is_registered(_indicator: &Self::LoadingIndicator) -> bool {
        true
    }

    fn advance(indicator: &mut Self::LoadingIndicator, now: Instant) {
        indicator.advance(now);
    }

    fn reset(indicator: &mut Self::LoadingIndicator) {
        indicator.reset();
    }

    fn view<Message>(
        indicator: &Self::LoadingIndicator,
        cx: &Self::ViewCx<'_>,
    ) -> Element<'static, Message>
    where
        Message: 'static,
    {
        indicator.view(cx)
    }
}
