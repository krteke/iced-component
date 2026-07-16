//! Visual demo for the Adwaita + Material loading indicator adapter.

use iced::{
    Background, Element, Length, Size, Subscription, Task, Theme,
    time::Instant,
    widget::{button, column, container, row, text},
    window,
};
use iced_component::{
    context::{ColorScheme, Context, ThemeFamily, UpdateCx, ViewCx},
    core::anim::MotionRuntime,
    loading_indicator::LoadingIndicator,
};
use spectrum_theme::iced::IcedColorAdapter;

fn main() -> iced::Result {
    iced::application(Demo::default, Demo::update, Demo::view)
        .title("Loading indicator adapter")
        .theme(Demo::iced_theme)
        .subscription(subscription)
        .window(window::Settings {
            size: Size::new(520.0, 300.0),
            min_size: Some(Size::new(420.0, 260.0)),
            position: window::Position::Centered,
            ..window::Settings::default()
        })
        .run()
}

#[derive(Clone, Copy, Debug)]
enum Message {
    Frame(Instant),
    ToggleFamily,
    ToggleScheme,
}

struct Demo {
    context: Context,
    runtime: MotionRuntime,
    indicators: [LoadingIndicator; 3],
}

impl Default for Demo {
    fn default() -> Self {
        let mut contained = LoadingIndicator::new().size(48.0);
        contained.material_mut().set_contained(true);
        let mut context = Context::default();
        let mut runtime = MotionRuntime::new();
        let mut indicators = [
            LoadingIndicator::new().size(32.0),
            contained,
            LoadingIndicator::new().size(72.0),
        ];

        {
            let mut cx = UpdateCx::new(&mut runtime, &mut context);
            for indicator in &mut indicators {
                indicator.register(&mut cx);
            }
        }

        Self {
            context,
            runtime,
            indicators,
        }
    }
}

impl Demo {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Frame(now) => {
                aura_anim::iced::frame(&mut self.runtime, now);
                for indicator in &mut self.indicators {
                    indicator.advance(now);
                }
            }
            Message::ToggleFamily => {
                UpdateCx::new(&mut self.runtime, &mut self.context).toggle_family();
            }
            Message::ToggleScheme => {
                let mut cx = UpdateCx::new(&mut self.runtime, &mut self.context);
                cx.toggle_color_scheme();
                for indicator in &mut self.indicators {
                    indicator
                        .sync(&mut cx)
                        .expect("loading indicator motion belongs to the demo runtime");
                }
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let cx = ViewCx::new(&self.runtime, &self.context);
        let (background, foreground) = self.colors();
        let family = match cx.family() {
            ThemeFamily::Adwaita => "Use Material",
            ThemeFamily::Material => "Use Adwaita",
        };
        let scheme = match cx.color_scheme() {
            ColorScheme::Dark => "Use light",
            ColorScheme::Light => "Use dark",
        };

        let content = column![
            text("Loading indicators").size(22).color(foreground),
            row(self.indicators.iter().map(|indicator| indicator.view(&cx)))
                .spacing(28)
                .align_y(iced::Alignment::Center),
            row![
                button(text(family)).on_press(Message::ToggleFamily),
                button(text(scheme)).on_press(Message::ToggleScheme),
            ]
            .spacing(12),
        ]
        .spacing(24)
        .align_x(iced::Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center(Length::Fill)
            .style(move |_| container::Style::default().background(Background::Color(background)))
            .into()
    }

    fn colors(&self) -> (iced::Color, iced::Color) {
        match self.context.family() {
            ThemeFamily::Adwaita => {
                let app = &self.context.adwaita().theme().pack().app.window;
                (app.bg.color(), app.fg.color())
            }
            ThemeFamily::Material => {
                let background = &self.context.material().theme().pack().palette.background;
                (background.color.color(), background.on_color.color())
            }
        }
    }

    fn iced_theme(&self) -> Theme {
        match self.context.color_scheme() {
            ColorScheme::Dark => Theme::Dark,
            ColorScheme::Light => Theme::Light,
        }
    }
}

fn subscription(_: &Demo) -> Subscription<Message> {
    window::frames().map(Message::Frame)
}
