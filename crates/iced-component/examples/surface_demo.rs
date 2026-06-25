//! Visual Iced demo for `Surface`.

use std::time::Duration;

use iced::widget::{column, container, row, text};
use iced::{Element, Fill, Subscription, Task, Theme, application, time};
use iced_component::component::ComponentContext;
use iced_component::surface::{Surface, SurfaceEvent, SurfaceInteraction};
use iced_component::theme::SurfaceRole;
use iced_component::{MotionError, MotionRuntime};

fn main() -> iced::Result {
    application(Demo::new, Demo::update, Demo::view)
        .title("aura-iced-component surface demo")
        .subscription(subscription)
        .theme(theme)
        .window_size([460.0, 280.0])
        .run()
}

struct Demo {
    runtime: MotionRuntime,
    context: ComponentContext,
    card: Surface,
    panel: Surface,
    motion_error: Option<String>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Tick,
    Card(SurfaceEvent),
    Panel(SurfaceEvent),
}

impl Demo {
    fn new() -> Self {
        let mut runtime = MotionRuntime::new();
        let context = ComponentContext::current();
        let mut card = Surface::raised().with_padding(18.0).with_width(190.0);
        let mut panel = Surface::regular().with_padding(18.0).with_width(190.0);

        card.register(&mut runtime, &context);
        panel.register(&mut runtime, &context);

        Self {
            runtime,
            context,
            card,
            panel,
            motion_error: None,
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Tick => {
                self.runtime
                    .tick(iced_component::motion::Duration::from_millis(16.0));
            }
            Message::Card(event) => {
                let result = self.card.update_event(event, &mut self.runtime);
                record_motion_result(self, result);
            }
            Message::Panel(event) => {
                let result = self.panel.update_event(event, &mut self.runtime);
                let role_result = match event {
                    SurfaceEvent::Interaction(SurfaceInteraction::HoverEnter) => {
                        self.panel.set_role(SurfaceRole::Raised, &mut self.runtime)
                    }
                    SurfaceEvent::Interaction(SurfaceInteraction::HoverExit) => {
                        self.panel.set_role(SurfaceRole::Regular, &mut self.runtime)
                    }
                };
                record_motion_result(self, result.and(role_result));
            }
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let card_snapshot = self.card.snapshot(&self.runtime, &self.context).unwrap();
        let panel_snapshot = self.panel.snapshot(&self.runtime, &self.context).unwrap();

        let card = self
            .card
            .view(
                &self.runtime,
                &self.context,
                column![
                    text("Raised surface").size(18),
                    text(format!("elevation {:.2}", card_snapshot.motion.elevation)).size(14),
                ]
                .spacing(8),
            )
            .connect(Message::Card);

        let panel = self
            .panel
            .view(
                &self.runtime,
                &self.context,
                column![
                    text("Role-switching surface").size(18),
                    text(format!("border {:.2}", panel_snapshot.motion.border_alpha)).size(14),
                ]
                .spacing(8),
            )
            .connect(Message::Panel);

        let content = column![
            text("Surface").size(28),
            row![card, panel].spacing(16),
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
