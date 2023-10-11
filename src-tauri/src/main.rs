// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use query::{Query, QueryResult, QueryResultPreview, QueryResultType};
use tauri::{
    utils::config::WindowUrl, window::WindowBuilder, AppHandle, CustomMenuItem, Manager,
    SystemTray, SystemTrayEvent, SystemTrayMenu,
};
use tray::{handle_tray_event, make_tray};
use windows::{get_main_window, get_settings_window};

use crate::{
    query::get_query_result,
    windows::{
        hide_main_window, hide_settings_window, show_main_window, show_settings_window,
        toggle_main_window, toggle_settings_window,
    },
};

mod constants;
mod query;
mod tray;
mod windows;

// fn handle_shortcuts(mut app: &App) {
//     let mut gsm = app.global_shortcut_manager();
//     let h = app.app_handle();
//     let w = get_main_window(&h);
//     match gsm.register("CommandOrControl+k", move || {
//         if let Ok(bool) = w.is_visible() {
//             if bool {
//                 println!("hide window");
//                 hide_main_window(h.clone());
//             } else {
//                 println!("show window");
//                 show_main_window(h.clone());
//             }
//         }
//     }) {
//         Ok(_) => println!("registered the shortcut successfully"),
//         Err(e) => println!("Error registering global shortcut: {}", e),
//     };
// }

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_query_result,
            show_main_window,
            hide_main_window,
            toggle_main_window,
            show_settings_window,
            hide_settings_window,
            toggle_settings_window
        ])
        .system_tray(make_tray())
        .on_system_tray_event(handle_tray_event)
        .setup(move |app| {
            let app_handle = app.app_handle();
            let main_window = get_main_window(&app_handle);
            main_window.set_always_on_top(true)?;

            let settings_window = get_settings_window(&app_handle);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("App crashed");
}
