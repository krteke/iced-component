use crate::{Color, Radius, ThemePack};

/// Resolved theme tokens shared by interactive controls.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ControlStyleTokens {
    /// Control border color.
    pub border: Color,
    /// Keyboard focus ring color.
    pub focus_ring: Color,
    /// Foreground used while disabled.
    pub disabled_foreground: Color,
    /// Hover-state overlay color.
    pub hover_overlay: Color,
    /// Pressed-state overlay color.
    pub pressed_overlay: Color,
    /// Control corner radius.
    pub radius: Radius,
}

impl ControlStyleTokens {
    /// Resolves shared control style tokens from the theme baseline.
    #[must_use]
    pub fn from_theme(theme: &ThemePack) -> Self {
        Self {
            border: theme.palette.border,
            focus_ring: theme.palette.focus_ring,
            disabled_foreground: theme.control.disabled_foreground,
            hover_overlay: theme.control.hover_overlay,
            pressed_overlay: theme.control.pressed_overlay,
            radius: theme.shape.control_radius,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ControlStyleTokens;
    use crate::ThemePack;

    #[test]
    fn control_style_uses_theme_control_tokens() {
        let theme = ThemePack::adwaita();
        let style = ControlStyleTokens::from_theme(&theme);

        assert_eq!(style.disabled_foreground, theme.control.disabled_foreground);
        assert_eq!(style.hover_overlay, theme.control.hover_overlay);
        assert_eq!(style.pressed_overlay, theme.control.pressed_overlay);
    }

    #[test]
    fn control_style_uses_shared_shape_and_focus_tokens() {
        let theme = ThemePack::adwaita();
        let style = ControlStyleTokens::from_theme(&theme);

        assert_eq!(style.radius, theme.shape.control_radius);
        assert_eq!(style.focus_ring, theme.palette.focus_ring);
        assert!(style.hover_overlay.alpha() <= 24);
        assert!(style.pressed_overlay.alpha() <= 36);
    }
}
