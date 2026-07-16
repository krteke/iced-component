use iced::{Background, Border, Element, Length, Rectangle, Theme, mouse};
use iced_component_core::anim::MotionError;
use iced_widget::{
    canvas::{self, Canvas},
    container,
};
use spectrum_theme::iced::IcedColorAdapter;

use crate::{context::ViewCx, loading_indicator::LoadingIndicator};

use super::{LoadingIndicatorMode, LoadingIndicatorSnapshot, geometry};

impl LoadingIndicator {
    /// Builds the Iced view using the Material expressive shape engine.
    #[must_use]
    pub fn view<Message>(&self, cx: &ViewCx<'_>) -> Element<'static, Message>
    where
        Message: 'static,
    {
        self.try_view(cx)
            .expect("loading indicator motion handle belongs to the provided runtime")
    }

    /// Tries to build the Iced view without panicking on a mismatched runtime.
    pub fn try_view<Message>(
        &self,
        cx: &ViewCx<'_>,
    ) -> Result<Element<'static, Message>, MotionError>
    where
        Message: 'static,
    {
        let snapshot = self.snapshot(cx)?;
        let size = Length::Fixed(snapshot.visual.size);
        let canvas: Element<'static, Message> = Canvas::new(LoadingIndicatorProgram { snapshot })
            .width(size)
            .height(size)
            .into();

        if snapshot.contained {
            Ok(container(canvas)
                .width(size)
                .height(size)
                .style(move |_| contained_style(snapshot))
                .into())
        } else {
            Ok(canvas)
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct LoadingIndicatorProgram {
    snapshot: LoadingIndicatorSnapshot,
}

impl<Message, Renderer> canvas::Program<Message, Theme, Renderer> for LoadingIndicatorProgram
where
    Renderer: iced_widget::graphics::geometry::Renderer,
{
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry<Renderer>> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        let side = frame.width().min(frame.height());

        let shape = match self.snapshot.mode {
            LoadingIndicatorMode::Indeterminate => {
                geometry::loading_shape_path(frame.center(), side, self.snapshot.phase)
            }
            LoadingIndicatorMode::Determinate(progress) => {
                geometry::determinate_loading_shape_path(frame.center(), side, progress)
            }
        };
        let active = if self.snapshot.contained {
            self.snapshot.visual.contained_active
        } else {
            self.snapshot.visual.active
        };
        frame.fill(&shape, active.color());

        vec![frame.into_geometry()]
    }
}

fn contained_style(snapshot: LoadingIndicatorSnapshot) -> container::Style {
    container::Style {
        background: Some(Background::Color(snapshot.visual.container.color())),
        border: Border {
            radius: (snapshot.visual.size / 2.0).into(),
            ..Border::default()
        },
        snap: true,
        ..container::Style::default()
    }
}

#[cfg(test)]
mod tests {
    use iced::Background;
    use spectrum_theme::{Color, iced::IcedColorAdapter};

    use super::contained_style;
    use crate::loading_indicator::{
        LoadingIndicatorMode, LoadingIndicatorSnapshot, LoadingIndicatorVisual,
    };

    #[test]
    fn contained_background_uses_a_snapped_round_renderer_quad() {
        let color = Color::new(1, 2, 3);
        let snapshot = LoadingIndicatorSnapshot {
            mode: LoadingIndicatorMode::Indeterminate,
            contained: true,
            phase: 0.0,
            visual: LoadingIndicatorVisual {
                size: 48.0,
                active: color,
                container: color,
                contained_active: color,
            },
        };

        let style = contained_style(snapshot);

        assert_eq!(style.background, Some(Background::Color(color.color())));
        assert!((style.border.radius.top_left - 24.0).abs() < f32::EPSILON);
        assert!(style.snap);
    }
}
