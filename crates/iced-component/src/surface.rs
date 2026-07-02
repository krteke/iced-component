//! Animated themed surface component.

mod layout;
mod motion;
mod state;
mod style;
#[cfg(test)]
mod tests;
mod view;

use aura_anim::prelude::{MotionError, MotionRuntime, Timing};
use iced::Length;

use crate::component::{ComponentContext, ComponentUpdateCx, ComponentViewCx, MotionSlot};

pub(crate) use layout::ResolvedSurfaceLayout;
pub use layout::SurfaceLayout;
pub use motion::SurfaceMotion;
use state::SurfaceState;
pub use style::{SurfaceRole, SurfaceTreatment, SurfaceVariant};
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

/// Visual state for resolving surface motion.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SurfaceStyleState {
    /// Resting surface state.
    Idle,
    /// Pointer hover state.
    Hovered,
}

/// Read-only surface state consumed by rendering code.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SurfaceSnapshot {
    /// Surface visual variant.
    pub variant: SurfaceVariant,
    /// Current animated motion values.
    pub motion: SurfaceMotion,
    /// Surface visual state.
    pub style_state: SurfaceStyleState,
}

/// Stateful animated surface.
#[derive(Debug)]
pub struct Surface {
    variant: SurfaceVariant,
    state: SurfaceState,
    layout: SurfaceLayout,
    motion: MotionSlot<SurfaceMotion>,
}

impl Surface {
    /// Creates an animated surface for a visual variant.
    #[must_use]
    pub fn new(variant: impl Into<SurfaceVariant>) -> Self {
        Self {
            variant: variant.into(),
            state: SurfaceState::default(),
            layout: SurfaceLayout::empty(),
            motion: MotionSlot::new(),
        }
    }

    /// Creates an app background surface.
    #[must_use]
    pub fn background() -> Self {
        Self::new(SurfaceVariant::BACKGROUND)
    }

    /// Creates a regular component surface.
    #[must_use]
    pub fn regular() -> Self {
        Self::new(SurfaceVariant::REGULAR)
    }

    /// Creates a raised component surface.
    #[must_use]
    pub fn raised() -> Self {
        Self::new(SurfaceVariant::RAISED)
    }

    /// Returns this surface with a different visual variant.
    #[must_use]
    pub fn with_variant(mut self, variant: SurfaceVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Returns this surface with a different visual role.
    #[must_use]
    pub fn with_role(mut self, role: SurfaceRole) -> Self {
        self.variant = self.variant.with_role(role);
        self
    }

    /// Returns this surface with a different visual treatment.
    #[must_use]
    pub fn with_treatment(mut self, treatment: SurfaceTreatment) -> Self {
        self.variant = self.variant.with_treatment(treatment);
        self
    }

    /// Returns this surface as an app/page background.
    #[must_use]
    pub fn as_background(self) -> Self {
        self.with_variant(SurfaceVariant::BACKGROUND)
    }

    /// Returns this surface as a regular container.
    #[must_use]
    pub fn as_regular(self) -> Self {
        self.with_variant(SurfaceVariant::REGULAR)
    }

    /// Returns this surface as an elevated container.
    #[must_use]
    pub fn as_raised(self) -> Self {
        self.with_variant(SurfaceVariant::RAISED)
    }

    /// Returns this surface with plain treatment.
    #[must_use]
    pub fn plain(self) -> Self {
        self.with_treatment(SurfaceTreatment::Plain)
    }

    /// Returns this surface with elevated treatment.
    #[must_use]
    pub fn elevated(self) -> Self {
        self.with_treatment(SurfaceTreatment::Elevated)
    }

    /// Returns this surface with a different stable layout configuration.
    #[must_use]
    pub const fn with_layout(mut self, layout: SurfaceLayout) -> Self {
        self.layout = layout;
        self
    }

    /// Replaces this surface's stable layout configuration.
    pub fn set_layout(&mut self, layout: SurfaceLayout) {
        self.layout = layout;
    }

    /// Returns this surface with explicit inner padding.
    #[must_use]
    pub const fn with_padding(self, padding: f32) -> Self {
        self.padding(padding)
    }

    /// Returns this surface with explicit inner padding.
    #[must_use]
    pub const fn padding(mut self, padding: f32) -> Self {
        self.layout.padding = Some(padding);
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
        self.layout.width = Some(width.into());
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
        self.layout.height = Some(height.into());
        self
    }

    /// Updates this surface's inner padding.
    pub fn set_padding(&mut self, padding: f32) {
        self.layout.padding = Some(padding);
    }

    /// Clears this surface's inner padding.
    pub fn clear_padding(&mut self) {
        self.layout.padding = None;
    }

    /// Updates this surface's fixed rendered width.
    pub fn set_width(&mut self, width: impl Into<Length>) {
        self.layout.width = Some(width.into());
    }

    /// Clears this surface's fixed rendered width.
    pub fn clear_width(&mut self) {
        self.layout.width = None;
    }

    /// Updates this surface's fixed rendered height.
    pub fn set_height(&mut self, height: impl Into<Length>) {
        self.layout.height = Some(height.into());
    }

    /// Clears this surface's fixed rendered height.
    pub fn clear_height(&mut self) {
        self.layout.height = None;
    }

    /// Registers the surface motion handle using the current component context.
    pub fn register(&mut self, cx: &mut ComponentUpdateCx<'_>) {
        if self.motion.is_registered() {
            return;
        }

        let _ = self.motion.register(
            cx.runtime,
            self.motion_from_ctx(cx.context()),
            cx.context().theme_revision(),
        );
    }

    /// Synchronizes this surface's current motion target with the runtime.
    pub fn sync(&mut self, cx: &mut ComponentUpdateCx<'_>) -> Result<bool, MotionError> {
        self.motion
            .tween_to_or_finish(self.motion_from_ctx(cx.context()), interaction_timing(), cx)
    }

    /// Applies a surface interaction and transitions motion when registered.
    pub fn update(
        &mut self,
        interaction: SurfaceInteraction,
        cx: &mut ComponentUpdateCx<'_>,
    ) -> Result<bool, MotionError> {
        let previous = self.state;
        self.state.apply(interaction);

        self.animate_from_state(previous, cx)
    }

    /// Sets whether the surface is hovered and updates its motion target.
    pub fn set_hovered(
        &mut self,
        hovered: bool,
        cx: &mut ComponentUpdateCx<'_>,
    ) -> Result<bool, MotionError> {
        self.update(
            if hovered {
                SurfaceInteraction::HoverEnter
            } else {
                SurfaceInteraction::HoverExit
            },
            cx,
        )
    }

    /// Sets the surface variant and transitions motion when registered.
    pub fn set_variant(
        &mut self,
        variant: SurfaceVariant,
        cx: &mut ComponentUpdateCx<'_>,
    ) -> Result<bool, MotionError> {
        let previous = self.variant;
        self.variant = variant;

        self.animate_from_variant(previous, cx)
    }

    /// Sets this surface as an app/page background.
    pub fn set_background(&mut self, cx: &mut ComponentUpdateCx<'_>) -> Result<bool, MotionError> {
        self.set_variant(SurfaceVariant::BACKGROUND, cx)
    }

    /// Sets this surface as a regular container.
    pub fn set_regular(&mut self, cx: &mut ComponentUpdateCx<'_>) -> Result<bool, MotionError> {
        self.set_variant(SurfaceVariant::REGULAR, cx)
    }

    /// Sets this surface as an elevated container.
    pub fn set_raised(&mut self, cx: &mut ComponentUpdateCx<'_>) -> Result<bool, MotionError> {
        self.set_variant(SurfaceVariant::RAISED, cx)
    }

    /// Sets the surface role and transitions motion when registered.
    pub fn set_role(
        &mut self,
        role: SurfaceRole,
        cx: &mut ComponentUpdateCx<'_>,
    ) -> Result<bool, MotionError> {
        self.set_variant(self.variant.with_role(role), cx)
    }

    /// Sets the surface treatment and transitions motion when registered.
    pub fn set_treatment(
        &mut self,
        treatment: SurfaceTreatment,
        cx: &mut ComponentUpdateCx<'_>,
    ) -> Result<bool, MotionError> {
        self.set_variant(self.variant.with_treatment(treatment), cx)
    }

    /// Sets this surface with plain treatment.
    pub fn set_plain(&mut self, cx: &mut ComponentUpdateCx<'_>) -> Result<bool, MotionError> {
        self.set_treatment(SurfaceTreatment::Plain, cx)
    }

    /// Sets this surface with elevated treatment.
    pub fn set_elevated(&mut self, cx: &mut ComponentUpdateCx<'_>) -> Result<bool, MotionError> {
        self.set_treatment(SurfaceTreatment::Elevated, cx)
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
        let style_state = self.state.style_state();
        let motion = self.motion_value_for_context(cx.runtime, cx.context())?;

        Ok(SurfaceSnapshot {
            variant: self.variant,
            motion,
            style_state,
        })
    }

    /// Returns this surface visual variant.
    #[must_use]
    pub const fn variant(&self) -> SurfaceVariant {
        self.variant
    }

    /// Returns this surface visual role.
    #[must_use]
    pub const fn role(&self) -> SurfaceRole {
        self.variant.role
    }

    /// Returns this surface visual treatment.
    #[must_use]
    pub const fn treatment(&self) -> SurfaceTreatment {
        self.variant.treatment
    }

    /// Returns whether the pointer is over this surface.
    #[must_use]
    pub const fn is_hovered(&self) -> bool {
        self.state.is_hovered()
    }

    /// Returns this surface's current visual state.
    #[must_use]
    pub const fn style_state(&self) -> SurfaceStyleState {
        self.state.style_state()
    }

    /// Returns this surface's stable layout configuration.
    #[must_use]
    pub const fn layout(&self) -> SurfaceLayout {
        self.layout
    }

    fn motion_from_ctx(&self, context: &ComponentContext) -> SurfaceMotion {
        self.motion_from_state(context, self.state)
    }

    fn motion_from_state(&self, context: &ComponentContext, state: SurfaceState) -> SurfaceMotion {
        SurfaceMotion::from_theme(context.theme().theme(), self.variant, state.style_state())
    }

    fn motion_from_variant(
        &self,
        context: &ComponentContext,
        variant: SurfaceVariant,
    ) -> SurfaceMotion {
        SurfaceMotion::from_theme(context.theme().theme(), variant, self.state.style_state())
    }

    fn motion_value_for_context(
        &self,
        runtime: &MotionRuntime,
        context: &ComponentContext,
    ) -> Result<SurfaceMotion, MotionError> {
        Ok(self
            .motion
            .value_if_current(runtime, context.theme_revision())?
            .copied()
            .unwrap_or_else(|| self.motion_from_ctx(context)))
    }

    fn animate_from_state(
        &mut self,
        previous: SurfaceState,
        cx: &mut ComponentUpdateCx<'_>,
    ) -> Result<bool, MotionError> {
        if previous == self.state && !self.motion.is_registered() {
            return Ok(false);
        }

        let initial = self
            .motion
            .value_if_current(cx.runtime, cx.context().theme_revision())?
            .copied()
            .unwrap_or_else(|| self.motion_from_state(cx.context(), previous));
        let target = self.motion_from_ctx(cx.context());

        self.motion
            .tween_from_to_or_finish(initial, target, interaction_timing(), cx)
    }

    fn animate_from_variant(
        &mut self,
        previous: SurfaceVariant,
        cx: &mut ComponentUpdateCx<'_>,
    ) -> Result<bool, MotionError> {
        if previous == self.variant && !self.motion.is_registered() {
            return Ok(false);
        }

        let initial = self
            .motion
            .value_if_current(cx.runtime, cx.context().theme_revision())?
            .copied()
            .unwrap_or_else(|| self.motion_from_variant(cx.context(), previous));
        let target = self.motion_from_ctx(cx.context());

        self.motion
            .tween_from_to_or_finish(initial, target, interaction_timing(), cx)
    }
}

fn interaction_timing() -> Timing {
    Timing::ease_out(200.0)
}
