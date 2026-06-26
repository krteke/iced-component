//! Visual Iced demo for `Button`.

use std::time::Duration;

use iced::widget::{column, container, row, text};
use iced::{Element, Fill, Subscription, Task, Theme, application, time};
use iced_component::anim::MotionRuntime;
use iced_component::button::{Button, ButtonEvent, IconButton, IconSource};
use iced_component::component::{ComponentContext, ComponentUpdateCx, ComponentViewCx};

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
        let mut runtime = MotionRuntime::new();
        let context = ComponentContext::default();
        let mut save_button = Button::suggested("Save 0");
        let mut reset_button = Button::standard("Reset").flat();
        let mut motion_button = IconButton::suggested(IconSource::svg_static(MOTION_ICON));

        iced_component::register_components!(runtime, [save_button, reset_button, motion_button]);

        Self {
            runtime,
            context,
            save_button,
            reset_button,
            motion_button,
            clicks: 0,
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
            Message::SaveButton(event) => match self.save_button.update_event(event, &mut cx) {
                Ok(Some(SaveAction::Save)) => {
                    self.motion_error = None;
                    self.clicks += 1;
                    self.save_button
                        .set_content(format!("Save {}", self.clicks));
                }
                Ok(None) => self.motion_error = None,
                Err(error) => self.motion_error = Some(error.to_string()),
            },
            Message::ResetButton(event) => match self.reset_button.update_event(event, &mut cx) {
                Ok(Some(ResetAction::Reset)) => {
                    self.motion_error = None;
                    self.clicks = 0;
                    self.save_button.set_content("Save 0");
                }
                Ok(None) => self.motion_error = None,
                Err(error) => self.motion_error = Some(error.to_string()),
            },
            Message::MotionButton(event) => match self.motion_button.update_event(event, &mut cx) {
                Ok(Some(MotionAction::Toggle)) => {
                    let reduce_motion = !cx.context().reduce_motion();
                    cx.context_mut().set_reduce_motion(reduce_motion);

                    match iced_component::sync_components!(
                        cx,
                        [self.save_button, self.reset_button, self.motion_button]
                    ) {
                        Ok(_) => self.motion_error = None,
                        Err(error) => self.motion_error = Some(error.to_string()),
                    }
                }
                Ok(None) => self.motion_error = None,
                Err(error) => self.motion_error = Some(error.to_string()),
            },
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let cx = ComponentViewCx::new(&self.runtime, &self.context);
        let save = self
            .save_button
            .view(&cx)
            .connect(SaveAction::Save, Message::SaveButton);
        let reset = self
            .reset_button
            .view(&cx)
            .connect(ResetAction::Reset, Message::ResetButton);
        let motion = self
            .motion_button
            .view(&cx)
            .connect(MotionAction::Toggle, Message::MotionButton);

        let snapshot = self
            .save_button
            .snapshot(&cx)
            .expect("button motion handle belongs to the demo runtime");

        let reduce_label = if self.context.reduce_motion() {
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

fn subscription(_state: &Demo) -> Subscription<Message> {
    time::every(Duration::from_millis(16)).map(|_| Message::Tick)
}

fn theme(_state: &Demo) -> Theme {
    Theme::Light
}
