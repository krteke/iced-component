use spectrum_theme::Color;

use super::{ADWAITA_LIGHT_TOML, ThemePack};

#[test]
fn embedded_theme_loads_from_toml() {
    let theme = ThemePack::try_from_toml(ADWAITA_LIGHT_TOML).unwrap();

    assert_eq!(theme.app.bg, "#f6f5f4".parse::<Color>().unwrap());
    assert_eq!(
        theme.button.standard_filled.idle.bg,
        theme.surface.raised.idle.bg
    );
    assert_eq!(theme.spinner.color, "#8c8c90".parse::<Color>().unwrap());
    assert!((theme.spinner.size.value() - 16.0).abs() < 0.001);
}
