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
    Color, FontStyle, FontWeight, Length, LengthUnit, LineHeight, Radius, Rgb, Rgba, ShadowLayer,
    ThemeBuildError,
};
