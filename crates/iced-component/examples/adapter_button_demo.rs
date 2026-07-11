//! Visual demo for the dual-theme button adapter.

use aura_anim::iced::{TickPolicy, subscription_with_policy};
use iced::{
    Element, Length, Size, Subscription, Task, Theme,
    time::Instant,
    widget::{column, container, row, text},
    window,
};
use iced_component::{
    button::{Button, ButtonEvent, ButtonOutcome},
    context::{ColorScheme, Context, ThemeFamily, UpdateCx, ViewCx},
    core::anim::MotionRuntime,
};

fn main() -> iced::Result {
    iced::application(Demo::default, Demo::update, Demo::view)
        .title("Iced component adapter")
        .theme(Demo::iced_theme)
        .subscription(Demo::subscription)
        .window(window::Settings {
            size: Size::new(680.0, 360.0),
            min_size: Some(Size::new(520.0, 300.0)),
            position: window::Position::Centered,
            ..window::Settings::default()
        })
        .run()
}

#[derive(Clone, Copy, Debug)]
enum Message {
    Frame(Instant),
    Button(Target, ButtonEvent),
}

#[derive(Clone, Copy, Debug)]
enum Target {
    Default,
    Primary,
    Secondary,
    Quiet,
    Family,
    Scheme,
    ReduceMotion,
}

struct Demo {
    context: Context,
    runtime: MotionRuntime,
    default: Button,
    primary: Button,
    secondary: Button,
    quiet: Button,
    disabled: Button,
    family: Button,
    scheme: Button,
    reduce_motion: Button,
    activations: usize,
}

impl Default for Demo {
    fn default() -> Self {
        let mut demo = Self {
            context: Context::default(),
            runtime: MotionRuntime::new(),
            default: Button::default(),
            primary: Button::primary(),
            secondary: Button::secondary(),
            quiet: Button::quiet(),
            disabled: Button::primary().disabled(true),
            family: Button::quiet(),
            scheme: Button::quiet(),
            reduce_motion: Button::quiet(),
            activations: 0,
        };

        let mut cx = UpdateCx::new(&mut demo.runtime, &mut demo.context);
        iced_component::core::register_components!(
            cx,
            [
                demo.default,
                demo.primary,
                demo.secondary,
                demo.quiet,
                demo.disabled,
                demo.family,
                demo.scheme,
                demo.reduce_motion,
            ]
        );
        demo
    }
}

impl Demo {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Frame(now) => aura_anim::iced::frame(&mut self.runtime, now),
            Message::Button(target, event) => self.update_button(target, event),
        }
        Task::none()
    }

    fn update_button(&mut self, target: Target, event: ButtonEvent) {
        let mut cx = UpdateCx::new(&mut self.runtime, &mut self.context);
        let outcome = match target {
            Target::Default => self.default.update_event(event, &mut cx),
            Target::Primary => self.primary.update_event(event, &mut cx),
            Target::Secondary => self.secondary.update_event(event, &mut cx),
            Target::Quiet => self.quiet.update_event(event, &mut cx),
            Target::Family => self.family.update_event(event, &mut cx),
            Target::Scheme => self.scheme.update_event(event, &mut cx),
            Target::ReduceMotion => self.reduce_motion.update_event(event, &mut cx),
        };
        if !outcome.is_ok_and(ButtonOutcome::is_activated) {
            return;
        }

        match target {
            Target::Default | Target::Primary | Target::Secondary | Target::Quiet => {
                self.activations += 1;
            }
            Target::Family => {
                cx.toggle_family();
            }
            Target::Scheme => {
                cx.toggle_color_scheme();
            }
            Target::ReduceMotion => {
                cx.toggle_reduce_motion();
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let cx = ViewCx::new(&self.runtime, &self.context);
        let family = match cx.family() {
            ThemeFamily::Adwaita => "Use Material",
            ThemeFamily::Material => "Use Adwaita",
        };
        let scheme = match cx.color_scheme() {
            ColorScheme::Dark => "Use light",
            ColorScheme::Light => "Use dark",
        };
        let motion = if cx.reduce_motion() {
            "Enable motion"
        } else {
            "Reduce motion"
        };
        let label = |value| text(value).size(15);

        let buttons = row![
            self.default
                .view(&cx)
                .content(label("Default"))
                .on_event(|event| Message::Button(Target::Default, event)),
            self.primary
                .view(&cx)
                .content(label("Primary"))
                .on_event(|event| Message::Button(Target::Primary, event)),
            self.secondary
                .view(&cx)
                .content(label("Secondary"))
                .on_event(|event| Message::Button(Target::Secondary, event)),
            self.quiet
                .view(&cx)
                .content(label("Quiet"))
                .on_event(|event| Message::Button(Target::Quiet, event)),
            self.disabled.view(&cx).content(label("Disabled")),
        ]
        .spacing(12)
        .wrap();
        let controls = row![
            self.family
                .view(&cx)
                .content(label(family))
                .on_event(|event| Message::Button(Target::Family, event)),
            self.scheme
                .view(&cx)
                .content(label(scheme))
                .on_event(|event| Message::Button(Target::Scheme, event)),
            self.reduce_motion
                .view(&cx)
                .content(label(motion))
                .on_event(|event| Message::Button(Target::ReduceMotion, event)),
        ]
        .spacing(12)
        .wrap();

        container(
            column![
                text(format!("Adapter Button · {} activations", self.activations)).size(22),
                buttons,
                controls,
            ]
            .spacing(22),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(32)
        .center_y(Length::Fill)
        .into()
    }

    fn iced_theme(&self) -> Theme {
        match self.context.color_scheme() {
            ColorScheme::Dark => Theme::Dark,
            ColorScheme::Light => Theme::Light,
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        subscription_with_policy(
            &self.runtime,
            TickPolicy::interval(core::time::Duration::from_millis(16)),
        )
        .map(Message::Frame)
    }
}
