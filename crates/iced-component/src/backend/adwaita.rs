use iced::Element;
use iced_component_core::anim::{MotionError, MotionRuntime};

use super::{ButtonBackend, ButtonViewBackend, ThemeBackend};
use crate::{
    button::{ButtonEvent, ButtonOutcome},
    context::ColorScheme,
};

/// Adapter marker for the `iced-adwaita` backend.
#[derive(Clone, Copy, Debug, Default)]
pub struct AdwaitaBackend;

impl ThemeBackend for AdwaitaBackend {
    type Context = iced_adwaita::Context;
    type UpdateCx<'a> = iced_adwaita::context::UpdateCx<'a>;
    type ViewCx<'a> = iced_adwaita::context::ViewCx<'a>;

    fn update_cx<'a>(
        runtime: &'a mut MotionRuntime,
        context: &'a mut Self::Context,
    ) -> Self::UpdateCx<'a> {
        Self::UpdateCx::new(runtime, context)
    }

    fn view_cx<'a>(runtime: &'a MotionRuntime, context: &'a Self::Context) -> Self::ViewCx<'a> {
        Self::ViewCx::new(runtime, context)
    }

    fn color_scheme(context: &Self::Context) -> ColorScheme {
        match context.theme().mode() {
            iced_adwaita::context::ThemeMode::Dark => ColorScheme::Dark,
            iced_adwaita::context::ThemeMode::Light => ColorScheme::Light,
        }
    }

    fn reduce_motion(context: &Self::Context) -> bool {
        context.reduce_motion()
    }

    fn set_color_scheme(cx: &mut Self::UpdateCx<'_>, color_scheme: ColorScheme) -> bool {
        if Self::color_scheme(cx.context()) == color_scheme {
            return false;
        }
        cx.toggle_theme();
        true
    }

    fn set_reduce_motion(cx: &mut Self::UpdateCx<'_>, reduce_motion: bool) {
        cx.set_reduce_motion(reduce_motion);
    }
}

impl ButtonBackend for AdwaitaBackend {
    type Button = iced_adwaita::button::Button;
    type View<'a, Message>
        = iced_adwaita::button::ButtonView<'a, Message>
    where
        Message: Clone + 'a;

    fn disabled(button: Self::Button, disabled: bool) -> Self::Button {
        button.disabled(disabled)
    }

    fn register(button: &mut Self::Button, cx: &mut Self::UpdateCx<'_>) {
        button.register(cx);
    }

    fn sync(button: &mut Self::Button, cx: &mut Self::UpdateCx<'_>) -> Result<bool, MotionError> {
        button.sync(cx)
    }

    fn set_disabled(
        button: &mut Self::Button,
        disabled: bool,
        cx: &mut Self::UpdateCx<'_>,
    ) -> Result<bool, MotionError> {
        button.set_disabled(disabled, cx)
    }

    fn update_event(
        button: &mut Self::Button,
        event: ButtonEvent,
        cx: &mut Self::UpdateCx<'_>,
    ) -> Result<ButtonOutcome, MotionError> {
        button.update_event(event, cx)
    }

    fn view<'a, Message>(button: &'a Self::Button, cx: &Self::ViewCx<'_>) -> Self::View<'a, Message>
    where
        Message: Clone + 'a,
    {
        button.view(cx)
    }

    fn is_registered(button: &Self::Button) -> bool {
        button.is_registered()
    }
}

impl<'a, Message> ButtonViewBackend<'a, Message> for iced_adwaita::button::ButtonView<'a, Message>
where
    Message: Clone + 'a,
{
    fn content(self, content: Element<'a, Message>) -> Self {
        self.content(content)
    }

    fn on_event<F>(self, mapper: F) -> Self
    where
        F: Fn(ButtonEvent) -> Message + 'a,
    {
        self.on_event(mapper)
    }
}
