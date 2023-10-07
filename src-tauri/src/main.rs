// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use query::{Query, QueryResult, QueryResultPreview, QueryResultType};
// #[cfg(target_os = "macos")]
// #[macro_use]
// extern crate objc;
use tauri::{
    utils::config::WindowUrl, window::WindowBuilder, AppHandle, CustomMenuItem, Manager,
    SystemTray, SystemTrayEvent, SystemTrayMenu,
};
mod query;

#[tauri::command]
fn get_query_result(query: Query) -> Vec<QueryResult> {
    vec![
        QueryResult {
            heading: "Exodia".to_string(),
            subheading: "The forbidden one".to_string(),
            preview: Some(QueryResultPreview::ClipboardPreview {
                filepath: None,
                text: Some("Its a clipboard entry :)".to_string()),
            }),
            r#type: QueryResultType::Other,
        },
        QueryResult {
            heading: "Halle Berry".to_string(),
            subheading: "Still hot tbh".to_string(),
            preview: None,
            r#type: QueryResultType::Other,
        },
        QueryResult {
            heading: "The Pope (really)".to_string(),
            subheading: "He is old".to_string(),
            preview: None,
            r#type: QueryResultType::Other,
        },
        QueryResult {
            heading: "MOOG".to_string(),
            subheading: "They kinda stink as a company but beep boop".to_string(),
            preview: None,
            r#type: QueryResultType::Other,
        },
        QueryResult {
            heading: "HubSpot".to_string(),
            subheading: "Its okay! Really!".to_string(),
            preview: None,
            r#type: QueryResultType::Other,
        },
        QueryResult {
            heading: "Stream Deck".to_string(),
            subheading: "It could be better, but it is aight".to_string(),
            preview: None,
            r#type: QueryResultType::Other,
        },
        QueryResult {
            heading: "GGWP BGEZ".to_string(),
            subheading: "Nerds are so rude".to_string(),
            preview: None,
            r#type: QueryResultType::Other,
        },
        QueryResult {
            heading: "Exodia".to_string(),
            subheading: "The forbidden one".to_string(),
            preview: Some(QueryResultPreview::ClipboardPreview {
                filepath: None,
                text: Some("Its a clipboard entry :)".to_string()),
            }),
            r#type: QueryResultType::Other,
        },
        QueryResult {
            heading: "Halle Berry".to_string(),
            subheading: "Still hot tbh".to_string(),
            preview: None,
            r#type: QueryResultType::Other,
        },
        QueryResult {
            heading: "The Pope (really)".to_string(),
            subheading: "He is old".to_string(),
            preview: None,
            r#type: QueryResultType::Other,
        },
        QueryResult {
            heading: "MOOG".to_string(),
            subheading: "They kinda stink as a company but beep boop".to_string(),
            preview: None,
            r#type: QueryResultType::Other,
        },
        QueryResult {
            heading: "HubSpot".to_string(),
            subheading: "Its okay! Really!".to_string(),
            preview: None,
            r#type: QueryResultType::Other,
        },
        QueryResult {
            heading: "Stream Deck".to_string(),
            subheading: "It could be better, but it is aight".to_string(),
            preview: None,
            r#type: QueryResultType::Other,
        },
        QueryResult {
            heading: "GGWP BGEZ".to_string(),
            subheading: "Nerds are so rude".to_string(),
            preview: None,
            r#type: QueryResultType::Other,
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

#[tauri::command]
fn hide_main_window(app: AppHandle) {
    let window = get_main_window(&app);
    let menu_item = app.tray_handle().get_item("toggle");
    if let Ok(_) = window.hide() {
        _ = app.hide();
        _ = menu_item.set_title("Show");
    };
}

#[tauri::command]
fn show_main_window(app: AppHandle) {
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
fn toggle_main_window(app: AppHandle) {
    let window = get_main_window(&app);
    if let Ok(is_visible) = window.is_visible() {
        if is_visible {
            hide_main_window(app)
        } else {
            show_main_window(app);
        }
    }
}

const WIDTH: f64 = 750.0;
const HEIGHT: f64 = 500.0;
const MAIN_WINDOW_HANDLE: &str = "main";

fn get_main_window(app: &AppHandle) -> tauri::Window {
    match app.get_window(MAIN_WINDOW_HANDLE) {
        Some(win) => win,
        None => WindowBuilder::new(app, MAIN_WINDOW_HANDLE, WindowUrl::App("index.html".into()))
            .title("Swordfish")
            .decorations(false)
            .accept_first_mouse(true)
            .visible(false)
            .transparent(true)
            .skip_taskbar(true)
            .disable_file_drop_handler()
            .inner_size(WIDTH, HEIGHT)
            .build()
            .expect("Unable to create searchbar window"),
    }
}

fn make_tray() -> SystemTray {
    let menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("toggle".to_string(), "Hide"))
        .add_item(CustomMenuItem::new("quit".to_string(), "Quit"));
    return SystemTray::new().with_menu(menu);
}

fn handle_tray_event(app_handle: &AppHandle, event: SystemTrayEvent) {
    if let SystemTrayEvent::MenuItemClick { id, .. } = event {
        match id.as_str() {
            "quit" => {
                std::process::exit(0);
            }
            "toggle" => {
                toggle_main_window(app_handle.clone());
            }
            _ => {}
        }
    }
}

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
            toggle_main_window
        ])
        .system_tray(make_tray())
        .on_system_tray_event(handle_tray_event)
        .setup(move |app| {
            let app_handle = app.app_handle();
            let window = get_main_window(&app_handle);
            // window.set_resizable(false)?;
            window.set_always_on_top(true)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("App crashed");
}
