use serde_variant::to_variant_name;
use swordfish_types::SFEvent;
use tauri::{AppHandle, Emitter};

use tauri::{Manager, WebviewWindowBuilder};

use crate::constants::{DEFAULT_HEIGHT, DEFAULT_WIDTH, MAIN_WINDOW_HANDLE, SETTINGS_WINDOW_HANDLE};

#[tauri::command]
pub fn hide_main_window(app: AppHandle) {
    let window = acquire_main_window(&app);
    println!("Hide main window");
    _ = app.hide();
    _ = window.hide();
}

#[tauri::command]
pub fn show_main_window(app: AppHandle) {
    let window = acquire_main_window(&app);
    println!("Show main window");
    _ = app.show();
    _ = window.show();

    _ = window.center();
    _ = window.set_focus();
}

#[tauri::command]
pub fn toggle_main_window(app: AppHandle) {
    let window = acquire_main_window(&app);
    if window.is_visible().unwrap_or(false) {
        hide_main_window(app)
    } else {
        show_main_window(app);
    }
}

#[tauri::command]
pub fn hide_settings_window(app: AppHandle) {
    let window = acquire_settings_window(&app);
    if window.hide().is_ok() {
        app.emit(to_variant_name(&SFEvent::SettingsWindowHidden).unwrap(), ())
            .ok();
    };
}

#[tauri::command]
pub fn show_settings_window(app: AppHandle) {
    let window = acquire_settings_window(&app);
    if window.show().is_ok() {
        _ = window.set_focus();
        app.emit(to_variant_name(&SFEvent::SettingsWindowShown).unwrap(), ())
            .ok();
    };
}

#[tauri::command]
pub fn toggle_settings_window(app: AppHandle) {
    let window = acquire_settings_window(&app);
    if window.is_visible().unwrap_or(false) {
        hide_settings_window(app)
    } else {
        show_settings_window(app);
    }
}

pub fn acquire_main_window(app: &tauri::AppHandle) -> tauri::WebviewWindow {
    match app.get_webview_window(MAIN_WINDOW_HANDLE) {
        Some(window) => window,
        None => WebviewWindowBuilder::new(
            app,
            MAIN_WINDOW_HANDLE,
            tauri::WebviewUrl::App("app.html".into()),
        )
        .title("Swordfish")
        .decorations(false)
        .accept_first_mouse(true)
        .visible(false)
        .transparent(true)
        .skip_taskbar(true)
        .inner_size(DEFAULT_WIDTH, DEFAULT_HEIGHT)
        .build()
        .expect("Unable to create searchbar window"),
    }
}

pub fn acquire_settings_window(app: &tauri::AppHandle) -> tauri::WebviewWindow {
    match app.get_webview_window(SETTINGS_WINDOW_HANDLE) {
        Some(win) => win,
        None => WebviewWindowBuilder::new(
            app,
            SETTINGS_WINDOW_HANDLE,
            tauri::WebviewUrl::App("settings.html".into()),
        )
        .title("Swordfish — Settings")
        // .decorations(false)
        .accept_first_mouse(true)
        .visible(false)
        // .transparent(true)
        // .skip_taskbar(true)
        // .disable_file_drop_handler()
        .inner_size(750.0, 750.0)
        .build()
        .expect("Unable to create searchbar window"),
    }
}
