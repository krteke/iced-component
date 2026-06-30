//! Iced view builder for animated surfaces.

use aura_anim::prelude::MotionError;
use iced::border::Radius;
use iced::widget::{container, mouse_area};
use iced::{Background, Border, Color, Element, Length, Shadow, Vector};
use spectrum_theme::iced::{IcedColorAdapter, IcedRadiusAdapter, IcedShadowAdapter};

use super::{Surface, SurfaceEvent, SurfaceInteraction, SurfaceSnapshot};
use crate::component::ComponentViewCx;

/// Iced view builder for [`Surface`].
pub struct SurfaceView<'a, Message> {
    snapshot: SurfaceSnapshot,
    content: Element<'a, Message>,
    on_event: Option<Box<dyn Fn(SurfaceEvent) -> Message + 'a>>,
    layout: SurfaceLayout,
}

/// Stable surface layout configuration.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SurfaceLayout {
    pub(crate) padding: f32,
    pub(crate) width: Option<Length>,
    pub(crate) height: Option<Length>,
}

impl SurfaceLayout {
    /// Creates a stable surface layout configuration.
    #[must_use]
    pub const fn new(padding: f32, width: Option<Length>, height: Option<Length>) -> Self {
        Self {
            padding,
            width,
            height,
        }
    }

    /// Returns the inner padding.
    #[must_use]
    pub const fn padding(self) -> f32 {
        self.padding
    }

    /// Returns the fixed width, if configured.
    #[must_use]
    pub const fn width(self) -> Option<Length> {
        self.width
    }

    /// Returns the fixed height, if configured.
    #[must_use]
    pub const fn height(self) -> Option<Length> {
        self.height
    }
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
            self.layout(),
        ))
    }
}

impl<'a, Message> SurfaceView<'a, Message> {
    pub(crate) fn from_parts(
        snapshot: SurfaceSnapshot,
        content: impl Into<Element<'a, Message>>,
        layout: SurfaceLayout,
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
    let style = snapshot.style;
    let motion = snapshot.motion;

    container::Style {
        text_color: Some(style.foreground.color()),
        background: Some(Background::Color(color_with_alpha(
            style.background.color(),
            motion.bg_alpha,
        ))),
        border: Border {
            color: color_with_alpha(style.border.color(), motion.border_alpha),
            width: motion.border_width,
            radius: scaled_radius(style.radius.radius_px(), motion.radius_scale),
        },
        shadow: style
            .shadow
            .map(|shadow| scaled_shadow(shadow.shadow_px(), motion))
            .unwrap_or_default(),
        snap: true,
    }
}

fn scaled_shadow(shadow: Shadow, motion: crate::surface::SurfaceMotion) -> Shadow {
    let intensity = motion.elevation * motion.shadow_alpha;

    Shadow {
        color: color_with_alpha(shadow.color, intensity),
        offset: Vector::new(
            shadow.offset.x * motion.elevation,
            shadow.offset.y * motion.elevation,
        ),
        blur_radius: shadow.blur_radius * motion.elevation * motion.shadow_blur,
    }
}

fn scaled_radius(radius: Radius, scale: f32) -> Radius {
    Radius {
        top_left: radius.top_left * scale,
        top_right: radius.top_right * scale,
        bottom_right: radius.bottom_right * scale,
        bottom_left: radius.bottom_left * scale,
    }
}

fn color_with_alpha(color: Color, alpha_multiplier: f32) -> Color {
    Color {
        a: color.a * alpha_multiplier.clamp(0.0, 1.0),
        ..color
    }
}
