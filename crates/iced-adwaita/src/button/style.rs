use spectrum_theme::{Color, Length, Radius};

use crate::theme::tokens::{
    ButtonDestructiveState, ButtonFlatState, ButtonStandardState, ButtonSuggestedState,
    ButtonTokens, ThemePack,
};

/// Semantic role for resolving Adwaita button style.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ButtonRole {
    /// Neutral action, matching a regular Adwaita `button`.
    Standard,
    /// Recommended action, matching `.suggested-action`.
    Suggested,
    /// Dangerous or destructive action, matching `.destructive-action`.
    Destructive,
}

/// Visual treatment for an Adwaita button.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ButtonTreatment {
    /// Regular raised-looking button surface.
    Regular,
    /// Minimal flat treatment, matching `.flat`.
    Flat,
}

/// Button outline shape.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ButtonShape {
    /// Rounded rectangle shape using Adwaita's regular button radius.
    Rounded,
    /// Fully rounded capsule shape, matching `.pill`.
    Pill,
    /// Fully rounded icon-style shape, matching `.circular`.
    Circular,
}

/// Complete style semantics for a button.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ButtonVariant {
    /// Semantic role.
    pub role: ButtonRole,
    /// Visual treatment.
    pub treatment: ButtonTreatment,
    /// Outline shape.
    pub shape: ButtonShape,
}

impl ButtonVariant {
    /// Standard Adwaita button variant.
    pub const STANDARD: Self = Self::new(
        ButtonRole::Standard,
        ButtonTreatment::Regular,
        ButtonShape::Rounded,
    );
    /// Suggested action variant.
    pub const SUGGESTED: Self = Self::new(
        ButtonRole::Suggested,
        ButtonTreatment::Regular,
        ButtonShape::Rounded,
    );
    /// Destructive action variant.
    pub const DESTRUCTIVE: Self = Self::new(
        ButtonRole::Destructive,
        ButtonTreatment::Regular,
        ButtonShape::Rounded,
    );

    /// Creates a button variant.
    #[must_use]
    pub const fn new(role: ButtonRole, treatment: ButtonTreatment, shape: ButtonShape) -> Self {
        Self {
            role,
            treatment,
            shape,
        }
    }

    /// Returns this variant with a different role.
    #[must_use]
    pub const fn with_role(mut self, role: ButtonRole) -> Self {
        self.role = role;
        self
    }

    /// Returns this variant with a different treatment.
    #[must_use]
    pub const fn with_treatment(mut self, treatment: ButtonTreatment) -> Self {
        self.treatment = treatment;
        self
    }

    /// Returns this variant with a different shape.
    #[must_use]
    pub const fn with_shape(mut self, shape: ButtonShape) -> Self {
        self.shape = shape;
        self
    }
}

impl Default for ButtonVariant {
    fn default() -> Self {
        Self::STANDARD
    }
}

/// Instance-level button visual overrides.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ButtonStyleOverride {
    /// Background color override.
    pub background: Option<Color>,
    /// Foreground color override.
    pub foreground: Option<Color>,
    /// Border color override.
    pub border: Option<Color>,
    /// Border width override.
    pub border_width: Option<Length>,
    /// Radius override.
    pub radius: Option<Radius>,
    /// Focus ring color override.
    pub focus_ring: Option<Color>,
}

impl ButtonStyleOverride {
    /// Returns whether this override changes no visual field.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.background.is_none()
            && self.foreground.is_none()
            && self.border.is_none()
            && self.border_width.is_none()
            && self.radius.is_none()
            && self.focus_ring.is_none()
    }

    pub(crate) fn apply(self, mut tokens: ButtonTokens) -> ButtonTokens {
        if let Some(background) = self.background {
            tokens.bg = background;
        }
        if let Some(foreground) = self.foreground {
            tokens.fg = foreground;
        }
        if let Some(border) = self.border {
            tokens.border = border;
        }
        if let Some(border_width) = self.border_width {
            tokens.border_width = border_width;
        }
        if let Some(radius) = self.radius {
            tokens.radius = radius;
        }
        if let Some(focus_ring) = self.focus_ring {
            tokens.focus_ring = focus_ring;
        }

        tokens
    }
}

/// Visual state used to resolve Adwaita button tokens.
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

impl ButtonStyleState {
    pub(crate) const fn standard_state(self) -> ButtonStandardState {
        match self {
            Self::Idle => ButtonStandardState::Idle,
            Self::Hovered => ButtonStandardState::Hover,
            Self::Pressed => ButtonStandardState::Pressed,
            Self::Disabled => ButtonStandardState::Disabled,
        }
    }

    const fn flat_state(self) -> ButtonFlatState {
        match self {
            Self::Idle => ButtonFlatState::Idle,
            Self::Hovered => ButtonFlatState::Hover,
            Self::Pressed => ButtonFlatState::Pressed,
            Self::Disabled => ButtonFlatState::Disabled,
        }
    }

    const fn suggested_state(self) -> ButtonSuggestedState {
        match self {
            Self::Idle => ButtonSuggestedState::Idle,
            Self::Hovered => ButtonSuggestedState::Hover,
            Self::Pressed => ButtonSuggestedState::Pressed,
            Self::Disabled => ButtonSuggestedState::Disabled,
        }
    }

    const fn destructive_state(self) -> ButtonDestructiveState {
        match self {
            Self::Idle => ButtonDestructiveState::Idle,
            Self::Hovered => ButtonDestructiveState::Hover,
            Self::Pressed => ButtonDestructiveState::Pressed,
            Self::Disabled => ButtonDestructiveState::Disabled,
        }
    }
}

/// Resolved button style carried by view snapshots.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ButtonResolvedStyle {
    /// Final background color.
    pub background: Color,
    /// Final foreground color.
    pub foreground: Color,
    /// Final border color.
    pub border: Color,
    /// Button border width.
    pub border_width: Length,
    /// Button corner radius.
    pub radius: Radius,
    /// Focus ring color.
    pub focus_ring: Color,
}

impl ButtonResolvedStyle {
    /// Resolves the current standard Adwaita button style from a theme pack.
    #[must_use]
    pub fn standard(theme: &ThemePack, state: ButtonStyleState) -> Self {
        Self::from_theme(
            theme,
            ButtonVariant::STANDARD,
            ButtonStyleOverride::default(),
            state,
        )
    }

    /// Resolves button style from theme, semantics, and instance overrides.
    #[must_use]
    pub fn from_theme(
        theme: &ThemePack,
        variant: ButtonVariant,
        overrides: ButtonStyleOverride,
        state: ButtonStyleState,
    ) -> Self {
        Self::from_tokens(component_tokens_from_theme(
            theme, variant, overrides, state,
        ))
    }

    pub(crate) const fn from_tokens(tokens: ButtonTokens) -> Self {
        Self {
            background: tokens.bg,
            foreground: tokens.fg,
            border: tokens.border,
            border_width: tokens.border_width,
            radius: tokens.radius,
            focus_ring: tokens.focus_ring,
        }
    }
}

pub(crate) fn component_tokens_from_theme(
    theme: &ThemePack,
    variant: ButtonVariant,
    overrides: ButtonStyleOverride,
    state: ButtonStyleState,
) -> ButtonTokens {
    let mut tokens = *match (variant.role, variant.treatment) {
        (_, ButtonTreatment::Flat) => theme.button.flat.get(state.flat_state()),
        (ButtonRole::Standard, ButtonTreatment::Regular) => {
            theme.button.standard.get(state.standard_state())
        }
        (ButtonRole::Suggested, ButtonTreatment::Regular) => {
            theme.button.suggested.get(state.suggested_state())
        }
        (ButtonRole::Destructive, ButtonTreatment::Regular) => {
            theme.button.destructive.get(state.destructive_state())
        }
    };

    if variant.treatment == ButtonTreatment::Flat && state != ButtonStyleState::Disabled {
        tokens.fg = match variant.role {
            ButtonRole::Standard => tokens.fg,
            ButtonRole::Suggested => theme.accent.color,
            ButtonRole::Destructive => theme.button.destructive.idle.fg,
        };
    }

    tokens.radius = match variant.shape {
        ButtonShape::Rounded => theme.button.shape.rounded.radius,
        ButtonShape::Pill => theme.button.shape.pill.radius,
        ButtonShape::Circular => theme.button.shape.circular.radius,
    };

    overrides.apply(tokens)
}

/// Rendering snapshot for the current button state.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ButtonSnapshot {
    /// Current interaction style state.
    pub style_state: ButtonStyleState,
    /// Resolved style for the current frame.
    pub style: ButtonResolvedStyle,
    /// Animated motion value for the current frame.
    pub motion: crate::button::ButtonMotion,
    /// Whether the button is disabled.
    pub disabled: bool,
    /// Whether the button has keyboard focus.
    pub focused: bool,
}
