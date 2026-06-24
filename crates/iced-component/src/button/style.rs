use spectrum_theme::{Color, Length, LengthUnit, Radius, ShadowLayer};

use crate::{
    component::ComponentContext,
    theme::{
        ButtonDestructiveFilledTokens, ButtonDestructiveFlatTokens, ButtonDestructiveRaisedTokens,
        ButtonStandardFilledTokens, ButtonStandardFlatTokens, ButtonStandardRaisedTokens,
        ButtonSuggestedFilledTokens, ButtonSuggestedFlatTokens, ButtonSuggestedRaisedTokens,
        ThemeContext, ThemePack,
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

/// Visual emphasis/treatment for a button.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ButtonTreatment {
    /// Filled button surface.
    Filled,
    /// Minimal treatment for low-emphasis buttons.
    Flat,
    /// Explicit raised treatment.
    Raised,
}

/// Button outline shape.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ButtonShape {
    /// Rounded rectangle shape.
    Rounded,
    /// Fully rounded capsule shape.
    Pill,
    /// Equal-width circular icon-style shape.
    Circular,
}

/// Complete visual variant for resolving button style.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ButtonVariant {
    /// Semantic role.
    pub role: ButtonRole,
    /// Visual emphasis/treatment.
    pub treatment: ButtonTreatment,
    /// Button outline shape.
    pub shape: ButtonShape,
}

impl ButtonVariant {
    /// Neutral default button.
    pub const STANDARD: Self = Self::new(
        ButtonRole::Standard,
        ButtonTreatment::Filled,
        ButtonShape::Rounded,
    );
    /// Recommended-action default button.
    pub const SUGGESTED: Self = Self::new(
        ButtonRole::Suggested,
        ButtonTreatment::Filled,
        ButtonShape::Rounded,
    );
    /// Backward-compatible alias for [`ButtonVariant::SUGGESTED`].
    pub const PRIMARY: Self = Self::SUGGESTED;
    /// Destructive-action default button.
    pub const DESTRUCTIVE: Self = Self::new(
        ButtonRole::Destructive,
        ButtonTreatment::Filled,
        ButtonShape::Rounded,
    );

    /// Creates a button variant from role, treatment, and shape.
    #[must_use]
    pub const fn new(role: ButtonRole, treatment: ButtonTreatment, shape: ButtonShape) -> Self {
        Self {
            role,
            treatment,
            shape,
        }
    }

    /// Returns this variant with a different visual treatment.
    #[must_use]
    pub const fn with_treatment(mut self, treatment: ButtonTreatment) -> Self {
        self.treatment = treatment;
        self
    }

    /// Returns this variant with a different outline shape.
    #[must_use]
    pub const fn with_shape(mut self, shape: ButtonShape) -> Self {
        self.shape = shape;
        self
    }

    /// Returns this variant with a different semantic role.
    #[must_use]
    pub const fn with_role(mut self, role: ButtonRole) -> Self {
        self.role = role;
        self
    }

    /// Returns this variant as a standard action.
    #[must_use]
    pub const fn set_standard(self) -> Self {
        self.with_role(ButtonRole::Standard)
    }

    /// Returns this variant as a suggested action.
    #[must_use]
    pub const fn set_suggested(self) -> Self {
        self.with_role(ButtonRole::Suggested)
    }

    /// Returns this variant as a destructive action.
    #[must_use]
    pub const fn set_destructive(self) -> Self {
        self.with_role(ButtonRole::Destructive)
    }

    /// Returns this variant with filled treatment.
    #[must_use]
    pub const fn set_filled(self) -> Self {
        self.with_treatment(ButtonTreatment::Filled)
    }

    /// Returns this variant with flat treatment.
    #[must_use]
    pub const fn set_flat(self) -> Self {
        self.with_treatment(ButtonTreatment::Flat)
    }

    /// Returns this variant with raised treatment.
    #[must_use]
    pub const fn set_raised(self) -> Self {
        self.with_treatment(ButtonTreatment::Raised)
    }

    /// Returns this variant with rounded shape.
    #[must_use]
    pub const fn set_rounded(self) -> Self {
        self.with_shape(ButtonShape::Rounded)
    }

    /// Returns this variant with pill shape.
    #[must_use]
    pub const fn set_pill(self) -> Self {
        self.with_shape(ButtonShape::Pill)
    }

    /// Returns this variant with circular shape.
    #[must_use]
    pub const fn set_circular(self) -> Self {
        self.with_shape(ButtonShape::Circular)
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
    /// Button border width.
    pub border_width: Length,
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
        let mut style = match (variant.role, variant.treatment) {
            (ButtonRole::Standard, ButtonTreatment::Filled) => {
                Self::from_tokens(&theme.button.standard.filled, state)
            }
            (ButtonRole::Standard, ButtonTreatment::Flat) => {
                Self::from_tokens(&theme.button.standard.flat, state)
            }
            (ButtonRole::Standard, ButtonTreatment::Raised) => {
                Self::from_tokens(&theme.button.standard.raised, state)
            }
            (ButtonRole::Suggested, ButtonTreatment::Filled) => {
                Self::from_tokens(&theme.button.suggested.filled, state)
            }
            (ButtonRole::Suggested, ButtonTreatment::Flat) => {
                Self::from_tokens(&theme.button.suggested.flat, state)
            }
            (ButtonRole::Suggested, ButtonTreatment::Raised) => {
                Self::from_tokens(&theme.button.suggested.raised, state)
            }
            (ButtonRole::Destructive, ButtonTreatment::Filled) => {
                Self::from_tokens(&theme.button.destructive.filled, state)
            }
            (ButtonRole::Destructive, ButtonTreatment::Flat) => {
                Self::from_tokens(&theme.button.destructive.flat, state)
            }
            (ButtonRole::Destructive, ButtonTreatment::Raised) => {
                Self::from_tokens(&theme.button.destructive.raised, state)
            }
        };
        style.radius = match variant.shape {
            ButtonShape::Rounded => theme.button.shape.rounded.radius,
            ButtonShape::Pill => theme.button.shape.pill.radius,
            ButtonShape::Circular => theme.button.shape.circular.radius,
        };
        style.border_width = theme.control.border.width;
        style
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

    /// Resolves final style from standard filled button tokens.
    #[must_use]
    pub fn from_standard_tokens(
        tokens: &ButtonStandardFilledTokens,
        state: ButtonStyleState,
    ) -> Self {
        Self::from_tokens(tokens, state)
    }

    /// Resolves final style from suggested filled button tokens.
    #[must_use]
    pub fn from_suggested_tokens(
        tokens: &ButtonSuggestedFilledTokens,
        state: ButtonStyleState,
    ) -> Self {
        Self::from_tokens(tokens, state)
    }

    /// Resolves final style from destructive filled button tokens.
    #[must_use]
    pub fn from_destructive_tokens(
        tokens: &ButtonDestructiveFilledTokens,
        state: ButtonStyleState,
    ) -> Self {
        Self::from_tokens(tokens, state)
    }

    fn from_tokens(tokens: &impl ButtonTokens, state: ButtonStyleState) -> Self {
        let (background, foreground, border) = tokens.colors(state);
        let placeholder = Length::new(0.0, LengthUnit::Px).expect("valid placeholder length");

        Self {
            background,
            foreground,
            border,
            border_width: placeholder,
            focus_ring: tokens.focus_ring(),
            radius: Radius::new(placeholder).expect("valid placeholder radius"),
            shadow: tokens.shadow(),
        }
    }
}

trait ButtonTokens {
    fn colors(&self, state: ButtonStyleState) -> (Color, Color, Color);
    fn focus_ring(&self) -> Color;
    fn shadow(&self) -> ShadowLayer;
}

macro_rules! impl_button_tokens {
    ($tokens:ty) => {
        impl ButtonTokens for $tokens {
            fn colors(&self, state: ButtonStyleState) -> (Color, Color, Color) {
                match state {
                    ButtonStyleState::Idle => (self.idle.bg, self.idle.fg, self.idle.border),
                    ButtonStyleState::Hovered => (self.hover.bg, self.hover.fg, self.hover.border),
                    ButtonStyleState::Pressed => {
                        (self.pressed.bg, self.pressed.fg, self.pressed.border)
                    }
                    ButtonStyleState::Disabled => {
                        (self.disabled.bg, self.disabled.fg, self.disabled.border)
                    }
                }
            }
            fn focus_ring(&self) -> Color {
                self.focus.ring
            }
            fn shadow(&self) -> ShadowLayer {
                self.shadow
            }
        }
    };
}

impl_button_tokens!(ButtonStandardFilledTokens);
impl_button_tokens!(ButtonStandardFlatTokens);
impl_button_tokens!(ButtonStandardRaisedTokens);
impl_button_tokens!(ButtonSuggestedFilledTokens);
impl_button_tokens!(ButtonSuggestedFlatTokens);
impl_button_tokens!(ButtonSuggestedRaisedTokens);
impl_button_tokens!(ButtonDestructiveFilledTokens);
impl_button_tokens!(ButtonDestructiveFlatTokens);
impl_button_tokens!(ButtonDestructiveRaisedTokens);

#[cfg(test)]
mod tests {
    use spectrum_theme::Color;

    use crate::{
        component::ComponentContext,
        theme::{ThemeContext, ThemePack},
    };

    use super::{
        ButtonResolvedStyle, ButtonRole, ButtonShape, ButtonStyleState, ButtonTreatment,
        ButtonVariant,
    };

    #[test]
    fn standard_button_uses_neutral_surface_tokens() {
        let theme = ThemePack::adwaita();
        let style = ButtonResolvedStyle::from_theme(
            &theme,
            ButtonVariant::STANDARD,
            ButtonStyleState::Idle,
        );

        assert_eq!(style.background, theme.button.standard.filled.idle.bg);
        assert_eq!(style.foreground, theme.button.standard.filled.idle.fg);
        assert_eq!(style.border, theme.button.standard.filled.idle.border);
        assert_eq!(style.border_width, theme.control.border.width);
    }

    #[test]
    fn suggested_button_uses_accent_tokens() {
        let theme = ThemePack::adwaita();
        let style = ButtonResolvedStyle::from_theme(
            &theme,
            ButtonVariant::SUGGESTED,
            ButtonStyleState::Idle,
        );

        assert_eq!(style.background, theme.button.suggested.filled.idle.bg);
        assert_eq!(style.foreground, theme.button.suggested.filled.idle.fg);
        assert_eq!(style.border, theme.button.suggested.filled.idle.border);
    }

    #[test]
    fn destructive_button_uses_destructive_tokens() {
        let theme = ThemePack::adwaita();
        let style = ButtonResolvedStyle::from_theme(
            &theme,
            ButtonVariant::DESTRUCTIVE,
            ButtonStyleState::Idle,
        );

        assert_eq!(style.background, theme.button.destructive.filled.idle.bg);
        assert_eq!(style.foreground, theme.button.destructive.filled.idle.fg);
        assert_eq!(style.border, theme.button.destructive.filled.idle.border);
    }

    #[test]
    fn button_variant_keeps_role_treatment_and_shape_separate() {
        let variant = ButtonVariant::new(
            ButtonRole::Suggested,
            ButtonTreatment::Flat,
            ButtonShape::Circular,
        );

        assert_eq!(variant.role, ButtonRole::Suggested);
        assert_eq!(variant.treatment, ButtonTreatment::Flat);
        assert_eq!(variant.shape, ButtonShape::Circular);
        assert_eq!(
            ButtonVariant::SUGGESTED.set_pill(),
            ButtonVariant::new(
                ButtonRole::Suggested,
                ButtonTreatment::Filled,
                ButtonShape::Pill
            )
        );
    }

    #[test]
    fn variant_helpers_can_compose_role_treatment_and_shape() {
        let variant = ButtonVariant::STANDARD
            .set_suggested()
            .set_flat()
            .set_circular();

        assert_eq!(variant.role, ButtonRole::Suggested);
        assert_eq!(variant.treatment, ButtonTreatment::Flat);
        assert_eq!(variant.shape, ButtonShape::Circular);
    }

    #[test]
    fn flat_treatment_uses_flat_theme_tokens() {
        let theme = ThemePack::adwaita();
        let variant = ButtonVariant::SUGGESTED.set_flat();
        let style = ButtonResolvedStyle::from_theme(&theme, variant, ButtonStyleState::Idle);

        assert_eq!(style.foreground, theme.button.suggested.flat.idle.fg);
        assert_eq!(style.background, theme.button.suggested.flat.idle.bg);
        assert_eq!(style.border, theme.button.suggested.flat.idle.border);
        assert_eq!(style.shadow, theme.button.suggested.flat.shadow);
    }

    #[test]
    fn raised_treatment_uses_raised_theme_tokens() {
        let theme = ThemePack::adwaita();
        let variant = ButtonVariant::SUGGESTED.set_raised();
        let style = ButtonResolvedStyle::from_theme(&theme, variant, ButtonStyleState::Idle);

        assert_eq!(style.background, theme.button.suggested.raised.idle.bg);
        assert_eq!(style.border, theme.button.suggested.raised.idle.border);
        assert_eq!(style.shadow, theme.button.suggested.raised.shadow);
    }

    #[test]
    fn pill_and_circular_shape_use_capsule_radius() {
        let theme = ThemePack::adwaita();

        for variant in [
            ButtonVariant::STANDARD.set_pill(),
            ButtonVariant::STANDARD.set_circular(),
        ] {
            let style = ButtonResolvedStyle::from_theme(&theme, variant, ButtonStyleState::Idle);

            assert!(
                style.radius.length().value() > theme.button.shape.rounded.radius.length().value()
            );
        }
    }

    #[test]
    fn flat_circular_keeps_flat_treatment_and_capsule_shape() {
        let theme = ThemePack::adwaita();
        let variant = ButtonVariant::STANDARD.set_flat().set_circular();
        let style = ButtonResolvedStyle::from_theme(&theme, variant, ButtonStyleState::Idle);

        assert_eq!(style.background, theme.button.standard.flat.idle.bg);
        assert_eq!(style.border, theme.button.standard.flat.idle.border);
        assert!(style.radius.length().value() > theme.button.shape.rounded.radius.length().value());
    }

    #[test]
    fn button_shape_and_elevation_come_from_theme() {
        let theme = ThemePack::adwaita();
        let style = ButtonResolvedStyle::from_theme(
            &theme,
            ButtonVariant::STANDARD,
            ButtonStyleState::Idle,
        );

        assert_eq!(style.radius, theme.button.shape.rounded.radius);
        assert_eq!(style.shadow, theme.button.standard.filled.shadow);
        assert!(style.shadow.color().alpha() <= 48);
    }

    #[test]
    fn button_interaction_tokens_come_from_component_theme() {
        let hover = Color::new(221, 238, 255);
        let pressed = Color::new(205, 225, 250);
        let disabled = Color::new(230, 230, 230);
        let disabled_foreground = Color::new_rgba(48, 48, 48, 128);
        let mut theme = ThemePack::adwaita();

        theme.button.standard.filled.hover.bg = hover;
        theme.button.standard.filled.pressed.bg = pressed;
        theme.button.standard.filled.disabled.bg = disabled;
        theme.button.standard.filled.disabled.fg = disabled_foreground;

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

        assert_eq!(idle.background, theme.button.standard.filled.idle.bg);
        assert_eq!(hovered.background, theme.button.standard.filled.hover.bg);
        assert_eq!(pressed.background, theme.button.standard.filled.pressed.bg);
        assert_eq!(hovered.foreground, theme.button.standard.filled.hover.fg);
    }

    #[test]
    fn resolved_button_style_uses_scoped_context() {
        let context = ThemeContext::from_theme(&ThemePack::adwaita());
        let scoped_bg = Color::new(221, 238, 255);
        let scoped = context.scoped(|theme| theme.button.standard.filled.hover.bg = scoped_bg);

        let style = ButtonResolvedStyle::from_context(
            &scoped,
            ButtonVariant::STANDARD,
            ButtonStyleState::Hovered,
        );

        assert_eq!(style.background, scoped_bg);
        assert_ne!(context.theme().button.standard.filled.hover.bg, scoped_bg);
    }

    #[test]
    fn resolved_button_style_uses_component_context() {
        let scoped_bg = Color::new(221, 238, 255);
        let context = ComponentContext::current()
            .scoped_theme(|theme| theme.button.standard.filled.hover.bg = scoped_bg);

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

        assert_eq!(
            disabled.background,
            theme.button.suggested.filled.disabled.bg
        );
        assert_eq!(
            disabled.foreground,
            theme.button.suggested.filled.disabled.fg
        );
        assert_eq!(disabled.radius, theme.button.shape.rounded.radius);
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
