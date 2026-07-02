use aura_anim::prelude::*;
use float_cmp::assert_approx_eq;
use iced::Element;
use spectrum_theme::{Color, Length as SpectrumLength, LengthUnit};

use crate::{
    component::{ComponentContext, ComponentUpdateCx, ComponentViewCx},
    surface::{
        Surface, SurfaceEvent, SurfaceInteraction, SurfaceLayout, SurfaceStyleState,
        SurfaceTreatment, SurfaceVariant, surface_style,
    },
};

#[test]
fn snapshot_resolves_surface_tokens() {
    let runtime = MotionRuntime::new();
    let context = ComponentContext::adwaita();
    let cx = ComponentViewCx::new(&runtime, &context);
    let surface = Surface::raised();

    let snapshot = surface.snapshot(&cx).unwrap();

    assert_eq!(snapshot.variant, SurfaceVariant::RAISED);
    assert_eq!(
        snapshot.motion.tokens.bg,
        context.theme().theme().surface.raised.idle.bg
    );
    assert_eq!(snapshot.style_state, SurfaceStyleState::Idle);
    assert_approx_eq!(f32, snapshot.motion.elevation, 1.0);
}

#[test]
fn snapshot_ignores_stale_runtime_motion_after_theme_change() {
    let mut runtime = MotionRuntime::new();
    let mut context = ComponentContext::adwaita();
    let mut surface = Surface::raised();

    {
        let mut cx = ComponentUpdateCx::new(&mut runtime, &mut context);
        surface
            .update(SurfaceInteraction::HoverEnter, &mut cx)
            .unwrap();
    }
    runtime.tick(Duration::from_millis(1.0));
    let stale_motion = surface.motion_value(&runtime).unwrap().unwrap();
    let scoped_bg = "#ddeeff".parse().unwrap();

    context.patch_theme(|theme| theme.surface.raised.hover.bg = scoped_bg);

    let cx = ComponentViewCx::new(&runtime, &context);
    let snapshot = surface.snapshot(&cx).unwrap();

    assert_ne!(stale_motion.tokens.bg, scoped_bg);
    assert_eq!(snapshot.motion.tokens.bg, scoped_bg);
    assert_eq!(snapshot.motion.tokens.bg, scoped_bg);
}

#[test]
fn first_hover_registers_runtime_motion() {
    let mut runtime = MotionRuntime::new();
    let mut context = ComponentContext::adwaita();
    let mut surface = Surface::raised();

    let changed = {
        let mut cx = ComponentUpdateCx::new(&mut runtime, &mut context);
        surface
            .update(SurfaceInteraction::HoverEnter, &mut cx)
            .unwrap()
    };
    runtime.tick(Duration::from_millis(200.0));

    let motion = surface.motion_value(&runtime).unwrap().unwrap();
    assert!(changed);
    assert_eq!(runtime.motion_count(), 1);
    assert_color_eq(
        motion.tokens.bg,
        context.theme().theme().surface.raised.hover.bg,
    );
    assert_approx_eq!(f32, motion.elevation, 1.0);
}

#[test]
fn registered_hover_animates_surface_tokens_without_shadow_boost() {
    let mut runtime = MotionRuntime::new();
    let mut context = ComponentContext::adwaita();
    let mut surface = Surface::raised();

    {
        let mut cx = ComponentUpdateCx::new(&mut runtime, &mut context);
        surface.register(&mut cx);
        surface
            .update_event(
                SurfaceEvent::Interaction(SurfaceInteraction::HoverEnter),
                &mut cx,
            )
            .unwrap();
    }
    runtime.tick(Duration::from_millis(200.0));

    let theme = context.theme().theme();
    let motion = surface.motion_value(&runtime).unwrap().unwrap();
    assert_color_eq(motion.tokens.bg, theme.surface.raised.hover.bg);
    assert_eq!(motion.tokens.shadow, theme.surface.raised.idle.shadow);
    assert_approx_eq!(f32, motion.elevation, 1.0);
}

#[test]
fn set_variant_updates_style_and_motion_target() {
    let mut runtime = MotionRuntime::new();
    let mut context = ComponentContext::adwaita();
    let mut surface = Surface::regular();

    let changed = {
        let mut cx = ComponentUpdateCx::new(&mut runtime, &mut context);
        surface
            .set_variant(SurfaceVariant::RAISED, &mut cx)
            .unwrap()
    };
    runtime.tick(Duration::from_millis(200.0));
    let cx = ComponentViewCx::new(&runtime, &context);
    let snapshot = surface.snapshot(&cx).unwrap();

    assert!(changed);
    assert_eq!(snapshot.variant, SurfaceVariant::RAISED);
    assert_color_eq(
        snapshot.motion.tokens.bg,
        context.theme().theme().surface.raised.idle.bg,
    );
    assert_approx_eq!(f32, snapshot.motion.elevation, 1.0);
}

#[test]
fn set_role_and_treatment_update_variant() {
    let mut runtime = MotionRuntime::new();
    let mut context = ComponentContext::adwaita();
    let mut surface = Surface::regular();

    {
        let mut cx = ComponentUpdateCx::new(&mut runtime, &mut context);
        surface.set_elevated(&mut cx).unwrap();
        surface.set_background(&mut cx).unwrap();
    }

    assert_eq!(surface.variant(), SurfaceVariant::BACKGROUND);
    assert_eq!(surface.treatment(), SurfaceTreatment::Plain);
}

#[test]
fn set_hovered_updates_state_and_motion_target() {
    let mut runtime = MotionRuntime::new();
    let mut context = ComponentContext::adwaita();
    let mut surface = Surface::raised();

    {
        let mut cx = ComponentUpdateCx::new(&mut runtime, &mut context);
        surface.set_hovered(true, &mut cx).unwrap();
    }
    runtime.tick(Duration::from_millis(200.0));

    let cx = ComponentViewCx::new(&runtime, &context);
    let snapshot = surface.snapshot(&cx).unwrap();

    assert!(surface.is_hovered());
    assert_eq!(surface.style_state(), SurfaceStyleState::Hovered);
    assert_eq!(snapshot.style_state, SurfaceStyleState::Hovered);
    assert_color_eq(
        snapshot.motion.tokens.bg,
        context.theme().theme().surface.raised.hover.bg,
    );
}

#[test]
fn layout_builders_and_setters_update_stable_config() {
    let mut surface = Surface::raised()
        .with_padding(10.0)
        .with_width(200.0)
        .with_height(96.0);

    assert_eq!(surface.layout().padding(), Some(10.0));
    assert_eq!(surface.layout().width(), Some(iced::Length::Fixed(200.0)));
    assert_eq!(surface.layout().height(), Some(iced::Length::Fixed(96.0)));

    surface.set_padding(14.0);
    surface.set_width(240.0);
    surface.clear_height();
    surface.clear_padding();

    assert_eq!(surface.layout().padding(), None);
    assert_eq!(surface.layout().width(), Some(iced::Length::Fixed(240.0)));
    assert_eq!(surface.layout().height(), None);

    surface.set_layout(SurfaceLayout::new(
        Some(8.0),
        None,
        Some(iced::Length::Fixed(72.0)),
    ));

    assert_eq!(surface.layout().padding(), Some(8.0));
    assert_eq!(surface.layout().width(), None);
    assert_eq!(surface.layout().height(), Some(iced::Length::Fixed(72.0)));
}

#[test]
fn layout_resolves_default_padding_from_theme() {
    let mut context = ComponentContext::adwaita();
    context.patch_theme(|theme| {
        theme.control.surface.padding = SpectrumLength::new(6.0, LengthUnit::Px).unwrap();
    });

    let layout = Surface::regular().layout().resolve(&context);
    assert_approx_eq!(f32, layout.padding, 6.0);
}

#[test]
fn view_builds_iced_element_and_style() {
    #[derive(Clone)]
    enum Message {
        Surface(SurfaceEvent),
    }

    let runtime = MotionRuntime::new();
    let context = ComponentContext::adwaita();
    let cx = ComponentViewCx::new(&runtime, &context);
    let surface = Surface::raised().with_padding(12.0).with_width(180.0);
    let snapshot = surface.snapshot(&cx).unwrap();
    let style = surface_style(snapshot);

    assert!(style.shadow.blur_radius > 0.0);
    assert_approx_eq!(f32, style.border.width, 1.0);
    assert!(style.border.radius.top_left > 0.0);

    let view = surface
        .view(&cx, iced::widget::text("Surface"))
        .connect(Message::Surface);
    let _element: Element<'_, Message> = view.into();

    let Message::Surface(event) =
        Message::Surface(SurfaceEvent::Interaction(SurfaceInteraction::HoverEnter));
    assert_eq!(
        event,
        SurfaceEvent::Interaction(SurfaceInteraction::HoverEnter)
    );
}

#[test]
fn surface_style_uses_component_border_width_tokens() {
    let runtime = MotionRuntime::new();
    let mut context = ComponentContext::adwaita();

    context.patch_theme(|theme| {
        theme.surface.raised.idle.border_width = SpectrumLength::new(2.0, LengthUnit::Px).unwrap();
    });

    let cx = ComponentViewCx::new(&runtime, &context);
    let raised = Surface::raised().snapshot(&cx).unwrap();
    let background = Surface::background().snapshot(&cx).unwrap();

    assert_approx_eq!(f32, surface_style(raised).border.width, 2.0);
    assert_approx_eq!(f32, surface_style(background).border.width, 0.0);
}

fn assert_color_eq(left: Color, right: Color) {
    assert_eq!(left.red(), right.red());
    assert_eq!(left.green(), right.green());
    assert_eq!(left.blue(), right.blue());
    assert_eq!(left.alpha(), right.alpha());
}
