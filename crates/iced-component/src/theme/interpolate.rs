use aura_anim::core::{interpolate::InterpolationProgress, traits::Interpolate};
use spectrum_theme::{Color, Length, Radius, ShadowLayer};

pub(crate) fn color(from: Color, to: Color, progress: InterpolationProgress) -> Color {
    Color::new_rgba(
        u8::interpolate_progress(&from.red(), &to.red(), progress),
        u8::interpolate_progress(&from.green(), &to.green(), progress),
        u8::interpolate_progress(&from.blue(), &to.blue(), progress),
        u8::interpolate_progress(&from.alpha(), &to.alpha(), progress),
    )
}

pub(crate) fn length(from: Length, to: Length, progress: InterpolationProgress) -> Length {
    Length::new(
        f32::interpolate_progress(&from.value(), &to.value(), progress),
        to.unit(),
    )
    .expect("interpolated theme length remains finite")
}

pub(crate) fn radius(from: Radius, to: Radius, progress: InterpolationProgress) -> Radius {
    Radius::new(length(from.length(), to.length(), progress))
        .expect("interpolated theme radius remains non-negative")
}

pub(crate) fn shadow(
    from: ShadowLayer,
    to: ShadowLayer,
    progress: InterpolationProgress,
) -> ShadowLayer {
    ShadowLayer::new(
        color(from.color(), to.color(), progress),
        length(from.offset_x(), to.offset_x(), progress),
        length(from.offset_y(), to.offset_y(), progress),
        length(from.blur(), to.blur(), progress),
        length(from.spread(), to.spread(), progress),
    )
    .expect("interpolated theme shadow remains valid")
}
