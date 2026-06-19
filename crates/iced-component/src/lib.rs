//! Animated component primitives for Iced applications.

pub mod component;
pub mod motion;
pub mod theme;

pub use aura_anim_core::{Motion, MotionError, MotionRuntime};
pub use component::ComponentMotion;
pub use motion::{
    Duration, Easing, MotionPreferences, MotionPreferencesController, MotionSpeed, MotionTokens,
    MotionTransition, Timing,
};
pub use theme::{
    Color, ElevationTokens, FontStyle, FontWeight, Length, LengthUnit, LineHeight, PaletteTokens,
    Radius, Rgb, Rgba, ShadowLayer, ShapeTokens, ThemeBuildError, ThemePack, set_theme_pack,
    with_theme_pack,
};
