use spectrum_theme::Color;

use super::{LoadingIndicator, LoadingIndicatorMode, LoadingIndicatorStyle};

impl LoadingIndicator {
    /// Returns this indicator with a different progress mode.
    #[must_use]
    pub const fn with_mode(mut self, mode: LoadingIndicatorMode) -> Self {
        self.mode = mode;
        self
    }

    /// Returns this indicator with determinate progress.
    #[must_use]
    pub const fn with_progress(self, progress: f32) -> Self {
        self.with_mode(LoadingIndicatorMode::Determinate(progress))
    }

    /// Returns this indicator in indeterminate mode.
    #[must_use]
    pub const fn indeterminate(self) -> Self {
        self.with_mode(LoadingIndicatorMode::Indeterminate)
    }

    /// Sets the progress mode and resets the indeterminate timeline.
    pub fn set_mode(&mut self, mode: LoadingIndicatorMode) {
        if self.mode != mode {
            self.mode = mode;
            self.timeline.reset();
        }
    }

    /// Switches to determinate progress, clamped while rendering.
    pub fn set_progress(&mut self, progress: f32) {
        self.set_mode(LoadingIndicatorMode::Determinate(progress));
    }

    /// Switches back to the indeterminate shape sequence.
    pub fn set_indeterminate(&mut self) {
        self.set_mode(LoadingIndicatorMode::Indeterminate);
    }

    /// Returns the configured progress mode.
    #[must_use]
    pub const fn mode(&self) -> LoadingIndicatorMode {
        self.mode
    }

    /// Returns determinate progress when configured.
    #[must_use]
    pub fn progress(&self) -> Option<f32> {
        match self.mode {
            LoadingIndicatorMode::Indeterminate => None,
            LoadingIndicatorMode::Determinate(progress) => Some(progress.clamp(0.0, 1.0)),
        }
    }

    /// Returns whether this indicator is indeterminate.
    #[must_use]
    pub const fn is_indeterminate(&self) -> bool {
        matches!(self.mode, LoadingIndicatorMode::Indeterminate)
    }

    /// Returns this indicator with a circular color container.
    #[must_use]
    pub const fn contained(self) -> Self {
        self.with_contained(true)
    }

    /// Returns this indicator without a circular color container.
    #[must_use]
    pub const fn uncontained(self) -> Self {
        self.with_contained(false)
    }

    /// Returns this indicator with an explicit containment mode.
    #[must_use]
    pub const fn with_contained(mut self, contained: bool) -> Self {
        self.contained = contained;
        self
    }

    /// Sets whether a circular color container is rendered.
    pub fn set_contained(&mut self, contained: bool) {
        self.contained = contained;
    }

    /// Returns whether the color container is enabled.
    #[must_use]
    pub const fn is_contained(&self) -> bool {
        self.contained
    }

    /// Returns this indicator with an explicit square size.
    #[must_use]
    pub const fn size(mut self, size: f32) -> Self {
        self.size = Some(size);
        self
    }

    /// Sets an explicit square size.
    pub fn set_size(&mut self, size: f32) {
        self.size = Some(size);
    }

    /// Clears the instance size override.
    pub fn clear_size(&mut self) {
        self.size = None;
    }

    /// Returns the explicit size override.
    #[must_use]
    pub const fn explicit_size(&self) -> Option<f32> {
        self.size
    }

    /// Returns this indicator with instance-level style overrides.
    #[must_use]
    pub const fn with_style(mut self, style: LoadingIndicatorStyle) -> Self {
        self.style = style;
        self
    }

    /// Replaces the instance-level style overrides.
    pub fn set_style(&mut self, style: LoadingIndicatorStyle) {
        self.style = style;
    }

    /// Clears every instance-level color override.
    pub fn clear_style(&mut self) {
        self.style = LoadingIndicatorStyle::new();
    }

    /// Returns the current instance-level style overrides.
    #[must_use]
    pub const fn style(&self) -> LoadingIndicatorStyle {
        self.style
    }

    /// Returns mutable instance-level style overrides.
    pub fn style_mut(&mut self) -> &mut LoadingIndicatorStyle {
        &mut self.style
    }

    /// Returns this indicator with an active shape color override.
    #[must_use]
    pub const fn active_color(mut self, color: Color) -> Self {
        self.style.active = Some(color);
        self
    }

    /// Sets the uncontained active shape color override.
    pub fn set_active_color(&mut self, color: Color) {
        self.style.active = Some(color);
    }

    /// Clears the uncontained active shape color override.
    pub fn clear_active_color(&mut self) {
        self.style.active = None;
    }

    /// Returns the uncontained active shape color override.
    #[must_use]
    pub const fn active_color_override(&self) -> Option<Color> {
        self.style.active
    }

    /// Returns this indicator with a contained background override.
    #[must_use]
    pub const fn container_color(mut self, color: Color) -> Self {
        self.style.container = Some(color);
        self
    }

    /// Sets the contained circular background color override.
    pub fn set_container_color(&mut self, color: Color) {
        self.style.container = Some(color);
    }

    /// Clears the contained circular background color override.
    pub fn clear_container_color(&mut self) {
        self.style.container = None;
    }

    /// Returns the contained circular background color override.
    #[must_use]
    pub const fn container_color_override(&self) -> Option<Color> {
        self.style.container
    }

    /// Returns this indicator with a contained active shape override.
    #[must_use]
    pub const fn contained_active_color(mut self, color: Color) -> Self {
        self.style.contained_active = Some(color);
        self
    }

    /// Sets the active shape color used inside the container.
    pub fn set_contained_active_color(&mut self, color: Color) {
        self.style.contained_active = Some(color);
    }

    /// Clears the contained active shape color override.
    pub fn clear_contained_active_color(&mut self) {
        self.style.contained_active = None;
    }

    /// Returns the contained active shape color override.
    #[must_use]
    pub const fn contained_active_color_override(&self) -> Option<Color> {
        self.style.contained_active
    }
}
