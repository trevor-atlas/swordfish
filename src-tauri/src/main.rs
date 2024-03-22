// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod browser;
mod constants;
mod datasource;
mod query;
mod query_engine;
mod search;
mod settings;
mod tray;
mod windows;

use crate::{
    search::filename::search,
    tray::{handle_tray_event, make_tray},
    windows::{
        acquire_main_window, acquire_settings_window, hide_main_window, hide_settings_window,
        show_main_window, show_settings_window, toggle_main_window, toggle_settings_window,
    },
};

use datasource::DataSource as _;

use query::Query;
use query_engine::{QueryEngine, QueryInterface};
use settings::AppConfig;
use std::{env, sync::Mutex};
use tauri::GlobalShortcutManager;
use tauri::{App, Manager, State};

fn handle_shortcuts(app: &App) {
    let mut gsm = app.global_shortcut_manager();
    let h = app.app_handle();
    let w = acquire_main_window(&h);
    match gsm.register("Control+Space", move || {
        if let Ok(bool) = w.is_visible() {
            if bool {
                h.emit_all("appwindow:hidden", ()).ok();
                hide_main_window(h.clone());
            } else {
                show_main_window(h.clone());
            }
        }
    }) {
        Ok(_) => println!("Registered the shortcut successfully"),
        Err(e) => println!("Error registering global shortcut: {}", e),
    };
}

struct AppState {
    config: Mutex<AppConfig>,
}

fn main() {
    let QUERY_ENGINE: QueryEngine = QueryEngine::new();
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

            let s: State<AppState> = app.state();
            let mut lock = s.config.try_lock();
            if let Ok(ref mut mutex) = lock {
                mutex
                    .get_search_directories()
                    .iter()
                    .for_each(|p| println!("{:?}", p));
            } else {
                println!("try_lock failed");
            }
            let main_window = acquire_main_window(&app_handle);
            main_window.set_always_on_top(true)?;
            if env::var_os("SWORDFISH_DEV").is_some() && !main_window.is_devtools_open() {
                main_window.open_devtools()
            }

            acquire_settings_window(&app_handle);
            handle_shortcuts(app);

            let handle = app.handle();
            let id = app_handle.listen_global("query", move |event| {
                let q: Result<Query, serde_json::Error> = match event.payload() {
                    Some(str) => serde_json::from_str(str),
                    None => {
                        println!("Unable to parse query string into Query");
                        return;
                    }
                };

                match q {
                    Ok(query) => {
                        let res = QUERY_ENGINE.query(query);
                        handle.emit_all("query", res);
                    }
                    Err(e) => println!("Error in query {}", e),
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("App crashed");
}
