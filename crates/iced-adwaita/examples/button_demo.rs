//! Visual demo for the Adwaita button component.

use aura_anim::iced::{TickPolicy, subscription_with_policy};
use iced::{
    Background, Element, Length, Size, Subscription, Task, Theme,
    time::Instant,
    widget::{column, container, row, text},
    window,
};
use iced_adwaita::{
    Context,
    button::{Button, ButtonEvent, ButtonSignal, ButtonSync},
    context::{ThemeMode, UpdateCx, ViewCx},
};
use iced_component_core::anim::MotionRuntime;
use spectrum_theme::{Color, iced::IcedColorAdapter};

fn main() -> iced::Result {
    init_tracing();

    iced::application(Demo::default, Demo::update, Demo::view)
        .title("Adwaita button demo")
        .subscription(Demo::subscription)
        .theme(theme)
        .window(window::Settings {
            size: Size::new(640.0, 360.0),
            min_size: Some(Size::new(440.0, 280.0)),
            position: window::Position::Centered,
            ..window::Settings::default()
        })
        .run()
}

#[cfg(feature = "tracing")]
fn init_tracing() {
    let filter = tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        tracing_subscriber::EnvFilter::new(
            "warn,iced_adwaita::button=debug,iced_adwaita::context=debug",
        )
    });

    let _ = tracing_subscriber::fmt()
        .compact()
        .with_env_filter(filter)
        .try_init();
}

#[cfg(not(feature = "tracing"))]
fn init_tracing() {}

fn theme(state: &Demo) -> Theme {
    match state.context.theme().mode() {
        ThemeMode::Light => Theme::Light,
        ThemeMode::Dark => Theme::Dark,
    }
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Frame(Instant),
    Save(ButtonEvent<SaveAction>),
    Passive(PassiveButton, ButtonEvent<()>),
    Theme(ButtonEvent<ThemeAction>),
    ReduceMotion(ButtonEvent<ReduceMotionAction>),
}

#[derive(Debug, Clone, Copy)]
enum SaveAction {
    Save,
}

#[derive(Debug, Clone, Copy)]
enum ThemeAction {
    Toggle,
}

#[derive(Debug, Clone, Copy)]
enum ReduceMotionAction {
    Toggle,
}

#[derive(Debug, Clone, Copy)]
enum PassiveButton {
    Suggested,
    Destructive,
    Flat,
    Pill,
    Custom,
}

struct Demo {
    context: Context,
    runtime: MotionRuntime,
    clicks: usize,
    save: Button,
    suggested: Button,
    destructive: Button,
    flat: Button,
    pill: Button,
    custom: Button,
    disabled: Button,
    theme_button: Button,
    reduce_button: Button,
}

impl Default for Demo {
    fn default() -> Self {
        let mut demo = Self {
            context: Context::light(),
            runtime: MotionRuntime::new(),
            clicks: 0,
            save: Button::new("Save"),
            suggested: Button::suggested("Suggested"),
            destructive: Button::destructive("Destructive"),
            flat: Button::new("Flat").flat(),
            pill: Button::suggested("Pill").pill(),
            custom: Button::new("Custom")
                .pill()
                .background(Color::new(0x2e, 0x34, 0x40))
                .foreground(Color::new(0xff, 0xff, 0xff)),
            disabled: Button::new("Disabled").disabled(true),
            theme_button: Button::new("Theme").flat(),
            reduce_button: Button::new("Reduce motion").flat(),
        };

        {
            let mut cx = UpdateCx::new(&mut demo.runtime, &mut demo.context);
            iced_component_core::register_components!(
                cx,
                [
                    demo.save,
                    demo.suggested,
                    demo.destructive,
                    demo.flat,
                    demo.pill,
                    demo.custom,
                    demo.disabled,
                    demo.theme_button,
                    demo.reduce_button,
                ]
            );
        }

        demo
    }
}

impl Demo {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Frame(now) => {
                aura_anim::iced::frame(&mut self.runtime, now);
            }
            Message::Save(event) => self.save_event(event),
            Message::Passive(button, event) => self.passive_event(button, event),
            Message::Theme(event) => self.theme_event(event),
            Message::ReduceMotion(event) => self.reduce_motion_event(event),
        }

        Task::none()
    }

    fn save_event(&mut self, event: ButtonEvent<SaveAction>) {
        let mut cx = UpdateCx::new(&mut self.runtime, &mut self.context);

        if matches!(
            self.save.update_event(event, &mut cx).unwrap_or(None),
            Some(SaveAction::Save)
        ) {
            self.clicks += 1;
            self.save.set_content(format!("Save {}", self.clicks));
        }
    }

    fn passive_event(&mut self, button: PassiveButton, event: ButtonEvent<()>) {
        let mut cx = UpdateCx::new(&mut self.runtime, &mut self.context);

        match button {
            PassiveButton::Suggested => {
                let _ = self.suggested.update_event(event, &mut cx);
            }
            PassiveButton::Destructive => {
                let _ = self.destructive.update_event(event, &mut cx);
            }
            PassiveButton::Flat => {
                let _ = self.flat.update_event(event, &mut cx);
            }
            PassiveButton::Pill => {
                let _ = self.pill.update_event(event, &mut cx);
            }
            PassiveButton::Custom => {
                let _ = self.custom.update_event(event, &mut cx);
            }
        }
    }

    fn theme_event(&mut self, event: ButtonEvent<ThemeAction>) {
        let mut cx = UpdateCx::new(&mut self.runtime, &mut self.context);

        if matches!(
            self.theme_button
                .update_event(event, &mut cx)
                .unwrap_or(None),
            Some(ThemeAction::Toggle)
        ) {
            let change = cx.toggle_theme();
            let signal = ButtonSignal::Sync(ButtonSync::StyleChanged(change));
            let _ = iced_component_core::update_components!(
                cx,
                signal,
                [
                    self.save,
                    self.suggested,
                    self.destructive,
                    self.flat,
                    self.pill,
                    self.custom,
                    self.disabled,
                    self.theme_button,
                    self.reduce_button,
                ]
            );
        }
    }

    fn reduce_motion_event(&mut self, event: ButtonEvent<ReduceMotionAction>) {
        let mut cx = UpdateCx::new(&mut self.runtime, &mut self.context);

        if let Ok(Some(ReduceMotionAction::Toggle)) =
            self.reduce_button.update_event(event, &mut cx)
        {
            cx.toggle_reduce_motion();
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let view = ViewCx::new(&self.runtime, &self.context);
        let pack = view.theme().pack();
        let fg = pack.app.window.fg.color();
        let bg = pack.app.window.bg.color();

        let controls = row![
            self.theme_button
                .view(&view)
                .connect(ThemeAction::Toggle, Message::Theme),
            self.reduce_button
                .view(&view)
                .connect(ReduceMotionAction::Toggle, Message::ReduceMotion),
        ]
        .spacing(12);

        let buttons = column![
            row![
                self.save
                    .view(&view)
                    .connect(SaveAction::Save, Message::Save),
                self.suggested
                    .view(&view)
                    .connect((), |event| Message::Passive(
                        PassiveButton::Suggested,
                        event
                    )),
                self.destructive
                    .view(&view)
                    .connect((), |event| Message::Passive(
                        PassiveButton::Destructive,
                        event
                    )),
            ]
            .spacing(12),
            row![
                self.flat
                    .view(&view)
                    .connect((), |event| Message::Passive(PassiveButton::Flat, event)),
                self.pill
                    .view(&view)
                    .connect((), |event| Message::Passive(PassiveButton::Pill, event)),
                self.custom
                    .view(&view)
                    .connect((), |event| Message::Passive(PassiveButton::Custom, event)),
                self.disabled.view(&view),
            ]
            .spacing(12),
        ]
        .spacing(12);

        let content = column![
            row![text("Adwaita Button").size(22).color(fg), controls,]
                .spacing(24)
                .align_y(iced::Alignment::Center),
            buttons,
        ]
        .spacing(20);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(28)
            .style(move |_| container::Style::default().background(Background::Color(bg)))
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
