//! Animated component primitives for Iced applications.

pub mod button;
pub mod component;
pub mod motion;
pub mod theme;

pub use aura_anim_core::{Motion, MotionError, MotionRuntime};
pub use button::{ButtonResolvedStyle, ButtonStyleState, ButtonVariant};
pub use component::ComponentMotion;
pub use motion::{
    Duration, Easing, MotionPreferences, MotionPreferencesController, MotionSpeed, MotionTokens,
    MotionTransition, Timing,
};
pub use theme::{
    ADWAITA_LIGHT_TOML, AppTokens, ButtonPrimaryTokens, ButtonStandardTokens, Color, FontStyle,
    FontWeight, Length, LengthUnit, LineHeight, Radius, Rgb, Rgba, ShadowLayer,
    SurfaceRaisedTokens, SurfaceRole, SurfaceStyleTokens, SurfaceTokens, ThemeBuildError,
    ThemeContext, ThemeLoadError, ThemePack, set_theme_pack, with_theme_context, with_theme_pack,
};
