use aura_anim::prelude::{Duration as MotionDuration, Timing};
use iced::time::{Duration, Instant};
use iced_component_core::anim::MotionRuntime;
use spectrum_theme::Color;

use crate::{
    context::{Context, ThemeMode, UpdateCx, ViewCx},
    loading_indicator::{
        LoadingIndicator, LoadingIndicatorAnimations, LoadingIndicatorMode, LoadingIndicatorStyle,
    },
};

#[test]
fn defaults_resolve_from_the_material_theme_pack() {
    let runtime = MotionRuntime::new();
    let context = Context::light();
    let cx = ViewCx::new(&runtime, &context);
    let snapshot = LoadingIndicator::new().snapshot(&cx).unwrap();

    assert_close(snapshot.visual.size, 48.0);
    assert_eq!(
        snapshot.visual.active,
        context.theme().pack().palette.primary.color
    );
}

#[test]
fn instance_configuration_overrides_theme_visuals() {
    let runtime = MotionRuntime::new();
    let context = Context::light();
    let cx = ViewCx::new(&runtime, &context);
    let active = Color::new(1, 2, 3);
    let indicator = LoadingIndicator::determinate(0.4)
        .contained()
        .size(72.0)
        .with_style(LoadingIndicatorStyle {
            active: Some(active),
            ..LoadingIndicatorStyle::new()
        });
    let snapshot = indicator.snapshot(&cx).unwrap();

    assert_eq!(snapshot.mode, LoadingIndicatorMode::Determinate(0.4));
    assert!(snapshot.contained);
    assert_close(snapshot.visual.size, 72.0);
    assert_eq!(snapshot.visual.active, active);
}

#[test]
fn mutable_instance_overrides_can_be_cleared() {
    let mut indicator = LoadingIndicator::new();
    let color = Color::new(1, 2, 3);

    indicator.set_size(64.0);
    indicator.set_active_color(color);
    indicator.set_container_color(color);
    indicator.set_contained_active_color(color);
    assert_eq!(indicator.explicit_size(), Some(64.0));
    assert_eq!(indicator.active_color_override(), Some(color));
    assert_eq!(indicator.container_color_override(), Some(color));
    assert_eq!(indicator.contained_active_color_override(), Some(color));

    indicator.clear_size();
    indicator.clear_style();
    assert_eq!(indicator.explicit_size(), None);
    assert_eq!(indicator.style(), LoadingIndicatorStyle::new());
}

#[test]
fn instance_layout_and_color_overrides_apply_without_motion_sync() {
    let mut runtime = MotionRuntime::new();
    let mut context = Context::light();
    let mut indicator = LoadingIndicator::new();
    let active = Color::new(1, 2, 3);

    indicator.register(&mut UpdateCx::new(&mut runtime, &mut context));
    indicator.set_size(64.0);
    indicator.set_active_color(active);

    let snapshot = indicator
        .snapshot(&ViewCx::new(&runtime, &context))
        .unwrap();
    assert_close(snapshot.visual.size, 64.0);
    assert_eq!(snapshot.visual.active, active);

    indicator.clear_active_color();
    assert_eq!(
        indicator
            .snapshot(&ViewCx::new(&runtime, &context))
            .unwrap()
            .visual
            .active,
        context.theme().pack().loading_indicator.active
    );
}

#[test]
fn registered_theme_sync_animates_resolved_colors() {
    let mut runtime = MotionRuntime::new();
    let mut context = Context::light();
    let mut indicator = LoadingIndicator::new();
    let from = context.theme().pack().loading_indicator.active;
    let to = context
        .theme()
        .pack_for(ThemeMode::Dark)
        .loading_indicator
        .active;

    {
        let mut cx = UpdateCx::new(&mut runtime, &mut context);
        indicator.register(&mut cx);
        cx.toggle_theme();
        assert!(indicator.sync(&mut cx).unwrap());
    }

    assert_eq!(
        indicator
            .motion_value(&runtime)
            .unwrap()
            .unwrap()
            .active()
            .rgba(),
        from.rgba()
    );
    runtime.tick(MotionDuration::from_millis(100.0));
    let midpoint = indicator
        .snapshot(&ViewCx::new(&runtime, &context))
        .unwrap()
        .visual
        .active;
    assert_ne!(midpoint.rgba(), from.rgba());
    assert_ne!(midpoint.rgba(), to.rgba());

    runtime.tick(MotionDuration::from_millis(100.0));
    assert_eq!(
        indicator
            .snapshot(&ViewCx::new(&runtime, &context))
            .unwrap()
            .visual
            .active
            .rgba(),
        to.rgba()
    );
}

#[test]
fn animation_override_controls_theme_sync_duration() {
    let mut runtime = MotionRuntime::new();
    let mut context = Context::light();
    let mut indicator = LoadingIndicator::new();
    let target = context
        .theme()
        .pack_for(ThemeMode::Dark)
        .loading_indicator
        .active;

    {
        let mut cx = UpdateCx::new(&mut runtime, &mut context);
        cx.set_animation_override(LoadingIndicatorAnimations::new(Timing::linear(400.0)));
        indicator.register(&mut cx);
        cx.toggle_theme();
        indicator.sync(&mut cx).unwrap();
    }

    runtime.tick(MotionDuration::from_millis(200.0));
    assert_ne!(
        indicator
            .snapshot(&ViewCx::new(&runtime, &context))
            .unwrap()
            .visual
            .active
            .rgba(),
        target.rgba()
    );

    runtime.tick(MotionDuration::from_millis(200.0));
    assert_eq!(
        indicator
            .snapshot(&ViewCx::new(&runtime, &context))
            .unwrap()
            .visual
            .active
            .rgba(),
        target.rgba()
    );
}

#[test]
fn unregistered_theme_change_resolves_final_color_without_motion() {
    let mut runtime = MotionRuntime::new();
    let mut context = Context::light();
    let mut indicator = LoadingIndicator::new();

    let mut cx = UpdateCx::new(&mut runtime, &mut context);
    cx.toggle_theme();
    assert!(!indicator.sync(&mut cx).unwrap());

    let expected = context.theme().pack().loading_indicator.active;
    assert_eq!(
        indicator
            .snapshot(&ViewCx::new(&runtime, &context))
            .unwrap()
            .visual
            .active,
        expected
    );
    assert_eq!(runtime.motion_count(), 0);
}

#[test]
fn reduced_motion_finishes_only_the_theme_transition() {
    let mut runtime = MotionRuntime::new();
    let mut context = Context::light();
    let mut indicator = LoadingIndicator::new();

    {
        let mut cx = UpdateCx::new(&mut runtime, &mut context);
        indicator.register(&mut cx);
        cx.set_reduce_motion(true);
        cx.toggle_theme();
        assert!(indicator.sync(&mut cx).unwrap());
    }

    let expected = context.theme().pack().loading_indicator.active;
    assert_eq!(
        indicator
            .motion_value(&runtime)
            .unwrap()
            .unwrap()
            .active()
            .rgba(),
        expected.rgba()
    );
}

#[test]
fn reduced_motion_does_not_disable_essential_loading_feedback() {
    let mut runtime = MotionRuntime::new();
    let mut context = Context::light();
    UpdateCx::new(&mut runtime, &mut context).set_reduce_motion(true);
    let start = Instant::now();
    let mut indicator = LoadingIndicator::new();

    indicator.advance(start);
    indicator.advance(start + Duration::from_millis(2_333));

    assert!((indicator.phase() - 0.5).abs() < 0.001);
}

#[test]
fn all_modes_build_iced_views() {
    let runtime = MotionRuntime::new();
    let context = Context::dark();
    let cx = ViewCx::new(&runtime, &context);

    let _: iced::Element<'static, ()> = LoadingIndicator::new().view(&cx);
    let _: iced::Element<'static, ()> = LoadingIndicator::new().contained().view(&cx);
    let _: iced::Element<'static, ()> = LoadingIndicator::determinate(0.5).view(&cx);
    let _: iced::Element<'static, ()> = LoadingIndicator::determinate(0.5).contained().view(&cx);
    let _: iced::Element<'static, ()> = LoadingIndicator::new().try_view(&cx).unwrap();
}

fn assert_close(actual: f32, expected: f32) {
    assert!((actual - expected).abs() < 0.001);
}
