use aura_anim_core::{MotionRuntime, timing::Duration};
use float_cmp::assert_approx_eq;

use crate::{
    button::{Button, ButtonContent, ButtonRole, ButtonStyleState, ButtonVariant},
    component::ComponentContext,
    motion::{MotionSpeed, MotionTokens, MotionTransition},
};

use super::{ButtonEvent, ButtonInteraction, ButtonMotion};

#[test]
fn interaction_before_registration_updates_target_without_runtime_motion() {
    let mut runtime = MotionRuntime::new();
    let mut button = Button::standard("Save");

    let changed = button
        .update(ButtonInteraction::HoverEnter, &mut runtime)
        .unwrap();

    assert!(!changed);
    assert_eq!(runtime.motion_count(), 0);
    assert_eq!(
        button.motion_value(&runtime).unwrap(),
        ButtonMotion {
            scale: 1.0,
            shadow_y: 1.2,
            bg_alpha: 1.0,
            border_glow: 0.0,
            focus_alpha: 0.0,
        }
    );
}

#[test]
fn registered_hover_transitions_runtime_motion() {
    let mut runtime = MotionRuntime::new();
    let context = ComponentContext::current();
    let mut button = Button::suggested("Save");

    button.register(&mut runtime, &context);
    let changed = button
        .update(ButtonInteraction::HoverEnter, &mut runtime)
        .unwrap();
    runtime.tick(Duration::from_millis(200.0));

    assert!(changed);
    assert_eq!(runtime.motion_count(), 1);
    assert_approx_eq!(f32, button.motion_value(&runtime).unwrap().shadow_y, 1.2);
    assert_eq!(button.variant(), ButtonVariant::SUGGESTED);
}

#[test]
fn register_uses_context_interaction_motion_token() {
    let mut runtime = MotionRuntime::new();
    let context = ComponentContext::current().with_motion_tokens(MotionTokens {
        interaction: MotionTransition::new(MotionSpeed::Fast, iced::animation::Easing::Linear),
        fast: Duration::from_millis(40.0),
        ..MotionTokens::default()
    });
    let mut button = Button::suggested("Save");

    button.register(&mut runtime, &context);
    button
        .update(ButtonInteraction::HoverEnter, &mut runtime)
        .unwrap();
    runtime.tick(Duration::from_millis(40.0));

    assert_approx_eq!(f32, button.motion_value(&runtime).unwrap().shadow_y, 1.2);
}

#[test]
fn builders_update_role_and_appearance() {
    let button = Button::standard("Save")
        .with_role(ButtonRole::Suggested)
        .flat();

    assert_eq!(button.variant(), ButtonVariant::SUGGESTED.set_flat());

    let button = Button::destructive("Delete").raised();

    assert_eq!(button.variant(), ButtonVariant::DESTRUCTIVE.set_raised());

    let button = Button::standard("Save").pill();
    assert_eq!(button.variant(), ButtonVariant::STANDARD.set_pill());

    let button = Button::standard("i").circular();
    assert_eq!(button.variant(), ButtonVariant::STANDARD.set_circular());
}

#[test]
fn disabled_button_ignores_press_down() {
    let mut runtime = MotionRuntime::new();
    let context = ComponentContext::current();
    let mut button = Button::standard("Save");

    button.register(&mut runtime, &context);
    button
        .update(ButtonInteraction::SetDisabled(true), &mut runtime)
        .unwrap();
    button
        .update(ButtonInteraction::PressDown, &mut runtime)
        .unwrap();
    runtime.tick(Duration::from_millis(200.0));

    let motion = button.motion_value(&runtime).unwrap();
    assert_approx_eq!(f32, motion.scale, 1.0);
    assert_approx_eq!(f32, motion.bg_alpha, 0.45);
}

#[test]
fn set_disabled_updates_button_state() {
    let mut runtime = MotionRuntime::new();
    let mut button = Button::standard("Save");

    button.set_disabled(true, &mut runtime).unwrap();

    let snapshot = button
        .snapshot(&runtime, &ComponentContext::current())
        .unwrap();
    assert!(snapshot.disabled);
    assert_eq!(snapshot.style_state, ButtonStyleState::Disabled);
}

#[test]
fn pressed_event_releases_button_and_returns_action() {
    let mut runtime = MotionRuntime::new();
    let mut button = Button::standard("Save");

    button
        .update(ButtonInteraction::PressDown, &mut runtime)
        .unwrap();
    let action = button
        .update_event(ButtonEvent::Pressed("save"), &mut runtime)
        .unwrap();

    assert_eq!(action, Some("save"));
    assert_eq!(
        button
            .snapshot(&runtime, &ComponentContext::current())
            .unwrap()
            .style_state,
        ButtonStyleState::Idle
    );
}

#[test]
fn pressed_event_returns_to_hovered_when_pointer_is_inside() {
    let mut runtime = MotionRuntime::new();
    let mut button = Button::standard("Save");

    button
        .update(ButtonInteraction::HoverEnter, &mut runtime)
        .unwrap();
    button
        .update(ButtonInteraction::PressDown, &mut runtime)
        .unwrap();
    button
        .update_event(ButtonEvent::Pressed("save"), &mut runtime)
        .unwrap();

    assert_eq!(
        button
            .snapshot(&runtime, &ComponentContext::current())
            .unwrap()
            .style_state,
        ButtonStyleState::Hovered
    );
}

#[test]
fn update_event_with_invokes_action_only_for_pressed_event() {
    let mut runtime = MotionRuntime::new();
    let mut button = Button::standard("Save");
    let mut action_count = 0;

    let handled = button
        .update_event_with(
            ButtonEvent::Interaction(ButtonInteraction::HoverEnter),
            &mut runtime,
            |()| action_count += 1,
        )
        .unwrap();

    assert!(!handled);
    assert_eq!(action_count, 0);

    let handled = button
        .update_event_with(ButtonEvent::Pressed(()), &mut runtime, |()| {
            action_count += 1;
        })
        .unwrap();

    assert!(handled);
    assert_eq!(action_count, 1);
}

#[test]
fn update_event_with_ignores_pressed_action_when_disabled() {
    let mut runtime = MotionRuntime::new();
    let mut button = Button::standard("Save");
    let mut action_count = 0;

    button
        .update(ButtonInteraction::SetDisabled(true), &mut runtime)
        .unwrap();
    let handled = button
        .update_event_with(ButtonEvent::Pressed(()), &mut runtime, |()| {
            action_count += 1;
        })
        .unwrap();

    assert!(!handled);
    assert_eq!(action_count, 0);
}

#[test]
fn snapshot_combines_style_and_motion() {
    let mut runtime = MotionRuntime::new();
    let context = ComponentContext::current();
    let mut button = Button::suggested("Save");

    button.register(&mut runtime, &context);
    button
        .update(ButtonInteraction::PressDown, &mut runtime)
        .unwrap();
    runtime.tick(Duration::from_millis(200.0));

    let snapshot = button.snapshot(&runtime, &context).unwrap();

    assert_eq!(snapshot.variant, ButtonVariant::SUGGESTED);
    assert_eq!(snapshot.style_state, ButtonStyleState::Pressed);
    assert_eq!(
        snapshot.style.background,
        context.theme().theme().button.suggested.filled.pressed.bg
    );
    assert_approx_eq!(f32, snapshot.motion.scale, 0.98);
}

#[test]
fn snapshot_reports_focus_and_disabled_state() {
    let mut runtime = MotionRuntime::new();
    let context = ComponentContext::current();
    let mut button = Button::standard("Save");

    button
        .update(ButtonInteraction::Focus, &mut runtime)
        .unwrap();
    button
        .update(ButtonInteraction::SetDisabled(true), &mut runtime)
        .unwrap();

    let snapshot = button.snapshot(&runtime, &context).unwrap();

    assert!(snapshot.focused);
    assert!(snapshot.disabled);
    assert_eq!(snapshot.style_state, ButtonStyleState::Disabled);
    assert_approx_eq!(f32, snapshot.motion.focus_alpha, 0.5);
}

#[test]
fn button_stores_stable_content_and_layout() {
    let mut button = Button::standard("Save").width(120.0).height(34.0);

    assert_eq!(button.content().as_text(), Some("Save"));
    assert_eq!(button.layout().width, Some(iced::Length::Fixed(120.0)));
    assert_eq!(button.layout().height, Some(iced::Length::Fixed(34.0)));

    button.set_content(ButtonContent::text("Saved"));

    assert_eq!(button.content().as_text(), Some("Saved"));
}
