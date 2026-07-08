//! Visual demo for the Adwaita spinner component.

use iced::{
    Background, Element, Length, Size, Subscription, Task, Theme,
    time::Instant,
    widget::{button, column, container, row, text},
    window,
};
use iced_adwaita::{
    Context,
    context::{ThemeMode, UpdateCx, ViewCx},
    spinner::Spinner,
};
use iced_component_core::anim::MotionRuntime;
use spectrum_theme::iced::IcedColorAdapter;

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

fn theme(state: &Demo) -> Theme {
    match state.mode {
        ThemeMode::Light => Theme::Light,
        ThemeMode::Dark => Theme::Dark,
    }
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Frame(Instant),
    ToggleTheme,
}

struct Demo {
    context: Context,
    runtime: MotionRuntime,
    mode: ThemeMode,
    spinners: [Spinner; 4],
}

impl Default for Demo {
    fn default() -> Self {
        Self {
            context: Context::light(),
            runtime: MotionRuntime::new(),
            mode: ThemeMode::Light,
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
        let mut cx = UpdateCx::new(&mut self.runtime, &mut self.context);

        match message {
            Message::Frame(now) => {
                for spinner in &mut self.spinners {
                    spinner.advance(now);
                }
            }
            Message::ToggleTheme => {
                self.mode = match self.mode {
                    ThemeMode::Light => ThemeMode::Dark,
                    ThemeMode::Dark => ThemeMode::Light,
                };

                cx.toggle_theme();
            }
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let view = ViewCx::new(&self.runtime, &self.context);
        let theme = view.theme();
        let fg = theme.pack().app.window.fg.color();
        let bg = theme.pack().app.window.bg.color();
        let toggle_label = match self.mode {
            ThemeMode::Light => "Dark",
            ThemeMode::Dark => "Light",
        };

        let content = column![
            row![
                text("Adwaita Spinner").size(20).color(fg),
                button(text(toggle_label)).on_press(Message::ToggleTheme),
            ]
            .spacing(16)
            .align_y(iced::Alignment::Center),
            row(self.spinners.map(|spinner| spinner.view(&view))).spacing(22),
        ]
        .spacing(14)
        .align_x(iced::Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center(Length::Fill)
            .style(move |_| container::Style::default().background(Background::Color(bg)))
            .into()
    }
}

fn subscription(_: &Demo) -> Subscription<Message> {
    window::frames().map(Message::Frame)
}
