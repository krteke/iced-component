//! Animated themed surface component.

mod motion;
mod view;

use aura_anim_core::{MotionError, MotionRuntime, timing::Timing};
use iced::Length;

use crate::{
    component::{ComponentContext, ComponentMotion},
    theme::{SurfaceRole, SurfaceStyleTokens},
};

pub use motion::SurfaceMotion;
pub use view::{SurfaceView, surface_style};

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
    motion: ComponentMotion<SurfaceMotion>,
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
            motion: ComponentMotion::new(SurfaceMotion::for_role(role, false), Timing::default()),
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

    /// Returns this surface with explicit inner padding.
    #[must_use]
    pub const fn padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    /// Returns this surface with a fixed rendered width.
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    /// Returns this surface with a fixed rendered height.
    #[must_use]
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = Some(height.into());
        self
    }

    /// Registers the surface motion handle in the application runtime.
    pub fn register(&mut self, runtime: &mut MotionRuntime, context: &ComponentContext) {
        if self.motion.is_registered() {
            return;
        }

        let motion_tokens = context.motion_tokens();
        let timing = motion_tokens.timing(motion_tokens.interaction, context.motion_preferences());
        self.motion = ComponentMotion::new(self.target_motion(), timing);
        let _ = self.motion.register(runtime);
    }

    /// Applies a surface interaction and transitions motion when registered.
    pub fn update(
        &mut self,
        interaction: SurfaceInteraction,
        runtime: &mut MotionRuntime,
    ) -> Result<bool, MotionError> {
        match interaction {
            SurfaceInteraction::HoverEnter => self.hovered = true,
            SurfaceInteraction::HoverExit => self.hovered = false,
        }

        self.motion.transition_to(self.target_motion(), runtime)
    }

    /// Sets the surface role and transitions motion when registered.
    pub fn set_role(
        &mut self,
        role: SurfaceRole,
        runtime: &mut MotionRuntime,
    ) -> Result<bool, MotionError> {
        self.role = role;
        self.motion.transition_to(self.target_motion(), runtime)
    }

    /// Applies a surface event.
    pub fn update_event(
        &mut self,
        event: SurfaceEvent,
        runtime: &mut MotionRuntime,
    ) -> Result<bool, MotionError> {
        match event {
            SurfaceEvent::Interaction(interaction) => self.update(interaction, runtime),
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
    pub fn snapshot(
        &self,
        runtime: &MotionRuntime,
        context: &ComponentContext,
    ) -> Result<SurfaceSnapshot, MotionError> {
        Ok(SurfaceSnapshot {
            role: self.role,
            style: SurfaceStyleTokens::from_component_context(context, self.role),
            motion: self.motion_value(runtime)?,
            hovered: self.hovered,
        })
    }

    /// Returns this surface visual role.
    #[must_use]
    pub const fn role(&self) -> SurfaceRole {
        self.role
    }

    pub(crate) const fn layout(&self) -> view::SurfaceLayout {
        view::SurfaceLayout {
            padding: self.padding,
            width: self.width,
            height: self.height,
        }
    }

    fn target_motion(&self) -> SurfaceMotion {
        SurfaceMotion::for_role(self.role, self.hovered)
    }
}

#[cfg(test)]
mod tests {
    use aura_anim_core::{MotionRuntime, timing::Duration};
    use float_cmp::assert_approx_eq;
    use iced::Element;

    use crate::{
        component::ComponentContext,
        surface::{Surface, SurfaceEvent, SurfaceInteraction, surface_style},
        theme::SurfaceRole,
    };

    #[test]
    fn snapshot_resolves_surface_tokens() {
        let runtime = MotionRuntime::new();
        let context = ComponentContext::current();
        let surface = Surface::raised();

        let snapshot = surface.snapshot(&runtime, &context).unwrap();

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
        let mut surface = Surface::raised();

        let changed = surface
            .update(SurfaceInteraction::HoverEnter, &mut runtime)
            .unwrap();

        assert!(!changed);
        assert_approx_eq!(f32, surface.motion_value(&runtime).unwrap().elevation, 1.15);
    }

    #[test]
    fn registered_hover_animates_elevation() {
        let mut runtime = MotionRuntime::new();
        let context = ComponentContext::current();
        let mut surface = Surface::raised();

        surface.register(&mut runtime, &context);
        surface
            .update_event(
                SurfaceEvent::Interaction(SurfaceInteraction::HoverEnter),
                &mut runtime,
            )
            .unwrap();
        runtime.tick(Duration::from_millis(200.0));

        assert_approx_eq!(f32, surface.motion_value(&runtime).unwrap().elevation, 1.15);
    }

    #[test]
    fn set_role_updates_style_and_motion_target() {
        let mut runtime = MotionRuntime::new();
        let context = ComponentContext::current();
        let mut surface = Surface::regular();

        let changed = surface.set_role(SurfaceRole::Raised, &mut runtime).unwrap();
        let snapshot = surface.snapshot(&runtime, &context).unwrap();

        assert!(!changed);
        assert_eq!(snapshot.role, SurfaceRole::Raised);
        assert_eq!(
            snapshot.style.background,
            context.theme().theme().surface.raised.bg
        );
        assert_approx_eq!(f32, snapshot.motion.elevation, 1.0);
    }

    #[test]
    fn view_builds_iced_element_and_style() {
        #[derive(Clone)]
        enum Message {
            Surface(SurfaceEvent),
        }

        let runtime = MotionRuntime::new();
        let context = ComponentContext::current();
        let surface = Surface::raised().padding(12.0).width(180.0);
        let snapshot = surface.snapshot(&runtime, &context).unwrap();
        let style = surface_style(snapshot);

        assert!(style.shadow.blur_radius > 0.0);

        let view = surface
            .view(&runtime, &context, iced::widget::text("Surface"))
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
