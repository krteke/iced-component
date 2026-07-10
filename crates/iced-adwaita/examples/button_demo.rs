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
    button::{
        Button, ButtonEvent, ButtonSignal, ButtonSync,
        icon::{IconButton, IconTextButton},
    },
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
    Icon(ButtonEvent<IconAction>),
    IconText(ButtonEvent<IconTextAction>),
    Passive(PassiveButton, ButtonEvent<()>),
    Theme(ButtonEvent<ThemeAction>),
    ReduceMotion(ButtonEvent<ReduceMotionAction>),
}

#[derive(Debug, Clone, Copy)]
enum SaveAction {
    Save,
}

#[derive(Debug, Clone, Copy)]
enum IconAction {
    Press,
}

#[derive(Debug, Clone, Copy)]
enum IconTextAction {
    Open,
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
    icon: IconButton,
    icon_text: IconTextButton,
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
            icon: IconButton::svg_static(SEARCH_ICON).flat(),
            icon_text: IconTextButton::svg_static(OPEN_ICON, "Open"),
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
                    demo.icon,
                    demo.icon_text,
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
            Message::Icon(event) => self.icon_event(event),
            Message::IconText(event) => self.icon_text_event(event),
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

    fn icon_event(&mut self, event: ButtonEvent<IconAction>) {
        let mut cx = UpdateCx::new(&mut self.runtime, &mut self.context);

        let _ = self.icon.update_event(event, &mut cx);
    }

    fn icon_text_event(&mut self, event: ButtonEvent<IconTextAction>) {
        let mut cx = UpdateCx::new(&mut self.runtime, &mut self.context);

        let _ = self.icon_text.update_event(event, &mut cx);
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
                    self.icon,
                    self.icon_text,
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
                self.icon
                    .view(&view)
                    .connect(IconAction::Press, Message::Icon),
                self.icon_text
                    .view(&view)
                    .connect(IconTextAction::Open, Message::IconText),
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

const SEARCH_ICON: &[u8] = br#"
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16">
  <path fill="currentColor" d="M7 2a5 5 0 0 1 3.9 8.13l2.49 2.48-.78.78-2.48-2.49A5 5 0 1 1 7 2Zm0 1.1a3.9 3.9 0 1 0 0 7.8 3.9 3.9 0 0 0 0-7.8Z"/>
</svg>
"#;

const OPEN_ICON: &[u8] = br#"
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16">
  <path fill="currentColor" d="M2 2.5A1.5 1.5 0 0 1 3.5 1h3.2l1.45 1.75h4.35A1.5 1.5 0 0 1 14 4.25v7.25A1.5 1.5 0 0 1 12.5 13h-9A1.5 1.5 0 0 1 2 11.5Zm1.1 2.35v6.65c0 .22.18.4.4.4h9a.4.4 0 0 0 .4-.4V4.25a.4.4 0 0 0-.4-.4H7.64L6.2 2.1H3.5a.4.4 0 0 0-.4.4Z"/>
</svg>
"#;
