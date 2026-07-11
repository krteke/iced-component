use iced_component_core::anim::MotionRuntime;

use super::{ColorScheme, Context, ThemeFamily, UpdateCx};

#[test]
fn family_switch_preserves_both_concrete_contexts() {
    let mut runtime = MotionRuntime::new();
    let mut context = Context::default();
    let adwaita_revision = context.adwaita().style_revision();
    let material_revision = context.material().style_revision();

    let changed = UpdateCx::new(&mut runtime, &mut context).set_family(ThemeFamily::Material);

    assert!(changed);
    assert_eq!(context.family(), ThemeFamily::Material);
    assert_eq!(context.adwaita().style_revision(), adwaita_revision);
    assert_eq!(context.material().style_revision(), material_revision);
    assert_eq!(runtime.motion_count(), 0);
}

#[test]
fn common_settings_are_applied_to_both_backends() {
    let mut runtime = MotionRuntime::new();
    let mut context = Context::default();
    let mut cx = UpdateCx::new(&mut runtime, &mut context);

    cx.set_reduce_motion(true);
    let scheme = cx.toggle_color_scheme();

    assert_eq!(scheme, ColorScheme::Dark);
    assert!(context.adwaita().reduce_motion());
    assert!(context.material().reduce_motion());
    assert_eq!(context.color_scheme(), ColorScheme::Dark);
}

#[test]
fn active_values_follow_independently_configured_backends() {
    let mut runtime = MotionRuntime::new();
    let mut adwaita = iced_adwaita::Context::light();
    iced_adwaita::context::UpdateCx::new(&mut runtime, &mut adwaita).set_reduce_motion(true);
    let material = iced_material::context::Context::dark();
    let mut context = Context::from_backends(ThemeFamily::Material, adwaita, material);

    assert_eq!(context.color_scheme(), ColorScheme::Dark);
    assert!(!context.reduce_motion());

    UpdateCx::new(&mut runtime, &mut context).set_family(ThemeFamily::Adwaita);

    assert_eq!(context.color_scheme(), ColorScheme::Light);
    assert!(context.reduce_motion());
}
