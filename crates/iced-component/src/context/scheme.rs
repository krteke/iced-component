//! Adapter-wide light and dark mode.

/// Color scheme shared by every retained themed context.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ColorScheme {
    /// Dark color scheme.
    Dark,
    /// Light color scheme.
    #[default]
    Light,
}

impl ColorScheme {
    pub(crate) const fn toggled(self) -> Self {
        match self {
            Self::Dark => Self::Light,
            Self::Light => Self::Dark,
        }
    }
}
