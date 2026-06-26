//! Visual Iced demo for `Panel`.

use std::time::Duration;

use iced::widget::{column, container, row, text};
use iced::{Element, Fill, Subscription, Task, Theme, application, time};
use iced_component::anim::{MotionError, MotionRuntime};
use iced_component::component::{ComponentContext, ComponentUpdateCx, ComponentViewCx};
use iced_component::panel::Panel;
use iced_component::surface::SurfaceEvent;

fn main() -> iced::Result {
    application(Demo::new, Demo::update, Demo::view)
        .title("aura-iced-component panel demo")
        .subscription(subscription)
        .theme(theme)
        .window_size([520.0, 300.0])
        .run()
}

struct Demo {
    runtime: MotionRuntime,
    context: ComponentContext,
    inspector: Panel,
    status: Panel,
    motion_error: Option<String>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Tick,
    Inspector(SurfaceEvent),
    Status(SurfaceEvent),
}

impl Demo {
    fn new() -> Self {
        let mut runtime = MotionRuntime::new();
        let context = ComponentContext::default();
        let mut inspector = Panel::titled("Inspector");
        let mut status = Panel::titled("Status");

        iced_component::register_components!(runtime, [inspector, status]);

        Self {
            runtime,
            context,
            inspector,
            status,
            motion_error: None,
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        let mut cx = ComponentUpdateCx::new(&mut self.runtime, &mut self.context);
        match message {
            Message::Tick => {
                self.runtime
                    .tick(iced_component::motion::Duration::from_millis(16.0));
            }
            Message::Inspector(event) => {
                let result = self.inspector.update_event(event, &mut cx);
                record_motion_result(self, result);
            }
            Message::Status(event) => {
                let result = self.status.update_event(event, &mut cx);
                record_motion_result(self, result);
            }
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let cx = ComponentViewCx::new(&self.runtime, &self.context);
        let inspector_snapshot = self.inspector.snapshot(&cx).unwrap();
        let status_snapshot = self.status.snapshot(&cx).unwrap();

        let inspector = self
            .inspector
            .view(&cx)
            .body(
                column![
                    text("Surface-backed panel body").size(14),
                    text(format!(
                        "elevation {:.2}",
                        inspector_snapshot.motion.elevation
                    ))
                    .size(14),
                ]
                .spacing(8),
            )
            .footer(text("Hover to raise the surface").size(13))
            .connect(Message::Inspector);

        let status = self
            .status
            .view(&cx)
            .header(row![text("Status").size(17), text("Live").size(13)].spacing(8))
            .body(
                column![
                    text("Reusable chrome for future panels").size(14),
                    text(format!("border {:.2}", status_snapshot.motion.border_alpha)).size(14),
                ]
                .spacing(8),
            )
            .connect(Message::Status);

        let content = column![
            text("Panel").size(28),
            row![inspector, status].spacing(16),
            text(self.motion_error.as_deref().unwrap_or("motion runtime: ok")).size(14),
        ]
        .spacing(18);

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
