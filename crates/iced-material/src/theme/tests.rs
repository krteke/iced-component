use spectrum_theme::Color;

use super::{MATERIAL_DARK_TOML, MATERIAL_LIGHT_TOML, ThemePack};

#[test]
fn embedded_light_theme_loads_material_color_roles() {
    let theme = ThemePack::try_from_toml(MATERIAL_LIGHT_TOML).unwrap();

    assert_eq!(
        theme.palette.primary.color,
        "#6750a4".parse::<Color>().unwrap()
    );
    assert_eq!(
        theme.palette.primary.on_container,
        "#4f378a".parse::<Color>().unwrap()
    );
    assert_ne!(
        theme.palette.surface.container.low,
        theme.palette.primary.color
    );
    assert!((theme.button.container_height.value() - 40.0).abs() < f32::EPSILON);
}

#[test]
fn embedded_dark_theme_loads_material_button_states() {
    let theme = ThemePack::try_from_toml(MATERIAL_DARK_TOML).unwrap();

    assert_eq!(
        theme.palette.primary.color,
        "#cfbcff".parse::<Color>().unwrap()
    );
    assert_eq!(
        theme.button.filled.idle.background,
        theme.palette.primary.color
    );
    assert_eq!(
        theme.button.outlined.idle.border,
        theme.palette.outline.color
    );
}
