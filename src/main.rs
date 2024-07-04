use std::path::Path;

use winreg::{enums::HKEY_CURRENT_USER, RegKey};

fn is_current_theme_light() -> bool {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = Path::new("SOFTWARE")
        .join("Microsoft")
        .join("Windows")
        .join("CurrentVersion")
        .join("Themes")
        .join("Personalize");

    let (key, _disp) = hkcu.create_subkey(&path).unwrap();

    let value: u32 = key.get_value("SystemUsesLightTheme").unwrap();

    value == 1
}

fn swith_theme(dark: bool) {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = Path::new("SOFTWARE")
        .join("Microsoft")
        .join("Windows")
        .join("CurrentVersion")
        .join("Themes")
        .join("Personalize");

    let (key, _disp) = hkcu.create_subkey(&path).unwrap();

    key.set_value("SystemUsesLightTheme", &if dark { 1u32 } else { 0u32 })
        .unwrap();
    key.set_value("AppsUseLightTheme", &if dark { 1u32 } else { 0u32 })
        .unwrap();
}

fn main() {
    let is_light = is_current_theme_light();
    swith_theme(!is_light);
}
