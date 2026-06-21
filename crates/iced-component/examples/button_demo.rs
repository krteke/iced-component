//! Visual Iced demo for `AnimatedButton`.

use std::time::Duration;

use iced::widget::{button, column, container, row, text};
use iced::{Element, Fill, Subscription, Task, Theme, application, time};
use iced_component::MotionRuntime;
use iced_component::button::{AnimatedButton, ButtonEvent};
use iced_component::component::ComponentContext;
use iced_component::motion::{MotionPreferences, MotionPreferencesController};

fn main() -> iced::Result {
    application(Demo::new, update, view)
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
    save_button: AnimatedButton,
    reset_button: AnimatedButton,
    clicks: u32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Tick,
    SaveButton(ButtonEvent<SaveAction>),
    ResetButton(ButtonEvent<ResetAction>),
    ToggleReduceMotion,
}

#[derive(Debug, Clone, Copy)]
enum SaveAction {
    Save,
}

#[derive(Debug, Clone, Copy)]
enum ResetAction {
    Reset,
}

impl Demo {
    fn new() -> Self {
        let (preferences, reduce_motion) = MotionPreferences::new(false);
        let mut runtime = MotionRuntime::new();
        let context = ComponentContext::current().with_motion_preferences(preferences);
        let mut save_button = AnimatedButton::primary("Save");
        let mut reset_button = AnimatedButton::standard("Reset");

        save_button.register(&mut runtime, &context);
        reset_button.register(&mut runtime, &context);

        Self {
            runtime,
            context,
            reduce_motion,
            save_button,
            reset_button,
            clicks: 0,
        }
    }
}

fn update(state: &mut Demo, message: Message) -> Task<Message> {
    match message {
        Message::Tick => {
            state
                .runtime
                .tick(iced_component::motion::Duration::from_millis(16.0));
        }
        Message::SaveButton(event) => {
            if state
                .save_button
                .update_event(event, &mut state.runtime)
                .ok()
                .flatten()
                .is_some()
            {
                state.clicks += 1;
            }
        }
        Message::ResetButton(event) => {
            if state
                .reset_button
                .update_event(event, &mut state.runtime)
                .ok()
                .flatten()
                .is_some()
            {
                state.clicks = 0;
            }
        }
        Message::ToggleReduceMotion => {
            let next = !state.reduce_motion.reduce_motion();
            state.reduce_motion.set_reduce_motion(next);
        }
    }

    Task::none()
}

fn subscription(_state: &Demo) -> Subscription<Message> {
    time::every(Duration::from_millis(16)).map(|_| Message::Tick)
}

fn theme(_state: &Demo) -> Theme {
    Theme::Dark
}

fn view(state: &Demo) -> Element<'_, Message> {
    let save = state
        .save_button
        .view(&state.runtime, &state.context)
        .on_press(SaveAction::Save)
        .on_event(Message::SaveButton);
    let reset = state
        .reset_button
        .view(&state.runtime, &state.context)
        .on_press(ResetAction::Reset)
        .on_event(Message::ResetButton);

    let snapshot = state
        .save_button
        .snapshot(&state.runtime, &state.context)
        .expect("button motion handle belongs to the demo runtime");

    let reduce_label = if state.reduce_motion.reduce_motion() {
        "Reduce motion: on"
    } else {
        "Reduce motion: off"
    };

    let content = column![
        text("AnimatedButton").size(28),
        text("Hover, press, and toggle reduced motion to see the component runtime path."),
        row![
            save,
            reset,
            button(text(reduce_label))
                .on_press(Message::ToggleReduceMotion)
                .padding([8.0, 12.0]),
        ]
        .spacing(12),
        text(format!(
            "motion: scale {:.2}, shadow_y {:.2}, bg_alpha {:.2}",
            snapshot.motion.scale, snapshot.motion.shadow_y, snapshot.motion.bg_alpha
        ))
        .size(14),
    ]
    .spacing(16);

    container(content)
        .padding(24)
        .center_x(Fill)
        .center_y(Fill)
        .into()
}
