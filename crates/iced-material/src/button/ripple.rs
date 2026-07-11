//! Reference-faithful Material patterned ripple rendering.

mod shader;
mod state;

use iced::{Color, Rectangle};
use iced_widget::{core::time::Instant, renderer::wgpu::primitive};

pub(crate) use state::PressRippleState;

/// Draws every active or exiting ripple with the Material patterned shader.
pub(crate) fn draw<Renderer>(
    renderer: &mut Renderer,
    bounds: Rectangle,
    ripples: &PressRippleState,
    color: Color,
    radius: f32,
    now: Instant,
) where
    Renderer: primitive::Renderer,
{
    for ripple in ripples.visible(now) {
        shader::draw(renderer, bounds, ripple, color, radius);
    }
}
