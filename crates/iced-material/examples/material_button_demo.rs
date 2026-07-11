//! Visual demo for Material 3 buttons.

use aura_anim::iced::{TickPolicy, subscription_with_policy};
use iced::{
    Background, Element, Font, Size, Subscription, Task, Theme,
    font::Weight,
    time::Instant,
    widget::{column, container, row, text},
    window,
};
use iced_component_core::anim::MotionRuntime;
use iced_material::{
    button::{Button, ButtonEvent, ButtonSignal, ButtonSync, ButtonVariant},
    context::{Context, ThemeMode, UpdateCx, ViewCx},
};
use spectrum_theme::iced::IcedColorAdapter;

fn main() -> iced::Result {
    iced::application(Demo::default, Demo::update, Demo::view)
        .title("Material button demo")
        .theme(theme)
        .subscription(Demo::subscription)
        .window(window::Settings {
            size: Size::new(680.0, 400.0),
            min_size: Some(Size::new(500.0, 320.0)),
            position: window::Position::Centered,
            ..window::Settings::default()
        })
        .run()
}

fn theme(demo: &Demo) -> Theme {
    match demo.context.theme().mode() {
        ThemeMode::Light => Theme::Light,
        ThemeMode::Dark => Theme::Dark,
    }
}

#[derive(Clone, Copy, Debug)]
enum Message {
    Frame(Instant),
    Variant(ButtonVariant, ButtonEvent),
    ToggleTheme(ButtonEvent),
}

struct Demo {
    context: Context,
    runtime: MotionRuntime,
    elevated: Button,
    filled: Button,
    filled_tonal: Button,
    outlined: Button,
    text: Button,
    disabled: Button,
    theme_button: Button,
}

impl Default for Demo {
    fn default() -> Self {
        let mut demo = Self {
            context: Context::light(),
            runtime: MotionRuntime::new(),
            elevated: Button::elevated(),
            filled: Button::filled(),
            filled_tonal: Button::filled_tonal(),
            outlined: Button::outlined(),
            text: Button::text(),
            disabled: Button::filled().disabled(true),
            theme_button: Button::outlined(),
        };

        let mut cx = UpdateCx::new(&mut demo.runtime, &mut demo.context);
        iced_component_core::register_components!(
            cx,
            [
                demo.elevated,
                demo.filled,
                demo.filled_tonal,
                demo.outlined,
                demo.text,
                demo.disabled,
                demo.theme_button,
            ]
        );

        demo
    }
}

impl Demo {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Frame(now) => aura_anim::iced::frame(&mut self.runtime, now),
            Message::Variant(variant, event) => self.update_variant(variant, event),
            Message::ToggleTheme(event) => self.toggle_theme(event),
        }

        Task::none()
    }

    fn update_variant(&mut self, variant: ButtonVariant, event: ButtonEvent) {
        let mut cx = UpdateCx::new(&mut self.runtime, &mut self.context);

        let result = match variant {
            ButtonVariant::Elevated => self.elevated.update_event(event, &mut cx),
            ButtonVariant::Filled => self.filled.update_event(event, &mut cx),
            ButtonVariant::FilledTonal => self.filled_tonal.update_event(event, &mut cx),
            ButtonVariant::Outlined => self.outlined.update_event(event, &mut cx),
            ButtonVariant::Text => self.text.update_event(event, &mut cx),
        };
        let _ = result;
    }

    fn toggle_theme(&mut self, event: ButtonEvent) {
        let mut cx = UpdateCx::new(&mut self.runtime, &mut self.context);
        let Ok(outcome) = self.theme_button.update_event(event, &mut cx) else {
            return;
        };
        if !outcome.is_activated() {
            return;
        }
        let signal = ButtonSignal::Sync(ButtonSync::StyleChanged(cx.toggle_theme()));
        let _ = iced_component_core::update_components!(
            cx,
            signal,
            [
                self.elevated,
                self.filled,
                self.filled_tonal,
                self.outlined,
                self.text,
                self.disabled,
                self.theme_button,
            ]
        );
    }

    fn view(&self) -> Element<'_, Message> {
        let cx = ViewCx::new(&self.runtime, &self.context);
        let pack = cx.theme().pack();
        let background = pack.palette.background.color.color();
        let label_size = pack.button.label_size.value();
        let label_line_height = pack.button.label_line_height.value();
        let label_font = Font {
            weight: Weight::Medium,
            ..Font::DEFAULT
        };
        let button_label = |value| {
            text(value)
                .size(label_size)
                .line_height(iced::widget::text::LineHeight::Absolute(
                    label_line_height.into(),
                ))
                .font(label_font)
        };

        let primary_row = row![
            self.elevated
                .view(&cx)
                .content(button_label("Elevated"))
                .on_event(|event| Message::Variant(ButtonVariant::Elevated, event)),
            self.filled
                .view(&cx)
                .content(button_label("Filled"))
                .on_event(|event| Message::Variant(ButtonVariant::Filled, event)),
            self.filled_tonal
                .view(&cx)
                .content(button_label("Filled tonal"))
                .on_event(|event| Message::Variant(ButtonVariant::FilledTonal, event)),
        ]
        .spacing(12);
        let secondary_row = row![
            self.outlined
                .view(&cx)
                .content(button_label("Outlined"))
                .on_event(|event| Message::Variant(ButtonVariant::Outlined, event)),
            self.text
                .view(&cx)
                .content(button_label("Text"))
                .on_event(|event| Message::Variant(ButtonVariant::Text, event)),
            self.disabled.view(&cx).content(button_label("Disabled")),
        ]
        .spacing(12);
        let scheme = match cx.theme_mode() {
            ThemeMode::Light => "Use dark scheme",
            ThemeMode::Dark => "Use light scheme",
        };
        let controls = self
            .theme_button
            .view(&cx)
            .content(button_label(scheme))
            .on_event(Message::ToggleTheme);
        let content = column![
            text("Material 3 Button")
                .size(24)
                .color(pack.palette.background.on_color.color()),
            text("Hover and press each variant to inspect its state layer and elevation.")
                .size(14)
                .color(pack.palette.surface.on_color_variant.color()),
            primary_row,
            secondary_row,
            controls,
        ]
        .spacing(20);

        container(content)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .padding(32)
            .style(move |_| container::Style::default().background(Background::Color(background)))
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        subscription_with_policy(
            &self.runtime,
            TickPolicy::interval(core::time::Duration::from_millis(16)),
        )
        .map(Message::Frame)
    }
}
