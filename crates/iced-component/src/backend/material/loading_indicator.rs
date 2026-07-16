use iced::{Element, time::Instant};
use iced_component_core::anim::MotionError;

use crate::backend::{MaterialBackend, loading_indicator::LoadingIndicatorBackend};

impl LoadingIndicatorBackend for MaterialBackend {
    type LoadingIndicator = iced_material::loading_indicator::LoadingIndicator;

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

    fn register(indicator: &mut Self::LoadingIndicator, cx: &mut Self::UpdateCx<'_>) {
        indicator.register(cx);
    }

    fn sync(
        indicator: &mut Self::LoadingIndicator,
        cx: &mut Self::UpdateCx<'_>,
    ) -> Result<bool, MotionError> {
        indicator.sync(cx)
    }

    fn is_registered(indicator: &Self::LoadingIndicator) -> bool {
        indicator.is_registered()
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
