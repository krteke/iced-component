//! Iced view builder for Adwaita buttons.

use iced::{
    Background, Border, Element, Length, Padding, Shadow, alignment, mouse,
    widget::{container, mouse_area, text},
};
use spectrum_theme::iced::IcedColorAdapter;

use crate::context::ViewCx;

use super::{Button, ButtonEvent, ButtonResolvedStyle, ButtonSignal, ButtonSnapshot};

/// Iced view builder for [`Button`].
pub struct ButtonView<'a, Message, Action = ()> {
    snapshot: ButtonSnapshot,
    label: String,
    layout: ResolvedButtonLayout,
    events: ButtonViewEvents<'a, Message, Action>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct ResolvedButtonLayout {
    width: Option<Length>,
    height: Option<Length>,
    padding_x: f32,
    padding_y: f32,
}

struct ButtonViewEvents<'a, Message, Action = ()> {
    on_event: Option<Box<dyn Fn(ButtonEvent<Action>) -> Message + 'a>>,
    on_press: Option<Action>,
}

impl<'a, Message, Action> ButtonViewEvents<'a, Message, Action> {
    const fn new() -> Self {
        Self {
            on_event: None,
            on_press: None,
        }
    }

    fn connected<NextAction>(
        action: NextAction,
        mapper: impl Fn(ButtonEvent<NextAction>) -> Message + 'a,
    ) -> ButtonViewEvents<'a, Message, NextAction> {
        ButtonViewEvents {
            on_event: Some(Box::new(mapper)),
            on_press: Some(action),
        }
    }
}

impl Button {
    /// Builds an Iced view for this button.
    #[must_use]
    pub fn view<'a, Message>(&self, cx: &ViewCx<'_>) -> ButtonView<'a, Message>
    where
        Message: Clone + 'a,
    {
        self.try_view(cx)
            .expect("button motion handle belongs to the provided runtime")
    }

    /// Tries to build an Iced view for this button.
    pub fn try_view<'a, Message>(
        &self,
        cx: &ViewCx<'_>,
    ) -> Result<ButtonView<'a, Message>, iced_component_core::anim::MotionError>
    where
        Message: Clone + 'a,
    {
        let theme = cx.theme().pack();
        let (width, height, padding_x, padding_y) = self.resolved_layout(theme);

        Ok(ButtonView {
            snapshot: self.snapshot(cx)?,
            label: self.content().as_text().unwrap_or_default().to_owned(),
            layout: ResolvedButtonLayout {
                width,
                height,
                padding_x,
                padding_y,
            },
            events: ButtonViewEvents::new(),
        })
    }
}

impl<'a, Message, Action> ButtonView<'a, Message, Action>
where
    Message: 'a,
{
    /// Sets the release action and maps button events into application messages.
    #[must_use]
    pub fn connect<NextAction>(
        self,
        action: NextAction,
        mapper: impl Fn(ButtonEvent<NextAction>) -> Message + 'a,
    ) -> ButtonView<'a, Message, NextAction> {
        ButtonView {
            snapshot: self.snapshot,
            label: self.label,
            layout: self.layout,
            events: ButtonViewEvents::<Message, Action>::connected(action, mapper),
        }
    }
}

impl<'a, Message, Action> From<ButtonView<'a, Message, Action>> for Element<'a, Message>
where
    Message: Clone + 'a,
    Action: 'a,
{
    fn from(view: ButtonView<'a, Message, Action>) -> Self {
        let style = ButtonResolvedStyle::from_tokens(view.snapshot.motion.tokens);

        let mut surface = container(text(view.label))
            .padding(Padding {
                top: view.layout.padding_y,
                bottom: view.layout.padding_y,
                right: view.layout.padding_x,
                left: view.layout.padding_x,
            })
            .align_x(alignment::Horizontal::Center)
            .align_y(alignment::Vertical::Center)
            .style(move |_| container_style_from_resolved(style));

        if let Some(width) = view.layout.width {
            surface = surface.width(width);
        }
        if let Some(height) = view.layout.height {
            surface = surface.height(height);
        }

        let element = surface.into();
        if view.snapshot.disabled {
            return element;
        }

        let Some(on_event) = view.events.on_event else {
            return element;
        };

        mouse_area(element)
            .on_enter(on_event(ButtonEvent::Signal(ButtonSignal::HoverEnter)))
            .on_exit(on_event(ButtonEvent::Signal(ButtonSignal::HoverExit)))
            .on_press(on_event(ButtonEvent::Signal(ButtonSignal::PressDown)))
            .on_release(match view.events.on_press {
                Some(action) => on_event(ButtonEvent::Pressed(action)),
                None => on_event(ButtonEvent::Signal(ButtonSignal::PressUp)),
            })
            .interaction(mouse::Interaction::Pointer)
            .into()
    }
}

fn container_style_from_resolved(style: ButtonResolvedStyle) -> container::Style {
    container::Style {
        text_color: Some(style.foreground.color()),
        background: Some(Background::Color(style.background.color())),
        border: Border {
            color: style.border.color(),
            width: style.border_width.value(),
            radius: style.radius.length().value().into(),
        },
        shadow: Shadow::default(),
        snap: true,
    }
}

#[cfg(test)]
mod tests {
    use iced::Element;
    use iced_component_core::anim::MotionRuntime;

    use crate::{
        Context,
        button::{Button, ButtonEvent},
        context::ViewCx,
    };

    #[test]
    fn connected_button_view_builds_app_element() {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        enum Action {
            Save,
        }

        #[derive(Clone)]
        struct Message;

        let runtime = MotionRuntime::new();
        let context = Context::light();
        let cx = ViewCx::new(&runtime, &context);
        let button = Button::suggested("Save");

        let _element: Element<'static, Message> =
            button.view(&cx).connect(Action::Save, |_| Message).into();
    }

    #[test]
    fn connected_button_view_accepts_borrowed_mapper() {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        enum Action {
            Save,
        }

        #[allow(dead_code)]
        #[derive(Clone)]
        struct Message<'a> {
            scope: &'a str,
            event: ButtonEvent<Action>,
        }

        let runtime = MotionRuntime::new();
        let context = Context::light();
        let cx = ViewCx::new(&runtime, &context);
        let button = Button::suggested("Save");
        let scope = String::from("toolbar");

        let _element: Element<'_, Message<'_>> = button
            .view(&cx)
            .connect(Action::Save, |event| Message {
                scope: scope.as_str(),
                event,
            })
            .into();
    }

    #[test]
    fn static_button_view_builds_without_event_mapper() {
        #[derive(Clone)]
        enum Message {}

        let runtime = MotionRuntime::new();
        let context = Context::light();
        let cx = ViewCx::new(&runtime, &context);
        let button = Button::new("Static");

        let _element: Element<'static, Message> = button.view(&cx).into();
    }
}
