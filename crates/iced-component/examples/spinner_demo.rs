//! Visual Iced demo for `Spinner`.

use std::time::Duration;

use iced::widget::{column, container, text};
use iced::{Element, Fill, Subscription, Task, Theme, application, time};
use iced_component::anim::{MotionError, MotionRuntime};
use iced_component::button::{Button, ButtonEvent};
use iced_component::component::{ComponentContext, ComponentUpdateCx, ComponentViewCx};
use iced_component::spinner::Spinner;

fn main() -> iced::Result {
    application(Demo::new, Demo::update, Demo::view)
        .title("aura-iced-component spinner demo")
        .subscription(subscription)
        .theme(theme)
        .window_size([420.0, 300.0])
        .run()
}

struct Demo {
    runtime: MotionRuntime,
    context: ComponentContext,
    spinner: Spinner,
    toggle_button: Button,
    size_button: Button,
    motion_error: Option<String>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Tick,
    ToggleButton(ButtonEvent<ToggleAction>),
    SizeButton(ButtonEvent<SizeAction>),
}

#[derive(Debug, Clone, Copy)]
enum ToggleAction {
    Toggle,
}

#[derive(Debug, Clone, Copy)]
enum SizeAction {
    Toggle,
}

impl Demo {
    fn new() -> Self {
        let mut runtime = MotionRuntime::new();
        let mut context = ComponentContext::default();
        let mut spinner = Spinner::new();
        let mut toggle_button = Button::suggested("Stop");
        let mut size_button = Button::standard("32 px");

        {
            let mut cx = ComponentUpdateCx::new(&mut runtime, &mut context);
            iced_component::register_components!(cx, [spinner, toggle_button, size_button]);
        }

        Self {
            runtime,
            context,
            spinner,
            toggle_button,
            size_button,
            motion_error: None,
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        let mut cx = ComponentUpdateCx::new(&mut self.runtime, &mut self.context);

        match message {
            Message::Tick => cx
                .runtime
                .tick(iced_component::motion::Duration::from_millis(16.0)),
            Message::ToggleButton(event) => match self.toggle_button.update_event(event, &mut cx) {
                Ok(Some(ToggleAction::Toggle)) => {
                    let next = !self.spinner.is_spinning();
                    let result = self.spinner.set_spinning(next, &mut cx);
                    self.toggle_button
                        .set_content(if next { "Stop" } else { "Start" });
                    record_motion_result(self, result);
                }
                Ok(None) => self.motion_error = None,
                Err(error) => self.motion_error = Some(error.to_string()),
            },
            Message::SizeButton(event) => match self.size_button.update_event(event, &mut cx) {
                Ok(Some(SizeAction::Toggle)) => {
                    let next = if self.spinner.size().unwrap_or(64.0) > 32.0 {
                        32.0
                    } else {
                        64.0
                    };
                    self.spinner.set_size(next);
                    self.size_button
                        .set_content(if next > 32.0 { "32 px" } else { "64 px" });
                    self.motion_error = None;
                }
                Ok(None) => self.motion_error = None,
                Err(error) => self.motion_error = Some(error.to_string()),
            },
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let cx = ComponentViewCx::new(&self.runtime, &self.context);
        let snapshot = self.spinner.snapshot(&cx).unwrap();
        let spinner: Element<'_, Message> = self.spinner.view(&cx).into();
        let state = if self.spinner.is_spinning() {
            "running"
        } else {
            "stopped"
        };

        let toggle = self
            .toggle_button
            .view(&cx)
            .connect(ToggleAction::Toggle, Message::ToggleButton);
        let size = self
            .size_button
            .view(&cx)
            .connect(SizeAction::Toggle, Message::SizeButton);

        let content = column![
            text("Spinner").size(28),
            text("Adwaita-style loading indicator").size(16),
            container(spinner).center_x(Fill).height(96),
            text(format!("Spinner: {state}")).size(16),
            text(format!(
                "size {:.0}px, stroke {:.1}px, rotation {:.1}deg",
                snapshot.size, snapshot.stroke_width, snapshot.motion.rotation
            ))
            .size(14),
            iced::widget::row![toggle, size].spacing(12),
            text(self.motion_error.as_deref().unwrap_or("motion runtime: ok")).size(14),
        ]
        .spacing(16);

        container(content)
            .padding(24)
            .center_x(Fill)
            .center_y(Fill)
            .into()
    }
}

fn record_motion_result(state: &mut Demo, result: Result<bool, MotionError>) {
    match result {
        Ok(_) => state.motion_error = None,
        Err(error) => state.motion_error = Some(error.to_string()),
    }
}

fn subscription(_state: &Demo) -> Subscription<Message> {
    time::every(Duration::from_millis(16)).map(|_| Message::Tick)
}

fn theme(_state: &Demo) -> Theme {
    Theme::Light
}
