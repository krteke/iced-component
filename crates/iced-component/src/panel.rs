//! Panel component built on top of [`crate::surface::Surface`].

use aura_anim_core::{MotionError, MotionRuntime};
use iced::Element;
use iced::widget::{column, text};

use crate::{
    component::ComponentContext,
    surface::{Surface, SurfaceEvent, SurfaceSnapshot, SurfaceView},
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
            surface: Surface::raised().padding(18.0),
            title: None,
            spacing: 12.0,
        }
    }

    /// Creates a raised panel with a title.
    #[must_use]
    pub fn titled(title: impl Into<String>) -> Self {
        Self::new().title(title)
    }

    /// Returns this panel with a title.
    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
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
        surface::{SurfaceEvent, SurfaceInteraction},
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
