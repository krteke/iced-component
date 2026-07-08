//! Visual demo for the Adwaita spinner component.

use iced::{
    Background, Color, Element, Length, Size, Subscription, Task, Theme,
    time::Instant,
    widget::{column, container, row, text},
    window,
};
use iced_adwaita::{Context, context::ViewCx, spinner::Spinner};
use iced_component_core::anim::MotionRuntime;

fn main() -> iced::Result {
    iced::application(Demo::default, Demo::update, Demo::view)
        .title("Adwaita spinner demo")
        .subscription(subscription)
        .theme(theme)
        .window(window::Settings {
            size: Size::new(420.0, 260.0),
            min_size: Some(Size::new(320.0, 220.0)),
            position: window::Position::Centered,
            ..window::Settings::default()
        })
        .run()
}

fn theme(_: &Demo) -> Theme {
    Theme::Light
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Frame(Instant),
}

struct Demo {
    context: Context,
    runtime: MotionRuntime,
    spinners: [Spinner; 4],
}

impl Default for Demo {
    fn default() -> Self {
        Self {
            context: Context::light(),
            runtime: MotionRuntime::new(),
            spinners: [
                Spinner::new().size(16.0),
                Spinner::new().size(24.0),
                Spinner::new().size(48.0),
                Spinner::new().size(64.0),
            ],
        }
    }
}

impl Demo {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Frame(now) => {
                for spinner in &mut self.spinners {
                    spinner.advance(now);
                }
            }
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let view = ViewCx::new(&self.runtime, &self.context);

        let content = column![
            text("Adwaita Spinner").size(20),
            row(self.spinners.map(|spinner| spinner.view(&view))).spacing(22),
        ]
        .spacing(14)
        .align_x(iced::Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center(Length::Fill)
            .style(|_| {
                container::Style::default()
                    .background(Background::Color(Color::from_rgb8(0xfa, 0xfa, 0xfa)))
            })
            .into()
    }
}

fn subscription(_: &Demo) -> Subscription<Message> {
    window::frames().map(Message::Frame)
}
