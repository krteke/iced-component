use float_cmp::assert_approx_eq;
use spectrum_theme::Color;

use super::{ADWAITA_DARK_TOML, ADWAITA_LIGHT_TOML, ThemePack};

#[test]
fn embedded_light_theme_loads_from_toml() {
    let theme = ThemePack::try_from_toml(ADWAITA_LIGHT_TOML).unwrap();

    assert_eq!(theme.app.window.bg, "#fafafb".parse::<Color>().unwrap());
    assert_eq!(theme.app.window.fg, "#000006cc".parse::<Color>().unwrap());
    assert_eq!(theme.app.view.bg, "#ffffff".parse::<Color>().unwrap());
    assert_eq!(theme.app.view.fg, "#000006cc".parse::<Color>().unwrap());
    assert_eq!(
        theme.spinner.foreground,
        "#8c8c90".parse::<Color>().unwrap()
    );
    assert_eq!(theme.spinner.track, "#eaeaeb".parse::<Color>().unwrap());
    assert_eq!(theme.accent.bg, "#3584e4".parse::<Color>().unwrap());
    assert_eq!(
        theme.button.standard.idle.bg,
        "#00000614".parse::<Color>().unwrap()
    );
    assert_approx_eq!(f32, theme.button.standard.idle.radius.length().value(), 9.0);
    assert!((theme.spinner.size.value() - 16.0).abs() < 0.001);
}

#[test]
fn embedded_dark_theme_loads_from_toml() {
    let theme = ThemePack::try_from_toml(ADWAITA_DARK_TOML).unwrap();

    assert_eq!(theme.app.window.bg, "#222226".parse::<Color>().unwrap());
    assert_eq!(theme.app.window.fg, "#ffffff".parse::<Color>().unwrap());
    assert_eq!(theme.app.view.bg, "#1d1d20".parse::<Color>().unwrap());
    assert_eq!(theme.app.view.fg, "#ffffff".parse::<Color>().unwrap());
    assert_eq!(
        theme.spinner.foreground,
        "#f0f0f0".parse::<Color>().unwrap()
    );
    assert_eq!(theme.spinner.track, "#3d3d42".parse::<Color>().unwrap());
    assert_eq!(theme.accent.color, "#99c1f1".parse::<Color>().unwrap());
    assert_eq!(
        theme.button.standard.pressed.bg,
        "#ffffff4d".parse::<Color>().unwrap()
    );
    assert_approx_eq!(f32, theme.button.padding_x.value(), 17.0);
    assert!((theme.spinner.size.value() - 16.0).abs() < 0.001);
}
