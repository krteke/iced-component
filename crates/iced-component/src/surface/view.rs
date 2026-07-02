//! Iced view builder for animated surfaces.

use aura_anim::prelude::MotionError;
use iced::widget::{container, mouse_area};
use iced::{Background, Border, Color, Element, Shadow, Vector};
use spectrum_theme::iced::{IcedColorAdapter, IcedRadiusAdapter, IcedShadowAdapter};

use super::{ResolvedSurfaceLayout, Surface, SurfaceEvent, SurfaceInteraction, SurfaceSnapshot};
use crate::component::ComponentViewCx;

/// Iced view builder for [`Surface`].
pub struct SurfaceView<'a, Message> {
    snapshot: SurfaceSnapshot,
    content: Element<'a, Message>,
    on_event: Option<Box<dyn Fn(SurfaceEvent) -> Message + 'a>>,
    layout: ResolvedSurfaceLayout,
}

impl Surface {
    /// Builds an Iced view for this surface.
    #[must_use]
    pub fn view<'a, Message>(
        &self,
        cx: &ComponentViewCx<'_>,
        child: impl Into<Element<'a, Message>>,
    ) -> SurfaceView<'a, Message>
    where
        Message: Clone + 'a,
    {
        self.try_view(cx, child)
            .expect("surface motion handle belongs to the provided runtime")
    }

    /// Tries to build an Iced view for this surface.
    pub fn try_view<'a, Message>(
        &self,
        cx: &ComponentViewCx<'_>,
        child: impl Into<Element<'a, Message>>,
    ) -> Result<SurfaceView<'a, Message>, MotionError>
    where
        Message: Clone + 'a,
    {
        Ok(SurfaceView::from_parts(
            self.snapshot(cx)?,
            child,
            self.layout().resolve(cx.context()),
        ))
    }
}

impl<'a, Message> SurfaceView<'a, Message> {
    pub(crate) fn from_parts(
        snapshot: SurfaceSnapshot,
        content: impl Into<Element<'a, Message>>,
        layout: ResolvedSurfaceLayout,
    ) -> Self {
        Self {
            snapshot,
            content: content.into(),
            on_event: None,
            layout,
        }
    }

    /// Maps surface events into application messages.
    #[must_use]
    pub fn connect(mut self, mapper: impl Fn(SurfaceEvent) -> Message + 'a) -> Self {
        self.on_event = Some(Box::new(mapper));
        self
    }
}

impl<'a, Message> From<SurfaceView<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(view: SurfaceView<'a, Message>) -> Self {
        let mut widget = container(view.content)
            .padding(view.layout.padding)
            .style(move |_theme| surface_style(view.snapshot));
        if let Some(width) = view.layout.width {
            widget = widget.width(width);
        }
        if let Some(height) = view.layout.height {
            widget = widget.height(height);
        }

        let Some(on_event) = view.on_event else {
            return widget.into();
        };

        mouse_area(widget)
            .on_enter(on_event(SurfaceEvent::Interaction(
                SurfaceInteraction::HoverEnter,
            )))
            .on_exit(on_event(SurfaceEvent::Interaction(
                SurfaceInteraction::HoverExit,
            )))
            .into()
    }
}

/// Converts an animated surface snapshot into an Iced container style.
#[must_use]
pub fn surface_style(snapshot: SurfaceSnapshot) -> container::Style {
    let motion = snapshot.motion;
    let tokens = motion.tokens;

    container::Style {
        text_color: Some(tokens.fg.color()),
        background: Some(Background::Color(color_with_alpha(tokens.bg.color(), 1.0))),
        border: Border {
            color: tokens.border.color(),
            width: tokens.border_width.value(),
            radius: tokens.radius.radius_px(),
        },
        shadow: scaled_shadow(tokens.shadow.shadow_px(), motion.elevation),
        snap: true,
    }
}

fn scaled_shadow(shadow: Shadow, elevation: f32) -> Shadow {
    Shadow {
        color: color_with_alpha(shadow.color, elevation),
        offset: Vector::new(shadow.offset.x * elevation, shadow.offset.y * elevation),
        blur_radius: shadow.blur_radius * elevation,
    }
}

fn color_with_alpha(color: Color, alpha_multiplier: f32) -> Color {
    Color {
        a: color.a * alpha_multiplier.clamp(0.0, 1.0),
        ..color
    }
}
