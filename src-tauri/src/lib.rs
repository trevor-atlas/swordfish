// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

mod app_state;
mod browser_data_source;
mod constants;
mod file_data_source;
mod query_engine;
mod settings;
mod sqlite;
mod tray;
mod utilities;
mod windows;

use crate::windows::{
    acquire_main_window, acquire_settings_window, hide_main_window, hide_settings_window,
    show_main_window, show_settings_window, toggle_main_window, toggle_settings_window,
};
use app_state::AppState;
use query_engine::{QueryEngine, QueryInterface};
use serde_variant::to_variant_name;
use std::env;
use std::str::FromStr;
use swordfish_types::SFEvent;
use tauri::{AppHandle, Emitter, Listener, Manager};
use tauri_plugin_global_shortcut::{Shortcut, ShortcutState};
use tracing::{error, info};

#[tokio::main]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    tauri::async_runtime::set(tokio::runtime::Handle::current());
    let query_engine = QueryEngine::new();

    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_http::init())
        .invoke_handler(tauri::generate_handler![
            show_main_window,
            hide_main_window,
            toggle_main_window,
            show_settings_window,
            hide_settings_window,
            toggle_settings_window,
        ])
        // .system_tray(make_tray())
        // .on_system_tray_event(handle_tray_event)
        .manage(AppState::new())
        .setup(move |app| {
            #[cfg(target_os = "macos")]
            {
                app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            }
            #[cfg(all(desktop))]
            {
                let handle = app.handle();
                tray::create_tray(handle)?;
            }

            let app_handle = app.app_handle();
            // QueryEngine::start_ipc_server(&app_handle);

            _ = acquire_settings_window(&app_handle);
            let main_window = acquire_main_window(&app_handle);
            main_window.set_always_on_top(true)?;

            if env::var_os("SWORDFISH_DEV").is_some() && !main_window.is_devtools_open() {
                main_window.open_devtools();
            }

            let config = app
                .state::<AppState>()
                .config
                .lock()
                .map(|config| config.read())
                .expect("Unable to read config");
            println!("{}", config.launch_shortcut);
            app.handle().plugin(
                tauri_plugin_global_shortcut::Builder::new()
                    .with_shortcuts(["ctrl+space"])?
                    .with_handler(move |app_handle, shortcut, event| {
                        if event.state == ShortcutState::Released {
                            let launch =
                                Shortcut::from_str(config.launch_shortcut.as_str()).unwrap();
                            println!("shortcut: {}, launch: {}", shortcut, launch);
                            toggle_main_window(app_handle.to_owned());
                        }
                    })
                    .build(),
            )?;

            let emitter = app_handle.clone();
            let _id = app_handle.listen(to_variant_name(&SFEvent::Query).unwrap(), move |event| {
                let str = event.payload();
                if let Ok(query) = serde_json::from_str(str) {
                    let res = query_engine.query(query);
                    let _ = emitter.emit(to_variant_name(&SFEvent::QueryResult).unwrap(), res);
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("App crashed");
}
