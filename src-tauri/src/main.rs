// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod browser;
mod constants;
mod datasource;
mod query_engine;
mod search;
mod settings;
mod tray;
mod utilities;
mod windows;

use crate::{
    tray::{handle_tray_event, make_tray},
    utilities::cache_all_app_icons,
    windows::{
        acquire_main_window, acquire_settings_window, hide_main_window, hide_settings_window,
        show_main_window, show_settings_window, toggle_main_window, toggle_settings_window,
    },
};
use query_engine::{QueryEngine, QueryInterface};
use settings::AppConfig;
use std::{env, sync::Mutex};
use swordfish_types::SFEvent;
use tauri::{App, GlobalShortcutManager, Manager, State};
use tokio;
use tracing::{error, info};

fn handle_shortcuts(app: &App) {
    let s: State<AppState> = app.state();
    let mut gsm = app.global_shortcut_manager();
    let h = app.app_handle();
    let w = acquire_main_window(&h);
    s.config
        .lock()
        .map(|config| config.read().launch_shortcut)
        .ok()
        .and_then(|shortcut| {
            gsm.register(shortcut.as_str(), move || {
                if let Ok(bool) = w.is_visible() {
                    if bool {
                        h.emit_all(SFEvent::MainWindowHidden.as_ref(), ()).ok();
                        hide_main_window(h.clone());
                    } else {
                        h.emit_all(SFEvent::MainWindowShown.as_ref(), ()).ok();
                        show_main_window(h.clone());
                    }
                }
            })
            .ok()
        })
        .and_then(|_| {
            info!("Global shortcuts registered");
            None::<()>
        })
        .or_else(|| {
            error!("Failed to register global shortcut");
            None
        });
}

struct AppState {
    config: Mutex<AppConfig>,
}
#[tokio::main]
async fn main() {
    let query_engine = QueryEngine::new();
    tauri::async_runtime::set(tokio::runtime::Handle::current());

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
        .manage(AppState {
            config: Mutex::new(AppConfig::new()),
        })
        .setup(move |app| {
            #[cfg(target_os = "macos")]
            {
                app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            }

            let app_handle = app.app_handle();

            let main_window = acquire_main_window(&app_handle);
            main_window.set_always_on_top(true)?;
            if env::var_os("SWORDFISH_DEV").is_some() && !main_window.is_devtools_open() {
                main_window.open_devtools()
            }

            acquire_settings_window(&app_handle);
            handle_shortcuts(app);

            let handle = app.handle();

            let _id = app_handle.listen_global(SFEvent::Query.as_ref(), move |event| {
                if let Some(str) = event.payload() {
                    if let Ok(query) = serde_json::from_str(str) {
                        let res = query_engine.query(query);
                        let _ = handle.emit_all(SFEvent::QueryResult.as_ref(), res);
                    }
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("App crashed");
}
