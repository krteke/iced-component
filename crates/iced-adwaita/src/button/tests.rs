use aura_anim::prelude::{Duration, Timing};
use float_cmp::assert_approx_eq;
use iced::Length;
use iced_component_core::anim::MotionRuntime;
use spectrum_theme::Color;

use crate::{
    Context,
    context::{UpdateCx, ViewCx},
};

use super::{
    Button, ButtonAnimations, ButtonContentLayout, ButtonSignal, ButtonStyleState, ButtonSync,
};

#[test]
fn unregistered_snapshot_resolves_current_theme() {
    let runtime = MotionRuntime::new();
    let context = Context::light();
    let button = Button::new("Save");
    let view = ViewCx::new(&runtime, &context);

    let snapshot = button.snapshot(&view).unwrap();

    assert_eq!(runtime.motion_count(), 0);
    assert_eq!(snapshot.style_state, ButtonStyleState::Idle);
    assert_eq!(
        snapshot.style.background,
        "#00000614".parse::<Color>().unwrap()
    );
}

#[test]
fn unregistered_update_jumps_to_final_state_without_runtime_motion() {
    let mut runtime = MotionRuntime::new();
    let mut context = Context::light();
    let mut button = Button::new("Save");

    {
        let mut update = UpdateCx::new(&mut runtime, &mut context);
        assert!(
            !button
                .update(ButtonSignal::HoverEnter, &mut update)
                .unwrap()
        );
    }

    let view = ViewCx::new(&runtime, &context);
    let snapshot = button.snapshot(&view).unwrap();

    assert_eq!(runtime.motion_count(), 0);
    assert_eq!(snapshot.style_state, ButtonStyleState::Hovered);
    assert_eq!(
        snapshot.style.background,
        "#0000061f".parse::<Color>().unwrap()
    );
}

#[test]
fn registered_interaction_animates_to_target_state() {
    let mut runtime = MotionRuntime::new();
    let mut context = Context::light();
    let mut button = Button::new("Save");

    {
        let mut update = UpdateCx::new(&mut runtime, &mut context);
        button.register(&mut update);
        assert!(
            button
                .update(ButtonSignal::HoverEnter, &mut update)
                .unwrap()
        );
    }

    runtime.tick(Duration::from_millis(200.0));

    let view = ViewCx::new(&runtime, &context);
    let snapshot = button.snapshot(&view).unwrap();

    assert_eq!(runtime.motion_count(), 1);
    assert_eq!(snapshot.style_state, ButtonStyleState::Hovered);
    assert_eq!(
        snapshot.style.background,
        "#0000061f".parse::<Color>().unwrap()
    );
}

#[test]
fn typed_animation_override_controls_button_interactions() {
    let mut runtime = MotionRuntime::new();
    let mut context = Context::light();
    let mut button = Button::new("Save");
    let target = "#0000061f".parse::<Color>().unwrap();

    {
        let mut update = UpdateCx::new(&mut runtime, &mut context);
        update.set_animation_override(ButtonAnimations::tween(
            Timing::linear(1_000.0),
            Timing::linear(1_000.0),
        ));
        button.register(&mut update);
        button
            .update(ButtonSignal::HoverEnter, &mut update)
            .unwrap();
    }

    runtime.tick(Duration::from_millis(200.0));
    assert_ne!(
        button.motion_value(&runtime).unwrap().unwrap().tokens.bg,
        target
    );

    runtime.tick(Duration::from_millis(800.0));
    assert_eq!(
        button.motion_value(&runtime).unwrap().unwrap().tokens.bg,
        target
    );
}

#[test]
fn theme_transition_starts_from_the_previous_theme_tokens() {
    let mut runtime = MotionRuntime::new();
    let mut context = Context::light();
    let mut button = Button::new("Save");

    {
        let mut update = UpdateCx::new(&mut runtime, &mut context);
        button.register(&mut update);
        button
            .update(ButtonSignal::HoverEnter, &mut update)
            .unwrap();
    }
    {
        let mut update = UpdateCx::new(&mut runtime, &mut context);
        update.toggle_theme();
    }

    let view = ViewCx::new(&runtime, &context);
    let snapshot = button.snapshot(&view).unwrap();

    assert_eq!(
        snapshot.style.background,
        "#00000614".parse::<Color>().unwrap()
    );

    runtime.tick(Duration::from_millis(200.0));
    let view = ViewCx::new(&runtime, &context);
    assert_eq!(
        button.snapshot(&view).unwrap().style.background,
        "#ffffff26".parse::<Color>().unwrap()
    );
}

#[test]
fn sync_after_theme_revision_uses_stale_runtime_value_as_start() {
    let mut runtime = MotionRuntime::new();
    let mut context = Context::light();
    let mut button = Button::new("Save");
    let old_hover = "#0000061f".parse::<Color>().unwrap();
    let new_hover = Color::new_rgba(1, 2, 3, 255);

    {
        let mut update = UpdateCx::new(&mut runtime, &mut context);
        button.register(&mut update);
        button
            .update(ButtonSignal::HoverEnter, &mut update)
            .unwrap();
    }
    runtime.tick(Duration::from_millis(200.0));
    {
        let mut update = UpdateCx::new(&mut runtime, &mut context);
        let change = update.patch_theme(|theme| {
            theme.button.standard.hover.bg = new_hover;
        });
        button
            .sync_with(ButtonSync::StyleChanged(change), &mut update)
            .unwrap();
    }

    assert_eq!(
        button.motion_value(&runtime).unwrap().unwrap().tokens.bg,
        old_hover
    );

    runtime.tick(Duration::from_millis(200.0));

    assert_eq!(
        button.motion_value(&runtime).unwrap().unwrap().tokens.bg,
        new_hover
    );
}

#[test]
fn interaction_during_style_transition_starts_from_the_visible_value() {
    let mut runtime = MotionRuntime::new();
    let mut context = Context::light();
    let mut button = Button::new("Save");
    let new_hover = Color::new_rgba(1, 2, 3, 255);

    {
        let mut update = UpdateCx::new(&mut runtime, &mut context);
        button.register(&mut update);
        button
            .update(ButtonSignal::HoverEnter, &mut update)
            .unwrap();
    }
    runtime.tick(Duration::from_millis(200.0));
    {
        let mut update = UpdateCx::new(&mut runtime, &mut context);
        update.patch_theme(|theme| {
            theme.button.standard.hover.bg = new_hover;
        });
    }
    let visible = button
        .snapshot(&ViewCx::new(&runtime, &context))
        .unwrap()
        .motion;
    {
        let mut update = UpdateCx::new(&mut runtime, &mut context);
        button.update(ButtonSignal::PressDown, &mut update).unwrap();
    }

    assert_eq!(button.motion_value(&runtime).unwrap().unwrap(), visible);
}

#[test]
fn reduced_motion_finishes_runtime_animation_immediately() {
    let mut runtime = MotionRuntime::new();
    let mut context = Context::light();
    let mut button = Button::new("Save");

    {
        let mut update = UpdateCx::new(&mut runtime, &mut context);
        update.set_reduce_motion(true);
        button.register(&mut update);
        button
            .update(ButtonSignal::HoverEnter, &mut update)
            .unwrap();
    }

    let motion = button
        .motion_value(&runtime)
        .unwrap()
        .expect("interaction registers button motion");

    assert_eq!(motion.tokens.bg, "#0000061f".parse::<Color>().unwrap());
}

#[test]
fn suggested_variant_resolves_suggested_tokens() {
    let runtime = MotionRuntime::new();
    let context = Context::light();
    let button = Button::suggested("Apply");
    let view = ViewCx::new(&runtime, &context);

    let snapshot = button.snapshot(&view).unwrap();

    assert_eq!(button.content().as_text(), Some("Apply"));
    assert_eq!(
        snapshot.style.background,
        "#3584e4".parse::<Color>().unwrap()
    );
}

#[test]
fn suggested_flat_variant_keeps_suggested_foreground() {
    let runtime = MotionRuntime::new();
    let context = Context::light();
    let button = Button::suggested("Apply").flat();
    let view = ViewCx::new(&runtime, &context);

    let snapshot = button.snapshot(&view).unwrap();

    assert_eq!(
        snapshot.style.background,
        "#00000000".parse::<Color>().unwrap()
    );
    assert_eq!(
        snapshot.style.foreground,
        context.theme().pack().accent.color
    );
}

#[test]
fn rounded_shape_uses_profile_layout_tokens() {
    let context = Context::light();
    let button = Button::new("Save");
    let theme = context.theme().pack();

    let (width, height, padding_x, padding_y) = button.resolved_layout(theme);
    let expected_height = theme.button.min_height.value() + theme.button.padding_y.value() * 2.0;

    assert_eq!(width, None);
    assert_eq!(height, Some(Length::Fixed(expected_height)));
    assert_approx_eq!(f32, padding_x, theme.button.padding_x.value());
    assert_approx_eq!(f32, padding_y, theme.button.padding_y.value());
}

#[test]
fn circular_shape_uses_profile_button_size() {
    let context = Context::light();
    let button = Button::new("i").circular();
    let theme = context.theme().pack();

    let (width, height, padding_x, padding_y) = button.resolved_layout(theme);
    let size = theme.button.shape.circular.size.value();

    assert_eq!(width, Some(Length::Fixed(size)));
    assert_eq!(height, Some(Length::Fixed(size)));
    assert_approx_eq!(f32, padding_x, 0.0);
    assert_approx_eq!(f32, padding_y, 0.0);
}

#[test]
fn plain_content_layout_uses_profile_base_button_padding() {
    let context = Context::light();
    let button = Button::empty();
    let theme = context.theme().pack();

    let (width, height, padding_x, padding_y) = button.resolved_layout(theme);
    let expected_height = theme.button.min_height.value() + theme.button.padding_y.value() * 2.0;

    assert_eq!(width, None);
    assert_eq!(height, Some(Length::Fixed(expected_height)));
    assert_approx_eq!(f32, padding_x, theme.button.base_padding_x.value());
    assert_approx_eq!(f32, padding_y, theme.button.padding_y.value());
}

#[test]
fn image_text_layout_uses_profile_image_text_padding() {
    let context = Context::light();
    let button = Button::empty().with_content_layout(ButtonContentLayout::ImageText);
    let theme = context.theme().pack();

    let (width, height, padding_x, padding_y) = button.resolved_layout(theme);
    let expected_height = theme.button.min_height.value() + theme.button.padding_y.value() * 2.0;

    assert_eq!(width, None);
    assert_eq!(height, Some(Length::Fixed(expected_height)));
    assert_approx_eq!(f32, padding_x, theme.button.image_text_padding_x.value());
    assert_approx_eq!(f32, padding_y, theme.button.padding_y.value());
}

#[test]
fn style_override_wins_over_theme_variant() {
    let runtime = MotionRuntime::new();
    let context = Context::light();
    let button = Button::destructive("Custom")
        .background(Color::new(1, 2, 3))
        .foreground(Color::new(4, 5, 6));
    let view = ViewCx::new(&runtime, &context);

    let snapshot = button.snapshot(&view).unwrap();

    assert_eq!(snapshot.style.background, Color::new(1, 2, 3));
    assert_eq!(snapshot.style.foreground, Color::new(4, 5, 6));
}
