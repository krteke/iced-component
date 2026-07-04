use aura_anim::prelude::*;
use float_cmp::assert_approx_eq;
use iced::Element;
use spectrum_theme::{Color, Length as SpectrumLength, LengthUnit};

use crate::{
    component::{ComponentContext, ComponentUpdateCx, ComponentViewCx},
    spinner::Spinner,
};

#[test]
fn register_starts_default_spinner() {
    let mut runtime = MotionRuntime::new();
    let mut context = ComponentContext::adwaita();
    let mut spinner = Spinner::new();

    {
        let mut cx = ComponentUpdateCx::new(&mut runtime, &mut context);
        spinner.register(&mut cx);
    }
    runtime.tick(Duration::from_millis(500.0));

    let motion = spinner.motion_value(&runtime).unwrap().unwrap();
    assert_approx_eq!(f32, motion.rotation, 180.0);
    assert!(spinner.is_spinning());
}

#[test]
fn stopped_spinner_waits_for_start() {
    let mut runtime = MotionRuntime::new();
    let mut context = ComponentContext::adwaita();
    let mut spinner = Spinner::stopped();

    {
        let mut cx = ComponentUpdateCx::new(&mut runtime, &mut context);
        spinner.register(&mut cx);
    }
    runtime.tick(Duration::from_millis(500.0));
    assert_approx_eq!(
        f32,
        spinner.motion_value(&runtime).unwrap().unwrap().rotation,
        0.0
    );

    {
        let mut cx = ComponentUpdateCx::new(&mut runtime, &mut context);
        spinner.start(&mut cx).unwrap();
    }
    runtime.tick(Duration::from_millis(500.0));
    assert!(spinner.motion_value(&runtime).unwrap().unwrap().rotation > 0.0);
}

#[test]
fn stop_keeps_current_spinner_angle() {
    let mut runtime = MotionRuntime::new();
    let mut context = ComponentContext::adwaita();
    let mut spinner = Spinner::new();

    {
        let mut cx = ComponentUpdateCx::new(&mut runtime, &mut context);
        spinner.register(&mut cx);
    }
    runtime.tick(Duration::from_millis(500.0));
    let held_rotation = spinner.motion_value(&runtime).unwrap().unwrap().rotation;
    {
        let mut cx = ComponentUpdateCx::new(&mut runtime, &mut context);
        spinner.stop(&mut cx).unwrap();
    }
    runtime.tick(Duration::from_millis(120.0));

    assert_approx_eq!(
        f32,
        spinner.motion_value(&runtime).unwrap().unwrap().rotation,
        held_rotation
    );
    assert!(!spinner.is_spinning());
}

#[test]
fn reduce_motion_does_not_disable_spinner() {
    let mut runtime = MotionRuntime::new();
    let mut context = ComponentContext::adwaita().with_reduce_motion(true);
    let mut spinner = Spinner::new();

    {
        let mut cx = ComponentUpdateCx::new(&mut runtime, &mut context);
        spinner.register(&mut cx);
    }
    runtime.tick(Duration::from_millis(500.0));

    assert_approx_eq!(
        f32,
        spinner.motion_value(&runtime).unwrap().unwrap().rotation,
        180.0
    );
}

#[test]
fn reduce_motion_sync_keeps_spinner_running() {
    let mut runtime = MotionRuntime::new();
    let mut context = ComponentContext::adwaita();
    let mut spinner = Spinner::new();

    {
        let mut cx = ComponentUpdateCx::new(&mut runtime, &mut context);
        spinner.register(&mut cx);
    }
    runtime.tick(Duration::from_millis(500.0));
    let held_rotation = spinner.motion_value(&runtime).unwrap().unwrap().rotation;

    {
        let mut cx = ComponentUpdateCx::new(&mut runtime, &mut context);
        cx.context_mut().set_reduce_motion(true);
        spinner.sync(&mut cx).unwrap();
    }
    runtime.tick(Duration::from_millis(250.0));

    assert!(spinner.motion_value(&runtime).unwrap().unwrap().rotation > held_rotation);
    assert!(spinner.is_spinning());
}

#[test]
fn snapshot_uses_current_theme_tokens() {
    let runtime = MotionRuntime::new();
    let mut context = ComponentContext::adwaita();
    let fg = Color::new(46, 52, 64);
    context.patch_theme(|theme| {
        theme.spinner.regular.fg = fg;
        theme.spinner.regular.size = SpectrumLength::new(32.0, LengthUnit::Px).unwrap();
    });
    let spinner = Spinner::stopped();

    let cx = ComponentViewCx::new(&runtime, &context);
    let snapshot = spinner.snapshot(&cx).unwrap();

    assert_eq!(snapshot.tokens.fg, fg);
    assert_approx_eq!(f32, snapshot.tokens.size.value(), 32.0);
    assert_approx_eq!(f32, snapshot.size, 32.0);
}

#[test]
fn instance_size_and_stroke_width_override_theme_defaults() {
    let runtime = MotionRuntime::new();
    let context = ComponentContext::adwaita();
    let cx = ComponentViewCx::new(&runtime, &context);
    let spinner = Spinner::stopped().with_size(32.0).with_stroke_width(1.8);

    let snapshot = spinner.snapshot(&cx).unwrap();

    assert_approx_eq!(f32, snapshot.size, 32.0);
    assert_approx_eq!(f32, snapshot.stroke_width, 2.4);
    assert_eq!(spinner.size(), Some(32.0));
    assert_eq!(spinner.stroke_width(), Some(1.8));
}

#[test]
fn stroke_width_scales_with_rendered_size() {
    let runtime = MotionRuntime::new();
    let context = ComponentContext::adwaita();
    let cx = ComponentViewCx::new(&runtime, &context);
    let spinner = Spinner::stopped().with_size(64.0);

    let snapshot = spinner.snapshot(&cx).unwrap();

    assert_approx_eq!(f32, snapshot.size, 64.0);
    assert_approx_eq!(f32, snapshot.stroke_width, 2.5 * 64.0 / 24.0);
}

#[test]
fn view_builds_iced_element() {
    #[derive(Clone)]
    struct Message;

    let runtime = MotionRuntime::new();
    let context = ComponentContext::adwaita();
    let cx = ComponentViewCx::new(&runtime, &context);
    let spinner = Spinner::stopped();

    let _element: Element<'_, Message> = spinner.view(&cx).into();
}
