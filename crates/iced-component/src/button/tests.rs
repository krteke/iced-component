use iced::{Element, widget::text};
use iced_component_core::anim::MotionRuntime;

use super::{Button, ButtonEvent, ButtonSignal, ButtonStyle};
use crate::context::{Context, ThemeFamily, UpdateCx, ViewCx};

#[test]
fn style_is_only_used_to_construct_concrete_buttons() {
    let mut button = Button::new(ButtonStyle::Primary);

    assert_eq!(
        button.adwaita().variant(),
        iced_adwaita::button::ButtonVariant::SUGGESTED
    );
    assert_eq!(
        button.material().variant(),
        iced_material::button::ButtonVariant::FILLED
    );

    button
        .adwaita_mut()
        .set_variant(iced_adwaita::button::ButtonVariant::DESTRUCTIVE);
    button
        .material_mut()
        .set_variant(iced_material::button::ButtonVariant::OUTLINED);

    assert_eq!(
        button.adwaita().variant(),
        iced_adwaita::button::ButtonVariant::DESTRUCTIVE
    );
    assert_eq!(
        button.material().variant(),
        iced_material::button::ButtonVariant::OUTLINED
    );
}

#[test]
fn register_prepares_exactly_the_selected_backend_pair() {
    let mut runtime = MotionRuntime::new();
    let mut context = Context::default();
    let mut button = Button::primary();

    button.register(&mut UpdateCx::new(&mut runtime, &mut context));

    assert!(button.is_registered());
    assert_eq!(runtime.motion_count(), 2);
}

#[test]
fn interaction_state_is_kept_in_sync_across_backends() {
    let mut runtime = MotionRuntime::new();
    let mut context = Context::default();
    let mut button = Button::default();
    button.register(&mut UpdateCx::new(&mut runtime, &mut context));

    button
        .update_event(
            ButtonEvent::Signal(ButtonSignal::HoverEnter),
            &mut UpdateCx::new(&mut runtime, &mut context),
        )
        .unwrap();

    assert_eq!(
        button.adwaita().style_state(),
        iced_adwaita::button::ButtonStyleState::Hovered
    );
    assert_eq!(
        button.material().style_state(),
        iced_material::button::ButtonStyleState::Hover
    );
}

#[test]
fn view_builds_for_each_selected_family() {
    #[derive(Clone)]
    struct Message;

    let mut runtime = MotionRuntime::new();
    let mut context = Context::default();
    let button = Button::default();

    let _: Element<'_, Message> = button
        .view(&ViewCx::new(&runtime, &context))
        .content(text("Action"))
        .on_event(|_| Message)
        .into();

    UpdateCx::new(&mut runtime, &mut context).set_family(ThemeFamily::Material);
    let _: Element<'_, Message> = button
        .view(&ViewCx::new(&runtime, &context))
        .content(text("Action"))
        .on_event(|_| Message)
        .into();
}
