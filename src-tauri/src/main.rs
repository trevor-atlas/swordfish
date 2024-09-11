// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app_state;
mod browser_data_source;
mod constants;
mod file_data_source;
mod input_handler;
mod query_engine;
mod settings;
mod sqlite;
mod tray;
mod utilities;
mod windows;

use crate::{
    tray::{handle_tray_event, make_tray},
    windows::{
        acquire_main_window, acquire_settings_window, hide_main_window, hide_settings_window,
        show_main_window, show_settings_window, toggle_main_window, toggle_settings_window,
    },
};
use app_state::AppState;
use input_handler::handle_shortcuts;
use query_engine::{QueryEngine, QueryInterface};
use serde_variant::to_variant_name;
use std::env;
use swordfish_types::SFEvent;
use tauri::Manager;
use tracing::{error, info};

#[tokio::main]
async fn main() {
    tauri::async_runtime::set(tokio::runtime::Handle::current());
    let query_engine = QueryEngine::new();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            show_main_window,
            hide_main_window,
            toggle_main_window,
            show_settings_window,
            hide_settings_window,
            toggle_settings_window,
        ])
        .system_tray(make_tray())
        .on_system_tray_event(handle_tray_event)
        .manage(AppState::new())
        .setup(move |app| {
            #[cfg(target_os = "macos")]
            {
                app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            }
            let app_handle = app.app_handle();
            QueryEngine::start_ipc_server(&app_handle);

            let main_window = acquire_main_window(&app_handle);
            main_window.set_always_on_top(true)?;

            // let sw = get_script_window(&app_handle, "test");
            if env::var_os("SWORDFISH_DEV").is_some() && !main_window.is_devtools_open() {
                // main_window.open_devtools();
                // sw.open_devtools();
            }

            acquire_settings_window(&app_handle);
            handle_shortcuts(app);

            let emitter = app_handle.clone();
            let _id =
                app_handle.listen_global(to_variant_name(&SFEvent::Query).unwrap(), move |event| {
                    if let Some(str) = event.payload() {
                        if let Ok(query) = serde_json::from_str(str) {
                            let res = query_engine.query(query);
                            let _ = emitter
                                .emit_all(to_variant_name(&SFEvent::QueryResult).unwrap(), res);
                        }
                    }
                });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("App crashed");
}
