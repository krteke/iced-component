use iced::Length;

use crate::component::ComponentContext;

/// Stable surface layout configuration.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SurfaceLayout {
    pub(crate) padding: Option<f32>,
    pub(crate) width: Option<Length>,
    pub(crate) height: Option<Length>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct ResolvedSurfaceLayout {
    pub(crate) padding: f32,
    pub(crate) width: Option<Length>,
    pub(crate) height: Option<Length>,
}

impl SurfaceLayout {
    /// Creates a stable surface layout configuration.
    #[must_use]
    pub const fn new(padding: Option<f32>, width: Option<Length>, height: Option<Length>) -> Self {
        Self {
            padding,
            width,
            height,
        }
    }

    /// Creates an empty surface layout.
    #[must_use]
    pub const fn empty() -> Self {
        Self::new(None, None, None)
    }

    /// Returns the explicit inner padding override.
    #[must_use]
    pub const fn padding(self) -> Option<f32> {
        self.padding
    }

    /// Returns the fixed width, if configured.
    #[must_use]
    pub const fn width(self) -> Option<Length> {
        self.width
    }

    /// Returns the fixed height, if configured.
    #[must_use]
    pub const fn height(self) -> Option<Length> {
        self.height
    }

    pub(crate) fn resolve(self, context: &ComponentContext) -> ResolvedSurfaceLayout {
        ResolvedSurfaceLayout {
            padding: self
                .padding
                .unwrap_or_else(|| context.theme().theme().control.surface.padding.value()),
            width: self.width,
            height: self.height,
        }
    }
}

impl Default for SurfaceLayout {
    fn default() -> Self {
        Self::empty()
    }
}

#[cfg(test)]
mod tests {
    use super::SurfaceLayout;

    #[test]
    fn empty_layout_has_no_fixed_size() {
        let layout = SurfaceLayout::empty();

        assert_eq!(layout.padding(), None);
        assert_eq!(layout.width(), None);
        assert_eq!(layout.height(), None);
    }
}
