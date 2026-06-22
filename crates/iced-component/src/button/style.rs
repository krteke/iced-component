use spectrum_theme::{Color, Radius, ShadowLayer};

use crate::{
    component::ComponentContext,
    theme::{
        ButtonDestructiveTokens, ButtonStandardTokens, ButtonSuggestedTokens, ThemeContext,
        ThemePack,
    },
};

/// Semantic role for resolving button theme tokens.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ButtonRole {
    /// Neutral action.
    Standard,
    /// Recommended action.
    Suggested,
    /// Dangerous or destructive action.
    Destructive,
}

/// Visual treatment for a button.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ButtonAppearance {
    /// Theme default treatment for the role.
    Default,
    /// Minimal treatment for low-emphasis buttons.
    Flat,
    /// Explicit raised treatment.
    Raised,
    /// Fully rounded capsule treatment.
    Pill,
    /// Equal-width circular icon-style treatment.
    Circular,
}

/// Complete visual variant for resolving button style.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ButtonVariant {
    /// Semantic role.
    pub role: ButtonRole,
    /// Visual treatment.
    pub appearance: ButtonAppearance,
}

impl ButtonVariant {
    /// Neutral default button.
    pub const STANDARD: Self = Self::new(ButtonRole::Standard, ButtonAppearance::Default);
    /// Recommended-action default button.
    pub const SUGGESTED: Self = Self::new(ButtonRole::Suggested, ButtonAppearance::Default);
    /// Backward-compatible alias for [`ButtonVariant::SUGGESTED`].
    pub const PRIMARY: Self = Self::SUGGESTED;
    /// Destructive-action default button.
    pub const DESTRUCTIVE: Self = Self::new(ButtonRole::Destructive, ButtonAppearance::Default);

    /// Creates a button variant from role and appearance.
    #[must_use]
    pub const fn new(role: ButtonRole, appearance: ButtonAppearance) -> Self {
        Self { role, appearance }
    }

    /// Returns this variant with a different visual appearance.
    #[must_use]
    pub const fn with_appearance(mut self, appearance: ButtonAppearance) -> Self {
        self.appearance = appearance;
        self
    }

    /// Returns this variant with a different semantic role.
    #[must_use]
    pub const fn with_role(mut self, role: ButtonRole) -> Self {
        self.role = role;
        self
    }
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
        let style = match variant.role {
            ButtonRole::Standard => Self::from_standard_tokens(&theme.button.standard, state),
            ButtonRole::Suggested => Self::from_suggested_tokens(&theme.button.suggested, state),
            ButtonRole::Destructive => {
                Self::from_destructive_tokens(&theme.button.destructive, state)
            }
        };

        style.apply_appearance(variant.appearance)
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

    /// Resolves final style from suggested-action button tokens.
    #[must_use]
    pub fn from_suggested_tokens(tokens: &ButtonSuggestedTokens, state: ButtonStyleState) -> Self {
        Self::from_tokens(tokens, state)
    }

    /// Resolves final style from destructive-action button tokens.
    #[must_use]
    pub fn from_destructive_tokens(
        tokens: &ButtonDestructiveTokens,
        state: ButtonStyleState,
    ) -> Self {
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

    fn apply_appearance(mut self, appearance: ButtonAppearance) -> Self {
        match appearance {
            ButtonAppearance::Flat => {
                self.background = transparent(self.background);
                self.border = transparent(self.border);
                self.shadow = transparent_shadow(self.shadow);
                self
            }
            ButtonAppearance::Default
            | ButtonAppearance::Raised
            | ButtonAppearance::Pill
            | ButtonAppearance::Circular => self,
        }
    }
}

fn transparent(color: Color) -> Color {
    Color::new_rgba(color.red(), color.green(), color.blue(), 0)
}

fn transparent_shadow(shadow: ShadowLayer) -> ShadowLayer {
    ShadowLayer::new(
        transparent(shadow.color()),
        shadow.offset_x(),
        shadow.offset_y(),
        shadow.blur(),
        shadow.spread(),
    )
    .expect("changing only shadow color alpha preserves a valid shadow")
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
impl_button_tokens!(ButtonSuggestedTokens);
impl_button_tokens!(ButtonDestructiveTokens);

#[cfg(test)]
mod tests {
    use spectrum_theme::Color;

    use crate::{
        component::ComponentContext,
        theme::{ThemeContext, ThemePack},
    };

    use super::{
        ButtonAppearance, ButtonResolvedStyle, ButtonRole, ButtonStyleState, ButtonVariant,
    };

    #[test]
    fn standard_button_uses_neutral_surface_tokens() {
        let theme = ThemePack::adwaita();
        let style = ButtonResolvedStyle::from_theme(
            &theme,
            ButtonVariant::STANDARD,
            ButtonStyleState::Idle,
        );

        assert_eq!(style.background, theme.button.standard.bg);
        assert_eq!(style.foreground, theme.button.standard.fg);
        assert_eq!(style.border, theme.button.standard.border);
    }

    #[test]
    fn suggested_button_uses_accent_tokens() {
        let theme = ThemePack::adwaita();
        let style = ButtonResolvedStyle::from_theme(
            &theme,
            ButtonVariant::SUGGESTED,
            ButtonStyleState::Idle,
        );

        assert_eq!(style.background, theme.button.suggested.bg);
        assert_eq!(style.foreground, theme.button.suggested.fg);
        assert_eq!(style.border, theme.button.suggested.border);
    }

    #[test]
    fn destructive_button_uses_destructive_tokens() {
        let theme = ThemePack::adwaita();
        let style = ButtonResolvedStyle::from_theme(
            &theme,
            ButtonVariant::DESTRUCTIVE,
            ButtonStyleState::Idle,
        );

        assert_eq!(style.background, theme.button.destructive.bg);
        assert_eq!(style.foreground, theme.button.destructive.fg);
        assert_eq!(style.border, theme.button.destructive.border);
    }

    #[test]
    fn button_variant_keeps_role_and_appearance_separate() {
        let variant = ButtonVariant::new(ButtonRole::Suggested, ButtonAppearance::Flat);

        assert_eq!(variant.role, ButtonRole::Suggested);
        assert_eq!(variant.appearance, ButtonAppearance::Flat);
        assert_eq!(
            ButtonVariant::SUGGESTED.with_appearance(ButtonAppearance::Pill),
            ButtonVariant::new(ButtonRole::Suggested, ButtonAppearance::Pill)
        );
    }

    #[test]
    fn flat_appearance_removes_background_border_and_shadow() {
        let theme = ThemePack::adwaita();
        let variant = ButtonVariant::SUGGESTED.with_appearance(ButtonAppearance::Flat);
        let style = ButtonResolvedStyle::from_theme(&theme, variant, ButtonStyleState::Idle);

        assert_eq!(style.foreground, theme.button.suggested.fg);
        assert_eq!(style.background.alpha(), 0);
        assert_eq!(style.border.alpha(), 0);
        assert_eq!(style.shadow.color().alpha(), 0);
    }

    #[test]
    fn raised_appearance_keeps_role_tokens() {
        let theme = ThemePack::adwaita();
        let variant = ButtonVariant::SUGGESTED.with_appearance(ButtonAppearance::Raised);
        let style = ButtonResolvedStyle::from_theme(&theme, variant, ButtonStyleState::Idle);

        assert_eq!(style.background, theme.button.suggested.bg);
        assert_eq!(style.border, theme.button.suggested.border);
        assert_eq!(style.shadow, theme.button.suggested.shadow);
    }

    #[test]
    fn button_shape_and_elevation_come_from_theme() {
        let theme = ThemePack::adwaita();
        let style = ButtonResolvedStyle::from_theme(
            &theme,
            ButtonVariant::STANDARD,
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
                ButtonVariant::STANDARD,
                ButtonStyleState::Hovered
            )
            .background,
            hover
        );
        assert_eq!(
            ButtonResolvedStyle::from_theme(
                &theme,
                ButtonVariant::STANDARD,
                ButtonStyleState::Pressed
            )
            .background,
            pressed
        );
        let disabled_style = ButtonResolvedStyle::from_theme(
            &theme,
            ButtonVariant::STANDARD,
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
            ButtonVariant::STANDARD,
            ButtonStyleState::Idle,
        );
        let hovered = ButtonResolvedStyle::from_theme(
            &theme,
            ButtonVariant::STANDARD,
            ButtonStyleState::Hovered,
        );
        let pressed = ButtonResolvedStyle::from_theme(
            &theme,
            ButtonVariant::STANDARD,
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
            ButtonVariant::STANDARD,
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
            ButtonVariant::STANDARD,
            ButtonStyleState::Hovered,
        );

        assert_eq!(style.background, scoped_bg);
    }

    #[test]
    fn resolved_disabled_button_uses_disabled_foreground() {
        let theme = ThemePack::adwaita();
        let disabled = ButtonResolvedStyle::from_theme(
            &theme,
            ButtonVariant::SUGGESTED,
            ButtonStyleState::Disabled,
        );

        assert_eq!(disabled.background, theme.button.suggested.disabled.bg);
        assert_eq!(disabled.foreground, theme.button.suggested.disabled.fg);
        assert_eq!(disabled.radius, theme.button.suggested.radius);
    }

    #[test]
    fn primary_variant_is_a_compatibility_alias_for_suggested() {
        let theme = ThemePack::adwaita();

        assert_eq!(
            ButtonResolvedStyle::from_theme(&theme, ButtonVariant::PRIMARY, ButtonStyleState::Idle),
            ButtonResolvedStyle::from_theme(
                &theme,
                ButtonVariant::SUGGESTED,
                ButtonStyleState::Idle
            )
        );
    }
}
