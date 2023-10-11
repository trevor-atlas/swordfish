use tauri::AppHandle;

use tauri::{utils::config::WindowUrl, window::WindowBuilder, Manager};

use crate::constants::{DEFAULT_HEIGHT, DEFAULT_WIDTH, MAIN_WINDOW_HANDLE, SETTINGS_WINDOW_HANDLE};

#[tauri::command]
pub fn hide_main_window(app: AppHandle) {
    let window = get_main_window(&app);
    let menu_item = app.tray_handle().get_item("toggle");
    if let Ok(_) = window.hide() {
        _ = app.hide();
        _ = menu_item.set_title("Show");
    };
}

#[tauri::command]
pub fn show_main_window(app: AppHandle) {
    let window = get_main_window(&app);
    let menu_item = app.tray_handle().get_item("toggle");
    if let Ok(_) = window.show() {
        _ = app.show();
        _ = window.center();
        _ = window.set_focus();
        _ = menu_item.set_title("Hide")
    };
}

#[tauri::command]
pub fn toggle_main_window(app: AppHandle) {
    let window = get_main_window(&app);
    if let Ok(is_visible) = window.is_visible() {
        if is_visible {
            hide_main_window(app)
        } else {
            show_main_window(app);
        }
    }
}

#[tauri::command]
pub fn hide_settings_window(app: AppHandle) {
    let window = get_settings_window(&app);
    if let Ok(_) = window.hide() {
        // _ = app.hide();
        // _ = menu_item.set_title("Show");
    };
}

#[tauri::command]
pub fn show_settings_window(app: AppHandle) {
    let window = get_settings_window(&app);
    if let Ok(_) = window.show() {
        _ = window.set_focus();
    };
}

#[tauri::command]
pub fn toggle_settings_window(app: AppHandle) {
    let window = get_settings_window(&app);
    if let Ok(is_visible) = window.is_visible() {
        if is_visible {
            hide_settings_window(app)
        } else {
            show_settings_window(app);
        }
    }
}

pub fn get_main_window(app: &AppHandle) -> tauri::Window {
    match app.get_window(MAIN_WINDOW_HANDLE) {
        Some(win) => win,
        None => WindowBuilder::new(app, MAIN_WINDOW_HANDLE, WindowUrl::App("app.html".into()))
            .title("Swordfish")
            .decorations(false)
            .accept_first_mouse(true)
            .visible(false)
            .transparent(true)
            .skip_taskbar(true)
            // .disable_file_drop_handler()
            .inner_size(DEFAULT_WIDTH, DEFAULT_HEIGHT)
            .build()
            .expect("Unable to create searchbar window"),
    }
}

pub fn get_settings_window(app: &AppHandle) -> tauri::Window {
    match app.get_window(SETTINGS_WINDOW_HANDLE) {
        Some(win) => win,
        None => WindowBuilder::new(
            app,
            SETTINGS_WINDOW_HANDLE,
            WindowUrl::App("settings.html".into()),
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
