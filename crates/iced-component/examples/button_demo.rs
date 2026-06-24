//! Visual Iced demo for `Button`.

use std::time::Duration;

use iced::widget::{column, container, row, text};
use iced::{Element, Fill, Subscription, Task, Theme, application, time};
use iced_component::button::{Button, ButtonEvent, IconButton, IconSource};
use iced_component::component::ComponentContext;
use iced_component::motion::{MotionPreferences, MotionPreferencesController};
use iced_component::{MotionError, MotionRuntime};

const MOTION_ICON: &[u8] = br#"
<svg viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg">
  <path d="M2 4h8v1.5H2zM12 3a2 2 0 1 1 0 4 2 2 0 0 1 0-4zM6 9h8v1.5H6zM4 8a2 2 0 1 0 0 4 2 2 0 0 0 0-4z"/>
</svg>
"#;

fn main() -> iced::Result {
    application(Demo::new, Demo::update, Demo::view)
        .title("aura-iced-component button demo")
        .subscription(subscription)
        .theme(theme)
        .window_size([420.0, 260.0])
        .run()
}

struct Demo {
    runtime: MotionRuntime,
    context: ComponentContext,
    reduce_motion: MotionPreferencesController,
    save_button: Button,
    reset_button: Button,
    motion_button: IconButton,
    clicks: u32,
    motion_error: Option<String>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Tick,
    SaveButton(ButtonEvent<SaveAction>),
    ResetButton(ButtonEvent<ResetAction>),
    MotionButton(ButtonEvent<MotionAction>),
}

#[derive(Debug, Clone, Copy)]
enum SaveAction {
    Save,
}

#[derive(Debug, Clone, Copy)]
enum ResetAction {
    Reset,
}

#[derive(Debug, Clone, Copy)]
enum MotionAction {
    Toggle,
}

impl Demo {
    fn new() -> Self {
        let (preferences, reduce_motion) = MotionPreferences::new(false);
        let mut runtime = MotionRuntime::new();
        let context = ComponentContext::current().with_motion_preferences(preferences);
        let mut save_button = Button::suggested("Save 0");
        let mut reset_button = Button::standard("Reset").flat();
        let mut motion_button = IconButton::suggested(IconSource::svg_bytes(MOTION_ICON));

        save_button.register(&mut runtime, &context);
        reset_button.register(&mut runtime, &context);
        motion_button.register(&mut runtime, &context);

        Self {
            runtime,
            context,
            reduce_motion,
            save_button,
            reset_button,
            motion_button,
            clicks: 0,
            motion_error: None,
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Tick => {
                self.runtime
                    .tick(iced_component::motion::Duration::from_millis(16.0));
            }
            Message::SaveButton(event) => {
                let result = self.save_button.update_event_with(
                    event,
                    &mut self.runtime,
                    |SaveAction::Save| {
                        self.clicks += 1;
                    },
                );
                let handled = matches!(result, Ok(true));
                record_motion_result(self, result);
                if handled {
                    self.save_button
                        .set_content(format!("Save {}", self.clicks));
                }
            }
            Message::ResetButton(event) => {
                let result = self.reset_button.update_event_with(
                    event,
                    &mut self.runtime,
                    |ResetAction::Reset| {
                        self.clicks = 0;
                    },
                );
                let handled = matches!(result, Ok(true));
                record_motion_result(self, result);
                if handled {
                    self.save_button.set_content("Save 0");
                }
            }
            Message::MotionButton(event) => {
                let result = self.motion_button.update_event_with(
                    event,
                    &mut self.runtime,
                    |MotionAction::Toggle| {
                        let next = !self.reduce_motion.reduce_motion();
                        self.reduce_motion.set_reduce_motion(next);
                    },
                );
                record_motion_result(self, result);
            }
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let save = self
            .save_button
            .view(&self.runtime, &self.context)
            .connect(SaveAction::Save, Message::SaveButton);
        let reset = self
            .reset_button
            .view(&self.runtime, &self.context)
            .connect(ResetAction::Reset, Message::ResetButton);
        let motion = self
            .motion_button
            .view(&self.runtime, &self.context)
            .connect(MotionAction::Toggle, Message::MotionButton);

        let snapshot = self
            .save_button
            .snapshot(&self.runtime, &self.context)
            .expect("button motion handle belongs to the demo runtime");

        let reduce_label = if self.reduce_motion.reduce_motion() {
            "Reduce motion: on"
        } else {
            "Reduce motion: off"
        };

        let content = column![
            text("AnimatedButton").size(28),
            text("Hover, press, and toggle reduced motion to see the component runtime path."),
            row![save, reset, motion, text(reduce_label).size(14),].spacing(12),
            text(format!(
                "motion: scale {:.2}, shadow_y {:.2}, bg_alpha {:.2}",
                snapshot.motion.scale, snapshot.motion.shadow_y, snapshot.motion.bg_alpha
            ))
            .size(14),
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
