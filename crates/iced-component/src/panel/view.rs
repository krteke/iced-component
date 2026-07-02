//! View builder for [`Panel`].

use aura_anim::prelude::MotionError;
use iced::Element;
use iced::widget::{Column, text};

use super::Panel;
use crate::{
    component::ComponentViewCx,
    surface::{ResolvedSurfaceLayout, SurfaceEvent, SurfaceSnapshot, SurfaceView},
};

/// Iced view builder for [`Panel`].
pub struct PanelView<'a, Message> {
    snapshot: SurfaceSnapshot,
    layout: ResolvedSurfaceLayout,
    title: Option<&'a str>,
    spacing: f32,
    header: Option<Element<'a, Message>>,
    body: Option<Element<'a, Message>>,
    footer: Option<Element<'a, Message>>,
    on_event: Option<Box<dyn Fn(SurfaceEvent) -> Message + 'a>>,
}

impl Panel {
    /// Builds an Iced view for this panel.
    #[must_use]
    pub fn view<'a, Message>(&'a self, cx: &ComponentViewCx<'_>) -> PanelView<'a, Message>
    where
        Message: Clone + 'a,
    {
        self.try_view(cx)
            .expect("panel surface motion handle belongs to the provided runtime")
    }

    /// Tries to build an Iced view for this panel.
    pub fn try_view<'a, Message>(
        &'a self,
        cx: &ComponentViewCx<'_>,
    ) -> Result<PanelView<'a, Message>, MotionError>
    where
        Message: Clone + 'a,
    {
        Ok(PanelView {
            snapshot: self.surface.snapshot(cx)?,
            layout: self.surface.layout().resolve(cx.context()),
            title: self.title.as_deref(),
            spacing: self.spacing,
            header: None,
            body: None,
            footer: None,
            on_event: None,
        })
    }
}

impl<'a, Message> PanelView<'a, Message> {
    /// Sets a custom header slot.
    #[must_use]
    pub fn header(mut self, header: impl Into<Element<'a, Message>>) -> Self {
        self.header = Some(header.into());
        self
    }

    /// Sets the body slot.
    #[must_use]
    pub fn body(mut self, body: impl Into<Element<'a, Message>>) -> Self {
        self.body = Some(body.into());
        self
    }

    /// Sets a footer slot.
    #[must_use]
    pub fn footer(mut self, footer: impl Into<Element<'a, Message>>) -> Self {
        self.footer = Some(footer.into());
        self
    }

    /// Maps panel surface events into application messages.
    #[must_use]
    pub fn connect(mut self, mapper: impl Fn(SurfaceEvent) -> Message + 'a) -> Self {
        self.on_event = Some(Box::new(mapper));
        self
    }
}

impl<'a, Message> From<PanelView<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(view: PanelView<'a, Message>) -> Self {
        let mut content = Column::new().spacing(view.spacing);
        let mut has_content = false;

        if let Some(header) = view.header {
            content = content.push(header);
            has_content = true;
        } else if let Some(title) = view.title {
            content = content.push(text(title).size(17));
            has_content = true;
        }

        if let Some(body) = view.body {
            content = content.push(body);
            has_content = true;
        }

        if let Some(footer) = view.footer {
            content = content.push(footer);
            has_content = true;
        }

        if !has_content {
            content = content.push(text(""));
        }

        let surface = SurfaceView::from_parts(view.snapshot, content, view.layout);
        connect_surface(surface, view.on_event).into()
    }
}

fn connect_surface<'a, Message>(
    surface: SurfaceView<'a, Message>,
    on_event: Option<Box<dyn Fn(SurfaceEvent) -> Message + 'a>>,
) -> SurfaceView<'a, Message>
where
    Message: 'a,
{
    match on_event {
        Some(mapper) => surface.connect(mapper),
        None => surface,
    }
}
