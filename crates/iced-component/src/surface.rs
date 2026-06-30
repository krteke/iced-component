//! Animated themed surface component.

mod motion;
mod view;

use aura_anim::prelude::{MotionError, MotionRuntime, Timing};
use iced::Length;

use crate::{
    component::{ComponentContext, ComponentUpdateCx, ComponentViewCx, MotionSlot},
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
            motion: MotionSlot::new(),
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

    /// Registers the surface motion handle using the current component context.
    pub fn register(&mut self, cx: &mut ComponentUpdateCx<'_>) {
        if self.motion.is_registered() {
            return;
        }

        let _ = self.motion.register(
            cx.runtime,
            self.target_motion(),
            cx.context().theme_revision(),
        );
    }

    /// Synchronizes this surface's current motion target with the runtime.
    pub fn sync(&mut self, cx: &mut ComponentUpdateCx<'_>) -> Result<bool, MotionError> {
        self.motion
            .tween_to_or_finish(self.target_motion(), interaction_timing(), cx)
    }

    /// Applies a surface interaction and transitions motion when registered.
    pub fn update(
        &mut self,
        interaction: SurfaceInteraction,
        cx: &mut ComponentUpdateCx<'_>,
    ) -> Result<bool, MotionError> {
        let initial = self.current_or_target_motion(cx.runtime, cx.context())?;
        match interaction {
            SurfaceInteraction::HoverEnter => self.hovered = true,
            SurfaceInteraction::HoverExit => self.hovered = false,
        }

        self.animate_from(initial, cx)
    }

    /// Sets the surface role and transitions motion when registered.
    pub fn set_role(
        &mut self,
        role: SurfaceRole,
        cx: &mut ComponentUpdateCx<'_>,
    ) -> Result<bool, MotionError> {
        let initial = self.current_or_target_motion(cx.runtime, cx.context())?;
        self.role = role;

        self.animate_from(initial, cx)
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

    /// Returns the raw runtime motion value, or `None` if not registered.
    ///
    /// This does not validate the current theme revision. Rendering code should
    /// use [`snapshot`](Self::snapshot), which falls back to current state when
    /// the runtime value belongs to an older theme.
    pub fn motion_value(
        &self,
        runtime: &MotionRuntime,
    ) -> Result<Option<SurfaceMotion>, MotionError> {
        Ok(self.motion.value(runtime)?.copied())
    }

    /// Returns a rendering snapshot without exposing internal state.
    pub fn snapshot(&self, cx: &ComponentViewCx<'_>) -> Result<SurfaceSnapshot, MotionError> {
        Ok(SurfaceSnapshot {
            role: self.role,
            style: SurfaceStyleTokens::from_component_context(cx.context(), self.role),
            motion: self.current_or_target_motion(cx.runtime, cx.context())?,
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

    fn current_or_target_motion(
        &self,
        runtime: &MotionRuntime,
        context: &ComponentContext,
    ) -> Result<SurfaceMotion, MotionError> {
        Ok(self
            .motion
            .value_if_current(runtime, context.theme_revision())?
            .copied()
            .unwrap_or_else(|| self.target_motion()))
    }

    fn animate_from(
        &mut self,
        initial: SurfaceMotion,
        cx: &mut ComponentUpdateCx<'_>,
    ) -> Result<bool, MotionError> {
        let target = self.target_motion();
        if initial == target && !self.motion.is_registered() {
            return Ok(false);
        }

        self.motion
            .tween_from_to_or_finish(initial, target, interaction_timing(), cx)
    }
}

fn interaction_timing() -> Timing {
    Timing::ease_out(200.0)
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

        context.patch_theme(|theme| theme.surface.raised.bg = scoped_bg);

        let cx = ComponentViewCx::new(&runtime, &context);
        let snapshot = surface.snapshot(&cx).unwrap();

        assert!(stale_motion.elevation < 1.15);
        assert_eq!(snapshot.style.background, scoped_bg);
        assert_approx_eq!(f32, snapshot.motion.elevation, 1.15);
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

        assert!(changed);
        assert_eq!(runtime.motion_count(), 1);
        assert_approx_eq!(
            f32,
            surface.motion_value(&runtime).unwrap().unwrap().elevation,
            1.15
        );
    }

    #[test]
    fn registered_hover_animates_elevation() {
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

        let motion = surface.motion_value(&runtime).unwrap().unwrap();
        assert_approx_eq!(f32, motion.elevation, 1.15);
        assert_approx_eq!(f32, motion.radius_scale, 1.02);
        assert_approx_eq!(f32, motion.shadow_blur, 1.06);
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
        runtime.tick(Duration::from_millis(200.0));
        let cx = ComponentViewCx::new(&runtime, &context);
        let snapshot = surface.snapshot(&cx).unwrap();

        assert!(changed);
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
}
