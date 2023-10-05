// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
// #[cfg(target_os = "macos")]
// #[macro_use]
// extern crate objc;
use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyManager,
};
use tauri::{Manager, WindowBuilder, WindowUrl};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Deserialize)]
enum QueryMode {
    Clipboard,
    BrowserHistory,
    Files,
    Scripts,
    Chat,
}

#[derive(Deserialize)]
struct Query {
    search_string: String,
    mode: QueryMode,
}

#[derive(Deserialize, Serialize, Clone)]
struct QueryResult {
    heading: String,
    subheading: String,
}

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

#[tauri::command]
fn get_query_result(query: Query) -> Vec<QueryResult> {
    vec![
        QueryResult {
            heading: "Exodia".to_string(),
            subheading: "The forbidden one".to_string(),
        },
        QueryResult {
            heading: "Halle Berry".to_string(),
            subheading: "Still hot tbh".to_string(),
        },
        QueryResult {
            heading: "The Pope (really)".to_string(),
            subheading: "He is old".to_string(),
        },
        QueryResult {
            heading: "MOOG".to_string(),
            subheading: "They kinda stink as a company but beep boop".to_string(),
        },
        QueryResult {
            heading: "HubSpot".to_string(),
            subheading: "Its okay! Really!".to_string(),
        },
        QueryResult {
            heading: "Stream Deck".to_string(),
            subheading: "It could be better, but it is aight".to_string(),
        },
        QueryResult {
            heading: "GGWP BGEZ".to_string(),
            subheading: "Nerds are so rude".to_string(),
        },
    ]
    .iter()
    .cloned()
    .filter(|item| {
        item.heading
            .to_lowercase()
            .contains(&query.search_string.to_lowercase())
    })
    .collect()
}

const WIDTH: f64 = 750.0;
const HEIGHT: f64 = 500.0;

fn main() {
    let manager = GlobalHotKeyManager::new().unwrap();
    let hotkey = HotKey::new(Some(Modifiers::SHIFT), Code::KeyD);
    manager.register(hotkey);
    let _app = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, get_query_result])
        .setup(move |app| {
            let window = WindowBuilder::new(
                app,
                "Swordfish".to_string(),
                WindowUrl::App("index.html".into()),
            )
            .title("Swordfish")
            .decorations(false)
            .visible(true)
            .transparent(true)
            .disable_file_drop_handler()
            .inner_size(WIDTH, HEIGHT)
            .build()
            .expect("Unable to create searchbar window");
            window.set_resizable(false)?;
            window.set_always_on_top(true)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("App crashed");
}
