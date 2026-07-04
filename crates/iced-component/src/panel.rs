//! Panel component built on top of [`crate::surface::Surface`].

mod layout;
mod view;

use aura_anim::prelude::MotionError;
use iced::Length;

use crate::{
    component::{ComponentUpdateCx, ComponentViewCx},
    surface::{Surface, SurfaceEvent, SurfaceLayout, SurfaceSnapshot, SurfaceStyleState},
};

pub use layout::PanelLayout;
pub(crate) use layout::ResolvedPanelLayout;
pub use view::PanelView;

/// Raised content panel with an optional title.
#[derive(Debug)]
pub struct Panel {
    surface: Surface,
    title: Option<String>,
    layout: PanelLayout,
}

impl Panel {
    /// Creates a raised panel.
    #[must_use]
    pub fn new() -> Self {
        Self {
            surface: Surface::raised(),
            title: None,
            layout: PanelLayout::default(),
        }
    }

    /// Creates a raised panel with a title.
    #[must_use]
    pub fn titled(title: impl Into<String>) -> Self {
        Self::new().with_title(title)
    }

    /// Returns this panel with a title.
    #[must_use]
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Updates this panel's title.
    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = Some(title.into());
    }

    /// Clears this panel's title.
    pub fn clear_title(&mut self) {
        self.title = None;
    }

    /// Returns this panel with a different body spacing.
    #[must_use]
    pub const fn with_spacing(mut self, spacing: f32) -> Self {
        self.layout.spacing = Some(spacing);
        self
    }

    /// Updates this panel's body spacing.
    pub fn set_spacing(&mut self, spacing: f32) {
        self.layout.spacing = Some(spacing);
    }

    /// Clears this panel's explicit body spacing.
    pub fn clear_spacing(&mut self) {
        self.layout.spacing = None;
    }

    /// Returns this panel with a different title text size.
    #[must_use]
    pub const fn with_title_size(mut self, title_size: f32) -> Self {
        self.layout.title_size = Some(title_size);
        self
    }

    /// Updates this panel's title text size.
    pub fn set_title_size(&mut self, title_size: f32) {
        self.layout.title_size = Some(title_size);
    }

    /// Clears this panel's explicit title text size.
    pub fn clear_title_size(&mut self) {
        self.layout.title_size = None;
    }

    /// Returns this panel with a different backing surface.
    #[must_use]
    pub fn with_surface(mut self, surface: Surface) -> Self {
        self.surface = surface;
        self
    }

    /// Replaces this panel's backing surface.
    pub fn set_surface(&mut self, surface: Surface) {
        self.surface = surface;
    }

    /// Returns this panel with a different stable panel layout override set.
    #[must_use]
    pub const fn with_panel_layout(mut self, layout: PanelLayout) -> Self {
        self.layout = layout;
        self
    }

    /// Replaces this panel's stable panel layout override set.
    pub fn set_panel_layout(&mut self, layout: PanelLayout) {
        self.layout = layout;
    }

    /// Returns this panel with a different stable surface layout.
    #[must_use]
    pub fn with_layout(mut self, layout: SurfaceLayout) -> Self {
        self.surface = self.surface.with_layout(layout);
        self
    }

    /// Replaces this panel's stable surface layout.
    pub fn set_layout(&mut self, layout: SurfaceLayout) {
        self.surface.set_layout(layout);
    }

    /// Returns this panel with explicit inner padding.
    #[must_use]
    pub fn with_padding(mut self, padding: f32) -> Self {
        self.surface = self.surface.with_padding(padding);
        self
    }

    /// Updates this panel's inner padding.
    pub fn set_padding(&mut self, padding: f32) {
        self.surface.set_padding(padding);
    }

    /// Clears this panel's inner padding.
    pub fn clear_padding(&mut self) {
        self.surface.clear_padding();
    }

    /// Returns this panel with a fixed rendered width.
    #[must_use]
    pub fn with_width(mut self, width: impl Into<Length>) -> Self {
        self.surface.set_width(width);
        self
    }

    /// Updates this panel's fixed rendered width.
    pub fn set_width(&mut self, width: impl Into<Length>) {
        self.surface.set_width(width);
    }

    /// Clears this panel's fixed rendered width.
    pub fn clear_width(&mut self) {
        self.surface.clear_width();
    }

    /// Returns this panel with a fixed rendered height.
    #[must_use]
    pub fn with_height(mut self, height: impl Into<Length>) -> Self {
        self.surface.set_height(height);
        self
    }

    /// Updates this panel's fixed rendered height.
    pub fn set_height(&mut self, height: impl Into<Length>) {
        self.surface.set_height(height);
    }

    /// Clears this panel's fixed rendered height.
    pub fn clear_height(&mut self) {
        self.surface.clear_height();
    }

    /// Registers the panel surface motion handle using the current component context.
    pub fn register(&mut self, cx: &mut ComponentUpdateCx<'_>) {
        self.surface.register(cx);
    }

    /// Synchronizes the panel surface's current motion target with the runtime.
    pub fn sync(&mut self, cx: &mut ComponentUpdateCx<'_>) -> Result<bool, MotionError> {
        self.surface.sync(cx)
    }

    /// Applies a panel surface event.
    pub fn update_event(
        &mut self,
        event: SurfaceEvent,
        cx: &mut ComponentUpdateCx<'_>,
    ) -> Result<bool, MotionError> {
        self.surface.update_event(event, cx)
    }

    /// Sets whether the panel surface is hovered.
    pub fn set_hovered(
        &mut self,
        hovered: bool,
        cx: &mut ComponentUpdateCx<'_>,
    ) -> Result<bool, MotionError> {
        self.surface.set_hovered(hovered, cx)
    }

    /// Returns a rendering snapshot of the panel surface.
    pub fn snapshot(&self, cx: &ComponentViewCx<'_>) -> Result<SurfaceSnapshot, MotionError> {
        self.surface.snapshot(cx)
    }

    /// Returns this panel's title.
    #[must_use]
    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    /// Returns this panel's body spacing.
    #[must_use]
    pub const fn spacing(&self) -> Option<f32> {
        self.layout.spacing()
    }

    /// Returns this panel's title text size.
    #[must_use]
    pub const fn title_size(&self) -> Option<f32> {
        self.layout.title_size()
    }

    /// Returns this panel's stable panel layout override set.
    #[must_use]
    pub const fn panel_layout(&self) -> PanelLayout {
        self.layout
    }

    /// Returns this panel's backing surface.
    #[must_use]
    pub const fn surface(&self) -> &Surface {
        &self.surface
    }

    /// Returns whether the panel surface is hovered.
    #[must_use]
    pub const fn is_hovered(&self) -> bool {
        self.surface.is_hovered()
    }

    /// Returns this panel surface's current visual state.
    #[must_use]
    pub const fn style_state(&self) -> SurfaceStyleState {
        self.surface.style_state()
    }

    /// Returns this panel's mutable backing surface.
    pub fn surface_mut(&mut self) -> &mut Surface {
        &mut self.surface
    }

    /// Consumes this panel and returns its backing surface.
    #[must_use]
    pub fn into_surface(self) -> Surface {
        self.surface
    }

    /// Returns this panel's stable surface layout.
    #[must_use]
    pub const fn layout(&self) -> SurfaceLayout {
        self.surface.layout()
    }

    /// Returns this panel's inner padding.
    #[must_use]
    pub const fn padding(&self) -> Option<f32> {
        self.layout().padding()
    }

    /// Returns this panel's fixed rendered width, if configured.
    #[must_use]
    pub const fn width(&self) -> Option<Length> {
        self.layout().width()
    }

    /// Returns this panel's fixed rendered height, if configured.
    #[must_use]
    pub const fn height(&self) -> Option<Length> {
        self.layout().height()
    }
}

impl Default for Panel {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use aura_anim::prelude::*;
    use float_cmp::assert_approx_eq;
    use iced::Element;
    use iced::widget::{row, text};
    use spectrum_theme::Color;

    use crate::{
        component::{ComponentContext, ComponentUpdateCx, ComponentViewCx},
        panel::Panel,
        surface::{
            Surface, SurfaceAnimationBuilder, SurfaceAnimationProvider, SurfaceEvent,
            SurfaceInteraction, SurfaceLayout, SurfaceMotionTransition, SurfaceMotionTrigger,
            SurfaceStyleState, SurfaceTreatment, SurfaceVariant,
        },
    };

    #[test]
    fn panel_defaults_to_raised_surface() {
        let runtime = MotionRuntime::new();
        let context = ComponentContext::adwaita();
        let cx = ComponentViewCx::new(&runtime, &context);
        let panel = Panel::new();

        let snapshot = panel.snapshot(&cx).unwrap();

        assert_eq!(snapshot.variant, SurfaceVariant::RAISED);
        assert!(snapshot.motion.tokens.shadow.blur().value() > 0.0);
    }

    #[test]
    fn panel_accessors_and_setters_update_stable_config() {
        let mut panel = Panel::titled("Overview")
            .with_spacing(10.0)
            .with_title_size(18.0)
            .with_padding(16.0)
            .with_width(240.0)
            .with_height(120.0);

        assert_eq!(panel.title(), Some("Overview"));
        assert_eq!(panel.spacing(), Some(10.0));
        assert_eq!(panel.title_size(), Some(18.0));
        assert_eq!(panel.padding(), Some(16.0));
        assert_eq!(panel.width(), Some(iced::Length::Fixed(240.0)));
        assert_eq!(panel.height(), Some(iced::Length::Fixed(120.0)));

        panel.set_title("Details");
        panel.set_spacing(8.0);
        panel.set_title_size(16.0);
        panel.set_layout(SurfaceLayout::new(
            Some(12.0),
            Some(iced::Length::Fixed(220.0)),
            None,
        ));

        assert_eq!(panel.title(), Some("Details"));
        assert_eq!(panel.spacing(), Some(8.0));
        assert_eq!(panel.title_size(), Some(16.0));
        assert_eq!(panel.padding(), Some(12.0));
        assert_eq!(panel.width(), Some(iced::Length::Fixed(220.0)));
        assert_eq!(panel.height(), None);

        panel.clear_title();
        panel.clear_spacing();
        panel.clear_title_size();
        panel.clear_padding();
        panel.clear_width();
        panel.set_height(96.0);

        assert_eq!(panel.padding(), None);
        assert_eq!(panel.spacing(), None);
        assert_eq!(panel.title_size(), None);
        assert_eq!(panel.width(), None);
        assert_eq!(panel.height(), Some(iced::Length::Fixed(96.0)));

        panel.set_surface(Surface::regular().with_padding(6.0));
        panel.surface_mut().set_padding(9.0);

        assert_eq!(panel.title(), None);
        assert_eq!(panel.width(), None);
        assert_eq!(panel.height(), None);
        assert_eq!(panel.surface().treatment(), SurfaceTreatment::Plain);
        assert_eq!(panel.padding(), Some(9.0));
    }

    #[test]
    fn panel_delegates_surface_events() {
        let mut runtime = MotionRuntime::new();
        let mut context = ComponentContext::adwaita();
        let mut panel = Panel::new();

        {
            let mut cx = ComponentUpdateCx::new(&mut runtime, &mut context);
            panel.register(&mut cx);
            panel
                .update_event(
                    SurfaceEvent::Interaction(SurfaceInteraction::HoverEnter),
                    &mut cx,
                )
                .unwrap();
        }
        runtime.tick(Duration::from_millis(200.0));

        let cx = ComponentViewCx::new(&runtime, &context);
        let snapshot = panel.snapshot(&cx).unwrap();
        assert_approx_eq!(f32, snapshot.motion.elevation, 1.0);
        assert_color_eq(
            snapshot.motion.tokens.bg,
            context.theme().theme().surface.raised.hover.bg,
        );
    }

    #[test]
    fn panel_uses_context_surface_animation_provider() {
        struct SlowVariantProvider;

        impl SurfaceAnimationProvider for SlowVariantProvider {
            fn surface_animation(
                &self,
                transition: &SurfaceMotionTransition,
            ) -> SurfaceAnimationBuilder {
                let timing = match transition.trigger {
                    SurfaceMotionTrigger::Variant => Timing::linear(1000.0),
                    _ => Timing::linear(100.0),
                };

                Arc::new(move |transition| {
                    Tween::between(transition.from, transition.to, timing).boxed()
                })
            }
        }

        let mut runtime = MotionRuntime::new();
        let mut context = ComponentContext::adwaita();
        context
            .animation_mut()
            .set_surface_provider(SlowVariantProvider);
        let mut panel = Panel::new();

        {
            let mut cx = ComponentUpdateCx::new(&mut runtime, &mut context);
            panel.register(&mut cx);
            panel.surface_mut().set_regular(&mut cx).unwrap();
        }
        runtime.tick(Duration::from_millis(500.0));

        let cx = ComponentViewCx::new(&runtime, &context);
        assert_approx_eq!(f32, panel.snapshot(&cx).unwrap().motion.elevation, 0.5);
    }

    #[test]
    fn panel_delegates_hover_setter() {
        let mut runtime = MotionRuntime::new();
        let mut context = ComponentContext::adwaita();
        let mut panel = Panel::new();

        {
            let mut cx = ComponentUpdateCx::new(&mut runtime, &mut context);
            panel.set_hovered(true, &mut cx).unwrap();
        }
        runtime.tick(Duration::from_millis(200.0));

        let cx = ComponentViewCx::new(&runtime, &context);

        assert!(panel.is_hovered());
        assert_eq!(panel.style_state(), SurfaceStyleState::Hovered);
        let snapshot = panel.snapshot(&cx).unwrap();
        assert_approx_eq!(f32, snapshot.motion.elevation, 1.0);
        assert_color_eq(
            snapshot.motion.tokens.bg,
            context.theme().theme().surface.raised.hover.bg,
        );
    }

    #[test]
    fn panel_snapshot_uses_current_theme_through_surface() {
        let mut runtime = MotionRuntime::new();
        let mut context = ComponentContext::adwaita();
        let mut panel = Panel::new();

        {
            let mut cx = ComponentUpdateCx::new(&mut runtime, &mut context);
            panel.register(&mut cx);
            panel
                .update_event(
                    SurfaceEvent::Interaction(SurfaceInteraction::HoverEnter),
                    &mut cx,
                )
                .unwrap();
        }
        runtime.tick(Duration::from_millis(1.0));

        let patched_bg = "#ddeeff".parse().unwrap();
        context.patch_theme(|theme| theme.surface.raised.hover.bg = patched_bg);

        let cx = ComponentViewCx::new(&runtime, &context);
        let snapshot = panel.snapshot(&cx).unwrap();

        assert_eq!(snapshot.motion.tokens.bg, patched_bg);
        assert_approx_eq!(f32, snapshot.motion.elevation, 1.0);
    }

    #[test]
    fn panel_view_builds_iced_element() {
        #[derive(Clone)]
        enum Message {
            Panel(SurfaceEvent),
        }

        let runtime = MotionRuntime::new();
        let context = ComponentContext::adwaita();
        let cx = ComponentViewCx::new(&runtime, &context);
        let panel = Panel::titled("Overview");

        let view = panel
            .view(&cx)
            .body(text("Panel body"))
            .footer(row![text("Ready"), text("Idle")])
            .connect(Message::Panel);
        let _element: Element<'_, Message> = view.into();

        let Message::Panel(event) =
            Message::Panel(SurfaceEvent::Interaction(SurfaceInteraction::HoverEnter));
        assert_eq!(
            event,
            SurfaceEvent::Interaction(SurfaceInteraction::HoverEnter)
        );
    }

    fn assert_color_eq(left: Color, right: Color) {
        assert_eq!(left.red(), right.red());
        assert_eq!(left.green(), right.green());
        assert_eq!(left.blue(), right.blue());
        assert_eq!(left.alpha(), right.alpha());
    }
}
