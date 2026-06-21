use spectrum_theme::{Color, Radius, ShadowLayer};

use crate::{
    component::ComponentContext,
    theme::{ButtonPrimaryTokens, ButtonStandardTokens, ThemeContext, ThemePack},
};

/// Visual role for resolving button theme tokens.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ButtonVariant {
    /// Neutral action using standard button tokens.
    Standard,
    /// Primary action using primary button tokens.
    Primary,
}

/// Visual state for resolving final button style.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ButtonStyleState {
    /// Resting button state.
    Idle,
    /// Pointer hover state.
    Hovered,
    /// Pressed button state.
    Pressed,
    /// Disabled button state.
    Disabled,
}

/// Final button style after applying interaction state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ButtonResolvedStyle {
    /// Final background color.
    pub background: Color,
    /// Final foreground color.
    pub foreground: Color,
    /// Final border color.
    pub border: Color,
    /// Focus ring color.
    pub focus_ring: Color,
    /// Button corner radius.
    pub radius: Radius,
    /// Raised-state shadow.
    pub shadow: ShadowLayer,
}

impl ButtonResolvedStyle {
    /// Resolves final button style from generated theme tokens.
    #[must_use]
    pub fn from_theme(theme: &ThemePack, variant: ButtonVariant, state: ButtonStyleState) -> Self {
        match variant {
            ButtonVariant::Standard => Self::from_standard_tokens(&theme.button.standard, state),
            ButtonVariant::Primary => Self::from_primary_tokens(&theme.button.primary, state),
        }
    }

    /// Resolves final button style from a theme context.
    #[must_use]
    pub fn from_context(
        context: &ThemeContext,
        variant: ButtonVariant,
        state: ButtonStyleState,
    ) -> Self {
        Self::from_theme(context.theme(), variant, state)
    }

    /// Resolves final button style from a component context.
    #[must_use]
    pub fn from_component_context(
        context: &ComponentContext,
        variant: ButtonVariant,
        state: ButtonStyleState,
    ) -> Self {
        Self::from_context(context.theme(), variant, state)
    }

    /// Resolves final style from standard button tokens.
    #[must_use]
    pub fn from_standard_tokens(tokens: &ButtonStandardTokens, state: ButtonStyleState) -> Self {
        Self::from_tokens(tokens, state)
    }

    /// Resolves final style from primary button tokens.
    #[must_use]
    pub fn from_primary_tokens(tokens: &ButtonPrimaryTokens, state: ButtonStyleState) -> Self {
        Self::from_tokens(tokens, state)
    }

    fn from_tokens(tokens: &impl ButtonTokens, state: ButtonStyleState) -> Self {
        let (background, foreground) = match state {
            ButtonStyleState::Idle => (tokens.bg(), tokens.fg()),
            ButtonStyleState::Hovered => (tokens.hover_bg(), tokens.fg()),
            ButtonStyleState::Pressed => (tokens.pressed_bg(), tokens.fg()),
            ButtonStyleState::Disabled => (tokens.disabled_bg(), tokens.disabled_fg()),
        };

        Self {
            background,
            foreground,
            border: tokens.border(),
            focus_ring: tokens.focus_ring(),
            radius: tokens.radius(),
            shadow: tokens.shadow(),
        }
    }
}

trait ButtonTokens {
    fn bg(&self) -> Color;
    fn fg(&self) -> Color;
    fn border(&self) -> Color;
    fn hover_bg(&self) -> Color;
    fn pressed_bg(&self) -> Color;
    fn disabled_bg(&self) -> Color;
    fn disabled_fg(&self) -> Color;
    fn focus_ring(&self) -> Color;
    fn radius(&self) -> Radius;
    fn shadow(&self) -> ShadowLayer;
}

macro_rules! impl_button_tokens {
    ($tokens:ty) => {
        impl ButtonTokens for $tokens {
            fn bg(&self) -> Color {
                self.bg
            }
            fn fg(&self) -> Color {
                self.fg
            }
            fn border(&self) -> Color {
                self.border
            }
            fn hover_bg(&self) -> Color {
                self.hover.bg
            }
            fn pressed_bg(&self) -> Color {
                self.pressed.bg
            }
            fn disabled_bg(&self) -> Color {
                self.disabled.bg
            }
            fn disabled_fg(&self) -> Color {
                self.disabled.fg
            }
            fn focus_ring(&self) -> Color {
                self.focus.ring
            }
            fn radius(&self) -> Radius {
                self.radius
            }
            fn shadow(&self) -> ShadowLayer {
                self.shadow
            }
        }
    };
}

impl_button_tokens!(ButtonStandardTokens);
impl_button_tokens!(ButtonPrimaryTokens);

#[cfg(test)]
mod tests {
    use spectrum_theme::Color;

    use crate::{
        component::ComponentContext,
        theme::{ThemeContext, ThemePack},
    };

    use super::{ButtonResolvedStyle, ButtonStyleState, ButtonVariant};

    #[test]
    fn standard_button_uses_neutral_surface_tokens() {
        let theme = ThemePack::adwaita();
        let style = ButtonResolvedStyle::from_theme(
            &theme,
            ButtonVariant::Standard,
            ButtonStyleState::Idle,
        );

        assert_eq!(style.background, theme.button.standard.bg);
        assert_eq!(style.foreground, theme.button.standard.fg);
        assert_eq!(style.border, theme.button.standard.border);
    }

    #[test]
    fn primary_button_uses_accent_tokens() {
        let theme = ThemePack::adwaita();
        let style =
            ButtonResolvedStyle::from_theme(&theme, ButtonVariant::Primary, ButtonStyleState::Idle);

        assert_eq!(style.background, theme.button.primary.bg);
        assert_eq!(style.foreground, theme.button.primary.fg);
        assert_eq!(style.border, theme.button.primary.border);
    }

    #[test]
    fn button_shape_and_elevation_come_from_theme() {
        let theme = ThemePack::adwaita();
        let style = ButtonResolvedStyle::from_theme(
            &theme,
            ButtonVariant::Standard,
            ButtonStyleState::Idle,
        );

        assert_eq!(style.radius, theme.button.standard.radius);
        assert_eq!(style.shadow, theme.button.standard.shadow);
        assert!(style.shadow.color().alpha() <= 48);
    }

    #[test]
    fn button_interaction_tokens_come_from_component_theme() {
        let hover = Color::new(221, 238, 255);
        let pressed = Color::new(205, 225, 250);
        let disabled = Color::new(230, 230, 230);
        let disabled_foreground = Color::new_rgba(48, 48, 48, 128);
        let mut theme = ThemePack::adwaita();

        theme.button.standard.hover.bg = hover;
        theme.button.standard.pressed.bg = pressed;
        theme.button.standard.disabled.bg = disabled;
        theme.button.standard.disabled.fg = disabled_foreground;

        assert_eq!(
            ButtonResolvedStyle::from_theme(
                &theme,
                ButtonVariant::Standard,
                ButtonStyleState::Hovered
            )
            .background,
            hover
        );
        assert_eq!(
            ButtonResolvedStyle::from_theme(
                &theme,
                ButtonVariant::Standard,
                ButtonStyleState::Pressed
            )
            .background,
            pressed
        );
        let disabled_style = ButtonResolvedStyle::from_theme(
            &theme,
            ButtonVariant::Standard,
            ButtonStyleState::Disabled,
        );
        assert_eq!(disabled_style.background, disabled);
        assert_eq!(disabled_style.foreground, disabled_foreground);
    }

    #[test]
    fn resolved_button_style_selects_state_backgrounds() {
        let theme = ThemePack::adwaita();

        let idle = ButtonResolvedStyle::from_theme(
            &theme,
            ButtonVariant::Standard,
            ButtonStyleState::Idle,
        );
        let hovered = ButtonResolvedStyle::from_theme(
            &theme,
            ButtonVariant::Standard,
            ButtonStyleState::Hovered,
        );
        let pressed = ButtonResolvedStyle::from_theme(
            &theme,
            ButtonVariant::Standard,
            ButtonStyleState::Pressed,
        );

        assert_eq!(idle.background, theme.button.standard.bg);
        assert_eq!(hovered.background, theme.button.standard.hover.bg);
        assert_eq!(pressed.background, theme.button.standard.pressed.bg);
        assert_eq!(hovered.foreground, theme.button.standard.fg);
    }

    #[test]
    fn resolved_button_style_uses_scoped_context() {
        let context = ThemeContext::from_theme(&ThemePack::adwaita());
        let scoped_bg = Color::new(221, 238, 255);
        let scoped = context.scoped(|theme| theme.button.standard.hover.bg = scoped_bg);

        let style = ButtonResolvedStyle::from_context(
            &scoped,
            ButtonVariant::Standard,
            ButtonStyleState::Hovered,
        );

        assert_eq!(style.background, scoped_bg);
        assert_ne!(context.theme().button.standard.hover.bg, scoped_bg);
    }

    #[test]
    fn resolved_button_style_uses_component_context() {
        let scoped_bg = Color::new(221, 238, 255);
        let context = ComponentContext::current()
            .scoped_theme(|theme| theme.button.standard.hover.bg = scoped_bg);

        let style = ButtonResolvedStyle::from_component_context(
            &context,
            ButtonVariant::Standard,
            ButtonStyleState::Hovered,
        );

        assert_eq!(style.background, scoped_bg);
    }

    #[test]
    fn resolved_disabled_button_uses_disabled_foreground() {
        let theme = ThemePack::adwaita();
        let disabled = ButtonResolvedStyle::from_theme(
            &theme,
            ButtonVariant::Primary,
            ButtonStyleState::Disabled,
        );

        assert_eq!(disabled.background, theme.button.primary.disabled.bg);
        assert_eq!(disabled.foreground, theme.button.primary.disabled.fg);
        assert_eq!(disabled.radius, theme.button.primary.radius);
    }
}
