use aura_anim::prelude::Duration as MotionDuration;
use iced::{
    Element,
    time::{Duration, Instant},
};
use iced_component_core::anim::MotionRuntime;

use super::LoadingIndicator;
use crate::context::{Context, ThemeFamily, UpdateCx, ViewCx};

#[test]
fn common_size_is_applied_to_both_concrete_components() {
    let runtime = MotionRuntime::new();
    let context = Context::default();
    let indicator = LoadingIndicator::new().size(72.0);

    let adwaita_size = indicator
        .adwaita()
        .appearance(&ViewCx::new(&runtime, &context).adwaita())
        .size;
    let material_size = indicator
        .material()
        .snapshot(&ViewCx::new(&runtime, &context).material())
        .unwrap()
        .visual
        .size;

    assert_close(adwaita_size, 72.0);
    assert_close(material_size, 72.0);
}

#[test]
fn common_size_can_be_changed_and_cleared() {
    let mut indicator = LoadingIndicator::new();

    indicator.set_size(64.0);
    assert_eq!(indicator.explicit_size(), Some(64.0));
    assert_eq!(indicator.adwaita().explicit_size(), Some(64.0));
    assert_eq!(indicator.material().explicit_size(), Some(64.0));

    indicator.clear_size();
    assert_eq!(indicator.explicit_size(), None);
    assert_eq!(indicator.adwaita().explicit_size(), None);
    assert_eq!(indicator.material().explicit_size(), None);
}

#[test]
fn registration_and_theme_sync_reach_both_backends() {
    let mut runtime = MotionRuntime::new();
    let mut context = Context::default();
    let mut indicator = LoadingIndicator::new();
    let from = context.material().theme().pack().loading_indicator.active;

    {
        let mut cx = UpdateCx::new(&mut runtime, &mut context);
        indicator.register(&mut cx);
        assert!(indicator.is_registered());
        cx.toggle_color_scheme();
        assert!(indicator.sync(&mut cx).unwrap());
    }

    assert_eq!(
        indicator
            .material()
            .motion_value(&runtime)
            .unwrap()
            .unwrap()
            .active()
            .rgba(),
        from.rgba()
    );

    runtime.tick(MotionDuration::from_millis(200.0));
    assert_eq!(
        indicator
            .material()
            .snapshot(&ViewCx::new(&runtime, &context).material())
            .unwrap()
            .visual
            .active
            .rgba(),
        context
            .material()
            .theme()
            .pack()
            .loading_indicator
            .active
            .rgba()
    );
}

#[test]
fn advance_keeps_both_backend_timelines_live() {
    let start = Instant::now();
    let mut indicator = LoadingIndicator::new();
    indicator.advance(start);
    indicator.advance(start + Duration::from_millis(800));

    assert_ne!(
        indicator.adwaita().sample(),
        iced_adwaita::spinner::Spinner::new().sample()
    );
    assert!(indicator.material().phase() > 0.0);
}

#[test]
fn view_switches_between_concrete_loading_implementations() {
    let mut runtime = MotionRuntime::new();
    let mut context = Context::default();
    let indicator = LoadingIndicator::new();

    let _: Element<'static, ()> = indicator.view(&ViewCx::new(&runtime, &context));
    UpdateCx::new(&mut runtime, &mut context).set_family(ThemeFamily::Material);
    let _: Element<'static, ()> = indicator.view(&ViewCx::new(&runtime, &context));
}

#[test]
fn material_specific_modes_remain_accessible() {
    let mut indicator = LoadingIndicator::new();
    indicator.material_mut().set_contained(true);
    indicator.material_mut().set_progress(0.6);

    assert!(indicator.material().is_contained());
    assert_eq!(
        indicator.material().mode(),
        iced_material::loading_indicator::LoadingIndicatorMode::Determinate(0.6)
    );
}

fn assert_close(actual: f32, expected: f32) {
    assert!((actual - expected).abs() < 0.001);
}
