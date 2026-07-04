//! Iced view builder for animated spinners.

use aura_anim::prelude::MotionError;
use iced::{
    Element, Length,
    widget::{container, shader},
};
use spectrum_theme::iced::IcedColorAdapter;

use super::{Spinner, SpinnerSnapshot, shader::SpinnerShader};
use crate::component::ComponentViewCx;

/// Iced view builder for [`Spinner`].
pub struct SpinnerView {
    snapshot: SpinnerSnapshot,
}

impl Spinner {
    /// Builds an Iced view for this spinner.
    #[must_use]
    pub fn view(&self, cx: &ComponentViewCx<'_>) -> SpinnerView {
        self.try_view(cx)
            .expect("spinner motion handle belongs to the provided runtime")
    }

    /// Tries to build an Iced view for this spinner.
    pub fn try_view(&self, cx: &ComponentViewCx<'_>) -> Result<SpinnerView, MotionError> {
        Ok(SpinnerView {
            snapshot: self.snapshot(cx)?,
        })
    }
}

impl<Message> From<SpinnerView> for Element<'_, Message>
where
    Message: 'static,
{
    fn from(view: SpinnerView) -> Self {
        let snapshot = view.snapshot;
        let tokens = snapshot.tokens;
        let size = Length::Fixed(snapshot.size);

        container(
            shader(SpinnerShader {
                motion: snapshot.motion,
                fg: tokens.fg.color(),
                track: tokens.track.color(),
                stroke_width: snapshot.stroke_width,
            })
            .width(size)
            .height(size),
        )
        .width(size)
        .height(size)
        .center(size)
        .into()
    }
}
