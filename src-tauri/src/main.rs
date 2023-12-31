// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod DataSource;
mod browser;
mod constants;
mod query;
mod query_engine;
mod tray;
mod windows;

use std::borrow::Borrow;

use crate::{
    query::{get_query_result, Query},
    tray::{handle_tray_event, make_tray},
    windows::{
        acquire_main_window, acquire_settings_window, hide_main_window, hide_settings_window,
        show_main_window, show_settings_window, toggle_main_window, toggle_settings_window,
    },
};
use query::QueryResult;
use query_engine::{QueryEngine, QueryInterface};
use tauri::{
    utils::config::WindowUrl, window::WindowBuilder, App, AppHandle, CustomMenuItem, Manager,
    SystemTray, SystemTrayEvent, SystemTrayMenu,
};
use tauri::{Event, GlobalShortcutManager};
use DataSource::{BrowserHistoryDataSource, DataSource as _};

fn handle_shortcuts(app: &App) {
    let mut gsm = app.global_shortcut_manager();
    let h = app.app_handle();
    let w = acquire_main_window(&h);
    match gsm.register("CommandOrControl+k", move || {
        if let Ok(bool) = w.is_visible() {
            if bool {
                println!("hide window");
                hide_main_window(h.clone());
                ()
            }
            println!("show window");
            show_main_window(h.clone());
        }
    }) {
        Ok(_) => println!("Registered the shortcut successfully"),
        Err(e) => println!("Error registering global shortcut: {}", e),
    };
}

fn handle_keypresses(event: Event) {
    println!("got keypress event with payload {:?}", event.payload());
}

fn main() {
    let QUERY_ENGINE: QueryEngine = QueryEngine::new();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_query_result,
            show_main_window,
            hide_main_window,
            toggle_main_window,
            show_settings_window,
            hide_settings_window,
            toggle_settings_window,
        ])
        .system_tray(make_tray())
        .on_system_tray_event(handle_tray_event)
        .setup(move |app| {
            let app_handle = app.app_handle();
            let main_window = acquire_main_window(&app_handle);
            main_window.set_always_on_top(true)?;
            if !main_window.is_devtools_open() {
                main_window.open_devtools()
            }

            acquire_settings_window(&app_handle);
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            handle_shortcuts(app);

            let _id = app_handle.listen_global("keypress", |event| {
                handle_keypresses(event);
            });

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
