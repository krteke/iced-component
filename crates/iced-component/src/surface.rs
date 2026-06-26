//! Animated themed surface component.

mod motion;
mod view;

use aura_anim::prelude::{MotionError, MotionRuntime, Timing};
use iced::Length;

use crate::{
    component::{ComponentContext, ComponentUpdateCx, ComponentViewCx, MotionSlot},
    motion::reduce_timing,
    theme::{SurfaceRole, SurfaceStyleTokens},
};

pub use motion::SurfaceMotion;
pub use view::{SurfaceLayout, SurfaceView, surface_style};

/// Surface interaction handled by [`Surface`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SurfaceInteraction {
    /// Pointer entered the surface.
    HoverEnter,
    /// Pointer left the surface.
    HoverExit,
}

/// Surface view event emitted by [`SurfaceView`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SurfaceEvent {
    /// Internal interaction that only updates surface state.
    Interaction(SurfaceInteraction),
}

/// Read-only surface state consumed by rendering code.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SurfaceSnapshot {
    /// Surface visual role.
    pub role: SurfaceRole,
    /// Resolved theme style.
    pub style: SurfaceStyleTokens,
    /// Current animated motion values.
    pub motion: SurfaceMotion,
    /// Whether the pointer is over the surface.
    pub hovered: bool,
}

/// Stateful animated surface.
#[derive(Debug)]
pub struct Surface {
    role: SurfaceRole,
    hovered: bool,
    padding: f32,
    width: Option<Length>,
    height: Option<Length>,
    motion: MotionSlot<SurfaceMotion>,
}

impl Surface {
    /// Creates an animated surface for a visual role.
    #[must_use]
    pub fn new(role: SurfaceRole) -> Self {
        Self {
            role,
            hovered: false,
            padding: 0.0,
            width: None,
            height: None,
            motion: MotionSlot::new(SurfaceMotion::for_role(role, false)),
        }
    }

    /// Creates an app background surface.
    #[must_use]
    pub fn background() -> Self {
        Self::new(SurfaceRole::Background)
    }

    /// Creates a regular component surface.
    #[must_use]
    pub fn regular() -> Self {
        Self::new(SurfaceRole::Regular)
    }

    /// Creates a raised component surface.
    #[must_use]
    pub fn raised() -> Self {
        Self::new(SurfaceRole::Raised)
    }

    /// Returns this surface with a different visual role.
    #[must_use]
    pub fn with_role(mut self, role: SurfaceRole) -> Self {
        self.role = role;
        self
    }

    /// Returns this surface with a different stable layout configuration.
    #[must_use]
    pub const fn with_layout(mut self, layout: SurfaceLayout) -> Self {
        self.padding = layout.padding();
        self.width = layout.width();
        self.height = layout.height();
        self
    }

    /// Replaces this surface's stable layout configuration.
    pub fn set_layout(&mut self, layout: SurfaceLayout) {
        self.padding = layout.padding();
        self.width = layout.width();
        self.height = layout.height();
    }

    /// Returns this surface with explicit inner padding.
    #[must_use]
    pub const fn with_padding(self, padding: f32) -> Self {
        self.padding(padding)
    }

    /// Returns this surface with explicit inner padding.
    #[must_use]
    pub const fn padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    /// Returns this surface with a fixed rendered width.
    #[must_use]
    pub fn with_width(self, width: impl Into<Length>) -> Self {
        self.width(width)
    }

    /// Returns this surface with a fixed rendered width.
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    /// Returns this surface with a fixed rendered height.
    #[must_use]
    pub fn with_height(self, height: impl Into<Length>) -> Self {
        self.height(height)
    }

    /// Returns this surface with a fixed rendered height.
    #[must_use]
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = Some(height.into());
        self
    }

    /// Updates this surface's inner padding.
    pub fn set_padding(&mut self, padding: f32) {
        self.padding = padding;
    }

    /// Updates this surface's fixed rendered width.
    pub fn set_width(&mut self, width: impl Into<Length>) {
        self.width = Some(width.into());
    }

    /// Clears this surface's fixed rendered width.
    pub fn clear_width(&mut self) {
        self.width = None;
    }

    /// Updates this surface's fixed rendered height.
    pub fn set_height(&mut self, height: impl Into<Length>) {
        self.height = Some(height.into());
    }

    /// Clears this surface's fixed rendered height.
    pub fn clear_height(&mut self) {
        self.height = None;
    }

    /// Registers the surface motion handle in the application runtime.
    pub fn register(&mut self, runtime: &mut MotionRuntime) {
        if self.motion.is_registered() {
            return;
        }

        self.motion.set_initial(self.target_motion());
        let _ = self.motion.register(runtime);
    }

    /// Applies a surface interaction and transitions motion when registered.
    pub fn update(
        &mut self,
        interaction: SurfaceInteraction,
        cx: &mut ComponentUpdateCx<'_>,
    ) -> Result<bool, MotionError> {
        match interaction {
            SurfaceInteraction::HoverEnter => self.hovered = true,
            SurfaceInteraction::HoverExit => self.hovered = false,
        }

        let timing = interaction_timing(cx.context());
        self.motion
            .tween_to(self.target_motion(), timing, cx.runtime)
    }

    /// Sets the surface role and transitions motion when registered.
    pub fn set_role(
        &mut self,
        role: SurfaceRole,
        cx: &mut ComponentUpdateCx<'_>,
    ) -> Result<bool, MotionError> {
        self.role = role;
        let timing = interaction_timing(cx.context());
        self.motion
            .tween_to(self.target_motion(), timing, cx.runtime)
    }

    /// Applies a surface event.
    pub fn update_event(
        &mut self,
        event: SurfaceEvent,
        cx: &mut ComponentUpdateCx<'_>,
    ) -> Result<bool, MotionError> {
        match event {
            SurfaceEvent::Interaction(interaction) => self.update(interaction, cx),
        }
    }

    /// Returns the current runtime motion value, or the target value before registration.
    pub fn motion_value(&self, runtime: &MotionRuntime) -> Result<SurfaceMotion, MotionError> {
        Ok(self
            .motion
            .value(runtime)?
            .copied()
            .unwrap_or_else(|| self.target_motion()))
    }

    /// Returns a rendering snapshot without exposing internal state.
    pub fn snapshot(&self, cx: &ComponentViewCx<'_>) -> Result<SurfaceSnapshot, MotionError> {
        Ok(SurfaceSnapshot {
            role: self.role,
            style: SurfaceStyleTokens::from_component_context(cx.context(), self.role),
            motion: self.motion_value(cx.runtime)?,
            hovered: self.hovered,
        })
    }

    /// Returns this surface visual role.
    #[must_use]
    pub const fn role(&self) -> SurfaceRole {
        self.role
    }

    /// Returns whether the pointer is over this surface.
    #[must_use]
    pub const fn is_hovered(&self) -> bool {
        self.hovered
    }

    /// Returns this surface's stable layout configuration.
    #[must_use]
    pub const fn layout(&self) -> SurfaceLayout {
        SurfaceLayout {
            padding: self.padding,
            width: self.width,
            height: self.height,
        }
    }

    fn target_motion(&self) -> SurfaceMotion {
        SurfaceMotion::for_role(self.role, self.hovered)
    }
}

fn interaction_timing(context: &ComponentContext) -> Timing {
    reduce_timing(Timing::ease_out(200.0), context.reduce_motion())
}

#[cfg(test)]
mod tests {
    use aura_anim::prelude::*;
    use float_cmp::assert_approx_eq;
    use iced::Element;

    use crate::{
        component::{ComponentContext, ComponentUpdateCx, ComponentViewCx},
        surface::{Surface, SurfaceEvent, SurfaceInteraction, SurfaceLayout, surface_style},
        theme::SurfaceRole,
    };

    #[test]
    fn snapshot_resolves_surface_tokens() {
        let runtime = MotionRuntime::new();
        let context = ComponentContext::adwaita();
        let cx = ComponentViewCx::new(&runtime, &context);
        let surface = Surface::raised();

        let snapshot = surface.snapshot(&cx).unwrap();

        assert_eq!(snapshot.role, SurfaceRole::Raised);
        assert_eq!(
            snapshot.style.background,
            context.theme().theme().surface.raised.bg
        );
        assert_approx_eq!(f32, snapshot.motion.elevation, 1.0);
    }

    #[test]
    fn hover_updates_target_before_registration() {
        let mut runtime = MotionRuntime::new();
        let mut context = ComponentContext::adwaita();
        let mut surface = Surface::raised();

        let changed = {
            let mut cx = ComponentUpdateCx::new(&mut runtime, &mut context);
            surface
                .update(SurfaceInteraction::HoverEnter, &mut cx)
                .unwrap()
        };

        assert!(!changed);
        assert_approx_eq!(f32, surface.motion_value(&runtime).unwrap().elevation, 1.15);
    }

    #[test]
    fn registered_hover_animates_elevation() {
        let mut runtime = MotionRuntime::new();
        let mut context = ComponentContext::adwaita();
        let mut surface = Surface::raised();

        surface.register(&mut runtime);
        {
            let mut cx = ComponentUpdateCx::new(&mut runtime, &mut context);
            surface
                .update_event(
                    SurfaceEvent::Interaction(SurfaceInteraction::HoverEnter),
                    &mut cx,
                )
                .unwrap();
        }
        runtime.tick(Duration::from_millis(200.0));

        assert_approx_eq!(f32, surface.motion_value(&runtime).unwrap().elevation, 1.15);
    }

    #[test]
    fn set_role_updates_style_and_motion_target() {
        let mut runtime = MotionRuntime::new();
        let mut context = ComponentContext::adwaita();
        let mut surface = Surface::regular();

        let changed = {
            let mut cx = ComponentUpdateCx::new(&mut runtime, &mut context);
            surface.set_role(SurfaceRole::Raised, &mut cx).unwrap()
        };
        let cx = ComponentViewCx::new(&runtime, &context);
        let snapshot = surface.snapshot(&cx).unwrap();

        assert!(!changed);
        assert_eq!(snapshot.role, SurfaceRole::Raised);
        assert_eq!(
            snapshot.style.background,
            context.theme().theme().surface.raised.bg
        );
        assert_approx_eq!(f32, snapshot.motion.elevation, 1.0);
    }

    #[test]
    fn layout_builders_and_setters_update_stable_config() {
        let mut surface = Surface::raised()
            .with_padding(10.0)
            .with_width(200.0)
            .with_height(96.0);

        assert_approx_eq!(f32, surface.layout().padding(), 10.0);
        assert_eq!(surface.layout().width(), Some(iced::Length::Fixed(200.0)));
        assert_eq!(surface.layout().height(), Some(iced::Length::Fixed(96.0)));

        surface.set_padding(14.0);
        surface.set_width(240.0);
        surface.clear_height();

        assert_approx_eq!(f32, surface.layout().padding(), 14.0);
        assert_eq!(surface.layout().width(), Some(iced::Length::Fixed(240.0)));
        assert_eq!(surface.layout().height(), None);

        surface.set_layout(SurfaceLayout::new(
            8.0,
            None,
            Some(iced::Length::Fixed(72.0)),
        ));

        assert_approx_eq!(f32, surface.layout().padding(), 8.0);
        assert_eq!(surface.layout().width(), None);
        assert_eq!(surface.layout().height(), Some(iced::Length::Fixed(72.0)));
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
}
