//! Animated component primitives for Iced applications.

pub mod button;
pub mod component;
pub mod motion;
pub mod theme;

pub use aura_anim_core::{Motion, MotionError, MotionRuntime};
#[cfg(feature = "iced")]
pub use button::{AnimatedButtonView, button_style};
