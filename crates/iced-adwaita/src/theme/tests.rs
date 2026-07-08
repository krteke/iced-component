use spectrum_theme::Color;

use super::{ADWAITA_DARK_TOML, ADWAITA_LIGHT_TOML, ThemePack};

#[test]
fn embedded_light_theme_loads_from_toml() {
    let theme = ThemePack::try_from_toml(ADWAITA_LIGHT_TOML).unwrap();

    assert_eq!(theme.app.window.bg, "#fafafb".parse::<Color>().unwrap());
    assert_eq!(theme.app.window.fg, "#000006cc".parse::<Color>().unwrap());
    assert_eq!(theme.app.view.bg, "#ffffff".parse::<Color>().unwrap());
    assert_eq!(theme.app.view.fg, "#000006cc".parse::<Color>().unwrap());
    assert_eq!(theme.spinner.color, "#00000670".parse::<Color>().unwrap());
    assert!((theme.spinner.size.value() - 16.0).abs() < 0.001);
}

#[test]
fn embedded_dark_theme_loads_from_toml() {
    let theme = ThemePack::try_from_toml(ADWAITA_DARK_TOML).unwrap();

    assert_eq!(theme.app.window.bg, "#222226".parse::<Color>().unwrap());
    assert_eq!(theme.app.window.fg, "#ffffff".parse::<Color>().unwrap());
    assert_eq!(theme.app.view.bg, "#1d1d20".parse::<Color>().unwrap());
    assert_eq!(theme.app.view.fg, "#ffffff".parse::<Color>().unwrap());
    assert_eq!(theme.spinner.color, "#ffffffe6".parse::<Color>().unwrap());
    assert!((theme.spinner.size.value() - 16.0).abs() < 0.001);
}
