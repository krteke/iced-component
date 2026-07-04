//! Animated component primitives for Iced applications.

pub mod button;
pub mod component;
pub mod motion;
pub mod motions;
pub mod panel;
pub mod spinner;
pub mod surface;
pub mod theme;

/// Re-exports animation runtime types from `aura_anim`.
pub mod anim {
    pub use aura_anim::core::runtime::{Motion, MotionError, MotionRuntime};
}
