use aura_anim::core::{interpolate::InterpolationProgress, traits::Interpolate};
use spectrum_theme::Color;

pub(crate) fn color(from: Color, to: Color, progress: InterpolationProgress) -> Color {
    Color::new_rgba(
        u8::interpolate_progress(&from.red(), &to.red(), progress),
        u8::interpolate_progress(&from.green(), &to.green(), progress),
        u8::interpolate_progress(&from.blue(), &to.blue(), progress),
        u8::interpolate_progress(&from.alpha(), &to.alpha(), progress),
    )
}
