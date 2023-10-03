// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// #[cfg(target_os = "macos")]
// #[macro_use]
// extern crate objc;
use tauri::{Manager, WindowBuilder, WindowUrl};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

const WIDTH: f64 = 620.0;
const HEIGHT: f64 = 500.0;

fn main() {
    // let ctx = tauri::generate_context!();

    let _app = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .setup(move |app| {
            let window = WindowBuilder::new(
                app,
                "Swordfish".to_string(),
                WindowUrl::App("index.html".into()),
            )
            .title("Swordfish")
            // .decorations(false)
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
