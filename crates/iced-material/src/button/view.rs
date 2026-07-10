//! Iced view builder for Material buttons.

use iced::{
    Background, Border, Color, Element, Length, Padding, Shadow, Vector, alignment, mouse,
    widget::{Space, Stack, container, mouse_area},
};
use spectrum_theme::iced::IcedColorAdapter;

use crate::context::ViewCx;

use super::{Button, ButtonEvent, ButtonSignal, ButtonSnapshot};

/// Iced view builder for a Material [`Button`].
pub struct ButtonView<'a, Message, Action = ()> {
    snapshot: ButtonSnapshot,
    content: Element<'a, Message>,
    layout: ButtonLayout,
    events: ButtonViewEvents<'a, Message, Action>,
}

#[derive(Clone, Copy, Debug)]
struct ButtonLayout {
    height: f32,
    leading_padding: f32,
    trailing_padding: f32,
}

struct ButtonViewEvents<'a, Message, Action> {
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

    fn connect<NextAction>(
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
    pub fn view<'a, Message>(&'a self, cx: &ViewCx<'_>) -> ButtonView<'a, Message>
    where
        Message: Clone + 'a,
    {
        self.try_view(cx)
            .expect("button motion handle belongs to the provided runtime")
    }

    /// Tries to build an Iced view for this button.
    pub fn try_view<'a, Message>(
        &'a self,
        cx: &ViewCx<'_>,
    ) -> Result<ButtonView<'a, Message>, iced_component_core::anim::MotionError>
    where
        Message: Clone + 'a,
    {
        let tokens = &cx.theme().pack().button;

        Ok(ButtonView {
            snapshot: self.snapshot(cx)?,
            content: Space::new().into(),
            layout: ButtonLayout {
                height: tokens.container_height.value(),
                leading_padding: tokens.leading_padding.value(),
                trailing_padding: tokens.trailing_padding.value(),
            },
            events: ButtonViewEvents::new(),
        })
    }
}

impl<'a, Message, Action> ButtonView<'a, Message, Action>
where
    Message: 'a,
{
    /// Replaces the rendered button content for this view.
    #[must_use]
    pub fn content(mut self, content: impl Into<Element<'a, Message>>) -> Self {
        self.content = content.into();
        self
    }

    /// Maps component events into application messages and sets the release action.
    #[must_use]
    pub fn connect<NextAction>(
        self,
        action: NextAction,
        mapper: impl Fn(ButtonEvent<NextAction>) -> Message + 'a,
    ) -> ButtonView<'a, Message, NextAction> {
        ButtonView {
            snapshot: self.snapshot,
            content: self.content,
            layout: self.layout,
            events: ButtonViewEvents::<Message, Action>::connect(action, mapper),
        }
    }
}

impl<'a, Message, Action> From<ButtonView<'a, Message, Action>> for Element<'a, Message>
where
    Message: Clone + 'a,
    Action: 'a,
{
    fn from(view: ButtonView<'a, Message, Action>) -> Self {
        let ButtonView {
            snapshot,
            content,
            layout,
            events,
        } = view;
        let surface = button_surface(snapshot, layout, content);

        if snapshot.disabled {
            return surface;
        }

        let Some(on_event) = events.on_event else {
            return surface;
        };
        let on_release = match events.on_press {
            Some(action) => on_event(ButtonEvent::Pressed(action)),
            None => on_event(ButtonEvent::Signal(ButtonSignal::PressUp)),
        };

        mouse_area(surface)
            .on_enter(on_event(ButtonEvent::Signal(ButtonSignal::HoverEnter)))
            .on_exit(on_event(ButtonEvent::Signal(ButtonSignal::HoverExit)))
            .on_press(on_event(ButtonEvent::Signal(ButtonSignal::PressDown)))
            .on_release(on_release)
            .interaction(mouse::Interaction::Pointer)
            .into()
    }
}

fn button_surface<'a, Message>(
    snapshot: ButtonSnapshot,
    layout: ButtonLayout,
    content: Element<'a, Message>,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let visual = snapshot.visual;
    let state_layer = container(Space::new().width(Length::Fill).height(Length::Fill))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_| container::Style {
            background: Some(Background::Color(color_with_opacity(
                visual.state_layer,
                visual.state_layer_opacity,
            ))),
            border: Border {
                radius: visual.radius.into(),
                ..Border::default()
            },
            ..container::Style::default()
        });
    let content = container(content)
        .height(Length::Fixed(layout.height))
        .padding(Padding {
            top: 0.0,
            right: layout.trailing_padding,
            bottom: 0.0,
            left: layout.leading_padding,
        })
        .align_x(alignment::Horizontal::Center)
        .align_y(alignment::Vertical::Center);

    let layers = Stack::new().push(content).push_under(state_layer);

    container(layers)
        .height(Length::Fixed(layout.height))
        .align_x(alignment::Horizontal::Center)
        .align_y(alignment::Vertical::Center)
        .style(move |_| container::Style {
            text_color: Some(color_with_opacity(
                visual.foreground,
                visual.foreground_opacity,
            )),
            background: Some(Background::Color(color_with_opacity(
                visual.background,
                visual.background_opacity,
            ))),
            border: Border {
                color: color_with_opacity(visual.border, visual.border_opacity),
                width: visual.border_width,
                radius: visual.radius.into(),
            },
            shadow: Shadow {
                color: color_with_opacity(visual.shadow, visual.shadow_opacity),
                offset: Vector::new(0.0, visual.shadow_y),
                blur_radius: visual.shadow_blur,
            },
            snap: true,
        })
        .into()
}

fn color_with_opacity(color: spectrum_theme::Color, opacity: f32) -> Color {
    let mut color = color.color();
    color.a *= opacity;
    color
}

#[cfg(test)]
mod tests {
    use iced::{Element, widget::text};
    use iced_component_core::anim::MotionRuntime;

    use crate::{
        button::{Button, ButtonEvent},
        context::{Context, ViewCx},
    };

    #[test]
    fn connected_view_builds_an_iced_element() {
        #[derive(Clone, Copy)]
        enum Action {
            Save,
        }

        type Message = ButtonEvent<Action>;

        let runtime = MotionRuntime::new();
        let context = Context::light();
        let cx = ViewCx::new(&runtime, &context);
        let button = Button::filled();

        let _: Element<'_, Message> = button
            .view(&cx)
            .content(text("Save"))
            .connect(Action::Save, |event| event)
            .into();
    }

    #[test]
    fn disabled_view_builds_without_an_event_mapper() {
        #[derive(Clone)]
        enum Message {}

        let runtime = MotionRuntime::new();
        let context = Context::light();
        let cx = ViewCx::new(&runtime, &context);
        let button = Button::outlined().disabled(true);

        let _: Element<'_, Message> = button.view(&cx).content(text("Disabled")).into();
    }
}
