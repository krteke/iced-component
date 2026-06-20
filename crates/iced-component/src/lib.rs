//! Animated component primitives for Iced applications.

pub mod button;
pub mod component;
pub mod motion;
pub mod theme;

pub use aura_anim_core::{Motion, MotionError, MotionRuntime};
pub use button::{ButtonStyleTokens, ButtonVariant};
pub use component::ComponentMotion;
pub use motion::{
    Duration, Easing, MotionPreferences, MotionPreferencesController, MotionSpeed, MotionTokens,
    MotionTransition, Timing,
};
pub use theme::{
    Color, ControlStyleTokens, ControlTokens, ElevationTokens, FontStyle, FontWeight, Length,
    LengthUnit, LineHeight, PaletteTokens, Radius, Rgb, Rgba, ShadowLayer, ShapeTokens,
    SurfaceRole, SurfaceStyleTokens, ThemeBuildError, ThemePack, set_theme_pack, with_theme_pack,
};
