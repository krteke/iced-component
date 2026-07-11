use crate::theme::tokens::{
    ButtonElevatedState, ButtonFilledState, ButtonFilledTonalState, ButtonOutlinedState,
    ButtonTextState, ThemePack,
};

/// Material button visual variant.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ButtonVariant {
    /// Raised container using the surface and primary roles.
    #[default]
    Elevated,
    /// Prominent primary-filled action.
    Filled,
    /// Secondary-container filled action.
    FilledTonal,
    /// Transparent action with an outline.
    Outlined,
    /// Transparent text-only action.
    Text,
}

impl ButtonVariant {
    /// Elevated Material button variant.
    pub const ELEVATED: Self = Self::Elevated;
    /// Filled Material button variant.
    pub const FILLED: Self = Self::Filled;
    /// Filled tonal Material button variant.
    pub const FILLED_TONAL: Self = Self::FilledTonal;
    /// Outlined Material button variant.
    pub const OUTLINED: Self = Self::Outlined;
    /// Text Material button variant.
    pub const TEXT: Self = Self::Text;
}

/// Interaction state used to resolve Material button tokens.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ButtonStyleState {
    /// Resting state.
    Idle,
    /// Pointer hover state.
    Hover,
    /// Pressed state. Ripple animation is added separately.
    Pressed,
    /// Keyboard focus state.
    Focus,
    /// Disabled state.
    Disabled,
}

/// Rendering snapshot for a Material button.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ButtonSnapshot {
    /// Current logical interaction state.
    pub style_state: ButtonStyleState,
    /// Current interpolated visual values.
    pub visual: super::ButtonVisual,
    /// Whether interactions are disabled.
    pub disabled: bool,
    /// Whether keyboard focus is active.
    pub focused: bool,
    /// Pressed-state layer color used by the active ripple.
    pub(crate) ripple_color: spectrum_theme::Color,
    /// Pressed-state layer opacity used by the active ripple.
    pub(crate) ripple_opacity: f32,
}

pub(crate) fn visual_from_theme(
    theme: &ThemePack,
    variant: ButtonVariant,
    state: ButtonStyleState,
) -> super::ButtonVisual {
    let tokens = match variant {
        ButtonVariant::Elevated => theme.button.elevated.get(elevated_state(state)),
        ButtonVariant::Filled => theme.button.filled.get(filled_state(state)),
        ButtonVariant::FilledTonal => theme.button.filled_tonal.get(filled_tonal_state(state)),
        ButtonVariant::Outlined => theme.button.outlined.get(outlined_state(state)),
        ButtonVariant::Text => theme.button.text.get(text_state(state)),
    };

    super::ButtonVisual::from_tokens(*tokens)
}

const fn elevated_state(state: ButtonStyleState) -> ButtonElevatedState {
    match state {
        ButtonStyleState::Idle => ButtonElevatedState::Idle,
        ButtonStyleState::Hover => ButtonElevatedState::Hover,
        ButtonStyleState::Pressed => ButtonElevatedState::Pressed,
        ButtonStyleState::Focus => ButtonElevatedState::Focus,
        ButtonStyleState::Disabled => ButtonElevatedState::Disabled,
    }
}

const fn filled_state(state: ButtonStyleState) -> ButtonFilledState {
    match state {
        ButtonStyleState::Idle => ButtonFilledState::Idle,
        ButtonStyleState::Hover => ButtonFilledState::Hover,
        ButtonStyleState::Pressed => ButtonFilledState::Pressed,
        ButtonStyleState::Focus => ButtonFilledState::Focus,
        ButtonStyleState::Disabled => ButtonFilledState::Disabled,
    }
}

const fn filled_tonal_state(state: ButtonStyleState) -> ButtonFilledTonalState {
    match state {
        ButtonStyleState::Idle => ButtonFilledTonalState::Idle,
        ButtonStyleState::Hover => ButtonFilledTonalState::Hover,
        ButtonStyleState::Pressed => ButtonFilledTonalState::Pressed,
        ButtonStyleState::Focus => ButtonFilledTonalState::Focus,
        ButtonStyleState::Disabled => ButtonFilledTonalState::Disabled,
    }
}

const fn outlined_state(state: ButtonStyleState) -> ButtonOutlinedState {
    match state {
        ButtonStyleState::Idle => ButtonOutlinedState::Idle,
        ButtonStyleState::Hover => ButtonOutlinedState::Hover,
        ButtonStyleState::Pressed => ButtonOutlinedState::Pressed,
        ButtonStyleState::Focus => ButtonOutlinedState::Focus,
        ButtonStyleState::Disabled => ButtonOutlinedState::Disabled,
    }
}

const fn text_state(state: ButtonStyleState) -> ButtonTextState {
    match state {
        ButtonStyleState::Idle => ButtonTextState::Idle,
        ButtonStyleState::Hover => ButtonTextState::Hover,
        ButtonStyleState::Pressed => ButtonTextState::Pressed,
        ButtonStyleState::Focus => ButtonTextState::Focus,
        ButtonStyleState::Disabled => ButtonTextState::Disabled,
    }
}
