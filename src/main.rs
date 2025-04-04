#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

use winapi::{
    shared::minwindef::{LPARAM, WPARAM},
    um::winuser::{SendMessageTimeoutA, HWND_BROADCAST, SMTO_ABORTIFHUNG, WM_SETTINGCHANGE},
};
use winreg::{enums::HKEY_CURRENT_USER, RegKey};

fn get_personalize_path() -> std::path::PathBuf {
    std::path::Path::new("SOFTWARE")
        .join("Microsoft")
        .join("Windows")
        .join("CurrentVersion")
        .join("Themes")
        .join("Personalize")
}

fn is_current_theme_dark() -> bool {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = get_personalize_path();
    let (key, _disp) = hkcu.create_subkey(&path).unwrap();
    let value: u32 = key.get_value("SystemUsesLightTheme").unwrap();

    value == 0
}

fn send_settings_change_event() {
    unsafe {
        let success = SendMessageTimeoutA(
            HWND_BROADCAST,
            WM_SETTINGCHANGE,
            0 as WPARAM,
            "ImmersiveColorSet\0".as_ptr() as LPARAM,
            SMTO_ABORTIFHUNG,
            5000,
            std::ptr::null_mut(),
        );

        if success == 0 {
            println!("Failed to send WM_SETTINGCHANGE message.");
            return;
        }
        println!("Sent WM_SETTINGCHANGE message successfully.");
    }
}

fn swith_theme(dark: bool) {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = get_personalize_path();
    let (key, _disp) = hkcu.create_subkey(&path).unwrap();

    key.set_value("AppsUseLightTheme", &if dark { 0u32 } else { 1u32 })
        .unwrap();
    key.set_value("SystemUsesLightTheme", &if dark { 0u32 } else { 1u32 })
        .unwrap();

    send_settings_change_event();
}

fn main() {
    swith_theme(!is_current_theme_dark());
}
