//! Iced widget that owns Material button input and patterned-ripple drawing.

use iced::{Background, Border, Color, Element, Padding, Point, Shadow};
use iced_widget::{
    core::{
        Clipboard, Event, Layout, Length, Rectangle, Shell, Size, Vector, Widget, layout, mouse,
        overlay, renderer,
        time::Instant,
        touch,
        widget::{Operation, Tree, tree},
        window,
    },
    graphics::geometry,
    renderer::wgpu::primitive,
};
use spectrum_theme::iced::IcedColorAdapter;

use super::{ButtonEvent, ButtonSignal, ButtonSnapshot, ripple};

/// Fixed layout values resolved by a [`super::ButtonView`].
#[derive(Clone, Copy, Debug)]
pub(crate) struct ButtonLayout {
    pub(crate) height: f32,
    pub(crate) leading_padding: f32,
    pub(crate) trailing_padding: f32,
}

/// Application event mapping for rendered button input.
pub(crate) struct ButtonEvents<'a, Message> {
    mapper: Box<dyn Fn(ButtonEvent) -> Message + 'a>,
}

impl<'a, Message> ButtonEvents<'a, Message> {
    pub(crate) fn new(mapper: impl Fn(ButtonEvent) -> Message + 'a) -> Self {
        Self {
            mapper: Box::new(mapper),
        }
    }

    fn signal(&self, signal: ButtonSignal) -> Message {
        (self.mapper)(ButtonEvent::Signal(signal))
    }

    fn release(&self) -> Message {
        (self.mapper)(ButtonEvent::Pressed)
    }
}

/// Private widget implementation produced by [`super::ButtonView`].
pub(crate) struct MaterialButtonWidget<'a, Message, Renderer = iced::Renderer>
where
    Renderer: geometry::Renderer,
{
    snapshot: ButtonSnapshot,
    layout: ButtonLayout,
    content: Element<'a, Message, iced::Theme, Renderer>,
    events: Option<ButtonEvents<'a, Message>>,
}

impl<'a, Message, Renderer> MaterialButtonWidget<'a, Message, Renderer>
where
    Renderer: geometry::Renderer,
{
    pub(crate) fn new(
        snapshot: ButtonSnapshot,
        layout: ButtonLayout,
        content: Element<'a, Message, iced::Theme, Renderer>,
        events: Option<ButtonEvents<'a, Message>>,
    ) -> Self {
        Self {
            snapshot,
            layout,
            content,
            events,
        }
    }
}

#[derive(Default)]
struct ButtonWidgetState {
    hovered: bool,
    pressed: bool,
    ripples: ripple::PressRippleState,
    now: Option<Instant>,
}

impl ButtonWidgetState {
    fn press(&mut self, origin: Point, now: Instant) {
        self.pressed = true;
        self.ripples.press(origin, now);
        self.now = Some(now);
    }

    fn release(&mut self, now: Instant) {
        self.pressed = false;
        self.ripples.release(now);
        self.now = Some(now);
    }

    fn advance(&mut self, now: Instant) -> bool {
        self.now = Some(now);
        self.ripples.prune(now);
        self.ripples.has_visible_ripples(now)
    }
}

impl<Message, Renderer> Widget<Message, iced::Theme, Renderer>
    for MaterialButtonWidget<'_, Message, Renderer>
where
    Renderer: geometry::Renderer + primitive::Renderer,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<ButtonWidgetState>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(ButtonWidgetState::default())
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.content));
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Shrink,
            height: Length::Fixed(self.layout.height),
        }
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::padded(
            limits,
            Length::Shrink,
            Length::Fixed(self.layout.height),
            Padding {
                top: 0.0,
                right: self.layout.trailing_padding,
                bottom: 0.0,
                left: self.layout.leading_padding,
            },
            |limits| {
                self.content
                    .as_widget_mut()
                    .layout(&mut tree.children[0], renderer, limits)
            },
        )
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        operation.container(None, layout.bounds());
        operation.traverse(&mut |operation| {
            self.content.as_widget_mut().operate(
                &mut tree.children[0],
                layout.child(0),
                renderer,
                operation,
            );
        });
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        self.content.as_widget_mut().update(
            &mut tree.children[0],
            event,
            layout.child(0),
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );
        if shell.is_event_captured() || self.snapshot.disabled {
            return;
        }

        let Some(events) = self.events.as_mut() else {
            return;
        };
        let bounds = layout.bounds();
        let state = tree.state.downcast_mut::<ButtonWidgetState>();
        let redraw_time = match event {
            Event::Window(window::Event::RedrawRequested(now)) => Some(*now),
            _ => None,
        };
        let now_or_current = || redraw_time.unwrap_or_else(Instant::now);
        sync_hover(events, state, event, cursor, bounds, shell);

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                let Some(position) = cursor.position_in(bounds) else {
                    return;
                };
                state.press(position, now_or_current());
                shell.publish(events.signal(ButtonSignal::PressDownAt(
                    iced_component_core::component::button::PointerPosition::new(
                        position.x, position.y,
                    ),
                )));
                shell.capture_event();
                shell.request_redraw();
            }
            Event::Touch(touch::Event::FingerPressed { position, .. })
                if bounds.contains(*position) =>
            {
                state.press(
                    Point::new(position.x - bounds.x, position.y - bounds.y),
                    now_or_current(),
                );
                shell.publish(events.signal(ButtonSignal::PressDownAt(
                    iced_component_core::component::button::PointerPosition::new(
                        position.x - bounds.x,
                        position.y - bounds.y,
                    ),
                )));
                shell.capture_event();
                shell.request_redraw();
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) if state.pressed => {
                state.release(now_or_current());
                let message = if cursor.is_over(bounds) {
                    events.release()
                } else {
                    events.signal(ButtonSignal::PressUp)
                };
                shell.publish(message);
                shell.capture_event();
                shell.request_redraw();
            }
            Event::Touch(touch::Event::FingerLifted { position, .. }) if state.pressed => {
                state.release(now_or_current());
                let message = if bounds.contains(*position) {
                    events.release()
                } else {
                    events.signal(ButtonSignal::PressUp)
                };
                shell.publish(message);
                shell.capture_event();
                shell.request_redraw();
            }
            Event::Touch(touch::Event::FingerLost { .. }) if state.pressed => {
                state.release(now_or_current());
                shell.publish(events.signal(ButtonSignal::PressUp));
                shell.capture_event();
                shell.request_redraw();
            }
            _ => {}
        }

        if let Some(now) = redraw_time
            && state.advance(now)
        {
            shell.request_redraw();
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &iced::Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        if bounds.width < 1.0 || bounds.height < 1.0 {
            return;
        }

        let visual = self.snapshot.visual;
        renderer.fill_quad(
            renderer::Quad {
                bounds,
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
            },
            Background::Color(color_with_opacity(
                visual.background,
                visual.background_opacity,
            )),
        );
        self.content.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            &renderer::Style {
                text_color: color_with_opacity(visual.foreground, visual.foreground_opacity),
            },
            layout.child(0),
            cursor,
            viewport,
        );
        if visual.state_layer_opacity > 0.0 {
            renderer.fill_quad(
                renderer::Quad {
                    bounds,
                    border: Border {
                        radius: visual.radius.into(),
                        ..Border::default()
                    },
                    ..renderer::Quad::default()
                },
                Background::Color(color_with_opacity(
                    visual.state_layer,
                    visual.state_layer_opacity,
                )),
            );
        }
        let state = tree.state.downcast_ref::<ButtonWidgetState>();
        ripple::draw(
            renderer,
            bounds,
            &state.ripples,
            color_with_opacity(self.snapshot.ripple_color, self.snapshot.ripple_opacity),
            visual.radius,
            state.now.unwrap_or_else(Instant::now),
        );
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        if !self.snapshot.disabled && self.events.is_some() && cursor.is_over(layout.bounds()) {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, iced::Theme, Renderer>> {
        self.content.as_widget_mut().overlay(
            &mut tree.children[0],
            layout.child(0),
            renderer,
            viewport,
            translation,
        )
    }
}

impl<'a, Message, Renderer> From<MaterialButtonWidget<'a, Message, Renderer>>
    for Element<'a, Message, iced::Theme, Renderer>
where
    Message: 'a,
    Renderer: geometry::Renderer + primitive::Renderer + 'a,
{
    fn from(widget: MaterialButtonWidget<'a, Message, Renderer>) -> Self {
        Element::new(widget)
    }
}

fn sync_hover<Message>(
    events: &ButtonEvents<'_, Message>,
    state: &mut ButtonWidgetState,
    event: &Event,
    cursor: mouse::Cursor,
    bounds: Rectangle,
    shell: &mut Shell<'_, Message>,
) {
    if matches!(event, Event::Touch(_)) {
        return;
    }

    let hovered = cursor.is_over(bounds);
    if state.hovered != hovered {
        state.hovered = hovered;
        shell.publish(events.signal(if hovered {
            ButtonSignal::HoverEnter
        } else {
            ButtonSignal::HoverExit
        }));
    }
}

fn color_with_opacity(color: spectrum_theme::Color, opacity: f32) -> Color {
    let mut color = color.color();
    color.a *= opacity;
    color
}

#[cfg(test)]
mod tests {
    use iced::{Element, widget::text};

    use super::{ButtonEvents, ButtonLayout, MaterialButtonWidget};
    use crate::button::{ButtonEvent, ButtonSnapshot, ButtonStyleState, ButtonVisual};
    use spectrum_theme::Color;

    #[test]
    fn widget_builds_with_an_event_mapper() {
        struct Message;

        let snapshot = ButtonSnapshot {
            style_state: ButtonStyleState::Idle,
            visual: ButtonVisual {
                background: Color::new(0, 0, 0),
                background_opacity: 1.0,
                foreground: Color::new(255, 255, 255),
                foreground_opacity: 1.0,
                border: Color::new(0, 0, 0),
                border_opacity: 0.0,
                border_width: 0.0,
                radius: 20.0,
                shadow: Color::new(0, 0, 0),
                shadow_opacity: 0.0,
                shadow_y: 0.0,
                shadow_blur: 0.0,
                state_layer: Color::new(255, 255, 255),
                state_layer_opacity: 0.0,
            },
            disabled: false,
            focused: false,
            ripple_color: Color::new(255, 255, 255),
            ripple_opacity: 0.1,
        };
        let events = ButtonEvents::new(|_event: ButtonEvent| Message);

        let _: Element<'_, Message> = MaterialButtonWidget::new(
            snapshot,
            ButtonLayout {
                height: 40.0,
                leading_padding: 24.0,
                trailing_padding: 24.0,
            },
            text("Save").into(),
            Some(events),
        )
        .into();
    }
}
