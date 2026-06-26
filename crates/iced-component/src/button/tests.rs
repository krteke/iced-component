use aura_anim::prelude::*;
use float_cmp::assert_approx_eq;

use crate::{
    button::{
        Button, ButtonContent, ButtonRole, ButtonShape, ButtonStyleState, ButtonTreatment,
        ButtonVariant,
    },
    component::{ComponentContext, ComponentUpdateCx, ComponentViewCx},
};

use super::{ButtonEvent, ButtonInteraction, ButtonMotion};

#[test]
fn interaction_before_registration_updates_target_without_runtime_motion() {
    let mut runtime = MotionRuntime::new();
    let mut context = ComponentContext::adwaita();
    let mut button = Button::standard("Save");

    let changed = {
        let mut cx = ComponentUpdateCx::new(&mut runtime, &mut context);
        button
            .update(ButtonInteraction::HoverEnter, &mut cx)
            .unwrap()
    };

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
    let mut context = ComponentContext::adwaita();
    let mut button = Button::suggested("Save");

    button.register(&mut runtime);
    let changed = {
        let mut cx = ComponentUpdateCx::new(&mut runtime, &mut context);
        button
            .update(ButtonInteraction::HoverEnter, &mut cx)
            .unwrap()
    };
    runtime.tick(Duration::from_millis(200.0));

    assert!(changed);
    assert_eq!(runtime.motion_count(), 1);
    assert_approx_eq!(f32, button.motion_value(&runtime).unwrap().shadow_y, 1.2);
    assert_eq!(button.variant(), ButtonVariant::SUGGESTED);
}

#[test]
fn update_respects_context_reduced_motion() {
    let mut runtime = MotionRuntime::new();
    let mut context = ComponentContext::adwaita().with_reduce_motion(true);
    let mut button = Button::suggested("Save");

    button.register(&mut runtime);
    {
        let mut cx = ComponentUpdateCx::new(&mut runtime, &mut context);
        button
            .update(ButtonInteraction::HoverEnter, &mut cx)
            .unwrap();
    }
    runtime.tick(Duration::from_millis(1.0));

    assert_approx_eq!(f32, button.motion_value(&runtime).unwrap().shadow_y, 1.2);
}

#[test]
fn reduce_motion_is_scoped_to_component_context() {
    let mut runtime = MotionRuntime::new();
    let mut reduced_context = ComponentContext::adwaita().with_reduce_motion(true);
    let mut regular_context = ComponentContext::adwaita();
    let mut reduced = Button::suggested("Reduced");
    let mut regular = Button::suggested("Regular");

    reduced.register(&mut runtime);
    regular.register(&mut runtime);
    {
        let mut cx = ComponentUpdateCx::new(&mut runtime, &mut reduced_context);
        reduced
            .update(ButtonInteraction::HoverEnter, &mut cx)
            .unwrap();
    }
    {
        let mut cx = ComponentUpdateCx::new(&mut runtime, &mut regular_context);
        regular
            .update(ButtonInteraction::HoverEnter, &mut cx)
            .unwrap();
    }
    runtime.tick(Duration::from_millis(1.0));

    assert_approx_eq!(f32, reduced.motion_value(&runtime).unwrap().shadow_y, 1.2);
    assert!(regular.motion_value(&runtime).unwrap().shadow_y < 1.2);
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
    let mut context = ComponentContext::adwaita();
    let mut button = Button::standard("Save");

    button.register(&mut runtime);
    {
        let mut cx = ComponentUpdateCx::new(&mut runtime, &mut context);
        button
            .update(ButtonInteraction::SetDisabled(true), &mut cx)
            .unwrap();
        button
            .update(ButtonInteraction::PressDown, &mut cx)
            .unwrap();
    }
    runtime.tick(Duration::from_millis(200.0));

    let motion = button.motion_value(&runtime).unwrap();
    assert_approx_eq!(f32, motion.scale, 1.0);
    assert_approx_eq!(f32, motion.bg_alpha, 0.45);
}

#[test]
fn set_disabled_updates_button_state() {
    let mut runtime = MotionRuntime::new();
    let mut context = ComponentContext::adwaita();
    let mut button = Button::standard("Save");

    {
        let mut cx = ComponentUpdateCx::new(&mut runtime, &mut context);
        button.set_disabled(true, &mut cx).unwrap();
    }

    let cx = ComponentViewCx::new(&runtime, &context);
    let snapshot = button.snapshot(&cx).unwrap();
    assert!(snapshot.disabled);
    assert_eq!(snapshot.style_state, ButtonStyleState::Disabled);
}

#[test]
fn pressed_event_releases_button_and_returns_action() {
    let mut runtime = MotionRuntime::new();
    let mut context = ComponentContext::adwaita();
    let mut button = Button::standard("Save");

    let action = {
        let mut cx = ComponentUpdateCx::new(&mut runtime, &mut context);
        button
            .update(ButtonInteraction::PressDown, &mut cx)
            .unwrap();
        button
            .update_event(ButtonEvent::Pressed("save"), &mut cx)
            .unwrap()
    };

    assert_eq!(action, Some("save"));
    let cx = ComponentViewCx::new(&runtime, &context);
    assert_eq!(
        button.snapshot(&cx).unwrap().style_state,
        ButtonStyleState::Idle
    );
}

#[test]
fn pressed_event_returns_to_hovered_when_pointer_is_inside() {
    let mut runtime = MotionRuntime::new();
    let mut context = ComponentContext::adwaita();
    let mut button = Button::standard("Save");

    {
        let mut cx = ComponentUpdateCx::new(&mut runtime, &mut context);
        button
            .update(ButtonInteraction::HoverEnter, &mut cx)
            .unwrap();
        button
            .update(ButtonInteraction::PressDown, &mut cx)
            .unwrap();
        button
            .update_event(ButtonEvent::Pressed("save"), &mut cx)
            .unwrap();
    }

    let cx = ComponentViewCx::new(&runtime, &context);
    assert_eq!(
        button.snapshot(&cx).unwrap().style_state,
        ButtonStyleState::Hovered
    );
}

#[test]
fn update_event_with_invokes_action_only_for_pressed_event() {
    let mut runtime = MotionRuntime::new();
    let mut context = ComponentContext::adwaita();
    let mut button = Button::standard("Save");
    let mut action_count = 0;

    let handled = {
        let mut cx = ComponentUpdateCx::new(&mut runtime, &mut context);
        button.update_event_with(
            ButtonEvent::Interaction(ButtonInteraction::HoverEnter),
            &mut cx,
            |()| action_count += 1,
        )
    }
    .unwrap();

    assert!(!handled);
    assert_eq!(action_count, 0);

    let handled = {
        let mut cx = ComponentUpdateCx::new(&mut runtime, &mut context);
        button.update_event_with(ButtonEvent::Pressed(()), &mut cx, |()| {
            action_count += 1;
        })
    }
    .unwrap();

    assert!(handled);
    assert_eq!(action_count, 1);
}

#[test]
fn update_event_with_ignores_pressed_action_when_disabled() {
    let mut runtime = MotionRuntime::new();
    let mut context = ComponentContext::adwaita();
    let mut button = Button::standard("Save");
    let mut action_count = 0;

    let handled = {
        let mut cx = ComponentUpdateCx::new(&mut runtime, &mut context);
        button
            .update(ButtonInteraction::SetDisabled(true), &mut cx)
            .unwrap();
        button.update_event_with(ButtonEvent::Pressed(()), &mut cx, |()| {
            action_count += 1;
        })
    }
    .unwrap();

    assert!(!handled);
    assert_eq!(action_count, 0);
}

#[test]
fn snapshot_combines_style_and_motion() {
    let mut runtime = MotionRuntime::new();
    let mut context = ComponentContext::adwaita();
    let mut button = Button::suggested("Save");

    button.register(&mut runtime);
    {
        let mut cx = ComponentUpdateCx::new(&mut runtime, &mut context);
        button
            .update(ButtonInteraction::PressDown, &mut cx)
            .unwrap();
    }
    runtime.tick(Duration::from_millis(200.0));

    let cx = ComponentViewCx::new(&runtime, &context);
    let snapshot = button.snapshot(&cx).unwrap();

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
    let mut context = ComponentContext::adwaita();
    let mut button = Button::standard("Save");

    {
        let mut cx = ComponentUpdateCx::new(&mut runtime, &mut context);
        button.update(ButtonInteraction::Focus, &mut cx).unwrap();
        button
            .update(ButtonInteraction::SetDisabled(true), &mut cx)
            .unwrap();
    }

    let cx = ComponentViewCx::new(&runtime, &context);
    let snapshot = button.snapshot(&cx).unwrap();

    assert!(snapshot.focused);
    assert!(snapshot.disabled);
    assert_eq!(snapshot.style_state, ButtonStyleState::Disabled);
    assert_approx_eq!(f32, snapshot.motion.focus_alpha, 0.5);
}

#[test]
fn button_stores_stable_content_and_layout() {
    let mut button = Button::standard("Save").with_width(120.0).with_height(34.0);

    assert_eq!(button.content().as_text(), Some("Save"));
    assert_eq!(button.layout().width(), Some(iced::Length::Fixed(120.0)));
    assert_eq!(button.layout().height(), Some(iced::Length::Fixed(34.0)));

    button.set_content(ButtonContent::text("Saved"));

    assert_eq!(button.content().as_text(), Some("Saved"));
}

#[test]
fn setters_update_stable_button_configuration() {
    let mut button = Button::standard("Save")
        .with_padding([6.0, 12.0])
        .with_width(120.0)
        .with_height(34.0)
        .with_disabled(true);

    assert!(button.is_disabled());
    assert_eq!(button.layout().padding(), Some([6.0, 12.0]));
    assert_eq!(button.layout().width(), Some(iced::Length::Fixed(120.0)));
    assert_eq!(button.layout().height(), Some(iced::Length::Fixed(34.0)));

    button.clear_content();
    button.set_content("Saved");
    button.set_role(ButtonRole::Destructive);
    button.set_treatment(ButtonTreatment::Raised);
    button.set_shape(ButtonShape::Pill);
    button.set_square(40.0);

    assert_eq!(button.content().as_text(), Some("Saved"));
    assert_eq!(
        button.variant(),
        ButtonVariant::DESTRUCTIVE.set_raised().set_pill()
    );
    assert_eq!(button.layout().width(), Some(iced::Length::Fixed(40.0)));
    assert_eq!(button.layout().height(), Some(iced::Length::Fixed(40.0)));
    assert!(button.layout().center_content());

    button.clear_padding();
    button.clear_width();
    button.clear_height();
    button.set_center_content(false);

    assert_eq!(button.layout().padding(), None);
    assert_eq!(button.layout().width(), None);
    assert_eq!(button.layout().height(), None);
    assert!(!button.layout().center_content());
}
