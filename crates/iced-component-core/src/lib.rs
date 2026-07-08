//! Shared runtime support for themed Iced component crates.

pub mod component;
pub mod motion;

/// Re-exports animation runtime types from `aura_anim`.
pub mod anim {
    pub use aura_anim::core::runtime::{Motion, MotionError, MotionRuntime};
}
