use aura_anim::prelude::Duration;
use iced_component_core::anim::MotionRuntime;
use spectrum_theme::Color;

use crate::context::{Context, ThemeMode, UpdateCx, ViewCx};

use super::{Button, ButtonEvent, ButtonSignal, ButtonStyleState, ButtonSync, ButtonVariant};

#[test]
fn variants_resolve_their_own_material_token_groups() {
    let runtime = MotionRuntime::new();
    let context = Context::light();
    let cx = ViewCx::new(&runtime, &context);

    let filled = Button::filled().snapshot(&cx).unwrap();
    let outlined = Button::outlined().snapshot(&cx).unwrap();

    assert_eq!(
        filled.visual.background,
        context.theme().pack().palette.primary.color
    );
    assert_eq!(
        outlined.visual.border,
        context.theme().pack().palette.outline.color
    );
    assert!((outlined.visual.background_opacity - 0.0).abs() < f32::EPSILON);
}

#[test]
fn unregistered_interaction_updates_the_final_state_without_runtime_motion() {
    let mut runtime = MotionRuntime::new();
    let mut context = Context::light();
    let mut button = Button::filled();

    let changed = button
        .update(
            ButtonSignal::HoverEnter,
            &mut UpdateCx::new(&mut runtime, &mut context),
        )
        .unwrap();
    let snapshot = button.snapshot(&ViewCx::new(&runtime, &context)).unwrap();

    assert!(!changed);
    assert_eq!(runtime.motion_count(), 0);
    assert_eq!(snapshot.style_state, ButtonStyleState::Hover);
    assert!((snapshot.visual.state_layer_opacity - 0.08).abs() < f32::EPSILON);
}

#[test]
fn registered_interaction_animates_the_state_layer() {
    let mut runtime = MotionRuntime::new();
    let mut context = Context::light();
    let mut button = Button::filled_tonal();

    {
        let mut cx = UpdateCx::new(&mut runtime, &mut context);
        button.register(&mut cx);
        assert!(button.update(ButtonSignal::HoverEnter, &mut cx).unwrap());
    }
    runtime.tick(Duration::from_millis(15.0));

    let snapshot = button.snapshot(&ViewCx::new(&runtime, &context)).unwrap();

    assert_eq!(runtime.motion_count(), 1);
    assert_eq!(snapshot.style_state, ButtonStyleState::Hover);
    assert!((snapshot.visual.state_layer_opacity - 0.08).abs() < f32::EPSILON);
}

#[test]
fn registering_a_button_allocates_only_its_theme_visual_motion() {
    let mut runtime = MotionRuntime::new();
    let mut context = Context::light();
    let mut button = Button::filled();

    {
        let mut cx = UpdateCx::new(&mut runtime, &mut context);
        button.register(&mut cx);
    }

    assert_eq!(runtime.motion_count(), 1);
}

#[test]
fn focus_uses_the_material_focus_state_layer() {
    let mut runtime = MotionRuntime::new();
    let mut context = Context::light();
    let mut button = Button::outlined();

    button
        .update(
            ButtonSignal::Focus,
            &mut UpdateCx::new(&mut runtime, &mut context),
        )
        .unwrap();

    let snapshot = button.snapshot(&ViewCx::new(&runtime, &context)).unwrap();

    assert_eq!(snapshot.style_state, ButtonStyleState::Focus);
    assert!((snapshot.visual.state_layer_opacity - 0.10).abs() < f32::EPSILON);
}

#[test]
fn pressed_state_uses_the_material_press_state_layer() {
    let mut runtime = MotionRuntime::new();
    let mut context = Context::light();

    for variant in [
        ButtonVariant::Elevated,
        ButtonVariant::Filled,
        ButtonVariant::FilledTonal,
        ButtonVariant::Outlined,
        ButtonVariant::Text,
    ] {
        let mut button = Button::with_variant(variant);
        button
            .update(
                ButtonSignal::PressDown,
                &mut UpdateCx::new(&mut runtime, &mut context),
            )
            .unwrap();
        let snapshot = button.snapshot(&ViewCx::new(&runtime, &context)).unwrap();

        assert!((snapshot.visual.state_layer_opacity - 0.10).abs() < f32::EPSILON);
    }
}

#[test]
fn disabled_button_does_not_activate() {
    let mut runtime = MotionRuntime::new();
    let mut context = Context::light();
    let mut button = Button::text().disabled(true);

    let outcome = button
        .update_event(
            ButtonEvent::Pressed,
            &mut UpdateCx::new(&mut runtime, &mut context),
        )
        .unwrap();

    assert_eq!(outcome, super::ButtonOutcome::None);
    assert_eq!(button.style_state(), ButtonStyleState::Disabled);
}

#[test]
fn source_backed_seed_sync_uses_the_old_visual_as_its_start() {
    let mut runtime = MotionRuntime::new();
    let mut context = Context::light();
    let mut button = Button::filled();
    let old_background = context.theme().pack().palette.primary.color;

    {
        let mut cx = UpdateCx::new(&mut runtime, &mut context);
        button.register(&mut cx);
        let change = cx.set_seed(Color::new(0, 90, 220)).unwrap();
        button
            .update(
                ButtonSignal::Sync(ButtonSync::StyleChanged(change)),
                &mut cx,
            )
            .unwrap();
    }

    assert_eq!(
        button
            .motion_value(&runtime)
            .unwrap()
            .unwrap()
            .visual
            .background
            .rgba(),
        old_background.rgba()
    );

    runtime.tick(Duration::from_millis(200.0));

    let snapshot = button.snapshot(&ViewCx::new(&runtime, &context)).unwrap();
    assert_ne!(snapshot.visual.background, old_background);
    assert_eq!(button.variant(), ButtonVariant::FILLED);
    assert_eq!(context.theme().mode(), ThemeMode::Light);
}
