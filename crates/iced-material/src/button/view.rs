//! Iced view builder for Material buttons.

use iced::{
    Element, Length, alignment,
    widget::{Space, container},
};

use crate::context::ViewCx;

use super::{
    Button, ButtonEvent, ButtonSnapshot,
    widget::{ButtonEvents, ButtonLayout, MaterialButtonWidget},
};

/// Iced view builder for a Material [`Button`].
pub struct ButtonView<'a, Message> {
    snapshot: ButtonSnapshot,
    content: Element<'a, Message>,
    layout: ButtonLayout,
    ripple_enabled: bool,
    events: ButtonViewEvents<'a, Message>,
}

struct ButtonViewEvents<'a, Message> {
    events: Option<ButtonEvents<'a, Message>>,
}

impl<'a, Message> ButtonViewEvents<'a, Message> {
    const fn new() -> Self {
        Self { events: None }
    }

    fn on_event(mapper: impl Fn(ButtonEvent) -> Message + 'a) -> Self {
        Self {
            events: Some(ButtonEvents::new(mapper)),
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
            ripple_enabled: !cx.reduce_motion(),
            events: ButtonViewEvents::new(),
        })
    }
}

impl<'a, Message> ButtonView<'a, Message>
where
    Message: 'a,
{
    /// Replaces the rendered button content for this view.
    #[must_use]
    pub fn content(mut self, content: impl Into<Element<'a, Message>>) -> Self {
        self.content = content.into();
        self
    }

    /// Maps rendered button events into application messages.
    #[must_use]
    pub fn on_event(mut self, mapper: impl Fn(ButtonEvent) -> Message + 'a) -> Self {
        self.events = ButtonViewEvents::on_event(mapper);
        self
    }
}

impl<'a, Message> From<ButtonView<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(view: ButtonView<'a, Message>) -> Self {
        let ButtonView {
            snapshot,
            content,
            layout,
            ripple_enabled,
            events,
        } = view;
        let events = if snapshot.disabled {
            None
        } else {
            events.events
        };
        let content = container(content)
            .height(Length::Fixed(layout.height))
            .align_y(alignment::Vertical::Center)
            .into();

        MaterialButtonWidget::new(snapshot, layout, content, events, ripple_enabled).into()
    }
}

#[cfg(test)]
mod tests {
    use iced::{Element, widget::text};
    use iced_component_core::anim::MotionRuntime;

    use crate::{
        button::{Button, ButtonEvent},
        context::{Context, UpdateCx, ViewCx},
    };

    #[test]
    fn event_mapped_view_builds_an_iced_element() {
        type Message = ButtonEvent;

        let runtime = MotionRuntime::new();
        let context = Context::light();
        let cx = ViewCx::new(&runtime, &context);
        let button = Button::filled();

        let _: Element<'_, Message> = button
            .view(&cx)
            .content(text("Save"))
            .on_event(|event| event)
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

    #[test]
    fn reduced_motion_disables_the_widget_ripple() {
        let mut runtime = MotionRuntime::new();
        let mut context = Context::light();
        UpdateCx::new(&mut runtime, &mut context).set_reduce_motion(true);
        let cx = ViewCx::new(&runtime, &context);
        let button = Button::filled();

        let view = button.view::<ButtonEvent>(&cx);

        assert!(!view.ripple_enabled);
    }
}
