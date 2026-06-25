//! Panel component built on top of [`crate::surface::Surface`].

use aura_anim_core::{MotionError, MotionRuntime};
use iced::widget::{column, text};
use iced::{Element, Length};

use crate::{
    component::ComponentContext,
    surface::{Surface, SurfaceEvent, SurfaceLayout, SurfaceSnapshot, SurfaceView},
};

/// Raised content panel with an optional title.
#[derive(Debug)]
pub struct Panel {
    surface: Surface,
    title: Option<String>,
    spacing: f32,
}

impl Panel {
    /// Creates a raised panel.
    #[must_use]
    pub fn new() -> Self {
        Self {
            surface: Surface::raised().with_padding(18.0),
            title: None,
            spacing: 12.0,
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
        self.spacing = spacing;
        self
    }

    /// Updates this panel's body spacing.
    pub fn set_spacing(&mut self, spacing: f32) {
        self.spacing = spacing;
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

    /// Returns this panel with a different stable surface layout.
    #[must_use]
    pub const fn with_layout(mut self, layout: SurfaceLayout) -> Self {
        self.surface = self.surface.with_layout(layout);
        self
    }

    /// Replaces this panel's stable surface layout.
    pub fn set_layout(&mut self, layout: SurfaceLayout) {
        self.surface.set_layout(layout);
    }

    /// Returns this panel with explicit inner padding.
    #[must_use]
    pub const fn with_padding(mut self, padding: f32) -> Self {
        self.surface = self.surface.with_padding(padding);
        self
    }

    /// Updates this panel's inner padding.
    pub fn set_padding(&mut self, padding: f32) {
        self.surface.set_padding(padding);
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

    /// Registers the panel surface motion handle.
    pub fn register(&mut self, runtime: &mut MotionRuntime, context: &ComponentContext) {
        self.surface.register(runtime, context);
    }

    /// Applies a panel surface event.
    pub fn update_event(
        &mut self,
        event: SurfaceEvent,
        runtime: &mut MotionRuntime,
    ) -> Result<bool, MotionError> {
        self.surface.update_event(event, runtime)
    }

    /// Returns a rendering snapshot of the panel surface.
    pub fn snapshot(
        &self,
        runtime: &MotionRuntime,
        context: &ComponentContext,
    ) -> Result<SurfaceSnapshot, MotionError> {
        self.surface.snapshot(runtime, context)
    }

    /// Builds an Iced view for this panel.
    #[must_use]
    pub fn view<'a, Message>(
        &'a self,
        runtime: &MotionRuntime,
        context: &ComponentContext,
        body: impl Into<Element<'a, Message>>,
    ) -> SurfaceView<'a, Message>
    where
        Message: Clone + 'a,
    {
        let body = body.into();
        let panel_content = match self.title.as_deref() {
            Some(title) => column![text(title).size(17), body]
                .spacing(self.spacing)
                .into(),
            None => body,
        };

        self.surface.view(runtime, context, panel_content)
    }

    /// Returns this panel's title.
    #[must_use]
    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    /// Returns this panel's body spacing.
    #[must_use]
    pub const fn spacing(&self) -> f32 {
        self.spacing
    }

    /// Returns this panel's backing surface.
    #[must_use]
    pub const fn surface(&self) -> &Surface {
        &self.surface
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
    pub const fn padding(&self) -> f32 {
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
    use aura_anim_core::{MotionRuntime, timing::Duration};
    use float_cmp::assert_approx_eq;
    use iced::Element;
    use iced::widget::text;

    use crate::{
        component::ComponentContext,
        panel::Panel,
        surface::{Surface, SurfaceEvent, SurfaceInteraction, SurfaceLayout},
        theme::SurfaceRole,
    };

    #[test]
    fn panel_defaults_to_raised_surface() {
        let runtime = MotionRuntime::new();
        let context = ComponentContext::current();
        let panel = Panel::new();

        let snapshot = panel.snapshot(&runtime, &context).unwrap();

        assert_eq!(snapshot.role, SurfaceRole::Raised);
        assert!(snapshot.style.shadow.is_some());
    }

    #[test]
    fn panel_accessors_and_setters_update_stable_config() {
        let mut panel = Panel::titled("Overview")
            .with_spacing(10.0)
            .with_padding(16.0)
            .with_width(240.0)
            .with_height(120.0);

        assert_eq!(panel.title(), Some("Overview"));
        assert_approx_eq!(f32, panel.spacing(), 10.0);
        assert_approx_eq!(f32, panel.padding(), 16.0);
        assert_eq!(panel.width(), Some(iced::Length::Fixed(240.0)));
        assert_eq!(panel.height(), Some(iced::Length::Fixed(120.0)));

        panel.set_title("Details");
        panel.set_spacing(8.0);
        panel.set_layout(SurfaceLayout::new(
            12.0,
            Some(iced::Length::Fixed(220.0)),
            None,
        ));

        assert_eq!(panel.title(), Some("Details"));
        assert_approx_eq!(f32, panel.spacing(), 8.0);
        assert_approx_eq!(f32, panel.padding(), 12.0);
        assert_eq!(panel.width(), Some(iced::Length::Fixed(220.0)));
        assert_eq!(panel.height(), None);

        panel.clear_title();
        panel.clear_width();
        panel.set_height(96.0);
        panel.set_surface(Surface::regular().with_padding(6.0));
        panel.surface_mut().set_padding(9.0);

        assert_eq!(panel.title(), None);
        assert_eq!(panel.width(), None);
        assert_eq!(panel.height(), None);
        assert_eq!(panel.surface().role(), SurfaceRole::Regular);
        assert_approx_eq!(f32, panel.padding(), 9.0);
    }

    #[test]
    fn panel_delegates_surface_events() {
        let mut runtime = MotionRuntime::new();
        let context = ComponentContext::current();
        let mut panel = Panel::new();

        panel.register(&mut runtime, &context);
        panel
            .update_event(
                SurfaceEvent::Interaction(SurfaceInteraction::HoverEnter),
                &mut runtime,
            )
            .unwrap();
        runtime.tick(Duration::from_millis(200.0));

        assert_approx_eq!(
            f32,
            panel.snapshot(&runtime, &context).unwrap().motion.elevation,
            1.15
        );
    }

    #[test]
    fn panel_view_builds_iced_element() {
        #[derive(Clone)]
        enum Message {
            Panel(SurfaceEvent),
        }

        let runtime = MotionRuntime::new();
        let context = ComponentContext::current();
        let panel = Panel::titled("Overview");

        let view = panel
            .view(&runtime, &context, text("Panel body"))
            .connect(Message::Panel);
        let _element: Element<'_, Message> = view.into();

        let Message::Panel(event) =
            Message::Panel(SurfaceEvent::Interaction(SurfaceInteraction::HoverEnter));
        assert_eq!(
            event,
            SurfaceEvent::Interaction(SurfaceInteraction::HoverEnter)
        );
    }
}
